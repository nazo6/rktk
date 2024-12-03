# corne v3 example

RKTK firmware example for corne v3 with ProMicro RP2040. To make example simple,
only keyboard feature is enabled (OLED, RGB is disabled).

Actually, I don't have a corne keyboard, so please report any issues you
encounter.

## Building

See [README.md](../../README.md) for prerequisites.

1. Clone this repository and move here
   ```bash
   git clone https://github.com/nazo6/rktk
   cd rktk/examples/corne
   ```

2. Install `rktk-cli`
   ```bash
   cargo install rktk-cli
   ```

3. Build the firmware
   ```bash
   # Use feature `left` or `right` to build firmware for left or right half.
   rktk-cli build -- --features left
   ```

4. Flash the firmware

   After building the firmware, you will find the uf2 file in the `target`. Boot
   ProMicro into the bootloader mode and drag and drop the uf2 file to the
   device.
