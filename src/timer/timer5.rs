#[doc = "Register `TIMER5` reader"]
pub type R = crate::R<Timer5Spec>;
#[doc = "Register `TIMER5` writer"]
pub type W = crate::W<Timer5Spec>;
#[doc = "Field `TIMER5` reader - Counter/Timer 5"]
pub type Timer5R = crate::FieldReader<u32>;
#[doc = "Field `TIMER5` writer - Counter/Timer 5"]
pub type Timer5W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Counter/Timer 5"]
    #[inline(always)]
    pub fn timer5(&self) -> Timer5R {
        Timer5R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Counter/Timer 5"]
    #[inline(always)]
    #[must_use]
    pub fn timer5(&mut self) -> Timer5W<Timer5Spec> {
        Timer5W::new(self, 0)
    }
}
#[doc = "This register holds the running time or event count for timer 5.\n\nYou can [`read`](crate::Reg::read) this register and get [`timer5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Timer5Spec;
impl crate::RegisterSpec for Timer5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer5::R`](R) reader structure"]
impl crate::Readable for Timer5Spec {}
#[doc = "`write(|w| ..)` method takes [`timer5::W`](W) writer structure"]
impl crate::Writable for Timer5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMER5 to value 0"]
impl crate::Resettable for Timer5Spec {
    const RESET_VALUE: u32 = 0;
}
