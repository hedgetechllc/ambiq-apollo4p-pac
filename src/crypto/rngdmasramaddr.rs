#[doc = "Register `RNGDMASRAMADDR` reader"]
pub type R = crate::R<RngdmasramaddrSpec>;
#[doc = "Register `RNGDMASRAMADDR` writer"]
pub type W = crate::W<RngdmasramaddrSpec>;
#[doc = "Field `RNGSRAMDMAADDR` reader - Defines the start address of the DMA for the TRNG data."]
pub type RngsramdmaaddrR = crate::FieldReader<u16>;
#[doc = "Field `RNGSRAMDMAADDR` writer - Defines the start address of the DMA for the TRNG data."]
pub type RngsramdmaaddrW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - Defines the start address of the DMA for the TRNG data."]
    #[inline(always)]
    pub fn rngsramdmaaddr(&self) -> RngsramdmaaddrR {
        RngsramdmaaddrR::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Defines the start address of the DMA for the TRNG data."]
    #[inline(always)]
    #[must_use]
    pub fn rngsramdmaaddr(&mut self) -> RngsramdmaaddrW<RngdmasramaddrSpec> {
        RngsramdmaaddrW::new(self, 0)
    }
}
#[doc = "This register defines the start address of the DMA for the TRNG data.\n\nYou can [`read`](crate::Reg::read) this register and get [`rngdmasramaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rngdmasramaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RngdmasramaddrSpec;
impl crate::RegisterSpec for RngdmasramaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rngdmasramaddr::R`](R) reader structure"]
impl crate::Readable for RngdmasramaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`rngdmasramaddr::W`](W) writer structure"]
impl crate::Writable for RngdmasramaddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RNGDMASRAMADDR to value 0"]
impl crate::Resettable for RngdmasramaddrSpec {
    const RESET_VALUE: u32 = 0;
}
