#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cfg: Cfg,
    stat: Stat,
    swt: Swt,
    sl0cfg: Sl0cfg,
    sl1cfg: Sl1cfg,
    sl2cfg: Sl2cfg,
    sl3cfg: Sl3cfg,
    sl4cfg: Sl4cfg,
    sl5cfg: Sl5cfg,
    sl6cfg: Sl6cfg,
    sl7cfg: Sl7cfg,
    wulim: Wulim,
    wllim: Wllim,
    scwlim: Scwlim,
    fifo: Fifo,
    fifopr: Fifopr,
    inttrigtimer: Inttrigtimer,
    _reserved17: [u8; 0x1c],
    zxcfg: Zxcfg,
    zxlim: Zxlim,
    gaincfg: Gaincfg,
    gain: Gain,
    _reserved21: [u8; 0x34],
    satcfg: Satcfg,
    satlim: Satlim,
    satmax: Satmax,
    satclr: Satclr,
    _reserved25: [u8; 0x64],
    calctrl: Calctrl,
    _reserved26: [u8; 0xe4],
    inten: Inten,
    intstat: Intstat,
    intclr: Intclr,
    intset: Intset,
    _reserved30: [u8; 0x30],
    dmatrigen: Dmatrigen,
    dmatrigstat: Dmatrigstat,
    _reserved32: [u8; 0x38],
    dmacfg: Dmacfg,
    _reserved33: [u8; 0x04],
    dmatotcount: Dmatotcount,
    dmatargaddr: Dmatargaddr,
    dmastat: Dmastat,
}
impl RegisterBlock {
    #[doc = "0x00 - The ADC Configuration Register contains the software control for selecting the clock frequency used for the SAR conversions, the trigger polarity, the trigger select, the reference voltage select, the low power mode, the operating mode (single scan per trigger vs. repeating mode) and ADC enable."]
    #[inline(always)]
    pub const fn cfg(&self) -> &Cfg {
        &self.cfg
    }
    #[doc = "0x04 - This register indicates the basic power status for the ADC. For detailed power status, see the power control power status register. ADC power mode 0 indicates the ADC is in its full power state and is ready to process scans. ADC Power mode 1 indicates the ADC enabled and in a low power state."]
    #[inline(always)]
    pub const fn stat(&self) -> &Stat {
        &self.stat
    }
    #[doc = "0x08 - This register enables initiating an ADC scan through software."]
    #[inline(always)]
    pub const fn swt(&self) -> &Swt {
        &self.swt
    }
    #[doc = "0x0c - Slot 0 Configuration"]
    #[inline(always)]
    pub const fn sl0cfg(&self) -> &Sl0cfg {
        &self.sl0cfg
    }
    #[doc = "0x10 - Slot 1 Configuration"]
    #[inline(always)]
    pub const fn sl1cfg(&self) -> &Sl1cfg {
        &self.sl1cfg
    }
    #[doc = "0x14 - Slot 2 Configuration"]
    #[inline(always)]
    pub const fn sl2cfg(&self) -> &Sl2cfg {
        &self.sl2cfg
    }
    #[doc = "0x18 - Slot 3 Configuration"]
    #[inline(always)]
    pub const fn sl3cfg(&self) -> &Sl3cfg {
        &self.sl3cfg
    }
    #[doc = "0x1c - Slot 4 Configuration"]
    #[inline(always)]
    pub const fn sl4cfg(&self) -> &Sl4cfg {
        &self.sl4cfg
    }
    #[doc = "0x20 - Slot 5 Configuration"]
    #[inline(always)]
    pub const fn sl5cfg(&self) -> &Sl5cfg {
        &self.sl5cfg
    }
    #[doc = "0x24 - Slot 6 Configuration"]
    #[inline(always)]
    pub const fn sl6cfg(&self) -> &Sl6cfg {
        &self.sl6cfg
    }
    #[doc = "0x28 - Slot 7 Configuration"]
    #[inline(always)]
    pub const fn sl7cfg(&self) -> &Sl7cfg {
        &self.sl7cfg
    }
    #[doc = "0x2c - Window Comparator Upper Limits"]
    #[inline(always)]
    pub const fn wulim(&self) -> &Wulim {
        &self.wulim
    }
    #[doc = "0x30 - Window Comparator Lower Limits"]
    #[inline(always)]
    pub const fn wllim(&self) -> &Wllim {
        &self.wllim
    }
    #[doc = "0x34 - Scale Window Comparator Limits"]
    #[inline(always)]
    pub const fn scwlim(&self) -> &Scwlim {
        &self.scwlim
    }
    #[doc = "0x38 - The ADC FIFO Register contains the slot number and fifo data for the oldest conversion data in the FIFO. The COUNT field indicates the total number of valid entries in the FIFO. A write to this register will pop one of the FIFO entries off the FIFO and decrease the COUNT by 1 if the COUNT is greater than zero."]
    #[inline(always)]
    pub const fn fifo(&self) -> &Fifo {
        &self.fifo
    }
    #[doc = "0x3c - This is a Pop Read mirrored copy of the ADCFIFO register with the only difference being that reading this register will result in a simultaneous FIFO POP which is also achieved by writing to the ADCFIFO Register. Note: The DFIFORDEN bit must be set in the CFG register for the the destructive read to be enabled."]
    #[inline(always)]
    pub const fn fifopr(&self) -> &Fifopr {
        &self.fifopr
    }
    #[doc = "0x40 - ADC-Internal Repeating Trigger Timer Configuration"]
    #[inline(always)]
    pub const fn inttrigtimer(&self) -> &Inttrigtimer {
        &self.inttrigtimer
    }
    #[doc = "0x60 - Zero Crossing Comparator Configuration"]
    #[inline(always)]
    pub const fn zxcfg(&self) -> &Zxcfg {
        &self.zxcfg
    }
    #[doc = "0x64 - Zero Crossing Comparator Limits"]
    #[inline(always)]
    pub const fn zxlim(&self) -> &Zxlim {
        &self.zxlim
    }
    #[doc = "0x68 - PGA Gain Configuration"]
    #[inline(always)]
    pub const fn gaincfg(&self) -> &Gaincfg {
        &self.gaincfg
    }
    #[doc = "0x6c - PGA Gain Codes"]
    #[inline(always)]
    pub const fn gain(&self) -> &Gain {
        &self.gain
    }
    #[doc = "0xa4 - Saturation Comparator Configuration"]
    #[inline(always)]
    pub const fn satcfg(&self) -> &Satcfg {
        &self.satcfg
    }
    #[doc = "0xa8 - Saturation Comparator Limits"]
    #[inline(always)]
    pub const fn satlim(&self) -> &Satlim {
        &self.satlim
    }
    #[doc = "0xac - Saturation Comparator Event Counter Limits"]
    #[inline(always)]
    pub const fn satmax(&self) -> &Satmax {
        &self.satmax
    }
    #[doc = "0xb0 - Clears the saturation event counter registers"]
    #[inline(always)]
    pub const fn satclr(&self) -> &Satclr {
        &self.satclr
    }
    #[doc = "0x118 - Calibration Control"]
    #[inline(always)]
    pub const fn calctrl(&self) -> &Calctrl {
        &self.calctrl
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
    #[doc = "0x240 - DMA Trigger Enable"]
    #[inline(always)]
    pub const fn dmatrigen(&self) -> &Dmatrigen {
        &self.dmatrigen
    }
    #[doc = "0x244 - DMA Trigger Status"]
    #[inline(always)]
    pub const fn dmatrigstat(&self) -> &Dmatrigstat {
        &self.dmatrigstat
    }
    #[doc = "0x280 - DMA Configuration"]
    #[inline(always)]
    pub const fn dmacfg(&self) -> &Dmacfg {
        &self.dmacfg
    }
    #[doc = "0x288 - DMA Total Transfer Count"]
    #[inline(always)]
    pub const fn dmatotcount(&self) -> &Dmatotcount {
        &self.dmatotcount
    }
    #[doc = "0x28c - DMA Target Address"]
    #[inline(always)]
    pub const fn dmatargaddr(&self) -> &Dmatargaddr {
        &self.dmatargaddr
    }
    #[doc = "0x290 - DMA Status"]
    #[inline(always)]
    pub const fn dmastat(&self) -> &Dmastat {
        &self.dmastat
    }
}
#[doc = "CFG (rw) register accessor: The ADC Configuration Register contains the software control for selecting the clock frequency used for the SAR conversions, the trigger polarity, the trigger select, the reference voltage select, the low power mode, the operating mode (single scan per trigger vs. repeating mode) and ADC enable.\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg`]
module"]
#[doc(alias = "CFG")]
pub type Cfg = crate::Reg<cfg::CfgSpec>;
#[doc = "The ADC Configuration Register contains the software control for selecting the clock frequency used for the SAR conversions, the trigger polarity, the trigger select, the reference voltage select, the low power mode, the operating mode (single scan per trigger vs. repeating mode) and ADC enable."]
pub mod cfg;
#[doc = "STAT (rw) register accessor: This register indicates the basic power status for the ADC. For detailed power status, see the power control power status register. ADC power mode 0 indicates the ADC is in its full power state and is ready to process scans. ADC Power mode 1 indicates the ADC enabled and in a low power state.\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat`]
module"]
#[doc(alias = "STAT")]
pub type Stat = crate::Reg<stat::StatSpec>;
#[doc = "This register indicates the basic power status for the ADC. For detailed power status, see the power control power status register. ADC power mode 0 indicates the ADC is in its full power state and is ready to process scans. ADC Power mode 1 indicates the ADC enabled and in a low power state."]
pub mod stat;
#[doc = "SWT (rw) register accessor: This register enables initiating an ADC scan through software.\n\nYou can [`read`](crate::Reg::read) this register and get [`swt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swt`]
module"]
#[doc(alias = "SWT")]
pub type Swt = crate::Reg<swt::SwtSpec>;
#[doc = "This register enables initiating an ADC scan through software."]
pub mod swt;
#[doc = "SL0CFG (rw) register accessor: Slot 0 Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`sl0cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sl0cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sl0cfg`]
module"]
#[doc(alias = "SL0CFG")]
pub type Sl0cfg = crate::Reg<sl0cfg::Sl0cfgSpec>;
#[doc = "Slot 0 Configuration"]
pub mod sl0cfg;
#[doc = "SL1CFG (rw) register accessor: Slot 1 Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`sl1cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sl1cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sl1cfg`]
module"]
#[doc(alias = "SL1CFG")]
pub type Sl1cfg = crate::Reg<sl1cfg::Sl1cfgSpec>;
#[doc = "Slot 1 Configuration"]
pub mod sl1cfg;
#[doc = "SL2CFG (rw) register accessor: Slot 2 Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`sl2cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sl2cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sl2cfg`]
module"]
#[doc(alias = "SL2CFG")]
pub type Sl2cfg = crate::Reg<sl2cfg::Sl2cfgSpec>;
#[doc = "Slot 2 Configuration"]
pub mod sl2cfg;
#[doc = "SL3CFG (rw) register accessor: Slot 3 Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`sl3cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sl3cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sl3cfg`]
module"]
#[doc(alias = "SL3CFG")]
pub type Sl3cfg = crate::Reg<sl3cfg::Sl3cfgSpec>;
#[doc = "Slot 3 Configuration"]
pub mod sl3cfg;
#[doc = "SL4CFG (rw) register accessor: Slot 4 Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`sl4cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sl4cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sl4cfg`]
module"]
#[doc(alias = "SL4CFG")]
pub type Sl4cfg = crate::Reg<sl4cfg::Sl4cfgSpec>;
#[doc = "Slot 4 Configuration"]
pub mod sl4cfg;
#[doc = "SL5CFG (rw) register accessor: Slot 5 Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`sl5cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sl5cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sl5cfg`]
module"]
#[doc(alias = "SL5CFG")]
pub type Sl5cfg = crate::Reg<sl5cfg::Sl5cfgSpec>;
#[doc = "Slot 5 Configuration"]
pub mod sl5cfg;
#[doc = "SL6CFG (rw) register accessor: Slot 6 Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`sl6cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sl6cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sl6cfg`]
module"]
#[doc(alias = "SL6CFG")]
pub type Sl6cfg = crate::Reg<sl6cfg::Sl6cfgSpec>;
#[doc = "Slot 6 Configuration"]
pub mod sl6cfg;
#[doc = "SL7CFG (rw) register accessor: Slot 7 Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`sl7cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sl7cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sl7cfg`]
module"]
#[doc(alias = "SL7CFG")]
pub type Sl7cfg = crate::Reg<sl7cfg::Sl7cfgSpec>;
#[doc = "Slot 7 Configuration"]
pub mod sl7cfg;
#[doc = "WULIM (rw) register accessor: Window Comparator Upper Limits\n\nYou can [`read`](crate::Reg::read) this register and get [`wulim::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wulim::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wulim`]
module"]
#[doc(alias = "WULIM")]
pub type Wulim = crate::Reg<wulim::WulimSpec>;
#[doc = "Window Comparator Upper Limits"]
pub mod wulim;
#[doc = "WLLIM (rw) register accessor: Window Comparator Lower Limits\n\nYou can [`read`](crate::Reg::read) this register and get [`wllim::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wllim::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wllim`]
module"]
#[doc(alias = "WLLIM")]
pub type Wllim = crate::Reg<wllim::WllimSpec>;
#[doc = "Window Comparator Lower Limits"]
pub mod wllim;
#[doc = "SCWLIM (rw) register accessor: Scale Window Comparator Limits\n\nYou can [`read`](crate::Reg::read) this register and get [`scwlim::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scwlim::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scwlim`]
module"]
#[doc(alias = "SCWLIM")]
pub type Scwlim = crate::Reg<scwlim::ScwlimSpec>;
#[doc = "Scale Window Comparator Limits"]
pub mod scwlim;
#[doc = "FIFO (rw) register accessor: The ADC FIFO Register contains the slot number and fifo data for the oldest conversion data in the FIFO. The COUNT field indicates the total number of valid entries in the FIFO. A write to this register will pop one of the FIFO entries off the FIFO and decrease the COUNT by 1 if the COUNT is greater than zero.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo`]
module"]
#[doc(alias = "FIFO")]
pub type Fifo = crate::Reg<fifo::FifoSpec>;
#[doc = "The ADC FIFO Register contains the slot number and fifo data for the oldest conversion data in the FIFO. The COUNT field indicates the total number of valid entries in the FIFO. A write to this register will pop one of the FIFO entries off the FIFO and decrease the COUNT by 1 if the COUNT is greater than zero."]
pub mod fifo;
#[doc = "FIFOPR (rw) register accessor: This is a Pop Read mirrored copy of the ADCFIFO register with the only difference being that reading this register will result in a simultaneous FIFO POP which is also achieved by writing to the ADCFIFO Register. Note: The DFIFORDEN bit must be set in the CFG register for the the destructive read to be enabled.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifopr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifopr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifopr`]
module"]
#[doc(alias = "FIFOPR")]
pub type Fifopr = crate::Reg<fifopr::FifoprSpec>;
#[doc = "This is a Pop Read mirrored copy of the ADCFIFO register with the only difference being that reading this register will result in a simultaneous FIFO POP which is also achieved by writing to the ADCFIFO Register. Note: The DFIFORDEN bit must be set in the CFG register for the the destructive read to be enabled."]
pub mod fifopr;
#[doc = "INTTRIGTIMER (rw) register accessor: ADC-Internal Repeating Trigger Timer Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`inttrigtimer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inttrigtimer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inttrigtimer`]
module"]
#[doc(alias = "INTTRIGTIMER")]
pub type Inttrigtimer = crate::Reg<inttrigtimer::InttrigtimerSpec>;
#[doc = "ADC-Internal Repeating Trigger Timer Configuration"]
pub mod inttrigtimer;
#[doc = "ZXCFG (rw) register accessor: Zero Crossing Comparator Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`zxcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`zxcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@zxcfg`]
module"]
#[doc(alias = "ZXCFG")]
pub type Zxcfg = crate::Reg<zxcfg::ZxcfgSpec>;
#[doc = "Zero Crossing Comparator Configuration"]
pub mod zxcfg;
#[doc = "ZXLIM (rw) register accessor: Zero Crossing Comparator Limits\n\nYou can [`read`](crate::Reg::read) this register and get [`zxlim::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`zxlim::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@zxlim`]
module"]
#[doc(alias = "ZXLIM")]
pub type Zxlim = crate::Reg<zxlim::ZxlimSpec>;
#[doc = "Zero Crossing Comparator Limits"]
pub mod zxlim;
#[doc = "GAINCFG (rw) register accessor: PGA Gain Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`gaincfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gaincfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gaincfg`]
module"]
#[doc(alias = "GAINCFG")]
pub type Gaincfg = crate::Reg<gaincfg::GaincfgSpec>;
#[doc = "PGA Gain Configuration"]
pub mod gaincfg;
#[doc = "GAIN (rw) register accessor: PGA Gain Codes\n\nYou can [`read`](crate::Reg::read) this register and get [`gain::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gain::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gain`]
module"]
#[doc(alias = "GAIN")]
pub type Gain = crate::Reg<gain::GainSpec>;
#[doc = "PGA Gain Codes"]
pub mod gain;
#[doc = "SATCFG (rw) register accessor: Saturation Comparator Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`satcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`satcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@satcfg`]
module"]
#[doc(alias = "SATCFG")]
pub type Satcfg = crate::Reg<satcfg::SatcfgSpec>;
#[doc = "Saturation Comparator Configuration"]
pub mod satcfg;
#[doc = "SATLIM (rw) register accessor: Saturation Comparator Limits\n\nYou can [`read`](crate::Reg::read) this register and get [`satlim::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`satlim::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@satlim`]
module"]
#[doc(alias = "SATLIM")]
pub type Satlim = crate::Reg<satlim::SatlimSpec>;
#[doc = "Saturation Comparator Limits"]
pub mod satlim;
#[doc = "SATMAX (rw) register accessor: Saturation Comparator Event Counter Limits\n\nYou can [`read`](crate::Reg::read) this register and get [`satmax::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`satmax::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@satmax`]
module"]
#[doc(alias = "SATMAX")]
pub type Satmax = crate::Reg<satmax::SatmaxSpec>;
#[doc = "Saturation Comparator Event Counter Limits"]
pub mod satmax;
#[doc = "SATCLR (rw) register accessor: Clears the saturation event counter registers\n\nYou can [`read`](crate::Reg::read) this register and get [`satclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`satclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@satclr`]
module"]
#[doc(alias = "SATCLR")]
pub type Satclr = crate::Reg<satclr::SatclrSpec>;
#[doc = "Clears the saturation event counter registers"]
pub mod satclr;
#[doc = "CALCTRL (rw) register accessor: Calibration Control\n\nYou can [`read`](crate::Reg::read) this register and get [`calctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@calctrl`]
module"]
#[doc(alias = "CALCTRL")]
pub type Calctrl = crate::Reg<calctrl::CalctrlSpec>;
#[doc = "Calibration Control"]
pub mod calctrl;
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
#[doc = "DMATOTCOUNT (rw) register accessor: DMA Total Transfer Count\n\nYou can [`read`](crate::Reg::read) this register and get [`dmatotcount::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmatotcount::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmatotcount`]
module"]
#[doc(alias = "DMATOTCOUNT")]
pub type Dmatotcount = crate::Reg<dmatotcount::DmatotcountSpec>;
#[doc = "DMA Total Transfer Count"]
pub mod dmatotcount;
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
