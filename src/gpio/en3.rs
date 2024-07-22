#[doc = "Register `EN3` reader"]
pub type R = crate::R<En3Spec>;
#[doc = "Register `EN3` writer"]
pub type W = crate::W<En3Spec>;
#[doc = "Field `EN3` reader - GPIO127-96 Enables tri-state pin output. Writing a 1 to any bit enables, and writing a 0 to any bit disables, the output for the corresponding GPIO. Reads return output enable/disable status of GPIO."]
pub type En3R = crate::FieldReader<u32>;
#[doc = "Field `EN3` writer - GPIO127-96 Enables tri-state pin output. Writing a 1 to any bit enables, and writing a 0 to any bit disables, the output for the corresponding GPIO. Reads return output enable/disable status of GPIO."]
pub type En3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - GPIO127-96 Enables tri-state pin output. Writing a 1 to any bit enables, and writing a 0 to any bit disables, the output for the corresponding GPIO. Reads return output enable/disable status of GPIO."]
    #[inline(always)]
    pub fn en3(&self) -> En3R {
        En3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - GPIO127-96 Enables tri-state pin output. Writing a 1 to any bit enables, and writing a 0 to any bit disables, the output for the corresponding GPIO. Reads return output enable/disable status of GPIO."]
    #[inline(always)]
    #[must_use]
    pub fn en3(&mut self) -> En3W<En3Spec> {
        En3W::new(self, 0)
    }
}
#[doc = "GPIO Enable 3 (127-96)\n\nYou can [`read`](crate::Reg::read) this register and get [`en3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`en3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct En3Spec;
impl crate::RegisterSpec for En3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`en3::R`](R) reader structure"]
impl crate::Readable for En3Spec {}
#[doc = "`write(|w| ..)` method takes [`en3::W`](W) writer structure"]
impl crate::Writable for En3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EN3 to value 0"]
impl crate::Resettable for En3Spec {
    const RESET_VALUE: u32 = 0;
}
