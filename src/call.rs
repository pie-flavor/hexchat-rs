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

use parking_lot::{MappedRwLockWriteGuard, RwLock, RwLockWriteGuard};
use std::any::{Any, TypeId};
use std::collections::HashSet;
use std::mem;
use std::os::raw::{c_char, c_int};
use std::panic;

use crate::{
    c, to_cstring, Command, Context, Plugin, PrintEventListener, RawServerEventListener, TimerTask,
    WindowEventListener, ALLOCATED,
};

static PLUGIN: RwLock<Option<PluginDef>> = RwLock::new(None);
unsafe impl Sync for PluginDef {}
unsafe impl Send for PluginDef {}

pub(crate) fn get_plugin() -> MappedRwLockWriteGuard<'static, PluginDef> {
    RwLockWriteGuard::map(PLUGIN.write(), |o| o.as_mut().unwrap())
}

#[allow(dead_code)]
pub(crate) fn is_loaded() -> bool {
    PLUGIN.read().is_some()
}

pub(crate) struct PluginDef {
    pub(crate) _type_id: TypeId,
    pub(crate) _ph: *mut c::hexchat_plugin,
    pub(crate) commands: HashSet<Command>,
    pub(crate) print_events: HashSet<PrintEventListener>,
    pub(crate) window_events: HashSet<WindowEventListener>,
    pub(crate) server_events: HashSet<RawServerEventListener>,
    pub(crate) timer_tasks: HashSet<TimerTask>,
    pub(crate) instance: Box<dyn Any>,
}

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
    {
        *ALLOCATED.write() = Some(Vec::new());
    }
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
        timer_tasks: HashSet::new(),
        instance: Box::new(t),
        _type_id: type_id,
        _ph: plugin_handle,
    };
    *PLUGIN.write() = Some(plugin_def);
    1
}

pub unsafe fn hexchat_plugin_deinit<T>(plugin_handle: *mut c::hexchat_plugin) -> c_int
where
    T: Plugin,
{
    let context = Context {
        handle: plugin_handle,
    };
    let mut plugin = None;
    let mut write = PLUGIN.write();
    mem::swap(&mut plugin, &mut *write);
    let plugin = plugin.unwrap();
    let PluginDef {
        server_events,
        window_events,
        print_events,
        commands,
        timer_tasks,
        instance,
        ..
    } = plugin;
    mem::drop(instance);
    for event in server_events {
        context.dealloc_raw_server_event_listener(event.0);
    }
    for event in window_events {
        context.dealloc_window_event_listener(event.0);
    }
    for event in print_events {
        context.dealloc_print_event_listener(event.0);
    }
    for command in commands {
        context.dealloc_command(command.0);
    }
    for timer_task in timer_tasks {
        context.dealloc_timer_task(timer_task.0);
    }
    let mut vec = None;
    let mut lock = ALLOCATED.write();
    mem::swap(&mut vec, &mut *lock);
    if let Some(vec) = vec {
        for func in vec {
            let boxed = func.0;
            boxed();
        }
    } else {
        return -5;
    }
    1
}
