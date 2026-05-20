use crate::pregel::{
    ComputeContext, DefaultValue, InitContext, MasterComputeContext, Messages, PregelResult,
    PregelSchema, PregelSchemaBuilder, SyncQueueMessageIterator, Visibility,
};
use crate::ValueType;
use std::sync::Arc;

use super::{HitsPregelRuntimeConfig, HitsRunResult};

const HUB_KEY: &str = "hub";
const AUTH_KEY: &str = "authority";

fn hits_schema(hub_property: &str, auth_property: &str) -> PregelSchema {
    PregelSchemaBuilder::new()
        .add_with_default(hub_property, DefaultValue::Double(1.0), Visibility::Public)
        .add_with_default(auth_property, DefaultValue::Double(1.0), Visibility::Public)
        // Keep a canonical node value as well (some infrastructure expects one)
        .add_public("value", ValueType::Double)
        .build()
}

/// Run HITS on a pre-projected graph.
///
/// Pure state runtime for HITS.
///
/// Provides Pregel schema/init/compute/master functions and finalization without owning graph
/// access or execution orchestration.
#[derive(Clone, Debug)]
pub struct HitsComputationRuntime {
    hub_property: String,
    auth_property: String,
}

impl HitsComputationRuntime {
    pub fn new(_tolerance: f64) -> Self {
        Self::with_properties(_tolerance, HUB_KEY.to_string(), AUTH_KEY.to_string())
    }

    pub fn with_properties(_tolerance: f64, hub_property: String, auth_property: String) -> Self {
        Self {
            hub_property,
            auth_property,
        }
    }

    pub fn schema(&self) -> PregelSchema {
        hits_schema(&self.hub_property, &self.auth_property)
    }

    pub fn init_fn(&self) -> Arc<dyn Fn(&mut InitContext<HitsPregelRuntimeConfig>) + Send + Sync> {
        let hub_property = self.hub_property.clone();
        let auth_property = self.auth_property.clone();

        Arc::new(move |context: &mut InitContext<HitsPregelRuntimeConfig>| {
            context.set_node_value(&hub_property, 1.0);
            context.set_node_value(&auth_property, 1.0);
            context.set_node_value("value", 0.0);
        })
    }

    pub fn compute_fn(
        &self,
    ) -> Arc<
        dyn Fn(
                &mut ComputeContext<HitsPregelRuntimeConfig, SyncQueueMessageIterator>,
                &mut Messages<SyncQueueMessageIterator>,
            ) + Send
            + Sync,
    > {
        let hub_property = self.hub_property.clone();
        let auth_property = self.auth_property.clone();

        Arc::new(
            move |context: &mut ComputeContext<
                HitsPregelRuntimeConfig,
                SyncQueueMessageIterator,
            >,
                  messages: &mut Messages<SyncQueueMessageIterator>| {
                let superstep = context.superstep();

                if superstep == 0 {
                    let auth = context.incoming_degree() as f64;
                    context.set_node_value(&auth_property, auth);
                    return;
                }

                match superstep % 4 {
                    // CALCULATE_AUTHS: sum incoming hub messages.
                    0 => {
                        let mut sum = 0.0f64;
                        for m in messages.by_ref() {
                            sum += m;
                        }
                        context.set_node_value(&auth_property, sum);
                    }
                    // NORMALIZE_AUTHS: master normalized the authority value; send it backwards.
                    1 => {
                        let auth = context.double_node_value(&auth_property);
                        context.send_to_incoming_neighbors(auth);
                    }
                    // CALCULATE_HUBS: sum authority messages.
                    2 => {
                        let mut sum = 0.0f64;
                        for m in messages.by_ref() {
                            sum += m;
                        }
                        context.set_node_value(&hub_property, sum);
                    }
                    // NORMALIZE_HUBS: master normalized the hub value; send it forwards.
                    _ => {
                        let hub = context.double_node_value(&hub_property);
                        context.send_to_neighbors(hub);
                    }
                }
            },
        )
    }

    pub fn master_compute_fn(
        &self,
    ) -> impl Fn(&mut MasterComputeContext<HitsPregelRuntimeConfig>) -> bool + Send + Sync + 'static
    {
        let hub_property = self.hub_property.clone();
        let auth_property = self.auth_property.clone();

        move |context: &mut MasterComputeContext<HitsPregelRuntimeConfig>| {
            let superstep = context.superstep();

            match superstep % 4 {
                // NORMALIZE_AUTHS
                0 => {
                    normalize_property(context, &auth_property);
                }
                // NORMALIZE_HUBS + convergence check
                2 => {
                    normalize_property(context, &hub_property);
                }
                _ => {}
            }

            false
        }
    }

    pub fn finalize(&self, result: &PregelResult, node_count: usize) -> HitsRunResult {
        let node_values = Arc::clone(&result.node_values);

        let mut hubs = vec![0.0f64; node_count];
        let mut auths = vec![0.0f64; node_count];

        for node_id in 0..node_count {
            hubs[node_id] = node_values.double_value(&self.hub_property, node_id);
            auths[node_id] = node_values.double_value(&self.auth_property, node_id);
        }

        // Translate pregel supersteps back into algorithm iterations.
        let ran_supersteps = result.ran_iterations;
        let ran_iterations = if ran_supersteps <= 1 {
            0
        } else {
            // After the initial seed step, each full HITS iteration consumes 4 supersteps.
            ((ran_supersteps - 1) / 4).max(1)
        };

        HitsRunResult {
            hub_scores: hubs,
            authority_scores: auths,
            iterations_ran: ran_iterations,
            did_converge: result.did_converge,
        }
    }
}

fn normalize_property(context: &mut MasterComputeContext<HitsPregelRuntimeConfig>, property: &str) {
    let mut sum_sq = 0.0f64;
    let node_count = context.node_count();

    for node_id in 0..node_count {
        let value = context.double_node_value(node_id, property);
        sum_sq += value * value;
    }

    let norm = sum_sq.sqrt();
    let denom = if norm > 0.0 && norm.is_finite() {
        norm
    } else {
        1.0
    };

    for node_id in 0..node_count {
        let normalized = context.double_node_value(node_id, property) / denom;
        context.set_double_node_value(node_id, property, normalized);
    }
}
