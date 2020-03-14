# rusty-vxi11

A standalone VXI11 server written in rust.

## Overview

This project was originally designed to integrate directly with [rusty-visa](https://https://gitlab.com/andeh575/rusty-visa) (note: the repo is private and I don't have permission to change that). The project was migrated to its own crate for various reasons which are discussed in [problems](#problems).

At the moment it relies fairly heavily on projects from others:

- [rust-xdr](https://github.com/benbrittain/rust-xdr/tree/633c17c9c3a59c541e68c35a24e343c98dcdeafd) - Ben Brittain
- [serde-xdr](https://github.com/benbrittain/rust-xdr/tree/633c17c9c3a59c541e68c35a24e343c98dcdeafd/src/serde_xdr) - Ben Brittain (though this uses a fork of the original)

**An important note**: This is still a work in progress and at the moment, does not actually stand up a VXI11 server to interact with. It does however implement an XDR-RPC server to support a VXI11 service.

## Problems

This project was unfortunately, improperly scoped. Before work started I felt very confident in my ability to complete the project within the term. In retrospect, that was very apparently not the case. A large part of starting was understanding how to properly integrate the module with the rusty-visa library. After spending a majority of the term attempting to integrate the VXI11 module into the rusty-visa library and failing, I made the decision to transition it a standalone crate.

This brought about other challenges. I was not able to use any of the utility functions available within the rusty-visa library. To try and get complete the project by the deadline, I ended up using a handful of third-party libraries and tools (see [references](#references)). Most notably, `rust-xdr` and `serde-xdr`. `rust-xdr` is a code generation tool that allows generating rust code from XDR definition files. `serde-xdr` is a library for serializing and deserializing XDR data from and into rust, respectively. Both of these have not been updated in ~3 years and it showed. The `rust-xdr` tool generated an acceptable architecture but required that a majority of "bones" be rewritten. Annoyingly, it heavily leverages several deprecated `tokio` crates such that upgrading to the latest versions would be approximately as much work as rewriting from scratch. It was as similar story with `serde-xdr` as well, the version bundled with the tool was severely antiquantid in comparison to the latest version of rust. I decided to fork and rewrite a majority of the library and was successful in doing so (it can be found [here](https://github.com/samgomena/serde-xdr)).

As an aside, based off of the project proposal given earlier this term, this project is a failure. However, the work is still important to me, and I plan to continue working on it.

## Building

```rust
cargo build
```

Note: The binary produced currently doesn't do anything.

## References

- [rust-xdr](https://github.com/benbrittain/rust-xdr/tree/633c17c9c3a59c541e68c35a24e343c98dcdeafd)
- [serde-xdr](https://github.com/benbrittain/rust-xdr/tree/633c17c9c3a59c541e68c35a24e343c98dcdeafd/src/serde_xdr)
- [VXI11 RPC Definition](https://github.com/applied-optics/vxi11/blob/master/library/vxi11.x)
