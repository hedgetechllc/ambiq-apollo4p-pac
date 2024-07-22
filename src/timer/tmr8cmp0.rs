#[doc = "Register `TMR8CMP0` reader"]
pub type R = crate::R<Tmr8cmp0Spec>;
#[doc = "Register `TMR8CMP0` writer"]
pub type W = crate::W<Tmr8cmp0Spec>;
#[doc = "Field `TMR8CMP0` reader - Counter/Timer 8 End Compare Register. For MEASURE mode indicates the high phase sample count."]
pub type Tmr8cmp0R = crate::FieldReader<u32>;
#[doc = "Field `TMR8CMP0` writer - Counter/Timer 8 End Compare Register. For MEASURE mode indicates the high phase sample count."]
pub type Tmr8cmp0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Counter/Timer 8 End Compare Register. For MEASURE mode indicates the high phase sample count."]
    #[inline(always)]
    pub fn tmr8cmp0(&self) -> Tmr8cmp0R {
        Tmr8cmp0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Counter/Timer 8 End Compare Register. For MEASURE mode indicates the high phase sample count."]
    #[inline(always)]
    #[must_use]
    pub fn tmr8cmp0(&mut self) -> Tmr8cmp0W<Tmr8cmp0Spec> {
        Tmr8cmp0W::new(self, 0)
    }
}
#[doc = "This contains the Compare limits for timer 8. This is the primary comparator that can be used to mark the END of a timer cycle (and thus restart the timer for repeat modes)\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr8cmp0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr8cmp0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tmr8cmp0Spec;
impl crate::RegisterSpec for Tmr8cmp0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmr8cmp0::R`](R) reader structure"]
impl crate::Readable for Tmr8cmp0Spec {}
#[doc = "`write(|w| ..)` method takes [`tmr8cmp0::W`](W) writer structure"]
impl crate::Writable for Tmr8cmp0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TMR8CMP0 to value 0"]
impl crate::Resettable for Tmr8cmp0Spec {
    const RESET_VALUE: u32 = 0;
}
