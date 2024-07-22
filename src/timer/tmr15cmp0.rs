#[doc = "Register `TMR15CMP0` reader"]
pub type R = crate::R<Tmr15cmp0Spec>;
#[doc = "Register `TMR15CMP0` writer"]
pub type W = crate::W<Tmr15cmp0Spec>;
#[doc = "Field `TMR15CMP0` reader - Counter/Timer 15 End Compare Register. For MEASURE mode indicates the high phase sample count."]
pub type Tmr15cmp0R = crate::FieldReader<u32>;
#[doc = "Field `TMR15CMP0` writer - Counter/Timer 15 End Compare Register. For MEASURE mode indicates the high phase sample count."]
pub type Tmr15cmp0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Counter/Timer 15 End Compare Register. For MEASURE mode indicates the high phase sample count."]
    #[inline(always)]
    pub fn tmr15cmp0(&self) -> Tmr15cmp0R {
        Tmr15cmp0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Counter/Timer 15 End Compare Register. For MEASURE mode indicates the high phase sample count."]
    #[inline(always)]
    #[must_use]
    pub fn tmr15cmp0(&mut self) -> Tmr15cmp0W<Tmr15cmp0Spec> {
        Tmr15cmp0W::new(self, 0)
    }
}
#[doc = "This contains the Compare limits for timer 15. This is the primary comparator that can be used to mark the END of a timer cycle (and thus restart the timer for repeat modes)\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr15cmp0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr15cmp0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tmr15cmp0Spec;
impl crate::RegisterSpec for Tmr15cmp0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmr15cmp0::R`](R) reader structure"]
impl crate::Readable for Tmr15cmp0Spec {}
#[doc = "`write(|w| ..)` method takes [`tmr15cmp0::W`](W) writer structure"]
impl crate::Writable for Tmr15cmp0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TMR15CMP0 to value 0"]
impl crate::Resettable for Tmr15cmp0Spec {
    const RESET_VALUE: u32 = 0;
}
