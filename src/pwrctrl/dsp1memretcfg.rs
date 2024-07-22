#[doc = "Register `DSP1MEMRETCFG` reader"]
pub type R = crate::R<Dsp1memretcfgSpec>;
#[doc = "Register `DSP1MEMRETCFG` writer"]
pub type W = crate::W<Dsp1memretcfgSpec>;
#[doc = "IRAM/DRAM banks are powered down when DSP1 is switched off, causing the contents of the bank to be lost.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rampwddsp1off {
    #[doc = "0: IRAM and DRAM retained"]
    Ret = 0,
    #[doc = "1: Power down all IRAM and DRAM"]
    Pwd = 1,
}
impl From<Rampwddsp1off> for bool {
    #[inline(always)]
    fn from(variant: Rampwddsp1off) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAMPWDDSP1OFF` reader - IRAM/DRAM banks are powered down when DSP1 is switched off, causing the contents of the bank to be lost."]
pub type Rampwddsp1offR = crate::BitReader<Rampwddsp1off>;
impl Rampwddsp1offR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rampwddsp1off {
        match self.bits {
            false => Rampwddsp1off::Ret,
            true => Rampwddsp1off::Pwd,
        }
    }
    #[doc = "IRAM and DRAM retained"]
    #[inline(always)]
    pub fn is_ret(&self) -> bool {
        *self == Rampwddsp1off::Ret
    }
    #[doc = "Power down all IRAM and DRAM"]
    #[inline(always)]
    pub fn is_pwd(&self) -> bool {
        *self == Rampwddsp1off::Pwd
    }
}
#[doc = "Field `RAMPWDDSP1OFF` writer - IRAM/DRAM banks are powered down when DSP1 is switched off, causing the contents of the bank to be lost."]
pub type Rampwddsp1offW<'a, REG> = crate::BitWriter<'a, REG, Rampwddsp1off>;
impl<'a, REG> Rampwddsp1offW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IRAM and DRAM retained"]
    #[inline(always)]
    pub fn ret(self) -> &'a mut crate::W<REG> {
        self.variant(Rampwddsp1off::Ret)
    }
    #[doc = "Power down all IRAM and DRAM"]
    #[inline(always)]
    pub fn pwd(self) -> &'a mut crate::W<REG> {
        self.variant(Rampwddsp1off::Pwd)
    }
}
#[doc = "Keep the memory domain active based on MCU state.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dsp1ramactmcu {
    #[doc = "0: Wakeup on demand"]
    Wakeondemand = 0,
    #[doc = "1: Keep RAMs active irrespective of MCU state"]
    Act = 1,
}
impl From<Dsp1ramactmcu> for bool {
    #[inline(always)]
    fn from(variant: Dsp1ramactmcu) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSP1RAMACTMCU` reader - Keep the memory domain active based on MCU state."]
pub type Dsp1ramactmcuR = crate::BitReader<Dsp1ramactmcu>;
impl Dsp1ramactmcuR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dsp1ramactmcu {
        match self.bits {
            false => Dsp1ramactmcu::Wakeondemand,
            true => Dsp1ramactmcu::Act,
        }
    }
    #[doc = "Wakeup on demand"]
    #[inline(always)]
    pub fn is_wakeondemand(&self) -> bool {
        *self == Dsp1ramactmcu::Wakeondemand
    }
    #[doc = "Keep RAMs active irrespective of MCU state"]
    #[inline(always)]
    pub fn is_act(&self) -> bool {
        *self == Dsp1ramactmcu::Act
    }
}
#[doc = "Field `DSP1RAMACTMCU` writer - Keep the memory domain active based on MCU state."]
pub type Dsp1ramactmcuW<'a, REG> = crate::BitWriter<'a, REG, Dsp1ramactmcu>;
impl<'a, REG> Dsp1ramactmcuW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup on demand"]
    #[inline(always)]
    pub fn wakeondemand(self) -> &'a mut crate::W<REG> {
        self.variant(Dsp1ramactmcu::Wakeondemand)
    }
    #[doc = "Keep RAMs active irrespective of MCU state"]
    #[inline(always)]
    pub fn act(self) -> &'a mut crate::W<REG> {
        self.variant(Dsp1ramactmcu::Act)
    }
}
#[doc = "ICACHE is powered down when DSP1 is switched off, causing the contents of the bank to be lost.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Icachepwddsp1off {
    #[doc = "0: ICACHE retained"]
    Ret = 0,
    #[doc = "1: Power down ICACHE"]
    Pwd = 1,
}
impl From<Icachepwddsp1off> for bool {
    #[inline(always)]
    fn from(variant: Icachepwddsp1off) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICACHEPWDDSP1OFF` reader - ICACHE is powered down when DSP1 is switched off, causing the contents of the bank to be lost."]
pub type Icachepwddsp1offR = crate::BitReader<Icachepwddsp1off>;
impl Icachepwddsp1offR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Icachepwddsp1off {
        match self.bits {
            false => Icachepwddsp1off::Ret,
            true => Icachepwddsp1off::Pwd,
        }
    }
    #[doc = "ICACHE retained"]
    #[inline(always)]
    pub fn is_ret(&self) -> bool {
        *self == Icachepwddsp1off::Ret
    }
    #[doc = "Power down ICACHE"]
    #[inline(always)]
    pub fn is_pwd(&self) -> bool {
        *self == Icachepwddsp1off::Pwd
    }
}
#[doc = "Field `ICACHEPWDDSP1OFF` writer - ICACHE is powered down when DSP1 is switched off, causing the contents of the bank to be lost."]
pub type Icachepwddsp1offW<'a, REG> = crate::BitWriter<'a, REG, Icachepwddsp1off>;
impl<'a, REG> Icachepwddsp1offW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ICACHE retained"]
    #[inline(always)]
    pub fn ret(self) -> &'a mut crate::W<REG> {
        self.variant(Icachepwddsp1off::Ret)
    }
    #[doc = "Power down ICACHE"]
    #[inline(always)]
    pub fn pwd(self) -> &'a mut crate::W<REG> {
        self.variant(Icachepwddsp1off::Pwd)
    }
}
#[doc = "Keep the memory domain active based on DISP state.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dsp1ramactdisp {
    #[doc = "0: Wakeup on demand"]
    Wakeondemand = 0,
    #[doc = "1: Keep RAMs active irrespective of DISP state"]
    Act = 1,
}
impl From<Dsp1ramactdisp> for bool {
    #[inline(always)]
    fn from(variant: Dsp1ramactdisp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSP1RAMACTDISP` reader - Keep the memory domain active based on DISP state."]
pub type Dsp1ramactdispR = crate::BitReader<Dsp1ramactdisp>;
impl Dsp1ramactdispR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dsp1ramactdisp {
        match self.bits {
            false => Dsp1ramactdisp::Wakeondemand,
            true => Dsp1ramactdisp::Act,
        }
    }
    #[doc = "Wakeup on demand"]
    #[inline(always)]
    pub fn is_wakeondemand(&self) -> bool {
        *self == Dsp1ramactdisp::Wakeondemand
    }
    #[doc = "Keep RAMs active irrespective of DISP state"]
    #[inline(always)]
    pub fn is_act(&self) -> bool {
        *self == Dsp1ramactdisp::Act
    }
}
#[doc = "Field `DSP1RAMACTDISP` writer - Keep the memory domain active based on DISP state."]
pub type Dsp1ramactdispW<'a, REG> = crate::BitWriter<'a, REG, Dsp1ramactdisp>;
impl<'a, REG> Dsp1ramactdispW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup on demand"]
    #[inline(always)]
    pub fn wakeondemand(self) -> &'a mut crate::W<REG> {
        self.variant(Dsp1ramactdisp::Wakeondemand)
    }
    #[doc = "Keep RAMs active irrespective of DISP state"]
    #[inline(always)]
    pub fn act(self) -> &'a mut crate::W<REG> {
        self.variant(Dsp1ramactdisp::Act)
    }
}
#[doc = "Keep the memory domain active based on GFX state.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dsp1ramactgfx {
    #[doc = "0: Wakeup on demand"]
    Wakeondemand = 0,
    #[doc = "1: Keep RAMs active irrespective of GFX state"]
    Act = 1,
}
impl From<Dsp1ramactgfx> for bool {
    #[inline(always)]
    fn from(variant: Dsp1ramactgfx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSP1RAMACTGFX` reader - Keep the memory domain active based on GFX state."]
pub type Dsp1ramactgfxR = crate::BitReader<Dsp1ramactgfx>;
impl Dsp1ramactgfxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dsp1ramactgfx {
        match self.bits {
            false => Dsp1ramactgfx::Wakeondemand,
            true => Dsp1ramactgfx::Act,
        }
    }
    #[doc = "Wakeup on demand"]
    #[inline(always)]
    pub fn is_wakeondemand(&self) -> bool {
        *self == Dsp1ramactgfx::Wakeondemand
    }
    #[doc = "Keep RAMs active irrespective of GFX state"]
    #[inline(always)]
    pub fn is_act(&self) -> bool {
        *self == Dsp1ramactgfx::Act
    }
}
#[doc = "Field `DSP1RAMACTGFX` writer - Keep the memory domain active based on GFX state."]
pub type Dsp1ramactgfxW<'a, REG> = crate::BitWriter<'a, REG, Dsp1ramactgfx>;
impl<'a, REG> Dsp1ramactgfxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup on demand"]
    #[inline(always)]
    pub fn wakeondemand(self) -> &'a mut crate::W<REG> {
        self.variant(Dsp1ramactgfx::Wakeondemand)
    }
    #[doc = "Keep RAMs active irrespective of GFX state"]
    #[inline(always)]
    pub fn act(self) -> &'a mut crate::W<REG> {
        self.variant(Dsp1ramactgfx::Act)
    }
}
impl R {
    #[doc = "Bit 0 - IRAM/DRAM banks are powered down when DSP1 is switched off, causing the contents of the bank to be lost."]
    #[inline(always)]
    pub fn rampwddsp1off(&self) -> Rampwddsp1offR {
        Rampwddsp1offR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Keep the memory domain active based on MCU state."]
    #[inline(always)]
    pub fn dsp1ramactmcu(&self) -> Dsp1ramactmcuR {
        Dsp1ramactmcuR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ICACHE is powered down when DSP1 is switched off, causing the contents of the bank to be lost."]
    #[inline(always)]
    pub fn icachepwddsp1off(&self) -> Icachepwddsp1offR {
        Icachepwddsp1offR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Keep the memory domain active based on DISP state."]
    #[inline(always)]
    pub fn dsp1ramactdisp(&self) -> Dsp1ramactdispR {
        Dsp1ramactdispR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Keep the memory domain active based on GFX state."]
    #[inline(always)]
    pub fn dsp1ramactgfx(&self) -> Dsp1ramactgfxR {
        Dsp1ramactgfxR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IRAM/DRAM banks are powered down when DSP1 is switched off, causing the contents of the bank to be lost."]
    #[inline(always)]
    #[must_use]
    pub fn rampwddsp1off(&mut self) -> Rampwddsp1offW<Dsp1memretcfgSpec> {
        Rampwddsp1offW::new(self, 0)
    }
    #[doc = "Bit 1 - Keep the memory domain active based on MCU state."]
    #[inline(always)]
    #[must_use]
    pub fn dsp1ramactmcu(&mut self) -> Dsp1ramactmcuW<Dsp1memretcfgSpec> {
        Dsp1ramactmcuW::new(self, 1)
    }
    #[doc = "Bit 2 - ICACHE is powered down when DSP1 is switched off, causing the contents of the bank to be lost."]
    #[inline(always)]
    #[must_use]
    pub fn icachepwddsp1off(&mut self) -> Icachepwddsp1offW<Dsp1memretcfgSpec> {
        Icachepwddsp1offW::new(self, 2)
    }
    #[doc = "Bit 3 - Keep the memory domain active based on DISP state."]
    #[inline(always)]
    #[must_use]
    pub fn dsp1ramactdisp(&mut self) -> Dsp1ramactdispW<Dsp1memretcfgSpec> {
        Dsp1ramactdispW::new(self, 3)
    }
    #[doc = "Bit 4 - Keep the memory domain active based on GFX state."]
    #[inline(always)]
    #[must_use]
    pub fn dsp1ramactgfx(&mut self) -> Dsp1ramactgfxW<Dsp1memretcfgSpec> {
        Dsp1ramactgfxW::new(self, 4)
    }
}
#[doc = "This controls the power down of the DRAM/IRAM/CACHE banks when DSP1 is powered off. If this is set, then the power for that corresponding SRAM bank will be gated when the DSP1 is powered off and data is erased. If this is not set, retention voltage will be applied when DSP1 is powered off. Do not set this if the SRAM bank is used as the target for DMA transfer while DSP1 is powered off.\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1memretcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1memretcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dsp1memretcfgSpec;
impl crate::RegisterSpec for Dsp1memretcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsp1memretcfg::R`](R) reader structure"]
impl crate::Readable for Dsp1memretcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`dsp1memretcfg::W`](W) writer structure"]
impl crate::Writable for Dsp1memretcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSP1MEMRETCFG to value 0"]
impl crate::Resettable for Dsp1memretcfgSpec {
    const RESET_VALUE: u32 = 0;
}
