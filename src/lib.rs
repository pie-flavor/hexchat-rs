//! A safe API for creating HexChat plugins.
//!
//! To get started, create a struct representing your plugin, and implement `Plugin` for it. Then,
//! call `plugin!` on the struct.
//!
//! All plugins should be built as cdylibs, or if for some reason you have no other choice, dylibs.
//! Do not attempt to define a `main()` symbol; `Plugin::new` is your plugin's 'entry point'. For that
//! matter, do not attempt to define the HexChat C docs' described `extern fn`s - this is taken care
//! of for you by the `plugin!` macro.
//!
//! If window manipulation is desired, the `window` feature should be enabled.

#![deny(missing_docs, clippy::pedantic)]
#![allow(
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap
)]
#![feature(slice_patterns, trait_alias, type_alias_enum_variants, never_type)]

use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[doc(hidden)]
pub mod c;

mod get_info;
pub use crate::get_info::*;
mod lists;
pub use crate::lists::*;
mod hook;
pub use crate::hook::*;
mod other;
pub use crate::other::*;
mod msg;
pub use crate::msg::*;
mod prefs;
pub use crate::prefs::*;
mod chan;
pub use crate::chan::*;
mod subplugin;
pub use crate::subplugin::*;

#[macro_use]
#[doc(hidden)]
pub mod call;

fn to_cstring(str: &str) -> CString {
    CString::new(str).unwrap_or_else(|_| CString::new(str.replace('\0', "")).unwrap())
}

unsafe fn from_cstring(ptr: *const c_char) -> String {
    CStr::from_ptr(ptr).to_string_lossy().into_owned()
}

unsafe fn from_cstring_opt(ptr: *const c_char) -> Option<String> {
    if ptr.is_null() {
        None
    } else {
        Some(from_cstring(ptr))
    }
}

/// The main object for working with HexChat. Passed to you via callbacks.
pub struct Context {
    handle: *mut c::hexchat_plugin,
}

impl Context {
    /// Executes a command as though it were typed in HexChat's input box.
    pub fn send_command(&self, command: &str) {
        let command = to_cstring(command);
        unsafe {
            c!(hexchat_command, self.handle, command.as_ptr());
        }
    }
}

/// This trait must be implemented on a type before the type is passed to `plugin!`.
pub trait Plugin {
    /// The name of your plugin.
    const NAME: &'static str;
    /// A short description of your plugin.
    const DESC: &'static str = "";
    /// The version string of your plugin.
    const VERSION: &'static str = "";
    /// Creates a new instance of your plugin. This is your 'entry point'.
    fn new(context: &Context) -> Self;
}
