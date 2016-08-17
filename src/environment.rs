// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

use std::env;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

//------------------------------------------------------------------------------

const CONFIG_DIR     : &'static str = ".config";
const CONFIG_FILE    : &'static str = "freyja.conf";
const DEFAULT_CONFIG : &'static str = "";

//------------------------------------------------------------------------------

/// Aim of tis struct is tu perform all tasks related to environment like
/// reading files.
pub struct Environment {
    config_file_path : PathBuf,
}

//------------------------------------------------------------------------------

impl Environment {
    /// Constructor.
    pub fn new() -> Self {
        let home_dir = env::home_dir().unwrap();
        let config_dir = home_dir.join(CONFIG_DIR);
        let config_file = config_dir.join(CONFIG_FILE);
        Environment {
            config_file_path : config_file,
        }
    }

    /// Get config file contents or default value if file not found.
    pub fn get_config_file(&mut self) -> String {
        if self.config_file_path.exists() {
            let mut file = File::open(&self.config_file_path).unwrap();
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            contents
        } else {
            println!("Config file '{}' does not exists! \
                     Using default settings!", self.config_file_path.display());
            DEFAULT_CONFIG.to_string()
        }
    }
}

//------------------------------------------------------------------------------

