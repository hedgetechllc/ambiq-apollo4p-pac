#[doc = "Register `CLOCKEN2STAT` reader"]
pub type R = crate::R<Clocken2statSpec>;
#[doc = "Register `CLOCKEN2STAT` writer"]
pub type W = crate::W<Clocken2statSpec>;
#[doc = "Clock enable status 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Clocken2stat {
    #[doc = "1: \\[0\\]
Clock enable for the ADC."]
    AdcClken = 1,
    #[doc = "2: \\[1\\]
Clock enable for the APBDMA ACTIVITY"]
    ApbdmaActivityClken = 2,
    #[doc = "4: \\[2\\]
Clock enable for the APBDMA AOH DOMAIN"]
    ApbdmaAohClken = 4,
    #[doc = "8: \\[3\\]
Clock enable for the APBDMA AOL DOMAIN"]
    ApbdmaAolClken = 8,
    #[doc = "16: \\[4\\]
Clock enable for the APBDMA_APB"]
    ApbdmaApbClken = 16,
    #[doc = "32: \\[5\\]
Clock enable for the APBDMA_AUD"]
    ApbdmaAudClken = 32,
    #[doc = "64: \\[6\\]
Clock enable for the APBDMA_HCPA"]
    ApbdmaCryptoClken = 64,
    #[doc = "128: \\[7\\]
Clock enable for the APBDMA_DBG"]
    ApbdmaDbgClken = 128,
    #[doc = "256: \\[8\\]
Clock enable for the APBDMA_DISP"]
    ApbdmaDispClken = 256,
    #[doc = "512: \\[9\\]
Clock enable for the APBDMA_DISPPHY"]
    ApbdmaDispphyClken = 512,
    #[doc = "1024: \\[10\\]
Clock enable for the APBDMA_DSPA"]
    ApbdmaDspaClken = 1024,
    #[doc = "2048: \\[11\\]
Clock enable for the APBDMA_GFX"]
    ApbdmaGfxClken = 2048,
    #[doc = "4096: \\[12\\]
Clock enable for the APBDMA_HSPA"]
    ApbdmaHspaClken = 4096,
    #[doc = "8192: \\[13\\]
Clock enable for the APBDMA_HSPB"]
    ApbdmaHspbClken = 8192,
    #[doc = "16384: \\[14\\]
Clock enable for the APBDMA_HSPC"]
    ApbdmaHspcClken = 16384,
    #[doc = "32768: \\[15\\]
Clock enable for the APBDMA_IOS"]
    ApbdmaIosClken = 32768,
    #[doc = "65536: \\[16\\]
Clock enable for the APBDMA_MSPI0"]
    ApbdmaMspi0Clken = 65536,
    #[doc = "131072: \\[17\\]
Clock enable for the APBDMA_MSPI1"]
    ApbdmaMspi1Clken = 131072,
    #[doc = "262144: \\[18\\]
Clock enable for the APBDMA_MSPI2"]
    ApbdmaMspi2Clken = 262144,
    #[doc = "524288: \\[19\\]
Clock enable for the APBDMA_SDIO"]
    ApbdmaSdioClken = 524288,
    #[doc = "1048576: \\[20\\]
Clock enable for the APBDMA_USB"]
    ApbdmaUsbClken = 1048576,
    #[doc = "2097152: \\[21\\]
Clock enable for the AUDADC"]
    AudadcClken = 2097152,
    #[doc = "4194304: \\[22\\]
Clock enable for the CM4_TPIU"]
    Cm4TpiuClken = 4194304,
    #[doc = "8388608: \\[23\\]
Clock enable for the DBG_TPIU"]
    DbgTpiuClken = 8388608,
    #[doc = "16777216: \\[24\\]
Clock enable for the DBG_TS"]
    DbgTsClken = 16777216,
    #[doc = "33554432: \\[25\\]
Clock enable for the DISP_CLK"]
    DispClkClken = 33554432,
    #[doc = "67108864: \\[26\\]
Clock enable for the DPHY_PLL_REF"]
    DphyPllRefClken = 67108864,
    #[doc = "134217728: \\[27\\]
Clock enable for the DSP_I2S0"]
    DspI2s0Clken = 134217728,
    #[doc = "268435456: \\[28\\]
Clock enable for the DSP_I2S0_REFCLK"]
    DspI2s0RefclkClken = 268435456,
    #[doc = "536870912: \\[29\\]
Clock enable for the DSP_I2S1"]
    DspI2s1Clken = 536870912,
    #[doc = "1073741824: \\[30\\]
Clock enable for the DSP_I2S1_REFCLK"]
    DspI2s1RefclkClken = 1073741824,
    #[doc = "2147483648: \\[31\\]
Clock enable for the DSP_MILLI"]
    DspMilliClken = 2147483648,
}
impl From<Clocken2stat> for u32 {
    #[inline(always)]
    fn from(variant: Clocken2stat) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Clocken2stat {
    type Ux = u32;
}
impl crate::IsEnum for Clocken2stat {}
#[doc = "Field `CLOCKEN2STAT` reader - Clock enable status 2"]
pub type Clocken2statR = crate::FieldReader<Clocken2stat>;
impl Clocken2statR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Clocken2stat> {
        match self.bits {
            1 => Some(Clocken2stat::AdcClken),
            2 => Some(Clocken2stat::ApbdmaActivityClken),
            4 => Some(Clocken2stat::ApbdmaAohClken),
            8 => Some(Clocken2stat::ApbdmaAolClken),
            16 => Some(Clocken2stat::ApbdmaApbClken),
            32 => Some(Clocken2stat::ApbdmaAudClken),
            64 => Some(Clocken2stat::ApbdmaCryptoClken),
            128 => Some(Clocken2stat::ApbdmaDbgClken),
            256 => Some(Clocken2stat::ApbdmaDispClken),
            512 => Some(Clocken2stat::ApbdmaDispphyClken),
            1024 => Some(Clocken2stat::ApbdmaDspaClken),
            2048 => Some(Clocken2stat::ApbdmaGfxClken),
            4096 => Some(Clocken2stat::ApbdmaHspaClken),
            8192 => Some(Clocken2stat::ApbdmaHspbClken),
            16384 => Some(Clocken2stat::ApbdmaHspcClken),
            32768 => Some(Clocken2stat::ApbdmaIosClken),
            65536 => Some(Clocken2stat::ApbdmaMspi0Clken),
            131072 => Some(Clocken2stat::ApbdmaMspi1Clken),
            262144 => Some(Clocken2stat::ApbdmaMspi2Clken),
            524288 => Some(Clocken2stat::ApbdmaSdioClken),
            1048576 => Some(Clocken2stat::ApbdmaUsbClken),
            2097152 => Some(Clocken2stat::AudadcClken),
            4194304 => Some(Clocken2stat::Cm4TpiuClken),
            8388608 => Some(Clocken2stat::DbgTpiuClken),
            16777216 => Some(Clocken2stat::DbgTsClken),
            33554432 => Some(Clocken2stat::DispClkClken),
            67108864 => Some(Clocken2stat::DphyPllRefClken),
            134217728 => Some(Clocken2stat::DspI2s0Clken),
            268435456 => Some(Clocken2stat::DspI2s0RefclkClken),
            536870912 => Some(Clocken2stat::DspI2s1Clken),
            1073741824 => Some(Clocken2stat::DspI2s1RefclkClken),
            2147483648 => Some(Clocken2stat::DspMilliClken),
            _ => None,
        }
    }
    #[doc = "0\\]
Clock enable for the ADC."]
    #[inline(always)]
    pub fn is_adc_clken(&self) -> bool {
        *self == Clocken2stat::AdcClken
    }
    #[doc = "1\\]
Clock enable for the APBDMA ACTIVITY"]
    #[inline(always)]
    pub fn is_apbdma_activity_clken(&self) -> bool {
        *self == Clocken2stat::ApbdmaActivityClken
    }
    #[doc = "2\\]
Clock enable for the APBDMA AOH DOMAIN"]
    #[inline(always)]
    pub fn is_apbdma_aoh_clken(&self) -> bool {
        *self == Clocken2stat::ApbdmaAohClken
    }
    #[doc = "3\\]
Clock enable for the APBDMA AOL DOMAIN"]
    #[inline(always)]
    pub fn is_apbdma_aol_clken(&self) -> bool {
        *self == Clocken2stat::ApbdmaAolClken
    }
    #[doc = "4\\]
Clock enable for the APBDMA_APB"]
    #[inline(always)]
    pub fn is_apbdma_apb_clken(&self) -> bool {
        *self == Clocken2stat::ApbdmaApbClken
    }
    #[doc = "5\\]
Clock enable for the APBDMA_AUD"]
    #[inline(always)]
    pub fn is_apbdma_aud_clken(&self) -> bool {
        *self == Clocken2stat::ApbdmaAudClken
    }
    #[doc = "6\\]
Clock enable for the APBDMA_HCPA"]
    #[inline(always)]
    pub fn is_apbdma_crypto_clken(&self) -> bool {
        *self == Clocken2stat::ApbdmaCryptoClken
    }
    #[doc = "7\\]
Clock enable for the APBDMA_DBG"]
    #[inline(always)]
    pub fn is_apbdma_dbg_clken(&self) -> bool {
        *self == Clocken2stat::ApbdmaDbgClken
    }
    #[doc = "8\\]
Clock enable for the APBDMA_DISP"]
    #[inline(always)]
    pub fn is_apbdma_disp_clken(&self) -> bool {
        *self == Clocken2stat::ApbdmaDispClken
    }
    #[doc = "9\\]
Clock enable for the APBDMA_DISPPHY"]
    #[inline(always)]
    pub fn is_apbdma_dispphy_clken(&self) -> bool {
        *self == Clocken2stat::ApbdmaDispphyClken
    }
    #[doc = "10\\]
Clock enable for the APBDMA_DSPA"]
    #[inline(always)]
    pub fn is_apbdma_dspa_clken(&self) -> bool {
        *self == Clocken2stat::ApbdmaDspaClken
    }
    #[doc = "11\\]
Clock enable for the APBDMA_GFX"]
    #[inline(always)]
    pub fn is_apbdma_gfx_clken(&self) -> bool {
        *self == Clocken2stat::ApbdmaGfxClken
    }
    #[doc = "12\\]
Clock enable for the APBDMA_HSPA"]
    #[inline(always)]
    pub fn is_apbdma_hspa_clken(&self) -> bool {
        *self == Clocken2stat::ApbdmaHspaClken
    }
    #[doc = "13\\]
Clock enable for the APBDMA_HSPB"]
    #[inline(always)]
    pub fn is_apbdma_hspb_clken(&self) -> bool {
        *self == Clocken2stat::ApbdmaHspbClken
    }
    #[doc = "14\\]
Clock enable for the APBDMA_HSPC"]
    #[inline(always)]
    pub fn is_apbdma_hspc_clken(&self) -> bool {
        *self == Clocken2stat::ApbdmaHspcClken
    }
    #[doc = "15\\]
Clock enable for the APBDMA_IOS"]
    #[inline(always)]
    pub fn is_apbdma_ios_clken(&self) -> bool {
        *self == Clocken2stat::ApbdmaIosClken
    }
    #[doc = "16\\]
Clock enable for the APBDMA_MSPI0"]
    #[inline(always)]
    pub fn is_apbdma_mspi0_clken(&self) -> bool {
        *self == Clocken2stat::ApbdmaMspi0Clken
    }
    #[doc = "17\\]
Clock enable for the APBDMA_MSPI1"]
    #[inline(always)]
    pub fn is_apbdma_mspi1_clken(&self) -> bool {
        *self == Clocken2stat::ApbdmaMspi1Clken
    }
    #[doc = "18\\]
Clock enable for the APBDMA_MSPI2"]
    #[inline(always)]
    pub fn is_apbdma_mspi2_clken(&self) -> bool {
        *self == Clocken2stat::ApbdmaMspi2Clken
    }
    #[doc = "19\\]
Clock enable for the APBDMA_SDIO"]
    #[inline(always)]
    pub fn is_apbdma_sdio_clken(&self) -> bool {
        *self == Clocken2stat::ApbdmaSdioClken
    }
    #[doc = "20\\]
Clock enable for the APBDMA_USB"]
    #[inline(always)]
    pub fn is_apbdma_usb_clken(&self) -> bool {
        *self == Clocken2stat::ApbdmaUsbClken
    }
    #[doc = "21\\]
Clock enable for the AUDADC"]
    #[inline(always)]
    pub fn is_audadc_clken(&self) -> bool {
        *self == Clocken2stat::AudadcClken
    }
    #[doc = "22\\]
Clock enable for the CM4_TPIU"]
    #[inline(always)]
    pub fn is_cm4_tpiu_clken(&self) -> bool {
        *self == Clocken2stat::Cm4TpiuClken
    }
    #[doc = "23\\]
Clock enable for the DBG_TPIU"]
    #[inline(always)]
    pub fn is_dbg_tpiu_clken(&self) -> bool {
        *self == Clocken2stat::DbgTpiuClken
    }
    #[doc = "24\\]
Clock enable for the DBG_TS"]
    #[inline(always)]
    pub fn is_dbg_ts_clken(&self) -> bool {
        *self == Clocken2stat::DbgTsClken
    }
    #[doc = "25\\]
Clock enable for the DISP_CLK"]
    #[inline(always)]
    pub fn is_disp_clk_clken(&self) -> bool {
        *self == Clocken2stat::DispClkClken
    }
    #[doc = "26\\]
Clock enable for the DPHY_PLL_REF"]
    #[inline(always)]
    pub fn is_dphy_pll_ref_clken(&self) -> bool {
        *self == Clocken2stat::DphyPllRefClken
    }
    #[doc = "27\\]
Clock enable for the DSP_I2S0"]
    #[inline(always)]
    pub fn is_dsp_i2s0_clken(&self) -> bool {
        *self == Clocken2stat::DspI2s0Clken
    }
    #[doc = "28\\]
Clock enable for the DSP_I2S0_REFCLK"]
    #[inline(always)]
    pub fn is_dsp_i2s0_refclk_clken(&self) -> bool {
        *self == Clocken2stat::DspI2s0RefclkClken
    }
    #[doc = "29\\]
Clock enable for the DSP_I2S1"]
    #[inline(always)]
    pub fn is_dsp_i2s1_clken(&self) -> bool {
        *self == Clocken2stat::DspI2s1Clken
    }
    #[doc = "30\\]
Clock enable for the DSP_I2S1_REFCLK"]
    #[inline(always)]
    pub fn is_dsp_i2s1_refclk_clken(&self) -> bool {
        *self == Clocken2stat::DspI2s1RefclkClken
    }
    #[doc = "31\\]
Clock enable for the DSP_MILLI"]
    #[inline(always)]
    pub fn is_dsp_milli_clken(&self) -> bool {
        *self == Clocken2stat::DspMilliClken
    }
}
#[doc = "Field `CLOCKEN2STAT` writer - Clock enable status 2"]
pub type Clocken2statW<'a, REG> = crate::FieldWriter<'a, REG, 32, Clocken2stat>;
impl<'a, REG> Clocken2statW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "0\\]
Clock enable for the ADC."]
    #[inline(always)]
    pub fn adc_clken(self) -> &'a mut crate::W<REG> {
        self.variant(Clocken2stat::AdcClken)
    }
    #[doc = "1\\]
Clock enable for the APBDMA ACTIVITY"]
    #[inline(always)]
    pub fn apbdma_activity_clken(self) -> &'a mut crate::W<REG> {
        self.variant(Clocken2stat::ApbdmaActivityClken)
    }
    #[doc = "2\\]
Clock enable for the APBDMA AOH DOMAIN"]
    #[inline(always)]
    pub fn apbdma_aoh_clken(self) -> &'a mut crate::W<REG> {
        self.variant(Clocken2stat::ApbdmaAohClken)
    }
    #[doc = "3\\]
Clock enable for the APBDMA AOL DOMAIN"]
    #[inline(always)]
    pub fn apbdma_aol_clken(self) -> &'a mut crate::W<REG> {
        self.variant(Clocken2stat::ApbdmaAolClken)
    }
    #[doc = "4\\]
Clock enable for the APBDMA_APB"]
    #[inline(always)]
    pub fn apbdma_apb_clken(self) -> &'a mut crate::W<REG> {
        self.variant(Clocken2stat::ApbdmaApbClken)
    }
    #[doc = "5\\]
Clock enable for the APBDMA_AUD"]
    #[inline(always)]
    pub fn apbdma_aud_clken(self) -> &'a mut crate::W<REG> {
        self.variant(Clocken2stat::ApbdmaAudClken)
    }
    #[doc = "6\\]
Clock enable for the APBDMA_HCPA"]
    #[inline(always)]
    pub fn apbdma_crypto_clken(self) -> &'a mut crate::W<REG> {
        self.variant(Clocken2stat::ApbdmaCryptoClken)
    }
    #[doc = "7\\]
Clock enable for the APBDMA_DBG"]
    #[inline(always)]
    pub fn apbdma_dbg_clken(self) -> &'a mut crate::W<REG> {
        self.variant(Clocken2stat::ApbdmaDbgClken)
    }
    #[doc = "8\\]
Clock enable for the APBDMA_DISP"]
    #[inline(always)]
    pub fn apbdma_disp_clken(self) -> &'a mut crate::W<REG> {
        self.variant(Clocken2stat::ApbdmaDispClken)
    }
    #[doc = "9\\]
Clock enable for the APBDMA_DISPPHY"]
    #[inline(always)]
    pub fn apbdma_dispphy_clken(self) -> &'a mut crate::W<REG> {
        self.variant(Clocken2stat::ApbdmaDispphyClken)
    }
    #[doc = "10\\]
Clock enable for the APBDMA_DSPA"]
    #[inline(always)]
    pub fn apbdma_dspa_clken(self) -> &'a mut crate::W<REG> {
        self.variant(Clocken2stat::ApbdmaDspaClken)
    }
    #[doc = "11\\]
Clock enable for the APBDMA_GFX"]
    #[inline(always)]
    pub fn apbdma_gfx_clken(self) -> &'a mut crate::W<REG> {
        self.variant(Clocken2stat::ApbdmaGfxClken)
    }
    #[doc = "12\\]
Clock enable for the APBDMA_HSPA"]
    #[inline(always)]
    pub fn apbdma_hspa_clken(self) -> &'a mut crate::W<REG> {
        self.variant(Clocken2stat::ApbdmaHspaClken)
    }
    #[doc = "13\\]
Clock enable for the APBDMA_HSPB"]
    #[inline(always)]
    pub fn apbdma_hspb_clken(self) -> &'a mut crate::W<REG> {
        self.variant(Clocken2stat::ApbdmaHspbClken)
    }
    #[doc = "14\\]
Clock enable for the APBDMA_HSPC"]
    #[inline(always)]
    pub fn apbdma_hspc_clken(self) -> &'a mut crate::W<REG> {
        self.variant(Clocken2stat::ApbdmaHspcClken)
    }
    #[doc = "15\\]
Clock enable for the APBDMA_IOS"]
    #[inline(always)]
    pub fn apbdma_ios_clken(self) -> &'a mut crate::W<REG> {
        self.variant(Clocken2stat::ApbdmaIosClken)
    }
    #[doc = "16\\]
Clock enable for the APBDMA_MSPI0"]
    #[inline(always)]
    pub fn apbdma_mspi0_clken(self) -> &'a mut crate::W<REG> {
        self.variant(Clocken2stat::ApbdmaMspi0Clken)
    }
    #[doc = "17\\]
Clock enable for the APBDMA_MSPI1"]
    #[inline(always)]
    pub fn apbdma_mspi1_clken(self) -> &'a mut crate::W<REG> {
        self.variant(Clocken2stat::ApbdmaMspi1Clken)
    }
    #[doc = "18\\]
Clock enable for the APBDMA_MSPI2"]
    #[inline(always)]
    pub fn apbdma_mspi2_clken(self) -> &'a mut crate::W<REG> {
        self.variant(Clocken2stat::ApbdmaMspi2Clken)
    }
    #[doc = "19\\]
Clock enable for the APBDMA_SDIO"]
    #[inline(always)]
    pub fn apbdma_sdio_clken(self) -> &'a mut crate::W<REG> {
        self.variant(Clocken2stat::ApbdmaSdioClken)
    }
    #[doc = "20\\]
Clock enable for the APBDMA_USB"]
    #[inline(always)]
    pub fn apbdma_usb_clken(self) -> &'a mut crate::W<REG> {
        self.variant(Clocken2stat::ApbdmaUsbClken)
    }
    #[doc = "21\\]
Clock enable for the AUDADC"]
    #[inline(always)]
    pub fn audadc_clken(self) -> &'a mut crate::W<REG> {
        self.variant(Clocken2stat::AudadcClken)
    }
    #[doc = "22\\]
Clock enable for the CM4_TPIU"]
    #[inline(always)]
    pub fn cm4_tpiu_clken(self) -> &'a mut crate::W<REG> {
        self.variant(Clocken2stat::Cm4TpiuClken)
    }
    #[doc = "23\\]
Clock enable for the DBG_TPIU"]
    #[inline(always)]
    pub fn dbg_tpiu_clken(self) -> &'a mut crate::W<REG> {
        self.variant(Clocken2stat::DbgTpiuClken)
    }
    #[doc = "24\\]
Clock enable for the DBG_TS"]
    #[inline(always)]
    pub fn dbg_ts_clken(self) -> &'a mut crate::W<REG> {
        self.variant(Clocken2stat::DbgTsClken)
    }
    #[doc = "25\\]
Clock enable for the DISP_CLK"]
    #[inline(always)]
    pub fn disp_clk_clken(self) -> &'a mut crate::W<REG> {
        self.variant(Clocken2stat::DispClkClken)
    }
    #[doc = "26\\]
Clock enable for the DPHY_PLL_REF"]
    #[inline(always)]
    pub fn dphy_pll_ref_clken(self) -> &'a mut crate::W<REG> {
        self.variant(Clocken2stat::DphyPllRefClken)
    }
    #[doc = "27\\]
Clock enable for the DSP_I2S0"]
    #[inline(always)]
    pub fn dsp_i2s0_clken(self) -> &'a mut crate::W<REG> {
        self.variant(Clocken2stat::DspI2s0Clken)
    }
    #[doc = "28\\]
Clock enable for the DSP_I2S0_REFCLK"]
    #[inline(always)]
    pub fn dsp_i2s0_refclk_clken(self) -> &'a mut crate::W<REG> {
        self.variant(Clocken2stat::DspI2s0RefclkClken)
    }
    #[doc = "29\\]
Clock enable for the DSP_I2S1"]
    #[inline(always)]
    pub fn dsp_i2s1_clken(self) -> &'a mut crate::W<REG> {
        self.variant(Clocken2stat::DspI2s1Clken)
    }
    #[doc = "30\\]
Clock enable for the DSP_I2S1_REFCLK"]
    #[inline(always)]
    pub fn dsp_i2s1_refclk_clken(self) -> &'a mut crate::W<REG> {
        self.variant(Clocken2stat::DspI2s1RefclkClken)
    }
    #[doc = "31\\]
Clock enable for the DSP_MILLI"]
    #[inline(always)]
    pub fn dsp_milli_clken(self) -> &'a mut crate::W<REG> {
        self.variant(Clocken2stat::DspMilliClken)
    }
}
impl R {
    #[doc = "Bits 0:31 - Clock enable status 2"]
    #[inline(always)]
    pub fn clocken2stat(&self) -> Clocken2statR {
        Clocken2statR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Clock enable status 2"]
    #[inline(always)]
    #[must_use]
    pub fn clocken2stat(&mut self) -> Clocken2statW<Clocken2statSpec> {
        Clocken2statW::new(self, 0)
    }
}
#[doc = "This is a continuation of the clock enable status.\n\nYou can [`read`](crate::Reg::read) this register and get [`clocken2stat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clocken2stat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Clocken2statSpec;
impl crate::RegisterSpec for Clocken2statSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clocken2stat::R`](R) reader structure"]
impl crate::Readable for Clocken2statSpec {}
#[doc = "`write(|w| ..)` method takes [`clocken2stat::W`](W) writer structure"]
impl crate::Writable for Clocken2statSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLOCKEN2STAT to value 0"]
impl crate::Resettable for Clocken2statSpec {
    const RESET_VALUE: u32 = 0;
}
