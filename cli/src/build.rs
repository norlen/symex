use anyhow::{anyhow, Result};
use cargo_project::{Artifact, Profile, Project};
use log::debug;
use regex::Regex;
use std::env;
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

/// Build settings.
pub struct Settings {
    /// Target to use, e.g. binary, example or lib.
    pub target: Target,

    /// The features to activate, if any.
    pub features: Features,

    /// Build in release.
    pub release: bool,
}

impl Settings {
    /// Return the path to the target directory for the project.
    pub fn get_target_dir(&self) -> Result<PathBuf> {
        let artifact = match &self.target {
            Target::Bin(name) => Artifact::Bin(name),
            Target::Example(name) => Artifact::Example(name),
            Target::Lib => Artifact::Lib,
        };

        let profile = match self.release {
            true => Profile::Release,
            false => Profile::Dev,
        };

        let cwd = env::current_dir()?;
        let meta = rustc_version::version_meta()?;
        let project = Project::query(cwd).map_err(|err| anyhow!(err.to_string()))?;
        let host = meta.host;
        let mut path = project
            .path(artifact, profile, None, &host)
            .map_err(|err| anyhow::anyhow!("{err}"))?;
        path = path.parent().expect("unreachable").to_path_buf();

        // Binary files --bin are in the `/deps` folder.
        if matches!(artifact, Artifact::Bin(_)) {
            path = path.join("deps");
        }

        Ok(path)
    }

    /// Returns the name of the built target.
    pub fn get_target_name(&self) -> Result<String> {
        let cwd = env::current_dir()?;
        let project = Project::query(cwd).map_err(|err| anyhow!(err.to_string()))?;

        let name = match &self.target {
            Target::Bin(name) => name.clone(),
            Target::Example(name) => name.clone(),
            Target::Lib => project.name().to_string(),
        };

        // Target names have `-` replaced with `_`.
        let name = name.replace('-', "_");
        Ok(name)
    }

    /// Returns the name of the module.
    pub fn get_module_name(&self) -> Result<String> {
        match &self.target {
            Target::Bin(_) | Target::Lib => {
                let cwd = env::current_dir()?;
                let project = Project::query(cwd).map_err(|err| anyhow!(err.to_string()))?;

                Ok(project.name().to_string())
            }
            Target::Example(name) => Ok(name.clone()),
        }
    }
}

pub enum Target {
    Bin(String),
    Example(String),
    Lib,
}

pub enum Features {
    None,
    Some(Vec<String>),
    All,
}

pub fn generate_build_command(opts: &Settings) -> Command {
    let mut cargo = Command::new("cargo");
    cargo.args(&["rustc", "--verbose", "--color=never"]);

    match &opts.features {
        Features::None => {}
        Features::Some(features) => {
            cargo.args(&["--features", &features.join(",")]);
        }
        Features::All => {
            cargo.arg("--all-features");
        }
    };

    match &opts.target {
        Target::Bin(name) => cargo.args(&["--bin", name]),
        Target::Example(name) => cargo.args(&["--example", name]),
        Target::Lib => cargo.arg("--lib"),
    };

    if opts.release {
        cargo.arg("--release");
    }

    // Commands to pass to rustc.
    cargo.args(&[
        "--",
        "--emit=llvm-bc",
        "--emit=llvm-ir",
        "-C",
        "link-dead-code=yes",
    ]);
    debug!("Running cargo command: {cargo:?}");
    cargo
}

/// Retrieves the hash appended to the build output from the compilation step.
pub fn get_extra_filename(output: &str) -> Result<Option<String>> {
    fn get_filename(re: Regex, output: &str) -> Option<String> {
        let captures = re.captures_iter(output).last()?;
        let extra_filename = captures.get(0)?;
        let extra_filename = extra_filename.as_str();

        let start_capture = extra_filename.find('=')?;
        let extra_filename: String = extra_filename.chars().skip(start_capture + 1).collect();
        Some(extra_filename)
    }
    let re = Regex::new(r#"-C extra-filename=(-\S+)"#)?;
    Ok(get_filename(re, output))
}

pub fn get_latest_bc(dir: impl AsRef<Path>, file_prefix: &str) -> Result<Option<PathBuf>> {
    let mut result = None;
    let mut last_modified = None;

    for entry in fs::read_dir(dir)? {
        let path = entry?.path();

        // Skip if file does not end with .bc
        let ends_in_bc = path.extension().map(|ext| ext == "bc").unwrap_or(false);
        if !ends_in_bc {
            continue;
        }

        // Skip if file does not start with prefix.
        let starts_with = match path.file_stem() {
            None => false,
            Some(stem) => stem
                .to_str()
                .map(|stem| stem.starts_with(file_prefix))
                .unwrap_or(false),
        };
        if !starts_with {
            continue;
        }

        let modified = path.metadata()?.modified()?;
        let update_result = last_modified
            .map(|last_modified| modified > last_modified)
            .unwrap_or(true);

        if update_result {
            result = Some(path);
            last_modified = Some(modified);
        }
    }

    Ok(result)
}
