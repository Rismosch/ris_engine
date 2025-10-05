pub mod asset;
pub mod build;
pub mod doc;
pub mod god_asset;
pub mod pipeline;
pub mod profiler_html;

use std::path::Path;

use ris_error::prelude::*;

use super::cmd;
use super::util;

pub enum ExplanationLevel {
    Short,
    Detailed,
}

pub trait ICommand {
    fn name(&self) -> String;
    fn args(&self) -> String;
    fn explanation(&self, level: ExplanationLevel) -> String;
    fn run(&self, args: Vec<String>, target_dir: &Path) -> RisResult<()>;
}
