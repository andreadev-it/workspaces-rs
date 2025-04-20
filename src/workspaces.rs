use super::config::{Config, Workspace};
use std::env::current_dir;
use std::fs;

use dialoguer::FuzzySelect;
use dialoguer::theme::ColorfulTheme;
use colored::Colorize;

pub fn list_workspaces(config: &Config) {
    if config.workspaces.is_empty() {
        println!("No workspaces found.");
        return;
    }

    // unwrap is safe because I've already checked for an empty vector, so
    // it will always have a value
    let label_size = config.workspaces.iter().map(|w| w.name.len()).max().unwrap() + 4;

    println!("{:<width$}{}", "Name:".bold(), "Path:".bold(), width=label_size);
    for workspace in config.workspaces.iter() {
        let pad = label_size - workspace.name.len();
        let label = format!("{}{:.<width$}", workspace.name, "".dimmed(), width=pad);
        println!("{}{}", label, workspace.path.yellow());
    }
}

pub fn add_workspace(name: &str, config: &mut Config) {
    let current_path = current_dir().expect("Should have been able to read current directory");

    let is_dup = config.workspaces
        .iter()
        .find(|w| w.name == name)
        .is_some();

    if is_dup {
        println!("There already is a workspace called \"{}\"", name);
        return;
    }

    config.workspaces.push(
        Workspace {
            name: name.to_string(),
            path: current_path.to_str().unwrap().to_string() 
        }
    );

    config.write_to_file().expect("Should have been able to write to config file");
    println!("Workspace added: {} ({})", name.bold(), current_path.to_str().unwrap().yellow());
}

pub fn remove_workspace(name: &str, config: &mut Config) {
    let found_index = match config.workspaces.iter().position(|workspace| workspace.name == name) {
        Some(index) => index,
        None => {
            println!("Workspace '{}' not found.", &name.bold());
            return;
        }
    };

    config.workspaces.remove(found_index);
    config.write_to_file().expect("Should have been able to write to config file");
    println!("Workspace removed: {}", name.bold());
}

pub fn search(config: &Config) {
    let workspaces_names = config.workspaces
        .iter()
        .map(|w| w.name.to_string())
        .collect::<Vec<String>>();

    let selected_opt = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select workspace")
        .default(0)
        .items(&workspaces_names)
        .interact_opt()
        .unwrap();
    
    if let Some(selected) = selected_opt {
        let name = workspaces_names[selected].to_string();

        // we are sure that the workspace exists
        let workspace = config.workspaces
            .iter()
            .find(|w| w.name == name)
            .unwrap();
        
        // output the result
        fs::write(&config.result_file, &workspace.path).expect("Should have been able to write to result file.");
    }
}
