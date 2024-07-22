#[doc = "Register `WTS3` reader"]
pub type R = crate::R<Wts3Spec>;
#[doc = "Register `WTS3` writer"]
pub type W = crate::W<Wts3Spec>;
#[doc = "Field `WTS3` reader - GPIO127-96 Sets pin state. Writing a 1 to any bit sets the corresponding bit in the WT register if the GPIO is enabled for output. Writing a value of 0 has no effect on the corresponding bit in the WT register. Status reads should be made via the WT Register."]
pub type Wts3R = crate::FieldReader<u32>;
#[doc = "Field `WTS3` writer - GPIO127-96 Sets pin state. Writing a 1 to any bit sets the corresponding bit in the WT register if the GPIO is enabled for output. Writing a value of 0 has no effect on the corresponding bit in the WT register. Status reads should be made via the WT Register."]
pub type Wts3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - GPIO127-96 Sets pin state. Writing a 1 to any bit sets the corresponding bit in the WT register if the GPIO is enabled for output. Writing a value of 0 has no effect on the corresponding bit in the WT register. Status reads should be made via the WT Register."]
    #[inline(always)]
    pub fn wts3(&self) -> Wts3R {
        Wts3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - GPIO127-96 Sets pin state. Writing a 1 to any bit sets the corresponding bit in the WT register if the GPIO is enabled for output. Writing a value of 0 has no effect on the corresponding bit in the WT register. Status reads should be made via the WT Register."]
    #[inline(always)]
    #[must_use]
    pub fn wts3(&mut self) -> Wts3W<Wts3Spec> {
        Wts3W::new(self, 0)
    }
}
#[doc = "GPIO Output Set 3 (127-96)\n\nYou can [`read`](crate::Reg::read) this register and get [`wts3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wts3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wts3Spec;
impl crate::RegisterSpec for Wts3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wts3::R`](R) reader structure"]
impl crate::Readable for Wts3Spec {}
#[doc = "`write(|w| ..)` method takes [`wts3::W`](W) writer structure"]
impl crate::Writable for Wts3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WTS3 to value 0"]
impl crate::Resettable for Wts3Spec {
    const RESET_VALUE: u32 = 0;
}
