#[doc = "Register `DSP0MEMRETCFG` reader"]
pub type R = crate::R<Dsp0memretcfgSpec>;
#[doc = "Register `DSP0MEMRETCFG` writer"]
pub type W = crate::W<Dsp0memretcfgSpec>;
#[doc = "IRAM/DRAM banks are powered down when DSP0 is switched off, causing the contents of the bank to be lost.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rampwddsp0off {
    #[doc = "0: IRAM and DRAM retained"]
    Ret = 0,
    #[doc = "1: Power down all IRAM and DRAM"]
    Pwd = 1,
}
impl From<Rampwddsp0off> for bool {
    #[inline(always)]
    fn from(variant: Rampwddsp0off) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAMPWDDSP0OFF` reader - IRAM/DRAM banks are powered down when DSP0 is switched off, causing the contents of the bank to be lost."]
pub type Rampwddsp0offR = crate::BitReader<Rampwddsp0off>;
impl Rampwddsp0offR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rampwddsp0off {
        match self.bits {
            false => Rampwddsp0off::Ret,
            true => Rampwddsp0off::Pwd,
        }
    }
    #[doc = "IRAM and DRAM retained"]
    #[inline(always)]
    pub fn is_ret(&self) -> bool {
        *self == Rampwddsp0off::Ret
    }
    #[doc = "Power down all IRAM and DRAM"]
    #[inline(always)]
    pub fn is_pwd(&self) -> bool {
        *self == Rampwddsp0off::Pwd
    }
}
#[doc = "Field `RAMPWDDSP0OFF` writer - IRAM/DRAM banks are powered down when DSP0 is switched off, causing the contents of the bank to be lost."]
pub type Rampwddsp0offW<'a, REG> = crate::BitWriter<'a, REG, Rampwddsp0off>;
impl<'a, REG> Rampwddsp0offW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IRAM and DRAM retained"]
    #[inline(always)]
    pub fn ret(self) -> &'a mut crate::W<REG> {
        self.variant(Rampwddsp0off::Ret)
    }
    #[doc = "Power down all IRAM and DRAM"]
    #[inline(always)]
    pub fn pwd(self) -> &'a mut crate::W<REG> {
        self.variant(Rampwddsp0off::Pwd)
    }
}
#[doc = "Keep the memory domain active based on MCU state.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dsp0ramactmcu {
    #[doc = "0: Wakeup on demand"]
    Wakeondemand = 0,
    #[doc = "1: Keep RAMs active irrespective of MCU state"]
    Act = 1,
}
impl From<Dsp0ramactmcu> for bool {
    #[inline(always)]
    fn from(variant: Dsp0ramactmcu) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSP0RAMACTMCU` reader - Keep the memory domain active based on MCU state."]
pub type Dsp0ramactmcuR = crate::BitReader<Dsp0ramactmcu>;
impl Dsp0ramactmcuR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dsp0ramactmcu {
        match self.bits {
            false => Dsp0ramactmcu::Wakeondemand,
            true => Dsp0ramactmcu::Act,
        }
    }
    #[doc = "Wakeup on demand"]
    #[inline(always)]
    pub fn is_wakeondemand(&self) -> bool {
        *self == Dsp0ramactmcu::Wakeondemand
    }
    #[doc = "Keep RAMs active irrespective of MCU state"]
    #[inline(always)]
    pub fn is_act(&self) -> bool {
        *self == Dsp0ramactmcu::Act
    }
}
#[doc = "Field `DSP0RAMACTMCU` writer - Keep the memory domain active based on MCU state."]
pub type Dsp0ramactmcuW<'a, REG> = crate::BitWriter<'a, REG, Dsp0ramactmcu>;
impl<'a, REG> Dsp0ramactmcuW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup on demand"]
    #[inline(always)]
    pub fn wakeondemand(self) -> &'a mut crate::W<REG> {
        self.variant(Dsp0ramactmcu::Wakeondemand)
    }
    #[doc = "Keep RAMs active irrespective of MCU state"]
    #[inline(always)]
    pub fn act(self) -> &'a mut crate::W<REG> {
        self.variant(Dsp0ramactmcu::Act)
    }
}
#[doc = "ICACHE is powered down when DSP0 is switched off, causing the contents of the bank to be lost.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Icachepwddsp0off {
    #[doc = "0: ICACHE retained"]
    Ret = 0,
    #[doc = "1: Power down ICACHE"]
    Pwd = 1,
}
impl From<Icachepwddsp0off> for bool {
    #[inline(always)]
    fn from(variant: Icachepwddsp0off) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICACHEPWDDSP0OFF` reader - ICACHE is powered down when DSP0 is switched off, causing the contents of the bank to be lost."]
pub type Icachepwddsp0offR = crate::BitReader<Icachepwddsp0off>;
impl Icachepwddsp0offR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Icachepwddsp0off {
        match self.bits {
            false => Icachepwddsp0off::Ret,
            true => Icachepwddsp0off::Pwd,
        }
    }
    #[doc = "ICACHE retained"]
    #[inline(always)]
    pub fn is_ret(&self) -> bool {
        *self == Icachepwddsp0off::Ret
    }
    #[doc = "Power down ICACHE"]
    #[inline(always)]
    pub fn is_pwd(&self) -> bool {
        *self == Icachepwddsp0off::Pwd
    }
}
#[doc = "Field `ICACHEPWDDSP0OFF` writer - ICACHE is powered down when DSP0 is switched off, causing the contents of the bank to be lost."]
pub type Icachepwddsp0offW<'a, REG> = crate::BitWriter<'a, REG, Icachepwddsp0off>;
impl<'a, REG> Icachepwddsp0offW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ICACHE retained"]
    #[inline(always)]
    pub fn ret(self) -> &'a mut crate::W<REG> {
        self.variant(Icachepwddsp0off::Ret)
    }
    #[doc = "Power down ICACHE"]
    #[inline(always)]
    pub fn pwd(self) -> &'a mut crate::W<REG> {
        self.variant(Icachepwddsp0off::Pwd)
    }
}
#[doc = "Keep the memory domain active based on DISP state.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dsp0ramactdisp {
    #[doc = "0: Wakeup on demand"]
    Wakeondemand = 0,
    #[doc = "1: Keep RAMs active irrespective of DISP state"]
    Act = 1,
}
impl From<Dsp0ramactdisp> for bool {
    #[inline(always)]
    fn from(variant: Dsp0ramactdisp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSP0RAMACTDISP` reader - Keep the memory domain active based on DISP state."]
pub type Dsp0ramactdispR = crate::BitReader<Dsp0ramactdisp>;
impl Dsp0ramactdispR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dsp0ramactdisp {
        match self.bits {
            false => Dsp0ramactdisp::Wakeondemand,
            true => Dsp0ramactdisp::Act,
        }
    }
    #[doc = "Wakeup on demand"]
    #[inline(always)]
    pub fn is_wakeondemand(&self) -> bool {
        *self == Dsp0ramactdisp::Wakeondemand
    }
    #[doc = "Keep RAMs active irrespective of DISP state"]
    #[inline(always)]
    pub fn is_act(&self) -> bool {
        *self == Dsp0ramactdisp::Act
    }
}
#[doc = "Field `DSP0RAMACTDISP` writer - Keep the memory domain active based on DISP state."]
pub type Dsp0ramactdispW<'a, REG> = crate::BitWriter<'a, REG, Dsp0ramactdisp>;
impl<'a, REG> Dsp0ramactdispW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup on demand"]
    #[inline(always)]
    pub fn wakeondemand(self) -> &'a mut crate::W<REG> {
        self.variant(Dsp0ramactdisp::Wakeondemand)
    }
    #[doc = "Keep RAMs active irrespective of DISP state"]
    #[inline(always)]
    pub fn act(self) -> &'a mut crate::W<REG> {
        self.variant(Dsp0ramactdisp::Act)
    }
}
#[doc = "Keep the memory domain active based on GFX state.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dsp0ramactgfx {
    #[doc = "0: Wakeup on demand"]
    Wakeondemand = 0,
    #[doc = "1: Keep RAMs active irrespective of GFX state"]
    Act = 1,
}
impl From<Dsp0ramactgfx> for bool {
    #[inline(always)]
    fn from(variant: Dsp0ramactgfx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSP0RAMACTGFX` reader - Keep the memory domain active based on GFX state."]
pub type Dsp0ramactgfxR = crate::BitReader<Dsp0ramactgfx>;
impl Dsp0ramactgfxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dsp0ramactgfx {
        match self.bits {
            false => Dsp0ramactgfx::Wakeondemand,
            true => Dsp0ramactgfx::Act,
        }
    }
    #[doc = "Wakeup on demand"]
    #[inline(always)]
    pub fn is_wakeondemand(&self) -> bool {
        *self == Dsp0ramactgfx::Wakeondemand
    }
    #[doc = "Keep RAMs active irrespective of GFX state"]
    #[inline(always)]
    pub fn is_act(&self) -> bool {
        *self == Dsp0ramactgfx::Act
    }
}
#[doc = "Field `DSP0RAMACTGFX` writer - Keep the memory domain active based on GFX state."]
pub type Dsp0ramactgfxW<'a, REG> = crate::BitWriter<'a, REG, Dsp0ramactgfx>;
impl<'a, REG> Dsp0ramactgfxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup on demand"]
    #[inline(always)]
    pub fn wakeondemand(self) -> &'a mut crate::W<REG> {
        self.variant(Dsp0ramactgfx::Wakeondemand)
    }
    #[doc = "Keep RAMs active irrespective of GFX state"]
    #[inline(always)]
    pub fn act(self) -> &'a mut crate::W<REG> {
        self.variant(Dsp0ramactgfx::Act)
    }
}
impl R {
    #[doc = "Bit 0 - IRAM/DRAM banks are powered down when DSP0 is switched off, causing the contents of the bank to be lost."]
    #[inline(always)]
    pub fn rampwddsp0off(&self) -> Rampwddsp0offR {
        Rampwddsp0offR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Keep the memory domain active based on MCU state."]
    #[inline(always)]
    pub fn dsp0ramactmcu(&self) -> Dsp0ramactmcuR {
        Dsp0ramactmcuR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ICACHE is powered down when DSP0 is switched off, causing the contents of the bank to be lost."]
    #[inline(always)]
    pub fn icachepwddsp0off(&self) -> Icachepwddsp0offR {
        Icachepwddsp0offR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Keep the memory domain active based on DISP state."]
    #[inline(always)]
    pub fn dsp0ramactdisp(&self) -> Dsp0ramactdispR {
        Dsp0ramactdispR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Keep the memory domain active based on GFX state."]
    #[inline(always)]
    pub fn dsp0ramactgfx(&self) -> Dsp0ramactgfxR {
        Dsp0ramactgfxR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IRAM/DRAM banks are powered down when DSP0 is switched off, causing the contents of the bank to be lost."]
    #[inline(always)]
    #[must_use]
    pub fn rampwddsp0off(&mut self) -> Rampwddsp0offW<Dsp0memretcfgSpec> {
        Rampwddsp0offW::new(self, 0)
    }
    #[doc = "Bit 1 - Keep the memory domain active based on MCU state."]
    #[inline(always)]
    #[must_use]
    pub fn dsp0ramactmcu(&mut self) -> Dsp0ramactmcuW<Dsp0memretcfgSpec> {
        Dsp0ramactmcuW::new(self, 1)
    }
    #[doc = "Bit 2 - ICACHE is powered down when DSP0 is switched off, causing the contents of the bank to be lost."]
    #[inline(always)]
    #[must_use]
    pub fn icachepwddsp0off(&mut self) -> Icachepwddsp0offW<Dsp0memretcfgSpec> {
        Icachepwddsp0offW::new(self, 2)
    }
    #[doc = "Bit 3 - Keep the memory domain active based on DISP state."]
    #[inline(always)]
    #[must_use]
    pub fn dsp0ramactdisp(&mut self) -> Dsp0ramactdispW<Dsp0memretcfgSpec> {
        Dsp0ramactdispW::new(self, 3)
    }
    #[doc = "Bit 4 - Keep the memory domain active based on GFX state."]
    #[inline(always)]
    #[must_use]
    pub fn dsp0ramactgfx(&mut self) -> Dsp0ramactgfxW<Dsp0memretcfgSpec> {
        Dsp0ramactgfxW::new(self, 4)
    }
}
#[doc = "This controls the power down of the DRAM/IRAM/CACHE banks when DSP0 is powered off. If this is set, then the power for that corresponding SRAM bank will be gated when the DSP0 is powered off and data is erased. If this is not set, retention voltage will be applied when DSP0 is powered off. Do not set this if the SRAM bank is used as the target for DMA transfer while DSP0 is powered off.\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0memretcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0memretcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dsp0memretcfgSpec;
impl crate::RegisterSpec for Dsp0memretcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsp0memretcfg::R`](R) reader structure"]
impl crate::Readable for Dsp0memretcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`dsp0memretcfg::W`](W) writer structure"]
impl crate::Writable for Dsp0memretcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSP0MEMRETCFG to value 0"]
impl crate::Resettable for Dsp0memretcfgSpec {
    const RESET_VALUE: u32 = 0;
}
