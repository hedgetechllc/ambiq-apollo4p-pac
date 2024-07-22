#[doc = "Register `SRAMDATAREADY` reader"]
pub type R = crate::R<SramdatareadySpec>;
#[doc = "Register `SRAMDATAREADY` writer"]
pub type W = crate::W<SramdatareadySpec>;
#[doc = "Field `SRAMREADY` reader - SRAM content is ready for read in SRAM_DATA."]
pub type SramreadyR = crate::BitReader;
#[doc = "Field `SRAMREADY` writer - SRAM content is ready for read in SRAM_DATA."]
pub type SramreadyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SRAM content is ready for read in SRAM_DATA."]
    #[inline(always)]
    pub fn sramready(&self) -> SramreadyR {
        SramreadyR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SRAM content is ready for read in SRAM_DATA."]
    #[inline(always)]
    #[must_use]
    pub fn sramready(&mut self) -> SramreadyW<SramdatareadySpec> {
        SramreadyW::new(self, 0)
    }
}
#[doc = "The SRAM content is ready for read in SRAM_DATA.\n\nYou can [`read`](crate::Reg::read) this register and get [`sramdataready::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sramdataready::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SramdatareadySpec;
impl crate::RegisterSpec for SramdatareadySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sramdataready::R`](R) reader structure"]
impl crate::Readable for SramdatareadySpec {}
#[doc = "`write(|w| ..)` method takes [`sramdataready::W`](W) writer structure"]
impl crate::Writable for SramdatareadySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRAMDATAREADY to value 0x01"]
impl crate::Resettable for SramdatareadySpec {
    const RESET_VALUE: u32 = 0x01;
}
