#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    deviceready: Deviceready,
    intrstat: Intrstat,
    intren: Intren,
    dsifuncprg: Dsifuncprg,
    hstxtimeout: Hstxtimeout,
    lprxto: Lprxto,
    turnarndto: Turnarndto,
    deviceresettimer: Deviceresettimer,
    dpiresolution: Dpiresolution,
    _reserved9: [u8; 0x04],
    hsynccnt: Hsynccnt,
    horizbkporchcnt: Horizbkporchcnt,
    horizfporchcnt: Horizfporchcnt,
    horzactiveareacnt: Horzactiveareacnt,
    vsynccnt: Vsynccnt,
    vertbkporchcnt: Vertbkporchcnt,
    vertfporchcnt: Vertfporchcnt,
    datalanehiloswcnt: Datalanehiloswcnt,
    dpi: Dpi,
    plllockcnt: Plllockcnt,
    initcnt: Initcnt,
    maxretpacsze: Maxretpacsze,
    videomodefmt: Videomodefmt,
    clkeot: Clkeot,
    polarity: Polarity,
    clklaneswt: Clklaneswt,
    lpbyteclk: Lpbyteclk,
    dphyparam: Dphyparam,
    clklanetimparm: Clklanetimparm,
    rstenbdfe: Rstenbdfe,
    afetrim0: Afetrim0,
    afetrim1: Afetrim1,
    afetrim2: Afetrim2,
    afetrim3: Afetrim3,
    _reserved33: [u8; 0x10],
    errorautorcov: Errorautorcov,
    mipidirdpidiff: Mipidirdpidiff,
    datalanepolswap: Datalanepolswap,
}
impl RegisterBlock {
    #[doc = "0x00 - Devide Ready register"]
    #[inline(always)]
    pub const fn deviceready(&self) -> &Deviceready {
        &self.deviceready
    }
    #[doc = "0x04 - The interrupt status register."]
    #[inline(always)]
    pub const fn intrstat(&self) -> &Intrstat {
        &self.intrstat
    }
    #[doc = "0x08 - Interrupt enable register."]
    #[inline(always)]
    pub const fn intren(&self) -> &Intren {
        &self.intren
    }
    #[doc = "0x0c - DSI function programming register"]
    #[inline(always)]
    pub const fn dsifuncprg(&self) -> &Dsifuncprg {
        &self.dsifuncprg
    }
    #[doc = "0x10 - Maximum duration allow for the DSi host to remain in High speed mode for transmission."]
    #[inline(always)]
    pub const fn hstxtimeout(&self) -> &Hstxtimeout {
        &self.hstxtimeout
    }
    #[doc = "0x14 - Timeout value to be checked for reverse communicationl"]
    #[inline(always)]
    pub const fn lprxto(&self) -> &Lprxto {
        &self.lprxto
    }
    #[doc = "0x18 - Timeout value to be checked after the DSI host makes a trun around in the direction of transfers."]
    #[inline(always)]
    pub const fn turnarndto(&self) -> &Turnarndto {
        &self.turnarndto
    }
    #[doc = "0x1c - Timeout value to be checked for device to be reset after issuing reset entry command"]
    #[inline(always)]
    pub const fn deviceresettimer(&self) -> &Deviceresettimer {
        &self.deviceresettimer
    }
    #[doc = "0x20 - Shows the horizontal address count in pixels"]
    #[inline(always)]
    pub const fn dpiresolution(&self) -> &Dpiresolution {
        &self.dpiresolution
    }
    #[doc = "0x28 - Shows the horizontal sync value in terms of byte clock."]
    #[inline(always)]
    pub const fn hsynccnt(&self) -> &Hsynccnt {
        &self.hsynccnt
    }
    #[doc = "0x2c - Shows the horizontal back porch value in terms of txbyteclkhs."]
    #[inline(always)]
    pub const fn horizbkporchcnt(&self) -> &Horizbkporchcnt {
        &self.horizbkporchcnt
    }
    #[doc = "0x30 - Shows the horizontal front porch value in terms of txbyteclkhs."]
    #[inline(always)]
    pub const fn horizfporchcnt(&self) -> &Horizfporchcnt {
        &self.horizfporchcnt
    }
    #[doc = "0x34 - Horizontal active area count / time for active image data / Horizontal Address"]
    #[inline(always)]
    pub const fn horzactiveareacnt(&self) -> &Horzactiveareacnt {
        &self.horzactiveareacnt
    }
    #[doc = "0x38 - Shows the vertical sync value"]
    #[inline(always)]
    pub const fn vsynccnt(&self) -> &Vsynccnt {
        &self.vsynccnt
    }
    #[doc = "0x3c - Shows the vertical back porch value"]
    #[inline(always)]
    pub const fn vertbkporchcnt(&self) -> &Vertbkporchcnt {
        &self.vertbkporchcnt
    }
    #[doc = "0x40 - Shows the vertical front porch value"]
    #[inline(always)]
    pub const fn vertfporchcnt(&self) -> &Vertfporchcnt {
        &self.vertfporchcnt
    }
    #[doc = "0x44 - High speed to low power or Low power to high speed switching time"]
    #[inline(always)]
    pub const fn datalanehiloswcnt(&self) -> &Datalanehiloswcnt {
        &self.datalanehiloswcnt
    }
    #[doc = "0x48 - DPI control register."]
    #[inline(always)]
    pub const fn dpi(&self) -> &Dpi {
        &self.dpi
    }
    #[doc = "0x4c - The PLL counter value"]
    #[inline(always)]
    pub const fn plllockcnt(&self) -> &Plllockcnt {
        &self.plllockcnt
    }
    #[doc = "0x50 - Count register to initialize the DSI HOST IP"]
    #[inline(always)]
    pub const fn initcnt(&self) -> &Initcnt {
        &self.initcnt
    }
    #[doc = "0x54 - MAXRETPACSZE register description needed here."]
    #[inline(always)]
    pub const fn maxretpacsze(&self) -> &Maxretpacsze {
        &self.maxretpacsze
    }
    #[doc = "0x58 - Sets the Video mode format (packet sequence) to be supported in DSI."]
    #[inline(always)]
    pub const fn videomodefmt(&self) -> &Videomodefmt {
        &self.videomodefmt
    }
    #[doc = "0x5c - The EOT clock register disables the video."]
    #[inline(always)]
    pub const fn clkeot(&self) -> &Clkeot {
        &self.clkeot
    }
    #[doc = "0x60 - Polarity Register"]
    #[inline(always)]
    pub const fn polarity(&self) -> &Polarity {
        &self.polarity
    }
    #[doc = "0x64 - High speed to low power switching time in terms ofbyte clock (txbyteclkhs)"]
    #[inline(always)]
    pub const fn clklaneswt(&self) -> &Clklaneswt {
        &self.clklaneswt
    }
    #[doc = "0x68 - Low power clock equivalence in terms of byte clock."]
    #[inline(always)]
    pub const fn lpbyteclk(&self) -> &Lpbyteclk {
        &self.lpbyteclk
    }
    #[doc = "0x6c - This field provides the timing requirement in byte clocks for the high speed preparation time."]
    #[inline(always)]
    pub const fn dphyparam(&self) -> &Dphyparam {
        &self.dphyparam
    }
    #[doc = "0x70 - This field provides the timing requirement in byte clocks"]
    #[inline(always)]
    pub const fn clklanetimparm(&self) -> &Clklanetimparm {
        &self.clklanetimparm
    }
    #[doc = "0x74 - This field provides the reset (enable) to the DFE"]
    #[inline(always)]
    pub const fn rstenbdfe(&self) -> &Rstenbdfe {
        &self.rstenbdfe
    }
    #[doc = "0x78 - Afe Trim reg0"]
    #[inline(always)]
    pub const fn afetrim0(&self) -> &Afetrim0 {
        &self.afetrim0
    }
    #[doc = "0x7c - Afe Trim reg1"]
    #[inline(always)]
    pub const fn afetrim1(&self) -> &Afetrim1 {
        &self.afetrim1
    }
    #[doc = "0x80 - Afe Trim reg2"]
    #[inline(always)]
    pub const fn afetrim2(&self) -> &Afetrim2 {
        &self.afetrim2
    }
    #[doc = "0x84 - Afe Trim reg3"]
    #[inline(always)]
    pub const fn afetrim3(&self) -> &Afetrim3 {
        &self.afetrim3
    }
    #[doc = "0x98 - Errir ayti recivert register"]
    #[inline(always)]
    pub const fn errorautorcov(&self) -> &Errorautorcov {
        &self.errorautorcov
    }
    #[doc = "0x9c - Mipi direction DPI difference"]
    #[inline(always)]
    pub const fn mipidirdpidiff(&self) -> &Mipidirdpidiff {
        &self.mipidirdpidiff
    }
    #[doc = "0xa0 - Data lane polarity swap register"]
    #[inline(always)]
    pub const fn datalanepolswap(&self) -> &Datalanepolswap {
        &self.datalanepolswap
    }
}
#[doc = "DEVICEREADY (rw) register accessor: Devide Ready register\n\nYou can [`read`](crate::Reg::read) this register and get [`deviceready::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`deviceready::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deviceready`]
module"]
#[doc(alias = "DEVICEREADY")]
pub type Deviceready = crate::Reg<deviceready::DevicereadySpec>;
#[doc = "Devide Ready register"]
pub mod deviceready;
#[doc = "INTRSTAT (rw) register accessor: The interrupt status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`intrstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intrstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intrstat`]
module"]
#[doc(alias = "INTRSTAT")]
pub type Intrstat = crate::Reg<intrstat::IntrstatSpec>;
#[doc = "The interrupt status register."]
pub mod intrstat;
#[doc = "INTREN (rw) register accessor: Interrupt enable register.\n\nYou can [`read`](crate::Reg::read) this register and get [`intren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intren`]
module"]
#[doc(alias = "INTREN")]
pub type Intren = crate::Reg<intren::IntrenSpec>;
#[doc = "Interrupt enable register."]
pub mod intren;
#[doc = "DSIFUNCPRG (rw) register accessor: DSI function programming register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsifuncprg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsifuncprg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsifuncprg`]
module"]
#[doc(alias = "DSIFUNCPRG")]
pub type Dsifuncprg = crate::Reg<dsifuncprg::DsifuncprgSpec>;
#[doc = "DSI function programming register"]
pub mod dsifuncprg;
#[doc = "HSTXTIMEOUT (rw) register accessor: Maximum duration allow for the DSi host to remain in High speed mode for transmission.\n\nYou can [`read`](crate::Reg::read) this register and get [`hstxtimeout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstxtimeout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstxtimeout`]
module"]
#[doc(alias = "HSTXTIMEOUT")]
pub type Hstxtimeout = crate::Reg<hstxtimeout::HstxtimeoutSpec>;
#[doc = "Maximum duration allow for the DSi host to remain in High speed mode for transmission."]
pub mod hstxtimeout;
#[doc = "LPRXTO (rw) register accessor: Timeout value to be checked for reverse communicationl\n\nYou can [`read`](crate::Reg::read) this register and get [`lprxto::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lprxto::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lprxto`]
module"]
#[doc(alias = "LPRXTO")]
pub type Lprxto = crate::Reg<lprxto::LprxtoSpec>;
#[doc = "Timeout value to be checked for reverse communicationl"]
pub mod lprxto;
#[doc = "TURNARNDTO (rw) register accessor: Timeout value to be checked after the DSI host makes a trun around in the direction of transfers.\n\nYou can [`read`](crate::Reg::read) this register and get [`turnarndto::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`turnarndto::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@turnarndto`]
module"]
#[doc(alias = "TURNARNDTO")]
pub type Turnarndto = crate::Reg<turnarndto::TurnarndtoSpec>;
#[doc = "Timeout value to be checked after the DSI host makes a trun around in the direction of transfers."]
pub mod turnarndto;
#[doc = "DEVICERESETTIMER (rw) register accessor: Timeout value to be checked for device to be reset after issuing reset entry command\n\nYou can [`read`](crate::Reg::read) this register and get [`deviceresettimer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`deviceresettimer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deviceresettimer`]
module"]
#[doc(alias = "DEVICERESETTIMER")]
pub type Deviceresettimer = crate::Reg<deviceresettimer::DeviceresettimerSpec>;
#[doc = "Timeout value to be checked for device to be reset after issuing reset entry command"]
pub mod deviceresettimer;
#[doc = "DPIRESOLUTION (rw) register accessor: Shows the horizontal address count in pixels\n\nYou can [`read`](crate::Reg::read) this register and get [`dpiresolution::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpiresolution::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpiresolution`]
module"]
#[doc(alias = "DPIRESOLUTION")]
pub type Dpiresolution = crate::Reg<dpiresolution::DpiresolutionSpec>;
#[doc = "Shows the horizontal address count in pixels"]
pub mod dpiresolution;
#[doc = "HSYNCCNT (rw) register accessor: Shows the horizontal sync value in terms of byte clock.\n\nYou can [`read`](crate::Reg::read) this register and get [`hsynccnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsynccnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsynccnt`]
module"]
#[doc(alias = "HSYNCCNT")]
pub type Hsynccnt = crate::Reg<hsynccnt::HsynccntSpec>;
#[doc = "Shows the horizontal sync value in terms of byte clock."]
pub mod hsynccnt;
#[doc = "HORIZBKPORCHCNT (rw) register accessor: Shows the horizontal back porch value in terms of txbyteclkhs.\n\nYou can [`read`](crate::Reg::read) this register and get [`horizbkporchcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`horizbkporchcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@horizbkporchcnt`]
module"]
#[doc(alias = "HORIZBKPORCHCNT")]
pub type Horizbkporchcnt = crate::Reg<horizbkporchcnt::HorizbkporchcntSpec>;
#[doc = "Shows the horizontal back porch value in terms of txbyteclkhs."]
pub mod horizbkporchcnt;
#[doc = "HORIZFPORCHCNT (rw) register accessor: Shows the horizontal front porch value in terms of txbyteclkhs.\n\nYou can [`read`](crate::Reg::read) this register and get [`horizfporchcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`horizfporchcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@horizfporchcnt`]
module"]
#[doc(alias = "HORIZFPORCHCNT")]
pub type Horizfporchcnt = crate::Reg<horizfporchcnt::HorizfporchcntSpec>;
#[doc = "Shows the horizontal front porch value in terms of txbyteclkhs."]
pub mod horizfporchcnt;
#[doc = "HORZACTIVEAREACNT (rw) register accessor: Horizontal active area count / time for active image data / Horizontal Address\n\nYou can [`read`](crate::Reg::read) this register and get [`horzactiveareacnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`horzactiveareacnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@horzactiveareacnt`]
module"]
#[doc(alias = "HORZACTIVEAREACNT")]
pub type Horzactiveareacnt = crate::Reg<horzactiveareacnt::HorzactiveareacntSpec>;
#[doc = "Horizontal active area count / time for active image data / Horizontal Address"]
pub mod horzactiveareacnt;
#[doc = "VSYNCCNT (rw) register accessor: Shows the vertical sync value\n\nYou can [`read`](crate::Reg::read) this register and get [`vsynccnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vsynccnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vsynccnt`]
module"]
#[doc(alias = "VSYNCCNT")]
pub type Vsynccnt = crate::Reg<vsynccnt::VsynccntSpec>;
#[doc = "Shows the vertical sync value"]
pub mod vsynccnt;
#[doc = "VERTBKPORCHCNT (rw) register accessor: Shows the vertical back porch value\n\nYou can [`read`](crate::Reg::read) this register and get [`vertbkporchcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vertbkporchcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vertbkporchcnt`]
module"]
#[doc(alias = "VERTBKPORCHCNT")]
pub type Vertbkporchcnt = crate::Reg<vertbkporchcnt::VertbkporchcntSpec>;
#[doc = "Shows the vertical back porch value"]
pub mod vertbkporchcnt;
#[doc = "VERTFPORCHCNT (rw) register accessor: Shows the vertical front porch value\n\nYou can [`read`](crate::Reg::read) this register and get [`vertfporchcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vertfporchcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vertfporchcnt`]
module"]
#[doc(alias = "VERTFPORCHCNT")]
pub type Vertfporchcnt = crate::Reg<vertfporchcnt::VertfporchcntSpec>;
#[doc = "Shows the vertical front porch value"]
pub mod vertfporchcnt;
#[doc = "DATALANEHILOSWCNT (rw) register accessor: High speed to low power or Low power to high speed switching time\n\nYou can [`read`](crate::Reg::read) this register and get [`datalanehiloswcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`datalanehiloswcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@datalanehiloswcnt`]
module"]
#[doc(alias = "DATALANEHILOSWCNT")]
pub type Datalanehiloswcnt = crate::Reg<datalanehiloswcnt::DatalanehiloswcntSpec>;
#[doc = "High speed to low power or Low power to high speed switching time"]
pub mod datalanehiloswcnt;
#[doc = "DPI (rw) register accessor: DPI control register.\n\nYou can [`read`](crate::Reg::read) this register and get [`dpi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpi`]
module"]
#[doc(alias = "DPI")]
pub type Dpi = crate::Reg<dpi::DpiSpec>;
#[doc = "DPI control register."]
pub mod dpi;
#[doc = "PLLLOCKCNT (rw) register accessor: The PLL counter value\n\nYou can [`read`](crate::Reg::read) this register and get [`plllockcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`plllockcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@plllockcnt`]
module"]
#[doc(alias = "PLLLOCKCNT")]
pub type Plllockcnt = crate::Reg<plllockcnt::PlllockcntSpec>;
#[doc = "The PLL counter value"]
pub mod plllockcnt;
#[doc = "INITCNT (rw) register accessor: Count register to initialize the DSI HOST IP\n\nYou can [`read`](crate::Reg::read) this register and get [`initcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`initcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@initcnt`]
module"]
#[doc(alias = "INITCNT")]
pub type Initcnt = crate::Reg<initcnt::InitcntSpec>;
#[doc = "Count register to initialize the DSI HOST IP"]
pub mod initcnt;
#[doc = "MAXRETPACSZE (rw) register accessor: MAXRETPACSZE register description needed here.\n\nYou can [`read`](crate::Reg::read) this register and get [`maxretpacsze::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maxretpacsze::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maxretpacsze`]
module"]
#[doc(alias = "MAXRETPACSZE")]
pub type Maxretpacsze = crate::Reg<maxretpacsze::MaxretpacszeSpec>;
#[doc = "MAXRETPACSZE register description needed here."]
pub mod maxretpacsze;
#[doc = "VIDEOMODEFMT (rw) register accessor: Sets the Video mode format (packet sequence) to be supported in DSI.\n\nYou can [`read`](crate::Reg::read) this register and get [`videomodefmt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`videomodefmt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@videomodefmt`]
module"]
#[doc(alias = "VIDEOMODEFMT")]
pub type Videomodefmt = crate::Reg<videomodefmt::VideomodefmtSpec>;
#[doc = "Sets the Video mode format (packet sequence) to be supported in DSI."]
pub mod videomodefmt;
#[doc = "CLKEOT (rw) register accessor: The EOT clock register disables the video.\n\nYou can [`read`](crate::Reg::read) this register and get [`clkeot::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkeot::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkeot`]
module"]
#[doc(alias = "CLKEOT")]
pub type Clkeot = crate::Reg<clkeot::ClkeotSpec>;
#[doc = "The EOT clock register disables the video."]
pub mod clkeot;
#[doc = "POLARITY (rw) register accessor: Polarity Register\n\nYou can [`read`](crate::Reg::read) this register and get [`polarity::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`polarity::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@polarity`]
module"]
#[doc(alias = "POLARITY")]
pub type Polarity = crate::Reg<polarity::PolaritySpec>;
#[doc = "Polarity Register"]
pub mod polarity;
#[doc = "CLKLANESWT (rw) register accessor: High speed to low power switching time in terms ofbyte clock (txbyteclkhs)\n\nYou can [`read`](crate::Reg::read) this register and get [`clklaneswt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clklaneswt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clklaneswt`]
module"]
#[doc(alias = "CLKLANESWT")]
pub type Clklaneswt = crate::Reg<clklaneswt::ClklaneswtSpec>;
#[doc = "High speed to low power switching time in terms ofbyte clock (txbyteclkhs)"]
pub mod clklaneswt;
#[doc = "LPBYTECLK (rw) register accessor: Low power clock equivalence in terms of byte clock.\n\nYou can [`read`](crate::Reg::read) this register and get [`lpbyteclk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpbyteclk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpbyteclk`]
module"]
#[doc(alias = "LPBYTECLK")]
pub type Lpbyteclk = crate::Reg<lpbyteclk::LpbyteclkSpec>;
#[doc = "Low power clock equivalence in terms of byte clock."]
pub mod lpbyteclk;
#[doc = "DPHYPARAM (rw) register accessor: This field provides the timing requirement in byte clocks for the high speed preparation time.\n\nYou can [`read`](crate::Reg::read) this register and get [`dphyparam::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dphyparam::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dphyparam`]
module"]
#[doc(alias = "DPHYPARAM")]
pub type Dphyparam = crate::Reg<dphyparam::DphyparamSpec>;
#[doc = "This field provides the timing requirement in byte clocks for the high speed preparation time."]
pub mod dphyparam;
#[doc = "CLKLANETIMPARM (rw) register accessor: This field provides the timing requirement in byte clocks\n\nYou can [`read`](crate::Reg::read) this register and get [`clklanetimparm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clklanetimparm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clklanetimparm`]
module"]
#[doc(alias = "CLKLANETIMPARM")]
pub type Clklanetimparm = crate::Reg<clklanetimparm::ClklanetimparmSpec>;
#[doc = "This field provides the timing requirement in byte clocks"]
pub mod clklanetimparm;
#[doc = "RSTENBDFE (rw) register accessor: This field provides the reset (enable) to the DFE\n\nYou can [`read`](crate::Reg::read) this register and get [`rstenbdfe::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstenbdfe::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstenbdfe`]
module"]
#[doc(alias = "RSTENBDFE")]
pub type Rstenbdfe = crate::Reg<rstenbdfe::RstenbdfeSpec>;
#[doc = "This field provides the reset (enable) to the DFE"]
pub mod rstenbdfe;
#[doc = "AFETRIM0 (rw) register accessor: Afe Trim reg0\n\nYou can [`read`](crate::Reg::read) this register and get [`afetrim0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afetrim0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@afetrim0`]
module"]
#[doc(alias = "AFETRIM0")]
pub type Afetrim0 = crate::Reg<afetrim0::Afetrim0Spec>;
#[doc = "Afe Trim reg0"]
pub mod afetrim0;
#[doc = "AFETRIM1 (rw) register accessor: Afe Trim reg1\n\nYou can [`read`](crate::Reg::read) this register and get [`afetrim1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afetrim1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@afetrim1`]
module"]
#[doc(alias = "AFETRIM1")]
pub type Afetrim1 = crate::Reg<afetrim1::Afetrim1Spec>;
#[doc = "Afe Trim reg1"]
pub mod afetrim1;
#[doc = "AFETRIM2 (rw) register accessor: Afe Trim reg2\n\nYou can [`read`](crate::Reg::read) this register and get [`afetrim2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afetrim2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@afetrim2`]
module"]
#[doc(alias = "AFETRIM2")]
pub type Afetrim2 = crate::Reg<afetrim2::Afetrim2Spec>;
#[doc = "Afe Trim reg2"]
pub mod afetrim2;
#[doc = "AFETRIM3 (rw) register accessor: Afe Trim reg3\n\nYou can [`read`](crate::Reg::read) this register and get [`afetrim3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afetrim3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@afetrim3`]
module"]
#[doc(alias = "AFETRIM3")]
pub type Afetrim3 = crate::Reg<afetrim3::Afetrim3Spec>;
#[doc = "Afe Trim reg3"]
pub mod afetrim3;
#[doc = "ERRORAUTORCOV (rw) register accessor: Errir ayti recivert register\n\nYou can [`read`](crate::Reg::read) this register and get [`errorautorcov::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`errorautorcov::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@errorautorcov`]
module"]
#[doc(alias = "ERRORAUTORCOV")]
pub type Errorautorcov = crate::Reg<errorautorcov::ErrorautorcovSpec>;
#[doc = "Errir ayti recivert register"]
pub mod errorautorcov;
#[doc = "MIPIDIRDPIDIFF (rw) register accessor: Mipi direction DPI difference\n\nYou can [`read`](crate::Reg::read) this register and get [`mipidirdpidiff::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mipidirdpidiff::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mipidirdpidiff`]
module"]
#[doc(alias = "MIPIDIRDPIDIFF")]
pub type Mipidirdpidiff = crate::Reg<mipidirdpidiff::MipidirdpidiffSpec>;
#[doc = "Mipi direction DPI difference"]
pub mod mipidirdpidiff;
#[doc = "DATALANEPOLSWAP (rw) register accessor: Data lane polarity swap register\n\nYou can [`read`](crate::Reg::read) this register and get [`datalanepolswap::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`datalanepolswap::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@datalanepolswap`]
module"]
#[doc(alias = "DATALANEPOLSWAP")]
pub type Datalanepolswap = crate::Reg<datalanepolswap::DatalanepolswapSpec>;
#[doc = "Data lane polarity swap register"]
pub mod datalanepolswap;
