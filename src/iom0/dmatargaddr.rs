#[doc = "Register `DMATARGADDR` reader"]
pub type R = crate::R<DmatargaddrSpec>;
#[doc = "Register `DMATARGADDR` writer"]
pub type W = crate::W<DmatargaddrSpec>;
#[doc = "Field `TARGADDR` reader - Bits \\[28:0\\]
of the target byte address for source of DMA (either read or write). The address can be any byte alignment, and does not have to be word aligned. In cases of non-word aligned addresses, the DMA logic will take care for ensuring only the target bytes are read/written."]
pub type TargaddrR = crate::FieldReader<u32>;
#[doc = "Field `TARGADDR` writer - Bits \\[28:0\\]
of the target byte address for source of DMA (either read or write). The address can be any byte alignment, and does not have to be word aligned. In cases of non-word aligned addresses, the DMA logic will take care for ensuring only the target bytes are read/written."]
pub type TargaddrW<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bits 0:28 - Bits \\[28:0\\]
of the target byte address for source of DMA (either read or write). The address can be any byte alignment, and does not have to be word aligned. In cases of non-word aligned addresses, the DMA logic will take care for ensuring only the target bytes are read/written."]
    #[inline(always)]
    pub fn targaddr(&self) -> TargaddrR {
        TargaddrR::new(self.bits & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:28 - Bits \\[28:0\\]
of the target byte address for source of DMA (either read or write). The address can be any byte alignment, and does not have to be word aligned. In cases of non-word aligned addresses, the DMA logic will take care for ensuring only the target bytes are read/written."]
    #[inline(always)]
    #[must_use]
    pub fn targaddr(&mut self) -> TargaddrW<DmatargaddrSpec> {
        TargaddrW::new(self, 0)
    }
}
#[doc = "The source or destination address internal the SRAM for the DMA data. For write operations, this can only be SRAM data (ADDR bit 28 = 1); For read operations, this can ve either SRAM or FLASH (ADDR bit 28 = 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`dmatargaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmatargaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmatargaddrSpec;
impl crate::RegisterSpec for DmatargaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmatargaddr::R`](R) reader structure"]
impl crate::Readable for DmatargaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`dmatargaddr::W`](W) writer structure"]
impl crate::Writable for DmatargaddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMATARGADDR to value 0"]
impl crate::Resettable for DmatargaddrSpec {
    const RESET_VALUE: u32 = 0;
}
