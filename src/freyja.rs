// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

use config;
use environment;
use executor;
use facade;

//------------------------------------------------------------------------------

/// This is main class of the application. Constructs and hods ownership to
/// other parts.
pub struct Freyja {
    environment: environment::Environment,
    config:      config::Config,
    executor:    executor::Executor,
    facade:      facade::Facade,
}

//------------------------------------------------------------------------------

impl Freyja {
    /// Construct `Freyja` and contained structures.
    pub fn new() -> Self {
        let environment = environment::Environment::new();
        let config      = config::Config::new();
        let executor    = executor::Executor::new();
        let facade      = facade::Facade::new();
        Freyja { environment : environment,
                 config      : config,
                 executor    : executor,
                 facade      : facade }
    }

    /// Execute the application.
    pub fn run(&mut self) {
        match self.config.setup(self.environment.get_config_file()) {
            Ok(_) => self.facade.run(&self.executor, &self.config),
            Err(err) => println!("{}", err),
        }
    }
}

//------------------------------------------------------------------------------

impl Drop for Freyja {
    fn drop(&mut self) {
        println!("Bye!");
    }
}

//------------------------------------------------------------------------------

