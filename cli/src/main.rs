use anyhow::{anyhow, Result};
use clap::Parser;
use log::debug;
use std::path::Path;
use x0001e::{project::Project, vm::VM};

const BINARY_NAME: &str = "x0001e";

mod args;
mod build;

use args::Args;
use build::{
    generate_build_command, get_extra_filename, get_latest_bc, Features, Settings, Target,
};

fn main() -> Result<()> {
    env_logger::init();

    run()?;
    Ok(())
}

fn run() -> Result<()> {
    let mut args = std::env::args().collect::<Vec<_>>();
    debug!("recevied arguments: {:?}", args);

    // If this is run as a cargo subcommand, the second argument will be the name of this binary.
    // So remove this if this is the case.
    if args
        .get(1)
        .map(|s| s.as_str() == BINARY_NAME)
        .unwrap_or(false)
    {
        debug!(
            "used as cargo subcommand: removing {} as second argument",
            BINARY_NAME
        );
        args.remove(1);
    }

    let args = Args::parse_from(args);
    let opts = settings_from_args(&args);

    // Build LLVM BC file.
    let cargo_out = generate_build_command(&opts).output()?;
    debug!("cargo output: {:?}", cargo_out);

    // Create path to .bc file.
    let output = String::from_utf8(cargo_out.stderr)?;
    let extra_filename = get_extra_filename(&output)?;

    let target_dir = opts.get_target_dir()?;
    let target_name = opts.get_target_name()?;

    let target_path = if let Some(extra) = extra_filename {
        let filename = format!("{}{}.bc", target_name, extra);
        target_dir.join(filename)
    } else {
        let name = get_latest_bc(&target_dir, &target_name)?;
        name.ok_or_else(|| anyhow!("Could not find .bc for {}", target_name))?
    };
    debug!("Target .bc path: {:?}", target_path);

    let fn_name = match args.function {
        None => "main".to_owned(),
        Some(name) => name,
    };
    let fn_name = format!("{}::{}", opts.get_module_name()?, fn_name);
    debug!(
        "Starting analysis on target: {:?}, function: {}",
        target_path, fn_name
    );
    run_analysis(&target_path, &fn_name)
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
    } else if opts.features.len() == 0 {
        Features::None
    } else {
        Features::Some(opts.features.clone())
    };

    Settings {
        target,
        features,
        release: opts.release,
    }
}

fn run_analysis(path: impl AsRef<Path>, fn_name: &str) -> Result<()> {
    let project = Project::from_bc_path(path)?;
    let vm = VM::new(fn_name, &project)?;

    let results = vm.into_iter().collect::<Vec<_>>();
    println!("Results: {:#?}", results);
    Ok(())
}
