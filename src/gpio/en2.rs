#[doc = "Register `EN2` reader"]
pub type R = crate::R<En2Spec>;
#[doc = "Register `EN2` writer"]
pub type W = crate::W<En2Spec>;
#[doc = "Field `EN2` reader - GPIO95-64 Enables tri-state pin output. Writing a 1 to any bit enables, and writing a 0 to any bit disables, the output for the corresponding GPIO. Reads return output enable/disable status of GPIO."]
pub type En2R = crate::FieldReader<u32>;
#[doc = "Field `EN2` writer - GPIO95-64 Enables tri-state pin output. Writing a 1 to any bit enables, and writing a 0 to any bit disables, the output for the corresponding GPIO. Reads return output enable/disable status of GPIO."]
pub type En2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - GPIO95-64 Enables tri-state pin output. Writing a 1 to any bit enables, and writing a 0 to any bit disables, the output for the corresponding GPIO. Reads return output enable/disable status of GPIO."]
    #[inline(always)]
    pub fn en2(&self) -> En2R {
        En2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - GPIO95-64 Enables tri-state pin output. Writing a 1 to any bit enables, and writing a 0 to any bit disables, the output for the corresponding GPIO. Reads return output enable/disable status of GPIO."]
    #[inline(always)]
    #[must_use]
    pub fn en2(&mut self) -> En2W<En2Spec> {
        En2W::new(self, 0)
    }
}
#[doc = "GPIO Enable 2 (95-64)\n\nYou can [`read`](crate::Reg::read) this register and get [`en2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`en2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct En2Spec;
impl crate::RegisterSpec for En2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`en2::R`](R) reader structure"]
impl crate::Readable for En2Spec {}
#[doc = "`write(|w| ..)` method takes [`en2::W`](W) writer structure"]
impl crate::Writable for En2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EN2 to value 0"]
impl crate::Resettable for En2Spec {
    const RESET_VALUE: u32 = 0;
}
