#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: Ctrl,
    _reserved1: [u8; 0x0c],
    srcaddr: Srcaddr,
    _reserved2: [u8; 0x0c],
    len: Len,
    _reserved3: [u8; 0x0c],
    result: Result,
    _reserved4: [u8; 0x44],
    lockctrl: Lockctrl,
    lockstat: Lockstat,
    key0: Key0,
    key1: Key1,
    key2: Key2,
    key3: Key3,
}
impl RegisterBlock {
    #[doc = "0x00 - Control"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x10 - Source Addresss"]
    #[inline(always)]
    pub const fn srcaddr(&self) -> &Srcaddr {
        &self.srcaddr
    }
    #[doc = "0x20 - Length"]
    #[inline(always)]
    pub const fn len(&self) -> &Len {
        &self.len
    }
    #[doc = "0x30 - CRC Seed/Result"]
    #[inline(always)]
    pub const fn result(&self) -> &Result {
        &self.result
    }
    #[doc = "0x78 - LOCK Control"]
    #[inline(always)]
    pub const fn lockctrl(&self) -> &Lockctrl {
        &self.lockctrl
    }
    #[doc = "0x7c - LOCK Status"]
    #[inline(always)]
    pub const fn lockstat(&self) -> &Lockstat {
        &self.lockstat
    }
    #[doc = "0x80 - Key0"]
    #[inline(always)]
    pub const fn key0(&self) -> &Key0 {
        &self.key0
    }
    #[doc = "0x84 - Key1"]
    #[inline(always)]
    pub const fn key1(&self) -> &Key1 {
        &self.key1
    }
    #[doc = "0x88 - Key2"]
    #[inline(always)]
    pub const fn key2(&self) -> &Key2 {
        &self.key2
    }
    #[doc = "0x8c - Key3"]
    #[inline(always)]
    pub const fn key3(&self) -> &Key3 {
        &self.key3
    }
}
#[doc = "CTRL (rw) register accessor: Control\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Control"]
pub mod ctrl;
#[doc = "SRCADDR (rw) register accessor: Source Addresss\n\nYou can [`read`](crate::Reg::read) this register and get [`srcaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srcaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srcaddr`]
module"]
#[doc(alias = "SRCADDR")]
pub type Srcaddr = crate::Reg<srcaddr::SrcaddrSpec>;
#[doc = "Source Addresss"]
pub mod srcaddr;
#[doc = "LEN (rw) register accessor: Length\n\nYou can [`read`](crate::Reg::read) this register and get [`len::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`len::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@len`]
module"]
#[doc(alias = "LEN")]
pub type Len = crate::Reg<len::LenSpec>;
#[doc = "Length"]
pub mod len;
#[doc = "RESULT (rw) register accessor: CRC Seed/Result\n\nYou can [`read`](crate::Reg::read) this register and get [`result::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`result::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@result`]
module"]
#[doc(alias = "RESULT")]
pub type Result = crate::Reg<result::ResultSpec>;
#[doc = "CRC Seed/Result"]
pub mod result;
#[doc = "LOCKCTRL (rw) register accessor: LOCK Control\n\nYou can [`read`](crate::Reg::read) this register and get [`lockctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lockctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lockctrl`]
module"]
#[doc(alias = "LOCKCTRL")]
pub type Lockctrl = crate::Reg<lockctrl::LockctrlSpec>;
#[doc = "LOCK Control"]
pub mod lockctrl;
#[doc = "LOCKSTAT (rw) register accessor: LOCK Status\n\nYou can [`read`](crate::Reg::read) this register and get [`lockstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lockstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lockstat`]
module"]
#[doc(alias = "LOCKSTAT")]
pub type Lockstat = crate::Reg<lockstat::LockstatSpec>;
#[doc = "LOCK Status"]
pub mod lockstat;
#[doc = "KEY0 (rw) register accessor: Key0\n\nYou can [`read`](crate::Reg::read) this register and get [`key0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key0`]
module"]
#[doc(alias = "KEY0")]
pub type Key0 = crate::Reg<key0::Key0Spec>;
#[doc = "Key0"]
pub mod key0;
#[doc = "KEY1 (rw) register accessor: Key1\n\nYou can [`read`](crate::Reg::read) this register and get [`key1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key1`]
module"]
#[doc(alias = "KEY1")]
pub type Key1 = crate::Reg<key1::Key1Spec>;
#[doc = "Key1"]
pub mod key1;
#[doc = "KEY2 (rw) register accessor: Key2\n\nYou can [`read`](crate::Reg::read) this register and get [`key2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key2`]
module"]
#[doc(alias = "KEY2")]
pub type Key2 = crate::Reg<key2::Key2Spec>;
#[doc = "Key2"]
pub mod key2;
#[doc = "KEY3 (rw) register accessor: Key3\n\nYou can [`read`](crate::Reg::read) this register and get [`key3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key3`]
module"]
#[doc(alias = "KEY3")]
pub type Key3 = crate::Reg<key3::Key3Spec>;
#[doc = "Key3"]
pub mod key3;
