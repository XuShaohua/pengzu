// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use backend::cmd;
use backend::error::print_error;

fn main() {
    if let Err(err) = cmd::run() {
        print_error(&err);
    }
}
