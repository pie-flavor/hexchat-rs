use crate::{from_cstring, ChannelRef, Context, IrcIdent, IrcIdentRef, UserString};
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
    user: UserString,
    target: PrivmsgTarget,
    message: String,
}

/// An enumeration of the possible targets for a `PRIVMSG`.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PrivmsgTarget {
    /// The `PRIVMSG` was sent to a user.
    User(IrcIdent),
    /// The `PRIVMSG` was sent to a channel.
    Channel {
        /// The name of the channel the message was sent to.
        channel_name: IrcIdent,
        /// The channel the message was sent to.
        channel: ChannelRef,
    },
    /// The `PRIVMSG` was sent to a hostmask.
    HostMask(IrcIdent),
    /// The `PRIVMSG` was sent to a servermask.
    ServerMask(IrcIdent),
}

impl PRIVMSG {
    /// Gets the user that sent this message.
    pub fn get_user(&self) -> &UserString {
        &self.user
    }
    /// Gets the target of the message.
    pub fn get_target(&self) -> &PrivmsgTarget {
        &self.target
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
        let user = UserString::new(user_string).unwrap();
        let arg3 = *word.offset(3);
        let target = match *arg3 as u8 {
            b'#' => {
                let mut target_string = IrcIdent(from_cstring(arg3));
                if target_string.contains('*') {
                    target_string.0.remove(1);
                    PrivmsgTarget::HostMask(target_string)
                } else {
                    let channel = context
                        .get_server_name()
                        .and_then(|s| context.get_channel(&s, &target_string))
                        .unwrap_or_else(|| context.get_first_channel(&target_string).unwrap());
                    PrivmsgTarget::Channel {
                        channel,
                        channel_name: target_string,
                    }
                }
            }
            b'$' => PrivmsgTarget::ServerMask(IrcIdent(from_cstring(arg3.offset(1)))),
            _ => PrivmsgTarget::User(IrcIdent(from_cstring(arg3))),
        };
        let arg4_eol = *word_eol.offset(4);
        let message = from_cstring(arg4_eol.offset(1));
        Self {
            user,
            target,
            message,
        }
    }
}

/// A `ServerEvent` corresponding to `JOIN`.
pub struct JOIN {
    user: UserString,
    channel_string: IrcIdent,
    channel: ChannelRef,
}

impl JOIN {
    /// Gets the user that joined.
    pub fn get_user(&self) -> &UserString {
        &self.user
    }
    /// Gets the name of the channel that was joined.
    pub fn get_channel_name(&self) -> IrcIdentRef {
        self.channel_string.as_ref()
    }
    /// Gets the channel that was joined.
    pub fn get_channel(&self) -> &ChannelRef {
        &self.channel
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
        let user = UserString::new(user_string).unwrap();
        let arg3 = *word.offset(3);
        let channel_string = IrcIdent(from_cstring(arg3.offset(1)));
        let channel = context
            .get_server_name()
            .and_then(|s| context.get_channel(&s, &channel_string))
            .unwrap_or_else(|| context.get_first_channel(&channel_string).unwrap());
        Self {
            user,
            channel_string,
            channel,
        }
    }
}

/// A `ServerEvent` corresponding to `QUIT`.
pub struct QUIT {
    user: UserString,
    message: Option<String>,
}

impl QUIT {
    /// Gets the user who quit.
    pub fn get_user(&self) -> &UserString {
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
        let user = UserString::new(user_string).unwrap();
        let arg3_eol_ptr = word_eol.offset(3);
        let message = if arg3_eol_ptr.is_null() {
            None
        } else {
            let arg3_eol = *arg3_eol_ptr;
            if arg3_eol.is_null() || *arg3_eol == b'\0' as _ || *arg3_eol.offset(1) == b'\0' as _ {
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
    user: UserString,
    channel_names: Vec<IrcIdent>,
    channels: Vec<ChannelRef>,
    message: Option<String>,
}

impl PART {
    /// Gets the user who left.
    pub fn get_user(&self) -> &UserString {
        &self.user
    }
    /// Gets the channel names that were left.
    pub fn get_channel_names(&self) -> &[IrcIdent] {
        &self.channel_names
    }
    /// Gets the channels that were left. Identical layout to `get_channel_names`.
    pub fn get_channels(&self) -> &[ChannelRef] {
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
        let user = UserString::new(user_string).unwrap();
        let arg3 = *word.offset(3);
        let channel_string = from_cstring(arg3);
        let channel_names: Vec<_> = channel_string
            .split(',')
            .map(|t| IrcIdent(t.to_string()))
            .collect();
        let server = context.get_server_name();
        let channels = channel_names
            .iter()
            .map(|c| {
                server
                    .as_ref()
                    .and_then(|s| context.get_channel(s, c))
                    .unwrap_or_else(|| context.get_first_channel(c).unwrap())
            })
            .collect();
        let arg4_eol_ptr = word.offset(4);
        let message = if arg4_eol_ptr.is_null() {
            None
        } else {
            let arg4_eol = *arg4_eol_ptr;
            if arg4_eol.is_null() || *arg4_eol == b'\0' as _ || *arg4_eol.offset(1) == b'\0' as _ {
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

/// A `ServerEvent` corresponding to `TOPIC`.
pub struct TOPIC {
    user: UserString,
    channel_string: IrcIdent,
    channel: ChannelRef,
    message: Option<String>,
}

impl TOPIC {
    /// Gets the user that changed the topic.
    pub fn get_user(&self) -> &UserString {
        &self.user
    }
    /// Gets the name of the channel whose topic was changed.
    pub fn get_channel_name(&self) -> IrcIdentRef {
        self.channel_string.as_ref()
    }
    /// Gets the channel whose topic was changed.
    pub fn get_channel(&self) -> &ChannelRef {
        &self.channel
    }
    /// Gets the new topic message, or `None` if it was reset.
    pub fn get_message(&self) -> Option<&str> {
        self.message.as_ref().map(|s| &**s)
    }
}

impl ServerEvent for TOPIC {
    const NAME: &'static str = "TOPIC";
    unsafe fn create(
        context: &Context,
        word: *mut *mut c_char,
        word_eol: *mut *mut c_char,
    ) -> Self {
        let arg1 = *word.offset(1);
        let user_string = from_cstring(arg1.offset(1));
        let user = UserString::new(user_string).unwrap();
        let arg3 = *word.offset(3);
        let channel_string = IrcIdent(from_cstring(arg3));
        let channel = context
            .get_server_name()
            .and_then(|s| context.get_channel(&s, &channel_string))
            .unwrap_or_else(|| context.get_first_channel(&channel_string).unwrap());
        let arg4_eol_ptr = word_eol.offset(4);
        let message = if arg4_eol_ptr.is_null() {
            None
        } else {
            let arg4_eol = *arg4_eol_ptr;
            if arg4_eol.is_null() || *arg4_eol == b'\0' as _ || *arg4_eol.offset(1) == b'\0' as _ {
                None
            } else {
                Some(from_cstring(arg4_eol.offset(1)))
            }
        };
        Self {
            user,
            channel_string,
            channel,
            message,
        }
    }
}

/// A `ServerEvent` corresponding to `INVITE`.
pub struct INVITE {
    sender: UserString,
    recipient: IrcIdent,
    channel_string: IrcIdent,
    channel: ChannelRef,
}

impl INVITE {
    /// Gets who sent the invite.
    pub fn get_sender(&self) -> &UserString {
        &self.sender
    }
    /// Gets the nickname of the invite recipient.
    pub fn get_recipient(&self) -> IrcIdentRef {
        self.recipient.as_ref()
    }
    /// Gets the name of the channel the recipient was invited to.
    pub fn get_channel_name(&self) -> IrcIdentRef {
        self.channel_string.as_ref()
    }
    /// Gets the channel the recipient was invited to.
    pub fn get_channel(&self) -> &ChannelRef {
        &self.channel
    }
}

impl ServerEvent for INVITE {
    const NAME: &'static str = "INVITE";
    unsafe fn create(
        context: &Context,
        word: *mut *mut c_char,
        _word_eol: *mut *mut c_char,
    ) -> Self {
        let arg1 = *word.offset(1);
        let sender_string = from_cstring(arg1.offset(1));
        let sender = UserString::new(sender_string).unwrap();
        let arg3 = *word.offset(3);
        let recipient = IrcIdent(from_cstring(arg3));
        let arg4 = *word.offset(4);
        let channel_string = IrcIdent(from_cstring(arg4));
        let channel = context
            .get_server_name()
            .and_then(|s| context.get_channel(&s, &channel_string))
            .unwrap_or_else(|| context.get_first_channel(&channel_string).unwrap());
        Self {
            sender,
            recipient,
            channel_string,
            channel,
        }
    }
}

/// A `ServerEvent` corresponding to `KICK`.
pub struct KICK {
    sender: UserString,
    channel: ChannelRef,
    channel_string: IrcIdent,
    kicked: IrcIdent,
    comment: Option<String>,
}

impl KICK {
    /// Gets the user who sent the kick.
    pub fn get_sender(&self) -> &UserString {
        &self.sender
    }
    /// Gets the channel the user was kicked from.
    pub fn get_channel(&self) -> &ChannelRef {
        &self.channel
    }
    /// Gets the name of the channel the user was kicked from.
    pub fn get_channel_name(&self) -> IrcIdentRef {
        self.channel_string.as_ref()
    }
    /// Gets the nick of the kicked user.
    pub fn get_kicked(&self) -> IrcIdentRef {
        self.kicked.as_ref()
    }
    /// Gets the kick comment, or `None` if there wasn't one.
    pub fn get_comment(&self) -> Option<&str> {
        self.comment.as_ref().map(|s| &**s)
    }
}

impl ServerEvent for KICK {
    const NAME: &'static str = "KICK";
    unsafe fn create(
        context: &Context,
        word: *mut *mut c_char,
        word_eol: *mut *mut c_char,
    ) -> Self {
        let arg1 = *word.offset(1);
        let sender_string = from_cstring(arg1.offset(1));
        let sender = UserString::new(sender_string).unwrap();
        let arg3 = *word.offset(3);
        let channel_string = IrcIdent(from_cstring(arg3));
        let channel = context
            .get_server_name()
            .and_then(|s| context.get_channel(&s, &channel_string))
            .unwrap_or_else(|| context.get_first_channel(&channel_string).unwrap());
        let arg4 = *word.offset(4);
        let kicked = IrcIdent(from_cstring(arg4));
        let arg5_eol_ptr = word_eol.offset(5);
        let comment = if arg5_eol_ptr.is_null() {
            None
        } else {
            let arg5_eol = *arg5_eol_ptr;
            if arg5_eol.is_null() || *arg5_eol == b'\0' as _ || *arg5_eol.offset(1) == b'\0' as _ {
                None
            } else {
                Some(from_cstring(arg5_eol.offset(1)))
            }
        };
        Self {
            sender,
            channel,
            channel_string,
            comment,
            kicked,
        }
    }
}

/// A `ServerEvent` corresponding to `NOTICE`.
pub struct NOTICE {
    privmsg: PRIVMSG,
}

impl NOTICE {
    /// Gets the user that sent the notice.
    pub fn get_user(&self) -> &UserString {
        self.privmsg.get_user()
    }
    /// Gets the target of the notice.
    pub fn get_target(&self) -> &PrivmsgTarget {
        self.privmsg.get_target()
    }
    /// Gets the notice that was sent.
    pub fn get_message(&self) -> &str {
        self.privmsg.get_message()
    }
}

impl ServerEvent for NOTICE {
    const NAME: &'static str = "NOTICE";
    unsafe fn create(
        context: &Context,
        word: *mut *mut c_char,
        word_eol: *mut *mut c_char,
    ) -> Self {
        Self {
            privmsg: PRIVMSG::create(context, word, word_eol),
        }
    }
}

/// A `ServerEvent` corresponding to `WALLOPS`.
pub struct WALLOPS {
    server_name: IrcIdent,
    message: String,
}

impl WALLOPS {
    /// Gets the name of the server that sent this operwall.
    pub fn get_server_name(&self) -> IrcIdentRef {
        self.server_name.as_ref()
    }
    /// Gets the message that was sent.
    pub fn get_message(&self) -> &str {
        &self.message
    }
}

impl ServerEvent for WALLOPS {
    const NAME: &'static str = "WALLOPS";
    unsafe fn create(
        _context: &Context,
        word: *mut *mut c_char,
        word_eol: *mut *mut c_char,
    ) -> Self {
        let arg1 = *word.offset(1);
        let server_name = IrcIdent(from_cstring(arg1.offset(1)));
        let arg3 = *word_eol.offset(3);
        let message = from_cstring(arg3.offset(1));
        Self {
            server_name,
            message,
        }
    }
}
