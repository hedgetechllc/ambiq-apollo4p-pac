#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    dr: Dr,
    rsr: Rsr,
    _reserved2: [u8; 0x10],
    fr: Fr,
    _reserved3: [u8; 0x04],
    ilpr: Ilpr,
    ibrd: Ibrd,
    fbrd: Fbrd,
    lcrh: Lcrh,
    cr: Cr,
    ifls: Ifls,
    ier: Ier,
    ies: Ies,
    mis: Mis,
    iec: Iec,
}
impl RegisterBlock {
    #[doc = "0x00 - UART Data"]
    #[inline(always)]
    pub const fn dr(&self) -> &Dr {
        &self.dr
    }
    #[doc = "0x04 - UART Status"]
    #[inline(always)]
    pub const fn rsr(&self) -> &Rsr {
        &self.rsr
    }
    #[doc = "0x18 - Flags"]
    #[inline(always)]
    pub const fn fr(&self) -> &Fr {
        &self.fr
    }
    #[doc = "0x20 - IrDA Counter"]
    #[inline(always)]
    pub const fn ilpr(&self) -> &Ilpr {
        &self.ilpr
    }
    #[doc = "0x24 - Integer Baud Rate Divisor"]
    #[inline(always)]
    pub const fn ibrd(&self) -> &Ibrd {
        &self.ibrd
    }
    #[doc = "0x28 - Fractional Baud Rate Divisor"]
    #[inline(always)]
    pub const fn fbrd(&self) -> &Fbrd {
        &self.fbrd
    }
    #[doc = "0x2c - Line Control High"]
    #[inline(always)]
    pub const fn lcrh(&self) -> &Lcrh {
        &self.lcrh
    }
    #[doc = "0x30 - Control"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x34 - FIFO Interrupt Level Select"]
    #[inline(always)]
    pub const fn ifls(&self) -> &Ifls {
        &self.ifls
    }
    #[doc = "0x38 - Interrupt Enable"]
    #[inline(always)]
    pub const fn ier(&self) -> &Ier {
        &self.ier
    }
    #[doc = "0x3c - Interrupt Status"]
    #[inline(always)]
    pub const fn ies(&self) -> &Ies {
        &self.ies
    }
    #[doc = "0x40 - Masked Interrupt Status"]
    #[inline(always)]
    pub const fn mis(&self) -> &Mis {
        &self.mis
    }
    #[doc = "0x44 - Interrupt Clear"]
    #[inline(always)]
    pub const fn iec(&self) -> &Iec {
        &self.iec
    }
}
#[doc = "DR (rw) register accessor: UART Data\n\nYou can [`read`](crate::Reg::read) this register and get [`dr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr`]
module"]
#[doc(alias = "DR")]
pub type Dr = crate::Reg<dr::DrSpec>;
#[doc = "UART Data"]
pub mod dr;
#[doc = "RSR (rw) register accessor: UART Status\n\nYou can [`read`](crate::Reg::read) this register and get [`rsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rsr`]
module"]
#[doc(alias = "RSR")]
pub type Rsr = crate::Reg<rsr::RsrSpec>;
#[doc = "UART Status"]
pub mod rsr;
#[doc = "FR (rw) register accessor: Flags\n\nYou can [`read`](crate::Reg::read) this register and get [`fr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fr`]
module"]
#[doc(alias = "FR")]
pub type Fr = crate::Reg<fr::FrSpec>;
#[doc = "Flags"]
pub mod fr;
#[doc = "ILPR (rw) register accessor: IrDA Counter\n\nYou can [`read`](crate::Reg::read) this register and get [`ilpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ilpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ilpr`]
module"]
#[doc(alias = "ILPR")]
pub type Ilpr = crate::Reg<ilpr::IlprSpec>;
#[doc = "IrDA Counter"]
pub mod ilpr;
#[doc = "IBRD (rw) register accessor: Integer Baud Rate Divisor\n\nYou can [`read`](crate::Reg::read) this register and get [`ibrd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibrd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibrd`]
module"]
#[doc(alias = "IBRD")]
pub type Ibrd = crate::Reg<ibrd::IbrdSpec>;
#[doc = "Integer Baud Rate Divisor"]
pub mod ibrd;
#[doc = "FBRD (rw) register accessor: Fractional Baud Rate Divisor\n\nYou can [`read`](crate::Reg::read) this register and get [`fbrd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fbrd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fbrd`]
module"]
#[doc(alias = "FBRD")]
pub type Fbrd = crate::Reg<fbrd::FbrdSpec>;
#[doc = "Fractional Baud Rate Divisor"]
pub mod fbrd;
#[doc = "LCRH (rw) register accessor: Line Control High\n\nYou can [`read`](crate::Reg::read) this register and get [`lcrh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcrh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcrh`]
module"]
#[doc(alias = "LCRH")]
pub type Lcrh = crate::Reg<lcrh::LcrhSpec>;
#[doc = "Line Control High"]
pub mod lcrh;
#[doc = "CR (rw) register accessor: Control\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "Control"]
pub mod cr;
#[doc = "IFLS (rw) register accessor: FIFO Interrupt Level Select\n\nYou can [`read`](crate::Reg::read) this register and get [`ifls::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifls::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ifls`]
module"]
#[doc(alias = "IFLS")]
pub type Ifls = crate::Reg<ifls::IflsSpec>;
#[doc = "FIFO Interrupt Level Select"]
pub mod ifls;
#[doc = "IER (rw) register accessor: Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`]
module"]
#[doc(alias = "IER")]
pub type Ier = crate::Reg<ier::IerSpec>;
#[doc = "Interrupt Enable"]
pub mod ier;
#[doc = "IES (rw) register accessor: Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`ies::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ies::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ies`]
module"]
#[doc(alias = "IES")]
pub type Ies = crate::Reg<ies::IesSpec>;
#[doc = "Interrupt Status"]
pub mod ies;
#[doc = "MIS (rw) register accessor: Masked Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`mis::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mis::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mis`]
module"]
#[doc(alias = "MIS")]
pub type Mis = crate::Reg<mis::MisSpec>;
#[doc = "Masked Interrupt Status"]
pub mod mis;
#[doc = "IEC (rw) register accessor: Interrupt Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`iec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iec`]
module"]
#[doc(alias = "IEC")]
pub type Iec = crate::Reg<iec::IecSpec>;
#[doc = "Interrupt Clear"]
pub mod iec;
