## Setup
___

This OS demo expects a bare metal environment. When compiling we need to point
the *target triple* with **no** underlying operating system:
```
rustup target add thumbv7em-none-eabihf
cargo build --target thumbv7em-none-eabihf
```

`x86_64-blog_os.json` provides the target specification.
**Note:** Both `llvm-target` and `os` are set to `None` as we are running on
bare metal.

Finally use Rust's cross platform linker over the default one (which might not
support Linux targets) for linking our kernel.

Our kernel can be compiled with our custom target specification using:
```
cargo build --target x86_64-blog_os.json
```


