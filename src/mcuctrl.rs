#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    chippn: Chippn,
    chipid0: Chipid0,
    chipid1: Chipid1,
    chiprev: Chiprev,
    vendorid: Vendorid,
    sku: Sku,
    _reserved6: [u8; 0x08],
    debugger: Debugger,
    _reserved7: [u8; 0x04],
    acrg: Acrg,
    _reserved8: [u8; 0x18],
    vrefgen2: Vrefgen2,
    _reserved9: [u8; 0x18],
    vrctrl: Vrctrl,
    _reserved10: [u8; 0x1c],
    ldoreg1: Ldoreg1,
    _reserved11: [u8; 0x04],
    ldoreg2: Ldoreg2,
    _reserved12: [u8; 0x38],
    hfrc2: Hfrc2,
    _reserved13: [u8; 0x18],
    lfrc: Lfrc,
    _reserved14: [u8; 0x1c],
    bodctrl: Bodctrl,
    adcpwrdly: Adcpwrdly,
    adcpwrctrl: Adcpwrctrl,
    adccal: Adccal,
    adcbattload: Adcbattload,
    _reserved19: [u8; 0x0c],
    xtalctrl: Xtalctrl,
    xtalgenctrl: Xtalgenctrl,
    xtalhstrims: Xtalhstrims,
    xtalhsctrl: Xtalhsctrl,
    _reserved23: [u8; 0x50],
    mrampwrctrl: Mrampwrctrl,
    _reserved24: [u8; 0x28],
    bodisable: Bodisable,
    d2aspare: D2aspare,
    _reserved26: [u8; 0x04],
    bootloader: Bootloader,
    shadowvalid: Shadowvalid,
    scratch0: Scratch0,
    scratch1: Scratch1,
    _reserved30: [u8; 0x38],
    dbgr1: Dbgr1,
    dbgr2: Dbgr2,
    _reserved32: [u8; 0x18],
    pmuenable: Pmuenable,
    _reserved33: [u8; 0x2c],
    dbgctrl: Dbgctrl,
    _reserved34: [u8; 0x10],
    otapointer: Otapointer,
    _reserved35: [u8; 0x18],
    apbdmactrl: Apbdmactrl,
    _reserved36: [u8; 0xb4],
    kextclksel: Kextclksel,
    simobuck0: Simobuck0,
    simobuck1: Simobuck1,
    simobuck2: Simobuck2,
    simobuck3: Simobuck3,
    _reserved41: [u8; 0x08],
    simobuck6: Simobuck6,
    simobuck7: Simobuck7,
    simobuck8: Simobuck8,
    simobuck9: Simobuck9,
    _reserved45: [u8; 0x08],
    simobuck12: Simobuck12,
    simobuck13: Simobuck13,
    _reserved47: [u8; 0x04],
    simobuck15: Simobuck15,
    pwrsw0: Pwrsw0,
    pwrsw1: Pwrsw1,
    _reserved50: [u8; 0x04],
    usbrstctrl: Usbrstctrl,
    _reserved51: [u8; 0x1c],
    flashwprot0: Flashwprot0,
    flashwprot1: Flashwprot1,
    flashwprot2: Flashwprot2,
    flashwprot3: Flashwprot3,
    flashrprot0: Flashrprot0,
    flashrprot1: Flashrprot1,
    flashrprot2: Flashrprot2,
    flashrprot3: Flashrprot3,
    dmasramwprot0: Dmasramwprot0,
    dmasramwprot1: Dmasramwprot1,
    dmasramrprot0: Dmasramrprot0,
    dmasramrprot1: Dmasramrprot1,
    _reserved63: [u8; 0x40],
    usbphyreset: Usbphyreset,
    _reserved64: [u8; 0x10],
    audadcpwrctrl: Audadcpwrctrl,
    audio1: Audio1,
    _reserved66: [u8; 0x04],
    pgaadcifctrl: Pgaadcifctrl,
    pgactrl1: Pgactrl1,
    pgactrl2: Pgactrl2,
    audadcpwrdly: Audadcpwrdly,
    _reserved70: [u8; 0x08],
    sdioctrl: Sdioctrl,
    pdmctrl: Pdmctrl,
}
impl RegisterBlock {
    #[doc = "0x00 - Chip Information"]
    #[inline(always)]
    pub const fn chippn(&self) -> &Chippn {
        &self.chippn
    }
    #[doc = "0x04 - Unique Chip ID 0"]
    #[inline(always)]
    pub const fn chipid0(&self) -> &Chipid0 {
        &self.chipid0
    }
    #[doc = "0x08 - Unique Chip ID 1"]
    #[inline(always)]
    pub const fn chipid1(&self) -> &Chipid1 {
        &self.chipid1
    }
    #[doc = "0x0c - Chip Revision"]
    #[inline(always)]
    pub const fn chiprev(&self) -> &Chiprev {
        &self.chiprev
    }
    #[doc = "0x10 - Unique Vendor ID"]
    #[inline(always)]
    pub const fn vendorid(&self) -> &Vendorid {
        &self.vendorid
    }
    #[doc = "0x14 - Unique Chip SKU"]
    #[inline(always)]
    pub const fn sku(&self) -> &Sku {
        &self.sku
    }
    #[doc = "0x20 - Debugger Control"]
    #[inline(always)]
    pub const fn debugger(&self) -> &Debugger {
        &self.debugger
    }
    #[doc = "0x28 - Active Current Reference Generator Control"]
    #[inline(always)]
    pub const fn acrg(&self) -> &Acrg {
        &self.acrg
    }
    #[doc = "0x44 - Voltage Reference Generator 2 Control"]
    #[inline(always)]
    pub const fn vrefgen2(&self) -> &Vrefgen2 {
        &self.vrefgen2
    }
    #[doc = "0x60 - Overrides for Voltage Regulators Controls"]
    #[inline(always)]
    pub const fn vrctrl(&self) -> &Vrctrl {
        &self.vrctrl
    }
    #[doc = "0x80 - CORELDO trims Reg"]
    #[inline(always)]
    pub const fn ldoreg1(&self) -> &Ldoreg1 {
        &self.ldoreg1
    }
    #[doc = "0x88 - MEMLDO and MEMLPLDO Trims"]
    #[inline(always)]
    pub const fn ldoreg2(&self) -> &Ldoreg2 {
        &self.ldoreg2
    }
    #[doc = "0xc4 - HFRC2 Control"]
    #[inline(always)]
    pub const fn hfrc2(&self) -> &Hfrc2 {
        &self.hfrc2
    }
    #[doc = "0xe0 - LFRC Control"]
    #[inline(always)]
    pub const fn lfrc(&self) -> &Lfrc {
        &self.lfrc
    }
    #[doc = "0x100 - BOD control"]
    #[inline(always)]
    pub const fn bodctrl(&self) -> &Bodctrl {
        &self.bodctrl
    }
    #[doc = "0x104 - ADC Power Up Delay Control"]
    #[inline(always)]
    pub const fn adcpwrdly(&self) -> &Adcpwrdly {
        &self.adcpwrdly
    }
    #[doc = "0x108 - ADC Power Control"]
    #[inline(always)]
    pub const fn adcpwrctrl(&self) -> &Adcpwrctrl {
        &self.adcpwrctrl
    }
    #[doc = "0x10c - ADC Calibration Control"]
    #[inline(always)]
    pub const fn adccal(&self) -> &Adccal {
        &self.adccal
    }
    #[doc = "0x110 - ADC Battery Load Enable"]
    #[inline(always)]
    pub const fn adcbattload(&self) -> &Adcbattload {
        &self.adcbattload
    }
    #[doc = "0x120 - XTAL Oscillator Control"]
    #[inline(always)]
    pub const fn xtalctrl(&self) -> &Xtalctrl {
        &self.xtalctrl
    }
    #[doc = "0x124 - XTAL Oscillator General Control"]
    #[inline(always)]
    pub const fn xtalgenctrl(&self) -> &Xtalgenctrl {
        &self.xtalgenctrl
    }
    #[doc = "0x128 - XTALHS Trims"]
    #[inline(always)]
    pub const fn xtalhstrims(&self) -> &Xtalhstrims {
        &self.xtalhstrims
    }
    #[doc = "0x12c - XTALHS Control"]
    #[inline(always)]
    pub const fn xtalhsctrl(&self) -> &Xtalhsctrl {
        &self.xtalhsctrl
    }
    #[doc = "0x180 - MRAM Power Control"]
    #[inline(always)]
    pub const fn mrampwrctrl(&self) -> &Mrampwrctrl {
        &self.mrampwrctrl
    }
    #[doc = "0x1ac - Brownout Disable"]
    #[inline(always)]
    pub const fn bodisable(&self) -> &Bodisable {
        &self.bodisable
    }
    #[doc = "0x1b0 - Spare registers to analog module"]
    #[inline(always)]
    pub const fn d2aspare(&self) -> &D2aspare {
        &self.d2aspare
    }
    #[doc = "0x1b8 - Bootloader and secure boot functions"]
    #[inline(always)]
    pub const fn bootloader(&self) -> &Bootloader {
        &self.bootloader
    }
    #[doc = "0x1bc - Register to indicate whether the shadow registers have been successfully loaded from the Flash Information Space."]
    #[inline(always)]
    pub const fn shadowvalid(&self) -> &Shadowvalid {
        &self.shadowvalid
    }
    #[doc = "0x1c0 - Scratch register that is not reset by any reset"]
    #[inline(always)]
    pub const fn scratch0(&self) -> &Scratch0 {
        &self.scratch0
    }
    #[doc = "0x1c4 - Scratch register that is not reset by any reset"]
    #[inline(always)]
    pub const fn scratch1(&self) -> &Scratch1 {
        &self.scratch1
    }
    #[doc = "0x200 - Read-only debug 1"]
    #[inline(always)]
    pub const fn dbgr1(&self) -> &Dbgr1 {
        &self.dbgr1
    }
    #[doc = "0x204 - Read-only debug 2"]
    #[inline(always)]
    pub const fn dbgr2(&self) -> &Dbgr2 {
        &self.dbgr2
    }
    #[doc = "0x220 - Control bit to enable/disable the PMU"]
    #[inline(always)]
    pub const fn pmuenable(&self) -> &Pmuenable {
        &self.pmuenable
    }
    #[doc = "0x250 - Debug subsystem Control. Determines the debug components enable and clk frequency."]
    #[inline(always)]
    pub const fn dbgctrl(&self) -> &Dbgctrl {
        &self.dbgctrl
    }
    #[doc = "0x264 - OTA (Over the Air) Update Pointer/Status. Reset only by POA"]
    #[inline(always)]
    pub const fn otapointer(&self) -> &Otapointer {
        &self.otapointer
    }
    #[doc = "0x280 - DMA Control Register. Determines misc settings for DMA operation"]
    #[inline(always)]
    pub const fn apbdmactrl(&self) -> &Apbdmactrl {
        &self.apbdmactrl
    }
    #[doc = "0x338 - Locks the state of the EXTCLKSEL register from writes. This is done to prevent errant writes to the register, as this could cause the chip to halt. Write a value of 0x53 to unlock write access to the EXTCLKSEL register. Once unlocked, the register will read back a 1 to undicate this is unlocked. Writing the register with any other value other than 0x53 will enable the lock."]
    #[inline(always)]
    pub const fn kextclksel(&self) -> &Kextclksel {
        &self.kextclksel
    }
    #[doc = "0x33c - This WRITE_ONLY register controls various buck parameters. It will read back as 0x00000000."]
    #[inline(always)]
    pub const fn simobuck0(&self) -> &Simobuck0 {
        &self.simobuck0
    }
    #[doc = "0x340 - 1. Control the even division of 3 clocks: refresh, low power and TONCLK. 2. Control gap bewteen secondary switches. 3. Debug features: control the amount of time TONCLK is on, and the time before snubber asserts for each buck sequence. 4. Enable or disable the observation bus. 5. Select the buck sequence operation mode. 6. Control delay between primary Pmos and Nmos transitions."]
    #[inline(always)]
    pub const fn simobuck1(&self) -> &Simobuck1 {
        &self.simobuck1
    }
    #[doc = "0x344 - SIMO Buck Muxed VDDC Active Sequence Trim Control"]
    #[inline(always)]
    pub const fn simobuck2(&self) -> &Simobuck2 {
        &self.simobuck2
    }
    #[doc = "0x348 - SIMO Buck Muxed VDDC low power Sequence Trim Control"]
    #[inline(always)]
    pub const fn simobuck3(&self) -> &Simobuck3 {
        &self.simobuck3
    }
    #[doc = "0x354 - SIMO Buck Muxed VDDF Active Sequence Trim Control"]
    #[inline(always)]
    pub const fn simobuck6(&self) -> &Simobuck6 {
        &self.simobuck6
    }
    #[doc = "0x358 - SIMO Buck Muxed VDDF active Sequence Trim Control"]
    #[inline(always)]
    pub const fn simobuck7(&self) -> &Simobuck7 {
        &self.simobuck7
    }
    #[doc = "0x35c - SIMO Buck Muxed VDDF Low Power Sequence Trim Control"]
    #[inline(always)]
    pub const fn simobuck8(&self) -> &Simobuck8 {
        &self.simobuck8
    }
    #[doc = "0x360 - SIMO Buck Muxed VDDS Active Sequence Trim Control"]
    #[inline(always)]
    pub const fn simobuck9(&self) -> &Simobuck9 {
        &self.simobuck9
    }
    #[doc = "0x36c - SIMO Buck Compare, Brown out, Active, Low power Trim Control"]
    #[inline(always)]
    pub const fn simobuck12(&self) -> &Simobuck12 {
        &self.simobuck12
    }
    #[doc = "0x370 - SIMO Buck Compare, Brown out, Active, Low power Trim Control"]
    #[inline(always)]
    pub const fn simobuck13(&self) -> &Simobuck13 {
        &self.simobuck13
    }
    #[doc = "0x378 - SIMO Buck Compare, Brown out, Active and Low power Trim Control"]
    #[inline(always)]
    pub const fn simobuck15(&self) -> &Simobuck15 {
        &self.simobuck15
    }
    #[doc = "0x37c - PWRSW Control 0"]
    #[inline(always)]
    pub const fn pwrsw0(&self) -> &Pwrsw0 {
        &self.pwrsw0
    }
    #[doc = "0x380 - PWRSW Control 1"]
    #[inline(always)]
    pub const fn pwrsw1(&self) -> &Pwrsw1 {
        &self.pwrsw1
    }
    #[doc = "0x388 - USB Reset Startup Control"]
    #[inline(always)]
    pub const fn usbrstctrl(&self) -> &Usbrstctrl {
        &self.usbrstctrl
    }
    #[doc = "0x3a8 - These bits write-protect flash in 16KB chunks."]
    #[inline(always)]
    pub const fn flashwprot0(&self) -> &Flashwprot0 {
        &self.flashwprot0
    }
    #[doc = "0x3ac - These bits write-protect flash in 16KB chunks."]
    #[inline(always)]
    pub const fn flashwprot1(&self) -> &Flashwprot1 {
        &self.flashwprot1
    }
    #[doc = "0x3b0 - These bits write-protect flash in 16KB chunks."]
    #[inline(always)]
    pub const fn flashwprot2(&self) -> &Flashwprot2 {
        &self.flashwprot2
    }
    #[doc = "0x3b4 - These bits write-protect flash in 16KB chunks."]
    #[inline(always)]
    pub const fn flashwprot3(&self) -> &Flashwprot3 {
        &self.flashwprot3
    }
    #[doc = "0x3b8 - These bits read-protect flash in 16KB chunks."]
    #[inline(always)]
    pub const fn flashrprot0(&self) -> &Flashrprot0 {
        &self.flashrprot0
    }
    #[doc = "0x3bc - These bits read-protect flash in 16KB chunks."]
    #[inline(always)]
    pub const fn flashrprot1(&self) -> &Flashrprot1 {
        &self.flashrprot1
    }
    #[doc = "0x3c0 - These bits read-protect flash in 16KB chunks."]
    #[inline(always)]
    pub const fn flashrprot2(&self) -> &Flashrprot2 {
        &self.flashrprot2
    }
    #[doc = "0x3c4 - These bits read-protect flash in 16KB chunks."]
    #[inline(always)]
    pub const fn flashrprot3(&self) -> &Flashrprot3 {
        &self.flashrprot3
    }
    #[doc = "0x3c8 - These bits write-protect system SRAM from DMA operations in 8KB chunks."]
    #[inline(always)]
    pub const fn dmasramwprot0(&self) -> &Dmasramwprot0 {
        &self.dmasramwprot0
    }
    #[doc = "0x3cc - These bits write-protect system SRAM from DMA operations in 8KB chunks."]
    #[inline(always)]
    pub const fn dmasramwprot1(&self) -> &Dmasramwprot1 {
        &self.dmasramwprot1
    }
    #[doc = "0x3d0 - These bits read-protect system SRAM from DMA operations in 8KB chunks."]
    #[inline(always)]
    pub const fn dmasramrprot0(&self) -> &Dmasramrprot0 {
        &self.dmasramrprot0
    }
    #[doc = "0x3d4 - These bits read-protect system SRAM from DMA operations in 8KB chunks."]
    #[inline(always)]
    pub const fn dmasramrprot1(&self) -> &Dmasramrprot1 {
        &self.dmasramrprot1
    }
    #[doc = "0x418 - DSP0 CACHE RAM TRIM"]
    #[inline(always)]
    pub const fn usbphyreset(&self) -> &Usbphyreset {
        &self.usbphyreset
    }
    #[doc = "0x42c - Audio ADC Power Control"]
    #[inline(always)]
    pub const fn audadcpwrctrl(&self) -> &Audadcpwrctrl {
        &self.audadcpwrctrl
    }
    #[doc = "0x430 - Audio trims 1"]
    #[inline(always)]
    pub const fn audio1(&self) -> &Audio1 {
        &self.audio1
    }
    #[doc = "0x438 - PGA ADCIF control"]
    #[inline(always)]
    pub const fn pgaadcifctrl(&self) -> &Pgaadcifctrl {
        &self.pgaadcifctrl
    }
    #[doc = "0x43c - PGA control 1"]
    #[inline(always)]
    pub const fn pgactrl1(&self) -> &Pgactrl1 {
        &self.pgactrl1
    }
    #[doc = "0x440 - PGA control 2"]
    #[inline(always)]
    pub const fn pgactrl2(&self) -> &Pgactrl2 {
        &self.pgactrl2
    }
    #[doc = "0x444 - Audio ADC Power Up Delay Control"]
    #[inline(always)]
    pub const fn audadcpwrdly(&self) -> &Audadcpwrdly {
        &self.audadcpwrdly
    }
    #[doc = "0x450 - SDIO/eMMC Control"]
    #[inline(always)]
    pub const fn sdioctrl(&self) -> &Sdioctrl {
        &self.sdioctrl
    }
    #[doc = "0x454 - PDM Control"]
    #[inline(always)]
    pub const fn pdmctrl(&self) -> &Pdmctrl {
        &self.pdmctrl
    }
}
#[doc = "CHIPPN (rw) register accessor: Chip Information\n\nYou can [`read`](crate::Reg::read) this register and get [`chippn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chippn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chippn`]
module"]
#[doc(alias = "CHIPPN")]
pub type Chippn = crate::Reg<chippn::ChippnSpec>;
#[doc = "Chip Information"]
pub mod chippn;
#[doc = "CHIPID0 (rw) register accessor: Unique Chip ID 0\n\nYou can [`read`](crate::Reg::read) this register and get [`chipid0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chipid0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chipid0`]
module"]
#[doc(alias = "CHIPID0")]
pub type Chipid0 = crate::Reg<chipid0::Chipid0Spec>;
#[doc = "Unique Chip ID 0"]
pub mod chipid0;
#[doc = "CHIPID1 (rw) register accessor: Unique Chip ID 1\n\nYou can [`read`](crate::Reg::read) this register and get [`chipid1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chipid1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chipid1`]
module"]
#[doc(alias = "CHIPID1")]
pub type Chipid1 = crate::Reg<chipid1::Chipid1Spec>;
#[doc = "Unique Chip ID 1"]
pub mod chipid1;
#[doc = "CHIPREV (rw) register accessor: Chip Revision\n\nYou can [`read`](crate::Reg::read) this register and get [`chiprev::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chiprev::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chiprev`]
module"]
#[doc(alias = "CHIPREV")]
pub type Chiprev = crate::Reg<chiprev::ChiprevSpec>;
#[doc = "Chip Revision"]
pub mod chiprev;
#[doc = "VENDORID (rw) register accessor: Unique Vendor ID\n\nYou can [`read`](crate::Reg::read) this register and get [`vendorid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vendorid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vendorid`]
module"]
#[doc(alias = "VENDORID")]
pub type Vendorid = crate::Reg<vendorid::VendoridSpec>;
#[doc = "Unique Vendor ID"]
pub mod vendorid;
#[doc = "SKU (rw) register accessor: Unique Chip SKU\n\nYou can [`read`](crate::Reg::read) this register and get [`sku::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sku::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sku`]
module"]
#[doc(alias = "SKU")]
pub type Sku = crate::Reg<sku::SkuSpec>;
#[doc = "Unique Chip SKU"]
pub mod sku;
#[doc = "DEBUGGER (rw) register accessor: Debugger Control\n\nYou can [`read`](crate::Reg::read) this register and get [`debugger::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debugger::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debugger`]
module"]
#[doc(alias = "DEBUGGER")]
pub type Debugger = crate::Reg<debugger::DebuggerSpec>;
#[doc = "Debugger Control"]
pub mod debugger;
#[doc = "ACRG (rw) register accessor: Active Current Reference Generator Control\n\nYou can [`read`](crate::Reg::read) this register and get [`acrg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acrg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acrg`]
module"]
#[doc(alias = "ACRG")]
pub type Acrg = crate::Reg<acrg::AcrgSpec>;
#[doc = "Active Current Reference Generator Control"]
pub mod acrg;
#[doc = "VREFGEN2 (rw) register accessor: Voltage Reference Generator 2 Control\n\nYou can [`read`](crate::Reg::read) this register and get [`vrefgen2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vrefgen2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vrefgen2`]
module"]
#[doc(alias = "VREFGEN2")]
pub type Vrefgen2 = crate::Reg<vrefgen2::Vrefgen2Spec>;
#[doc = "Voltage Reference Generator 2 Control"]
pub mod vrefgen2;
#[doc = "VRCTRL (rw) register accessor: Overrides for Voltage Regulators Controls\n\nYou can [`read`](crate::Reg::read) this register and get [`vrctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vrctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vrctrl`]
module"]
#[doc(alias = "VRCTRL")]
pub type Vrctrl = crate::Reg<vrctrl::VrctrlSpec>;
#[doc = "Overrides for Voltage Regulators Controls"]
pub mod vrctrl;
#[doc = "LDOREG1 (rw) register accessor: CORELDO trims Reg\n\nYou can [`read`](crate::Reg::read) this register and get [`ldoreg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ldoreg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ldoreg1`]
module"]
#[doc(alias = "LDOREG1")]
pub type Ldoreg1 = crate::Reg<ldoreg1::Ldoreg1Spec>;
#[doc = "CORELDO trims Reg"]
pub mod ldoreg1;
#[doc = "LDOREG2 (rw) register accessor: MEMLDO and MEMLPLDO Trims\n\nYou can [`read`](crate::Reg::read) this register and get [`ldoreg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ldoreg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ldoreg2`]
module"]
#[doc(alias = "LDOREG2")]
pub type Ldoreg2 = crate::Reg<ldoreg2::Ldoreg2Spec>;
#[doc = "MEMLDO and MEMLPLDO Trims"]
pub mod ldoreg2;
#[doc = "HFRC2 (rw) register accessor: HFRC2 Control\n\nYou can [`read`](crate::Reg::read) this register and get [`hfrc2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfrc2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfrc2`]
module"]
#[doc(alias = "HFRC2")]
pub type Hfrc2 = crate::Reg<hfrc2::Hfrc2Spec>;
#[doc = "HFRC2 Control"]
pub mod hfrc2;
#[doc = "LFRC (rw) register accessor: LFRC Control\n\nYou can [`read`](crate::Reg::read) this register and get [`lfrc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfrc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfrc`]
module"]
#[doc(alias = "LFRC")]
pub type Lfrc = crate::Reg<lfrc::LfrcSpec>;
#[doc = "LFRC Control"]
pub mod lfrc;
#[doc = "BODCTRL (rw) register accessor: BOD control\n\nYou can [`read`](crate::Reg::read) this register and get [`bodctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bodctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bodctrl`]
module"]
#[doc(alias = "BODCTRL")]
pub type Bodctrl = crate::Reg<bodctrl::BodctrlSpec>;
#[doc = "BOD control"]
pub mod bodctrl;
#[doc = "ADCPWRDLY (rw) register accessor: ADC Power Up Delay Control\n\nYou can [`read`](crate::Reg::read) this register and get [`adcpwrdly::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcpwrdly::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcpwrdly`]
module"]
#[doc(alias = "ADCPWRDLY")]
pub type Adcpwrdly = crate::Reg<adcpwrdly::AdcpwrdlySpec>;
#[doc = "ADC Power Up Delay Control"]
pub mod adcpwrdly;
#[doc = "ADCPWRCTRL (rw) register accessor: ADC Power Control\n\nYou can [`read`](crate::Reg::read) this register and get [`adcpwrctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcpwrctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcpwrctrl`]
module"]
#[doc(alias = "ADCPWRCTRL")]
pub type Adcpwrctrl = crate::Reg<adcpwrctrl::AdcpwrctrlSpec>;
#[doc = "ADC Power Control"]
pub mod adcpwrctrl;
#[doc = "ADCCAL (rw) register accessor: ADC Calibration Control\n\nYou can [`read`](crate::Reg::read) this register and get [`adccal::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adccal::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adccal`]
module"]
#[doc(alias = "ADCCAL")]
pub type Adccal = crate::Reg<adccal::AdccalSpec>;
#[doc = "ADC Calibration Control"]
pub mod adccal;
#[doc = "ADCBATTLOAD (rw) register accessor: ADC Battery Load Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`adcbattload::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcbattload::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcbattload`]
module"]
#[doc(alias = "ADCBATTLOAD")]
pub type Adcbattload = crate::Reg<adcbattload::AdcbattloadSpec>;
#[doc = "ADC Battery Load Enable"]
pub mod adcbattload;
#[doc = "XTALCTRL (rw) register accessor: XTAL Oscillator Control\n\nYou can [`read`](crate::Reg::read) this register and get [`xtalctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xtalctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xtalctrl`]
module"]
#[doc(alias = "XTALCTRL")]
pub type Xtalctrl = crate::Reg<xtalctrl::XtalctrlSpec>;
#[doc = "XTAL Oscillator Control"]
pub mod xtalctrl;
#[doc = "XTALGENCTRL (rw) register accessor: XTAL Oscillator General Control\n\nYou can [`read`](crate::Reg::read) this register and get [`xtalgenctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xtalgenctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xtalgenctrl`]
module"]
#[doc(alias = "XTALGENCTRL")]
pub type Xtalgenctrl = crate::Reg<xtalgenctrl::XtalgenctrlSpec>;
#[doc = "XTAL Oscillator General Control"]
pub mod xtalgenctrl;
#[doc = "XTALHSTRIMS (rw) register accessor: XTALHS Trims\n\nYou can [`read`](crate::Reg::read) this register and get [`xtalhstrims::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xtalhstrims::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xtalhstrims`]
module"]
#[doc(alias = "XTALHSTRIMS")]
pub type Xtalhstrims = crate::Reg<xtalhstrims::XtalhstrimsSpec>;
#[doc = "XTALHS Trims"]
pub mod xtalhstrims;
#[doc = "XTALHSCTRL (rw) register accessor: XTALHS Control\n\nYou can [`read`](crate::Reg::read) this register and get [`xtalhsctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xtalhsctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xtalhsctrl`]
module"]
#[doc(alias = "XTALHSCTRL")]
pub type Xtalhsctrl = crate::Reg<xtalhsctrl::XtalhsctrlSpec>;
#[doc = "XTALHS Control"]
pub mod xtalhsctrl;
#[doc = "MRAMPWRCTRL (rw) register accessor: MRAM Power Control\n\nYou can [`read`](crate::Reg::read) this register and get [`mrampwrctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mrampwrctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mrampwrctrl`]
module"]
#[doc(alias = "MRAMPWRCTRL")]
pub type Mrampwrctrl = crate::Reg<mrampwrctrl::MrampwrctrlSpec>;
#[doc = "MRAM Power Control"]
pub mod mrampwrctrl;
#[doc = "BODISABLE (rw) register accessor: Brownout Disable\n\nYou can [`read`](crate::Reg::read) this register and get [`bodisable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bodisable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bodisable`]
module"]
#[doc(alias = "BODISABLE")]
pub type Bodisable = crate::Reg<bodisable::BodisableSpec>;
#[doc = "Brownout Disable"]
pub mod bodisable;
#[doc = "D2ASPARE (rw) register accessor: Spare registers to analog module\n\nYou can [`read`](crate::Reg::read) this register and get [`d2aspare::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d2aspare::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d2aspare`]
module"]
#[doc(alias = "D2ASPARE")]
pub type D2aspare = crate::Reg<d2aspare::D2aspareSpec>;
#[doc = "Spare registers to analog module"]
pub mod d2aspare;
#[doc = "BOOTLOADER (rw) register accessor: Bootloader and secure boot functions\n\nYou can [`read`](crate::Reg::read) this register and get [`bootloader::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootloader::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bootloader`]
module"]
#[doc(alias = "BOOTLOADER")]
pub type Bootloader = crate::Reg<bootloader::BootloaderSpec>;
#[doc = "Bootloader and secure boot functions"]
pub mod bootloader;
#[doc = "SHADOWVALID (rw) register accessor: Register to indicate whether the shadow registers have been successfully loaded from the Flash Information Space.\n\nYou can [`read`](crate::Reg::read) this register and get [`shadowvalid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shadowvalid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shadowvalid`]
module"]
#[doc(alias = "SHADOWVALID")]
pub type Shadowvalid = crate::Reg<shadowvalid::ShadowvalidSpec>;
#[doc = "Register to indicate whether the shadow registers have been successfully loaded from the Flash Information Space."]
pub mod shadowvalid;
#[doc = "SCRATCH0 (rw) register accessor: Scratch register that is not reset by any reset\n\nYou can [`read`](crate::Reg::read) this register and get [`scratch0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scratch0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scratch0`]
module"]
#[doc(alias = "SCRATCH0")]
pub type Scratch0 = crate::Reg<scratch0::Scratch0Spec>;
#[doc = "Scratch register that is not reset by any reset"]
pub mod scratch0;
#[doc = "SCRATCH1 (rw) register accessor: Scratch register that is not reset by any reset\n\nYou can [`read`](crate::Reg::read) this register and get [`scratch1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scratch1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scratch1`]
module"]
#[doc(alias = "SCRATCH1")]
pub type Scratch1 = crate::Reg<scratch1::Scratch1Spec>;
#[doc = "Scratch register that is not reset by any reset"]
pub mod scratch1;
#[doc = "DBGR1 (rw) register accessor: Read-only debug 1\n\nYou can [`read`](crate::Reg::read) this register and get [`dbgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbgr1`]
module"]
#[doc(alias = "DBGR1")]
pub type Dbgr1 = crate::Reg<dbgr1::Dbgr1Spec>;
#[doc = "Read-only debug 1"]
pub mod dbgr1;
#[doc = "DBGR2 (rw) register accessor: Read-only debug 2\n\nYou can [`read`](crate::Reg::read) this register and get [`dbgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbgr2`]
module"]
#[doc(alias = "DBGR2")]
pub type Dbgr2 = crate::Reg<dbgr2::Dbgr2Spec>;
#[doc = "Read-only debug 2"]
pub mod dbgr2;
#[doc = "PMUENABLE (rw) register accessor: Control bit to enable/disable the PMU\n\nYou can [`read`](crate::Reg::read) this register and get [`pmuenable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmuenable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmuenable`]
module"]
#[doc(alias = "PMUENABLE")]
pub type Pmuenable = crate::Reg<pmuenable::PmuenableSpec>;
#[doc = "Control bit to enable/disable the PMU"]
pub mod pmuenable;
#[doc = "DBGCTRL (rw) register accessor: Debug subsystem Control. Determines the debug components enable and clk frequency.\n\nYou can [`read`](crate::Reg::read) this register and get [`dbgctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbgctrl`]
module"]
#[doc(alias = "DBGCTRL")]
pub type Dbgctrl = crate::Reg<dbgctrl::DbgctrlSpec>;
#[doc = "Debug subsystem Control. Determines the debug components enable and clk frequency."]
pub mod dbgctrl;
#[doc = "OTAPOINTER (rw) register accessor: OTA (Over the Air) Update Pointer/Status. Reset only by POA\n\nYou can [`read`](crate::Reg::read) this register and get [`otapointer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otapointer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otapointer`]
module"]
#[doc(alias = "OTAPOINTER")]
pub type Otapointer = crate::Reg<otapointer::OtapointerSpec>;
#[doc = "OTA (Over the Air) Update Pointer/Status. Reset only by POA"]
pub mod otapointer;
#[doc = "APBDMACTRL (rw) register accessor: DMA Control Register. Determines misc settings for DMA operation\n\nYou can [`read`](crate::Reg::read) this register and get [`apbdmactrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbdmactrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbdmactrl`]
module"]
#[doc(alias = "APBDMACTRL")]
pub type Apbdmactrl = crate::Reg<apbdmactrl::ApbdmactrlSpec>;
#[doc = "DMA Control Register. Determines misc settings for DMA operation"]
pub mod apbdmactrl;
#[doc = "KEXTCLKSEL (rw) register accessor: Locks the state of the EXTCLKSEL register from writes. This is done to prevent errant writes to the register, as this could cause the chip to halt. Write a value of 0x53 to unlock write access to the EXTCLKSEL register. Once unlocked, the register will read back a 1 to undicate this is unlocked. Writing the register with any other value other than 0x53 will enable the lock.\n\nYou can [`read`](crate::Reg::read) this register and get [`kextclksel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`kextclksel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@kextclksel`]
module"]
#[doc(alias = "KEXTCLKSEL")]
pub type Kextclksel = crate::Reg<kextclksel::KextclkselSpec>;
#[doc = "Locks the state of the EXTCLKSEL register from writes. This is done to prevent errant writes to the register, as this could cause the chip to halt. Write a value of 0x53 to unlock write access to the EXTCLKSEL register. Once unlocked, the register will read back a 1 to undicate this is unlocked. Writing the register with any other value other than 0x53 will enable the lock."]
pub mod kextclksel;
#[doc = "SIMOBUCK0 (rw) register accessor: This WRITE_ONLY register controls various buck parameters. It will read back as 0x00000000.\n\nYou can [`read`](crate::Reg::read) this register and get [`simobuck0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`simobuck0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@simobuck0`]
module"]
#[doc(alias = "SIMOBUCK0")]
pub type Simobuck0 = crate::Reg<simobuck0::Simobuck0Spec>;
#[doc = "This WRITE_ONLY register controls various buck parameters. It will read back as 0x00000000."]
pub mod simobuck0;
#[doc = "SIMOBUCK1 (rw) register accessor: 1. Control the even division of 3 clocks: refresh, low power and TONCLK. 2. Control gap bewteen secondary switches. 3. Debug features: control the amount of time TONCLK is on, and the time before snubber asserts for each buck sequence. 4. Enable or disable the observation bus. 5. Select the buck sequence operation mode. 6. Control delay between primary Pmos and Nmos transitions.\n\nYou can [`read`](crate::Reg::read) this register and get [`simobuck1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`simobuck1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@simobuck1`]
module"]
#[doc(alias = "SIMOBUCK1")]
pub type Simobuck1 = crate::Reg<simobuck1::Simobuck1Spec>;
#[doc = "1. Control the even division of 3 clocks: refresh, low power and TONCLK. 2. Control gap bewteen secondary switches. 3. Debug features: control the amount of time TONCLK is on, and the time before snubber asserts for each buck sequence. 4. Enable or disable the observation bus. 5. Select the buck sequence operation mode. 6. Control delay between primary Pmos and Nmos transitions."]
pub mod simobuck1;
#[doc = "SIMOBUCK2 (rw) register accessor: SIMO Buck Muxed VDDC Active Sequence Trim Control\n\nYou can [`read`](crate::Reg::read) this register and get [`simobuck2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`simobuck2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@simobuck2`]
module"]
#[doc(alias = "SIMOBUCK2")]
pub type Simobuck2 = crate::Reg<simobuck2::Simobuck2Spec>;
#[doc = "SIMO Buck Muxed VDDC Active Sequence Trim Control"]
pub mod simobuck2;
#[doc = "SIMOBUCK3 (rw) register accessor: SIMO Buck Muxed VDDC low power Sequence Trim Control\n\nYou can [`read`](crate::Reg::read) this register and get [`simobuck3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`simobuck3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@simobuck3`]
module"]
#[doc(alias = "SIMOBUCK3")]
pub type Simobuck3 = crate::Reg<simobuck3::Simobuck3Spec>;
#[doc = "SIMO Buck Muxed VDDC low power Sequence Trim Control"]
pub mod simobuck3;
#[doc = "SIMOBUCK6 (rw) register accessor: SIMO Buck Muxed VDDF Active Sequence Trim Control\n\nYou can [`read`](crate::Reg::read) this register and get [`simobuck6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`simobuck6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@simobuck6`]
module"]
#[doc(alias = "SIMOBUCK6")]
pub type Simobuck6 = crate::Reg<simobuck6::Simobuck6Spec>;
#[doc = "SIMO Buck Muxed VDDF Active Sequence Trim Control"]
pub mod simobuck6;
#[doc = "SIMOBUCK7 (rw) register accessor: SIMO Buck Muxed VDDF active Sequence Trim Control\n\nYou can [`read`](crate::Reg::read) this register and get [`simobuck7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`simobuck7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@simobuck7`]
module"]
#[doc(alias = "SIMOBUCK7")]
pub type Simobuck7 = crate::Reg<simobuck7::Simobuck7Spec>;
#[doc = "SIMO Buck Muxed VDDF active Sequence Trim Control"]
pub mod simobuck7;
#[doc = "SIMOBUCK8 (rw) register accessor: SIMO Buck Muxed VDDF Low Power Sequence Trim Control\n\nYou can [`read`](crate::Reg::read) this register and get [`simobuck8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`simobuck8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@simobuck8`]
module"]
#[doc(alias = "SIMOBUCK8")]
pub type Simobuck8 = crate::Reg<simobuck8::Simobuck8Spec>;
#[doc = "SIMO Buck Muxed VDDF Low Power Sequence Trim Control"]
pub mod simobuck8;
#[doc = "SIMOBUCK9 (rw) register accessor: SIMO Buck Muxed VDDS Active Sequence Trim Control\n\nYou can [`read`](crate::Reg::read) this register and get [`simobuck9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`simobuck9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@simobuck9`]
module"]
#[doc(alias = "SIMOBUCK9")]
pub type Simobuck9 = crate::Reg<simobuck9::Simobuck9Spec>;
#[doc = "SIMO Buck Muxed VDDS Active Sequence Trim Control"]
pub mod simobuck9;
#[doc = "SIMOBUCK12 (rw) register accessor: SIMO Buck Compare, Brown out, Active, Low power Trim Control\n\nYou can [`read`](crate::Reg::read) this register and get [`simobuck12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`simobuck12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@simobuck12`]
module"]
#[doc(alias = "SIMOBUCK12")]
pub type Simobuck12 = crate::Reg<simobuck12::Simobuck12Spec>;
#[doc = "SIMO Buck Compare, Brown out, Active, Low power Trim Control"]
pub mod simobuck12;
#[doc = "SIMOBUCK13 (rw) register accessor: SIMO Buck Compare, Brown out, Active, Low power Trim Control\n\nYou can [`read`](crate::Reg::read) this register and get [`simobuck13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`simobuck13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@simobuck13`]
module"]
#[doc(alias = "SIMOBUCK13")]
pub type Simobuck13 = crate::Reg<simobuck13::Simobuck13Spec>;
#[doc = "SIMO Buck Compare, Brown out, Active, Low power Trim Control"]
pub mod simobuck13;
#[doc = "SIMOBUCK15 (rw) register accessor: SIMO Buck Compare, Brown out, Active and Low power Trim Control\n\nYou can [`read`](crate::Reg::read) this register and get [`simobuck15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`simobuck15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@simobuck15`]
module"]
#[doc(alias = "SIMOBUCK15")]
pub type Simobuck15 = crate::Reg<simobuck15::Simobuck15Spec>;
#[doc = "SIMO Buck Compare, Brown out, Active and Low power Trim Control"]
pub mod simobuck15;
#[doc = "PWRSW0 (rw) register accessor: PWRSW Control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pwrsw0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrsw0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwrsw0`]
module"]
#[doc(alias = "PWRSW0")]
pub type Pwrsw0 = crate::Reg<pwrsw0::Pwrsw0Spec>;
#[doc = "PWRSW Control 0"]
pub mod pwrsw0;
#[doc = "PWRSW1 (rw) register accessor: PWRSW Control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pwrsw1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrsw1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwrsw1`]
module"]
#[doc(alias = "PWRSW1")]
pub type Pwrsw1 = crate::Reg<pwrsw1::Pwrsw1Spec>;
#[doc = "PWRSW Control 1"]
pub mod pwrsw1;
#[doc = "USBRSTCTRL (rw) register accessor: USB Reset Startup Control\n\nYou can [`read`](crate::Reg::read) this register and get [`usbrstctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbrstctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbrstctrl`]
module"]
#[doc(alias = "USBRSTCTRL")]
pub type Usbrstctrl = crate::Reg<usbrstctrl::UsbrstctrlSpec>;
#[doc = "USB Reset Startup Control"]
pub mod usbrstctrl;
#[doc = "FLASHWPROT0 (rw) register accessor: These bits write-protect flash in 16KB chunks.\n\nYou can [`read`](crate::Reg::read) this register and get [`flashwprot0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashwprot0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashwprot0`]
module"]
#[doc(alias = "FLASHWPROT0")]
pub type Flashwprot0 = crate::Reg<flashwprot0::Flashwprot0Spec>;
#[doc = "These bits write-protect flash in 16KB chunks."]
pub mod flashwprot0;
#[doc = "FLASHWPROT1 (rw) register accessor: These bits write-protect flash in 16KB chunks.\n\nYou can [`read`](crate::Reg::read) this register and get [`flashwprot1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashwprot1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashwprot1`]
module"]
#[doc(alias = "FLASHWPROT1")]
pub type Flashwprot1 = crate::Reg<flashwprot1::Flashwprot1Spec>;
#[doc = "These bits write-protect flash in 16KB chunks."]
pub mod flashwprot1;
#[doc = "FLASHWPROT2 (rw) register accessor: These bits write-protect flash in 16KB chunks.\n\nYou can [`read`](crate::Reg::read) this register and get [`flashwprot2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashwprot2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashwprot2`]
module"]
#[doc(alias = "FLASHWPROT2")]
pub type Flashwprot2 = crate::Reg<flashwprot2::Flashwprot2Spec>;
#[doc = "These bits write-protect flash in 16KB chunks."]
pub mod flashwprot2;
#[doc = "FLASHWPROT3 (rw) register accessor: These bits write-protect flash in 16KB chunks.\n\nYou can [`read`](crate::Reg::read) this register and get [`flashwprot3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashwprot3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashwprot3`]
module"]
#[doc(alias = "FLASHWPROT3")]
pub type Flashwprot3 = crate::Reg<flashwprot3::Flashwprot3Spec>;
#[doc = "These bits write-protect flash in 16KB chunks."]
pub mod flashwprot3;
#[doc = "FLASHRPROT0 (rw) register accessor: These bits read-protect flash in 16KB chunks.\n\nYou can [`read`](crate::Reg::read) this register and get [`flashrprot0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashrprot0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashrprot0`]
module"]
#[doc(alias = "FLASHRPROT0")]
pub type Flashrprot0 = crate::Reg<flashrprot0::Flashrprot0Spec>;
#[doc = "These bits read-protect flash in 16KB chunks."]
pub mod flashrprot0;
#[doc = "FLASHRPROT1 (rw) register accessor: These bits read-protect flash in 16KB chunks.\n\nYou can [`read`](crate::Reg::read) this register and get [`flashrprot1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashrprot1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashrprot1`]
module"]
#[doc(alias = "FLASHRPROT1")]
pub type Flashrprot1 = crate::Reg<flashrprot1::Flashrprot1Spec>;
#[doc = "These bits read-protect flash in 16KB chunks."]
pub mod flashrprot1;
#[doc = "FLASHRPROT2 (rw) register accessor: These bits read-protect flash in 16KB chunks.\n\nYou can [`read`](crate::Reg::read) this register and get [`flashrprot2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashrprot2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashrprot2`]
module"]
#[doc(alias = "FLASHRPROT2")]
pub type Flashrprot2 = crate::Reg<flashrprot2::Flashrprot2Spec>;
#[doc = "These bits read-protect flash in 16KB chunks."]
pub mod flashrprot2;
#[doc = "FLASHRPROT3 (rw) register accessor: These bits read-protect flash in 16KB chunks.\n\nYou can [`read`](crate::Reg::read) this register and get [`flashrprot3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashrprot3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashrprot3`]
module"]
#[doc(alias = "FLASHRPROT3")]
pub type Flashrprot3 = crate::Reg<flashrprot3::Flashrprot3Spec>;
#[doc = "These bits read-protect flash in 16KB chunks."]
pub mod flashrprot3;
#[doc = "DMASRAMWPROT0 (rw) register accessor: These bits write-protect system SRAM from DMA operations in 8KB chunks.\n\nYou can [`read`](crate::Reg::read) this register and get [`dmasramwprot0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmasramwprot0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmasramwprot0`]
module"]
#[doc(alias = "DMASRAMWPROT0")]
pub type Dmasramwprot0 = crate::Reg<dmasramwprot0::Dmasramwprot0Spec>;
#[doc = "These bits write-protect system SRAM from DMA operations in 8KB chunks."]
pub mod dmasramwprot0;
#[doc = "DMASRAMWPROT1 (rw) register accessor: These bits write-protect system SRAM from DMA operations in 8KB chunks.\n\nYou can [`read`](crate::Reg::read) this register and get [`dmasramwprot1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmasramwprot1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmasramwprot1`]
module"]
#[doc(alias = "DMASRAMWPROT1")]
pub type Dmasramwprot1 = crate::Reg<dmasramwprot1::Dmasramwprot1Spec>;
#[doc = "These bits write-protect system SRAM from DMA operations in 8KB chunks."]
pub mod dmasramwprot1;
#[doc = "DMASRAMRPROT0 (rw) register accessor: These bits read-protect system SRAM from DMA operations in 8KB chunks.\n\nYou can [`read`](crate::Reg::read) this register and get [`dmasramrprot0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmasramrprot0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmasramrprot0`]
module"]
#[doc(alias = "DMASRAMRPROT0")]
pub type Dmasramrprot0 = crate::Reg<dmasramrprot0::Dmasramrprot0Spec>;
#[doc = "These bits read-protect system SRAM from DMA operations in 8KB chunks."]
pub mod dmasramrprot0;
#[doc = "DMASRAMRPROT1 (rw) register accessor: These bits read-protect system SRAM from DMA operations in 8KB chunks.\n\nYou can [`read`](crate::Reg::read) this register and get [`dmasramrprot1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmasramrprot1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmasramrprot1`]
module"]
#[doc(alias = "DMASRAMRPROT1")]
pub type Dmasramrprot1 = crate::Reg<dmasramrprot1::Dmasramrprot1Spec>;
#[doc = "These bits read-protect system SRAM from DMA operations in 8KB chunks."]
pub mod dmasramrprot1;
#[doc = "USBPHYRESET (rw) register accessor: DSP0 CACHE RAM TRIM\n\nYou can [`read`](crate::Reg::read) this register and get [`usbphyreset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbphyreset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbphyreset`]
module"]
#[doc(alias = "USBPHYRESET")]
pub type Usbphyreset = crate::Reg<usbphyreset::UsbphyresetSpec>;
#[doc = "DSP0 CACHE RAM TRIM"]
pub mod usbphyreset;
#[doc = "AUDADCPWRCTRL (rw) register accessor: Audio ADC Power Control\n\nYou can [`read`](crate::Reg::read) this register and get [`audadcpwrctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`audadcpwrctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@audadcpwrctrl`]
module"]
#[doc(alias = "AUDADCPWRCTRL")]
pub type Audadcpwrctrl = crate::Reg<audadcpwrctrl::AudadcpwrctrlSpec>;
#[doc = "Audio ADC Power Control"]
pub mod audadcpwrctrl;
#[doc = "AUDIO1 (rw) register accessor: Audio trims 1\n\nYou can [`read`](crate::Reg::read) this register and get [`audio1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`audio1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@audio1`]
module"]
#[doc(alias = "AUDIO1")]
pub type Audio1 = crate::Reg<audio1::Audio1Spec>;
#[doc = "Audio trims 1"]
pub mod audio1;
#[doc = "PGAADCIFCTRL (rw) register accessor: PGA ADCIF control\n\nYou can [`read`](crate::Reg::read) this register and get [`pgaadcifctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pgaadcifctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pgaadcifctrl`]
module"]
#[doc(alias = "PGAADCIFCTRL")]
pub type Pgaadcifctrl = crate::Reg<pgaadcifctrl::PgaadcifctrlSpec>;
#[doc = "PGA ADCIF control"]
pub mod pgaadcifctrl;
#[doc = "PGACTRL1 (rw) register accessor: PGA control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pgactrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pgactrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pgactrl1`]
module"]
#[doc(alias = "PGACTRL1")]
pub type Pgactrl1 = crate::Reg<pgactrl1::Pgactrl1Spec>;
#[doc = "PGA control 1"]
pub mod pgactrl1;
#[doc = "PGACTRL2 (rw) register accessor: PGA control 2\n\nYou can [`read`](crate::Reg::read) this register and get [`pgactrl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pgactrl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pgactrl2`]
module"]
#[doc(alias = "PGACTRL2")]
pub type Pgactrl2 = crate::Reg<pgactrl2::Pgactrl2Spec>;
#[doc = "PGA control 2"]
pub mod pgactrl2;
#[doc = "AUDADCPWRDLY (rw) register accessor: Audio ADC Power Up Delay Control\n\nYou can [`read`](crate::Reg::read) this register and get [`audadcpwrdly::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`audadcpwrdly::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@audadcpwrdly`]
module"]
#[doc(alias = "AUDADCPWRDLY")]
pub type Audadcpwrdly = crate::Reg<audadcpwrdly::AudadcpwrdlySpec>;
#[doc = "Audio ADC Power Up Delay Control"]
pub mod audadcpwrdly;
#[doc = "SDIOCTRL (rw) register accessor: SDIO/eMMC Control\n\nYou can [`read`](crate::Reg::read) this register and get [`sdioctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdioctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdioctrl`]
module"]
#[doc(alias = "SDIOCTRL")]
pub type Sdioctrl = crate::Reg<sdioctrl::SdioctrlSpec>;
#[doc = "SDIO/eMMC Control"]
pub mod sdioctrl;
#[doc = "PDMCTRL (rw) register accessor: PDM Control\n\nYou can [`read`](crate::Reg::read) this register and get [`pdmctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdmctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdmctrl`]
module"]
#[doc(alias = "PDMCTRL")]
pub type Pdmctrl = crate::Reg<pdmctrl::PdmctrlSpec>;
#[doc = "PDM Control"]
pub mod pdmctrl;
