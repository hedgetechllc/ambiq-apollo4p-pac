#[doc = "Register `TIMER11` reader"]
pub type R = crate::R<Timer11Spec>;
#[doc = "Register `TIMER11` writer"]
pub type W = crate::W<Timer11Spec>;
#[doc = "Field `TIMER11` reader - Counter/Timer 11"]
pub type Timer11R = crate::FieldReader<u32>;
#[doc = "Field `TIMER11` writer - Counter/Timer 11"]
pub type Timer11W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Counter/Timer 11"]
    #[inline(always)]
    pub fn timer11(&self) -> Timer11R {
        Timer11R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Counter/Timer 11"]
    #[inline(always)]
    #[must_use]
    pub fn timer11(&mut self) -> Timer11W<Timer11Spec> {
        Timer11W::new(self, 0)
    }
}
#[doc = "This register holds the running time or event count for timer 11.\n\nYou can [`read`](crate::Reg::read) this register and get [`timer11::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer11::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Timer11Spec;
impl crate::RegisterSpec for Timer11Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer11::R`](R) reader structure"]
impl crate::Readable for Timer11Spec {}
#[doc = "`write(|w| ..)` method takes [`timer11::W`](W) writer structure"]
impl crate::Writable for Timer11Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMER11 to value 0"]
impl crate::Resettable for Timer11Spec {
    const RESET_VALUE: u32 = 0;
}
