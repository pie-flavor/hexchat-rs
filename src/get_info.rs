use crate::other::PrintEvent;
use crate::{c, from_cstring, from_cstring_opt, to_cstring, Context};
use charsets::Charset;
use std::ffi::CStr;
use std::path::PathBuf;
use std::str::FromStr;

impl Context {
    /// Gets the client's currently set away reason, or `None` if the client is not away.
    pub fn get_away_reason(&self) -> Option<String> {
        let away = to_cstring(AWAY);
        unsafe { from_cstring_opt(c!(hexchat_get_info, self.handle, away.as_ptr())) }
    }
    /// Gets whether or not the client is currently away.
    pub fn is_away(&self) -> bool {
        let away = to_cstring(AWAY);
        unsafe { c!(hexchat_get_info, self.handle, away.as_ptr()).is_null() }
    }
    /// Gets the name of the current channel.
    pub fn get_channel_name(&self) -> String {
        let channel = to_cstring(CHANNEL);
        unsafe { from_cstring(c!(hexchat_get_info, self.handle, channel.as_ptr())) }
    }
    /// Gets the current charset in use.
    pub fn get_charset(&self) -> Charset {
        let charset = to_cstring(CHARSET);
        unsafe {
            Charset::from_str(
                &CStr::from_ptr(c!(hexchat_get_info, self.handle, charset.as_ptr()))
                    .to_string_lossy(),
            )
            .unwrap()
        }
    }
    /// Gets HexChat's configuration directory.
    pub fn get_config_dir(&self) -> PathBuf {
        let config_dir = to_cstring(CONFIG_DIR);
        unsafe {
            PathBuf::from(
                &*CStr::from_ptr(c!(hexchat_get_info, self.handle, config_dir.as_ptr()))
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
        let host = to_cstring(HOST);
        unsafe { from_cstring(c!(hexchat_get_info, self.handle, host.as_ptr())) }
    }
    /// Gets the current contents of the input box.
    pub fn get_inputbox_contents(&self) -> String {
        let input_box = to_cstring(INPUT_BOX);
        unsafe { from_cstring(c!(hexchat_get_info, self.handle, input_box.as_ptr())) }
    }
    /// Gets HexChat's library directory, also known as the plugin directory.
    pub fn get_hexchat_library_dir(&self) -> PathBuf {
        let lib_dir_fs = to_cstring(LIB_DIR_FS);
        unsafe {
            PathBuf::from(
                &*CStr::from_ptr(c!(hexchat_get_info, self.handle, lib_dir_fs.as_ptr()))
                    .to_string_lossy(),
            )
        }
    }
    /// Gets the channel mode string for the current channel, or `None` if unknown.
    pub fn get_channel_mode_string(&self) -> Option<String> {
        let modes = to_cstring(MODES);
        unsafe { from_cstring_opt(c!(hexchat_get_info, self.handle, modes.as_ptr())) }
    }
    /// Gets the name of the current server network, or `None` if unknown.
    pub fn get_network_name(&self) -> Option<String> {
        let network = to_cstring(NETWORK);
        unsafe { from_cstring_opt(c!(hexchat_get_info, self.handle, network.as_ptr())) }
    }
    /// Gets the nickname in use on the current server.
    pub fn get_nickname(&self) -> String {
        let nick = to_cstring(NICK);
        unsafe { from_cstring(c!(hexchat_get_info, self.handle, nick.as_ptr())) }
    }
    /// Gets the NickServ password for the current server, or `None` if none is set.
    pub fn get_nickserv_password(&self) -> Option<String> {
        let nickserv = to_cstring(NICKSERV);
        unsafe { from_cstring_opt(c!(hexchat_get_info, self.handle, nickserv.as_ptr())) }
    }
    /// Gets the name of the current server, or `None` if unknown.
    pub fn get_server_name(&self) -> Option<String> {
        let server = to_cstring(SERVER);
        unsafe { from_cstring_opt(c!(hexchat_get_info, self.handle, server.as_ptr())) }
    }
    /// Gets the topic of the current channel.
    pub fn get_channel_topic(&self) -> String {
        let topic = to_cstring(TOPIC);
        unsafe { from_cstring(c!(hexchat_get_info, self.handle, topic.as_ptr())) }
    }
    /// Gets the version string of the build of Hexchat you're running on.
    pub fn get_hexchat_version(&self) -> String {
        let version = to_cstring(VERSION);
        unsafe { from_cstring(c!(hexchat_get_info, self.handle, version.as_ptr())) }
    }
}

#[cfg(feature = "window")]
impl Context {
    /// Gets the GTK window.
    pub fn get_window(&self) -> gtk::Window {
        use std::marker::PhantomData;
        let gtk_win_ptr = to_cstring(GTK_WIN_PTR);
        unsafe {
            let ptr = c!(hexchat_get_info, self.handle, gtk_win_ptr.as_ptr());
            let ptr = ptr as *const gtk_sys::GtkWindow;
            gtk::Window(glib::translate::from_glib_borrow(ptr), PhantomData)
        }
    }
    /// Gets the raw `GtkWindow` pointer.
    pub unsafe fn get_window_handle(&self) -> *const gtk_sys::GtkWindow {
        let gtk_win_ptr = to_cstring(GTK_WIN_PTR);
        c!(hexchat_get_info, self.handle, gtk_win_ptr.as_ptr()) as *const gtk_sys::GtkWindow
    }
    /// Gets the status of the window.
    pub fn get_window_status(&self) -> WindowStatus {
        let win_status = to_cstring(WIN_STATUS);
        let cow = unsafe { CStr::from_ptr(c!(hexchat_get_info, self.handle, win_status.as_ptr())) }
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
        let win_ptr = to_cstring(WIN_PTR);
        c!(hexchat_get_info, self.handle, win_ptr.as_ptr()) as winapi::shared::windef::HWND
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

const AWAY: &str = "away";
const CHANNEL: &str = "channel";
const CHARSET: &str = "charset";
const CONFIG_DIR: &str = "configdir";
const HOST: &str = "host";
const INPUT_BOX: &str = "inputbox";
const LIB_DIR_FS: &str = "libdirfs";
const MODES: &str = "modes";
const NETWORK: &str = "network";
const NICK: &str = "nick";
const NICKSERV: &str = "nickserv";
const SERVER: &str = "server";
const TOPIC: &str = "topic";
const VERSION: &str = "version";
#[cfg(feature = "window")]
const WIN_PTR: &str = "win_ptr";
#[cfg(feature = "window")]
const WIN_STATUS: &str = "win_status";
#[cfg(feature = "window")]
const GTK_WIN_PTR: &str = "gtkwin_ptr";
