# mp4ra-rust

Rust types and constants representing code points managed by the [MPEG-4
Registration Authority](https://mp4ra.org/), together with the code used to
automatically generate these constants from data published by MP4RA.

 - [mp4ra-rust](mp4ra-rust) - Rust crate containing the constant definitions
 - [mp4ra_crategen](mp4ra_crategen) - code generation tool - not a published
   Rust crate
 - [mp4ra.github.io](mp4ra.github.io) - git submodule containing the CSV data
   that drives the code generation process


A code generation step is performed by the `mp4ra_crategen` tool whose updated output
needs to be committed to this repo.  This is hopefully faster than having the
user of the crate have to do this code generation step themselves (e.g. by
using a `build.rs` file).

When MP4RA update their data, then:
 - the `mp4ra.github.io` git submodule needs to be updated
 - the `mp4ra_crategen` too needs to be run to regenerate constants in the
   `mp4ra-rust` crate
 - the updated code in `mp4ra-rust` needs to be comitted to this repo
