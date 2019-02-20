#![allow(non_camel_case_types)]

use std::os::raw::{c_int, c_char};
use libc::time_t;
use std::ffi::c_void;

#[repr(C)]
pub struct hexchat_context(!);
#[repr(C)]
pub struct hexchat_hook(!);
#[repr(C)]
pub struct hexchat_list(!);
#[repr(C)]
pub struct hexchat_plugin(!);

#[repr(C)]
pub struct hexchat_event_attrs {
    pub server_time_utc: time_t,
}

extern {
    pub fn hexchat_hook_command(
        ph: *mut hexchat_plugin,
        name: *const c_char,
        pri: c_int,
        callback: unsafe extern fn (
            word: *mut *mut c_char,
            word_eol: *mut *mut c_char,
            user_data: *mut c_void
        ) -> c_int,
        help_text: *const c_char,
        userdata: *mut c_void,
    ) -> *mut hexchat_hook;

    pub fn hexchat_event_attrs_create(
        ph: *mut hexchat_plugin,
    ) -> *mut hexchat_event_attrs;

    pub fn hexchat_event_attrs_free(
        ph: *mut hexchat_plugin,
        attrs: *mut hexchat_event_attrs,
    );

    pub fn hexchat_hook_server(
        ph: *mut hexchat_plugin,
        name: *const c_char,
        pri: c_int,
        callback: unsafe extern fn (
            word: *mut *mut c_char,
            word_eol: *mut *mut c_char,
            user_data: *mut c_void
        ) -> c_int,
        userdata: *mut c_void,
    ) -> *mut hexchat_hook;

    pub fn hexchat_hook_server_attrs(
        ph: *mut hexchat_plugin,
        name: *const c_char,
        pri: c_int,
        callback: unsafe extern fn (
            word: *mut *mut c_char,
            word_eol: *mut *mut c_char,
            attrs: *mut hexchat_event_attrs,
            user_data: *mut c_void,
        ) -> c_int,
        userdata: *mut c_void,
    ) -> *mut hexchat_hook;

    pub fn hexchat_hook_print(
        ph: *mut hexchat_plugin,
        name: *const c_char,
        pri: c_int,
        callback: unsafe extern fn (
            word: *mut *mut c_char,
            user_data: *mut c_void,
        ) -> c_int,
        userdata: *mut c_void,
    ) -> *mut hexchat_hook;

    pub fn hexchat_hook_print_attrs(
        ph: *mut hexchat_plugin,
        name: *const c_char,
        pri: c_int,
        callback: unsafe extern fn (
            word: *mut *mut c_char,
            attrs: *mut hexchat_event_attrs,
            user_data: *mut c_void,
        ) -> c_int,
        userdata: *mut c_void,
    ) -> *mut hexchat_hook;

    pub fn hexchat_hook_timer(
        ph: *mut hexchat_plugin,
        timeout: c_int,
        callback: unsafe extern fn (
            user_data: *mut c_void,
        ) -> c_int,
        userdata: *mut c_void,
    ) -> hexchat_hook;

    pub fn hexchat_hook_fd(
        ph: *mut hexchat_plugin,
        fd: c_int,
        flags: c_int,
        callback: unsafe extern fn (
            fd_: c_int,
            flags_: c_int,
            user_data: *mut c_void,
        ) -> c_int,
        userdata: *mut c_void,
    ) -> *mut hexchat_hook;

    pub fn hexchat_unhook(
        ph: *mut hexchat_plugin,
        hook: *mut hexchat_hook,
    ) -> *mut c_void;

    pub fn hexchat_print(
        ph: *mut hexchat_plugin,
        text: *const c_char,
    );

    pub fn hexchat_printf(
        ph: *mut hexchat_plugin,
        format: *const c_char,
        ...
    );

    pub fn hexchat_command(
        ph: *mut hexchat_plugin,
        command: *const c_char,
    );

    pub fn hexchat_commandf(
        ph: *mut hexchat_plugin,
        format: *const c_char,
        ...
    );

    pub fn hexchat_nickcmp(
        ph: *mut hexchat_plugin,
        s1: *const c_char,
        s2: *const c_char,
    ) -> c_int;

    pub fn hexchat_set_context(
        ph: *mut hexchat_plugin,
        ctx: *mut hexchat_context,
    ) -> c_int;

    pub fn hexchat_find_context(
        ph: *mut hexchat_plugin,
        servname: *const c_char,
        channel: *const c_char,
    ) -> *mut hexchat_context;

    pub fn hexchat_get_context(
        ph: *mut hexchat_plugin,
    ) -> *mut hexchat_context;

    pub fn hexchat_get_info(
        ph: *mut hexchat_plugin,
        id: *const c_char,
    ) -> *const c_char;

    pub fn hexchat_get_prefs(
        ph: *mut hexchat_plugin,
        name: *const c_char,
        string: *mut *const c_char,
        integer: *mut c_int,
    ) -> c_int;

    pub fn hexchat_list_get(
        ph: *mut hexchat_plugin,
        name: *const c_char,
    ) -> *mut hexchat_list;

    pub fn hexchat_list_free(
        ph: *mut hexchat_plugin,
        xlist: *mut hexchat_list,
    );

    pub fn hexchat_list_fields(
        ph: *mut hexchat_plugin,
        name: *const c_char,
    ) -> *const *const c_char;

    pub fn hexchat_list_next(
        ph: *mut hexchat_plugin,
        xlist: *mut hexchat_list,
    ) -> c_int;

    pub fn hexchat_list_str(
        ph: *mut hexchat_plugin,
        xlist: *mut hexchat_list,
        name: *const c_char,
    ) -> *const c_char;

    pub fn hexchat_list_int(
        ph: *mut hexchat_plugin,
        xlist: *mut hexchat_list,
        name: *const c_char,
    ) -> c_int;

    pub fn hexchat_list_time(
        ph: *mut hexchat_plugin,
        xlist: *mut hexchat_list,
        name: *const c_char,
    ) ->  time_t;

    pub fn hexchat_plugingui_add(
        ph: *mut hexchat_plugin,
        filename: *const c_char,
        name: *const c_char,
        desc: *const c_char,
        version: *const c_char,
        reserved: *const c_char,
    ) -> *mut c_void;

    pub fn hexchat_plugingui_remove(
        ph: *mut hexchat_plugin,
        handle: *mut c_void,
    );

    pub fn hexchat_emit_print(
        ph: *mut hexchat_plugin,
        name: *const c_char,
        ...
    ) -> c_int;

    pub fn hexchat_emit_print_attrs(
        ph: *mut hexchat_plugin,
        attrs: *mut hexchat_event_attrs,
        event_name: *const c_char,
        ...
    ) -> c_int;

    pub fn hexchat_gettext(
        ph: *mut hexchat_plugin,
        msgid: *const c_char,
    ) -> *mut c_char;

    pub fn hexchat_send_modes(
        ph: *mut hexchat_plugin,
        targets: *mut *const c_char,
        ntargets: c_int,
        modes_per_line: c_int,
        sign: c_char,
        mode: c_char,
    );

    pub fn hexchat_strip(
        ph: *mut hexchat_plugin,
        str: *const c_char,
        len: c_int,
        flags: c_int,
    ) -> *mut c_char;

    pub fn hexchat_free(
        ph: *mut hexchat_plugin,
        ptr: *mut c_void,
    );

    pub fn hexchat_pluginpref_set_str(
        ph: *mut hexchat_plugin,
        var: *const c_char,
        value: *const c_char,
    ) -> c_int;

    pub fn hexchat_pluginpref_get_str(
        ph: *mut hexchat_plugin,
        var: *const c_char,
        dest: *mut c_char,
    ) -> c_int;

    pub fn hexchat_pluginpref_set_int(
        ph: *mut hexchat_plugin,
        var: *const c_char,
        value: c_int,
    ) -> c_int;

    pub fn hexchat_pluginpref_get_int(
        ph: *mut hexchat_plugin,
        var: *const c_char,
    ) -> c_int;

    pub fn hexchat_pluginpref_delete(
        ph: *mut hexchat_plugin,
        var: *const c_char,
    ) -> c_int;

    pub fn hexchat_pluginpref_list(
        ph: *mut hexchat_plugin,
        dest: *const c_char,
    ) -> c_int;
}
