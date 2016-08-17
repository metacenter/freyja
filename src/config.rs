// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

use std::collections::LinkedList;
use std::error::Error;
use rustc_serialize::json;

//------------------------------------------------------------------------------

pub trait ConfigReader {
    /// Return command assigned to given alias if found, `None` otherwise.
    fn get_command_for_alias(&self, alias: &String) -> Option<String>;
}

//------------------------------------------------------------------------------

/// Helper structure for binding command to alias
/// and decoding JSON configuration.
#[derive(RustcDecodable)]
struct Entry {
    alias: String,
    command: String,
}

//------------------------------------------------------------------------------

/// Helper structure for decoding JSON configuration.
#[derive(RustcDecodable)]
struct Configuration {
    bindings: Vec<Entry>,
}

//------------------------------------------------------------------------------

/// This class represents configuration. Allows to read from configuration file
/// and access settings.
pub struct Config {
    entry_list: LinkedList<Entry>,
}

//------------------------------------------------------------------------------

impl Config {
    /// Constructor.
    pub fn new() -> Self {
        Config { entry_list : LinkedList::new() }
    }

    /// Initialize configuration.
    /// Return number of read entries on success or error description.
    pub fn setup(&mut self, conf: String) -> Result<usize,String> {
        // Decode received JSON string
        let decoded: Configuration = match json::decode(&conf) {
            Ok(decoded) => decoded,
            Err(err) => {
                return Err("Failed to parse configuration file: ".to_string() +
                           err.description() );
            },
        };

        // Copy entries
        for entry in decoded.bindings {
            self.entry_list.push_front(entry);
        };
        Ok(self.entry_list.len())
    }
}

//------------------------------------------------------------------------------

impl ConfigReader for Config {
    fn get_command_for_alias(&self, alias: &String) -> Option<String> {
        for entry in self.entry_list.iter() {
            if &entry.alias == alias {
                return Some(entry.command.clone());
            }
        }
        None
    }
}

//------------------------------------------------------------------------------

