#[doc = "Register `RXDMATOTCNT` reader"]
pub type R = crate::R<RxdmatotcntSpec>;
#[doc = "Register `RXDMATOTCNT` writer"]
pub type W = crate::W<RxdmatotcntSpec>;
#[doc = "Field `RXTOTCNT` reader - Number of 32b audio samples to transfer for RX DMA."]
pub type RxtotcntR = crate::FieldReader<u16>;
#[doc = "Field `RXTOTCNT` writer - Number of 32b audio samples to transfer for RX DMA."]
pub type RxtotcntW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Number of 32b audio samples to transfer for RX DMA."]
    #[inline(always)]
    pub fn rxtotcnt(&self) -> RxtotcntR {
        RxtotcntR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Number of 32b audio samples to transfer for RX DMA."]
    #[inline(always)]
    #[must_use]
    pub fn rxtotcnt(&mut self) -> RxtotcntW<RxdmatotcntSpec> {
        RxtotcntW::new(self, 0)
    }
}
#[doc = "Contains the total count of samples to be stored for the current RX DMA operation. This register is updated as DMA beats complete.\n\nYou can [`read`](crate::Reg::read) this register and get [`rxdmatotcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxdmatotcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxdmatotcntSpec;
impl crate::RegisterSpec for RxdmatotcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxdmatotcnt::R`](R) reader structure"]
impl crate::Readable for RxdmatotcntSpec {}
#[doc = "`write(|w| ..)` method takes [`rxdmatotcnt::W`](W) writer structure"]
impl crate::Writable for RxdmatotcntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RXDMATOTCNT to value 0"]
impl crate::Resettable for RxdmatotcntSpec {
    const RESET_VALUE: u32 = 0;
}
