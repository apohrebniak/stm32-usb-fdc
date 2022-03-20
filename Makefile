build:
	cargo build
	arm-none-eabi-objcopy -O binary target/thumbv7em-none-eabihf/debug/firmware target/firmware.bin

release:
	cargo build --release
	arm-none-eabi-objcopy -O binary target/thumbv7em-none-eabihf/release/firmware target/firmware.bin

dfu: build
	dfu-util -D target/firmware.bin -d "0483:df11" -a 0 -s 0x08000000:leave -R


flash: build
	st-flash --connect-under-reset write target/firmware.bin 0x08000000

list:
	dfu-util -l
