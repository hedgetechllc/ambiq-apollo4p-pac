#[doc = "Register `TIMER10` reader"]
pub type R = crate::R<Timer10Spec>;
#[doc = "Register `TIMER10` writer"]
pub type W = crate::W<Timer10Spec>;
#[doc = "Field `TIMER10` reader - Counter/Timer 10"]
pub type Timer10R = crate::FieldReader<u32>;
#[doc = "Field `TIMER10` writer - Counter/Timer 10"]
pub type Timer10W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Counter/Timer 10"]
    #[inline(always)]
    pub fn timer10(&self) -> Timer10R {
        Timer10R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Counter/Timer 10"]
    #[inline(always)]
    #[must_use]
    pub fn timer10(&mut self) -> Timer10W<Timer10Spec> {
        Timer10W::new(self, 0)
    }
}
#[doc = "This register holds the running time or event count for timer 10.\n\nYou can [`read`](crate::Reg::read) this register and get [`timer10::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer10::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Timer10Spec;
impl crate::RegisterSpec for Timer10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer10::R`](R) reader structure"]
impl crate::Readable for Timer10Spec {}
#[doc = "`write(|w| ..)` method takes [`timer10::W`](W) writer structure"]
impl crate::Writable for Timer10Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMER10 to value 0"]
impl crate::Resettable for Timer10Spec {
    const RESET_VALUE: u32 = 0;
}
