#[doc = "Register `RNGDMAENABLE` reader"]
pub type R = crate::R<RngdmaenableSpec>;
#[doc = "Register `RNGDMAENABLE` writer"]
pub type W = crate::W<RngdmaenableSpec>;
#[doc = "Field `EN` reader - Writing value 0x1 enables RNG DMA to SRAM. The Value is cleared when DMA completes its operation."]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - Writing value 0x1 enables RNG DMA to SRAM. The Value is cleared when DMA completes its operation."]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Writing value 0x1 enables RNG DMA to SRAM. The Value is cleared when DMA completes its operation."]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Writing value 0x1 enables RNG DMA to SRAM. The Value is cleared when DMA completes its operation."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EnW<RngdmaenableSpec> {
        EnW::new(self, 0)
    }
}
#[doc = "Writing to this register enables_disables the RNG DMA.\n\nYou can [`read`](crate::Reg::read) this register and get [`rngdmaenable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rngdmaenable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RngdmaenableSpec;
impl crate::RegisterSpec for RngdmaenableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rngdmaenable::R`](R) reader structure"]
impl crate::Readable for RngdmaenableSpec {}
#[doc = "`write(|w| ..)` method takes [`rngdmaenable::W`](W) writer structure"]
impl crate::Writable for RngdmaenableSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RNGDMAENABLE to value 0"]
impl crate::Resettable for RngdmaenableSpec {
    const RESET_VALUE: u32 = 0;
}
