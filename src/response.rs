#![allow(non_camel_case_types)]

use crate::{from_cstring, Context, IrcIdent, IrcIdentRef, UserMask};
use chrono::{DateTime, TimeZone, Utc};
use std::os::raw::c_char;
use std::time::Duration;

/// A type representing a server response. Used with `Context::add_server_response_listener`. It is
/// not recommended you implement this on your own types.
pub trait ServerResponse {
    /// The numeric ID of this response.
    const ID: &'static str;
    #[doc(hidden)]
    unsafe fn create(context: &Context, word: *mut *mut c_char, word_eol: *mut *mut c_char)
        -> Self;
}

///// A `ServerResponse` corresponding to `RPL_WELCOME` (`001`).
//pub struct RPL_WELCOME {
//    nick: IrcIdent,
//    user: IrcIdent,
//    host: String,
//}
//
//impl RPL_WELCOME {
//    pub fn get_nick(&self) -> IrcIdentRef {
//        self.nick.as_ref()
//    }
//    pub fn get_user(&self) -> IrcIdentRef {
//        self.user.as_ref()
//    }
//    pub fn get_host(&self) -> &str {
//        &self.host
//    }
//}

macro_rules! rpl {
    ($t:ident[$e:expr] { global($word:ident $word_eol:ident) { $($s:stmt;)* } ($this:ident) $([$desc:expr] $name:ident : $ftype:ty [$rtype:ty] get $getter:block parse $parser:block)* }) => {
        rpl!(@RPL (stringify!($t), stringify!($e)) $t[$e] { global($word $word_eol) { $($s;)* } ($this) $([$desc] $name : $ftype [$rtype] get $getter parse $parser)* });
    };
    (@RPL ($te:expr, $ee:expr) $t:ident[$e:expr] { global($word:ident $word_eol:ident) { $($s:stmt;)* } ($this:ident) $([$desc:expr] $name:ident : $ftype:ty [$rtype:ty] get $getter:block parse $parser:block)* }) => {
        #[doc = "A `ServerResponse` corresponding to `"]
        #[doc = $te]
        #[doc = "` (`"]
        #[doc = $ee]
        #[doc = "`)"]
        pub struct $t {
            server: IrcIdent,
            target: IrcIdent,
            $(
            $name : $ftype,
            )*
        }

        impl $t {
            #[doc = "The server that sent the response."]
            pub fn server(&self) -> IrcIdentRef {
                self.server.as_ref()
            }
            #[doc = "The target of the response."]
            pub fn target(&self) -> IrcIdentRef {
                self.target.as_ref()
            }
            $(
            #[doc = $desc]
            pub fn $name(&self) -> $rtype {
                let $this = self;
                $getter
            }
            )*
        }

        impl ServerResponse for $t {
            const ID: &'static str = stringify!($e);
            unsafe fn create(
                _context: &Context,
                word: *mut *mut c_char,
                word_eol: *mut *mut c_char,
            ) -> Self {
                let server = IrcIdent(from_cstring((*word.offset(1)).offset(1)));
                let target = IrcIdent(from_cstring(*word.offset(3)));
                let $word = word.offset(4);
                let $word_eol = word_eol.offset(4);
                $(
                $s;
                )*
                $(
                let $name = $parser;
                )*
                Self {
                    server,
                    target,
                    $(
                    $name,
                    )*
                }
            }
        }
    }
}

rpl!(RPL_WELCOME[001] {
    global(msg _a) {
        let user_str = from_cstring(*msg.offset(6));
        let user_offset = user_str.find('!').unwrap();
        let host_offset = user_str.find('@').unwrap();
    }
    (this)
    ["The welcomed nickname."]
    nick: String [&str]
        get { &this.nick }
        parse { user_str[..user_offset].to_string() }
    ["The welcomed username."]
    user: String [&str]
        get { &this.user }
        parse { user_str[(user_offset + 1)..host_offset].to_string() }
    ["The host of the welcomed user."]
    host: String [&str]
        get { &this.host }
        parse { user_str[(host_offset + 1)..].to_string() }
});

rpl!(RPL_YOURHOST[002] {
    global(msg _a) {
        let server_str = from_cstring(*msg.offset(3));
        let server_str = server_str[..(server_str.len() - 1)].to_string();
        let version_str = from_cstring(*msg.offset(6));
    }
    (this)
    ["The server name."]
    server_name: String [&str]
        get { &this.server_name }
        parse { server_str.to_string() }
    ["The version string."]
    version: String [&str]
        get { &this.version }
        parse { version_str.to_string() }
});

rpl!(RPL_CREATED[003] {
    global(_a msg) {
        let string = from_cstring(*msg.offset(4));
    }
    (this)
    ["The date the server was created."]
    date: DateTime<Utc> [DateTime<Utc>]
        get { this.date.clone() }
        parse { Utc.datetime_from_str(&string, "%T %b %-e %Y").unwrap() }
});

rpl!(RPL_MYINFO[004] {
    global(msg _a) {}
    (this)
    ["The name of the server."]
    server_name: IrcIdent [IrcIdentRef]
        get { this.server_name.as_ref() }
        parse { IrcIdent(from_cstring(*msg)) }
    ["The version of the server."]
    version: String [&str]
        get { &this.version }
        parse { from_cstring(*msg.offset(1)) }
    ["The available user modes."]
    usermodes: String [&str]
        get { &this.usermodes }
        parse { from_cstring(*msg.offset(2)) }
    ["The available channel modes."]
    chanmodes: String [&str]
        get { &this.chanmodes }
        parse { from_cstring(*msg.offset(3)) }
});

rpl!(RPL_BOUNCE[005] {
    global(msg _a) {
        let server_string = from_cstring(*msg.offset(2));
    }
    (this)
    ["The alternative server to use."]
    server_name: String [&str]
        get { &this.server_name }
        parse { server_string[..(server_string.len() - 1)].to_string() }
    ["The port number of the alternative server."]
    port_number: u16 [u16]
        get { this.port_number }
        parse { from_cstring(*msg.offset(4)).parse().unwrap() }
});

rpl!(RPL_USERHOST[302] {
    global(_a msg) {
        let string = from_cstring((*msg).offset(1));
        let mut vec = Vec::new();
        for reply in string.split(' ') {
            let away_offset = reply.find('=').unwrap();
            let (is_op, nickname) = if let Some(idx) = reply[..away_offset].find('*') {
                (true, IrcIdent(reply[..idx].to_string()))
            } else {
                (false, IrcIdent(reply[..away_offset].to_string()))
            };
            let away_offset = away_offset + 1;
            let is_away = &reply[away_offset..(away_offset + 1)] == "-";
            let hostname = reply[(away_offset + 1)..].to_string();
            vec.push(UserReply { nickname, is_op, is_away, hostname });
        };
    }
    (this)
    ["The users that were replied with."]
    replies: Vec<UserReply> [&[UserReply]]
        get { &this.replies }
        parse { vec }
});

#[derive(Clone, Debug, Eq, PartialEq)]
/// A reply entry to `RPL_USERHOST`.
pub struct UserReply {
    nickname: IrcIdent,
    is_op: bool,
    is_away: bool,
    hostname: String,
}

impl UserReply {
    /// The nickname of the user.
    pub fn nickname(&self) -> IrcIdentRef {
        self.nickname.as_ref()
    }
    /// Whether the user is operator.
    pub fn is_op(&self) -> bool {
        self.is_op
    }
    /// Whether the user is away.
    pub fn is_away(&self) -> bool {
        self.is_away
    }
    /// The hostname of the user.
    pub fn hostname(&self) -> &str {
        &self.hostname
    }
}

rpl!(RPL_ISON[303] {
    global(_a msg) {
        let string = from_cstring((*msg).offset(1));
        let mut vec = Vec::new();
        for nick in string.split(' ') {
            vec.push(IrcIdent(nick.to_string()));
        };
    }
    (this)
    ["The list of nicknames that are online."]
    nicks: Vec<IrcIdent> [&[IrcIdent]]
        get { &this.nicks }
        parse { vec }
});

rpl!(RPL_AWAY[301] {
    global(msg eol) {}
    (this)
    ["The nickname of the user."]
    nick: IrcIdent [IrcIdentRef]
        get { this.nick.as_ref() }
        parse { IrcIdent(from_cstring(*msg)) }
    ["The away message."]
    message: String [&str]
        get { &this.message }
        parse { from_cstring((*eol.offset(1)).offset(1)) }
});

rpl!(RPL_UNAWAY[305] {
    global(_a _b) {} (_c)
});

rpl!(RPL_NOWAWAY[306] {
    global(_a _b) {} (_c)
});

rpl!(RPL_WHOISUSER[311] {
    global(msg eol) {}
    (this)
    ["The nick of the user."]
    nick: IrcIdent [IrcIdentRef]
        get { this.nick.as_ref() }
        parse { IrcIdent(from_cstring(*msg)) }
    ["The username of the user."]
    user: IrcIdent [IrcIdentRef]
        get { this.user.as_ref() }
        parse { IrcIdent(from_cstring(*msg.offset(1))) }
    ["The host of the user."]
    host: String [&str]
        get { &this.host }
        parse { from_cstring(*msg.offset(2)) }
    ["The real name of the user."]
    real_name: String [&str]
        get { &this.real_name }
        parse { from_cstring((*eol.offset(4)).offset(1)) }
});

rpl!(RPL_WHOISSERVER[312] {
    global(msg eol) {}
    (this)
    ["The nick of the user."]
    nick: IrcIdent [IrcIdentRef]
        get { this.nick.as_ref() }
        parse { IrcIdent(from_cstring(*msg)) }
    ["The server of the user."]
    rpl_server: IrcIdent [IrcIdentRef]
        get { this.rpl_server.as_ref() }
        parse { IrcIdent(from_cstring(*msg.offset(1))) }
    ["The server info message."]
    info: String [&str]
        get { &this.info }
        parse { from_cstring((*eol.offset(2)).offset(1)) }
});

rpl!(RPL_WHOISOPERATOR[313] {
    global(msg _a) {}
    (this)
    ["The nick of the user."]
    nick: IrcIdent [IrcIdentRef]
        get { this.nick.as_ref() }
        parse { IrcIdent(from_cstring(*msg)) }
});

rpl!(RPL_WHOISIDLE[317] {
    global(msg _a) {}
    (this)
    ["The nick of the user."]
    nick: IrcIdent [IrcIdentRef]
        get { this.nick.as_ref() }
        parse { IrcIdent(from_cstring(*msg)) }
    ["The duration this user has been idle for."]
    idle: Duration [Duration]
        get { this.idle }
        parse { Duration::from_secs(from_cstring(*msg.offset(1)).parse().unwrap()) }
});

rpl!(RPL_ENDOFWHOIS[318] {
    global(msg _a) {}
    (this)
    ["The nick of the user."]
    nick: IrcIdent [IrcIdentRef]
        get { this.nick.as_ref() }
        parse { IrcIdent(from_cstring(*msg)) }
});

rpl!(RPL_WHOISCHANNELS[319] {
    global(msg eol) {}
    (this)
    ["The nick of the user."]
    nick: IrcIdent [IrcIdentRef]
        get { this.nick.as_ref() }
        parse { IrcIdent(from_cstring(*msg)) }
    ["The channels the user is in."]
    channels: Vec<ChannelEntry> [&[ChannelEntry]]
        get { &this.channels }
        parse {
            let mut vec = Vec::new();
            let channels = from_cstring((*eol.offset(1)).offset(1));
            for channel in channels.split(' ') {
                let (operator, channel) = if &channel[..1] == "@" { (true, &channel[1..]) } else { (false, channel) };
                let (voice, channel) = if &channel[..1] == "+" { (true, &channel[1..]) } else { (false, channel) };
                let channel = IrcIdent(channel.to_string());
                vec.push(ChannelEntry { channel, operator, voice });
            }
            vec
        }
});

/// An entry for `RPL_WHOISCHANNELS`.
pub struct ChannelEntry {
    channel: IrcIdent,
    operator: bool,
    voice: bool,
}

impl ChannelEntry {
    /// The channel that the user is in.
    pub fn channel(&self) -> IrcIdentRef {
        self.channel.as_ref()
    }
    /// Whether the user is an operator.
    pub fn operator(&self) -> bool {
        self.operator
    }
    /// Whether the user has voice.
    pub fn voice(&self) -> bool {
        self.voice
    }
}

rpl!(RPL_WHOWASUSER[314] {
    global(msg eol) {}
    (this)
    ["The nick of the user."]
    nick: IrcIdent [IrcIdentRef]
        get { this.nick.as_ref() }
        parse { IrcIdent(from_cstring(*msg)) }
    ["The username of the user."]
    username: IrcIdent [IrcIdentRef]
        get { this.username.as_ref() }
        parse { IrcIdent(from_cstring(*msg.offset(1))) }
    ["The host of the user."]
    hostname: String [&str]
        get { &this.hostname }
        parse { from_cstring(*msg.offset(2)) }
    ["The real name of the user."]
    realname: String [&str]
        get { &this.realname }
        parse { from_cstring((*eol.offset(4)).offset(1)) }
});

rpl!(RPL_ENDOFWHOWAS[369] {
    global(msg _a) {}
    (this)
    ["The nick of the user."]
    nick: IrcIdent [IrcIdentRef]
        get { this.nick.as_ref() }
        parse { IrcIdent(from_cstring(*msg)) }
});

rpl!(RPL_LIST[322] {
    global(msg eol) {}
    (this)
    ["The channel being listed."]
    channel: IrcIdent [IrcIdentRef]
        get { this.channel.as_ref() }
        parse { IrcIdent(from_cstring(*msg)) }
    ["The number of users visible."]
    visible: u32 [u32]
        get { this.visible }
        parse { from_cstring(*msg.offset(1)).parse().unwrap() }
    ["The topic of the channel."]
    topic: String [&str]
        get { &this.topic }
        parse { from_cstring((*eol.offset(2)).offset(1)) }
});

rpl!(RPL_LISTEND[323] {
    global(_a _b) {} (_c)
});

rpl!(RPL_UNIQOPIS[325] {
    global(msg _a) {}
    (this)
    ["The channel being queried."]
    channel: IrcIdent [IrcIdentRef]
        get { this.channel.as_ref() }
        parse { IrcIdent(from_cstring(*msg)) }
    ["The nick of the user."]
    nick: IrcIdent [IrcIdentRef]
        get { this.nick.as_ref() }
        parse { IrcIdent(from_cstring(*msg.offset(1))) }
});

rpl!(RPL_CHANNELMODEIS[324] {
    global(msg eol) {}
    (this)
    ["The channel being queried."]
    channel: IrcIdent [IrcIdentRef]
        get { this.channel.as_ref() }
        parse { IrcIdent(from_cstring(*msg)) }
    ["The channel mode string."]
    mode: String [&str]
        get { &this.mode }
        parse { from_cstring(*msg.offset(1)) }
    ["The channel mode parameters."]
    params: Vec<String> [&[String]]
        get { &this.params }
        parse {
            let mut vec = Vec::new();
            let string = from_cstring(*eol.offset(2));
            for param in string.split(' ') {
                vec.push(param.to_string());
            }
            vec
        }
});

rpl!(RPL_NOTOPIC[331] {
    global(msg _a) {}
    (this)
    ["The channel being queried."]
    channel: IrcIdent [IrcIdentRef]
        get { this.channel.as_ref() }
        parse { IrcIdent(from_cstring(*msg)) }
});

rpl!(RPL_TOPIC[332] {
    global(msg eol) {}
    (this)
    ["The channel being queried."]
    channel: IrcIdent [IrcIdentRef]
        get { this.channel.as_ref() }
        parse { IrcIdent(from_cstring(*msg)) }
    ["The channel topic."]
    topic: String [&str]
        get { &this.topic }
        parse { from_cstring((*eol.offset(1)).offset(1)) }
});

rpl!(RPL_INVITING[341] {
    global(msg _a) {}
    (this)
    ["The channel being invited to."]
    channel: IrcIdent [IrcIdentRef]
        get { this.channel.as_ref() }
        parse { IrcIdent(from_cstring(*msg)) }
    ["The nick of the user being invited."]
    nick: IrcIdent [IrcIdentRef]
        get { this.nick.as_ref() }
        parse { IrcIdent(from_cstring(*msg.offset(1))) }
});

rpl!(RPL_SUMMONING[342] {
    global(msg _a) {}
    (this)
    ["The user being summoned."]
    user: IrcIdent [IrcIdentRef]
        get { this.user.as_ref() }
        parse { IrcIdent(from_cstring(*msg)) }
});

rpl!(RPL_INVITELIST[346] {
    global(msg _a) {}
    (this)
    ["The channel being invited to."]
    channel: IrcIdent [IrcIdentRef]
        get { this.channel.as_ref() }
        parse { IrcIdent(from_cstring(*msg)) }
    ["The invite mask being invited."]
    invite_mask: UserMask [&UserMask]
        get { &this.invite_mask }
        parse { UserMask::new(from_cstring(*msg.offset(1))).unwrap() }
});

rpl!(RPL_ENDOFINVITELIST[347] {
    global(msg _a) {}
    (this)
    ["The channel being invited to."]
    channel: IrcIdent [IrcIdentRef]
        get { this.channel.as_ref() }
        parse { IrcIdent(from_cstring(*msg)) }
});

rpl!(RPL_EXCEPTLIST[348] {
    global(msg _a) {}
    (this)
    ["The channel being excepted from."]
    channel: IrcIdent [IrcIdentRef]
        get { this.channel.as_ref() }
        parse { IrcIdent(from_cstring(*msg)) }
    ["The exception mask being excepted."]
    exception_mask: UserMask [&UserMask]
        get { &this.exception_mask }
        parse { UserMask::new(from_cstring(*msg.offset(1))).unwrap() }
});

rpl!(RPL_ENDOFEXCEPTLIST[349] {
    global(msg _a) {}
    (this)
    ["The channel being excepted from."]
    channel: IrcIdent [IrcIdentRef]
        get { this.channel.as_ref() }
        parse { IrcIdent(from_cstring(*msg)) }
});

rpl!(RPL_VERSION[351] {
    global(msg eol) {
        let version_string = from_cstring(*msg);
        let debug_offset = version_string.find('.').unwrap();
    }
    (this)
    ["The version of the server."]
    version: String [&str]
        get { &this.version }
        parse { version_string[..debug_offset].to_string() }
    ["The debug level of the server."]
    debug_level: String [&str]
        get { &this.debug_level }
        parse { version_string[(debug_offset + 1)..].to_string() }
    ["The server name."]
    server_name: String [&str]
        get { &this.server_name }
        parse { from_cstring(*msg.offset(1)) }
    ["The server comment."]
    comment: String [&str]
        get { &this.comment }
        parse { from_cstring((*eol.offset(2)).offset(1)) }

});
