#[doc = "Register `WTC0` reader"]
pub type R = crate::R<Wtc0Spec>;
#[doc = "Register `WTC0` writer"]
pub type W = crate::W<Wtc0Spec>;
#[doc = "Field `WTC0` reader - GPIO31-0 Clears pin state. Writing a 1 to any bit clears the corresponding bit in the WT register if the GPIO is enabled for output. Writing a value of 0 has no effect on the corresponding bit in the WT register. Status reads should be made via the WT register."]
pub type Wtc0R = crate::FieldReader<u32>;
#[doc = "Field `WTC0` writer - GPIO31-0 Clears pin state. Writing a 1 to any bit clears the corresponding bit in the WT register if the GPIO is enabled for output. Writing a value of 0 has no effect on the corresponding bit in the WT register. Status reads should be made via the WT register."]
pub type Wtc0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - GPIO31-0 Clears pin state. Writing a 1 to any bit clears the corresponding bit in the WT register if the GPIO is enabled for output. Writing a value of 0 has no effect on the corresponding bit in the WT register. Status reads should be made via the WT register."]
    #[inline(always)]
    pub fn wtc0(&self) -> Wtc0R {
        Wtc0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - GPIO31-0 Clears pin state. Writing a 1 to any bit clears the corresponding bit in the WT register if the GPIO is enabled for output. Writing a value of 0 has no effect on the corresponding bit in the WT register. Status reads should be made via the WT register."]
    #[inline(always)]
    #[must_use]
    pub fn wtc0(&mut self) -> Wtc0W<Wtc0Spec> {
        Wtc0W::new(self, 0)
    }
}
#[doc = "GPIO Output Clear 0 (31-0)\n\nYou can [`read`](crate::Reg::read) this register and get [`wtc0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wtc0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wtc0Spec;
impl crate::RegisterSpec for Wtc0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wtc0::R`](R) reader structure"]
impl crate::Readable for Wtc0Spec {}
#[doc = "`write(|w| ..)` method takes [`wtc0::W`](W) writer structure"]
impl crate::Writable for Wtc0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WTC0 to value 0"]
impl crate::Resettable for Wtc0Spec {
    const RESET_VALUE: u32 = 0;
}
