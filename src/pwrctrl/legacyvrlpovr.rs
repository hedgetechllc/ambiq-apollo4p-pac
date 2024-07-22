#[doc = "Register `LEGACYVRLPOVR` reader"]
pub type R = crate::R<LegacyvrlpovrSpec>;
#[doc = "Register `LEGACYVRLPOVR` writer"]
pub type W = crate::W<LegacyvrlpovrSpec>;
#[doc = "Field `IGNOREIOS` reader - Ignore IOS"]
pub type IgnoreiosR = crate::BitReader;
#[doc = "Field `IGNOREIOS` writer - Ignore IOS"]
pub type IgnoreiosW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IGNOREHCPA` reader - Ignore HCPA"]
pub type IgnorehcpaR = crate::BitReader;
#[doc = "Field `IGNOREHCPA` writer - Ignore HCPA"]
pub type IgnorehcpaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IGNOREHCPB` reader - Ignore HCPB"]
pub type IgnorehcpbR = crate::BitReader;
#[doc = "Field `IGNOREHCPB` writer - Ignore HCPB"]
pub type IgnorehcpbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IGNOREHCPC` reader - Ignore HCPC"]
pub type IgnorehcpcR = crate::BitReader;
#[doc = "Field `IGNOREHCPC` writer - Ignore HCPC"]
pub type IgnorehcpcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IGNOREHCPD` reader - Ignore HCPD"]
pub type IgnorehcpdR = crate::BitReader;
#[doc = "Field `IGNOREHCPD` writer - Ignore HCPD"]
pub type IgnorehcpdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IGNOREHCPE` reader - Ignore HCPE"]
pub type IgnorehcpeR = crate::BitReader;
#[doc = "Field `IGNOREHCPE` writer - Ignore HCPE"]
pub type IgnorehcpeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IGNOREMSPI` reader - Ignore MSPI"]
pub type IgnoremspiR = crate::BitReader;
#[doc = "Field `IGNOREMSPI` writer - Ignore MSPI"]
pub type IgnoremspiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IGNOREGFX` reader - Ignore GFX"]
pub type IgnoregfxR = crate::BitReader;
#[doc = "Field `IGNOREGFX` writer - Ignore GFX"]
pub type IgnoregfxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IGNOREDISP` reader - Ignore DISP Control"]
pub type IgnoredispR = crate::BitReader;
#[doc = "Field `IGNOREDISP` writer - Ignore DISP Control"]
pub type IgnoredispW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IGNOREDISPPHY` reader - Ignore DISP PHY"]
pub type IgnoredispphyR = crate::BitReader;
#[doc = "Field `IGNOREDISPPHY` writer - Ignore DISP PHY"]
pub type IgnoredispphyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IGNORECRYPTO` reader - Ignore CRYPTO"]
pub type IgnorecryptoR = crate::BitReader;
#[doc = "Field `IGNORECRYPTO` writer - Ignore CRYPTO"]
pub type IgnorecryptoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IGNORESDIO` reader - Ignore SDIO"]
pub type IgnoresdioR = crate::BitReader;
#[doc = "Field `IGNORESDIO` writer - Ignore SDIO"]
pub type IgnoresdioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IGNOREUSB` reader - Ignore USB Control"]
pub type IgnoreusbR = crate::BitReader;
#[doc = "Field `IGNOREUSB` writer - Ignore USB Control"]
pub type IgnoreusbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IGNOREUSBPHY` reader - Ignore USB PHY"]
pub type IgnoreusbphyR = crate::BitReader;
#[doc = "Field `IGNOREUSBPHY` writer - Ignore USB PHY"]
pub type IgnoreusbphyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IGNOREAUD` reader - Ignore AUD"]
pub type IgnoreaudR = crate::BitReader;
#[doc = "Field `IGNOREAUD` writer - Ignore AUD"]
pub type IgnoreaudW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IGNOREDSPA` reader - Ignore DSPA"]
pub type IgnoredspaR = crate::BitReader;
#[doc = "Field `IGNOREDSPA` writer - Ignore DSPA"]
pub type IgnoredspaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IGNOREDSP0H` reader - Ignore DSP0H"]
pub type Ignoredsp0hR = crate::BitReader;
#[doc = "Field `IGNOREDSP0H` writer - Ignore DSP0H"]
pub type Ignoredsp0hW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IGNOREDSP1H` reader - Ignore DSP1H"]
pub type Ignoredsp1hR = crate::BitReader;
#[doc = "Field `IGNOREDSP1H` writer - Ignore DSP1H"]
pub type Ignoredsp1hW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IGNOREDBG` reader - Ignore DBG"]
pub type IgnoredbgR = crate::BitReader;
#[doc = "Field `IGNOREDBG` writer - Ignore DBG"]
pub type IgnoredbgW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Ignore IOS"]
    #[inline(always)]
    pub fn ignoreios(&self) -> IgnoreiosR {
        IgnoreiosR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Ignore HCPA"]
    #[inline(always)]
    pub fn ignorehcpa(&self) -> IgnorehcpaR {
        IgnorehcpaR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Ignore HCPB"]
    #[inline(always)]
    pub fn ignorehcpb(&self) -> IgnorehcpbR {
        IgnorehcpbR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Ignore HCPC"]
    #[inline(always)]
    pub fn ignorehcpc(&self) -> IgnorehcpcR {
        IgnorehcpcR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Ignore HCPD"]
    #[inline(always)]
    pub fn ignorehcpd(&self) -> IgnorehcpdR {
        IgnorehcpdR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Ignore HCPE"]
    #[inline(always)]
    pub fn ignorehcpe(&self) -> IgnorehcpeR {
        IgnorehcpeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Ignore MSPI"]
    #[inline(always)]
    pub fn ignoremspi(&self) -> IgnoremspiR {
        IgnoremspiR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Ignore GFX"]
    #[inline(always)]
    pub fn ignoregfx(&self) -> IgnoregfxR {
        IgnoregfxR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Ignore DISP Control"]
    #[inline(always)]
    pub fn ignoredisp(&self) -> IgnoredispR {
        IgnoredispR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Ignore DISP PHY"]
    #[inline(always)]
    pub fn ignoredispphy(&self) -> IgnoredispphyR {
        IgnoredispphyR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Ignore CRYPTO"]
    #[inline(always)]
    pub fn ignorecrypto(&self) -> IgnorecryptoR {
        IgnorecryptoR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Ignore SDIO"]
    #[inline(always)]
    pub fn ignoresdio(&self) -> IgnoresdioR {
        IgnoresdioR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Ignore USB Control"]
    #[inline(always)]
    pub fn ignoreusb(&self) -> IgnoreusbR {
        IgnoreusbR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Ignore USB PHY"]
    #[inline(always)]
    pub fn ignoreusbphy(&self) -> IgnoreusbphyR {
        IgnoreusbphyR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Ignore AUD"]
    #[inline(always)]
    pub fn ignoreaud(&self) -> IgnoreaudR {
        IgnoreaudR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Ignore DSPA"]
    #[inline(always)]
    pub fn ignoredspa(&self) -> IgnoredspaR {
        IgnoredspaR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Ignore DSP0H"]
    #[inline(always)]
    pub fn ignoredsp0h(&self) -> Ignoredsp0hR {
        Ignoredsp0hR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Ignore DSP1H"]
    #[inline(always)]
    pub fn ignoredsp1h(&self) -> Ignoredsp1hR {
        Ignoredsp1hR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Ignore DBG"]
    #[inline(always)]
    pub fn ignoredbg(&self) -> IgnoredbgR {
        IgnoredbgR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Ignore IOS"]
    #[inline(always)]
    #[must_use]
    pub fn ignoreios(&mut self) -> IgnoreiosW<LegacyvrlpovrSpec> {
        IgnoreiosW::new(self, 0)
    }
    #[doc = "Bit 1 - Ignore HCPA"]
    #[inline(always)]
    #[must_use]
    pub fn ignorehcpa(&mut self) -> IgnorehcpaW<LegacyvrlpovrSpec> {
        IgnorehcpaW::new(self, 1)
    }
    #[doc = "Bit 2 - Ignore HCPB"]
    #[inline(always)]
    #[must_use]
    pub fn ignorehcpb(&mut self) -> IgnorehcpbW<LegacyvrlpovrSpec> {
        IgnorehcpbW::new(self, 2)
    }
    #[doc = "Bit 3 - Ignore HCPC"]
    #[inline(always)]
    #[must_use]
    pub fn ignorehcpc(&mut self) -> IgnorehcpcW<LegacyvrlpovrSpec> {
        IgnorehcpcW::new(self, 3)
    }
    #[doc = "Bit 4 - Ignore HCPD"]
    #[inline(always)]
    #[must_use]
    pub fn ignorehcpd(&mut self) -> IgnorehcpdW<LegacyvrlpovrSpec> {
        IgnorehcpdW::new(self, 4)
    }
    #[doc = "Bit 5 - Ignore HCPE"]
    #[inline(always)]
    #[must_use]
    pub fn ignorehcpe(&mut self) -> IgnorehcpeW<LegacyvrlpovrSpec> {
        IgnorehcpeW::new(self, 5)
    }
    #[doc = "Bit 6 - Ignore MSPI"]
    #[inline(always)]
    #[must_use]
    pub fn ignoremspi(&mut self) -> IgnoremspiW<LegacyvrlpovrSpec> {
        IgnoremspiW::new(self, 6)
    }
    #[doc = "Bit 7 - Ignore GFX"]
    #[inline(always)]
    #[must_use]
    pub fn ignoregfx(&mut self) -> IgnoregfxW<LegacyvrlpovrSpec> {
        IgnoregfxW::new(self, 7)
    }
    #[doc = "Bit 8 - Ignore DISP Control"]
    #[inline(always)]
    #[must_use]
    pub fn ignoredisp(&mut self) -> IgnoredispW<LegacyvrlpovrSpec> {
        IgnoredispW::new(self, 8)
    }
    #[doc = "Bit 9 - Ignore DISP PHY"]
    #[inline(always)]
    #[must_use]
    pub fn ignoredispphy(&mut self) -> IgnoredispphyW<LegacyvrlpovrSpec> {
        IgnoredispphyW::new(self, 9)
    }
    #[doc = "Bit 10 - Ignore CRYPTO"]
    #[inline(always)]
    #[must_use]
    pub fn ignorecrypto(&mut self) -> IgnorecryptoW<LegacyvrlpovrSpec> {
        IgnorecryptoW::new(self, 10)
    }
    #[doc = "Bit 11 - Ignore SDIO"]
    #[inline(always)]
    #[must_use]
    pub fn ignoresdio(&mut self) -> IgnoresdioW<LegacyvrlpovrSpec> {
        IgnoresdioW::new(self, 11)
    }
    #[doc = "Bit 12 - Ignore USB Control"]
    #[inline(always)]
    #[must_use]
    pub fn ignoreusb(&mut self) -> IgnoreusbW<LegacyvrlpovrSpec> {
        IgnoreusbW::new(self, 12)
    }
    #[doc = "Bit 13 - Ignore USB PHY"]
    #[inline(always)]
    #[must_use]
    pub fn ignoreusbphy(&mut self) -> IgnoreusbphyW<LegacyvrlpovrSpec> {
        IgnoreusbphyW::new(self, 13)
    }
    #[doc = "Bit 14 - Ignore AUD"]
    #[inline(always)]
    #[must_use]
    pub fn ignoreaud(&mut self) -> IgnoreaudW<LegacyvrlpovrSpec> {
        IgnoreaudW::new(self, 14)
    }
    #[doc = "Bit 15 - Ignore DSPA"]
    #[inline(always)]
    #[must_use]
    pub fn ignoredspa(&mut self) -> IgnoredspaW<LegacyvrlpovrSpec> {
        IgnoredspaW::new(self, 15)
    }
    #[doc = "Bit 16 - Ignore DSP0H"]
    #[inline(always)]
    #[must_use]
    pub fn ignoredsp0h(&mut self) -> Ignoredsp0hW<LegacyvrlpovrSpec> {
        Ignoredsp0hW::new(self, 16)
    }
    #[doc = "Bit 17 - Ignore DSP1H"]
    #[inline(always)]
    #[must_use]
    pub fn ignoredsp1h(&mut self) -> Ignoredsp1hW<LegacyvrlpovrSpec> {
        Ignoredsp1hW::new(self, 17)
    }
    #[doc = "Bit 18 - Ignore DBG"]
    #[inline(always)]
    #[must_use]
    pub fn ignoredbg(&mut self) -> IgnoredbgW<LegacyvrlpovrSpec> {
        IgnoredbgW::new(self, 18)
    }
}
#[doc = "When an override is set for a power domain, VR logic will ignore that power domain state in making a decision to go into lp state.\n\nYou can [`read`](crate::Reg::read) this register and get [`legacyvrlpovr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`legacyvrlpovr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LegacyvrlpovrSpec;
impl crate::RegisterSpec for LegacyvrlpovrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`legacyvrlpovr::R`](R) reader structure"]
impl crate::Readable for LegacyvrlpovrSpec {}
#[doc = "`write(|w| ..)` method takes [`legacyvrlpovr::W`](W) writer structure"]
impl crate::Writable for LegacyvrlpovrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LEGACYVRLPOVR to value 0"]
impl crate::Resettable for LegacyvrlpovrSpec {
    const RESET_VALUE: u32 = 0;
}
