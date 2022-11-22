// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use serde::{Deserialize, Serialize};

use crate::page::{default_page_id, PageId};

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

impl Default for GeneralQuery {
    fn default() -> Self {
        Self {
            page: default_page_id(),
            order: GeneralOrder::default(),
        }
    }
}

impl GeneralQuery {
    pub fn backend_page_id(&self) -> PageId {
        if self.page < 1 {
            0
        } else {
            self.page - 1
        }
    }

    pub fn frontend_page_id(&self) -> PageId {
        if self.page < 1 {
            1
        } else {
            self.page
        }
    }
}
