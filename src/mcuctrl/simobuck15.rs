#[doc = "Register `SIMOBUCK15` reader"]
pub type R = crate::R<Simobuck15Spec>;
#[doc = "Register `SIMOBUCK15` writer"]
pub type W = crate::W<Simobuck15Spec>;
#[doc = "Field `ZXCOMPOFFSETTRIM` reader - Zxcomp offset trim. Feedthrough to analog."]
pub type ZxcompoffsettrimR = crate::FieldReader;
#[doc = "Field `ZXCOMPOFFSETTRIM` writer - Zxcomp offset trim. Feedthrough to analog."]
pub type ZxcompoffsettrimW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TRIMLATCHOVER` reader - Override / Bypass the simobuck trim latch to enable on-the-fly trimming for VDDF and VDDS active and LP trims"]
pub type TrimlatchoverR = crate::BitReader;
#[doc = "Field `TRIMLATCHOVER` writer - Override / Bypass the simobuck trim latch to enable on-the-fly trimming for VDDF and VDDS active and LP trims"]
pub type TrimlatchoverW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 24:28 - Zxcomp offset trim. Feedthrough to analog."]
    #[inline(always)]
    pub fn zxcompoffsettrim(&self) -> ZxcompoffsettrimR {
        ZxcompoffsettrimR::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - Override / Bypass the simobuck trim latch to enable on-the-fly trimming for VDDF and VDDS active and LP trims"]
    #[inline(always)]
    pub fn trimlatchover(&self) -> TrimlatchoverR {
        TrimlatchoverR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 24:28 - Zxcomp offset trim. Feedthrough to analog."]
    #[inline(always)]
    #[must_use]
    pub fn zxcompoffsettrim(&mut self) -> ZxcompoffsettrimW<Simobuck15Spec> {
        ZxcompoffsettrimW::new(self, 24)
    }
    #[doc = "Bit 31 - Override / Bypass the simobuck trim latch to enable on-the-fly trimming for VDDF and VDDS active and LP trims"]
    #[inline(always)]
    #[must_use]
    pub fn trimlatchover(&mut self) -> TrimlatchoverW<Simobuck15Spec> {
        TrimlatchoverW::new(self, 31)
    }
}
#[doc = "SIMO Buck Compare, Brown out, Active and Low power Trim Control\n\nYou can [`read`](crate::Reg::read) this register and get [`simobuck15::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`simobuck15::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Simobuck15Spec;
impl crate::RegisterSpec for Simobuck15Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`simobuck15::R`](R) reader structure"]
impl crate::Readable for Simobuck15Spec {}
#[doc = "`write(|w| ..)` method takes [`simobuck15::W`](W) writer structure"]
impl crate::Writable for Simobuck15Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SIMOBUCK15 to value 0"]
impl crate::Resettable for Simobuck15Spec {
    const RESET_VALUE: u32 = 0;
}
