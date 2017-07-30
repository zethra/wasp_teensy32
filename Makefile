build:
	@xargo build --release

hex: build
	arm-none-eabi-objcopy -O ihex -R .eeprom target/thumbv7em-none-eabi/release/wasp target/wasp.hex

flash: hex
	teensy-loader-cli -w -s -mmcu=mk20dx128 target/wasp.hex

clean:
	xargo clean
	git clean -Xf

