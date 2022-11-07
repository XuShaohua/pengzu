// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use clap::Command;
use std::env;

use crate::error::Error;
use crate::import;

mod add_user;
mod run_server;

pub fn run() -> Result<(), Error> {
    dotenv::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let mut cmd = Command::new("backend")
        .version(env!("VERGEN_GIT_SEMVER"))
        .author("Xu Shaohua <shaohua@biofan.org>")
        .about("Pengzu backend app")
        .subcommand(run_server::run_server_cmd())
        .subcommand(add_user::add_user_cmd())
        .subcommand(import::parse_cmdline());
    let matches = cmd.clone().get_matches();

    if let Some(matches) = matches.subcommand_matches(add_user::CMD_ADD_USER) {
        return add_user::add_user(matches);
    }
    if let Some(_matches) = matches.subcommand_matches(run_server::CMD_RUN_SERVER) {
        return run_server::run_server();
    }
    if let Some(matches) = matches.subcommand_matches(import::CMD_IMPORT_LIBRARY) {
        return import::run_daemon(matches);
    }

    cmd.print_help().map_err(Into::into)
}
