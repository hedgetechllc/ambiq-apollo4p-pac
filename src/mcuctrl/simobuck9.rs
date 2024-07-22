#[doc = "Register `SIMOBUCK9` reader"]
pub type R = crate::R<Simobuck9Spec>;
#[doc = "Register `SIMOBUCK9` writer"]
pub type W = crate::W<Simobuck9Spec>;
#[doc = "Field `VDDSACTHIGHTONTRIM` reader - VDDS active high ton trim control for Buck sequence."]
pub type VddsacthightontrimR = crate::FieldReader;
#[doc = "Field `VDDSACTHIGHTONTRIM` writer - VDDS active high ton trim control for Buck sequence."]
pub type VddsacthightontrimW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `VDDSACTLOWTONTRIM` reader - VDDS active low ton trim control for Buck sequence."]
pub type VddsactlowtontrimR = crate::FieldReader;
#[doc = "Field `VDDSACTLOWTONTRIM` writer - VDDS active low ton trim control for Buck sequence."]
pub type VddsactlowtontrimW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 17:20 - VDDS active high ton trim control for Buck sequence."]
    #[inline(always)]
    pub fn vddsacthightontrim(&self) -> VddsacthightontrimR {
        VddsacthightontrimR::new(((self.bits >> 17) & 0x0f) as u8)
    }
    #[doc = "Bits 22:26 - VDDS active low ton trim control for Buck sequence."]
    #[inline(always)]
    pub fn vddsactlowtontrim(&self) -> VddsactlowtontrimR {
        VddsactlowtontrimR::new(((self.bits >> 22) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 17:20 - VDDS active high ton trim control for Buck sequence."]
    #[inline(always)]
    #[must_use]
    pub fn vddsacthightontrim(&mut self) -> VddsacthightontrimW<Simobuck9Spec> {
        VddsacthightontrimW::new(self, 17)
    }
    #[doc = "Bits 22:26 - VDDS active low ton trim control for Buck sequence."]
    #[inline(always)]
    #[must_use]
    pub fn vddsactlowtontrim(&mut self) -> VddsactlowtontrimW<Simobuck9Spec> {
        VddsactlowtontrimW::new(self, 22)
    }
}
#[doc = "SIMO Buck Muxed VDDS Active Sequence Trim Control\n\nYou can [`read`](crate::Reg::read) this register and get [`simobuck9::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`simobuck9::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Simobuck9Spec;
impl crate::RegisterSpec for Simobuck9Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`simobuck9::R`](R) reader structure"]
impl crate::Readable for Simobuck9Spec {}
#[doc = "`write(|w| ..)` method takes [`simobuck9::W`](W) writer structure"]
impl crate::Writable for Simobuck9Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SIMOBUCK9 to value 0"]
impl crate::Resettable for Simobuck9Spec {
    const RESET_VALUE: u32 = 0;
}
