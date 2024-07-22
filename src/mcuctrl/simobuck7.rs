#[doc = "Register `SIMOBUCK7` reader"]
pub type R = crate::R<Simobuck7Spec>;
#[doc = "Register `SIMOBUCK7` writer"]
pub type W = crate::W<Simobuck7Spec>;
#[doc = "Field `VDDFACTLOWTONTRIM` reader - VDDF active low ton trim control for Buck sequence."]
pub type VddfactlowtontrimR = crate::FieldReader;
#[doc = "Field `VDDFACTLOWTONTRIM` writer - VDDF active low ton trim control for Buck sequence."]
pub type VddfactlowtontrimW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ZXCOMPZXTRIM` reader - Zxcomp trim. Feedthrough to analog."]
pub type ZxcompzxtrimR = crate::FieldReader;
#[doc = "Field `ZXCOMPZXTRIM` writer - Zxcomp trim. Feedthrough to analog."]
pub type ZxcompzxtrimW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 8:12 - VDDF active low ton trim control for Buck sequence."]
    #[inline(always)]
    pub fn vddfactlowtontrim(&self) -> VddfactlowtontrimR {
        VddfactlowtontrimR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 18:22 - Zxcomp trim. Feedthrough to analog."]
    #[inline(always)]
    pub fn zxcompzxtrim(&self) -> ZxcompzxtrimR {
        ZxcompzxtrimR::new(((self.bits >> 18) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:12 - VDDF active low ton trim control for Buck sequence."]
    #[inline(always)]
    #[must_use]
    pub fn vddfactlowtontrim(&mut self) -> VddfactlowtontrimW<Simobuck7Spec> {
        VddfactlowtontrimW::new(self, 8)
    }
    #[doc = "Bits 18:22 - Zxcomp trim. Feedthrough to analog."]
    #[inline(always)]
    #[must_use]
    pub fn zxcompzxtrim(&mut self) -> ZxcompzxtrimW<Simobuck7Spec> {
        ZxcompzxtrimW::new(self, 18)
    }
}
#[doc = "SIMO Buck Muxed VDDF active Sequence Trim Control\n\nYou can [`read`](crate::Reg::read) this register and get [`simobuck7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`simobuck7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Simobuck7Spec;
impl crate::RegisterSpec for Simobuck7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`simobuck7::R`](R) reader structure"]
impl crate::Readable for Simobuck7Spec {}
#[doc = "`write(|w| ..)` method takes [`simobuck7::W`](W) writer structure"]
impl crate::Writable for Simobuck7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SIMOBUCK7 to value 0"]
impl crate::Resettable for Simobuck7Spec {
    const RESET_VALUE: u32 = 0;
}
