#[doc = "Register `CFG` reader"]
pub type R = crate::R<CfgSpec>;
#[doc = "Register `CFG` writer"]
pub type W = crate::W<CfgSpec>;
#[doc = "Field `BODHREN` reader - Brown out high (2.1v) reset enable. Note - Enabling this bit for Apollo4, which operates at 1.8v/1.9v, will cause a continual reset loop."]
pub type BodhrenR = crate::BitReader;
#[doc = "Field `BODHREN` writer - Brown out high (2.1v) reset enable. Note - Enabling this bit for Apollo4, which operates at 1.8v/1.9v, will cause a continual reset loop."]
pub type BodhrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDREN` reader - Watchdog Timer Reset Enable. NOTE: The WDT module must also be configured for WDT reset. This includes enabling the RESEN bit in WDTCFG register in Watch dog timer block."]
pub type WdrenR = crate::BitReader;
#[doc = "Field `WDREN` writer - Watchdog Timer Reset Enable. NOTE: The WDT module must also be configured for WDT reset. This includes enabling the RESEN bit in WDTCFG register in Watch dog timer block."]
pub type WdrenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Brown out high (2.1v) reset enable. Note - Enabling this bit for Apollo4, which operates at 1.8v/1.9v, will cause a continual reset loop."]
    #[inline(always)]
    pub fn bodhren(&self) -> BodhrenR {
        BodhrenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Watchdog Timer Reset Enable. NOTE: The WDT module must also be configured for WDT reset. This includes enabling the RESEN bit in WDTCFG register in Watch dog timer block."]
    #[inline(always)]
    pub fn wdren(&self) -> WdrenR {
        WdrenR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Brown out high (2.1v) reset enable. Note - Enabling this bit for Apollo4, which operates at 1.8v/1.9v, will cause a continual reset loop."]
    #[inline(always)]
    #[must_use]
    pub fn bodhren(&mut self) -> BodhrenW<CfgSpec> {
        BodhrenW::new(self, 0)
    }
    #[doc = "Bit 1 - Watchdog Timer Reset Enable. NOTE: The WDT module must also be configured for WDT reset. This includes enabling the RESEN bit in WDTCFG register in Watch dog timer block."]
    #[inline(always)]
    #[must_use]
    pub fn wdren(&mut self) -> WdrenW<CfgSpec> {
        WdrenW::new(self, 1)
    }
}
#[doc = "Reset configuration register. This controls the reset enables for brownout condition, choice of brownout method and for the expiration of the watch dog timer.\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgSpec;
impl crate::RegisterSpec for CfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg::R`](R) reader structure"]
impl crate::Readable for CfgSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg::W`](W) writer structure"]
impl crate::Writable for CfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CfgSpec {
    const RESET_VALUE: u32 = 0;
}
