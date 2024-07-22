#[doc = "Register `TIMER12` reader"]
pub type R = crate::R<Timer12Spec>;
#[doc = "Register `TIMER12` writer"]
pub type W = crate::W<Timer12Spec>;
#[doc = "Field `TIMER12` reader - Counter/Timer 12"]
pub type Timer12R = crate::FieldReader<u32>;
#[doc = "Field `TIMER12` writer - Counter/Timer 12"]
pub type Timer12W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Counter/Timer 12"]
    #[inline(always)]
    pub fn timer12(&self) -> Timer12R {
        Timer12R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Counter/Timer 12"]
    #[inline(always)]
    #[must_use]
    pub fn timer12(&mut self) -> Timer12W<Timer12Spec> {
        Timer12W::new(self, 0)
    }
}
#[doc = "This register holds the running time or event count for timer 12.\n\nYou can [`read`](crate::Reg::read) this register and get [`timer12::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer12::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Timer12Spec;
impl crate::RegisterSpec for Timer12Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer12::R`](R) reader structure"]
impl crate::Readable for Timer12Spec {}
#[doc = "`write(|w| ..)` method takes [`timer12::W`](W) writer structure"]
impl crate::Writable for Timer12Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMER12 to value 0"]
impl crate::Resettable for Timer12Spec {
    const RESET_VALUE: u32 = 0;
}
