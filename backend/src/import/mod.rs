// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

pub mod convert;
mod db;
mod file_util;
mod import_books;
pub mod models;
mod new_task;

use clap::{Arg, ArgAction, ArgMatches, Command, ValueHint};

use crate::error::Error;
use crate::import::import_books::ImportBookFileAction;
use crate::import::new_task::new_task;
use crate::settings::get_library_root_dir;

const CALIBRE_LIBRARY: &str = "calibre-library";
const OPT_MOVE_FILES: &str = "move-files";

pub const CMD_IMPORT_LIBRARY: &str = "import-library";
const OPT_CALIBRE_PATH: &str = "calibre_path";

#[must_use]
pub fn parse_cmdline() -> Command {
    Command::new(CMD_IMPORT_LIBRARY)
        .about("Import books from calibra library")
        .arg(
            Arg::new(OPT_MOVE_FILES)
                .long(OPT_MOVE_FILES)
                .action(ArgAction::SetTrue)
                .help("Move files to new path, instead of copy"),
        )
        .arg(
            Arg::new(CALIBRE_LIBRARY)
                .value_hint(ValueHint::DirPath)
                .value_name(OPT_CALIBRE_PATH)
                .help("Importing books from calibre path"),
        )
}

pub fn run_daemon(matches: &ArgMatches) -> Result<(), Error> {
    let file_action = matches.get_one::<bool>(OPT_MOVE_FILES).map_or(
        ImportBookFileAction::DoNothing,
        |move_files| {
            if *move_files {
                ImportBookFileAction::Move
            } else {
                ImportBookFileAction::Copy
            }
        },
    );

    let library_path = get_library_root_dir()?;
    let library_path = library_path.into_os_string().into_string()?;

    if let Some(calibre_path) = matches.get_one::<String>(CALIBRE_LIBRARY) {
        return new_task(calibre_path, &library_path, file_action);
    }

    parse_cmdline().print_help().map_err(Into::into)
}
