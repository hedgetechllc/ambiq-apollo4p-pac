#[doc = "Register `TIMER6` reader"]
pub type R = crate::R<Timer6Spec>;
#[doc = "Register `TIMER6` writer"]
pub type W = crate::W<Timer6Spec>;
#[doc = "Field `TIMER6` reader - Counter/Timer 6"]
pub type Timer6R = crate::FieldReader<u32>;
#[doc = "Field `TIMER6` writer - Counter/Timer 6"]
pub type Timer6W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Counter/Timer 6"]
    #[inline(always)]
    pub fn timer6(&self) -> Timer6R {
        Timer6R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Counter/Timer 6"]
    #[inline(always)]
    #[must_use]
    pub fn timer6(&mut self) -> Timer6W<Timer6Spec> {
        Timer6W::new(self, 0)
    }
}
#[doc = "This register holds the running time or event count for timer 6.\n\nYou can [`read`](crate::Reg::read) this register and get [`timer6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Timer6Spec;
impl crate::RegisterSpec for Timer6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer6::R`](R) reader structure"]
impl crate::Readable for Timer6Spec {}
#[doc = "`write(|w| ..)` method takes [`timer6::W`](W) writer structure"]
impl crate::Writable for Timer6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMER6 to value 0"]
impl crate::Resettable for Timer6Spec {
    const RESET_VALUE: u32 = 0;
}
