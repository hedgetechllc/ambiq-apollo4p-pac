#[doc = "Register `SDIOCTRL` reader"]
pub type R = crate::R<SdioctrlSpec>;
#[doc = "Register `SDIOCTRL` writer"]
pub type W = crate::W<SdioctrlSpec>;
#[doc = "Field `SDIOSYSCLKEN` reader - SDIO system clock enable."]
pub type SdiosysclkenR = crate::BitReader;
#[doc = "Field `SDIOSYSCLKEN` writer - SDIO system clock enable."]
pub type SdiosysclkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIOXINCLKEN` reader - SDIO serial clock source enable."]
pub type SdioxinclkenR = crate::BitReader;
#[doc = "Field `SDIOXINCLKEN` writer - SDIO serial clock source enable."]
pub type SdioxinclkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIOITAPCHGWIN` reader - This is used to gate the output of the Tap Delay lines so as to avoid glithches being propagated into the Core. This signal should be asserted few clocks before the itapdlysel changes and should be asserted for few clocks after."]
pub type SdioitapchgwinR = crate::BitReader;
#[doc = "Field `SDIOITAPCHGWIN` writer - This is used to gate the output of the Tap Delay lines so as to avoid glithches being propagated into the Core. This signal should be asserted few clocks before the itapdlysel changes and should be asserted for few clocks after."]
pub type SdioitapchgwinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIOITAPDLYENA` reader - Used to enable selective Tap delay line on the Looped back SD Clock (rxclk_in). This signal along with the itapdlysel\\[4:0\\]
selects the the amount of delay to be inserted on the line. When Tuning is enabled (for SDR104 and optionally for SDR50), this signal is ignored and internalcontrols are used instead. This should not be asserted when operating in DS mode."]
pub type SdioitapdlyenaR = crate::BitReader;
#[doc = "Field `SDIOITAPDLYENA` writer - Used to enable selective Tap delay line on the Looped back SD Clock (rxclk_in). This signal along with the itapdlysel\\[4:0\\]
selects the the amount of delay to be inserted on the line. When Tuning is enabled (for SDR104 and optionally for SDR50), this signal is ignored and internalcontrols are used instead. This should not be asserted when operating in DS mode."]
pub type SdioitapdlyenaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIOITAPDLYSEL` reader - Selects one of the 32 Taps on the rxclk_in line. This is effective only when itapdlyena is asserted and Tuning is not enabled."]
pub type SdioitapdlyselR = crate::FieldReader;
#[doc = "Field `SDIOITAPDLYSEL` writer - Selects one of the 32 Taps on the rxclk_in line. This is effective only when itapdlyena is asserted and Tuning is not enabled."]
pub type SdioitapdlyselW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SDIOOTAPDLYENA` reader - Used to enable the selective Tap delay on the sdcard_clk so as to generate the delayed sdcard_clk. This is used to latch the CMD/DAT outputs to generate delay on them w.r.t CLK going out. This signal along with otapdlysel\\[3:0\\]
selects the amount of delay to be inserted on the Clock line. This signal should not be asserted when operating in DS mode."]
pub type SdiootapdlyenaR = crate::BitReader;
#[doc = "Field `SDIOOTAPDLYENA` writer - Used to enable the selective Tap delay on the sdcard_clk so as to generate the delayed sdcard_clk. This is used to latch the CMD/DAT outputs to generate delay on them w.r.t CLK going out. This signal along with otapdlysel\\[3:0\\]
selects the amount of delay to be inserted on the Clock line. This signal should not be asserted when operating in DS mode."]
pub type SdiootapdlyenaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIOOTAPDLYSEL` reader - Selects one of the 16 Taps on the sdcard_clk. This is effective only when otapdlyena is asserted."]
pub type SdiootapdlyselR = crate::FieldReader;
#[doc = "Field `SDIOOTAPDLYSEL` writer - Selects one of the 16 Taps on the sdcard_clk. This is effective only when otapdlyena is asserted."]
pub type SdiootapdlyselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SDIOASYNCWKUPENA` reader - SDIO asynchronous wakeup mode. 0: Synchronous wakeup mode, 1: Asynchronous wakeup mode"]
pub type SdioasyncwkupenaR = crate::BitReader;
#[doc = "Field `SDIOASYNCWKUPENA` writer - SDIO asynchronous wakeup mode. 0: Synchronous wakeup mode, 1: Asynchronous wakeup mode"]
pub type SdioasyncwkupenaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIOXINCLKSEL` reader - Select clock source for SDIO xin_clk."]
pub type SdioxinclkselR = crate::FieldReader;
#[doc = "Field `SDIOXINCLKSEL` writer - Select clock source for SDIO xin_clk."]
pub type SdioxinclkselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SDIOCMDOPENDRAINEN` reader - SDIO CMD line configured as open-drian. 0: Push-pull mode, 1: Open-drain mode"]
pub type SdiocmdopendrainenR = crate::BitReader;
#[doc = "Field `SDIOCMDOPENDRAINEN` writer - SDIO CMD line configured as open-drian. 0: Push-pull mode, 1: Open-drain mode"]
pub type SdiocmdopendrainenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIODATOPENDRAINEN` reader - SDIO DAT line configured as open-drian. 0: Push-pull mode, 1: Open-drain mode"]
pub type SdiodatopendrainenR = crate::BitReader;
#[doc = "Field `SDIODATOPENDRAINEN` writer - SDIO DAT line configured as open-drian. 0: Push-pull mode, 1: Open-drain mode"]
pub type SdiodatopendrainenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SDIO system clock enable."]
    #[inline(always)]
    pub fn sdiosysclken(&self) -> SdiosysclkenR {
        SdiosysclkenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SDIO serial clock source enable."]
    #[inline(always)]
    pub fn sdioxinclken(&self) -> SdioxinclkenR {
        SdioxinclkenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This is used to gate the output of the Tap Delay lines so as to avoid glithches being propagated into the Core. This signal should be asserted few clocks before the itapdlysel changes and should be asserted for few clocks after."]
    #[inline(always)]
    pub fn sdioitapchgwin(&self) -> SdioitapchgwinR {
        SdioitapchgwinR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Used to enable selective Tap delay line on the Looped back SD Clock (rxclk_in). This signal along with the itapdlysel\\[4:0\\]
selects the the amount of delay to be inserted on the line. When Tuning is enabled (for SDR104 and optionally for SDR50), this signal is ignored and internalcontrols are used instead. This should not be asserted when operating in DS mode."]
    #[inline(always)]
    pub fn sdioitapdlyena(&self) -> SdioitapdlyenaR {
        SdioitapdlyenaR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:8 - Selects one of the 32 Taps on the rxclk_in line. This is effective only when itapdlyena is asserted and Tuning is not enabled."]
    #[inline(always)]
    pub fn sdioitapdlysel(&self) -> SdioitapdlyselR {
        SdioitapdlyselR::new(((self.bits >> 4) & 0x1f) as u8)
    }
    #[doc = "Bit 9 - Used to enable the selective Tap delay on the sdcard_clk so as to generate the delayed sdcard_clk. This is used to latch the CMD/DAT outputs to generate delay on them w.r.t CLK going out. This signal along with otapdlysel\\[3:0\\]
selects the amount of delay to be inserted on the Clock line. This signal should not be asserted when operating in DS mode."]
    #[inline(always)]
    pub fn sdiootapdlyena(&self) -> SdiootapdlyenaR {
        SdiootapdlyenaR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:13 - Selects one of the 16 Taps on the sdcard_clk. This is effective only when otapdlyena is asserted."]
    #[inline(always)]
    pub fn sdiootapdlysel(&self) -> SdiootapdlyselR {
        SdiootapdlyselR::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 14 - SDIO asynchronous wakeup mode. 0: Synchronous wakeup mode, 1: Asynchronous wakeup mode"]
    #[inline(always)]
    pub fn sdioasyncwkupena(&self) -> SdioasyncwkupenaR {
        SdioasyncwkupenaR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:16 - Select clock source for SDIO xin_clk."]
    #[inline(always)]
    pub fn sdioxinclksel(&self) -> SdioxinclkselR {
        SdioxinclkselR::new(((self.bits >> 15) & 3) as u8)
    }
    #[doc = "Bit 17 - SDIO CMD line configured as open-drian. 0: Push-pull mode, 1: Open-drain mode"]
    #[inline(always)]
    pub fn sdiocmdopendrainen(&self) -> SdiocmdopendrainenR {
        SdiocmdopendrainenR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SDIO DAT line configured as open-drian. 0: Push-pull mode, 1: Open-drain mode"]
    #[inline(always)]
    pub fn sdiodatopendrainen(&self) -> SdiodatopendrainenR {
        SdiodatopendrainenR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SDIO system clock enable."]
    #[inline(always)]
    #[must_use]
    pub fn sdiosysclken(&mut self) -> SdiosysclkenW<SdioctrlSpec> {
        SdiosysclkenW::new(self, 0)
    }
    #[doc = "Bit 1 - SDIO serial clock source enable."]
    #[inline(always)]
    #[must_use]
    pub fn sdioxinclken(&mut self) -> SdioxinclkenW<SdioctrlSpec> {
        SdioxinclkenW::new(self, 1)
    }
    #[doc = "Bit 2 - This is used to gate the output of the Tap Delay lines so as to avoid glithches being propagated into the Core. This signal should be asserted few clocks before the itapdlysel changes and should be asserted for few clocks after."]
    #[inline(always)]
    #[must_use]
    pub fn sdioitapchgwin(&mut self) -> SdioitapchgwinW<SdioctrlSpec> {
        SdioitapchgwinW::new(self, 2)
    }
    #[doc = "Bit 3 - Used to enable selective Tap delay line on the Looped back SD Clock (rxclk_in). This signal along with the itapdlysel\\[4:0\\]
selects the the amount of delay to be inserted on the line. When Tuning is enabled (for SDR104 and optionally for SDR50), this signal is ignored and internalcontrols are used instead. This should not be asserted when operating in DS mode."]
    #[inline(always)]
    #[must_use]
    pub fn sdioitapdlyena(&mut self) -> SdioitapdlyenaW<SdioctrlSpec> {
        SdioitapdlyenaW::new(self, 3)
    }
    #[doc = "Bits 4:8 - Selects one of the 32 Taps on the rxclk_in line. This is effective only when itapdlyena is asserted and Tuning is not enabled."]
    #[inline(always)]
    #[must_use]
    pub fn sdioitapdlysel(&mut self) -> SdioitapdlyselW<SdioctrlSpec> {
        SdioitapdlyselW::new(self, 4)
    }
    #[doc = "Bit 9 - Used to enable the selective Tap delay on the sdcard_clk so as to generate the delayed sdcard_clk. This is used to latch the CMD/DAT outputs to generate delay on them w.r.t CLK going out. This signal along with otapdlysel\\[3:0\\]
selects the amount of delay to be inserted on the Clock line. This signal should not be asserted when operating in DS mode."]
    #[inline(always)]
    #[must_use]
    pub fn sdiootapdlyena(&mut self) -> SdiootapdlyenaW<SdioctrlSpec> {
        SdiootapdlyenaW::new(self, 9)
    }
    #[doc = "Bits 10:13 - Selects one of the 16 Taps on the sdcard_clk. This is effective only when otapdlyena is asserted."]
    #[inline(always)]
    #[must_use]
    pub fn sdiootapdlysel(&mut self) -> SdiootapdlyselW<SdioctrlSpec> {
        SdiootapdlyselW::new(self, 10)
    }
    #[doc = "Bit 14 - SDIO asynchronous wakeup mode. 0: Synchronous wakeup mode, 1: Asynchronous wakeup mode"]
    #[inline(always)]
    #[must_use]
    pub fn sdioasyncwkupena(&mut self) -> SdioasyncwkupenaW<SdioctrlSpec> {
        SdioasyncwkupenaW::new(self, 14)
    }
    #[doc = "Bits 15:16 - Select clock source for SDIO xin_clk."]
    #[inline(always)]
    #[must_use]
    pub fn sdioxinclksel(&mut self) -> SdioxinclkselW<SdioctrlSpec> {
        SdioxinclkselW::new(self, 15)
    }
    #[doc = "Bit 17 - SDIO CMD line configured as open-drian. 0: Push-pull mode, 1: Open-drain mode"]
    #[inline(always)]
    #[must_use]
    pub fn sdiocmdopendrainen(&mut self) -> SdiocmdopendrainenW<SdioctrlSpec> {
        SdiocmdopendrainenW::new(self, 17)
    }
    #[doc = "Bit 18 - SDIO DAT line configured as open-drian. 0: Push-pull mode, 1: Open-drain mode"]
    #[inline(always)]
    #[must_use]
    pub fn sdiodatopendrainen(&mut self) -> SdiodatopendrainenW<SdioctrlSpec> {
        SdiodatopendrainenW::new(self, 18)
    }
}
#[doc = "SDIO/eMMC Control\n\nYou can [`read`](crate::Reg::read) this register and get [`sdioctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdioctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdioctrlSpec;
impl crate::RegisterSpec for SdioctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdioctrl::R`](R) reader structure"]
impl crate::Readable for SdioctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`sdioctrl::W`](W) writer structure"]
impl crate::Writable for SdioctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDIOCTRL to value 0x1080"]
impl crate::Resettable for SdioctrlSpec {
    const RESET_VALUE: u32 = 0x1080;
}
