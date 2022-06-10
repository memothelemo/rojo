// Recursion limit bump is to support Ritz, a JSX-like proc macro used for
// Rojo's web UI currently.
#![recursion_limit = "1024"]

pub mod cli;
mod project;

#[cfg(test)]
mod tree_view;

pub mod auth_cookie;
pub mod change_processor;
pub mod glob;
pub mod lua_ast;
pub mod message_queue;
pub mod multimap;
pub mod path_serializer;
pub use project::*;
pub mod resolution;
pub mod session_id;
pub mod snapshot;
pub mod snapshot_middleware;
pub mod web;
pub use session_id::SessionId;
pub mod serve_session;
pub use web::interface as web_api;
