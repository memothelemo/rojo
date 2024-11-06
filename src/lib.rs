// Recursion limit bump is to support Ritz, a JSX-like proc macro used for
// Rojo's web UI currently.
#![cfg_attr(feature = "binary", recursion_limit = "1024")]

#[cfg(feature = "binary")]
pub mod cli;

#[cfg(test)]
mod tree_view;

#[cfg(feature = "binary")]
mod auth_cookie;
#[cfg(feature = "binary")]
mod change_processor;
#[cfg(feature = "binary")]
mod glob;
#[cfg(feature = "binary")]
mod lua_ast;
#[cfg(feature = "binary")]
mod message_queue;
#[cfg(feature = "binary")]
mod multimap;
mod path_serializer;
mod project;
#[cfg(feature = "binary")]
mod resolution;
#[cfg(feature = "binary")]
mod serve_session;
mod session_id;
#[cfg(feature = "binary")]
mod snapshot;
#[cfg(feature = "binary")]
mod snapshot_middleware;
#[cfg(feature = "binary")]
mod web;

#[cfg(not(feature = "binary"))]
pub mod glob;
#[cfg(not(feature = "binary"))]
pub mod multimap;
#[cfg(not(feature = "binary"))]
pub mod resolution;
#[cfg(not(feature = "binary"))]
pub mod snapshot;
#[cfg(not(feature = "binary"))]
pub mod snapshot_middleware;

pub use project::*;
pub use session_id::SessionId;

#[cfg(feature = "binary")]
pub use web::interface as web_api;
