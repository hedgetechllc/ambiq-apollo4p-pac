#[doc = "Register `TIMER2` reader"]
pub type R = crate::R<Timer2Spec>;
#[doc = "Register `TIMER2` writer"]
pub type W = crate::W<Timer2Spec>;
#[doc = "Field `TIMER2` reader - Counter/Timer 2"]
pub type Timer2R = crate::FieldReader<u32>;
#[doc = "Field `TIMER2` writer - Counter/Timer 2"]
pub type Timer2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Counter/Timer 2"]
    #[inline(always)]
    pub fn timer2(&self) -> Timer2R {
        Timer2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Counter/Timer 2"]
    #[inline(always)]
    #[must_use]
    pub fn timer2(&mut self) -> Timer2W<Timer2Spec> {
        Timer2W::new(self, 0)
    }
}
#[doc = "This register holds the running time or event count for timer 2.\n\nYou can [`read`](crate::Reg::read) this register and get [`timer2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Timer2Spec;
impl crate::RegisterSpec for Timer2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer2::R`](R) reader structure"]
impl crate::Readable for Timer2Spec {}
#[doc = "`write(|w| ..)` method takes [`timer2::W`](W) writer structure"]
impl crate::Writable for Timer2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMER2 to value 0"]
impl crate::Resettable for Timer2Spec {
    const RESET_VALUE: u32 = 0;
}
