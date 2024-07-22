#[doc = "Register `WTC1` reader"]
pub type R = crate::R<Wtc1Spec>;
#[doc = "Register `WTC1` writer"]
pub type W = crate::W<Wtc1Spec>;
#[doc = "Field `WTC1` reader - GPIO63-32 Clears pin state. Writing a 1 to any bit clears the corresponding bit in the WT register if the GPIO is enabled for output. Writing a value of 0 has no effect on the corresponding bit in the WT register. Status reads should be made via the WT register."]
pub type Wtc1R = crate::FieldReader<u32>;
#[doc = "Field `WTC1` writer - GPIO63-32 Clears pin state. Writing a 1 to any bit clears the corresponding bit in the WT register if the GPIO is enabled for output. Writing a value of 0 has no effect on the corresponding bit in the WT register. Status reads should be made via the WT register."]
pub type Wtc1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - GPIO63-32 Clears pin state. Writing a 1 to any bit clears the corresponding bit in the WT register if the GPIO is enabled for output. Writing a value of 0 has no effect on the corresponding bit in the WT register. Status reads should be made via the WT register."]
    #[inline(always)]
    pub fn wtc1(&self) -> Wtc1R {
        Wtc1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - GPIO63-32 Clears pin state. Writing a 1 to any bit clears the corresponding bit in the WT register if the GPIO is enabled for output. Writing a value of 0 has no effect on the corresponding bit in the WT register. Status reads should be made via the WT register."]
    #[inline(always)]
    #[must_use]
    pub fn wtc1(&mut self) -> Wtc1W<Wtc1Spec> {
        Wtc1W::new(self, 0)
    }
}
#[doc = "GPIO Output Clear 1 (63-32)\n\nYou can [`read`](crate::Reg::read) this register and get [`wtc1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wtc1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wtc1Spec;
impl crate::RegisterSpec for Wtc1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wtc1::R`](R) reader structure"]
impl crate::Readable for Wtc1Spec {}
#[doc = "`write(|w| ..)` method takes [`wtc1::W`](W) writer structure"]
impl crate::Writable for Wtc1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WTC1 to value 0"]
impl crate::Resettable for Wtc1Spec {
    const RESET_VALUE: u32 = 0;
}
