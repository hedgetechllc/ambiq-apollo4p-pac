#[doc = "Register `TIMER0` reader"]
pub type R = crate::R<Timer0Spec>;
#[doc = "Register `TIMER0` writer"]
pub type W = crate::W<Timer0Spec>;
#[doc = "Field `TIMER0` reader - Counter/Timer 0"]
pub type Timer0R = crate::FieldReader<u32>;
#[doc = "Field `TIMER0` writer - Counter/Timer 0"]
pub type Timer0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Counter/Timer 0"]
    #[inline(always)]
    pub fn timer0(&self) -> Timer0R {
        Timer0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Counter/Timer 0"]
    #[inline(always)]
    #[must_use]
    pub fn timer0(&mut self) -> Timer0W<Timer0Spec> {
        Timer0W::new(self, 0)
    }
}
#[doc = "This register holds the running time or event count for timer 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`timer0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Timer0Spec;
impl crate::RegisterSpec for Timer0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer0::R`](R) reader structure"]
impl crate::Readable for Timer0Spec {}
#[doc = "`write(|w| ..)` method takes [`timer0::W`](W) writer structure"]
impl crate::Writable for Timer0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMER0 to value 0"]
impl crate::Resettable for Timer0Spec {
    const RESET_VALUE: u32 = 0;
}