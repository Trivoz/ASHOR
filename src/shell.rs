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
use log::{info, error};
use std::error::Error;
use std::process::{Child, Command};

use crate::path::invoke_output;

use std::path;

/// Invoke a system command
///
/// # Arguments
///
/// * name - the name of the command to call as one might do when inside a terminal
/// * arguments - an optional vector of String arguments to tac along to `name`
///
///
/// # Returns
///
/// A result type that either takes [`std::process::Child`] or [`dyn std::error::Error`]
/// contained in a [`Box`]
///
/// ## Resources
///
/// * [invoke-a-system-command](https://stackoverflow.com/questions/21011330/how-do-i-invoke-a-system-command-and-capture-its-output)
/// * [The `Command` type](https://doc.rust-lang.org/std/process/struct.Command.html)
pub fn invoke_system_command(
    name: &str,
    arguments: Option<&Vec<String>>,
) -> Result<Child, Box<dyn Error>> {
    match Command::new(name)
        .args(arguments.unwrap_or(&Vec::new()))
        .spawn()
    {
        Ok(cout) => Ok(cout),
        Err(e) => {
            error!("Failed to call {}", name);
            Err(Box::new(e))
        }
    }
}

pub struct Shell {
    /// Is the shell the default shell?
    pub is_default: bool,
    /// Has system config in /etc/ashor/ashor.toml
    pub has_system_config: bool,
    /// Has user config in $XDG_CONFIG_HOME/ashor/ashor.toml
    pub has_user_config: bool,
    // TODO: this.
    // /// System variables in the path
    // // TODO somehow make system_variables impl Sized
    // // system_variables: Vec<dyn Variable>
}

impl Shell {
    /// The user configuation path wherein the configuration file is stored
    // TODO: make ashor.toml just config.toml instead (globally for user and system)
    const USER_CONFIG_PATH: &'static str = "~/.config/ashor/ashor.toml";
    const SYSTEM_CONFIG_PATH: &'static str = "/etc/ashor/ashor.toml";

    /// Returns if 'ashor' is the default shell
    fn is_default() -> bool {
        let mut get_shell_command = String::from("SHELL");
        invoke_output(&mut get_shell_command).is_some()
    }

    /// Show debugging information in the console
    ///
    /// Note: these are called with the info! macro, also this function is public
    /// because its intended to be accessed from other files as a class method
    pub fn stdout_debug_info(&self) -> () {
        info!("default_shell: {}", &self.is_default);
        info!("user config: {}", &self.has_user_config);
        info!("system config: {}", &self.has_system_config);
    }

    /// Returns if the user config file exists
    fn user_config_exists() -> bool {
        let user_path = path::Path::new(Self::USER_CONFIG_PATH);
        user_path.exists()
    }

    /// Returns if the system config file exists
    fn system_config_exists() -> bool {
        let system_path = path::Path::new(Self::SYSTEM_CONFIG_PATH);
        system_path.exists()
    }

    /// Create an instance of [`Shell`]
    ///
    /// # Example
    ///
    /// ```rust,no_test
    /// let shell = Shell::new();
    /// ```
    pub fn new() -> Self {
        Self {
            is_default: Self::is_default(),
            has_user_config: Self::user_config_exists(),
            has_system_config: Self::system_config_exists()
        }
    }
}
