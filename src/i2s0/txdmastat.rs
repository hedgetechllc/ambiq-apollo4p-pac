#[doc = "Register `TXDMASTAT` reader"]
pub type R = crate::R<TxdmastatSpec>;
#[doc = "Register `TXDMASTAT` writer"]
pub type W = crate::W<TxdmastatSpec>;
#[doc = "Field `TXDMATIP` reader - TX DMA Transfer In Progress indicator. 1 will indicate that a DMA transfer is active. The DMA transfer may be waiting on data, transferring data, or waiting for priority. All of these will be indicated with a 1. A 0 will indicate that the DMA is fully complete and no further transactions will be done. This bit is read only."]
pub type TxdmatipR = crate::BitReader;
#[doc = "Field `TXDMATIP` writer - TX DMA Transfer In Progress indicator. 1 will indicate that a DMA transfer is active. The DMA transfer may be waiting on data, transferring data, or waiting for priority. All of these will be indicated with a 1. A 0 will indicate that the DMA is fully complete and no further transactions will be done. This bit is read only."]
pub type TxdmatipW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXDMACPL` reader - TX DMA Transfer Complete. This signals the end of the DMA operation. This bit can be cleared by writing to 0, and will also be cleared when a new DMA is started."]
pub type TxdmacplR = crate::BitReader;
#[doc = "Field `TXDMACPL` writer - TX DMA Transfer Complete. This signals the end of the DMA operation. This bit can be cleared by writing to 0, and will also be cleared when a new DMA is started."]
pub type TxdmacplW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXDMAERR` reader - TX DMA Error. This active high bit signals an error was encountered during the DMA operation. The bit can be cleared by writing to 0. Once set, this bit will remain set until cleared by software."]
pub type TxdmaerrR = crate::BitReader;
#[doc = "Field `TXDMAERR` writer - TX DMA Error. This active high bit signals an error was encountered during the DMA operation. The bit can be cleared by writing to 0. Once set, this bit will remain set until cleared by software."]
pub type TxdmaerrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TX DMA Transfer In Progress indicator. 1 will indicate that a DMA transfer is active. The DMA transfer may be waiting on data, transferring data, or waiting for priority. All of these will be indicated with a 1. A 0 will indicate that the DMA is fully complete and no further transactions will be done. This bit is read only."]
    #[inline(always)]
    pub fn txdmatip(&self) -> TxdmatipR {
        TxdmatipR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TX DMA Transfer Complete. This signals the end of the DMA operation. This bit can be cleared by writing to 0, and will also be cleared when a new DMA is started."]
    #[inline(always)]
    pub fn txdmacpl(&self) -> TxdmacplR {
        TxdmacplR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TX DMA Error. This active high bit signals an error was encountered during the DMA operation. The bit can be cleared by writing to 0. Once set, this bit will remain set until cleared by software."]
    #[inline(always)]
    pub fn txdmaerr(&self) -> TxdmaerrR {
        TxdmaerrR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TX DMA Transfer In Progress indicator. 1 will indicate that a DMA transfer is active. The DMA transfer may be waiting on data, transferring data, or waiting for priority. All of these will be indicated with a 1. A 0 will indicate that the DMA is fully complete and no further transactions will be done. This bit is read only."]
    #[inline(always)]
    #[must_use]
    pub fn txdmatip(&mut self) -> TxdmatipW<TxdmastatSpec> {
        TxdmatipW::new(self, 0)
    }
    #[doc = "Bit 1 - TX DMA Transfer Complete. This signals the end of the DMA operation. This bit can be cleared by writing to 0, and will also be cleared when a new DMA is started."]
    #[inline(always)]
    #[must_use]
    pub fn txdmacpl(&mut self) -> TxdmacplW<TxdmastatSpec> {
        TxdmacplW::new(self, 1)
    }
    #[doc = "Bit 2 - TX DMA Error. This active high bit signals an error was encountered during the DMA operation. The bit can be cleared by writing to 0. Once set, this bit will remain set until cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn txdmaerr(&mut self) -> TxdmaerrW<TxdmastatSpec> {
        TxdmaerrW::new(self, 2)
    }
}
#[doc = "Status of the TX DMA operation currently in progress.\n\nYou can [`read`](crate::Reg::read) this register and get [`txdmastat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdmastat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxdmastatSpec;
impl crate::RegisterSpec for TxdmastatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txdmastat::R`](R) reader structure"]
impl crate::Readable for TxdmastatSpec {}
#[doc = "`write(|w| ..)` method takes [`txdmastat::W`](W) writer structure"]
impl crate::Writable for TxdmastatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXDMASTAT to value 0"]
impl crate::Resettable for TxdmastatSpec {
    const RESET_VALUE: u32 = 0;
}
