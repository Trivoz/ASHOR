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

use env_logger::Env;
use shell::Shell;

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();

    let shell = Shell::new();
    shell.stdout_debug_info();
}
