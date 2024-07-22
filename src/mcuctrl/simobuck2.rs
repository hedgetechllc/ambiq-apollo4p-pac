#[doc = "Register `SIMOBUCK2` reader"]
pub type R = crate::R<Simobuck2Spec>;
#[doc = "Register `SIMOBUCK2` writer"]
pub type W = crate::W<Simobuck2Spec>;
#[doc = "Field `VDDCACTHIGHTONTRIM` reader - VDDC active high ton trim control for Buck sequence."]
pub type VddcacthightontrimR = crate::FieldReader;
#[doc = "Field `VDDCACTHIGHTONTRIM` writer - VDDC active high ton trim control for Buck sequence."]
pub type VddcacthightontrimW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `VDDCACTLOWTONTRIM` reader - VDDC active high ton trim control for Buck sequence."]
pub type VddcactlowtontrimR = crate::FieldReader;
#[doc = "Field `VDDCACTLOWTONTRIM` writer - VDDC active high ton trim control for Buck sequence."]
pub type VddcactlowtontrimW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 11:14 - VDDC active high ton trim control for Buck sequence."]
    #[inline(always)]
    pub fn vddcacthightontrim(&self) -> VddcacthightontrimR {
        VddcacthightontrimR::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bits 24:28 - VDDC active high ton trim control for Buck sequence."]
    #[inline(always)]
    pub fn vddcactlowtontrim(&self) -> VddcactlowtontrimR {
        VddcactlowtontrimR::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 11:14 - VDDC active high ton trim control for Buck sequence."]
    #[inline(always)]
    #[must_use]
    pub fn vddcacthightontrim(&mut self) -> VddcacthightontrimW<Simobuck2Spec> {
        VddcacthightontrimW::new(self, 11)
    }
    #[doc = "Bits 24:28 - VDDC active high ton trim control for Buck sequence."]
    #[inline(always)]
    #[must_use]
    pub fn vddcactlowtontrim(&mut self) -> VddcactlowtontrimW<Simobuck2Spec> {
        VddcactlowtontrimW::new(self, 24)
    }
}
#[doc = "SIMO Buck Muxed VDDC Active Sequence Trim Control\n\nYou can [`read`](crate::Reg::read) this register and get [`simobuck2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`simobuck2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Simobuck2Spec;
impl crate::RegisterSpec for Simobuck2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`simobuck2::R`](R) reader structure"]
impl crate::Readable for Simobuck2Spec {}
#[doc = "`write(|w| ..)` method takes [`simobuck2::W`](W) writer structure"]
impl crate::Writable for Simobuck2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SIMOBUCK2 to value 0"]
impl crate::Resettable for Simobuck2Spec {
    const RESET_VALUE: u32 = 0;
}
