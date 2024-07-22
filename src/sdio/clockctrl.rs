#[doc = "Register `CLOCKCTRL` reader"]
pub type R = crate::R<ClockctrlSpec>;
#[doc = "Register `CLOCKCTRL` writer"]
pub type W = crate::W<ClockctrlSpec>;
#[doc = "This bit is set to 0 when the HD is not using the HC or the HC awaits a wakeup event. The HC should stop its internal clock to go very low power state. Still, registers shall be able to be read and written. Clock starts to oscillate when this bit is set to 1. When clock oscillation is stable, the HC shall set Internal Clock Stable in this register to 1. This bit shall not affect card detection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clken {
    #[doc = "1: Oscillate"]
    Osc = 1,
    #[doc = "0: Stop"]
    Stop = 0,
}
impl From<Clken> for bool {
    #[inline(always)]
    fn from(variant: Clken) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKEN` reader - This bit is set to 0 when the HD is not using the HC or the HC awaits a wakeup event. The HC should stop its internal clock to go very low power state. Still, registers shall be able to be read and written. Clock starts to oscillate when this bit is set to 1. When clock oscillation is stable, the HC shall set Internal Clock Stable in this register to 1. This bit shall not affect card detection."]
pub type ClkenR = crate::BitReader<Clken>;
impl ClkenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Clken {
        match self.bits {
            true => Clken::Osc,
            false => Clken::Stop,
        }
    }
    #[doc = "Oscillate"]
    #[inline(always)]
    pub fn is_osc(&self) -> bool {
        *self == Clken::Osc
    }
    #[doc = "Stop"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == Clken::Stop
    }
}
#[doc = "Field `CLKEN` writer - This bit is set to 0 when the HD is not using the HC or the HC awaits a wakeup event. The HC should stop its internal clock to go very low power state. Still, registers shall be able to be read and written. Clock starts to oscillate when this bit is set to 1. When clock oscillation is stable, the HC shall set Internal Clock Stable in this register to 1. This bit shall not affect card detection."]
pub type ClkenW<'a, REG> = crate::BitWriter<'a, REG, Clken>;
impl<'a, REG> ClkenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Oscillate"]
    #[inline(always)]
    pub fn osc(self) -> &'a mut crate::W<REG> {
        self.variant(Clken::Osc)
    }
    #[doc = "Stop"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(Clken::Stop)
    }
}
#[doc = "This bit is set to 1 when SD clock is stable after writing to Internal Clock Enable in this register to 1. The SD Host Driver shall wait to set SD Clock Enable until this bit is set to 1. Note: This is useful when using PLL for a clock oscillator that requires setup time.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clkstable {
    #[doc = "1: Ready"]
    Ready = 1,
    #[doc = "0: Not Ready"]
    Notready = 0,
}
impl From<Clkstable> for bool {
    #[inline(always)]
    fn from(variant: Clkstable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKSTABLE` reader - This bit is set to 1 when SD clock is stable after writing to Internal Clock Enable in this register to 1. The SD Host Driver shall wait to set SD Clock Enable until this bit is set to 1. Note: This is useful when using PLL for a clock oscillator that requires setup time."]
pub type ClkstableR = crate::BitReader<Clkstable>;
impl ClkstableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Clkstable {
        match self.bits {
            true => Clkstable::Ready,
            false => Clkstable::Notready,
        }
    }
    #[doc = "Ready"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == Clkstable::Ready
    }
    #[doc = "Not Ready"]
    #[inline(always)]
    pub fn is_notready(&self) -> bool {
        *self == Clkstable::Notready
    }
}
#[doc = "Field `CLKSTABLE` writer - This bit is set to 1 when SD clock is stable after writing to Internal Clock Enable in this register to 1. The SD Host Driver shall wait to set SD Clock Enable until this bit is set to 1. Note: This is useful when using PLL for a clock oscillator that requires setup time."]
pub type ClkstableW<'a, REG> = crate::BitWriter<'a, REG, Clkstable>;
impl<'a, REG> ClkstableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ready"]
    #[inline(always)]
    pub fn ready(self) -> &'a mut crate::W<REG> {
        self.variant(Clkstable::Ready)
    }
    #[doc = "Not Ready"]
    #[inline(always)]
    pub fn notready(self) -> &'a mut crate::W<REG> {
        self.variant(Clkstable::Notready)
    }
}
#[doc = "The HC shall stop SDCLK when writing this bit to 0. SDCLK frequency Select can be changed when this bit is 0. Then, the HC shall maintain the same clock frequency until SDCLK is stopped (Stop at SDCLK = 0). If the HC detects the No Card state, this bit shall be cleared.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sdclken {
    #[doc = "1: Enable"]
    Enable = 1,
    #[doc = "0: Disable"]
    Disable = 0,
}
impl From<Sdclken> for bool {
    #[inline(always)]
    fn from(variant: Sdclken) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDCLKEN` reader - The HC shall stop SDCLK when writing this bit to 0. SDCLK frequency Select can be changed when this bit is 0. Then, the HC shall maintain the same clock frequency until SDCLK is stopped (Stop at SDCLK = 0). If the HC detects the No Card state, this bit shall be cleared."]
pub type SdclkenR = crate::BitReader<Sdclken>;
impl SdclkenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sdclken {
        match self.bits {
            true => Sdclken::Enable,
            false => Sdclken::Disable,
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Sdclken::Enable
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Sdclken::Disable
    }
}
#[doc = "Field `SDCLKEN` writer - The HC shall stop SDCLK when writing this bit to 0. SDCLK frequency Select can be changed when this bit is 0. Then, the HC shall maintain the same clock frequency until SDCLK is stopped (Stop at SDCLK = 0). If the HC detects the No Card state, this bit shall be cleared."]
pub type SdclkenW<'a, REG> = crate::BitWriter<'a, REG, Sdclken>;
impl<'a, REG> SdclkenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Sdclken::Enable)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Sdclken::Disable)
    }
}
#[doc = "This bit is used to select the clock generator mode in SDCLK Frequency Select. If the Programmable Clock Mode is supported (non-zero value is set to Clock Multiplier in the Capabilities register), this bit attribute is RW, and if not supported, this bit attribute is RO and zero is read. This bit depends on the setting of Preset Value Enable in the Host Control 2 register. If the Preset Value Enable = 0, this bit is set by Host Driver. If the Preset Value Enable = 1, this bit is automatically set to a value specified in one of Preset Value registers.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clkgensel {
    #[doc = "1: Programmable Clock Mode"]
    Progclk = 1,
    #[doc = "0: Divided Clock Mode"]
    Divclk = 0,
}
impl From<Clkgensel> for bool {
    #[inline(always)]
    fn from(variant: Clkgensel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKGENSEL` reader - This bit is used to select the clock generator mode in SDCLK Frequency Select. If the Programmable Clock Mode is supported (non-zero value is set to Clock Multiplier in the Capabilities register), this bit attribute is RW, and if not supported, this bit attribute is RO and zero is read. This bit depends on the setting of Preset Value Enable in the Host Control 2 register. If the Preset Value Enable = 0, this bit is set by Host Driver. If the Preset Value Enable = 1, this bit is automatically set to a value specified in one of Preset Value registers."]
pub type ClkgenselR = crate::BitReader<Clkgensel>;
impl ClkgenselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Clkgensel {
        match self.bits {
            true => Clkgensel::Progclk,
            false => Clkgensel::Divclk,
        }
    }
    #[doc = "Programmable Clock Mode"]
    #[inline(always)]
    pub fn is_progclk(&self) -> bool {
        *self == Clkgensel::Progclk
    }
    #[doc = "Divided Clock Mode"]
    #[inline(always)]
    pub fn is_divclk(&self) -> bool {
        *self == Clkgensel::Divclk
    }
}
#[doc = "Field `CLKGENSEL` writer - This bit is used to select the clock generator mode in SDCLK Frequency Select. If the Programmable Clock Mode is supported (non-zero value is set to Clock Multiplier in the Capabilities register), this bit attribute is RW, and if not supported, this bit attribute is RO and zero is read. This bit depends on the setting of Preset Value Enable in the Host Control 2 register. If the Preset Value Enable = 0, this bit is set by Host Driver. If the Preset Value Enable = 1, this bit is automatically set to a value specified in one of Preset Value registers."]
pub type ClkgenselW<'a, REG> = crate::BitWriter<'a, REG, Clkgensel>;
impl<'a, REG> ClkgenselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Programmable Clock Mode"]
    #[inline(always)]
    pub fn progclk(self) -> &'a mut crate::W<REG> {
        self.variant(Clkgensel::Progclk)
    }
    #[doc = "Divided Clock Mode"]
    #[inline(always)]
    pub fn divclk(self) -> &'a mut crate::W<REG> {
        self.variant(Clkgensel::Divclk)
    }
}
#[doc = "Field `UPRCLKDIV` reader - Bit 07-06 is assigned to bit 09-08 of clock divider in SDCLK Frequency Select"]
pub type UprclkdivR = crate::FieldReader;
#[doc = "Field `UPRCLKDIV` writer - Bit 07-06 is assigned to bit 09-08 of clock divider in SDCLK Frequency Select"]
pub type UprclkdivW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "This register is used to select the frequency of the SDCLK pin. The frequency is not programmed directly; rather this register holds the divisor of the Base Clock Frequency For SD clock in the capabilities register. Only the following settings are allowed. (1) 8-bit Divided Clock Mode Setting 00h specifies the highest frequency of the SD Clock. When setting multiple bits, the most significant bit is used as the divisor. But multiple bits should not be set. The two default divider values can be calculated by the frequency that is defined by the Base Clock Frequency For SD Clock in the Capabilities register. 1) 25 MHz divider value 2) 400 KHz divider value The frequency of the SDCLK is set by the following formula: Clock Frequency = (Baseclock) / divisor. Thus choose the smallest possible divisor which results in a clock frequency that is less than or equal to the target frequency. Maximum Frequency for SD = 50Mhz (base clock) Maximum Frequency for MMC = 52Mhz (base clock) Minimum Frequency = 195.3125Khz (50Mhz / 256), same calculation for MMC also (2) 10-bit Divided Clock Mode Host Controller Version 3.00 supports this mandatory mode instead of the 8-bit Divided Clock Mode. The length of divider is extended to10 bits and all divider values shall be supported. 3FFh --1/2046 Divided Clock N -------1/2N Divided Clock (Duty 50%) 002h -- 1/4 Divided Clock 001h ---1/2 Divided Clock 000h --- Base Clock (10MHz-254MHz)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Freqsel {
    #[doc = "128: base clock divided by 256"]
    Div256 = 128,
    #[doc = "64: base clock divided by 128"]
    Div128 = 64,
    #[doc = "32: base clock divided by 64"]
    Div64 = 32,
    #[doc = "16: base clock divided by 32"]
    Div32 = 16,
    #[doc = "8: base clock divided by 16"]
    Div16 = 8,
    #[doc = "4: base clock divided by 8"]
    Div8 = 4,
    #[doc = "2: base clock divided by 4"]
    Div4 = 2,
    #[doc = "1: base clock divided by 2"]
    Div2 = 1,
    #[doc = "0: Base clock (10MHz - 63MHz)"]
    Baseclk = 0,
}
impl From<Freqsel> for u8 {
    #[inline(always)]
    fn from(variant: Freqsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Freqsel {
    type Ux = u8;
}
impl crate::IsEnum for Freqsel {}
#[doc = "Field `FREQSEL` reader - This register is used to select the frequency of the SDCLK pin. The frequency is not programmed directly; rather this register holds the divisor of the Base Clock Frequency For SD clock in the capabilities register. Only the following settings are allowed. (1) 8-bit Divided Clock Mode Setting 00h specifies the highest frequency of the SD Clock. When setting multiple bits, the most significant bit is used as the divisor. But multiple bits should not be set. The two default divider values can be calculated by the frequency that is defined by the Base Clock Frequency For SD Clock in the Capabilities register. 1) 25 MHz divider value 2) 400 KHz divider value The frequency of the SDCLK is set by the following formula: Clock Frequency = (Baseclock) / divisor. Thus choose the smallest possible divisor which results in a clock frequency that is less than or equal to the target frequency. Maximum Frequency for SD = 50Mhz (base clock) Maximum Frequency for MMC = 52Mhz (base clock) Minimum Frequency = 195.3125Khz (50Mhz / 256), same calculation for MMC also (2) 10-bit Divided Clock Mode Host Controller Version 3.00 supports this mandatory mode instead of the 8-bit Divided Clock Mode. The length of divider is extended to10 bits and all divider values shall be supported. 3FFh --1/2046 Divided Clock N -------1/2N Divided Clock (Duty 50%) 002h -- 1/4 Divided Clock 001h ---1/2 Divided Clock 000h --- Base Clock (10MHz-254MHz)"]
pub type FreqselR = crate::FieldReader<Freqsel>;
impl FreqselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Freqsel> {
        match self.bits {
            128 => Some(Freqsel::Div256),
            64 => Some(Freqsel::Div128),
            32 => Some(Freqsel::Div64),
            16 => Some(Freqsel::Div32),
            8 => Some(Freqsel::Div16),
            4 => Some(Freqsel::Div8),
            2 => Some(Freqsel::Div4),
            1 => Some(Freqsel::Div2),
            0 => Some(Freqsel::Baseclk),
            _ => None,
        }
    }
    #[doc = "base clock divided by 256"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == Freqsel::Div256
    }
    #[doc = "base clock divided by 128"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == Freqsel::Div128
    }
    #[doc = "base clock divided by 64"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == Freqsel::Div64
    }
    #[doc = "base clock divided by 32"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == Freqsel::Div32
    }
    #[doc = "base clock divided by 16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == Freqsel::Div16
    }
    #[doc = "base clock divided by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == Freqsel::Div8
    }
    #[doc = "base clock divided by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Freqsel::Div4
    }
    #[doc = "base clock divided by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Freqsel::Div2
    }
    #[doc = "Base clock (10MHz - 63MHz)"]
    #[inline(always)]
    pub fn is_baseclk(&self) -> bool {
        *self == Freqsel::Baseclk
    }
}
#[doc = "Field `FREQSEL` writer - This register is used to select the frequency of the SDCLK pin. The frequency is not programmed directly; rather this register holds the divisor of the Base Clock Frequency For SD clock in the capabilities register. Only the following settings are allowed. (1) 8-bit Divided Clock Mode Setting 00h specifies the highest frequency of the SD Clock. When setting multiple bits, the most significant bit is used as the divisor. But multiple bits should not be set. The two default divider values can be calculated by the frequency that is defined by the Base Clock Frequency For SD Clock in the Capabilities register. 1) 25 MHz divider value 2) 400 KHz divider value The frequency of the SDCLK is set by the following formula: Clock Frequency = (Baseclock) / divisor. Thus choose the smallest possible divisor which results in a clock frequency that is less than or equal to the target frequency. Maximum Frequency for SD = 50Mhz (base clock) Maximum Frequency for MMC = 52Mhz (base clock) Minimum Frequency = 195.3125Khz (50Mhz / 256), same calculation for MMC also (2) 10-bit Divided Clock Mode Host Controller Version 3.00 supports this mandatory mode instead of the 8-bit Divided Clock Mode. The length of divider is extended to10 bits and all divider values shall be supported. 3FFh --1/2046 Divided Clock N -------1/2N Divided Clock (Duty 50%) 002h -- 1/4 Divided Clock 001h ---1/2 Divided Clock 000h --- Base Clock (10MHz-254MHz)"]
pub type FreqselW<'a, REG> = crate::FieldWriter<'a, REG, 8, Freqsel>;
impl<'a, REG> FreqselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "base clock divided by 256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut crate::W<REG> {
        self.variant(Freqsel::Div256)
    }
    #[doc = "base clock divided by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(Freqsel::Div128)
    }
    #[doc = "base clock divided by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(Freqsel::Div64)
    }
    #[doc = "base clock divided by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(Freqsel::Div32)
    }
    #[doc = "base clock divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(Freqsel::Div16)
    }
    #[doc = "base clock divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(Freqsel::Div8)
    }
    #[doc = "base clock divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Freqsel::Div4)
    }
    #[doc = "base clock divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Freqsel::Div2)
    }
    #[doc = "Base clock (10MHz - 63MHz)"]
    #[inline(always)]
    pub fn baseclk(self) -> &'a mut crate::W<REG> {
        self.variant(Freqsel::Baseclk)
    }
}
#[doc = "This value determines the interval by which DAT line time-outs are detected. Refer to the Data Time-out Error in the Error Interrupt Status register for information on factors that dictate time-out generation. Time-out clock frequency will be generated by dividing the sdclockTMCLK by this value. When setting this register, prevent inadvertent time-out events by clearing the Data Time-out Error Status Enable (in the Error Interrupt Status Enable register) At the initialization of the HC, the HD shall set the Data Time-out Counter Value according to the Capabilities register. The general formula is TIMEOUT = 2 ^ (TIMEOUTCNT + 13).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Timeoutcnt {
    #[doc = "14: TMCLK * 2^27"]
    _27 = 14,
    #[doc = "0: TMCLK * 2^26"]
    _26 = 0,
}
impl From<Timeoutcnt> for u8 {
    #[inline(always)]
    fn from(variant: Timeoutcnt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Timeoutcnt {
    type Ux = u8;
}
impl crate::IsEnum for Timeoutcnt {}
#[doc = "Field `TIMEOUTCNT` reader - This value determines the interval by which DAT line time-outs are detected. Refer to the Data Time-out Error in the Error Interrupt Status register for information on factors that dictate time-out generation. Time-out clock frequency will be generated by dividing the sdclockTMCLK by this value. When setting this register, prevent inadvertent time-out events by clearing the Data Time-out Error Status Enable (in the Error Interrupt Status Enable register) At the initialization of the HC, the HD shall set the Data Time-out Counter Value according to the Capabilities register. The general formula is TIMEOUT = 2 ^ (TIMEOUTCNT + 13)."]
pub type TimeoutcntR = crate::FieldReader<Timeoutcnt>;
impl TimeoutcntR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Timeoutcnt> {
        match self.bits {
            14 => Some(Timeoutcnt::_27),
            0 => Some(Timeoutcnt::_26),
            _ => None,
        }
    }
    #[doc = "TMCLK * 2^27"]
    #[inline(always)]
    pub fn is_27(&self) -> bool {
        *self == Timeoutcnt::_27
    }
    #[doc = "TMCLK * 2^26"]
    #[inline(always)]
    pub fn is_26(&self) -> bool {
        *self == Timeoutcnt::_26
    }
}
#[doc = "Field `TIMEOUTCNT` writer - This value determines the interval by which DAT line time-outs are detected. Refer to the Data Time-out Error in the Error Interrupt Status register for information on factors that dictate time-out generation. Time-out clock frequency will be generated by dividing the sdclockTMCLK by this value. When setting this register, prevent inadvertent time-out events by clearing the Data Time-out Error Status Enable (in the Error Interrupt Status Enable register) At the initialization of the HC, the HD shall set the Data Time-out Counter Value according to the Capabilities register. The general formula is TIMEOUT = 2 ^ (TIMEOUTCNT + 13)."]
pub type TimeoutcntW<'a, REG> = crate::FieldWriter<'a, REG, 4, Timeoutcnt>;
impl<'a, REG> TimeoutcntW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TMCLK * 2^27"]
    #[inline(always)]
    pub fn _27(self) -> &'a mut crate::W<REG> {
        self.variant(Timeoutcnt::_27)
    }
    #[doc = "TMCLK * 2^26"]
    #[inline(always)]
    pub fn _26(self) -> &'a mut crate::W<REG> {
        self.variant(Timeoutcnt::_26)
    }
}
#[doc = "This reset affects the entire HC except for the card detection circuit. Register bits of type ROC, RW, RW1C, RWAC are cleared to 0. During its initialization, the HD shall set this bit to 1 to reset the HC. The HC shall reset this bit to 0 when capabilities registers are valid and the HD can read them. Additional use of Software Reset For All may not affect the value of the Capabilities registers. If this bit is set to 1, the SD card shall reset itself and must be re initialized by the HD. A reset pulse is generated when writing 1 to each bit of this register. After completing the reset, the HC shall clear each bit. Because it takes some time to complete software reset, the SD Host Driver shall confirm that these bits are 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swrstall {
    #[doc = "1: Reset"]
    Reset = 1,
    #[doc = "0: Work"]
    Work = 0,
}
impl From<Swrstall> for bool {
    #[inline(always)]
    fn from(variant: Swrstall) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWRSTALL` reader - This reset affects the entire HC except for the card detection circuit. Register bits of type ROC, RW, RW1C, RWAC are cleared to 0. During its initialization, the HD shall set this bit to 1 to reset the HC. The HC shall reset this bit to 0 when capabilities registers are valid and the HD can read them. Additional use of Software Reset For All may not affect the value of the Capabilities registers. If this bit is set to 1, the SD card shall reset itself and must be re initialized by the HD. A reset pulse is generated when writing 1 to each bit of this register. After completing the reset, the HC shall clear each bit. Because it takes some time to complete software reset, the SD Host Driver shall confirm that these bits are 0."]
pub type SwrstallR = crate::BitReader<Swrstall>;
impl SwrstallR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swrstall {
        match self.bits {
            true => Swrstall::Reset,
            false => Swrstall::Work,
        }
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Swrstall::Reset
    }
    #[doc = "Work"]
    #[inline(always)]
    pub fn is_work(&self) -> bool {
        *self == Swrstall::Work
    }
}
#[doc = "Field `SWRSTALL` writer - This reset affects the entire HC except for the card detection circuit. Register bits of type ROC, RW, RW1C, RWAC are cleared to 0. During its initialization, the HD shall set this bit to 1 to reset the HC. The HC shall reset this bit to 0 when capabilities registers are valid and the HD can read them. Additional use of Software Reset For All may not affect the value of the Capabilities registers. If this bit is set to 1, the SD card shall reset itself and must be re initialized by the HD. A reset pulse is generated when writing 1 to each bit of this register. After completing the reset, the HC shall clear each bit. Because it takes some time to complete software reset, the SD Host Driver shall confirm that these bits are 0."]
pub type SwrstallW<'a, REG> = crate::BitWriter<'a, REG, Swrstall>;
impl<'a, REG> SwrstallW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(Swrstall::Reset)
    }
    #[doc = "Work"]
    #[inline(always)]
    pub fn work(self) -> &'a mut crate::W<REG> {
        self.variant(Swrstall::Work)
    }
}
#[doc = "Only part of command circuit is reset. The following registers and bits are cleared by this bit: Present State register Command Inhibit (CMD) Normal Interrupt Status register Command Complete\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swrstcmd {
    #[doc = "1: Reset"]
    Reset = 1,
    #[doc = "0: Work"]
    Work = 0,
}
impl From<Swrstcmd> for bool {
    #[inline(always)]
    fn from(variant: Swrstcmd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWRSTCMD` reader - Only part of command circuit is reset. The following registers and bits are cleared by this bit: Present State register Command Inhibit (CMD) Normal Interrupt Status register Command Complete"]
pub type SwrstcmdR = crate::BitReader<Swrstcmd>;
impl SwrstcmdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swrstcmd {
        match self.bits {
            true => Swrstcmd::Reset,
            false => Swrstcmd::Work,
        }
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Swrstcmd::Reset
    }
    #[doc = "Work"]
    #[inline(always)]
    pub fn is_work(&self) -> bool {
        *self == Swrstcmd::Work
    }
}
#[doc = "Field `SWRSTCMD` writer - Only part of command circuit is reset. The following registers and bits are cleared by this bit: Present State register Command Inhibit (CMD) Normal Interrupt Status register Command Complete"]
pub type SwrstcmdW<'a, REG> = crate::BitWriter<'a, REG, Swrstcmd>;
impl<'a, REG> SwrstcmdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(Swrstcmd::Reset)
    }
    #[doc = "Work"]
    #[inline(always)]
    pub fn work(self) -> &'a mut crate::W<REG> {
        self.variant(Swrstcmd::Work)
    }
}
#[doc = "Only part of data circuit is reset. The following registers and bits are cleared by this bit: Buffer Data Port Register Buffer is cleared and Initialized. Present State register Buffer read Enable Buffer write Enable Read Transfer Active Write Transfer Active DAT Line Active Command Inhibit (DAT) Block Gap Control register Continue Request Stop At Block Gap Request Normal Interrupt Status register Buffer Read Ready Buffer Write Ready Block Gap Event Transfer Complete\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swrstdat {
    #[doc = "1: Reset"]
    Reset = 1,
    #[doc = "0: Work"]
    Work = 0,
}
impl From<Swrstdat> for bool {
    #[inline(always)]
    fn from(variant: Swrstdat) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWRSTDAT` reader - Only part of data circuit is reset. The following registers and bits are cleared by this bit: Buffer Data Port Register Buffer is cleared and Initialized. Present State register Buffer read Enable Buffer write Enable Read Transfer Active Write Transfer Active DAT Line Active Command Inhibit (DAT) Block Gap Control register Continue Request Stop At Block Gap Request Normal Interrupt Status register Buffer Read Ready Buffer Write Ready Block Gap Event Transfer Complete"]
pub type SwrstdatR = crate::BitReader<Swrstdat>;
impl SwrstdatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swrstdat {
        match self.bits {
            true => Swrstdat::Reset,
            false => Swrstdat::Work,
        }
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Swrstdat::Reset
    }
    #[doc = "Work"]
    #[inline(always)]
    pub fn is_work(&self) -> bool {
        *self == Swrstdat::Work
    }
}
#[doc = "Field `SWRSTDAT` writer - Only part of data circuit is reset. The following registers and bits are cleared by this bit: Buffer Data Port Register Buffer is cleared and Initialized. Present State register Buffer read Enable Buffer write Enable Read Transfer Active Write Transfer Active DAT Line Active Command Inhibit (DAT) Block Gap Control register Continue Request Stop At Block Gap Request Normal Interrupt Status register Buffer Read Ready Buffer Write Ready Block Gap Event Transfer Complete"]
pub type SwrstdatW<'a, REG> = crate::BitWriter<'a, REG, Swrstdat>;
impl<'a, REG> SwrstdatW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(Swrstdat::Reset)
    }
    #[doc = "Work"]
    #[inline(always)]
    pub fn work(self) -> &'a mut crate::W<REG> {
        self.variant(Swrstdat::Work)
    }
}
impl R {
    #[doc = "Bit 0 - This bit is set to 0 when the HD is not using the HC or the HC awaits a wakeup event. The HC should stop its internal clock to go very low power state. Still, registers shall be able to be read and written. Clock starts to oscillate when this bit is set to 1. When clock oscillation is stable, the HC shall set Internal Clock Stable in this register to 1. This bit shall not affect card detection."]
    #[inline(always)]
    pub fn clken(&self) -> ClkenR {
        ClkenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit is set to 1 when SD clock is stable after writing to Internal Clock Enable in this register to 1. The SD Host Driver shall wait to set SD Clock Enable until this bit is set to 1. Note: This is useful when using PLL for a clock oscillator that requires setup time."]
    #[inline(always)]
    pub fn clkstable(&self) -> ClkstableR {
        ClkstableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The HC shall stop SDCLK when writing this bit to 0. SDCLK frequency Select can be changed when this bit is 0. Then, the HC shall maintain the same clock frequency until SDCLK is stopped (Stop at SDCLK = 0). If the HC detects the No Card state, this bit shall be cleared."]
    #[inline(always)]
    pub fn sdclken(&self) -> SdclkenR {
        SdclkenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - This bit is used to select the clock generator mode in SDCLK Frequency Select. If the Programmable Clock Mode is supported (non-zero value is set to Clock Multiplier in the Capabilities register), this bit attribute is RW, and if not supported, this bit attribute is RO and zero is read. This bit depends on the setting of Preset Value Enable in the Host Control 2 register. If the Preset Value Enable = 0, this bit is set by Host Driver. If the Preset Value Enable = 1, this bit is automatically set to a value specified in one of Preset Value registers."]
    #[inline(always)]
    pub fn clkgensel(&self) -> ClkgenselR {
        ClkgenselR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Bit 07-06 is assigned to bit 09-08 of clock divider in SDCLK Frequency Select"]
    #[inline(always)]
    pub fn uprclkdiv(&self) -> UprclkdivR {
        UprclkdivR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:15 - This register is used to select the frequency of the SDCLK pin. The frequency is not programmed directly; rather this register holds the divisor of the Base Clock Frequency For SD clock in the capabilities register. Only the following settings are allowed. (1) 8-bit Divided Clock Mode Setting 00h specifies the highest frequency of the SD Clock. When setting multiple bits, the most significant bit is used as the divisor. But multiple bits should not be set. The two default divider values can be calculated by the frequency that is defined by the Base Clock Frequency For SD Clock in the Capabilities register. 1) 25 MHz divider value 2) 400 KHz divider value The frequency of the SDCLK is set by the following formula: Clock Frequency = (Baseclock) / divisor. Thus choose the smallest possible divisor which results in a clock frequency that is less than or equal to the target frequency. Maximum Frequency for SD = 50Mhz (base clock) Maximum Frequency for MMC = 52Mhz (base clock) Minimum Frequency = 195.3125Khz (50Mhz / 256), same calculation for MMC also (2) 10-bit Divided Clock Mode Host Controller Version 3.00 supports this mandatory mode instead of the 8-bit Divided Clock Mode. The length of divider is extended to10 bits and all divider values shall be supported. 3FFh --1/2046 Divided Clock N -------1/2N Divided Clock (Duty 50%) 002h -- 1/4 Divided Clock 001h ---1/2 Divided Clock 000h --- Base Clock (10MHz-254MHz)"]
    #[inline(always)]
    pub fn freqsel(&self) -> FreqselR {
        FreqselR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - This value determines the interval by which DAT line time-outs are detected. Refer to the Data Time-out Error in the Error Interrupt Status register for information on factors that dictate time-out generation. Time-out clock frequency will be generated by dividing the sdclockTMCLK by this value. When setting this register, prevent inadvertent time-out events by clearing the Data Time-out Error Status Enable (in the Error Interrupt Status Enable register) At the initialization of the HC, the HD shall set the Data Time-out Counter Value according to the Capabilities register. The general formula is TIMEOUT = 2 ^ (TIMEOUTCNT + 13)."]
    #[inline(always)]
    pub fn timeoutcnt(&self) -> TimeoutcntR {
        TimeoutcntR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - This reset affects the entire HC except for the card detection circuit. Register bits of type ROC, RW, RW1C, RWAC are cleared to 0. During its initialization, the HD shall set this bit to 1 to reset the HC. The HC shall reset this bit to 0 when capabilities registers are valid and the HD can read them. Additional use of Software Reset For All may not affect the value of the Capabilities registers. If this bit is set to 1, the SD card shall reset itself and must be re initialized by the HD. A reset pulse is generated when writing 1 to each bit of this register. After completing the reset, the HC shall clear each bit. Because it takes some time to complete software reset, the SD Host Driver shall confirm that these bits are 0."]
    #[inline(always)]
    pub fn swrstall(&self) -> SwrstallR {
        SwrstallR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Only part of command circuit is reset. The following registers and bits are cleared by this bit: Present State register Command Inhibit (CMD) Normal Interrupt Status register Command Complete"]
    #[inline(always)]
    pub fn swrstcmd(&self) -> SwrstcmdR {
        SwrstcmdR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Only part of data circuit is reset. The following registers and bits are cleared by this bit: Buffer Data Port Register Buffer is cleared and Initialized. Present State register Buffer read Enable Buffer write Enable Read Transfer Active Write Transfer Active DAT Line Active Command Inhibit (DAT) Block Gap Control register Continue Request Stop At Block Gap Request Normal Interrupt Status register Buffer Read Ready Buffer Write Ready Block Gap Event Transfer Complete"]
    #[inline(always)]
    pub fn swrstdat(&self) -> SwrstdatR {
        SwrstdatR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit is set to 0 when the HD is not using the HC or the HC awaits a wakeup event. The HC should stop its internal clock to go very low power state. Still, registers shall be able to be read and written. Clock starts to oscillate when this bit is set to 1. When clock oscillation is stable, the HC shall set Internal Clock Stable in this register to 1. This bit shall not affect card detection."]
    #[inline(always)]
    #[must_use]
    pub fn clken(&mut self) -> ClkenW<ClockctrlSpec> {
        ClkenW::new(self, 0)
    }
    #[doc = "Bit 1 - This bit is set to 1 when SD clock is stable after writing to Internal Clock Enable in this register to 1. The SD Host Driver shall wait to set SD Clock Enable until this bit is set to 1. Note: This is useful when using PLL for a clock oscillator that requires setup time."]
    #[inline(always)]
    #[must_use]
    pub fn clkstable(&mut self) -> ClkstableW<ClockctrlSpec> {
        ClkstableW::new(self, 1)
    }
    #[doc = "Bit 2 - The HC shall stop SDCLK when writing this bit to 0. SDCLK frequency Select can be changed when this bit is 0. Then, the HC shall maintain the same clock frequency until SDCLK is stopped (Stop at SDCLK = 0). If the HC detects the No Card state, this bit shall be cleared."]
    #[inline(always)]
    #[must_use]
    pub fn sdclken(&mut self) -> SdclkenW<ClockctrlSpec> {
        SdclkenW::new(self, 2)
    }
    #[doc = "Bit 5 - This bit is used to select the clock generator mode in SDCLK Frequency Select. If the Programmable Clock Mode is supported (non-zero value is set to Clock Multiplier in the Capabilities register), this bit attribute is RW, and if not supported, this bit attribute is RO and zero is read. This bit depends on the setting of Preset Value Enable in the Host Control 2 register. If the Preset Value Enable = 0, this bit is set by Host Driver. If the Preset Value Enable = 1, this bit is automatically set to a value specified in one of Preset Value registers."]
    #[inline(always)]
    #[must_use]
    pub fn clkgensel(&mut self) -> ClkgenselW<ClockctrlSpec> {
        ClkgenselW::new(self, 5)
    }
    #[doc = "Bits 6:7 - Bit 07-06 is assigned to bit 09-08 of clock divider in SDCLK Frequency Select"]
    #[inline(always)]
    #[must_use]
    pub fn uprclkdiv(&mut self) -> UprclkdivW<ClockctrlSpec> {
        UprclkdivW::new(self, 6)
    }
    #[doc = "Bits 8:15 - This register is used to select the frequency of the SDCLK pin. The frequency is not programmed directly; rather this register holds the divisor of the Base Clock Frequency For SD clock in the capabilities register. Only the following settings are allowed. (1) 8-bit Divided Clock Mode Setting 00h specifies the highest frequency of the SD Clock. When setting multiple bits, the most significant bit is used as the divisor. But multiple bits should not be set. The two default divider values can be calculated by the frequency that is defined by the Base Clock Frequency For SD Clock in the Capabilities register. 1) 25 MHz divider value 2) 400 KHz divider value The frequency of the SDCLK is set by the following formula: Clock Frequency = (Baseclock) / divisor. Thus choose the smallest possible divisor which results in a clock frequency that is less than or equal to the target frequency. Maximum Frequency for SD = 50Mhz (base clock) Maximum Frequency for MMC = 52Mhz (base clock) Minimum Frequency = 195.3125Khz (50Mhz / 256), same calculation for MMC also (2) 10-bit Divided Clock Mode Host Controller Version 3.00 supports this mandatory mode instead of the 8-bit Divided Clock Mode. The length of divider is extended to10 bits and all divider values shall be supported. 3FFh --1/2046 Divided Clock N -------1/2N Divided Clock (Duty 50%) 002h -- 1/4 Divided Clock 001h ---1/2 Divided Clock 000h --- Base Clock (10MHz-254MHz)"]
    #[inline(always)]
    #[must_use]
    pub fn freqsel(&mut self) -> FreqselW<ClockctrlSpec> {
        FreqselW::new(self, 8)
    }
    #[doc = "Bits 16:19 - This value determines the interval by which DAT line time-outs are detected. Refer to the Data Time-out Error in the Error Interrupt Status register for information on factors that dictate time-out generation. Time-out clock frequency will be generated by dividing the sdclockTMCLK by this value. When setting this register, prevent inadvertent time-out events by clearing the Data Time-out Error Status Enable (in the Error Interrupt Status Enable register) At the initialization of the HC, the HD shall set the Data Time-out Counter Value according to the Capabilities register. The general formula is TIMEOUT = 2 ^ (TIMEOUTCNT + 13)."]
    #[inline(always)]
    #[must_use]
    pub fn timeoutcnt(&mut self) -> TimeoutcntW<ClockctrlSpec> {
        TimeoutcntW::new(self, 16)
    }
    #[doc = "Bit 24 - This reset affects the entire HC except for the card detection circuit. Register bits of type ROC, RW, RW1C, RWAC are cleared to 0. During its initialization, the HD shall set this bit to 1 to reset the HC. The HC shall reset this bit to 0 when capabilities registers are valid and the HD can read them. Additional use of Software Reset For All may not affect the value of the Capabilities registers. If this bit is set to 1, the SD card shall reset itself and must be re initialized by the HD. A reset pulse is generated when writing 1 to each bit of this register. After completing the reset, the HC shall clear each bit. Because it takes some time to complete software reset, the SD Host Driver shall confirm that these bits are 0."]
    #[inline(always)]
    #[must_use]
    pub fn swrstall(&mut self) -> SwrstallW<ClockctrlSpec> {
        SwrstallW::new(self, 24)
    }
    #[doc = "Bit 25 - Only part of command circuit is reset. The following registers and bits are cleared by this bit: Present State register Command Inhibit (CMD) Normal Interrupt Status register Command Complete"]
    #[inline(always)]
    #[must_use]
    pub fn swrstcmd(&mut self) -> SwrstcmdW<ClockctrlSpec> {
        SwrstcmdW::new(self, 25)
    }
    #[doc = "Bit 26 - Only part of data circuit is reset. The following registers and bits are cleared by this bit: Buffer Data Port Register Buffer is cleared and Initialized. Present State register Buffer read Enable Buffer write Enable Read Transfer Active Write Transfer Active DAT Line Active Command Inhibit (DAT) Block Gap Control register Continue Request Stop At Block Gap Request Normal Interrupt Status register Buffer Read Ready Buffer Write Ready Block Gap Event Transfer Complete"]
    #[inline(always)]
    #[must_use]
    pub fn swrstdat(&mut self) -> SwrstdatW<ClockctrlSpec> {
        SwrstdatW::new(self, 26)
    }
}
#[doc = "Clock control\n\nYou can [`read`](crate::Reg::read) this register and get [`clockctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clockctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClockctrlSpec;
impl crate::RegisterSpec for ClockctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clockctrl::R`](R) reader structure"]
impl crate::Readable for ClockctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`clockctrl::W`](W) writer structure"]
impl crate::Writable for ClockctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLOCKCTRL to value 0"]
impl crate::Resettable for ClockctrlSpec {
    const RESET_VALUE: u32 = 0;
}
