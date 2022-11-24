#![feature(type_alias_impl_trait)]

// Public Interface defined within the root.
pub mod file_io;
pub mod graph;
pub mod manifest;
pub mod print;
pub mod query;

// Implementations in their own folders.
pub mod file_io_backends;
mod graph_backends;
mod print_backends;
