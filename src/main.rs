use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::io::Write;
use std::process::Command;

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
struct Cmder {
    commands: Vec<CommandData>,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
struct CommandData {
    name: String,
    execs: Vec<String>,
}

static DUMMY_JSON: Cmder = Cmder { commands: vec![] };
static FILEPATH: &str = "./cmderdata.json";

fn show_saved_commands(json_data: &Cmder) {
    if json_data.commands.len() == 0 {
        println!("No saved commands");
    } else {
        for (pos, data) in json_data.commands.iter().enumerate() {
            println!("{}. {:}", pos + 1, data.name);
        }
    }
}

fn get_saved_json_data() -> Cmder {
    let file_content = fs::read_to_string(FILEPATH);
    let _json;
    match file_content {
        Ok(file_content) => {
            match serde_json::from_str::<Cmder>(&file_content) {
                Ok(json_data) => {
                    _json = json_data;
                }
                Err(_) => {
                    save_json_file(&DUMMY_JSON);
                    return get_saved_json_data();
                }
            };
        }
        Err(_) => {
            save_json_file(&DUMMY_JSON);
            return get_saved_json_data();
        }
    }
    _json
}

fn save_json_file(cmder: &Cmder) {
    let json_str = serde_json::to_string(cmder).unwrap();
    fs::write(FILEPATH, json_str).expect("Could not wr");
    println!("::: JSON WRITTEN :::");
}

fn get_user_selected_option() -> String {
    let mut option = String::new();
    io::stdin()
        .read_line(&mut option)
        .expect("Select an option");
    option
}

fn add_option(json_data: &mut Cmder) {
    println!("\n::: Add a command :::");
    println!("Enter command name: ");
    let mut name_input = String::new();
    io::stdin()
        .read_line(&mut name_input)
        .expect("Enter command name");

    let command_name = name_input.trim_end();

    match command_name {
        "" => {
            println!("*** Please enter a name ***");
            add_option(json_data);
        }
        _ => {
            println!("\nEnter series of commands to run. Enter 0 to stop");
            let mut command_array: Vec<String> = Default::default();
            loop {
                let mut command = String::new();
                io::stdin()
                    .read_line(&mut command)
                    .expect("Enter command name");

                match command.trim_end() {
                    "0" => break,
                    _ => {
                        let ss = String::from(command.trim_end());
                        command_array.push(ss);
                    }
                }
            }
            json_data.commands.push(CommandData {
                name: String::from(command_name),
                execs: command_array,
            });

            save_json_file(json_data);
            println!("New command added");
            println!("Restarting the CLI\n");
            main();
        }
    }
}

fn delete_option() {
    println!("\n::: Delete a command :::");
}

fn edit_option() {
    println!("\n::: Edit a command :::");
}

// fn string_to_static_str(s: String) -> &'static str {
//     Box::leak(s.into_boxed_str())
// }

// fn get_new_command() -> &Command {
//     let command = Command::new("cmd");
//     command.arg("/C");
//     &command
// }

fn run_commands(commands: &CommandData) {
    println!("Current: {}", std::env::consts::OS);
    let windows_os = "windows";
    let command_types = {
        if windows_os == std::env::consts::OS {
            ("cmd", "/C")
        } else {
            ("sh", "-c")
        }
    };

    let mut proc_command = Command::new(command_types.0);
    proc_command.arg(command_types.1);

    let mut just_inited_cmd = false;
    let mut dir = "";

    for command in &commands.execs {
        if command.starts_with("cd ") {
            dir = command.split(" ").last().unwrap();

            proc_command = Command::new(command_types.0);
            proc_command.arg(command_types.1);
            proc_command.current_dir(dir);

            just_inited_cmd = true;
        } else {
            if just_inited_cmd {
                proc_command.arg(command);
            } else {
                proc_command = Command::new(command_types.0);
                proc_command.arg(command_types.1);
                if dir != "" {
                    proc_command.current_dir(dir);
                }
                proc_command.arg(command);
            }

            let child = proc_command.spawn();
            match child {
                Ok(mut child) => {
                    if let Err(error) = child.wait() {
                        eprintln!("{}", error);
                    }
                }
                Err(error) => eprintln!("{}", error),
            }
        }
    }
}

fn start_command_selection(json_data: &mut Cmder) {
    let option = get_user_selected_option();
    match option.to_lowercase().trim_end() {
        "a" => add_option(json_data),
        "d" => delete_option(),
        "e" => edit_option(),
        val => {
            match val.parse::<u32>() {
                Ok(parsed_num) => {
                    let len = json_data.commands.len() as u32;
                    if parsed_num >= 1 && parsed_num <= len {
                        let index = parsed_num as usize;
                        let command_data = &json_data.commands[index - 1];
                        println!("Running commands in {}", command_data.name);
                        run_commands(command_data);
                    } else {
                        println!("\nError! Please enter an available command");
                        start_command_selection(json_data);
                    }
                }
                Err(_) => {
                    println!("Error! Please enter an available command");
                    start_command_selection(json_data);
                }
            };
        }
    }
}

fn main() {
    println!(":::::::::::::::::::");
    println!(":::::: CMDER ::::::");
    println!(":::::::::::::::::::");
    println!("\n::: Run a saved command :::");
    let mut json_data: Cmder = get_saved_json_data();
    show_saved_commands(&json_data);
    println!("\n::: Select a predefined options :::");
    println!("a. Add a command");
    println!("d. Delete a command");
    println!("e. Edit a command");

    start_command_selection(&mut json_data);

    // println!("{}", std::env::consts::OS);
    // let windows_os = "windows";
    // let command_types = {
    //     if windows_os == std::env::consts::OS {
    //         ("cmd", "/C")
    //     } else {
    //         ("sh", "-c")
    //     }
    // };
    // println!("{:?}", command_types.0);
}
