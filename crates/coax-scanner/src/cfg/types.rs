//! CFG Data Structures for Vulnerability Slicing
//!
//! This module defines the core data structures for control flow graph
//! representation and vulnerability path analysis.

use petgraph::graph::{DiGraph, NodeIndex};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Control Flow Graph representing program execution paths
#[derive(Debug, Clone)]
pub struct CFG {
    pub graph: DiGraph<BasicBlock, EdgeLabel>,
    pub entry: NodeIndex,
    pub exit: NodeIndex,
}

/// Basic block in CFG - a sequence of statements with single entry/exit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasicBlock {
    pub id: usize,
    pub statements: Vec<Statement>,
    pub kind: BlockKind,
    pub line_start: u32,
    pub line_end: u32,
    /// Variables defined in this block
    pub variables_defined: HashSet<String>,
    /// Variables used in this block
    pub variables_used: HashSet<String>,
}

impl BasicBlock {
    pub fn new(id: usize, kind: BlockKind) -> Self {
        Self {
            id,
            statements: Vec::new(),
            kind,
            line_start: 0,
            line_end: 0,
            variables_defined: HashSet::new(),
            variables_used: HashSet::new(),
        }
    }

    pub fn add_statement(&mut self, stmt: Statement) {
        if self.line_start == 0 || stmt.line < self.line_start {
            self.line_start = stmt.line;
        }
        if stmt.line > self.line_end {
            self.line_end = stmt.line;
        }
        self.variables_defined
            .extend(stmt.variables_defined.clone());
        self.variables_used.extend(stmt.variables_used.clone());
        self.statements.push(stmt);
    }
}

/// Kind of basic block
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BlockKind {
    Entry,
    Exit,
    Normal,
    Branch,
    Loop,
    Return,
}

/// Statement in a basic block
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Statement {
    pub text: String,
    pub kind: StatementKind,
    pub variables_defined: Vec<String>,
    pub variables_used: Vec<String>,
    pub line: u32,
}

/// Kind of statement
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StatementKind {
    Assignment,
    Call,
    Return,
    Branch,
    Loop,
    Declaration,
    Expression,
}

/// Edge label in CFG - describes the control flow relationship
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EdgeLabel {
    Unconditional,
    TrueBranch,
    FalseBranch,
    LoopBack,
    LoopExit,
    Return,
}

/// Entry point types - where external input enters the program
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EntryPoint {
    HttpRoute {
        method: String,
        path: String,
        function: String,
        framework: String,
    },
    CliCommand {
        name: String,
        function: String,
    },
    PublicFunction {
        name: String,
        file: String,
    },
    EventHandler {
        event: String,
        handler: String,
    },
}

impl EntryPoint {
    pub fn name(&self) -> String {
        match self {
            EntryPoint::HttpRoute { method, path, .. } => format!("{} {}", method, path),
            EntryPoint::CliCommand { name, .. } => format!("CLI: {}", name),
            EntryPoint::PublicFunction { name, .. } => format!("fn {}", name),
            EntryPoint::EventHandler { event, .. } => format!("on {}", event),
        }
    }

    pub fn function_name(&self) -> String {
        match self {
            EntryPoint::HttpRoute { function, .. } => function.clone(),
            EntryPoint::CliCommand { function, .. } => function.clone(),
            EntryPoint::PublicFunction { name, .. } => name.clone(),
            EntryPoint::EventHandler { handler, .. } => handler.clone(),
        }
    }
}

/// Sink point types - where vulnerabilities manifest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SinkPoint {
    SqlExecution {
        query: String,
        method: String,
    },
    CommandExecution {
        command: String,
        method: String,
    },
    FileOperation {
        path: String,
        mode: String,
        method: String,
    },
    NetworkRequest {
        url: String,
        method: String,
    },
    SecretUsage {
        secret_type: String,
        operation: String,
    },
    Deserialization {
        source: String,
        method: String,
    },
    PathTraversal {
        path: String,
        method: String,
    },
}

impl SinkPoint {
    pub fn category(&self) -> &'static str {
        match self {
            SinkPoint::SqlExecution { .. } => "SQL Injection",
            SinkPoint::CommandExecution { .. } => "Command Injection",
            SinkPoint::FileOperation { .. } => "File I/O",
            SinkPoint::NetworkRequest { .. } => "SSRF",
            SinkPoint::SecretUsage { .. } => "Secret Exposure",
            SinkPoint::Deserialization { .. } => "Deserialization",
            SinkPoint::PathTraversal { .. } => "Path Traversal",
        }
    }

    pub fn method(&self) -> String {
        match self {
            SinkPoint::SqlExecution { method, .. } => method.clone(),
            SinkPoint::CommandExecution { method, .. } => method.clone(),
            SinkPoint::FileOperation { method, .. } => method.clone(),
            SinkPoint::NetworkRequest { method, .. } => method.clone(),
            SinkPoint::SecretUsage { operation, .. } => operation.clone(),
            SinkPoint::Deserialization { method, .. } => method.clone(),
            SinkPoint::PathTraversal { method, .. } => method.clone(),
        }
    }
}

/// Vulnerability slice - the path from entry to sink
#[derive(Debug, Clone)]
pub struct VulnerabilitySlice {
    pub entry_point: EntryPoint,
    pub sink_point: SinkPoint,
    pub nodes: Vec<NodeIndex>,
    pub variables: HashSet<String>,
    pub line_numbers: Vec<u32>,
    pub confidence: f32,
}

impl VulnerabilitySlice {
    pub fn new(entry: EntryPoint, sink: SinkPoint) -> Self {
        Self {
            entry_point: entry,
            sink_point: sink,
            nodes: Vec::new(),
            variables: HashSet::new(),
            line_numbers: Vec::new(),
            confidence: 0.0,
        }
    }

    pub fn add_node(&mut self, node: NodeIndex, block: &BasicBlock) {
        self.nodes.push(node);
        self.variables.extend(block.variables_used.iter().cloned());
        self.variables
            .extend(block.variables_defined.iter().cloned());
        if block.line_start > 0 {
            self.line_numbers.push(block.line_start);
        }
        if block.line_end > 0 && block.line_end != block.line_start {
            self.line_numbers.push(block.line_end);
        }
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    pub fn calculate_confidence(&mut self) -> f32 {
        // Confidence based on:
        // - Path length (shorter = more confident)
        // - Variable tracking (more tracked = more confident)
        // - Direct data flow (direct = more confident)
        let path_factor = if self.nodes.len() <= 3 {
            1.0
        } else if self.nodes.len() <= 6 {
            0.8
        } else {
            0.6
        };

        let var_factor = if self.variables.len() >= 3 {
            1.0
        } else if self.variables.len() >= 1 {
            0.8
        } else {
            0.6
        };

        self.confidence = path_factor * var_factor;
        self.confidence
    }
}

/// CFG Finding - enhanced finding with vulnerability path
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CfgFinding {
    pub file: String,
    pub line: u32,
    pub pattern: String,
    pub severity: String,
    pub recommendation: String,
    pub entry_point: EntryPoint,
    pub sink_point: SinkPoint,
    pub vulnerability_path: Vec<u32>,
    pub confidence: f32,
}
