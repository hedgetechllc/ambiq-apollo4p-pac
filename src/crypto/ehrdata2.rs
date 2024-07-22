#[doc = "Register `EHRDATA2` reader"]
pub type R = crate::R<Ehrdata2Spec>;
#[doc = "Register `EHRDATA2` writer"]
pub type W = crate::W<Ehrdata2Spec>;
#[doc = "Field `EHRDATA` reader - Contains the data collected in the TRNG\\[95_64\\]. Note: can only be set while in debug mode (rng_debug_enable input is set)."]
pub type EhrdataR = crate::FieldReader<u32>;
#[doc = "Field `EHRDATA` writer - Contains the data collected in the TRNG\\[95_64\\]. Note: can only be set while in debug mode (rng_debug_enable input is set)."]
pub type EhrdataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Contains the data collected in the TRNG\\[95_64\\]. Note: can only be set while in debug mode (rng_debug_enable input is set)."]
    #[inline(always)]
    pub fn ehrdata(&self) -> EhrdataR {
        EhrdataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Contains the data collected in the TRNG\\[95_64\\]. Note: can only be set while in debug mode (rng_debug_enable input is set)."]
    #[inline(always)]
    #[must_use]
    pub fn ehrdata(&mut self) -> EhrdataW<Ehrdata2Spec> {
        EhrdataW::new(self, 0)
    }
}
#[doc = "This register contains the data collected in the TRNG\\[95_64\\]. Note: can only be set while in debug mode (rng_debug_enable input is set).\n\nYou can [`read`](crate::Reg::read) this register and get [`ehrdata2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ehrdata2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ehrdata2Spec;
impl crate::RegisterSpec for Ehrdata2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ehrdata2::R`](R) reader structure"]
impl crate::Readable for Ehrdata2Spec {}
#[doc = "`write(|w| ..)` method takes [`ehrdata2::W`](W) writer structure"]
impl crate::Writable for Ehrdata2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EHRDATA2 to value 0"]
impl crate::Resettable for Ehrdata2Spec {
    const RESET_VALUE: u32 = 0;
}
