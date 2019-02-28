use crate::{c, from_cstring_opt, to_cstring};
use bitflags::bitflags;
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use std::marker::PhantomData;
use std::mem;
use std::net::Ipv4Addr;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::ptr;

struct XList<T>
where
    T: FromXList,
{
    handle: *mut c::hexchat_list,
    _ref: PhantomData<c::hexchat_list>,
    _item: PhantomData<T>,
}

impl<T> Drop for XList<T>
where
    T: FromXList,
{
    fn drop(&mut self) {
        unsafe {
            c!(hexchat_list_free, self.handle);
        }
    }
}

trait FromXList
where
    Self: Sized,
{
    const LIST_NAME: &'static str;
    fn map_list(list: &XList<Self>) -> Self;
}

impl<T> XList<T>
where
    T: FromXList,
{
    fn new() -> Self {
        let name = to_cstring(T::LIST_NAME);
        Self {
            handle: unsafe { c!(hexchat_list_get, name.as_ptr()) },
            _ref: PhantomData,
            _item: PhantomData,
        }
    }
    fn move_next(&mut self) -> bool {
        unsafe { c!(hexchat_list_next, self.handle) != 0 }
    }
    fn get_item_string(&self, field: &str) -> Option<String> {
        unsafe {
            let cstr = to_cstring(field);
            let ptr = c!(hexchat_list_str, self.handle, cstr.as_ptr());
            from_cstring_opt(ptr)
        }
    }
    fn get_item_int(&self, field: &str) -> i32 {
        unsafe {
            let cstr = to_cstring(field);
            c!(hexchat_list_int, self.handle, cstr.as_ptr()) as _
        }
    }
    fn get_item_time(&self, field: &str) -> DateTime<Utc> {
        unsafe {
            let cstr = to_cstring(field);
            let time = c!(hexchat_list_time, self.handle, cstr.as_ptr());
            let naive = NaiveDateTime::from_timestamp(time as _, 0);
            Utc.from_utc_datetime(&naive)
        }
    }
    fn get_item_context(&self, field: &str) -> *mut c::hexchat_context {
        unsafe {
            let cstr = to_cstring(field);
            let ptr = c!(hexchat_list_str, self.handle, cstr.as_ptr());
            ptr as *mut _
        }
    }
    fn get_item_char(&self, field: &str) -> char {
        unsafe {
            let cstr = to_cstring(field);
            let ptr = c!(hexchat_list_str, self.handle, cstr.as_ptr());
            if ptr.is_null() {
                '\0'
            } else {
                mem::transmute::<_, u8>(*ptr) as _
            }
        }
    }
    fn get_current(&self) -> T {
        T::map_list(self)
    }
}

impl<T> Iterator for XList<T>
where
    T: FromXList,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        let res = unsafe { c!(hexchat_list_next, self.handle) };
        if res == 0 {
            Some(self.get_current())
        } else {
            None
        }
    }
}

/// A full set of information about an IRC channel.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ChannelInfo {
    channel_name: String,
    channel_key: Option<String>,
    channel_modes: String,
    channel_types: String,
    cref: ChannelRef,
    flags: ChannelFlags,
    id: i32,
    lag: u32,
    max_modes: u32,
    network_name: String,
    nick_prefixes: String,
    nick_modes: String,
    send_queue_size: u32,
    server_name: String,
    channel_type: ChannelType,
    user_count: u32,
}

impl ChannelInfo {
    /// Gets the channel name.
    pub fn get_name(&self) -> &str {
        &self.channel_name
    }
    /// Gets the channel key, or `None` if unknown.
    pub fn get_key(&self) -> Option<&str> {
        self.channel_key.as_ref().map(|x| &**x)
    }
    /// Gets the chanmode string for the channel.
    pub fn get_mode_string(&self) -> &str {
        &self.channel_modes
    }
    /// Gets the chantype string for the channel.
    pub fn get_type_string(&self) -> &str {
        &self.channel_types
    }
    /// Gets various flags about the channel.
    pub fn get_flags(&self) -> ChannelFlags {
        self.flags
    }
    /// Gets the server ID.
    pub fn get_id(&self) -> i32 {
        self.id
    }
    /// Gets the current latency in milliseconds.
    pub fn get_lag_ms(&self) -> u32 {
        self.lag
    }
    /// Gets the maximum number of channel modes that can be sent on a single line.
    pub fn get_max_modes_per_line(&self) -> u32 {
        self.max_modes
    }
    /// Gets the name of the network.
    pub fn get_network_name(&self) -> &str {
        &self.network_name
    }
    /// Gets the client's prefix string for their nickname, usually corresponding to operator or
    /// voice.
    pub fn get_nick_prefix_string(&self) -> &str {
        &self.nick_prefixes
    }
    /// Gets the client's user mode string.
    pub fn get_nick_mode_string(&self) -> &str {
        &self.nick_modes
    }
    /// Gets the number of bytes in the send queue.
    pub fn get_send_queue_size(&self) -> u32 {
        self.send_queue_size
    }
    /// Gets the name of the server.
    pub fn get_server_name(&self) -> &str {
        &self.server_name
    }
    /// Gets the type of channel this is.
    pub fn get_type(&self) -> ChannelType {
        self.channel_type
    }
    /// Gets the number of users currently in the channel.
    pub fn get_user_count(&self) -> u32 {
        self.user_count
    }
}

impl FromXList for ChannelInfo {
    const LIST_NAME: &'static str = "channels";
    fn map_list(list: &XList<Self>) -> Self {
        Self {
            channel_name: list.get_item_string("channel").unwrap_or_default(),
            channel_key: list.get_item_string("channelkey"),
            channel_modes: list.get_item_string("chanmodes").unwrap_or_default(),
            channel_types: list.get_item_string("chantypes").unwrap_or_default(),
            cref: ChannelRef {
                handle: list.get_item_context("context"),
            },
            flags: ChannelFlags::from_bits_truncate(list.get_item_int("flags") as _),
            id: list.get_item_int("id"),
            lag: list.get_item_int("lag") as _,
            max_modes: list.get_item_int("maxmodes") as _,
            network_name: list.get_item_string("network").unwrap_or_default(),
            nick_prefixes: list.get_item_string("nickprefixes").unwrap_or_default(),
            nick_modes: list.get_item_string("nickmodes").unwrap_or_default(),
            send_queue_size: list.get_item_int("queue") as _,
            server_name: list.get_item_string("server").unwrap_or_default(),
            channel_type: ChannelType::VALUES[list.get_item_int("type") as usize + 1],
            user_count: list.get_item_int("users") as _,
        }
    }
}

bitflags! {
    /// Various boolean flags about a `ChannelInfo`.
    pub struct ChannelFlags: u32 {
        /// Whether or not the client is currently connected.
        const CONNECTED = 1;
        /// Whether or not the client is currrently connecting.
        const CONNECTING = 1 << 1;
        /// Whether or not the client is currently away.
        const AWAY = 1 << 2;
        /// Whether or not the MOTD has ended yet.
        const MOTD_END = 1 << 3;
        /// Whether the server supports the WHOX protocol.
        const WHOX = 1 << 4;
        /// Whether the server supports the IDMSG protocol.
        const IDMSG = 1 << 5;
        /// Whether the channel is hiding join/part messages.
        const HIDE_JOINS_AND_PARTS = 1 << 6;
        /// Whether the channel is using the global default on whether to hide join/part messages.
        const HIDE_JOINS_AND_PARTS_UNSET = 1 << 7;
        /// Whether the channel is set to beep when a message is received.
        const BEEP_ON_MESSAGE = 1 << 8;
        /// Whether the channel is currently blinking the tray.
        const BLINK_TRAY = 1 << 9;
        /// Whether the channel is currently blinking the taskbar.
        const BLINK_TASKBAR = 1 << 10;
        /// Whether the channel is logging messages.
        const LOGGING = 1 << 11;
        /// Whether the channel is using the global default on whether to log messages.
        const LOGGING_UNSET = 1 << 12;
        /// Whether the channel allows for scrollback.
        const SCROLLBACK = 1 << 13;
        /// Whether the channel is using the global default on whether to allow for scrollback.
        const SCROLLBACK_UNSET = 1 << 14;
        /// Whether the channel is stripping all colors.
        const STRIP_COLORS = 1  << 15;
        /// Whether the channel is using the global default on whether to strip colors.
        const STRIP_COLORS_UNSET = 1 << 16;
    }
}

/// An enumeration of different channel types.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ChannelType {
    /// A server main channel.
    Server,
    /// A channel within a server.
    Channel,
    /// A dialog.
    Dialog,
    /// A channel notice channel.
    Notice,
    /// A server notice channel.
    SNotice,
}

impl ChannelType {
    const VALUES: [Self; 5] = [
        Self::Server,
        Self::Channel,
        Self::Dialog,
        Self::Notice,
        Self::SNotice,
    ];
}

/// A full set of information about a DCC transfer.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct DccTransferInfo {
    address: Ipv4Addr,
    bytes_per_second: u32,
    destination: PathBuf,
    filename: String,
    sender_nick: String,
    port: u16,
    bytes_processed: u64,
    resume_point: Option<u64>,
    file_size: u64,
    status: DccTransferStatus,
    transfer_type: DccTransferType,
}

impl DccTransferInfo {
    /// Gets the source address of the other party.
    pub fn get_address(&self) -> Ipv4Addr {
        self.address
    }
    /// Gets the speed of sending in bytes per second.
    pub fn get_bytes_per_second(&self) -> u32 {
        self.bytes_per_second
    }
    /// Gets the destination path of the transfer.
    pub fn get_destination(&self) -> &Path {
        &self.destination
    }
    /// Gets the name of the file being transferred.
    pub fn get_filename(&self) -> &str {
        &self.filename
    }
    /// Gets the nick of the other party.
    pub fn get_sender_nick(&self) -> &str {
        &self.sender_nick
    }
    /// Gets the port the transfer is being sent over.
    pub fn get_port(&self) -> u16 {
        self.port
    }
    /// Gets the number of bytes processed so far.
    pub fn get_bytes_processed(&self) -> u64 {
        self.bytes_processed
    }
    /// Gets the byte count at which this transfer was last resumed, or `None` if it hasn't been
    /// paused.
    pub fn get_resume_point(&self) -> Option<u64> {
        self.resume_point
    }
    /// Gets the size of the file in bytes.
    pub fn get_file_size(&self) -> u64 {
        self.file_size
    }
    /// Gets the status of the transfer.
    pub fn get_status(&self) -> DccTransferStatus {
        self.status
    }
    /// Gets the type of the transfer.
    pub fn get_transfer_type(&self) -> DccTransferType {
        self.transfer_type
    }
}

impl FromXList for DccTransferInfo {
    const LIST_NAME: &'static str = "dcc";
    fn map_list(list: &XList<Self>) -> Self {
        let bytes_processed_low = list.get_item_int("pos");
        let bytes_processed_high = list.get_item_int("poshigh");
        let bytes_processed = merge_unsigned(bytes_processed_low, bytes_processed_high);
        let resume_point_low = list.get_item_int("resume");
        let resume_point = if resume_point_low == 0 {
            None
        } else {
            let resume_point_high = list.get_item_int("resumehigh");
            Some(merge_unsigned(resume_point_low, resume_point_high))
        };
        let file_size_low = list.get_item_int("size");
        let file_size_high = list.get_item_int("sizehigh");
        let file_size = merge_unsigned(file_size_low, file_size_high);
        Self {
            address: Ipv4Addr::from(unsafe {
                mem::transmute::<_, u32>(list.get_item_int("address32"))
            }),
            bytes_per_second: list.get_item_int("cps") as _,
            destination: list
                .get_item_string("destfile")
                .map(PathBuf::from)
                .unwrap_or_default(),
            filename: list.get_item_string("file").unwrap_or_default(),
            sender_nick: list.get_item_string("nick").unwrap_or_default(),
            port: list.get_item_int("port") as _,
            bytes_processed,
            resume_point,
            file_size,
            status: DccTransferStatus::VALUES[list.get_item_int("status") as usize],
            transfer_type: DccTransferType::VALUES[list.get_item_int("type") as usize],
        }
    }
}

/// The various statuses that can describe a `DccTransferInfo`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DccTransferStatus {
    /// A queued transfer.
    Queued,
    /// An active transfer.
    Active,
    /// A failed transfer.
    Failed,
    /// A finished transfer.
    Done,
    /// A connection-in-progress transfer.
    Connecting,
    /// An aborted transfer.
    Aborted,
}

impl DccTransferStatus {
    const VALUES: [Self; 6] = [
        Self::Queued,
        Self::Active,
        Self::Failed,
        Self::Done,
        Self::Connecting,
        Self::Aborted,
    ];
}

/// An enumeration of all possible DCC transfer types.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DccTransferType {
    /// This DCC transfer is being sent.
    Send,
    /// This DCC transfer is being received.
    Receive,
    /// This DCC transfer is being received over chat.
    ChatReceive,
    /// This DCC transfer is being sent over chat.
    ChatSend,
}

impl DccTransferType {
    const VALUES: [Self; 4] = [Self::Send, Self::Receive, Self::ChatReceive, Self::ChatSend];
}

/// An entry in HexChat's ignore list.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct IgnoreEntry {
    mask: String,
    ignore_type: IgnoreType,
}

impl IgnoreEntry {
    /// Gets the hostmask that's being ignored.
    pub fn get_mask(&self) -> &str {
        &self.mask
    }
    /// Gets the type of message that's being ignored.
    pub fn get_ignore_type(&self) -> IgnoreType {
        self.ignore_type
    }
}

impl FromXList for IgnoreEntry {
    const LIST_NAME: &'static str = "ignore";
    fn map_list(list: &XList<Self>) -> Self {
        Self {
            mask: list.get_item_string("mask").unwrap_or_default(),
            ignore_type: IgnoreType::from_bits_truncate(list.get_item_int("flags")),
        }
    }
}

bitflags! {
    /// All the message types that can be present in an `IgnoreEntry`.
    pub struct IgnoreType: i32 {
        /// Ignores private messages.
        const PRIVATE = 1;
        /// Ignores channel notices.
        const NOTICE = 1 << 1;
        /// Ignores channel messages.
        const CHANNEL = 1 << 2;
        /// Ignores CTCP messages.
        const CTCP = 1 << 3;
        /// Ignores invite messages.
        const INVITE = 1 << 4;
        /// Sets this to 'explicitly unignore' instead of 'explicitly ignore'.
        const UNIGNORE = 1 << 5;
        /// This ignore entry will not be saved to file.
        const NO_SAVE = 1 << 6;
        /// Ignores DCC messages.
        const DCC = 1 << 7;
    }
}

/// An entry in HexChat's notify list.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct NotifyEntry {
    networks: Vec<String>,
    nick: String,
    is_online: bool,
    time_online: DateTime<Utc>,
    time_offline: DateTime<Utc>,
    last_seen: DateTime<Utc>,
}

impl NotifyEntry {
    /// Gets the networks in which this entry is active.
    pub fn get_networks(&self) -> &[String] {
        &self.networks
    }
    /// Gets the nick on notify.
    pub fn get_nick(&self) -> &str {
        &self.nick
    }
    /// Gets whether the nick is currently online.
    pub fn is_online(&self) -> bool {
        self.is_online
    }
    /// Gets the last point the user came online.
    pub fn get_time_online(&self) -> DateTime<Utc> {
        self.time_online
    }
    /// Gets the last point the user went offline.
    pub fn get_time_offline(&self) -> DateTime<Utc> {
        self.time_offline
    }
    /// Gets the last point the user was verified still online.
    pub fn get_time_last_seen(&self) -> DateTime<Utc> {
        self.last_seen
    }
}

impl FromXList for NotifyEntry {
    const LIST_NAME: &'static str = "notify";
    fn map_list(list: &XList<Self>) -> Self {
        Self {
            networks: list
                .get_item_string("networks")
                .map(|t| t.split(',').map(String::from).collect())
                .unwrap_or_default(),
            nick: list.get_item_string("nick").unwrap_or_default(),
            is_online: list.get_item_int("flags") == 0,
            time_online: list.get_item_time("on"),
            time_offline: list.get_item_time("off"),
            last_seen: list.get_item_time("seen"),
        }
    }
}

/// Information about another user.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct UserInfo {
    account_name: Option<String>,
    away: bool,
    last_posted: DateTime<Utc>,
    nick: String,
    host: String,
    prefix: char,
    real_name: Option<String>,
    is_selected: bool,
}

impl UserInfo {
    /// Gets the account name of the user, or `None` if not set.
    pub fn get_account_name(&self) -> Option<&str> {
        self.account_name.as_ref().map(|x| &**x)
    }
    /// Gets whether or not the user is marked away.
    pub fn is_away(&self) -> bool {
        self.away
    }
    /// Gets the last time this user sent a message.
    pub fn get_time_last_posted(&self) -> DateTime<Utc> {
        self.last_posted
    }
    /// Gets this user's nick.
    pub fn get_nick(&self) -> &str {
        &self.nick
    }
    /// Gets this user's hostmask string.
    pub fn get_host_string(&self) -> &str {
        &self.host
    }
    /// Gets the nickname prefixes applied to this user.
    pub fn get_prefix(&self) -> char {
        self.prefix
    }
    /// Gets this user's real name field, or `None` if not set.
    pub fn get_real_name(&self) -> Option<&str> {
        self.real_name.as_ref().map(|x| &**x)
    }
    /// Gets whether this user is selected in the user list, if they're in the currently focused
    /// tab.
    pub fn is_selected(&self) -> bool {
        self.is_selected
    }
}

impl FromXList for UserInfo {
    const LIST_NAME: &'static str = "users";
    fn map_list(list: &XList<Self>) -> Self {
        Self {
            account_name: list.get_item_string("account"),
            away: list.get_item_int("away") != 0,
            last_posted: list.get_item_time("lasttalk"),
            nick: list.get_item_string("nick").unwrap_or_default(),
            host: list.get_item_string("host").unwrap_or_default(),
            prefix: list.get_item_char("prefix"),
            real_name: list.get_item_string("realname"),
            is_selected: list.get_item_int("selected") != 0,
        }
    }
}

/// Gets all channels currently open.
pub fn get_all_channels() -> impl Iterator<Item = ChannelInfo> {
    XList::new()
}
/// Gets all DCC transfers currently active.
pub fn get_current_dcc_transfers() -> impl Iterator<Item = DccTransferInfo> {
    XList::new()
}
/// Gets all entries in the ignore list.
pub fn get_ignore_entries() -> impl Iterator<Item = IgnoreEntry> {
    XList::new()
}
/// Gets all entries in the notify list.
pub fn get_notify_users() -> impl Iterator<Item = NotifyEntry> {
    XList::new()
}
/// Gets all the users in the current channel.
pub fn get_users_in_current_channel() -> impl Iterator<Item = UserInfo> {
    XList::new()
}
/// Gets all the users in a specific channel, or `None` if the channel is invalid.
pub fn get_users_in_channel(channel: &ChannelRef) -> Option<impl Iterator<Item = UserInfo>> {
    unsafe {
        let ctx = c!(hexchat_get_context);
        if c!(hexchat_set_context, channel.handle) == 0 {
            None
        } else {
            let list = get_users_in_current_channel();
            if c!(hexchat_set_context, ctx) == 0 {
                c!(
                    hexchat_set_context,
                    c!(hexchat_find_context, ptr::null(), ptr::null()),
                );
            }
            Some(list)
        }
    }
}

fn merge_unsigned(low: i32, high: i32) -> u64 {
    let [b0, b1, b2, b3] = high.to_be_bytes();
    let [b4, b5, b6, b7] = low.to_be_bytes();
    u64::from_be_bytes([b0, b1, b2, b3, b4, b5, b6, b7])
}

/// A channel reference, for identification purposes only. Use `into_info` to request channel
/// information.
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct ChannelRef {
    pub(crate) handle: *mut c::hexchat_context,
}

impl ChannelRef {
    /// Turns this `ChannelRef` into a `ChannelInfo`, or `None` if the channel represented by this
    /// `ChannelRef` is no longer valid.
    pub fn get_info(&self) -> Option<ChannelInfo> {
        let mut list = XList::new();
        while list.move_next() {
            if list.get_item_context("context") == self.handle {
                return Some(list.get_current());
            }
        }
        None
    }
}

impl Deref for ChannelInfo {
    type Target = ChannelRef;
    fn deref(&self) -> &Self::Target {
        &self.cref
    }
}
