#[doc = "Register `EN0` reader"]
pub type R = crate::R<En0Spec>;
#[doc = "Register `EN0` writer"]
pub type W = crate::W<En0Spec>;
#[doc = "Field `EN0` reader - GPIO31-0 Enables tri-state pin output. Writing a 1 to any bit enables, and writing a 0 to any bit disables, the output for the corresponding GPIO. Reads return output enable/disable status of GPIO."]
pub type En0R = crate::FieldReader<u32>;
#[doc = "Field `EN0` writer - GPIO31-0 Enables tri-state pin output. Writing a 1 to any bit enables, and writing a 0 to any bit disables, the output for the corresponding GPIO. Reads return output enable/disable status of GPIO."]
pub type En0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - GPIO31-0 Enables tri-state pin output. Writing a 1 to any bit enables, and writing a 0 to any bit disables, the output for the corresponding GPIO. Reads return output enable/disable status of GPIO."]
    #[inline(always)]
    pub fn en0(&self) -> En0R {
        En0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - GPIO31-0 Enables tri-state pin output. Writing a 1 to any bit enables, and writing a 0 to any bit disables, the output for the corresponding GPIO. Reads return output enable/disable status of GPIO."]
    #[inline(always)]
    #[must_use]
    pub fn en0(&mut self) -> En0W<En0Spec> {
        En0W::new(self, 0)
    }
}
#[doc = "GPIO Enable 0 (31-0)\n\nYou can [`read`](crate::Reg::read) this register and get [`en0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`en0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct En0Spec;
impl crate::RegisterSpec for En0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`en0::R`](R) reader structure"]
impl crate::Readable for En0Spec {}
#[doc = "`write(|w| ..)` method takes [`en0::W`](W) writer structure"]
impl crate::Writable for En0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EN0 to value 0"]
impl crate::Resettable for En0Spec {
    const RESET_VALUE: u32 = 0;
}
