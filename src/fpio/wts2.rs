#[doc = "Register `WTS2` reader"]
pub type R = crate::R<Wts2Spec>;
#[doc = "Register `WTS2` writer"]
pub type W = crate::W<Wts2Spec>;
#[doc = "Field `WTS2` reader - GPIO95-64 Sets pin state. Writing a 1 to any bit sets the corresponding bit in the WT register if the GPIO is enabled for output. Writing a value of 0 has no effect on the corresponding bit in the WT register. Status reads should be made via the WT Register."]
pub type Wts2R = crate::FieldReader<u32>;
#[doc = "Field `WTS2` writer - GPIO95-64 Sets pin state. Writing a 1 to any bit sets the corresponding bit in the WT register if the GPIO is enabled for output. Writing a value of 0 has no effect on the corresponding bit in the WT register. Status reads should be made via the WT Register."]
pub type Wts2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - GPIO95-64 Sets pin state. Writing a 1 to any bit sets the corresponding bit in the WT register if the GPIO is enabled for output. Writing a value of 0 has no effect on the corresponding bit in the WT register. Status reads should be made via the WT Register."]
    #[inline(always)]
    pub fn wts2(&self) -> Wts2R {
        Wts2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - GPIO95-64 Sets pin state. Writing a 1 to any bit sets the corresponding bit in the WT register if the GPIO is enabled for output. Writing a value of 0 has no effect on the corresponding bit in the WT register. Status reads should be made via the WT Register."]
    #[inline(always)]
    #[must_use]
    pub fn wts2(&mut self) -> Wts2W<Wts2Spec> {
        Wts2W::new(self, 0)
    }
}
#[doc = "GPIO Output Set 2 (95-64)\n\nYou can [`read`](crate::Reg::read) this register and get [`wts2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wts2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wts2Spec;
impl crate::RegisterSpec for Wts2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wts2::R`](R) reader structure"]
impl crate::Readable for Wts2Spec {}
#[doc = "`write(|w| ..)` method takes [`wts2::W`](W) writer structure"]
impl crate::Writable for Wts2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WTS2 to value 0"]
impl crate::Resettable for Wts2Spec {
    const RESET_VALUE: u32 = 0;
}
