// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use backend::routers;
use std::io;

#[actix_web::main]
async fn main() -> io::Result<()> {
    routers::init()
}
