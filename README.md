# sysinfo dot h

A tiny <sys/sysinfo.h> wrapper in Rust

## How?

Quick start:

```rust
use sysinfo_dot_h::try_collect;

let info = unsafe { try_collect().unwrap() };
dbg!(info.uptime); // uptime in seconds
```

or

```rust
use sysinfo_dot_h::collect;

let info = unsafe { collect() };
dbg!(info.uptime); // uptime in seconds
```

## Why?

This makes it easier (and quicker) to port the following C code to rust:

```c
#include <sys/sysinfo.h>
int main() {
    struct sysinfo *info;
    sysinfo(info);
}
```

The [sysinfo](https://crates.io/crates/sysinfo) crate was too slow for my use case (200 milliseconds) so I made my own.

Please note that unlike sysinfo, this crate only works on Linux, so if your application is cross platform it may not be favourable to use this crate.

