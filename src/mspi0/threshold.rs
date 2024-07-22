#[doc = "Register `THRESHOLD` reader"]
pub type R = crate::R<ThresholdSpec>;
#[doc = "Register `THRESHOLD` writer"]
pub type W = crate::W<ThresholdSpec>;
#[doc = "Field `TXTHRESH` reader - Number of entries in TX FIFO that cause TXF interrupt"]
pub type TxthreshR = crate::FieldReader;
#[doc = "Field `TXTHRESH` writer - Number of entries in TX FIFO that cause TXF interrupt"]
pub type TxthreshW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RXTHRESH` reader - Number of entries in TX FIFO that cause RXE interrupt"]
pub type RxthreshR = crate::FieldReader;
#[doc = "Field `RXTHRESH` writer - Number of entries in TX FIFO that cause RXE interrupt"]
pub type RxthreshW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Number of entries in TX FIFO that cause TXF interrupt"]
    #[inline(always)]
    pub fn txthresh(&self) -> TxthreshR {
        TxthreshR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Number of entries in TX FIFO that cause RXE interrupt"]
    #[inline(always)]
    pub fn rxthresh(&self) -> RxthreshR {
        RxthreshR::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Number of entries in TX FIFO that cause TXF interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn txthresh(&mut self) -> TxthreshW<ThresholdSpec> {
        TxthreshW::new(self, 0)
    }
    #[doc = "Bits 8:13 - Number of entries in TX FIFO that cause RXE interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rxthresh(&mut self) -> RxthreshW<ThresholdSpec> {
        RxthreshW::new(self, 8)
    }
}
#[doc = "Threshold levels that trigger RXFull and TXEmpty interrupts\n\nYou can [`read`](crate::Reg::read) this register and get [`threshold::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`threshold::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ThresholdSpec;
impl crate::RegisterSpec for ThresholdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`threshold::R`](R) reader structure"]
impl crate::Readable for ThresholdSpec {}
#[doc = "`write(|w| ..)` method takes [`threshold::W`](W) writer structure"]
impl crate::Writable for ThresholdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets THRESHOLD to value 0"]
impl crate::Resettable for ThresholdSpec {
    const RESET_VALUE: u32 = 0;
}
