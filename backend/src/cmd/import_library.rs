// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use clap::{value_parser, Arg, ArgAction, ArgMatches, Command, ValueHint};

use crate::error::Error;
use crate::import::new_task::new_task;
use crate::import::options::ImportBookFileAction;
use crate::settings::get_library_root_dir;

pub const CMD_IMPORT_LIBRARY: &str = "import-library";

const CALIBRE_LIBRARY: &str = "calibre-library";
const OPT_MOVE_FILES: &str = "move-files";
const OPT_UID: &str = "uid";
const OPT_GID: &str = "gid";
const OPT_CALIBRE_PATH: &str = "calibre_path";

// TODO(Shaohua): Replace with clap_derive.
#[must_use]
pub fn new_cmd() -> Command {
    Command::new(CMD_IMPORT_LIBRARY)
        .about("Import books from calibra library")
        .arg(
            Arg::new(OPT_MOVE_FILES)
                .long(OPT_MOVE_FILES)
                .action(ArgAction::SetTrue)
                .help("Move files to new path, instead of copy"),
        )
        .arg(
            Arg::new(OPT_UID)
                .long(OPT_UID)
                .action(ArgAction::Set)
                .value_parser(value_parser!(u32))
                .help("Specify user id book new book"),
        )
        .arg(
            Arg::new(OPT_GID)
                .long(OPT_GID)
                .action(ArgAction::Set)
                .value_parser(value_parser!(u32))
                .help("Specify gid id book new book"),
        )
        .arg(
            Arg::new(CALIBRE_LIBRARY)
                .value_hint(ValueHint::DirPath)
                .value_name(OPT_CALIBRE_PATH)
                .help("Importing books from calibre path"),
        )
}

#[allow(clippy::similar_names)]
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

    let uid = matches.get_one::<u32>(OPT_UID).copied();
    let gid = matches.get_one::<u32>(OPT_GID).copied();

    let library_path = get_library_root_dir()?;
    let library_path = library_path.into_os_string().into_string()?;

    if let Some(calibre_path) = matches.get_one::<String>(CALIBRE_LIBRARY) {
        return new_task(calibre_path, &library_path, file_action, uid, gid);
    }

    new_cmd().print_help().map_err(Into::into)
}
