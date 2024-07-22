#[doc = "Register `TIMER7` reader"]
pub type R = crate::R<Timer7Spec>;
#[doc = "Register `TIMER7` writer"]
pub type W = crate::W<Timer7Spec>;
#[doc = "Field `TIMER7` reader - Counter/Timer 7"]
pub type Timer7R = crate::FieldReader<u32>;
#[doc = "Field `TIMER7` writer - Counter/Timer 7"]
pub type Timer7W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Counter/Timer 7"]
    #[inline(always)]
    pub fn timer7(&self) -> Timer7R {
        Timer7R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Counter/Timer 7"]
    #[inline(always)]
    #[must_use]
    pub fn timer7(&mut self) -> Timer7W<Timer7Spec> {
        Timer7W::new(self, 0)
    }
}
#[doc = "This register holds the running time or event count for timer 7.\n\nYou can [`read`](crate::Reg::read) this register and get [`timer7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Timer7Spec;
impl crate::RegisterSpec for Timer7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer7::R`](R) reader structure"]
impl crate::Readable for Timer7Spec {}
#[doc = "`write(|w| ..)` method takes [`timer7::W`](W) writer structure"]
impl crate::Writable for Timer7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMER7 to value 0"]
impl crate::Resettable for Timer7Spec {
    const RESET_VALUE: u32 = 0;
}
