#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cachecfg: Cachecfg,
    _reserved1: [u8; 0x04],
    cachectrl: Cachectrl,
    _reserved2: [u8; 0x04],
    ncr0start: Ncr0start,
    ncr0end: Ncr0end,
    ncr1start: Ncr1start,
    ncr1end: Ncr1end,
    _reserved6: [u8; 0x30],
    daxicfg: Daxicfg,
    daxictrl: Daxictrl,
    _reserved8: [u8; 0x28],
    icodefaultaddr: Icodefaultaddr,
    dcodefaultaddr: Dcodefaultaddr,
    sysfaultaddr: Sysfaultaddr,
    faultstatus: Faultstatus,
    faultcaptureen: Faultcaptureen,
    _reserved13: [u8; 0x2c],
    inten: Inten,
    intstat: Intstat,
    intclr: Intclr,
    intset: Intset,
    writeerraddr: Writeerraddr,
    _reserved18: [u8; 0x2c],
    dmon0: Dmon0,
    dmon1: Dmon1,
    dmon2: Dmon2,
    dmon3: Dmon3,
    imon0: Imon0,
    imon1: Imon1,
    imon2: Imon2,
    imon3: Imon3,
}
impl RegisterBlock {
    #[doc = "0x00 - CM4 Cache Control"]
    #[inline(always)]
    pub const fn cachecfg(&self) -> &Cachecfg {
        &self.cachecfg
    }
    #[doc = "0x08 - Cache Control"]
    #[inline(always)]
    pub const fn cachectrl(&self) -> &Cachectrl {
        &self.cachectrl
    }
    #[doc = "0x10 - CM4 Cache Noncachable Region 0 Start"]
    #[inline(always)]
    pub const fn ncr0start(&self) -> &Ncr0start {
        &self.ncr0start
    }
    #[doc = "0x14 - CM4 Cache Noncachable Region 0 End"]
    #[inline(always)]
    pub const fn ncr0end(&self) -> &Ncr0end {
        &self.ncr0end
    }
    #[doc = "0x18 - CM4 Cache Noncachable Region 1 Start"]
    #[inline(always)]
    pub const fn ncr1start(&self) -> &Ncr1start {
        &self.ncr1start
    }
    #[doc = "0x1c - CM4 Cache Noncachable Region 1 End"]
    #[inline(always)]
    pub const fn ncr1end(&self) -> &Ncr1end {
        &self.ncr1end
    }
    #[doc = "0x50 - DAXI Config"]
    #[inline(always)]
    pub const fn daxicfg(&self) -> &Daxicfg {
        &self.daxicfg
    }
    #[doc = "0x54 - DAXI Control"]
    #[inline(always)]
    pub const fn daxictrl(&self) -> &Daxictrl {
        &self.daxictrl
    }
    #[doc = "0x80 - ICODE bus address which was present when a bus fault occurred."]
    #[inline(always)]
    pub const fn icodefaultaddr(&self) -> &Icodefaultaddr {
        &self.icodefaultaddr
    }
    #[doc = "0x84 - DCODE bus address which was present when a bus fault occurred."]
    #[inline(always)]
    pub const fn dcodefaultaddr(&self) -> &Dcodefaultaddr {
        &self.dcodefaultaddr
    }
    #[doc = "0x88 - System bus address which was present when a bus fault occurred."]
    #[inline(always)]
    pub const fn sysfaultaddr(&self) -> &Sysfaultaddr {
        &self.sysfaultaddr
    }
    #[doc = "0x8c - Reflects the status of the bus decoders' fault detection. Any write to this register will clear all of the status bits within the register."]
    #[inline(always)]
    pub const fn faultstatus(&self) -> &Faultstatus {
        &self.faultstatus
    }
    #[doc = "0x90 - Enable the fault capture registers"]
    #[inline(always)]
    pub const fn faultcaptureen(&self) -> &Faultcaptureen {
        &self.faultcaptureen
    }
    #[doc = "0xc0 - Set bits in this register to allow this module to generate the corresponding interrupt."]
    #[inline(always)]
    pub const fn inten(&self) -> &Inten {
        &self.inten
    }
    #[doc = "0xc4 - Read bits from this register to discover the cause of a recent interrupt."]
    #[inline(always)]
    pub const fn intstat(&self) -> &Intstat {
        &self.intstat
    }
    #[doc = "0xc8 - Write a 1 to a bit in this register to clear the interrupt status associated with that bit."]
    #[inline(always)]
    pub const fn intclr(&self) -> &Intclr {
        &self.intclr
    }
    #[doc = "0xcc - Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes)."]
    #[inline(always)]
    pub const fn intset(&self) -> &Intset {
        &self.intset
    }
    #[doc = "0xd0 - DAXI Write Error Address"]
    #[inline(always)]
    pub const fn writeerraddr(&self) -> &Writeerraddr {
        &self.writeerraddr
    }
    #[doc = "0x100 - Data Cache Total Accesses"]
    #[inline(always)]
    pub const fn dmon0(&self) -> &Dmon0 {
        &self.dmon0
    }
    #[doc = "0x104 - Data Cache Tag Lookups"]
    #[inline(always)]
    pub const fn dmon1(&self) -> &Dmon1 {
        &self.dmon1
    }
    #[doc = "0x108 - Data Cache Hits"]
    #[inline(always)]
    pub const fn dmon2(&self) -> &Dmon2 {
        &self.dmon2
    }
    #[doc = "0x10c - Data Cache Line Hits"]
    #[inline(always)]
    pub const fn dmon3(&self) -> &Dmon3 {
        &self.dmon3
    }
    #[doc = "0x110 - Instruction Cache Total Accesses"]
    #[inline(always)]
    pub const fn imon0(&self) -> &Imon0 {
        &self.imon0
    }
    #[doc = "0x114 - Instruction Cache Tag Lookups"]
    #[inline(always)]
    pub const fn imon1(&self) -> &Imon1 {
        &self.imon1
    }
    #[doc = "0x118 - Instruction Cache Hits"]
    #[inline(always)]
    pub const fn imon2(&self) -> &Imon2 {
        &self.imon2
    }
    #[doc = "0x11c - Instruction Cache Line Hits"]
    #[inline(always)]
    pub const fn imon3(&self) -> &Imon3 {
        &self.imon3
    }
}
#[doc = "CACHECFG (rw) register accessor: CM4 Cache Control\n\nYou can [`read`](crate::Reg::read) this register and get [`cachecfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cachecfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cachecfg`]
module"]
#[doc(alias = "CACHECFG")]
pub type Cachecfg = crate::Reg<cachecfg::CachecfgSpec>;
#[doc = "CM4 Cache Control"]
pub mod cachecfg;
#[doc = "CACHECTRL (rw) register accessor: Cache Control\n\nYou can [`read`](crate::Reg::read) this register and get [`cachectrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cachectrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cachectrl`]
module"]
#[doc(alias = "CACHECTRL")]
pub type Cachectrl = crate::Reg<cachectrl::CachectrlSpec>;
#[doc = "Cache Control"]
pub mod cachectrl;
#[doc = "NCR0START (rw) register accessor: CM4 Cache Noncachable Region 0 Start\n\nYou can [`read`](crate::Reg::read) this register and get [`ncr0start::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ncr0start::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ncr0start`]
module"]
#[doc(alias = "NCR0START")]
pub type Ncr0start = crate::Reg<ncr0start::Ncr0startSpec>;
#[doc = "CM4 Cache Noncachable Region 0 Start"]
pub mod ncr0start;
#[doc = "NCR0END (rw) register accessor: CM4 Cache Noncachable Region 0 End\n\nYou can [`read`](crate::Reg::read) this register and get [`ncr0end::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ncr0end::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ncr0end`]
module"]
#[doc(alias = "NCR0END")]
pub type Ncr0end = crate::Reg<ncr0end::Ncr0endSpec>;
#[doc = "CM4 Cache Noncachable Region 0 End"]
pub mod ncr0end;
#[doc = "NCR1START (rw) register accessor: CM4 Cache Noncachable Region 1 Start\n\nYou can [`read`](crate::Reg::read) this register and get [`ncr1start::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ncr1start::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ncr1start`]
module"]
#[doc(alias = "NCR1START")]
pub type Ncr1start = crate::Reg<ncr1start::Ncr1startSpec>;
#[doc = "CM4 Cache Noncachable Region 1 Start"]
pub mod ncr1start;
#[doc = "NCR1END (rw) register accessor: CM4 Cache Noncachable Region 1 End\n\nYou can [`read`](crate::Reg::read) this register and get [`ncr1end::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ncr1end::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ncr1end`]
module"]
#[doc(alias = "NCR1END")]
pub type Ncr1end = crate::Reg<ncr1end::Ncr1endSpec>;
#[doc = "CM4 Cache Noncachable Region 1 End"]
pub mod ncr1end;
#[doc = "DAXICFG (rw) register accessor: DAXI Config\n\nYou can [`read`](crate::Reg::read) this register and get [`daxicfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`daxicfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@daxicfg`]
module"]
#[doc(alias = "DAXICFG")]
pub type Daxicfg = crate::Reg<daxicfg::DaxicfgSpec>;
#[doc = "DAXI Config"]
pub mod daxicfg;
#[doc = "DAXICTRL (rw) register accessor: DAXI Control\n\nYou can [`read`](crate::Reg::read) this register and get [`daxictrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`daxictrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@daxictrl`]
module"]
#[doc(alias = "DAXICTRL")]
pub type Daxictrl = crate::Reg<daxictrl::DaxictrlSpec>;
#[doc = "DAXI Control"]
pub mod daxictrl;
#[doc = "ICODEFAULTADDR (rw) register accessor: ICODE bus address which was present when a bus fault occurred.\n\nYou can [`read`](crate::Reg::read) this register and get [`icodefaultaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icodefaultaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icodefaultaddr`]
module"]
#[doc(alias = "ICODEFAULTADDR")]
pub type Icodefaultaddr = crate::Reg<icodefaultaddr::IcodefaultaddrSpec>;
#[doc = "ICODE bus address which was present when a bus fault occurred."]
pub mod icodefaultaddr;
#[doc = "DCODEFAULTADDR (rw) register accessor: DCODE bus address which was present when a bus fault occurred.\n\nYou can [`read`](crate::Reg::read) this register and get [`dcodefaultaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcodefaultaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcodefaultaddr`]
module"]
#[doc(alias = "DCODEFAULTADDR")]
pub type Dcodefaultaddr = crate::Reg<dcodefaultaddr::DcodefaultaddrSpec>;
#[doc = "DCODE bus address which was present when a bus fault occurred."]
pub mod dcodefaultaddr;
#[doc = "SYSFAULTADDR (rw) register accessor: System bus address which was present when a bus fault occurred.\n\nYou can [`read`](crate::Reg::read) this register and get [`sysfaultaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysfaultaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysfaultaddr`]
module"]
#[doc(alias = "SYSFAULTADDR")]
pub type Sysfaultaddr = crate::Reg<sysfaultaddr::SysfaultaddrSpec>;
#[doc = "System bus address which was present when a bus fault occurred."]
pub mod sysfaultaddr;
#[doc = "FAULTSTATUS (rw) register accessor: Reflects the status of the bus decoders' fault detection. Any write to this register will clear all of the status bits within the register.\n\nYou can [`read`](crate::Reg::read) this register and get [`faultstatus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`faultstatus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@faultstatus`]
module"]
#[doc(alias = "FAULTSTATUS")]
pub type Faultstatus = crate::Reg<faultstatus::FaultstatusSpec>;
#[doc = "Reflects the status of the bus decoders' fault detection. Any write to this register will clear all of the status bits within the register."]
pub mod faultstatus;
#[doc = "FAULTCAPTUREEN (rw) register accessor: Enable the fault capture registers\n\nYou can [`read`](crate::Reg::read) this register and get [`faultcaptureen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`faultcaptureen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@faultcaptureen`]
module"]
#[doc(alias = "FAULTCAPTUREEN")]
pub type Faultcaptureen = crate::Reg<faultcaptureen::FaultcaptureenSpec>;
#[doc = "Enable the fault capture registers"]
pub mod faultcaptureen;
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
#[doc = "WRITEERRADDR (rw) register accessor: DAXI Write Error Address\n\nYou can [`read`](crate::Reg::read) this register and get [`writeerraddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`writeerraddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@writeerraddr`]
module"]
#[doc(alias = "WRITEERRADDR")]
pub type Writeerraddr = crate::Reg<writeerraddr::WriteerraddrSpec>;
#[doc = "DAXI Write Error Address"]
pub mod writeerraddr;
#[doc = "DMON0 (rw) register accessor: Data Cache Total Accesses\n\nYou can [`read`](crate::Reg::read) this register and get [`dmon0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmon0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmon0`]
module"]
#[doc(alias = "DMON0")]
pub type Dmon0 = crate::Reg<dmon0::Dmon0Spec>;
#[doc = "Data Cache Total Accesses"]
pub mod dmon0;
#[doc = "DMON1 (rw) register accessor: Data Cache Tag Lookups\n\nYou can [`read`](crate::Reg::read) this register and get [`dmon1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmon1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmon1`]
module"]
#[doc(alias = "DMON1")]
pub type Dmon1 = crate::Reg<dmon1::Dmon1Spec>;
#[doc = "Data Cache Tag Lookups"]
pub mod dmon1;
#[doc = "DMON2 (rw) register accessor: Data Cache Hits\n\nYou can [`read`](crate::Reg::read) this register and get [`dmon2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmon2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmon2`]
module"]
#[doc(alias = "DMON2")]
pub type Dmon2 = crate::Reg<dmon2::Dmon2Spec>;
#[doc = "Data Cache Hits"]
pub mod dmon2;
#[doc = "DMON3 (rw) register accessor: Data Cache Line Hits\n\nYou can [`read`](crate::Reg::read) this register and get [`dmon3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmon3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmon3`]
module"]
#[doc(alias = "DMON3")]
pub type Dmon3 = crate::Reg<dmon3::Dmon3Spec>;
#[doc = "Data Cache Line Hits"]
pub mod dmon3;
#[doc = "IMON0 (rw) register accessor: Instruction Cache Total Accesses\n\nYou can [`read`](crate::Reg::read) this register and get [`imon0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imon0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imon0`]
module"]
#[doc(alias = "IMON0")]
pub type Imon0 = crate::Reg<imon0::Imon0Spec>;
#[doc = "Instruction Cache Total Accesses"]
pub mod imon0;
#[doc = "IMON1 (rw) register accessor: Instruction Cache Tag Lookups\n\nYou can [`read`](crate::Reg::read) this register and get [`imon1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imon1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imon1`]
module"]
#[doc(alias = "IMON1")]
pub type Imon1 = crate::Reg<imon1::Imon1Spec>;
#[doc = "Instruction Cache Tag Lookups"]
pub mod imon1;
#[doc = "IMON2 (rw) register accessor: Instruction Cache Hits\n\nYou can [`read`](crate::Reg::read) this register and get [`imon2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imon2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imon2`]
module"]
#[doc(alias = "IMON2")]
pub type Imon2 = crate::Reg<imon2::Imon2Spec>;
#[doc = "Instruction Cache Hits"]
pub mod imon2;
#[doc = "IMON3 (rw) register accessor: Instruction Cache Line Hits\n\nYou can [`read`](crate::Reg::read) this register and get [`imon3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imon3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imon3`]
module"]
#[doc(alias = "IMON3")]
pub type Imon3 = crate::Reg<imon3::Imon3Spec>;
#[doc = "Instruction Cache Line Hits"]
pub mod imon3;
