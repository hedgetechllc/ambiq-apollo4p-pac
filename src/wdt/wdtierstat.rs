#[doc = "Register `WDTIERSTAT` reader"]
pub type R = crate::R<WdtierstatSpec>;
#[doc = "Register `WDTIERSTAT` writer"]
pub type W = crate::W<WdtierstatSpec>;
#[doc = "Field `WDTINT` reader - Watchdog Timer Interrupt."]
pub type WdtintR = crate::BitReader;
#[doc = "Field `WDTINT` writer - Watchdog Timer Interrupt."]
pub type WdtintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSPRESETINT` reader - Indicates that one of the DSP timers has issued a reset or pmreset to the DSP core. This is used to interrupt the main CPU."]
pub type DspresetintR = crate::BitReader;
#[doc = "Field `DSPRESETINT` writer - Indicates that one of the DSP timers has issued a reset or pmreset to the DSP core. This is used to interrupt the main CPU."]
pub type DspresetintW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Watchdog Timer Interrupt."]
    #[inline(always)]
    pub fn wdtint(&self) -> WdtintR {
        WdtintR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Indicates that one of the DSP timers has issued a reset or pmreset to the DSP core. This is used to interrupt the main CPU."]
    #[inline(always)]
    pub fn dspresetint(&self) -> DspresetintR {
        DspresetintR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Watchdog Timer Interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn wdtint(&mut self) -> WdtintW<WdtierstatSpec> {
        WdtintW::new(self, 0)
    }
    #[doc = "Bit 1 - Indicates that one of the DSP timers has issued a reset or pmreset to the DSP core. This is used to interrupt the main CPU."]
    #[inline(always)]
    #[must_use]
    pub fn dspresetint(&mut self) -> DspresetintW<WdtierstatSpec> {
        DspresetintW::new(self, 1)
    }
}
#[doc = "Read bits from this register to discover the cause of a recent interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`wdtierstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdtierstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdtierstatSpec;
impl crate::RegisterSpec for WdtierstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdtierstat::R`](R) reader structure"]
impl crate::Readable for WdtierstatSpec {}
#[doc = "`write(|w| ..)` method takes [`wdtierstat::W`](W) writer structure"]
impl crate::Writable for WdtierstatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WDTIERSTAT to value 0"]
impl crate::Resettable for WdtierstatSpec {
    const RESET_VALUE: u32 = 0;
}
