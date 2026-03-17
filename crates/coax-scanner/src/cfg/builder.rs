//! CFG Builder - Constructs Control Flow Graph from AST

use petgraph::graph::{DiGraph, NodeIndex};
use std::collections::HashMap;
use tree_sitter::{Node, Parser, Tree};

use super::types::*;
use super::{CFGError, Language, Result};

/// CFG Builder - constructs CFG from source code
pub struct CFGBuilder {
    language: Language,
}

impl CFGBuilder {
    /// Create a new CFG builder for a specific language
    pub fn new(language: Language) -> Self {
        Self { language }
    }

    /// Create builder from file extension
    pub fn for_extension(ext: &str) -> Option<Self> {
        let lang = Language::from_extension(ext);
        if lang == Language::Unknown {
            None
        } else {
            Some(Self::new(lang))
        }
    }

    /// Create builder from language name
    pub fn for_language(name: &str) -> Option<Self> {
        let lang = Language::from_name(name);
        if lang == Language::Unknown {
            None
        } else {
            Some(Self::new(lang))
        }
    }

    /// Build CFG from source code
    pub fn build(&self, source: &str) -> Result<CFG> {
        if self.language == Language::Unknown {
            return Err(CFGError::LanguageNotSupported(
                "Unknown language".to_string(),
            ));
        }

        // Parse source
        let mut parser = Parser::new();

        // Set language-specific parser
        let tree = match self.language {
            Language::Rust => {
                parser.set_language(&tree_sitter_rust::LANGUAGE.into()).ok();
                parser.parse(source, None)
            }
            Language::Python => {
                parser
                    .set_language(&tree_sitter_python::LANGUAGE.into())
                    .ok();
                parser.parse(source, None)
            }
            Language::JavaScript => {
                parser
                    .set_language(&tree_sitter_javascript::LANGUAGE.into())
                    .ok();
                parser.parse(source, None)
            }
            Language::TypeScript => {
                parser
                    .set_language(&tree_sitter_typescript::LANGUAGE_TSX.into())
                    .ok();
                parser.parse(source, None)
            }
            Language::Unknown => None,
        }
        .ok_or_else(|| CFGError::Parse("Failed to parse source".to_string()))?;

        // Extract basic blocks
        let blocks = self.extract_blocks(&tree, source)?;

        // Build graph
        let mut graph = DiGraph::new();
        let mut block_to_node: HashMap<usize, NodeIndex> = HashMap::new();

        // Add nodes
        for block in &blocks {
            let node = graph.add_node(block.clone());
            block_to_node.insert(block.id, node);
        }

        // Find entry and exit
        let entry_id = blocks
            .iter()
            .find(|b| b.kind == BlockKind::Entry)
            .map(|b| b.id)
            .unwrap_or(0);

        let exit_id = blocks
            .iter()
            .find(|b| b.kind == BlockKind::Exit)
            .map(|b| b.id)
            .unwrap_or_else(|| blocks.len().saturating_sub(1));

        let entry_node = *block_to_node
            .get(&entry_id)
            .ok_or_else(|| CFGError::Construction("Entry block not found".to_string()))?;

        let exit_node = *block_to_node
            .get(&exit_id)
            .ok_or_else(|| CFGError::Construction("Exit block not found".to_string()))?;

        // Build edges
        self.build_edges(&blocks, &mut graph, &block_to_node)?;

        Ok(CFG {
            graph,
            entry: entry_node,
            exit: exit_node,
        })
    }

    /// Extract basic blocks from AST
    fn extract_blocks(&self, tree: &Tree, _source: &str) -> Result<Vec<BasicBlock>> {
        let mut blocks = Vec::new();
        let mut block_id = 0;

        // Create entry block
        let mut entry_block = BasicBlock::new(block_id, BlockKind::Entry);
        entry_block.line_start = 1;
        entry_block.line_end = 1;
        blocks.push(entry_block);
        block_id += 1;

        // Walk AST
        let root = tree.root_node();
        self.walk_ast(root, &mut blocks, &mut block_id)?;

        // Create exit block
        let max_line = blocks.iter().map(|b| b.line_end).max().unwrap_or(1);

        let mut exit_block = BasicBlock::new(block_id, BlockKind::Exit);
        exit_block.line_start = max_line;
        exit_block.line_end = max_line;
        blocks.push(exit_block);

        Ok(blocks)
    }

    /// Walk AST and extract blocks
    fn walk_ast(
        &self,
        node: Node,
        blocks: &mut Vec<BasicBlock>,
        block_id: &mut usize,
    ) -> Result<()> {
        let mut cursor = node.walk();

        for child in node.children(&mut cursor) {
            match child.kind() {
                "function_definition" | "function_item" | "function_declaration" => {
                    self.handle_function(child, blocks, block_id)?;
                }
                "if_statement" | "if" => {
                    self.handle_if(child, blocks, block_id)?;
                }
                "match_expression" | "switch_statement" => {
                    self.handle_match(child, blocks, block_id)?;
                }
                "for_statement" | "for_in_statement" | "while_statement" | "loop_expression" => {
                    self.handle_loop(child, blocks, block_id)?;
                }
                "return_statement" | "return_expression" => {
                    self.handle_return(child, blocks, block_id)?;
                }
                "expression_statement" | "statement" => {
                    self.handle_statement(child, blocks)?;
                }
                "assignment_expression"
                | "let_declaration"
                | "const_declaration"
                | "variable_declaration" => {
                    self.handle_assignment(child, blocks)?;
                }
                "call_expression" | "call" => {
                    self.handle_call(child, blocks)?;
                }
                _ => {
                    self.walk_ast(child, blocks, block_id)?;
                }
            }
        }

        Ok(())
    }

    fn handle_function(
        &self,
        node: Node,
        blocks: &mut Vec<BasicBlock>,
        block_id: &mut usize,
    ) -> Result<()> {
        let mut block = BasicBlock::new(*block_id, BlockKind::Normal);
        *block_id += 1;
        let start = node.start_position().row as u32;
        let end = node.end_position().row as u32;
        block.line_start = start + 1;
        block.line_end = end + 1;
        blocks.push(block);

        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.walk_ast(child, blocks, block_id)?;
        }
        Ok(())
    }

    fn handle_if(
        &self,
        node: Node,
        blocks: &mut Vec<BasicBlock>,
        block_id: &mut usize,
    ) -> Result<()> {
        let mut branch_block = BasicBlock::new(*block_id, BlockKind::Branch);
        *block_id += 1;
        let start = node.start_position().row as u32;
        let end = node.end_position().row as u32;
        branch_block.line_start = start + 1;
        branch_block.line_end = end + 1;
        blocks.push(branch_block);

        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.walk_ast(child, blocks, block_id)?;
        }
        Ok(())
    }

    fn handle_match(
        &self,
        node: Node,
        blocks: &mut Vec<BasicBlock>,
        block_id: &mut usize,
    ) -> Result<()> {
        let mut match_block = BasicBlock::new(*block_id, BlockKind::Branch);
        *block_id += 1;
        let start = node.start_position().row as u32;
        let end = node.end_position().row as u32;
        match_block.line_start = start + 1;
        match_block.line_end = end + 1;
        blocks.push(match_block);

        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.walk_ast(child, blocks, block_id)?;
        }
        Ok(())
    }

    fn handle_loop(
        &self,
        node: Node,
        blocks: &mut Vec<BasicBlock>,
        block_id: &mut usize,
    ) -> Result<()> {
        let mut loop_block = BasicBlock::new(*block_id, BlockKind::Loop);
        *block_id += 1;
        let start = node.start_position().row as u32;
        let end = node.end_position().row as u32;
        loop_block.line_start = start + 1;
        loop_block.line_end = end + 1;
        blocks.push(loop_block);

        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.walk_ast(child, blocks, block_id)?;
        }
        Ok(())
    }

    fn handle_return(
        &self,
        node: Node,
        blocks: &mut Vec<BasicBlock>,
        block_id: &mut usize,
    ) -> Result<()> {
        let mut return_block = BasicBlock::new(*block_id, BlockKind::Return);
        *block_id += 1;
        let start = node.start_position().row as u32;
        return_block.line_start = start + 1;
        return_block.line_end = start + 1;
        blocks.push(return_block);
        Ok(())
    }

    fn handle_statement(&self, node: Node, blocks: &mut Vec<BasicBlock>) -> Result<()> {
        if blocks.is_empty() {
            return Ok(());
        }
        let last_block = blocks.last_mut().unwrap();
        let start = node.start_position().row as u32;
        if last_block.line_end < start + 1 {
            last_block.line_end = start + 1;
        }
        Ok(())
    }

    fn handle_assignment(&self, node: Node, blocks: &mut Vec<BasicBlock>) -> Result<()> {
        if blocks.is_empty() {
            return Ok(());
        }
        let last_block = blocks.last_mut().unwrap();
        let start = node.start_position().row as u32;
        if last_block.line_end < start + 1 {
            last_block.line_end = start + 1;
        }
        Ok(())
    }

    fn handle_call(&self, node: Node, blocks: &mut Vec<BasicBlock>) -> Result<()> {
        if blocks.is_empty() {
            return Ok(());
        }
        let last_block = blocks.last_mut().unwrap();
        let start = node.start_position().row as u32;
        if last_block.line_end < start + 1 {
            last_block.line_end = start + 1;
        }
        Ok(())
    }

    fn build_edges(
        &self,
        blocks: &[BasicBlock],
        graph: &mut DiGraph<BasicBlock, EdgeLabel>,
        block_to_node: &HashMap<usize, NodeIndex>,
    ) -> Result<()> {
        for i in 0..blocks.len() {
            let block = &blocks[i];
            let current_node = *block_to_node.get(&block.id).unwrap();

            match block.kind {
                BlockKind::Entry => {
                    if blocks.len() > 1 {
                        let next_node = *block_to_node.get(&blocks[1].id).unwrap();
                        graph.add_edge(current_node, next_node, EdgeLabel::Unconditional);
                    }
                }
                BlockKind::Branch => {
                    if i + 1 < blocks.len() {
                        let next_node = *block_to_node.get(&blocks[i + 1].id).unwrap();
                        graph.add_edge(current_node, next_node, EdgeLabel::TrueBranch);
                    }
                    if i + 2 < blocks.len() {
                        let next_node = *block_to_node.get(&blocks[i + 2].id).unwrap();
                        graph.add_edge(current_node, next_node, EdgeLabel::FalseBranch);
                    }
                }
                BlockKind::Loop => {
                    if i + 1 < blocks.len() {
                        let next_node = *block_to_node.get(&blocks[i + 1].id).unwrap();
                        graph.add_edge(current_node, next_node, EdgeLabel::Unconditional);
                    }
                }
                BlockKind::Return => {
                    let exit_node = block_to_node
                        .values()
                        .find(|&&n| graph[n].kind == BlockKind::Exit)
                        .copied();
                    if let Some(exit) = exit_node {
                        graph.add_edge(current_node, exit, EdgeLabel::Return);
                    }
                }
                BlockKind::Exit => {}
                BlockKind::Normal => {
                    if i + 1 < blocks.len() {
                        let next_node = *block_to_node.get(&blocks[i + 1].id).unwrap();
                        graph.add_edge(current_node, next_node, EdgeLabel::Unconditional);
                    }
                }
            }
        }
        Ok(())
    }
}
