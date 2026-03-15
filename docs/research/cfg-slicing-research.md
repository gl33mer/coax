# CFG-Based Vulnerability Slicing Research

**Date:** 2026-03-15
**Author:** Coax Research Team
**Status:** Complete

---

## Executive Summary

This document researches Control Flow Graph (CFG) based vulnerability slicing for integration into Coax's Phase 3 P1 features. CFG-based slicing enables precise identification of vulnerability paths by analyzing data flow from sources to sinks, significantly improving detection accuracy over regex-only approaches.

**Key Finding:** CFG-based slicing combined with LLM analysis (VulnLLM-R-7B) provides project-level vulnerability detection with superior accuracy compared to pattern-only scanning.

---

## Background: Why CFG-Based Slicing?

### Limitations of Regex-Only Detection

| Issue | Impact | Example |
|-------|--------|---------|
| **No context awareness** | High false positives | `AWS_KEY = "placeholder"` flagged |
| **No data flow tracking** | Misses obfuscated secrets | `key = decode(encoded)` not detected |
| **No cross-file analysis** | Misses multi-file vulns | Taint flows across modules |
| **No semantic understanding** | Can't distinguish intent | `test_key` vs `prod_key` |

### Benefits of CFG-Based Slicing

| Benefit | Impact | Example |
|---------|--------|---------|
| **Context awareness** | Reduced FPs | Ignores placeholders, tests |
| **Data flow tracking** | Detects obfuscation | Follows `decode()` chains |
| **Cross-file analysis** | Project-level detection | Tracks taint across modules |
| **Semantic understanding** | Intent detection | Distinguishes test/prod |

---

## Existing Implementations Analysis

### Semgrep

**Website:** https://semgrep.dev/
**Language:** OCaml (77%), Python (19%)
**License:** LGPL-2.1

#### CFG Analysis Approach

| Aspect | Details |
|--------|---------|
| **Analysis Type** | AST + CFG hybrid |
| **Data Flow** | Taint tracking via data flow graphs |
| **Cross-File** | Pro version only (AppSec Platform) |
| **Reachability** | Call graph + CFG traversal |

**Semgrep Rule Example (Taint Tracking):**
```yaml
rules:
  - id: sql-injection
    pattern-either:
      - pattern: db.execute($USER_INPUT)
    sources:
      - pattern: request.args.get(...)
    sinks:
      - pattern: db.execute(...)
    severity: ERROR
```

**Key Features:**
- Pattern matching with metavariables (`$USER_INPUT`)
- Source-to-sink taint tracking
- Interprocedural analysis (Pro version)
- 2,000+ community rules

**Limitations for Coax:**
- OCaml core (not Rust)
- Cross-file analysis requires Pro subscription
- Complex rule DSL

---

### CodeQL

**Website:** https://codeql.github.com/
**Language:** C++, Java, Python, etc.
**License:** Proprietary (GitHub Advanced Security)

#### Data Flow Tracking Approach

| Aspect | Details |
|--------|---------|
| **Analysis Type** | AST + CFG + Data Flow Graph |
| **Data Flow** | Taint tracking with path exploration |
| **Cross-File** | Full project-level analysis |
| **Reachability** | Interprocedural data flow |

**CodeQL Query Example (SQL Injection):**
```ql
from Source source, Sink sink, DataFlow::Path path
where path.hasSource(source) and path.hasSink(sink)
select path, "SQL injection from " + source + " to " + sink
```

**Key Features:**
- Precise interprocedural data flow
- Path exploration with constraints
- Extensive standard libraries
- GitHub Advanced Security integration

**Limitations for Coax:**
- Proprietary (not open source)
- Complex query language
- Heavy resource requirements

---

### Joern

**Website:** https://joern.io/
**Language:** Scala
**License:** Apache-2.0

#### Code Property Graph (CPG) Approach

| Aspect | Details |
|--------|---------|
| **Analysis Type** | Code Property Graph (CPG) |
| **Graph Types** | AST + CFG + PDG combined |
| **Data Flow** | Graph traversal via Scala/Python API |
| **Cross-File** | Full project-level CPG |

**Joern Query Example (Use-After-Free):**
```scala
cpg.call("free").argument
  .where(_.defUseChain.exists(_.isCall("malloc")))
  .where(_.callInMethod("process_data"))
  .l
```

**Key Features:**
- Unified CPG representation
- Query via Scala or Python API
- Open source (Apache-2.0)
- Supports C/C++, Java, Python, JavaScript

**CPG Structure:**
```
┌─────────────────────────────────────────┐
│           Code Property Graph           │
├─────────────────────────────────────────┤
│  ┌─────┐    ┌─────┐    ┌─────┐         │
│  │ AST │ +  │ CFG │ +  │ PDG │         │
│  └─────┘    └─────┘    └─────┘         │
│     │          │          │             │
│     └──────────┼──────────┘             │
│                ▼                        │
│        Unified Graph DB                 │
│        (Overflow/Neo4j)                 │
└─────────────────────────────────────────┘
```

**Advantages for Coax:**
- Open source (Apache-2.0 compatible)
- Graph-based approach aligns with slicing
- Active community
- Good documentation

**Limitations:**
- Scala-based (FFI required for Rust)
- Heavy memory usage (graph database)
- Complex setup

---

### Comparison Summary

| Tool | Language | License | CFG Approach | Cross-File | Open Source | Rust Integration |
|------|----------|---------|--------------|------------|-------------|------------------|
| **Semgrep** | OCaml | LGPL-2.1 | AST+CFG | Pro only | Partial | ❌ Complex |
| **CodeQL** | C++ | Proprietary | AST+CFG+DFG | ✅ Full | ❌ No | ❌ No |
| **Joern** | Scala | Apache-2.0 | CPG (AST+CFG+PDG) | ✅ Full | ✅ Yes | ⚠️ FFI |
| **oxc_cfg** | Rust | MIT | CFG from AST | ⚠️ Manual | ✅ Yes | ✅ Native |

**Recommendation:** Use `oxc_cfg` (Rust native) + custom slicing logic

---

## Key Concepts

### Control Flow Graph (CFG)

**Definition:** A directed graph where nodes represent basic blocks (sequences of instructions) and edges represent control flow paths.

**CFG Components:**
```
┌─────────────────┐
│  Entry Node   │ ← Function entry point
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  Basic Block 1 │ ← Sequential statements
│  x = 1         │
│  y = 2         │
└────────┬────────┘
         │
    ┌────┴────┐
    │  if (x) │ ← Conditional branch
    └────┬────┘
         │
    ┌────┴────┐
    ▼         ▼
┌─────────┐ ┌─────────┐
│ Block 2 │ │ Block 3 │ ← Branch targets
└────┬────┘ └────┬────┘
     │           │
     └─────┬─────┘
           ▼
    ┌─────────────┐
    │  Exit Node  │ ← Function exit
    └─────────────┘
```

---

### Entry Points

**Definition:** Locations where external input enters the program.

**Common Entry Points:**

| Type | Examples | Detection Pattern |
|------|----------|-------------------|
| **HTTP Routes** | `@app.route()`, `router.get()` | Route decorator/handler |
| **CLI Commands** | `argparse`, `clap` | Command parser |
| **Public Functions** | `pub fn`, `export` | Visibility modifier |
| **API Endpoints** | `@api.get`, `@RestController` | API framework annotation |
| **Event Handlers** | `onClick`, `onMessage` | Event registration |
| **File I/O** | `read()`, `load()` | External data load |

**Entry Point Detection (Rust/Tree-sitter):**
```rust
// HTTP route detection
#[axum::routing::get("/users")]
async fn get_users() {}  // ← Entry point

// CLI command detection
#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    input: String,  // ← Entry point
}

// Public function detection
pub fn process_input(data: &str) {}  // ← Entry point
```

---

### Sink Points

**Definition:** Locations where sensitive operations occur (potential vulnerability manifestation).

**Common Sink Points:**

| Category | Sinks | Risk |
|----------|-------|------|
| **SQL Execution** | `execute()`, `query()`, `raw_sql()` | SQL Injection |
| **Command Execution** | `system()`, `exec()`, `spawn()` | Command Injection |
| **File I/O** | `open()`, `read_file()`, `write_file()` | Path Traversal |
| **Network** | `send()`, `request()`, `fetch()` | SSRF |
| **Crypto** | `encrypt()`, `decrypt()`, `hash()` | Crypto Misuse |
| **Serialization** | `eval()`, `deserialize()`, `unpickle()` | Deserialization |
| **Memory** | `malloc()`, `free()`, `memcpy()` | Memory Corruption |

**Sink Point Detection (Rust):**
```rust
// SQL sink
db.execute("SELECT * FROM users WHERE id = ?", &[user_id]);  // ← Sink

// Command execution sink
std::process::Command::new("ls").arg(path).spawn();  // ← Sink

// File I/O sink
std::fs::read_to_string(&user_path);  // ← Sink

// Network sink
reqwest::get(&user_url).await?;  // ← Sink
```

---

### Backward Slicing

**Definition:** Starting from a sink, trace backward through the CFG to find all statements that could affect the sink's inputs.

**Algorithm:**
```
function backward_slice(sink):
    slice = {sink}
    worklist = [sink]
    
    while worklist not empty:
        node = worklist.pop()
        for pred in predecessors(node):
            if pred affects node's inputs:
                slice.add(pred)
                worklist.push(pred)
    
    return slice
```

**Example:**
```rust
// Sink: SQL execution at line 10
// Backward slice:
fn handle_request(req: Request) {           // ← Line 1 (entry)
    let user_id = req.param("id");          // ← Line 2 (taint source)
    let query = format!(                    // ← Line 3 (string concat)
        "SELECT * FROM users WHERE id = {}",
        user_id                              // ← Line 4 (taint flow)
    );
    db.execute(&query);                     // ← Line 5 (sink)
}

// Backward slice from Line 5: {5, 4, 3, 2, 1}
// Vulnerability path: user input → string concat → SQL execution
```

---

### Forward Slicing

**Definition:** Starting from a source (entry point), trace forward through the CFG to find all statements affected by the source's outputs.

**Algorithm:**
```
function forward_slice(source):
    slice = {source}
    worklist = [source]
    
    while worklist not empty:
        node = worklist.pop()
        for succ in successors(node):
            if node's outputs affect succ:
                slice.add(succ)
                worklist.push(succ)
    
    return slice
```

**Example:**
```rust
// Source: User input at line 2
// Forward slice:
fn handle_request(req: Request) {           // ← Line 1
    let user_input = req.param("data");     // ← Line 2 (source)
    let sanitized = sanitize(user_input);   // ← Line 3 (sanitization)
    let query = format!(                    // ← Line 4 (string concat)
        "INSERT INTO logs VALUES ('{}')",
        sanitized                            // ← Line 5 (taint flow)
    );
    db.execute(&query);                     // ← Line 6 (sink)
}

// Forward slice from Line 2: {2, 3, 4, 5, 6}
// Vulnerability path: user input → sanitization → string concat → SQL execution
```

---

### Slice Intersection

**Definition:** Intersect backward slice (from sink) and forward slice (from source) to identify the vulnerability path.

**Algorithm:**
```
function vulnerability_slice(source, sink):
    forward = forward_slice(source)
    backward = backward_slice(sink)
    return forward ∩ backward
```

**Example:**
```
Forward slice (from source):  {2, 3, 4, 5, 6}
Backward slice (from sink):   {1, 2, 3, 4, 5, 6}
Intersection:                 {2, 3, 4, 5, 6}

// Vulnerability path identified:
// Source (2) → Sanitization (3) → Concat (4) → Flow (5) → Sink (6)
```

---

## Implementation Approach for Coax

### Architecture

```
┌─────────────────────────────────────────────────────────┐
│                    Coax Scanner                         │
│                  (Phase 3 P1 + CFG)                     │
└─────────────────────────────────────────────────────────┘
                            │
        ┌───────────────────┼───────────────────┐
        │                   │                   │
        ▼                   ▼                   ▼
┌───────────────┐  ┌────────────────┐  ┌───────────────┐
│  Pattern Scan │  │  CFG Builder   │  │  Slice Engine │
│  (regex/      │  │  (tree-sitter  │  │  (backward/   │
│   entropy)    │  │   + oxc_cfg)   │  │   forward)    │
└───────────────┘  └────────────────┘  └───────────────┘
                            │                   │
                            └─────────┬─────────┘
                                      ▼
                              ┌───────────────┐
                              │  LLM Analysis │
                              │  (VulnLLM-R)  │
                              └───────────────┘
```

---

### Step 1: AST Parsing with Tree-sitter

**Tree-sitter** is a parser generator tool and incremental parsing library.

**Installation:**
```toml
# Cargo.toml
[dependencies]
tree-sitter = "0.20"
tree-sitter-rust = "0.20"
tree-sitter-python = "0.20"
tree-sitter-javascript = "0.20"
```

**Basic Usage:**
```rust
use tree_sitter::{Parser, Language};

fn parse_file(content: &str, language: Language) -> tree_sitter::Tree {
    let mut parser = Parser::new();
    parser.set_language(language).unwrap();
    parser.parse(content, None).unwrap()
}

fn extract_functions(tree: &tree_sitter::Tree) -> Vec<FunctionInfo> {
    let root = tree.root_node();
    let mut functions = Vec::new();
    
    // Traverse AST to find function definitions
    traverse(&root, &mut |node| {
        if node.kind() == "function_definition" {
            functions.push(FunctionInfo::from(node));
        }
    });
    
    functions
}
```

---

### Step 2: CFG Construction with oxc_cfg

**oxc_cfg** is a Rust crate for building and analyzing Control Flow Graphs.

**Repository:** https://github.com/oxc-project/oxc/tree/main/crates/oxc_cfg
**License:** MIT

**Installation:**
```toml
# Cargo.toml
[dependencies]
oxc_cfg = "0.0"
oxc_ast = "0.0"  # AST types
```

**Basic Usage:**
```rust
use oxc_cfg::{CfgBuilder, CfgContext};
use oxc_ast::ast::Program;

fn build_cfg(program: &Program) -> CfgGraph {
    let ctx = CfgContext::default();
    CfgBuilder::build(program, ctx)
}

// CfgGraph provides:
// - Basic blocks
// - Control flow edges
// - Entry/exit nodes
// - Dominator tree
```

**oxc_cfg API:**
```rust
pub struct CfgGraph {
    pub basic_blocks: Vec<BasicBlock>,
    pub edges: Vec<Edge>,
    pub entry: NodeId,
    pub exit: NodeId,
}

impl CfgGraph {
    pub fn predecessors(&self, node: NodeId) -> Vec<NodeId>;
    pub fn successors(&self, node: NodeId) -> Vec<NodeId>;
    pub fn dominators(&self, node: NodeId) -> Vec<NodeId>;
    pub fn back_edges(&self) -> Vec<(NodeId, NodeId)>;
}
```

---

### Step 3: Entry Point Detection

**Implementation:**
```rust
use tree_sitter::{Tree, Node};

pub struct EntryPoint {
    pub name: String,
    pub location: SourceLocation,
    pub entry_type: EntryPointType,
}

pub enum EntryPointType {
    HttpRoute,
    CliCommand,
    PublicFunction,
    ApiEndpoint,
    EventHandler,
}

pub fn detect_entry_points(tree: &Tree, source: &str) -> Vec<EntryPoint> {
    let mut entry_points = Vec::new();
    let root = tree.root_node();
    
    // Detect HTTP routes (Axum example)
    find_http_routes(&root, source, &mut entry_points);
    
    // Detect CLI commands
    find_cli_commands(&root, source, &mut entry_points);
    
    // Detect public functions
    find_public_functions(&root, source, &mut entry_points);
    
    entry_points
}

fn find_http_routes(node: &Node, source: &str, results: &mut Vec<EntryPoint>) {
    // Look for #[route(...)] attributes
    if node.kind() == "attribute_item" {
        let content = node.utf8_text(source.as_bytes()).unwrap();
        if content.contains("route") || content.contains("get") || content.contains("post") {
            // Found route attribute, next function is entry point
            if let Some(parent) = node.parent() {
                if let Some(fn_node) = parent.child_by_field_name("name") {
                    results.push(EntryPoint {
                        name: fn_node.utf8_text(source.as_bytes()).unwrap().to_string(),
                        location: SourceLocation::from(fn_node),
                        entry_type: EntryPointType::HttpRoute,
                    });
                }
            }
        }
    }
    
    // Recurse into children
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        find_http_routes(&child, source, results);
    }
}
```

---

### Step 4: Sink Point Detection

**Implementation:**
```rust
pub struct SinkPoint {
    pub name: String,
    pub location: SourceLocation,
    pub sink_type: SinkType,
    pub risk_level: RiskLevel,
}

pub enum SinkType {
    SqlExecution,
    CommandExecution,
    FileIo,
    Network,
    Crypto,
    Serialization,
    Memory,
}

pub enum RiskLevel {
    Critical,  // SQL injection, command injection
    High,      // File I/O, network
    Medium,    // Crypto, serialization
    Low,       // Memory (in safe languages)
}

pub fn detect_sink_points(tree: &Tree, source: &str) -> Vec<SinkPoint> {
    let mut sinks = Vec::new();
    let root = tree.root_node();
    
    // SQL sinks
    find_sql_sinks(&root, source, &mut sinks);
    
    // Command execution sinks
    find_command_sinks(&root, source, &mut sinks);
    
    // File I/O sinks
    find_file_sinks(&root, source, &mut sinks);
    
    // Network sinks
    find_network_sinks(&root, source, &mut sinks);
    
    sinks
}

fn find_sql_sinks(node: &Node, source: &str, results: &mut Vec<SinkPoint>) {
    if node.kind() == "call_expression" {
        if let Some(func_node) = node.child_by_field_name("function") {
            let func_name = func_node.utf8_text(source.as_bytes()).unwrap();
            
            // SQL execution patterns
            if matches!(func_name, "execute" | "query" | "raw_sql" | "execute_query") {
                results.push(SinkPoint {
                    name: func_name.to_string(),
                    location: SourceLocation::from(node),
                    sink_type: SinkType::SqlExecution,
                    risk_level: RiskLevel::Critical,
                });
            }
        }
    }
    
    // Recurse
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        find_sql_sinks(&child, source, results);
    }
}
```

---

### Step 5: Backward Slicing Implementation

**Implementation:**
```rust
use std::collections::{HashSet, VecDeque};
use oxc_cfg::{CfgGraph, NodeId};

pub struct SliceEngine {
    cfg: CfgGraph,
}

impl SliceEngine {
    pub fn new(cfg: CfgGraph) -> Self {
        Self { cfg }
    }
    
    /// Backward slice from sink to all possible sources
    pub fn backward_slice(&self, sink_node: NodeId) -> HashSet<NodeId> {
        let mut slice = HashSet::new();
        let mut worklist = VecDeque::new();
        
        slice.insert(sink_node);
        worklist.push_back(sink_node);
        
        while let Some(node) = worklist.pop_front() {
            // Get predecessors in CFG
            for pred in self.cfg.predecessors(node) {
                if !slice.contains(&pred) {
                    // Check if predecessor affects this node's inputs
                    if self.affects_inputs(pred, node) {
                        slice.insert(pred);
                        worklist.push_back(pred);
                    }
                }
            }
        }
        
        slice
    }
    
    /// Check if node's outputs affect target's inputs
    fn affects_inputs(&self, node: NodeId, target: NodeId) -> bool {
        // Get variables defined in node
        let defined = self.get_defined_variables(node);
        
        // Get variables used in target
        let used = self.get_used_variables(target);
        
        // Check for intersection
        !defined.is_disjoint(&used)
    }
    
    fn get_defined_variables(&self, node: NodeId) -> HashSet<String> {
        // Analyze AST node to find variable definitions
        // e.g., x = ... defines x
        let mut defined = HashSet::new();
        // ... implementation
        defined
    }
    
    fn get_used_variables(&self, node: NodeId) -> HashSet<String> {
        // Analyze AST node to find variable uses
        // e.g., ... = x uses x
        let mut used = HashSet::new();
        // ... implementation
        used
    }
}
```

---

### Step 6: Forward Slicing Implementation

**Implementation:**
```rust
impl SliceEngine {
    /// Forward slice from source to all affected sinks
    pub fn forward_slice(&self, source_node: NodeId) -> HashSet<NodeId> {
        let mut slice = HashSet::new();
        let mut worklist = VecDeque::new();
        
        slice.insert(source_node);
        worklist.push_back(source_node);
        
        while let Some(node) = worklist.pop_front() {
            // Get successors in CFG
            for succ in self.cfg.successors(node) {
                if !slice.contains(&succ) {
                    // Check if this node's outputs affect successor's inputs
                    if self.affects_inputs(node, succ) {
                        slice.insert(succ);
                        worklist.push_back(succ);
                    }
                }
            }
        }
        
        slice
    }
    
    /// Find vulnerability path (intersection of forward and backward slices)
    pub fn vulnerability_slice(
        &self,
        source_node: NodeId,
        sink_node: NodeId,
    ) -> HashSet<NodeId> {
        let forward = self.forward_slice(source_node);
        let backward = self.backward_slice(sink_node);
        
        // Intersection = vulnerability path
        forward.intersection(&backward).copied().collect()
    }
}
```

---

### Step 7: LLM Integration

**Slice-to-LLM Pipeline:**
```rust
use serde_json::json;

pub struct LlmAnalyzer {
    client: reqwest::Client,
    endpoint: String,
    model: String,
}

impl LlmAnalyzer {
    pub async fn analyze_slice(
        &self,
        slice_nodes: &[NodeId],
        source_code: &str,
    ) -> Result<LlmAnalysis, LlmError> {
        // Extract code from slice nodes
        let slice_code = self.extract_slice_code(slice_nodes, source_code);
        
        // Build prompt
        let prompt = self.build_vulnerability_prompt(&slice_code);
        
        // Call LLM
        let response = self.client
            .post(&self.endpoint)
            .json(&json!({
                "model": self.model,
                "messages": [
                    {"role": "system", "content": self.get_system_prompt()},
                    {"role": "user", "content": prompt}
                ],
                "max_tokens": 1024,
                "temperature": 0.0,  // Deterministic for security analysis
            }))
            .send()
            .await?;
        
        let result: LlmResponse = response.json().await?;
        Ok(self.parse_llm_analysis(result))
    }
    
    fn build_vulnerability_prompt(&self, slice_code: &str) -> String {
        format!(
            r#"Analyze this code slice for security vulnerabilities.

Code:
```
{}
```

Provide:
1. Vulnerability type (if any)
2. CWE ID
3. Severity (Critical/High/Medium/Low)
4. Explanation
5. Remediation

Respond in JSON format."#,
            slice_code
        )
    }
}
```

---

## Integration Plan with Coax

### Phase 1: CFG Foundation (Week 1-2)

| Task | Description | Effort |
|------|-------------|--------|
| Add tree-sitter dependency | Multi-language parser | 1 day |
| Add oxc_cfg dependency | CFG construction | 0.5 days |
| Implement AST extraction | Parse files to AST | 2 days |
| Implement CFG builder | Build CFG from AST | 3 days |
| Unit tests | Verify CFG correctness | 1 day |

**Deliverable:** Working CFG builder for Rust/Python/JavaScript

---

### Phase 2: Entry/Sink Detection (Week 3)

| Task | Description | Effort |
|------|-------------|--------|
| Entry point detection | HTTP routes, CLI, public functions | 2 days |
| Sink point detection | SQL, exec, file I/O, network | 2 days |
| Source point detection | User input, external data | 1 day |
| Unit tests | Verify detection accuracy | 1 day |

**Deliverable:** Entry/sink/source detection for common patterns

---

### Phase 3: Slicing Engine (Week 4-5)

| Task | Description | Effort |
|------|-------------|--------|
| Backward slicing | Sink → source traversal | 3 days |
| Forward slicing | Source → sink traversal | 3 days |
| Slice intersection | Vulnerability path extraction | 2 days |
| Code extraction | Extract slice code for LLM | 2 days |

**Deliverable:** Working slicing engine

---

### Phase 4: LLM Integration (Week 6-7)

| Task | Description | Effort |
|------|-------------|--------|
| LLM client module | HTTP client for vLLM/Modal | 2 days |
| Prompt engineering | Vulnerability analysis prompts | 3 days |
| Response parsing | JSON extraction from LLM | 1 day |
| Caching layer | Cache LLM responses | 2 days |

**Deliverable:** LLM-powered vulnerability analysis

---

### Phase 5: CLI Integration (Week 8)

| Task | Description | Effort |
|------|-------------|--------|
| Add `--cfg` flag | Enable CFG-based analysis | 1 day |
| Add `--llm` flag | Enable LLM analysis | 1 day |
| Output formatting | Integrate with existing output | 2 days |
| Performance optimization | Parallel slicing, caching | 2 days |

**Deliverable:** Full CFG+LLM integration in Coax CLI

---

## Estimated Effort Summary

| Phase | Tasks | Effort | Timeline |
|-------|-------|--------|----------|
| **Phase 1** | CFG Foundation | 7.5 days | 2 weeks |
| **Phase 2** | Entry/Sink Detection | 6 days | 1 week |
| **Phase 3** | Slicing Engine | 10 days | 2 weeks |
| **Phase 4** | LLM Integration | 8 days | 2 weeks |
| **Phase 5** | CLI Integration | 6 days | 1 week |
| **Total** | Full implementation | 37.5 days | 8 weeks |

---

## Complexity Assessment

| Component | Complexity | Risk | Mitigation |
|-----------|------------|------|------------|
| **Tree-sitter integration** | Low | Low | Well-documented, mature library |
| **oxc_cfg integration** | Medium | Low | Good docs, but newer crate |
| **Entry/sink detection** | Medium | Medium | Requires language-specific patterns |
| **Slicing algorithm** | High | Medium | Complex data flow analysis |
| **LLM integration** | Medium | Low | Standard HTTP API |
| **Performance** | Medium | Medium | Parallel processing, caching needed |

---

## Success Criteria

### End of Phase 1

- [ ] Tree-sitter parses Rust/Python/JS files
- [ ] oxc_cfg builds valid CFGs
- [ ] CFG visualization for debugging
- [ ] Unit tests pass

### End of Phase 3

- [ ] Entry points detected accurately
- [ ] Sink points detected accurately
- [ ] Backward slicing works
- [ ] Forward slicing works
- [ ] Vulnerability paths extracted

### End of Phase 5

- [ ] LLM analyzes slices correctly
- [ ] `coax scan --cfg` works
- [ ] `coax scan --llm` works
- [ ] Performance acceptable (<5s per file)
- [ ] False positive rate <10%

---

## References

- **Tree-sitter:** https://tree-sitter.github.io/
- **oxc_cfg:** https://docs.rs/oxc_cfg
- **Semgrep:** https://semgrep.dev/
- **Joern:** https://joern.io/
- **CodeQL:** https://codeql.github.com/
- **VulnLLM-R-7B:** https://huggingface.co/UCSB-SURFI/VulnLLM-R-7B

---

*Research completed: 2026-03-15*
