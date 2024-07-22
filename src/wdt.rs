#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cfg: Cfg,
    rstrt: Rstrt,
    lock: Lock,
    count: Count,
    dsp0cfg: Dsp0cfg,
    dsp0rstrt: Dsp0rstrt,
    dsp0tlock: Dsp0tlock,
    dsp0count: Dsp0count,
    dsp1cfg: Dsp1cfg,
    dsp1rstrt: Dsp1rstrt,
    dsp1tlock: Dsp1tlock,
    dsp1count: Dsp1count,
    _reserved12: [u8; 0x01d0],
    wdtieren: Wdtieren,
    wdtierstat: Wdtierstat,
    wdtierclr: Wdtierclr,
    wdtierset: Wdtierset,
    dsp0ieren: Dsp0ieren,
    dsp0ierstat: Dsp0ierstat,
    dsp0ierclr: Dsp0ierclr,
    dsp0ierset: Dsp0ierset,
    dsp1ieren: Dsp1ieren,
    dsp1ierstat: Dsp1ierstat,
    dsp1ierclr: Dsp1ierclr,
    dsp1ierset: Dsp1ierset,
}
impl RegisterBlock {
    #[doc = "0x00 - This is the configuration register for the watch dog timer. It controls the enable, interrupt set, clocks for the timer, the compare values for the counters to trigger a reset or interrupt. This register can only be written to if the watch dog timer is unlocked (WDTLOCK is not set)."]
    #[inline(always)]
    pub const fn cfg(&self) -> &Cfg {
        &self.cfg
    }
    #[doc = "0x04 - This register will Restart the watchdog timer. Writing a special key value into this register will result in the watch dog timer being reset, so that the count will start again. It is expected that the software will periodically write to this register to indicate that the system is functional. The watch dog timer can continue running when the system is in deep sleep, and the interrupt will trigger the wake. After the wake, the core can reset the watch dog timer."]
    #[inline(always)]
    pub const fn rstrt(&self) -> &Rstrt {
        &self.rstrt
    }
    #[doc = "0x08 - This register locks the watch dog timer. Once it is locked, the configuration register (WDTCFG) for watch dog timer cannot be written to."]
    #[inline(always)]
    pub const fn lock(&self) -> &Lock {
        &self.lock
    }
    #[doc = "0x0c - This register holds the current count for the watch dog timer. This is a read only register. SW cannot set the value in the counter, but can reset it."]
    #[inline(always)]
    pub const fn count(&self) -> &Count {
        &self.count
    }
    #[doc = "0x10 - This is the configuration register for the DSP0 watch dog timer. It controls the enable, interrupt set, clocks for the timer, the compare values for the counters to trigger a reset or interrupt. This register can only be written to if the associated DSP0TLOCK is not set."]
    #[inline(always)]
    pub const fn dsp0cfg(&self) -> &Dsp0cfg {
        &self.dsp0cfg
    }
    #[doc = "0x14 - This register will restart the watchdog timer. Writing a special key value into this register will result in the watch dog timer being reset, so that the count will start again. It is expected that the software will periodically write to this register to indicate that the system is functional. The watch dog timer can continue running when the system is in deep sleep, and the interrupt will trigger the wake. After the wake, the core can reset the watch dog timer."]
    #[inline(always)]
    pub const fn dsp0rstrt(&self) -> &Dsp0rstrt {
        &self.dsp0rstrt
    }
    #[doc = "0x18 - This register locks the watch dog timer. Once it is locked, the configuration register (DSP0CFG) for watch dog timer cannot be written to and the timer is automatically enabled (WDTEN is set)."]
    #[inline(always)]
    pub const fn dsp0tlock(&self) -> &Dsp0tlock {
        &self.dsp0tlock
    }
    #[doc = "0x1c - This register holds the current count for the watch dog timer. This is a read only register. SW cannot set the value in the counter, but can reset it."]
    #[inline(always)]
    pub const fn dsp0count(&self) -> &Dsp0count {
        &self.dsp0count
    }
    #[doc = "0x20 - This is the configuration register for the DSP1 watch dog timer. It controls the enable, interrupt set, clocks for the timer, the compare values for the counters to trigger a reset or interrupt. This register can only be written to if the associated DSP1TLOCK is not set."]
    #[inline(always)]
    pub const fn dsp1cfg(&self) -> &Dsp1cfg {
        &self.dsp1cfg
    }
    #[doc = "0x24 - This register will restart the watchdog timer. Writing a special key value into this register will result in the watch dog timer being reset, so that the count will start again. It is expected that the software will periodically write to this register to indicate that the system is functional. The watch dog timer can continue running when the system is in deep sleep, and the interrupt will trigger the wake. After the wake, the core can reset the watch dog timer."]
    #[inline(always)]
    pub const fn dsp1rstrt(&self) -> &Dsp1rstrt {
        &self.dsp1rstrt
    }
    #[doc = "0x28 - This register locks the watch dog timer. Once it is locked, the configuration register (DSP1CFG) for watch dog timer cannot be written to and the timer is automatically enabled (WDTEN is set)."]
    #[inline(always)]
    pub const fn dsp1tlock(&self) -> &Dsp1tlock {
        &self.dsp1tlock
    }
    #[doc = "0x2c - This register holds the current count for the watch dog timer. This is a read only register. SW cannot set the value in the counter, but can reset it."]
    #[inline(always)]
    pub const fn dsp1count(&self) -> &Dsp1count {
        &self.dsp1count
    }
    #[doc = "0x200 - Set bits in this register to allow this module to generate the corresponding interrupt."]
    #[inline(always)]
    pub const fn wdtieren(&self) -> &Wdtieren {
        &self.wdtieren
    }
    #[doc = "0x204 - Read bits from this register to discover the cause of a recent interrupt."]
    #[inline(always)]
    pub const fn wdtierstat(&self) -> &Wdtierstat {
        &self.wdtierstat
    }
    #[doc = "0x208 - Write a 1 to a bit in this register to clear the interrupt status associated with that bit."]
    #[inline(always)]
    pub const fn wdtierclr(&self) -> &Wdtierclr {
        &self.wdtierclr
    }
    #[doc = "0x20c - Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes)."]
    #[inline(always)]
    pub const fn wdtierset(&self) -> &Wdtierset {
        &self.wdtierset
    }
    #[doc = "0x210 - Set bits in this register to allow this module to generate the corresponding interrupt."]
    #[inline(always)]
    pub const fn dsp0ieren(&self) -> &Dsp0ieren {
        &self.dsp0ieren
    }
    #[doc = "0x214 - Read bits from this register to discover the cause of a recent interrupt."]
    #[inline(always)]
    pub const fn dsp0ierstat(&self) -> &Dsp0ierstat {
        &self.dsp0ierstat
    }
    #[doc = "0x218 - Write a 1 to a bit in this register to clear the interrupt status associated with that bit."]
    #[inline(always)]
    pub const fn dsp0ierclr(&self) -> &Dsp0ierclr {
        &self.dsp0ierclr
    }
    #[doc = "0x21c - Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes)."]
    #[inline(always)]
    pub const fn dsp0ierset(&self) -> &Dsp0ierset {
        &self.dsp0ierset
    }
    #[doc = "0x220 - Set bits in this register to allow this module to generate the corresponding interrupt."]
    #[inline(always)]
    pub const fn dsp1ieren(&self) -> &Dsp1ieren {
        &self.dsp1ieren
    }
    #[doc = "0x224 - Read bits from this register to discover the cause of a recent interrupt."]
    #[inline(always)]
    pub const fn dsp1ierstat(&self) -> &Dsp1ierstat {
        &self.dsp1ierstat
    }
    #[doc = "0x228 - Write a 1 to a bit in this register to clear the interrupt status associated with that bit."]
    #[inline(always)]
    pub const fn dsp1ierclr(&self) -> &Dsp1ierclr {
        &self.dsp1ierclr
    }
    #[doc = "0x22c - Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes)."]
    #[inline(always)]
    pub const fn dsp1ierset(&self) -> &Dsp1ierset {
        &self.dsp1ierset
    }
}
#[doc = "CFG (rw) register accessor: This is the configuration register for the watch dog timer. It controls the enable, interrupt set, clocks for the timer, the compare values for the counters to trigger a reset or interrupt. This register can only be written to if the watch dog timer is unlocked (WDTLOCK is not set).\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg`]
module"]
#[doc(alias = "CFG")]
pub type Cfg = crate::Reg<cfg::CfgSpec>;
#[doc = "This is the configuration register for the watch dog timer. It controls the enable, interrupt set, clocks for the timer, the compare values for the counters to trigger a reset or interrupt. This register can only be written to if the watch dog timer is unlocked (WDTLOCK is not set)."]
pub mod cfg;
#[doc = "RSTRT (rw) register accessor: This register will Restart the watchdog timer. Writing a special key value into this register will result in the watch dog timer being reset, so that the count will start again. It is expected that the software will periodically write to this register to indicate that the system is functional. The watch dog timer can continue running when the system is in deep sleep, and the interrupt will trigger the wake. After the wake, the core can reset the watch dog timer.\n\nYou can [`read`](crate::Reg::read) this register and get [`rstrt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstrt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstrt`]
module"]
#[doc(alias = "RSTRT")]
pub type Rstrt = crate::Reg<rstrt::RstrtSpec>;
#[doc = "This register will Restart the watchdog timer. Writing a special key value into this register will result in the watch dog timer being reset, so that the count will start again. It is expected that the software will periodically write to this register to indicate that the system is functional. The watch dog timer can continue running when the system is in deep sleep, and the interrupt will trigger the wake. After the wake, the core can reset the watch dog timer."]
pub mod rstrt;
#[doc = "LOCK (rw) register accessor: This register locks the watch dog timer. Once it is locked, the configuration register (WDTCFG) for watch dog timer cannot be written to.\n\nYou can [`read`](crate::Reg::read) this register and get [`lock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lock`]
module"]
#[doc(alias = "LOCK")]
pub type Lock = crate::Reg<lock::LockSpec>;
#[doc = "This register locks the watch dog timer. Once it is locked, the configuration register (WDTCFG) for watch dog timer cannot be written to."]
pub mod lock;
#[doc = "COUNT (rw) register accessor: This register holds the current count for the watch dog timer. This is a read only register. SW cannot set the value in the counter, but can reset it.\n\nYou can [`read`](crate::Reg::read) this register and get [`count::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`count::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@count`]
module"]
#[doc(alias = "COUNT")]
pub type Count = crate::Reg<count::CountSpec>;
#[doc = "This register holds the current count for the watch dog timer. This is a read only register. SW cannot set the value in the counter, but can reset it."]
pub mod count;
#[doc = "DSP0CFG (rw) register accessor: This is the configuration register for the DSP0 watch dog timer. It controls the enable, interrupt set, clocks for the timer, the compare values for the counters to trigger a reset or interrupt. This register can only be written to if the associated DSP0TLOCK is not set.\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp0cfg`]
module"]
#[doc(alias = "DSP0CFG")]
pub type Dsp0cfg = crate::Reg<dsp0cfg::Dsp0cfgSpec>;
#[doc = "This is the configuration register for the DSP0 watch dog timer. It controls the enable, interrupt set, clocks for the timer, the compare values for the counters to trigger a reset or interrupt. This register can only be written to if the associated DSP0TLOCK is not set."]
pub mod dsp0cfg;
#[doc = "DSP0RSTRT (rw) register accessor: This register will restart the watchdog timer. Writing a special key value into this register will result in the watch dog timer being reset, so that the count will start again. It is expected that the software will periodically write to this register to indicate that the system is functional. The watch dog timer can continue running when the system is in deep sleep, and the interrupt will trigger the wake. After the wake, the core can reset the watch dog timer.\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0rstrt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0rstrt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp0rstrt`]
module"]
#[doc(alias = "DSP0RSTRT")]
pub type Dsp0rstrt = crate::Reg<dsp0rstrt::Dsp0rstrtSpec>;
#[doc = "This register will restart the watchdog timer. Writing a special key value into this register will result in the watch dog timer being reset, so that the count will start again. It is expected that the software will periodically write to this register to indicate that the system is functional. The watch dog timer can continue running when the system is in deep sleep, and the interrupt will trigger the wake. After the wake, the core can reset the watch dog timer."]
pub mod dsp0rstrt;
#[doc = "DSP0TLOCK (rw) register accessor: This register locks the watch dog timer. Once it is locked, the configuration register (DSP0CFG) for watch dog timer cannot be written to and the timer is automatically enabled (WDTEN is set).\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0tlock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0tlock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp0tlock`]
module"]
#[doc(alias = "DSP0TLOCK")]
pub type Dsp0tlock = crate::Reg<dsp0tlock::Dsp0tlockSpec>;
#[doc = "This register locks the watch dog timer. Once it is locked, the configuration register (DSP0CFG) for watch dog timer cannot be written to and the timer is automatically enabled (WDTEN is set)."]
pub mod dsp0tlock;
#[doc = "DSP0COUNT (rw) register accessor: This register holds the current count for the watch dog timer. This is a read only register. SW cannot set the value in the counter, but can reset it.\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0count::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0count::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp0count`]
module"]
#[doc(alias = "DSP0COUNT")]
pub type Dsp0count = crate::Reg<dsp0count::Dsp0countSpec>;
#[doc = "This register holds the current count for the watch dog timer. This is a read only register. SW cannot set the value in the counter, but can reset it."]
pub mod dsp0count;
#[doc = "DSP1CFG (rw) register accessor: This is the configuration register for the DSP1 watch dog timer. It controls the enable, interrupt set, clocks for the timer, the compare values for the counters to trigger a reset or interrupt. This register can only be written to if the associated DSP1TLOCK is not set.\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp1cfg`]
module"]
#[doc(alias = "DSP1CFG")]
pub type Dsp1cfg = crate::Reg<dsp1cfg::Dsp1cfgSpec>;
#[doc = "This is the configuration register for the DSP1 watch dog timer. It controls the enable, interrupt set, clocks for the timer, the compare values for the counters to trigger a reset or interrupt. This register can only be written to if the associated DSP1TLOCK is not set."]
pub mod dsp1cfg;
#[doc = "DSP1RSTRT (rw) register accessor: This register will restart the watchdog timer. Writing a special key value into this register will result in the watch dog timer being reset, so that the count will start again. It is expected that the software will periodically write to this register to indicate that the system is functional. The watch dog timer can continue running when the system is in deep sleep, and the interrupt will trigger the wake. After the wake, the core can reset the watch dog timer.\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1rstrt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1rstrt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp1rstrt`]
module"]
#[doc(alias = "DSP1RSTRT")]
pub type Dsp1rstrt = crate::Reg<dsp1rstrt::Dsp1rstrtSpec>;
#[doc = "This register will restart the watchdog timer. Writing a special key value into this register will result in the watch dog timer being reset, so that the count will start again. It is expected that the software will periodically write to this register to indicate that the system is functional. The watch dog timer can continue running when the system is in deep sleep, and the interrupt will trigger the wake. After the wake, the core can reset the watch dog timer."]
pub mod dsp1rstrt;
#[doc = "DSP1TLOCK (rw) register accessor: This register locks the watch dog timer. Once it is locked, the configuration register (DSP1CFG) for watch dog timer cannot be written to and the timer is automatically enabled (WDTEN is set).\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1tlock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1tlock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp1tlock`]
module"]
#[doc(alias = "DSP1TLOCK")]
pub type Dsp1tlock = crate::Reg<dsp1tlock::Dsp1tlockSpec>;
#[doc = "This register locks the watch dog timer. Once it is locked, the configuration register (DSP1CFG) for watch dog timer cannot be written to and the timer is automatically enabled (WDTEN is set)."]
pub mod dsp1tlock;
#[doc = "DSP1COUNT (rw) register accessor: This register holds the current count for the watch dog timer. This is a read only register. SW cannot set the value in the counter, but can reset it.\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1count::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1count::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp1count`]
module"]
#[doc(alias = "DSP1COUNT")]
pub type Dsp1count = crate::Reg<dsp1count::Dsp1countSpec>;
#[doc = "This register holds the current count for the watch dog timer. This is a read only register. SW cannot set the value in the counter, but can reset it."]
pub mod dsp1count;
#[doc = "WDTIEREN (rw) register accessor: Set bits in this register to allow this module to generate the corresponding interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`wdtieren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdtieren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdtieren`]
module"]
#[doc(alias = "WDTIEREN")]
pub type Wdtieren = crate::Reg<wdtieren::WdtierenSpec>;
#[doc = "Set bits in this register to allow this module to generate the corresponding interrupt."]
pub mod wdtieren;
#[doc = "WDTIERSTAT (rw) register accessor: Read bits from this register to discover the cause of a recent interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`wdtierstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdtierstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdtierstat`]
module"]
#[doc(alias = "WDTIERSTAT")]
pub type Wdtierstat = crate::Reg<wdtierstat::WdtierstatSpec>;
#[doc = "Read bits from this register to discover the cause of a recent interrupt."]
pub mod wdtierstat;
#[doc = "WDTIERCLR (rw) register accessor: Write a 1 to a bit in this register to clear the interrupt status associated with that bit.\n\nYou can [`read`](crate::Reg::read) this register and get [`wdtierclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdtierclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdtierclr`]
module"]
#[doc(alias = "WDTIERCLR")]
pub type Wdtierclr = crate::Reg<wdtierclr::WdtierclrSpec>;
#[doc = "Write a 1 to a bit in this register to clear the interrupt status associated with that bit."]
pub mod wdtierclr;
#[doc = "WDTIERSET (rw) register accessor: Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes).\n\nYou can [`read`](crate::Reg::read) this register and get [`wdtierset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdtierset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdtierset`]
module"]
#[doc(alias = "WDTIERSET")]
pub type Wdtierset = crate::Reg<wdtierset::WdtiersetSpec>;
#[doc = "Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes)."]
pub mod wdtierset;
#[doc = "DSP0IEREN (rw) register accessor: Set bits in this register to allow this module to generate the corresponding interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0ieren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0ieren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp0ieren`]
module"]
#[doc(alias = "DSP0IEREN")]
pub type Dsp0ieren = crate::Reg<dsp0ieren::Dsp0ierenSpec>;
#[doc = "Set bits in this register to allow this module to generate the corresponding interrupt."]
pub mod dsp0ieren;
#[doc = "DSP0IERSTAT (rw) register accessor: Read bits from this register to discover the cause of a recent interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0ierstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0ierstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp0ierstat`]
module"]
#[doc(alias = "DSP0IERSTAT")]
pub type Dsp0ierstat = crate::Reg<dsp0ierstat::Dsp0ierstatSpec>;
#[doc = "Read bits from this register to discover the cause of a recent interrupt."]
pub mod dsp0ierstat;
#[doc = "DSP0IERCLR (rw) register accessor: Write a 1 to a bit in this register to clear the interrupt status associated with that bit.\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0ierclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0ierclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp0ierclr`]
module"]
#[doc(alias = "DSP0IERCLR")]
pub type Dsp0ierclr = crate::Reg<dsp0ierclr::Dsp0ierclrSpec>;
#[doc = "Write a 1 to a bit in this register to clear the interrupt status associated with that bit."]
pub mod dsp0ierclr;
#[doc = "DSP0IERSET (rw) register accessor: Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes).\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0ierset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0ierset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp0ierset`]
module"]
#[doc(alias = "DSP0IERSET")]
pub type Dsp0ierset = crate::Reg<dsp0ierset::Dsp0iersetSpec>;
#[doc = "Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes)."]
pub mod dsp0ierset;
#[doc = "DSP1IEREN (rw) register accessor: Set bits in this register to allow this module to generate the corresponding interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1ieren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1ieren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp1ieren`]
module"]
#[doc(alias = "DSP1IEREN")]
pub type Dsp1ieren = crate::Reg<dsp1ieren::Dsp1ierenSpec>;
#[doc = "Set bits in this register to allow this module to generate the corresponding interrupt."]
pub mod dsp1ieren;
#[doc = "DSP1IERSTAT (rw) register accessor: Read bits from this register to discover the cause of a recent interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1ierstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1ierstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp1ierstat`]
module"]
#[doc(alias = "DSP1IERSTAT")]
pub type Dsp1ierstat = crate::Reg<dsp1ierstat::Dsp1ierstatSpec>;
#[doc = "Read bits from this register to discover the cause of a recent interrupt."]
pub mod dsp1ierstat;
#[doc = "DSP1IERCLR (rw) register accessor: Write a 1 to a bit in this register to clear the interrupt status associated with that bit.\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1ierclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1ierclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp1ierclr`]
module"]
#[doc(alias = "DSP1IERCLR")]
pub type Dsp1ierclr = crate::Reg<dsp1ierclr::Dsp1ierclrSpec>;
#[doc = "Write a 1 to a bit in this register to clear the interrupt status associated with that bit."]
pub mod dsp1ierclr;
#[doc = "DSP1IERSET (rw) register accessor: Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes).\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1ierset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1ierset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp1ierset`]
module"]
#[doc(alias = "DSP1IERSET")]
pub type Dsp1ierset = crate::Reg<dsp1ierset::Dsp1iersetSpec>;
#[doc = "Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes)."]
pub mod dsp1ierset;
