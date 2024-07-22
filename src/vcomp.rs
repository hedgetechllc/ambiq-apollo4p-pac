#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cfg: Cfg,
    stat: Stat,
    pwdkey: Pwdkey,
    _reserved3: [u8; 0x01f4],
    inten: Inten,
    intstat: Intstat,
    intclr: Intclr,
    intset: Intset,
}
impl RegisterBlock {
    #[doc = "0x00 - The Voltage Comparator Configuration Register contains the software control for selecting beween the 4 options for the positive input as well as the multiple options for the reference input."]
    #[inline(always)]
    pub const fn cfg(&self) -> &Cfg {
        &self.cfg
    }
    #[doc = "0x04 - Status"]
    #[inline(always)]
    pub const fn stat(&self) -> &Stat {
        &self.stat
    }
    #[doc = "0x08 - Write a value of 0x37 to unlock, write any other value to lock. This register also indicates lock status when read. When in the unlccked state (i.e. 0x37 has been written), it reads as 1. When in the locked state, it reads as 0."]
    #[inline(always)]
    pub const fn pwdkey(&self) -> &Pwdkey {
        &self.pwdkey
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
}
#[doc = "CFG (rw) register accessor: The Voltage Comparator Configuration Register contains the software control for selecting beween the 4 options for the positive input as well as the multiple options for the reference input.\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg`]
module"]
#[doc(alias = "CFG")]
pub type Cfg = crate::Reg<cfg::CfgSpec>;
#[doc = "The Voltage Comparator Configuration Register contains the software control for selecting beween the 4 options for the positive input as well as the multiple options for the reference input."]
pub mod cfg;
#[doc = "STAT (rw) register accessor: Status\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat`]
module"]
#[doc(alias = "STAT")]
pub type Stat = crate::Reg<stat::StatSpec>;
#[doc = "Status"]
pub mod stat;
#[doc = "PWDKEY (rw) register accessor: Write a value of 0x37 to unlock, write any other value to lock. This register also indicates lock status when read. When in the unlccked state (i.e. 0x37 has been written), it reads as 1. When in the locked state, it reads as 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`pwdkey::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwdkey::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwdkey`]
module"]
#[doc(alias = "PWDKEY")]
pub type Pwdkey = crate::Reg<pwdkey::PwdkeySpec>;
#[doc = "Write a value of 0x37 to unlock, write any other value to lock. This register also indicates lock status when read. When in the unlccked state (i.e. 0x37 has been written), it reads as 1. When in the locked state, it reads as 0."]
pub mod pwdkey;
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
