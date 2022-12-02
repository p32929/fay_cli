use miniserde::{json, Deserialize, Serialize};
use std::fs;
use std::io;
use std::io::Error;
use std::io::Write;
use std::process;
use std::process::Child;
use std::process::Command;
use std::process::Stdio;

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
struct FayData {
    commands: Vec<CommandData>,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
struct CommandData {
    name: String,
    execs: Vec<String>,
}
static FILEPATH: &str = "./faydata.json";

enum CommandTypes {
    CD,
    INPUT,
    DONE,
}

fn show_saved_commands(fay_data: &FayData) {
    if fay_data.commands.len() == 0 {
        println!("No saved commands");
    } else {
        for (pos, data) in fay_data.commands.iter().enumerate() {
            println!(">> {}. {} ", pos + 1, data.name);
        }
    }
}

fn get_saved_json_data() -> FayData {
    let dummy_json_values: FayData = FayData { commands: vec![] };
    let file_content = fs::read_to_string(FILEPATH).unwrap_or(String::from(""));
    let json_data = json::from_str::<FayData>(&file_content).unwrap_or(dummy_json_values);
    if file_content.is_empty() {
        save_json_file(&json_data);
    }
    json_data

    // let file_content = fs::read_to_string(FILEPATH);
    // let _json;
    // match file_content {
    //     Ok(file_content) => {
    //         match json::from_str::<FayData>(&file_content) {
    //             Ok(json_data) => {
    //                 _json = json_data;
    //             }
    //             Err(_) => {
    //                 save_json_file(&DUMMY_JSON);
    //                 return get_saved_json_data();
    //             }
    //         };
    //     }
    //     Err(_) => {
    //         save_json_file(&DUMMY_JSON);
    //         return get_saved_json_data();
    //     }
    // }
    // _json
}

fn save_json_file(fay_data: &FayData) {
    let json_str = json::to_string(fay_data);
    fs::write(FILEPATH, json_str).expect("Unable to write file");
    // println!(">> JSON WRITTEN <<");
}

fn add_option(json_data: &mut FayData) {
    println!("\n>>> Add a command <<<");
    println!("> Enter command name: ");
    let mut name_input = String::new();
    io::stdin().read_line(&mut name_input).unwrap();

    let command_name = name_input.trim_end();

    match command_name {
        "" => {
            println!(">>> Please enter a name <<<");
            add_option(json_data);
        }
        _ => {
            println!("\n>>> Enter series of commands <<<");
            println!("> Enter 0 to stop");
            let mut commands_array: Vec<String> = Default::default();
            loop {
                let mut command = String::new();
                io::stdin().read_line(&mut command).unwrap();

                match command.trim_end() {
                    "0" => break,
                    _ => {
                        let ss = String::from(command.trim_end());
                        commands_array.push(ss);
                    }
                }
            }
            json_data.commands.push(CommandData {
                name: String::from(command_name),
                execs: commands_array,
            });

            save_json_file(json_data);
            println!("New command added");
            println!("Restarting the CLI\n");
            main();
        }
    }
}

fn delete_option(json_data: &mut FayData) {
    println!("\n>> Delete a command <<");
    if json_data.commands.is_empty() {
        println!(">> No commands to delete <<");
        println!(">> Restarting the CLI <<\n");
        main();
    } else {
        println!("> Enter command number: ");

        let mut command_number_input = String::new();
        io::stdin().read_line(&mut command_number_input).unwrap();
        let command_number = command_number_input.trim_end();

        match command_number.to_lowercase().trim_end().parse::<u32>() {
            Ok(parsed_num) => {
                let len = json_data.commands.len() as u32;
                if parsed_num >= 1 && parsed_num <= len {
                    println!("\n>> Deleting command <<");
                    let index_to_remove = parsed_num - 1;
                    json_data.commands.remove(index_to_remove as usize);
                    println!(">> Command deleted <<");

                    save_json_file(json_data);
                    println!(">> Restarting the CLI <<\n");
                    main();
                } else {
                    println!(">> Invalid command number <<");
                    delete_option(json_data);
                }
            }
            Err(_) => {
                println!(">> Invalid command number <<");
                delete_option(json_data);
            }
        };
    }
}

fn edit_option(json_data: &mut FayData) {
    println!("\n>> Edit a command <<");
    if json_data.commands.is_empty() {
        println!(">> No commands to edit <<");
        println!(">> Restarting the CLI <<\n");
        main();
    } else {
        println!("> Enter command number: ");

        let mut command_number_input = String::new();
        io::stdin().read_line(&mut command_number_input).unwrap();
        let command_number = command_number_input.trim_end();

        match command_number.to_lowercase().trim_end().parse::<u32>() {
            Ok(parsed_num) => {
                let len = json_data.commands.len() as u32;
                if parsed_num >= 1 && parsed_num <= len {
                    println!("\n>> Editing command <<");
                    let index_to_edit = parsed_num - 1;
                    println!("> Enter new command name: ");
                    let mut new_command_name = String::new();
                    io::stdin().read_line(&mut new_command_name).unwrap();

                    println!("\n>>> Enter new series of commands <<<");
                    println!("> Enter 0 to stop");
                    let mut new_commands_array: Vec<String> = Default::default();
                    loop {
                        let mut command = String::new();
                        io::stdin().read_line(&mut command).unwrap();

                        match command.trim_end() {
                            "0" => break,
                            _ => {
                                let ss = String::from(command.trim_end());
                                new_commands_array.push(ss);
                            }
                        }
                    }

                    json_data.commands[index_to_edit as usize] = CommandData {
                        name: new_command_name,
                        execs: new_commands_array,
                    };

                    save_json_file(json_data);
                    println!(">> Command updated <<");
                    println!(">> Restarting the CLI <<\n");
                    main();
                } else {
                    println!(">> Invalid command number <<");
                    edit_option(json_data);
                }
            }
            Err(_) => {
                println!(">> Invalid command number <<");
                edit_option(json_data);
            }
        };
    }
}

// fn string_to_static_str(s: String) -> &'static str {
//     Box::leak(s.into_boxed_str())
// }

fn run_commands(commands: &CommandData) {
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
        println!("\n> {}", command);
        if command.starts_with("cd ") {
            dir = command.split(" ").last().unwrap();

            proc_command = Command::new(command_types.0);
            proc_command.arg(command_types.1);
            proc_command.current_dir(dir);

            just_inited_cmd = true;
        } else {
            if just_inited_cmd {
                proc_command.arg(command);
                just_inited_cmd = false;
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

fn start_command_selection(fay_data: &mut FayData) {
    let mut selected_option = String::new();
    io::stdin().read_line(&mut selected_option).unwrap();

    match selected_option.to_lowercase().trim_end() {
        "a" => add_option(fay_data),
        "d" => delete_option(fay_data),
        "e" => edit_option(fay_data),
        val => {
            let parsed_num = val.parse::<u32>().unwrap_or(u32::MIN);
            let len = fay_data.commands.len() as u32;
            if parsed_num >= 1 && parsed_num <= len {
                let index = parsed_num as usize;
                let command_data = &fay_data.commands[index - 1];
                println!(">>> Running commands in \"{}\" <<<", command_data.name);
                run_commands(command_data);
            } else {
                println!("\n>> Invalid command number <<");
                process::exit(404);
            }
        }
    }

    // match selected_option.to_lowercase().trim_end() {
    //     "a" => add_option(fay_data),
    //     "d" => delete_option(fay_data),
    //     "e" => edit_option(fay_data),
    //     val => {
    //         match val.parse::<u32>() {
    //             Ok(parsed_num) => {
    //                 let len = fay_data.commands.len() as u32;
    //                 if parsed_num >= 1 && parsed_num <= len {
    //                     let index = parsed_num as usize;
    //                     let command_data = &fay_data.commands[index - 1];
    //                     println!(">>> Running commands in \"{}\" <<<", command_data.name);
    //                     run_commands(command_data);
    //                 } else {
    //                     println!("\n>> Invalid command number <<");
    //                     start_command_selection(fay_data);
    //                 }
    //             }
    //             Err(_) => {
    //                 println!("\n>> Invalid command number <<");
    //                 start_command_selection(fay_data);
    //             }
    //         };
    //     }
    // }
}

fn main() {
    println!(":::::::::::::::::::");
    println!(">>>>>>  Fay  <<<<<<");
    println!(":::::::::::::::::::");
    println!("\n> Saved commands <");
    let mut fay_data: FayData = get_saved_json_data();
    show_saved_commands(&fay_data);
    println!("\n> Predefined options <");
    println!(">> a. Add a command");
    println!(">> d. Delete a command");
    println!(">> e. Edit a command\n> ");

    start_command_selection(&mut fay_data);
}
