#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: Ctrl,
    corecfg0: Corecfg0,
    corecfg1: Corecfg1,
    corectrl: Corectrl,
    fifocnt: Fifocnt,
    fiforead: Fiforead,
    fifoflush: Fifoflush,
    fifothr: Fifothr,
    _reserved8: [u8; 0xe0],
    inten: Inten,
    intstat: Intstat,
    intclr: Intclr,
    intset: Intset,
    _reserved12: [u8; 0x30],
    dmatrigen: Dmatrigen,
    dmatrigstat: Dmatrigstat,
    dmacfg: Dmacfg,
    _reserved15: [u8; 0x08],
    dmatargaddr: Dmatargaddr,
    dmastat: Dmastat,
    _reserved17: [u8; 0xf4],
    dmatotcount: Dmatotcount,
}
impl RegisterBlock {
    #[doc = "0x00 - PDM Control"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x04 - PDM to PCM Core Configuration"]
    #[inline(always)]
    pub const fn corecfg0(&self) -> &Corecfg0 {
        &self.corecfg0
    }
    #[doc = "0x08 - PDM to PCM Extra Configuration"]
    #[inline(always)]
    pub const fn corecfg1(&self) -> &Corecfg1 {
        &self.corecfg1
    }
    #[doc = "0x0c - PDM to PCM Control"]
    #[inline(always)]
    pub const fn corectrl(&self) -> &Corectrl {
        &self.corectrl
    }
    #[doc = "0x10 - FIFO count"]
    #[inline(always)]
    pub const fn fifocnt(&self) -> &Fifocnt {
        &self.fifocnt
    }
    #[doc = "0x14 - FIFO Read"]
    #[inline(always)]
    pub const fn fiforead(&self) -> &Fiforead {
        &self.fiforead
    }
    #[doc = "0x18 - FIFO Flush"]
    #[inline(always)]
    pub const fn fifoflush(&self) -> &Fifoflush {
        &self.fifoflush
    }
    #[doc = "0x1c - FIFO Threshold"]
    #[inline(always)]
    pub const fn fifothr(&self) -> &Fifothr {
        &self.fifothr
    }
    #[doc = "0x100 - Set bits in this register to allow this module to generate the corresponding interrupt."]
    #[inline(always)]
    pub const fn inten(&self) -> &Inten {
        &self.inten
    }
    #[doc = "0x104 - Read bits from this register to discover the cause of a recent interrupt."]
    #[inline(always)]
    pub const fn intstat(&self) -> &Intstat {
        &self.intstat
    }
    #[doc = "0x108 - Write a 1 to a bit in this register to clear the interrupt status associated with that bit."]
    #[inline(always)]
    pub const fn intclr(&self) -> &Intclr {
        &self.intclr
    }
    #[doc = "0x10c - Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes)."]
    #[inline(always)]
    pub const fn intset(&self) -> &Intset {
        &self.intset
    }
    #[doc = "0x140 - DMA Trigger Enable"]
    #[inline(always)]
    pub const fn dmatrigen(&self) -> &Dmatrigen {
        &self.dmatrigen
    }
    #[doc = "0x144 - DMA Trigger Status"]
    #[inline(always)]
    pub const fn dmatrigstat(&self) -> &Dmatrigstat {
        &self.dmatrigstat
    }
    #[doc = "0x148 - DMA Configuration"]
    #[inline(always)]
    pub const fn dmacfg(&self) -> &Dmacfg {
        &self.dmacfg
    }
    #[doc = "0x154 - DMA Target Address"]
    #[inline(always)]
    pub const fn dmatargaddr(&self) -> &Dmatargaddr {
        &self.dmatargaddr
    }
    #[doc = "0x158 - DMA Status"]
    #[inline(always)]
    pub const fn dmastat(&self) -> &Dmastat {
        &self.dmastat
    }
    #[doc = "0x250 - DMA Total Transfer Count"]
    #[inline(always)]
    pub const fn dmatotcount(&self) -> &Dmatotcount {
        &self.dmatotcount
    }
}
#[doc = "CTRL (rw) register accessor: PDM Control\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "PDM Control"]
pub mod ctrl;
#[doc = "CORECFG0 (rw) register accessor: PDM to PCM Core Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`corecfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`corecfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@corecfg0`]
module"]
#[doc(alias = "CORECFG0")]
pub type Corecfg0 = crate::Reg<corecfg0::Corecfg0Spec>;
#[doc = "PDM to PCM Core Configuration"]
pub mod corecfg0;
#[doc = "CORECFG1 (rw) register accessor: PDM to PCM Extra Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`corecfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`corecfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@corecfg1`]
module"]
#[doc(alias = "CORECFG1")]
pub type Corecfg1 = crate::Reg<corecfg1::Corecfg1Spec>;
#[doc = "PDM to PCM Extra Configuration"]
pub mod corecfg1;
#[doc = "CORECTRL (rw) register accessor: PDM to PCM Control\n\nYou can [`read`](crate::Reg::read) this register and get [`corectrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`corectrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@corectrl`]
module"]
#[doc(alias = "CORECTRL")]
pub type Corectrl = crate::Reg<corectrl::CorectrlSpec>;
#[doc = "PDM to PCM Control"]
pub mod corectrl;
#[doc = "FIFOCNT (rw) register accessor: FIFO count\n\nYou can [`read`](crate::Reg::read) this register and get [`fifocnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifocnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifocnt`]
module"]
#[doc(alias = "FIFOCNT")]
pub type Fifocnt = crate::Reg<fifocnt::FifocntSpec>;
#[doc = "FIFO count"]
pub mod fifocnt;
#[doc = "FIFOREAD (rw) register accessor: FIFO Read\n\nYou can [`read`](crate::Reg::read) this register and get [`fiforead::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fiforead::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fiforead`]
module"]
#[doc(alias = "FIFOREAD")]
pub type Fiforead = crate::Reg<fiforead::FiforeadSpec>;
#[doc = "FIFO Read"]
pub mod fiforead;
#[doc = "FIFOFLUSH (rw) register accessor: FIFO Flush\n\nYou can [`read`](crate::Reg::read) this register and get [`fifoflush::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifoflush::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifoflush`]
module"]
#[doc(alias = "FIFOFLUSH")]
pub type Fifoflush = crate::Reg<fifoflush::FifoflushSpec>;
#[doc = "FIFO Flush"]
pub mod fifoflush;
#[doc = "FIFOTHR (rw) register accessor: FIFO Threshold\n\nYou can [`read`](crate::Reg::read) this register and get [`fifothr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifothr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifothr`]
module"]
#[doc(alias = "FIFOTHR")]
pub type Fifothr = crate::Reg<fifothr::FifothrSpec>;
#[doc = "FIFO Threshold"]
pub mod fifothr;
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
#[doc = "DMATRIGEN (rw) register accessor: DMA Trigger Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`dmatrigen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmatrigen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmatrigen`]
module"]
#[doc(alias = "DMATRIGEN")]
pub type Dmatrigen = crate::Reg<dmatrigen::DmatrigenSpec>;
#[doc = "DMA Trigger Enable"]
pub mod dmatrigen;
#[doc = "DMATRIGSTAT (rw) register accessor: DMA Trigger Status\n\nYou can [`read`](crate::Reg::read) this register and get [`dmatrigstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmatrigstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmatrigstat`]
module"]
#[doc(alias = "DMATRIGSTAT")]
pub type Dmatrigstat = crate::Reg<dmatrigstat::DmatrigstatSpec>;
#[doc = "DMA Trigger Status"]
pub mod dmatrigstat;
#[doc = "DMACFG (rw) register accessor: DMA Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`dmacfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmacfg`]
module"]
#[doc(alias = "DMACFG")]
pub type Dmacfg = crate::Reg<dmacfg::DmacfgSpec>;
#[doc = "DMA Configuration"]
pub mod dmacfg;
#[doc = "DMATARGADDR (rw) register accessor: DMA Target Address\n\nYou can [`read`](crate::Reg::read) this register and get [`dmatargaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmatargaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmatargaddr`]
module"]
#[doc(alias = "DMATARGADDR")]
pub type Dmatargaddr = crate::Reg<dmatargaddr::DmatargaddrSpec>;
#[doc = "DMA Target Address"]
pub mod dmatargaddr;
#[doc = "DMASTAT (rw) register accessor: DMA Status\n\nYou can [`read`](crate::Reg::read) this register and get [`dmastat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmastat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmastat`]
module"]
#[doc(alias = "DMASTAT")]
pub type Dmastat = crate::Reg<dmastat::DmastatSpec>;
#[doc = "DMA Status"]
pub mod dmastat;
#[doc = "DMATOTCOUNT (rw) register accessor: DMA Total Transfer Count\n\nYou can [`read`](crate::Reg::read) this register and get [`dmatotcount::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmatotcount::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmatotcount`]
module"]
#[doc(alias = "DMATOTCOUNT")]
pub type Dmatotcount = crate::Reg<dmatotcount::DmatotcountSpec>;
#[doc = "DMA Total Transfer Count"]
pub mod dmatotcount;
