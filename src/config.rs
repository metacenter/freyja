// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

use std::collections::LinkedList;

//------------------------------------------------------------------------------

pub trait ConfigReader {
    /// Return command assigned to given alias if found, `None` otherwise.
    fn get_command_for_alias(&self, alias: &String) -> Option<String>;
}

//------------------------------------------------------------------------------

/// Helper structure for binding command to alias.
struct Entry {
    alias: String,
    command: String,
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
        Config {  entry_list : LinkedList::new() }
    }

    /// Initialize configuration.
    /// Fills list with made up data.
    /// TODO Implement reading configuration from file.
    pub fn setup(&mut self) {
        self.entry_list.push_front(Entry{alias:   String::from("opera"),
                                         command: String::from("opera")});
        self.entry_list.push_front(Entry{alias:   String::from("firefox"),
                                         command: String::from("firefox")});
    }

    /// Finalize configuration.
    pub fn teardown(&mut self) {
        // Nothing to do so far.
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

