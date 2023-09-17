# xan-log

# Usage

1. the logger will initialized with LOG_LEVEL env variable

> list: off, trace, debug, info, warn, error
> both upper, lower case are parsable

2. add log lib and extern it in your main.rs or lib.rs

```rust
#[macro_use]
extern crate log;
```

3. init logger and use the macros

```rust
use xan_log::init_logger;

#[macro_use]
extern crate log;

fn main() {
    init_logger();
    error!("idk, some error: {}", "some error");
}
```
