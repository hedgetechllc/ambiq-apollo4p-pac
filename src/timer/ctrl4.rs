#[doc = "Register `CTRL4` reader"]
pub type R = crate::R<Ctrl4Spec>;
#[doc = "Register `CTRL4` writer"]
pub type W = crate::W<Ctrl4Spec>;
#[doc = "Counter/Timer 4 Enable bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tmr4en {
    #[doc = "0: Counter/Timer 4 Disable."]
    Dis = 0,
    #[doc = "1: Counter/Timer 4 Enable."]
    En = 1,
}
impl From<Tmr4en> for bool {
    #[inline(always)]
    fn from(variant: Tmr4en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMR4EN` reader - Counter/Timer 4 Enable bit."]
pub type Tmr4enR = crate::BitReader<Tmr4en>;
impl Tmr4enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tmr4en {
        match self.bits {
            false => Tmr4en::Dis,
            true => Tmr4en::En,
        }
    }
    #[doc = "Counter/Timer 4 Disable."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Tmr4en::Dis
    }
    #[doc = "Counter/Timer 4 Enable."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Tmr4en::En
    }
}
#[doc = "Field `TMR4EN` writer - Counter/Timer 4 Enable bit."]
pub type Tmr4enW<'a, REG> = crate::BitWriter<'a, REG, Tmr4en>;
impl<'a, REG> Tmr4enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter/Timer 4 Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr4en::Dis)
    }
    #[doc = "Counter/Timer 4 Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr4en::En)
    }
}
#[doc = "Counter/Timer Clear bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tmr4clr {
    #[doc = "1: When written to a 1, the timer will automatically be cleared to its reset state # (0 for count up counter, CMP0 for down counter)"]
    Clear = 1,
    #[doc = "0: Default value set to 0. Timer works normally."]
    Default = 0,
}
impl From<Tmr4clr> for bool {
    #[inline(always)]
    fn from(variant: Tmr4clr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMR4CLR` reader - Counter/Timer Clear bit."]
pub type Tmr4clrR = crate::BitReader<Tmr4clr>;
impl Tmr4clrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tmr4clr {
        match self.bits {
            true => Tmr4clr::Clear,
            false => Tmr4clr::Default,
        }
    }
    #[doc = "When written to a 1, the timer will automatically be cleared to its reset state # (0 for count up counter, CMP0 for down counter)"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Tmr4clr::Clear
    }
    #[doc = "Default value set to 0. Timer works normally."]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == Tmr4clr::Default
    }
}
#[doc = "Field `TMR4CLR` writer - Counter/Timer Clear bit."]
pub type Tmr4clrW<'a, REG> = crate::BitWriter<'a, REG, Tmr4clr>;
impl<'a, REG> Tmr4clrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "When written to a 1, the timer will automatically be cleared to its reset state # (0 for count up counter, CMP0 for down counter)"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr4clr::Clear)
    }
    #[doc = "Default value set to 0. Timer works normally."]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr4clr::Default)
    }
}
#[doc = "Counter/Timer 4 output 0 polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tmr4pol0 {
    #[doc = "0: The polarity of the TMR4OUT0 pin is the same as the timer output."]
    Normal = 0,
    #[doc = "1: The polarity of the TMR4OUT0 pin is the inverse of the timer output."]
    Inverted = 1,
}
impl From<Tmr4pol0> for bool {
    #[inline(always)]
    fn from(variant: Tmr4pol0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMR4POL0` reader - Counter/Timer 4 output 0 polarity."]
pub type Tmr4pol0R = crate::BitReader<Tmr4pol0>;
impl Tmr4pol0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tmr4pol0 {
        match self.bits {
            false => Tmr4pol0::Normal,
            true => Tmr4pol0::Inverted,
        }
    }
    #[doc = "The polarity of the TMR4OUT0 pin is the same as the timer output."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Tmr4pol0::Normal
    }
    #[doc = "The polarity of the TMR4OUT0 pin is the inverse of the timer output."]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == Tmr4pol0::Inverted
    }
}
#[doc = "Field `TMR4POL0` writer - Counter/Timer 4 output 0 polarity."]
pub type Tmr4pol0W<'a, REG> = crate::BitWriter<'a, REG, Tmr4pol0>;
impl<'a, REG> Tmr4pol0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The polarity of the TMR4OUT0 pin is the same as the timer output."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr4pol0::Normal)
    }
    #[doc = "The polarity of the TMR4OUT0 pin is the inverse of the timer output."]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr4pol0::Inverted)
    }
}
#[doc = "Counter/Timer 4 output 1 polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tmr4pol1 {
    #[doc = "0: The polarity of the TMR4OUT1 pin is the same as the timer output."]
    Normal = 0,
    #[doc = "1: The polarity of the TMR4OUT1 pin is the inverse of the timer output."]
    Inverted = 1,
}
impl From<Tmr4pol1> for bool {
    #[inline(always)]
    fn from(variant: Tmr4pol1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMR4POL1` reader - Counter/Timer 4 output 1 polarity."]
pub type Tmr4pol1R = crate::BitReader<Tmr4pol1>;
impl Tmr4pol1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tmr4pol1 {
        match self.bits {
            false => Tmr4pol1::Normal,
            true => Tmr4pol1::Inverted,
        }
    }
    #[doc = "The polarity of the TMR4OUT1 pin is the same as the timer output."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Tmr4pol1::Normal
    }
    #[doc = "The polarity of the TMR4OUT1 pin is the inverse of the timer output."]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == Tmr4pol1::Inverted
    }
}
#[doc = "Field `TMR4POL1` writer - Counter/Timer 4 output 1 polarity."]
pub type Tmr4pol1W<'a, REG> = crate::BitWriter<'a, REG, Tmr4pol1>;
impl<'a, REG> Tmr4pol1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The polarity of the TMR4OUT1 pin is the same as the timer output."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr4pol1::Normal)
    }
    #[doc = "The polarity of the TMR4OUT1 pin is the inverse of the timer output."]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr4pol1::Inverted)
    }
}
#[doc = "Counter/Timer 4 Function Select. For all Functions, CMP0 marks the end of timer cycle, and thus restarts the timer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tmr4fn {
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
impl From<Tmr4fn> for u8 {
    #[inline(always)]
    fn from(variant: Tmr4fn) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tmr4fn {
    type Ux = u8;
}
impl crate::IsEnum for Tmr4fn {}
#[doc = "Field `TMR4FN` reader - Counter/Timer 4 Function Select. For all Functions, CMP0 marks the end of timer cycle, and thus restarts the timer."]
pub type Tmr4fnR = crate::FieldReader<Tmr4fn>;
impl Tmr4fnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Tmr4fn> {
        match self.bits {
            1 => Some(Tmr4fn::Edge),
            2 => Some(Tmr4fn::Upcount),
            4 => Some(Tmr4fn::Pwm),
            12 => Some(Tmr4fn::Singlepattern),
            13 => Some(Tmr4fn::Repeatpattern),
            _ => None,
        }
    }
    #[doc = "This Mode generates a single edge on OUT0/OUT1 when TIMER value hits CMP0/CMP1 respectively. OUT\\[0\\]=0, counter increments to CMP0, OUT\\[0\\]=1, counter stops. OUT\\[1\\]
follows CMP1."]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Tmr4fn::Edge
    }
    #[doc = "This mode is run up counter generating a pulse on CMP. OUT\\[0\\]/OUT\\[1\\]
is pulsed for one source clock period when TIMER matches CMP0/CMP1 respectively. Timer repeats for TMR_LMT iterations."]
    #[inline(always)]
    pub fn is_upcount(&self) -> bool {
        *self == Tmr4fn::Upcount
    }
    #[doc = "PWM mode. OUT0 and OUT1 are waveforms, and not just one clock pulse. CMP1 dictates the low phase of the output and CMP0 dictates the period. OUT\\[1\\]=~OUT\\[0\\]."]
    #[inline(always)]
    pub fn is_pwm(&self) -> bool {
        *self == Tmr4fn::Pwm
    }
    #[doc = "Single pattern. OUT0=CMP0\\[TIMER\\], OUT1=CMP1\\[TIMER\\]. LMT field specifies length of pattern. When LMT GT 32 OUT0 and OUT1 is the same 64-bit pattern consisting of concatenated CMP1,CMP0. When LMT LT 32 OUT0 and OUT1 are independent. Both OUT0 and OUT1 can be inverted individually applications with POL0/POL1 = 0x1."]
    #[inline(always)]
    pub fn is_singlepattern(&self) -> bool {
        *self == Tmr4fn::Singlepattern
    }
    #[doc = "Repeated pattern. Like SINGLEPATTERN mode, but pattern repeats after reaching LMT."]
    #[inline(always)]
    pub fn is_repeatpattern(&self) -> bool {
        *self == Tmr4fn::Repeatpattern
    }
}
#[doc = "Field `TMR4FN` writer - Counter/Timer 4 Function Select. For all Functions, CMP0 marks the end of timer cycle, and thus restarts the timer."]
pub type Tmr4fnW<'a, REG> = crate::FieldWriter<'a, REG, 4, Tmr4fn>;
impl<'a, REG> Tmr4fnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "This Mode generates a single edge on OUT0/OUT1 when TIMER value hits CMP0/CMP1 respectively. OUT\\[0\\]=0, counter increments to CMP0, OUT\\[0\\]=1, counter stops. OUT\\[1\\]
follows CMP1."]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr4fn::Edge)
    }
    #[doc = "This mode is run up counter generating a pulse on CMP. OUT\\[0\\]/OUT\\[1\\]
is pulsed for one source clock period when TIMER matches CMP0/CMP1 respectively. Timer repeats for TMR_LMT iterations."]
    #[inline(always)]
    pub fn upcount(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr4fn::Upcount)
    }
    #[doc = "PWM mode. OUT0 and OUT1 are waveforms, and not just one clock pulse. CMP1 dictates the low phase of the output and CMP0 dictates the period. OUT\\[1\\]=~OUT\\[0\\]."]
    #[inline(always)]
    pub fn pwm(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr4fn::Pwm)
    }
    #[doc = "Single pattern. OUT0=CMP0\\[TIMER\\], OUT1=CMP1\\[TIMER\\]. LMT field specifies length of pattern. When LMT GT 32 OUT0 and OUT1 is the same 64-bit pattern consisting of concatenated CMP1,CMP0. When LMT LT 32 OUT0 and OUT1 are independent. Both OUT0 and OUT1 can be inverted individually applications with POL0/POL1 = 0x1."]
    #[inline(always)]
    pub fn singlepattern(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr4fn::Singlepattern)
    }
    #[doc = "Repeated pattern. Like SINGLEPATTERN mode, but pattern repeats after reaching LMT."]
    #[inline(always)]
    pub fn repeatpattern(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr4fn::Repeatpattern)
    }
}
#[doc = "Counter/Timer 4 Clock Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tmr4clk {
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
impl From<Tmr4clk> for u8 {
    #[inline(always)]
    fn from(variant: Tmr4clk) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tmr4clk {
    type Ux = u8;
}
impl crate::IsEnum for Tmr4clk {}
#[doc = "Field `TMR4CLK` reader - Counter/Timer 4 Clock Select."]
pub type Tmr4clkR = crate::FieldReader<Tmr4clk>;
impl Tmr4clkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Tmr4clk> {
        match self.bits {
            0 => Some(Tmr4clk::HfrcDiv4),
            1 => Some(Tmr4clk::HfrcDiv16),
            2 => Some(Tmr4clk::HfrcDiv64),
            3 => Some(Tmr4clk::HfrcDiv256),
            4 => Some(Tmr4clk::HfrcDiv1024),
            5 => Some(Tmr4clk::HfrcDiv4k),
            6 => Some(Tmr4clk::Lfrc),
            7 => Some(Tmr4clk::LfrcDiv2),
            8 => Some(Tmr4clk::LfrcDiv32),
            9 => Some(Tmr4clk::LfrcDiv1k),
            10 => Some(Tmr4clk::Xt),
            11 => Some(Tmr4clk::XtDiv2),
            12 => Some(Tmr4clk::XtDiv4),
            13 => Some(Tmr4clk::XtDiv8),
            14 => Some(Tmr4clk::XtDiv16),
            15 => Some(Tmr4clk::XtDiv32),
            16 => Some(Tmr4clk::XtDiv128),
            17 => Some(Tmr4clk::Rtc100hz),
            28 => Some(Tmr4clk::Buckc),
            29 => Some(Tmr4clk::Buckf),
            30 => Some(Tmr4clk::Bucks),
            31 => Some(Tmr4clk::BuckcLv),
            32 => Some(Tmr4clk::Tmr00),
            33 => Some(Tmr4clk::Tmr01),
            34 => Some(Tmr4clk::Tmr10),
            35 => Some(Tmr4clk::Tmr11),
            36 => Some(Tmr4clk::Tmr20),
            37 => Some(Tmr4clk::Tmr21),
            38 => Some(Tmr4clk::Tmr30),
            39 => Some(Tmr4clk::Tmr31),
            40 => Some(Tmr4clk::Tmr40),
            41 => Some(Tmr4clk::Tmr41),
            42 => Some(Tmr4clk::Tmr50),
            43 => Some(Tmr4clk::Tmr51),
            44 => Some(Tmr4clk::Tmr60),
            45 => Some(Tmr4clk::Tmr61),
            46 => Some(Tmr4clk::Tmr70),
            47 => Some(Tmr4clk::Tmr71),
            48 => Some(Tmr4clk::Tmr80),
            49 => Some(Tmr4clk::Tmr81),
            50 => Some(Tmr4clk::Tmr90),
            51 => Some(Tmr4clk::Tmr91),
            52 => Some(Tmr4clk::Tmr100),
            53 => Some(Tmr4clk::Tmr101),
            54 => Some(Tmr4clk::Tmr110),
            55 => Some(Tmr4clk::Tmr111),
            56 => Some(Tmr4clk::Tmr120),
            57 => Some(Tmr4clk::Tmr121),
            58 => Some(Tmr4clk::Tmr130),
            59 => Some(Tmr4clk::Tmr131),
            60 => Some(Tmr4clk::Tmr140),
            61 => Some(Tmr4clk::Tmr141),
            62 => Some(Tmr4clk::Tmr150),
            63 => Some(Tmr4clk::Tmr151),
            128 => Some(Tmr4clk::Gpio0),
            191 => Some(Tmr4clk::Gpio63),
            223 => Some(Tmr4clk::Gpio95),
            255 => Some(Tmr4clk::Gpio127),
            _ => None,
        }
    }
    #[doc = "Clock source is the HFRC / 4"]
    #[inline(always)]
    pub fn is_hfrc_div4(&self) -> bool {
        *self == Tmr4clk::HfrcDiv4
    }
    #[doc = "Clock source is HFRC / 16"]
    #[inline(always)]
    pub fn is_hfrc_div16(&self) -> bool {
        *self == Tmr4clk::HfrcDiv16
    }
    #[doc = "Clock source is HFRC / 64"]
    #[inline(always)]
    pub fn is_hfrc_div64(&self) -> bool {
        *self == Tmr4clk::HfrcDiv64
    }
    #[doc = "Clock source is HFRC / 256"]
    #[inline(always)]
    pub fn is_hfrc_div256(&self) -> bool {
        *self == Tmr4clk::HfrcDiv256
    }
    #[doc = "Clock source is HFRC / 1024"]
    #[inline(always)]
    pub fn is_hfrc_div1024(&self) -> bool {
        *self == Tmr4clk::HfrcDiv1024
    }
    #[doc = "Clock source is HFRC / 4096"]
    #[inline(always)]
    pub fn is_hfrc_div4k(&self) -> bool {
        *self == Tmr4clk::HfrcDiv4k
    }
    #[doc = "Clock source is LFRC"]
    #[inline(always)]
    pub fn is_lfrc(&self) -> bool {
        *self == Tmr4clk::Lfrc
    }
    #[doc = "Clock source is LFRC / 2"]
    #[inline(always)]
    pub fn is_lfrc_div2(&self) -> bool {
        *self == Tmr4clk::LfrcDiv2
    }
    #[doc = "Clock source is LFRC / 32"]
    #[inline(always)]
    pub fn is_lfrc_div32(&self) -> bool {
        *self == Tmr4clk::LfrcDiv32
    }
    #[doc = "Clock source is LFRC / 1024"]
    #[inline(always)]
    pub fn is_lfrc_div1k(&self) -> bool {
        *self == Tmr4clk::LfrcDiv1k
    }
    #[doc = "Clock source is the XT (uncalibrated)."]
    #[inline(always)]
    pub fn is_xt(&self) -> bool {
        *self == Tmr4clk::Xt
    }
    #[doc = "Clock source is XT / 2"]
    #[inline(always)]
    pub fn is_xt_div2(&self) -> bool {
        *self == Tmr4clk::XtDiv2
    }
    #[doc = "Clock source is XT / 4"]
    #[inline(always)]
    pub fn is_xt_div4(&self) -> bool {
        *self == Tmr4clk::XtDiv4
    }
    #[doc = "Clock source is XT / 8"]
    #[inline(always)]
    pub fn is_xt_div8(&self) -> bool {
        *self == Tmr4clk::XtDiv8
    }
    #[doc = "Clock source is XT / 16"]
    #[inline(always)]
    pub fn is_xt_div16(&self) -> bool {
        *self == Tmr4clk::XtDiv16
    }
    #[doc = "Clock source is XT / 32"]
    #[inline(always)]
    pub fn is_xt_div32(&self) -> bool {
        *self == Tmr4clk::XtDiv32
    }
    #[doc = "Clock source is XT / 128"]
    #[inline(always)]
    pub fn is_xt_div128(&self) -> bool {
        *self == Tmr4clk::XtDiv128
    }
    #[doc = "Clock source is 100 Hz from the current RTC oscillator."]
    #[inline(always)]
    pub fn is_rtc_100hz(&self) -> bool {
        *self == Tmr4clk::Rtc100hz
    }
    #[doc = "Clock source is Buck VDDC TON pulses."]
    #[inline(always)]
    pub fn is_buckc(&self) -> bool {
        *self == Tmr4clk::Buckc
    }
    #[doc = "Clock source is Buck VDDF TON pulses."]
    #[inline(always)]
    pub fn is_buckf(&self) -> bool {
        *self == Tmr4clk::Buckf
    }
    #[doc = "Clock source is Buck VDDS TON pulses."]
    #[inline(always)]
    pub fn is_bucks(&self) -> bool {
        *self == Tmr4clk::Bucks
    }
    #[doc = "Clock source is Buck VDDC_LV TON pulses."]
    #[inline(always)]
    pub fn is_buckc_lv(&self) -> bool {
        *self == Tmr4clk::BuckcLv
    }
    #[doc = "Clock source is TIMER 0 Output 0"]
    #[inline(always)]
    pub fn is_tmr00(&self) -> bool {
        *self == Tmr4clk::Tmr00
    }
    #[doc = "Clock source is TIMER 0 Output 1"]
    #[inline(always)]
    pub fn is_tmr01(&self) -> bool {
        *self == Tmr4clk::Tmr01
    }
    #[doc = "Clock source is TIMER 1 Output 0"]
    #[inline(always)]
    pub fn is_tmr10(&self) -> bool {
        *self == Tmr4clk::Tmr10
    }
    #[doc = "Clock source is TIMER 1 Output 1"]
    #[inline(always)]
    pub fn is_tmr11(&self) -> bool {
        *self == Tmr4clk::Tmr11
    }
    #[doc = "Clock source is TIMER 2 Output 0"]
    #[inline(always)]
    pub fn is_tmr20(&self) -> bool {
        *self == Tmr4clk::Tmr20
    }
    #[doc = "Clock source is TIMER 2 Output 1"]
    #[inline(always)]
    pub fn is_tmr21(&self) -> bool {
        *self == Tmr4clk::Tmr21
    }
    #[doc = "Clock source is TIMER 3 Output 0"]
    #[inline(always)]
    pub fn is_tmr30(&self) -> bool {
        *self == Tmr4clk::Tmr30
    }
    #[doc = "Clock source is TIMER 3 Output 1"]
    #[inline(always)]
    pub fn is_tmr31(&self) -> bool {
        *self == Tmr4clk::Tmr31
    }
    #[doc = "Clock source is TIMER 4 Output 0"]
    #[inline(always)]
    pub fn is_tmr40(&self) -> bool {
        *self == Tmr4clk::Tmr40
    }
    #[doc = "Clock source is TIMER 4 Output 1"]
    #[inline(always)]
    pub fn is_tmr41(&self) -> bool {
        *self == Tmr4clk::Tmr41
    }
    #[doc = "Clock source is TIMER 5 Output 0"]
    #[inline(always)]
    pub fn is_tmr50(&self) -> bool {
        *self == Tmr4clk::Tmr50
    }
    #[doc = "Clock source is TIMER 5 Output 1"]
    #[inline(always)]
    pub fn is_tmr51(&self) -> bool {
        *self == Tmr4clk::Tmr51
    }
    #[doc = "Clock source is TIMER 6 Output 0"]
    #[inline(always)]
    pub fn is_tmr60(&self) -> bool {
        *self == Tmr4clk::Tmr60
    }
    #[doc = "Clock source is TIMER 6 Output 1"]
    #[inline(always)]
    pub fn is_tmr61(&self) -> bool {
        *self == Tmr4clk::Tmr61
    }
    #[doc = "Clock source is TIMER 7 Output 0"]
    #[inline(always)]
    pub fn is_tmr70(&self) -> bool {
        *self == Tmr4clk::Tmr70
    }
    #[doc = "Clock source is TIMER 7 Output 1"]
    #[inline(always)]
    pub fn is_tmr71(&self) -> bool {
        *self == Tmr4clk::Tmr71
    }
    #[doc = "Clock source is TIMER 8 Output 0"]
    #[inline(always)]
    pub fn is_tmr80(&self) -> bool {
        *self == Tmr4clk::Tmr80
    }
    #[doc = "Clock source is TIMER 8 Output 1"]
    #[inline(always)]
    pub fn is_tmr81(&self) -> bool {
        *self == Tmr4clk::Tmr81
    }
    #[doc = "Clock source is TIMER 9 Output 0"]
    #[inline(always)]
    pub fn is_tmr90(&self) -> bool {
        *self == Tmr4clk::Tmr90
    }
    #[doc = "Clock source is TIMER 9 Output 1"]
    #[inline(always)]
    pub fn is_tmr91(&self) -> bool {
        *self == Tmr4clk::Tmr91
    }
    #[doc = "Clock source is TIMER 10 Output 0"]
    #[inline(always)]
    pub fn is_tmr100(&self) -> bool {
        *self == Tmr4clk::Tmr100
    }
    #[doc = "Clock source is TIMER 10 Output 1"]
    #[inline(always)]
    pub fn is_tmr101(&self) -> bool {
        *self == Tmr4clk::Tmr101
    }
    #[doc = "Clock source is TIMER 11 Output 0"]
    #[inline(always)]
    pub fn is_tmr110(&self) -> bool {
        *self == Tmr4clk::Tmr110
    }
    #[doc = "Clock source is TIMER 11 Output 1"]
    #[inline(always)]
    pub fn is_tmr111(&self) -> bool {
        *self == Tmr4clk::Tmr111
    }
    #[doc = "Clock source is TIMER 12 Output 0"]
    #[inline(always)]
    pub fn is_tmr120(&self) -> bool {
        *self == Tmr4clk::Tmr120
    }
    #[doc = "Clock source is TIMER 12 Output 1"]
    #[inline(always)]
    pub fn is_tmr121(&self) -> bool {
        *self == Tmr4clk::Tmr121
    }
    #[doc = "Clock source is TIMER 13 Output 0"]
    #[inline(always)]
    pub fn is_tmr130(&self) -> bool {
        *self == Tmr4clk::Tmr130
    }
    #[doc = "Clock source is TIMER 13 Output 1"]
    #[inline(always)]
    pub fn is_tmr131(&self) -> bool {
        *self == Tmr4clk::Tmr131
    }
    #[doc = "Clock source is TIMER 14 Output 0"]
    #[inline(always)]
    pub fn is_tmr140(&self) -> bool {
        *self == Tmr4clk::Tmr140
    }
    #[doc = "Clock source is TIMER 14 Output 1"]
    #[inline(always)]
    pub fn is_tmr141(&self) -> bool {
        *self == Tmr4clk::Tmr141
    }
    #[doc = "Clock source is TIMER 15 Output 0"]
    #[inline(always)]
    pub fn is_tmr150(&self) -> bool {
        *self == Tmr4clk::Tmr150
    }
    #[doc = "Clock source is TIMER 15 Output 1"]
    #[inline(always)]
    pub fn is_tmr151(&self) -> bool {
        *self == Tmr4clk::Tmr151
    }
    #[doc = "GPIO #0 is clock source"]
    #[inline(always)]
    pub fn is_gpio0(&self) -> bool {
        *self == Tmr4clk::Gpio0
    }
    #[doc = "GPIO #63 is clock source"]
    #[inline(always)]
    pub fn is_gpio63(&self) -> bool {
        *self == Tmr4clk::Gpio63
    }
    #[doc = "GPIO #95 is clock source"]
    #[inline(always)]
    pub fn is_gpio95(&self) -> bool {
        *self == Tmr4clk::Gpio95
    }
    #[doc = "GPIO #127 is clock source"]
    #[inline(always)]
    pub fn is_gpio127(&self) -> bool {
        *self == Tmr4clk::Gpio127
    }
}
#[doc = "Field `TMR4CLK` writer - Counter/Timer 4 Clock Select."]
pub type Tmr4clkW<'a, REG> = crate::FieldWriter<'a, REG, 8, Tmr4clk>;
impl<'a, REG> Tmr4clkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Clock source is the HFRC / 4"]
    #[inline(always)]
    pub fn hfrc_div4(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr4clk::HfrcDiv4)
    }
    #[doc = "Clock source is HFRC / 16"]
    #[inline(always)]
    pub fn hfrc_div16(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr4clk::HfrcDiv16)
    }
    #[doc = "Clock source is HFRC / 64"]
    #[inline(always)]
    pub fn hfrc_div64(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr4clk::HfrcDiv64)
    }
    #[doc = "Clock source is HFRC / 256"]
    #[inline(always)]
    pub fn hfrc_div256(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr4clk::HfrcDiv256)
    }
    #[doc = "Clock source is HFRC / 1024"]
    #[inline(always)]
    pub fn hfrc_div1024(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr4clk::HfrcDiv1024)
    }
    #[doc = "Clock source is HFRC / 4096"]
    #[inline(always)]
    pub fn hfrc_div4k(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr4clk::HfrcDiv4k)
    }
    #[doc = "Clock source is LFRC"]
    #[inline(always)]
    pub fn lfrc(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr4clk::Lfrc)
    }
    #[doc = "Clock source is LFRC / 2"]
    #[inline(always)]
    pub fn lfrc_div2(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr4clk::LfrcDiv2)
    }
    #[doc = "Clock source is LFRC / 32"]
    #[inline(always)]
    pub fn lfrc_div32(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr4clk::LfrcDiv32)
    }
    #[doc = "Clock source is LFRC / 1024"]
    #[inline(always)]
    pub fn lfrc_div1k(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr4clk::LfrcDiv1k)
    }
    #[doc = "Clock source is the XT (uncalibrated)."]
    #[inline(always)]
    pub fn xt(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr4clk::Xt)
    }
    #[doc = "Clock source is XT / 2"]
    #[inline(always)]
    pub fn xt_div2(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr4clk::XtDiv2)
    }
    #[doc = "Clock source is XT / 4"]
    #[inline(always)]
    pub fn xt_div4(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr4clk::XtDiv4)
    }
    #[doc = "Clock source is XT / 8"]
    #[inline(always)]
    pub fn xt_div8(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr4clk::XtDiv8)
    }
    #[doc = "Clock source is XT / 16"]
    #[inline(always)]
    pub fn xt_div16(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr4clk::XtDiv16)
    }
    #[doc = "Clock source is XT / 32"]
    #[inline(always)]
    pub fn xt_div32(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr4clk::XtDiv32)
    }
    #[doc = "Clock source is XT / 128"]
    #[inline(always)]
    pub fn xt_div128(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr4clk::XtDiv128)
    }
    #[doc = "Clock source is 100 Hz from the current RTC oscillator."]
    #[inline(always)]
    pub fn rtc_100hz(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr4clk::Rtc100hz)
    }
    #[doc = "Clock source is Buck VDDC TON pulses."]
    #[inline(always)]
    pub fn buckc(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr4clk::Buckc)
    }
    #[doc = "Clock source is Buck VDDF TON pulses."]
    #[inline(always)]
    pub fn buckf(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr4clk::Buckf)
    }
    #[doc = "Clock source is Buck VDDS TON pulses."]
    #[inline(always)]
    pub fn bucks(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr4clk::Bucks)
    }
    #[doc = "Clock source is Buck VDDC_LV TON pulses."]
    #[inline(always)]
    pub fn buckc_lv(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr4clk::BuckcLv)
    }
    #[doc = "Clock source is TIMER 0 Output 0"]
    #[inline(always)]
    pub fn tmr00(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr4clk::Tmr00)
    }
    #[doc = "Clock source is TIMER 0 Output 1"]
    #[inline(always)]
    pub fn tmr01(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr4clk::Tmr01)
    }
    #[doc = "Clock source is TIMER 1 Output 0"]
    #[inline(always)]
    pub fn tmr10(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr4clk::Tmr10)
    }
    #[doc = "Clock source is TIMER 1 Output 1"]
    #[inline(always)]
    pub fn tmr11(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr4clk::Tmr11)
    }
    #[doc = "Clock source is TIMER 2 Output 0"]
    #[inline(always)]
    pub fn tmr20(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr4clk::Tmr20)
    }
    #[doc = "Clock source is TIMER 2 Output 1"]
    #[inline(always)]
    pub fn tmr21(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr4clk::Tmr21)
    }
    #[doc = "Clock source is TIMER 3 Output 0"]
    #[inline(always)]
    pub fn tmr30(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr4clk::Tmr30)
    }
    #[doc = "Clock source is TIMER 3 Output 1"]
    #[inline(always)]
    pub fn tmr31(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr4clk::Tmr31)
    }
    #[doc = "Clock source is TIMER 4 Output 0"]
    #[inline(always)]
    pub fn tmr40(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr4clk::Tmr40)
    }
    #[doc = "Clock source is TIMER 4 Output 1"]
    #[inline(always)]
    pub fn tmr41(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr4clk::Tmr41)
    }
    #[doc = "Clock source is TIMER 5 Output 0"]
    #[inline(always)]
    pub fn tmr50(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr4clk::Tmr50)
    }
    #[doc = "Clock source is TIMER 5 Output 1"]
    #[inline(always)]
    pub fn tmr51(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr4clk::Tmr51)
    }
    #[doc = "Clock source is TIMER 6 Output 0"]
    #[inline(always)]
    pub fn tmr60(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr4clk::Tmr60)
    }
    #[doc = "Clock source is TIMER 6 Output 1"]
    #[inline(always)]
    pub fn tmr61(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr4clk::Tmr61)
    }
    #[doc = "Clock source is TIMER 7 Output 0"]
    #[inline(always)]
    pub fn tmr70(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr4clk::Tmr70)
    }
    #[doc = "Clock source is TIMER 7 Output 1"]
    #[inline(always)]
    pub fn tmr71(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr4clk::Tmr71)
    }
    #[doc = "Clock source is TIMER 8 Output 0"]
    #[inline(always)]
    pub fn tmr80(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr4clk::Tmr80)
    }
    #[doc = "Clock source is TIMER 8 Output 1"]
    #[inline(always)]
    pub fn tmr81(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr4clk::Tmr81)
    }
    #[doc = "Clock source is TIMER 9 Output 0"]
    #[inline(always)]
    pub fn tmr90(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr4clk::Tmr90)
    }
    #[doc = "Clock source is TIMER 9 Output 1"]
    #[inline(always)]
    pub fn tmr91(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr4clk::Tmr91)
    }
    #[doc = "Clock source is TIMER 10 Output 0"]
    #[inline(always)]
    pub fn tmr100(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr4clk::Tmr100)
    }
    #[doc = "Clock source is TIMER 10 Output 1"]
    #[inline(always)]
    pub fn tmr101(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr4clk::Tmr101)
    }
    #[doc = "Clock source is TIMER 11 Output 0"]
    #[inline(always)]
    pub fn tmr110(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr4clk::Tmr110)
    }
    #[doc = "Clock source is TIMER 11 Output 1"]
    #[inline(always)]
    pub fn tmr111(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr4clk::Tmr111)
    }
    #[doc = "Clock source is TIMER 12 Output 0"]
    #[inline(always)]
    pub fn tmr120(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr4clk::Tmr120)
    }
    #[doc = "Clock source is TIMER 12 Output 1"]
    #[inline(always)]
    pub fn tmr121(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr4clk::Tmr121)
    }
    #[doc = "Clock source is TIMER 13 Output 0"]
    #[inline(always)]
    pub fn tmr130(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr4clk::Tmr130)
    }
    #[doc = "Clock source is TIMER 13 Output 1"]
    #[inline(always)]
    pub fn tmr131(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr4clk::Tmr131)
    }
    #[doc = "Clock source is TIMER 14 Output 0"]
    #[inline(always)]
    pub fn tmr140(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr4clk::Tmr140)
    }
    #[doc = "Clock source is TIMER 14 Output 1"]
    #[inline(always)]
    pub fn tmr141(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr4clk::Tmr141)
    }
    #[doc = "Clock source is TIMER 15 Output 0"]
    #[inline(always)]
    pub fn tmr150(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr4clk::Tmr150)
    }
    #[doc = "Clock source is TIMER 15 Output 1"]
    #[inline(always)]
    pub fn tmr151(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr4clk::Tmr151)
    }
    #[doc = "GPIO #0 is clock source"]
    #[inline(always)]
    pub fn gpio0(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr4clk::Gpio0)
    }
    #[doc = "GPIO #63 is clock source"]
    #[inline(always)]
    pub fn gpio63(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr4clk::Gpio63)
    }
    #[doc = "GPIO #95 is clock source"]
    #[inline(always)]
    pub fn gpio95(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr4clk::Gpio95)
    }
    #[doc = "GPIO #127 is clock source"]
    #[inline(always)]
    pub fn gpio127(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr4clk::Gpio127)
    }
}
#[doc = "Counter/Timer 4 Trigger Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tmr4tmode {
    #[doc = "0: Trigger not enabled"]
    Dis = 0,
    #[doc = "1: Trigger on rising edge of TRIGSEL source"]
    Rise = 1,
    #[doc = "2: Trigger on falling edge of TRIGSEL source"]
    Fall = 2,
    #[doc = "3: Trigger on either edge of TRIGSEL source"]
    Both = 3,
}
impl From<Tmr4tmode> for u8 {
    #[inline(always)]
    fn from(variant: Tmr4tmode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tmr4tmode {
    type Ux = u8;
}
impl crate::IsEnum for Tmr4tmode {}
#[doc = "Field `TMR4TMODE` reader - Counter/Timer 4 Trigger Mode"]
pub type Tmr4tmodeR = crate::FieldReader<Tmr4tmode>;
impl Tmr4tmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tmr4tmode {
        match self.bits {
            0 => Tmr4tmode::Dis,
            1 => Tmr4tmode::Rise,
            2 => Tmr4tmode::Fall,
            3 => Tmr4tmode::Both,
            _ => unreachable!(),
        }
    }
    #[doc = "Trigger not enabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Tmr4tmode::Dis
    }
    #[doc = "Trigger on rising edge of TRIGSEL source"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == Tmr4tmode::Rise
    }
    #[doc = "Trigger on falling edge of TRIGSEL source"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == Tmr4tmode::Fall
    }
    #[doc = "Trigger on either edge of TRIGSEL source"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == Tmr4tmode::Both
    }
}
#[doc = "Field `TMR4TMODE` writer - Counter/Timer 4 Trigger Mode"]
pub type Tmr4tmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Tmr4tmode, crate::Safe>;
impl<'a, REG> Tmr4tmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Trigger not enabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr4tmode::Dis)
    }
    #[doc = "Trigger on rising edge of TRIGSEL source"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr4tmode::Rise)
    }
    #[doc = "Trigger on falling edge of TRIGSEL source"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr4tmode::Fall)
    }
    #[doc = "Trigger on either edge of TRIGSEL source"]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr4tmode::Both)
    }
}
#[doc = "Field `TMR4LMT` reader - This field decides the number of iterations of Counter/Timer 4. For Single/Repeat Patterns, it indicates number of bits to be shifted out and so, max value is 63."]
pub type Tmr4lmtR = crate::FieldReader;
#[doc = "Field `TMR4LMT` writer - This field decides the number of iterations of Counter/Timer 4. For Single/Repeat Patterns, it indicates number of bits to be shifted out and so, max value is 63."]
pub type Tmr4lmtW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Counter/Timer 4 Enable bit."]
    #[inline(always)]
    pub fn tmr4en(&self) -> Tmr4enR {
        Tmr4enR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Counter/Timer Clear bit."]
    #[inline(always)]
    pub fn tmr4clr(&self) -> Tmr4clrR {
        Tmr4clrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Counter/Timer 4 output 0 polarity."]
    #[inline(always)]
    pub fn tmr4pol0(&self) -> Tmr4pol0R {
        Tmr4pol0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Counter/Timer 4 output 1 polarity."]
    #[inline(always)]
    pub fn tmr4pol1(&self) -> Tmr4pol1R {
        Tmr4pol1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Counter/Timer 4 Function Select. For all Functions, CMP0 marks the end of timer cycle, and thus restarts the timer."]
    #[inline(always)]
    pub fn tmr4fn(&self) -> Tmr4fnR {
        Tmr4fnR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - Counter/Timer 4 Clock Select."]
    #[inline(always)]
    pub fn tmr4clk(&self) -> Tmr4clkR {
        Tmr4clkR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:17 - Counter/Timer 4 Trigger Mode"]
    #[inline(always)]
    pub fn tmr4tmode(&self) -> Tmr4tmodeR {
        Tmr4tmodeR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:31 - This field decides the number of iterations of Counter/Timer 4. For Single/Repeat Patterns, it indicates number of bits to be shifted out and so, max value is 63."]
    #[inline(always)]
    pub fn tmr4lmt(&self) -> Tmr4lmtR {
        Tmr4lmtR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Counter/Timer 4 Enable bit."]
    #[inline(always)]
    #[must_use]
    pub fn tmr4en(&mut self) -> Tmr4enW<Ctrl4Spec> {
        Tmr4enW::new(self, 0)
    }
    #[doc = "Bit 1 - Counter/Timer Clear bit."]
    #[inline(always)]
    #[must_use]
    pub fn tmr4clr(&mut self) -> Tmr4clrW<Ctrl4Spec> {
        Tmr4clrW::new(self, 1)
    }
    #[doc = "Bit 2 - Counter/Timer 4 output 0 polarity."]
    #[inline(always)]
    #[must_use]
    pub fn tmr4pol0(&mut self) -> Tmr4pol0W<Ctrl4Spec> {
        Tmr4pol0W::new(self, 2)
    }
    #[doc = "Bit 3 - Counter/Timer 4 output 1 polarity."]
    #[inline(always)]
    #[must_use]
    pub fn tmr4pol1(&mut self) -> Tmr4pol1W<Ctrl4Spec> {
        Tmr4pol1W::new(self, 3)
    }
    #[doc = "Bits 4:7 - Counter/Timer 4 Function Select. For all Functions, CMP0 marks the end of timer cycle, and thus restarts the timer."]
    #[inline(always)]
    #[must_use]
    pub fn tmr4fn(&mut self) -> Tmr4fnW<Ctrl4Spec> {
        Tmr4fnW::new(self, 4)
    }
    #[doc = "Bits 8:15 - Counter/Timer 4 Clock Select."]
    #[inline(always)]
    #[must_use]
    pub fn tmr4clk(&mut self) -> Tmr4clkW<Ctrl4Spec> {
        Tmr4clkW::new(self, 8)
    }
    #[doc = "Bits 16:17 - Counter/Timer 4 Trigger Mode"]
    #[inline(always)]
    #[must_use]
    pub fn tmr4tmode(&mut self) -> Tmr4tmodeW<Ctrl4Spec> {
        Tmr4tmodeW::new(self, 16)
    }
    #[doc = "Bits 24:31 - This field decides the number of iterations of Counter/Timer 4. For Single/Repeat Patterns, it indicates number of bits to be shifted out and so, max value is 63."]
    #[inline(always)]
    #[must_use]
    pub fn tmr4lmt(&mut self) -> Tmr4lmtW<Ctrl4Spec> {
        Tmr4lmtW::new(self, 24)
    }
}
#[doc = "This includes the Control bit fields for timer 4.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctrl4Spec;
impl crate::RegisterSpec for Ctrl4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl4::R`](R) reader structure"]
impl crate::Readable for Ctrl4Spec {}
#[doc = "`write(|w| ..)` method takes [`ctrl4::W`](W) writer structure"]
impl crate::Writable for Ctrl4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL4 to value 0"]
impl crate::Resettable for Ctrl4Spec {
    const RESET_VALUE: u32 = 0;
}
