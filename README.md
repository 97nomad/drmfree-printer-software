# drmfree-printer-software
Software that can convert some g-code to special commands and send them to [drmfree-printer](https://github.com/97nomad/drmfree-printer-firmware) controller.

Support `G00`, `G01`, `G02`, `G03`, `G20` and `G21` g-codes.

## Usage
```
cargo run --release -- [options] FILE

Options:
  -p PORT   Port to output ("COM0" in Windows or "/dev/ttyACM0" in Linux)
  -t        Test mode
```
