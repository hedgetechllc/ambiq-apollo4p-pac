#[doc = "Register `FIFOTHR` reader"]
pub type R = crate::R<FifothrSpec>;
#[doc = "Register `FIFOTHR` writer"]
pub type W = crate::W<FifothrSpec>;
#[doc = "Field `FIFORTHR` reader - FIFO read threshold in bytes. A value of 0 will disable the read FIFO level from activating the threshold interrupt. If this field is non-zero, it will trigger a threshold interrupt when the read fifo contains FIFORTHR valid bytes of data, as indicated by the FIFO1SIZ field. This is intended to signal when a data transfer of FIFORTHR bytes can be done from the IOM module to the host via the read fifo to support large IOM read operations."]
pub type FiforthrR = crate::FieldReader;
#[doc = "Field `FIFORTHR` writer - FIFO read threshold in bytes. A value of 0 will disable the read FIFO level from activating the threshold interrupt. If this field is non-zero, it will trigger a threshold interrupt when the read fifo contains FIFORTHR valid bytes of data, as indicated by the FIFO1SIZ field. This is intended to signal when a data transfer of FIFORTHR bytes can be done from the IOM module to the host via the read fifo to support large IOM read operations."]
pub type FiforthrW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `FIFOWTHR` reader - FIFO write threshold in bytes. A value of 0 will disable the write FIFO level from activating the threshold interrupt. If this field is non-zero, it will trigger a threshold interrupt when the write fifo contains FIFOWTHR free bytes, as indicated by the FIFO0REM field. This is intended to signal when a transfer of FIFOWTHR bytes can be done from the host to the IOM write fifo to support large IOM write operations."]
pub type FifowthrR = crate::FieldReader;
#[doc = "Field `FIFOWTHR` writer - FIFO write threshold in bytes. A value of 0 will disable the write FIFO level from activating the threshold interrupt. If this field is non-zero, it will trigger a threshold interrupt when the write fifo contains FIFOWTHR free bytes, as indicated by the FIFO0REM field. This is intended to signal when a transfer of FIFOWTHR bytes can be done from the host to the IOM write fifo to support large IOM write operations."]
pub type FifowthrW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - FIFO read threshold in bytes. A value of 0 will disable the read FIFO level from activating the threshold interrupt. If this field is non-zero, it will trigger a threshold interrupt when the read fifo contains FIFORTHR valid bytes of data, as indicated by the FIFO1SIZ field. This is intended to signal when a data transfer of FIFORTHR bytes can be done from the IOM module to the host via the read fifo to support large IOM read operations."]
    #[inline(always)]
    pub fn fiforthr(&self) -> FiforthrR {
        FiforthrR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - FIFO write threshold in bytes. A value of 0 will disable the write FIFO level from activating the threshold interrupt. If this field is non-zero, it will trigger a threshold interrupt when the write fifo contains FIFOWTHR free bytes, as indicated by the FIFO0REM field. This is intended to signal when a transfer of FIFOWTHR bytes can be done from the host to the IOM write fifo to support large IOM write operations."]
    #[inline(always)]
    pub fn fifowthr(&self) -> FifowthrR {
        FifowthrR::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - FIFO read threshold in bytes. A value of 0 will disable the read FIFO level from activating the threshold interrupt. If this field is non-zero, it will trigger a threshold interrupt when the read fifo contains FIFORTHR valid bytes of data, as indicated by the FIFO1SIZ field. This is intended to signal when a data transfer of FIFORTHR bytes can be done from the IOM module to the host via the read fifo to support large IOM read operations."]
    #[inline(always)]
    #[must_use]
    pub fn fiforthr(&mut self) -> FiforthrW<FifothrSpec> {
        FiforthrW::new(self, 0)
    }
    #[doc = "Bits 8:13 - FIFO write threshold in bytes. A value of 0 will disable the write FIFO level from activating the threshold interrupt. If this field is non-zero, it will trigger a threshold interrupt when the write fifo contains FIFOWTHR free bytes, as indicated by the FIFO0REM field. This is intended to signal when a transfer of FIFOWTHR bytes can be done from the host to the IOM write fifo to support large IOM write operations."]
    #[inline(always)]
    #[must_use]
    pub fn fifowthr(&mut self) -> FifowthrW<FifothrSpec> {
        FifowthrW::new(self, 8)
    }
}
#[doc = "Sets the threshold values for incoming and outgoing transactions. The threshold values are used to assert the interrupt if enabled, and also used during DMA to set the transfer size as a result of DMATHR trigger. The WTHR is used to indicate when there are more than WTHR bytes of open fifo locations available in the outgoing FIFO (FIFO0). The intended use to invoke an interrupt or DMA transfer that will refill the FIFO with a byte count up to this value. The RTHR is used to indicate when there are more than RTHR bytes in the incoming FIFO (FIFO1) and a data transfer of this size can be supported, either through direct POP of the FIFO, or through DMA. The value of both RTHR and WTHR are also used to set the data transfer size of DMA operations if DMATHR trigger is enabled.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifothr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifothr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FifothrSpec;
impl crate::RegisterSpec for FifothrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifothr::R`](R) reader structure"]
impl crate::Readable for FifothrSpec {}
#[doc = "`write(|w| ..)` method takes [`fifothr::W`](W) writer structure"]
impl crate::Writable for FifothrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FIFOTHR to value 0"]
impl crate::Resettable for FifothrSpec {
    const RESET_VALUE: u32 = 0;
}
