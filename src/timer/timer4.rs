#[doc = "Register `TIMER4` reader"]
pub type R = crate::R<Timer4Spec>;
#[doc = "Register `TIMER4` writer"]
pub type W = crate::W<Timer4Spec>;
#[doc = "Field `TIMER4` reader - Counter/Timer 4"]
pub type Timer4R = crate::FieldReader<u32>;
#[doc = "Field `TIMER4` writer - Counter/Timer 4"]
pub type Timer4W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Counter/Timer 4"]
    #[inline(always)]
    pub fn timer4(&self) -> Timer4R {
        Timer4R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Counter/Timer 4"]
    #[inline(always)]
    #[must_use]
    pub fn timer4(&mut self) -> Timer4W<Timer4Spec> {
        Timer4W::new(self, 0)
    }
}
#[doc = "This register holds the running time or event count for timer 4.\n\nYou can [`read`](crate::Reg::read) this register and get [`timer4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Timer4Spec;
impl crate::RegisterSpec for Timer4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer4::R`](R) reader structure"]
impl crate::Readable for Timer4Spec {}
#[doc = "`write(|w| ..)` method takes [`timer4::W`](W) writer structure"]
impl crate::Writable for Timer4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMER4 to value 0"]
impl crate::Resettable for Timer4Spec {
    const RESET_VALUE: u32 = 0;
}
