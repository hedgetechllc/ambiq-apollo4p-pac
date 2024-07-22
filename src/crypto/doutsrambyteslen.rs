#[doc = "Register `DOUTSRAMBYTESLEN` reader"]
pub type R = crate::R<DoutsrambyteslenSpec>;
#[doc = "Register `DOUTSRAMBYTESLEN` writer"]
pub type W = crate::W<DoutsrambyteslenSpec>;
#[doc = "Field `BYTESLEN` reader - Size of data to write to SRAM (bytes). This is the trigger to the SRAM DST DMA."]
pub type ByteslenR = crate::FieldReader<u32>;
#[doc = "Field `BYTESLEN` writer - Size of data to write to SRAM (bytes). This is the trigger to the SRAM DST DMA."]
pub type ByteslenW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Size of data to write to SRAM (bytes). This is the trigger to the SRAM DST DMA."]
    #[inline(always)]
    pub fn byteslen(&self) -> ByteslenR {
        ByteslenR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Size of data to write to SRAM (bytes). This is the trigger to the SRAM DST DMA."]
    #[inline(always)]
    #[must_use]
    pub fn byteslen(&mut self) -> ByteslenW<DoutsrambyteslenSpec> {
        ByteslenW::new(self, 0)
    }
}
#[doc = "This register holds the size of the data (in bytes) to be written to the SRAM. Note: This is a special register, affected by internal logic. Test result of this register is NA.\n\nYou can [`read`](crate::Reg::read) this register and get [`doutsrambyteslen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doutsrambyteslen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DoutsrambyteslenSpec;
impl crate::RegisterSpec for DoutsrambyteslenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doutsrambyteslen::R`](R) reader structure"]
impl crate::Readable for DoutsrambyteslenSpec {}
#[doc = "`write(|w| ..)` method takes [`doutsrambyteslen::W`](W) writer structure"]
impl crate::Writable for DoutsrambyteslenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOUTSRAMBYTESLEN to value 0"]
impl crate::Resettable for DoutsrambyteslenSpec {
    const RESET_VALUE: u32 = 0;
}
