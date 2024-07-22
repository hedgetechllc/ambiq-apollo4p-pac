#[doc = "Register `SCAPT3` reader"]
pub type R = crate::R<Scapt3Spec>;
#[doc = "Register `SCAPT3` writer"]
pub type W = crate::W<Scapt3Spec>;
#[doc = "Field `SCAPT3` reader - Whenever the event is detected, the value in the COUNTER is copied into this register and the corresponding interrupt status bit is set."]
pub type Scapt3R = crate::FieldReader<u32>;
#[doc = "Field `SCAPT3` writer - Whenever the event is detected, the value in the COUNTER is copied into this register and the corresponding interrupt status bit is set."]
pub type Scapt3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Whenever the event is detected, the value in the COUNTER is copied into this register and the corresponding interrupt status bit is set."]
    #[inline(always)]
    pub fn scapt3(&self) -> Scapt3R {
        Scapt3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Whenever the event is detected, the value in the COUNTER is copied into this register and the corresponding interrupt status bit is set."]
    #[inline(always)]
    #[must_use]
    pub fn scapt3(&mut self) -> Scapt3W<Scapt3Spec> {
        Scapt3W::new(self, 0)
    }
}
#[doc = "The STIMER capture Register 3 captures the VALUE in the COUNTER register whenever capture condition (event) occurs. This register holds a time stamp for the event.\n\nYou can [`read`](crate::Reg::read) this register and get [`scapt3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scapt3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scapt3Spec;
impl crate::RegisterSpec for Scapt3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scapt3::R`](R) reader structure"]
impl crate::Readable for Scapt3Spec {}
#[doc = "`write(|w| ..)` method takes [`scapt3::W`](W) writer structure"]
impl crate::Writable for Scapt3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCAPT3 to value 0"]
impl crate::Resettable for Scapt3Spec {
    const RESET_VALUE: u32 = 0;
}
