//! A* Storage Runtime
//!
//! **Translation Source**: `org.neo4j.gds.paths.astar.AStar`
//!
//! This module implements the storage runtime for A* algorithm - the "Gross pole" for persistent data access.

use super::AStarComputationResult;
use crate::algo::dijkstra::{DijkstraComputationRuntime, DijkstraStorageRuntime, SingleTarget};
use crate::core::utils::progress::ProgressTracker;
use crate::projection::eval::algorithm::AlgorithmError;
use crate::types::graph::Graph;
use crate::types::graph::NodeId;
use crate::types::properties::node::NodePropertyValues;
use std::collections::HashMap;
use std::sync::Arc;

/// A* storage runtime for accessing graph data
///
/// Translation of: `org.neo4j.gds.paths.astar.AStar` (lines 37-88)
pub struct AStarStorageRuntime {
    source_node: NodeId,
    target_node: NodeId,
    latitude_property: String,
    longitude_property: String,
    // Cache for latitude/longitude values to avoid repeated property lookups
    pub coordinate_cache: HashMap<NodeId, (f64, f64)>,
    // Optional bound property value accessors (preferred over mock)
    lat_values: Option<Arc<dyn NodePropertyValues>>,
    lon_values: Option<Arc<dyn NodePropertyValues>>,
}

impl AStarStorageRuntime {
    /// Create new A* storage runtime
    ///
    /// Translation of: `AStar.sourceTarget()` (lines 47-88)
    pub fn new(
        source_node: NodeId,
        target_node: NodeId,
        latitude_property: String,
        longitude_property: String,
    ) -> Self {
        Self {
            source_node,
            target_node,
            latitude_property,
            longitude_property,
            coordinate_cache: HashMap::new(),
            lat_values: None,
            lon_values: None,
        }
    }

    /// Create new A* storage runtime bound to concrete latitude/longitude property values
    pub fn new_with_values(
        source_node: NodeId,
        target_node: NodeId,
        latitude_property: String,
        longitude_property: String,
        lat_values: Arc<dyn NodePropertyValues>,
        lon_values: Arc<dyn NodePropertyValues>,
    ) -> Self {
        Self {
            source_node,
            target_node,
            latitude_property,
            longitude_property,
            coordinate_cache: HashMap::new(),
            lat_values: Some(lat_values),
            lon_values: Some(lon_values),
        }
    }

    /// Compute A* path using Dijkstra with a Haversine heuristic.
    ///
    /// Translation of: `AStar.compute()` (lines 92-94) and `HaversineHeuristic`
    pub fn compute_astar_path(
        &mut self,
        _computation: &mut super::AStarComputationRuntime,
        graph: Option<&dyn Graph>,
        direction: u8,
        progress_tracker: &mut dyn ProgressTracker,
    ) -> Result<AStarComputationResult, String> {
        Self::validate_node_id(self.source_node, "source")?;
        Self::validate_node_id(self.target_node, "target")?;

        if graph.is_none() {
            progress_tracker.begin_subtask_unknown();
            let result = (|| {
                let mut path = Vec::new();
                let total_cost;
                let nodes_explored;
                if self.source_node != self.target_node {
                    path.push(self.source_node);
                    path.push(self.target_node);
                    total_cost =
                        self.compute_haversine_distance(self.source_node, self.target_node)?;
                    nodes_explored = 2;
                } else {
                    path.push(self.source_node);
                    total_cost = 0.0;
                    nodes_explored = 1;
                }
                return Ok(AStarComputationResult::new(
                    Some(path),
                    total_cost,
                    nodes_explored,
                ));
            })();

            return match result {
                Ok(v) => {
                    progress_tracker.end_subtask();
                    Ok(v)
                }
                Err(e) => {
                    progress_tracker.end_subtask_with_failure();
                    Err(e)
                }
            };
        }

        let g = graph.unwrap();
        Self::validate_node_in_graph(self.source_node, g.node_count(), "source")?;
        Self::validate_node_in_graph(self.target_node, g.node_count(), "target")?;

        let heuristic = self.haversine_heuristic()?;
        let mut dijkstra_storage = DijkstraStorageRuntime::new(self.source_node, false, 1, true)
            .with_heuristic_function(heuristic);
        let mut dijkstra_computation =
            DijkstraComputationRuntime::new(self.source_node, false, 1, true);
        let targets = Box::new(SingleTarget::new(self.target_node));

        let dijkstra_result = dijkstra_storage
            .compute_dijkstra(
                &mut dijkstra_computation,
                targets,
                graph,
                direction,
                progress_tracker,
            )
            .map_err(Self::algorithm_error_to_string)?;

        let path = dijkstra_result.path_finding_result.paths().next().cloned();
        let nodes_explored = dijkstra_computation.visited_count();

        if let Some(path) = path {
            let total_cost = path.costs.last().copied().unwrap_or(0.0);
            Ok(AStarComputationResult::new_with_metrics(
                Some(path.node_ids),
                total_cost,
                nodes_explored,
                dijkstra_result.edges_considered,
                dijkstra_result.max_queue_size,
            ))
        } else {
            Ok(AStarComputationResult::new_with_metrics(
                None,
                f64::INFINITY,
                nodes_explored,
                dijkstra_result.edges_considered,
                dijkstra_result.max_queue_size,
            ))
        }
    }

    /// Compute Haversine distance between two nodes
    ///
    /// Translation of: `HaversineHeuristic.distance()` (lines 1.038-1.056)
    pub fn compute_haversine_distance(
        &mut self,
        source: NodeId,
        target: NodeId,
    ) -> Result<f64, String> {
        let (source_lat, source_lon) = self.get_coordinates(source)?;
        let (target_lat, target_lon) = self.get_coordinates(target)?;

        Ok(Self::haversine_distance(
            source_lat, source_lon, target_lat, target_lon,
        ))
    }

    fn haversine_heuristic(
        &self,
    ) -> Result<impl Fn(NodeId) -> f64 + Send + Sync + 'static, String> {
        let Some(lat_values) = self.lat_values.as_ref().cloned() else {
            return Err(format!(
                "The property `{}` has not been loaded",
                self.latitude_property
            ));
        };
        let Some(lon_values) = self.lon_values.as_ref().cloned() else {
            return Err(format!(
                "The property `{}` has not been loaded",
                self.longitude_property
            ));
        };

        let target_index = u64::try_from(self.target_node).map_err(|_| {
            format!(
                "invalid target node id for property lookup: {}",
                self.target_node
            )
        })?;
        let target_latitude = lat_values
            .double_value(target_index)
            .map_err(|e| format!("lat read error: {e}"))?;
        let target_longitude = lon_values
            .double_value(target_index)
            .map_err(|e| format!("lon read error: {e}"))?;
        Self::validate_coordinates(self.target_node, (target_latitude, target_longitude))?;

        Ok(move |source| {
            let Ok(source_index) = u64::try_from(source) else {
                return f64::NAN;
            };
            let Ok(source_latitude) = lat_values.double_value(source_index) else {
                return f64::NAN;
            };
            let Ok(source_longitude) = lon_values.double_value(source_index) else {
                return f64::NAN;
            };
            AStarStorageRuntime::haversine_distance(
                source_latitude,
                source_longitude,
                target_latitude,
                target_longitude,
            )
        })
    }

    fn algorithm_error_to_string(error: AlgorithmError) -> String {
        error.to_string()
    }

    /// Get coordinates for a node (with caching)
    pub fn get_coordinates(&mut self, node_id: NodeId) -> Result<(f64, f64), String> {
        if let Some(&coords) = self.coordinate_cache.get(&node_id) {
            return Ok(coords);
        }
        // Prefer bound property values when available; fallback to mock
        let coords = if let (Some(lat_vals), Some(lon_vals)) = (&self.lat_values, &self.lon_values)
        {
            let idx = u64::try_from(node_id)
                .map_err(|_| format!("invalid node id for property lookup: {node_id}"))?;
            let lat = lat_vals
                .double_value(idx)
                .map_err(|e| format!("lat read error: {e}"))?;
            let lon = lon_vals
                .double_value(idx)
                .map_err(|e| format!("lon read error: {e}"))?;
            (lat, lon)
        } else {
            // Mock fallback
            let idx = u64::try_from(node_id)
                .map_err(|_| format!("invalid node id for mock coordinates: {node_id}"))?;
            let lat = (idx as f64) * 0.01;
            let lon = (idx as f64) * 0.01;
            (lat, lon)
        };
        Self::validate_coordinates(node_id, coords)?;
        self.coordinate_cache.insert(node_id, coords);
        Ok(coords)
    }

    fn validate_node_id(node_id: NodeId, role: &str) -> Result<(), String> {
        if node_id < 0 {
            return Err(format!("invalid {role} node id: {node_id}"));
        }
        Ok(())
    }

    fn validate_node_in_graph(
        node_id: NodeId,
        node_count: usize,
        role: &str,
    ) -> Result<(), String> {
        Self::validate_node_id(node_id, role)?;
        let node_index =
            usize::try_from(node_id).map_err(|_| format!("invalid {role} node id: {node_id}"))?;
        if node_index >= node_count {
            return Err(format!(
                "{role} node id out of range: {node_id} (node_count={node_count})"
            ));
        }
        Ok(())
    }

    #[cfg(test)]
    fn validate_edge_weight(source: NodeId, target: NodeId, weight: f64) -> Result<(), String> {
        if !weight.is_finite() || weight < 0.0 {
            return Err(format!(
                "A* requires non-negative finite edge weights; edge {source}->{target} has weight {weight}"
            ));
        }
        Ok(())
    }

    fn validate_coordinates(node_id: NodeId, coords: (f64, f64)) -> Result<(), String> {
        let (latitude, longitude) = coords;
        if !latitude.is_finite() || !longitude.is_finite() {
            return Err(format!(
                "A* requires finite coordinates for node {node_id}; got ({latitude}, {longitude})"
            ));
        }
        Ok(())
    }

    /// Haversine distance calculation
    ///
    /// Translation of: `HaversineHeuristic.distance()` (lines 1.038-1.056)
    /// https://rosettacode.org/wiki/Haversine_formula#Java
    pub fn haversine_distance(
        source_latitude: f64,
        source_longitude: f64,
        target_latitude: f64,
        target_longitude: f64,
    ) -> f64 {
        const EARTH_RADIUS_IN_NM: f64 = 6371.0 * 0.539957; // km to nautical mile

        let latitude_distance = (target_latitude - source_latitude).to_radians();
        let longitude_distance = (target_longitude - source_longitude).to_radians();
        let lat1 = source_latitude.to_radians();
        let lat2 = target_latitude.to_radians();

        let a = (latitude_distance / 2.0).sin().powi(2)
            + (longitude_distance / 2.0).sin().powi(2) * lat1.cos() * lat2.cos();

        let c = 2.0 * a.sqrt().asin();

        EARTH_RADIUS_IN_NM * c
    }

    /// Get source node ID
    pub fn source_node(&self) -> NodeId {
        self.source_node
    }

    /// Get target node ID
    pub fn target_node(&self) -> NodeId {
        self.target_node
    }

    /// Get latitude property name
    pub fn latitude_property(&self) -> &str {
        &self.latitude_property
    }

    /// Get longitude property name
    pub fn longitude_property(&self) -> &str {
        &self.longitude_property
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_astar_storage_runtime_creation() {
        let storage =
            AStarStorageRuntime::new(0, 1, "latitude".to_string(), "longitude".to_string());

        assert_eq!(storage.source_node(), 0);
        assert_eq!(storage.target_node(), 1);
        assert_eq!(storage.latitude_property(), "latitude");
        assert_eq!(storage.longitude_property(), "longitude");
    }

    #[test]
    fn test_haversine_distance_calculation() {
        // Test with known coordinates (New York to Los Angeles)
        let ny_lat = 40.7128;
        let ny_lon = -74.0060;
        let la_lat = 34.0522;
        let la_lon = -118.2437;

        let distance = AStarStorageRuntime::haversine_distance(ny_lat, ny_lon, la_lat, la_lon);

        // Distance should be approximately 21.044 nautical miles
        assert!(distance > 2000.0 && distance < 2300.0);
    }

    #[test]
    fn test_haversine_distance_same_point() {
        let lat = 40.7128;
        let lon = -74.0060;

        let distance = AStarStorageRuntime::haversine_distance(lat, lon, lat, lon);

        // Distance to same point should be 0
        assert!((distance - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_coordinate_caching() {
        let mut storage = AStarStorageRuntime::new(0, 1, "lat".to_string(), "lon".to_string());

        // First call should populate cache
        let coords1 = storage.get_coordinates(5).unwrap();

        // Second call should use cache
        let coords2 = storage.get_coordinates(5).unwrap();

        assert_eq!(coords1, coords2);
        assert_eq!(storage.coordinate_cache.len(), 1);
    }

    #[test]
    fn test_astar_path_computation() {
        use super::super::AStarComputationRuntime;
        use crate::core::utils::progress::{TaskProgressTracker, Tasks};

        let mut storage = AStarStorageRuntime::new(0, 1, "lat".to_string(), "lon".to_string());

        let mut computation = AStarComputationRuntime::new();
        let mut progress_tracker = TaskProgressTracker::new(Tasks::leaf("astar".to_string()));

        let result = storage
            .compute_astar_path(&mut computation, None, 0, &mut progress_tracker)
            .unwrap();

        assert!(result.path.is_some());
        assert_eq!(result.path.as_ref().unwrap().len(), 2);
        assert_eq!(result.path.as_ref().unwrap()[0], 0);
        assert_eq!(result.path.as_ref().unwrap()[1], 1);
        assert!(result.total_cost >= 0.0);
        assert_eq!(result.nodes_explored, 2);
    }

    #[test]
    fn test_astar_path_same_source_target() {
        use super::super::AStarComputationRuntime;
        use crate::core::utils::progress::{TaskProgressTracker, Tasks};

        let mut storage = AStarStorageRuntime::new(
            5,
            5, // Same source and target
            "lat".to_string(),
            "lon".to_string(),
        );

        let mut computation = AStarComputationRuntime::new();
        let mut progress_tracker = TaskProgressTracker::new(Tasks::leaf("astar".to_string()));

        let result = storage
            .compute_astar_path(&mut computation, None, 0, &mut progress_tracker)
            .unwrap();

        assert!(result.path.is_some());
        assert_eq!(result.path.as_ref().unwrap().len(), 1);
        assert_eq!(result.path.as_ref().unwrap()[0], 5);
        assert_eq!(result.total_cost, 0.0);
        assert_eq!(result.nodes_explored, 1);
    }

    #[test]
    fn rejects_invalid_node_ids_and_weights() {
        assert!(AStarStorageRuntime::validate_node_id(-1, "source").is_err());
        assert!(AStarStorageRuntime::validate_node_id(0, "source").is_ok());
        assert!(AStarStorageRuntime::validate_edge_weight(0, 1, -1.0).is_err());
        assert!(AStarStorageRuntime::validate_edge_weight(0, 1, f64::NAN).is_err());
        assert!(AStarStorageRuntime::validate_edge_weight(0, 1, f64::INFINITY).is_err());
        assert!(AStarStorageRuntime::validate_edge_weight(0, 1, 0.0).is_ok());
    }

    #[test]
    fn rejects_non_finite_coordinates() {
        assert!(AStarStorageRuntime::validate_coordinates(0, (f64::NAN, 0.0)).is_err());
        assert!(AStarStorageRuntime::validate_coordinates(0, (0.0, f64::INFINITY)).is_err());
        assert!(AStarStorageRuntime::validate_coordinates(0, (0.0, 0.0)).is_ok());
    }
}
