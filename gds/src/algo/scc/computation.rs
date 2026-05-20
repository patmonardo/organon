//! SCC Computation Runtime
//!
//! Implements an iterative, stack-based SCC algorithm (path-based / Tarjan-family)
//! suitable for large graphs without relying on recursion.

use crate::collections::HugeLongArray;
use crate::concurrency::TerminationFlag;
use crate::core::utils::paged::HugeLongArrayStack;
use crate::core::utils::progress::ProgressTracker;
use crate::types::graph::Graph;

#[derive(Clone)]
pub struct SccComputationResult {
    pub components: Vec<u64>,
    pub component_count: usize,
    pub computation_time_ms: u64,
}

pub struct SccComputationRuntime {}

impl Default for SccComputationRuntime {
    fn default() -> Self {
        Self::new()
    }
}

impl SccComputationRuntime {
    pub fn new() -> Self {
        Self {}
    }

    pub fn compute(
        &mut self,
        graph: &dyn Graph,
        progress_tracker: &mut dyn ProgressTracker,
        termination_flag: &TerminationFlag,
    ) -> Result<(Vec<u64>, usize), String> {
        let node_count = graph.node_count();
        if node_count == 0 {
            return Ok((Vec::new(), 0));
        }

        let mut index = HugeLongArray::new(node_count);
        let mut component = HugeLongArray::new(node_count);
        index.fill(-1);
        component.fill(-1);

        let mut stack = HugeLongArrayStack::new(node_count);
        let mut boundaries = HugeLongArrayStack::new(node_count);

        let fallback = graph.default_property_value();

        let mut next_index: i64 = 0;
        let mut component_count: usize = 0;

        #[derive(Debug)]
        struct Frame {
            node: usize,
            neighbors: Vec<usize>,
            next_neighbor: usize,
        }

        for start_node in 0..node_count {
            if index.get(start_node) != -1 {
                continue;
            }

            let mut frames: Vec<Frame> = Vec::new();

            // Enter start node
            index.set(start_node, next_index);
            next_index += 1;
            stack.push(start_node as i64);
            boundaries.push(index.get(start_node));

            let neighbors: Vec<usize> = graph
                .stream_relationships(start_node as i64, fallback)
                .map(|c| c.target_id())
                .filter(|t| *t >= 0)
                .map(|t| t as usize)
                .collect();
            frames.push(Frame {
                node: start_node,
                neighbors,
                next_neighbor: 0,
            });

            loop {
                if !termination_flag.running() {
                    return Err("terminated".to_string());
                }

                let Some(top) = frames.last_mut() else {
                    break;
                };

                if top.next_neighbor < top.neighbors.len() {
                    let w = top.neighbors[top.next_neighbor];
                    top.next_neighbor += 1;

                    if index.get(w) == -1 {
                        // Descend to unvisited neighbor
                        index.set(w, next_index);
                        next_index += 1;
                        stack.push(w as i64);
                        boundaries.push(index.get(w));

                        let w_neighbors: Vec<usize> = graph
                            .stream_relationships(w as i64, fallback)
                            .map(|c| c.target_id())
                            .filter(|t| *t >= 0)
                            .map(|t| t as usize)
                            .collect();

                        frames.push(Frame {
                            node: w,
                            neighbors: w_neighbors,
                            next_neighbor: 0,
                        });
                        continue;
                    }

                    // If neighbor is discovered but not yet assigned, adjust boundaries.
                    if component.get(w) == -1 {
                        let w_index = index.get(w);
                        while !boundaries.is_empty() && boundaries.peek() > w_index {
                            boundaries.pop();
                        }
                    }

                    continue;
                }

                // Finished exploring `top.node`.
                let v = top.node;
                let v_index = index.get(v);

                if !boundaries.is_empty() && boundaries.peek() == v_index {
                    boundaries.pop();
                    let component_id = v as i64;

                    loop {
                        let w = stack.pop() as usize;
                        component.set(w, component_id);
                        if w == v {
                            break;
                        }
                    }
                    component_count += 1;
                }

                progress_tracker.log_progress(1);
                frames.pop();
            }
        }

        let mut components: Vec<u64> = vec![0u64; node_count];
        for i in 0..node_count {
            let c = component.get(i);
            if c < 0 {
                return Err(format!(
                    "internal error: node {i} was not assigned to a component"
                ));
            }
            components[i] = c as u64;
        }

        Ok((components, component_count))
    }
}
