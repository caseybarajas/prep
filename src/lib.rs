//! Prep - A beautiful CLI tool to refine prompts for AI assistants

pub mod cli;
pub mod config;
pub mod history;
pub mod providers;
pub mod refiner;
pub mod templates;
pub mod ui;

pub use config::Config;
pub use refiner::{Refiner, RefinerResponse};
