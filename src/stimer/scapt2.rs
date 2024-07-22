#[doc = "Register `SCAPT2` reader"]
pub type R = crate::R<Scapt2Spec>;
#[doc = "Register `SCAPT2` writer"]
pub type W = crate::W<Scapt2Spec>;
#[doc = "Field `SCAPT2` reader - Whenever the event is detected, the value in the COUNTER is copied into this register and the corresponding interrupt status bit is set."]
pub type Scapt2R = crate::FieldReader<u32>;
#[doc = "Field `SCAPT2` writer - Whenever the event is detected, the value in the COUNTER is copied into this register and the corresponding interrupt status bit is set."]
pub type Scapt2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Whenever the event is detected, the value in the COUNTER is copied into this register and the corresponding interrupt status bit is set."]
    #[inline(always)]
    pub fn scapt2(&self) -> Scapt2R {
        Scapt2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Whenever the event is detected, the value in the COUNTER is copied into this register and the corresponding interrupt status bit is set."]
    #[inline(always)]
    #[must_use]
    pub fn scapt2(&mut self) -> Scapt2W<Scapt2Spec> {
        Scapt2W::new(self, 0)
    }
}
#[doc = "The STIMER capture Register 2 captures the VALUE in the COUNTER register whenever capture condition (event) occurs. This register holds a time stamp for the event.\n\nYou can [`read`](crate::Reg::read) this register and get [`scapt2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scapt2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scapt2Spec;
impl crate::RegisterSpec for Scapt2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scapt2::R`](R) reader structure"]
impl crate::Readable for Scapt2Spec {}
#[doc = "`write(|w| ..)` method takes [`scapt2::W`](W) writer structure"]
impl crate::Writable for Scapt2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCAPT2 to value 0"]
impl crate::Resettable for Scapt2Spec {
    const RESET_VALUE: u32 = 0;
}
