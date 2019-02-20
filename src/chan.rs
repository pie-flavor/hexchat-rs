use crate::{c, to_cstring, ChannelRef, Context};
use std::ptr;

impl Context {
    /// Gets the current channel context.
    pub fn get_current_channel(&self) -> ChannelRef {
        ChannelRef {
            ph: self.handle,
            handle: unsafe { c!(hexchat_get_context, self.handle) },
        }
    }
    /// Gets the channel that's currently focused in the HexChat window. Returns the `ChannelRef` if
    /// found, or `None` if none exists.
    pub fn get_focused_channel(&self) -> Option<ChannelRef> {
        let handle = unsafe { c!(hexchat_find_context, self.handle, ptr::null(), ptr::null()) };
        if handle.is_null() {
            None
        } else {
            Some(ChannelRef {
                ph: self.handle,
                handle,
            })
        }
    }
    /// Gets the frontmost channel in a particular server. Returns the `ChannelRef` if found, or
    /// `None` if none exists.
    pub fn get_focused_channel_in_server(&self, server_name: &str) -> Option<ChannelRef> {
        let server_name = to_cstring(server_name);
        let handle = unsafe {
            c!(
                hexchat_find_context,
                self.handle,
                server_name.as_ptr(),
                ptr::null()
            )
        };
        if handle.is_null() {
            None
        } else {
            Some(ChannelRef {
                ph: self.handle,
                handle,
            })
        }
    }
    /// Gets the first channel with the specified name in any server. Returns the `ChannelRef` if
    /// found, or `None` if none exists.
    pub fn get_first_channel(&self, channel_name: &str) -> Option<ChannelRef> {
        let channel_name = to_cstring(channel_name);
        let handle = unsafe {
            c!(
                hexchat_find_context,
                self.handle,
                ptr::null(),
                channel_name.as_ptr()
            )
        };
        if handle.is_null() {
            None
        } else {
            Some(ChannelRef {
                ph: self.handle,
                handle,
            })
        }
    }
    /// Gets the first channel with the specified name in the specified server. Returns the
    /// `ChannelRef` if found, or `None` if none exists.
    pub fn get_channel(&self, server_name: &str, channel_name: &str) -> Option<ChannelRef> {
        let channel_name = to_cstring(channel_name);
        let server_name = to_cstring(server_name);
        let handle = unsafe {
            c!(
                hexchat_find_context,
                self.handle,
                server_name.as_ptr(),
                channel_name.as_ptr()
            )
        };
        if handle.is_null() {
            None
        } else {
            Some(ChannelRef {
                ph: self.handle,
                handle,
            })
        }
    }
}
