//! Forward Slicer
//!
//! Implements forward slicing from entry points to find all
//! statements that could be influenced by the entry.

use petgraph::graph::NodeIndex;
use std::collections::HashSet;

use super::types::{EntryPoint, SinkPoint, VulnerabilitySlice, CFG};

/// Forward Slicer - slices forward from entry to find data sinks
pub struct ForwardSlicer<'a> {
    cfg: &'a CFG,
}

impl<'a> ForwardSlicer<'a> {
    /// Create a new forward slicer
    pub fn new(cfg: &'a CFG) -> Self {
        Self { cfg }
    }

    /// Perform forward slice from an entry node
    pub fn slice(&self, entry_node: NodeIndex) -> VulnerabilitySlice {
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
        let mut worklist = vec![entry_node];
        let mut visited = HashSet::new();

        while let Some(node) = worklist.pop() {
            if visited.contains(&node) {
                continue;
            }
            visited.insert(node);

            // Add node to slice
            let block = &self.cfg.graph[node];
            slice.add_node(node, block);

            // Find successors (data flow forward)
            for succ in self
                .cfg
                .graph
                .neighbors_directed(node, petgraph::Direction::Outgoing)
            {
                if self.has_data_dependency(node, succ) {
                    if !visited.contains(&succ) {
                        worklist.push(succ);
                    }
                } else {
                    // Also follow control flow edges
                    if !visited.contains(&succ) {
                        worklist.push(succ);
                    }
                }
            }
        }

        slice.calculate_confidence();
        slice
    }

    /// Perform forward slice with explicit entry point
    pub fn slice_from_entry(
        &self,
        entry_node: NodeIndex,
        entry_point: EntryPoint,
    ) -> VulnerabilitySlice {
        let dummy_sink = SinkPoint::SqlExecution {
            query: "unknown".to_string(),
            method: "unknown".to_string(),
        };

        let mut slice = VulnerabilitySlice::new(entry_point, dummy_sink);
        let mut worklist = vec![entry_node];
        let mut visited = HashSet::new();

        while let Some(node) = worklist.pop() {
            if visited.contains(&node) {
                continue;
            }
            visited.insert(node);

            let block = &self.cfg.graph[node];
            slice.add_node(node, block);

            // Follow successors
            for succ in self
                .cfg
                .graph
                .neighbors_directed(node, petgraph::Direction::Outgoing)
            {
                if !visited.contains(&succ) {
                    worklist.push(succ);
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

    /// Get all variables that flow out of a node
    pub fn get_output_variables(&self, node: NodeIndex) -> HashSet<String> {
        let mut vars = HashSet::new();
        let block = &self.cfg.graph[node];

        // Variables defined in this block
        vars.extend(block.variables_defined.iter().cloned());

        // Trace forward to find uses
        for succ in self
            .cfg
            .graph
            .neighbors_directed(node, petgraph::Direction::Outgoing)
        {
            let succ_block = &self.cfg.graph[succ];
            vars.extend(succ_block.variables_used.iter().cloned());
        }

        vars
    }

    /// Get all nodes that use a specific variable
    pub fn find_uses(&self, var_name: &str) -> Vec<NodeIndex> {
        let mut uses = Vec::new();

        for node in self.cfg.graph.node_indices() {
            let block = &self.cfg.graph[node];
            if block.variables_used.contains(var_name) {
                uses.push(node);
            }
        }

        uses
    }

    /// Forward slice to multiple sink nodes
    pub fn slice_multiple(&self, entry_nodes: &[NodeIndex]) -> Vec<VulnerabilitySlice> {
        entry_nodes.iter().map(|&node| self.slice(node)).collect()
    }

    /// Forward slice with sink point constraint
    pub fn slice_to_sink(
        &self,
        entry_node: NodeIndex,
        sink_node: NodeIndex,
    ) -> Option<VulnerabilitySlice> {
        let slice = self.slice(entry_node);

        // Check if sink node is in the slice
        if slice.nodes.contains(&sink_node) {
            Some(slice)
        } else {
            None
        }
    }

    /// Find all reachable nodes from entry
    pub fn find_reachable(&self, entry_node: NodeIndex) -> HashSet<NodeIndex> {
        let mut reachable = HashSet::new();
        let mut worklist = vec![entry_node];

        while let Some(node) = worklist.pop() {
            if reachable.contains(&node) {
                continue;
            }
            reachable.insert(node);

            for succ in self
                .cfg
                .graph
                .neighbors_directed(node, petgraph::Direction::Outgoing)
            {
                if !reachable.contains(&succ) {
                    worklist.push(succ);
                }
            }
        }

        reachable
    }
}

/// Convenience function for forward slicing
pub fn forward_slice(cfg: &CFG, entry_node: NodeIndex) -> VulnerabilitySlice {
    let slicer = ForwardSlicer::new(cfg);
    slicer.slice(entry_node)
}

/// Forward slice with entry point information
pub fn forward_slice_from_entry(
    cfg: &CFG,
    entry_node: NodeIndex,
    entry_point: EntryPoint,
) -> VulnerabilitySlice {
    let slicer = ForwardSlicer::new(cfg);
    slicer.slice_from_entry(entry_node, entry_point)
}
