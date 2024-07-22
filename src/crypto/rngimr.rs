#[doc = "Register `RNGIMR` reader"]
pub type R = crate::R<RngimrSpec>;
#[doc = "Register `RNGIMR` writer"]
pub type W = crate::W<RngimrSpec>;
#[doc = "Field `EHRVALIDINTMASK` reader - 0x1 - masks the EHR interrupt. No interrupt is generated."]
pub type EhrvalidintmaskR = crate::BitReader;
#[doc = "Field `EHRVALIDINTMASK` writer - 0x1 - masks the EHR interrupt. No interrupt is generated."]
pub type EhrvalidintmaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTOCORRERRINTMASK` reader - 0x1 - masks the autocorrelation interrupt. No interrupt is generated."]
pub type AutocorrerrintmaskR = crate::BitReader;
#[doc = "Field `AUTOCORRERRINTMASK` writer - 0x1 - masks the autocorrelation interrupt. No interrupt is generated."]
pub type AutocorrerrintmaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRNGTERRINTMASK` reader - 0x1 - masks the CRNGT error interrupt. No interrupt is generated."]
pub type CrngterrintmaskR = crate::BitReader;
#[doc = "Field `CRNGTERRINTMASK` writer - 0x1 - masks the CRNGT error interrupt. No interrupt is generated."]
pub type CrngterrintmaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VNERRINTMASK` reader - 0x1 - masks the Von-Neumann error interrupt. No interrupt is generated."]
pub type VnerrintmaskR = crate::BitReader;
#[doc = "Field `VNERRINTMASK` writer - 0x1 - masks the Von-Neumann error interrupt. No interrupt is generated."]
pub type VnerrintmaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WATCHDOGINTMASK` reader - 0x1 - masks the watchdog interrupt. No interrupt is generated."]
pub type WatchdogintmaskR = crate::BitReader;
#[doc = "Field `WATCHDOGINTMASK` writer - 0x1 - masks the watchdog interrupt. No interrupt is generated."]
pub type WatchdogintmaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RNGDMADONEINT` reader - 0x1 - masks the RNG DMA completion interrupt. No interrupt is generated."]
pub type RngdmadoneintR = crate::BitReader;
#[doc = "Field `RNGDMADONEINT` writer - 0x1 - masks the RNG DMA completion interrupt. No interrupt is generated."]
pub type RngdmadoneintW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0x1 - masks the EHR interrupt. No interrupt is generated."]
    #[inline(always)]
    pub fn ehrvalidintmask(&self) -> EhrvalidintmaskR {
        EhrvalidintmaskR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 0x1 - masks the autocorrelation interrupt. No interrupt is generated."]
    #[inline(always)]
    pub fn autocorrerrintmask(&self) -> AutocorrerrintmaskR {
        AutocorrerrintmaskR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 0x1 - masks the CRNGT error interrupt. No interrupt is generated."]
    #[inline(always)]
    pub fn crngterrintmask(&self) -> CrngterrintmaskR {
        CrngterrintmaskR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 0x1 - masks the Von-Neumann error interrupt. No interrupt is generated."]
    #[inline(always)]
    pub fn vnerrintmask(&self) -> VnerrintmaskR {
        VnerrintmaskR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 0x1 - masks the watchdog interrupt. No interrupt is generated."]
    #[inline(always)]
    pub fn watchdogintmask(&self) -> WatchdogintmaskR {
        WatchdogintmaskR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 0x1 - masks the RNG DMA completion interrupt. No interrupt is generated."]
    #[inline(always)]
    pub fn rngdmadoneint(&self) -> RngdmadoneintR {
        RngdmadoneintR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0x1 - masks the EHR interrupt. No interrupt is generated."]
    #[inline(always)]
    #[must_use]
    pub fn ehrvalidintmask(&mut self) -> EhrvalidintmaskW<RngimrSpec> {
        EhrvalidintmaskW::new(self, 0)
    }
    #[doc = "Bit 1 - 0x1 - masks the autocorrelation interrupt. No interrupt is generated."]
    #[inline(always)]
    #[must_use]
    pub fn autocorrerrintmask(&mut self) -> AutocorrerrintmaskW<RngimrSpec> {
        AutocorrerrintmaskW::new(self, 1)
    }
    #[doc = "Bit 2 - 0x1 - masks the CRNGT error interrupt. No interrupt is generated."]
    #[inline(always)]
    #[must_use]
    pub fn crngterrintmask(&mut self) -> CrngterrintmaskW<RngimrSpec> {
        CrngterrintmaskW::new(self, 2)
    }
    #[doc = "Bit 3 - 0x1 - masks the Von-Neumann error interrupt. No interrupt is generated."]
    #[inline(always)]
    #[must_use]
    pub fn vnerrintmask(&mut self) -> VnerrintmaskW<RngimrSpec> {
        VnerrintmaskW::new(self, 3)
    }
    #[doc = "Bit 4 - 0x1 - masks the watchdog interrupt. No interrupt is generated."]
    #[inline(always)]
    #[must_use]
    pub fn watchdogintmask(&mut self) -> WatchdogintmaskW<RngimrSpec> {
        WatchdogintmaskW::new(self, 4)
    }
    #[doc = "Bit 5 - 0x1 - masks the RNG DMA completion interrupt. No interrupt is generated."]
    #[inline(always)]
    #[must_use]
    pub fn rngdmadoneint(&mut self) -> RngdmadoneintW<RngimrSpec> {
        RngdmadoneintW::new(self, 5)
    }
}
#[doc = "Interrupt masking register. Consists of {prng_imr trng_imr} bit\\[31-16\\]
- PRNG_IMR bit\\[15-0\\]
- TRNG_IMR(Ws - PRNG bit exists only if PRNG_EXISTS flag)\n\nYou can [`read`](crate::Reg::read) this register and get [`rngimr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rngimr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RngimrSpec;
impl crate::RegisterSpec for RngimrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rngimr::R`](R) reader structure"]
impl crate::Readable for RngimrSpec {}
#[doc = "`write(|w| ..)` method takes [`rngimr::W`](W) writer structure"]
impl crate::Writable for RngimrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RNGIMR to value 0x3f"]
impl crate::Resettable for RngimrSpec {
    const RESET_VALUE: u32 = 0x3f;
}
