// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

use std::cell::Cell;
use dbus::{tree, Connection, BusType, Message};

use config;
use executor;

//------------------------------------------------------------------------------

const INTERFACE_NAME  : &'static str = "org.metanoia.freyja";
const OBJECT_PATH     : &'static str = "/org/metanoia/freyja";
const CONNECTION_NAME : &'static str = "org.metanoia.freyja";

const METHOD_EXEC : &'static str = "exec";
const METHOD_PING : &'static str = "ping";
const METHOD_QUIT : &'static str = "quit";

//------------------------------------------------------------------------------

/// Helper function for `exec` dbus method. One may say it is central point of
/// whole application. Interprets dbus message and binds `config` to `executor`
/// to execute command. Returns `true` on success, `false` otherwise.
fn handle_exec(msg: &Message,
               executor: &executor::Exec,
               config: &config::ConfigReader) -> bool {
    println!("Execution requested");
    match msg.get1::<String>() {
        None        => false,
        Some(alias) => executor.exec(&alias, config),
    }
}

//------------------------------------------------------------------------------

/// Facade represents interface to application from outside of application.
/// dbus is used as IPC mechanism.
///
/// Currently dbus interface has three methods:
///  - `exec(alias: str) -> bool` - allows to execute command assigned to alias
///  - `ping() -> str` - pings this application
///  - `quit() -> str` - tells this application too quit
pub struct Facade {
    is_running: Cell<bool>,
    connection: Connection,
}

//------------------------------------------------------------------------------

impl Facade {
    /// Constructor.
    pub fn new() -> Self {
        Facade {
            is_running: Cell::new(true),
            connection: Connection::get_private(BusType::Session).unwrap(),
        }
    }

    /// Prepare dbus tree and run for ever.
    pub fn run(&mut self,
               executor: &executor::Exec,
               config: &config::ConfigReader) {
        // Construct factory
        let factory = tree::Factory::new_fnmut();

        // Create method handlers
        let exec_method = factory.method(METHOD_EXEC, |m,_,_| {
                let result = handle_exec(m, executor, config);
                Ok(vec!(m.method_return().append(result)))
            }).out_arg("b").in_arg("s");

        let ping_method = factory.method(METHOD_PING, |m,_,_| {
                println!("I was pinged!");
                Ok(vec!(m.method_return().append("pong")))
            }).out_arg("s");

        let quit_method = factory.method(METHOD_QUIT, |m,_,_| {
                println!("I was requested to quit!");
                self.is_running.set(false);
                Ok(vec!(m.method_return().append("quiting...")))
            }).out_arg("s");

        // Build interface
        let interface = factory.interface(INTERFACE_NAME)
                               .add_m(exec_method)
                               .add_m(ping_method)
                               .add_m(quit_method);

        // Define object path
        let object_path = factory.object_path(OBJECT_PATH)
                                 .introspectable()
                                 .add(interface);

        // Add path to tree
        let tree = factory.tree().add(object_path);

        // Register connection name
        match self.connection.register_name(CONNECTION_NAME, 0x1) {
            Err(_) => println!("Failed to connect to dbus"),
            Ok(_)  => println!("Successfully connected to dbus"),
        };

        tree.set_registered(&self.connection, true).unwrap();

        // Run forever
        for _ in tree.run(&self.connection, self.connection.iter(1000)) {
            if self.is_running.get() == false {
                break;
            }
        }
    }
}

//------------------------------------------------------------------------------

