use crate::other::PrintEvent;
use crate::{c, from_cstring, from_cstring_opt, to_cstring, Context};
use charsets::Charset;
use lazy_static::lazy_static;
use std::ffi::{CStr, CString};
use std::path::PathBuf;
use std::str::FromStr;

impl Context {
    /// Gets the client's currently set away reason, or `None` if the client is not away.
    pub fn get_away_reason(&self) -> Option<String> {
        unsafe { from_cstring_opt(c!(hexchat_get_info, self.handle, AWAY.as_ptr())) }
    }
    /// Gets whether or not the client is currently away.
    pub fn is_away(&self) -> bool {
        unsafe { c!(hexchat_get_info, self.handle, AWAY.as_ptr()).is_null() }
    }
    /// Gets the name of the current channel.
    pub fn get_channel_name(&self) -> String {
        unsafe { from_cstring(c!(hexchat_get_info, self.handle, CHANNEL.as_ptr())) }
    }
    /// Gets the current charset in use.
    pub fn get_charset(&self) -> Charset {
        unsafe {
            Charset::from_str(
                &CStr::from_ptr(c!(hexchat_get_info, self.handle, CHARSET.as_ptr()))
                    .to_string_lossy(),
            )
            .unwrap()
        }
    }
    /// Gets HexChat's configuration directory.
    pub fn get_config_dir(&self) -> PathBuf {
        unsafe {
            PathBuf::from(
                &*CStr::from_ptr(c!(hexchat_get_info, self.handle, CONFIG_DIR.as_ptr()))
                    .to_string_lossy(),
            )
        }
    }
    /// Gets the format string that gets printed to the window when the specified `PrintEvent` is
    /// fired.
    pub fn get_event_format_string(&self, event: PrintEvent) -> String {
        unsafe {
            let mut id = String::with_capacity(11 + event.0.len());
            //event text string for every single event?
            id.push_str("event_text ");
            id.push_str(event.0);
            let id = to_cstring(&id);
            from_cstring(c!(hexchat_get_info, self.handle, id.as_ptr()))
        }
    }
    /// Gets the client's current hostname.
    pub fn get_hostname(&self) -> String {
        unsafe { from_cstring(c!(hexchat_get_info, self.handle, HOST.as_ptr())) }
    }
    /// Gets the current contents of the input box.
    pub fn get_inputbox_contents(&self) -> String {
        unsafe { from_cstring(c!(hexchat_get_info, self.handle, INPUT_BOX.as_ptr())) }
    }
    /// Gets HexChat's library directory, also known as the plugin directory.
    pub fn get_hexchat_library_dir(&self) -> PathBuf {
        unsafe {
            PathBuf::from(
                &*CStr::from_ptr(c!(hexchat_get_info, self.handle, LIB_DIR_FS.as_ptr()))
                    .to_string_lossy(),
            )
        }
    }
    /// Gets the channel mode string for the current channel, or `None` if unknown.
    pub fn get_channel_mode_string(&self) -> Option<String> {
        unsafe { from_cstring_opt(c!(hexchat_get_info, self.handle, MODES.as_ptr())) }
    }
    /// Gets the name of the current server network, or `None` if unknown.
    pub fn get_network_name(&self) -> Option<String> {
        unsafe { from_cstring_opt(c!(hexchat_get_info, self.handle, NETWORK.as_ptr())) }
    }
    /// Gets the nickname in use on the current server.
    pub fn get_nickname(&self) -> String {
        unsafe { from_cstring(c!(hexchat_get_info, self.handle, NICK.as_ptr())) }
    }
    /// Gets the NickServ password for the current server, or `None` if none is set.
    pub fn get_nickserv_password(&self) -> Option<String> {
        unsafe { from_cstring_opt(c!(hexchat_get_info, self.handle, NICKSERV.as_ptr())) }
    }
    /// Gets the name of the current server, or `None` if unknown.
    pub fn get_server_name(&self) -> Option<String> {
        unsafe { from_cstring_opt(c!(hexchat_get_info, self.handle, SERVER.as_ptr())) }
    }
    /// Gets the topic of the current channel.
    pub fn get_channel_topic(&self) -> String {
        unsafe { from_cstring(c!(hexchat_get_info, self.handle, TOPIC.as_ptr())) }
    }
    /// Gets the version string of the build of Hexchat you're running on.
    pub fn get_hexchat_version(&self) -> String {
        unsafe { from_cstring(c!(hexchat_get_info, self.handle, VERSION.as_ptr())) }
    }
}

#[cfg(feature = "window")]
impl Context {
    /// Gets the GTK window.
    pub fn get_window(&self) -> gtk::Window {
        use std::marker::PhantomData;
        unsafe {
            let ptr = c!(hexchat_get_info, self.handle, GTK_WIN_PTR.as_ptr());
            let ptr = ptr as *const gtk_sys::GtkWindow;
            gtk::Window(glib::translate::from_glib_borrow(ptr), PhantomData)
        }
    }
    /// Gets the raw `GtkWindow` pointer.
    pub unsafe fn get_window_handle(&self) -> *const gtk_sys::GtkWindow {
        c!(hexchat_get_info, self.handle, GTK_WIN_PTR.as_ptr()) as *const gtk_sys::GtkWindow
    }
    /// Gets the status of the window.
    pub fn get_window_status(&self) -> WindowStatus {
        let cow = unsafe { CStr::from_ptr(c!(hexchat_get_info, self.handle, WIN_STATUS.as_ptr())) }
            .to_string_lossy();
        match &*cow {
            "active" => WindowStatus::Active,
            "hidden" => WindowStatus::Hidden,
            "normal" => WindowStatus::Normal,
        }
    }
    /// Gets the raw `HWND` pointer.
    #[cfg(windows)]
    pub unsafe fn get_os_window_handle(&self) -> winapi::shared::windef::HWND {
        c!(hexchat_get_info, self.handle, WIN_PTR.as_ptr()) as winapi::shared::windef::HWND
    }
    /// Gets the raw `GtkWindow` pointer.
    #[cfg(not(windows))]
    pub unsafe fn get_os_window_handle(&self) -> *const gtk_sys::GtkWindow {
        self.get_window_handle()
    }
    /// Gets the GTK window.
    #[cfg(not(windows))]
    pub fn get_os_window(&self) -> gtk::Window {
        self.get_window()
    }
}

/// The possible statuses of the HexChat window.
#[cfg(feature = "window")]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum WindowStatus {
    /// The window is currently active.
    Active,
    /// The window is currently minimized.
    Hidden,
    /// The window is currently backgrounded.
    Normal,
}

lazy_static! {
    static ref AWAY: CString = CString::new("away").unwrap();
    static ref CHANNEL: CString = CString::new("channel").unwrap();
    static ref CHARSET: CString = CString::new("charset").unwrap();
    static ref CONFIG_DIR: CString = CString::new("configdir").unwrap();
    static ref HOST: CString = CString::new("host").unwrap();
    static ref INPUT_BOX: CString = CString::new("inputbox").unwrap();
    static ref LIB_DIR_FS: CString = CString::new("libdirfs").unwrap();
    static ref MODES: CString = CString::new("modes").unwrap();
    static ref NETWORK: CString = CString::new("network").unwrap();
    static ref NICK: CString = CString::new("nick").unwrap();
    static ref NICKSERV: CString = CString::new("nickserv").unwrap();
    static ref SERVER: CString = CString::new("server").unwrap();
    static ref TOPIC: CString = CString::new("topic").unwrap();
    static ref VERSION: CString = CString::new("version").unwrap();
}

#[cfg(feature = "window")]
lazy_static! {
    static ref WIN_PTR: CString = CString::new("win_ptr").unwrap();
    static ref WIN_STATUS: CString = CString::new("win_status").unwrap();
    static ref GTK_WIN_PTR: CString = CString::new("gtkwin_ptr").unwrap();
}
