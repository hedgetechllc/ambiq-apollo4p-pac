#[doc = "Register `STAT` reader"]
pub type R = crate::R<StatSpec>;
#[doc = "Register `STAT` writer"]
pub type W = crate::W<StatSpec>;
#[doc = "Field `EXRSTAT` reader - Reset was initiated by an External Reset."]
pub type ExrstatR = crate::BitReader;
#[doc = "Field `EXRSTAT` writer - Reset was initiated by an External Reset."]
pub type ExrstatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORSTAT` reader - Reset was initiated by a Power-On Reset."]
pub type PorstatR = crate::BitReader;
#[doc = "Field `PORSTAT` writer - Reset was initiated by a Power-On Reset."]
pub type PorstatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BORSTAT` reader - Reset was initiated by a Brown-Out Reset."]
pub type BorstatR = crate::BitReader;
#[doc = "Field `BORSTAT` writer - Reset was initiated by a Brown-Out Reset."]
pub type BorstatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWRSTAT` reader - Reset was a initiated by SW POR or AIRCR Reset."]
pub type SwrstatR = crate::BitReader;
#[doc = "Field `SWRSTAT` writer - Reset was a initiated by SW POR or AIRCR Reset."]
pub type SwrstatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POIRSTAT` reader - Reset was a initiated by Software POI Reset."]
pub type PoirstatR = crate::BitReader;
#[doc = "Field `POIRSTAT` writer - Reset was a initiated by Software POI Reset."]
pub type PoirstatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBGRSTAT` reader - Reset was a initiated by Debugger Reset."]
pub type DbgrstatR = crate::BitReader;
#[doc = "Field `DBGRSTAT` writer - Reset was a initiated by Debugger Reset."]
pub type DbgrstatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDRSTAT` reader - Reset was initiated by a Watchdog Timer Reset."]
pub type WdrstatR = crate::BitReader;
#[doc = "Field `WDRSTAT` writer - Reset was initiated by a Watchdog Timer Reset."]
pub type WdrstatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOUSTAT` reader - An Unregulated Supply Brownout Event occured."]
pub type BoustatR = crate::BitReader;
#[doc = "Field `BOUSTAT` writer - An Unregulated Supply Brownout Event occured."]
pub type BoustatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOCSTAT` reader - VDDC Analog Brownout Event occured."]
pub type BocstatR = crate::BitReader;
#[doc = "Field `BOCSTAT` writer - VDDC Analog Brownout Event occured."]
pub type BocstatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOFSTAT` reader - VDDF Analog Brownout Event occured."]
pub type BofstatR = crate::BitReader;
#[doc = "Field `BOFSTAT` writer - VDDF Analog Brownout Event occured."]
pub type BofstatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOSSTAT` reader - VDDS Analog Brownout Event occured."]
pub type BosstatR = crate::BitReader;
#[doc = "Field `BOSSTAT` writer - VDDS Analog Brownout Event occured."]
pub type BosstatW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Reset was initiated by an External Reset."]
    #[inline(always)]
    pub fn exrstat(&self) -> ExrstatR {
        ExrstatR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reset was initiated by a Power-On Reset."]
    #[inline(always)]
    pub fn porstat(&self) -> PorstatR {
        PorstatR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reset was initiated by a Brown-Out Reset."]
    #[inline(always)]
    pub fn borstat(&self) -> BorstatR {
        BorstatR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reset was a initiated by SW POR or AIRCR Reset."]
    #[inline(always)]
    pub fn swrstat(&self) -> SwrstatR {
        SwrstatR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Reset was a initiated by Software POI Reset."]
    #[inline(always)]
    pub fn poirstat(&self) -> PoirstatR {
        PoirstatR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Reset was a initiated by Debugger Reset."]
    #[inline(always)]
    pub fn dbgrstat(&self) -> DbgrstatR {
        DbgrstatR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Reset was initiated by a Watchdog Timer Reset."]
    #[inline(always)]
    pub fn wdrstat(&self) -> WdrstatR {
        WdrstatR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - An Unregulated Supply Brownout Event occured."]
    #[inline(always)]
    pub fn boustat(&self) -> BoustatR {
        BoustatR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - VDDC Analog Brownout Event occured."]
    #[inline(always)]
    pub fn bocstat(&self) -> BocstatR {
        BocstatR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - VDDF Analog Brownout Event occured."]
    #[inline(always)]
    pub fn bofstat(&self) -> BofstatR {
        BofstatR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - VDDS Analog Brownout Event occured."]
    #[inline(always)]
    pub fn bosstat(&self) -> BosstatR {
        BosstatR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reset was initiated by an External Reset."]
    #[inline(always)]
    #[must_use]
    pub fn exrstat(&mut self) -> ExrstatW<StatSpec> {
        ExrstatW::new(self, 0)
    }
    #[doc = "Bit 1 - Reset was initiated by a Power-On Reset."]
    #[inline(always)]
    #[must_use]
    pub fn porstat(&mut self) -> PorstatW<StatSpec> {
        PorstatW::new(self, 1)
    }
    #[doc = "Bit 2 - Reset was initiated by a Brown-Out Reset."]
    #[inline(always)]
    #[must_use]
    pub fn borstat(&mut self) -> BorstatW<StatSpec> {
        BorstatW::new(self, 2)
    }
    #[doc = "Bit 3 - Reset was a initiated by SW POR or AIRCR Reset."]
    #[inline(always)]
    #[must_use]
    pub fn swrstat(&mut self) -> SwrstatW<StatSpec> {
        SwrstatW::new(self, 3)
    }
    #[doc = "Bit 4 - Reset was a initiated by Software POI Reset."]
    #[inline(always)]
    #[must_use]
    pub fn poirstat(&mut self) -> PoirstatW<StatSpec> {
        PoirstatW::new(self, 4)
    }
    #[doc = "Bit 5 - Reset was a initiated by Debugger Reset."]
    #[inline(always)]
    #[must_use]
    pub fn dbgrstat(&mut self) -> DbgrstatW<StatSpec> {
        DbgrstatW::new(self, 5)
    }
    #[doc = "Bit 6 - Reset was initiated by a Watchdog Timer Reset."]
    #[inline(always)]
    #[must_use]
    pub fn wdrstat(&mut self) -> WdrstatW<StatSpec> {
        WdrstatW::new(self, 6)
    }
    #[doc = "Bit 7 - An Unregulated Supply Brownout Event occured."]
    #[inline(always)]
    #[must_use]
    pub fn boustat(&mut self) -> BoustatW<StatSpec> {
        BoustatW::new(self, 7)
    }
    #[doc = "Bit 8 - VDDC Analog Brownout Event occured."]
    #[inline(always)]
    #[must_use]
    pub fn bocstat(&mut self) -> BocstatW<StatSpec> {
        BocstatW::new(self, 8)
    }
    #[doc = "Bit 9 - VDDF Analog Brownout Event occured."]
    #[inline(always)]
    #[must_use]
    pub fn bofstat(&mut self) -> BofstatW<StatSpec> {
        BofstatW::new(self, 9)
    }
    #[doc = "Bit 10 - VDDS Analog Brownout Event occured."]
    #[inline(always)]
    #[must_use]
    pub fn bosstat(&mut self) -> BosstatW<StatSpec> {
        BosstatW::new(self, 10)
    }
}
#[doc = "This register contains the status for brownout events and the causes for resets.\n NOTE 1: All bits in this register, including reserved bits, are writable. Therefore care should be taken not to write this register.\n NOTE 2: This register is only reset by POI not by HRESETn. Its contents are intended to survive all reset level except POI and full power cycles.\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatSpec;
impl crate::RegisterSpec for StatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat::R`](R) reader structure"]
impl crate::Readable for StatSpec {}
#[doc = "`write(|w| ..)` method takes [`stat::W`](W) writer structure"]
impl crate::Writable for StatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STAT to value 0"]
impl crate::Resettable for StatSpec {
    const RESET_VALUE: u32 = 0;
}
