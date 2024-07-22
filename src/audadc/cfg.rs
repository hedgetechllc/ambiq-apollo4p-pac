#[doc = "Register `CFG` reader"]
pub type R = crate::R<CfgSpec>;
#[doc = "Register `CFG` writer"]
pub type W = crate::W<CfgSpec>;
#[doc = "This bit enables the AUDADC module. While the AUDADC is enabled, the ADCCFG and SLOT Configuration regsiter settings must remain stable and unchanged. All configuration register settings, slot configuration settings and window comparison settings should be written prior to setting the ADCEN bit to '1'.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adcen {
    #[doc = "0: Disable the AUDADC module."]
    Dis = 0,
    #[doc = "1: Enable the AUDADC module."]
    En = 1,
}
impl From<Adcen> for bool {
    #[inline(always)]
    fn from(variant: Adcen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCEN` reader - This bit enables the AUDADC module. While the AUDADC is enabled, the ADCCFG and SLOT Configuration regsiter settings must remain stable and unchanged. All configuration register settings, slot configuration settings and window comparison settings should be written prior to setting the ADCEN bit to '1'."]
pub type AdcenR = crate::BitReader<Adcen>;
impl AdcenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adcen {
        match self.bits {
            false => Adcen::Dis,
            true => Adcen::En,
        }
    }
    #[doc = "Disable the AUDADC module."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Adcen::Dis
    }
    #[doc = "Enable the AUDADC module."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Adcen::En
    }
}
#[doc = "Field `ADCEN` writer - This bit enables the AUDADC module. While the AUDADC is enabled, the ADCCFG and SLOT Configuration regsiter settings must remain stable and unchanged. All configuration register settings, slot configuration settings and window comparison settings should be written prior to setting the ADCEN bit to '1'."]
pub type AdcenW<'a, REG> = crate::BitWriter<'a, REG, Adcen>;
impl<'a, REG> AdcenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the AUDADC module."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Adcen::Dis)
    }
    #[doc = "Enable the AUDADC module."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Adcen::En)
    }
}
#[doc = "This bit enables Repeating Scan Mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rpten {
    #[doc = "0: In Single Scan Mode, the AUDADC will complete a single scan upon each trigger event."]
    SingleScan = 0,
    #[doc = "1: In Repeating Scan Mode, the AUDADC will complete its first scan upon the initial trigger event and all subsequent scans will occur at regular intervals defined by the configuration programmed for the CTTMRA3 timer or the AUDADC-internal timer (see the RPTTRIGSEL field) until the timer is disabled or the AUDADC is disabled. When disabling the AUDADC (setting ADCEN to '0'), the RPTEN bit should be cleared."]
    RepeatingScan = 1,
}
impl From<Rpten> for bool {
    #[inline(always)]
    fn from(variant: Rpten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RPTEN` reader - This bit enables Repeating Scan Mode."]
pub type RptenR = crate::BitReader<Rpten>;
impl RptenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rpten {
        match self.bits {
            false => Rpten::SingleScan,
            true => Rpten::RepeatingScan,
        }
    }
    #[doc = "In Single Scan Mode, the AUDADC will complete a single scan upon each trigger event."]
    #[inline(always)]
    pub fn is_single_scan(&self) -> bool {
        *self == Rpten::SingleScan
    }
    #[doc = "In Repeating Scan Mode, the AUDADC will complete its first scan upon the initial trigger event and all subsequent scans will occur at regular intervals defined by the configuration programmed for the CTTMRA3 timer or the AUDADC-internal timer (see the RPTTRIGSEL field) until the timer is disabled or the AUDADC is disabled. When disabling the AUDADC (setting ADCEN to '0'), the RPTEN bit should be cleared."]
    #[inline(always)]
    pub fn is_repeating_scan(&self) -> bool {
        *self == Rpten::RepeatingScan
    }
}
#[doc = "Field `RPTEN` writer - This bit enables Repeating Scan Mode."]
pub type RptenW<'a, REG> = crate::BitWriter<'a, REG, Rpten>;
impl<'a, REG> RptenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "In Single Scan Mode, the AUDADC will complete a single scan upon each trigger event."]
    #[inline(always)]
    pub fn single_scan(self) -> &'a mut crate::W<REG> {
        self.variant(Rpten::SingleScan)
    }
    #[doc = "In Repeating Scan Mode, the AUDADC will complete its first scan upon the initial trigger event and all subsequent scans will occur at regular intervals defined by the configuration programmed for the CTTMRA3 timer or the AUDADC-internal timer (see the RPTTRIGSEL field) until the timer is disabled or the AUDADC is disabled. When disabling the AUDADC (setting ADCEN to '0'), the RPTEN bit should be cleared."]
    #[inline(always)]
    pub fn repeating_scan(self) -> &'a mut crate::W<REG> {
        self.variant(Rpten::RepeatingScan)
    }
}
#[doc = "Select power mode to enter between active scans.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lpmode {
    #[doc = "0: Low Power Mode 0. Leaves the AUDADC fully powered between scans with minimum latency between a trigger event and sample data collection."]
    Mode0 = 0,
    #[doc = "1: Low Power Mode 1. Powers down all circuity and clocks associated with the AUDADC until the next trigger event. Between scans, the reference buffer requires up to 50us of delay from a scan trigger event before the conversion will commence while operating in this mode."]
    Mode1 = 1,
}
impl From<Lpmode> for bool {
    #[inline(always)]
    fn from(variant: Lpmode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPMODE` reader - Select power mode to enter between active scans."]
pub type LpmodeR = crate::BitReader<Lpmode>;
impl LpmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lpmode {
        match self.bits {
            false => Lpmode::Mode0,
            true => Lpmode::Mode1,
        }
    }
    #[doc = "Low Power Mode 0. Leaves the AUDADC fully powered between scans with minimum latency between a trigger event and sample data collection."]
    #[inline(always)]
    pub fn is_mode0(&self) -> bool {
        *self == Lpmode::Mode0
    }
    #[doc = "Low Power Mode 1. Powers down all circuity and clocks associated with the AUDADC until the next trigger event. Between scans, the reference buffer requires up to 50us of delay from a scan trigger event before the conversion will commence while operating in this mode."]
    #[inline(always)]
    pub fn is_mode1(&self) -> bool {
        *self == Lpmode::Mode1
    }
}
#[doc = "Field `LPMODE` writer - Select power mode to enter between active scans."]
pub type LpmodeW<'a, REG> = crate::BitWriter<'a, REG, Lpmode>;
impl<'a, REG> LpmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low Power Mode 0. Leaves the AUDADC fully powered between scans with minimum latency between a trigger event and sample data collection."]
    #[inline(always)]
    pub fn mode0(self) -> &'a mut crate::W<REG> {
        self.variant(Lpmode::Mode0)
    }
    #[doc = "Low Power Mode 1. Powers down all circuity and clocks associated with the AUDADC until the next trigger event. Between scans, the reference buffer requires up to 50us of delay from a scan trigger event before the conversion will commence while operating in this mode."]
    #[inline(always)]
    pub fn mode1(self) -> &'a mut crate::W<REG> {
        self.variant(Lpmode::Mode1)
    }
}
#[doc = "Clock mode register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ckmode {
    #[doc = "0: Disable the clock between scans for LPMODE0. Set LPCKMODE to 0x1 while configuring the AUDADC."]
    Lpckmode = 0,
    #[doc = "1: Low Latency Clock Mode. When set, HFRC and the adc_clk will remain on while in functioning in LPMODE0."]
    Llckmode = 1,
}
impl From<Ckmode> for bool {
    #[inline(always)]
    fn from(variant: Ckmode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CKMODE` reader - Clock mode register"]
pub type CkmodeR = crate::BitReader<Ckmode>;
impl CkmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ckmode {
        match self.bits {
            false => Ckmode::Lpckmode,
            true => Ckmode::Llckmode,
        }
    }
    #[doc = "Disable the clock between scans for LPMODE0. Set LPCKMODE to 0x1 while configuring the AUDADC."]
    #[inline(always)]
    pub fn is_lpckmode(&self) -> bool {
        *self == Ckmode::Lpckmode
    }
    #[doc = "Low Latency Clock Mode. When set, HFRC and the adc_clk will remain on while in functioning in LPMODE0."]
    #[inline(always)]
    pub fn is_llckmode(&self) -> bool {
        *self == Ckmode::Llckmode
    }
}
#[doc = "Field `CKMODE` writer - Clock mode register"]
pub type CkmodeW<'a, REG> = crate::BitWriter<'a, REG, Ckmode>;
impl<'a, REG> CkmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the clock between scans for LPMODE0. Set LPCKMODE to 0x1 while configuring the AUDADC."]
    #[inline(always)]
    pub fn lpckmode(self) -> &'a mut crate::W<REG> {
        self.variant(Ckmode::Lpckmode)
    }
    #[doc = "Low Latency Clock Mode. When set, HFRC and the adc_clk will remain on while in functioning in LPMODE0."]
    #[inline(always)]
    pub fn llckmode(self) -> &'a mut crate::W<REG> {
        self.variant(Ckmode::Llckmode)
    }
}
#[doc = "Destructive FIFO Read Enable. Setting this will enable FIFO pop upon reading the FIFOPR register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dfiforden {
    #[doc = "0: Destructive Reads are prevented. Reads to the FIFOPR register will not POP an entry off the FIFO."]
    Dis = 0,
    #[doc = "1: Reads to the FIFOPR registger will automatically pop an entry off the FIFO."]
    En = 1,
}
impl From<Dfiforden> for bool {
    #[inline(always)]
    fn from(variant: Dfiforden) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DFIFORDEN` reader - Destructive FIFO Read Enable. Setting this will enable FIFO pop upon reading the FIFOPR register."]
pub type DfifordenR = crate::BitReader<Dfiforden>;
impl DfifordenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dfiforden {
        match self.bits {
            false => Dfiforden::Dis,
            true => Dfiforden::En,
        }
    }
    #[doc = "Destructive Reads are prevented. Reads to the FIFOPR register will not POP an entry off the FIFO."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Dfiforden::Dis
    }
    #[doc = "Reads to the FIFOPR registger will automatically pop an entry off the FIFO."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Dfiforden::En
    }
}
#[doc = "Field `DFIFORDEN` writer - Destructive FIFO Read Enable. Setting this will enable FIFO pop upon reading the FIFOPR register."]
pub type DfifordenW<'a, REG> = crate::BitWriter<'a, REG, Dfiforden>;
impl<'a, REG> DfifordenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Destructive Reads are prevented. Reads to the FIFOPR register will not POP an entry off the FIFO."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Dfiforden::Dis)
    }
    #[doc = "Reads to the FIFOPR registger will automatically pop an entry off the FIFO."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Dfiforden::En)
    }
}
#[doc = "Audio ADC sampling mode. Changes to this control bit are applied when the audio ADC is not performing conversions. This is the only control bit which is properly synchronized to AUDADC operation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sampmode {
    #[doc = "0: Max of 2 low-gain PGA channels configured on slots 0 and 2. In this mode, slots 1 and 3, if enabled, will still consume time but not perform conversions."]
    Lp = 0,
    #[doc = "1: Max of 2 low-gain and 2 high-gain PGA channels. In this mode, conversions will be performed on all enabled slots 0 through 3."]
    Med = 1,
}
impl From<Sampmode> for bool {
    #[inline(always)]
    fn from(variant: Sampmode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SAMPMODE` reader - Audio ADC sampling mode. Changes to this control bit are applied when the audio ADC is not performing conversions. This is the only control bit which is properly synchronized to AUDADC operation."]
pub type SampmodeR = crate::BitReader<Sampmode>;
impl SampmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sampmode {
        match self.bits {
            false => Sampmode::Lp,
            true => Sampmode::Med,
        }
    }
    #[doc = "Max of 2 low-gain PGA channels configured on slots 0 and 2. In this mode, slots 1 and 3, if enabled, will still consume time but not perform conversions."]
    #[inline(always)]
    pub fn is_lp(&self) -> bool {
        *self == Sampmode::Lp
    }
    #[doc = "Max of 2 low-gain and 2 high-gain PGA channels. In this mode, conversions will be performed on all enabled slots 0 through 3."]
    #[inline(always)]
    pub fn is_med(&self) -> bool {
        *self == Sampmode::Med
    }
}
#[doc = "Field `SAMPMODE` writer - Audio ADC sampling mode. Changes to this control bit are applied when the audio ADC is not performing conversions. This is the only control bit which is properly synchronized to AUDADC operation."]
pub type SampmodeW<'a, REG> = crate::BitWriter<'a, REG, Sampmode>;
impl<'a, REG> SampmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Max of 2 low-gain PGA channels configured on slots 0 and 2. In this mode, slots 1 and 3, if enabled, will still consume time but not perform conversions."]
    #[inline(always)]
    pub fn lp(self) -> &'a mut crate::W<REG> {
        self.variant(Sampmode::Lp)
    }
    #[doc = "Max of 2 low-gain and 2 high-gain PGA channels. In this mode, conversions will be performed on all enabled slots 0 through 3."]
    #[inline(always)]
    pub fn med(self) -> &'a mut crate::W<REG> {
        self.variant(Sampmode::Med)
    }
}
#[doc = "Select the AUDADC trigger source.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Trigsel {
    #[doc = "0: Off chip External Trigger0 (ADC_ET0)"]
    Ext0 = 0,
    #[doc = "1: Off chip External Trigger1 (ADC_ET1)"]
    Ext1 = 1,
    #[doc = "2: Off chip External Trigger2 (ADC_ET2)"]
    Ext2 = 2,
    #[doc = "3: Off chip External Trigger3 (ADC_ET3)"]
    Ext3 = 3,
    #[doc = "4: Voltage Comparator Output"]
    Vcomp = 4,
    #[doc = "7: Software Trigger"]
    Swt = 7,
}
impl From<Trigsel> for u8 {
    #[inline(always)]
    fn from(variant: Trigsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Trigsel {
    type Ux = u8;
}
impl crate::IsEnum for Trigsel {}
#[doc = "Field `TRIGSEL` reader - Select the AUDADC trigger source."]
pub type TrigselR = crate::FieldReader<Trigsel>;
impl TrigselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Trigsel> {
        match self.bits {
            0 => Some(Trigsel::Ext0),
            1 => Some(Trigsel::Ext1),
            2 => Some(Trigsel::Ext2),
            3 => Some(Trigsel::Ext3),
            4 => Some(Trigsel::Vcomp),
            7 => Some(Trigsel::Swt),
            _ => None,
        }
    }
    #[doc = "Off chip External Trigger0 (ADC_ET0)"]
    #[inline(always)]
    pub fn is_ext0(&self) -> bool {
        *self == Trigsel::Ext0
    }
    #[doc = "Off chip External Trigger1 (ADC_ET1)"]
    #[inline(always)]
    pub fn is_ext1(&self) -> bool {
        *self == Trigsel::Ext1
    }
    #[doc = "Off chip External Trigger2 (ADC_ET2)"]
    #[inline(always)]
    pub fn is_ext2(&self) -> bool {
        *self == Trigsel::Ext2
    }
    #[doc = "Off chip External Trigger3 (ADC_ET3)"]
    #[inline(always)]
    pub fn is_ext3(&self) -> bool {
        *self == Trigsel::Ext3
    }
    #[doc = "Voltage Comparator Output"]
    #[inline(always)]
    pub fn is_vcomp(&self) -> bool {
        *self == Trigsel::Vcomp
    }
    #[doc = "Software Trigger"]
    #[inline(always)]
    pub fn is_swt(&self) -> bool {
        *self == Trigsel::Swt
    }
}
#[doc = "Field `TRIGSEL` writer - Select the AUDADC trigger source."]
pub type TrigselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Trigsel>;
impl<'a, REG> TrigselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Off chip External Trigger0 (ADC_ET0)"]
    #[inline(always)]
    pub fn ext0(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsel::Ext0)
    }
    #[doc = "Off chip External Trigger1 (ADC_ET1)"]
    #[inline(always)]
    pub fn ext1(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsel::Ext1)
    }
    #[doc = "Off chip External Trigger2 (ADC_ET2)"]
    #[inline(always)]
    pub fn ext2(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsel::Ext2)
    }
    #[doc = "Off chip External Trigger3 (ADC_ET3)"]
    #[inline(always)]
    pub fn ext3(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsel::Ext3)
    }
    #[doc = "Voltage Comparator Output"]
    #[inline(always)]
    pub fn vcomp(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsel::Vcomp)
    }
    #[doc = "Software Trigger"]
    #[inline(always)]
    pub fn swt(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsel::Swt)
    }
}
#[doc = "This bit selects the AUDADC trigger polarity for external off chip triggers.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trigpol {
    #[doc = "0: Trigger on rising edge."]
    RisingEdge = 0,
    #[doc = "1: Trigger on falling edge."]
    FallingEdge = 1,
}
impl From<Trigpol> for bool {
    #[inline(always)]
    fn from(variant: Trigpol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGPOL` reader - This bit selects the AUDADC trigger polarity for external off chip triggers."]
pub type TrigpolR = crate::BitReader<Trigpol>;
impl TrigpolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trigpol {
        match self.bits {
            false => Trigpol::RisingEdge,
            true => Trigpol::FallingEdge,
        }
    }
    #[doc = "Trigger on rising edge."]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == Trigpol::RisingEdge
    }
    #[doc = "Trigger on falling edge."]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == Trigpol::FallingEdge
    }
}
#[doc = "Field `TRIGPOL` writer - This bit selects the AUDADC trigger polarity for external off chip triggers."]
pub type TrigpolW<'a, REG> = crate::BitWriter<'a, REG, Trigpol>;
impl<'a, REG> TrigpolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger on rising edge."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(Trigpol::RisingEdge)
    }
    #[doc = "Trigger on falling edge."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(Trigpol::FallingEdge)
    }
}
#[doc = "This bit selects which periodic trigger to use with RPTEN = 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rpttrigsel {
    #[doc = "0: Trigger from on-chip timer."]
    Tmr = 0,
    #[doc = "1: Trigger from AUDADC-internal timer."]
    Int = 1,
}
impl From<Rpttrigsel> for bool {
    #[inline(always)]
    fn from(variant: Rpttrigsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RPTTRIGSEL` reader - This bit selects which periodic trigger to use with RPTEN = 1."]
pub type RpttrigselR = crate::BitReader<Rpttrigsel>;
impl RpttrigselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rpttrigsel {
        match self.bits {
            false => Rpttrigsel::Tmr,
            true => Rpttrigsel::Int,
        }
    }
    #[doc = "Trigger from on-chip timer."]
    #[inline(always)]
    pub fn is_tmr(&self) -> bool {
        *self == Rpttrigsel::Tmr
    }
    #[doc = "Trigger from AUDADC-internal timer."]
    #[inline(always)]
    pub fn is_int(&self) -> bool {
        *self == Rpttrigsel::Int
    }
}
#[doc = "Field `RPTTRIGSEL` writer - This bit selects which periodic trigger to use with RPTEN = 1."]
pub type RpttrigselW<'a, REG> = crate::BitWriter<'a, REG, Rpttrigsel>;
impl<'a, REG> RpttrigselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger from on-chip timer."]
    #[inline(always)]
    pub fn tmr(self) -> &'a mut crate::W<REG> {
        self.variant(Rpttrigsel::Tmr)
    }
    #[doc = "Trigger from AUDADC-internal timer."]
    #[inline(always)]
    pub fn int(self) -> &'a mut crate::W<REG> {
        self.variant(Rpttrigsel::Int)
    }
}
#[doc = "Select the source and frequency for the AUDADC clock.This field should be left as is because HFRC_48MHz is the only clock selection and it is selected by default. All values enumerated below as 'RSVD' are undefined. The AUDADC controller automatically shuts off the clock in its low power modes. When setting ADCEN to '0', CLKSEL should remain set to 0x0 or 0x1 for proper power down sequencing.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Clksel {
    #[doc = "0: Off mode. The HFRC, HFRC2, or high frequency XTAL clock must be selected for the AUDADC to function. The AUDADC controller automatically shuts off the clock in its low power modes. When setting ADCEN to '0', the CLKSEL should remain set to one of the two clock selects for proper power down sequencing."]
    Off = 0,
    #[doc = "1: HFRC Clock"]
    Hfrc48mhz = 1,
    #[doc = "2: High frequency XTAL (nominally 24.567 MHz, but can vary depending on which XTAL is selected)"]
    Xtalhs24mhz = 2,
    #[doc = "3: HFRC2 Clock"]
    Hfrc2_48mhz = 3,
}
impl From<Clksel> for u8 {
    #[inline(always)]
    fn from(variant: Clksel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Clksel {
    type Ux = u8;
}
impl crate::IsEnum for Clksel {}
#[doc = "Field `CLKSEL` reader - Select the source and frequency for the AUDADC clock.This field should be left as is because HFRC_48MHz is the only clock selection and it is selected by default. All values enumerated below as 'RSVD' are undefined. The AUDADC controller automatically shuts off the clock in its low power modes. When setting ADCEN to '0', CLKSEL should remain set to 0x0 or 0x1 for proper power down sequencing."]
pub type ClkselR = crate::FieldReader<Clksel>;
impl ClkselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Clksel {
        match self.bits {
            0 => Clksel::Off,
            1 => Clksel::Hfrc48mhz,
            2 => Clksel::Xtalhs24mhz,
            3 => Clksel::Hfrc2_48mhz,
            _ => unreachable!(),
        }
    }
    #[doc = "Off mode. The HFRC, HFRC2, or high frequency XTAL clock must be selected for the AUDADC to function. The AUDADC controller automatically shuts off the clock in its low power modes. When setting ADCEN to '0', the CLKSEL should remain set to one of the two clock selects for proper power down sequencing."]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Clksel::Off
    }
    #[doc = "HFRC Clock"]
    #[inline(always)]
    pub fn is_hfrc_48mhz(&self) -> bool {
        *self == Clksel::Hfrc48mhz
    }
    #[doc = "High frequency XTAL (nominally 24.567 MHz, but can vary depending on which XTAL is selected)"]
    #[inline(always)]
    pub fn is_xtalhs_24mhz(&self) -> bool {
        *self == Clksel::Xtalhs24mhz
    }
    #[doc = "HFRC2 Clock"]
    #[inline(always)]
    pub fn is_hfrc2_48mhz(&self) -> bool {
        *self == Clksel::Hfrc2_48mhz
    }
}
#[doc = "Field `CLKSEL` writer - Select the source and frequency for the AUDADC clock.This field should be left as is because HFRC_48MHz is the only clock selection and it is selected by default. All values enumerated below as 'RSVD' are undefined. The AUDADC controller automatically shuts off the clock in its low power modes. When setting ADCEN to '0', CLKSEL should remain set to 0x0 or 0x1 for proper power down sequencing."]
pub type ClkselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Clksel, crate::Safe>;
impl<'a, REG> ClkselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Off mode. The HFRC, HFRC2, or high frequency XTAL clock must be selected for the AUDADC to function. The AUDADC controller automatically shuts off the clock in its low power modes. When setting ADCEN to '0', the CLKSEL should remain set to one of the two clock selects for proper power down sequencing."]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Off)
    }
    #[doc = "HFRC Clock"]
    #[inline(always)]
    pub fn hfrc_48mhz(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Hfrc48mhz)
    }
    #[doc = "High frequency XTAL (nominally 24.567 MHz, but can vary depending on which XTAL is selected)"]
    #[inline(always)]
    pub fn xtalhs_24mhz(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Xtalhs24mhz)
    }
    #[doc = "HFRC2 Clock"]
    #[inline(always)]
    pub fn hfrc2_48mhz(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Hfrc2_48mhz)
    }
}
impl R {
    #[doc = "Bit 0 - This bit enables the AUDADC module. While the AUDADC is enabled, the ADCCFG and SLOT Configuration regsiter settings must remain stable and unchanged. All configuration register settings, slot configuration settings and window comparison settings should be written prior to setting the ADCEN bit to '1'."]
    #[inline(always)]
    pub fn adcen(&self) -> AdcenR {
        AdcenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - This bit enables Repeating Scan Mode."]
    #[inline(always)]
    pub fn rpten(&self) -> RptenR {
        RptenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Select power mode to enter between active scans."]
    #[inline(always)]
    pub fn lpmode(&self) -> LpmodeR {
        LpmodeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Clock mode register"]
    #[inline(always)]
    pub fn ckmode(&self) -> CkmodeR {
        CkmodeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 12 - Destructive FIFO Read Enable. Setting this will enable FIFO pop upon reading the FIFOPR register."]
    #[inline(always)]
    pub fn dfiforden(&self) -> DfifordenR {
        DfifordenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Audio ADC sampling mode. Changes to this control bit are applied when the audio ADC is not performing conversions. This is the only control bit which is properly synchronized to AUDADC operation."]
    #[inline(always)]
    pub fn sampmode(&self) -> SampmodeR {
        SampmodeR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Select the AUDADC trigger source."]
    #[inline(always)]
    pub fn trigsel(&self) -> TrigselR {
        TrigselR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - This bit selects the AUDADC trigger polarity for external off chip triggers."]
    #[inline(always)]
    pub fn trigpol(&self) -> TrigpolR {
        TrigpolR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - This bit selects which periodic trigger to use with RPTEN = 1."]
    #[inline(always)]
    pub fn rpttrigsel(&self) -> RpttrigselR {
        RpttrigselR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Select the source and frequency for the AUDADC clock.This field should be left as is because HFRC_48MHz is the only clock selection and it is selected by default. All values enumerated below as 'RSVD' are undefined. The AUDADC controller automatically shuts off the clock in its low power modes. When setting ADCEN to '0', CLKSEL should remain set to 0x0 or 0x1 for proper power down sequencing."]
    #[inline(always)]
    pub fn clksel(&self) -> ClkselR {
        ClkselR::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - This bit enables the AUDADC module. While the AUDADC is enabled, the ADCCFG and SLOT Configuration regsiter settings must remain stable and unchanged. All configuration register settings, slot configuration settings and window comparison settings should be written prior to setting the ADCEN bit to '1'."]
    #[inline(always)]
    #[must_use]
    pub fn adcen(&mut self) -> AdcenW<CfgSpec> {
        AdcenW::new(self, 0)
    }
    #[doc = "Bit 2 - This bit enables Repeating Scan Mode."]
    #[inline(always)]
    #[must_use]
    pub fn rpten(&mut self) -> RptenW<CfgSpec> {
        RptenW::new(self, 2)
    }
    #[doc = "Bit 3 - Select power mode to enter between active scans."]
    #[inline(always)]
    #[must_use]
    pub fn lpmode(&mut self) -> LpmodeW<CfgSpec> {
        LpmodeW::new(self, 3)
    }
    #[doc = "Bit 4 - Clock mode register"]
    #[inline(always)]
    #[must_use]
    pub fn ckmode(&mut self) -> CkmodeW<CfgSpec> {
        CkmodeW::new(self, 4)
    }
    #[doc = "Bit 12 - Destructive FIFO Read Enable. Setting this will enable FIFO pop upon reading the FIFOPR register."]
    #[inline(always)]
    #[must_use]
    pub fn dfiforden(&mut self) -> DfifordenW<CfgSpec> {
        DfifordenW::new(self, 12)
    }
    #[doc = "Bit 13 - Audio ADC sampling mode. Changes to this control bit are applied when the audio ADC is not performing conversions. This is the only control bit which is properly synchronized to AUDADC operation."]
    #[inline(always)]
    #[must_use]
    pub fn sampmode(&mut self) -> SampmodeW<CfgSpec> {
        SampmodeW::new(self, 13)
    }
    #[doc = "Bits 16:18 - Select the AUDADC trigger source."]
    #[inline(always)]
    #[must_use]
    pub fn trigsel(&mut self) -> TrigselW<CfgSpec> {
        TrigselW::new(self, 16)
    }
    #[doc = "Bit 19 - This bit selects the AUDADC trigger polarity for external off chip triggers."]
    #[inline(always)]
    #[must_use]
    pub fn trigpol(&mut self) -> TrigpolW<CfgSpec> {
        TrigpolW::new(self, 19)
    }
    #[doc = "Bit 20 - This bit selects which periodic trigger to use with RPTEN = 1."]
    #[inline(always)]
    #[must_use]
    pub fn rpttrigsel(&mut self) -> RpttrigselW<CfgSpec> {
        RpttrigselW::new(self, 20)
    }
    #[doc = "Bits 24:25 - Select the source and frequency for the AUDADC clock.This field should be left as is because HFRC_48MHz is the only clock selection and it is selected by default. All values enumerated below as 'RSVD' are undefined. The AUDADC controller automatically shuts off the clock in its low power modes. When setting ADCEN to '0', CLKSEL should remain set to 0x0 or 0x1 for proper power down sequencing."]
    #[inline(always)]
    #[must_use]
    pub fn clksel(&mut self) -> ClkselW<CfgSpec> {
        ClkselW::new(self, 24)
    }
}
#[doc = "The Audio ADC Configuration Register contains the software control for selecting the clock frequency used for the SAR conversions, the trigger polarity, the trigger select, the reference voltage select, the low power mode, the operating mode (single scan per trigger vs. repeating mode) and AUDADC enable.\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
