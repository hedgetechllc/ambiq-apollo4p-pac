#[doc = "Register `WTC2` reader"]
pub type R = crate::R<Wtc2Spec>;
#[doc = "Register `WTC2` writer"]
pub type W = crate::W<Wtc2Spec>;
#[doc = "Field `WTC2` reader - GPIO95-64 Clears pin state. Writing a 1 to any bit clears the corresponding bit in the WT register if the GPIO is enabled for output. Writing a value of 0 has no effect on the corresponding bit in the WT register. Status reads should be made via the WT register."]
pub type Wtc2R = crate::FieldReader<u32>;
#[doc = "Field `WTC2` writer - GPIO95-64 Clears pin state. Writing a 1 to any bit clears the corresponding bit in the WT register if the GPIO is enabled for output. Writing a value of 0 has no effect on the corresponding bit in the WT register. Status reads should be made via the WT register."]
pub type Wtc2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - GPIO95-64 Clears pin state. Writing a 1 to any bit clears the corresponding bit in the WT register if the GPIO is enabled for output. Writing a value of 0 has no effect on the corresponding bit in the WT register. Status reads should be made via the WT register."]
    #[inline(always)]
    pub fn wtc2(&self) -> Wtc2R {
        Wtc2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - GPIO95-64 Clears pin state. Writing a 1 to any bit clears the corresponding bit in the WT register if the GPIO is enabled for output. Writing a value of 0 has no effect on the corresponding bit in the WT register. Status reads should be made via the WT register."]
    #[inline(always)]
    #[must_use]
    pub fn wtc2(&mut self) -> Wtc2W<Wtc2Spec> {
        Wtc2W::new(self, 0)
    }
}
#[doc = "GPIO Output Clear 2 (95-64)\n\nYou can [`read`](crate::Reg::read) this register and get [`wtc2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wtc2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wtc2Spec;
impl crate::RegisterSpec for Wtc2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wtc2::R`](R) reader structure"]
impl crate::Readable for Wtc2Spec {}
#[doc = "`write(|w| ..)` method takes [`wtc2::W`](W) writer structure"]
impl crate::Writable for Wtc2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WTC2 to value 0"]
impl crate::Resettable for Wtc2Spec {
    const RESET_VALUE: u32 = 0;
}
