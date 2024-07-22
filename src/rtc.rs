#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    rtcctl: Rtcctl,
    rtcstat: Rtcstat,
    _reserved2: [u8; 0x18],
    ctrlow: Ctrlow,
    ctrup: Ctrup,
    _reserved4: [u8; 0x08],
    almlow: Almlow,
    almup: Almup,
    _reserved6: [u8; 0x01c8],
    inten: Inten,
    intstat: Intstat,
    intclr: Intclr,
    intset: Intset,
}
impl RegisterBlock {
    #[doc = "0x00 - This is the register control for the RTC module. It enables counter writes and sets the alarm repeat interval."]
    #[inline(always)]
    pub const fn rtcctl(&self) -> &Rtcctl {
        &self.rtcctl
    }
    #[doc = "0x04 - This is the register status for the RTC module."]
    #[inline(always)]
    pub const fn rtcstat(&self) -> &Rtcstat {
        &self.rtcstat
    }
    #[doc = "0x20 - This counter contains the values for hour, minutes, seconds and 100ths of a second Counter."]
    #[inline(always)]
    pub const fn ctrlow(&self) -> &Ctrlow {
        &self.ctrlow
    }
    #[doc = "0x24 - This register contains the day, month and year information. It contains which day in the week, and the century as well. The information of the century can also be derived from the year information. The 31st bit contains the error bit. See description in the register bit for condition when error is triggered."]
    #[inline(always)]
    pub const fn ctrup(&self) -> &Ctrup {
        &self.ctrup
    }
    #[doc = "0x30 - This register is the Alarm settings for hours, minutes, second and 1/100th seconds settings."]
    #[inline(always)]
    pub const fn almlow(&self) -> &Almlow {
        &self.almlow
    }
    #[doc = "0x34 - This register is the alarm settings for week, month and day."]
    #[inline(always)]
    pub const fn almup(&self) -> &Almup {
        &self.almup
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
#[doc = "RTCCTL (rw) register accessor: This is the register control for the RTC module. It enables counter writes and sets the alarm repeat interval.\n\nYou can [`read`](crate::Reg::read) this register and get [`rtcctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtcctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtcctl`]
module"]
#[doc(alias = "RTCCTL")]
pub type Rtcctl = crate::Reg<rtcctl::RtcctlSpec>;
#[doc = "This is the register control for the RTC module. It enables counter writes and sets the alarm repeat interval."]
pub mod rtcctl;
#[doc = "RTCSTAT (rw) register accessor: This is the register status for the RTC module.\n\nYou can [`read`](crate::Reg::read) this register and get [`rtcstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtcstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtcstat`]
module"]
#[doc(alias = "RTCSTAT")]
pub type Rtcstat = crate::Reg<rtcstat::RtcstatSpec>;
#[doc = "This is the register status for the RTC module."]
pub mod rtcstat;
#[doc = "CTRLOW (rw) register accessor: This counter contains the values for hour, minutes, seconds and 100ths of a second Counter.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlow::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlow::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrlow`]
module"]
#[doc(alias = "CTRLOW")]
pub type Ctrlow = crate::Reg<ctrlow::CtrlowSpec>;
#[doc = "This counter contains the values for hour, minutes, seconds and 100ths of a second Counter."]
pub mod ctrlow;
#[doc = "CTRUP (rw) register accessor: This register contains the day, month and year information. It contains which day in the week, and the century as well. The information of the century can also be derived from the year information. The 31st bit contains the error bit. See description in the register bit for condition when error is triggered.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrup::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrup::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrup`]
module"]
#[doc(alias = "CTRUP")]
pub type Ctrup = crate::Reg<ctrup::CtrupSpec>;
#[doc = "This register contains the day, month and year information. It contains which day in the week, and the century as well. The information of the century can also be derived from the year information. The 31st bit contains the error bit. See description in the register bit for condition when error is triggered."]
pub mod ctrup;
#[doc = "ALMLOW (rw) register accessor: This register is the Alarm settings for hours, minutes, second and 1/100th seconds settings.\n\nYou can [`read`](crate::Reg::read) this register and get [`almlow::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`almlow::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@almlow`]
module"]
#[doc(alias = "ALMLOW")]
pub type Almlow = crate::Reg<almlow::AlmlowSpec>;
#[doc = "This register is the Alarm settings for hours, minutes, second and 1/100th seconds settings."]
pub mod almlow;
#[doc = "ALMUP (rw) register accessor: This register is the alarm settings for week, month and day.\n\nYou can [`read`](crate::Reg::read) this register and get [`almup::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`almup::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@almup`]
module"]
#[doc(alias = "ALMUP")]
pub type Almup = crate::Reg<almup::AlmupSpec>;
#[doc = "This register is the alarm settings for week, month and day."]
pub mod almup;
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
