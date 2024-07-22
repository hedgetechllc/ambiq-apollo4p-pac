#[doc = "Register `SIMOBUCK3` reader"]
pub type R = crate::R<Simobuck3Spec>;
#[doc = "Register `SIMOBUCK3` writer"]
pub type W = crate::W<Simobuck3Spec>;
#[doc = "Field `VDDCLPHIGHTONTRIM` reader - VDDC LP high ton trim control for Buck sequence."]
pub type VddclphightontrimR = crate::FieldReader;
#[doc = "Field `VDDCLPHIGHTONTRIM` writer - VDDC LP high ton trim control for Buck sequence."]
pub type VddclphightontrimW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `VDDCLPLOWTONTRIM` reader - VDDC LP low ton trim control for Buck sequence."]
pub type VddclplowtontrimR = crate::FieldReader;
#[doc = "Field `VDDCLPLOWTONTRIM` writer - VDDC LP low ton trim control for Buck sequence."]
pub type VddclplowtontrimW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 13:16 - VDDC LP high ton trim control for Buck sequence."]
    #[inline(always)]
    pub fn vddclphightontrim(&self) -> VddclphightontrimR {
        VddclphightontrimR::new(((self.bits >> 13) & 0x0f) as u8)
    }
    #[doc = "Bits 26:29 - VDDC LP low ton trim control for Buck sequence."]
    #[inline(always)]
    pub fn vddclplowtontrim(&self) -> VddclplowtontrimR {
        VddclplowtontrimR::new(((self.bits >> 26) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 13:16 - VDDC LP high ton trim control for Buck sequence."]
    #[inline(always)]
    #[must_use]
    pub fn vddclphightontrim(&mut self) -> VddclphightontrimW<Simobuck3Spec> {
        VddclphightontrimW::new(self, 13)
    }
    #[doc = "Bits 26:29 - VDDC LP low ton trim control for Buck sequence."]
    #[inline(always)]
    #[must_use]
    pub fn vddclplowtontrim(&mut self) -> VddclplowtontrimW<Simobuck3Spec> {
        VddclplowtontrimW::new(self, 26)
    }
}
#[doc = "SIMO Buck Muxed VDDC low power Sequence Trim Control\n\nYou can [`read`](crate::Reg::read) this register and get [`simobuck3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`simobuck3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Simobuck3Spec;
impl crate::RegisterSpec for Simobuck3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`simobuck3::R`](R) reader structure"]
impl crate::Readable for Simobuck3Spec {}
#[doc = "`write(|w| ..)` method takes [`simobuck3::W`](W) writer structure"]
impl crate::Writable for Simobuck3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SIMOBUCK3 to value 0"]
impl crate::Resettable for Simobuck3Spec {
    const RESET_VALUE: u32 = 0;
}
