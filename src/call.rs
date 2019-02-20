/// This macro will set up your plugin's actual callability from HexChat.
///
/// # Example
///
/// ```rust
/// use hexchat::{Plugin, plugin}
/// struct MyPlugin;
/// impl Plugin for MyPlugin {
///     //...
/// #    const NAME: &'static str = "myplugin";
/// #    fn new(context: &Context) -> Self {
/// #        Self
/// #    }
/// }
/// plugin!(MyPlugin);
/// ```
///
#[macro_export]
macro_rules! plugin {
    ($x:ty) => {
        #[no_mangle]
        pub unsafe extern fn hexchat_plugin_init(
            plugin_handle: *mut $crate::c::hexchat_plugin,
            plugin_name: *mut *const ::std::os::raw:c_char,
            plugin_desc: *mut *const ::std::os::raw::c_char,
            plugin_version: *mut *const ::std::os::raw::c_char,
            arg: *mut ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int {
            $crate::call::hexchat_plugin_init::<$x>(plugin_handle, plugin_name, plugin_desc, plugin_version, arg)
        }

        #[no_mangle]
        pub unsafe extern fn hexchat_plugin_deinit(
            plugin_handle: *mut $crate::c::hexchat_plugin,
        ) -> ::std::os::raw::c_int {
            $crate::call::hexchat_plugin_deinit::<$x>(plugin_handle)
        }
    }
}

use std::os::raw::{c_char, c_int};

use crate::{c, to_cstring, Context, Plugin};

pub unsafe fn hexchat_plugin_init<T>(
    plugin_handle: *mut c::hexchat_plugin,
    plugin_name: *mut *const c_char,
    plugin_desc: *mut *const c_char,
    plugin_version: *mut *const c_char,
    _arg: *mut c_char,
) -> c_int
where
    T: Plugin,
{
    let name = to_cstring(T::NAME);
    *plugin_name = name.into_raw();
    let desc = to_cstring(T::DESC);
    *plugin_desc = desc.into_raw();
    let version = to_cstring(T::VERSION);
    *plugin_version = version.into_raw();
    //    match crate::register_plugin(T::new(&Context { handle: plugin_handle }), plugin_handle) {
    //        Ok(_) => 1 as c_int,
    //        Err(code) => code as c_int,
    //    }
    T::new(&Context {
        handle: plugin_handle,
    });
    1
}

pub unsafe fn hexchat_plugin_deinit<T>(_plugin_handle: *mut c::hexchat_plugin) -> c_int
where
    T: Plugin,
{
    //    match crate::deregister_plugin::<T>() {
    //        Ok(_) => 1 as c_int,
    //        Err(code) => code as c_int,
    //    }
    1
}
