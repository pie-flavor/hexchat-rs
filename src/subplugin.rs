use crate::{c, to_cstring};
use std::ffi::c_void;
use std::path::Path;
use std::ptr;

/// Adds a fake plugin to the plugin list.
///
/// Useful for scripting enablement plugins, like the Lua plugin. You don't need to use this to add
/// your crate-registered plugin, and please don't. Returns a corresponding object suitable for
/// passing to `remove_fake_plugin`.
pub fn add_fake_plugin(
    path: impl AsRef<Path>,
    name: &str,
    description: &str,
    version: &str,
) -> FakePlugin {
    let path = to_cstring(&path.as_ref().to_string_lossy());
    let name = to_cstring(name);
    let description = to_cstring(description);
    let version = to_cstring(version);
    let handle = unsafe {
        c!(
            hexchat_plugingui_add,
            path.as_ptr(),
            name.as_ptr(),
            description.as_ptr(),
            version.as_ptr(),
            ptr::null(),
        )
    };
    FakePlugin { handle }
}
/// Removes a fake plugin entry from the plugin list added by `add_fake_plugin`.
///
/// In case you missed it the first time around, please do not add your crate-registered plugin to
/// this list.
#[allow(clippy::needless_pass_by_value)]
pub fn remove_fake_plugin(plugin: FakePlugin) {
    unsafe { c!(hexchat_plugingui_remove, plugin.handle) }
}

/// A handle to a fake plugin entry in the plugin list.
///
/// In case you missed it from the
/// `add_fake_plugin` documentation, please do not add your crate-registered plugin to this
/// list.
pub struct FakePlugin {
    handle: *mut c_void,
}
