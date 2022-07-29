// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use clap::{Arg, ArgMatches, Command};
use tokio::runtime::Runtime;

use crate::error::Error;
use crate::import::new_task::new_task;

const OPT_RESUME: &str = "resume";
const OPT_IMPORT: &str = "import";
const OPT_STOP: &str = "stop";

pub const CMD_IMPORT_LIBRARY: &str = "import-library";
const OPT_LIBRARY_ID: &str = "library_id";
const OPT_CALIBRE_PATH: &str = "calibre_path";

pub fn parse_cmdline() -> Command<'static> {
    Command::new(CMD_IMPORT_LIBRARY)
        .about("Import books from calibra library")
        .arg(
            Arg::new(OPT_RESUME)
                .long(OPT_RESUME)
                .takes_value(true)
                .value_name(OPT_LIBRARY_ID)
                .help("Resume importing task specified in library id."),
        )
        .arg(
            Arg::new(OPT_IMPORT)
                .long(OPT_IMPORT)
                .takes_value(true)
                .value_name(OPT_CALIBRE_PATH)
                .help("Start a new importing task from specified calibre path"),
        )
        .arg(
            Arg::new(OPT_STOP)
                .long(OPT_STOP)
                .takes_value(true)
                .value_name(OPT_LIBRARY_ID)
                .help("Stop importing task of specified library id"),
        )
}

fn resume_task(_library_id: i32) -> Result<(), Error> {
    let runtime = Runtime::new()?;
    runtime.block_on(async {
        println!("hello");
        Ok(())
    })
}

fn stop_task(_library_id: i32) -> Result<(), Error> {
    todo!()
}

pub fn run_daemon(matches: &ArgMatches) -> Result<(), Error> {
    if let Some(library_id) = matches.value_of(OPT_RESUME) {
        let library_id = library_id.parse()?;
        return resume_task(library_id);
    }
    if let Some(library_id) = matches.value_of(OPT_STOP) {
        let library_id = library_id.parse()?;
        return stop_task(library_id);
    }
    if let Some(calibre_path) = matches.value_of(OPT_IMPORT) {
        return new_task(calibre_path);
    }

    parse_cmdline().print_help().map_err(Into::into)
}
