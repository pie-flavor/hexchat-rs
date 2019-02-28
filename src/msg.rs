use std::ffi::CString;
use std::mem;
use std::ptr;

use chrono::{DateTime, TimeZone};

use crate::{c, from_cstring, to_cstring, ChannelRef, PrintEvent};
use std::cmp::Ordering;

/// Prints plain text to the current tab.
pub fn print_plain(text: &str) {
    let text = to_cstring(text);
    unsafe {
        c!(hexchat_print, text.as_ptr());
    }
}
/// Prints a specific print event to the current tab.
///
/// Returns whether or not it succeeded.
pub fn print_event(event: PrintEvent, arguments: &[impl AsRef<str>]) -> bool {
    let event = to_cstring(event.0);
    let res = unsafe {
        match arguments {
            [] => c!(hexchat_emit_print, event.as_ptr(), ptr::null::<()>()),
            [arg_1] => {
                let arg_1 = to_cstring(arg_1.as_ref());
                c!(
                    hexchat_emit_print,
                    event.as_ptr(),
                    arg_1.as_ptr(),
                    ptr::null::<()>(),
                )
            }
            [arg_1, arg_2] => {
                let (arg_1, arg_2) = (to_cstring(arg_1.as_ref()), to_cstring(arg_2.as_ref()));
                c!(
                    hexchat_emit_print,
                    event.as_ptr(),
                    arg_1.as_ptr(),
                    arg_2.as_ptr(),
                    ptr::null::<()>(),
                )
            }
            [arg_1, arg_2, arg_3] => {
                let (arg_1, arg_2, arg_3) = (
                    to_cstring(arg_1.as_ref()),
                    to_cstring(arg_2.as_ref()),
                    to_cstring(arg_3.as_ref()),
                );
                c!(
                    hexchat_emit_print,
                    event.as_ptr(),
                    arg_1.as_ptr(),
                    arg_2.as_ptr(),
                    arg_3.as_ptr(),
                    ptr::null::<()>(),
                )
            }
            [arg_1, arg_2, arg_3, arg_4, ..] => {
                let (arg_1, arg_2, arg_3, arg_4) = (
                    to_cstring(arg_1.as_ref()),
                    to_cstring(arg_2.as_ref()),
                    to_cstring(arg_3.as_ref()),
                    to_cstring(arg_4.as_ref()),
                );
                c!(
                    hexchat_emit_print,
                    event.as_ptr(),
                    arg_1.as_ptr(),
                    arg_2.as_ptr(),
                    arg_3.as_ptr(),
                    arg_4.as_ptr(),
                    ptr::null::<()>(),
                )
            }
        }
    };
    res != 0
}
/// Prints a specific print event to the current tab with a specified timestamp.
///
/// Returns whether or not it succeeded.
pub fn print_event_at(
    event: PrintEvent,
    timestamp: &DateTime<impl TimeZone>,
    arguments: &[impl AsRef<str>],
) -> bool {
    unsafe {
        let event_attrs = c!(hexchat_event_attrs_create);
        let unixtime = timestamp.timestamp();
        (*event_attrs).server_time_utc = unixtime;
        let event = to_cstring(event.0);
        let res = match arguments {
            [] => c!(
                hexchat_emit_print_attrs,
                event_attrs,
                event.as_ptr(),
                ptr::null::<()>(),
            ),
            [arg_1] => {
                let arg_1 = to_cstring(arg_1.as_ref());
                c!(
                    hexchat_emit_print_attrs,
                    event_attrs,
                    event.as_ptr(),
                    arg_1.as_ptr(),
                    ptr::null::<()>(),
                )
            }
            [arg_1, arg_2] => {
                let (arg_1, arg_2) = (to_cstring(arg_1.as_ref()), to_cstring(arg_2.as_ref()));
                c!(
                    hexchat_emit_print_attrs,
                    event_attrs,
                    event.as_ptr(),
                    arg_1.as_ptr(),
                    arg_2.as_ptr(),
                    ptr::null::<()>(),
                )
            }
            [arg_1, arg_2, arg_3] => {
                let (arg_1, arg_2, arg_3) = (
                    to_cstring(arg_1.as_ref()),
                    to_cstring(arg_2.as_ref()),
                    to_cstring(arg_3.as_ref()),
                );
                c!(
                    hexchat_emit_print_attrs,
                    event_attrs,
                    event.as_ptr(),
                    arg_1.as_ptr(),
                    arg_2.as_ptr(),
                    arg_3.as_ptr(),
                    ptr::null::<()>(),
                )
            }
            [arg_1, arg_2, arg_3, arg_4, ..] => {
                let (arg_1, arg_2, arg_3, arg_4) = (
                    to_cstring(arg_1.as_ref()),
                    to_cstring(arg_2.as_ref()),
                    to_cstring(arg_3.as_ref()),
                    to_cstring(arg_4.as_ref()),
                );
                c!(
                    hexchat_emit_print_attrs,
                    event_attrs,
                    event.as_ptr(),
                    arg_1.as_ptr(),
                    arg_2.as_ptr(),
                    arg_3.as_ptr(),
                    arg_4.as_ptr(),
                    ptr::null::<()>(),
                )
            }
        };
        c!(hexchat_event_attrs_free, event_attrs);
        res != 0
    }
}
/// Prints a specific print event to a particular `ChannelRef`.
///
/// Returns whether or not it succeeded.
pub fn print_event_to_channel(
    channel: &ChannelRef,
    event: PrintEvent,
    args: &[impl AsRef<str>],
) -> bool {
    unsafe {
        let ctx = c!(hexchat_get_context);
        if c!(hexchat_set_context, channel.handle) == 0 {
            return false;
        }
        let res = print_event(event, args);
        if c!(hexchat_set_context, ctx) == 0 {
            c!(
                hexchat_set_context,
                c!(hexchat_find_context, ptr::null(), ptr::null()),
            );
        }
        res
    }
}
/// Prints a specific print event to a particular channel with a specified timestamp.
///
/// Returns whether or not it succeeded.
pub fn print_event_to_channel_at(
    channel: &ChannelRef,
    event: PrintEvent,
    timestamp: &DateTime<impl TimeZone>,
    args: &[impl AsRef<str>],
) -> bool {
    unsafe {
        let ctx = c!(hexchat_get_context);
        if c!(hexchat_set_context, channel.handle) == 0 {
            return false;
        }
        let res = print_event_at(event, &timestamp, args);
        if c!(hexchat_set_context, ctx) == 0 {
            c!(
                hexchat_set_context,
                c!(hexchat_find_context, ptr::null(), ptr::null()),
            );
        }
        res
    }
}
/// Adds a user mode char to one or more users in the current channel.
///
/// Returns whether or not it succeeded.
pub fn add_modes(targets: &[impl AsRef<str>], mode: char) -> bool {
    if !mode.is_ascii() {
        return false;
    }
    let len = targets.len();
    let ptrs = targets
        .iter()
        .map(|x| to_cstring(x.as_ref()).into_raw())
        .collect::<Box<[_]>>();
    let ptr_ptr = Box::into_raw(ptrs);
    unsafe {
        c!(
            hexchat_send_modes,
            ptr_ptr as _,
            len as _,
            0,
            mem::transmute(b'+'),
            mem::transmute(mode as u8),
        );
        let ptrs = Box::from_raw(ptr_ptr);
        ptrs.into_vec().into_iter().for_each(|x| {
            CString::from_raw(x);
        });
    }
    true
}
/// Removes a user mode char from one or more users in the current channel.
///
/// Returns whether or not it succeeded.
pub fn remove_modes(targets: &[impl AsRef<str>], mode: char) -> bool {
    if !mode.is_ascii() {
        return false;
    }
    let ptrs = targets
        .iter()
        .map(|x| to_cstring(x.as_ref()).into_raw())
        .collect::<Box<[_]>>();
    let ptr_ptr = Box::into_raw(ptrs);
    unsafe {
        c!(
            hexchat_send_modes,
            ptr_ptr as _,
            targets.len() as _,
            0,
            mem::transmute(b'-'),
            mem::transmute(mode as u8),
        );
        let ptrs = Box::from_raw(ptr_ptr);
        ptrs.into_vec().into_iter().for_each(|x| {
            CString::from_raw(x);
        });
    }
    true
}
/// Adds a user mode char to one or more users in the specified channel.
///
/// Returns whether or not it succeeded.
pub fn add_modes_in_channel(targets: &[impl AsRef<str>], mode: char, channel: &ChannelRef) -> bool {
    unsafe {
        let ctx = c!(hexchat_get_context);
        if c!(hexchat_set_context, channel.handle) == 0 {
            return false;
        }
        let res = add_modes(targets, mode);
        if c!(hexchat_set_context, ctx) == 0 {
            c!(
                hexchat_set_context,
                c!(hexchat_find_context, ptr::null(), ptr::null()),
            );
        }
        res
    }
}
/// Removes a user mode char from one or more users in the specified channel.
///
/// Returns whether or not it succeeded.
pub fn remove_modes_in_channel(
    targets: &[impl AsRef<str>],
    mode: char,
    channel: &ChannelRef,
) -> bool {
    unsafe {
        let ctx = c!(hexchat_get_context);
        if c!(hexchat_set_context, channel.handle) == 0 {
            return false;
        }
        let res = remove_modes(targets, mode);
        if c!(hexchat_set_context, ctx) == 0 {
            c!(
                hexchat_set_context,
                c!(hexchat_find_context, ptr::null(), ptr::null()),
            );
        }
        res
    }
}
/// Compares two names (nicks, channel names, etc.) according to IRC comparison rules.
pub fn name_cmp(nick1: &str, nick2: &str) -> Ordering {
    let nick1 = to_cstring(nick1);
    let nick2 = to_cstring(nick2);
    let res = unsafe { c!(hexchat_nickcmp, nick1.as_ptr(), nick2.as_ptr()) };
    res.cmp(&0)
}
/// Strips color characters from a string.
///
/// Returns the stripped string, or `Err` if the color characters are malformed.
pub fn strip_colors(string: &str) -> Result<String, ()> {
    strip(string, STRIP_COLORS)
}
/// Strips non-color formatting characters from a string.
///
/// Returns the stripped string, or `Err` if the formatting characters are malformed.
pub fn strip_attributes(string: &str) -> Result<String, ()> {
    strip(string, STRIP_ATTRIBUTES)
}
/// Strips all formatting characters from a string.
///
/// Returns the stripped string, or `Err` if the formatting characters are malformed.
pub fn strip_formatting(string: &str) -> Result<String, ()> {
    strip(string, STRIP_ALL)
}
fn strip(string: &str, mode: i32) -> Result<String, ()> {
    let stripped = unsafe {
        c!(
            hexchat_strip,
            string.as_bytes() as *const [u8] as _,
            string.len() as _,
            mode,
        )
    };
    if stripped.is_null() {
        Err(())
    } else {
        let stripped_string = unsafe { from_cstring(stripped) };
        let res = stripped_string.to_string();
        unsafe {
            c!(hexchat_free, stripped as _);
        }
        Ok(res)
    }
}
/// Strips color characters from a string and puts the result back into the string.
/// Returns `Ok` if it succeeded or `Err` if the color characters are malformed.
pub fn strip_colors_in_place(string: &mut String) -> Result<(), ()> {
    strip_in_place(string, STRIP_COLORS)
}
/// Strips non-color formatting characters from a string and puts the result back into the
/// string.
///
/// Returns `Ok` if it succeeded or `Err` if the formatting characters are malformed.
pub fn strip_attributes_in_place(string: &mut String) -> Result<(), ()> {
    strip_in_place(string, STRIP_ATTRIBUTES)
}
/// Strips all formatting characters from a string and puts the result back into the string.
///
/// Returns `Ok` if it succeeded or `Err` if the formatting characters are malformed
pub fn strip_formatting_in_place(string: &mut String) -> Result<(), ()> {
    strip_in_place(string, STRIP_ALL)
}
fn strip_in_place(string: &mut String, mode: i32) -> Result<(), ()> {
    let stripped = unsafe {
        c!(
            hexchat_strip,
            string.as_bytes() as *const [u8] as _,
            string.len() as _,
            mode,
        )
    };
    if stripped.is_null() {
        Err(())
    } else {
        let stripped_string = unsafe { from_cstring(stripped) };
        string.clear();
        string.push_str(stripped_string.as_str());
        unsafe {
            c!(hexchat_free, stripped as _);
        }
        Ok(())
    }
}

const STRIP_COLORS: i32 = 1;
const STRIP_ATTRIBUTES: i32 = 2;
const STRIP_ALL: i32 = 3;
