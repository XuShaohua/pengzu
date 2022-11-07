// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use clap::Command;

use crate::error::Error;
use crate::routers;

pub const CMD_RUN_SERVER: &str = "run";

pub fn run_server_cmd() -> Command {
    Command::new(CMD_RUN_SERVER).about("Run server")
}

pub fn run_server() -> Result<(), Error> {
    let rt = actix_rt::Runtime::new()?;
    let handle = rt.spawn(async { routers::run().await });
    rt.block_on(handle).unwrap()
}
