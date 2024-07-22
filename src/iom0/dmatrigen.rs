#[doc = "Register `DMATRIGEN` reader"]
pub type R = crate::R<DmatrigenSpec>;
#[doc = "Register `DMATRIGEN` writer"]
pub type W = crate::W<DmatrigenSpec>;
#[doc = "Field `DCMDCMPEN` reader - Trigger DMA upon command complete. Enables the trigger of the DMA when a command is completed. When this event is triggered, the number of words transferred will be the lesser of the remaining TOTCOUNT bytes, or"]
pub type DcmdcmpenR = crate::BitReader;
#[doc = "Field `DCMDCMPEN` writer - Trigger DMA upon command complete. Enables the trigger of the DMA when a command is completed. When this event is triggered, the number of words transferred will be the lesser of the remaining TOTCOUNT bytes, or"]
pub type DcmdcmpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTHREN` reader - Trigger DMA upon THR level reached. For M2P DMA operations (IOM writes), the trigger will assert when the write FIFO has (WTHR/4) number of words free in the write FIFO, and will transfer (WTHR/4) number of words or, if the number of words left to transfer is less than the WTHR value, will transfer the remaining byte count. For P2M DMA operations, the trigger will assert when the read FIFO has (RTHR/4) words available in the read FIFO, and will transfer (RTHR/4) words to SRAM. This trigger will NOT assert when the transaction completes and there are less than RTHR bytes left in the fifo, since the RTHR has not been reached. In this case, the CMDCMP trigger must also be enabled to transfer the remaining read FIFO data to SRAM."]
pub type DthrenR = crate::BitReader;
#[doc = "Field `DTHREN` writer - Trigger DMA upon THR level reached. For M2P DMA operations (IOM writes), the trigger will assert when the write FIFO has (WTHR/4) number of words free in the write FIFO, and will transfer (WTHR/4) number of words or, if the number of words left to transfer is less than the WTHR value, will transfer the remaining byte count. For P2M DMA operations, the trigger will assert when the read FIFO has (RTHR/4) words available in the read FIFO, and will transfer (RTHR/4) words to SRAM. This trigger will NOT assert when the transaction completes and there are less than RTHR bytes left in the fifo, since the RTHR has not been reached. In this case, the CMDCMP trigger must also be enabled to transfer the remaining read FIFO data to SRAM."]
pub type DthrenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Trigger DMA upon command complete. Enables the trigger of the DMA when a command is completed. When this event is triggered, the number of words transferred will be the lesser of the remaining TOTCOUNT bytes, or"]
    #[inline(always)]
    pub fn dcmdcmpen(&self) -> DcmdcmpenR {
        DcmdcmpenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Trigger DMA upon THR level reached. For M2P DMA operations (IOM writes), the trigger will assert when the write FIFO has (WTHR/4) number of words free in the write FIFO, and will transfer (WTHR/4) number of words or, if the number of words left to transfer is less than the WTHR value, will transfer the remaining byte count. For P2M DMA operations, the trigger will assert when the read FIFO has (RTHR/4) words available in the read FIFO, and will transfer (RTHR/4) words to SRAM. This trigger will NOT assert when the transaction completes and there are less than RTHR bytes left in the fifo, since the RTHR has not been reached. In this case, the CMDCMP trigger must also be enabled to transfer the remaining read FIFO data to SRAM."]
    #[inline(always)]
    pub fn dthren(&self) -> DthrenR {
        DthrenR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Trigger DMA upon command complete. Enables the trigger of the DMA when a command is completed. When this event is triggered, the number of words transferred will be the lesser of the remaining TOTCOUNT bytes, or"]
    #[inline(always)]
    #[must_use]
    pub fn dcmdcmpen(&mut self) -> DcmdcmpenW<DmatrigenSpec> {
        DcmdcmpenW::new(self, 0)
    }
    #[doc = "Bit 1 - Trigger DMA upon THR level reached. For M2P DMA operations (IOM writes), the trigger will assert when the write FIFO has (WTHR/4) number of words free in the write FIFO, and will transfer (WTHR/4) number of words or, if the number of words left to transfer is less than the WTHR value, will transfer the remaining byte count. For P2M DMA operations, the trigger will assert when the read FIFO has (RTHR/4) words available in the read FIFO, and will transfer (RTHR/4) words to SRAM. This trigger will NOT assert when the transaction completes and there are less than RTHR bytes left in the fifo, since the RTHR has not been reached. In this case, the CMDCMP trigger must also be enabled to transfer the remaining read FIFO data to SRAM."]
    #[inline(always)]
    #[must_use]
    pub fn dthren(&mut self) -> DthrenW<DmatrigenSpec> {
        DthrenW::new(self, 1)
    }
}
#[doc = "Provides control on which event will trigger the DMA transfer after the DMA operation is setup and enabled. The trigger event will cause a number of bytes (depending on trigger event) to be transferred via the DMA operation, and can be used to adjust the latency of data to/from the IOM module to/from the dma target. DMA transfers are broken into smaller transfers internally of up to 16 bytes each, and multiple trigger events can be used to complete the entire programmed DMA transfer.\n\nYou can [`read`](crate::Reg::read) this register and get [`dmatrigen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmatrigen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmatrigenSpec;
impl crate::RegisterSpec for DmatrigenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmatrigen::R`](R) reader structure"]
impl crate::Readable for DmatrigenSpec {}
#[doc = "`write(|w| ..)` method takes [`dmatrigen::W`](W) writer structure"]
impl crate::Writable for DmatrigenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMATRIGEN to value 0"]
impl crate::Resettable for DmatrigenSpec {
    const RESET_VALUE: u32 = 0;
}
