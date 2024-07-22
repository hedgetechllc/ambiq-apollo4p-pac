#[doc = "Register `TXDATA` reader"]
pub type R = crate::R<TxdataSpec>;
#[doc = "Register `TXDATA` writer"]
pub type W = crate::W<TxdataSpec>;
#[doc = "Field `TXSAMPLE` reader - 32b I2S sample to send out of the I2S module via the external pins. All sample have the MSB in bit 31 regardless of number of bits per sample and data justification"]
pub type TxsampleR = crate::FieldReader<u32>;
#[doc = "Field `TXSAMPLE` writer - 32b I2S sample to send out of the I2S module via the external pins. All sample have the MSB in bit 31 regardless of number of bits per sample and data justification"]
pub type TxsampleW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 32b I2S sample to send out of the I2S module via the external pins. All sample have the MSB in bit 31 regardless of number of bits per sample and data justification"]
    #[inline(always)]
    pub fn txsample(&self) -> TxsampleR {
        TxsampleR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 32b I2S sample to send out of the I2S module via the external pins. All sample have the MSB in bit 31 regardless of number of bits per sample and data justification"]
    #[inline(always)]
    #[must_use]
    pub fn txsample(&mut self) -> TxsampleW<TxdataSpec> {
        TxsampleW::new(self, 0)
    }
}
#[doc = "Write only register to hold the i2S sample to transmit via the write FIFO\n\nYou can [`read`](crate::Reg::read) this register and get [`txdata::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdata::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxdataSpec;
impl crate::RegisterSpec for TxdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txdata::R`](R) reader structure"]
impl crate::Readable for TxdataSpec {}
#[doc = "`write(|w| ..)` method takes [`txdata::W`](W) writer structure"]
impl crate::Writable for TxdataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXDATA to value 0"]
impl crate::Resettable for TxdataSpec {
    const RESET_VALUE: u32 = 0;
}
