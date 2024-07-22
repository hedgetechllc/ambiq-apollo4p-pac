#[doc = "Register `WTS1` reader"]
pub type R = crate::R<Wts1Spec>;
#[doc = "Register `WTS1` writer"]
pub type W = crate::W<Wts1Spec>;
#[doc = "Field `WTS1` reader - GPIO63-32 Sets pin state. Writing a 1 to any bit sets the corresponding bit in the WT register if the GPIO is enabled for output. Writing a value of 0 has no effect on the corresponding bit in the WT register. Status reads should be made via the WT Register."]
pub type Wts1R = crate::FieldReader<u32>;
#[doc = "Field `WTS1` writer - GPIO63-32 Sets pin state. Writing a 1 to any bit sets the corresponding bit in the WT register if the GPIO is enabled for output. Writing a value of 0 has no effect on the corresponding bit in the WT register. Status reads should be made via the WT Register."]
pub type Wts1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - GPIO63-32 Sets pin state. Writing a 1 to any bit sets the corresponding bit in the WT register if the GPIO is enabled for output. Writing a value of 0 has no effect on the corresponding bit in the WT register. Status reads should be made via the WT Register."]
    #[inline(always)]
    pub fn wts1(&self) -> Wts1R {
        Wts1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - GPIO63-32 Sets pin state. Writing a 1 to any bit sets the corresponding bit in the WT register if the GPIO is enabled for output. Writing a value of 0 has no effect on the corresponding bit in the WT register. Status reads should be made via the WT Register."]
    #[inline(always)]
    #[must_use]
    pub fn wts1(&mut self) -> Wts1W<Wts1Spec> {
        Wts1W::new(self, 0)
    }
}
#[doc = "GPIO Output Set 1 (63-32)\n\nYou can [`read`](crate::Reg::read) this register and get [`wts1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wts1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wts1Spec;
impl crate::RegisterSpec for Wts1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wts1::R`](R) reader structure"]
impl crate::Readable for Wts1Spec {}
#[doc = "`write(|w| ..)` method takes [`wts1::W`](W) writer structure"]
impl crate::Writable for Wts1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WTS1 to value 0"]
impl crate::Resettable for Wts1Spec {
    const RESET_VALUE: u32 = 0;
}
