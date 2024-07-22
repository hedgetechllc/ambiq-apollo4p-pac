#[doc = "Register `WTC3` reader"]
pub type R = crate::R<Wtc3Spec>;
#[doc = "Register `WTC3` writer"]
pub type W = crate::W<Wtc3Spec>;
#[doc = "Field `WTC3` reader - GPIO127-96 Clears pin state. Writing a 1 to any bit clears the corresponding bit in the WT register if the GPIO is enabled for output. Writing a value of 0 has no effect on the corresponding bit in the WT register. Status reads should be made via the WT register."]
pub type Wtc3R = crate::FieldReader<u32>;
#[doc = "Field `WTC3` writer - GPIO127-96 Clears pin state. Writing a 1 to any bit clears the corresponding bit in the WT register if the GPIO is enabled for output. Writing a value of 0 has no effect on the corresponding bit in the WT register. Status reads should be made via the WT register."]
pub type Wtc3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - GPIO127-96 Clears pin state. Writing a 1 to any bit clears the corresponding bit in the WT register if the GPIO is enabled for output. Writing a value of 0 has no effect on the corresponding bit in the WT register. Status reads should be made via the WT register."]
    #[inline(always)]
    pub fn wtc3(&self) -> Wtc3R {
        Wtc3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - GPIO127-96 Clears pin state. Writing a 1 to any bit clears the corresponding bit in the WT register if the GPIO is enabled for output. Writing a value of 0 has no effect on the corresponding bit in the WT register. Status reads should be made via the WT register."]
    #[inline(always)]
    #[must_use]
    pub fn wtc3(&mut self) -> Wtc3W<Wtc3Spec> {
        Wtc3W::new(self, 0)
    }
}
#[doc = "GPIO Output Clear 3 (127-96)\n\nYou can [`read`](crate::Reg::read) this register and get [`wtc3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wtc3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wtc3Spec;
impl crate::RegisterSpec for Wtc3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wtc3::R`](R) reader structure"]
impl crate::Readable for Wtc3Spec {}
#[doc = "`write(|w| ..)` method takes [`wtc3::W`](W) writer structure"]
impl crate::Writable for Wtc3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WTC3 to value 0"]
impl crate::Resettable for Wtc3Spec {
    const RESET_VALUE: u32 = 0;
}
