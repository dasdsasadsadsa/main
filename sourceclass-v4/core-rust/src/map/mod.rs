//! Project Map Module
//! 
//! Builds and maintains the interactive map of the entire codebase.

pub mod builder;
pub mod schema;
pub mod indexer;

pub use builder::ProjectMap;
pub use schema::*;
