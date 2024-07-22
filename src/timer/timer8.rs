#[doc = "Register `TIMER8` reader"]
pub type R = crate::R<Timer8Spec>;
#[doc = "Register `TIMER8` writer"]
pub type W = crate::W<Timer8Spec>;
#[doc = "Field `TIMER8` reader - Counter/Timer 8"]
pub type Timer8R = crate::FieldReader<u32>;
#[doc = "Field `TIMER8` writer - Counter/Timer 8"]
pub type Timer8W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Counter/Timer 8"]
    #[inline(always)]
    pub fn timer8(&self) -> Timer8R {
        Timer8R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Counter/Timer 8"]
    #[inline(always)]
    #[must_use]
    pub fn timer8(&mut self) -> Timer8W<Timer8Spec> {
        Timer8W::new(self, 0)
    }
}
#[doc = "This register holds the running time or event count for timer 8.\n\nYou can [`read`](crate::Reg::read) this register and get [`timer8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Timer8Spec;
impl crate::RegisterSpec for Timer8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer8::R`](R) reader structure"]
impl crate::Readable for Timer8Spec {}
#[doc = "`write(|w| ..)` method takes [`timer8::W`](W) writer structure"]
impl crate::Writable for Timer8Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMER8 to value 0"]
impl crate::Resettable for Timer8Spec {
    const RESET_VALUE: u32 = 0;
}
