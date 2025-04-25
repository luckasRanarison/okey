pub mod cli;

mod config;
mod core;
mod fs;

#[cfg(test)]
mod tests;

pub use fs::log;
