#[doc = "Register `PINCFG29` reader"]
pub type R = crate::R<Pincfg29Spec>;
#[doc = "Register `PINCFG29` writer"]
pub type W = crate::W<Pincfg29Spec>;
#[doc = "Function select for GPIO pin 29\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fncsel29 {
    #[doc = "0: ADC trigger input"]
    Trig0 = 0,
    #[doc = "1: Output of the voltage comparator signal"]
    Vcmpo = 1,
    #[doc = "2: Bidirectional I2S Data. Operates in output mode in master mode and input mode for slave mode. (I2S Master/Slave 2)"]
    I2s0Data = 2,
    #[doc = "3: General purpose I/O"]
    Gpio = 3,
    #[doc = "4: UART Clear to Send (CTS) (UART 1)"]
    Uart1cts = 4,
    #[doc = "5: JTAG TRSTN input"]
    DspTrstn = 5,
    #[doc = "6: Timer/Counter input or output; Selection of direction is done via CTIMER register settings."]
    Ct29 = 6,
    #[doc = "7: IOMSTR/MSPI N Chip Select. Polarity is determined by CE_POLARITY field"]
    Nce29 = 7,
    #[doc = "8: Observation bus bit 13"]
    Obsbus13 = 8,
    #[doc = "9: I2S Data output (I2S Master/Slave 2)"]
    I2s0Sdout = 9,
    #[doc = "10: Reserved selection. Operation unknown if selected."]
    Reserved10 = 10,
    #[doc = "11: Fast PIO"]
    Fpio = 11,
    #[doc = "12: Reserved selection. Operation unknown if selected."]
    Reserved12 = 12,
    #[doc = "13: Reserved selection. Operation unknown if selected."]
    Reserved13 = 13,
    #[doc = "14: Reserved selection. Operation unknown if selected."]
    Reserved14 = 14,
    #[doc = "15: Internal function (SCAN)"]
    Cmle = 15,
}
impl From<Fncsel29> for u8 {
    #[inline(always)]
    fn from(variant: Fncsel29) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fncsel29 {
    type Ux = u8;
}
impl crate::IsEnum for Fncsel29 {}
#[doc = "Field `FNCSEL29` reader - Function select for GPIO pin 29"]
pub type Fncsel29R = crate::FieldReader<Fncsel29>;
impl Fncsel29R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fncsel29 {
        match self.bits {
            0 => Fncsel29::Trig0,
            1 => Fncsel29::Vcmpo,
            2 => Fncsel29::I2s0Data,
            3 => Fncsel29::Gpio,
            4 => Fncsel29::Uart1cts,
            5 => Fncsel29::DspTrstn,
            6 => Fncsel29::Ct29,
            7 => Fncsel29::Nce29,
            8 => Fncsel29::Obsbus13,
            9 => Fncsel29::I2s0Sdout,
            10 => Fncsel29::Reserved10,
            11 => Fncsel29::Fpio,
            12 => Fncsel29::Reserved12,
            13 => Fncsel29::Reserved13,
            14 => Fncsel29::Reserved14,
            15 => Fncsel29::Cmle,
            _ => unreachable!(),
        }
    }
    #[doc = "ADC trigger input"]
    #[inline(always)]
    pub fn is_trig0(&self) -> bool {
        *self == Fncsel29::Trig0
    }
    #[doc = "Output of the voltage comparator signal"]
    #[inline(always)]
    pub fn is_vcmpo(&self) -> bool {
        *self == Fncsel29::Vcmpo
    }
    #[doc = "Bidirectional I2S Data. Operates in output mode in master mode and input mode for slave mode. (I2S Master/Slave 2)"]
    #[inline(always)]
    pub fn is_i2s0_data(&self) -> bool {
        *self == Fncsel29::I2s0Data
    }
    #[doc = "General purpose I/O"]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == Fncsel29::Gpio
    }
    #[doc = "UART Clear to Send (CTS) (UART 1)"]
    #[inline(always)]
    pub fn is_uart1cts(&self) -> bool {
        *self == Fncsel29::Uart1cts
    }
    #[doc = "JTAG TRSTN input"]
    #[inline(always)]
    pub fn is_dsp_trstn(&self) -> bool {
        *self == Fncsel29::DspTrstn
    }
    #[doc = "Timer/Counter input or output; Selection of direction is done via CTIMER register settings."]
    #[inline(always)]
    pub fn is_ct29(&self) -> bool {
        *self == Fncsel29::Ct29
    }
    #[doc = "IOMSTR/MSPI N Chip Select. Polarity is determined by CE_POLARITY field"]
    #[inline(always)]
    pub fn is_nce29(&self) -> bool {
        *self == Fncsel29::Nce29
    }
    #[doc = "Observation bus bit 13"]
    #[inline(always)]
    pub fn is_obsbus13(&self) -> bool {
        *self == Fncsel29::Obsbus13
    }
    #[doc = "I2S Data output (I2S Master/Slave 2)"]
    #[inline(always)]
    pub fn is_i2s0_sdout(&self) -> bool {
        *self == Fncsel29::I2s0Sdout
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved10(&self) -> bool {
        *self == Fncsel29::Reserved10
    }
    #[doc = "Fast PIO"]
    #[inline(always)]
    pub fn is_fpio(&self) -> bool {
        *self == Fncsel29::Fpio
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved12(&self) -> bool {
        *self == Fncsel29::Reserved12
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved13(&self) -> bool {
        *self == Fncsel29::Reserved13
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved14(&self) -> bool {
        *self == Fncsel29::Reserved14
    }
    #[doc = "Internal function (SCAN)"]
    #[inline(always)]
    pub fn is_cmle(&self) -> bool {
        *self == Fncsel29::Cmle
    }
}
#[doc = "Field `FNCSEL29` writer - Function select for GPIO pin 29"]
pub type Fncsel29W<'a, REG> = crate::FieldWriter<'a, REG, 4, Fncsel29, crate::Safe>;
impl<'a, REG> Fncsel29W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ADC trigger input"]
    #[inline(always)]
    pub fn trig0(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel29::Trig0)
    }
    #[doc = "Output of the voltage comparator signal"]
    #[inline(always)]
    pub fn vcmpo(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel29::Vcmpo)
    }
    #[doc = "Bidirectional I2S Data. Operates in output mode in master mode and input mode for slave mode. (I2S Master/Slave 2)"]
    #[inline(always)]
    pub fn i2s0_data(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel29::I2s0Data)
    }
    #[doc = "General purpose I/O"]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel29::Gpio)
    }
    #[doc = "UART Clear to Send (CTS) (UART 1)"]
    #[inline(always)]
    pub fn uart1cts(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel29::Uart1cts)
    }
    #[doc = "JTAG TRSTN input"]
    #[inline(always)]
    pub fn dsp_trstn(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel29::DspTrstn)
    }
    #[doc = "Timer/Counter input or output; Selection of direction is done via CTIMER register settings."]
    #[inline(always)]
    pub fn ct29(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel29::Ct29)
    }
    #[doc = "IOMSTR/MSPI N Chip Select. Polarity is determined by CE_POLARITY field"]
    #[inline(always)]
    pub fn nce29(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel29::Nce29)
    }
    #[doc = "Observation bus bit 13"]
    #[inline(always)]
    pub fn obsbus13(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel29::Obsbus13)
    }
    #[doc = "I2S Data output (I2S Master/Slave 2)"]
    #[inline(always)]
    pub fn i2s0_sdout(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel29::I2s0Sdout)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved10(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel29::Reserved10)
    }
    #[doc = "Fast PIO"]
    #[inline(always)]
    pub fn fpio(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel29::Fpio)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved12(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel29::Reserved12)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved13(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel29::Reserved13)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved14(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel29::Reserved14)
    }
    #[doc = "Internal function (SCAN)"]
    #[inline(always)]
    pub fn cmle(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel29::Cmle)
    }
}
#[doc = "Field `INPEN29` reader - Input enable for GPIO 29"]
pub type Inpen29R = crate::BitReader;
#[doc = "Field `INPEN29` writer - Input enable for GPIO 29"]
pub type Inpen29W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDZERO29` reader - Return 0 for read data on GPIO 29"]
pub type Rdzero29R = crate::BitReader;
#[doc = "Field `RDZERO29` writer - Return 0 for read data on GPIO 29"]
pub type Rdzero29W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Interrupt enable for GPIO 29\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Irpten29 {
    #[doc = "0: Interrupts are disabled for this GPIO"]
    Dis = 0,
    #[doc = "1: Interrupts are enabled for falling edge transition on this GPIO"]
    Intfall = 1,
    #[doc = "2: Interrupts are enabled for rising edge transitions on this GPIO"]
    Intrise = 2,
    #[doc = "3: Interrupts are enabled for any edge transition on this GPIO"]
    Intany = 3,
}
impl From<Irpten29> for u8 {
    #[inline(always)]
    fn from(variant: Irpten29) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Irpten29 {
    type Ux = u8;
}
impl crate::IsEnum for Irpten29 {}
#[doc = "Field `IRPTEN29` reader - Interrupt enable for GPIO 29"]
pub type Irpten29R = crate::FieldReader<Irpten29>;
impl Irpten29R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irpten29 {
        match self.bits {
            0 => Irpten29::Dis,
            1 => Irpten29::Intfall,
            2 => Irpten29::Intrise,
            3 => Irpten29::Intany,
            _ => unreachable!(),
        }
    }
    #[doc = "Interrupts are disabled for this GPIO"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Irpten29::Dis
    }
    #[doc = "Interrupts are enabled for falling edge transition on this GPIO"]
    #[inline(always)]
    pub fn is_intfall(&self) -> bool {
        *self == Irpten29::Intfall
    }
    #[doc = "Interrupts are enabled for rising edge transitions on this GPIO"]
    #[inline(always)]
    pub fn is_intrise(&self) -> bool {
        *self == Irpten29::Intrise
    }
    #[doc = "Interrupts are enabled for any edge transition on this GPIO"]
    #[inline(always)]
    pub fn is_intany(&self) -> bool {
        *self == Irpten29::Intany
    }
}
#[doc = "Field `IRPTEN29` writer - Interrupt enable for GPIO 29"]
pub type Irpten29W<'a, REG> = crate::FieldWriter<'a, REG, 2, Irpten29, crate::Safe>;
impl<'a, REG> Irpten29W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Interrupts are disabled for this GPIO"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten29::Dis)
    }
    #[doc = "Interrupts are enabled for falling edge transition on this GPIO"]
    #[inline(always)]
    pub fn intfall(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten29::Intfall)
    }
    #[doc = "Interrupts are enabled for rising edge transitions on this GPIO"]
    #[inline(always)]
    pub fn intrise(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten29::Intrise)
    }
    #[doc = "Interrupts are enabled for any edge transition on this GPIO"]
    #[inline(always)]
    pub fn intany(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten29::Intany)
    }
}
#[doc = "Pin IO mode selection for GPIO pin 29\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Outcfg29 {
    #[doc = "0: Output Disabled"]
    Dis = 0,
    #[doc = "1: Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    Pushpull = 1,
    #[doc = "2: Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    Od = 2,
    #[doc = "3: Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    Ts = 3,
}
impl From<Outcfg29> for u8 {
    #[inline(always)]
    fn from(variant: Outcfg29) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Outcfg29 {
    type Ux = u8;
}
impl crate::IsEnum for Outcfg29 {}
#[doc = "Field `OUTCFG29` reader - Pin IO mode selection for GPIO pin 29"]
pub type Outcfg29R = crate::FieldReader<Outcfg29>;
impl Outcfg29R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Outcfg29 {
        match self.bits {
            0 => Outcfg29::Dis,
            1 => Outcfg29::Pushpull,
            2 => Outcfg29::Od,
            3 => Outcfg29::Ts,
            _ => unreachable!(),
        }
    }
    #[doc = "Output Disabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Outcfg29::Dis
    }
    #[doc = "Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == Outcfg29::Pushpull
    }
    #[doc = "Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == Outcfg29::Od
    }
    #[doc = "Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == Outcfg29::Ts
    }
}
#[doc = "Field `OUTCFG29` writer - Pin IO mode selection for GPIO pin 29"]
pub type Outcfg29W<'a, REG> = crate::FieldWriter<'a, REG, 2, Outcfg29, crate::Safe>;
impl<'a, REG> Outcfg29W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Output Disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg29::Dis)
    }
    #[doc = "Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg29::Pushpull)
    }
    #[doc = "Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    #[inline(always)]
    pub fn od(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg29::Od)
    }
    #[doc = "Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    #[inline(always)]
    pub fn ts(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg29::Ts)
    }
}
#[doc = "Drive strength selection for GPIO 29\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ds29 {
    #[doc = "0: 0.1x output driver selected"]
    _0p1x = 0,
    #[doc = "1: 0.5x output driver selected"]
    _0p5x = 1,
}
impl From<Ds29> for u8 {
    #[inline(always)]
    fn from(variant: Ds29) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ds29 {
    type Ux = u8;
}
impl crate::IsEnum for Ds29 {}
#[doc = "Field `DS29` reader - Drive strength selection for GPIO 29"]
pub type Ds29R = crate::FieldReader<Ds29>;
impl Ds29R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ds29> {
        match self.bits {
            0 => Some(Ds29::_0p1x),
            1 => Some(Ds29::_0p5x),
            _ => None,
        }
    }
    #[doc = "0.1x output driver selected"]
    #[inline(always)]
    pub fn is_0p1x(&self) -> bool {
        *self == Ds29::_0p1x
    }
    #[doc = "0.5x output driver selected"]
    #[inline(always)]
    pub fn is_0p5x(&self) -> bool {
        *self == Ds29::_0p5x
    }
}
#[doc = "Field `DS29` writer - Drive strength selection for GPIO 29"]
pub type Ds29W<'a, REG> = crate::FieldWriter<'a, REG, 2, Ds29>;
impl<'a, REG> Ds29W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "0.1x output driver selected"]
    #[inline(always)]
    pub fn _0p1x(self) -> &'a mut crate::W<REG> {
        self.variant(Ds29::_0p1x)
    }
    #[doc = "0.5x output driver selected"]
    #[inline(always)]
    pub fn _0p5x(self) -> &'a mut crate::W<REG> {
        self.variant(Ds29::_0p5x)
    }
}
#[doc = "Field `SR29` reader - Configure the slew rate"]
pub type Sr29R = crate::BitReader;
#[doc = "Field `SR29` writer - Configure the slew rate"]
pub type Sr29W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Pullup/Pulldown configuration for GPIO 29\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pullcfg29 {
    #[doc = "0: No pullup or pulldown selected"]
    Dis = 0,
    #[doc = "1: 50K Pulldown selected"]
    Pd50k = 1,
    #[doc = "2: 1.5K Pullup selected"]
    Pu15k = 2,
    #[doc = "3: 6K Pullup selected"]
    Pu6k = 3,
    #[doc = "4: 12K Pullup selected"]
    Pu12k = 4,
    #[doc = "5: 24K Pullup selected"]
    Pu24k = 5,
    #[doc = "6: 50K Pullup selected"]
    Pu50k = 6,
    #[doc = "7: 100K Pullup selected"]
    Pu100k = 7,
}
impl From<Pullcfg29> for u8 {
    #[inline(always)]
    fn from(variant: Pullcfg29) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pullcfg29 {
    type Ux = u8;
}
impl crate::IsEnum for Pullcfg29 {}
#[doc = "Field `PULLCFG29` reader - Pullup/Pulldown configuration for GPIO 29"]
pub type Pullcfg29R = crate::FieldReader<Pullcfg29>;
impl Pullcfg29R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pullcfg29 {
        match self.bits {
            0 => Pullcfg29::Dis,
            1 => Pullcfg29::Pd50k,
            2 => Pullcfg29::Pu15k,
            3 => Pullcfg29::Pu6k,
            4 => Pullcfg29::Pu12k,
            5 => Pullcfg29::Pu24k,
            6 => Pullcfg29::Pu50k,
            7 => Pullcfg29::Pu100k,
            _ => unreachable!(),
        }
    }
    #[doc = "No pullup or pulldown selected"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Pullcfg29::Dis
    }
    #[doc = "50K Pulldown selected"]
    #[inline(always)]
    pub fn is_pd50k(&self) -> bool {
        *self == Pullcfg29::Pd50k
    }
    #[doc = "1.5K Pullup selected"]
    #[inline(always)]
    pub fn is_pu15k(&self) -> bool {
        *self == Pullcfg29::Pu15k
    }
    #[doc = "6K Pullup selected"]
    #[inline(always)]
    pub fn is_pu6k(&self) -> bool {
        *self == Pullcfg29::Pu6k
    }
    #[doc = "12K Pullup selected"]
    #[inline(always)]
    pub fn is_pu12k(&self) -> bool {
        *self == Pullcfg29::Pu12k
    }
    #[doc = "24K Pullup selected"]
    #[inline(always)]
    pub fn is_pu24k(&self) -> bool {
        *self == Pullcfg29::Pu24k
    }
    #[doc = "50K Pullup selected"]
    #[inline(always)]
    pub fn is_pu50k(&self) -> bool {
        *self == Pullcfg29::Pu50k
    }
    #[doc = "100K Pullup selected"]
    #[inline(always)]
    pub fn is_pu100k(&self) -> bool {
        *self == Pullcfg29::Pu100k
    }
}
#[doc = "Field `PULLCFG29` writer - Pullup/Pulldown configuration for GPIO 29"]
pub type Pullcfg29W<'a, REG> = crate::FieldWriter<'a, REG, 3, Pullcfg29, crate::Safe>;
impl<'a, REG> Pullcfg29W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No pullup or pulldown selected"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg29::Dis)
    }
    #[doc = "50K Pulldown selected"]
    #[inline(always)]
    pub fn pd50k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg29::Pd50k)
    }
    #[doc = "1.5K Pullup selected"]
    #[inline(always)]
    pub fn pu15k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg29::Pu15k)
    }
    #[doc = "6K Pullup selected"]
    #[inline(always)]
    pub fn pu6k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg29::Pu6k)
    }
    #[doc = "12K Pullup selected"]
    #[inline(always)]
    pub fn pu12k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg29::Pu12k)
    }
    #[doc = "24K Pullup selected"]
    #[inline(always)]
    pub fn pu24k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg29::Pu24k)
    }
    #[doc = "50K Pullup selected"]
    #[inline(always)]
    pub fn pu50k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg29::Pu50k)
    }
    #[doc = "100K Pullup selected"]
    #[inline(always)]
    pub fn pu100k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg29::Pu100k)
    }
}
#[doc = "IOMSTR/MSPI N Chip Select 29, DISP control signals DE, CSX, and CS. Polarity is determined by CE_POLARITY field\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ncesrc29 {
    #[doc = "0: IOM 0 NCE 0 module"]
    Iom0ce0 = 0,
    #[doc = "1: IOM 0 NCE 1 module"]
    Iom0ce1 = 1,
    #[doc = "2: IOM 0 NCE 2 module"]
    Iom0ce2 = 2,
    #[doc = "3: IOM 0 NCE 3 module"]
    Iom0ce3 = 3,
    #[doc = "4: IOM 1 NCE 0 module"]
    Iom1ce0 = 4,
    #[doc = "5: IOM 1 NCE 1 module"]
    Iom1ce1 = 5,
    #[doc = "6: IOM 1 NCE 2 module"]
    Iom1ce2 = 6,
    #[doc = "7: IOM 1 NCE 3 module"]
    Iom1ce3 = 7,
    #[doc = "8: IOM 2 NCE 0 module"]
    Iom2ce0 = 8,
    #[doc = "9: IOM 2 NCE 1 module"]
    Iom2ce1 = 9,
    #[doc = "10: IOM 2 NCE 2 module"]
    Iom2ce2 = 10,
    #[doc = "11: IOM 2 NCE 3 module"]
    Iom2ce3 = 11,
    #[doc = "12: IOM 3 NCE 0 module"]
    Iom3ce0 = 12,
    #[doc = "13: IOM 3 NCE 1 module"]
    Iom3ce1 = 13,
    #[doc = "14: IOM 3 NCE 2 module"]
    Iom3ce2 = 14,
    #[doc = "15: IOM 3 NCE 3 module"]
    Iom3ce3 = 15,
    #[doc = "16: IOM 4 NCE 0 module"]
    Iom4ce0 = 16,
    #[doc = "17: IOM 4 NCE 1 module"]
    Iom4ce1 = 17,
    #[doc = "18: IOM 4 NCE 2 module"]
    Iom4ce2 = 18,
    #[doc = "19: IOM 4 NCE 3 module"]
    Iom4ce3 = 19,
    #[doc = "20: IOM 5 NCE 0 module"]
    Iom5ce0 = 20,
    #[doc = "21: IOM 5 NCE 1 module"]
    Iom5ce1 = 21,
    #[doc = "22: IOM 5 NCE 2 module"]
    Iom5ce2 = 22,
    #[doc = "23: IOM 5 NCE 3 module"]
    Iom5ce3 = 23,
    #[doc = "24: IOM 6 NCE 0 module"]
    Iom6ce0 = 24,
    #[doc = "25: IOM 6 NCE 1 module"]
    Iom6ce1 = 25,
    #[doc = "26: IOM 6 NCE 2 module"]
    Iom6ce2 = 26,
    #[doc = "27: IOM 6 NCE 3 module"]
    Iom6ce3 = 27,
    #[doc = "28: IOM 7 NCE 0 module"]
    Iom7ce0 = 28,
    #[doc = "29: IOM 7 NCE 1 module"]
    Iom7ce1 = 29,
    #[doc = "30: IOM 7 NCE 2 module"]
    Iom7ce2 = 30,
    #[doc = "31: IOM 7 NCE 3 module"]
    Iom7ce3 = 31,
    #[doc = "32: MSPI 0 NCE 0 module"]
    Mspi0cen0 = 32,
    #[doc = "33: MSPI 0 NCE 1 module"]
    Mspi0cen1 = 33,
    #[doc = "34: MSPI 1 NCE 0 module"]
    Mspi1cen0 = 34,
    #[doc = "35: MSPI 1 NCE 1 module"]
    Mspi1cen1 = 35,
    #[doc = "36: MSPI 2 NCE 0 module"]
    Mspi2cen0 = 36,
    #[doc = "37: MSPI 2 NCE 1 module"]
    Mspi2cen1 = 37,
    #[doc = "38: DC DPI DE module"]
    DcDpiDe = 38,
    #[doc = "39: DISP CONT CSX module"]
    DispContCsx = 39,
    #[doc = "40: DC SPI CS_N module"]
    DcSpiCsN = 40,
    #[doc = "41: DC QSPI CS_N module"]
    DcQspiCsN = 41,
    #[doc = "42: DC module RESX"]
    DcResx = 42,
}
impl From<Ncesrc29> for u8 {
    #[inline(always)]
    fn from(variant: Ncesrc29) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ncesrc29 {
    type Ux = u8;
}
impl crate::IsEnum for Ncesrc29 {}
#[doc = "Field `NCESRC29` reader - IOMSTR/MSPI N Chip Select 29, DISP control signals DE, CSX, and CS. Polarity is determined by CE_POLARITY field"]
pub type Ncesrc29R = crate::FieldReader<Ncesrc29>;
impl Ncesrc29R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ncesrc29> {
        match self.bits {
            0 => Some(Ncesrc29::Iom0ce0),
            1 => Some(Ncesrc29::Iom0ce1),
            2 => Some(Ncesrc29::Iom0ce2),
            3 => Some(Ncesrc29::Iom0ce3),
            4 => Some(Ncesrc29::Iom1ce0),
            5 => Some(Ncesrc29::Iom1ce1),
            6 => Some(Ncesrc29::Iom1ce2),
            7 => Some(Ncesrc29::Iom1ce3),
            8 => Some(Ncesrc29::Iom2ce0),
            9 => Some(Ncesrc29::Iom2ce1),
            10 => Some(Ncesrc29::Iom2ce2),
            11 => Some(Ncesrc29::Iom2ce3),
            12 => Some(Ncesrc29::Iom3ce0),
            13 => Some(Ncesrc29::Iom3ce1),
            14 => Some(Ncesrc29::Iom3ce2),
            15 => Some(Ncesrc29::Iom3ce3),
            16 => Some(Ncesrc29::Iom4ce0),
            17 => Some(Ncesrc29::Iom4ce1),
            18 => Some(Ncesrc29::Iom4ce2),
            19 => Some(Ncesrc29::Iom4ce3),
            20 => Some(Ncesrc29::Iom5ce0),
            21 => Some(Ncesrc29::Iom5ce1),
            22 => Some(Ncesrc29::Iom5ce2),
            23 => Some(Ncesrc29::Iom5ce3),
            24 => Some(Ncesrc29::Iom6ce0),
            25 => Some(Ncesrc29::Iom6ce1),
            26 => Some(Ncesrc29::Iom6ce2),
            27 => Some(Ncesrc29::Iom6ce3),
            28 => Some(Ncesrc29::Iom7ce0),
            29 => Some(Ncesrc29::Iom7ce1),
            30 => Some(Ncesrc29::Iom7ce2),
            31 => Some(Ncesrc29::Iom7ce3),
            32 => Some(Ncesrc29::Mspi0cen0),
            33 => Some(Ncesrc29::Mspi0cen1),
            34 => Some(Ncesrc29::Mspi1cen0),
            35 => Some(Ncesrc29::Mspi1cen1),
            36 => Some(Ncesrc29::Mspi2cen0),
            37 => Some(Ncesrc29::Mspi2cen1),
            38 => Some(Ncesrc29::DcDpiDe),
            39 => Some(Ncesrc29::DispContCsx),
            40 => Some(Ncesrc29::DcSpiCsN),
            41 => Some(Ncesrc29::DcQspiCsN),
            42 => Some(Ncesrc29::DcResx),
            _ => None,
        }
    }
    #[doc = "IOM 0 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom0ce0(&self) -> bool {
        *self == Ncesrc29::Iom0ce0
    }
    #[doc = "IOM 0 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom0ce1(&self) -> bool {
        *self == Ncesrc29::Iom0ce1
    }
    #[doc = "IOM 0 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom0ce2(&self) -> bool {
        *self == Ncesrc29::Iom0ce2
    }
    #[doc = "IOM 0 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom0ce3(&self) -> bool {
        *self == Ncesrc29::Iom0ce3
    }
    #[doc = "IOM 1 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom1ce0(&self) -> bool {
        *self == Ncesrc29::Iom1ce0
    }
    #[doc = "IOM 1 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom1ce1(&self) -> bool {
        *self == Ncesrc29::Iom1ce1
    }
    #[doc = "IOM 1 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom1ce2(&self) -> bool {
        *self == Ncesrc29::Iom1ce2
    }
    #[doc = "IOM 1 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom1ce3(&self) -> bool {
        *self == Ncesrc29::Iom1ce3
    }
    #[doc = "IOM 2 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom2ce0(&self) -> bool {
        *self == Ncesrc29::Iom2ce0
    }
    #[doc = "IOM 2 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom2ce1(&self) -> bool {
        *self == Ncesrc29::Iom2ce1
    }
    #[doc = "IOM 2 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom2ce2(&self) -> bool {
        *self == Ncesrc29::Iom2ce2
    }
    #[doc = "IOM 2 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom2ce3(&self) -> bool {
        *self == Ncesrc29::Iom2ce3
    }
    #[doc = "IOM 3 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom3ce0(&self) -> bool {
        *self == Ncesrc29::Iom3ce0
    }
    #[doc = "IOM 3 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom3ce1(&self) -> bool {
        *self == Ncesrc29::Iom3ce1
    }
    #[doc = "IOM 3 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom3ce2(&self) -> bool {
        *self == Ncesrc29::Iom3ce2
    }
    #[doc = "IOM 3 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom3ce3(&self) -> bool {
        *self == Ncesrc29::Iom3ce3
    }
    #[doc = "IOM 4 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom4ce0(&self) -> bool {
        *self == Ncesrc29::Iom4ce0
    }
    #[doc = "IOM 4 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom4ce1(&self) -> bool {
        *self == Ncesrc29::Iom4ce1
    }
    #[doc = "IOM 4 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom4ce2(&self) -> bool {
        *self == Ncesrc29::Iom4ce2
    }
    #[doc = "IOM 4 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom4ce3(&self) -> bool {
        *self == Ncesrc29::Iom4ce3
    }
    #[doc = "IOM 5 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom5ce0(&self) -> bool {
        *self == Ncesrc29::Iom5ce0
    }
    #[doc = "IOM 5 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom5ce1(&self) -> bool {
        *self == Ncesrc29::Iom5ce1
    }
    #[doc = "IOM 5 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom5ce2(&self) -> bool {
        *self == Ncesrc29::Iom5ce2
    }
    #[doc = "IOM 5 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom5ce3(&self) -> bool {
        *self == Ncesrc29::Iom5ce3
    }
    #[doc = "IOM 6 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom6ce0(&self) -> bool {
        *self == Ncesrc29::Iom6ce0
    }
    #[doc = "IOM 6 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom6ce1(&self) -> bool {
        *self == Ncesrc29::Iom6ce1
    }
    #[doc = "IOM 6 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom6ce2(&self) -> bool {
        *self == Ncesrc29::Iom6ce2
    }
    #[doc = "IOM 6 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom6ce3(&self) -> bool {
        *self == Ncesrc29::Iom6ce3
    }
    #[doc = "IOM 7 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom7ce0(&self) -> bool {
        *self == Ncesrc29::Iom7ce0
    }
    #[doc = "IOM 7 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom7ce1(&self) -> bool {
        *self == Ncesrc29::Iom7ce1
    }
    #[doc = "IOM 7 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom7ce2(&self) -> bool {
        *self == Ncesrc29::Iom7ce2
    }
    #[doc = "IOM 7 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom7ce3(&self) -> bool {
        *self == Ncesrc29::Iom7ce3
    }
    #[doc = "MSPI 0 NCE 0 module"]
    #[inline(always)]
    pub fn is_mspi0cen0(&self) -> bool {
        *self == Ncesrc29::Mspi0cen0
    }
    #[doc = "MSPI 0 NCE 1 module"]
    #[inline(always)]
    pub fn is_mspi0cen1(&self) -> bool {
        *self == Ncesrc29::Mspi0cen1
    }
    #[doc = "MSPI 1 NCE 0 module"]
    #[inline(always)]
    pub fn is_mspi1cen0(&self) -> bool {
        *self == Ncesrc29::Mspi1cen0
    }
    #[doc = "MSPI 1 NCE 1 module"]
    #[inline(always)]
    pub fn is_mspi1cen1(&self) -> bool {
        *self == Ncesrc29::Mspi1cen1
    }
    #[doc = "MSPI 2 NCE 0 module"]
    #[inline(always)]
    pub fn is_mspi2cen0(&self) -> bool {
        *self == Ncesrc29::Mspi2cen0
    }
    #[doc = "MSPI 2 NCE 1 module"]
    #[inline(always)]
    pub fn is_mspi2cen1(&self) -> bool {
        *self == Ncesrc29::Mspi2cen1
    }
    #[doc = "DC DPI DE module"]
    #[inline(always)]
    pub fn is_dc_dpi_de(&self) -> bool {
        *self == Ncesrc29::DcDpiDe
    }
    #[doc = "DISP CONT CSX module"]
    #[inline(always)]
    pub fn is_disp_cont_csx(&self) -> bool {
        *self == Ncesrc29::DispContCsx
    }
    #[doc = "DC SPI CS_N module"]
    #[inline(always)]
    pub fn is_dc_spi_cs_n(&self) -> bool {
        *self == Ncesrc29::DcSpiCsN
    }
    #[doc = "DC QSPI CS_N module"]
    #[inline(always)]
    pub fn is_dc_qspi_cs_n(&self) -> bool {
        *self == Ncesrc29::DcQspiCsN
    }
    #[doc = "DC module RESX"]
    #[inline(always)]
    pub fn is_dc_resx(&self) -> bool {
        *self == Ncesrc29::DcResx
    }
}
#[doc = "Field `NCESRC29` writer - IOMSTR/MSPI N Chip Select 29, DISP control signals DE, CSX, and CS. Polarity is determined by CE_POLARITY field"]
pub type Ncesrc29W<'a, REG> = crate::FieldWriter<'a, REG, 6, Ncesrc29>;
impl<'a, REG> Ncesrc29W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "IOM 0 NCE 0 module"]
    #[inline(always)]
    pub fn iom0ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc29::Iom0ce0)
    }
    #[doc = "IOM 0 NCE 1 module"]
    #[inline(always)]
    pub fn iom0ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc29::Iom0ce1)
    }
    #[doc = "IOM 0 NCE 2 module"]
    #[inline(always)]
    pub fn iom0ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc29::Iom0ce2)
    }
    #[doc = "IOM 0 NCE 3 module"]
    #[inline(always)]
    pub fn iom0ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc29::Iom0ce3)
    }
    #[doc = "IOM 1 NCE 0 module"]
    #[inline(always)]
    pub fn iom1ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc29::Iom1ce0)
    }
    #[doc = "IOM 1 NCE 1 module"]
    #[inline(always)]
    pub fn iom1ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc29::Iom1ce1)
    }
    #[doc = "IOM 1 NCE 2 module"]
    #[inline(always)]
    pub fn iom1ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc29::Iom1ce2)
    }
    #[doc = "IOM 1 NCE 3 module"]
    #[inline(always)]
    pub fn iom1ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc29::Iom1ce3)
    }
    #[doc = "IOM 2 NCE 0 module"]
    #[inline(always)]
    pub fn iom2ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc29::Iom2ce0)
    }
    #[doc = "IOM 2 NCE 1 module"]
    #[inline(always)]
    pub fn iom2ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc29::Iom2ce1)
    }
    #[doc = "IOM 2 NCE 2 module"]
    #[inline(always)]
    pub fn iom2ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc29::Iom2ce2)
    }
    #[doc = "IOM 2 NCE 3 module"]
    #[inline(always)]
    pub fn iom2ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc29::Iom2ce3)
    }
    #[doc = "IOM 3 NCE 0 module"]
    #[inline(always)]
    pub fn iom3ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc29::Iom3ce0)
    }
    #[doc = "IOM 3 NCE 1 module"]
    #[inline(always)]
    pub fn iom3ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc29::Iom3ce1)
    }
    #[doc = "IOM 3 NCE 2 module"]
    #[inline(always)]
    pub fn iom3ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc29::Iom3ce2)
    }
    #[doc = "IOM 3 NCE 3 module"]
    #[inline(always)]
    pub fn iom3ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc29::Iom3ce3)
    }
    #[doc = "IOM 4 NCE 0 module"]
    #[inline(always)]
    pub fn iom4ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc29::Iom4ce0)
    }
    #[doc = "IOM 4 NCE 1 module"]
    #[inline(always)]
    pub fn iom4ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc29::Iom4ce1)
    }
    #[doc = "IOM 4 NCE 2 module"]
    #[inline(always)]
    pub fn iom4ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc29::Iom4ce2)
    }
    #[doc = "IOM 4 NCE 3 module"]
    #[inline(always)]
    pub fn iom4ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc29::Iom4ce3)
    }
    #[doc = "IOM 5 NCE 0 module"]
    #[inline(always)]
    pub fn iom5ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc29::Iom5ce0)
    }
    #[doc = "IOM 5 NCE 1 module"]
    #[inline(always)]
    pub fn iom5ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc29::Iom5ce1)
    }
    #[doc = "IOM 5 NCE 2 module"]
    #[inline(always)]
    pub fn iom5ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc29::Iom5ce2)
    }
    #[doc = "IOM 5 NCE 3 module"]
    #[inline(always)]
    pub fn iom5ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc29::Iom5ce3)
    }
    #[doc = "IOM 6 NCE 0 module"]
    #[inline(always)]
    pub fn iom6ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc29::Iom6ce0)
    }
    #[doc = "IOM 6 NCE 1 module"]
    #[inline(always)]
    pub fn iom6ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc29::Iom6ce1)
    }
    #[doc = "IOM 6 NCE 2 module"]
    #[inline(always)]
    pub fn iom6ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc29::Iom6ce2)
    }
    #[doc = "IOM 6 NCE 3 module"]
    #[inline(always)]
    pub fn iom6ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc29::Iom6ce3)
    }
    #[doc = "IOM 7 NCE 0 module"]
    #[inline(always)]
    pub fn iom7ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc29::Iom7ce0)
    }
    #[doc = "IOM 7 NCE 1 module"]
    #[inline(always)]
    pub fn iom7ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc29::Iom7ce1)
    }
    #[doc = "IOM 7 NCE 2 module"]
    #[inline(always)]
    pub fn iom7ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc29::Iom7ce2)
    }
    #[doc = "IOM 7 NCE 3 module"]
    #[inline(always)]
    pub fn iom7ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc29::Iom7ce3)
    }
    #[doc = "MSPI 0 NCE 0 module"]
    #[inline(always)]
    pub fn mspi0cen0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc29::Mspi0cen0)
    }
    #[doc = "MSPI 0 NCE 1 module"]
    #[inline(always)]
    pub fn mspi0cen1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc29::Mspi0cen1)
    }
    #[doc = "MSPI 1 NCE 0 module"]
    #[inline(always)]
    pub fn mspi1cen0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc29::Mspi1cen0)
    }
    #[doc = "MSPI 1 NCE 1 module"]
    #[inline(always)]
    pub fn mspi1cen1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc29::Mspi1cen1)
    }
    #[doc = "MSPI 2 NCE 0 module"]
    #[inline(always)]
    pub fn mspi2cen0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc29::Mspi2cen0)
    }
    #[doc = "MSPI 2 NCE 1 module"]
    #[inline(always)]
    pub fn mspi2cen1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc29::Mspi2cen1)
    }
    #[doc = "DC DPI DE module"]
    #[inline(always)]
    pub fn dc_dpi_de(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc29::DcDpiDe)
    }
    #[doc = "DISP CONT CSX module"]
    #[inline(always)]
    pub fn disp_cont_csx(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc29::DispContCsx)
    }
    #[doc = "DC SPI CS_N module"]
    #[inline(always)]
    pub fn dc_spi_cs_n(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc29::DcSpiCsN)
    }
    #[doc = "DC QSPI CS_N module"]
    #[inline(always)]
    pub fn dc_qspi_cs_n(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc29::DcQspiCsN)
    }
    #[doc = "DC module RESX"]
    #[inline(always)]
    pub fn dc_resx(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc29::DcResx)
    }
}
#[doc = "Polarity select for NCE for GPIO 29\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ncepol29 {
    #[doc = "0: Polarity is active low"]
    Low = 0,
    #[doc = "1: Polarity is active high"]
    High = 1,
}
impl From<Ncepol29> for bool {
    #[inline(always)]
    fn from(variant: Ncepol29) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NCEPOL29` reader - Polarity select for NCE for GPIO 29"]
pub type Ncepol29R = crate::BitReader<Ncepol29>;
impl Ncepol29R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ncepol29 {
        match self.bits {
            false => Ncepol29::Low,
            true => Ncepol29::High,
        }
    }
    #[doc = "Polarity is active low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Ncepol29::Low
    }
    #[doc = "Polarity is active high"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Ncepol29::High
    }
}
#[doc = "Field `NCEPOL29` writer - Polarity select for NCE for GPIO 29"]
pub type Ncepol29W<'a, REG> = crate::BitWriter<'a, REG, Ncepol29>;
impl<'a, REG> Ncepol29W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Polarity is active low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Ncepol29::Low)
    }
    #[doc = "Polarity is active high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Ncepol29::High)
    }
}
#[doc = "Field `FIEN29` reader - Force input enable active regardless of function selected. Otherwise the selected function will enable the input only when needed"]
pub type Fien29R = crate::BitReader;
#[doc = "Field `FIEN29` writer - Force input enable active regardless of function selected. Otherwise the selected function will enable the input only when needed"]
pub type Fien29W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FOEN29` reader - Force output enable active regardless of function selected. Otherwise the selected function will enable the output only when needed"]
pub type Foen29R = crate::BitReader;
#[doc = "Field `FOEN29` writer - Force output enable active regardless of function selected. Otherwise the selected function will enable the output only when needed"]
pub type Foen29W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Function select for GPIO pin 29"]
    #[inline(always)]
    pub fn fncsel29(&self) -> Fncsel29R {
        Fncsel29R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Input enable for GPIO 29"]
    #[inline(always)]
    pub fn inpen29(&self) -> Inpen29R {
        Inpen29R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Return 0 for read data on GPIO 29"]
    #[inline(always)]
    pub fn rdzero29(&self) -> Rdzero29R {
        Rdzero29R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Interrupt enable for GPIO 29"]
    #[inline(always)]
    pub fn irpten29(&self) -> Irpten29R {
        Irpten29R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Pin IO mode selection for GPIO pin 29"]
    #[inline(always)]
    pub fn outcfg29(&self) -> Outcfg29R {
        Outcfg29R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Drive strength selection for GPIO 29"]
    #[inline(always)]
    pub fn ds29(&self) -> Ds29R {
        Ds29R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - Configure the slew rate"]
    #[inline(always)]
    pub fn sr29(&self) -> Sr29R {
        Sr29R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - Pullup/Pulldown configuration for GPIO 29"]
    #[inline(always)]
    pub fn pullcfg29(&self) -> Pullcfg29R {
        Pullcfg29R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:21 - IOMSTR/MSPI N Chip Select 29, DISP control signals DE, CSX, and CS. Polarity is determined by CE_POLARITY field"]
    #[inline(always)]
    pub fn ncesrc29(&self) -> Ncesrc29R {
        Ncesrc29R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 22 - Polarity select for NCE for GPIO 29"]
    #[inline(always)]
    pub fn ncepol29(&self) -> Ncepol29R {
        Ncepol29R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 26 - Force input enable active regardless of function selected. Otherwise the selected function will enable the input only when needed"]
    #[inline(always)]
    pub fn fien29(&self) -> Fien29R {
        Fien29R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Force output enable active regardless of function selected. Otherwise the selected function will enable the output only when needed"]
    #[inline(always)]
    pub fn foen29(&self) -> Foen29R {
        Foen29R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Function select for GPIO pin 29"]
    #[inline(always)]
    #[must_use]
    pub fn fncsel29(&mut self) -> Fncsel29W<Pincfg29Spec> {
        Fncsel29W::new(self, 0)
    }
    #[doc = "Bit 4 - Input enable for GPIO 29"]
    #[inline(always)]
    #[must_use]
    pub fn inpen29(&mut self) -> Inpen29W<Pincfg29Spec> {
        Inpen29W::new(self, 4)
    }
    #[doc = "Bit 5 - Return 0 for read data on GPIO 29"]
    #[inline(always)]
    #[must_use]
    pub fn rdzero29(&mut self) -> Rdzero29W<Pincfg29Spec> {
        Rdzero29W::new(self, 5)
    }
    #[doc = "Bits 6:7 - Interrupt enable for GPIO 29"]
    #[inline(always)]
    #[must_use]
    pub fn irpten29(&mut self) -> Irpten29W<Pincfg29Spec> {
        Irpten29W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Pin IO mode selection for GPIO pin 29"]
    #[inline(always)]
    #[must_use]
    pub fn outcfg29(&mut self) -> Outcfg29W<Pincfg29Spec> {
        Outcfg29W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Drive strength selection for GPIO 29"]
    #[inline(always)]
    #[must_use]
    pub fn ds29(&mut self) -> Ds29W<Pincfg29Spec> {
        Ds29W::new(self, 10)
    }
    #[doc = "Bit 12 - Configure the slew rate"]
    #[inline(always)]
    #[must_use]
    pub fn sr29(&mut self) -> Sr29W<Pincfg29Spec> {
        Sr29W::new(self, 12)
    }
    #[doc = "Bits 13:15 - Pullup/Pulldown configuration for GPIO 29"]
    #[inline(always)]
    #[must_use]
    pub fn pullcfg29(&mut self) -> Pullcfg29W<Pincfg29Spec> {
        Pullcfg29W::new(self, 13)
    }
    #[doc = "Bits 16:21 - IOMSTR/MSPI N Chip Select 29, DISP control signals DE, CSX, and CS. Polarity is determined by CE_POLARITY field"]
    #[inline(always)]
    #[must_use]
    pub fn ncesrc29(&mut self) -> Ncesrc29W<Pincfg29Spec> {
        Ncesrc29W::new(self, 16)
    }
    #[doc = "Bit 22 - Polarity select for NCE for GPIO 29"]
    #[inline(always)]
    #[must_use]
    pub fn ncepol29(&mut self) -> Ncepol29W<Pincfg29Spec> {
        Ncepol29W::new(self, 22)
    }
    #[doc = "Bit 26 - Force input enable active regardless of function selected. Otherwise the selected function will enable the input only when needed"]
    #[inline(always)]
    #[must_use]
    pub fn fien29(&mut self) -> Fien29W<Pincfg29Spec> {
        Fien29W::new(self, 26)
    }
    #[doc = "Bit 27 - Force output enable active regardless of function selected. Otherwise the selected function will enable the output only when needed"]
    #[inline(always)]
    #[must_use]
    pub fn foen29(&mut self) -> Foen29W<Pincfg29Spec> {
        Foen29W::new(self, 27)
    }
}
#[doc = "Controls the operation of GPIO pin 29.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg29::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg29::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pincfg29Spec;
impl crate::RegisterSpec for Pincfg29Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pincfg29::R`](R) reader structure"]
impl crate::Readable for Pincfg29Spec {}
#[doc = "`write(|w| ..)` method takes [`pincfg29::W`](W) writer structure"]
impl crate::Writable for Pincfg29Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PINCFG29 to value 0x03"]
impl crate::Resettable for Pincfg29Spec {
    const RESET_VALUE: u32 = 0x03;
}
