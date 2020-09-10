# wall-cli

A CLI program to set/get your wallpaper in MacOS/Windows/Linux.  
Uses the [`wall`](https://github.com/agnipau/wall) library.

## Installation

You can either download a pre-compiled binary (for `linux`, `linux-arm`,
`macos`, `win-msvc`, `win-gnu`, `win32-msvc`) from the
[Releases](https://github.com/agnipau/wall-cli/releases), or use cargo.

### Cargo

This will install you a binary named `wallc`.

```bash
cargo install wall-cli
```

Make sure to add `~/.cargo/bin` to your `$PATH`.

#### License

<sup>
Everything outside of <a href="src/lib.rs">src/lib.rs</a>, <a href="src/client/unsafe_.rs">src/client/unsafe_.rs</a> and <a href="src/client/untyped.rs">src/client/untyped.rs</a> is licensed under either of <a
href="LICENSE-APACHE">Apache License, Version 2.0</a> or <a
href="LICENSE-MIT">MIT license</a> at your option. <a href="src/lib.rs">src/lib.rs</a>, <a href="src/client/unsafe_.rs">src/client/unsafe_.rs</a> and <a href="src/client/untyped.rs">src/client/untyped.rs</a> are
licensed under the <a href="src/client/LICENSE-MIT">MIT license</a>.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>

