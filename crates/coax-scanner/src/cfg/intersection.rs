//! Slice Intersection
//!
//! Intersects backward and forward slices to find vulnerability paths
//! from entry points to sink points.

use petgraph::graph::NodeIndex;
use std::collections::HashSet;

use super::backward::BackwardSlicer;
use super::forward::ForwardSlicer;
use super::types::{CfgFinding, EntryPoint, SinkPoint, VulnerabilitySlice, CFG};

/// Slice Intersection - finds vulnerability paths by intersecting slices
pub struct SliceIntersection;

impl SliceIntersection {
    /// Intersect backward and forward slices
    pub fn intersect(
        backward_slice: &VulnerabilitySlice,
        forward_slice: &VulnerabilitySlice,
    ) -> VulnerabilitySlice {
        // Find common nodes between slices
        let backward_nodes: HashSet<NodeIndex> = backward_slice.nodes.iter().cloned().collect();
        let forward_nodes: HashSet<NodeIndex> = forward_slice.nodes.iter().cloned().collect();

        let common_nodes: Vec<NodeIndex> = backward_nodes
            .intersection(&forward_nodes)
            .cloned()
            .collect();

        // Build vulnerability slice from common nodes
        let mut result = VulnerabilitySlice::new(
            forward_slice.entry_point.clone(),
            backward_slice.sink_point.clone(),
        );

        // Add common nodes (in order from forward slice)
        for node in &forward_slice.nodes {
            if common_nodes.contains(node) {
                // We need the block to add the node properly
                // For now, just add the node index
                result.nodes.push(*node);
            }
        }

        // Merge variables
        result
            .variables
            .extend(backward_slice.variables.iter().cloned());
        result
            .variables
            .extend(forward_slice.variables.iter().cloned());

        // Merge line numbers
        result.line_numbers.extend(&backward_slice.line_numbers);
        result.line_numbers.extend(&forward_slice.line_numbers);
        result.line_numbers.sort();
        result.line_numbers.dedup();

        // Calculate confidence
        result.calculate_confidence();

        result
    }

    /// Find all vulnerability paths in a CFG
    pub fn find_vulnerability_paths(
        cfg: &CFG,
        entry_points: &[EntryPoint],
        sink_points: &[SinkPoint],
    ) -> Vec<VulnerabilitySlice> {
        let mut paths = Vec::new();

        // Map entry points to nodes and sink points to nodes
        // For now, we'll use a simple heuristic: first node is entry, last is sink
        // In a real implementation, we'd map based on function names, etc.

        let entry_nodes: Vec<NodeIndex> = entry_points
            .iter()
            .enumerate()
            .map(|(i, _)| {
                // Get node by index (simplified - real impl would map by name)
                cfg.graph
                    .node_indices()
                    .nth(i % cfg.graph.node_count())
                    .unwrap_or(cfg.entry)
            })
            .collect();

        let sink_nodes: Vec<NodeIndex> = sink_points
            .iter()
            .enumerate()
            .map(|(i, _)| {
                // Get node by index (simplified)
                cfg.graph
                    .node_indices()
                    .nth((i + cfg.graph.node_count() - 1) % cfg.graph.node_count())
                    .unwrap_or(cfg.exit)
            })
            .collect();

        // For each entry-sink pair, find vulnerability path
        for (entry_idx, entry) in entry_points.iter().enumerate() {
            for (sink_idx, sink) in sink_points.iter().enumerate() {
                let entry_node = entry_nodes.get(entry_idx).copied().unwrap_or(cfg.entry);
                let sink_node = sink_nodes.get(sink_idx).copied().unwrap_or(cfg.exit);

                // Forward slice from entry
                let forward_slicer = ForwardSlicer::new(cfg);
                let forward_slice = forward_slicer.slice_from_entry(entry_node, entry.clone());

                // Backward slice from sink
                let backward_slicer = BackwardSlicer::new(cfg);
                let backward_slice = backward_slicer.slice_from_sink(sink_node, sink.clone());

                // Intersect slices
                let intersection = Self::intersect(&backward_slice, &forward_slice);

                // Only add if there's a valid path
                if !intersection.is_empty() && intersection.nodes.len() >= 2 {
                    paths.push(intersection);
                }
            }
        }

        paths
    }

    /// Find vulnerability paths with node mapping
    pub fn find_paths_with_nodes(
        cfg: &CFG,
        entry_sink_pairs: &[(EntryPoint, SinkPoint, NodeIndex, NodeIndex)],
    ) -> Vec<VulnerabilitySlice> {
        let mut paths = Vec::new();

        for (entry, sink, entry_node, sink_node) in entry_sink_pairs {
            // Forward slice from entry
            let forward_slicer = ForwardSlicer::new(cfg);
            let forward_slice = forward_slicer.slice_from_entry(*entry_node, entry.clone());

            // Backward slice from sink
            let backward_slicer = BackwardSlicer::new(cfg);
            let backward_slice = backward_slicer.slice_from_sink(*sink_node, sink.clone());

            // Intersect slices
            let intersection = Self::intersect(&backward_slice, &forward_slice);

            if !intersection.is_empty() {
                paths.push(intersection);
            }
        }

        paths
    }

    /// Filter paths by minimum confidence
    pub fn filter_by_confidence(
        paths: Vec<VulnerabilitySlice>,
        min_confidence: f32,
    ) -> Vec<VulnerabilitySlice> {
        paths
            .into_iter()
            .filter(|p| p.confidence >= min_confidence)
            .collect()
    }

    /// Sort paths by confidence (highest first)
    pub fn sort_by_confidence(paths: &mut Vec<VulnerabilitySlice>) {
        paths.sort_by(|a, b| {
            b.confidence
                .partial_cmp(&a.confidence)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
    }

    /// Convert vulnerability slices to CFG findings
    pub fn to_findings(paths: Vec<VulnerabilitySlice>, file_path: &str) -> Vec<CfgFinding> {
        paths
            .into_iter()
            .map(|path| {
                let line = path.line_numbers.first().copied().unwrap_or(1);

                CfgFinding {
                    file: file_path.to_string(),
                    line,
                    pattern: path.sink_point.category().to_string(),
                    severity: Self::severity_from_sink(&path.sink_point),
                    recommendation: Self::recommendation_from_sink(&path.sink_point),
                    entry_point: path.entry_point,
                    sink_point: path.sink_point,
                    vulnerability_path: path.line_numbers.clone(),
                    confidence: path.confidence,
                }
            })
            .collect()
    }

    /// Determine severity from sink point
    fn severity_from_sink(sink: &SinkPoint) -> String {
        match sink {
            SinkPoint::SqlExecution { .. } => "critical".to_string(),
            SinkPoint::CommandExecution { .. } => "critical".to_string(),
            SinkPoint::FileOperation { mode, .. } => {
                if mode == "write" || mode == "append" {
                    "high".to_string()
                } else {
                    "medium".to_string()
                }
            }
            SinkPoint::NetworkRequest { .. } => "high".to_string(),
            SinkPoint::SecretUsage { secret_type, .. } => {
                if secret_type == "password" || secret_type == "private_key" {
                    "critical".to_string()
                } else {
                    "high".to_string()
                }
            }
            SinkPoint::Deserialization { .. } => "critical".to_string(),
            SinkPoint::PathTraversal { .. } => "high".to_string(),
        }
    }

    /// Generate recommendation from sink point
    fn recommendation_from_sink(sink: &SinkPoint) -> String {
        match sink {
            SinkPoint::SqlExecution { .. } => {
                "Use parameterized queries or prepared statements to prevent SQL injection"
                    .to_string()
            }
            SinkPoint::CommandExecution { .. } => {
                "Avoid executing commands with user input. Use allowlists and validate input"
                    .to_string()
            }
            SinkPoint::FileOperation { .. } => {
                "Validate and sanitize file paths. Use path canonicalization".to_string()
            }
            SinkPoint::NetworkRequest { .. } => {
                "Validate URLs and use allowlists for external requests".to_string()
            }
            SinkPoint::SecretUsage { .. } => {
                "Use secure secret management. Never log or expose secrets".to_string()
            }
            SinkPoint::Deserialization { .. } => {
                "Use safe deserialization. Validate input and use allowlists for types".to_string()
            }
            SinkPoint::PathTraversal { .. } => {
                "Validate paths and use path canonicalization. Restrict to allowed directories"
                    .to_string()
            }
        }
    }
}

/// Find entry point nodes in CFG
pub fn find_entry_nodes(cfg: &CFG, entry_points: &[EntryPoint]) -> Vec<(EntryPoint, NodeIndex)> {
    let mut nodes = Vec::new();

    // For now, use entry node for all entry points
    // Real implementation would map by function name
    for entry in entry_points {
        nodes.push((entry.clone(), cfg.entry));
    }

    nodes
}

/// Find sink point nodes in CFG
pub fn find_sink_nodes(cfg: &CFG, sink_points: &[SinkPoint]) -> Vec<(SinkPoint, NodeIndex)> {
    let mut nodes = Vec::new();

    // For now, use exit node for all sink points
    // Real implementation would map by statement content
    for sink in sink_points {
        nodes.push((sink.clone(), cfg.exit));
    }

    nodes
}

/// Quick vulnerability detection
pub fn quick_detect(cfg: &CFG) -> Vec<VulnerabilitySlice> {
    use super::entry_points;
    use super::sinks;

    let entries = entry_points::detect_all(cfg);
    let sinks = sinks::detect_all(cfg);

    if entries.is_empty() || sinks.is_empty() {
        return Vec::new();
    }

    SliceIntersection::find_vulnerability_paths(cfg, &entries, &sinks)
}
