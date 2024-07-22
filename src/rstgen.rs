#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cfg: Cfg,
    swpoi: Swpoi,
    swpor: Swpor,
    _reserved3: [u8; 0x08],
    simobodm: Simobodm,
    _reserved4: [u8; 0x01e8],
    inten: Inten,
    intstat: Intstat,
    intclr: Intclr,
    intset: Intset,
    _reserved8: [u8; 0x864c],
    stat: Stat,
}
impl RegisterBlock {
    #[doc = "0x00 - Reset configuration register. This controls the reset enables for brownout condition, choice of brownout method and for the expiration of the watch dog timer."]
    #[inline(always)]
    pub const fn cfg(&self) -> &Cfg {
        &self.cfg
    }
    #[doc = "0x04 - This is the software POI reset. writing the key value to this register will trigger a POI to the system. This will cause a reset to all blocks except for registers in clock gen, RTC and the stimer."]
    #[inline(always)]
    pub const fn swpoi(&self) -> &Swpoi {
        &self.swpoi
    }
    #[doc = "0x08 - This is the software POR reset. Writing the key value to this register will trigger a POR to the system. This will cause a reset to all blocks except for registers in clock gen, RTC, power management unit, the stimer, and the power management unit."]
    #[inline(always)]
    pub const fn swpor(&self) -> &Swpor {
        &self.swpor
    }
    #[doc = "0x14 - This register unmasks the individual digital detection brownout bits into the interrupt block"]
    #[inline(always)]
    pub const fn simobodm(&self) -> &Simobodm {
        &self.simobodm
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
    #[doc = "0x885c - This register contains the status for brownout events and the causes for resets.\n NOTE 1: All bits in this register, including reserved bits, are writable. Therefore care should be taken not to write this register.\n NOTE 2: This register is only reset by POI not by HRESETn. Its contents are intended to survive all reset level except POI and full power cycles."]
    #[inline(always)]
    pub const fn stat(&self) -> &Stat {
        &self.stat
    }
}
#[doc = "CFG (rw) register accessor: Reset configuration register. This controls the reset enables for brownout condition, choice of brownout method and for the expiration of the watch dog timer.\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg`]
module"]
#[doc(alias = "CFG")]
pub type Cfg = crate::Reg<cfg::CfgSpec>;
#[doc = "Reset configuration register. This controls the reset enables for brownout condition, choice of brownout method and for the expiration of the watch dog timer."]
pub mod cfg;
#[doc = "SWPOI (rw) register accessor: This is the software POI reset. writing the key value to this register will trigger a POI to the system. This will cause a reset to all blocks except for registers in clock gen, RTC and the stimer.\n\nYou can [`read`](crate::Reg::read) this register and get [`swpoi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swpoi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swpoi`]
module"]
#[doc(alias = "SWPOI")]
pub type Swpoi = crate::Reg<swpoi::SwpoiSpec>;
#[doc = "This is the software POI reset. writing the key value to this register will trigger a POI to the system. This will cause a reset to all blocks except for registers in clock gen, RTC and the stimer."]
pub mod swpoi;
#[doc = "SWPOR (rw) register accessor: This is the software POR reset. Writing the key value to this register will trigger a POR to the system. This will cause a reset to all blocks except for registers in clock gen, RTC, power management unit, the stimer, and the power management unit.\n\nYou can [`read`](crate::Reg::read) this register and get [`swpor::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swpor::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swpor`]
module"]
#[doc(alias = "SWPOR")]
pub type Swpor = crate::Reg<swpor::SwporSpec>;
#[doc = "This is the software POR reset. Writing the key value to this register will trigger a POR to the system. This will cause a reset to all blocks except for registers in clock gen, RTC, power management unit, the stimer, and the power management unit."]
pub mod swpor;
#[doc = "SIMOBODM (rw) register accessor: This register unmasks the individual digital detection brownout bits into the interrupt block\n\nYou can [`read`](crate::Reg::read) this register and get [`simobodm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`simobodm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@simobodm`]
module"]
#[doc(alias = "SIMOBODM")]
pub type Simobodm = crate::Reg<simobodm::SimobodmSpec>;
#[doc = "This register unmasks the individual digital detection brownout bits into the interrupt block"]
pub mod simobodm;
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
#[doc = "STAT (rw) register accessor: This register contains the status for brownout events and the causes for resets.\n NOTE 1: All bits in this register, including reserved bits, are writable. Therefore care should be taken not to write this register.\n NOTE 2: This register is only reset by POI not by HRESETn. Its contents are intended to survive all reset level except POI and full power cycles.\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat`]
module"]
#[doc(alias = "STAT")]
pub type Stat = crate::Reg<stat::StatSpec>;
#[doc = "This register contains the status for brownout events and the causes for resets.\n NOTE 1: All bits in this register, including reserved bits, are writable. Therefore care should be taken not to write this register.\n NOTE 2: This register is only reset by POI not by HRESETn. Its contents are intended to survive all reset level except POI and full power cycles."]
pub mod stat;
