#[doc = "Register `INTEN` reader"]
pub type R = crate::R<IntenSpec>;
#[doc = "Register `INTEN` writer"]
pub type W = crate::W<IntenSpec>;
#[doc = "Field `IPB` reader - Interrupt from I2S module"]
pub type IpbR = crate::BitReader;
#[doc = "Field `IPB` writer - Interrupt from I2S module"]
pub type IpbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXREQCNT` reader - The I2S module has completed RXREQCNT number of DMA transfers of size 8 samples. This interrupt allows servicing of buffers at a programmable location within the overall DMA transfer."]
pub type RxreqcntR = crate::BitReader;
#[doc = "Field `RXREQCNT` writer - The I2S module has completed RXREQCNT number of DMA transfers of size 8 samples. This interrupt allows servicing of buffers at a programmable location within the overall DMA transfer."]
pub type RxreqcntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXREQCNT` reader - The I2S module has asserted the dma read request, based on TX fifo level."]
pub type TxreqcntR = crate::BitReader;
#[doc = "Field `TXREQCNT` writer - The I2S module has asserted the dma read request, based on TX fifo level."]
pub type TxreqcntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXDMACPL` reader - A TX dma operation has completed"]
pub type TxdmacplR = crate::BitReader;
#[doc = "Field `TXDMACPL` writer - A TX dma operation has completed"]
pub type TxdmacplW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXDMACPL` reader - A RX dma operation has completed"]
pub type RxdmacplR = crate::BitReader;
#[doc = "Field `RXDMACPL` writer - A RX dma operation has completed"]
pub type RxdmacplW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Interrupt from I2S module"]
    #[inline(always)]
    pub fn ipb(&self) -> IpbR {
        IpbR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The I2S module has completed RXREQCNT number of DMA transfers of size 8 samples. This interrupt allows servicing of buffers at a programmable location within the overall DMA transfer."]
    #[inline(always)]
    pub fn rxreqcnt(&self) -> RxreqcntR {
        RxreqcntR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The I2S module has asserted the dma read request, based on TX fifo level."]
    #[inline(always)]
    pub fn txreqcnt(&self) -> TxreqcntR {
        TxreqcntR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - A TX dma operation has completed"]
    #[inline(always)]
    pub fn txdmacpl(&self) -> TxdmacplR {
        TxdmacplR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - A RX dma operation has completed"]
    #[inline(always)]
    pub fn rxdmacpl(&self) -> RxdmacplR {
        RxdmacplR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt from I2S module"]
    #[inline(always)]
    #[must_use]
    pub fn ipb(&mut self) -> IpbW<IntenSpec> {
        IpbW::new(self, 0)
    }
    #[doc = "Bit 1 - The I2S module has completed RXREQCNT number of DMA transfers of size 8 samples. This interrupt allows servicing of buffers at a programmable location within the overall DMA transfer."]
    #[inline(always)]
    #[must_use]
    pub fn rxreqcnt(&mut self) -> RxreqcntW<IntenSpec> {
        RxreqcntW::new(self, 1)
    }
    #[doc = "Bit 2 - The I2S module has asserted the dma read request, based on TX fifo level."]
    #[inline(always)]
    #[must_use]
    pub fn txreqcnt(&mut self) -> TxreqcntW<IntenSpec> {
        TxreqcntW::new(self, 2)
    }
    #[doc = "Bit 3 - A TX dma operation has completed"]
    #[inline(always)]
    #[must_use]
    pub fn txdmacpl(&mut self) -> TxdmacplW<IntenSpec> {
        TxdmacplW::new(self, 3)
    }
    #[doc = "Bit 4 - A RX dma operation has completed"]
    #[inline(always)]
    #[must_use]
    pub fn rxdmacpl(&mut self) -> RxdmacplW<IntenSpec> {
        RxdmacplW::new(self, 4)
    }
}
#[doc = "Set bits in this register to allow this module to generate the corresponding interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`inten::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenSpec;
impl crate::RegisterSpec for IntenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inten::R`](R) reader structure"]
impl crate::Readable for IntenSpec {}
#[doc = "`write(|w| ..)` method takes [`inten::W`](W) writer structure"]
impl crate::Writable for IntenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for IntenSpec {
    const RESET_VALUE: u32 = 0;
}
