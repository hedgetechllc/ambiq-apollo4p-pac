#[doc = "Register `TIMER14` reader"]
pub type R = crate::R<Timer14Spec>;
#[doc = "Register `TIMER14` writer"]
pub type W = crate::W<Timer14Spec>;
#[doc = "Field `TIMER14` reader - Counter/Timer 14"]
pub type Timer14R = crate::FieldReader<u32>;
#[doc = "Field `TIMER14` writer - Counter/Timer 14"]
pub type Timer14W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Counter/Timer 14"]
    #[inline(always)]
    pub fn timer14(&self) -> Timer14R {
        Timer14R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Counter/Timer 14"]
    #[inline(always)]
    #[must_use]
    pub fn timer14(&mut self) -> Timer14W<Timer14Spec> {
        Timer14W::new(self, 0)
    }
}
#[doc = "This register holds the running time or event count for timer 14.\n\nYou can [`read`](crate::Reg::read) this register and get [`timer14::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer14::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Timer14Spec;
impl crate::RegisterSpec for Timer14Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer14::R`](R) reader structure"]
impl crate::Readable for Timer14Spec {}
#[doc = "`write(|w| ..)` method takes [`timer14::W`](W) writer structure"]
impl crate::Writable for Timer14Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMER14 to value 0"]
impl crate::Resettable for Timer14Spec {
    const RESET_VALUE: u32 = 0;
}
