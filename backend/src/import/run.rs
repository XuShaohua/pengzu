// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use crate::db::DbPool;
use crate::error::Error;
use tokio::signal::unix::{signal, SignalKind};

fn stop_worker() {
    todo!();
}

pub async fn run_inner_loop(_pool: DbPool, _library_id: i32) -> Result<(), Error> {
    let mut sigterm_stream = signal(SignalKind::terminate())?;
    let mut sigquit_stream = signal(SignalKind::quit())?;
    let mut sigint_stream = signal(SignalKind::interrupt())?;

    loop {
        tokio::select! {
            Some(_) = sigterm_stream.recv() => {
                stop_worker();
                break;
            }
            Some(_) = sigquit_stream.recv() => {
                stop_worker();
                break;
            }
            Some(_) = sigint_stream.recv() => {
                stop_worker();
                break;
            }
        }
    }

    Ok(())
}
