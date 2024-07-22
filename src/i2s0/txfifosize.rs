#[doc = "Register `TXFIFOSIZE` reader"]
pub type R = crate::R<TxfifosizeSpec>;
#[doc = "Register `TXFIFOSIZE` writer"]
pub type W = crate::W<TxfifosizeSpec>;
#[doc = "Field `SIZE` reader - Size of the transmit FIFO in units of I2S samples. Read only value."]
pub type SizeR = crate::FieldReader<u32>;
#[doc = "Field `SIZE` writer - Size of the transmit FIFO in units of I2S samples. Read only value."]
pub type SizeW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Size of the transmit FIFO in units of I2S samples. Read only value."]
    #[inline(always)]
    pub fn size(&self) -> SizeR {
        SizeR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Size of the transmit FIFO in units of I2S samples. Read only value."]
    #[inline(always)]
    #[must_use]
    pub fn size(&mut self) -> SizeW<TxfifosizeSpec> {
        SizeW::new(self, 0)
    }
}
#[doc = "Holds the size of the transmit FIFO in samples\n\nYou can [`read`](crate::Reg::read) this register and get [`txfifosize::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txfifosize::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxfifosizeSpec;
impl crate::RegisterSpec for TxfifosizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txfifosize::R`](R) reader structure"]
impl crate::Readable for TxfifosizeSpec {}
#[doc = "`write(|w| ..)` method takes [`txfifosize::W`](W) writer structure"]
impl crate::Writable for TxfifosizeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXFIFOSIZE to value 0"]
impl crate::Resettable for TxfifosizeSpec {
    const RESET_VALUE: u32 = 0;
}
