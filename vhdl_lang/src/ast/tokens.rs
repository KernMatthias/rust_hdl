use crate::{
    data::ContentReader,
    syntax::{Symbols, Token},
    NullDiagnostics, Source, SrcPos,
};

pub enum SourceTokenKind {
    Keyword,
    Variable,
    Signal,
    Port,
    Entity,
    Architecture,
    Procedure,
    Function,
    Component,
    Comment,
    String,
    Number,
    BitString,
    Character,
    Operator,
}

pub struct SourceToken {
    pub kind: SourceTokenKind,
    pub pos: SrcPos,
}

impl From<&Token> for SourceToken {
    fn from(token: &Token) -> Self {
        let kind = if token.is_keyword() {
            SourceTokenKind::Keyword
        } else if token.is_operator() {
            SourceTokenKind::Operator
        } else {
            match token.value {
                // find out what kind of identifier -> most important operation
                crate::syntax::Value::Identifier(_) => {
                    todo!("{:?}", token)
                }
                crate::syntax::Value::String(_) => SourceTokenKind::String,
                crate::syntax::Value::BitString(_) => SourceTokenKind::BitString,
                crate::syntax::Value::AbstractLiteral(_) => SourceTokenKind::Number,
                crate::syntax::Value::Character(_) => SourceTokenKind::String,
                // tool directive
                crate::syntax::Value::Text(_) => todo!("{:?}", token),
                // no idea...
                crate::syntax::Value::NoValue => todo!("{:?}", token),
            }
        };

        Self {
            kind,
            pos: token.pos.clone(),
        }
    }
}

impl From<&Source> for Vec<SourceToken> {
    fn from(source: &Source) -> Self {
        // seems rly inefficient, but tokenize the source file, if identifier: look up identifier
        // and use the received information
        let symbols = Symbols::default();
        let contents = source.contents();
        let reader = ContentReader::new(&contents);
        let tokenizer = crate::syntax::Tokenizer::new(&symbols, source, reader);
        let mut diagnostics = NullDiagnostics {};
        let tokenstream = crate::syntax::TokenStream::new(tokenizer, &mut diagnostics);

        let mut tokens = Vec::new();

        while let Some(token) = tokenstream.peek() {
            tokens.push(SourceToken::from(token));
            tokenstream.skip();
        }

        tokens
    }
}

