st-flash erase &&
echo "ERASE OK" &&
arm-none-eabi-objcopy -O binary $1 target/binary.bin &&
st-flash write target/binary.bin 0x8000000 &&
echo "FLASH OK"