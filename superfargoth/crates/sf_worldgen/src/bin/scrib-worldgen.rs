//! `scrib-worldgen`: runs a world recipe (spec `superfargoth/docs/specs/worldgen.md`).
//!
//! ```text
//! scrib-worldgen run <recipe.toml> [--arms A0,A1] [--macro-spacing 25] [--iterations N] [--out DIR] [--no-images]
//! ```

use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Parser, Subcommand};
use sf_worldgen::run::{Overrides, run};

#[derive(Parser)]
#[command(
    name = "scrib-worldgen",
    about = "Runs Scrib world recipes (sf_worldgen)"
)]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Runs every arm of a recipe and writes heights, previews, sheets and README.md.
    Run {
        recipe: PathBuf,
        /// Only these arms (comma-separated names).
        #[arg(long, value_delimiter = ',')]
        arms: Option<Vec<String>>,
        /// Macro grid spacing override, metres.
        #[arg(long)]
        macro_spacing: Option<f64>,
        /// Stream power iteration override.
        #[arg(long)]
        iterations: Option<usize>,
        /// Output folder override.
        #[arg(long)]
        out: Option<PathBuf>,
        /// Skip PNG previews.
        #[arg(long)]
        no_images: bool,
    },
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match cli.cmd {
        Cmd::Run {
            recipe,
            arms,
            macro_spacing,
            iterations,
            out,
            no_images,
        } => {
            let ov = Overrides {
                arms,
                macro_spacing_m: macro_spacing,
                iterations,
                out,
                no_images,
            };
            match run(&recipe, &ov) {
                Ok(_) => ExitCode::SUCCESS,
                Err(e) => {
                    eprintln!("scrib-worldgen: {e}");
                    ExitCode::FAILURE
                }
            }
        }
    }
}
