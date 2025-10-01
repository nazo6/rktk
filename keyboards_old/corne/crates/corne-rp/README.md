# corne v3 example

RKTK firmware example for corne v3 with ProMicro RP2040. To make example simple,
only keyboard feature is enabled (OLED, RGB is disabled).

Actually, I don't have a corne keyboard, so please report any issues you
encounter.

## Building

1. Clone this repository and move here
   ```bash
   git clone https://github.com/nazo6/rktk
   cd rktk/examples/corne
   ```

2. Install `uf2deploy`
   ```bash
   cargo install uf2deploy
   ```

3. Build and deploy the firmware
   ```bash
   # By running cargo run, uf2deploy will be executed automatically. uf2deploy converts elf to uf2 and copies uf2 to attached device.
   # Use feature `left` or `right` to build firmware for left or right half.
   cargo run --release --features left
   ```
