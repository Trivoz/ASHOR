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

use log::{debug, error, info, log_enabled, Level};

/// Main function
///
/// # Implementations
///
/// ## Logging
///
/// * Implemented logging boilerplate in commit `cb03349`
///
/// ```rust,no_run
/// env_logger::init();
///
/// debug!("This is a debug {}", "message");
/// error!("This is printed by default");
///
/// if log_enabled!(Level::Info) {
///     let x = 3 * 4; // expensive computation
///     info!("the anwser was: {}", x);
/// }
/// ```
fn main() {
}
