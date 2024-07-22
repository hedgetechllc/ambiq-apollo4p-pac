#[doc = "Register `TIMER13` reader"]
pub type R = crate::R<Timer13Spec>;
#[doc = "Register `TIMER13` writer"]
pub type W = crate::W<Timer13Spec>;
#[doc = "Field `TIMER13` reader - Counter/Timer 13"]
pub type Timer13R = crate::FieldReader<u32>;
#[doc = "Field `TIMER13` writer - Counter/Timer 13"]
pub type Timer13W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Counter/Timer 13"]
    #[inline(always)]
    pub fn timer13(&self) -> Timer13R {
        Timer13R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Counter/Timer 13"]
    #[inline(always)]
    #[must_use]
    pub fn timer13(&mut self) -> Timer13W<Timer13Spec> {
        Timer13W::new(self, 0)
    }
}
#[doc = "This register holds the running time or event count for timer 13.\n\nYou can [`read`](crate::Reg::read) this register and get [`timer13::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer13::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Timer13Spec;
impl crate::RegisterSpec for Timer13Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer13::R`](R) reader structure"]
impl crate::Readable for Timer13Spec {}
#[doc = "`write(|w| ..)` method takes [`timer13::W`](W) writer structure"]
impl crate::Writable for Timer13Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMER13 to value 0"]
impl crate::Resettable for Timer13Spec {
    const RESET_VALUE: u32 = 0;
}
