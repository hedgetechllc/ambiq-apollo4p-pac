#[doc = "Register `EN1` reader"]
pub type R = crate::R<En1Spec>;
#[doc = "Register `EN1` writer"]
pub type W = crate::W<En1Spec>;
#[doc = "Field `EN1` reader - GPIO63-32 Enables tri-state pin output. Writing a 1 to any bit enables, and writing a 0 to any bit disables, the output for the corresponding GPIO. Reads return output enable/disable status of GPIO."]
pub type En1R = crate::FieldReader<u32>;
#[doc = "Field `EN1` writer - GPIO63-32 Enables tri-state pin output. Writing a 1 to any bit enables, and writing a 0 to any bit disables, the output for the corresponding GPIO. Reads return output enable/disable status of GPIO."]
pub type En1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - GPIO63-32 Enables tri-state pin output. Writing a 1 to any bit enables, and writing a 0 to any bit disables, the output for the corresponding GPIO. Reads return output enable/disable status of GPIO."]
    #[inline(always)]
    pub fn en1(&self) -> En1R {
        En1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - GPIO63-32 Enables tri-state pin output. Writing a 1 to any bit enables, and writing a 0 to any bit disables, the output for the corresponding GPIO. Reads return output enable/disable status of GPIO."]
    #[inline(always)]
    #[must_use]
    pub fn en1(&mut self) -> En1W<En1Spec> {
        En1W::new(self, 0)
    }
}
#[doc = "GPIO Enable 1 (63-32)\n\nYou can [`read`](crate::Reg::read) this register and get [`en1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`en1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct En1Spec;
impl crate::RegisterSpec for En1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`en1::R`](R) reader structure"]
impl crate::Readable for En1Spec {}
#[doc = "`write(|w| ..)` method takes [`en1::W`](W) writer structure"]
impl crate::Writable for En1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EN1 to value 0"]
impl crate::Resettable for En1Spec {
    const RESET_VALUE: u32 = 0;
}
