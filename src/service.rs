#![allow(deprecated)]
// Most functions and fields are named identically to the VXI11 standard
// In an effort for continuity they share the same names here.
#![allow(non_snake_case)]
// Certain cases are used for documenting possible interactions that could occur
// if this were written in another language
#![allow(unreachable_patterns)]

use crate::proto::*;
use crate::vxi11::*;
use crate::xdr_rpc;
use futures::future::BoxFuture;
use futures::{future, Future};
use std::io;
use tokio_service::Service;

impl<'a> Service for DeviceAsyncService<'a> {
  type Request = xdr_rpc::XdrRequest<DeviceAsyncRequest>;
  type Response = xdr_rpc::XdrResponse<DeviceAsyncResponse>;
  type Error = io::Error;
  type Future = BoxFuture<Self::Response, Self::Error>;
  fn call(&self, req: Self::Request) -> Self::Future {
    let xid = req.xid;
    match req.val {
      DeviceAsyncRequest::V1(data) => {
        let res = match data {
          DeviceAsyncRequestV1::DeviceAbort(arg0) => self
            .device_abort_v1(arg0)
            .map(DeviceAsyncResponseV1::DeviceAbort)
            .boxed(),
          _ => {
            return future::err(io::Error::new(io::ErrorKind::Other, "unknown procedure")).boxed();
          }
        };
        res
          .map(move |r| xdr_rpc::XdrResponse {
            xid,
            val: DeviceAsyncResponse::V1(r),
          })
          .boxed()
      }
      _ => future::err(io::Error::new(io::ErrorKind::Other, "unknown version")).boxed(),
    }
  }
}
impl<'a> Service for DeviceCoreService<'a> {
  type Request = xdr_rpc::XdrRequest<DeviceCoreRequest>;
  type Response = xdr_rpc::XdrResponse<DeviceCoreResponse>;
  type Error = io::Error;
  type Future = BoxFuture<Self::Response, Self::Error>;
  fn call(&self, req: Self::Request) -> Self::Future {
    let xid = req.xid;
    match req.val {
      DeviceCoreRequest::V1(data) => {
        let res = match data {
          DeviceCoreRequestV1::CreateLink(arg0) => self
            .create_link_v1(arg0)
            .map(DeviceCoreResponseV1::CreateLink)
            .boxed(),
          DeviceCoreRequestV1::DeviceWrite(arg0) => self
            .device_write_v1(arg0)
            .map(DeviceCoreResponseV1::DeviceWrite)
            .boxed(),
          DeviceCoreRequestV1::DeviceRead(arg0) => self
            .device_read_v1(arg0)
            .map(DeviceCoreResponseV1::DeviceRead)
            .boxed(),
          DeviceCoreRequestV1::DeviceReadstb(arg0) => self
            .device_readstb_v1(arg0)
            .map(DeviceCoreResponseV1::DeviceReadstb)
            .boxed(),
          DeviceCoreRequestV1::DeviceTrigger(arg0) => self
            .device_trigger_v1(arg0)
            .map(DeviceCoreResponseV1::DeviceTrigger)
            .boxed(),
          DeviceCoreRequestV1::DeviceClear(arg0) => self
            .device_clear_v1(arg0)
            .map(DeviceCoreResponseV1::DeviceClear)
            .boxed(),
          DeviceCoreRequestV1::DeviceRemote(arg0) => self
            .device_remote_v1(arg0)
            .map(DeviceCoreResponseV1::DeviceRemote)
            .boxed(),
          DeviceCoreRequestV1::DeviceLocal(arg0) => self
            .device_local_v1(arg0)
            .map(DeviceCoreResponseV1::DeviceLocal)
            .boxed(),
          DeviceCoreRequestV1::DeviceLock(arg0) => self
            .device_lock_v1(arg0)
            .map(DeviceCoreResponseV1::DeviceLock)
            .boxed(),
          DeviceCoreRequestV1::DeviceUnlock(arg0) => self
            .device_unlock_v1(arg0)
            .map(DeviceCoreResponseV1::DeviceUnlock)
            .boxed(),
          DeviceCoreRequestV1::DeviceEnableSrq(arg0) => self
            .device_enable_srq_v1(arg0)
            .map(DeviceCoreResponseV1::DeviceEnableSrq)
            .boxed(),
          DeviceCoreRequestV1::DeviceDocmd(arg0) => self
            .device_docmd_v1(arg0)
            .map(DeviceCoreResponseV1::DeviceDocmd)
            .boxed(),
          DeviceCoreRequestV1::DestroyLink(arg0) => self
            .destroy_link_v1(arg0)
            .map(DeviceCoreResponseV1::DestroyLink)
            .boxed(),
          DeviceCoreRequestV1::CreateIntrChan(arg0) => self
            .create_intr_chan_v1(arg0)
            .map(DeviceCoreResponseV1::CreateIntrChan)
            .boxed(),
          DeviceCoreRequestV1::DestroyIntrChan => self
            .destroy_intr_chan_v1()
            .map(DeviceCoreResponseV1::DestroyIntrChan)
            .boxed(),
          _ => {
            return future::err(io::Error::new(io::ErrorKind::Other, "unknown procedure")).boxed();
          }
        };
        res
          .map(move |r| xdr_rpc::XdrResponse {
            xid,
            val: DeviceCoreResponse::V1(r),
          })
          .boxed()
      }
      _ => future::err(io::Error::new(io::ErrorKind::Other, "unknown version")).boxed(),
    }
  }
}

impl<'a> Service for DeviceIntrService<'a> {
  type Request = xdr_rpc::XdrRequest<DeviceIntrRequest>;
  type Response = xdr_rpc::XdrResponse<DeviceIntrResponse>;
  type Error = io::Error;
  type Future = BoxFuture<Self::Response, Self::Error>;
  fn call(&self, req: Self::Request) -> Self::Future {
    let xid = req.xid;
    match req.val {
      DeviceIntrRequest::V1(data) => {
        let res = match data {
          DeviceIntrRequestV1::DeviceIntrSrq(arg0) => self
            .device_intr_srq_v1(arg0)
            .map(|_r| DeviceIntrResponseV1::DeviceIntrSrq)
            .boxed(),
          _ => {
            return future::err(io::Error::new(io::ErrorKind::Other, "unknown procedure")).boxed()
          }
        };
        res
          .map(move |r| xdr_rpc::XdrResponse {
            xid,
            val: DeviceIntrResponse::V1(r),
          })
          .boxed()
      }
      _ => future::err(io::Error::new(io::ErrorKind::Other, "unknown version")).boxed(),
    }
  }
}
