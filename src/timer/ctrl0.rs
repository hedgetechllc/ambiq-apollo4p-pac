#[doc = "Register `CTRL0` reader"]
pub type R = crate::R<Ctrl0Spec>;
#[doc = "Register `CTRL0` writer"]
pub type W = crate::W<Ctrl0Spec>;
#[doc = "Counter/Timer 0 Enable bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tmr0en {
    #[doc = "0: Counter/Timer 0 Disable."]
    Dis = 0,
    #[doc = "1: Counter/Timer 0 Enable."]
    En = 1,
}
impl From<Tmr0en> for bool {
    #[inline(always)]
    fn from(variant: Tmr0en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMR0EN` reader - Counter/Timer 0 Enable bit."]
pub type Tmr0enR = crate::BitReader<Tmr0en>;
impl Tmr0enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tmr0en {
        match self.bits {
            false => Tmr0en::Dis,
            true => Tmr0en::En,
        }
    }
    #[doc = "Counter/Timer 0 Disable."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Tmr0en::Dis
    }
    #[doc = "Counter/Timer 0 Enable."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Tmr0en::En
    }
}
#[doc = "Field `TMR0EN` writer - Counter/Timer 0 Enable bit."]
pub type Tmr0enW<'a, REG> = crate::BitWriter<'a, REG, Tmr0en>;
impl<'a, REG> Tmr0enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter/Timer 0 Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr0en::Dis)
    }
    #[doc = "Counter/Timer 0 Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr0en::En)
    }
}
#[doc = "Counter/Timer Clear bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tmr0clr {
    #[doc = "1: When written to a 1, the timer will automatically be cleared to its reset state # (0 for count up counter, CMP0 for down counter)"]
    Clear = 1,
    #[doc = "0: Default value set to 0. Timer works normally."]
    Default = 0,
}
impl From<Tmr0clr> for bool {
    #[inline(always)]
    fn from(variant: Tmr0clr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMR0CLR` reader - Counter/Timer Clear bit."]
pub type Tmr0clrR = crate::BitReader<Tmr0clr>;
impl Tmr0clrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tmr0clr {
        match self.bits {
            true => Tmr0clr::Clear,
            false => Tmr0clr::Default,
        }
    }
    #[doc = "When written to a 1, the timer will automatically be cleared to its reset state # (0 for count up counter, CMP0 for down counter)"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Tmr0clr::Clear
    }
    #[doc = "Default value set to 0. Timer works normally."]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == Tmr0clr::Default
    }
}
#[doc = "Field `TMR0CLR` writer - Counter/Timer Clear bit."]
pub type Tmr0clrW<'a, REG> = crate::BitWriter<'a, REG, Tmr0clr>;
impl<'a, REG> Tmr0clrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "When written to a 1, the timer will automatically be cleared to its reset state # (0 for count up counter, CMP0 for down counter)"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr0clr::Clear)
    }
    #[doc = "Default value set to 0. Timer works normally."]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr0clr::Default)
    }
}
#[doc = "Counter/Timer 0 output 0 polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tmr0pol0 {
    #[doc = "0: The polarity of the TMR0OUT0 pin is the same as the timer output."]
    Normal = 0,
    #[doc = "1: The polarity of the TMR0OUT0 pin is the inverse of the timer output."]
    Inverted = 1,
}
impl From<Tmr0pol0> for bool {
    #[inline(always)]
    fn from(variant: Tmr0pol0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMR0POL0` reader - Counter/Timer 0 output 0 polarity."]
pub type Tmr0pol0R = crate::BitReader<Tmr0pol0>;
impl Tmr0pol0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tmr0pol0 {
        match self.bits {
            false => Tmr0pol0::Normal,
            true => Tmr0pol0::Inverted,
        }
    }
    #[doc = "The polarity of the TMR0OUT0 pin is the same as the timer output."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Tmr0pol0::Normal
    }
    #[doc = "The polarity of the TMR0OUT0 pin is the inverse of the timer output."]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == Tmr0pol0::Inverted
    }
}
#[doc = "Field `TMR0POL0` writer - Counter/Timer 0 output 0 polarity."]
pub type Tmr0pol0W<'a, REG> = crate::BitWriter<'a, REG, Tmr0pol0>;
impl<'a, REG> Tmr0pol0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The polarity of the TMR0OUT0 pin is the same as the timer output."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr0pol0::Normal)
    }
    #[doc = "The polarity of the TMR0OUT0 pin is the inverse of the timer output."]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr0pol0::Inverted)
    }
}
#[doc = "Counter/Timer 0 output 1 polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tmr0pol1 {
    #[doc = "0: The polarity of the TMR0OUT1 pin is the same as the timer output."]
    Normal = 0,
    #[doc = "1: The polarity of the TMR0OUT1 pin is the inverse of the timer output."]
    Inverted = 1,
}
impl From<Tmr0pol1> for bool {
    #[inline(always)]
    fn from(variant: Tmr0pol1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMR0POL1` reader - Counter/Timer 0 output 1 polarity."]
pub type Tmr0pol1R = crate::BitReader<Tmr0pol1>;
impl Tmr0pol1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tmr0pol1 {
        match self.bits {
            false => Tmr0pol1::Normal,
            true => Tmr0pol1::Inverted,
        }
    }
    #[doc = "The polarity of the TMR0OUT1 pin is the same as the timer output."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Tmr0pol1::Normal
    }
    #[doc = "The polarity of the TMR0OUT1 pin is the inverse of the timer output."]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == Tmr0pol1::Inverted
    }
}
#[doc = "Field `TMR0POL1` writer - Counter/Timer 0 output 1 polarity."]
pub type Tmr0pol1W<'a, REG> = crate::BitWriter<'a, REG, Tmr0pol1>;
impl<'a, REG> Tmr0pol1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The polarity of the TMR0OUT1 pin is the same as the timer output."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr0pol1::Normal)
    }
    #[doc = "The polarity of the TMR0OUT1 pin is the inverse of the timer output."]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr0pol1::Inverted)
    }
}
#[doc = "Counter/Timer 0 Function Select. For all Functions, CMP0 marks the end of timer cycle, and thus restarts the timer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tmr0fn {
    #[doc = "1: This Mode generates a single edge on OUT0/OUT1 when TIMER value hits CMP0/CMP1 respectively. OUT\\[0\\]=0, counter increments to CMP0, OUT\\[0\\]=1, counter stops. OUT\\[1\\]
follows CMP1."]
    Edge = 1,
    #[doc = "2: This mode is run up counter generating a pulse on CMP. OUT\\[0\\]/OUT\\[1\\]
is pulsed for one source clock period when TIMER matches CMP0/CMP1 respectively. Timer repeats for TMR_LMT iterations."]
    Upcount = 2,
    #[doc = "4: PWM mode. OUT0 and OUT1 are waveforms, and not just one clock pulse. CMP1 dictates the low phase of the output and CMP0 dictates the period. OUT\\[1\\]=~OUT\\[0\\]."]
    Pwm = 4,
    #[doc = "12: Single pattern. OUT0=CMP0\\[TIMER\\], OUT1=CMP1\\[TIMER\\]. LMT field specifies length of pattern. When LMT GT 32 OUT0 and OUT1 is the same 64-bit pattern consisting of concatenated CMP1,CMP0. When LMT LT 32 OUT0 and OUT1 are independent. Both OUT0 and OUT1 can be inverted individually applications with POL0/POL1 = 0x1."]
    Singlepattern = 12,
    #[doc = "13: Repeated pattern. Like SINGLEPATTERN mode, but pattern repeats after reaching LMT."]
    Repeatpattern = 13,
}
impl From<Tmr0fn> for u8 {
    #[inline(always)]
    fn from(variant: Tmr0fn) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tmr0fn {
    type Ux = u8;
}
impl crate::IsEnum for Tmr0fn {}
#[doc = "Field `TMR0FN` reader - Counter/Timer 0 Function Select. For all Functions, CMP0 marks the end of timer cycle, and thus restarts the timer."]
pub type Tmr0fnR = crate::FieldReader<Tmr0fn>;
impl Tmr0fnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Tmr0fn> {
        match self.bits {
            1 => Some(Tmr0fn::Edge),
            2 => Some(Tmr0fn::Upcount),
            4 => Some(Tmr0fn::Pwm),
            12 => Some(Tmr0fn::Singlepattern),
            13 => Some(Tmr0fn::Repeatpattern),
            _ => None,
        }
    }
    #[doc = "This Mode generates a single edge on OUT0/OUT1 when TIMER value hits CMP0/CMP1 respectively. OUT\\[0\\]=0, counter increments to CMP0, OUT\\[0\\]=1, counter stops. OUT\\[1\\]
follows CMP1."]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Tmr0fn::Edge
    }
    #[doc = "This mode is run up counter generating a pulse on CMP. OUT\\[0\\]/OUT\\[1\\]
is pulsed for one source clock period when TIMER matches CMP0/CMP1 respectively. Timer repeats for TMR_LMT iterations."]
    #[inline(always)]
    pub fn is_upcount(&self) -> bool {
        *self == Tmr0fn::Upcount
    }
    #[doc = "PWM mode. OUT0 and OUT1 are waveforms, and not just one clock pulse. CMP1 dictates the low phase of the output and CMP0 dictates the period. OUT\\[1\\]=~OUT\\[0\\]."]
    #[inline(always)]
    pub fn is_pwm(&self) -> bool {
        *self == Tmr0fn::Pwm
    }
    #[doc = "Single pattern. OUT0=CMP0\\[TIMER\\], OUT1=CMP1\\[TIMER\\]. LMT field specifies length of pattern. When LMT GT 32 OUT0 and OUT1 is the same 64-bit pattern consisting of concatenated CMP1,CMP0. When LMT LT 32 OUT0 and OUT1 are independent. Both OUT0 and OUT1 can be inverted individually applications with POL0/POL1 = 0x1."]
    #[inline(always)]
    pub fn is_singlepattern(&self) -> bool {
        *self == Tmr0fn::Singlepattern
    }
    #[doc = "Repeated pattern. Like SINGLEPATTERN mode, but pattern repeats after reaching LMT."]
    #[inline(always)]
    pub fn is_repeatpattern(&self) -> bool {
        *self == Tmr0fn::Repeatpattern
    }
}
#[doc = "Field `TMR0FN` writer - Counter/Timer 0 Function Select. For all Functions, CMP0 marks the end of timer cycle, and thus restarts the timer."]
pub type Tmr0fnW<'a, REG> = crate::FieldWriter<'a, REG, 4, Tmr0fn>;
impl<'a, REG> Tmr0fnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "This Mode generates a single edge on OUT0/OUT1 when TIMER value hits CMP0/CMP1 respectively. OUT\\[0\\]=0, counter increments to CMP0, OUT\\[0\\]=1, counter stops. OUT\\[1\\]
follows CMP1."]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr0fn::Edge)
    }
    #[doc = "This mode is run up counter generating a pulse on CMP. OUT\\[0\\]/OUT\\[1\\]
is pulsed for one source clock period when TIMER matches CMP0/CMP1 respectively. Timer repeats for TMR_LMT iterations."]
    #[inline(always)]
    pub fn upcount(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr0fn::Upcount)
    }
    #[doc = "PWM mode. OUT0 and OUT1 are waveforms, and not just one clock pulse. CMP1 dictates the low phase of the output and CMP0 dictates the period. OUT\\[1\\]=~OUT\\[0\\]."]
    #[inline(always)]
    pub fn pwm(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr0fn::Pwm)
    }
    #[doc = "Single pattern. OUT0=CMP0\\[TIMER\\], OUT1=CMP1\\[TIMER\\]. LMT field specifies length of pattern. When LMT GT 32 OUT0 and OUT1 is the same 64-bit pattern consisting of concatenated CMP1,CMP0. When LMT LT 32 OUT0 and OUT1 are independent. Both OUT0 and OUT1 can be inverted individually applications with POL0/POL1 = 0x1."]
    #[inline(always)]
    pub fn singlepattern(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr0fn::Singlepattern)
    }
    #[doc = "Repeated pattern. Like SINGLEPATTERN mode, but pattern repeats after reaching LMT."]
    #[inline(always)]
    pub fn repeatpattern(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr0fn::Repeatpattern)
    }
}
#[doc = "Counter/Timer 0 Clock Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tmr0clk {
    #[doc = "0: Clock source is the HFRC / 4"]
    HfrcDiv4 = 0,
    #[doc = "1: Clock source is HFRC / 16"]
    HfrcDiv16 = 1,
    #[doc = "2: Clock source is HFRC / 64"]
    HfrcDiv64 = 2,
    #[doc = "3: Clock source is HFRC / 256"]
    HfrcDiv256 = 3,
    #[doc = "4: Clock source is HFRC / 1024"]
    HfrcDiv1024 = 4,
    #[doc = "5: Clock source is HFRC / 4096"]
    HfrcDiv4k = 5,
    #[doc = "6: Clock source is LFRC"]
    Lfrc = 6,
    #[doc = "7: Clock source is LFRC / 2"]
    LfrcDiv2 = 7,
    #[doc = "8: Clock source is LFRC / 32"]
    LfrcDiv32 = 8,
    #[doc = "9: Clock source is LFRC / 1024"]
    LfrcDiv1k = 9,
    #[doc = "10: Clock source is the XT (uncalibrated)."]
    Xt = 10,
    #[doc = "11: Clock source is XT / 2"]
    XtDiv2 = 11,
    #[doc = "12: Clock source is XT / 4"]
    XtDiv4 = 12,
    #[doc = "13: Clock source is XT / 8"]
    XtDiv8 = 13,
    #[doc = "14: Clock source is XT / 16"]
    XtDiv16 = 14,
    #[doc = "15: Clock source is XT / 32"]
    XtDiv32 = 15,
    #[doc = "16: Clock source is XT / 128"]
    XtDiv128 = 16,
    #[doc = "17: Clock source is 100 Hz from the current RTC oscillator."]
    Rtc100hz = 17,
    #[doc = "28: Clock source is Buck VDDC TON pulses."]
    Buckc = 28,
    #[doc = "29: Clock source is Buck VDDF TON pulses."]
    Buckf = 29,
    #[doc = "30: Clock source is Buck VDDS TON pulses."]
    Bucks = 30,
    #[doc = "31: Clock source is Buck VDDC_LV TON pulses."]
    BuckcLv = 31,
    #[doc = "32: Clock source is TIMER 0 Output 0"]
    Tmr00 = 32,
    #[doc = "33: Clock source is TIMER 0 Output 1"]
    Tmr01 = 33,
    #[doc = "34: Clock source is TIMER 1 Output 0"]
    Tmr10 = 34,
    #[doc = "35: Clock source is TIMER 1 Output 1"]
    Tmr11 = 35,
    #[doc = "36: Clock source is TIMER 2 Output 0"]
    Tmr20 = 36,
    #[doc = "37: Clock source is TIMER 2 Output 1"]
    Tmr21 = 37,
    #[doc = "38: Clock source is TIMER 3 Output 0"]
    Tmr30 = 38,
    #[doc = "39: Clock source is TIMER 3 Output 1"]
    Tmr31 = 39,
    #[doc = "40: Clock source is TIMER 4 Output 0"]
    Tmr40 = 40,
    #[doc = "41: Clock source is TIMER 4 Output 1"]
    Tmr41 = 41,
    #[doc = "42: Clock source is TIMER 5 Output 0"]
    Tmr50 = 42,
    #[doc = "43: Clock source is TIMER 5 Output 1"]
    Tmr51 = 43,
    #[doc = "44: Clock source is TIMER 6 Output 0"]
    Tmr60 = 44,
    #[doc = "45: Clock source is TIMER 6 Output 1"]
    Tmr61 = 45,
    #[doc = "46: Clock source is TIMER 7 Output 0"]
    Tmr70 = 46,
    #[doc = "47: Clock source is TIMER 7 Output 1"]
    Tmr71 = 47,
    #[doc = "48: Clock source is TIMER 8 Output 0"]
    Tmr80 = 48,
    #[doc = "49: Clock source is TIMER 8 Output 1"]
    Tmr81 = 49,
    #[doc = "50: Clock source is TIMER 9 Output 0"]
    Tmr90 = 50,
    #[doc = "51: Clock source is TIMER 9 Output 1"]
    Tmr91 = 51,
    #[doc = "52: Clock source is TIMER 10 Output 0"]
    Tmr100 = 52,
    #[doc = "53: Clock source is TIMER 10 Output 1"]
    Tmr101 = 53,
    #[doc = "54: Clock source is TIMER 11 Output 0"]
    Tmr110 = 54,
    #[doc = "55: Clock source is TIMER 11 Output 1"]
    Tmr111 = 55,
    #[doc = "56: Clock source is TIMER 12 Output 0"]
    Tmr120 = 56,
    #[doc = "57: Clock source is TIMER 12 Output 1"]
    Tmr121 = 57,
    #[doc = "58: Clock source is TIMER 13 Output 0"]
    Tmr130 = 58,
    #[doc = "59: Clock source is TIMER 13 Output 1"]
    Tmr131 = 59,
    #[doc = "60: Clock source is TIMER 14 Output 0"]
    Tmr140 = 60,
    #[doc = "61: Clock source is TIMER 14 Output 1"]
    Tmr141 = 61,
    #[doc = "62: Clock source is TIMER 15 Output 0"]
    Tmr150 = 62,
    #[doc = "63: Clock source is TIMER 15 Output 1"]
    Tmr151 = 63,
    #[doc = "128: GPIO #0 is clock source"]
    Gpio0 = 128,
    #[doc = "191: GPIO #63 is clock source"]
    Gpio63 = 191,
    #[doc = "223: GPIO #95 is clock source"]
    Gpio95 = 223,
    #[doc = "255: GPIO #127 is clock source"]
    Gpio127 = 255,
}
impl From<Tmr0clk> for u8 {
    #[inline(always)]
    fn from(variant: Tmr0clk) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tmr0clk {
    type Ux = u8;
}
impl crate::IsEnum for Tmr0clk {}
#[doc = "Field `TMR0CLK` reader - Counter/Timer 0 Clock Select."]
pub type Tmr0clkR = crate::FieldReader<Tmr0clk>;
impl Tmr0clkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Tmr0clk> {
        match self.bits {
            0 => Some(Tmr0clk::HfrcDiv4),
            1 => Some(Tmr0clk::HfrcDiv16),
            2 => Some(Tmr0clk::HfrcDiv64),
            3 => Some(Tmr0clk::HfrcDiv256),
            4 => Some(Tmr0clk::HfrcDiv1024),
            5 => Some(Tmr0clk::HfrcDiv4k),
            6 => Some(Tmr0clk::Lfrc),
            7 => Some(Tmr0clk::LfrcDiv2),
            8 => Some(Tmr0clk::LfrcDiv32),
            9 => Some(Tmr0clk::LfrcDiv1k),
            10 => Some(Tmr0clk::Xt),
            11 => Some(Tmr0clk::XtDiv2),
            12 => Some(Tmr0clk::XtDiv4),
            13 => Some(Tmr0clk::XtDiv8),
            14 => Some(Tmr0clk::XtDiv16),
            15 => Some(Tmr0clk::XtDiv32),
            16 => Some(Tmr0clk::XtDiv128),
            17 => Some(Tmr0clk::Rtc100hz),
            28 => Some(Tmr0clk::Buckc),
            29 => Some(Tmr0clk::Buckf),
            30 => Some(Tmr0clk::Bucks),
            31 => Some(Tmr0clk::BuckcLv),
            32 => Some(Tmr0clk::Tmr00),
            33 => Some(Tmr0clk::Tmr01),
            34 => Some(Tmr0clk::Tmr10),
            35 => Some(Tmr0clk::Tmr11),
            36 => Some(Tmr0clk::Tmr20),
            37 => Some(Tmr0clk::Tmr21),
            38 => Some(Tmr0clk::Tmr30),
            39 => Some(Tmr0clk::Tmr31),
            40 => Some(Tmr0clk::Tmr40),
            41 => Some(Tmr0clk::Tmr41),
            42 => Some(Tmr0clk::Tmr50),
            43 => Some(Tmr0clk::Tmr51),
            44 => Some(Tmr0clk::Tmr60),
            45 => Some(Tmr0clk::Tmr61),
            46 => Some(Tmr0clk::Tmr70),
            47 => Some(Tmr0clk::Tmr71),
            48 => Some(Tmr0clk::Tmr80),
            49 => Some(Tmr0clk::Tmr81),
            50 => Some(Tmr0clk::Tmr90),
            51 => Some(Tmr0clk::Tmr91),
            52 => Some(Tmr0clk::Tmr100),
            53 => Some(Tmr0clk::Tmr101),
            54 => Some(Tmr0clk::Tmr110),
            55 => Some(Tmr0clk::Tmr111),
            56 => Some(Tmr0clk::Tmr120),
            57 => Some(Tmr0clk::Tmr121),
            58 => Some(Tmr0clk::Tmr130),
            59 => Some(Tmr0clk::Tmr131),
            60 => Some(Tmr0clk::Tmr140),
            61 => Some(Tmr0clk::Tmr141),
            62 => Some(Tmr0clk::Tmr150),
            63 => Some(Tmr0clk::Tmr151),
            128 => Some(Tmr0clk::Gpio0),
            191 => Some(Tmr0clk::Gpio63),
            223 => Some(Tmr0clk::Gpio95),
            255 => Some(Tmr0clk::Gpio127),
            _ => None,
        }
    }
    #[doc = "Clock source is the HFRC / 4"]
    #[inline(always)]
    pub fn is_hfrc_div4(&self) -> bool {
        *self == Tmr0clk::HfrcDiv4
    }
    #[doc = "Clock source is HFRC / 16"]
    #[inline(always)]
    pub fn is_hfrc_div16(&self) -> bool {
        *self == Tmr0clk::HfrcDiv16
    }
    #[doc = "Clock source is HFRC / 64"]
    #[inline(always)]
    pub fn is_hfrc_div64(&self) -> bool {
        *self == Tmr0clk::HfrcDiv64
    }
    #[doc = "Clock source is HFRC / 256"]
    #[inline(always)]
    pub fn is_hfrc_div256(&self) -> bool {
        *self == Tmr0clk::HfrcDiv256
    }
    #[doc = "Clock source is HFRC / 1024"]
    #[inline(always)]
    pub fn is_hfrc_div1024(&self) -> bool {
        *self == Tmr0clk::HfrcDiv1024
    }
    #[doc = "Clock source is HFRC / 4096"]
    #[inline(always)]
    pub fn is_hfrc_div4k(&self) -> bool {
        *self == Tmr0clk::HfrcDiv4k
    }
    #[doc = "Clock source is LFRC"]
    #[inline(always)]
    pub fn is_lfrc(&self) -> bool {
        *self == Tmr0clk::Lfrc
    }
    #[doc = "Clock source is LFRC / 2"]
    #[inline(always)]
    pub fn is_lfrc_div2(&self) -> bool {
        *self == Tmr0clk::LfrcDiv2
    }
    #[doc = "Clock source is LFRC / 32"]
    #[inline(always)]
    pub fn is_lfrc_div32(&self) -> bool {
        *self == Tmr0clk::LfrcDiv32
    }
    #[doc = "Clock source is LFRC / 1024"]
    #[inline(always)]
    pub fn is_lfrc_div1k(&self) -> bool {
        *self == Tmr0clk::LfrcDiv1k
    }
    #[doc = "Clock source is the XT (uncalibrated)."]
    #[inline(always)]
    pub fn is_xt(&self) -> bool {
        *self == Tmr0clk::Xt
    }
    #[doc = "Clock source is XT / 2"]
    #[inline(always)]
    pub fn is_xt_div2(&self) -> bool {
        *self == Tmr0clk::XtDiv2
    }
    #[doc = "Clock source is XT / 4"]
    #[inline(always)]
    pub fn is_xt_div4(&self) -> bool {
        *self == Tmr0clk::XtDiv4
    }
    #[doc = "Clock source is XT / 8"]
    #[inline(always)]
    pub fn is_xt_div8(&self) -> bool {
        *self == Tmr0clk::XtDiv8
    }
    #[doc = "Clock source is XT / 16"]
    #[inline(always)]
    pub fn is_xt_div16(&self) -> bool {
        *self == Tmr0clk::XtDiv16
    }
    #[doc = "Clock source is XT / 32"]
    #[inline(always)]
    pub fn is_xt_div32(&self) -> bool {
        *self == Tmr0clk::XtDiv32
    }
    #[doc = "Clock source is XT / 128"]
    #[inline(always)]
    pub fn is_xt_div128(&self) -> bool {
        *self == Tmr0clk::XtDiv128
    }
    #[doc = "Clock source is 100 Hz from the current RTC oscillator."]
    #[inline(always)]
    pub fn is_rtc_100hz(&self) -> bool {
        *self == Tmr0clk::Rtc100hz
    }
    #[doc = "Clock source is Buck VDDC TON pulses."]
    #[inline(always)]
    pub fn is_buckc(&self) -> bool {
        *self == Tmr0clk::Buckc
    }
    #[doc = "Clock source is Buck VDDF TON pulses."]
    #[inline(always)]
    pub fn is_buckf(&self) -> bool {
        *self == Tmr0clk::Buckf
    }
    #[doc = "Clock source is Buck VDDS TON pulses."]
    #[inline(always)]
    pub fn is_bucks(&self) -> bool {
        *self == Tmr0clk::Bucks
    }
    #[doc = "Clock source is Buck VDDC_LV TON pulses."]
    #[inline(always)]
    pub fn is_buckc_lv(&self) -> bool {
        *self == Tmr0clk::BuckcLv
    }
    #[doc = "Clock source is TIMER 0 Output 0"]
    #[inline(always)]
    pub fn is_tmr00(&self) -> bool {
        *self == Tmr0clk::Tmr00
    }
    #[doc = "Clock source is TIMER 0 Output 1"]
    #[inline(always)]
    pub fn is_tmr01(&self) -> bool {
        *self == Tmr0clk::Tmr01
    }
    #[doc = "Clock source is TIMER 1 Output 0"]
    #[inline(always)]
    pub fn is_tmr10(&self) -> bool {
        *self == Tmr0clk::Tmr10
    }
    #[doc = "Clock source is TIMER 1 Output 1"]
    #[inline(always)]
    pub fn is_tmr11(&self) -> bool {
        *self == Tmr0clk::Tmr11
    }
    #[doc = "Clock source is TIMER 2 Output 0"]
    #[inline(always)]
    pub fn is_tmr20(&self) -> bool {
        *self == Tmr0clk::Tmr20
    }
    #[doc = "Clock source is TIMER 2 Output 1"]
    #[inline(always)]
    pub fn is_tmr21(&self) -> bool {
        *self == Tmr0clk::Tmr21
    }
    #[doc = "Clock source is TIMER 3 Output 0"]
    #[inline(always)]
    pub fn is_tmr30(&self) -> bool {
        *self == Tmr0clk::Tmr30
    }
    #[doc = "Clock source is TIMER 3 Output 1"]
    #[inline(always)]
    pub fn is_tmr31(&self) -> bool {
        *self == Tmr0clk::Tmr31
    }
    #[doc = "Clock source is TIMER 4 Output 0"]
    #[inline(always)]
    pub fn is_tmr40(&self) -> bool {
        *self == Tmr0clk::Tmr40
    }
    #[doc = "Clock source is TIMER 4 Output 1"]
    #[inline(always)]
    pub fn is_tmr41(&self) -> bool {
        *self == Tmr0clk::Tmr41
    }
    #[doc = "Clock source is TIMER 5 Output 0"]
    #[inline(always)]
    pub fn is_tmr50(&self) -> bool {
        *self == Tmr0clk::Tmr50
    }
    #[doc = "Clock source is TIMER 5 Output 1"]
    #[inline(always)]
    pub fn is_tmr51(&self) -> bool {
        *self == Tmr0clk::Tmr51
    }
    #[doc = "Clock source is TIMER 6 Output 0"]
    #[inline(always)]
    pub fn is_tmr60(&self) -> bool {
        *self == Tmr0clk::Tmr60
    }
    #[doc = "Clock source is TIMER 6 Output 1"]
    #[inline(always)]
    pub fn is_tmr61(&self) -> bool {
        *self == Tmr0clk::Tmr61
    }
    #[doc = "Clock source is TIMER 7 Output 0"]
    #[inline(always)]
    pub fn is_tmr70(&self) -> bool {
        *self == Tmr0clk::Tmr70
    }
    #[doc = "Clock source is TIMER 7 Output 1"]
    #[inline(always)]
    pub fn is_tmr71(&self) -> bool {
        *self == Tmr0clk::Tmr71
    }
    #[doc = "Clock source is TIMER 8 Output 0"]
    #[inline(always)]
    pub fn is_tmr80(&self) -> bool {
        *self == Tmr0clk::Tmr80
    }
    #[doc = "Clock source is TIMER 8 Output 1"]
    #[inline(always)]
    pub fn is_tmr81(&self) -> bool {
        *self == Tmr0clk::Tmr81
    }
    #[doc = "Clock source is TIMER 9 Output 0"]
    #[inline(always)]
    pub fn is_tmr90(&self) -> bool {
        *self == Tmr0clk::Tmr90
    }
    #[doc = "Clock source is TIMER 9 Output 1"]
    #[inline(always)]
    pub fn is_tmr91(&self) -> bool {
        *self == Tmr0clk::Tmr91
    }
    #[doc = "Clock source is TIMER 10 Output 0"]
    #[inline(always)]
    pub fn is_tmr100(&self) -> bool {
        *self == Tmr0clk::Tmr100
    }
    #[doc = "Clock source is TIMER 10 Output 1"]
    #[inline(always)]
    pub fn is_tmr101(&self) -> bool {
        *self == Tmr0clk::Tmr101
    }
    #[doc = "Clock source is TIMER 11 Output 0"]
    #[inline(always)]
    pub fn is_tmr110(&self) -> bool {
        *self == Tmr0clk::Tmr110
    }
    #[doc = "Clock source is TIMER 11 Output 1"]
    #[inline(always)]
    pub fn is_tmr111(&self) -> bool {
        *self == Tmr0clk::Tmr111
    }
    #[doc = "Clock source is TIMER 12 Output 0"]
    #[inline(always)]
    pub fn is_tmr120(&self) -> bool {
        *self == Tmr0clk::Tmr120
    }
    #[doc = "Clock source is TIMER 12 Output 1"]
    #[inline(always)]
    pub fn is_tmr121(&self) -> bool {
        *self == Tmr0clk::Tmr121
    }
    #[doc = "Clock source is TIMER 13 Output 0"]
    #[inline(always)]
    pub fn is_tmr130(&self) -> bool {
        *self == Tmr0clk::Tmr130
    }
    #[doc = "Clock source is TIMER 13 Output 1"]
    #[inline(always)]
    pub fn is_tmr131(&self) -> bool {
        *self == Tmr0clk::Tmr131
    }
    #[doc = "Clock source is TIMER 14 Output 0"]
    #[inline(always)]
    pub fn is_tmr140(&self) -> bool {
        *self == Tmr0clk::Tmr140
    }
    #[doc = "Clock source is TIMER 14 Output 1"]
    #[inline(always)]
    pub fn is_tmr141(&self) -> bool {
        *self == Tmr0clk::Tmr141
    }
    #[doc = "Clock source is TIMER 15 Output 0"]
    #[inline(always)]
    pub fn is_tmr150(&self) -> bool {
        *self == Tmr0clk::Tmr150
    }
    #[doc = "Clock source is TIMER 15 Output 1"]
    #[inline(always)]
    pub fn is_tmr151(&self) -> bool {
        *self == Tmr0clk::Tmr151
    }
    #[doc = "GPIO #0 is clock source"]
    #[inline(always)]
    pub fn is_gpio0(&self) -> bool {
        *self == Tmr0clk::Gpio0
    }
    #[doc = "GPIO #63 is clock source"]
    #[inline(always)]
    pub fn is_gpio63(&self) -> bool {
        *self == Tmr0clk::Gpio63
    }
    #[doc = "GPIO #95 is clock source"]
    #[inline(always)]
    pub fn is_gpio95(&self) -> bool {
        *self == Tmr0clk::Gpio95
    }
    #[doc = "GPIO #127 is clock source"]
    #[inline(always)]
    pub fn is_gpio127(&self) -> bool {
        *self == Tmr0clk::Gpio127
    }
}
#[doc = "Field `TMR0CLK` writer - Counter/Timer 0 Clock Select."]
pub type Tmr0clkW<'a, REG> = crate::FieldWriter<'a, REG, 8, Tmr0clk>;
impl<'a, REG> Tmr0clkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Clock source is the HFRC / 4"]
    #[inline(always)]
    pub fn hfrc_div4(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr0clk::HfrcDiv4)
    }
    #[doc = "Clock source is HFRC / 16"]
    #[inline(always)]
    pub fn hfrc_div16(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr0clk::HfrcDiv16)
    }
    #[doc = "Clock source is HFRC / 64"]
    #[inline(always)]
    pub fn hfrc_div64(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr0clk::HfrcDiv64)
    }
    #[doc = "Clock source is HFRC / 256"]
    #[inline(always)]
    pub fn hfrc_div256(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr0clk::HfrcDiv256)
    }
    #[doc = "Clock source is HFRC / 1024"]
    #[inline(always)]
    pub fn hfrc_div1024(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr0clk::HfrcDiv1024)
    }
    #[doc = "Clock source is HFRC / 4096"]
    #[inline(always)]
    pub fn hfrc_div4k(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr0clk::HfrcDiv4k)
    }
    #[doc = "Clock source is LFRC"]
    #[inline(always)]
    pub fn lfrc(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr0clk::Lfrc)
    }
    #[doc = "Clock source is LFRC / 2"]
    #[inline(always)]
    pub fn lfrc_div2(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr0clk::LfrcDiv2)
    }
    #[doc = "Clock source is LFRC / 32"]
    #[inline(always)]
    pub fn lfrc_div32(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr0clk::LfrcDiv32)
    }
    #[doc = "Clock source is LFRC / 1024"]
    #[inline(always)]
    pub fn lfrc_div1k(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr0clk::LfrcDiv1k)
    }
    #[doc = "Clock source is the XT (uncalibrated)."]
    #[inline(always)]
    pub fn xt(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr0clk::Xt)
    }
    #[doc = "Clock source is XT / 2"]
    #[inline(always)]
    pub fn xt_div2(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr0clk::XtDiv2)
    }
    #[doc = "Clock source is XT / 4"]
    #[inline(always)]
    pub fn xt_div4(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr0clk::XtDiv4)
    }
    #[doc = "Clock source is XT / 8"]
    #[inline(always)]
    pub fn xt_div8(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr0clk::XtDiv8)
    }
    #[doc = "Clock source is XT / 16"]
    #[inline(always)]
    pub fn xt_div16(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr0clk::XtDiv16)
    }
    #[doc = "Clock source is XT / 32"]
    #[inline(always)]
    pub fn xt_div32(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr0clk::XtDiv32)
    }
    #[doc = "Clock source is XT / 128"]
    #[inline(always)]
    pub fn xt_div128(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr0clk::XtDiv128)
    }
    #[doc = "Clock source is 100 Hz from the current RTC oscillator."]
    #[inline(always)]
    pub fn rtc_100hz(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr0clk::Rtc100hz)
    }
    #[doc = "Clock source is Buck VDDC TON pulses."]
    #[inline(always)]
    pub fn buckc(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr0clk::Buckc)
    }
    #[doc = "Clock source is Buck VDDF TON pulses."]
    #[inline(always)]
    pub fn buckf(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr0clk::Buckf)
    }
    #[doc = "Clock source is Buck VDDS TON pulses."]
    #[inline(always)]
    pub fn bucks(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr0clk::Bucks)
    }
    #[doc = "Clock source is Buck VDDC_LV TON pulses."]
    #[inline(always)]
    pub fn buckc_lv(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr0clk::BuckcLv)
    }
    #[doc = "Clock source is TIMER 0 Output 0"]
    #[inline(always)]
    pub fn tmr00(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr0clk::Tmr00)
    }
    #[doc = "Clock source is TIMER 0 Output 1"]
    #[inline(always)]
    pub fn tmr01(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr0clk::Tmr01)
    }
    #[doc = "Clock source is TIMER 1 Output 0"]
    #[inline(always)]
    pub fn tmr10(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr0clk::Tmr10)
    }
    #[doc = "Clock source is TIMER 1 Output 1"]
    #[inline(always)]
    pub fn tmr11(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr0clk::Tmr11)
    }
    #[doc = "Clock source is TIMER 2 Output 0"]
    #[inline(always)]
    pub fn tmr20(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr0clk::Tmr20)
    }
    #[doc = "Clock source is TIMER 2 Output 1"]
    #[inline(always)]
    pub fn tmr21(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr0clk::Tmr21)
    }
    #[doc = "Clock source is TIMER 3 Output 0"]
    #[inline(always)]
    pub fn tmr30(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr0clk::Tmr30)
    }
    #[doc = "Clock source is TIMER 3 Output 1"]
    #[inline(always)]
    pub fn tmr31(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr0clk::Tmr31)
    }
    #[doc = "Clock source is TIMER 4 Output 0"]
    #[inline(always)]
    pub fn tmr40(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr0clk::Tmr40)
    }
    #[doc = "Clock source is TIMER 4 Output 1"]
    #[inline(always)]
    pub fn tmr41(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr0clk::Tmr41)
    }
    #[doc = "Clock source is TIMER 5 Output 0"]
    #[inline(always)]
    pub fn tmr50(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr0clk::Tmr50)
    }
    #[doc = "Clock source is TIMER 5 Output 1"]
    #[inline(always)]
    pub fn tmr51(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr0clk::Tmr51)
    }
    #[doc = "Clock source is TIMER 6 Output 0"]
    #[inline(always)]
    pub fn tmr60(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr0clk::Tmr60)
    }
    #[doc = "Clock source is TIMER 6 Output 1"]
    #[inline(always)]
    pub fn tmr61(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr0clk::Tmr61)
    }
    #[doc = "Clock source is TIMER 7 Output 0"]
    #[inline(always)]
    pub fn tmr70(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr0clk::Tmr70)
    }
    #[doc = "Clock source is TIMER 7 Output 1"]
    #[inline(always)]
    pub fn tmr71(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr0clk::Tmr71)
    }
    #[doc = "Clock source is TIMER 8 Output 0"]
    #[inline(always)]
    pub fn tmr80(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr0clk::Tmr80)
    }
    #[doc = "Clock source is TIMER 8 Output 1"]
    #[inline(always)]
    pub fn tmr81(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr0clk::Tmr81)
    }
    #[doc = "Clock source is TIMER 9 Output 0"]
    #[inline(always)]
    pub fn tmr90(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr0clk::Tmr90)
    }
    #[doc = "Clock source is TIMER 9 Output 1"]
    #[inline(always)]
    pub fn tmr91(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr0clk::Tmr91)
    }
    #[doc = "Clock source is TIMER 10 Output 0"]
    #[inline(always)]
    pub fn tmr100(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr0clk::Tmr100)
    }
    #[doc = "Clock source is TIMER 10 Output 1"]
    #[inline(always)]
    pub fn tmr101(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr0clk::Tmr101)
    }
    #[doc = "Clock source is TIMER 11 Output 0"]
    #[inline(always)]
    pub fn tmr110(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr0clk::Tmr110)
    }
    #[doc = "Clock source is TIMER 11 Output 1"]
    #[inline(always)]
    pub fn tmr111(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr0clk::Tmr111)
    }
    #[doc = "Clock source is TIMER 12 Output 0"]
    #[inline(always)]
    pub fn tmr120(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr0clk::Tmr120)
    }
    #[doc = "Clock source is TIMER 12 Output 1"]
    #[inline(always)]
    pub fn tmr121(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr0clk::Tmr121)
    }
    #[doc = "Clock source is TIMER 13 Output 0"]
    #[inline(always)]
    pub fn tmr130(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr0clk::Tmr130)
    }
    #[doc = "Clock source is TIMER 13 Output 1"]
    #[inline(always)]
    pub fn tmr131(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr0clk::Tmr131)
    }
    #[doc = "Clock source is TIMER 14 Output 0"]
    #[inline(always)]
    pub fn tmr140(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr0clk::Tmr140)
    }
    #[doc = "Clock source is TIMER 14 Output 1"]
    #[inline(always)]
    pub fn tmr141(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr0clk::Tmr141)
    }
    #[doc = "Clock source is TIMER 15 Output 0"]
    #[inline(always)]
    pub fn tmr150(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr0clk::Tmr150)
    }
    #[doc = "Clock source is TIMER 15 Output 1"]
    #[inline(always)]
    pub fn tmr151(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr0clk::Tmr151)
    }
    #[doc = "GPIO #0 is clock source"]
    #[inline(always)]
    pub fn gpio0(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr0clk::Gpio0)
    }
    #[doc = "GPIO #63 is clock source"]
    #[inline(always)]
    pub fn gpio63(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr0clk::Gpio63)
    }
    #[doc = "GPIO #95 is clock source"]
    #[inline(always)]
    pub fn gpio95(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr0clk::Gpio95)
    }
    #[doc = "GPIO #127 is clock source"]
    #[inline(always)]
    pub fn gpio127(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr0clk::Gpio127)
    }
}
#[doc = "Counter/Timer 0 Trigger Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tmr0tmode {
    #[doc = "0: Trigger not enabled"]
    Dis = 0,
    #[doc = "1: Trigger on rising edge of TRIGSEL source"]
    Rise = 1,
    #[doc = "2: Trigger on falling edge of TRIGSEL source"]
    Fall = 2,
    #[doc = "3: Trigger on either edge of TRIGSEL source"]
    Both = 3,
}
impl From<Tmr0tmode> for u8 {
    #[inline(always)]
    fn from(variant: Tmr0tmode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tmr0tmode {
    type Ux = u8;
}
impl crate::IsEnum for Tmr0tmode {}
#[doc = "Field `TMR0TMODE` reader - Counter/Timer 0 Trigger Mode"]
pub type Tmr0tmodeR = crate::FieldReader<Tmr0tmode>;
impl Tmr0tmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tmr0tmode {
        match self.bits {
            0 => Tmr0tmode::Dis,
            1 => Tmr0tmode::Rise,
            2 => Tmr0tmode::Fall,
            3 => Tmr0tmode::Both,
            _ => unreachable!(),
        }
    }
    #[doc = "Trigger not enabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Tmr0tmode::Dis
    }
    #[doc = "Trigger on rising edge of TRIGSEL source"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == Tmr0tmode::Rise
    }
    #[doc = "Trigger on falling edge of TRIGSEL source"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == Tmr0tmode::Fall
    }
    #[doc = "Trigger on either edge of TRIGSEL source"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == Tmr0tmode::Both
    }
}
#[doc = "Field `TMR0TMODE` writer - Counter/Timer 0 Trigger Mode"]
pub type Tmr0tmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Tmr0tmode, crate::Safe>;
impl<'a, REG> Tmr0tmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Trigger not enabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr0tmode::Dis)
    }
    #[doc = "Trigger on rising edge of TRIGSEL source"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr0tmode::Rise)
    }
    #[doc = "Trigger on falling edge of TRIGSEL source"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr0tmode::Fall)
    }
    #[doc = "Trigger on either edge of TRIGSEL source"]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr0tmode::Both)
    }
}
#[doc = "Field `TMR0LMT` reader - This field decides the number of iterations of Counter/Timer 0. For Single/Repeat Patterns, it indicates number of bits to be shifted out and so, max value is 63."]
pub type Tmr0lmtR = crate::FieldReader;
#[doc = "Field `TMR0LMT` writer - This field decides the number of iterations of Counter/Timer 0. For Single/Repeat Patterns, it indicates number of bits to be shifted out and so, max value is 63."]
pub type Tmr0lmtW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Counter/Timer 0 Enable bit."]
    #[inline(always)]
    pub fn tmr0en(&self) -> Tmr0enR {
        Tmr0enR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Counter/Timer Clear bit."]
    #[inline(always)]
    pub fn tmr0clr(&self) -> Tmr0clrR {
        Tmr0clrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Counter/Timer 0 output 0 polarity."]
    #[inline(always)]
    pub fn tmr0pol0(&self) -> Tmr0pol0R {
        Tmr0pol0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Counter/Timer 0 output 1 polarity."]
    #[inline(always)]
    pub fn tmr0pol1(&self) -> Tmr0pol1R {
        Tmr0pol1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Counter/Timer 0 Function Select. For all Functions, CMP0 marks the end of timer cycle, and thus restarts the timer."]
    #[inline(always)]
    pub fn tmr0fn(&self) -> Tmr0fnR {
        Tmr0fnR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - Counter/Timer 0 Clock Select."]
    #[inline(always)]
    pub fn tmr0clk(&self) -> Tmr0clkR {
        Tmr0clkR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:17 - Counter/Timer 0 Trigger Mode"]
    #[inline(always)]
    pub fn tmr0tmode(&self) -> Tmr0tmodeR {
        Tmr0tmodeR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:31 - This field decides the number of iterations of Counter/Timer 0. For Single/Repeat Patterns, it indicates number of bits to be shifted out and so, max value is 63."]
    #[inline(always)]
    pub fn tmr0lmt(&self) -> Tmr0lmtR {
        Tmr0lmtR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Counter/Timer 0 Enable bit."]
    #[inline(always)]
    #[must_use]
    pub fn tmr0en(&mut self) -> Tmr0enW<Ctrl0Spec> {
        Tmr0enW::new(self, 0)
    }
    #[doc = "Bit 1 - Counter/Timer Clear bit."]
    #[inline(always)]
    #[must_use]
    pub fn tmr0clr(&mut self) -> Tmr0clrW<Ctrl0Spec> {
        Tmr0clrW::new(self, 1)
    }
    #[doc = "Bit 2 - Counter/Timer 0 output 0 polarity."]
    #[inline(always)]
    #[must_use]
    pub fn tmr0pol0(&mut self) -> Tmr0pol0W<Ctrl0Spec> {
        Tmr0pol0W::new(self, 2)
    }
    #[doc = "Bit 3 - Counter/Timer 0 output 1 polarity."]
    #[inline(always)]
    #[must_use]
    pub fn tmr0pol1(&mut self) -> Tmr0pol1W<Ctrl0Spec> {
        Tmr0pol1W::new(self, 3)
    }
    #[doc = "Bits 4:7 - Counter/Timer 0 Function Select. For all Functions, CMP0 marks the end of timer cycle, and thus restarts the timer."]
    #[inline(always)]
    #[must_use]
    pub fn tmr0fn(&mut self) -> Tmr0fnW<Ctrl0Spec> {
        Tmr0fnW::new(self, 4)
    }
    #[doc = "Bits 8:15 - Counter/Timer 0 Clock Select."]
    #[inline(always)]
    #[must_use]
    pub fn tmr0clk(&mut self) -> Tmr0clkW<Ctrl0Spec> {
        Tmr0clkW::new(self, 8)
    }
    #[doc = "Bits 16:17 - Counter/Timer 0 Trigger Mode"]
    #[inline(always)]
    #[must_use]
    pub fn tmr0tmode(&mut self) -> Tmr0tmodeW<Ctrl0Spec> {
        Tmr0tmodeW::new(self, 16)
    }
    #[doc = "Bits 24:31 - This field decides the number of iterations of Counter/Timer 0. For Single/Repeat Patterns, it indicates number of bits to be shifted out and so, max value is 63."]
    #[inline(always)]
    #[must_use]
    pub fn tmr0lmt(&mut self) -> Tmr0lmtW<Ctrl0Spec> {
        Tmr0lmtW::new(self, 24)
    }
}
#[doc = "This includes the Control bit fields for timer 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctrl0Spec;
impl crate::RegisterSpec for Ctrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl0::R`](R) reader structure"]
impl crate::Readable for Ctrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`ctrl0::W`](W) writer structure"]
impl crate::Writable for Ctrl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL0 to value 0"]
impl crate::Resettable for Ctrl0Spec {
    const RESET_VALUE: u32 = 0;
}
