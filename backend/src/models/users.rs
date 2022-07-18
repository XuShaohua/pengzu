// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum UserRole {
    User = 0,
    Guest = 1,
    Admin = 2,
}
