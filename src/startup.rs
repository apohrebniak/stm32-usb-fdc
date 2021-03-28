/// This is an analog on C's crt0
use crate::main;
use core::ptr;

#[allow(dead_code)]
union Vector {
    reserved: u32,
    handler: extern "C" fn(),
}

#[link_section = ".vctr"]
#[no_mangle]
#[allow(dead_code)]
static VECTOR_TABLE: [Vector; 85] = [
    Vector {
        handler: reset_handler,
    },
    Vector {
        handler: noop_handler,
    },
    Vector {
        handler: noop_handler,
    },
    Vector {
        handler: noop_handler,
    },
    Vector {
        handler: noop_handler,
    },
    Vector {
        handler: noop_handler,
    },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector {
        handler: noop_handler,
    },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector {
        handler: noop_handler,
    },
    Vector {
        handler: noop_handler,
    },
    Vector {
        handler: noop_handler,
    },
    Vector {
        handler: noop_handler,
    },
    Vector {
        handler: noop_handler,
    },
    Vector {
        handler: noop_handler,
    },
    Vector {
        handler: noop_handler,
    },
    Vector {
        handler: noop_handler,
    },
    Vector {
        handler: noop_handler,
    },
    Vector {
        handler: noop_handler,
    },
    Vector {
        handler: noop_handler,
    },
    Vector {
        handler: noop_handler,
    },
    Vector {
        handler: noop_handler,
    },
    Vector {
        handler: noop_handler,
    },
    Vector {
        handler: noop_handler,
    },
    Vector {
        handler: noop_handler,
    },
    Vector {
        handler: noop_handler,
    },
    Vector {
        handler: noop_handler,
    },
    Vector {
        handler: noop_handler,
    },
    Vector {
        handler: noop_handler,
    },
    Vector {
        handler: noop_handler,
    },
    Vector {
        handler: noop_handler,
    },
    Vector {
        handler: noop_handler,
    },
    Vector {
        handler: noop_handler,
    },
    Vector {
        handler: noop_handler,
    },
    Vector {
        handler: noop_handler,
    },
    Vector {
        handler: noop_handler,
    },
    Vector {
        handler: noop_handler,
    },
    Vector {
        handler: noop_handler,
    },
    Vector {
        handler: noop_handler,
    },
    Vector {
        handler: noop_handler,
    },
    Vector {
        handler: noop_handler,
    },
    Vector {
        handler: noop_handler,
    },
    Vector {
        handler: noop_handler,
    },
    Vector {
        handler: noop_handler,
    },
    Vector {
        handler: noop_handler,
    },
    Vector {
        handler: noop_handler,
    },
    Vector {
        handler: noop_handler,
    },
    Vector {
        handler: noop_handler,
    },
    Vector {
        handler: noop_handler,
    },
    Vector {
        handler: noop_handler,
    },
    Vector {
        handler: noop_handler,
    },
    Vector {
        handler: noop_handler,
    },
    Vector {
        handler: noop_handler,
    },
    Vector {
        handler: noop_handler,
    },
    Vector {
        handler: noop_handler,
    },
    Vector {
        handler: noop_handler,
    },
    Vector {
        handler: noop_handler,
    },
    Vector {
        handler: noop_handler,
    },
    Vector {
        handler: noop_handler,
    },
    Vector {
        handler: noop_handler,
    },
    Vector {
        handler: noop_handler,
    },
    Vector {
        handler: noop_handler,
    },
    Vector {
        handler: noop_handler,
    },
    Vector {
        handler: noop_handler,
    },
    Vector {
        handler: noop_handler,
    },
    Vector {
        handler: noop_handler,
    },
    Vector {
        handler: noop_handler,
    },
    Vector {
        handler: noop_handler,
    },
    Vector {
        handler: noop_handler,
    },
    Vector {
        handler: noop_handler,
    },
    Vector {
        handler: noop_handler,
    },
    Vector {
        handler: noop_handler,
    },
    Vector {
        handler: noop_handler,
    },
    Vector {
        handler: noop_handler,
    },
    Vector {
        handler: noop_handler,
    },
    Vector {
        handler: noop_handler,
    },
    Vector {
        handler: noop_handler,
    },
    Vector {
        handler: noop_handler,
    },
    Vector {
        handler: noop_handler,
    },
    Vector {
        handler: noop_handler,
    },
    Vector {
        handler: noop_handler,
    },
];

#[no_mangle]
extern "C" fn reset_handler() {
    // extern symbols from the linker script
    extern "C" {
        static mut _sbss: u8;
        static mut _ebss: u8;

        static mut _sdata: u8;
        static mut _edata: u8;
        static _lma_data: u8;
    }

    // initialize .bss and .data
    unsafe {
        let count = &_ebss as *const u8 as usize - &_sbss as *const u8 as usize;
        ptr::write_bytes(&mut _sbss as *mut u8, 0, count);

        let count = &_edata as *const u8 as usize - &_sdata as *const u8 as usize;
        ptr::copy_nonoverlapping(&_lma_data as *const u8, &mut _sdata as *mut u8, count);
    }

    main()
}

#[no_mangle]
extern "C" fn noop_handler() {
    loop {}
}
