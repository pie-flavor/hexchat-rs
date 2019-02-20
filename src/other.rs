/// Represents a 'print event' in HexChat, i.e. an invocation of one of a hundred and fifty
/// different format strings corresponding to any possible action. For example, `PrintEvent::JOIN`
/// corresponds to channel join messages. Note that these are only the physically displayed
/// messages; this should only be used for message formatting. If you want to listen to, respond to,
/// and appropriately eat the actual server-to-client correspondence, you should instead be using
/// raw server event listeners.
///
/// TODO Document this insanity.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct PrintEvent(pub(crate) &'static str);

#[allow(missing_docs)]
impl PrintEvent {
    pub const ADD_NOTIFY: Self = Self("Add Notify");
    pub const BAN_LIST: Self = Self("Ban List");
    pub const BANNED: Self = Self("Banned");
    pub const BEEP: Self = Self("Beep");
    pub const CAPABILITY_ACKNOWLEDGEMENT: Self = Self("Capability Acknowledgement");
    pub const CAPABILITY_DELETED: Self = Self("Capability Deleted");
    pub const CAPABILITY_LIST: Self = Self("Capability List");
    pub const CAPABILITY_REQUEST: Self = Self("Capability Request");
    pub const CHANGE_NICK: Self = Self("Change Nick");
    pub const CHANNEL_ACTION: Self = Self("Channel Action");
    pub const CHANNEL_ACTION_HILIGHT: Self = Self("Channel Action Hilight");
    pub const CHANNEL_BAN: Self = Self("Channel Ban");
    pub const CHANNEL_CREATION: Self = Self("Channel Creation");
    pub const CHANNEL_DEHALFOP: Self = Self("Channel DeHalfOp");
    pub const CHANNEL_DEOP: Self = Self("Channel DeOp");
    pub const CHANNEL_DEVOICE: Self = Self("Channel DeVoice");
    pub const CHANNEL_EXEMPT: Self = Self("Channel Exempt");
    pub const CHANNEL_HALFOP: Self = Self("Channel Half-Operator");
    pub const CHANNEL_INVITE: Self = Self("Channel INVITE");
    pub const CHANNEL_LIST: Self = Self("Channel List");
    pub const CHANNEL_MESSAGE: Self = Self("Channel Message");
    pub const CHANNEL_MODE_GENERIC: Self = Self("Channel Mode Generic");
    pub const CHANNEL_MODES: Self = Self("Channel Modes");
    pub const CHANNEL_MSG_HILIGHT: Self = Self("Channel Msg Hilight");
    pub const CHANNEL_NOTICE: Self = Self("Channel Notice");
    pub const CHANNEL_OPERATOR: Self = Self("Channel Operator");
    pub const CHANNEL_QUIET: Self = Self("Channel Quiet");
    pub const CHANNEL_REMOVE_EXEMPT: Self = Self("Channel Remove Exempt");
    pub const CHANNEL_REMOVE_INVITE: Self = Self("Channel Remove Invite");
    pub const CHANNEL_REMOVE_KEYWORD: Self = Self("Channel Remove Keyword");
    pub const CHANNEL_REMOVE_LIMIT: Self = Self("Channel Remove Limit");
    pub const CHANNEL_SET_KEY: Self = Self("Channel Set Key");
    pub const CHANNEL_SET_LIMIT: Self = Self("Channel Set Limit");
    pub const CHANNEL_UNBAN: Self = Self("Channel UnBan");
    pub const CHANNEL_UNQUIET: Self = Self("Channel UnQuiet");
    pub const CHANNEL_URL: Self = Self("Channel Url");
    pub const CHANNEL_VOICE: Self = Self("Channel Voice");
    pub const CONNECTED: Self = Self("Connected");
    pub const CONNECTING: Self = Self("Connecting");
    pub const CONNECTION_FAILED: Self = Self("Connection Failed");
    pub const CTCP_GENERIC: Self = Self("CTCP Generic");
    pub const CTCP_GENERIC_TO_CHANNEL: Self = Self("CTCP Generic to Channel");
    pub const CTCP_SEND: Self = Self("CTCP Send");
    pub const CTCP_SOUND: Self = Self("CTCP Sound");
    pub const CTCP_SOUND_TO_CHANNEL: Self = Self("CTCP Sound to Channel");
    pub const DCC_CHAT_ABORT: Self = Self("DCC CHAT Abort");
    pub const DCC_CHAT_CONNECT: Self = Self("DCC CHAT Connect");
    pub const DCC_CHAT_FAILED: Self = Self("DCC CHAT Failed");
    pub const DCC_CHAT_OFFER: Self = Self("DCC CHAT Offer");
    pub const DCC_CHAT_OFFERING: Self = Self("DCC CHAT Offering");
    pub const DCC_CHAT_REOFFER: Self = Self("DCC CHAT Reoffer");
    pub const DCC_CONECTION_FAILED: Self = Self("DCC Conection Failed");
    pub const DCC_GENERIC_OFFER: Self = Self("DCC Generic Offer");
    pub const DCC_HEADER: Self = Self("DCC Header");
    pub const DCC_MALFORMED: Self = Self("DCC Malformed");
    pub const DCC_OFFER: Self = Self("DCC Offer");
    pub const DCC_OFFER_NOT_VALID: Self = Self("DCC Offer Not Valid");
    pub const DCC_RECV_ABORT: Self = Self("DCC RECV Abort");
    pub const DCC_RECV_COMPLETE: Self = Self("DCC RECV Complete");
    pub const DCC_RECV_CONNECT: Self = Self("DCC RECV Connect");
    pub const DCC_RECV_FAILED: Self = Self("DCC RECV Failed");
    pub const DCC_RECV_FILE_OPEN_ERROR: Self = Self("DCC RECV File Open Error");
    pub const DCC_RENAME: Self = Self("DCC Rename");
    pub const DCC_RESUME_REQUEST: Self = Self("DCC RESUME Request");
    pub const DCC_SEND_ABORT: Self = Self("DCC SEND Abort");
    pub const DCC_SEND_COMPLETE: Self = Self("DCC SEND Complete");
    pub const DCC_SEND_CONNECT: Self = Self("DCC SEND Connect");
    pub const DCC_SEND_FAILED: Self = Self("DCC SEND Failed");
    pub const DCC_SEND_OFFER: Self = Self("DCC SEND Offer");
    pub const DCC_STALL: Self = Self("DCC Stall");
    pub const DCC_TIMEOUT: Self = Self("DCC Timeout");
    pub const DELETE_NOTIFY: Self = Self("Delete Notify");
    pub const DISCONNECTED: Self = Self("Disconnected");
    pub const FOUND_IP: Self = Self("Found IP");
    pub const GENERIC_MESSAGE: Self = Self("Generic Message");
    pub const IGNORE_ADD: Self = Self("Ignore Add");
    pub const IGNORE_CHANGED: Self = Self("Ignore Changed");
    pub const IGNORE_FOOTER: Self = Self("Ignore Footer");
    pub const IGNORE_HEADER: Self = Self("Ignore Header");
    pub const IGNORE_REMOVE: Self = Self("Ignore Remove");
    pub const IGNORELIST_EMPTY: Self = Self("Ignorelist Empty");
    pub const INVITE: Self = Self("Invite");
    pub const INVITED: Self = Self("Invited");
    pub const JOIN: Self = Self("Join");
    pub const KEYWORD: Self = Self("Keyword");
    pub const KICK: Self = Self("Kick");
    pub const KILLED: Self = Self("Killed");
    pub const MESSAGE_SEND: Self = Self("Message Send");
    pub const MOTD: Self = Self("Motd");
    pub const MOTD_SKIPPED: Self = Self("MOTD Skipped");
    pub const NICK_CLASH: Self = Self("Nick Clash");
    pub const NICK_ERRONEOUS: Self = Self("Nick Erroneous");
    pub const NICK_FAILED: Self = Self("Nick Failed");
    pub const NO_DCC: Self = Self("No DCC");
    pub const NO_RUNNING_PROCESS: Self = Self("No Running Process");
    pub const NOTICE: Self = Self("Notice");
    pub const NOTICE_SEND: Self = Self("Notice Send");
    pub const NOTIFY_AWAY: Self = Self("Notify Away");
    pub const NOTIFY_BACK: Self = Self("Notify Back");
    pub const NOTIFY_EMPTY: Self = Self("Notify Empty");
    pub const NOTIFY_HEADER: Self = Self("Notify Header");
    pub const NOTIFY_NUMBER: Self = Self("Notify Number");
    pub const NOTIFY_OFFLINE: Self = Self("Notify Offline");
    pub const NOTIFY_ONLINE: Self = Self("Notify Online");
    pub const OPEN_DIALOG: Self = Self("Open Dialog");
    pub const PART: Self = Self("Part");
    pub const PART_WITH_REASON: Self = Self("Part with Reason");
    pub const PING_REPLY: Self = Self("Ping Reply");
    pub const PING_TIMEOUT: Self = Self("Ping Timeout");
    pub const PRIVATE_ACTION: Self = Self("Private Action");
    pub const PRIVATE_ACTION_TO_DIALOG: Self = Self("Private Action to Dialog");
    pub const PRIVATE_MESSAGE: Self = Self("Private Message");
    pub const PRIVATE_MESSAGE_TO_DIALOG: Self = Self("Private Message to Dialog");
    pub const PROCESS_ALREADY_RUNNING: Self = Self("Process Already Running");
    pub const QUIT: Self = Self("Quit");
    pub const RAW_MODES: Self = Self("Raw Modes");
    pub const RECEIVE_WALLOPS: Self = Self("Receive Wallops");
    pub const RESOLVING_USER: Self = Self("Resolving User");
    pub const SASL_AUTHENTICATING: Self = Self("SASL Authenticating");
    pub const SASL_RESPONSE: Self = Self("SASL Response");
    pub const SERVER_CONNECTED: Self = Self("Server Connected");
    pub const SERVER_ERROR: Self = Self("Server Error");
    pub const SERVER_LOOKUP: Self = Self("Server Lookup");
    pub const SERVER_NOTICE: Self = Self("Server Notice");
    pub const SERVER_TEXT: Self = Self("Server Text");
    pub const SSL_MESSAGE: Self = Self("SSL Message");
    pub const STOP_CONNECTION: Self = Self("Stop Connection");
    pub const TOPIC: Self = Self("Topic");
    pub const TOPIC_CHANGE: Self = Self("Topic Change");
    pub const TOPIC_CREATION: Self = Self("Topic Creation");
    pub const UNKNOWN_HOST: Self = Self("Unknown Host");
    pub const USER_LIMIT: Self = Self("User Limit");
    pub const USERS_ON_CHANNEL: Self = Self("Users On Channel");
    pub const WHOIS_AUTHENTICATED: Self = Self("WhoIs Authenticated");
    pub const WHOIS_AWAY_LINE: Self = Self("WhoIs Away Line");
    pub const WHOIS_CHANNEL_OR_OPER_LINE: Self = Self("WhoIs Channel/Oper Line");
    pub const WHOIS_END: Self = Self("WhoIs End");
    pub const WHOIS_IDENTIFIED: Self = Self("WhoIs Identified");
    pub const WHOIS_IDLE_LINE: Self = Self("WhoIs Idle Line");
    pub const WHOIS_IDLE_LINE_WITH_SIGNON: Self = Self("WhoIs Idle Line with Signon");
    pub const WHOIS_NAME_LINE: Self = Self("WhoIs Name Line");
    pub const WHOIS_REAL_HOST: Self = Self("WhoIs Real Host");
    pub const WHOIS_SERVER_LINE: Self = Self("WhoIs Server Line");
    pub const WHOIS_SPECIAL: Self = Self("WhoIs Special");
    pub const YOU_JOIN: Self = Self("You Join");
    pub const YOU_KICKED: Self = Self("You Kicked");
    pub const YOU_PART: Self = Self("You Part");
    pub const YOU_PART_WITH_REASON: Self = Self("You Part with Reason");
    pub const YOUR_ACTION: Self = Self("Your Action");
    pub const YOUR_INVITATION: Self = Self("Your Invitation");
    pub const YOUR_MESSAGE: Self = Self("Your Message");
    pub const YOUR_NICK_CHANGING: Self = Self("Your Nick Changing");
}

/// An event corresponding to a window action.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct WindowEvent(pub(crate) &'static str);

impl WindowEvent {
    /// Fired when a channel context is opened.
    pub const OPEN_CHANNEL: Self = Self("Open Context");
    /// Fired when a channel context is closed.
    pub const CLOSE_CHANNEL: Self = Self("Close Context");
    /// Fired when a channel is focused.
    pub const FOCUS_TAB: Self = Self("Focus Tab");
    /// Fired when the window is focused, having previously been unfocused.
    pub const FOCUS_WINDOW: Self = Self("Focus Window");
}
