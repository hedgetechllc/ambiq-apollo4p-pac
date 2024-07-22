#[doc = "Register `SIMOBUCK0` reader"]
pub type R = crate::R<Simobuck0Spec>;
#[doc = "Register `SIMOBUCK0` writer"]
pub type W = crate::W<Simobuck0Spec>;
#[doc = "Field `VDDCRXCOMPEN` reader - Enable the VDDC rail."]
pub type VddcrxcompenR = crate::BitReader;
#[doc = "Field `VDDCRXCOMPEN` writer - Enable the VDDC rail."]
pub type VddcrxcompenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDDFRXCOMPEN` reader - Enable the VDDS rail."]
pub type VddfrxcompenR = crate::BitReader;
#[doc = "Field `VDDFRXCOMPEN` writer - Enable the VDDS rail."]
pub type VddfrxcompenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDDSRXCOMPEN` reader - Enable the VDDS rail."]
pub type VddsrxcompenR = crate::BitReader;
#[doc = "Field `VDDSRXCOMPEN` writer - Enable the VDDS rail."]
pub type VddsrxcompenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDDCLVRXCOMPEN` reader - Enable the VDDC LV rail."]
pub type VddclvrxcompenR = crate::BitReader;
#[doc = "Field `VDDCLVRXCOMPEN` writer - Enable the VDDC LV rail."]
pub type VddclvrxcompenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TONTOFFNODEGLITCH` reader - Enable the ton and toff signals no deglitch output."]
pub type TontoffnodeglitchR = crate::BitReader;
#[doc = "Field `TONTOFFNODEGLITCH` writer - Enable the ton and toff signals no deglitch output."]
pub type TontoffnodeglitchW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable the VDDC rail."]
    #[inline(always)]
    pub fn vddcrxcompen(&self) -> VddcrxcompenR {
        VddcrxcompenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable the VDDS rail."]
    #[inline(always)]
    pub fn vddfrxcompen(&self) -> VddfrxcompenR {
        VddfrxcompenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable the VDDS rail."]
    #[inline(always)]
    pub fn vddsrxcompen(&self) -> VddsrxcompenR {
        VddsrxcompenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable the VDDC LV rail."]
    #[inline(always)]
    pub fn vddclvrxcompen(&self) -> VddclvrxcompenR {
        VddclvrxcompenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable the ton and toff signals no deglitch output."]
    #[inline(always)]
    pub fn tontoffnodeglitch(&self) -> TontoffnodeglitchR {
        TontoffnodeglitchR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable the VDDC rail."]
    #[inline(always)]
    #[must_use]
    pub fn vddcrxcompen(&mut self) -> VddcrxcompenW<Simobuck0Spec> {
        VddcrxcompenW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable the VDDS rail."]
    #[inline(always)]
    #[must_use]
    pub fn vddfrxcompen(&mut self) -> VddfrxcompenW<Simobuck0Spec> {
        VddfrxcompenW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable the VDDS rail."]
    #[inline(always)]
    #[must_use]
    pub fn vddsrxcompen(&mut self) -> VddsrxcompenW<Simobuck0Spec> {
        VddsrxcompenW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable the VDDC LV rail."]
    #[inline(always)]
    #[must_use]
    pub fn vddclvrxcompen(&mut self) -> VddclvrxcompenW<Simobuck0Spec> {
        VddclvrxcompenW::new(self, 3)
    }
    #[doc = "Bit 4 - Enable the ton and toff signals no deglitch output."]
    #[inline(always)]
    #[must_use]
    pub fn tontoffnodeglitch(&mut self) -> TontoffnodeglitchW<Simobuck0Spec> {
        TontoffnodeglitchW::new(self, 4)
    }
}
#[doc = "This WRITE_ONLY register controls various buck parameters. It will read back as 0x00000000.\n\nYou can [`read`](crate::Reg::read) this register and get [`simobuck0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`simobuck0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Simobuck0Spec;
impl crate::RegisterSpec for Simobuck0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`simobuck0::R`](R) reader structure"]
impl crate::Readable for Simobuck0Spec {}
#[doc = "`write(|w| ..)` method takes [`simobuck0::W`](W) writer structure"]
impl crate::Writable for Simobuck0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SIMOBUCK0 to value 0"]
impl crate::Resettable for Simobuck0Spec {
    const RESET_VALUE: u32 = 0;
}
