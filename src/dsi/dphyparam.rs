#[doc = "Register `DPHYPARAM` reader"]
pub type R = crate::R<DphyparamSpec>;
#[doc = "Register `DPHYPARAM` writer"]
pub type W = crate::W<DphyparamSpec>;
#[doc = "Field `HSPREP` reader - This field provides the timing requirement in byte clocks for the high speed preparation time. This corresponds to the THS-PREP parameter specified in the DPHY specificaton"]
pub type HsprepR = crate::FieldReader;
#[doc = "Field `HSPREP` writer - This field provides the timing requirement in byte clocks for the high speed preparation time. This corresponds to the THS-PREP parameter specified in the DPHY specificaton"]
pub type HsprepW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HSZERO` reader - This field provides the timing requirement in byte clocks for the high speed drive zero time. This corresponds to the THS-ZERO parameter specified in the DPHY specification"]
pub type HszeroR = crate::FieldReader;
#[doc = "Field `HSZERO` writer - This field provides the timing requirement in byte clocks for the high speed drive zero time. This corresponds to the THS-ZERO parameter specified in the DPHY specification"]
pub type HszeroW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HSTRAIL` reader - This field provides the timing requirement in byte clocks for the high speed trail time; this corresponds to the THS-TRAIL parameter specified in the DPHY specification"]
pub type HstrailR = crate::FieldReader;
#[doc = "Field `HSTRAIL` writer - This field provides the timing requirement in byte clocks for the high speed trail time; this corresponds to the THS-TRAIL parameter specified in the DPHY specification"]
pub type HstrailW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HSEXIT` reader - This field provides the timing requirement in byte clocks for the high speed exit time; this corresponds to the THS-EXIT parameter specified in the DPHY specification"]
pub type HsexitR = crate::FieldReader;
#[doc = "Field `HSEXIT` writer - This field provides the timing requirement in byte clocks for the high speed exit time; this corresponds to the THS-EXIT parameter specified in the DPHY specification"]
pub type HsexitW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - This field provides the timing requirement in byte clocks for the high speed preparation time. This corresponds to the THS-PREP parameter specified in the DPHY specificaton"]
    #[inline(always)]
    pub fn hsprep(&self) -> HsprepR {
        HsprepR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - This field provides the timing requirement in byte clocks for the high speed drive zero time. This corresponds to the THS-ZERO parameter specified in the DPHY specification"]
    #[inline(always)]
    pub fn hszero(&self) -> HszeroR {
        HszeroR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - This field provides the timing requirement in byte clocks for the high speed trail time; this corresponds to the THS-TRAIL parameter specified in the DPHY specification"]
    #[inline(always)]
    pub fn hstrail(&self) -> HstrailR {
        HstrailR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - This field provides the timing requirement in byte clocks for the high speed exit time; this corresponds to the THS-EXIT parameter specified in the DPHY specification"]
    #[inline(always)]
    pub fn hsexit(&self) -> HsexitR {
        HsexitR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - This field provides the timing requirement in byte clocks for the high speed preparation time. This corresponds to the THS-PREP parameter specified in the DPHY specificaton"]
    #[inline(always)]
    #[must_use]
    pub fn hsprep(&mut self) -> HsprepW<DphyparamSpec> {
        HsprepW::new(self, 0)
    }
    #[doc = "Bits 8:15 - This field provides the timing requirement in byte clocks for the high speed drive zero time. This corresponds to the THS-ZERO parameter specified in the DPHY specification"]
    #[inline(always)]
    #[must_use]
    pub fn hszero(&mut self) -> HszeroW<DphyparamSpec> {
        HszeroW::new(self, 8)
    }
    #[doc = "Bits 16:23 - This field provides the timing requirement in byte clocks for the high speed trail time; this corresponds to the THS-TRAIL parameter specified in the DPHY specification"]
    #[inline(always)]
    #[must_use]
    pub fn hstrail(&mut self) -> HstrailW<DphyparamSpec> {
        HstrailW::new(self, 16)
    }
    #[doc = "Bits 24:31 - This field provides the timing requirement in byte clocks for the high speed exit time; this corresponds to the THS-EXIT parameter specified in the DPHY specification"]
    #[inline(always)]
    #[must_use]
    pub fn hsexit(&mut self) -> HsexitW<DphyparamSpec> {
        HsexitW::new(self, 24)
    }
}
#[doc = "This field provides the timing requirement in byte clocks for the high speed preparation time.\n\nYou can [`read`](crate::Reg::read) this register and get [`dphyparam::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dphyparam::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DphyparamSpec;
impl crate::RegisterSpec for DphyparamSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dphyparam::R`](R) reader structure"]
impl crate::Readable for DphyparamSpec {}
#[doc = "`write(|w| ..)` method takes [`dphyparam::W`](W) writer structure"]
impl crate::Writable for DphyparamSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DPHYPARAM to value 0x0b06_1a04"]
impl crate::Resettable for DphyparamSpec {
    const RESET_VALUE: u32 = 0x0b06_1a04;
}
