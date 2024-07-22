#[doc = "Register `CLOCKEN3STAT` reader"]
pub type R = crate::R<Clocken3statSpec>;
#[doc = "Register `CLOCKEN3STAT` writer"]
pub type W = crate::W<Clocken3statSpec>;
#[doc = "Clock enable status 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Clocken3stat {
    #[doc = "1: \\[0\\]
Clock enable for the DSP_PDM0"]
    DspPdm0Clken = 1,
    #[doc = "2: \\[1\\]
Clock enable for the DSP_PDM1"]
    DspPdm1Clken = 2,
    #[doc = "4: \\[2\\]
Clock enable for the DSP_PDM2"]
    DspPdm2Clken = 4,
    #[doc = "8: \\[3\\]
Clock enable for the DSP_PDM3"]
    DspPdm3Clken = 8,
    #[doc = "16: Reserved selection. Operation unknown if selected."]
    Reserved16 = 16,
    #[doc = "32: Reserved selection. Operation unknown if selected."]
    Reserved32 = 32,
    #[doc = "64: \\[6\\]
Clock enable for the IOMSTRIFC0"]
    Iomstrifc0Clken = 64,
    #[doc = "128: \\[7\\]
Clock enable for the IO MASTER 1 IFC INTERFACE"]
    Iomstrifc1Clken = 128,
    #[doc = "256: \\[8\\]
Clock enable for the IO MASTER 2 IFC INTERFACE"]
    Iomstrifc2Clken = 256,
    #[doc = "512: \\[9\\]
Clock enable for the IO MASTER 3 IFC INTERFACE"]
    Iomstrifc3Clken = 512,
    #[doc = "1024: \\[10\\]
Clock enable for the IO MASTER 4 IFC INTERFACE"]
    Iomstrifc4Clken = 1024,
    #[doc = "2048: \\[11\\]
Clock enable for the IO MASTER 5 IFC INTERFACE"]
    Iomstrifc5Clken = 2048,
    #[doc = "4096: \\[12\\]
Clock enable for the IO MASTER 6 IFC INTERFACE"]
    Iomstrifc6Clken = 4096,
    #[doc = "8192: \\[13\\]
Clock enable for the IO MASTER 7 IFC INTERFACE"]
    Iomstrifc7Clken = 8192,
    #[doc = "16384: \\[14\\]
Clock enable for the RSTGEN"]
    RstgenClken = 16384,
    #[doc = "32768: \\[15\\]
Clock enable for the RSTGEN"]
    RstgenPosClken = 32768,
    #[doc = "65536: \\[16\\]
Clock enable for the RTC"]
    RtcClken = 65536,
    #[doc = "131072: \\[17\\]
Clock enable for the SDIO_XIN"]
    SdioXinClken = 131072,
    #[doc = "262144: \\[18\\]
Clock enable for the UART0 HF"]
    Uart0hfClken = 262144,
    #[doc = "524288: \\[19\\]
Clock enable for the UART1 HF"]
    Uart1hfClken = 524288,
    #[doc = "1048576: \\[20\\]
Clock enable for the UART2 HF"]
    Uart2hfClken = 1048576,
    #[doc = "2097152: \\[21\\]
Clock enable for the UART3 HF"]
    Uart3hfClken = 2097152,
    #[doc = "4194304: \\[22\\]
Clock enable for the USB_REFCLK"]
    UsbRefclkClken = 4194304,
    #[doc = "8388608: \\[23\\]
Clock enable for the WDT"]
    WdtClken = 8388608,
}
impl From<Clocken3stat> for u32 {
    #[inline(always)]
    fn from(variant: Clocken3stat) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Clocken3stat {
    type Ux = u32;
}
impl crate::IsEnum for Clocken3stat {}
#[doc = "Field `CLOCKEN3STAT` reader - Clock enable status 3"]
pub type Clocken3statR = crate::FieldReader<Clocken3stat>;
impl Clocken3statR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Clocken3stat> {
        match self.bits {
            1 => Some(Clocken3stat::DspPdm0Clken),
            2 => Some(Clocken3stat::DspPdm1Clken),
            4 => Some(Clocken3stat::DspPdm2Clken),
            8 => Some(Clocken3stat::DspPdm3Clken),
            16 => Some(Clocken3stat::Reserved16),
            32 => Some(Clocken3stat::Reserved32),
            64 => Some(Clocken3stat::Iomstrifc0Clken),
            128 => Some(Clocken3stat::Iomstrifc1Clken),
            256 => Some(Clocken3stat::Iomstrifc2Clken),
            512 => Some(Clocken3stat::Iomstrifc3Clken),
            1024 => Some(Clocken3stat::Iomstrifc4Clken),
            2048 => Some(Clocken3stat::Iomstrifc5Clken),
            4096 => Some(Clocken3stat::Iomstrifc6Clken),
            8192 => Some(Clocken3stat::Iomstrifc7Clken),
            16384 => Some(Clocken3stat::RstgenClken),
            32768 => Some(Clocken3stat::RstgenPosClken),
            65536 => Some(Clocken3stat::RtcClken),
            131072 => Some(Clocken3stat::SdioXinClken),
            262144 => Some(Clocken3stat::Uart0hfClken),
            524288 => Some(Clocken3stat::Uart1hfClken),
            1048576 => Some(Clocken3stat::Uart2hfClken),
            2097152 => Some(Clocken3stat::Uart3hfClken),
            4194304 => Some(Clocken3stat::UsbRefclkClken),
            8388608 => Some(Clocken3stat::WdtClken),
            _ => None,
        }
    }
    #[doc = "0\\]
Clock enable for the DSP_PDM0"]
    #[inline(always)]
    pub fn is_dsp_pdm0_clken(&self) -> bool {
        *self == Clocken3stat::DspPdm0Clken
    }
    #[doc = "1\\]
Clock enable for the DSP_PDM1"]
    #[inline(always)]
    pub fn is_dsp_pdm1_clken(&self) -> bool {
        *self == Clocken3stat::DspPdm1Clken
    }
    #[doc = "2\\]
Clock enable for the DSP_PDM2"]
    #[inline(always)]
    pub fn is_dsp_pdm2_clken(&self) -> bool {
        *self == Clocken3stat::DspPdm2Clken
    }
    #[doc = "3\\]
Clock enable for the DSP_PDM3"]
    #[inline(always)]
    pub fn is_dsp_pdm3_clken(&self) -> bool {
        *self == Clocken3stat::DspPdm3Clken
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved16(&self) -> bool {
        *self == Clocken3stat::Reserved16
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved32(&self) -> bool {
        *self == Clocken3stat::Reserved32
    }
    #[doc = "6\\]
Clock enable for the IOMSTRIFC0"]
    #[inline(always)]
    pub fn is_iomstrifc0_clken(&self) -> bool {
        *self == Clocken3stat::Iomstrifc0Clken
    }
    #[doc = "7\\]
Clock enable for the IO MASTER 1 IFC INTERFACE"]
    #[inline(always)]
    pub fn is_iomstrifc1_clken(&self) -> bool {
        *self == Clocken3stat::Iomstrifc1Clken
    }
    #[doc = "8\\]
Clock enable for the IO MASTER 2 IFC INTERFACE"]
    #[inline(always)]
    pub fn is_iomstrifc2_clken(&self) -> bool {
        *self == Clocken3stat::Iomstrifc2Clken
    }
    #[doc = "9\\]
Clock enable for the IO MASTER 3 IFC INTERFACE"]
    #[inline(always)]
    pub fn is_iomstrifc3_clken(&self) -> bool {
        *self == Clocken3stat::Iomstrifc3Clken
    }
    #[doc = "10\\]
Clock enable for the IO MASTER 4 IFC INTERFACE"]
    #[inline(always)]
    pub fn is_iomstrifc4_clken(&self) -> bool {
        *self == Clocken3stat::Iomstrifc4Clken
    }
    #[doc = "11\\]
Clock enable for the IO MASTER 5 IFC INTERFACE"]
    #[inline(always)]
    pub fn is_iomstrifc5_clken(&self) -> bool {
        *self == Clocken3stat::Iomstrifc5Clken
    }
    #[doc = "12\\]
Clock enable for the IO MASTER 6 IFC INTERFACE"]
    #[inline(always)]
    pub fn is_iomstrifc6_clken(&self) -> bool {
        *self == Clocken3stat::Iomstrifc6Clken
    }
    #[doc = "13\\]
Clock enable for the IO MASTER 7 IFC INTERFACE"]
    #[inline(always)]
    pub fn is_iomstrifc7_clken(&self) -> bool {
        *self == Clocken3stat::Iomstrifc7Clken
    }
    #[doc = "14\\]
Clock enable for the RSTGEN"]
    #[inline(always)]
    pub fn is_rstgen_clken(&self) -> bool {
        *self == Clocken3stat::RstgenClken
    }
    #[doc = "15\\]
Clock enable for the RSTGEN"]
    #[inline(always)]
    pub fn is_rstgen_pos_clken(&self) -> bool {
        *self == Clocken3stat::RstgenPosClken
    }
    #[doc = "16\\]
Clock enable for the RTC"]
    #[inline(always)]
    pub fn is_rtc_clken(&self) -> bool {
        *self == Clocken3stat::RtcClken
    }
    #[doc = "17\\]
Clock enable for the SDIO_XIN"]
    #[inline(always)]
    pub fn is_sdio_xin_clken(&self) -> bool {
        *self == Clocken3stat::SdioXinClken
    }
    #[doc = "18\\]
Clock enable for the UART0 HF"]
    #[inline(always)]
    pub fn is_uart0hf_clken(&self) -> bool {
        *self == Clocken3stat::Uart0hfClken
    }
    #[doc = "19\\]
Clock enable for the UART1 HF"]
    #[inline(always)]
    pub fn is_uart1hf_clken(&self) -> bool {
        *self == Clocken3stat::Uart1hfClken
    }
    #[doc = "20\\]
Clock enable for the UART2 HF"]
    #[inline(always)]
    pub fn is_uart2hf_clken(&self) -> bool {
        *self == Clocken3stat::Uart2hfClken
    }
    #[doc = "21\\]
Clock enable for the UART3 HF"]
    #[inline(always)]
    pub fn is_uart3hf_clken(&self) -> bool {
        *self == Clocken3stat::Uart3hfClken
    }
    #[doc = "22\\]
Clock enable for the USB_REFCLK"]
    #[inline(always)]
    pub fn is_usb_refclk_clken(&self) -> bool {
        *self == Clocken3stat::UsbRefclkClken
    }
    #[doc = "23\\]
Clock enable for the WDT"]
    #[inline(always)]
    pub fn is_wdt_clken(&self) -> bool {
        *self == Clocken3stat::WdtClken
    }
}
#[doc = "Field `CLOCKEN3STAT` writer - Clock enable status 3"]
pub type Clocken3statW<'a, REG> = crate::FieldWriter<'a, REG, 32, Clocken3stat>;
impl<'a, REG> Clocken3statW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "0\\]
Clock enable for the DSP_PDM0"]
    #[inline(always)]
    pub fn dsp_pdm0_clken(self) -> &'a mut crate::W<REG> {
        self.variant(Clocken3stat::DspPdm0Clken)
    }
    #[doc = "1\\]
Clock enable for the DSP_PDM1"]
    #[inline(always)]
    pub fn dsp_pdm1_clken(self) -> &'a mut crate::W<REG> {
        self.variant(Clocken3stat::DspPdm1Clken)
    }
    #[doc = "2\\]
Clock enable for the DSP_PDM2"]
    #[inline(always)]
    pub fn dsp_pdm2_clken(self) -> &'a mut crate::W<REG> {
        self.variant(Clocken3stat::DspPdm2Clken)
    }
    #[doc = "3\\]
Clock enable for the DSP_PDM3"]
    #[inline(always)]
    pub fn dsp_pdm3_clken(self) -> &'a mut crate::W<REG> {
        self.variant(Clocken3stat::DspPdm3Clken)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved16(self) -> &'a mut crate::W<REG> {
        self.variant(Clocken3stat::Reserved16)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved32(self) -> &'a mut crate::W<REG> {
        self.variant(Clocken3stat::Reserved32)
    }
    #[doc = "6\\]
Clock enable for the IOMSTRIFC0"]
    #[inline(always)]
    pub fn iomstrifc0_clken(self) -> &'a mut crate::W<REG> {
        self.variant(Clocken3stat::Iomstrifc0Clken)
    }
    #[doc = "7\\]
Clock enable for the IO MASTER 1 IFC INTERFACE"]
    #[inline(always)]
    pub fn iomstrifc1_clken(self) -> &'a mut crate::W<REG> {
        self.variant(Clocken3stat::Iomstrifc1Clken)
    }
    #[doc = "8\\]
Clock enable for the IO MASTER 2 IFC INTERFACE"]
    #[inline(always)]
    pub fn iomstrifc2_clken(self) -> &'a mut crate::W<REG> {
        self.variant(Clocken3stat::Iomstrifc2Clken)
    }
    #[doc = "9\\]
Clock enable for the IO MASTER 3 IFC INTERFACE"]
    #[inline(always)]
    pub fn iomstrifc3_clken(self) -> &'a mut crate::W<REG> {
        self.variant(Clocken3stat::Iomstrifc3Clken)
    }
    #[doc = "10\\]
Clock enable for the IO MASTER 4 IFC INTERFACE"]
    #[inline(always)]
    pub fn iomstrifc4_clken(self) -> &'a mut crate::W<REG> {
        self.variant(Clocken3stat::Iomstrifc4Clken)
    }
    #[doc = "11\\]
Clock enable for the IO MASTER 5 IFC INTERFACE"]
    #[inline(always)]
    pub fn iomstrifc5_clken(self) -> &'a mut crate::W<REG> {
        self.variant(Clocken3stat::Iomstrifc5Clken)
    }
    #[doc = "12\\]
Clock enable for the IO MASTER 6 IFC INTERFACE"]
    #[inline(always)]
    pub fn iomstrifc6_clken(self) -> &'a mut crate::W<REG> {
        self.variant(Clocken3stat::Iomstrifc6Clken)
    }
    #[doc = "13\\]
Clock enable for the IO MASTER 7 IFC INTERFACE"]
    #[inline(always)]
    pub fn iomstrifc7_clken(self) -> &'a mut crate::W<REG> {
        self.variant(Clocken3stat::Iomstrifc7Clken)
    }
    #[doc = "14\\]
Clock enable for the RSTGEN"]
    #[inline(always)]
    pub fn rstgen_clken(self) -> &'a mut crate::W<REG> {
        self.variant(Clocken3stat::RstgenClken)
    }
    #[doc = "15\\]
Clock enable for the RSTGEN"]
    #[inline(always)]
    pub fn rstgen_pos_clken(self) -> &'a mut crate::W<REG> {
        self.variant(Clocken3stat::RstgenPosClken)
    }
    #[doc = "16\\]
Clock enable for the RTC"]
    #[inline(always)]
    pub fn rtc_clken(self) -> &'a mut crate::W<REG> {
        self.variant(Clocken3stat::RtcClken)
    }
    #[doc = "17\\]
Clock enable for the SDIO_XIN"]
    #[inline(always)]
    pub fn sdio_xin_clken(self) -> &'a mut crate::W<REG> {
        self.variant(Clocken3stat::SdioXinClken)
    }
    #[doc = "18\\]
Clock enable for the UART0 HF"]
    #[inline(always)]
    pub fn uart0hf_clken(self) -> &'a mut crate::W<REG> {
        self.variant(Clocken3stat::Uart0hfClken)
    }
    #[doc = "19\\]
Clock enable for the UART1 HF"]
    #[inline(always)]
    pub fn uart1hf_clken(self) -> &'a mut crate::W<REG> {
        self.variant(Clocken3stat::Uart1hfClken)
    }
    #[doc = "20\\]
Clock enable for the UART2 HF"]
    #[inline(always)]
    pub fn uart2hf_clken(self) -> &'a mut crate::W<REG> {
        self.variant(Clocken3stat::Uart2hfClken)
    }
    #[doc = "21\\]
Clock enable for the UART3 HF"]
    #[inline(always)]
    pub fn uart3hf_clken(self) -> &'a mut crate::W<REG> {
        self.variant(Clocken3stat::Uart3hfClken)
    }
    #[doc = "22\\]
Clock enable for the USB_REFCLK"]
    #[inline(always)]
    pub fn usb_refclk_clken(self) -> &'a mut crate::W<REG> {
        self.variant(Clocken3stat::UsbRefclkClken)
    }
    #[doc = "23\\]
Clock enable for the WDT"]
    #[inline(always)]
    pub fn wdt_clken(self) -> &'a mut crate::W<REG> {
        self.variant(Clocken3stat::WdtClken)
    }
}
impl R {
    #[doc = "Bits 0:31 - Clock enable status 3"]
    #[inline(always)]
    pub fn clocken3stat(&self) -> Clocken3statR {
        Clocken3statR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Clock enable status 3"]
    #[inline(always)]
    #[must_use]
    pub fn clocken3stat(&mut self) -> Clocken3statW<Clocken3statSpec> {
        Clocken3statW::new(self, 0)
    }
}
#[doc = "This is a continuation of the clock enable status.\n\nYou can [`read`](crate::Reg::read) this register and get [`clocken3stat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clocken3stat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Clocken3statSpec;
impl crate::RegisterSpec for Clocken3statSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clocken3stat::R`](R) reader structure"]
impl crate::Readable for Clocken3statSpec {}
#[doc = "`write(|w| ..)` method takes [`clocken3stat::W`](W) writer structure"]
impl crate::Writable for Clocken3statSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLOCKEN3STAT to value 0"]
impl crate::Resettable for Clocken3statSpec {
    const RESET_VALUE: u32 = 0;
}
