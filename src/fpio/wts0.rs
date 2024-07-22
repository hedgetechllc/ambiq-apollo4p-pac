#[doc = "Register `WTS0` reader"]
pub type R = crate::R<Wts0Spec>;
#[doc = "Register `WTS0` writer"]
pub type W = crate::W<Wts0Spec>;
#[doc = "Field `WTS0` reader - GPIO31-0 Sets pin state. Writing a 1 to any bit sets the corresponding bit in the WT register if the GPIO is enabled for output. Writing a value of 0 has no effect on the corresponding bit in the WT register. Status reads should be made via the WT Register."]
pub type Wts0R = crate::FieldReader<u32>;
#[doc = "Field `WTS0` writer - GPIO31-0 Sets pin state. Writing a 1 to any bit sets the corresponding bit in the WT register if the GPIO is enabled for output. Writing a value of 0 has no effect on the corresponding bit in the WT register. Status reads should be made via the WT Register."]
pub type Wts0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - GPIO31-0 Sets pin state. Writing a 1 to any bit sets the corresponding bit in the WT register if the GPIO is enabled for output. Writing a value of 0 has no effect on the corresponding bit in the WT register. Status reads should be made via the WT Register."]
    #[inline(always)]
    pub fn wts0(&self) -> Wts0R {
        Wts0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - GPIO31-0 Sets pin state. Writing a 1 to any bit sets the corresponding bit in the WT register if the GPIO is enabled for output. Writing a value of 0 has no effect on the corresponding bit in the WT register. Status reads should be made via the WT Register."]
    #[inline(always)]
    #[must_use]
    pub fn wts0(&mut self) -> Wts0W<Wts0Spec> {
        Wts0W::new(self, 0)
    }
}
#[doc = "GPIO Output Set 0 (31-0)\n\nYou can [`read`](crate::Reg::read) this register and get [`wts0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wts0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wts0Spec;
impl crate::RegisterSpec for Wts0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wts0::R`](R) reader structure"]
impl crate::Readable for Wts0Spec {}
#[doc = "`write(|w| ..)` method takes [`wts0::W`](W) writer structure"]
impl crate::Writable for Wts0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WTS0 to value 0"]
impl crate::Resettable for Wts0Spec {
    const RESET_VALUE: u32 = 0;
}
