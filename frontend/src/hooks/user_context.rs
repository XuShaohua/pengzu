// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use shared::users::{UserInfo, UserRole};
use std::fmt;
use std::fmt::Formatter;
use std::ops::Deref;
use yew::prelude::*;
use yew_router::hooks::use_navigator;
use yew_router::prelude::Navigator;

use crate::router::Route;
use crate::services::auth::set_token;

/// State handle for the [`use_user_context`] hook.
#[derive(Clone)]
pub struct UseUserContextHandle {
    inner: UseStateHandle<UserInfo>,
    navigator: Navigator,
}

impl UseUserContextHandle {
    pub fn login(&self, info: UserInfo) {
        set_token(Some(&info.token));
        self.inner.set(info);
        self.navigator.push(&Route::Home);
    }

    pub fn logout(&self) {
        set_token(None);
        self.inner.set(UserInfo::default());
        self.navigator.push(&Route::Home);
    }

    pub fn is_login(&self) -> bool {
        self.inner.id > 0 && self.inner.role != UserRole::Nil
    }
}

impl Deref for UseUserContextHandle {
    type Target = UserInfo;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl PartialEq for UseUserContextHandle {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl fmt::Debug for UseUserContextHandle {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("UseUserContextHandle")
            .field("value", &format!("{:#?}", *self.inner))
            .finish()
    }
}

/// Manages user login context.
///
/// # Panics
/// Raise panic if failed to get history object.
#[must_use]
#[hook]
pub fn use_user_context() -> UseUserContextHandle {
    let inner: UseStateHandle<UserInfo> = use_context::<UseStateHandle<UserInfo>>().unwrap();
    let navigator = use_navigator().unwrap();

    UseUserContextHandle { inner, navigator }
}
