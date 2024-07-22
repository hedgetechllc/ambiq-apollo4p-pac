#[doc = "Register `TMR1CMP0` reader"]
pub type R = crate::R<Tmr1cmp0Spec>;
#[doc = "Register `TMR1CMP0` writer"]
pub type W = crate::W<Tmr1cmp0Spec>;
#[doc = "Field `TMR1CMP0` reader - Counter/Timer 1 End Compare Register. For MEASURE mode indicates the high phase sample count."]
pub type Tmr1cmp0R = crate::FieldReader<u32>;
#[doc = "Field `TMR1CMP0` writer - Counter/Timer 1 End Compare Register. For MEASURE mode indicates the high phase sample count."]
pub type Tmr1cmp0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Counter/Timer 1 End Compare Register. For MEASURE mode indicates the high phase sample count."]
    #[inline(always)]
    pub fn tmr1cmp0(&self) -> Tmr1cmp0R {
        Tmr1cmp0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Counter/Timer 1 End Compare Register. For MEASURE mode indicates the high phase sample count."]
    #[inline(always)]
    #[must_use]
    pub fn tmr1cmp0(&mut self) -> Tmr1cmp0W<Tmr1cmp0Spec> {
        Tmr1cmp0W::new(self, 0)
    }
}
#[doc = "This contains the Compare limits for timer 1. This is the primary comparator that can be used to mark the END of a timer cycle (and thus restart the timer for repeat modes)\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr1cmp0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr1cmp0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tmr1cmp0Spec;
impl crate::RegisterSpec for Tmr1cmp0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmr1cmp0::R`](R) reader structure"]
impl crate::Readable for Tmr1cmp0Spec {}
#[doc = "`write(|w| ..)` method takes [`tmr1cmp0::W`](W) writer structure"]
impl crate::Writable for Tmr1cmp0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TMR1CMP0 to value 0"]
impl crate::Resettable for Tmr1cmp0Spec {
    const RESET_VALUE: u32 = 0;
}
