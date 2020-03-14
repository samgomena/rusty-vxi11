#![allow(deprecated)]
// Most functions and fields are named identically to the VXI11 standard
// In an effort for continuity they share the same names here.
#![allow(non_snake_case)]

use crate::proto::*;
use crate::xdr_codec::{AppCodec, XdrCodec};
use crate::xdr_rpc;
use std::io;
use tokio_core::io::{Codec, EasyBuf, Framed, Io};
use tokio_proto::pipeline::ServerProto;

pub struct DeviceAsyncCodec;
impl Codec for DeviceAsyncCodec {
  type In = DeviceAsyncRequest;
  type Out = DeviceAsyncResponse;
  fn decode(&mut self, _buf: &mut EasyBuf) -> io::Result<Option<Self::In>> {
    unreachable!()
  }
  fn encode(&mut self, msg: Self::Out, buf: &mut Vec<u8>) -> io::Result<()> {
    encode_device_async(msg, buf)
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
  type Request = xdr_rpc::XdrRequest<DeviceAsyncRequest>;
  type Response = xdr_rpc::XdrResponse<DeviceAsyncResponse>;
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
  fn decode(&mut self, _buf: &mut EasyBuf) -> io::Result<Option<Self::In>> {
    unreachable!()
  }
  fn encode(&mut self, msg: Self::Out, buf: &mut Vec<u8>) -> io::Result<()> {
    encode_device_core(msg, buf)
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
impl<T> ServerProto<T> for DeviceCoreProtocol
where
  T: Io + 'static,
{
  type Request = xdr_rpc::XdrRequest<DeviceCoreRequest>;
  type Response = xdr_rpc::XdrResponse<DeviceCoreResponse>;
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
  fn decode(&mut self, _buf: &mut EasyBuf) -> io::Result<Option<Self::In>> {
    unreachable!()
  }
  fn encode(&mut self, msg: Self::Out, buf: &mut Vec<u8>) -> io::Result<()> {
    encode_device_intr(msg, buf)
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
impl<T> ServerProto<T> for DeviceIntrProtocol
where
  T: Io + 'static,
{
  type Request = xdr_rpc::XdrRequest<DeviceIntrRequest>;
  type Response = xdr_rpc::XdrResponse<DeviceIntrResponse>;
  type Transport = Framed<T, DeviceIntr>;
  type BindTransport = io::Result<Self::Transport>;
  fn bind_transport(&self, io: T) -> Self::BindTransport {
    Ok(io.framed(DeviceIntr::new(DeviceIntrCodec)))
  }
}
