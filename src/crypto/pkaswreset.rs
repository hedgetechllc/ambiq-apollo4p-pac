#[doc = "Register `PKASWRESET` reader"]
pub type R = crate::R<PkaswresetSpec>;
#[doc = "Register `PKASWRESET` writer"]
pub type W = crate::W<PkaswresetSpec>;
#[doc = "Field `PKASWRESET` reader - The reset mechanism takes about four PKA clock cycles until the reset line is deasserted"]
pub type PkaswresetR = crate::BitReader;
#[doc = "Field `PKASWRESET` writer - The reset mechanism takes about four PKA clock cycles until the reset line is deasserted"]
pub type PkaswresetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The reset mechanism takes about four PKA clock cycles until the reset line is deasserted"]
    #[inline(always)]
    pub fn pkaswreset(&self) -> PkaswresetR {
        PkaswresetR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The reset mechanism takes about four PKA clock cycles until the reset line is deasserted"]
    #[inline(always)]
    #[must_use]
    pub fn pkaswreset(&mut self) -> PkaswresetW<PkaswresetSpec> {
        PkaswresetW::new(self, 0)
    }
}
#[doc = "Writing to this register triggers a software reset of the PKA.\n\nYou can [`read`](crate::Reg::read) this register and get [`pkaswreset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pkaswreset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PkaswresetSpec;
impl crate::RegisterSpec for PkaswresetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pkaswreset::R`](R) reader structure"]
impl crate::Readable for PkaswresetSpec {}
#[doc = "`write(|w| ..)` method takes [`pkaswreset::W`](W) writer structure"]
impl crate::Writable for PkaswresetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PKASWRESET to value 0"]
impl crate::Resettable for PkaswresetSpec {
    const RESET_VALUE: u32 = 0;
}
