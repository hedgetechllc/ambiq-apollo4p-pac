#[doc = "Register `SCAPT1` reader"]
pub type R = crate::R<Scapt1Spec>;
#[doc = "Register `SCAPT1` writer"]
pub type W = crate::W<Scapt1Spec>;
#[doc = "Field `SCAPT1` reader - Whenever the event is detected, the value in the COUNTER is copied into this register and the corresponding interrupt status bit is set."]
pub type Scapt1R = crate::FieldReader<u32>;
#[doc = "Field `SCAPT1` writer - Whenever the event is detected, the value in the COUNTER is copied into this register and the corresponding interrupt status bit is set."]
pub type Scapt1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Whenever the event is detected, the value in the COUNTER is copied into this register and the corresponding interrupt status bit is set."]
    #[inline(always)]
    pub fn scapt1(&self) -> Scapt1R {
        Scapt1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Whenever the event is detected, the value in the COUNTER is copied into this register and the corresponding interrupt status bit is set."]
    #[inline(always)]
    #[must_use]
    pub fn scapt1(&mut self) -> Scapt1W<Scapt1Spec> {
        Scapt1W::new(self, 0)
    }
}
#[doc = "The STIMER capture Register 1 captures the VALUE in the COUNTER register whenever capture condition (event) occurs. This register holds a time stamp for the event.\n\nYou can [`read`](crate::Reg::read) this register and get [`scapt1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scapt1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scapt1Spec;
impl crate::RegisterSpec for Scapt1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scapt1::R`](R) reader structure"]
impl crate::Readable for Scapt1Spec {}
#[doc = "`write(|w| ..)` method takes [`scapt1::W`](W) writer structure"]
impl crate::Writable for Scapt1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCAPT1 to value 0"]
impl crate::Resettable for Scapt1Spec {
    const RESET_VALUE: u32 = 0;
}
