#[doc = "Register `EHRDATA3` reader"]
pub type R = crate::R<Ehrdata3Spec>;
#[doc = "Register `EHRDATA3` writer"]
pub type W = crate::W<Ehrdata3Spec>;
#[doc = "Field `EHRDATA` reader - Contains the data collected in the TRNG\\[127_96\\]. Note: can only be set while in debug mode (rng_debug_enable input is set)."]
pub type EhrdataR = crate::FieldReader<u32>;
#[doc = "Field `EHRDATA` writer - Contains the data collected in the TRNG\\[127_96\\]. Note: can only be set while in debug mode (rng_debug_enable input is set)."]
pub type EhrdataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Contains the data collected in the TRNG\\[127_96\\]. Note: can only be set while in debug mode (rng_debug_enable input is set)."]
    #[inline(always)]
    pub fn ehrdata(&self) -> EhrdataR {
        EhrdataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Contains the data collected in the TRNG\\[127_96\\]. Note: can only be set while in debug mode (rng_debug_enable input is set)."]
    #[inline(always)]
    #[must_use]
    pub fn ehrdata(&mut self) -> EhrdataW<Ehrdata3Spec> {
        EhrdataW::new(self, 0)
    }
}
#[doc = "This register contains the data collected in the TRNG\\[127_96\\]. Note: can only be set while in debug mode (rng_debug_enable input is set).\n\nYou can [`read`](crate::Reg::read) this register and get [`ehrdata3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ehrdata3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ehrdata3Spec;
impl crate::RegisterSpec for Ehrdata3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ehrdata3::R`](R) reader structure"]
impl crate::Readable for Ehrdata3Spec {}
#[doc = "`write(|w| ..)` method takes [`ehrdata3::W`](W) writer structure"]
impl crate::Writable for Ehrdata3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EHRDATA3 to value 0"]
impl crate::Resettable for Ehrdata3Spec {
    const RESET_VALUE: u32 = 0;
}
