#[doc = "Register `CLKCFG` reader"]
pub type R = crate::R<ClkcfgSpec>;
#[doc = "Register `CLKCFG` writer"]
pub type W = crate::W<ClkcfgSpec>;
#[doc = "Field `MCLKEN` reader - Enable for the master audio clock."]
pub type MclkenR = crate::BitReader;
#[doc = "Field `MCLKEN` writer - Enable for the master audio clock."]
pub type MclkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSEL` reader - Select the input clock frequency for the MCLK.Whenever changing the clock source here, the MISC_HFRC2FRC bit in the CLKGEN module must first be set. The sequence for changing the clock source regardless of clock selection is to first force HFRC2 on by setting the CLKGEN_MISC_HFRC2FRC bit, select the clock source in this field, clear the CLKGEN_MISC_HFRC2FRC bit only if HFRC2 is NOT selected, and then engage the peripheral.If HFRC2 is the clock source, then shutting the module down cleanly requires switching to HFRC, for example, and then disabling the HFRC2 by clearing the CLKGEN_MISC_HFRC2FRC bit. 0: HFRC2_48MHz 1: HFRC2_24MHz 2: HFRC2_12MHz 3: HFRC2_6MHz 4: HFRC2_3MHz 5: HFRC2_1_5MHz 6: HFRC2_750KHz 7: HFRC2_375KHz 8: HFRC_24MHz 9: HFRC_12MHz 10: HFRC_6MHz 11: HFRC_3MHz 12: HFRC_1_5MHz 13: HFRC_750KHz 14: HFRC_375KHz 15: XTHS_EXTREF_CLK 16: XTHS_16MHz 17: XTHS_8MHz 18: XTHS_4MHz 19: XTHS_2MHz 20: XTHS_1MHz 21: XTHS_500KHz 22: HFRC_48MHz"]
pub type FselR = crate::FieldReader;
#[doc = "Field `FSEL` writer - Select the input clock frequency for the MCLK.Whenever changing the clock source here, the MISC_HFRC2FRC bit in the CLKGEN module must first be set. The sequence for changing the clock source regardless of clock selection is to first force HFRC2 on by setting the CLKGEN_MISC_HFRC2FRC bit, select the clock source in this field, clear the CLKGEN_MISC_HFRC2FRC bit only if HFRC2 is NOT selected, and then engage the peripheral.If HFRC2 is the clock source, then shutting the module down cleanly requires switching to HFRC, for example, and then disabling the HFRC2 by clearing the CLKGEN_MISC_HFRC2FRC bit. 0: HFRC2_48MHz 1: HFRC2_24MHz 2: HFRC2_12MHz 3: HFRC2_6MHz 4: HFRC2_3MHz 5: HFRC2_1_5MHz 6: HFRC2_750KHz 7: HFRC2_375KHz 8: HFRC_24MHz 9: HFRC_12MHz 10: HFRC_6MHz 11: HFRC_3MHz 12: HFRC_1_5MHz 13: HFRC_750KHz 14: HFRC_375KHz 15: XTHS_EXTREF_CLK 16: XTHS_16MHz 17: XTHS_8MHz 18: XTHS_4MHz 19: XTHS_2MHz 20: XTHS_1MHz 21: XTHS_500KHz 22: HFRC_48MHz"]
pub type FselW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `REFCLKEN` reader - FUTURE USE Enable for the reference clock"]
pub type RefclkenR = crate::BitReader;
#[doc = "Field `REFCLKEN` writer - FUTURE USE Enable for the reference clock"]
pub type RefclkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REFFSEL` reader - FUTURE USE Select the input clock frequency for the ref_clk. 0: HFRC_48MHz 1: HFRC_48MHz_GATED 2: XT_24MHz 3: HFRC2_48MHz"]
pub type ReffselR = crate::FieldReader;
#[doc = "Field `REFFSEL` writer - FUTURE USE Select the input clock frequency for the ref_clk. 0: HFRC_48MHz 1: HFRC_48MHz_GATED 2: XT_24MHz 3: HFRC2_48MHz"]
pub type ReffselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DIV3` reader - 0: no change to the clock selected by FSEL 1: frequency divide-by-3 of the clock selected by FSEL"]
pub type Div3R = crate::BitReader;
#[doc = "Field `DIV3` writer - 0: no change to the clock selected by FSEL 1: frequency divide-by-3 of the clock selected by FSEL"]
pub type Div3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable for the master audio clock."]
    #[inline(always)]
    pub fn mclken(&self) -> MclkenR {
        MclkenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:8 - Select the input clock frequency for the MCLK.Whenever changing the clock source here, the MISC_HFRC2FRC bit in the CLKGEN module must first be set. The sequence for changing the clock source regardless of clock selection is to first force HFRC2 on by setting the CLKGEN_MISC_HFRC2FRC bit, select the clock source in this field, clear the CLKGEN_MISC_HFRC2FRC bit only if HFRC2 is NOT selected, and then engage the peripheral.If HFRC2 is the clock source, then shutting the module down cleanly requires switching to HFRC, for example, and then disabling the HFRC2 by clearing the CLKGEN_MISC_HFRC2FRC bit. 0: HFRC2_48MHz 1: HFRC2_24MHz 2: HFRC2_12MHz 3: HFRC2_6MHz 4: HFRC2_3MHz 5: HFRC2_1_5MHz 6: HFRC2_750KHz 7: HFRC2_375KHz 8: HFRC_24MHz 9: HFRC_12MHz 10: HFRC_6MHz 11: HFRC_3MHz 12: HFRC_1_5MHz 13: HFRC_750KHz 14: HFRC_375KHz 15: XTHS_EXTREF_CLK 16: XTHS_16MHz 17: XTHS_8MHz 18: XTHS_4MHz 19: XTHS_2MHz 20: XTHS_1MHz 21: XTHS_500KHz 22: HFRC_48MHz"]
    #[inline(always)]
    pub fn fsel(&self) -> FselR {
        FselR::new(((self.bits >> 4) & 0x1f) as u8)
    }
    #[doc = "Bit 12 - FUTURE USE Enable for the reference clock"]
    #[inline(always)]
    pub fn refclken(&self) -> RefclkenR {
        RefclkenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:17 - FUTURE USE Select the input clock frequency for the ref_clk. 0: HFRC_48MHz 1: HFRC_48MHz_GATED 2: XT_24MHz 3: HFRC2_48MHz"]
    #[inline(always)]
    pub fn reffsel(&self) -> ReffselR {
        ReffselR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 20 - 0: no change to the clock selected by FSEL 1: frequency divide-by-3 of the clock selected by FSEL"]
    #[inline(always)]
    pub fn div3(&self) -> Div3R {
        Div3R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable for the master audio clock."]
    #[inline(always)]
    #[must_use]
    pub fn mclken(&mut self) -> MclkenW<ClkcfgSpec> {
        MclkenW::new(self, 0)
    }
    #[doc = "Bits 4:8 - Select the input clock frequency for the MCLK.Whenever changing the clock source here, the MISC_HFRC2FRC bit in the CLKGEN module must first be set. The sequence for changing the clock source regardless of clock selection is to first force HFRC2 on by setting the CLKGEN_MISC_HFRC2FRC bit, select the clock source in this field, clear the CLKGEN_MISC_HFRC2FRC bit only if HFRC2 is NOT selected, and then engage the peripheral.If HFRC2 is the clock source, then shutting the module down cleanly requires switching to HFRC, for example, and then disabling the HFRC2 by clearing the CLKGEN_MISC_HFRC2FRC bit. 0: HFRC2_48MHz 1: HFRC2_24MHz 2: HFRC2_12MHz 3: HFRC2_6MHz 4: HFRC2_3MHz 5: HFRC2_1_5MHz 6: HFRC2_750KHz 7: HFRC2_375KHz 8: HFRC_24MHz 9: HFRC_12MHz 10: HFRC_6MHz 11: HFRC_3MHz 12: HFRC_1_5MHz 13: HFRC_750KHz 14: HFRC_375KHz 15: XTHS_EXTREF_CLK 16: XTHS_16MHz 17: XTHS_8MHz 18: XTHS_4MHz 19: XTHS_2MHz 20: XTHS_1MHz 21: XTHS_500KHz 22: HFRC_48MHz"]
    #[inline(always)]
    #[must_use]
    pub fn fsel(&mut self) -> FselW<ClkcfgSpec> {
        FselW::new(self, 4)
    }
    #[doc = "Bit 12 - FUTURE USE Enable for the reference clock"]
    #[inline(always)]
    #[must_use]
    pub fn refclken(&mut self) -> RefclkenW<ClkcfgSpec> {
        RefclkenW::new(self, 12)
    }
    #[doc = "Bits 16:17 - FUTURE USE Select the input clock frequency for the ref_clk. 0: HFRC_48MHz 1: HFRC_48MHz_GATED 2: XT_24MHz 3: HFRC2_48MHz"]
    #[inline(always)]
    #[must_use]
    pub fn reffsel(&mut self) -> ReffselW<ClkcfgSpec> {
        ReffselW::new(self, 16)
    }
    #[doc = "Bit 20 - 0: no change to the clock selected by FSEL 1: frequency divide-by-3 of the clock selected by FSEL"]
    #[inline(always)]
    #[must_use]
    pub fn div3(&mut self) -> Div3W<ClkcfgSpec> {
        Div3W::new(self, 20)
    }
}
#[doc = "Provides clock selection and control for I2S clocks\n\nYou can [`read`](crate::Reg::read) this register and get [`clkcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkcfgSpec;
impl crate::RegisterSpec for ClkcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkcfg::R`](R) reader structure"]
impl crate::Readable for ClkcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`clkcfg::W`](W) writer structure"]
impl crate::Writable for ClkcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKCFG to value 0x0160"]
impl crate::Resettable for ClkcfgSpec {
    const RESET_VALUE: u32 = 0x0160;
}
