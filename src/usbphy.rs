#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    reg00: Reg00,
    reg04: Reg04,
    reg08: Reg08,
    reg0c: Reg0c,
    reg10: Reg10,
    reg14: Reg14,
    reg18: Reg18,
    reg1c: Reg1c,
    reg20: Reg20,
    reg24: Reg24,
    reg28: Reg28,
    reg2c: Reg2c,
    reg30: Reg30,
    reg34: Reg34,
    reg38: Reg38,
    reg3c: Reg3c,
    reg40: Reg40,
    reg44: Reg44,
    reg48: Reg48,
    reg4c: Reg4c,
    reg50: Reg50,
    reg54: Reg54,
    reg58: Reg58,
    reg5c: Reg5c,
    reg60: Reg60,
    reg64: Reg64,
    reg68: Reg68,
    reg6c: Reg6c,
    reg70: Reg70,
    reg74: Reg74,
    reg78: Reg78,
    reg7c: Reg7c,
    reg80: Reg80,
    reg84: Reg84,
}
impl RegisterBlock {
    #[doc = "0x00 - Register description here"]
    #[inline(always)]
    pub const fn reg00(&self) -> &Reg00 {
        &self.reg00
    }
    #[doc = "0x04 - Register description here"]
    #[inline(always)]
    pub const fn reg04(&self) -> &Reg04 {
        &self.reg04
    }
    #[doc = "0x08 - Register description here"]
    #[inline(always)]
    pub const fn reg08(&self) -> &Reg08 {
        &self.reg08
    }
    #[doc = "0x0c - Register description here"]
    #[inline(always)]
    pub const fn reg0c(&self) -> &Reg0c {
        &self.reg0c
    }
    #[doc = "0x10 - Register description here"]
    #[inline(always)]
    pub const fn reg10(&self) -> &Reg10 {
        &self.reg10
    }
    #[doc = "0x14 - Register description here"]
    #[inline(always)]
    pub const fn reg14(&self) -> &Reg14 {
        &self.reg14
    }
    #[doc = "0x18 - Register description here"]
    #[inline(always)]
    pub const fn reg18(&self) -> &Reg18 {
        &self.reg18
    }
    #[doc = "0x1c - Register description here"]
    #[inline(always)]
    pub const fn reg1c(&self) -> &Reg1c {
        &self.reg1c
    }
    #[doc = "0x20 - Register description here"]
    #[inline(always)]
    pub const fn reg20(&self) -> &Reg20 {
        &self.reg20
    }
    #[doc = "0x24 - Register description here"]
    #[inline(always)]
    pub const fn reg24(&self) -> &Reg24 {
        &self.reg24
    }
    #[doc = "0x28 - Register description here"]
    #[inline(always)]
    pub const fn reg28(&self) -> &Reg28 {
        &self.reg28
    }
    #[doc = "0x2c - Register description here"]
    #[inline(always)]
    pub const fn reg2c(&self) -> &Reg2c {
        &self.reg2c
    }
    #[doc = "0x30 - Register description here"]
    #[inline(always)]
    pub const fn reg30(&self) -> &Reg30 {
        &self.reg30
    }
    #[doc = "0x34 - Register description here"]
    #[inline(always)]
    pub const fn reg34(&self) -> &Reg34 {
        &self.reg34
    }
    #[doc = "0x38 - Register description here"]
    #[inline(always)]
    pub const fn reg38(&self) -> &Reg38 {
        &self.reg38
    }
    #[doc = "0x3c - Register description here"]
    #[inline(always)]
    pub const fn reg3c(&self) -> &Reg3c {
        &self.reg3c
    }
    #[doc = "0x40 - Register description here"]
    #[inline(always)]
    pub const fn reg40(&self) -> &Reg40 {
        &self.reg40
    }
    #[doc = "0x44 - Register description here"]
    #[inline(always)]
    pub const fn reg44(&self) -> &Reg44 {
        &self.reg44
    }
    #[doc = "0x48 - Register description here"]
    #[inline(always)]
    pub const fn reg48(&self) -> &Reg48 {
        &self.reg48
    }
    #[doc = "0x4c - Register description here"]
    #[inline(always)]
    pub const fn reg4c(&self) -> &Reg4c {
        &self.reg4c
    }
    #[doc = "0x50 - Register description here"]
    #[inline(always)]
    pub const fn reg50(&self) -> &Reg50 {
        &self.reg50
    }
    #[doc = "0x54 - Register description here"]
    #[inline(always)]
    pub const fn reg54(&self) -> &Reg54 {
        &self.reg54
    }
    #[doc = "0x58 - Register description here"]
    #[inline(always)]
    pub const fn reg58(&self) -> &Reg58 {
        &self.reg58
    }
    #[doc = "0x5c - Register description here"]
    #[inline(always)]
    pub const fn reg5c(&self) -> &Reg5c {
        &self.reg5c
    }
    #[doc = "0x60 - Register description here"]
    #[inline(always)]
    pub const fn reg60(&self) -> &Reg60 {
        &self.reg60
    }
    #[doc = "0x64 - Register description here"]
    #[inline(always)]
    pub const fn reg64(&self) -> &Reg64 {
        &self.reg64
    }
    #[doc = "0x68 - Register description here"]
    #[inline(always)]
    pub const fn reg68(&self) -> &Reg68 {
        &self.reg68
    }
    #[doc = "0x6c - Register description here"]
    #[inline(always)]
    pub const fn reg6c(&self) -> &Reg6c {
        &self.reg6c
    }
    #[doc = "0x70 - Register description here"]
    #[inline(always)]
    pub const fn reg70(&self) -> &Reg70 {
        &self.reg70
    }
    #[doc = "0x74 - Register description here"]
    #[inline(always)]
    pub const fn reg74(&self) -> &Reg74 {
        &self.reg74
    }
    #[doc = "0x78 - Register description here"]
    #[inline(always)]
    pub const fn reg78(&self) -> &Reg78 {
        &self.reg78
    }
    #[doc = "0x7c - Register description here"]
    #[inline(always)]
    pub const fn reg7c(&self) -> &Reg7c {
        &self.reg7c
    }
    #[doc = "0x80 - Register description here"]
    #[inline(always)]
    pub const fn reg80(&self) -> &Reg80 {
        &self.reg80
    }
    #[doc = "0x84 - Register description here"]
    #[inline(always)]
    pub const fn reg84(&self) -> &Reg84 {
        &self.reg84
    }
}
#[doc = "REG00 (rw) register accessor: Register description here\n\nYou can [`read`](crate::Reg::read) this register and get [`reg00::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg00::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg00`]
module"]
#[doc(alias = "REG00")]
pub type Reg00 = crate::Reg<reg00::Reg00Spec>;
#[doc = "Register description here"]
pub mod reg00;
#[doc = "REG04 (rw) register accessor: Register description here\n\nYou can [`read`](crate::Reg::read) this register and get [`reg04::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg04::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg04`]
module"]
#[doc(alias = "REG04")]
pub type Reg04 = crate::Reg<reg04::Reg04Spec>;
#[doc = "Register description here"]
pub mod reg04;
#[doc = "REG08 (rw) register accessor: Register description here\n\nYou can [`read`](crate::Reg::read) this register and get [`reg08::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg08::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg08`]
module"]
#[doc(alias = "REG08")]
pub type Reg08 = crate::Reg<reg08::Reg08Spec>;
#[doc = "Register description here"]
pub mod reg08;
#[doc = "REG0C (rw) register accessor: Register description here\n\nYou can [`read`](crate::Reg::read) this register and get [`reg0c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg0c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg0c`]
module"]
#[doc(alias = "REG0C")]
pub type Reg0c = crate::Reg<reg0c::Reg0cSpec>;
#[doc = "Register description here"]
pub mod reg0c;
#[doc = "REG10 (rw) register accessor: Register description here\n\nYou can [`read`](crate::Reg::read) this register and get [`reg10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg10`]
module"]
#[doc(alias = "REG10")]
pub type Reg10 = crate::Reg<reg10::Reg10Spec>;
#[doc = "Register description here"]
pub mod reg10;
#[doc = "REG14 (rw) register accessor: Register description here\n\nYou can [`read`](crate::Reg::read) this register and get [`reg14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg14`]
module"]
#[doc(alias = "REG14")]
pub type Reg14 = crate::Reg<reg14::Reg14Spec>;
#[doc = "Register description here"]
pub mod reg14;
#[doc = "REG18 (rw) register accessor: Register description here\n\nYou can [`read`](crate::Reg::read) this register and get [`reg18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg18`]
module"]
#[doc(alias = "REG18")]
pub type Reg18 = crate::Reg<reg18::Reg18Spec>;
#[doc = "Register description here"]
pub mod reg18;
#[doc = "REG1C (rw) register accessor: Register description here\n\nYou can [`read`](crate::Reg::read) this register and get [`reg1c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg1c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg1c`]
module"]
#[doc(alias = "REG1C")]
pub type Reg1c = crate::Reg<reg1c::Reg1cSpec>;
#[doc = "Register description here"]
pub mod reg1c;
#[doc = "REG20 (rw) register accessor: Register description here\n\nYou can [`read`](crate::Reg::read) this register and get [`reg20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg20`]
module"]
#[doc(alias = "REG20")]
pub type Reg20 = crate::Reg<reg20::Reg20Spec>;
#[doc = "Register description here"]
pub mod reg20;
#[doc = "REG24 (rw) register accessor: Register description here\n\nYou can [`read`](crate::Reg::read) this register and get [`reg24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg24`]
module"]
#[doc(alias = "REG24")]
pub type Reg24 = crate::Reg<reg24::Reg24Spec>;
#[doc = "Register description here"]
pub mod reg24;
#[doc = "REG28 (rw) register accessor: Register description here\n\nYou can [`read`](crate::Reg::read) this register and get [`reg28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg28`]
module"]
#[doc(alias = "REG28")]
pub type Reg28 = crate::Reg<reg28::Reg28Spec>;
#[doc = "Register description here"]
pub mod reg28;
#[doc = "REG2C (rw) register accessor: Register description here\n\nYou can [`read`](crate::Reg::read) this register and get [`reg2c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg2c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg2c`]
module"]
#[doc(alias = "REG2C")]
pub type Reg2c = crate::Reg<reg2c::Reg2cSpec>;
#[doc = "Register description here"]
pub mod reg2c;
#[doc = "REG30 (rw) register accessor: Register description here\n\nYou can [`read`](crate::Reg::read) this register and get [`reg30::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg30`]
module"]
#[doc(alias = "REG30")]
pub type Reg30 = crate::Reg<reg30::Reg30Spec>;
#[doc = "Register description here"]
pub mod reg30;
#[doc = "REG34 (rw) register accessor: Register description here\n\nYou can [`read`](crate::Reg::read) this register and get [`reg34::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg34::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg34`]
module"]
#[doc(alias = "REG34")]
pub type Reg34 = crate::Reg<reg34::Reg34Spec>;
#[doc = "Register description here"]
pub mod reg34;
#[doc = "REG38 (rw) register accessor: Register description here\n\nYou can [`read`](crate::Reg::read) this register and get [`reg38::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg38::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg38`]
module"]
#[doc(alias = "REG38")]
pub type Reg38 = crate::Reg<reg38::Reg38Spec>;
#[doc = "Register description here"]
pub mod reg38;
#[doc = "REG3C (rw) register accessor: Register description here\n\nYou can [`read`](crate::Reg::read) this register and get [`reg3c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg3c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg3c`]
module"]
#[doc(alias = "REG3C")]
pub type Reg3c = crate::Reg<reg3c::Reg3cSpec>;
#[doc = "Register description here"]
pub mod reg3c;
#[doc = "REG40 (rw) register accessor: Register description here\n\nYou can [`read`](crate::Reg::read) this register and get [`reg40::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg40::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg40`]
module"]
#[doc(alias = "REG40")]
pub type Reg40 = crate::Reg<reg40::Reg40Spec>;
#[doc = "Register description here"]
pub mod reg40;
#[doc = "REG44 (rw) register accessor: Register description here\n\nYou can [`read`](crate::Reg::read) this register and get [`reg44::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg44::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg44`]
module"]
#[doc(alias = "REG44")]
pub type Reg44 = crate::Reg<reg44::Reg44Spec>;
#[doc = "Register description here"]
pub mod reg44;
#[doc = "REG48 (rw) register accessor: Register description here\n\nYou can [`read`](crate::Reg::read) this register and get [`reg48::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg48::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg48`]
module"]
#[doc(alias = "REG48")]
pub type Reg48 = crate::Reg<reg48::Reg48Spec>;
#[doc = "Register description here"]
pub mod reg48;
#[doc = "REG4C (rw) register accessor: Register description here\n\nYou can [`read`](crate::Reg::read) this register and get [`reg4c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg4c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg4c`]
module"]
#[doc(alias = "REG4C")]
pub type Reg4c = crate::Reg<reg4c::Reg4cSpec>;
#[doc = "Register description here"]
pub mod reg4c;
#[doc = "REG50 (rw) register accessor: Register description here\n\nYou can [`read`](crate::Reg::read) this register and get [`reg50::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg50::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg50`]
module"]
#[doc(alias = "REG50")]
pub type Reg50 = crate::Reg<reg50::Reg50Spec>;
#[doc = "Register description here"]
pub mod reg50;
#[doc = "REG54 (rw) register accessor: Register description here\n\nYou can [`read`](crate::Reg::read) this register and get [`reg54::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg54::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg54`]
module"]
#[doc(alias = "REG54")]
pub type Reg54 = crate::Reg<reg54::Reg54Spec>;
#[doc = "Register description here"]
pub mod reg54;
#[doc = "REG58 (rw) register accessor: Register description here\n\nYou can [`read`](crate::Reg::read) this register and get [`reg58::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg58::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg58`]
module"]
#[doc(alias = "REG58")]
pub type Reg58 = crate::Reg<reg58::Reg58Spec>;
#[doc = "Register description here"]
pub mod reg58;
#[doc = "REG5C (rw) register accessor: Register description here\n\nYou can [`read`](crate::Reg::read) this register and get [`reg5c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg5c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg5c`]
module"]
#[doc(alias = "REG5C")]
pub type Reg5c = crate::Reg<reg5c::Reg5cSpec>;
#[doc = "Register description here"]
pub mod reg5c;
#[doc = "REG60 (rw) register accessor: Register description here\n\nYou can [`read`](crate::Reg::read) this register and get [`reg60::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg60::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg60`]
module"]
#[doc(alias = "REG60")]
pub type Reg60 = crate::Reg<reg60::Reg60Spec>;
#[doc = "Register description here"]
pub mod reg60;
#[doc = "REG64 (rw) register accessor: Register description here\n\nYou can [`read`](crate::Reg::read) this register and get [`reg64::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg64::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg64`]
module"]
#[doc(alias = "REG64")]
pub type Reg64 = crate::Reg<reg64::Reg64Spec>;
#[doc = "Register description here"]
pub mod reg64;
#[doc = "REG68 (rw) register accessor: Register description here\n\nYou can [`read`](crate::Reg::read) this register and get [`reg68::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg68::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg68`]
module"]
#[doc(alias = "REG68")]
pub type Reg68 = crate::Reg<reg68::Reg68Spec>;
#[doc = "Register description here"]
pub mod reg68;
#[doc = "REG6C (rw) register accessor: Register description here\n\nYou can [`read`](crate::Reg::read) this register and get [`reg6c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg6c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg6c`]
module"]
#[doc(alias = "REG6C")]
pub type Reg6c = crate::Reg<reg6c::Reg6cSpec>;
#[doc = "Register description here"]
pub mod reg6c;
#[doc = "REG70 (rw) register accessor: Register description here\n\nYou can [`read`](crate::Reg::read) this register and get [`reg70::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg70::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg70`]
module"]
#[doc(alias = "REG70")]
pub type Reg70 = crate::Reg<reg70::Reg70Spec>;
#[doc = "Register description here"]
pub mod reg70;
#[doc = "REG74 (rw) register accessor: Register description here\n\nYou can [`read`](crate::Reg::read) this register and get [`reg74::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg74::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg74`]
module"]
#[doc(alias = "REG74")]
pub type Reg74 = crate::Reg<reg74::Reg74Spec>;
#[doc = "Register description here"]
pub mod reg74;
#[doc = "REG78 (rw) register accessor: Register description here\n\nYou can [`read`](crate::Reg::read) this register and get [`reg78::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg78::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg78`]
module"]
#[doc(alias = "REG78")]
pub type Reg78 = crate::Reg<reg78::Reg78Spec>;
#[doc = "Register description here"]
pub mod reg78;
#[doc = "REG7C (rw) register accessor: Register description here\n\nYou can [`read`](crate::Reg::read) this register and get [`reg7c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg7c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg7c`]
module"]
#[doc(alias = "REG7C")]
pub type Reg7c = crate::Reg<reg7c::Reg7cSpec>;
#[doc = "Register description here"]
pub mod reg7c;
#[doc = "REG80 (rw) register accessor: Register description here\n\nYou can [`read`](crate::Reg::read) this register and get [`reg80::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg80::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg80`]
module"]
#[doc(alias = "REG80")]
pub type Reg80 = crate::Reg<reg80::Reg80Spec>;
#[doc = "Register description here"]
pub mod reg80;
#[doc = "REG84 (rw) register accessor: Register description here\n\nYou can [`read`](crate::Reg::read) this register and get [`reg84::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg84::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg84`]
module"]
#[doc(alias = "REG84")]
pub type Reg84 = crate::Reg<reg84::Reg84Spec>;
#[doc = "Register description here"]
pub mod reg84;
