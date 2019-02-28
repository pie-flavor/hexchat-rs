use crate::{c, from_cstring, to_cstring};
use std::ffi::CStr;
use std::os::raw::c_char;
use std::ptr;

/// Gets a HexChat user preference by name. Returns the preference if found, or `None` if no
/// preference by that name exists.
pub fn get_global_pref(name: &str) -> Option<GlobalPreferenceValue> {
    let mut res_str = ptr::null();
    let mut res_int = 0;
    let name = to_cstring(name);
    let res = unsafe { c!(hexchat_get_prefs, name.as_ptr(), &mut res_str, &mut res_int) };
    match res {
        1 => Some(GlobalPreferenceValue::String(unsafe {
            from_cstring(res_str)
        })),
        2 => Some(GlobalPreferenceValue::Int(res_int as _)),
        3 => Some(GlobalPreferenceValue::Bool(res_int != 0)),
        _ => None,
    }
}
/// Gets the current cursor position in the text box, or `None` if it's deselected.
pub fn get_cursor_pos() -> Option<usize> {
    let mut res_int = 0;
    let mut res_str = ptr::null();
    let cursor_pos = to_cstring(CURSOR_POS);
    let res = unsafe {
        c!(
            hexchat_get_prefs,
            cursor_pos.as_ptr(),
            &mut res_str,
            &mut res_int
        )
    };
    match res {
        2 => Some(res_int as usize),
        _ => None,
    }
}
/// Gets the unique ID of the current server, or `None` if unknown.
pub fn get_server_id() -> Option<i32> {
    let mut res_int = 0;
    let mut res_str = ptr::null();
    let server_id = to_cstring(SERVER_ID);
    let res = unsafe {
        c!(
            hexchat_get_prefs,
            server_id.as_ptr(),
            &mut res_str,
            &mut res_int
        )
    };
    match res {
        2 => Some(res_int as _),
        _ => None,
    }
}
/// Saves a plugin preference string to file. Returns `Ok` if it succeeded, or `Err` if there
/// was an IO error.
pub fn set_pref_string(name: &str, value: &str) -> Result<(), ()> {
    let name = to_cstring(name);
    let value = to_cstring(value);
    let res = unsafe { c!(hexchat_pluginpref_set_str, name.as_ptr(), value.as_ptr()) };
    if res == 0 {
        Err(())
    } else {
        Ok(())
    }
}
/// Saves a plugin preference integer to file. Returns `Ok` if it succeeeded, or `Err` if there
/// was an IO error.
pub fn set_pref_int(name: &str, value: u32) -> Result<(), ()> {
    let name = to_cstring(name);
    let res = unsafe { c!(hexchat_pluginpref_set_int, name.as_ptr(), value as _) };
    if res == 0 {
        Err(())
    } else {
        Ok(())
    }
}
/// Gets a plugin preference string from file. Returns the result, or `None` if no such
/// preference exists or there was an IO error.
pub fn get_pref_string(name: &str) -> Option<String> {
    let name = to_cstring(name);
    let mut string_buf = [0 as c_char; 512];
    let res = unsafe {
        c!(
            hexchat_pluginpref_get_str,
            name.as_ptr(),
            &mut string_buf as *mut [c_char] as _,
        )
    };
    if res == 0 {
        None
    } else {
        Some(unsafe { from_cstring(&string_buf as *const [c_char] as _) }.to_string())
    }
}
/// Gets a plugin preference integer from file. Returns the result, or `None` if no such
/// preference exists or there was an IO error.
pub fn get_pref_int(name: &str) -> Option<u32> {
    let name = to_cstring(name);
    let res = unsafe { c!(hexchat_pluginpref_get_int, name.as_ptr()) };
    if res == -1 {
        None
    } else {
        Some(res as _)
    }
}
/// Deletes a plugin preference from the preferences file. Returns `Ok` if it succeeded or `Err`
/// if no such preference exists or there was an IO error.
pub fn delete_pref(name: &str) -> Result<(), ()> {
    let name = to_cstring(name);
    let res = unsafe { c!(hexchat_pluginpref_delete, name.as_ptr()) };
    if res == 0 {
        Err(())
    } else {
        Ok(())
    }
}
/// Gets every plugin preference name that has been saved.
pub fn get_prefs() -> Vec<String> {
    let mut buf = [0 as c_char; 4096];
    unsafe {
        c!(hexchat_pluginpref_list, &mut buf as *mut [c_char] as _);
    }
    let list = unsafe { CStr::from_ptr(&buf as *const [c_char] as _).to_string_lossy() };
    list.split(',').map(ToString::to_string).collect()
}

const CURSOR_POS: &str = "state_cursor";
const SERVER_ID: &str = "id";

/// Possible values from `Context::get_global_pref`.
pub enum GlobalPreferenceValue {
    /// A boolean value.
    Bool(bool),
    /// An integer value.
    Int(i32),
    /// A string value.
    String(String),
}
