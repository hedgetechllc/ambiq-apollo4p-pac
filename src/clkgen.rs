#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0c],
    octrl: Octrl,
    clkout: Clkout,
    _reserved2: [u8; 0x0c],
    hfadj: Hfadj,
    _reserved3: [u8; 0x0c],
    clockenstat: Clockenstat,
    clocken2stat: Clocken2stat,
    clocken3stat: Clocken3stat,
    _reserved6: [u8; 0x08],
    misc: Misc,
    hf2adj0: Hf2adj0,
    hf2adj1: Hf2adj1,
    hf2adj2: Hf2adj2,
    hf2val: Hf2val,
    _reserved11: [u8; 0x20],
    lfrcctrl: Lfrcctrl,
    _reserved12: [u8; 0x08],
    dispclkctrl: Dispclkctrl,
    clkgenspares: Clkgenspares,
    hfrcidlecounters: Hfrcidlecounters,
    _reserved15: [u8; 0x70],
    intrpten: Intrpten,
    intrptstat: Intrptstat,
    intrptclr: Intrptclr,
    intrptset: Intrptset,
}
impl RegisterBlock {
    #[doc = "0x0c - This register includes controls for autocalibration in addition to the RTC oscillator controls."]
    #[inline(always)]
    pub const fn octrl(&self) -> &Octrl {
        &self.octrl
    }
    #[doc = "0x10 - This register enables the CLKOUT to the GPIOs, and selects the clock source to that."]
    #[inline(always)]
    pub const fn clkout(&self) -> &Clkout {
        &self.clkout
    }
    #[doc = "0x20 - This register controls the HFRC adjustment. The HFRC clock can change with temperature and process corners, and this register controls the HFRC adjustment logic which reduces the fluctuations to the clock."]
    #[inline(always)]
    pub const fn hfadj(&self) -> &Hfadj {
        &self.hfadj
    }
    #[doc = "0x30 - This register provides the enable status to all the peripheral clocks."]
    #[inline(always)]
    pub const fn clockenstat(&self) -> &Clockenstat {
        &self.clockenstat
    }
    #[doc = "0x34 - This is a continuation of the clock enable status."]
    #[inline(always)]
    pub const fn clocken2stat(&self) -> &Clocken2stat {
        &self.clocken2stat
    }
    #[doc = "0x38 - This is a continuation of the clock enable status."]
    #[inline(always)]
    pub const fn clocken3stat(&self) -> &Clocken3stat {
        &self.clocken3stat
    }
    #[doc = "0x44 - This register controls a 'safe' mode for burst, which disables the clock when burst transition is happening. It also includes a register to force the HFRC during deep sleep. It is mainly used for debug and testing."]
    #[inline(always)]
    pub const fn misc(&self) -> &Misc {
        &self.misc
    }
    #[doc = "0x48 - This register controls hf2adj enable, fast_start enable, fast_start_delay setting and counter input offset."]
    #[inline(always)]
    pub const fn hf2adj0(&self) -> &Hf2adj0 {
        &self.hf2adj0
    }
    #[doc = "0x4c - This register controls hf2adj trimming enable and trimming offset."]
    #[inline(always)]
    pub const fn hf2adj1(&self) -> &Hf2adj1 {
        &self.hf2adj1
    }
    #[doc = "0x50 - This register controls xtal32m divider ratio and HF2ADJ ration setting."]
    #[inline(always)]
    pub const fn hf2adj2(&self) -> &Hf2adj2 {
        &self.hf2adj2
    }
    #[doc = "0x54 - This register provides the read back of the HF2TUNE"]
    #[inline(always)]
    pub const fn hf2val(&self) -> &Hf2val {
        &self.hf2val
    }
    #[doc = "0x78 - LFRC control"]
    #[inline(always)]
    pub const fn lfrcctrl(&self) -> &Lfrcctrl {
        &self.lfrcctrl
    }
    #[doc = "0x84 - Provides ability to select the PLL reference clock, and derivative of the display clock"]
    #[inline(always)]
    pub const fn dispclkctrl(&self) -> &Dispclkctrl {
        &self.dispclkctrl
    }
    #[doc = "0x88 - CLKGEN Spare Regs"]
    #[inline(always)]
    pub const fn clkgenspares(&self) -> &Clkgenspares {
        &self.clkgenspares
    }
    #[doc = "0x8c - Provides SW controlled # idle cycles before powering down HFRC, HFRC2., core clock enable(s)"]
    #[inline(always)]
    pub const fn hfrcidlecounters(&self) -> &Hfrcidlecounters {
        &self.hfrcidlecounters
    }
    #[doc = "0x100 - Set bits in this register to allow this module to generate the corresponding interrupt."]
    #[inline(always)]
    pub const fn intrpten(&self) -> &Intrpten {
        &self.intrpten
    }
    #[doc = "0x104 - Read bits from this register to discover the cause of a recent interrupt."]
    #[inline(always)]
    pub const fn intrptstat(&self) -> &Intrptstat {
        &self.intrptstat
    }
    #[doc = "0x108 - Write a 1 to a bit in this register to clear the interrupt status associated with that bit."]
    #[inline(always)]
    pub const fn intrptclr(&self) -> &Intrptclr {
        &self.intrptclr
    }
    #[doc = "0x10c - Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes)."]
    #[inline(always)]
    pub const fn intrptset(&self) -> &Intrptset {
        &self.intrptset
    }
}
#[doc = "OCTRL (rw) register accessor: This register includes controls for autocalibration in addition to the RTC oscillator controls.\n\nYou can [`read`](crate::Reg::read) this register and get [`octrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`octrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@octrl`]
module"]
#[doc(alias = "OCTRL")]
pub type Octrl = crate::Reg<octrl::OctrlSpec>;
#[doc = "This register includes controls for autocalibration in addition to the RTC oscillator controls."]
pub mod octrl;
#[doc = "CLKOUT (rw) register accessor: This register enables the CLKOUT to the GPIOs, and selects the clock source to that.\n\nYou can [`read`](crate::Reg::read) this register and get [`clkout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkout`]
module"]
#[doc(alias = "CLKOUT")]
pub type Clkout = crate::Reg<clkout::ClkoutSpec>;
#[doc = "This register enables the CLKOUT to the GPIOs, and selects the clock source to that."]
pub mod clkout;
#[doc = "HFADJ (rw) register accessor: This register controls the HFRC adjustment. The HFRC clock can change with temperature and process corners, and this register controls the HFRC adjustment logic which reduces the fluctuations to the clock.\n\nYou can [`read`](crate::Reg::read) this register and get [`hfadj::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfadj::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfadj`]
module"]
#[doc(alias = "HFADJ")]
pub type Hfadj = crate::Reg<hfadj::HfadjSpec>;
#[doc = "This register controls the HFRC adjustment. The HFRC clock can change with temperature and process corners, and this register controls the HFRC adjustment logic which reduces the fluctuations to the clock."]
pub mod hfadj;
#[doc = "CLOCKENSTAT (rw) register accessor: This register provides the enable status to all the peripheral clocks.\n\nYou can [`read`](crate::Reg::read) this register and get [`clockenstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clockenstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clockenstat`]
module"]
#[doc(alias = "CLOCKENSTAT")]
pub type Clockenstat = crate::Reg<clockenstat::ClockenstatSpec>;
#[doc = "This register provides the enable status to all the peripheral clocks."]
pub mod clockenstat;
#[doc = "CLOCKEN2STAT (rw) register accessor: This is a continuation of the clock enable status.\n\nYou can [`read`](crate::Reg::read) this register and get [`clocken2stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clocken2stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clocken2stat`]
module"]
#[doc(alias = "CLOCKEN2STAT")]
pub type Clocken2stat = crate::Reg<clocken2stat::Clocken2statSpec>;
#[doc = "This is a continuation of the clock enable status."]
pub mod clocken2stat;
#[doc = "CLOCKEN3STAT (rw) register accessor: This is a continuation of the clock enable status.\n\nYou can [`read`](crate::Reg::read) this register and get [`clocken3stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clocken3stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clocken3stat`]
module"]
#[doc(alias = "CLOCKEN3STAT")]
pub type Clocken3stat = crate::Reg<clocken3stat::Clocken3statSpec>;
#[doc = "This is a continuation of the clock enable status."]
pub mod clocken3stat;
#[doc = "MISC (rw) register accessor: This register controls a 'safe' mode for burst, which disables the clock when burst transition is happening. It also includes a register to force the HFRC during deep sleep. It is mainly used for debug and testing.\n\nYou can [`read`](crate::Reg::read) this register and get [`misc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`misc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@misc`]
module"]
#[doc(alias = "MISC")]
pub type Misc = crate::Reg<misc::MiscSpec>;
#[doc = "This register controls a 'safe' mode for burst, which disables the clock when burst transition is happening. It also includes a register to force the HFRC during deep sleep. It is mainly used for debug and testing."]
pub mod misc;
#[doc = "HF2ADJ0 (rw) register accessor: This register controls hf2adj enable, fast_start enable, fast_start_delay setting and counter input offset.\n\nYou can [`read`](crate::Reg::read) this register and get [`hf2adj0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hf2adj0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hf2adj0`]
module"]
#[doc(alias = "HF2ADJ0")]
pub type Hf2adj0 = crate::Reg<hf2adj0::Hf2adj0Spec>;
#[doc = "This register controls hf2adj enable, fast_start enable, fast_start_delay setting and counter input offset."]
pub mod hf2adj0;
#[doc = "HF2ADJ1 (rw) register accessor: This register controls hf2adj trimming enable and trimming offset.\n\nYou can [`read`](crate::Reg::read) this register and get [`hf2adj1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hf2adj1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hf2adj1`]
module"]
#[doc(alias = "HF2ADJ1")]
pub type Hf2adj1 = crate::Reg<hf2adj1::Hf2adj1Spec>;
#[doc = "This register controls hf2adj trimming enable and trimming offset."]
pub mod hf2adj1;
#[doc = "HF2ADJ2 (rw) register accessor: This register controls xtal32m divider ratio and HF2ADJ ration setting.\n\nYou can [`read`](crate::Reg::read) this register and get [`hf2adj2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hf2adj2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hf2adj2`]
module"]
#[doc(alias = "HF2ADJ2")]
pub type Hf2adj2 = crate::Reg<hf2adj2::Hf2adj2Spec>;
#[doc = "This register controls xtal32m divider ratio and HF2ADJ ration setting."]
pub mod hf2adj2;
#[doc = "HF2VAL (rw) register accessor: This register provides the read back of the HF2TUNE\n\nYou can [`read`](crate::Reg::read) this register and get [`hf2val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hf2val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hf2val`]
module"]
#[doc(alias = "HF2VAL")]
pub type Hf2val = crate::Reg<hf2val::Hf2valSpec>;
#[doc = "This register provides the read back of the HF2TUNE"]
pub mod hf2val;
#[doc = "LFRCCTRL (rw) register accessor: LFRC control\n\nYou can [`read`](crate::Reg::read) this register and get [`lfrcctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfrcctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfrcctrl`]
module"]
#[doc(alias = "LFRCCTRL")]
pub type Lfrcctrl = crate::Reg<lfrcctrl::LfrcctrlSpec>;
#[doc = "LFRC control"]
pub mod lfrcctrl;
#[doc = "DISPCLKCTRL (rw) register accessor: Provides ability to select the PLL reference clock, and derivative of the display clock\n\nYou can [`read`](crate::Reg::read) this register and get [`dispclkctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dispclkctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dispclkctrl`]
module"]
#[doc(alias = "DISPCLKCTRL")]
pub type Dispclkctrl = crate::Reg<dispclkctrl::DispclkctrlSpec>;
#[doc = "Provides ability to select the PLL reference clock, and derivative of the display clock"]
pub mod dispclkctrl;
#[doc = "CLKGENSPARES (rw) register accessor: CLKGEN Spare Regs\n\nYou can [`read`](crate::Reg::read) this register and get [`clkgenspares::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkgenspares::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkgenspares`]
module"]
#[doc(alias = "CLKGENSPARES")]
pub type Clkgenspares = crate::Reg<clkgenspares::ClkgensparesSpec>;
#[doc = "CLKGEN Spare Regs"]
pub mod clkgenspares;
#[doc = "HFRCIDLECOUNTERS (rw) register accessor: Provides SW controlled # idle cycles before powering down HFRC, HFRC2., core clock enable(s)\n\nYou can [`read`](crate::Reg::read) this register and get [`hfrcidlecounters::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfrcidlecounters::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfrcidlecounters`]
module"]
#[doc(alias = "HFRCIDLECOUNTERS")]
pub type Hfrcidlecounters = crate::Reg<hfrcidlecounters::HfrcidlecountersSpec>;
#[doc = "Provides SW controlled # idle cycles before powering down HFRC, HFRC2., core clock enable(s)"]
pub mod hfrcidlecounters;
#[doc = "INTRPTEN (rw) register accessor: Set bits in this register to allow this module to generate the corresponding interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`intrpten::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intrpten::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intrpten`]
module"]
#[doc(alias = "INTRPTEN")]
pub type Intrpten = crate::Reg<intrpten::IntrptenSpec>;
#[doc = "Set bits in this register to allow this module to generate the corresponding interrupt."]
pub mod intrpten;
#[doc = "INTRPTSTAT (rw) register accessor: Read bits from this register to discover the cause of a recent interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`intrptstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intrptstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intrptstat`]
module"]
#[doc(alias = "INTRPTSTAT")]
pub type Intrptstat = crate::Reg<intrptstat::IntrptstatSpec>;
#[doc = "Read bits from this register to discover the cause of a recent interrupt."]
pub mod intrptstat;
#[doc = "INTRPTCLR (rw) register accessor: Write a 1 to a bit in this register to clear the interrupt status associated with that bit.\n\nYou can [`read`](crate::Reg::read) this register and get [`intrptclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intrptclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intrptclr`]
module"]
#[doc(alias = "INTRPTCLR")]
pub type Intrptclr = crate::Reg<intrptclr::IntrptclrSpec>;
#[doc = "Write a 1 to a bit in this register to clear the interrupt status associated with that bit."]
pub mod intrptclr;
#[doc = "INTRPTSET (rw) register accessor: Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes).\n\nYou can [`read`](crate::Reg::read) this register and get [`intrptset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intrptset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intrptset`]
module"]
#[doc(alias = "INTRPTSET")]
pub type Intrptset = crate::Reg<intrptset::IntrptsetSpec>;
#[doc = "Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes)."]
pub mod intrptset;
