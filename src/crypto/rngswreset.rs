#[doc = "Register `RNGSWRESET` reader"]
pub type R = crate::R<RngswresetSpec>;
#[doc = "Register `RNGSWRESET` writer"]
pub type W = crate::W<RngswresetSpec>;
#[doc = "Field `RNGSWRESET` reader - Any value written (0x0 or 0x1) causes a reset cycle to the TRNG block."]
pub type RngswresetR = crate::BitReader;
#[doc = "Field `RNGSWRESET` writer - Any value written (0x0 or 0x1) causes a reset cycle to the TRNG block."]
pub type RngswresetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Any value written (0x0 or 0x1) causes a reset cycle to the TRNG block."]
    #[inline(always)]
    pub fn rngswreset(&self) -> RngswresetR {
        RngswresetR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Any value written (0x0 or 0x1) causes a reset cycle to the TRNG block."]
    #[inline(always)]
    #[must_use]
    pub fn rngswreset(&mut self) -> RngswresetW<RngswresetSpec> {
        RngswresetW::new(self, 0)
    }
}
#[doc = "Generate SW reset solely to RNG block.\n\nYou can [`read`](crate::Reg::read) this register and get [`rngswreset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rngswreset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RngswresetSpec;
impl crate::RegisterSpec for RngswresetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rngswreset::R`](R) reader structure"]
impl crate::Readable for RngswresetSpec {}
#[doc = "`write(|w| ..)` method takes [`rngswreset::W`](W) writer structure"]
impl crate::Writable for RngswresetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RNGSWRESET to value 0"]
impl crate::Resettable for RngswresetSpec {
    const RESET_VALUE: u32 = 0;
}
