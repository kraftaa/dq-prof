pub mod app;
pub mod baseline;
pub mod cli;
pub mod error;
pub mod io;
pub mod profile;
pub mod report;
pub mod rules;
pub mod types;
pub mod util;

pub use app::run as analyze_path;
