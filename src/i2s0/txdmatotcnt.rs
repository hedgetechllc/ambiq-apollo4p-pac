#[doc = "Register `TXDMATOTCNT` reader"]
pub type R = crate::R<TxdmatotcntSpec>;
#[doc = "Register `TXDMATOTCNT` writer"]
pub type W = crate::W<TxdmatotcntSpec>;
#[doc = "Field `TXTOTCNT` reader - Number of 32b audio samples to transmit"]
pub type TxtotcntR = crate::FieldReader<u16>;
#[doc = "Field `TXTOTCNT` writer - Number of 32b audio samples to transmit"]
pub type TxtotcntW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Number of 32b audio samples to transmit"]
    #[inline(always)]
    pub fn txtotcnt(&self) -> TxtotcntR {
        TxtotcntR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Number of 32b audio samples to transmit"]
    #[inline(always)]
    #[must_use]
    pub fn txtotcnt(&mut self) -> TxtotcntW<TxdmatotcntSpec> {
        TxtotcntW::new(self, 0)
    }
}
#[doc = "Contains the total count of samples to be read and transmitted for the current TX DMA operation. This register is updated as DMA beats complete.\n\nYou can [`read`](crate::Reg::read) this register and get [`txdmatotcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdmatotcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxdmatotcntSpec;
impl crate::RegisterSpec for TxdmatotcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txdmatotcnt::R`](R) reader structure"]
impl crate::Readable for TxdmatotcntSpec {}
#[doc = "`write(|w| ..)` method takes [`txdmatotcnt::W`](W) writer structure"]
impl crate::Writable for TxdmatotcntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXDMATOTCNT to value 0"]
impl crate::Resettable for TxdmatotcntSpec {
    const RESET_VALUE: u32 = 0;
}
