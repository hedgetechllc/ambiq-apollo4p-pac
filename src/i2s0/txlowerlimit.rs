#[doc = "Register `TXLOWERLIMIT` reader"]
pub type R = crate::R<TxlowerlimitSpec>;
#[doc = "Register `TXLOWERLIMIT` writer"]
pub type W = crate::W<TxlowerlimitSpec>;
#[doc = "Field `SIZE` reader - When the number of sample in the TX FIFO goes below this value, the interrupt TX_FFi bit is asserted."]
pub type SizeR = crate::FieldReader<u32>;
#[doc = "Field `SIZE` writer - When the number of sample in the TX FIFO goes below this value, the interrupt TX_FFi bit is asserted."]
pub type SizeW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - When the number of sample in the TX FIFO goes below this value, the interrupt TX_FFi bit is asserted."]
    #[inline(always)]
    pub fn size(&self) -> SizeR {
        SizeR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - When the number of sample in the TX FIFO goes below this value, the interrupt TX_FFi bit is asserted."]
    #[inline(always)]
    #[must_use]
    pub fn size(&mut self) -> SizeW<TxlowerlimitSpec> {
        SizeW::new(self, 0)
    }
}
#[doc = "Minimum number of samples have been reached in the transmit FIFO.\n\nYou can [`read`](crate::Reg::read) this register and get [`txlowerlimit::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txlowerlimit::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxlowerlimitSpec;
impl crate::RegisterSpec for TxlowerlimitSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txlowerlimit::R`](R) reader structure"]
impl crate::Readable for TxlowerlimitSpec {}
#[doc = "`write(|w| ..)` method takes [`txlowerlimit::W`](W) writer structure"]
impl crate::Writable for TxlowerlimitSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXLOWERLIMIT to value 0"]
impl crate::Resettable for TxlowerlimitSpec {
    const RESET_VALUE: u32 = 0;
}
