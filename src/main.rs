// TODO: Remove unused attribute
#![allow(unused)]

use serde::Deserialize;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::{exit, Command};

#[derive(Debug, Deserialize)]
struct GitUser {
    name: Option<String>,
    email: Option<String>,
    signingkey: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GitConfig {
    user: GitUser,
}

#[derive(Debug, Deserialize)]
struct Profile {
    gitdir: Option<String>,
    path: String,
    content: GitConfig,
}

#[derive(Debug, Deserialize)]
struct Config {
    include: Option<Profile>,
    // TODO: Use includeIf instead of include_if
    include_if: Vec<Profile>,
}

fn main() {
    let command = "git";

    if !command_exists(command) {
        println!("{} command not found", command);
        exit(1);
    }

    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Usage: git-config-manager <config file>");
        exit(1);
    }

    let config_file = &args[1];

    let config = match std::fs::read_to_string(config_file) {
        Ok(config) => config,
        Err(e) => {
            println!("Error reading config file: {}", e);
            exit(1);
        }
    };

    let json = match serde_json::from_str::<Config>(&config) {
        Ok(json) => json,
        Err(e) => {
            println!("Error parsing config file: {}", e);
            exit(1);
        }
    };

    if let Some(include) = json.include {
        let include_path = get_absolute_path(&include.path);

        write_file(
            include_path.as_str(),
            &build_git_user_config_content(include.content),
        );

        let output = Command::new("git")
            .arg("config")
            .arg("--global")
            .arg("--add")
            .arg("include.path")
            .arg(&include.path)
            .output()
            .expect("Failed to execute git command");

        if !output.status.success() {
            println!("Error executing git command");
            exit(1);
        }
    }

    for include_if in json.include_if {
        if let Some(gitdir) = include_if.gitdir {
            let gitdir_path = Path::new(&gitdir);
            let include_if_path = get_absolute_path(&include_if.path);

            write_file(
                include_if_path.as_str(),
                &build_git_user_config_content(include_if.content),
            );

            // TODO: Figure out how to replace instead of adding.
            let output = Command::new("git")
                .arg("config")
                .arg("--global")
                .arg("--add")
                .arg(&format!("includeIf.gitdir:{}.path", gitdir_path.display()))
                .arg(&include_if.path)
                .output()
                .expect("Failed to execute git command");

            if !output.status.success() {
                println!("Error executing git command");
                exit(1);
            }
        }
    }
}

// TODO
fn create_git_config_file(path: &str, content: &str) {}

// TODO
fn edit_global_config(content: &str) {}

// TODO: Use gix instead of git
// https://github.com/Byron/gitoxide/tree/main/gix-credentials
fn command_exists(cmd: &str) -> bool {
    let status = Command::new("which")
        .arg(cmd)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .expect("Failed to execute which command");

    status.success()
}

fn build_git_user_config_content(content: GitConfig) -> String {
    let mut output = String::from("[user]\n");

    if let Some(name) = content.user.name {
        output.push_str(&format!("\tname = \"{}\"\n", name));
    }

    if let Some(email) = content.user.email {
        output.push_str(&format!("\temail = \"{}\"\n", email));
    }

    if let Some(signingkey) = content.user.signingkey {
        output.push_str(&format!("\tsigningkey = \"{}\"\n", signingkey));
    }

    output
}

fn create_folder_if_not_exists(path: &str) -> io::Result<()> {
    let folder_path = Path::new(path).parent().unwrap();

    if !folder_path.exists() {
        fs::create_dir_all(folder_path)?;
    }

    Ok(())
}

fn write_file(path: &str, content: &str) {
    if let Err(e) = create_folder_if_not_exists(path) {
        println!("Error creating folder: {}", e);
        std::process::exit(1);
    }

    let mut file = match std::fs::File::create(path) {
        Ok(file) => file,
        Err(e) => {
            println!("Error creating file: {}", e);
            exit(1);
        }
    };

    match file.write_all(content.as_bytes()) {
        Ok(_) => (),
        Err(e) => {
            println!("Error writing to file: {}", e);
            exit(1);
        }
    }
}

fn get_absolute_path(relative_path: &str) -> String {
    let expanded = shellexpand::tilde(relative_path).into_owned();
    let absolute_path = PathBuf::from(expanded);

    absolute_path.to_str().unwrap().to_string()
}
