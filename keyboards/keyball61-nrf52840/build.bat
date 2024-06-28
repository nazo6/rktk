cargo build --release
cd target\thumbv7em-none-eabihf\release
..\..\..\utils\arm-none-eabi-objcopy.exe -Oihex keyball61-nrf52840 keyball61-nrf52840.hex
python3 ..\..\..\utils\uf2conv.py keyball61-nrf52840.hex -c -b 0x26000 -f 0xADA52840
copy flash.uf2 E:\
