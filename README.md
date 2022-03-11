


Base on
https://github.com/rp-rs/rp-hal


```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

rustup target install thumbv6m-none-eabi

sudo apt-get install libudev-dev
cargo install elf2uf2-rs

before plugin the pico, keep the bootsel press

cargo run 
```



```bash

sudo cp tools/99-aardvark-pico-clone.rules  /etc/udev/rules.d/
```



