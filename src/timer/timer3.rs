#[doc = "Register `TIMER3` reader"]
pub type R = crate::R<Timer3Spec>;
#[doc = "Register `TIMER3` writer"]
pub type W = crate::W<Timer3Spec>;
#[doc = "Field `TIMER3` reader - Counter/Timer 3"]
pub type Timer3R = crate::FieldReader<u32>;
#[doc = "Field `TIMER3` writer - Counter/Timer 3"]
pub type Timer3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Counter/Timer 3"]
    #[inline(always)]
    pub fn timer3(&self) -> Timer3R {
        Timer3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Counter/Timer 3"]
    #[inline(always)]
    #[must_use]
    pub fn timer3(&mut self) -> Timer3W<Timer3Spec> {
        Timer3W::new(self, 0)
    }
}
#[doc = "This register holds the running time or event count for timer 3.\n\nYou can [`read`](crate::Reg::read) this register and get [`timer3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Timer3Spec;
impl crate::RegisterSpec for Timer3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer3::R`](R) reader structure"]
impl crate::Readable for Timer3Spec {}
#[doc = "`write(|w| ..)` method takes [`timer3::W`](W) writer structure"]
impl crate::Writable for Timer3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMER3 to value 0"]
impl crate::Resettable for Timer3Spec {
    const RESET_VALUE: u32 = 0;
}
