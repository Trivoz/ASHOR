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
use log::{debug, error};
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
    debug!("args: {:?}", arguments);
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

#[cfg(test)]
pub mod test {
    #[test]
    fn invoke_system_command() {
        assert!(true)
    }
}

pub struct Shell {
    /// Is the shell the default shell?
    ///
    /// TODO: make this private when its not used
    pub is_default: bool,
    /// Has system config in /etc/ashor/config.toml
    // has_system_config: bool,
    // TODO: this.
    /// Has user config in $XDG_CONFIG_HOME/ashor/config.toml
    has_user_config: bool,
    // /// System variables in the path
    // // TODO somehow make system_variables impl Sized
    // // system_variables: Vec<dyn Variable>
}

impl Shell {
    /// The user configuation path wherein the configuration file is stored
    // TODO: make ashor.toml just config.toml instead (globally for user and system)
    const USER_CONFIG_PATH: &'static str = "~/.config/ashor/ashor.toml";

    /// Returns if 'ashor' is the default shell
    fn is_default() -> bool {
        invoke_output("SHELL".to_string()).is_some()
    }

    /// Create an instance of [`Shell`]
    ///
    /// # Example
    ///
    /// ```rust,no_test
    /// let shell = Shell::new();
    /// ```
    pub fn new() -> Self {
        let _has_system_config: bool;
        let _has_user_config: bool;

        Self {
            is_default: Self::is_default(),
            has_user_config: path::Path::new(Self::USER_CONFIG_PATH).exists(),
        }
    }
}
