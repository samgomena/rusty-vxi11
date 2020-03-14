#![allow(deprecated)]
// Most functions and fields are named identically to the VXI11 standard
// In an effort for continuity they share the same names here.
#![allow(non_snake_case)]

use std::{fmt, io};
use tokio_core::io::EasyBuf;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DeviceAddrfamily {
  DeviceTcp,
  DeviceUdp,
}

impl ::serde::Serialize for DeviceAddrfamily {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: ::serde::Serializer,
  {
    serializer.serialize_i32(*self as i32) // All Enums are signed ints in XDR
  }
}

impl<'de> ::serde::Deserialize<'de> for DeviceAddrfamily {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: ::serde::Deserializer<'de>,
  {
    struct Visitor;

    impl<'a> ::serde::de::Visitor<'a> for Visitor {
      type Value = DeviceAddrfamily;

      fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("i32")
      }

      fn visit_i32<E>(self, value: i32) -> Result<DeviceAddrfamily, E>
      where
        E: ::serde::de::Error,
      {
        match value {
          0 => Ok(DeviceAddrfamily::DeviceTcp),
          1 => Ok(DeviceAddrfamily::DeviceUdp),
          _ => Err(E::custom(format!(
            "unknown {} value: {}",
            stringify!(DeviceAddrfamily),
            value
          ))),
        }
      }
    }
    deserializer.deserialize_i32(Visitor)
  }
}

type Long = u32;
pub type DeviceLink = Long;
pub type DeviceFlags = Long;
pub type DeviceErrorcode = Long;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct DeviceError {
  pub error: DeviceErrorcode,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct CreateLinkparms {
  pub clientId: Long,
  pub lockDevice: bool,
  pub lock_timeout: u64,
  pub device: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct CreateLinkresp {
  pub error: DeviceErrorcode,
  pub lid: DeviceLink,
  pub abortPort: u32,
  pub maxRecvSize: u64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct DeviceWriteparms {
  pub lid: DeviceLink,
  pub io_timeout: u64,
  pub lock_timeout: u64,
  pub flags: DeviceFlags,
  pub data: Vec<u8>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct DeviceWriteresp {
  pub error: DeviceErrorcode,
  pub size: u64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct DeviceReadparms {
  pub lid: DeviceLink,
  pub requestSize: u64,
  pub io_timeout: u64,
  pub lock_timeout: u64,
  pub flags: DeviceFlags,
  pub termChar: u8,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct DeviceReadresp {
  pub error: DeviceErrorcode,
  pub reason: Long,
  pub data: Vec<u8>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct DeviceReadstbresp {
  pub error: DeviceErrorcode,
  pub stb: u32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct DeviceGenericparms {
  pub lid: DeviceLink,
  pub flags: DeviceFlags,
  pub lock_timeout: u64,
  pub io_timeout: u64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct DeviceRemotefunc {
  pub hostAddr: u64,
  pub hostPort: u32,
  pub progNum: u64,
  pub progVers: u64,
  pub progFamily: DeviceAddrfamily,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct DeviceEnablesrqparms {
  pub lid: DeviceLink,
  pub enable: bool,
  pub handle: Vec<u8>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct DeviceLockparms {
  pub lid: DeviceLink,
  pub flags: DeviceFlags,
  pub lock_timeout: u64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct DeviceDocmdparms {
  pub lid: DeviceLink,
  pub flags: DeviceFlags,
  pub io_timeout: u64,
  pub lock_timeout: u64,
  pub cmd: Long,
  pub network_order: bool,
  pub datasize: Long,
  pub data_in: Vec<u8>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct DeviceDocmdresp {
  pub error: DeviceErrorcode,
  pub data_out: Vec<u8>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum DeviceAsyncRequest {
  V1(DeviceAsyncRequestV1),
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum DeviceAsyncResponse {
  V1(DeviceAsyncResponseV1),
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum DeviceAsyncRequestV1 {
  DeviceAbort(DeviceLink),
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum DeviceAsyncResponseV1 {
  DeviceAbort(DeviceError),
}
pub fn device_async_decode(
  _program: u32,
  version: u32,
  procedure: u32,
  buf: &mut EasyBuf,
) -> io::Result<Option<DeviceAsyncRequest>> {
  match version {
    1u32 => device_async_decode_v1(procedure, buf),
    _ => {
      return Err(io::Error::new(io::ErrorKind::Other, "unknown version"));
    }
  }
}
pub fn device_async_decode_v1(
  procedure: u32,
  buf: &mut EasyBuf,
) -> io::Result<Option<DeviceAsyncRequest>> {
  let request = match procedure {
    1u32 => device_async_decode_v1_device_abort(buf),
    _ => {
      return Err(io::Error::new(io::ErrorKind::Other, "unknown procedure"));
    }
  };
  Ok(Some(DeviceAsyncRequest::V1(request.unwrap().unwrap())))
}
pub fn device_async_decode_v1_device_abort(
  buf: &mut EasyBuf,
) -> io::Result<Option<DeviceAsyncRequestV1>> {
  let res0 = serde_xdr::from_bytes::<DeviceLink>(buf.as_slice());
  let arg0 = match res0 {
    Ok((arg, consumed)) => {
      buf.drain_to(consumed);
      arg
    }
    Err(e) => match e {
      serde_xdr::EncoderError::Io(i) => {
        return Err(i);
      }
      serde_xdr::EncoderError::Unknown(s) => {
        return Err(io::Error::new(
          io::ErrorKind::Other,
          format!("argument 0 parse failure: {}", s),
        ));
      }
    },
  };

  Ok(Some(DeviceAsyncRequestV1::DeviceAbort(arg0)))
}
pub fn encode_device_async(msg: DeviceAsyncResponse, buf: &mut Vec<u8>) -> io::Result<()> {
  match msg {
    DeviceAsyncResponse::V1(rsp) => {
      match rsp {
        DeviceAsyncResponseV1::DeviceAbort(r) => {
          serde_xdr::to_bytes(&r, buf)?;
        }
        _ => {
          return Err(io::Error::new(io::ErrorKind::Other, "unknown procedure"));
        }
      }
      Ok(())
    }
    _ => {
      return Err(io::Error::new(io::ErrorKind::Other, "unknown version"));
    }
  }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum DeviceCoreRequest {
  V1(DeviceCoreRequestV1),
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum DeviceCoreResponse {
  V1(DeviceCoreResponseV1),
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum DeviceCoreRequestV1 {
  CreateLink(CreateLinkparms),
  DeviceWrite(DeviceWriteparms),
  DeviceRead(DeviceReadparms),
  DeviceReadstb(DeviceGenericparms),
  DeviceTrigger(DeviceGenericparms),
  DeviceClear(DeviceGenericparms),
  DeviceRemote(DeviceGenericparms),
  DeviceLocal(DeviceGenericparms),
  DeviceLock(DeviceLockparms),
  DeviceUnlock(DeviceLink),
  DeviceEnableSrq(DeviceEnablesrqparms),
  DeviceDocmd(DeviceDocmdparms),
  DestroyLink(DeviceLink),
  CreateIntrChan(DeviceRemotefunc),
  DestroyIntrChan,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum DeviceCoreResponseV1 {
  CreateLink(CreateLinkresp),
  DeviceWrite(DeviceWriteresp),
  DeviceRead(DeviceReadresp),
  DeviceReadstb(DeviceReadstbresp),
  DeviceTrigger(DeviceError),
  DeviceClear(DeviceError),
  DeviceRemote(DeviceError),
  DeviceLocal(DeviceError),
  DeviceLock(DeviceError),
  DeviceUnlock(DeviceError),
  DeviceEnableSrq(DeviceError),
  DeviceDocmd(DeviceDocmdresp),
  DestroyLink(DeviceError),
  CreateIntrChan(DeviceError),
  DestroyIntrChan(DeviceError),
}
pub fn device_core_decode(
  _program: u32,
  version: u32,
  procedure: u32,
  buf: &mut EasyBuf,
) -> io::Result<Option<DeviceCoreRequest>> {
  match version {
    1u32 => device_core_decode_v1(procedure, buf),
    _ => {
      return Err(io::Error::new(io::ErrorKind::Other, "unknown version"));
    }
  }
}
pub fn device_core_decode_v1(
  procedure: u32,
  buf: &mut EasyBuf,
) -> io::Result<Option<DeviceCoreRequest>> {
  let request = match procedure {
    10u32 => device_core_decode_v1_create_link(buf),
    11u32 => device_core_decode_v1_device_write(buf),
    12u32 => device_core_decode_v1_device_read(buf),
    13u32 => device_core_decode_v1_device_readstb(buf),
    14u32 => device_core_decode_v1_device_trigger(buf),
    15u32 => device_core_decode_v1_device_clear(buf),
    16u32 => device_core_decode_v1_device_remote(buf),
    17u32 => device_core_decode_v1_device_local(buf),
    18u32 => device_core_decode_v1_device_lock(buf),
    19u32 => device_core_decode_v1_device_unlock(buf),
    20u32 => device_core_decode_v1_device_enable_srq(buf),
    22u32 => device_core_decode_v1_device_docmd(buf),
    23u32 => device_core_decode_v1_destroy_link(buf),
    25u32 => device_core_decode_v1_create_intr_chan(buf),
    26u32 => device_core_decode_v1_destroy_intr_chan(buf),
    _ => {
      return Err(io::Error::new(io::ErrorKind::Other, "unknown procedure"));
    }
  };
  Ok(Some(DeviceCoreRequest::V1(request.unwrap().unwrap())))
}
pub fn device_core_decode_v1_create_link(
  buf: &mut EasyBuf,
) -> io::Result<Option<DeviceCoreRequestV1>> {
  let res0 = serde_xdr::from_bytes::<CreateLinkparms>(buf.as_slice());
  let arg0 = match res0 {
    Ok((arg, consumed)) => {
      buf.drain_to(consumed);
      arg
    }
    Err(e) => match e {
      serde_xdr::EncoderError::Io(i) => {
        return Err(i);
      }
      serde_xdr::EncoderError::Unknown(s) => {
        return Err(io::Error::new(
          io::ErrorKind::Other,
          format!("argument 0 parse failure: {}", s),
        ));
      }
    },
  };

  Ok(Some(DeviceCoreRequestV1::CreateLink(arg0)))
}
pub fn device_core_decode_v1_device_write(
  buf: &mut EasyBuf,
) -> io::Result<Option<DeviceCoreRequestV1>> {
  let res0 = serde_xdr::from_bytes::<DeviceWriteparms>(buf.as_slice());
  let arg0 = match res0 {
    Ok((arg, consumed)) => {
      buf.drain_to(consumed);
      arg
    }
    Err(e) => match e {
      serde_xdr::EncoderError::Io(i) => {
        return Err(i);
      }
      serde_xdr::EncoderError::Unknown(s) => {
        return Err(io::Error::new(
          io::ErrorKind::Other,
          format!("argument 0 parse failure: {}", s),
        ));
      }
    },
  };

  Ok(Some(DeviceCoreRequestV1::DeviceWrite(arg0)))
}
pub fn device_core_decode_v1_device_read(
  buf: &mut EasyBuf,
) -> io::Result<Option<DeviceCoreRequestV1>> {
  let res0 = serde_xdr::from_bytes::<DeviceReadparms>(buf.as_slice());
  let arg0 = match res0 {
    Ok((arg, consumed)) => {
      buf.drain_to(consumed);
      arg
    }
    Err(e) => match e {
      serde_xdr::EncoderError::Io(i) => {
        return Err(i);
      }
      serde_xdr::EncoderError::Unknown(s) => {
        return Err(io::Error::new(
          io::ErrorKind::Other,
          format!("argument 0 parse failure: {}", s),
        ));
      }
    },
  };

  Ok(Some(DeviceCoreRequestV1::DeviceRead(arg0)))
}
pub fn device_core_decode_v1_device_readstb(
  buf: &mut EasyBuf,
) -> io::Result<Option<DeviceCoreRequestV1>> {
  let res0 = serde_xdr::from_bytes::<DeviceGenericparms>(buf.as_slice());
  let arg0 = match res0 {
    Ok((arg, consumed)) => {
      buf.drain_to(consumed);
      arg
    }
    Err(e) => match e {
      serde_xdr::EncoderError::Io(i) => {
        return Err(i);
      }
      serde_xdr::EncoderError::Unknown(s) => {
        return Err(io::Error::new(
          io::ErrorKind::Other,
          format!("argument 0 parse failure: {}", s),
        ));
      }
    },
  };

  Ok(Some(DeviceCoreRequestV1::DeviceReadstb(arg0)))
}
pub fn device_core_decode_v1_device_trigger(
  buf: &mut EasyBuf,
) -> io::Result<Option<DeviceCoreRequestV1>> {
  let res0 = serde_xdr::from_bytes::<DeviceGenericparms>(buf.as_slice());
  let arg0 = match res0 {
    Ok((arg, consumed)) => {
      buf.drain_to(consumed);
      arg
    }
    Err(e) => match e {
      serde_xdr::EncoderError::Io(i) => {
        return Err(i);
      }
      serde_xdr::EncoderError::Unknown(s) => {
        return Err(io::Error::new(
          io::ErrorKind::Other,
          format!("argument 0 parse failure: {}", s),
        ));
      }
    },
  };

  Ok(Some(DeviceCoreRequestV1::DeviceTrigger(arg0)))
}
pub fn device_core_decode_v1_device_clear(
  buf: &mut EasyBuf,
) -> io::Result<Option<DeviceCoreRequestV1>> {
  let res0 = serde_xdr::from_bytes::<DeviceGenericparms>(buf.as_slice());
  let arg0 = match res0 {
    Ok((arg, consumed)) => {
      buf.drain_to(consumed);
      arg
    }
    Err(e) => match e {
      serde_xdr::EncoderError::Io(i) => {
        return Err(i);
      }
      serde_xdr::EncoderError::Unknown(s) => {
        return Err(io::Error::new(
          io::ErrorKind::Other,
          format!("argument 0 parse failure: {}", s),
        ));
      }
    },
  };

  Ok(Some(DeviceCoreRequestV1::DeviceClear(arg0)))
}
pub fn device_core_decode_v1_device_remote(
  buf: &mut EasyBuf,
) -> io::Result<Option<DeviceCoreRequestV1>> {
  let res0 = serde_xdr::from_bytes::<DeviceGenericparms>(buf.as_slice());
  let arg0 = match res0 {
    Ok((arg, consumed)) => {
      buf.drain_to(consumed);
      arg
    }
    Err(e) => match e {
      serde_xdr::EncoderError::Io(i) => {
        return Err(i);
      }
      serde_xdr::EncoderError::Unknown(s) => {
        return Err(io::Error::new(
          io::ErrorKind::Other,
          format!("argument 0 parse failure: {}", s),
        ));
      }
    },
  };

  Ok(Some(DeviceCoreRequestV1::DeviceRemote(arg0)))
}
pub fn device_core_decode_v1_device_local(
  buf: &mut EasyBuf,
) -> io::Result<Option<DeviceCoreRequestV1>> {
  let res0 = serde_xdr::from_bytes::<DeviceGenericparms>(buf.as_slice());
  let arg0 = match res0 {
    Ok((arg, consumed)) => {
      buf.drain_to(consumed);
      arg
    }
    Err(e) => match e {
      serde_xdr::EncoderError::Io(i) => {
        return Err(i);
      }
      serde_xdr::EncoderError::Unknown(s) => {
        return Err(io::Error::new(
          io::ErrorKind::Other,
          format!("argument 0 parse failure: {}", s),
        ));
      }
    },
  };

  Ok(Some(DeviceCoreRequestV1::DeviceLocal(arg0)))
}
pub fn device_core_decode_v1_device_lock(
  buf: &mut EasyBuf,
) -> io::Result<Option<DeviceCoreRequestV1>> {
  let res0 = serde_xdr::from_bytes::<DeviceLockparms>(buf.as_slice());
  let arg0 = match res0 {
    Ok((arg, consumed)) => {
      buf.drain_to(consumed);
      arg
    }
    Err(e) => match e {
      serde_xdr::EncoderError::Io(i) => {
        return Err(i);
      }
      serde_xdr::EncoderError::Unknown(s) => {
        return Err(io::Error::new(
          io::ErrorKind::Other,
          format!("argument 0 parse failure: {}", s),
        ));
      }
    },
  };

  Ok(Some(DeviceCoreRequestV1::DeviceLock(arg0)))
}
pub fn device_core_decode_v1_device_unlock(
  buf: &mut EasyBuf,
) -> io::Result<Option<DeviceCoreRequestV1>> {
  let res0 = serde_xdr::from_bytes::<DeviceLink>(buf.as_slice());
  let arg0 = match res0 {
    Ok((arg, consumed)) => {
      buf.drain_to(consumed);
      arg
    }
    Err(e) => match e {
      serde_xdr::EncoderError::Io(i) => {
        return Err(i);
      }
      serde_xdr::EncoderError::Unknown(s) => {
        return Err(io::Error::new(
          io::ErrorKind::Other,
          format!("argument 0 parse failure: {}", s),
        ));
      }
    },
  };

  Ok(Some(DeviceCoreRequestV1::DeviceUnlock(arg0)))
}
pub fn device_core_decode_v1_device_enable_srq(
  buf: &mut EasyBuf,
) -> io::Result<Option<DeviceCoreRequestV1>> {
  let res0 = serde_xdr::from_bytes::<DeviceEnablesrqparms>(buf.as_slice());
  let arg0 = match res0 {
    Ok((arg, consumed)) => {
      buf.drain_to(consumed);
      arg
    }
    Err(e) => match e {
      serde_xdr::EncoderError::Io(i) => {
        return Err(i);
      }
      serde_xdr::EncoderError::Unknown(s) => {
        return Err(io::Error::new(
          io::ErrorKind::Other,
          format!("argument 0 parse failure: {}", s),
        ));
      }
    },
  };

  Ok(Some(DeviceCoreRequestV1::DeviceEnableSrq(arg0)))
}
pub fn device_core_decode_v1_device_docmd(
  buf: &mut EasyBuf,
) -> io::Result<Option<DeviceCoreRequestV1>> {
  let res0 = serde_xdr::from_bytes::<DeviceDocmdparms>(buf.as_slice());
  let arg0 = match res0 {
    Ok((arg, consumed)) => {
      buf.drain_to(consumed);
      arg
    }
    Err(e) => match e {
      serde_xdr::EncoderError::Io(i) => {
        return Err(i);
      }
      serde_xdr::EncoderError::Unknown(s) => {
        return Err(io::Error::new(
          io::ErrorKind::Other,
          format!("argument 0 parse failure: {}", s),
        ));
      }
    },
  };

  Ok(Some(DeviceCoreRequestV1::DeviceDocmd(arg0)))
}
pub fn device_core_decode_v1_destroy_link(
  buf: &mut EasyBuf,
) -> io::Result<Option<DeviceCoreRequestV1>> {
  let res0 = serde_xdr::from_bytes::<DeviceLink>(buf.as_slice());
  let arg0 = match res0 {
    Ok((arg, consumed)) => {
      buf.drain_to(consumed);
      arg
    }
    Err(e) => match e {
      serde_xdr::EncoderError::Io(i) => {
        return Err(i);
      }
      serde_xdr::EncoderError::Unknown(s) => {
        return Err(io::Error::new(
          io::ErrorKind::Other,
          format!("argument 0 parse failure: {}", s),
        ));
      }
    },
  };

  Ok(Some(DeviceCoreRequestV1::DestroyLink(arg0)))
}
pub fn device_core_decode_v1_create_intr_chan(
  buf: &mut EasyBuf,
) -> io::Result<Option<DeviceCoreRequestV1>> {
  let res0 = serde_xdr::from_bytes::<DeviceRemotefunc>(buf.as_slice());
  let arg0 = match res0 {
    Ok((arg, consumed)) => {
      buf.drain_to(consumed);
      arg
    }
    Err(e) => match e {
      serde_xdr::EncoderError::Io(i) => {
        return Err(i);
      }
      serde_xdr::EncoderError::Unknown(s) => {
        return Err(io::Error::new(
          io::ErrorKind::Other,
          format!("argument 0 parse failure: {}", s),
        ));
      }
    },
  };

  Ok(Some(DeviceCoreRequestV1::CreateIntrChan(arg0)))
}
pub fn device_core_decode_v1_destroy_intr_chan(
  _buf: &mut EasyBuf,
) -> io::Result<Option<DeviceCoreRequestV1>> {
  Ok(Some(DeviceCoreRequestV1::DestroyIntrChan))
}
pub fn encode_device_core(msg: DeviceCoreResponse, buf: &mut Vec<u8>) -> io::Result<()> {
  match msg {
    DeviceCoreResponse::V1(rsp) => {
      match rsp {
        DeviceCoreResponseV1::CreateLink(r) => {
          serde_xdr::to_bytes(&r, buf)?;
        }
        DeviceCoreResponseV1::DeviceWrite(r) => {
          serde_xdr::to_bytes(&r, buf)?;
        }
        DeviceCoreResponseV1::DeviceRead(r) => {
          serde_xdr::to_bytes(&r, buf)?;
        }
        DeviceCoreResponseV1::DeviceReadstb(r) => {
          serde_xdr::to_bytes(&r, buf)?;
        }
        DeviceCoreResponseV1::DeviceTrigger(r) => {
          serde_xdr::to_bytes(&r, buf)?;
        }
        DeviceCoreResponseV1::DeviceClear(r) => {
          serde_xdr::to_bytes(&r, buf)?;
        }
        DeviceCoreResponseV1::DeviceRemote(r) => {
          serde_xdr::to_bytes(&r, buf)?;
        }
        DeviceCoreResponseV1::DeviceLocal(r) => {
          serde_xdr::to_bytes(&r, buf)?;
        }
        DeviceCoreResponseV1::DeviceLock(r) => {
          serde_xdr::to_bytes(&r, buf)?;
        }
        DeviceCoreResponseV1::DeviceUnlock(r) => {
          serde_xdr::to_bytes(&r, buf)?;
        }
        DeviceCoreResponseV1::DeviceEnableSrq(r) => {
          serde_xdr::to_bytes(&r, buf)?;
        }
        DeviceCoreResponseV1::DeviceDocmd(r) => {
          serde_xdr::to_bytes(&r, buf)?;
        }
        DeviceCoreResponseV1::DestroyLink(r) => {
          serde_xdr::to_bytes(&r, buf)?;
        }
        DeviceCoreResponseV1::CreateIntrChan(r) => {
          serde_xdr::to_bytes(&r, buf)?;
        }
        DeviceCoreResponseV1::DestroyIntrChan(r) => {
          serde_xdr::to_bytes(&r, buf)?;
        }
        _ => {
          return Err(io::Error::new(io::ErrorKind::Other, "unknown procedure"));
        }
      }
      Ok(())
    }
    _ => {
      return Err(io::Error::new(io::ErrorKind::Other, "unknown version"));
    }
  }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct DeviceSrqparms {
  pub handle: Vec<u8>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum DeviceIntrRequest {
  V1(DeviceIntrRequestV1),
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum DeviceIntrResponse {
  V1(DeviceIntrResponseV1),
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum DeviceIntrRequestV1 {
  DeviceIntrSrq(DeviceSrqparms),
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum DeviceIntrResponseV1 {
  DeviceIntrSrq,
}
pub fn device_intr_decode(
  _program: u32,
  version: u32,
  procedure: u32,
  buf: &mut EasyBuf,
) -> io::Result<Option<DeviceIntrRequest>> {
  match version {
    1u32 => device_intr_decode_v1(procedure, buf),
    _ => {
      return Err(io::Error::new(io::ErrorKind::Other, "unknown version"));
    }
  }
}
pub fn device_intr_decode_v1(
  procedure: u32,
  buf: &mut EasyBuf,
) -> io::Result<Option<DeviceIntrRequest>> {
  let request = match procedure {
    30u32 => device_intr_decode_v1_device_intr_srq(buf),
    _ => {
      return Err(io::Error::new(io::ErrorKind::Other, "unknown procedure"));
    }
  };
  Ok(Some(DeviceIntrRequest::V1(request.unwrap().unwrap())))
}
pub fn device_intr_decode_v1_device_intr_srq(
  buf: &mut EasyBuf,
) -> io::Result<Option<DeviceIntrRequestV1>> {
  let res0 = serde_xdr::from_bytes::<DeviceSrqparms>(buf.as_slice());
  let arg0 = match res0 {
    Ok((arg, consumed)) => {
      buf.drain_to(consumed);
      arg
    }
    Err(e) => match e {
      serde_xdr::EncoderError::Io(i) => {
        return Err(i);
      }
      serde_xdr::EncoderError::Unknown(s) => {
        return Err(io::Error::new(
          io::ErrorKind::Other,
          format!("argument 0 parse failure: {}", s),
        ));
      }
    },
  };

  Ok(Some(DeviceIntrRequestV1::DeviceIntrSrq(arg0)))
}
pub fn encode_device_intr(msg: DeviceIntrResponse, _buf: &mut Vec<u8>) -> io::Result<()> {
  match msg {
    DeviceIntrResponse::V1(rsp) => {
      match rsp {
        DeviceIntrResponseV1::DeviceIntrSrq => {}
        _ => {
          return Err(io::Error::new(io::ErrorKind::Other, "unknown procedure"));
        }
      }
      Ok(())
    }
    _ => {
      return Err(io::Error::new(io::ErrorKind::Other, "unknown version"));
    }
  }
}
