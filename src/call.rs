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
        pub unsafe extern "C" fn hexchat_plugin_init(
            plugin_handle: *mut $crate::c::hexchat_plugin,
            plugin_name: *mut *const ::std::os::raw::c_char,
            plugin_desc: *mut *const ::std::os::raw::c_char,
            plugin_version: *mut *const ::std::os::raw::c_char,
            arg: *mut ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int {
            $crate::call::hexchat_plugin_init::<$x>(
                plugin_handle,
                plugin_name,
                plugin_desc,
                plugin_version,
                arg,
            )
        }

        #[no_mangle]
        pub unsafe extern "C" fn hexchat_plugin_deinit(
            plugin_handle: *mut $crate::c::hexchat_plugin,
        ) -> ::std::os::raw::c_int {
            $crate::call::hexchat_plugin_deinit::<$x>(plugin_handle)
        }
    };
}

use lazy_static::lazy_static;
use std::any::{Any, TypeId};
use std::collections::{HashMap, HashSet};
use std::os::raw::{c_char, c_int};
use std::panic;
use std::sync::Mutex;

use crate::{
    c, to_cstring, Command, Context, Plugin, PrintEventListener, RawServerEventListener,
    WindowEventListener,
};

lazy_static! {
    pub(crate) static ref PLUGINS: Mutex<HashMap<PhWrapper, PluginDef>> =
        Mutex::new(HashMap::new());
    pub(crate) static ref TYPES: Mutex<HashMap<TypeId, PhWrapper>> = Mutex::new(HashMap::new());
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct PhWrapper(pub(crate) *mut c::hexchat_plugin);
unsafe impl Send for PhWrapper {}

pub(crate) struct PluginDef {
    pub(crate) commands: HashSet<Command>,
    pub(crate) print_events: HashSet<PrintEventListener>,
    pub(crate) window_events: HashSet<WindowEventListener>,
    pub(crate) server_events: HashSet<RawServerEventListener>,
    pub(crate) instance: Box<dyn Any>,
}
unsafe impl Send for PluginDef {}

pub unsafe fn hexchat_plugin_init<T>(
    plugin_handle: *mut c::hexchat_plugin,
    plugin_name: *mut *const c_char,
    plugin_desc: *mut *const c_char,
    plugin_version: *mut *const c_char,
    _arg: *mut c_char,
) -> c_int
where
    T: Plugin + 'static,
{
    let name = to_cstring(T::NAME);
    *plugin_name = name.into_raw();
    let desc = to_cstring(T::DESC);
    *plugin_desc = desc.into_raw();
    let version = to_cstring(T::VERSION);
    *plugin_version = version.into_raw();
    let t = panic::catch_unwind(|| {
        T::new(&Context {
            handle: plugin_handle,
        })
    });
    let t = match t {
        Ok(t) => t,
        Err(e) => {
            Context {
                handle: plugin_handle,
            }
            .send_command(&if let Some(string) = (*e).downcast_ref::<&str>() {
                format!(
                    "GUI MSGBOX Plugin '{} {}' failed to load. Panic message: {}",
                    T::NAME,
                    T::VERSION,
                    string
                )
            } else {
                format!(
                    "GUI MSGBOX Plugin '{} {}' failed to load.",
                    T::NAME,
                    T::VERSION
                )
            });
            return -4;
        }
    };
    let type_id = t.type_id();
    let plugin_def = PluginDef {
        commands: HashSet::new(),
        print_events: HashSet::new(),
        window_events: HashSet::new(),
        server_events: HashSet::new(),
        instance: Box::new(t),
    };
    if let Ok(mut types) = TYPES.lock() {
        types.insert(type_id, PhWrapper(plugin_handle));
    } else {
        return -2;
    }
    if let Ok(mut plugins) = PLUGINS.lock() {
        plugins.insert(PhWrapper(plugin_handle), plugin_def);
        1
    } else {
        -1
    }
}

pub unsafe fn hexchat_plugin_deinit<T>(plugin_handle: *mut c::hexchat_plugin) -> c_int
where
    T: Plugin,
{
    let context = Context {
        handle: plugin_handle,
    };
    if let Ok(mut plugins) = PLUGINS.lock() {
        if let Some(plugin_def) = plugins.remove(&PhWrapper(plugin_handle)) {
            let PluginDef {
                instance,
                server_events,
                window_events,
                print_events,
                commands,
            } = plugin_def;
            for event in server_events {
                context.dealloc_raw_server_event_listener(event);
            }
            for event in window_events {
                context.dealloc_window_event_listener(event);
            }
            for event in print_events {
                context.dealloc_print_event_listener(event);
            }
            for command in commands {
                context.dealloc_command(command);
            }
            if let Ok(mut types) = TYPES.lock() {
                types.remove(&instance.type_id());
                1
            } else {
                -2
            }
        } else {
            -3
        }
    } else {
        -1
    }
}
