#[doc = "Register `RXDATA` reader"]
pub type R = crate::R<RxdataSpec>;
#[doc = "Register `RXDATA` writer"]
pub type W = crate::W<RxdataSpec>;
#[doc = "Field `RXSAMPLE` reader - 32b audio sample from the internal receive FIFO. MSB is always in bit 31"]
pub type RxsampleR = crate::FieldReader<u32>;
#[doc = "Field `RXSAMPLE` writer - 32b audio sample from the internal receive FIFO. MSB is always in bit 31"]
pub type RxsampleW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 32b audio sample from the internal receive FIFO. MSB is always in bit 31"]
    #[inline(always)]
    pub fn rxsample(&self) -> RxsampleR {
        RxsampleR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 32b audio sample from the internal receive FIFO. MSB is always in bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn rxsample(&mut self) -> RxsampleW<RxdataSpec> {
        RxsampleW::new(self, 0)
    }
}
#[doc = "Read only access to the i2S receive data\n\nYou can [`read`](crate::Reg::read) this register and get [`rxdata::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxdata::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxdataSpec;
impl crate::RegisterSpec for RxdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxdata::R`](R) reader structure"]
impl crate::Readable for RxdataSpec {}
#[doc = "`write(|w| ..)` method takes [`rxdata::W`](W) writer structure"]
impl crate::Writable for RxdataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RXDATA to value 0"]
impl crate::Resettable for RxdataSpec {
    const RESET_VALUE: u32 = 0;
}
