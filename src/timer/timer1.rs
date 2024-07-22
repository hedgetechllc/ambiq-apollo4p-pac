#[doc = "Register `TIMER1` reader"]
pub type R = crate::R<Timer1Spec>;
#[doc = "Register `TIMER1` writer"]
pub type W = crate::W<Timer1Spec>;
#[doc = "Field `TIMER1` reader - Counter/Timer 1"]
pub type Timer1R = crate::FieldReader<u32>;
#[doc = "Field `TIMER1` writer - Counter/Timer 1"]
pub type Timer1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Counter/Timer 1"]
    #[inline(always)]
    pub fn timer1(&self) -> Timer1R {
        Timer1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Counter/Timer 1"]
    #[inline(always)]
    #[must_use]
    pub fn timer1(&mut self) -> Timer1W<Timer1Spec> {
        Timer1W::new(self, 0)
    }
}
#[doc = "This register holds the running time or event count for timer 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`timer1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Timer1Spec;
impl crate::RegisterSpec for Timer1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer1::R`](R) reader structure"]
impl crate::Readable for Timer1Spec {}
#[doc = "`write(|w| ..)` method takes [`timer1::W`](W) writer structure"]
impl crate::Writable for Timer1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMER1 to value 0"]
impl crate::Resettable for Timer1Spec {
    const RESET_VALUE: u32 = 0;
}
