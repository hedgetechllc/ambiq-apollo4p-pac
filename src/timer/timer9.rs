#[doc = "Register `TIMER9` reader"]
pub type R = crate::R<Timer9Spec>;
#[doc = "Register `TIMER9` writer"]
pub type W = crate::W<Timer9Spec>;
#[doc = "Field `TIMER9` reader - Counter/Timer 9"]
pub type Timer9R = crate::FieldReader<u32>;
#[doc = "Field `TIMER9` writer - Counter/Timer 9"]
pub type Timer9W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Counter/Timer 9"]
    #[inline(always)]
    pub fn timer9(&self) -> Timer9R {
        Timer9R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Counter/Timer 9"]
    #[inline(always)]
    #[must_use]
    pub fn timer9(&mut self) -> Timer9W<Timer9Spec> {
        Timer9W::new(self, 0)
    }
}
#[doc = "This register holds the running time or event count for timer 9.\n\nYou can [`read`](crate::Reg::read) this register and get [`timer9::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer9::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Timer9Spec;
impl crate::RegisterSpec for Timer9Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer9::R`](R) reader structure"]
impl crate::Readable for Timer9Spec {}
#[doc = "`write(|w| ..)` method takes [`timer9::W`](W) writer structure"]
impl crate::Writable for Timer9Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMER9 to value 0"]
impl crate::Resettable for Timer9Spec {
    const RESET_VALUE: u32 = 0;
}
