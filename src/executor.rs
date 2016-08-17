// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

use std::error::Error;
use std::process::Command;

use config;

//------------------------------------------------------------------------------

pub trait Exec {
    /// Given configuration execute command assigned to alias.
    /// Return `true` on success, `false` otherwise.
    fn exec(&self, alias: &String, config: &config::ConfigReader) -> bool;
}

//------------------------------------------------------------------------------

/// Executor handles execution of child programs.
pub struct Executor { }

//------------------------------------------------------------------------------

impl Executor {
    /// Constructor.
    pub fn new() -> Self {
        Executor { }
    }
}

//------------------------------------------------------------------------------

impl Exec for Executor {
    fn exec(&self, alias: &String, config: &config::ConfigReader) -> bool {
        println!("Executing alias: '{}'", alias);

        // Get command for given alias from configuration.
        let entry = match config.get_command_for_alias(&alias) {
            Some(entry) => entry,
            None => {
                println!("Alias was not configured!");
                return false;
            },
        };

        // Build command
        let mut command = Command::new(entry.command);
        for arg in entry.args {
            command.arg(arg);
        }

        // Execute command
        match command.spawn() {
            Ok(_) => {
                println!("'{}' spawned!", alias);
                true
            }
            Err(why) => {
                println!("Couldn't spawn! '{}'", why.description());
                false
            }
        }
    }
}

//------------------------------------------------------------------------------

