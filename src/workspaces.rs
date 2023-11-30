use super::config::{Config, Workspace};
use std::env::current_dir;
use std::fs;
use std::time;

use ruscii::app::{App, State};
use ruscii::terminal::Window;
use ruscii::drawing::Pencil;
use ruscii::keyboard::{KeyEvent, Key};
use ruscii::spatial::Vec2;
use ruscii::terminal::Color;
use fuzzy_search::symspell::SymSpell;
use fuzzy_search::distance::levenshtein;


pub fn list_workspaces(config: &Config) {
    if config.workspaces.is_empty() {
        println!("No workspaces found.");
        return ();
    }

    // unwrap is safe because I've already checked for an empty vector, so
    // it will always have a value
    let label_size = config.workspaces.iter().map(|w| w.name.len()).max().unwrap() + 2;

    for workspace in config.workspaces.iter() {
        let label = format!("{:.<width$}", workspace.name, width=label_size);
        println!("{}{}", label, workspace.path);
    }
}

pub fn add_workspace(name: &str, config: &mut Config) {
    let current_path = current_dir().expect("Should have been able to read current directory");
    config.workspaces.push(
        Workspace {
            name: name.to_string(),
            path: current_path.to_str().unwrap().to_string() 
        }
    );

    config.write_to_file().expect("Should have been able to write to config file");
    println!("Workspace added: {} ({})", name, current_path.to_str().unwrap().to_string());
}

pub fn remove_workspace(name: &str, config: &mut Config) {
    let found_index = match config.workspaces.iter().position(|workspace| workspace.name == name) {
        Some(index) => index,
        None => {
            println!("Workspace '{}' not found.", &name);
            return ()
        }
    };

    config.workspaces.remove(found_index);
    config.write_to_file().expect("Should have been able to write to config file");
    println!("Workspace removed: {}", name);
}

pub fn search(config: &Config) {
    let mut app = App::default();
    let mut input = String::new();
    let mut result = String::new();

    let mut sym = SymSpell::new(levenshtein, 100);
    for workspace in config.workspaces.iter() {
        sym.insert(workspace.name.clone());
    }

    let mut last_results = sym.fuzzy_search(&input);
    let mut current_choice = 0;

    // Check the timing of the first "Enter" input to fix an issue with
    // windows. See https://github.com/crossterm-rs/crossterm/issues/752
    let startup_time = get_milliseconds();

    app.run(|app_state: &mut State, window: &mut Window| {
        for key_event in app_state.keyboard().last_key_events() {
            let mut last_character = '\0';
            let mut remove_last_char = false;
            match key_event {
                KeyEvent::Pressed(Key::Esc) => {
                    app_state.stop();
                },
                KeyEvent::Released(Key::Enter) => {
                    if get_milliseconds() - startup_time < 500 {
                        break;
                    }

                    if let Some(w_name) = last_results.get(current_choice) {
                        if let Some(workspace) = config.workspaces.iter().find(|workspace| workspace.name == *w_name) {
                            result = workspace.path.clone();
                        }
                    }
                    app_state.stop();
                },
                KeyEvent::Pressed(Key::A) => last_character = 'a',
                KeyEvent::Pressed(Key::B) => last_character = 'b',
                KeyEvent::Pressed(Key::C) => last_character = 'c',
                KeyEvent::Pressed(Key::D) => last_character = 'd',
                KeyEvent::Pressed(Key::E) => last_character = 'e',
                KeyEvent::Pressed(Key::F) => last_character = 'f',
                KeyEvent::Pressed(Key::G) => last_character = 'g',
                KeyEvent::Pressed(Key::H) => last_character = 'h',
                KeyEvent::Pressed(Key::I) => last_character = 'i',
                KeyEvent::Pressed(Key::J) => last_character = 'j',
                KeyEvent::Pressed(Key::K) => last_character = 'k',
                KeyEvent::Pressed(Key::L) => last_character = 'l',
                KeyEvent::Pressed(Key::M) => last_character = 'm',
                KeyEvent::Pressed(Key::N) => last_character = 'n',
                KeyEvent::Pressed(Key::O) => last_character = 'o',
                KeyEvent::Pressed(Key::P) => last_character = 'p',
                KeyEvent::Pressed(Key::Q) => last_character = 'q',
                KeyEvent::Pressed(Key::R) => last_character = 'r',
                KeyEvent::Pressed(Key::S) => last_character = 's',
                KeyEvent::Pressed(Key::T) => last_character = 't',
                KeyEvent::Pressed(Key::U) => last_character = 'u',
                KeyEvent::Pressed(Key::V) => last_character = 'v',
                KeyEvent::Pressed(Key::W) => last_character = 'w',
                KeyEvent::Pressed(Key::X) => last_character = 'x',
                KeyEvent::Pressed(Key::Y) => last_character = 'y',
                KeyEvent::Pressed(Key::Z) => last_character = 'z',
                KeyEvent::Pressed(Key::Space) => last_character = ' ',
                KeyEvent::Pressed(Key::Backspace) => remove_last_char = true,
                KeyEvent::Pressed(Key::Up) => {
                    if current_choice > 0 {
                        current_choice -= 1;
                    }
                },
                KeyEvent::Pressed(Key::Down) => {
                    if current_choice < last_results.len() - 1 {
                        current_choice += 1;
                    }
                }
                _ => {}
            }

            if remove_last_char && input.len() > 0 {
                input.pop();
            }

            if last_character != '\0' {
                input.push(last_character);
                current_choice = 0;
            }

            last_results = sym.fuzzy_search(&input);
        }

        // Draw input and ordered workspace list
        let mut pencil = Pencil::new(window.canvas_mut());
        let input_text = format!("> {}", &input);
        pencil.draw_text(&input_text, Vec2::xy(1,1));
        for (i, res) in last_results.iter().enumerate() {
            if i == current_choice {
                pencil.set_foreground(Color::Yellow);
            }
            pencil.draw_text(res, Vec2::xy(1, i as i32 + 2));
            if i == current_choice {
                pencil.set_foreground(Color::White);
            }
        }
    });

    // output the result
    if !result.is_empty() {
        fs::write(&config.result_file, &result).expect("Should have been able to write to result file.");
    }
}

// Utility to fix a double input problem with Windows
// See https://github.com/crossterm-rs/crossterm/issues/752
fn get_milliseconds() -> u128 {
    let start = time::SystemTime::now();
    let millis = start
        .duration_since(time::UNIX_EPOCH)
        .unwrap()
        .as_millis();

    return millis;
}
