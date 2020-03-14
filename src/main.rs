// extern crate serde_xdr;

#[macro_use]
extern crate serde_derive;
extern crate tokio_core;
extern crate tokio_proto;
extern crate tokio_service;

use tokio_proto::TcpServer;

mod codec;
mod proto;
mod service;
mod vxi11;
mod xdr_codec;
mod xdr_rpc;

fn main() {
    println!("Hello, World")
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
