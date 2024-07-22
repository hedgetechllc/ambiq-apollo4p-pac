#[doc = "Register `RXUPPERLIMIT` reader"]
pub type R = crate::R<RxupperlimitSpec>;
#[doc = "Register `RXUPPERLIMIT` writer"]
pub type W = crate::W<RxupperlimitSpec>;
#[doc = "Field `SIZE` reader - When the I2S sample count stored within the receive FIFO reaches this value or is larger, the interrupt RX_FFi bit is asserted."]
pub type SizeR = crate::FieldReader<u32>;
#[doc = "Field `SIZE` writer - When the I2S sample count stored within the receive FIFO reaches this value or is larger, the interrupt RX_FFi bit is asserted."]
pub type SizeW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - When the I2S sample count stored within the receive FIFO reaches this value or is larger, the interrupt RX_FFi bit is asserted."]
    #[inline(always)]
    pub fn size(&self) -> SizeR {
        SizeR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - When the I2S sample count stored within the receive FIFO reaches this value or is larger, the interrupt RX_FFi bit is asserted."]
    #[inline(always)]
    #[must_use]
    pub fn size(&mut self) -> SizeW<RxupperlimitSpec> {
        SizeW::new(self, 0)
    }
}
#[doc = "The number of samples required to be in the RX FIFO before asserting the RX_FFi interrupt bit\n\nYou can [`read`](crate::Reg::read) this register and get [`rxupperlimit::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxupperlimit::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxupperlimitSpec;
impl crate::RegisterSpec for RxupperlimitSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxupperlimit::R`](R) reader structure"]
impl crate::Readable for RxupperlimitSpec {}
#[doc = "`write(|w| ..)` method takes [`rxupperlimit::W`](W) writer structure"]
impl crate::Writable for RxupperlimitSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RXUPPERLIMIT to value 0"]
impl crate::Resettable for RxupperlimitSpec {
    const RESET_VALUE: u32 = 0;
}
