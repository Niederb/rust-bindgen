//! Intermediate representation for modules (AKA C++ namespaces).

use super::context::{BindgenContext, ItemId};
use clang;
use parse::{ClangSubItemParser, ParseError, ParseResult};
use parse_one;

/// Whether this module is inline or not.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ModuleKind {
    /// This module is not inline.
    Normal,
    /// This module is inline, as in `inline namespace foo {}`.
    Inline,
}

/// A module, as in, a C++ namespace.
#[derive(Clone, Debug)]
pub struct Module {
    /// The name of the module, or none if it's anonymous.
    name: Option<String>,
    /// The kind of module this is.
    kind: ModuleKind,
    /// The children of this module, just here for convenience.
    children_ids: Vec<ItemId>,
}

impl Module {
    /// Construct a new `Module`.
    pub fn new(name: Option<String>, kind: ModuleKind) -> Self {
        Module {
            name: name,
            kind: kind,
            children_ids: vec![],
        }
    }

    /// Get this module's name.
    pub fn name(&self) -> Option<&str> {
        self.name.as_ref().map(|n| &**n)
    }

    /// Get a mutable reference to this module's children.
    pub fn children_mut(&mut self) -> &mut Vec<ItemId> {
        &mut self.children_ids
    }

    /// Get this module's children.
    pub fn children(&self) -> &[ItemId] {
        &self.children_ids
    }

    /// Whether this namespace is inline.
    pub fn is_inline(&self) -> bool {
        self.kind == ModuleKind::Inline
    }
}

impl ClangSubItemParser for Module {
    fn parse(cursor: clang::Cursor,
             ctx: &mut BindgenContext)
             -> Result<ParseResult<Self>, ParseError> {
        use clang_sys::*;
        match cursor.kind() {
            CXCursor_Namespace => {
                let module_id = ctx.module(cursor);
                ctx.with_module(module_id, |ctx| {
                    cursor.visit(|cursor| {
                        parse_one(ctx, cursor, Some(module_id))
                    })
                });

                Ok(ParseResult::AlreadyResolved(module_id))
            }
            _ => Err(ParseError::Continue),
        }
    }
}
