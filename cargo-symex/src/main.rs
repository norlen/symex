use anyhow::{anyhow, Result};
use clap::Parser;
use log::debug;
use std::{fs, path::PathBuf};
use symex::run::RunConfig;
use tracing_subscriber;

const BINARY_NAME: &str = "symex";

mod args;
mod build;
mod build_c;

use args::{Args, ClangArgs};
use build::{
    generate_build_command, get_extra_filename, get_latest_bc, Features, Settings, Target,
};

use crate::args::Subcommands;

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    match run() {
        Ok(_) => {}
        Err(err) => {
            eprintln!("{err}");
        }
    }
    Ok(())
}

fn run() -> Result<()> {
    let mut args = std::env::args().collect::<Vec<_>>();
    debug!("received arguments: {args:?}");

    // If this is run as a cargo subcommand, the second argument will be the name of this binary.
    // So remove this if this is the case.
    if args
        .get(1)
        .map(|s| s.as_str() == BINARY_NAME)
        .unwrap_or(false)
    {
        debug!("used as cargo subcommand: removing {BINARY_NAME} as second argument");
        args.remove(1);
    }

    let args = Args::parse_from(args);

    match args.subcommand {
        Some(subcommand) => match subcommand {
            Subcommands::C(clang_args) => run_c(clang_args),
        },
        None => run_rs(args),
    }
}

fn run_rs(args: Args) -> Result<()> {
    let opts = settings_from_args(&args);

    // Build LLVM BC file.
    let cargo_out = generate_build_command(&opts).output()?;
    debug!("cargo output: {cargo_out:?}");
    if !cargo_out.status.success() {
        let cargo_output = String::from_utf8(cargo_out.stderr)?;
        return Err(anyhow!(cargo_output));
    }
    let output = String::from_utf8(cargo_out.stderr)?;
    if !output.is_empty() {
        eprintln!("{output}");
    }

    // Create path to .bc file.
    let extra_filename = get_extra_filename(&output)?;

    let target_dir = opts.get_target_dir()?;
    let target_name = opts.get_target_name()?;

    let target_path = if let Some(extra) = extra_filename {
        let filename = format!("{target_name}{extra}.bc");
        target_dir.join(filename)
    } else {
        let name = get_latest_bc(&target_dir, &target_name)?;
        name.ok_or_else(|| anyhow!("Could not find .bc for {target_name}"))?
    };
    debug!("Target .bc path: {target_path:?}");

    // Get function name and analyze code.
    let fn_name = match args.function {
        None => "main".to_owned(),
        Some(name) => name,
    };
    let fn_name = format!("{}::{fn_name}", opts.get_module_name()?);
    debug!("Starting analysis on target: {target_path:?}, function: {fn_name}");

    let cfg = RunConfig {
        solve_inputs: true,
        solve_symbolics: true,
        solve_output: true,
        solve_for: symex::run::SolveFor::All,
    };

    symex::run::run(&target_path, &fn_name, &cfg)?;
    Ok(())
}

fn settings_from_args(opts: &Args) -> Settings {
    let target = if let Some(name) = &opts.bin {
        Target::Bin(name.clone())
    } else if let Some(name) = &opts.example {
        Target::Example(name.clone())
    } else {
        Target::Lib
    };

    let features = if opts.all_features {
        Features::All
    } else if opts.features.is_empty() {
        Features::None
    } else {
        Features::Some(opts.features.clone())
    };

    Settings {
        target,
        features,
        release: opts.release,
        embed_bitcode: opts.embed_bitcode.unwrap_or(false),
    }
}

fn run_c(args: ClangArgs) -> Result<()> {
    let opts = clang_settings_from_args(&args);

    // Create output directory
    let mut dir = opts.out_path.clone();
    dir.pop();
    fs::create_dir_all(dir)?;

    // Build .bc
    let clang_out = build_c::generate_build_command(&opts).output()?;
    debug!("clang output: {clang_out:?}");
    if !clang_out.status.success() {
        let clang_output = String::from_utf8(clang_out.stderr)?;
        return Err(anyhow!(clang_output));
    }

    // Get function name and analyze code.
    let fn_name = match args.function {
        None => "main".to_owned(),
        Some(name) => name,
    };
    debug!(
        "Starting analysis on target: {:?}, function: {fn_name}",
        opts.out_path
    );
    todo!()
    // runner::run(&opts.out_path, &fn_name)
}

fn clang_settings_from_args(opts: &ClangArgs) -> build_c::Settings {
    let mut out_path = PathBuf::from("target/c");
    out_path.push(opts.path.file_stem().unwrap());
    out_path.set_extension("bc");

    build_c::Settings {
        path: opts.path.clone(),
        out_path,
    }
}
