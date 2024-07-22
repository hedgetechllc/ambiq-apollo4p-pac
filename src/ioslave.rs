#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0100],
    fifoptr: Fifoptr,
    fifocfg: Fifocfg,
    fifothr: Fifothr,
    fupd: Fupd,
    fifoctr: Fifoctr,
    fifoinc: Fifoinc,
    cfg: Cfg,
    prenc: Prenc,
    iointctl: Iointctl,
    genadd: Genadd,
    addptr: Addptr,
    _reserved11: [u8; 0xd4],
    inten: Inten,
    intstat: Intstat,
    intclr: Intclr,
    intset: Intset,
    regaccinten: Regaccinten,
    regaccintstat: Regaccintstat,
    regaccintclr: Regaccintclr,
    regaccintset: Regaccintset,
}
impl RegisterBlock {
    #[doc = "0x100 - Current FIFO Pointer"]
    #[inline(always)]
    pub const fn fifoptr(&self) -> &Fifoptr {
        &self.fifoptr
    }
    #[doc = "0x104 - FIFO Configuration"]
    #[inline(always)]
    pub const fn fifocfg(&self) -> &Fifocfg {
        &self.fifocfg
    }
    #[doc = "0x108 - FIFO Threshold Configuration"]
    #[inline(always)]
    pub const fn fifothr(&self) -> &Fifothr {
        &self.fifothr
    }
    #[doc = "0x10c - FIFO Update Status"]
    #[inline(always)]
    pub const fn fupd(&self) -> &Fupd {
        &self.fupd
    }
    #[doc = "0x110 - Overall FIFO Counter"]
    #[inline(always)]
    pub const fn fifoctr(&self) -> &Fifoctr {
        &self.fifoctr
    }
    #[doc = "0x114 - Overall FIFO Counter Increment"]
    #[inline(always)]
    pub const fn fifoinc(&self) -> &Fifoinc {
        &self.fifoinc
    }
    #[doc = "0x118 - I/O Slave Configuration"]
    #[inline(always)]
    pub const fn cfg(&self) -> &Cfg {
        &self.cfg
    }
    #[doc = "0x11c - I/O Slave Interrupt Priority Encode"]
    #[inline(always)]
    pub const fn prenc(&self) -> &Prenc {
        &self.prenc
    }
    #[doc = "0x120 - I/O Interrupt Control"]
    #[inline(always)]
    pub const fn iointctl(&self) -> &Iointctl {
        &self.iointctl
    }
    #[doc = "0x124 - General Address Data"]
    #[inline(always)]
    pub const fn genadd(&self) -> &Genadd {
        &self.genadd
    }
    #[doc = "0x128 - Address pointer"]
    #[inline(always)]
    pub const fn addptr(&self) -> &Addptr {
        &self.addptr
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
    #[doc = "0x210 - Set bits in this register to allow this module to generate the corresponding interrupt."]
    #[inline(always)]
    pub const fn regaccinten(&self) -> &Regaccinten {
        &self.regaccinten
    }
    #[doc = "0x214 - Read bits from this register to discover the cause of a recent interrupt."]
    #[inline(always)]
    pub const fn regaccintstat(&self) -> &Regaccintstat {
        &self.regaccintstat
    }
    #[doc = "0x218 - Write a 1 to a bit in this register to clear the interrupt status associated with that bit."]
    #[inline(always)]
    pub const fn regaccintclr(&self) -> &Regaccintclr {
        &self.regaccintclr
    }
    #[doc = "0x21c - Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes)."]
    #[inline(always)]
    pub const fn regaccintset(&self) -> &Regaccintset {
        &self.regaccintset
    }
}
#[doc = "FIFOPTR (rw) register accessor: Current FIFO Pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`fifoptr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifoptr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifoptr`]
module"]
#[doc(alias = "FIFOPTR")]
pub type Fifoptr = crate::Reg<fifoptr::FifoptrSpec>;
#[doc = "Current FIFO Pointer"]
pub mod fifoptr;
#[doc = "FIFOCFG (rw) register accessor: FIFO Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`fifocfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifocfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifocfg`]
module"]
#[doc(alias = "FIFOCFG")]
pub type Fifocfg = crate::Reg<fifocfg::FifocfgSpec>;
#[doc = "FIFO Configuration"]
pub mod fifocfg;
#[doc = "FIFOTHR (rw) register accessor: FIFO Threshold Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`fifothr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifothr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifothr`]
module"]
#[doc(alias = "FIFOTHR")]
pub type Fifothr = crate::Reg<fifothr::FifothrSpec>;
#[doc = "FIFO Threshold Configuration"]
pub mod fifothr;
#[doc = "FUPD (rw) register accessor: FIFO Update Status\n\nYou can [`read`](crate::Reg::read) this register and get [`fupd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fupd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fupd`]
module"]
#[doc(alias = "FUPD")]
pub type Fupd = crate::Reg<fupd::FupdSpec>;
#[doc = "FIFO Update Status"]
pub mod fupd;
#[doc = "FIFOCTR (rw) register accessor: Overall FIFO Counter\n\nYou can [`read`](crate::Reg::read) this register and get [`fifoctr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifoctr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifoctr`]
module"]
#[doc(alias = "FIFOCTR")]
pub type Fifoctr = crate::Reg<fifoctr::FifoctrSpec>;
#[doc = "Overall FIFO Counter"]
pub mod fifoctr;
#[doc = "FIFOINC (rw) register accessor: Overall FIFO Counter Increment\n\nYou can [`read`](crate::Reg::read) this register and get [`fifoinc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifoinc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifoinc`]
module"]
#[doc(alias = "FIFOINC")]
pub type Fifoinc = crate::Reg<fifoinc::FifoincSpec>;
#[doc = "Overall FIFO Counter Increment"]
pub mod fifoinc;
#[doc = "CFG (rw) register accessor: I/O Slave Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg`]
module"]
#[doc(alias = "CFG")]
pub type Cfg = crate::Reg<cfg::CfgSpec>;
#[doc = "I/O Slave Configuration"]
pub mod cfg;
#[doc = "PRENC (rw) register accessor: I/O Slave Interrupt Priority Encode\n\nYou can [`read`](crate::Reg::read) this register and get [`prenc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prenc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prenc`]
module"]
#[doc(alias = "PRENC")]
pub type Prenc = crate::Reg<prenc::PrencSpec>;
#[doc = "I/O Slave Interrupt Priority Encode"]
pub mod prenc;
#[doc = "IOINTCTL (rw) register accessor: I/O Interrupt Control\n\nYou can [`read`](crate::Reg::read) this register and get [`iointctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iointctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iointctl`]
module"]
#[doc(alias = "IOINTCTL")]
pub type Iointctl = crate::Reg<iointctl::IointctlSpec>;
#[doc = "I/O Interrupt Control"]
pub mod iointctl;
#[doc = "GENADD (rw) register accessor: General Address Data\n\nYou can [`read`](crate::Reg::read) this register and get [`genadd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`genadd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@genadd`]
module"]
#[doc(alias = "GENADD")]
pub type Genadd = crate::Reg<genadd::GenaddSpec>;
#[doc = "General Address Data"]
pub mod genadd;
#[doc = "ADDPTR (rw) register accessor: Address pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`addptr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addptr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addptr`]
module"]
#[doc(alias = "ADDPTR")]
pub type Addptr = crate::Reg<addptr::AddptrSpec>;
#[doc = "Address pointer"]
pub mod addptr;
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
#[doc = "REGACCINTEN (rw) register accessor: Set bits in this register to allow this module to generate the corresponding interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`regaccinten::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`regaccinten::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regaccinten`]
module"]
#[doc(alias = "REGACCINTEN")]
pub type Regaccinten = crate::Reg<regaccinten::RegaccintenSpec>;
#[doc = "Set bits in this register to allow this module to generate the corresponding interrupt."]
pub mod regaccinten;
#[doc = "REGACCINTSTAT (rw) register accessor: Read bits from this register to discover the cause of a recent interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`regaccintstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`regaccintstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regaccintstat`]
module"]
#[doc(alias = "REGACCINTSTAT")]
pub type Regaccintstat = crate::Reg<regaccintstat::RegaccintstatSpec>;
#[doc = "Read bits from this register to discover the cause of a recent interrupt."]
pub mod regaccintstat;
#[doc = "REGACCINTCLR (rw) register accessor: Write a 1 to a bit in this register to clear the interrupt status associated with that bit.\n\nYou can [`read`](crate::Reg::read) this register and get [`regaccintclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`regaccintclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regaccintclr`]
module"]
#[doc(alias = "REGACCINTCLR")]
pub type Regaccintclr = crate::Reg<regaccintclr::RegaccintclrSpec>;
#[doc = "Write a 1 to a bit in this register to clear the interrupt status associated with that bit."]
pub mod regaccintclr;
#[doc = "REGACCINTSET (rw) register accessor: Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes).\n\nYou can [`read`](crate::Reg::read) this register and get [`regaccintset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`regaccintset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regaccintset`]
module"]
#[doc(alias = "REGACCINTSET")]
pub type Regaccintset = crate::Reg<regaccintset::RegaccintsetSpec>;
#[doc = "Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes)."]
pub mod regaccintset;
