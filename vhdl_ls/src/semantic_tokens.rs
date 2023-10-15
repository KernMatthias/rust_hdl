use lsp_types::{SemanticToken, SemanticTokenModifier, SemanticTokenType, SemanticTokens};

pub const VHDL_TOKENS: &[SemanticTokenType] = &[
    SemanticTokenType::TYPE,
    SemanticTokenType::ENUM_MEMBER,
    SemanticTokenType::STRING,
    SemanticTokenType::NUMBER,
    SemanticTokenType::OPERATOR,
    SemanticTokenType::KEYWORD,
    SemanticTokenType::FUNCTION,
    SemanticTokenType::VARIABLE,
    SemanticTokenType::PARAMETER,
    SemanticTokenType::PROPERTY,
    SemanticTokenType::COMMENT,
    // entity/component
    SemanticTokenType::INTERFACE,
    // architectures
    SemanticTokenType::CLASS,
    // Not yet decided: struct members, ...
];

pub const VHDL_TOKEN_MODIFIER: &[SemanticTokenModifier] = &[
    SemanticTokenModifier::READONLY,
    SemanticTokenModifier::DEFINITION,
    SemanticTokenModifier::DECLARATION,
    SemanticTokenModifier::DOCUMENTATION,
];

// adapted from rust_analzer: https://github.com/rust-lang/rust-analyzer/blob/master/crates/rust-analyzer/src/lsp/semantic_tokens.rs
pub struct SemanticTokenBuilder {
    line: u32,
    column: u32,
    length: u32,
    data: Vec<SemanticToken>,
}

impl SemanticTokenBuilder {
    pub fn new() -> Self {
        SemanticTokenBuilder {
            line: 0,
            column: 0,
            length: 0,
            data: Vec::new(),
        }
    }

    pub fn push(&mut self, range: &lsp_types::Range, token_type_idx: u32) {
        todo!("not yet implemented");

        self.line = line;
        self.column = column;
        self.length = length;
        self.data.push(SemanticToken {
            delta_line,
            delta_start: delta_column,
            length,
            token_type: token_type_idx,
            token_modifiers_bitset: 0,
        });
    }

    pub fn build(&self) -> SemanticTokens {
        let result = SemanticTokens {
            result_id: None,
            data: self.data.clone(),
        };
        result
    }
}

