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
//! contains path-related traits and structs

// TODO: add 'OSTYPE' variable to config loader

use crate::shell::invoke_system_command;
use std::process::ChildStdout;

/// Capable of being a path variable
///
/// ## About
///
/// Variables will have a name and a value.
///
/// ## Types
///
/// For python users, think of a variable as a dict
/// with str and any() types.
/// And for normal users, think of a variable as a hash map.
pub trait Variable {
    // TODO add compatability for different variable data types
}

/// Get a variable from the terminal
///
/// # Description
///
/// Invoke the terminal to produce an echo command that
/// will return the value of a given variable
///
/// # Arguments
///
/// * name - the name of the variable
///
/// # Returns `Option<&str>`
///  * Some - the value of the variable
///  * None - `None`.
pub fn invoke_output(name: &mut String) -> Option<ChildStdout> {

    // this is assuming that $ isn't already in this string.
    // XXX: handle this case maybe?
    name.insert_str(0, " $");

    let result = invoke_system_command("echo", Some(&vec![name.to_string()]));

    match result {
        Ok(result) => result.stdout,
        Err(_err) => None,
    }
}

/// Represents the global system path which contains environment variables.
pub struct _Path<T: Variable> {
    /// A vector of variables that implement [`variable`].
    ///
    /// [`variable`]: [`variable`][`Variable`]
    variables: Vec<T>,
}
