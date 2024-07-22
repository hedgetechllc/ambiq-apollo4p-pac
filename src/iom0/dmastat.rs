#[doc = "Register `DMASTAT` reader"]
pub type R = crate::R<DmastatSpec>;
#[doc = "Register `DMASTAT` writer"]
pub type W = crate::W<DmastatSpec>;
#[doc = "Field `DMATIP` reader - DMA Transfer In Progress indicator. 1 will indicate that a DMA transfer is active. The DMA transfer may be waiting on data, transferring data, or waiting for priority. All of these will be indicated with a 1. A 0 will indicate that the DMA is fully complete and no further transactions will be done. This bit is read only."]
pub type DmatipR = crate::BitReader;
#[doc = "Field `DMATIP` writer - DMA Transfer In Progress indicator. 1 will indicate that a DMA transfer is active. The DMA transfer may be waiting on data, transferring data, or waiting for priority. All of these will be indicated with a 1. A 0 will indicate that the DMA is fully complete and no further transactions will be done. This bit is read only."]
pub type DmatipW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMACPL` reader - DMA Transfer Complete. This signals the end of the DMA operation. This bit can be cleared by writing to 0, and will also be cleared when a new DMA is started."]
pub type DmacplR = crate::BitReader;
#[doc = "Field `DMACPL` writer - DMA Transfer Complete. This signals the end of the DMA operation. This bit can be cleared by writing to 0, and will also be cleared when a new DMA is started."]
pub type DmacplW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAERR` reader - DMA Error. This active high bit signals an error was encountered during the DMA operation. The bit can be cleared by writing to 0. Once set, this bit will remain set until cleared by software."]
pub type DmaerrR = crate::BitReader;
#[doc = "Field `DMAERR` writer - DMA Error. This active high bit signals an error was encountered during the DMA operation. The bit can be cleared by writing to 0. Once set, this bit will remain set until cleared by software."]
pub type DmaerrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DMA Transfer In Progress indicator. 1 will indicate that a DMA transfer is active. The DMA transfer may be waiting on data, transferring data, or waiting for priority. All of these will be indicated with a 1. A 0 will indicate that the DMA is fully complete and no further transactions will be done. This bit is read only."]
    #[inline(always)]
    pub fn dmatip(&self) -> DmatipR {
        DmatipR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA Transfer Complete. This signals the end of the DMA operation. This bit can be cleared by writing to 0, and will also be cleared when a new DMA is started."]
    #[inline(always)]
    pub fn dmacpl(&self) -> DmacplR {
        DmacplR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMA Error. This active high bit signals an error was encountered during the DMA operation. The bit can be cleared by writing to 0. Once set, this bit will remain set until cleared by software."]
    #[inline(always)]
    pub fn dmaerr(&self) -> DmaerrR {
        DmaerrR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA Transfer In Progress indicator. 1 will indicate that a DMA transfer is active. The DMA transfer may be waiting on data, transferring data, or waiting for priority. All of these will be indicated with a 1. A 0 will indicate that the DMA is fully complete and no further transactions will be done. This bit is read only."]
    #[inline(always)]
    #[must_use]
    pub fn dmatip(&mut self) -> DmatipW<DmastatSpec> {
        DmatipW::new(self, 0)
    }
    #[doc = "Bit 1 - DMA Transfer Complete. This signals the end of the DMA operation. This bit can be cleared by writing to 0, and will also be cleared when a new DMA is started."]
    #[inline(always)]
    #[must_use]
    pub fn dmacpl(&mut self) -> DmacplW<DmastatSpec> {
        DmacplW::new(self, 1)
    }
    #[doc = "Bit 2 - DMA Error. This active high bit signals an error was encountered during the DMA operation. The bit can be cleared by writing to 0. Once set, this bit will remain set until cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn dmaerr(&mut self) -> DmaerrW<DmastatSpec> {
        DmaerrW::new(self, 2)
    }
}
#[doc = "Status of the DMA operation currently in progress.\n\nYou can [`read`](crate::Reg::read) this register and get [`dmastat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmastat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmastatSpec;
impl crate::RegisterSpec for DmastatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmastat::R`](R) reader structure"]
impl crate::Readable for DmastatSpec {}
#[doc = "`write(|w| ..)` method takes [`dmastat::W`](W) writer structure"]
impl crate::Writable for DmastatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMASTAT to value 0"]
impl crate::Resettable for DmastatSpec {
    const RESET_VALUE: u32 = 0;
}
