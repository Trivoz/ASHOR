/*
 * Copyright 2023 Joshua Rose
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
// #![allow(unused)]
#![allow(non_snake_case)]

mod path;
mod shell;

use std::env::{self};
use std::fs;
use std::path::Path;
use std::process::Command;
use std::io::{self, Write, stdin};

use env_logger::Env;
use shell::Shell;

/// The log level that is sought to when there is none found in the system path.
const DEFAULT_LOG_LEVEL: &'static str = "error";

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or(DEFAULT_LOG_LEVEL)).init();

    let shell = Shell::new();

    shell.stdout_debug_info();

    // see 91f7a78 for more implementation ideas
    let prefix = String::from(">>> ");

    'running: loop {
        // print the prefix, whatever that may be
        print!("{}", &prefix);

        // Update buffer before reading
        io::stdout().flush().unwrap();

        // create a stdin buffer
        let mut stdinput = String::new();

        // add input into stdin buffer
        stdin().read_line(&mut stdinput).unwrap();

        // Derive components from stdin
        let mut components = stdinput.trim().split_whitespace();

        // Derive the base component
        let base = components.next().unwrap_or("");

        match base {
            "cd" => {
                // default to '/' as new directory if one was not provided
                let new_dir = components.peekable().peek().map_or("/", |x| *x);
                let root = Path::new(new_dir);
                if let Err(e) = env::set_current_dir(&root) {
                    eprintln!("{}", e);
                }
            },
            "ls" => {
                // ls: iterate through files
                //
                // ref: https://doc.rust-lang.org/std/fs/struct.DirEntry.html
                for entry in fs::read_dir(".").unwrap() {
                    let dir = entry.unwrap();
                    println!("{:?}", dir.path());
                }
            },
            "quit" | "exit" => {
                break 'running
            },
            // if a buffer loads into itself
            base => {
                // Execute a new instance of the command in base
                let child = Command::new(base).
                    args(components)
                    .spawn();

                // handle malformed input
                let _ = match child {
                    Ok(mut child) => {
                        // await value
                        child.wait()
                    },
                    Err(_) => { 
                        continue
                    }
                };

            }
        }
    }
}
