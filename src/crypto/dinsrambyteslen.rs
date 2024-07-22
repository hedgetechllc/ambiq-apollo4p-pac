#[doc = "Register `DINSRAMBYTESLEN` reader"]
pub type R = crate::R<DinsrambyteslenSpec>;
#[doc = "Register `DINSRAMBYTESLEN` writer"]
pub type W = crate::W<DinsrambyteslenSpec>;
#[doc = "Field `BYTESLEN` reader - Size of data to read from SRAM (bytes). This is the trigger to the SRAM SRC DMA."]
pub type ByteslenR = crate::FieldReader<u32>;
#[doc = "Field `BYTESLEN` writer - Size of data to read from SRAM (bytes). This is the trigger to the SRAM SRC DMA."]
pub type ByteslenW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Size of data to read from SRAM (bytes). This is the trigger to the SRAM SRC DMA."]
    #[inline(always)]
    pub fn byteslen(&self) -> ByteslenR {
        ByteslenR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Size of data to read from SRAM (bytes). This is the trigger to the SRAM SRC DMA."]
    #[inline(always)]
    #[must_use]
    pub fn byteslen(&mut self) -> ByteslenW<DinsrambyteslenSpec> {
        ByteslenW::new(self, 0)
    }
}
#[doc = "This register holds the size of the data (in bytes) to be read from the SRAM. Note: This is a special register, affected by internal logic. Test result of this register is NA.\n\nYou can [`read`](crate::Reg::read) this register and get [`dinsrambyteslen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dinsrambyteslen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DinsrambyteslenSpec;
impl crate::RegisterSpec for DinsrambyteslenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dinsrambyteslen::R`](R) reader structure"]
impl crate::Readable for DinsrambyteslenSpec {}
#[doc = "`write(|w| ..)` method takes [`dinsrambyteslen::W`](W) writer structure"]
impl crate::Writable for DinsrambyteslenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DINSRAMBYTESLEN to value 0"]
impl crate::Resettable for DinsrambyteslenSpec {
    const RESET_VALUE: u32 = 0;
}
