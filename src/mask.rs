use crate::call;
use crate::Context;
use std::cmp::Ordering;
use std::convert::TryFrom;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::ops::{Deref, Range};

/// Represents a usermask, typically formatted like `nick!user@address`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserMask {
    mask: String,
    nick: Range<usize>,
    username: Range<usize>,
    address: Range<usize>,
    host: Range<usize>,
    domain: Range<usize>,
}

impl Ord for UserMask {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_nick()
            .cmp(&other.get_nick())
            .then_with(|| self.get_username().cmp(&other.get_username()))
            .then_with(|| self.get_address().cmp(other.get_address()))
    }
}

impl PartialOrd for UserMask {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl UserMask {
    /// Creates a new `UserMask` from a user mask string. Must be in `nick!user@address` form.
    /// Returns the newly created usermask, or `None` if `mask` was not formatted correctly.
    pub fn new(mask: impl Into<String>) -> Option<Self> {
        let mask = mask.into();
        let user_offset = mask.find('!')?;
        let ip_offset = mask.find('@')?;
        let len = mask.len();
        if user_offset > ip_offset
            || user_offset == 0
            || ip_offset == user_offset + 1
            || ip_offset == len - 1
        {
            return None;
        }
        let (host, domain) = {
            let address = &mask[(ip_offset + 1)..len];
            let begin = ip_offset + 1;
            if address.chars().all(|c| c.is_ascii_digit()) {
                let offset = address.rfind('.')?;
                ((begin + offset)..len, begin..(begin + offset))
            } else {
                let first_dot = address.rfind('.')?;
                let offset = address[..first_dot].rfind('.')?;
                (begin..(begin + offset), (begin + offset)..len)
            }
        };
        Some(Self {
            mask,
            nick: 0..user_offset,
            username: (user_offset + 1)..ip_offset,
            address: (ip_offset + 1)..len,
            host,
            domain,
        })
    }
    /// Gets the usermask string.
    pub fn as_str(&self) -> &str {
        &self.mask
    }
    /// Consumes this object and returns the inner usermask string.
    pub fn into_string(self) -> String {
        self.mask
    }
    /// Gets the nick component of the usermask string.
    pub fn get_nick(&self) -> IrcIdentRef {
        IrcIdentRef(&self.mask[self.nick.clone()])
    }
    /// Gets the username component of the usermask string.
    pub fn get_username(&self) -> IrcIdentRef {
        IrcIdentRef(&self.mask[self.username.clone()])
    }
    /// Gets the address component of the usermask string.
    pub fn get_address(&self) -> &str {
        &self.mask[self.address.clone()]
    }
    /// Gets the host component of the address.
    pub fn get_host(&self) -> &str {
        &self.mask[self.host.clone()]
    }
    /// Gets the domain component of the address.
    pub fn get_domain(&self) -> &str {
        &self.mask[self.domain.clone()]
    }
}

impl Deref for UserMask {
    type Target = str;
    fn deref(&self) -> &str {
        self.as_str()
    }
}

impl TryFrom<String> for UserMask {
    type Error = ();
    fn try_from(string: String) -> Result<Self, Self::Error> {
        Self::new(string).ok_or(())
    }
}

impl Into<String> for UserMask {
    fn into(self) -> String {
        self.into_string()
    }
}

impl Display for UserMask {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", &self.mask)
    }
}

/// Represents a ban mask, typically formatted like `nick!user@address`, where any of the components
/// can be replaced with a `*`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BanMask {
    mask: String,
    nick: Range<usize>,
    username: Range<usize>,
    address: Range<usize>,
    host: Range<usize>,
    domain: Range<usize>,
}

impl Ord for BanMask {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_nick()
            .cmp(&other.get_nick())
            .then_with(|| self.get_username().cmp(&other.get_username()))
            .then_with(|| self.get_host().cmp(&other.get_host()))
            .then_with(|| self.get_domain().cmp(&other.get_domain()))
    }
}

impl PartialOrd for BanMask {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl BanMask {
    /// Creates a new `BanMask` from a banmask string. Must be in `nick!user@address` form.
    /// Returns the newly created banmask, or `None` if `mask` was not formatted correctly.
    pub fn new(mask: impl Into<String>) -> Option<Self> {
        let mask = mask.into();
        let user_offset = mask.find('!')?;
        let ip_offset = mask.find('@')?;
        let len = mask.len();
        if user_offset > ip_offset
            || user_offset == 0
            || ip_offset == user_offset + 1
            || ip_offset == len - 1
        {
            return None;
        }
        let (host, domain) = {
            let address = &mask[(ip_offset + 1)..len];
            let begin = ip_offset + 1;
            if address.chars().all(|c| c.is_ascii_digit()) {
                let offset = address.rfind('.')?;
                ((begin + offset)..len, begin..(begin + offset))
            } else {
                let first_dot = address.rfind('.')?;
                let offset = address[..first_dot].rfind('.')?;
                (begin..(begin + offset), (begin + offset)..len)
            }
        };
        Some(Self {
            mask,
            nick: 0..user_offset,
            username: (user_offset + 1)..ip_offset,
            address: (ip_offset + 1)..len,
            host,
            domain,
        })
    }
    /// Gets the banmask string.
    pub fn as_str(&self) -> &str {
        &self.mask
    }
    /// Consumes this object and returns the inner banmask string.
    pub fn into_string(self) -> String {
        self.mask
    }
    fn get_or_wildcard(&self, range: Range<usize>) -> Option<&str> {
        let string = &self.mask[range];
        if string == "*" {
            None
        } else {
            Some(string)
        }
    }
    /// Gets the nick component of the banmask string, or `None` if wildcard.
    pub fn get_nick(&self) -> Option<IrcIdentRef> {
        self.get_or_wildcard(self.nick.clone()).map(IrcIdentRef)
    }
    /// Gets the username component of the banmask string, or `None` if wildcard.
    pub fn get_username(&self) -> Option<IrcIdentRef> {
        self.get_or_wildcard(self.username.clone()).map(IrcIdentRef)
    }
    /// Gets the host component of the address, or `None` if wildcard.
    pub fn get_host(&self) -> Option<&str> {
        self.get_or_wildcard(self.host.clone())
    }
    /// Gets the domain component of the address, or `None` if wildcard.
    pub fn get_domain(&self) -> Option<&str> {
        self.get_or_wildcard(self.domain.clone())
    }
}

impl TryFrom<String> for BanMask {
    type Error = ();
    fn try_from(string: String) -> Result<Self, Self::Error> {
        Self::new(string).ok_or(())
    }
}

impl Into<String> for BanMask {
    fn into(self) -> String {
        self.into_string()
    }
}

impl Deref for BanMask {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl Display for BanMask {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", &self.mask)
    }
}

impl TryFrom<BanMask> for UserMask {
    type Error = ();
    fn try_from(mask: BanMask) -> Result<Self, Self::Error> {
        if mask.get_nick() == None
            || mask.get_username() == None
            || mask.get_host() == None
            || mask.get_domain() == None
        {
            Err(())
        } else {
            let BanMask {
                mask,
                nick,
                username,
                address,
                host,
                domain,
            } = mask;
            Ok(Self {
                mask,
                nick,
                username,
                address,
                host,
                domain,
            })
        }
    }
}

impl From<UserMask> for BanMask {
    fn from(mask: UserMask) -> Self {
        let UserMask {
            mask,
            nick,
            username,
            address,
            host,
            domain,
        } = mask;
        Self {
            mask,
            nick,
            username,
            address,
            host,
            domain,
        }
    }
}

/// An IRC identifier. Mainly used for its `Ord` implementation.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct IrcIdentRef<'a>(pub &'a str);

impl<'a> Ord for IrcIdentRef<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        let guard = call::get_plugin();
        let context = Context { handle: guard.ph };
        context.name_cmp(self.0, other.0)
    }
}

impl<'a> PartialOrd for IrcIdentRef<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Deref for IrcIdentRef<'a> {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'a> Into<&'a str> for IrcIdentRef<'a> {
    fn into(self) -> &'a str {
        self.0
    }
}

impl<'a> From<&'a str> for IrcIdentRef<'a> {
    fn from(string: &'a str) -> Self {
        Self(string)
    }
}

impl<'a> Display for IrcIdentRef<'a> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.0)
    }
}

/// An IRC identifier. Mainly used for its `Ord` implementation.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct IrcIdent(pub String);

impl IrcIdent {
    /// Turns an `IrcIdent` into an `IrcIdentRef`.
    pub fn as_ref(&self) -> IrcIdentRef {
        IrcIdentRef(&self.0)
    }
}

impl Ord for IrcIdent {
    fn cmp(&self, other: &Self) -> Ordering {
        let guard = call::get_plugin();
        let context = Context { handle: guard.ph };
        context.name_cmp(&self.0, &other.0)
    }
}

impl PartialOrd for IrcIdent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Deref for IrcIdent {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Into<String> for IrcIdent {
    fn into(self) -> String {
        self.0
    }
}

impl From<String> for IrcIdent {
    fn from(string: String) -> Self {
        Self(string)
    }
}

impl Display for IrcIdent {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", &self.0)
    }
}
