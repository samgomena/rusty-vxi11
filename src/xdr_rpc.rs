use serde_xdr::*;
use std::fmt;

// TODO: Could not get this to import from `serde_xdr`
macro_rules! xdr_enum {
    ($name:ident { $($variant:ident = $value:expr, )* }) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub enum $name {
            $($variant = $value,)*
        }

        impl ::serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
                serializer.serialize_i32(*self as i32) // All Enums are signed ints in XDR
            }
        }

        impl<'de> ::serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {

                struct Visitor;

                impl<'a> ::serde::de::Visitor<'a> for Visitor {
                    type Value = $name;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("i32")
                    }

                    fn visit_i32<E>(self, value: i32) -> Result<$name, E> where E: ::serde::de::Error {
                        match value {
                            $( $value => Ok($name::$variant), )*
                            _ => Err(E::custom(
                                format!("unknown {} value: {}",
                                stringify!($name), value))),
                        }
                    }
                }
                deserializer.deserialize_i32(Visitor)
            }
        }
    }
}

xdr_enum!(MsgType {
  Call = 0,
  Reply = 1,
});
xdr_enum!(ReplyStat {
  MsgAccepted = 0,
  MsgDenied = 1,
});
xdr_enum!(AcceptStat {
  Success = 0,
  ProgUnavail = 1,
  ProgMismatch = 2,
  ProcUnavail = 3,
  GarbageArgs = 4,
  SystemErr = 5,
});
xdr_enum!(RejectStat {
  RpcMismatch = 0,
  AuthError = 1,
});
xdr_enum!(AuthStat {
  AuthOk = 0,
  AuthBadcred = 1,
  AuthRejectedcred = 2,
  AuthBadverf = 3,
  AuthRejectedverf = 4,
  AuthTooweak = 5,
  AuthInvalidresp = 6,
  AuthFailed = 7,
  AuthKerbGeneric = 8,
  AuthTimeexpire = 9,
  AuthTktFile = 10,
  AuthDecode = 11,
  AuthNetAddr = 12,
  RpcsecGssCredproblem = 13,
  RpcsecGssCtxproblem = 14,
});
xdr_enum!(AuthFlavor {
  AuthNone = 0,
  AuthSys = 1,
  AuthShort = 2,
  AuthDh = 3,
  RpcsecGss = 6,
});

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct OpaqueAuth {
    pub flavor: AuthFlavor,
    pub body: Vec<u8>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct CallBody {
    pub rpcvers: u32,
    pub prog: u32,
    pub vers: u32,
    pub proc_: u32,
    pub cred: OpaqueAuth,
    pub verf: OpaqueAuth,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct AcceptedReply {
    pub verf: OpaqueAuth,
    pub reply_data: ReplyData,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename(deserialize = "__UNION_SYMBOL__"))]
pub enum RejectedReply {
    #[serde(rename = "0")]
    RpcMismatch { mismatch_info: MismatchInfo },
    #[serde(rename = "1")]
    AuthError { stat: AuthStat },
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename(deserialize = "__UNION_SYMBOL__"))]
pub enum ReplyBody {
    #[serde(rename = "0")]
    MsgAccepted { areply: AcceptedReply },
    #[serde(rename = "1")]
    MsgDenied { rreply: RejectedReply },
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct RpcMsg {
    pub xid: u32,
    pub body: Body,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename(deserialize = "__UNION_SYMBOL__"))]
pub enum ReplyData {
    #[serde(rename = "0")]
    Success {},
    #[serde(rename = "2")]
    ProgMismatch {
        vers: u32,
    },
    // Default case for the XDR Union
    UnionDefault_ {},
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct MismatchInfo {
    pub low: u32,
    pub high: u32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename(deserialize = "__UNION_SYMBOL__"))]
pub enum Body {
    #[serde(rename = "0")]
    Call { cbody: CallBody },
    #[serde(rename = "1")]
    Reply { rbody: ReplyBody },
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct XdrRequest<T> {
    pub xid: u32,
    pub val: T,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct XdrResponse<T> {
    pub xid: u32,
    pub val: T,
}

pub trait HasXid {
    fn get_xid(&self) -> u32;
}

impl<T> HasXid for XdrRequest<T> {
    fn get_xid(&self) -> u32 {
        self.xid
    }
}

impl<T> HasXid for XdrResponse<T> {
    fn get_xid(&self) -> u32 {
        self.xid
    }
}
