// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum ImportBookFileAction {
    Copy = 1,
    Move = 2,
    DoNothing = 3,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct ImportBookOptions {
    pub file_action: ImportBookFileAction,
    pub allow_duplication: bool,
    pub uid: Option<u32>,
    pub gid: Option<u32>,
}

impl Default for ImportBookOptions {
    fn default() -> Self {
        Self {
            file_action: ImportBookFileAction::Copy,
            allow_duplication: false,
            uid: None,
            gid: None,
        }
    }
}
