//! Backward Slicer
//!
//! Implements backward slicing from sink points to find all
//! statements that could influence the sink.

use petgraph::graph::NodeIndex;
use petgraph::visit::EdgeRef;
use std::collections::{HashMap, HashSet};

use super::types::{EntryPoint, SinkPoint, VulnerabilitySlice, CFG};

/// Backward Slicer - slices backward from sink to find data sources
pub struct BackwardSlicer<'a> {
    cfg: &'a CFG,
}

impl<'a> BackwardSlicer<'a> {
    /// Create a new backward slicer
    pub fn new(cfg: &'a CFG) -> Self {
        Self { cfg }
    }

    /// Perform backward slice from a sink node
    pub fn slice(&self, sink_node: NodeIndex) -> VulnerabilitySlice {
        // Create a dummy entry/sink for the slice
        let dummy_entry = EntryPoint::PublicFunction {
            name: "unknown".to_string(),
            file: "unknown".to_string(),
        };
        let dummy_sink = SinkPoint::SqlExecution {
            query: "unknown".to_string(),
            method: "unknown".to_string(),
        };

        let mut slice = VulnerabilitySlice::new(dummy_entry, dummy_sink);
        let mut worklist = vec![sink_node];
        let mut visited = HashSet::new();

        while let Some(node) = worklist.pop() {
            if visited.contains(&node) {
                continue;
            }
            visited.insert(node);

            // Add node to slice
            let block = &self.cfg.graph[node];
            slice.add_node(node, block);

            // Find predecessors (data dependencies)
            for pred in self
                .cfg
                .graph
                .neighbors_directed(node, petgraph::Direction::Incoming)
            {
                if self.has_data_dependency(pred, node) {
                    if !visited.contains(&pred) {
                        worklist.push(pred);
                    }
                } else {
                    // Also follow control flow edges
                    if !visited.contains(&pred) {
                        worklist.push(pred);
                    }
                }
            }

            // Find control dependencies
            if let Some(ctrl_dep) = self.find_control_dependency(node) {
                if !visited.contains(&ctrl_dep) {
                    worklist.push(ctrl_dep);
                }
            }
        }

        slice.calculate_confidence();
        slice
    }

    /// Perform backward slice with explicit sink point
    pub fn slice_from_sink(
        &self,
        sink_node: NodeIndex,
        sink_point: SinkPoint,
    ) -> VulnerabilitySlice {
        let dummy_entry = EntryPoint::PublicFunction {
            name: "unknown".to_string(),
            file: "unknown".to_string(),
        };

        let mut slice = VulnerabilitySlice::new(dummy_entry, sink_point);
        let mut worklist = vec![sink_node];
        let mut visited = HashSet::new();

        while let Some(node) = worklist.pop() {
            if visited.contains(&node) {
                continue;
            }
            visited.insert(node);

            let block = &self.cfg.graph[node];
            slice.add_node(node, block);

            // Follow predecessors
            for pred in self
                .cfg
                .graph
                .neighbors_directed(node, petgraph::Direction::Incoming)
            {
                if !visited.contains(&pred) {
                    worklist.push(pred);
                }
            }
        }

        slice.calculate_confidence();
        slice
    }

    /// Check if there's a data dependency between two nodes
    fn has_data_dependency(&self, from: NodeIndex, to: NodeIndex) -> bool {
        let from_block = &self.cfg.graph[from];
        let to_block = &self.cfg.graph[to];

        // Check if 'from' defines variables used by 'to'
        for def in &from_block.variables_defined {
            if to_block.variables_used.contains(def) {
                return true;
            }
        }

        // Check statement-level dependencies
        for from_stmt in &from_block.statements {
            for to_stmt in &to_block.statements {
                for def in &from_stmt.variables_defined {
                    if to_stmt.variables_used.contains(def) {
                        return true;
                    }
                }
            }
        }

        false
    }

    /// Find control dependency for a node
    fn find_control_dependency(&self, node: NodeIndex) -> Option<NodeIndex> {
        // Look for a branch/conditional that controls this node
        // by finding a predecessor that is a branch block

        for pred in self
            .cfg
            .graph
            .neighbors_directed(node, petgraph::Direction::Incoming)
        {
            let pred_block = &self.cfg.graph[pred];
            if pred_block.kind == super::types::BlockKind::Branch {
                return Some(pred);
            }
        }

        None
    }

    /// Get all variables that flow into a node
    pub fn get_input_variables(&self, node: NodeIndex) -> HashSet<String> {
        let mut vars = HashSet::new();
        let block = &self.cfg.graph[node];

        // Variables used in this block
        vars.extend(block.variables_used.iter().cloned());

        // Trace back to find definitions
        for pred in self
            .cfg
            .graph
            .neighbors_directed(node, petgraph::Direction::Incoming)
        {
            let pred_block = &self.cfg.graph[pred];
            vars.extend(pred_block.variables_defined.iter().cloned());
        }

        vars
    }

    /// Get all nodes that define a specific variable
    pub fn find_definitions(&self, var_name: &str) -> Vec<NodeIndex> {
        let mut defs = Vec::new();

        for node in self.cfg.graph.node_indices() {
            let block = &self.cfg.graph[node];
            if block.variables_defined.contains(var_name) {
                defs.push(node);
            }
        }

        defs
    }

    /// Backward slice from multiple sink nodes
    pub fn slice_multiple(&self, sink_nodes: &[NodeIndex]) -> Vec<VulnerabilitySlice> {
        sink_nodes.iter().map(|&node| self.slice(node)).collect()
    }

    /// Backward slice with entry point constraint
    pub fn slice_to_entry(
        &self,
        sink_node: NodeIndex,
        entry_node: NodeIndex,
    ) -> Option<VulnerabilitySlice> {
        let slice = self.slice(sink_node);

        // Check if entry node is in the slice
        if slice.nodes.contains(&entry_node) {
            Some(slice)
        } else {
            None
        }
    }
}

/// Convenience function for backward slicing
pub fn backward_slice(cfg: &CFG, sink_node: NodeIndex) -> VulnerabilitySlice {
    let slicer = BackwardSlicer::new(cfg);
    slicer.slice(sink_node)
}

/// Backward slice with sink point information
pub fn backward_slice_from_sink(
    cfg: &CFG,
    sink_node: NodeIndex,
    sink_point: SinkPoint,
) -> VulnerabilitySlice {
    let slicer = BackwardSlicer::new(cfg);
    slicer.slice_from_sink(sink_node, sink_point)
}
