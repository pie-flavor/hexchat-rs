use std::ffi::CString;
use std::mem;
use std::ptr;

use chrono::{DateTime, TimeZone};

use crate::{c, from_cstring, to_cstring, ChannelRef, Context, PrintEvent};
use std::cmp::Ordering;

impl Context {
    /// Prints plain text to the current tab.
    pub fn print_plain(&self, text: &str) {
        let text = to_cstring(text);
        unsafe {
            c!(hexchat_print, self.handle, text.as_ptr());
        }
    }
    /// Prints a specific print event to the current tab. Returns whether or not it succeeded.
    pub fn print_event(&self, event: PrintEvent, arguments: &[impl AsRef<str>]) -> bool {
        let event = to_cstring(event.0);
        let res = unsafe {
            match arguments {
                [] => c!(
                    hexchat_emit_print,
                    self.handle,
                    event.as_ptr(),
                    ptr::null::<()>()
                ),
                [arg_1] => {
                    let arg_1 = to_cstring(arg_1.as_ref());
                    c!(
                        hexchat_emit_print,
                        self.handle,
                        event.as_ptr(),
                        arg_1.as_ptr(),
                        ptr::null::<()>(),
                    )
                }
                [arg_1, arg_2] => {
                    let (arg_1, arg_2) = (to_cstring(arg_1.as_ref()), to_cstring(arg_2.as_ref()));
                    c!(
                        hexchat_emit_print,
                        self.handle,
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
                        self.handle,
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
                        self.handle,
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
    /// Prints a specific print event to the current tab with a specified timestamp. Returns whether
    /// or not it succeeded.
    pub fn print_event_at(
        &self,
        event: PrintEvent,
        timestamp: &DateTime<impl TimeZone>,
        arguments: &[impl AsRef<str>],
    ) -> bool {
        unsafe {
            let event_attrs = c!(hexchat_event_attrs_create, self.handle);
            let unixtime = timestamp.timestamp();
            (*event_attrs).server_time_utc = unixtime;
            let event = to_cstring(event.0);
            let res = match arguments {
                [] => c!(
                    hexchat_emit_print_attrs,
                    self.handle,
                    event_attrs,
                    event.as_ptr(),
                    ptr::null::<()>(),
                ),
                [arg_1] => {
                    let arg_1 = to_cstring(arg_1.as_ref());
                    c!(
                        hexchat_emit_print_attrs,
                        self.handle,
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
                        self.handle,
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
                        self.handle,
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
                        self.handle,
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
            c!(hexchat_event_attrs_free, self.handle, event_attrs);
            res != 0
        }
    }
    /// Prints a specific print event to a particular `ChannelRef`. Returns whether or not it
    /// succeeded.
    pub fn print_event_to_channel(
        &self,
        channel: &ChannelRef,
        event: PrintEvent,
        args: &[impl AsRef<str>],
    ) -> bool {
        unsafe {
            let ctx = c!(hexchat_get_context, self.handle);
            if c!(hexchat_set_context, self.handle, channel.handle) == 0 {
                return false;
            }
            let res = self.print_event(event, args);
            if c!(hexchat_set_context, self.handle, ctx) == 0 {
                c!(
                    hexchat_set_context,
                    self.handle,
                    c!(hexchat_find_context, self.handle, ptr::null(), ptr::null()),
                );
            }
            res
        }
    }
    /// Prints a specific print event to a particular channel with a specified timestamp. Returns
    /// whether or not it succeeded.
    pub fn print_event_to_channel_at(
        &self,
        channel: &ChannelRef,
        event: PrintEvent,
        timestamp: &DateTime<impl TimeZone>,
        args: &[impl AsRef<str>],
    ) -> bool {
        unsafe {
            let ctx = c!(hexchat_get_context, self.handle);
            if c!(hexchat_set_context, self.handle, channel.handle) == 0 {
                return false;
            }
            let res = self.print_event_at(event, &timestamp, args);
            if c!(hexchat_set_context, self.handle, ctx) == 0 {
                c!(
                    hexchat_set_context,
                    self.handle,
                    c!(hexchat_find_context, self.handle, ptr::null(), ptr::null()),
                );
            }
            res
        }
    }
    /// Adds a user mode char to one or more users in the current channel. Returns whether or not it
    /// succeeded.
    pub fn add_modes(&self, targets: &[impl AsRef<str>], mode: char) -> bool {
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
                self.handle,
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
    /// Removes a user mode char from one or more users in the current channel. Returns whether or
    /// not it succeeded.
    pub fn remove_modes(&self, targets: &[impl AsRef<str>], mode: char) -> bool {
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
                self.handle,
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
    /// Adds a user mode char to one or more users in the specified channel. Returns whether or not
    /// it succeeded.
    pub fn add_modes_in_channel(
        &self,
        targets: &[impl AsRef<str>],
        mode: char,
        channel: &ChannelRef,
    ) -> bool {
        unsafe {
            let ctx = c!(hexchat_get_context, self.handle);
            if c!(hexchat_set_context, self.handle, channel.handle) == 0 {
                return false;
            }
            let res = self.add_modes(targets, mode);
            if c!(hexchat_set_context, self.handle, ctx) == 0 {
                c!(
                    hexchat_set_context,
                    self.handle,
                    c!(hexchat_find_context, self.handle, ptr::null(), ptr::null()),
                );
            }
            res
        }
    }
    /// Removes a user mode char from one or more users in the specified channel. Returns whether or
    /// not it succeeded.
    pub fn remove_modes_in_channel(
        &self,
        targets: &[impl AsRef<str>],
        mode: char,
        channel: &ChannelRef,
    ) -> bool {
        unsafe {
            let ctx = c!(hexchat_get_context, self.handle);
            if c!(hexchat_set_context, self.handle, channel.handle) == 0 {
                return false;
            }
            let res = self.remove_modes(targets, mode);
            if c!(hexchat_set_context, self.handle, ctx) == 0 {
                c!(
                    hexchat_set_context,
                    self.handle,
                    c!(hexchat_find_context, self.handle, ptr::null(), ptr::null()),
                );
            }
            res
        }
    }
    /// Compares two names (nicks, channel names, etc.) according to IRC comparison rules.
    pub fn name_cmp(&self, nick1: &str, nick2: &str) -> Ordering {
        let nick1 = to_cstring(nick1);
        let nick2 = to_cstring(nick2);
        let res = unsafe { c!(hexchat_nickcmp, self.handle, nick1.as_ptr(), nick2.as_ptr()) };
        res.cmp(&0)
    }
    /// Strips color characters from a string. Returns the stripped string, or `Err` if the
    /// color characters are malformed.
    pub fn strip_colors(&self, string: &str) -> Result<String, ()> {
        self.strip(string, STRIP_COLORS)
    }
    /// Strips non-color formatting characters from a string. Returns the stripped string, or `Err`
    /// if the formatting characters are malformed.
    pub fn strip_attributes(&self, string: &str) -> Result<String, ()> {
        self.strip(string, STRIP_ATTRIBUTES)
    }
    /// Strips all formatting characters from a string. Returns the stripped string, or `Err` if the
    /// formatting characters are malformed.
    pub fn strip_formatting(&self, string: &str) -> Result<String, ()> {
        self.strip(string, STRIP_ALL)
    }
    fn strip(&self, string: &str, mode: i32) -> Result<String, ()> {
        let stripped = unsafe {
            c!(
                hexchat_strip,
                self.handle,
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
                c!(hexchat_free, self.handle, stripped as _);
            }
            Ok(res)
        }
    }
    /// Strips color characters from a string and puts the result back into the string. Returns `Ok`
    /// if it succeeded or `Err` if the color characters are malformed.
    pub fn strip_colors_in_place(&self, string: &mut String) -> Result<(), ()> {
        self.strip_in_place(string, STRIP_COLORS)
    }
    /// Strips non-color formatting characters from a string and puts the result back into the
    /// string. Returns `Ok` if it succeeded or `Err` if the formatting characters are malformed.
    pub fn strip_attributes_in_place(&self, string: &mut String) -> Result<(), ()> {
        self.strip_in_place(string, STRIP_ATTRIBUTES)
    }
    /// Strips all formatting characters from a string and puts the result back into the string.
    /// Returns `Ok` if it succeeded or `Err` if the formatting characters are malformed
    pub fn strip_formatting_in_place(&self, string: &mut String) -> Result<(), ()> {
        self.strip_in_place(string, STRIP_ALL)
    }
    fn strip_in_place(&self, string: &mut String, mode: i32) -> Result<(), ()> {
        let stripped = unsafe {
            c!(
                hexchat_strip,
                self.handle,
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
                c!(hexchat_free, self.handle, stripped as _);
            }
            Ok(())
        }
    }
}

const STRIP_COLORS: i32 = 1;
const STRIP_ATTRIBUTES: i32 = 2;
const STRIP_ALL: i32 = 3;
