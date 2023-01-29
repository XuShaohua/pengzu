// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use cip::parsers::parse_ebook_file;
use clap::{Arg, Command};

const OPT_PATH: &str = "path";

fn main() {
    let cmd = Command::new("cip")
        .version("0.1.0")
        .author("Xu Shaohua <shaohua@biofan.org>")
        .about("Parse CIP record from ebook files")
        .arg(Arg::new(OPT_PATH).help("path to ebook file"));
    let matches = cmd.get_matches();

    let path = matches
        .get_one::<String>(OPT_PATH)
        .expect("path is required");

    match parse_ebook_file(path) {
        Ok(cip) => {
            println!("cip: {:?}", cip);
        }
        Err(err) => {
            eprintln!("err: {err:?}");
        }
    }
}
