#![allow(deprecated)]
// Most functions and fields are named identically to the VXI11 standard
// In an effort for continuity they share the same names here.
#![allow(non_snake_case)]

use futures::future::BoxFuture;
use futures::{future, Future};
use std::io;
use std::marker::PhantomData;

use crate::proto::*;

pub struct DeviceAsyncService<'a> {
    phantom: PhantomData<&'a Self>,
}
impl<'a> DeviceAsyncService<'a> {
    pub fn device_abort_v1(&self, _arg0: u32) -> BoxFuture<DeviceError, io::Error> {
        future::err(io::Error::new(io::ErrorKind::Other, "implement me!")).boxed()
    }
}

pub struct DeviceCoreService<'a> {
    phantom: PhantomData<&'a Self>,
}
impl<'a> DeviceCoreService<'a> {
    pub fn create_link_v1(&self, _arg0: CreateLinkparms) -> BoxFuture<CreateLinkresp, io::Error> {
        future::err(io::Error::new(io::ErrorKind::Other, "implement me!")).boxed()
    }

    pub fn device_write_v1(
        &self,
        _arg0: DeviceWriteparms,
    ) -> BoxFuture<DeviceWriteresp, io::Error> {
        future::err(io::Error::new(io::ErrorKind::Other, "implement me!")).boxed()
    }

    pub fn device_read_v1(&self, _arg0: DeviceReadparms) -> BoxFuture<DeviceReadresp, io::Error> {
        future::err(io::Error::new(io::ErrorKind::Other, "implement me!")).boxed()
    }

    pub fn device_readstb_v1(
        &self,
        _arg0: DeviceGenericparms,
    ) -> BoxFuture<DeviceReadstbresp, io::Error> {
        future::err(io::Error::new(io::ErrorKind::Other, "implement me!")).boxed()
    }

    pub fn device_trigger_v1(
        &self,
        _arg0: DeviceGenericparms,
    ) -> BoxFuture<DeviceError, io::Error> {
        future::err(io::Error::new(io::ErrorKind::Other, "implement me!")).boxed()
    }

    pub fn device_clear_v1(&self, _arg0: DeviceGenericparms) -> BoxFuture<DeviceError, io::Error> {
        future::err(io::Error::new(io::ErrorKind::Other, "implement me!")).boxed()
    }

    pub fn device_remote_v1(&self, _arg0: DeviceGenericparms) -> BoxFuture<DeviceError, io::Error> {
        future::err(io::Error::new(io::ErrorKind::Other, "implement me!")).boxed()
    }

    pub fn device_local_v1(&self, _arg0: DeviceGenericparms) -> BoxFuture<DeviceError, io::Error> {
        future::err(io::Error::new(io::ErrorKind::Other, "implement me!")).boxed()
    }

    pub fn device_lock_v1(&self, _arg0: DeviceLockparms) -> BoxFuture<DeviceError, io::Error> {
        future::err(io::Error::new(io::ErrorKind::Other, "implement me!")).boxed()
    }

    pub fn device_unlock_v1(&self, _arg0: u32) -> BoxFuture<DeviceError, io::Error> {
        future::err(io::Error::new(io::ErrorKind::Other, "implement me!")).boxed()
    }

    pub fn device_enable_srq_v1(
        &self,
        _arg0: DeviceEnablesrqparms,
    ) -> BoxFuture<DeviceError, io::Error> {
        future::err(io::Error::new(io::ErrorKind::Other, "implement me!")).boxed()
    }

    pub fn device_docmd_v1(
        &self,
        _arg0: DeviceDocmdparms,
    ) -> BoxFuture<DeviceDocmdresp, io::Error> {
        future::err(io::Error::new(io::ErrorKind::Other, "implement me!")).boxed()
    }

    pub fn destroy_link_v1(&self, _arg0: u32) -> BoxFuture<DeviceError, io::Error> {
        future::err(io::Error::new(io::ErrorKind::Other, "implement me!")).boxed()
    }

    pub fn create_intr_chan_v1(
        &self,
        _arg0: DeviceRemotefunc,
    ) -> BoxFuture<DeviceError, io::Error> {
        future::err(io::Error::new(io::ErrorKind::Other, "implement me!")).boxed()
    }

    pub fn destroy_intr_chan_v1(&self) -> BoxFuture<DeviceError, io::Error> {
        future::err(io::Error::new(io::ErrorKind::Other, "implement me!")).boxed()
    }
}

pub struct DeviceIntrService<'a> {
    phantom: PhantomData<&'a Self>,
}
impl<'a> DeviceIntrService<'a> {
    pub fn device_intr_srq_v1(
        &self,
        _arg0: DeviceSrqparms,
    ) -> BoxFuture<std::io::Error, io::Error> {
        future::err(io::Error::new(io::ErrorKind::Other, "implement me!")).boxed()
    }
}
