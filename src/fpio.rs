#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    rd0: Rd0,
    rd1: Rd1,
    rd2: Rd2,
    rd3: Rd3,
    wt0: Wt0,
    wt1: Wt1,
    wt2: Wt2,
    wt3: Wt3,
    wts0: Wts0,
    wts1: Wts1,
    wts2: Wts2,
    wts3: Wts3,
    wtc0: Wtc0,
    wtc1: Wtc1,
    wtc2: Wtc2,
    wtc3: Wtc3,
    en0: En0,
    en1: En1,
    en2: En2,
    en3: En3,
    ens0: Ens0,
    ens1: Ens1,
    ens2: Ens2,
    ens3: Ens3,
    enc0: Enc0,
    enc1: Enc1,
    enc2: Enc2,
    enc3: Enc3,
}
impl RegisterBlock {
    #[doc = "0x00 - GPIO Input 0 (31-0)"]
    #[inline(always)]
    pub const fn rd0(&self) -> &Rd0 {
        &self.rd0
    }
    #[doc = "0x04 - GPIO Input 1 (63-32)"]
    #[inline(always)]
    pub const fn rd1(&self) -> &Rd1 {
        &self.rd1
    }
    #[doc = "0x08 - GPIO Input 2 (95-64)"]
    #[inline(always)]
    pub const fn rd2(&self) -> &Rd2 {
        &self.rd2
    }
    #[doc = "0x0c - GPIO Input 3 (127-96)"]
    #[inline(always)]
    pub const fn rd3(&self) -> &Rd3 {
        &self.rd3
    }
    #[doc = "0x10 - GPIO Output 0 (31-0)"]
    #[inline(always)]
    pub const fn wt0(&self) -> &Wt0 {
        &self.wt0
    }
    #[doc = "0x14 - GPIO Output 1 (63-32)"]
    #[inline(always)]
    pub const fn wt1(&self) -> &Wt1 {
        &self.wt1
    }
    #[doc = "0x18 - GPIO Output 2 (95-64)"]
    #[inline(always)]
    pub const fn wt2(&self) -> &Wt2 {
        &self.wt2
    }
    #[doc = "0x1c - GPIO Output 3 (127-96)"]
    #[inline(always)]
    pub const fn wt3(&self) -> &Wt3 {
        &self.wt3
    }
    #[doc = "0x20 - GPIO Output Set 0 (31-0)"]
    #[inline(always)]
    pub const fn wts0(&self) -> &Wts0 {
        &self.wts0
    }
    #[doc = "0x24 - GPIO Output Set 1 (63-32)"]
    #[inline(always)]
    pub const fn wts1(&self) -> &Wts1 {
        &self.wts1
    }
    #[doc = "0x28 - GPIO Output Set 2 (95-64)"]
    #[inline(always)]
    pub const fn wts2(&self) -> &Wts2 {
        &self.wts2
    }
    #[doc = "0x2c - GPIO Output Set 3 (127-96)"]
    #[inline(always)]
    pub const fn wts3(&self) -> &Wts3 {
        &self.wts3
    }
    #[doc = "0x30 - GPIO Output Clear 0 (31-0)"]
    #[inline(always)]
    pub const fn wtc0(&self) -> &Wtc0 {
        &self.wtc0
    }
    #[doc = "0x34 - GPIO Output Clear 1 (63-32)"]
    #[inline(always)]
    pub const fn wtc1(&self) -> &Wtc1 {
        &self.wtc1
    }
    #[doc = "0x38 - GPIO Output Clear 2 (95-64)"]
    #[inline(always)]
    pub const fn wtc2(&self) -> &Wtc2 {
        &self.wtc2
    }
    #[doc = "0x3c - GPIO Output Clear 3 (127-96)"]
    #[inline(always)]
    pub const fn wtc3(&self) -> &Wtc3 {
        &self.wtc3
    }
    #[doc = "0x40 - GPIO Enable 0 (31-0)"]
    #[inline(always)]
    pub const fn en0(&self) -> &En0 {
        &self.en0
    }
    #[doc = "0x44 - GPIO Enable 1 (63-32)"]
    #[inline(always)]
    pub const fn en1(&self) -> &En1 {
        &self.en1
    }
    #[doc = "0x48 - GPIO Enable 2 (95-64)"]
    #[inline(always)]
    pub const fn en2(&self) -> &En2 {
        &self.en2
    }
    #[doc = "0x4c - GPIO Enable 3 (127-96)"]
    #[inline(always)]
    pub const fn en3(&self) -> &En3 {
        &self.en3
    }
    #[doc = "0x50 - GPIO Enable Set 0 (31-0)"]
    #[inline(always)]
    pub const fn ens0(&self) -> &Ens0 {
        &self.ens0
    }
    #[doc = "0x54 - GPIO Enable Set 1 (63-32)"]
    #[inline(always)]
    pub const fn ens1(&self) -> &Ens1 {
        &self.ens1
    }
    #[doc = "0x58 - GPIO Enable Set 2 (95-64)"]
    #[inline(always)]
    pub const fn ens2(&self) -> &Ens2 {
        &self.ens2
    }
    #[doc = "0x5c - GPIO Enable Set 3 (127-96)"]
    #[inline(always)]
    pub const fn ens3(&self) -> &Ens3 {
        &self.ens3
    }
    #[doc = "0x60 - GPIO Enable Clear 0 (31-0)"]
    #[inline(always)]
    pub const fn enc0(&self) -> &Enc0 {
        &self.enc0
    }
    #[doc = "0x64 - GPIO Enable Clear 1 (63-32)"]
    #[inline(always)]
    pub const fn enc1(&self) -> &Enc1 {
        &self.enc1
    }
    #[doc = "0x68 - GPIO Enable Clear 2 (95-64)"]
    #[inline(always)]
    pub const fn enc2(&self) -> &Enc2 {
        &self.enc2
    }
    #[doc = "0x6c - GPIO Enable Clear 3 (127-96)"]
    #[inline(always)]
    pub const fn enc3(&self) -> &Enc3 {
        &self.enc3
    }
}
#[doc = "RD0 (rw) register accessor: GPIO Input 0 (31-0)\n\nYou can [`read`](crate::Reg::read) this register and get [`rd0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rd0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd0`]
module"]
#[doc(alias = "RD0")]
pub type Rd0 = crate::Reg<rd0::Rd0Spec>;
#[doc = "GPIO Input 0 (31-0)"]
pub mod rd0;
#[doc = "RD1 (rw) register accessor: GPIO Input 1 (63-32)\n\nYou can [`read`](crate::Reg::read) this register and get [`rd1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rd1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd1`]
module"]
#[doc(alias = "RD1")]
pub type Rd1 = crate::Reg<rd1::Rd1Spec>;
#[doc = "GPIO Input 1 (63-32)"]
pub mod rd1;
#[doc = "RD2 (rw) register accessor: GPIO Input 2 (95-64)\n\nYou can [`read`](crate::Reg::read) this register and get [`rd2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rd2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd2`]
module"]
#[doc(alias = "RD2")]
pub type Rd2 = crate::Reg<rd2::Rd2Spec>;
#[doc = "GPIO Input 2 (95-64)"]
pub mod rd2;
#[doc = "RD3 (rw) register accessor: GPIO Input 3 (127-96)\n\nYou can [`read`](crate::Reg::read) this register and get [`rd3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rd3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd3`]
module"]
#[doc(alias = "RD3")]
pub type Rd3 = crate::Reg<rd3::Rd3Spec>;
#[doc = "GPIO Input 3 (127-96)"]
pub mod rd3;
#[doc = "WT0 (rw) register accessor: GPIO Output 0 (31-0)\n\nYou can [`read`](crate::Reg::read) this register and get [`wt0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wt0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wt0`]
module"]
#[doc(alias = "WT0")]
pub type Wt0 = crate::Reg<wt0::Wt0Spec>;
#[doc = "GPIO Output 0 (31-0)"]
pub mod wt0;
#[doc = "WT1 (rw) register accessor: GPIO Output 1 (63-32)\n\nYou can [`read`](crate::Reg::read) this register and get [`wt1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wt1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wt1`]
module"]
#[doc(alias = "WT1")]
pub type Wt1 = crate::Reg<wt1::Wt1Spec>;
#[doc = "GPIO Output 1 (63-32)"]
pub mod wt1;
#[doc = "WT2 (rw) register accessor: GPIO Output 2 (95-64)\n\nYou can [`read`](crate::Reg::read) this register and get [`wt2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wt2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wt2`]
module"]
#[doc(alias = "WT2")]
pub type Wt2 = crate::Reg<wt2::Wt2Spec>;
#[doc = "GPIO Output 2 (95-64)"]
pub mod wt2;
#[doc = "WT3 (rw) register accessor: GPIO Output 3 (127-96)\n\nYou can [`read`](crate::Reg::read) this register and get [`wt3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wt3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wt3`]
module"]
#[doc(alias = "WT3")]
pub type Wt3 = crate::Reg<wt3::Wt3Spec>;
#[doc = "GPIO Output 3 (127-96)"]
pub mod wt3;
#[doc = "WTS0 (rw) register accessor: GPIO Output Set 0 (31-0)\n\nYou can [`read`](crate::Reg::read) this register and get [`wts0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wts0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wts0`]
module"]
#[doc(alias = "WTS0")]
pub type Wts0 = crate::Reg<wts0::Wts0Spec>;
#[doc = "GPIO Output Set 0 (31-0)"]
pub mod wts0;
#[doc = "WTS1 (rw) register accessor: GPIO Output Set 1 (63-32)\n\nYou can [`read`](crate::Reg::read) this register and get [`wts1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wts1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wts1`]
module"]
#[doc(alias = "WTS1")]
pub type Wts1 = crate::Reg<wts1::Wts1Spec>;
#[doc = "GPIO Output Set 1 (63-32)"]
pub mod wts1;
#[doc = "WTS2 (rw) register accessor: GPIO Output Set 2 (95-64)\n\nYou can [`read`](crate::Reg::read) this register and get [`wts2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wts2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wts2`]
module"]
#[doc(alias = "WTS2")]
pub type Wts2 = crate::Reg<wts2::Wts2Spec>;
#[doc = "GPIO Output Set 2 (95-64)"]
pub mod wts2;
#[doc = "WTS3 (rw) register accessor: GPIO Output Set 3 (127-96)\n\nYou can [`read`](crate::Reg::read) this register and get [`wts3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wts3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wts3`]
module"]
#[doc(alias = "WTS3")]
pub type Wts3 = crate::Reg<wts3::Wts3Spec>;
#[doc = "GPIO Output Set 3 (127-96)"]
pub mod wts3;
#[doc = "WTC0 (rw) register accessor: GPIO Output Clear 0 (31-0)\n\nYou can [`read`](crate::Reg::read) this register and get [`wtc0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wtc0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wtc0`]
module"]
#[doc(alias = "WTC0")]
pub type Wtc0 = crate::Reg<wtc0::Wtc0Spec>;
#[doc = "GPIO Output Clear 0 (31-0)"]
pub mod wtc0;
#[doc = "WTC1 (rw) register accessor: GPIO Output Clear 1 (63-32)\n\nYou can [`read`](crate::Reg::read) this register and get [`wtc1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wtc1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wtc1`]
module"]
#[doc(alias = "WTC1")]
pub type Wtc1 = crate::Reg<wtc1::Wtc1Spec>;
#[doc = "GPIO Output Clear 1 (63-32)"]
pub mod wtc1;
#[doc = "WTC2 (rw) register accessor: GPIO Output Clear 2 (95-64)\n\nYou can [`read`](crate::Reg::read) this register and get [`wtc2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wtc2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wtc2`]
module"]
#[doc(alias = "WTC2")]
pub type Wtc2 = crate::Reg<wtc2::Wtc2Spec>;
#[doc = "GPIO Output Clear 2 (95-64)"]
pub mod wtc2;
#[doc = "WTC3 (rw) register accessor: GPIO Output Clear 3 (127-96)\n\nYou can [`read`](crate::Reg::read) this register and get [`wtc3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wtc3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wtc3`]
module"]
#[doc(alias = "WTC3")]
pub type Wtc3 = crate::Reg<wtc3::Wtc3Spec>;
#[doc = "GPIO Output Clear 3 (127-96)"]
pub mod wtc3;
#[doc = "EN0 (rw) register accessor: GPIO Enable 0 (31-0)\n\nYou can [`read`](crate::Reg::read) this register and get [`en0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`en0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@en0`]
module"]
#[doc(alias = "EN0")]
pub type En0 = crate::Reg<en0::En0Spec>;
#[doc = "GPIO Enable 0 (31-0)"]
pub mod en0;
#[doc = "EN1 (rw) register accessor: GPIO Enable 1 (63-32)\n\nYou can [`read`](crate::Reg::read) this register and get [`en1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`en1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@en1`]
module"]
#[doc(alias = "EN1")]
pub type En1 = crate::Reg<en1::En1Spec>;
#[doc = "GPIO Enable 1 (63-32)"]
pub mod en1;
#[doc = "EN2 (rw) register accessor: GPIO Enable 2 (95-64)\n\nYou can [`read`](crate::Reg::read) this register and get [`en2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`en2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@en2`]
module"]
#[doc(alias = "EN2")]
pub type En2 = crate::Reg<en2::En2Spec>;
#[doc = "GPIO Enable 2 (95-64)"]
pub mod en2;
#[doc = "EN3 (rw) register accessor: GPIO Enable 3 (127-96)\n\nYou can [`read`](crate::Reg::read) this register and get [`en3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`en3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@en3`]
module"]
#[doc(alias = "EN3")]
pub type En3 = crate::Reg<en3::En3Spec>;
#[doc = "GPIO Enable 3 (127-96)"]
pub mod en3;
#[doc = "ENS0 (rw) register accessor: GPIO Enable Set 0 (31-0)\n\nYou can [`read`](crate::Reg::read) this register and get [`ens0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ens0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ens0`]
module"]
#[doc(alias = "ENS0")]
pub type Ens0 = crate::Reg<ens0::Ens0Spec>;
#[doc = "GPIO Enable Set 0 (31-0)"]
pub mod ens0;
#[doc = "ENS1 (rw) register accessor: GPIO Enable Set 1 (63-32)\n\nYou can [`read`](crate::Reg::read) this register and get [`ens1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ens1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ens1`]
module"]
#[doc(alias = "ENS1")]
pub type Ens1 = crate::Reg<ens1::Ens1Spec>;
#[doc = "GPIO Enable Set 1 (63-32)"]
pub mod ens1;
#[doc = "ENS2 (rw) register accessor: GPIO Enable Set 2 (95-64)\n\nYou can [`read`](crate::Reg::read) this register and get [`ens2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ens2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ens2`]
module"]
#[doc(alias = "ENS2")]
pub type Ens2 = crate::Reg<ens2::Ens2Spec>;
#[doc = "GPIO Enable Set 2 (95-64)"]
pub mod ens2;
#[doc = "ENS3 (rw) register accessor: GPIO Enable Set 3 (127-96)\n\nYou can [`read`](crate::Reg::read) this register and get [`ens3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ens3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ens3`]
module"]
#[doc(alias = "ENS3")]
pub type Ens3 = crate::Reg<ens3::Ens3Spec>;
#[doc = "GPIO Enable Set 3 (127-96)"]
pub mod ens3;
#[doc = "ENC0 (rw) register accessor: GPIO Enable Clear 0 (31-0)\n\nYou can [`read`](crate::Reg::read) this register and get [`enc0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enc0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enc0`]
module"]
#[doc(alias = "ENC0")]
pub type Enc0 = crate::Reg<enc0::Enc0Spec>;
#[doc = "GPIO Enable Clear 0 (31-0)"]
pub mod enc0;
#[doc = "ENC1 (rw) register accessor: GPIO Enable Clear 1 (63-32)\n\nYou can [`read`](crate::Reg::read) this register and get [`enc1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enc1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enc1`]
module"]
#[doc(alias = "ENC1")]
pub type Enc1 = crate::Reg<enc1::Enc1Spec>;
#[doc = "GPIO Enable Clear 1 (63-32)"]
pub mod enc1;
#[doc = "ENC2 (rw) register accessor: GPIO Enable Clear 2 (95-64)\n\nYou can [`read`](crate::Reg::read) this register and get [`enc2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enc2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enc2`]
module"]
#[doc(alias = "ENC2")]
pub type Enc2 = crate::Reg<enc2::Enc2Spec>;
#[doc = "GPIO Enable Clear 2 (95-64)"]
pub mod enc2;
#[doc = "ENC3 (rw) register accessor: GPIO Enable Clear 3 (127-96)\n\nYou can [`read`](crate::Reg::read) this register and get [`enc3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enc3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enc3`]
module"]
#[doc(alias = "ENC3")]
pub type Enc3 = crate::Reg<enc3::Enc3Spec>;
#[doc = "GPIO Enable Clear 3 (127-96)"]
pub mod enc3;
