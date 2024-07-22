#[doc = "Register `SIMOBUCK8` reader"]
pub type R = crate::R<Simobuck8Spec>;
#[doc = "Register `SIMOBUCK8` writer"]
pub type W = crate::W<Simobuck8Spec>;
#[doc = "Field `VDDFLPHIGHTONTRIM` reader - VDDF low power high ton trim control for Buck sequence."]
pub type VddflphightontrimR = crate::FieldReader;
#[doc = "Field `VDDFLPHIGHTONTRIM` writer - VDDF low power high ton trim control for Buck sequence."]
pub type VddflphightontrimW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `VDDFLPLOWTONTRIM` reader - VDDF low power low ton trim control for Buck sequence."]
pub type VddflplowtontrimR = crate::FieldReader;
#[doc = "Field `VDDFLPLOWTONTRIM` writer - VDDF low power low ton trim control for Buck sequence."]
pub type VddflplowtontrimW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 9:12 - VDDF low power high ton trim control for Buck sequence."]
    #[inline(always)]
    pub fn vddflphightontrim(&self) -> VddflphightontrimR {
        VddflphightontrimR::new(((self.bits >> 9) & 0x0f) as u8)
    }
    #[doc = "Bits 22:25 - VDDF low power low ton trim control for Buck sequence."]
    #[inline(always)]
    pub fn vddflplowtontrim(&self) -> VddflplowtontrimR {
        VddflplowtontrimR::new(((self.bits >> 22) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 9:12 - VDDF low power high ton trim control for Buck sequence."]
    #[inline(always)]
    #[must_use]
    pub fn vddflphightontrim(&mut self) -> VddflphightontrimW<Simobuck8Spec> {
        VddflphightontrimW::new(self, 9)
    }
    #[doc = "Bits 22:25 - VDDF low power low ton trim control for Buck sequence."]
    #[inline(always)]
    #[must_use]
    pub fn vddflplowtontrim(&mut self) -> VddflplowtontrimW<Simobuck8Spec> {
        VddflplowtontrimW::new(self, 22)
    }
}
#[doc = "SIMO Buck Muxed VDDF Low Power Sequence Trim Control\n\nYou can [`read`](crate::Reg::read) this register and get [`simobuck8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`simobuck8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Simobuck8Spec;
impl crate::RegisterSpec for Simobuck8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`simobuck8::R`](R) reader structure"]
impl crate::Readable for Simobuck8Spec {}
#[doc = "`write(|w| ..)` method takes [`simobuck8::W`](W) writer structure"]
impl crate::Writable for Simobuck8Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SIMOBUCK8 to value 0"]
impl crate::Resettable for Simobuck8Spec {
    const RESET_VALUE: u32 = 0;
}
