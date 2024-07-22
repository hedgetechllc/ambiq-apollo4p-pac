#[doc = "Register `SCAPT0` reader"]
pub type R = crate::R<Scapt0Spec>;
#[doc = "Register `SCAPT0` writer"]
pub type W = crate::W<Scapt0Spec>;
#[doc = "Field `SCAPT0` reader - Whenever the event is detected, the value in the COUNTER is copied into this register and the corresponding interrupt status bit is set."]
pub type Scapt0R = crate::FieldReader<u32>;
#[doc = "Field `SCAPT0` writer - Whenever the event is detected, the value in the COUNTER is copied into this register and the corresponding interrupt status bit is set."]
pub type Scapt0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Whenever the event is detected, the value in the COUNTER is copied into this register and the corresponding interrupt status bit is set."]
    #[inline(always)]
    pub fn scapt0(&self) -> Scapt0R {
        Scapt0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Whenever the event is detected, the value in the COUNTER is copied into this register and the corresponding interrupt status bit is set."]
    #[inline(always)]
    #[must_use]
    pub fn scapt0(&mut self) -> Scapt0W<Scapt0Spec> {
        Scapt0W::new(self, 0)
    }
}
#[doc = "The STIMER capture Register 0 captures the VALUE in the COUNTER register whenever capture condition (event) occurs. This register holds a time stamp for the event.\n\nYou can [`read`](crate::Reg::read) this register and get [`scapt0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scapt0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scapt0Spec;
impl crate::RegisterSpec for Scapt0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scapt0::R`](R) reader structure"]
impl crate::Readable for Scapt0Spec {}
#[doc = "`write(|w| ..)` method takes [`scapt0::W`](W) writer structure"]
impl crate::Writable for Scapt0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCAPT0 to value 0"]
impl crate::Resettable for Scapt0Spec {
    const RESET_VALUE: u32 = 0;
}
