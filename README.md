## Setup
___

This OS demo expects a bare metal environment. When compiling we need to point
the *target triple* with **no** underlying operating system:
```
rustup target add thumbv7em-none-eabihf
cargo build --target thumbv7em-none-eabihf
```


