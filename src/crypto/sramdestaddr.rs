#[doc = "Register `SRAMDESTADDR` reader"]
pub type R = crate::R<SramdestaddrSpec>;
#[doc = "Register `SRAMDESTADDR` writer"]
pub type W = crate::W<SramdestaddrSpec>;
#[doc = "Field `SRAMDEST` reader - SRAM destination base address for data."]
pub type SramdestR = crate::FieldReader<u32>;
#[doc = "Field `SRAMDEST` writer - SRAM destination base address for data."]
pub type SramdestW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SRAM destination base address for data."]
    #[inline(always)]
    pub fn sramdest(&self) -> SramdestR {
        SramdestR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SRAM destination base address for data."]
    #[inline(always)]
    #[must_use]
    pub fn sramdest(&mut self) -> SramdestW<SramdestaddrSpec> {
        SramdestW::new(self, 0)
    }
}
#[doc = "Location of result to be sent to in SRAM. Note: This is a special register, affected by internal logic. Test result of this register is NA.\n\nYou can [`read`](crate::Reg::read) this register and get [`sramdestaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sramdestaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SramdestaddrSpec;
impl crate::RegisterSpec for SramdestaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sramdestaddr::R`](R) reader structure"]
impl crate::Readable for SramdestaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`sramdestaddr::W`](W) writer structure"]
impl crate::Writable for SramdestaddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRAMDESTADDR to value 0"]
impl crate::Resettable for SramdestaddrSpec {
    const RESET_VALUE: u32 = 0;
}
