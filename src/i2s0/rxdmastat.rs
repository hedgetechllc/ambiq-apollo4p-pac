#[doc = "Register `RXDMASTAT` reader"]
pub type R = crate::R<RxdmastatSpec>;
#[doc = "Register `RXDMASTAT` writer"]
pub type W = crate::W<RxdmastatSpec>;
#[doc = "Field `RXDMATIP` reader - RX DMA Transfer In Progress indicator. 1 will indicate that a DMA transfer is active. The DMA transfer may be waiting on data, transferring data, or waiting for priority. All of these will be indicated with a 1. A 0 will indicate that the DMA is fully complete and no further transactions will be done. This bit is read only."]
pub type RxdmatipR = crate::BitReader;
#[doc = "Field `RXDMATIP` writer - RX DMA Transfer In Progress indicator. 1 will indicate that a DMA transfer is active. The DMA transfer may be waiting on data, transferring data, or waiting for priority. All of these will be indicated with a 1. A 0 will indicate that the DMA is fully complete and no further transactions will be done. This bit is read only."]
pub type RxdmatipW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXDMACPL` reader - RX DMA Transfer Complete. This signals the end of the DMA operation. This bit can be cleared by writing to 0, and will also be cleared when a new DMA is started."]
pub type RxdmacplR = crate::BitReader;
#[doc = "Field `RXDMACPL` writer - RX DMA Transfer Complete. This signals the end of the DMA operation. This bit can be cleared by writing to 0, and will also be cleared when a new DMA is started."]
pub type RxdmacplW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXDMAERR` reader - RX DMA Error. This active high bit signals an error was encountered during the DMA operation. The bit can be cleared by writing to 0. Once set, this bit will remain set until cleared by software."]
pub type RxdmaerrR = crate::BitReader;
#[doc = "Field `RXDMAERR` writer - RX DMA Error. This active high bit signals an error was encountered during the DMA operation. The bit can be cleared by writing to 0. Once set, this bit will remain set until cleared by software."]
pub type RxdmaerrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RX DMA Transfer In Progress indicator. 1 will indicate that a DMA transfer is active. The DMA transfer may be waiting on data, transferring data, or waiting for priority. All of these will be indicated with a 1. A 0 will indicate that the DMA is fully complete and no further transactions will be done. This bit is read only."]
    #[inline(always)]
    pub fn rxdmatip(&self) -> RxdmatipR {
        RxdmatipR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RX DMA Transfer Complete. This signals the end of the DMA operation. This bit can be cleared by writing to 0, and will also be cleared when a new DMA is started."]
    #[inline(always)]
    pub fn rxdmacpl(&self) -> RxdmacplR {
        RxdmacplR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RX DMA Error. This active high bit signals an error was encountered during the DMA operation. The bit can be cleared by writing to 0. Once set, this bit will remain set until cleared by software."]
    #[inline(always)]
    pub fn rxdmaerr(&self) -> RxdmaerrR {
        RxdmaerrR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RX DMA Transfer In Progress indicator. 1 will indicate that a DMA transfer is active. The DMA transfer may be waiting on data, transferring data, or waiting for priority. All of these will be indicated with a 1. A 0 will indicate that the DMA is fully complete and no further transactions will be done. This bit is read only."]
    #[inline(always)]
    #[must_use]
    pub fn rxdmatip(&mut self) -> RxdmatipW<RxdmastatSpec> {
        RxdmatipW::new(self, 0)
    }
    #[doc = "Bit 1 - RX DMA Transfer Complete. This signals the end of the DMA operation. This bit can be cleared by writing to 0, and will also be cleared when a new DMA is started."]
    #[inline(always)]
    #[must_use]
    pub fn rxdmacpl(&mut self) -> RxdmacplW<RxdmastatSpec> {
        RxdmacplW::new(self, 1)
    }
    #[doc = "Bit 2 - RX DMA Error. This active high bit signals an error was encountered during the DMA operation. The bit can be cleared by writing to 0. Once set, this bit will remain set until cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn rxdmaerr(&mut self) -> RxdmaerrW<RxdmastatSpec> {
        RxdmaerrW::new(self, 2)
    }
}
#[doc = "Status of the RX DMA operation currently in progress.\n\nYou can [`read`](crate::Reg::read) this register and get [`rxdmastat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxdmastat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxdmastatSpec;
impl crate::RegisterSpec for RxdmastatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxdmastat::R`](R) reader structure"]
impl crate::Readable for RxdmastatSpec {}
#[doc = "`write(|w| ..)` method takes [`rxdmastat::W`](W) writer structure"]
impl crate::Writable for RxdmastatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RXDMASTAT to value 0"]
impl crate::Resettable for RxdmastatSpec {
    const RESET_VALUE: u32 = 0;
}
