use crate::peripheral::Register;

// beginning of RCC memory
const RCC_ORIGIN: usize = 0x40021000;

// RCC registers
const RCC_CR: usize = RCC_ORIGIN;
const RCC_CFGR: usize = RCC_ORIGIN + 0x04;
const RCC_CIR: usize = RCC_ORIGIN + 0x08;
const RCC_APB2RSTR: usize = RCC_ORIGIN + 0x0C;
const RCC_APB1RSTR: usize = RCC_ORIGIN + 0x10;
const RCC_AHBENR: usize = RCC_ORIGIN + 0x14;
const RCC_APB2ENR: usize = RCC_ORIGIN + 0x18;
const RCC_APB1ENR: usize = RCC_ORIGIN + 0x1C;
const RCC_BDCR: usize = RCC_ORIGIN + 0x20;
const RCC_CSR: usize = RCC_ORIGIN + 0x24;

//
const IOPCEN: usize = 1 << 4;
const IOPBEN: usize = 1 << 3;
const IOPAEN: usize = 1 << 2;

/// Reset and clock control
pub(crate) struct Rcc {
    clk_ctrl: Register,
    clk_cfg: Register,
    clk_int: Register,
    apb1_rst: Register,
    apb2_rst: Register,
    ahb_clk_en: Register,
    apb1_clk_en: Register,
    apb2_clk_en: Register,
    bkp_ctrl: Register,
    ctrl_st: Register,
}

impl Rcc {
    pub(in crate::peripheral) const fn new() -> Rcc {
        Rcc {
            clk_ctrl: Register(RCC_CR as *const usize),
            clk_cfg: Register(RCC_CFGR as *const usize),
            clk_int: Register(RCC_CIR as *const usize),
            apb1_rst: Register(RCC_APB1RSTR as *const usize),
            apb2_rst: Register(RCC_APB2RSTR as *const usize),
            ahb_clk_en: Register(RCC_AHBENR as *const usize),
            apb1_clk_en: Register(RCC_APB1ENR as *const usize),
            apb2_clk_en: Register(RCC_APB2ENR as *const usize),
            bkp_ctrl: Register(RCC_BDCR as *const usize),
            ctrl_st: Register(RCC_CSR as *const usize),
        }
    }

    pub(crate) fn io_c_clk_enable(mut self) -> Rcc {
        self.apb2_clk_en.or(IOPCEN);
        self
    }

    pub(crate) fn io_b_clk_enable(mut self) -> Rcc {
        self.apb2_clk_en.or(IOPBEN);
        self
    }

    pub(crate) fn io_a_clk_enable(mut self) -> Rcc {
        self.apb2_clk_en.or(IOPAEN);
        self
    }
}
