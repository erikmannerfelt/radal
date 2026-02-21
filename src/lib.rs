//! # radal  --- Speeding up Ground Penetrating Radar (GPR) processing
//! A Ground Penetrating Radar (GPR) processing tool written in rust.
//!
//! **This is a WIP.**
//!
//! The main aims of `radal` are:
//! - **Ease of use**: A command line interface to process data or batches of data in one command.
//! - **Transparency**: All code is (or will be) thoroughly documented to show exactly how the data are modified.
//! - **Low memory usage and high speed**: While data are processed in-memory, they are usually no larger than an image (say 4000x2000 px). The functions of `radal` avoid copying as much as possible, to keep memory usage to a minimum. Wherever possible, it is also multithreaded for fast processing times.
//! - **Reliability**: All functions will be tested in CI, meaning no crash or invalid behaviour should occur.
//!
#[cfg(feature = "python")]
use pyo3::prelude::*;

mod cli;
mod coords;
mod dem;
mod filters;
mod gpr;
mod io;
mod tools;

#[cfg(feature = "python")]
const PROGRAM_VERSION: &str = env!("CARGO_PKG_VERSION");
#[cfg(feature = "python")]
const PROGRAM_NAME: &str = env!("CARGO_PKG_NAME");
#[cfg(feature = "python")]
const PROGRAM_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

#[cfg(feature = "python")]
#[pymodule]
pub mod radal {
    use crate::{cli, gpr};
    use pyo3::prelude::*;
    use std::path::PathBuf;

    #[pymodule_init]
    fn init(m: &Bound<'_, PyModule>) -> PyResult<()> {
        m.add("version", crate::PROGRAM_VERSION)?;
        m.add("__version__", crate::PROGRAM_VERSION)
    }

    #[pyfunction]
    #[pyo3(
        signature = (
            filepath=None,
            velocity=0.168,
            info=false,
            cor=None,
            dem=None,
            crs=None,
            track=None,
            default=false,
            default_with_topo=false,
            show_default=false,
            show_all_steps=false,
            steps=None,
            output=None,
            quiet=false,
            render=None,
            no_export=false,
            merge=None,
        )
    )]
    fn run_cli(
        filepath: Option<String>,
        velocity: f32,
        info: bool,
        cor: Option<PathBuf>,
        dem: Option<PathBuf>,
        crs: Option<String>,
        track: Option<PathBuf>,
        default: bool,
        default_with_topo: bool,
        show_default: bool,
        show_all_steps: bool,
        steps: Option<Vec<String>>,
        output: Option<PathBuf>,
        quiet: bool,
        render: Option<PathBuf>,
        no_export: bool,
        merge: Option<String>,
        _py: Python<'_>,
    ) -> PyResult<i32> {
        let track_opt: Option<Option<PathBuf>> = match track {
            Some(s) => Some(Some(PathBuf::from(s))),
            None => None,
        };

        // render: CLI uses Option<Option<PathBuf>>
        let render_opt: Option<Option<PathBuf>> = match render {
            Some(s) => Some(Some(PathBuf::from(s))),
            None => None,
        };
        // Construct the same Args struct the CLI uses
        let args = cli::Args {
            filepath,
            velocity,
            info,
            cor,
            dem,
            crs,
            track: track_opt,
            default,
            default_with_topo,
            show_default,
            show_all_steps,
            steps: steps.and_then(|s| Some(s.join(","))),
            output,
            quiet,
            render: render_opt,
            no_export,
            merge,
        };

        // Use the shared core logic
        match cli::args_to_action(&args) {
            cli::CliAction::Run(params) => {
                // run the core processing
                match gpr::run(params) {
                    Ok(_) => Ok(0),
                    Err(e) => Err(pyo3::exceptions::PyRuntimeError::new_err(format!("{e:?}"))),
                }
            }
            cli::CliAction::Done => Ok(0),
            cli::CliAction::Error(msg) => Err(pyo3::exceptions::PyValueError::new_err(msg)),
        }
    }
}
