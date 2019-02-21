#![allow(clippy::type_complexity)] // todo fix when intellij-rust supports trait typedefs

use crate::call;
use crate::{c, from_cstring, to_cstring, ChannelRef, Context, PrintEvent, WindowEvent};
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use std::ffi::c_void;
use std::os::raw::{c_char, c_int};
use std::panic::{self, AssertUnwindSafe};
use std::time::Duration;

/// A handle to a registered command.
#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Command(pub(crate) *mut c::hexchat_hook);
/// A handle to a registered print event listener.
#[derive(Debug, Eq, PartialEq, Hash)]
pub struct PrintEventListener(pub(crate) *mut c::hexchat_hook);
/// A handle to a registered window event listener.
#[derive(Debug, Eq, PartialEq, Hash)]
pub struct WindowEventListener(pub(crate) *mut c::hexchat_hook);
/// A handle to a registered raw server event listener.
#[derive(Debug, Eq, PartialEq, Hash)]
pub struct RawServerEventListener(pub(crate) *mut c::hexchat_hook);
/// A handle to a registered timer task.
#[derive(Debug, Eq, PartialEq, Hash)]
pub struct TimerTask(pub(crate) *mut c::hexchat_hook);

impl Context {
    /// Registers a new command accessible to the user via `/<COMMAND> [args]`. Returns a
    /// corresponding object that can be passed to `deregister_command`.
    ///
    /// # Callback
    ///
    /// The callback's signature corresponds to this context, followed by a slice of all the command
    /// arguments. If you intend to get command arguments, you should probably start at 1; argument
    /// 0 is the name of the command. The callback should return who the command event should be
    /// hidden from.
    pub fn register_command(
        &self,
        name: &str,
        help_text: &str,
        priority: Priority,
        function: impl Fn(&Self, &[String]) -> EatMode + 'static,
    ) -> Command {
        let hook_ref = CommandHookRef {
            function: Box::new(function),
            ph: self.handle,
        };
        let boxed = Box::new(hook_ref);
        let ptr = Box::into_raw(boxed);
        let name = to_cstring(name);
        let help_text = to_cstring(help_text);
        let hook_ptr = unsafe {
            c!(
                hexchat_hook_command,
                self.handle,
                name.as_ptr(),
                c_int::from(priority.0),
                command_hook,
                help_text.as_ptr(),
                ptr as _,
            )
        };
        if let Ok(mut plugin) = call::get_plugin().lock() {
            plugin.commands.insert(Command(hook_ptr));
        }
        Command(hook_ptr)
    }

    /// Deregisters a command registered by `register_command`.
    #[allow(clippy::needless_pass_by_value)]
    pub fn deregister_command(&self, command: Command) {
        self.dealloc_command(command.0);
        if let Ok(mut plugin) = call::get_plugin().lock() {
            plugin.commands.remove(&command);
        }
    }

    pub(crate) fn dealloc_command(&self, command: *mut c::hexchat_hook) {
        unsafe {
            let ptr = c!(hexchat_unhook, self.handle, command);
            let ptr = ptr as *mut CommandHookRef;
            Box::from_raw(ptr);
        }
    }

    /// Adds a listener for a particular `PrintEvent`; see `PrintEvent`'s documentation for more
    /// details. Returns a corresponding object that can be passed to `remove_print_event_listener`.
    ///
    /// # Callback
    ///
    /// The callback's signature is this context, followed by a slice of all the print event's
    /// arguments, followed by the time this message was printed. Note that the argument `$1`
    /// corresponds to `args[0]` and so forth. The callback should return who the event should be
    /// hidden from.
    pub fn add_print_event_listener(
        &self,
        event: PrintEvent,
        priority: Priority,
        function: impl Fn(&Self, &[String], DateTime<Utc>) -> EatMode + 'static,
    ) -> PrintEventListener {
        let hook_ref = PrintHookRef {
            function: Box::new(function),
            ph: self.handle,
        };
        let boxed = Box::new(hook_ref);
        let ptr = Box::into_raw(boxed);
        let name = to_cstring(event.0);
        let hook_ptr = unsafe {
            c!(
                hexchat_hook_print_attrs,
                self.handle,
                name.as_ptr(),
                c_int::from(priority.0),
                print_hook,
                ptr as _,
            )
        };
        if let Ok(mut plugin) = call::get_plugin().lock() {
            plugin.print_events.insert(PrintEventListener(hook_ptr));
        }
        PrintEventListener(hook_ptr)
    }

    /// Removes a listener added by `add_print_event_listener`.
    #[allow(clippy::needless_pass_by_value)]
    pub fn remove_print_event_listener(&self, listener: PrintEventListener) {
        self.dealloc_print_event_listener(listener.0);
        if let Ok(mut plugin) = call::get_plugin().lock() {
            plugin.print_events.remove(&listener);
        }
    }

    pub(crate) fn dealloc_print_event_listener(&self, listener: *mut c::hexchat_hook) {
        unsafe {
            let ptr = c!(hexchat_unhook, self.handle, listener);
            let ptr = ptr as *mut PrintHookRef;
            Box::from_raw(ptr);
        }
    }

    /// Adds a listener for a particular `WindowEvent`. See `WindowEvent`'s docs for more details.
    /// Returns a corresponding object that can be passed to `remove_window_event_listener`.
    ///
    /// # Callback
    ///
    /// The callback's signature is this context, followed by a `ChannelRef` corresponding to the
    /// channel this event is regarding or the current channel if none applies. The callback should
    /// return who the event should be hidden from.
    pub fn add_window_event_listener(
        &self,
        event: WindowEvent,
        priority: Priority,
        function: impl Fn(&Self, ChannelRef) -> EatMode + 'static,
    ) -> WindowEventListener {
        let context_ref = ContextHookRef {
            function: Box::new(function),
            ph: self.handle,
        };
        let boxed = Box::new(context_ref);
        let ptr = Box::into_raw(boxed);
        let name = to_cstring(event.0);
        let hook_ptr = unsafe {
            c!(
                hexchat_hook_print,
                self.handle,
                name.as_ptr(),
                c_int::from(priority.0),
                context_hook,
                ptr as _,
            )
        };
        if let Ok(mut plugin) = call::get_plugin().lock() {
            plugin.window_events.insert(WindowEventListener(hook_ptr));
        }
        WindowEventListener(hook_ptr)
    }

    /// Removes a listener added by `add_window_event_listener`.
    #[allow(clippy::needless_pass_by_value)]
    pub fn remove_window_event_listener(&self, listener: WindowEventListener) {
        self.dealloc_window_event_listener(listener.0);
        if let Ok(mut plugin) = call::get_plugin().lock() {
            plugin.window_events.remove(&listener);
        }
    }

    pub(crate) fn dealloc_window_event_listener(&self, listener: *mut c::hexchat_hook) {
        unsafe {
            let ptr = c!(hexchat_unhook, self.handle, listener);
            let ptr = ptr as *mut ContextHookRef;
            Box::from_raw(ptr);
        }
    }
    // todo figure out how the hell keypress and dcc chat text events work

    /// Adds a listener for raw server events, i.e. commands coming straight from the server. Will
    /// be superseded by a fuller event API soon. Returns a corresponding object suitable for
    /// passing to `remove_raw_server_event_listener`.
    ///
    /// # Callback
    ///
    /// The callback's signature is this context, followed by a slice of all the event's arguments,
    /// followed by the time this event was sent. If you intend to get event arguments, you probably
    /// should start at 1, since argument 0 is the event name. The callback should return who the
    /// event should be hidden from.
    pub fn add_raw_server_event_listener(
        &self,
        event: &str,
        priority: Priority,
        function: impl Fn(&Self, &[String], DateTime<Utc>) -> EatMode + 'static,
    ) -> RawServerEventListener {
        let server_ref = ServerHookRef {
            function: Box::new(function),
            ph: self.handle,
        };
        let boxed = Box::new(server_ref);
        let ptr = Box::into_raw(boxed);
        let event = to_cstring(event);
        let hook_ptr = unsafe {
            c!(
                hexchat_hook_server_attrs,
                self.handle,
                event.as_ptr(),
                c_int::from(priority.0),
                server_hook,
                ptr as _,
            )
        };
        if let Ok(mut plugin) = call::get_plugin().lock() {
            plugin
                .server_events
                .insert(RawServerEventListener(hook_ptr));
        }
        RawServerEventListener(hook_ptr)
    }

    /// Removes a listener added by `add_raw_server_event_listener`.
    #[allow(clippy::needless_pass_by_value)]
    pub fn remove_raw_server_event_listener(&self, listener: RawServerEventListener) {
        self.dealloc_raw_server_event_listener(listener.0);
        if let Ok(mut plugin) = call::get_plugin().lock() {
            plugin.server_events.remove(&listener);
        }
    }

    pub(crate) fn dealloc_raw_server_event_listener(&self, listener: *mut c::hexchat_hook) {
        unsafe {
            let ptr = c!(hexchat_unhook, self.handle, listener);
            let ptr = ptr as *mut ServerHookRef;
            Box::from_raw(ptr);
        }
    }

    /// Registers a task to be run repeatedly with a specified interval. Returns a corresponding
    /// object suitable for passing to `remove_timer_task`.
    ///
    /// # Note
    ///
    /// Right now the interval cannot be more than `i32::max_value()` milliseconds. If it is more
    /// than `i32::max_value()` milliseconds, it will be truncated to `i32::max_value()`
    /// milliseconds. This restriction will be lifted in the future.
    pub fn add_timer_task(&self, interval: Duration, task: impl Fn(&Self) + 'static) -> TimerTask {
        let timer_ref = TimerHookRef {
            function: Box::new(task),
            ph: self.handle,
        };
        let boxed = Box::new(timer_ref);
        let ptr = Box::into_raw(boxed);
        let ms = interval.as_millis();
        let ms = if ms > i32::max_value() as u128 {
            i32::max_value()
        } else {
            ms as i32
        }; //todo implement a way to handle u128-length timeouts
        let hook_ptr = unsafe { c!(hexchat_hook_timer, self.handle, ms, timer_hook, ptr as _) };
        if let Ok(mut plugin) = call::get_plugin().lock() {
            plugin.timer_tasks.insert(TimerTask(hook_ptr));
        }
        TimerTask(hook_ptr)
    }

    /// Removes a timer task added by `add_timer_task`.
    #[allow(clippy::needless_pass_by_value)]
    pub fn remove_timer_task(&self, task: TimerTask) {
        self.dealloc_timer_task(task.0);
        if let Ok(mut plugin) = call::get_plugin().lock() {
            plugin.timer_tasks.remove(&task);
        }
    }

    pub(crate) fn dealloc_timer_task(&self, task: *mut c::hexchat_hook) {
        unsafe {
            let ptr = c!(hexchat_unhook, self.handle, task);
            let ptr = ptr as *mut TimerHookRef;
            Box::from_raw(ptr);
        }
    }
}

struct CommandHookRef {
    function: Box<dyn Fn(&Context, &[String]) -> EatMode>,
    ph: *mut c::hexchat_plugin,
}

struct PrintHookRef {
    function: Box<dyn Fn(&Context, &[String], DateTime<Utc>) -> EatMode>,
    ph: *mut c::hexchat_plugin,
}

struct ContextHookRef {
    function: Box<dyn Fn(&Context, ChannelRef) -> EatMode>,
    ph: *mut c::hexchat_plugin,
}

struct ServerHookRef {
    function: Box<dyn Fn(&Context, &[String], DateTime<Utc>) -> EatMode>,
    ph: *mut c::hexchat_plugin,
}

struct TimerHookRef {
    function: Box<dyn Fn(&Context)>,
    ph: *mut c::hexchat_plugin,
}

unsafe extern "C" fn command_hook(
    word: *mut *mut c_char,
    _word_eol: *mut *mut c_char,
    user_data: *mut c_void,
) -> c_int {
    let user_data = user_data as *mut CommandHookRef;
    let context = Context {
        handle: (*user_data).ph,
    };
    let mut vec = Vec::new();
    for i in 1..32 {
        let offset = word.offset(i);
        if !offset.is_null() {
            let ptr = *offset;
            if !ptr.is_null() {
                let cstr = from_cstring(ptr);
                vec.push(cstr);
            }
        }
    }
    let res =
        match panic::catch_unwind(AssertUnwindSafe(|| ((*user_data).function)(&context, &vec))) {
            Ok(eat) => eat,
            Err(e) => {
                context.print_plain(&format!("Error in command '/{}'", &vec.join(" ")));
                if let Some(string) = (*e).downcast_ref::<&str>() {
                    context.print_plain(&format!("Error message: {}", string));
                }
                EatMode::All
            }
        };
    res as _
}

unsafe extern "C" fn print_hook(
    word: *mut *mut c_char,
    attrs: *mut c::hexchat_event_attrs,
    user_data: *mut c_void,
) -> c_int {
    let user_data = user_data as *mut PrintHookRef;
    let context = Context {
        handle: (*user_data).ph,
    };
    let mut vec = Vec::new();
    for i in 1..32 {
        let offset = word.offset(i);
        if !offset.is_null() {
            let ptr = *offset;
            if !ptr.is_null() {
                let cstr = from_cstring(ptr);
                vec.push(cstr);
            }
        }
    }
    let naive = NaiveDateTime::from_timestamp((*attrs).server_time_utc as _, 0);
    let utc = Utc.from_utc_datetime(&naive);
    panic::catch_unwind(AssertUnwindSafe(|| {
        ((*user_data).function)(&context, &vec, utc)
    }))
    .unwrap_or(EatMode::None) as _
}

unsafe extern "C" fn context_hook(_word: *mut *mut c_char, user_data: *mut c_void) -> c_int {
    let user_data = user_data as *mut ContextHookRef;
    let context = Context {
        handle: (*user_data).ph,
    };
    let ctx = c!(hexchat_get_context, (*user_data).ph);
    let cref = ChannelRef {
        ph: (*user_data).ph,
        handle: ctx,
    };
    panic::catch_unwind(AssertUnwindSafe(|| ((*user_data).function)(&context, cref)))
        .unwrap_or(EatMode::None) as _
}

unsafe extern "C" fn server_hook(
    word: *mut *mut c_char,
    _word_eol: *mut *mut c_char,
    attrs: *mut c::hexchat_event_attrs,
    user_data: *mut c_void,
) -> c_int {
    let user_data = user_data as *mut ServerHookRef;
    let context = Context {
        handle: (*user_data).ph,
    };
    let mut vec = Vec::new();
    for i in 1..32 {
        let offset = word.offset(i);
        if !offset.is_null() {
            let ptr = *offset;
            if !ptr.is_null() {
                let cstr = from_cstring(ptr);
                vec.push(cstr);
            }
        }
    }
    let naive = NaiveDateTime::from_timestamp((*attrs).server_time_utc as _, 0);
    let utc = Utc.from_utc_datetime(&naive);
    panic::catch_unwind(AssertUnwindSafe(|| {
        ((*user_data).function)(&context, &vec, utc)
    }))
    .unwrap_or(EatMode::None) as _
}

unsafe extern "C" fn timer_hook(user_data: *mut c_void) -> c_int {
    let user_data = user_data as *mut TimerHookRef;
    let context = Context {
        handle: (*user_data).ph,
    };
    panic::catch_unwind(AssertUnwindSafe(|| {
        ((*user_data).function)(&context);
    }))
    .ok();
    EatMode::All as _
}

/// The priority of an event listener or command.
///
/// This represents what order listeners or command
/// handlers will be called, and earlier listeners or command handlers can prevent later listeners
/// or command handlers from seeing the event or command via `EatMode::Plugin` or `EatMode::All`.
/// `Priority` instances can be constructed from any `i8`, but you are encouraged to use the
/// built-in constants, and especially `Priority::NORMAL` at that.
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Priority(pub i8);

impl Priority {
    /// The highest possible priority.
    pub const HIGHEST: Self = Self(127);
    /// A mediumly high Self.
    pub const HIGH: Self = Self(64);
    /// Normal Self. Prefer this Self.
    pub const NORMAL: Self = Self(0);
    /// A mediumly low Self.
    pub const LOW: Self = Self(-64);
    /// The lowest possible Self.
    pub const LOWEST: Self = Self(-128);
}

/// Represents who to hide an event or command from.
pub enum EatMode {
    /// Do not hide the event or command from anyone. Plugins and HexChat will continue to receive
    /// this event or command, even if you have already processed it.
    None,
    /// Hide this event or command from HexChat. HexChat will not process this event or command
    /// since you clearly already have, but other plugins still can.
    Hexchat,
    /// Hide this event or command from other plugins. Other plugins will not process this event or
    /// command since you clearly already have, but HexChat still will.
    Plugin,
    /// Hide this event or command from both HexChat and other plugins. This effectively says that
    /// you are the intended receiver of this event or command, and is the option you should use
    /// in most cases.
    All,
}
