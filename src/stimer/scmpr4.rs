#[doc = "Register `SCMPR4` reader"]
pub type R = crate::R<Scmpr4Spec>;
#[doc = "Register `SCMPR4` writer"]
pub type W = crate::W<Scmpr4Spec>;
#[doc = "Field `SCMPR4` reader - Compare this value to the value in the COUNTER register according to the match criterion, as selected in the COMPARE_A_EN bit in the REG_CTIMER_STCFG register."]
pub type Scmpr4R = crate::FieldReader<u32>;
#[doc = "Field `SCMPR4` writer - Compare this value to the value in the COUNTER register according to the match criterion, as selected in the COMPARE_A_EN bit in the REG_CTIMER_STCFG register."]
pub type Scmpr4W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Compare this value to the value in the COUNTER register according to the match criterion, as selected in the COMPARE_A_EN bit in the REG_CTIMER_STCFG register."]
    #[inline(always)]
    pub fn scmpr4(&self) -> Scmpr4R {
        Scmpr4R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Compare this value to the value in the COUNTER register according to the match criterion, as selected in the COMPARE_A_EN bit in the REG_CTIMER_STCFG register."]
    #[inline(always)]
    #[must_use]
    pub fn scmpr4(&mut self) -> Scmpr4W<Scmpr4Spec> {
        Scmpr4W::new(self, 0)
    }
}
#[doc = "The VALUE in this bit field is used to compare against the VALUE in the COUNTER register. If the match criterion in the configuration register is met then a corresponding interrupt status bit is set. The match criterion is defined as COUNTER equal to COMPARE. To establish a desired value in this COMPARE register, write the number of ticks in the future to this register to indicate when to interrupt. The hardware does the addition to the COUNTER value in the STIMER clock domain so that the math is precise. Reading this register shows the COUNTER value at which this interrupt will occur.\n\nYou can [`read`](crate::Reg::read) this register and get [`scmpr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scmpr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scmpr4Spec;
impl crate::RegisterSpec for Scmpr4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scmpr4::R`](R) reader structure"]
impl crate::Readable for Scmpr4Spec {}
#[doc = "`write(|w| ..)` method takes [`scmpr4::W`](W) writer structure"]
impl crate::Writable for Scmpr4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCMPR4 to value 0"]
impl crate::Resettable for Scmpr4Spec {
    const RESET_VALUE: u32 = 0;
}
