//! Entry Point Detection

use petgraph::graph::NodeIndex;
use super::types::{EntryPoint, CFG, BasicBlock};

/// Detect all entry points in a CFG
pub fn detect_all(cfg: &CFG) -> Vec<EntryPoint> {
    let mut entries = Vec::new();
    
    for node in cfg.graph.node_indices() {
        let block = &cfg.graph[node];
        if let Some(entry) = detect_from_block(block, node) {
            entries.push(entry);
        }
    }
    
    entries
}

/// Detect entry point from a basic block
fn detect_from_block(block: &BasicBlock, _node: NodeIndex) -> Option<EntryPoint> {
    for stmt in &block.statements {
        if let Some(entry) = detect_http_route(&stmt.text) {
            return Some(entry);
        }
        if let Some(entry) = detect_cli_command(&stmt.text) {
            return Some(entry);
        }
        if let Some(entry) = detect_public_function(&stmt.text) {
            return Some(entry);
        }
    }
    None
}

/// Detect HTTP route patterns
fn detect_http_route(text: &str) -> Option<EntryPoint> {
    let text = text.trim();
    
    // Express.js: app.get("/api/users", ...)
    if let Some(route) = parse_express_route(text) {
        return Some(EntryPoint::HttpRoute {
            method: route.0,
            path: route.1,
            function: route.2,
            framework: "express".to_string(),
        });
    }
    
    // Flask: @app.route("/api/users")
    if let Some(route) = parse_flask_route(text) {
        return Some(EntryPoint::HttpRoute {
            method: route.0,
            path: route.1,
            function: route.2,
            framework: "flask".to_string(),
        });
    }
    
    // FastAPI: @app.get("/api/users")
    if let Some(route) = parse_fastapi_route(text) {
        return Some(EntryPoint::HttpRoute {
            method: route.0,
            path: route.1,
            function: route.2,
            framework: "fastapi".to_string(),
        });
    }
    
    None
}

fn parse_express_route(text: &str) -> Option<(String, String, String)> {
    // Simple pattern matching for app.get("/path", handler)
    if text.contains("app.get(") || text.contains("app.post(") || 
       text.contains("router.get(") || text.contains("router.post(") {
        let method = if text.contains(".get(") { "GET" } else { "POST" };
        
        // Extract path between quotes
        if let Some(start) = text.find('"').or_else(|| text.find('\'')) {
            if let Some(end) = text[start+1..].find('"').or_else(|| text[start+1..].find('\'')) {
                let path = text[start+1..start+1+end].to_string();
                return Some((method.to_string(), path, "handler".to_string()));
            }
        }
    }
    None
}

fn parse_flask_route(text: &str) -> Option<(String, String, String)> {
    if text.starts_with("@app.route") || text.starts_with("@blueprint.route") {
        if let Some(start) = text.find('"').or_else(|| text.find('\'')) {
            if let Some(end) = text[start+1..].find('"').or_else(|| text[start+1..].find('\'')) {
                let path = text[start+1..start+1+end].to_string();
                let method = if text.contains("POST") { "POST" } else { "GET" };
                return Some((method.to_string(), path, "unknown".to_string()));
            }
        }
    }
    None
}

fn parse_fastapi_route(text: &str) -> Option<(String, String, String)> {
    if text.contains("@app.get(") || text.contains("@app.post(") ||
       text.contains("@router.get(") || text.contains("@router.post(") {
        let method = if text.contains(".get(") { "GET" } else { "POST" };
        if let Some(start) = text.find('"').or_else(|| text.find('\'')) {
            if let Some(end) = text[start+1..].find('"').or_else(|| text[start+1..].find('\'')) {
                let path = text[start+1..start+1+end].to_string();
                return Some((method.to_string(), path, "unknown".to_string()));
            }
        }
    }
    None
}

/// Detect CLI command patterns
fn detect_cli_command(text: &str) -> Option<EntryPoint> {
    let text = text.trim();
    
    if text.contains("Command::new(") || text.contains("#[command(") {
        if let Some(start) = text.find('"') {
            if let Some(end) = text[start+1..].find('"') {
                let name = text[start+1..start+1+end].to_string();
                return Some(EntryPoint::CliCommand {
                    name,
                    function: "main".to_string(),
                });
            }
        }
    }
    None
}

/// Detect public function patterns
fn detect_public_function(text: &str) -> Option<EntryPoint> {
    let text = text.trim();
    
    if text.starts_with("pub fn") || text.starts_with("pub async fn") {
        if let Some(fn_pos) = text.find("fn") {
            let after_fn = text[fn_pos + 2..].trim_start();
            let name: String = after_fn
                .chars()
                .take_while(|c| c.is_alphanumeric() || *c == '_')
                .collect();
            
            if !name.is_empty() {
                return Some(EntryPoint::PublicFunction {
                    name,
                    file: "unknown".to_string(),
                });
            }
        }
    }
    None
}
