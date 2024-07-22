#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: Ctrl,
    ctrl1: Ctrl1,
    addr: Addr,
    instr: Instr,
    txfifo: Txfifo,
    rxfifo: Rxfifo,
    txentries: Txentries,
    rxentries: Rxentries,
    threshold: Threshold,
    _reserved9: [u8; 0x0c],
    mspicfg: Mspicfg,
    _reserved10: [u8; 0x10],
    padouten: Padouten,
    padoveren: Padoveren,
    padover: Padover,
    _reserved13: [u8; 0x30],
    dev0axi: Dev0axi,
    dev0cfg: Dev0cfg,
    dev0ddr: Dev0ddr,
    dev0cfg1: Dev0cfg1,
    dev0xip: Dev0xip,
    dev0instr: Dev0instr,
    dev0boundary: Dev0boundary,
    dev0scrambling: Dev0scrambling,
    dev0xipmisc: Dev0xipmisc,
    _reserved22: [u8; 0x5c],
    dmacfg: Dmacfg,
    dmastat: Dmastat,
    dmatargaddr: Dmatargaddr,
    dmadevaddr: Dmadevaddr,
    dmatotcount: Dmatotcount,
    dmabcount: Dmabcount,
    dmathresh: Dmathresh,
    _reserved29: [u8; 0xe4],
    inten: Inten,
    intstat: Intstat,
    intclr: Intclr,
    intset: Intset,
    _reserved33: [u8; 0x90],
    cqcfg: Cqcfg,
    _reserved34: [u8; 0x04],
    cqaddr: Cqaddr,
    cqstat: Cqstat,
    cqflags: Cqflags,
    cqsetclear: Cqsetclear,
    cqpause: Cqpause,
    _reserved39: [u8; 0x04],
    cqcuridx: Cqcuridx,
    cqendidx: Cqendidx,
    _reserved41: [u8; 0x48],
    statxipdma: Statxipdma,
}
impl RegisterBlock {
    #[doc = "0x00 - This register is used to enable individual PIO based transactions to a device on the bus. The CFG register must be programmed properly for the transfer, and the ADDR and INSTR registers should be programmed if the SENDI and SENDA fields are enabled."]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x04 - These registers are used to enable individual PIO based transactions to a device on the bus. The CFG register must be programmed properly for the transfer, and the ADDR and INSTR registers should be programmed if the SENDI and SENDA fields are enabled."]
    #[inline(always)]
    pub const fn ctrl1(&self) -> &Ctrl1 {
        &self.ctrl1
    }
    #[doc = "0x08 - Optional Address field to send for PIO transfers"]
    #[inline(always)]
    pub const fn addr(&self) -> &Addr {
        &self.addr
    }
    #[doc = "0x0c - Optional Instruction field to send for PIO transfers"]
    #[inline(always)]
    pub const fn instr(&self) -> &Instr {
        &self.instr
    }
    #[doc = "0x10 - TX Data FIFO"]
    #[inline(always)]
    pub const fn txfifo(&self) -> &Txfifo {
        &self.txfifo
    }
    #[doc = "0x14 - RX Data FIFO"]
    #[inline(always)]
    pub const fn rxfifo(&self) -> &Rxfifo {
        &self.rxfifo
    }
    #[doc = "0x18 - Number of words in TX FIFO"]
    #[inline(always)]
    pub const fn txentries(&self) -> &Txentries {
        &self.txentries
    }
    #[doc = "0x1c - Number of words in RX FIFO"]
    #[inline(always)]
    pub const fn rxentries(&self) -> &Rxentries {
        &self.rxentries
    }
    #[doc = "0x20 - Threshold levels that trigger RXFull and TXEmpty interrupts"]
    #[inline(always)]
    pub const fn threshold(&self) -> &Threshold {
        &self.threshold
    }
    #[doc = "0x30 - Timing configuration bits for the MSPI module. PRSTN, IPRSTN, and FIFORESET can be used to reset portions of the MSPI interface in order to clear error conditions. The remaining bits control clock frequency and TX/RX capture timings."]
    #[inline(always)]
    pub const fn mspicfg(&self) -> &Mspicfg {
        &self.mspicfg
    }
    #[doc = "0x44 - Enable bits for the MSPI output pads. Each active MSPI line should be set to 1 in the OUTEN field below."]
    #[inline(always)]
    pub const fn padouten(&self) -> &Padouten {
        &self.padouten
    }
    #[doc = "0x48 - Enables PIO-like pad override control"]
    #[inline(always)]
    pub const fn padoveren(&self) -> &Padoveren {
        &self.padoveren
    }
    #[doc = "0x4c - Override data value"]
    #[inline(always)]
    pub const fn padover(&self) -> &Padover {
        &self.padover
    }
    #[doc = "0x80 - Specifies the base address and aperture range of the device as mapped onto the AXI bus"]
    #[inline(always)]
    pub const fn dev0axi(&self) -> &Dev0axi {
        &self.dev0axi
    }
    #[doc = "0x84 - Command formatting for PIO based transactions (initiated by writes to CTRL register)"]
    #[inline(always)]
    pub const fn dev0cfg(&self) -> &Dev0cfg {
        &self.dev0cfg
    }
    #[doc = "0x88 - Timing configuration bits for DDR operation of the MSPI module."]
    #[inline(always)]
    pub const fn dev0ddr(&self) -> &Dev0ddr {
        &self.dev0ddr
    }
    #[doc = "0x8c - Timing and mode configuration bits for the MSPI module."]
    #[inline(always)]
    pub const fn dev0cfg1(&self) -> &Dev0cfg1 {
        &self.dev0cfg1
    }
    #[doc = "0x90 - When any SPI flash is configured, this register must be properly programmed before XIP or AUTO DMA operations commence."]
    #[inline(always)]
    pub const fn dev0xip(&self) -> &Dev0xip {
        &self.dev0xip
    }
    #[doc = "0x94 - When any SPI flash is configured, this register must be properly programmed before XIP or AUTO DMA operations commence."]
    #[inline(always)]
    pub const fn dev0instr(&self) -> &Dev0instr {
        &self.dev0instr
    }
    #[doc = "0x98 - Allows large transfers to be broken up into smaller ones in hardware to accommodate needs of external devices and allow XIP/XIPMM. Only applicable for memory-mapped devices (PSRAM, Flash, etc) where address can be retransmitted without side effects."]
    #[inline(always)]
    pub const fn dev0boundary(&self) -> &Dev0boundary {
        &self.dev0boundary
    }
    #[doc = "0x9c - Enables data scrambling for the specified range external flash addresses. Scrambling does not impact flash access performance."]
    #[inline(always)]
    pub const fn dev0scrambling(&self) -> &Dev0scrambling {
        &self.dev0scrambling
    }
    #[doc = "0xa0 - Miscellaneous XIP control registers for AXI logic"]
    #[inline(always)]
    pub const fn dev0xipmisc(&self) -> &Dev0xipmisc {
        &self.dev0xipmisc
    }
    #[doc = "0x100 - DMA Configuration"]
    #[inline(always)]
    pub const fn dmacfg(&self) -> &Dmacfg {
        &self.dmacfg
    }
    #[doc = "0x104 - DMA Status"]
    #[inline(always)]
    pub const fn dmastat(&self) -> &Dmastat {
        &self.dmastat
    }
    #[doc = "0x108 - DMA Target Address"]
    #[inline(always)]
    pub const fn dmatargaddr(&self) -> &Dmatargaddr {
        &self.dmatargaddr
    }
    #[doc = "0x10c - DMA Device Address"]
    #[inline(always)]
    pub const fn dmadevaddr(&self) -> &Dmadevaddr {
        &self.dmadevaddr
    }
    #[doc = "0x110 - DMA Total Transfer Count"]
    #[inline(always)]
    pub const fn dmatotcount(&self) -> &Dmatotcount {
        &self.dmatotcount
    }
    #[doc = "0x114 - DMA BYTE Transfer Count"]
    #[inline(always)]
    pub const fn dmabcount(&self) -> &Dmabcount {
        &self.dmabcount
    }
    #[doc = "0x118 - Indicates FIFO level at which a DMA should be triggered. For most configurations, a setting of 8 is recommended for both read and write operations."]
    #[inline(always)]
    pub const fn dmathresh(&self) -> &Dmathresh {
        &self.dmathresh
    }
    #[doc = "0x200 - Set bits in this register to allow this module to generate the corresponding interrupt."]
    #[inline(always)]
    pub const fn inten(&self) -> &Inten {
        &self.inten
    }
    #[doc = "0x204 - Read bits from this register to discover the cause of a recent interrupt."]
    #[inline(always)]
    pub const fn intstat(&self) -> &Intstat {
        &self.intstat
    }
    #[doc = "0x208 - Write a 1 to a bit in this register to clear the interrupt status associated with that bit."]
    #[inline(always)]
    pub const fn intclr(&self) -> &Intclr {
        &self.intclr
    }
    #[doc = "0x20c - Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes)."]
    #[inline(always)]
    pub const fn intset(&self) -> &Intset {
        &self.intset
    }
    #[doc = "0x2a0 - This register controls Command Queuing (CQ) operations in a manner similar to the DMACFG register."]
    #[inline(always)]
    pub const fn cqcfg(&self) -> &Cqcfg {
        &self.cqcfg
    }
    #[doc = "0x2a8 - Location of the command queue in SRAM or flash memory. This register will increment as CQ operations commence. Software should only write CQADDR when CQEN is disabled, however the command queue script itself may update CQADDR in order to perform queue management functions (like resetting the pointers)"]
    #[inline(always)]
    pub const fn cqaddr(&self) -> &Cqaddr {
        &self.cqaddr
    }
    #[doc = "0x2ac - Command Queue Status"]
    #[inline(always)]
    pub const fn cqstat(&self) -> &Cqstat {
        &self.cqstat
    }
    #[doc = "0x2b0 - Command Queue Flags"]
    #[inline(always)]
    pub const fn cqflags(&self) -> &Cqflags {
        &self.cqflags
    }
    #[doc = "0x2b4 - Command Queue Flag Set/Clear"]
    #[inline(always)]
    pub const fn cqsetclear(&self) -> &Cqsetclear {
        &self.cqsetclear
    }
    #[doc = "0x2b8 - Command Queue Pause Mask"]
    #[inline(always)]
    pub const fn cqpause(&self) -> &Cqpause {
        &self.cqpause
    }
    #[doc = "0x2c0 - This register can be used in conjunction with the CQENDIDX register to manage the command queue. Typically software will initialize the CQCURIDX and CQENDIDX to the same value, which will cause the CQ to be paused when enabled. Software may then add entries to the command queue (in SRAM) and update CQENDIDX. The command queue operations will then increment CQCURIDX as it processes operations. Once CQCURIDX==CQENDIDX, the command queue hardware will automatically pause since no additional operations have been appended to the queue."]
    #[inline(always)]
    pub const fn cqcuridx(&self) -> &Cqcuridx {
        &self.cqcuridx
    }
    #[doc = "0x2c4 - Command Queue End Index"]
    #[inline(always)]
    pub const fn cqendidx(&self) -> &Cqendidx {
        &self.cqendidx
    }
    #[doc = "0x310 - Debug XIP DMA State"]
    #[inline(always)]
    pub const fn statxipdma(&self) -> &Statxipdma {
        &self.statxipdma
    }
}
#[doc = "CTRL (rw) register accessor: This register is used to enable individual PIO based transactions to a device on the bus. The CFG register must be programmed properly for the transfer, and the ADDR and INSTR registers should be programmed if the SENDI and SENDA fields are enabled.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "This register is used to enable individual PIO based transactions to a device on the bus. The CFG register must be programmed properly for the transfer, and the ADDR and INSTR registers should be programmed if the SENDI and SENDA fields are enabled."]
pub mod ctrl;
#[doc = "CTRL1 (rw) register accessor: These registers are used to enable individual PIO based transactions to a device on the bus. The CFG register must be programmed properly for the transfer, and the ADDR and INSTR registers should be programmed if the SENDI and SENDA fields are enabled.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl1`]
module"]
#[doc(alias = "CTRL1")]
pub type Ctrl1 = crate::Reg<ctrl1::Ctrl1Spec>;
#[doc = "These registers are used to enable individual PIO based transactions to a device on the bus. The CFG register must be programmed properly for the transfer, and the ADDR and INSTR registers should be programmed if the SENDI and SENDA fields are enabled."]
pub mod ctrl1;
#[doc = "ADDR (rw) register accessor: Optional Address field to send for PIO transfers\n\nYou can [`read`](crate::Reg::read) this register and get [`addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr`]
module"]
#[doc(alias = "ADDR")]
pub type Addr = crate::Reg<addr::AddrSpec>;
#[doc = "Optional Address field to send for PIO transfers"]
pub mod addr;
#[doc = "INSTR (rw) register accessor: Optional Instruction field to send for PIO transfers\n\nYou can [`read`](crate::Reg::read) this register and get [`instr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`instr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@instr`]
module"]
#[doc(alias = "INSTR")]
pub type Instr = crate::Reg<instr::InstrSpec>;
#[doc = "Optional Instruction field to send for PIO transfers"]
pub mod instr;
#[doc = "TXFIFO (rw) register accessor: TX Data FIFO\n\nYou can [`read`](crate::Reg::read) this register and get [`txfifo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txfifo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txfifo`]
module"]
#[doc(alias = "TXFIFO")]
pub type Txfifo = crate::Reg<txfifo::TxfifoSpec>;
#[doc = "TX Data FIFO"]
pub mod txfifo;
#[doc = "RXFIFO (rw) register accessor: RX Data FIFO\n\nYou can [`read`](crate::Reg::read) this register and get [`rxfifo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxfifo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxfifo`]
module"]
#[doc(alias = "RXFIFO")]
pub type Rxfifo = crate::Reg<rxfifo::RxfifoSpec>;
#[doc = "RX Data FIFO"]
pub mod rxfifo;
#[doc = "TXENTRIES (rw) register accessor: Number of words in TX FIFO\n\nYou can [`read`](crate::Reg::read) this register and get [`txentries::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txentries::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txentries`]
module"]
#[doc(alias = "TXENTRIES")]
pub type Txentries = crate::Reg<txentries::TxentriesSpec>;
#[doc = "Number of words in TX FIFO"]
pub mod txentries;
#[doc = "RXENTRIES (rw) register accessor: Number of words in RX FIFO\n\nYou can [`read`](crate::Reg::read) this register and get [`rxentries::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxentries::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxentries`]
module"]
#[doc(alias = "RXENTRIES")]
pub type Rxentries = crate::Reg<rxentries::RxentriesSpec>;
#[doc = "Number of words in RX FIFO"]
pub mod rxentries;
#[doc = "THRESHOLD (rw) register accessor: Threshold levels that trigger RXFull and TXEmpty interrupts\n\nYou can [`read`](crate::Reg::read) this register and get [`threshold::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`threshold::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@threshold`]
module"]
#[doc(alias = "THRESHOLD")]
pub type Threshold = crate::Reg<threshold::ThresholdSpec>;
#[doc = "Threshold levels that trigger RXFull and TXEmpty interrupts"]
pub mod threshold;
#[doc = "MSPICFG (rw) register accessor: Timing configuration bits for the MSPI module. PRSTN, IPRSTN, and FIFORESET can be used to reset portions of the MSPI interface in order to clear error conditions. The remaining bits control clock frequency and TX/RX capture timings.\n\nYou can [`read`](crate::Reg::read) this register and get [`mspicfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mspicfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mspicfg`]
module"]
#[doc(alias = "MSPICFG")]
pub type Mspicfg = crate::Reg<mspicfg::MspicfgSpec>;
#[doc = "Timing configuration bits for the MSPI module. PRSTN, IPRSTN, and FIFORESET can be used to reset portions of the MSPI interface in order to clear error conditions. The remaining bits control clock frequency and TX/RX capture timings."]
pub mod mspicfg;
#[doc = "PADOUTEN (rw) register accessor: Enable bits for the MSPI output pads. Each active MSPI line should be set to 1 in the OUTEN field below.\n\nYou can [`read`](crate::Reg::read) this register and get [`padouten::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`padouten::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padouten`]
module"]
#[doc(alias = "PADOUTEN")]
pub type Padouten = crate::Reg<padouten::PadoutenSpec>;
#[doc = "Enable bits for the MSPI output pads. Each active MSPI line should be set to 1 in the OUTEN field below."]
pub mod padouten;
#[doc = "PADOVEREN (rw) register accessor: Enables PIO-like pad override control\n\nYou can [`read`](crate::Reg::read) this register and get [`padoveren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`padoveren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padoveren`]
module"]
#[doc(alias = "PADOVEREN")]
pub type Padoveren = crate::Reg<padoveren::PadoverenSpec>;
#[doc = "Enables PIO-like pad override control"]
pub mod padoveren;
#[doc = "PADOVER (rw) register accessor: Override data value\n\nYou can [`read`](crate::Reg::read) this register and get [`padover::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`padover::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padover`]
module"]
#[doc(alias = "PADOVER")]
pub type Padover = crate::Reg<padover::PadoverSpec>;
#[doc = "Override data value"]
pub mod padover;
#[doc = "DEV0AXI (rw) register accessor: Specifies the base address and aperture range of the device as mapped onto the AXI bus\n\nYou can [`read`](crate::Reg::read) this register and get [`dev0axi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev0axi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev0axi`]
module"]
#[doc(alias = "DEV0AXI")]
pub type Dev0axi = crate::Reg<dev0axi::Dev0axiSpec>;
#[doc = "Specifies the base address and aperture range of the device as mapped onto the AXI bus"]
pub mod dev0axi;
#[doc = "DEV0CFG (rw) register accessor: Command formatting for PIO based transactions (initiated by writes to CTRL register)\n\nYou can [`read`](crate::Reg::read) this register and get [`dev0cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev0cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev0cfg`]
module"]
#[doc(alias = "DEV0CFG")]
pub type Dev0cfg = crate::Reg<dev0cfg::Dev0cfgSpec>;
#[doc = "Command formatting for PIO based transactions (initiated by writes to CTRL register)"]
pub mod dev0cfg;
#[doc = "DEV0DDR (rw) register accessor: Timing configuration bits for DDR operation of the MSPI module.\n\nYou can [`read`](crate::Reg::read) this register and get [`dev0ddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev0ddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev0ddr`]
module"]
#[doc(alias = "DEV0DDR")]
pub type Dev0ddr = crate::Reg<dev0ddr::Dev0ddrSpec>;
#[doc = "Timing configuration bits for DDR operation of the MSPI module."]
pub mod dev0ddr;
#[doc = "DEV0CFG1 (rw) register accessor: Timing and mode configuration bits for the MSPI module.\n\nYou can [`read`](crate::Reg::read) this register and get [`dev0cfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev0cfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev0cfg1`]
module"]
#[doc(alias = "DEV0CFG1")]
pub type Dev0cfg1 = crate::Reg<dev0cfg1::Dev0cfg1Spec>;
#[doc = "Timing and mode configuration bits for the MSPI module."]
pub mod dev0cfg1;
#[doc = "DEV0XIP (rw) register accessor: When any SPI flash is configured, this register must be properly programmed before XIP or AUTO DMA operations commence.\n\nYou can [`read`](crate::Reg::read) this register and get [`dev0xip::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev0xip::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev0xip`]
module"]
#[doc(alias = "DEV0XIP")]
pub type Dev0xip = crate::Reg<dev0xip::Dev0xipSpec>;
#[doc = "When any SPI flash is configured, this register must be properly programmed before XIP or AUTO DMA operations commence."]
pub mod dev0xip;
#[doc = "DEV0INSTR (rw) register accessor: When any SPI flash is configured, this register must be properly programmed before XIP or AUTO DMA operations commence.\n\nYou can [`read`](crate::Reg::read) this register and get [`dev0instr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev0instr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev0instr`]
module"]
#[doc(alias = "DEV0INSTR")]
pub type Dev0instr = crate::Reg<dev0instr::Dev0instrSpec>;
#[doc = "When any SPI flash is configured, this register must be properly programmed before XIP or AUTO DMA operations commence."]
pub mod dev0instr;
#[doc = "DEV0BOUNDARY (rw) register accessor: Allows large transfers to be broken up into smaller ones in hardware to accommodate needs of external devices and allow XIP/XIPMM. Only applicable for memory-mapped devices (PSRAM, Flash, etc) where address can be retransmitted without side effects.\n\nYou can [`read`](crate::Reg::read) this register and get [`dev0boundary::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev0boundary::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev0boundary`]
module"]
#[doc(alias = "DEV0BOUNDARY")]
pub type Dev0boundary = crate::Reg<dev0boundary::Dev0boundarySpec>;
#[doc = "Allows large transfers to be broken up into smaller ones in hardware to accommodate needs of external devices and allow XIP/XIPMM. Only applicable for memory-mapped devices (PSRAM, Flash, etc) where address can be retransmitted without side effects."]
pub mod dev0boundary;
#[doc = "DEV0SCRAMBLING (rw) register accessor: Enables data scrambling for the specified range external flash addresses. Scrambling does not impact flash access performance.\n\nYou can [`read`](crate::Reg::read) this register and get [`dev0scrambling::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev0scrambling::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev0scrambling`]
module"]
#[doc(alias = "DEV0SCRAMBLING")]
pub type Dev0scrambling = crate::Reg<dev0scrambling::Dev0scramblingSpec>;
#[doc = "Enables data scrambling for the specified range external flash addresses. Scrambling does not impact flash access performance."]
pub mod dev0scrambling;
#[doc = "DEV0XIPMISC (rw) register accessor: Miscellaneous XIP control registers for AXI logic\n\nYou can [`read`](crate::Reg::read) this register and get [`dev0xipmisc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev0xipmisc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev0xipmisc`]
module"]
#[doc(alias = "DEV0XIPMISC")]
pub type Dev0xipmisc = crate::Reg<dev0xipmisc::Dev0xipmiscSpec>;
#[doc = "Miscellaneous XIP control registers for AXI logic"]
pub mod dev0xipmisc;
#[doc = "DMACFG (rw) register accessor: DMA Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`dmacfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmacfg`]
module"]
#[doc(alias = "DMACFG")]
pub type Dmacfg = crate::Reg<dmacfg::DmacfgSpec>;
#[doc = "DMA Configuration"]
pub mod dmacfg;
#[doc = "DMASTAT (rw) register accessor: DMA Status\n\nYou can [`read`](crate::Reg::read) this register and get [`dmastat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmastat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmastat`]
module"]
#[doc(alias = "DMASTAT")]
pub type Dmastat = crate::Reg<dmastat::DmastatSpec>;
#[doc = "DMA Status"]
pub mod dmastat;
#[doc = "DMATARGADDR (rw) register accessor: DMA Target Address\n\nYou can [`read`](crate::Reg::read) this register and get [`dmatargaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmatargaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmatargaddr`]
module"]
#[doc(alias = "DMATARGADDR")]
pub type Dmatargaddr = crate::Reg<dmatargaddr::DmatargaddrSpec>;
#[doc = "DMA Target Address"]
pub mod dmatargaddr;
#[doc = "DMADEVADDR (rw) register accessor: DMA Device Address\n\nYou can [`read`](crate::Reg::read) this register and get [`dmadevaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmadevaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmadevaddr`]
module"]
#[doc(alias = "DMADEVADDR")]
pub type Dmadevaddr = crate::Reg<dmadevaddr::DmadevaddrSpec>;
#[doc = "DMA Device Address"]
pub mod dmadevaddr;
#[doc = "DMATOTCOUNT (rw) register accessor: DMA Total Transfer Count\n\nYou can [`read`](crate::Reg::read) this register and get [`dmatotcount::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmatotcount::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmatotcount`]
module"]
#[doc(alias = "DMATOTCOUNT")]
pub type Dmatotcount = crate::Reg<dmatotcount::DmatotcountSpec>;
#[doc = "DMA Total Transfer Count"]
pub mod dmatotcount;
#[doc = "DMABCOUNT (rw) register accessor: DMA BYTE Transfer Count\n\nYou can [`read`](crate::Reg::read) this register and get [`dmabcount::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmabcount::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmabcount`]
module"]
#[doc(alias = "DMABCOUNT")]
pub type Dmabcount = crate::Reg<dmabcount::DmabcountSpec>;
#[doc = "DMA BYTE Transfer Count"]
pub mod dmabcount;
#[doc = "DMATHRESH (rw) register accessor: Indicates FIFO level at which a DMA should be triggered. For most configurations, a setting of 8 is recommended for both read and write operations.\n\nYou can [`read`](crate::Reg::read) this register and get [`dmathresh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmathresh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmathresh`]
module"]
#[doc(alias = "DMATHRESH")]
pub type Dmathresh = crate::Reg<dmathresh::DmathreshSpec>;
#[doc = "Indicates FIFO level at which a DMA should be triggered. For most configurations, a setting of 8 is recommended for both read and write operations."]
pub mod dmathresh;
#[doc = "INTEN (rw) register accessor: Set bits in this register to allow this module to generate the corresponding interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`inten::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten`]
module"]
#[doc(alias = "INTEN")]
pub type Inten = crate::Reg<inten::IntenSpec>;
#[doc = "Set bits in this register to allow this module to generate the corresponding interrupt."]
pub mod inten;
#[doc = "INTSTAT (rw) register accessor: Read bits from this register to discover the cause of a recent interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`intstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intstat`]
module"]
#[doc(alias = "INTSTAT")]
pub type Intstat = crate::Reg<intstat::IntstatSpec>;
#[doc = "Read bits from this register to discover the cause of a recent interrupt."]
pub mod intstat;
#[doc = "INTCLR (rw) register accessor: Write a 1 to a bit in this register to clear the interrupt status associated with that bit.\n\nYou can [`read`](crate::Reg::read) this register and get [`intclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intclr`]
module"]
#[doc(alias = "INTCLR")]
pub type Intclr = crate::Reg<intclr::IntclrSpec>;
#[doc = "Write a 1 to a bit in this register to clear the interrupt status associated with that bit."]
pub mod intclr;
#[doc = "INTSET (rw) register accessor: Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes).\n\nYou can [`read`](crate::Reg::read) this register and get [`intset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intset`]
module"]
#[doc(alias = "INTSET")]
pub type Intset = crate::Reg<intset::IntsetSpec>;
#[doc = "Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes)."]
pub mod intset;
#[doc = "CQCFG (rw) register accessor: This register controls Command Queuing (CQ) operations in a manner similar to the DMACFG register.\n\nYou can [`read`](crate::Reg::read) this register and get [`cqcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cqcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cqcfg`]
module"]
#[doc(alias = "CQCFG")]
pub type Cqcfg = crate::Reg<cqcfg::CqcfgSpec>;
#[doc = "This register controls Command Queuing (CQ) operations in a manner similar to the DMACFG register."]
pub mod cqcfg;
#[doc = "CQADDR (rw) register accessor: Location of the command queue in SRAM or flash memory. This register will increment as CQ operations commence. Software should only write CQADDR when CQEN is disabled, however the command queue script itself may update CQADDR in order to perform queue management functions (like resetting the pointers)\n\nYou can [`read`](crate::Reg::read) this register and get [`cqaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cqaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cqaddr`]
module"]
#[doc(alias = "CQADDR")]
pub type Cqaddr = crate::Reg<cqaddr::CqaddrSpec>;
#[doc = "Location of the command queue in SRAM or flash memory. This register will increment as CQ operations commence. Software should only write CQADDR when CQEN is disabled, however the command queue script itself may update CQADDR in order to perform queue management functions (like resetting the pointers)"]
pub mod cqaddr;
#[doc = "CQSTAT (rw) register accessor: Command Queue Status\n\nYou can [`read`](crate::Reg::read) this register and get [`cqstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cqstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cqstat`]
module"]
#[doc(alias = "CQSTAT")]
pub type Cqstat = crate::Reg<cqstat::CqstatSpec>;
#[doc = "Command Queue Status"]
pub mod cqstat;
#[doc = "CQFLAGS (rw) register accessor: Command Queue Flags\n\nYou can [`read`](crate::Reg::read) this register and get [`cqflags::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cqflags::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cqflags`]
module"]
#[doc(alias = "CQFLAGS")]
pub type Cqflags = crate::Reg<cqflags::CqflagsSpec>;
#[doc = "Command Queue Flags"]
pub mod cqflags;
#[doc = "CQSETCLEAR (rw) register accessor: Command Queue Flag Set/Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`cqsetclear::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cqsetclear::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cqsetclear`]
module"]
#[doc(alias = "CQSETCLEAR")]
pub type Cqsetclear = crate::Reg<cqsetclear::CqsetclearSpec>;
#[doc = "Command Queue Flag Set/Clear"]
pub mod cqsetclear;
#[doc = "CQPAUSE (rw) register accessor: Command Queue Pause Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`cqpause::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cqpause::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cqpause`]
module"]
#[doc(alias = "CQPAUSE")]
pub type Cqpause = crate::Reg<cqpause::CqpauseSpec>;
#[doc = "Command Queue Pause Mask"]
pub mod cqpause;
#[doc = "CQCURIDX (rw) register accessor: This register can be used in conjunction with the CQENDIDX register to manage the command queue. Typically software will initialize the CQCURIDX and CQENDIDX to the same value, which will cause the CQ to be paused when enabled. Software may then add entries to the command queue (in SRAM) and update CQENDIDX. The command queue operations will then increment CQCURIDX as it processes operations. Once CQCURIDX==CQENDIDX, the command queue hardware will automatically pause since no additional operations have been appended to the queue.\n\nYou can [`read`](crate::Reg::read) this register and get [`cqcuridx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cqcuridx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cqcuridx`]
module"]
#[doc(alias = "CQCURIDX")]
pub type Cqcuridx = crate::Reg<cqcuridx::CqcuridxSpec>;
#[doc = "This register can be used in conjunction with the CQENDIDX register to manage the command queue. Typically software will initialize the CQCURIDX and CQENDIDX to the same value, which will cause the CQ to be paused when enabled. Software may then add entries to the command queue (in SRAM) and update CQENDIDX. The command queue operations will then increment CQCURIDX as it processes operations. Once CQCURIDX==CQENDIDX, the command queue hardware will automatically pause since no additional operations have been appended to the queue."]
pub mod cqcuridx;
#[doc = "CQENDIDX (rw) register accessor: Command Queue End Index\n\nYou can [`read`](crate::Reg::read) this register and get [`cqendidx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cqendidx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cqendidx`]
module"]
#[doc(alias = "CQENDIDX")]
pub type Cqendidx = crate::Reg<cqendidx::CqendidxSpec>;
#[doc = "Command Queue End Index"]
pub mod cqendidx;
#[doc = "STATXIPDMA (rw) register accessor: Debug XIP DMA State\n\nYou can [`read`](crate::Reg::read) this register and get [`statxipdma::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`statxipdma::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@statxipdma`]
module"]
#[doc(alias = "STATXIPDMA")]
pub type Statxipdma = crate::Reg<statxipdma::StatxipdmaSpec>;
#[doc = "Debug XIP DMA State"]
pub mod statxipdma;
