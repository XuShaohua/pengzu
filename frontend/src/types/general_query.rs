// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use serde::{Deserialize, Serialize};

use crate::types::page::{default_page_id, PageId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum GeneralOrder {
    IdDesc,
    IdAsc,
    TitleDesc,
    TitleAsc,
    NumberDesc,
    NumberAsc,
}

impl Default for GeneralOrder {
    fn default() -> Self {
        Self::IdDesc
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct GeneralQuery {
    #[serde(default = "default_page_id")]
    pub page: PageId,
    #[serde(default = "GeneralOrder::default")]
    pub order: GeneralOrder,
}
