mod terminal;

use crate::terminal::{ModuleType, Terminal};
use anyhow::Result;
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() -> Result<()> {
    let terminal = Terminal::new();
    let root_name = terminal.determine_root_folder_name()?;
    let resolver_version = determine_resolver_version();
    create_root_cargo_toml(&root_name, resolver_version)?;
    let modules_to_create = terminal.determine_modules_for_workspace()?;

    for module in &modules_to_create {
        let module_type_flag = match module.module_type {
            ModuleType::Lib => "--lib",
            ModuleType::Bin => "--bin",
        };
        let p = Path::new(&root_name).join(&module.name);
        let full_path = p.to_str().expect("invalid module name");
        Command::new("cargo")
            .args(["new", module_type_flag, full_path, "--vcs", "none"])
            .status()?;
    }
    println!("Workspace created: '{root_name}'");
    match init_git_repo(&root_name) {
        Ok(_) => {
            println!("Initialized Git-Repository in: '{root_name}'")
        }
        Err(_) => {
            println!("Could not initialize git repository in: '{root_name}'")
        }
    }
    Ok(())
}

fn create_root_cargo_toml(root_name: &str, resolver_version: i32) -> Result<()> {
    fs::create_dir(root_name)?;
    let path = format!("{root_name}/Cargo.toml");
    fs::write(
        &path,
        format!("[workspace]\nresolver = \"{resolver_version}\"\n"),
    )?;

    Ok(())
}

fn determine_resolver_version() -> i32 {
    let resolver_version = map_rust_version_to_resolver_version();
    println!("Using resolver version '{resolver_version}'");
    resolver_version
}
fn map_rust_version_to_resolver_version() -> i32 {
    let pre_2021 = "1.55.0";
    let v2021 = "1.56.0";
    let v2024 = "1.85.0";

    let is_2024 = version_check::is_min_version(v2024).unwrap_or(false);
    let is_2021 = version_check::is_min_version(v2021).unwrap_or(false);
    let is_pre_2021 = version_check::is_min_version(pre_2021).unwrap_or(false);

    if is_2024 || is_2021 {
        2
    } else if is_pre_2021 {
        1
    } else {
        println!("Could not determine Rust version -  defaulting resolver to '2'");
        2
    }
}

fn init_git_repo(root_folder: &str) -> Result<()> {
    Command::new("git").args(["init", root_folder]).status()?;
    add_gitignore(root_folder)?;
    Ok(())
}

fn add_gitignore(root_folder: &str) -> Result<()> {
    let gitignore_string = include_str!("resources/.gitignore_template");
    let gitignore_path = format!("./{root_folder}/.gitignore");
    let gitignore_path = Path::new(&gitignore_path);
    fs::write(gitignore_path, gitignore_string)?;
    Ok(())
}
