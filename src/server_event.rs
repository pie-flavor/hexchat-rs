use crate::{from_cstring, ChannelRef, Context, IrcIdent, IrcIdentRef, UserMask};
use std::os::raw::c_char;

/// A type representing a raw server event. Used with `Context::add_server_event_listener`. It is
/// not recommended you implement this on your own types.
pub trait ServerEvent {
    /// The name of the event, e.g. `PRIVMSG`.
    const NAME: &'static str;
    #[doc(hidden)]
    unsafe fn create(context: &Context, word: *mut *mut c_char, word_eol: *mut *mut c_char)
        -> Self;
}

/// A `ServerEvent` corresponding to `PRIVMSG`.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PRIVMSG {
    user: UserMask,
    channel_string: IrcIdent,
    channel: Option<ChannelRef>,
    message: String,
}

impl PRIVMSG {
    /// Gets the usermask that sent this message.
    pub fn get_user(&self) -> &UserMask {
        &self.user
    }
    /// Gets the name of the channel this was sent to.
    pub fn get_channel_name(&self) -> IrcIdentRef {
        self.channel_string.as_ref()
    }
    /// Gets the channel if available, or `None` if unavailable.
    pub fn get_channel(&self) -> Option<&ChannelRef> {
        self.channel.as_ref()
    }
    /// Gets the message that was sent.
    pub fn get_message(&self) -> &str {
        &self.message
    }
}

impl ServerEvent for PRIVMSG {
    const NAME: &'static str = "PRIVMSG";
    unsafe fn create(
        context: &Context,
        word: *mut *mut c_char,
        word_eol: *mut *mut c_char,
    ) -> Self {
        let arg1 = *word.offset(1);
        let user_string = from_cstring(arg1.offset(1));
        let user = UserMask::new(user_string).unwrap();
        let arg3 = *word.offset(3);
        let channel_string = IrcIdent(from_cstring(arg3));
        let arg4_eol = *word_eol.offset(4);
        let message = from_cstring(arg4_eol.offset(1));
        let channel = context
            .get_server_name()
            .and_then(|s| context.get_channel(&s, &channel_string));
        Self {
            user,
            channel_string,
            channel,
            message,
        }
    }
}

/// A `ServerEvent` corresponding to `JOIN`.
pub struct JOIN {
    user: UserMask,
    channel_string: IrcIdent,
    channel: Option<ChannelRef>,
}

impl JOIN {
    /// Gets the user that joined.
    pub fn get_user(&self) -> &UserMask {
        &self.user
    }
    /// Gets the name of the channel that was joined.
    pub fn get_channel_name(&self) -> IrcIdentRef {
        self.channel_string.as_ref()
    }
    /// Gets the channel that was joined, if available, or `None` if unavailable.
    pub fn get_channel(&self) -> Option<&ChannelRef> {
        self.channel.as_ref()
    }
}

impl ServerEvent for JOIN {
    const NAME: &'static str = "JOIN";
    unsafe fn create(
        context: &Context,
        word: *mut *mut c_char,
        _word_eol: *mut *mut c_char,
    ) -> Self {
        let arg1 = *word.offset(1);
        let user_string = from_cstring(arg1.offset(1));
        let user = UserMask::new(user_string).unwrap();
        let arg3 = *word.offset(3);
        let channel_string = IrcIdent(from_cstring(arg3.offset(1)));
        let channel = context
            .get_server_name()
            .and_then(|s| context.get_channel(&s, &channel_string));
        Self {
            user,
            channel_string,
            channel,
        }
    }
}

/// A `ServerEvent` corresponding to `QUIT`.
pub struct QUIT {
    user: UserMask,
    message: Option<String>,
}

impl QUIT {
    /// Gets the user who quit.
    pub fn get_user(&self) -> &UserMask {
        &self.user
    }
    /// Gets the quit message, or `None` if there wasn't one.
    pub fn get_message(&self) -> Option<&str> {
        self.message.as_ref().map(|s| &**s)
    }
}

impl ServerEvent for QUIT {
    const NAME: &'static str = "QUIT";
    unsafe fn create(
        _context: &Context,
        word: *mut *mut c_char,
        word_eol: *mut *mut c_char,
    ) -> Self {
        let arg1 = *word.offset(1);
        let user_string = from_cstring(arg1.offset(1));
        let user = UserMask::new(user_string).unwrap();
        let arg3_eol_ptr = word_eol.offset(3);
        let message = if arg3_eol_ptr.is_null() {
            None
        } else {
            let arg3_eol = *arg3_eol_ptr;
            if arg3_eol.is_null() || *arg3_eol == b'\0' as _ {
                None
            } else {
                Some(from_cstring(arg3_eol.offset(1)))
            }
        };
        Self { user, message }
    }
}

/// A `ServerEvent` corresponding to `PART`.
pub struct PART {
    user: UserMask,
    channel_names: Vec<IrcIdent>,
    channels: Vec<Option<ChannelRef>>,
    message: Option<String>,
}

impl PART {
    /// Gets the user who left.
    pub fn get_user(&self) -> &UserMask {
        &self.user
    }
    /// Gets the channel names that were left.
    pub fn get_channel_names(&self) -> &[IrcIdent] {
        &self.channel_names
    }
    /// Gets the channels that were left, or `None` if unavailable. Identical layout to
    /// `get_channel_names`.
    pub fn get_channels(&self) -> &[Option<ChannelRef>] {
        &self.channels
    }
    /// Gets the message that was sent, or `None` if there wasn't one.
    pub fn get_message(&self) -> Option<&str> {
        self.message.as_ref().map(|s| &**s)
    }
}

impl ServerEvent for PART {
    const NAME: &'static str = "PART";
    //noinspection RsTypeCheck
    unsafe fn create(
        context: &Context,
        word: *mut *mut c_char,
        _word_eol: *mut *mut c_char,
    ) -> Self {
        let arg1 = *word.offset(1);
        let user_string = from_cstring(arg1.offset(1));
        let user = UserMask::new(user_string).unwrap();
        let arg3 = *word.offset(3);
        let channel_string = from_cstring(arg3);
        let channel_names: Vec<_> = channel_string
            .split(',')
            .map(|t| IrcIdent(t.to_string()))
            .collect();
        let server = context.get_server_name();
        let channels = channel_names
            .iter()
            .map(|c| server.as_ref().and_then(|s| context.get_channel(s, c)))
            .collect();
        let arg4_eol_ptr = word.offset(4);
        let message = if arg4_eol_ptr.is_null() {
            None
        } else {
            let arg4_eol = *arg4_eol_ptr;
            if arg4_eol.is_null() || *arg4_eol == b'\0' as _ {
                None
            } else {
                Some(from_cstring(arg4_eol.offset(1)))
            }
        };
        Self {
            user,
            channel_names,
            channels,
            message,
        }
    }
}
