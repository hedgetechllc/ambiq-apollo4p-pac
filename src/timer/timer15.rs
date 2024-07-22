#[doc = "Register `TIMER15` reader"]
pub type R = crate::R<Timer15Spec>;
#[doc = "Register `TIMER15` writer"]
pub type W = crate::W<Timer15Spec>;
#[doc = "Field `TIMER15` reader - Counter/Timer 15"]
pub type Timer15R = crate::FieldReader<u32>;
#[doc = "Field `TIMER15` writer - Counter/Timer 15"]
pub type Timer15W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Counter/Timer 15"]
    #[inline(always)]
    pub fn timer15(&self) -> Timer15R {
        Timer15R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Counter/Timer 15"]
    #[inline(always)]
    #[must_use]
    pub fn timer15(&mut self) -> Timer15W<Timer15Spec> {
        Timer15W::new(self, 0)
    }
}
#[doc = "This register holds the running time or event count for timer 15.\n\nYou can [`read`](crate::Reg::read) this register and get [`timer15::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer15::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Timer15Spec;
impl crate::RegisterSpec for Timer15Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer15::R`](R) reader structure"]
impl crate::Readable for Timer15Spec {}
#[doc = "`write(|w| ..)` method takes [`timer15::W`](W) writer structure"]
impl crate::Writable for Timer15Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMER15 to value 0"]
impl crate::Resettable for Timer15Spec {
    const RESET_VALUE: u32 = 0;
}
