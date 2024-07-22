#[doc = "Register `SRAMDATA` reader"]
pub type R = crate::R<SramdataSpec>;
#[doc = "Register `SRAMDATA` writer"]
pub type W = crate::W<SramdataSpec>;
#[doc = "Field `SRAMDATA` reader - 32 bit write or read from SRAM: read - triggers the SRAM read DMA address automatically incremented write - triggers the SRAM write DMA address automatically incremented"]
pub type SramdataR = crate::FieldReader<u32>;
#[doc = "Field `SRAMDATA` writer - 32 bit write or read from SRAM: read - triggers the SRAM read DMA address automatically incremented write - triggers the SRAM write DMA address automatically incremented"]
pub type SramdataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 32 bit write or read from SRAM: read - triggers the SRAM read DMA address automatically incremented write - triggers the SRAM write DMA address automatically incremented"]
    #[inline(always)]
    pub fn sramdata(&self) -> SramdataR {
        SramdataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 32 bit write or read from SRAM: read - triggers the SRAM read DMA address automatically incremented write - triggers the SRAM write DMA address automatically incremented"]
    #[inline(always)]
    #[must_use]
    pub fn sramdata(&mut self) -> SramdataW<SramdataSpec> {
        SramdataW::new(self, 0)
    }
}
#[doc = "READ WRITE DATA FROM SRAM. Note: This is a special register, affected by internal logic. Test result of this register is NA.\n\nYou can [`read`](crate::Reg::read) this register and get [`sramdata::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sramdata::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SramdataSpec;
impl crate::RegisterSpec for SramdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sramdata::R`](R) reader structure"]
impl crate::Readable for SramdataSpec {}
#[doc = "`write(|w| ..)` method takes [`sramdata::W`](W) writer structure"]
impl crate::Writable for SramdataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRAMDATA to value 0"]
impl crate::Resettable for SramdataSpec {
    const RESET_VALUE: u32 = 0;
}
