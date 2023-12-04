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

use std::fs;
use std::io::{self, Write};

use env_logger::Env;
use shell::Shell;

/// The log level that is sought to when there is none found in the system path.
const DEFAULT_LOG_LEVEL: &'static str = "error";

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("error")).init();

    let shell = Shell::new();

    shell.stdout_debug_info();

    // see 91f7a78 for more implementation ideas
    let prefix = String::from(">>> ");

    'running: loop {
        // holds whatever the user should choose to type in
        // (right now its an empty buffer)
        let mut buffer = String::new();

        // create an input buffer
        let stdin = io::stdin();

        // Ensure the input buffer contents is loaded before reading into the string buffer
        io::stdout().flush().unwrap();

        // copy the input buffer into the empty string buffer
        stdin.read_line(&mut buffer).unwrap();

        match buffer.as_str() {
            "clear" => {
                unimplemented!()
            }
            "help" => {
                unimplemented!()
            }
            "ashor" => {
                unimplemented!()
            }
            "ls" => {
                unimplemented!()
            }
            _ => {
                // print out a new line
                print!("{}", prefix);
            }
        }

        // Once each case has been handled, clear the string inside the buffer
        buffer.clear();
    }
}
