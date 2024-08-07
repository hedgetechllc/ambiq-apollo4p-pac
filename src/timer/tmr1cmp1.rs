#[doc = "Register `TMR1CMP1` reader"]
pub type R = crate::R<Tmr1cmp1Spec>;
#[doc = "Register `TMR1CMP1` writer"]
pub type W = crate::W<Tmr1cmp1Spec>;
#[doc = "Field `TMR1CMP1` reader - Holds the secondary comparator that can be used to generate a PWM or generate secondary pulses. CMP0 should ALWAYS be used first."]
pub type Tmr1cmp1R = crate::FieldReader<u32>;
#[doc = "Field `TMR1CMP1` writer - Holds the secondary comparator that can be used to generate a PWM or generate secondary pulses. CMP0 should ALWAYS be used first."]
pub type Tmr1cmp1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Holds the secondary comparator that can be used to generate a PWM or generate secondary pulses. CMP0 should ALWAYS be used first."]
    #[inline(always)]
    pub fn tmr1cmp1(&self) -> Tmr1cmp1R {
        Tmr1cmp1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Holds the secondary comparator that can be used to generate a PWM or generate secondary pulses. CMP0 should ALWAYS be used first."]
    #[inline(always)]
    #[must_use]
    pub fn tmr1cmp1(&mut self) -> Tmr1cmp1W<Tmr1cmp1Spec> {
        Tmr1cmp1W::new(self, 0)
    }
}
#[doc = "This comparator is used as a secondary compare count for modes that generate pulses. For MEASURE mode indicates the low phase sample count.\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr1cmp1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr1cmp1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tmr1cmp1Spec;
impl crate::RegisterSpec for Tmr1cmp1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmr1cmp1::R`](R) reader structure"]
impl crate::Readable for Tmr1cmp1Spec {}
#[doc = "`write(|w| ..)` method takes [`tmr1cmp1::W`](W) writer structure"]
impl crate::Writable for Tmr1cmp1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TMR1CMP1 to value 0"]
impl crate::Resettable for Tmr1cmp1Spec {
    const RESET_VALUE: u32 = 0;
}
