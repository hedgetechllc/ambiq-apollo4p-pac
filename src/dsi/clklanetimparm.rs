#[doc = "Register `CLKLANETIMPARM` reader"]
pub type R = crate::R<ClklanetimparmSpec>;
#[doc = "Register `CLKLANETIMPARM` writer"]
pub type W = crate::W<ClklanetimparmSpec>;
#[doc = "Field `HSPREP` reader - This field provides the timing requirement in byte corresponds to the TCLK-PREP parameter specified in the DPHY specificatio"]
pub type HsprepR = crate::FieldReader;
#[doc = "Field `HSPREP` writer - This field provides the timing requirement in byte corresponds to the TCLK-PREP parameter specified in the DPHY specificatio"]
pub type HsprepW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HSZERO` reader - This field provides the timing requirement in byte clocks for the high speed drive zero time; this corresponds to the TCLK-ZERO parameter specified in the DPHY specification"]
pub type HszeroR = crate::FieldReader;
#[doc = "Field `HSZERO` writer - This field provides the timing requirement in byte clocks for the high speed drive zero time; this corresponds to the TCLK-ZERO parameter specified in the DPHY specification"]
pub type HszeroW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HSTRAIL` reader - This field provides the timing requirement in byte clocks for the high speed trail time; This corresponds to the TCLK-TRAIL parameter specified in the DPHY specification"]
pub type HstrailR = crate::FieldReader;
#[doc = "Field `HSTRAIL` writer - This field provides the timing requirement in byte clocks for the high speed trail time; This corresponds to the TCLK-TRAIL parameter specified in the DPHY specification"]
pub type HstrailW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HSEXIT` reader - This field provides the timing requirement in byte clocks for the high speed exit time; This corresponds to the THS-EXIT parameter specified in the DPHY specification."]
pub type HsexitR = crate::FieldReader;
#[doc = "Field `HSEXIT` writer - This field provides the timing requirement in byte clocks for the high speed exit time; This corresponds to the THS-EXIT parameter specified in the DPHY specification."]
pub type HsexitW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - This field provides the timing requirement in byte corresponds to the TCLK-PREP parameter specified in the DPHY specificatio"]
    #[inline(always)]
    pub fn hsprep(&self) -> HsprepR {
        HsprepR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - This field provides the timing requirement in byte clocks for the high speed drive zero time; this corresponds to the TCLK-ZERO parameter specified in the DPHY specification"]
    #[inline(always)]
    pub fn hszero(&self) -> HszeroR {
        HszeroR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - This field provides the timing requirement in byte clocks for the high speed trail time; This corresponds to the TCLK-TRAIL parameter specified in the DPHY specification"]
    #[inline(always)]
    pub fn hstrail(&self) -> HstrailR {
        HstrailR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - This field provides the timing requirement in byte clocks for the high speed exit time; This corresponds to the THS-EXIT parameter specified in the DPHY specification."]
    #[inline(always)]
    pub fn hsexit(&self) -> HsexitR {
        HsexitR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - This field provides the timing requirement in byte corresponds to the TCLK-PREP parameter specified in the DPHY specificatio"]
    #[inline(always)]
    #[must_use]
    pub fn hsprep(&mut self) -> HsprepW<ClklanetimparmSpec> {
        HsprepW::new(self, 0)
    }
    #[doc = "Bits 8:15 - This field provides the timing requirement in byte clocks for the high speed drive zero time; this corresponds to the TCLK-ZERO parameter specified in the DPHY specification"]
    #[inline(always)]
    #[must_use]
    pub fn hszero(&mut self) -> HszeroW<ClklanetimparmSpec> {
        HszeroW::new(self, 8)
    }
    #[doc = "Bits 16:23 - This field provides the timing requirement in byte clocks for the high speed trail time; This corresponds to the TCLK-TRAIL parameter specified in the DPHY specification"]
    #[inline(always)]
    #[must_use]
    pub fn hstrail(&mut self) -> HstrailW<ClklanetimparmSpec> {
        HstrailW::new(self, 16)
    }
    #[doc = "Bits 24:31 - This field provides the timing requirement in byte clocks for the high speed exit time; This corresponds to the THS-EXIT parameter specified in the DPHY specification."]
    #[inline(always)]
    #[must_use]
    pub fn hsexit(&mut self) -> HsexitW<ClklanetimparmSpec> {
        HsexitW::new(self, 24)
    }
}
#[doc = "This field provides the timing requirement in byte clocks\n\nYou can [`read`](crate::Reg::read) this register and get [`clklanetimparm::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clklanetimparm::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClklanetimparmSpec;
impl crate::RegisterSpec for ClklanetimparmSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clklanetimparm::R`](R) reader structure"]
impl crate::Readable for ClklanetimparmSpec {}
#[doc = "`write(|w| ..)` method takes [`clklanetimparm::W`](W) writer structure"]
impl crate::Writable for ClklanetimparmSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKLANETIMPARM to value 0x0b06_1a04"]
impl crate::Resettable for ClklanetimparmSpec {
    const RESET_VALUE: u32 = 0x0b06_1a04;
}
