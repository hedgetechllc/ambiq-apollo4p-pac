#[doc = "Register `TMR3CMP0` reader"]
pub type R = crate::R<Tmr3cmp0Spec>;
#[doc = "Register `TMR3CMP0` writer"]
pub type W = crate::W<Tmr3cmp0Spec>;
#[doc = "Field `TMR3CMP0` reader - Counter/Timer 3 End Compare Register. For MEASURE mode indicates the high phase sample count."]
pub type Tmr3cmp0R = crate::FieldReader<u32>;
#[doc = "Field `TMR3CMP0` writer - Counter/Timer 3 End Compare Register. For MEASURE mode indicates the high phase sample count."]
pub type Tmr3cmp0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Counter/Timer 3 End Compare Register. For MEASURE mode indicates the high phase sample count."]
    #[inline(always)]
    pub fn tmr3cmp0(&self) -> Tmr3cmp0R {
        Tmr3cmp0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Counter/Timer 3 End Compare Register. For MEASURE mode indicates the high phase sample count."]
    #[inline(always)]
    #[must_use]
    pub fn tmr3cmp0(&mut self) -> Tmr3cmp0W<Tmr3cmp0Spec> {
        Tmr3cmp0W::new(self, 0)
    }
}
#[doc = "This contains the Compare limits for timer 3. This is the primary comparator that can be used to mark the END of a timer cycle (and thus restart the timer for repeat modes)\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr3cmp0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr3cmp0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tmr3cmp0Spec;
impl crate::RegisterSpec for Tmr3cmp0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmr3cmp0::R`](R) reader structure"]
impl crate::Readable for Tmr3cmp0Spec {}
#[doc = "`write(|w| ..)` method takes [`tmr3cmp0::W`](W) writer structure"]
impl crate::Writable for Tmr3cmp0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TMR3CMP0 to value 0"]
impl crate::Resettable for Tmr3cmp0Spec {
    const RESET_VALUE: u32 = 0;
}
