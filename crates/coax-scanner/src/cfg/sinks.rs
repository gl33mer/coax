//! Sink Point Detection
//!
//! Detects sink points where vulnerabilities manifest:
//! - SQL execution (execute, query, raw)
//! - Command injection (exec, system, spawn, subprocess)
//! - File I/O (open, read, write, unlink)
//! - Network requests (fetch, request, http)
//! - Secret usage (decrypt, verify, sign)
//! - Deserialization
//! - Path traversal

use petgraph::graph::NodeIndex;
use super::types::{SinkPoint, CFG, BasicBlock};

/// Detect all sink points in a CFG
pub fn detect_all(cfg: &CFG) -> Vec<SinkPoint> {
    let mut sinks = Vec::new();
    
    for node in cfg.graph.node_indices() {
        let block = &cfg.graph[node];
        let block_sinks = detect_from_block(block, node);
        sinks.extend(block_sinks);
    }
    
    sinks
}

/// Detect sink points from a basic block
pub fn detect_from_block(block: &BasicBlock, node: NodeIndex) -> Vec<SinkPoint> {
    let mut sinks = Vec::new();
    
    for stmt in &block.statements {
        // SQL sinks
        if let Some(sink) = detect_sql_sink(&stmt.text, node) {
            sinks.push(sink);
        }
        
        // Command execution sinks
        if let Some(sink) = detect_command_sink(&stmt.text, node) {
            sinks.push(sink);
        }
        
        // File I/O sinks
        if let Some(sink) = detect_file_sink(&stmt.text, node) {
            sinks.push(sink);
        }
        
        // Network sinks
        if let Some(sink) = detect_network_sink(&stmt.text, node) {
            sinks.push(sink);
        }
        
        // Secret usage sinks
        if let Some(sink) = detect_secret_sink(&stmt.text, node) {
            sinks.push(sink);
        }
        
        // Deserialization sinks
        if let Some(sink) = detect_deserialization_sink(&stmt.text, node) {
            sinks.push(sink);
        }
        
        // Path traversal sinks
        if let Some(sink) = detect_path_traversal_sink(&stmt.text, node) {
            sinks.push(sink);
        }
    }
    
    sinks
}

/// Detect SQL execution sinks
fn detect_sql_sink(text: &str, _node: NodeIndex) -> Option<SinkPoint> {
    let text_lower = text.to_lowercase();
    
    // SQL execution patterns
    let sql_patterns = [
        (".execute(", "execute"),
        (".query(", "query"),
        (".raw(", "raw"),
        (".exec(", "exec"),
        (".execute_query(", "execute_query"),
        (".execute_update(", "execute_update"),
        (".execute_batch(", "execute_batch"),
        ("sql_query(", "sql_query"),
        ("db.execute(", "execute"),
        ("connection.execute(", "execute"),
        ("client.query(", "query"),
        ("pool.execute(", "execute"),
        ("transaction.execute(", "execute"),
        ("session.execute(", "execute"),
        ("cursor.execute(", "execute"),
        ("conn.execute(", "execute"),
    ];
    
    for (pattern, method) in sql_patterns {
        if text_lower.contains(pattern) {
            // Try to extract query
            let query = extract_string_argument(text).unwrap_or_else(|| "unknown".to_string());
            return Some(SinkPoint::SqlExecution {
                query,
                method: method.to_string(),
            });
        }
    }
    
    // SQLAlchemy patterns
    if text_lower.contains("session.query(") || text_lower.contains("db.session.execute(") {
        let query = extract_string_argument(text).unwrap_or_else(|| "unknown".to_string());
        return Some(SinkPoint::SqlExecution {
            query,
            method: "sqlalchemy".to_string(),
        });
    }
    
    // Prisma patterns
    if text_lower.contains("prisma.") && (text_lower.contains("queryraw(") || text_lower.contains("executeraw(")) {
        let query = extract_string_argument(text).unwrap_or_else(|| "unknown".to_string());
        return Some(SinkPoint::SqlExecution {
            query,
            method: "prisma_raw".to_string(),
        });
    }
    
    // TypeORM patterns
    if text_lower.contains(".query(") && text_lower.contains("repository") {
        let query = extract_string_argument(text).unwrap_or_else(|| "unknown".to_string());
        return Some(SinkPoint::SqlExecution {
            query,
            method: "typeorm".to_string(),
        });
    }
    
    None
}

/// Detect command execution sinks
fn detect_command_sink(text: &str, _node: NodeIndex) -> Option<SinkPoint> {
    let text_lower = text.to_lowercase();
    
    // Command execution patterns
    let cmd_patterns = [
        (".exec(", "exec"),
        (".execsync(", "execSync"),
        (".execasync(", "execAsync"),
        ("exec(", "exec"),
        ("system(", "system"),
        ("popen(", "popen"),
        ("spawn(", "spawn"),
        (".spawn(", "spawn"),
        (".spawn_sync(", "spawn_sync"),
        ("subprocess.call(", "subprocess.call"),
        ("subprocess.run(", "subprocess.run"),
        ("subprocess.Popen(", "subprocess.Popen"),
        ("subprocess.check_output(", "subprocess.check_output"),
        ("child_process.exec(", "child_process.exec"),
        ("child_process.spawn(", "child_process.spawn"),
        ("child_process.execFile(", "child_process.execFile"),
        ("runtime.exec(", "runtime.exec"),
        ("processbuilder(", "ProcessBuilder"),
        ("runtime.getruntime().exec(", "Runtime.exec"),
        (".start(", "start"),  // Process.start
    ];
    
    for (pattern, method) in cmd_patterns {
        if text_lower.contains(pattern) {
            let command = extract_string_argument(text).unwrap_or_else(|| "unknown".to_string());
            return Some(SinkPoint::CommandExecution {
                command,
                method: method.to_string(),
            });
        }
    }
    
    // Shell execution
    if text_lower.contains("shell_exec(") || text_lower.contains("sh -c") {
        let command = extract_string_argument(text).unwrap_or_else(|| "unknown".to_string());
        return Some(SinkPoint::CommandExecution {
            command,
            method: "shell_exec".to_string(),
        });
    }
    
    // Backtick execution (shell)
    if text.contains("`") && text.contains("$") {
        return Some(SinkPoint::CommandExecution {
            command: "shell_interpolation".to_string(),
            method: "backtick".to_string(),
        });
    }
    
    None
}

/// Detect file I/O sinks
fn detect_file_sink(text: &str, _node: NodeIndex) -> Option<SinkPoint> {
    let text_lower = text.to_lowercase();
    
    // File operation patterns
    let file_patterns = [
        (".open(", "open"),
        ("open(", "open"),
        ("fs.open(", "fs.open"),
        ("fs.openSync(", "fs.openSync"),
        ("fs.readFile(", "fs.readFile"),
        ("fs.readFileSync(", "fs.readFileSync"),
        ("fs.writeFile(", "fs.writeFile"),
        ("fs.writeFileSync(", "fs.writeFileSync"),
        ("fs.appendFile(", "fs.appendFile"),
        ("fs.unlink(", "fs.unlink"),
        ("fs.unlinkSync(", "fs.unlinkSync"),
        ("fs.stat(", "fs.stat"),
        ("fs.readdir(", "fs.readdir"),
        ("fs.mkdir(", "fs.mkdir"),
        ("fs.rmdir(", "fs.rmdir"),
        ("fs.rename(", "fs.rename"),
        ("fs.copyFile(", "fs.copyFile"),
        ("file.read(", "file.read"),
        ("file.write(", "file.write"),
        ("io.open(", "io.open"),
        ("pathlib.Path(", "Path"),
        (".read_text(", "read_text"),
        (".write_text(", "write_text"),
        (".read_bytes(", "read_bytes"),
        (".write_bytes(", "write_bytes"),
        ("std::fs::File::open(", "File::open"),
        ("std::fs::read(", "fs::read"),
        ("std::fs::write(", "fs::write"),
        ("std::fs::remove_file(", "fs::remove_file"),
        ("std::fs::create_dir(", "fs::create_dir"),
        ("FileInputStream(", "FileInputStream"),
        ("FileOutputStream(", "FileOutputStream"),
        ("Files.read(", "Files.read"),
        ("Files.write(", "Files.write"),
        ("Files.delete(", "Files.delete"),
        ("Files.copy(", "Files.copy"),
        ("Files.move(", "Files.move"),
    ];
    
    for (pattern, method) in file_patterns {
        if text_lower.contains(pattern) {
            let path = extract_string_argument(text).unwrap_or_else(|| "unknown".to_string());
            let mode = detect_file_mode(text);
            return Some(SinkPoint::FileOperation {
                path,
                mode,
                method: method.to_string(),
            });
        }
    }
    
    None
}

/// Detect file mode from text
fn detect_file_mode(text: &str) -> String {
    let text_lower = text.to_lowercase();
    
    if text_lower.contains("\"w\"") || text_lower.contains("'w'") || 
       text_lower.contains("write") {
        return "write".to_string();
    }
    if text_lower.contains("\"a\"") || text_lower.contains("'a'") || 
       text_lower.contains("append") {
        return "append".to_string();
    }
    if text_lower.contains("\"r+\"") || text_lower.contains("'r+'") || 
       text_lower.contains("readwrite") {
        return "readwrite".to_string();
    }
    if text_lower.contains("\"w+\"") || text_lower.contains("'w+'") {
        return "write+".to_string();
    }
    
    // Default is read
    "read".to_string()
}

/// Detect network request sinks
fn detect_network_sink(text: &str, _node: NodeIndex) -> Option<SinkPoint> {
    let text_lower = text.to_lowercase();
    
    // Network request patterns
    let net_patterns = [
        (".fetch(", "fetch"),
        ("fetch(", "fetch"),
        ("axios.get(", "axios.get"),
        ("axios.post(", "axios.post"),
        ("axios.put(", "axios.put"),
        ("axios.delete(", "axios.delete"),
        ("axios.request(", "axios.request"),
        ("requests.get(", "requests.get"),
        ("requests.post(", "requests.post"),
        ("requests.put(", "requests.put"),
        ("requests.delete(", "requests.delete"),
        ("requests.request(", "requests.request"),
        ("http.get(", "http.get"),
        ("http.post(", "http.post"),
        ("http.request(", "http.request"),
        ("https.get(", "https.get"),
        ("https.request(", "https.request"),
        ("urllib.request(", "urllib.request"),
        ("urllib3.request(", "urllib3.request"),
        ("httpclient.get(", "httpClient.get"),
        ("httpclient.post(", "httpClient.post"),
        ("httpclient.send(", "httpClient.send"),
        ("okhttpclient.newcall(", "OkHttpClient.newCall"),
        ("webclient.get(", "WebClient.get"),
        ("webclient.post(", "WebClient.post"),
        ("reqwest::get(", "reqwest::get"),
        ("reqwest::Client::new().get(", "reqwest::get"),
        ("reqwest::Client::new().post(", "reqwest::post"),
        ("reqwest::Client::new().send(", "reqwest::send"),
        (".send(", "send"),
        (".get(", "get"),
        (".post(", "post"),
        (".put(", "put"),
        (".delete(", "delete"),
        (".head(", "head"),
        (".patch(", "patch"),
        (".options(", "options"),
    ];
    
    for (pattern, method) in net_patterns {
        if text_lower.contains(pattern) {
            let url = extract_string_argument(text).unwrap_or_else(|| "unknown".to_string());
            return Some(SinkPoint::NetworkRequest {
                url,
                method: method.to_string(),
            });
        }
    }
    
    // WebSocket
    if text_lower.contains("websocket(") || text_lower.contains("ws(") || 
       text_lower.contains("wss(") {
        let url = extract_string_argument(text).unwrap_or_else(|| "ws://unknown".to_string());
        return Some(SinkPoint::NetworkRequest {
            url,
            method: "websocket".to_string(),
        });
    }
    
    None
}

/// Detect secret usage sinks
fn detect_secret_sink(text: &str, _node: NodeIndex) -> Option<SinkPoint> {
    let text_lower = text.to_lowercase();
    
    // Secret/crypto operations
    let secret_patterns = [
        (".decrypt(", "decrypt"),
        (".encrypt(", "encrypt"),
        (".verify(", "verify"),
        (".sign(", "sign"),
        (".validate(", "validate"),
        ("crypto.decrypt(", "crypto.decrypt"),
        ("crypto.encrypt(", "crypto.encrypt"),
        ("crypto.verify(", "crypto.verify"),
        ("crypto.sign(", "crypto.sign"),
        ("Cipher.doFinal(", "Cipher.doFinal"),
        ("MessageDigest.digest(", "MessageDigest.digest"),
        ("Signature.verify(", "Signature.verify"),
        ("Signature.sign(", "Signature.sign"),
        ("secret.get(", "secret.get"),
        ("vault.read(", "vault.read"),
        ("kms.decrypt(", "KMS.decrypt"),
        ("kms.encrypt(", "KMS.encrypt"),
        ("password.verify(", "password.verify"),
        ("bcrypt.compare(", "bcrypt.compare"),
        ("bcrypt.hash(", "bcrypt.hash"),
        ("argon2.verify(", "argon2.verify"),
        ("argon2.hash(", "argon2.hash"),
        ("jwt.verify(", "jwt.verify"),
        ("jwt.sign(", "jwt.sign"),
        ("jwt.decode(", "jwt.decode"),
    ];
    
    for (pattern, operation) in secret_patterns {
        if text_lower.contains(pattern) {
            let secret_type = detect_secret_type(text);
            return Some(SinkPoint::SecretUsage {
                secret_type,
                operation: operation.to_string(),
            });
        }
    }
    
    // API key usage
    if text_lower.contains("api_key") || text_lower.contains("apikey") ||
       text_lower.contains("api-key") {
        return Some(SinkPoint::SecretUsage {
            secret_type: "api_key".to_string(),
            operation: "usage".to_string(),
        });
    }
    
    // Token usage
    if text_lower.contains("access_token") || text_lower.contains("auth_token") ||
       text_lower.contains("bearer") {
        return Some(SinkPoint::SecretUsage {
            secret_type: "token".to_string(),
            operation: "usage".to_string(),
        });
    }
    
    None
}

/// Detect secret type from text
fn detect_secret_type(text: &str) -> String {
    let text_lower = text.to_lowercase();
    
    if text_lower.contains("password") || text_lower.contains("passwd") {
        return "password".to_string();
    }
    if text_lower.contains("api_key") || text_lower.contains("apikey") {
        return "api_key".to_string();
    }
    if text_lower.contains("token") {
        return "token".to_string();
    }
    if text_lower.contains("secret") {
        return "secret".to_string();
    }
    if text_lower.contains("key") && text_lower.contains("private") {
        return "private_key".to_string();
    }
    if text_lower.contains("jwt") {
        return "jwt".to_string();
    }
    if text_lower.contains("aes") || text_lower.contains("cipher") {
        return "encryption_key".to_string();
    }
    
    "unknown".to_string()
}

/// Detect deserialization sinks
fn detect_deserialization_sink(text: &str, _node: NodeIndex) -> Option<SinkPoint> {
    let text_lower = text.to_lowercase();
    
    // Deserialization patterns
    let deser_patterns = [
        (".deserialize(", "deserialize"),
        ("json.loads(", "json.loads"),
        ("json.load(", "json.load"),
        ("pickle.loads(", "pickle.loads"),
        ("pickle.load(", "pickle.load"),
        ("yaml.load(", "yaml.load"),
        ("yaml.safe_load(", "yaml.safe_load"),
        ("yaml.unsafe_load(", "yaml.unsafe_load"),
        ("xml.loads(", "xml.loads"),
        ("xml.parse(", "xml.parse"),
        ("xml.read(", "xml.read"),
        ("xmlserializer.deserialize(", "XmlSerializer.Deserialize"),
        ("javascriptserializer.deserialize(", "JavaScriptSerializer.Deserialize"),
        ("binaryformatter.deserialize(", "BinaryFormatter.Deserialize"),
        ("netdatacontractserializer.readobject(", "DataContractSerializer.ReadObject"),
        ("gson.fromjson(", "Gson.fromJson"),
        ("jackson.readvalue(", "Jackson.readValue"),
        ("objectmapper.readvalue(", "ObjectMapper.readValue"),
        ("serde_json::from_str(", "serde_json::from_str"),
        ("serde_json::from_slice(", "serde_json::from_slice"),
        ("serde_yaml::from_str(", "serde_yaml::from_str"),
        ("bincode::deserialize(", "bincode::deserialize"),
        ("messagepack::from_slice(", "rmp_serde::from_slice"),
        ("protobuf::parse(", "protobuf::parse"),
        ("unmarshal(", "unmarshal"),
        ("json.unmarshal(", "json.Unmarshal"),
        ("gob.decode(", "gob.Decode"),
        ("bson.decode(", "bson.Decode"),
    ];
    
    for (pattern, method) in deser_patterns {
        if text_lower.contains(pattern) {
            let source = extract_string_argument(text).unwrap_or_else(|| "unknown".to_string());
            return Some(SinkPoint::Deserialization {
                source,
                method: method.to_string(),
            });
        }
    }
    
    None
}

/// Detect path traversal sinks
fn detect_path_traversal_sink(text: &str, _node: NodeIndex) -> Option<SinkPoint> {
    let text_lower = text.to_lowercase();
    
    // Path traversal patterns - file operations with user input
    let path_patterns = [
        (".join(", "path.join"),
        ("path.join(", "path.join"),
        ("os.path.join(", "os.path.join"),
        ("pathlib.Path(", "Path"),
        ("file.resolve(", "file.resolve"),
        ("path.resolve(", "path.resolve"),
        ("path.combine(", "Path.Combine"),
    ];
    
    // Check if path operation contains user input indicators
    let user_input_indicators = ["request.", "params", "query", "body", "input", "user_input"];
    
    for (pattern, method) in path_patterns {
        if text_lower.contains(pattern) {
            // Check if there's user input involved
            if user_input_indicators.iter().any(|ind| text_lower.contains(ind)) {
                let path = extract_string_argument(text).unwrap_or_else(|| "user_input".to_string());
                return Some(SinkPoint::PathTraversal {
                    path,
                    method: method.to_string(),
                });
            }
        }
    }
    
    // Direct file access with user input
    if text_lower.contains("request.") && (text_lower.contains("param") || text_lower.contains("query")) {
        if text_lower.contains("open(") || text_lower.contains("readfile(") || 
           text_lower.contains("include(") || text_lower.contains("require(") {
            return Some(SinkPoint::PathTraversal {
                path: "request_input".to_string(),
                method: "direct".to_string(),
            });
        }
    }
    
    None
}

/// Extract string argument from text
fn extract_string_argument(text: &str) -> Option<String> {
    // Find first quoted string
    if let Some(start) = text.find('"') {
        if let Some(end) = text[start+1..].find('"') {
            return Some(text[start+1..start+1+end].to_string());
        }
    }
    if let Some(start) = text.find('\'') {
        if let Some(end) = text[start+1..].find('\'') {
            return Some(text[start+1..start+1+end].to_string());
        }
    }
    
    // Find variable name
    if let Some(paren) = text.find('(') {
        let args = &text[paren+1..];
        if let Some(end) = args.find(|c: char| c == ')' || c == ',') {
            let arg = args[..end].trim();
            if !arg.is_empty() && !arg.starts_with('"') && !arg.starts_with('\'') {
                return Some(arg.to_string());
            }
        }
    }
    
    None
}

/// Detect sinks from specific pattern match
pub fn detect_sinks_for_pattern(cfg: &CFG, pattern: &str) -> Vec<SinkPoint> {
    let all_sinks = detect_all(cfg);
    
    // Filter sinks based on pattern
    all_sinks.into_iter().filter(|sink| {
        match pattern.to_lowercase().as_str() {
            p if p.contains("sql") => matches!(sink, SinkPoint::SqlExecution { .. }),
            p if p.contains("command") || p.contains("exec") => matches!(sink, SinkPoint::CommandExecution { .. }),
            p if p.contains("file") => matches!(sink, SinkPoint::FileOperation { .. }),
            p if p.contains("network") || p.contains("http") => matches!(sink, SinkPoint::NetworkRequest { .. }),
            p if p.contains("secret") => matches!(sink, SinkPoint::SecretUsage { .. }),
            p if p.contains("deserial") => matches!(sink, SinkPoint::Deserialization { .. }),
            p if p.contains("path") => matches!(sink, SinkPoint::PathTraversal { .. }),
            _ => true,
        }
    }).collect()
}
