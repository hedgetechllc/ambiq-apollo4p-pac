#[doc = "Register `RSTBITSCOUNTER` reader"]
pub type R = crate::R<RstbitscounterSpec>;
#[doc = "Register `RSTBITSCOUNTER` writer"]
pub type W = crate::W<RstbitscounterSpec>;
#[doc = "Field `RSTBITSCOUNTER` reader - Writing any value to this address resets the bits counter and trng valid registers."]
pub type RstbitscounterR = crate::BitReader;
#[doc = "Field `RSTBITSCOUNTER` writer - Writing any value to this address resets the bits counter and trng valid registers."]
pub type RstbitscounterW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Writing any value to this address resets the bits counter and trng valid registers."]
    #[inline(always)]
    pub fn rstbitscounter(&self) -> RstbitscounterR {
        RstbitscounterR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Writing any value to this address resets the bits counter and trng valid registers."]
    #[inline(always)]
    #[must_use]
    pub fn rstbitscounter(&mut self) -> RstbitscounterW<RstbitscounterSpec> {
        RstbitscounterW::new(self, 0)
    }
}
#[doc = "Resets the counter of collected bits in the TRNG\n\nYou can [`read`](crate::Reg::read) this register and get [`rstbitscounter::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstbitscounter::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RstbitscounterSpec;
impl crate::RegisterSpec for RstbitscounterSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rstbitscounter::R`](R) reader structure"]
impl crate::Readable for RstbitscounterSpec {}
#[doc = "`write(|w| ..)` method takes [`rstbitscounter::W`](W) writer structure"]
impl crate::Writable for RstbitscounterSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RSTBITSCOUNTER to value 0"]
impl crate::Resettable for RstbitscounterSpec {
    const RESET_VALUE: u32 = 0;
}
