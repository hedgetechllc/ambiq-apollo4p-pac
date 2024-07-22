#[doc = "Register `SSRAMRETCFG` reader"]
pub type R = crate::R<SsramretcfgSpec>;
#[doc = "Register `SSRAMRETCFG` writer"]
pub type W = crate::W<SsramretcfgSpec>;
#[doc = "Selects which shared SRAM banks are powered down in deep sleep mode, causing the contents of the bank to be lost.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ssrampwdslp {
    #[doc = "0: All banks retained"]
    None = 0,
    #[doc = "1: Power down only SRAM group0"]
    Group0 = 1,
    #[doc = "2: Power down only SRAM group1"]
    Group1 = 2,
    #[doc = "3: All shared SRAM banks powered down"]
    All = 3,
}
impl From<Ssrampwdslp> for u8 {
    #[inline(always)]
    fn from(variant: Ssrampwdslp) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ssrampwdslp {
    type Ux = u8;
}
impl crate::IsEnum for Ssrampwdslp {}
#[doc = "Field `SSRAMPWDSLP` reader - Selects which shared SRAM banks are powered down in deep sleep mode, causing the contents of the bank to be lost."]
pub type SsrampwdslpR = crate::FieldReader<Ssrampwdslp>;
impl SsrampwdslpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ssrampwdslp {
        match self.bits {
            0 => Ssrampwdslp::None,
            1 => Ssrampwdslp::Group0,
            2 => Ssrampwdslp::Group1,
            3 => Ssrampwdslp::All,
            _ => unreachable!(),
        }
    }
    #[doc = "All banks retained"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Ssrampwdslp::None
    }
    #[doc = "Power down only SRAM group0"]
    #[inline(always)]
    pub fn is_group0(&self) -> bool {
        *self == Ssrampwdslp::Group0
    }
    #[doc = "Power down only SRAM group1"]
    #[inline(always)]
    pub fn is_group1(&self) -> bool {
        *self == Ssrampwdslp::Group1
    }
    #[doc = "All shared SRAM banks powered down"]
    #[inline(always)]
    pub fn is_all(&self) -> bool {
        *self == Ssrampwdslp::All
    }
}
#[doc = "Field `SSRAMPWDSLP` writer - Selects which shared SRAM banks are powered down in deep sleep mode, causing the contents of the bank to be lost."]
pub type SsrampwdslpW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ssrampwdslp, crate::Safe>;
impl<'a, REG> SsrampwdslpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "All banks retained"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Ssrampwdslp::None)
    }
    #[doc = "Power down only SRAM group0"]
    #[inline(always)]
    pub fn group0(self) -> &'a mut crate::W<REG> {
        self.variant(Ssrampwdslp::Group0)
    }
    #[doc = "Power down only SRAM group1"]
    #[inline(always)]
    pub fn group1(self) -> &'a mut crate::W<REG> {
        self.variant(Ssrampwdslp::Group1)
    }
    #[doc = "All shared SRAM banks powered down"]
    #[inline(always)]
    pub fn all(self) -> &'a mut crate::W<REG> {
        self.variant(Ssrampwdslp::All)
    }
}
#[doc = "Field `SSRAMACTMCU` reader - Keep the memory domain active based on MCU state. Each bit corresponds to a domain. 1: Keep SRAM active 0: Wakeup on demand (i.e. when MCU is powered up)"]
pub type SsramactmcuR = crate::FieldReader;
#[doc = "Field `SSRAMACTMCU` writer - Keep the memory domain active based on MCU state. Each bit corresponds to a domain. 1: Keep SRAM active 0: Wakeup on demand (i.e. when MCU is powered up)"]
pub type SsramactmcuW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SSRAMACTDSP` reader - Keep the memory domain active based on DSP state. Each bit corresponds to a domain. 1: Keep SRAM active 0: Powerup on demand (i.e. when DSP is powered up)"]
pub type SsramactdspR = crate::FieldReader;
#[doc = "Field `SSRAMACTDSP` writer - Keep the memory domain active based on DSP state. Each bit corresponds to a domain. 1: Keep SRAM active 0: Powerup on demand (i.e. when DSP is powered up)"]
pub type SsramactdspW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SSRAMACTGFX` reader - Keep the memory domain active based on GFX state. Each bit corresponds to a domain. 1: Keep SRAM active 0: Powerup on demand (i.e. when GFX is powered up)"]
pub type SsramactgfxR = crate::FieldReader;
#[doc = "Field `SSRAMACTGFX` writer - Keep the memory domain active based on GFX state. Each bit corresponds to a domain. 1: Keep SRAM active 0: Powerup on demand (i.e. when GFX is powered up)"]
pub type SsramactgfxW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SSRAMACTDISP` reader - Keep the memory domain active based on DISP state. Each bit corresponds to a domain. 1: Keep SRAM active 0: Powerup on demand (i.e. when DISP is powered up)"]
pub type SsramactdispR = crate::FieldReader;
#[doc = "Field `SSRAMACTDISP` writer - Keep the memory domain active based on DISP state. Each bit corresponds to a domain. 1: Keep SRAM active 0: Powerup on demand (i.e. when DISP is powered up)"]
pub type SsramactdispW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Selects which shared SRAM banks are powered down in deep sleep mode, causing the contents of the bank to be lost."]
    #[inline(always)]
    pub fn ssrampwdslp(&self) -> SsrampwdslpR {
        SsrampwdslpR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Keep the memory domain active based on MCU state. Each bit corresponds to a domain. 1: Keep SRAM active 0: Wakeup on demand (i.e. when MCU is powered up)"]
    #[inline(always)]
    pub fn ssramactmcu(&self) -> SsramactmcuR {
        SsramactmcuR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Keep the memory domain active based on DSP state. Each bit corresponds to a domain. 1: Keep SRAM active 0: Powerup on demand (i.e. when DSP is powered up)"]
    #[inline(always)]
    pub fn ssramactdsp(&self) -> SsramactdspR {
        SsramactdspR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Keep the memory domain active based on GFX state. Each bit corresponds to a domain. 1: Keep SRAM active 0: Powerup on demand (i.e. when GFX is powered up)"]
    #[inline(always)]
    pub fn ssramactgfx(&self) -> SsramactgfxR {
        SsramactgfxR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Keep the memory domain active based on DISP state. Each bit corresponds to a domain. 1: Keep SRAM active 0: Powerup on demand (i.e. when DISP is powered up)"]
    #[inline(always)]
    pub fn ssramactdisp(&self) -> SsramactdispR {
        SsramactdispR::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Selects which shared SRAM banks are powered down in deep sleep mode, causing the contents of the bank to be lost."]
    #[inline(always)]
    #[must_use]
    pub fn ssrampwdslp(&mut self) -> SsrampwdslpW<SsramretcfgSpec> {
        SsrampwdslpW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Keep the memory domain active based on MCU state. Each bit corresponds to a domain. 1: Keep SRAM active 0: Wakeup on demand (i.e. when MCU is powered up)"]
    #[inline(always)]
    #[must_use]
    pub fn ssramactmcu(&mut self) -> SsramactmcuW<SsramretcfgSpec> {
        SsramactmcuW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Keep the memory domain active based on DSP state. Each bit corresponds to a domain. 1: Keep SRAM active 0: Powerup on demand (i.e. when DSP is powered up)"]
    #[inline(always)]
    #[must_use]
    pub fn ssramactdsp(&mut self) -> SsramactdspW<SsramretcfgSpec> {
        SsramactdspW::new(self, 4)
    }
    #[doc = "Bits 6:7 - Keep the memory domain active based on GFX state. Each bit corresponds to a domain. 1: Keep SRAM active 0: Powerup on demand (i.e. when GFX is powered up)"]
    #[inline(always)]
    #[must_use]
    pub fn ssramactgfx(&mut self) -> SsramactgfxW<SsramretcfgSpec> {
        SsramactgfxW::new(self, 6)
    }
    #[doc = "Bits 8:9 - Keep the memory domain active based on DISP state. Each bit corresponds to a domain. 1: Keep SRAM active 0: Powerup on demand (i.e. when DISP is powered up)"]
    #[inline(always)]
    #[must_use]
    pub fn ssramactdisp(&mut self) -> SsramactdispW<SsramretcfgSpec> {
        SsramactdispW::new(self, 8)
    }
}
#[doc = "This controls the power down of the Shared SRAM banks in deep sleep mode. If this is set, then the power for that SRAM bank will be gated when the core goes into deep sleep. Upon wake, the data within the SRAMs will be erased. If this is not set, retention voltage will be applied to the SRAM bank when none of the CPU agents are in powered up and active mode. Do not set this if the SRAM bank is used as the target for DMA transfer while CPU in deepsleep.\n\nYou can [`read`](crate::Reg::read) this register and get [`ssramretcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssramretcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SsramretcfgSpec;
impl crate::RegisterSpec for SsramretcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ssramretcfg::R`](R) reader structure"]
impl crate::Readable for SsramretcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`ssramretcfg::W`](W) writer structure"]
impl crate::Writable for SsramretcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SSRAMRETCFG to value 0x03fc"]
impl crate::Resettable for SsramretcfgSpec {
    const RESET_VALUE: u32 = 0x03fc;
}
