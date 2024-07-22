#[doc = "Register `SIMOBUCK6` reader"]
pub type R = crate::R<Simobuck6Spec>;
#[doc = "Register `SIMOBUCK6` writer"]
pub type W = crate::W<Simobuck6Spec>;
#[doc = "Field `VDDFACTHIGHTONTRIM` reader - VDDF active high ton trim control for Buck sequence."]
pub type VddfacthightontrimR = crate::FieldReader;
#[doc = "Field `VDDFACTHIGHTONTRIM` writer - VDDF active high ton trim control for Buck sequence."]
pub type VddfacthightontrimW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 17:20 - VDDF active high ton trim control for Buck sequence."]
    #[inline(always)]
    pub fn vddfacthightontrim(&self) -> VddfacthightontrimR {
        VddfacthightontrimR::new(((self.bits >> 17) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 17:20 - VDDF active high ton trim control for Buck sequence."]
    #[inline(always)]
    #[must_use]
    pub fn vddfacthightontrim(&mut self) -> VddfacthightontrimW<Simobuck6Spec> {
        VddfacthightontrimW::new(self, 17)
    }
}
#[doc = "SIMO Buck Muxed VDDF Active Sequence Trim Control\n\nYou can [`read`](crate::Reg::read) this register and get [`simobuck6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`simobuck6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Simobuck6Spec;
impl crate::RegisterSpec for Simobuck6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`simobuck6::R`](R) reader structure"]
impl crate::Readable for Simobuck6Spec {}
#[doc = "`write(|w| ..)` method takes [`simobuck6::W`](W) writer structure"]
impl crate::Writable for Simobuck6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SIMOBUCK6 to value 0"]
impl crate::Resettable for Simobuck6Spec {
    const RESET_VALUE: u32 = 0;
}
