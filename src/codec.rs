use crate::proto::*;
use crate::service::*;
use crate::xdr_codec::*;
use crate::xdr_codec::{AppCodec, XdrCodec};
use crate::xdr_rpc;
use serde_xdr;
use std::{io, result};
use tokio_core::io::{Codec, EasyBuf, Framed, Io};
// use tokio_io::{Codec, EasyBuf, Framed, IoFuture};
use tokio_proto::pipeline::ServerProto;

pub struct DeviceAsyncCodec;
impl Codec for DeviceAsyncCodec {
  type In = DeviceAsyncRequest;
  type Out = DeviceAsyncResponse;
  fn decode(&mut self, buf: &mut EasyBuf) -> io::Result<Option<Self::In>> {
    unreachable!()
  }
  fn encode(&mut self, msg: Self::Out, buf: &mut Vec<u8>) -> io::Result<()> {
    encode(msg, buf)
  }
}
impl AppCodec for DeviceAsyncCodec {
  fn app_decode(
    &mut self,
    prog: u32,
    version: u32,
    procedure: u32,
    buf: &mut EasyBuf,
  ) -> io::Result<Option<Self::In>> {
    device_async_decode(prog, version, procedure, buf)
  }
}
pub type DeviceAsync = XdrCodec<DeviceAsyncCodec>;

pub struct DeviceAsyncProtocol;
impl<T> ServerProto<T> for DeviceAsyncProtocol
where
  T: Io + 'static,
{
  type Request = xdr_rpc::XdrRequest<DeviceAsync>;
  type Response = xdr_rpc::XdrResponse<DeviceAsync>;
  type Transport = Framed<T, DeviceAsync>;
  type BindTransport = io::Result<Self::Transport>;
  fn bind_transport(&self, io: T) -> Self::BindTransport {
    Ok(io.framed(DeviceAsync::new(DeviceAsyncCodec)))
  }
}

pub struct DeviceCoreCodec;
impl Codec for DeviceCoreCodec {
  type In = DeviceCoreRequest;
  type Out = DeviceCoreResponse;
  fn decode(&mut self, buf: &mut EasyBuf) -> io::Result<Option<Self::In>> {
    unreachable!()
  }
  fn encode(&mut self, msg: Self::Out, buf: &mut Vec<u8>) -> io::Result<()> {
    _encode(msg, buf)
  }
}
impl AppCodec for DeviceCoreCodec {
  fn app_decode(
    &mut self,
    prog: u32,
    version: u32,
    procedure: u32,
    buf: &mut EasyBuf,
  ) -> io::Result<Option<Self::In>> {
    device_core_decode(prog, version, procedure, buf)
  }
}
pub type DeviceCore = XdrCodec<DeviceCoreCodec>;

pub struct DeviceCoreProtocol;
impl<T> ServerProto<T> for DeviceCore
where
  T: Io + 'static,
{
  type Request = xdr_rpc::XdrRequest<DeviceCore>;
  type Response = xdr_rpc::XdrResponse<DeviceCore>;
  type Transport = Framed<T, DeviceCore>;
  type BindTransport = io::Result<Self::Transport>;
  fn bind_transport(&self, io: T) -> Self::BindTransport {
    Ok(io.framed(DeviceCore::new(DeviceCoreCodec)))
  }
}

pub struct DeviceIntrCodec;
impl Codec for DeviceIntrCodec {
  type In = DeviceIntrRequest;
  type Out = DeviceIntrResponse;
  fn decode(&mut self, buf: &mut EasyBuf) -> io::Result<Option<Self::In>> {
    unreachable!()
  }
  fn encode(&mut self, msg: Self::Out, buf: &mut Vec<u8>) -> io::Result<()> {
    __encode(msg, buf)
  }
}
impl AppCodec for DeviceIntrCodec {
  fn app_decode(
    &mut self,
    prog: u32,
    version: u32,
    procedure: u32,
    buf: &mut EasyBuf,
  ) -> io::Result<Option<Self::In>> {
    device_intr_decode(prog, version, procedure, buf)
  }
}
pub type DeviceIntr = XdrCodec<DeviceIntrCodec>;

pub struct DeviceIntrProtocol;
impl<T> ServerProto<T> for DeviceIntr
where
  T: Io + 'static,
{
  type Request = xdr_rpc::XdrRequest<DeviceIntr>;
  type Response = xdr_rpc::XdrResponse<DeviceIntr>;
  type Transport = Framed<T, DeviceIntr>;
  type BindTransport = io::Result<Self::Transport>;
  fn bind_transport(&self, io: T) -> Self::BindTransport {
    Ok(io.framed(DeviceIntr::new(DeviceIntrCodec)))
  }
}
