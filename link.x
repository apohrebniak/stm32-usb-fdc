ENTRY(reset_handler)

MEMORY {
    FLASH (rx): ORIGIN = 0x08000000, LENGTH = 64K
    SRAM (rwx): ORIGIN = 0x20000000, LENGTH = 20K
}

EXTERN(VECTOR_TABLE)

SECTIONS
{
    .vctr : {
        LONG(ORIGIN(SRAM) + LENGTH(SRAM))
        *(.vctr)
    } > FLASH
    .text : {
        *(.text .text.*)
    } > FLASH
    .rodata : {
        *(.rodata .rodata.*)
    } > FLASH

    _lma_data = LOADADDR(.data);

    .data : {
        _sdata = .;
        *(.data .data.*)
        . = ALIGN(4);
        _edata = .;
    } > SRAM AT> FLASH

    .bss : {
        _sbss = .;
        *(.bss .bss.*)
        . = ALIGN(4);
        _ebss = .;
    } > SRAM

   /DISCARD/ :
   {
     *(.ARM.exidx .ARM.exidx.*);
   }
}