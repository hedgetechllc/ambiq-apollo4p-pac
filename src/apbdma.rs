#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    bbvalue: Bbvalue,
    bbsetclear: Bbsetclear,
    bbinput: Bbinput,
    _reserved3: [u8; 0x14],
    debugdata: Debugdata,
    _reserved4: [u8; 0x1c],
    debug: Debug,
}
impl RegisterBlock {
    #[doc = "0x00 - Control"]
    #[inline(always)]
    pub const fn bbvalue(&self) -> &Bbvalue {
        &self.bbvalue
    }
    #[doc = "0x04 - Set/Clear"]
    #[inline(always)]
    pub const fn bbsetclear(&self) -> &Bbsetclear {
        &self.bbsetclear
    }
    #[doc = "0x08 - PIO Input Values"]
    #[inline(always)]
    pub const fn bbinput(&self) -> &Bbinput {
        &self.bbinput
    }
    #[doc = "0x20 - PIO Input Values"]
    #[inline(always)]
    pub const fn debugdata(&self) -> &Debugdata {
        &self.debugdata
    }
    #[doc = "0x40 - PIO Input Values"]
    #[inline(always)]
    pub const fn debug(&self) -> &Debug {
        &self.debug
    }
}
#[doc = "BBVALUE (rw) register accessor: Control\n\nYou can [`read`](crate::Reg::read) this register and get [`bbvalue::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bbvalue::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bbvalue`]
module"]
#[doc(alias = "BBVALUE")]
pub type Bbvalue = crate::Reg<bbvalue::BbvalueSpec>;
#[doc = "Control"]
pub mod bbvalue;
#[doc = "BBSETCLEAR (rw) register accessor: Set/Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`bbsetclear::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bbsetclear::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bbsetclear`]
module"]
#[doc(alias = "BBSETCLEAR")]
pub type Bbsetclear = crate::Reg<bbsetclear::BbsetclearSpec>;
#[doc = "Set/Clear"]
pub mod bbsetclear;
#[doc = "BBINPUT (rw) register accessor: PIO Input Values\n\nYou can [`read`](crate::Reg::read) this register and get [`bbinput::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bbinput::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bbinput`]
module"]
#[doc(alias = "BBINPUT")]
pub type Bbinput = crate::Reg<bbinput::BbinputSpec>;
#[doc = "PIO Input Values"]
pub mod bbinput;
#[doc = "DEBUGDATA (rw) register accessor: PIO Input Values\n\nYou can [`read`](crate::Reg::read) this register and get [`debugdata::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debugdata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debugdata`]
module"]
#[doc(alias = "DEBUGDATA")]
pub type Debugdata = crate::Reg<debugdata::DebugdataSpec>;
#[doc = "PIO Input Values"]
pub mod debugdata;
#[doc = "DEBUG (rw) register accessor: PIO Input Values\n\nYou can [`read`](crate::Reg::read) this register and get [`debug::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debug::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debug`]
module"]
#[doc(alias = "DEBUG")]
pub type Debug = crate::Reg<debug::DebugSpec>;
#[doc = "PIO Input Values"]
pub mod debug;
