#[doc = "Register `PINCFG7` reader"]
pub type R = crate::R<Pincfg7Spec>;
#[doc = "Register `PINCFG7` writer"]
pub type W = crate::W<Pincfg7Spec>;
#[doc = "Function select for GPIO pin 7\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fncsel7 {
    #[doc = "0: Serial SPI MASTER MISO input (IOM 0)"]
    M0miso = 0,
    #[doc = "1: ADC trigger input"]
    Trig0 = 1,
    #[doc = "2: Bidirectional I2S L/R clock. Operates in output mode in master mode and input mode for slave mode. (I2S Master/Slave 2)"]
    I2s0Ws = 2,
    #[doc = "3: General purpose I/O"]
    Gpio = 3,
    #[doc = "4: UART Clear to Send (CTS) (UART 2)"]
    Uart2cts = 4,
    #[doc = "5: UART Clear to Send (CTS) (UART 3)"]
    Uart3cts = 5,
    #[doc = "6: Timer/Counter input or output; Selection of direction is done via CTIMER register settings."]
    Ct7 = 6,
    #[doc = "7: IOMSTR/MSPI N Chip Select. Polarity is determined by CE_POLARITY field"]
    Nce7 = 7,
    #[doc = "8: Observation bus bit 7"]
    Obsbus7 = 8,
    #[doc = "9: Reserved selection. Operation unknown if selected."]
    Reserved9 = 9,
    #[doc = "10: Bidirectional I2S L/R clock. Operates in output mode in master mode and input mode for slave mode. (I2S Master/Slave 2)"]
    I2s1Ws = 10,
    #[doc = "11: Fast PIO"]
    Fpio = 11,
    #[doc = "12: Reserved selection. Operation unknown if selected."]
    Reserved12 = 12,
    #[doc = "13: Reserved selection. Operation unknown if selected."]
    Reserved13 = 13,
    #[doc = "14: Reserved selection. Operation unknown if selected."]
    Reserved14 = 14,
    #[doc = "15: Internal function (SCAN)"]
    Scanin7 = 15,
}
impl From<Fncsel7> for u8 {
    #[inline(always)]
    fn from(variant: Fncsel7) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fncsel7 {
    type Ux = u8;
}
impl crate::IsEnum for Fncsel7 {}
#[doc = "Field `FNCSEL7` reader - Function select for GPIO pin 7"]
pub type Fncsel7R = crate::FieldReader<Fncsel7>;
impl Fncsel7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fncsel7 {
        match self.bits {
            0 => Fncsel7::M0miso,
            1 => Fncsel7::Trig0,
            2 => Fncsel7::I2s0Ws,
            3 => Fncsel7::Gpio,
            4 => Fncsel7::Uart2cts,
            5 => Fncsel7::Uart3cts,
            6 => Fncsel7::Ct7,
            7 => Fncsel7::Nce7,
            8 => Fncsel7::Obsbus7,
            9 => Fncsel7::Reserved9,
            10 => Fncsel7::I2s1Ws,
            11 => Fncsel7::Fpio,
            12 => Fncsel7::Reserved12,
            13 => Fncsel7::Reserved13,
            14 => Fncsel7::Reserved14,
            15 => Fncsel7::Scanin7,
            _ => unreachable!(),
        }
    }
    #[doc = "Serial SPI MASTER MISO input (IOM 0)"]
    #[inline(always)]
    pub fn is_m0miso(&self) -> bool {
        *self == Fncsel7::M0miso
    }
    #[doc = "ADC trigger input"]
    #[inline(always)]
    pub fn is_trig0(&self) -> bool {
        *self == Fncsel7::Trig0
    }
    #[doc = "Bidirectional I2S L/R clock. Operates in output mode in master mode and input mode for slave mode. (I2S Master/Slave 2)"]
    #[inline(always)]
    pub fn is_i2s0_ws(&self) -> bool {
        *self == Fncsel7::I2s0Ws
    }
    #[doc = "General purpose I/O"]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == Fncsel7::Gpio
    }
    #[doc = "UART Clear to Send (CTS) (UART 2)"]
    #[inline(always)]
    pub fn is_uart2cts(&self) -> bool {
        *self == Fncsel7::Uart2cts
    }
    #[doc = "UART Clear to Send (CTS) (UART 3)"]
    #[inline(always)]
    pub fn is_uart3cts(&self) -> bool {
        *self == Fncsel7::Uart3cts
    }
    #[doc = "Timer/Counter input or output; Selection of direction is done via CTIMER register settings."]
    #[inline(always)]
    pub fn is_ct7(&self) -> bool {
        *self == Fncsel7::Ct7
    }
    #[doc = "IOMSTR/MSPI N Chip Select. Polarity is determined by CE_POLARITY field"]
    #[inline(always)]
    pub fn is_nce7(&self) -> bool {
        *self == Fncsel7::Nce7
    }
    #[doc = "Observation bus bit 7"]
    #[inline(always)]
    pub fn is_obsbus7(&self) -> bool {
        *self == Fncsel7::Obsbus7
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved9(&self) -> bool {
        *self == Fncsel7::Reserved9
    }
    #[doc = "Bidirectional I2S L/R clock. Operates in output mode in master mode and input mode for slave mode. (I2S Master/Slave 2)"]
    #[inline(always)]
    pub fn is_i2s1_ws(&self) -> bool {
        *self == Fncsel7::I2s1Ws
    }
    #[doc = "Fast PIO"]
    #[inline(always)]
    pub fn is_fpio(&self) -> bool {
        *self == Fncsel7::Fpio
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved12(&self) -> bool {
        *self == Fncsel7::Reserved12
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved13(&self) -> bool {
        *self == Fncsel7::Reserved13
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved14(&self) -> bool {
        *self == Fncsel7::Reserved14
    }
    #[doc = "Internal function (SCAN)"]
    #[inline(always)]
    pub fn is_scanin7(&self) -> bool {
        *self == Fncsel7::Scanin7
    }
}
#[doc = "Field `FNCSEL7` writer - Function select for GPIO pin 7"]
pub type Fncsel7W<'a, REG> = crate::FieldWriter<'a, REG, 4, Fncsel7, crate::Safe>;
impl<'a, REG> Fncsel7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Serial SPI MASTER MISO input (IOM 0)"]
    #[inline(always)]
    pub fn m0miso(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel7::M0miso)
    }
    #[doc = "ADC trigger input"]
    #[inline(always)]
    pub fn trig0(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel7::Trig0)
    }
    #[doc = "Bidirectional I2S L/R clock. Operates in output mode in master mode and input mode for slave mode. (I2S Master/Slave 2)"]
    #[inline(always)]
    pub fn i2s0_ws(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel7::I2s0Ws)
    }
    #[doc = "General purpose I/O"]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel7::Gpio)
    }
    #[doc = "UART Clear to Send (CTS) (UART 2)"]
    #[inline(always)]
    pub fn uart2cts(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel7::Uart2cts)
    }
    #[doc = "UART Clear to Send (CTS) (UART 3)"]
    #[inline(always)]
    pub fn uart3cts(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel7::Uart3cts)
    }
    #[doc = "Timer/Counter input or output; Selection of direction is done via CTIMER register settings."]
    #[inline(always)]
    pub fn ct7(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel7::Ct7)
    }
    #[doc = "IOMSTR/MSPI N Chip Select. Polarity is determined by CE_POLARITY field"]
    #[inline(always)]
    pub fn nce7(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel7::Nce7)
    }
    #[doc = "Observation bus bit 7"]
    #[inline(always)]
    pub fn obsbus7(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel7::Obsbus7)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved9(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel7::Reserved9)
    }
    #[doc = "Bidirectional I2S L/R clock. Operates in output mode in master mode and input mode for slave mode. (I2S Master/Slave 2)"]
    #[inline(always)]
    pub fn i2s1_ws(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel7::I2s1Ws)
    }
    #[doc = "Fast PIO"]
    #[inline(always)]
    pub fn fpio(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel7::Fpio)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved12(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel7::Reserved12)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved13(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel7::Reserved13)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved14(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel7::Reserved14)
    }
    #[doc = "Internal function (SCAN)"]
    #[inline(always)]
    pub fn scanin7(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel7::Scanin7)
    }
}
#[doc = "Field `INPEN7` reader - Input enable for GPIO 7"]
pub type Inpen7R = crate::BitReader;
#[doc = "Field `INPEN7` writer - Input enable for GPIO 7"]
pub type Inpen7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDZERO7` reader - Return 0 for read data on GPIO 7"]
pub type Rdzero7R = crate::BitReader;
#[doc = "Field `RDZERO7` writer - Return 0 for read data on GPIO 7"]
pub type Rdzero7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Interrupt enable for GPIO 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Irpten7 {
    #[doc = "0: Interrupts are disabled for this GPIO"]
    Dis = 0,
    #[doc = "1: Interrupts are enabled for falling edge transition on this GPIO"]
    Intfall = 1,
    #[doc = "2: Interrupts are enabled for rising edge transitions on this GPIO"]
    Intrise = 2,
    #[doc = "3: Interrupts are enabled for any edge transition on this GPIO"]
    Intany = 3,
}
impl From<Irpten7> for u8 {
    #[inline(always)]
    fn from(variant: Irpten7) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Irpten7 {
    type Ux = u8;
}
impl crate::IsEnum for Irpten7 {}
#[doc = "Field `IRPTEN7` reader - Interrupt enable for GPIO 7"]
pub type Irpten7R = crate::FieldReader<Irpten7>;
impl Irpten7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irpten7 {
        match self.bits {
            0 => Irpten7::Dis,
            1 => Irpten7::Intfall,
            2 => Irpten7::Intrise,
            3 => Irpten7::Intany,
            _ => unreachable!(),
        }
    }
    #[doc = "Interrupts are disabled for this GPIO"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Irpten7::Dis
    }
    #[doc = "Interrupts are enabled for falling edge transition on this GPIO"]
    #[inline(always)]
    pub fn is_intfall(&self) -> bool {
        *self == Irpten7::Intfall
    }
    #[doc = "Interrupts are enabled for rising edge transitions on this GPIO"]
    #[inline(always)]
    pub fn is_intrise(&self) -> bool {
        *self == Irpten7::Intrise
    }
    #[doc = "Interrupts are enabled for any edge transition on this GPIO"]
    #[inline(always)]
    pub fn is_intany(&self) -> bool {
        *self == Irpten7::Intany
    }
}
#[doc = "Field `IRPTEN7` writer - Interrupt enable for GPIO 7"]
pub type Irpten7W<'a, REG> = crate::FieldWriter<'a, REG, 2, Irpten7, crate::Safe>;
impl<'a, REG> Irpten7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Interrupts are disabled for this GPIO"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten7::Dis)
    }
    #[doc = "Interrupts are enabled for falling edge transition on this GPIO"]
    #[inline(always)]
    pub fn intfall(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten7::Intfall)
    }
    #[doc = "Interrupts are enabled for rising edge transitions on this GPIO"]
    #[inline(always)]
    pub fn intrise(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten7::Intrise)
    }
    #[doc = "Interrupts are enabled for any edge transition on this GPIO"]
    #[inline(always)]
    pub fn intany(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten7::Intany)
    }
}
#[doc = "Pin IO mode selection for GPIO pin 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Outcfg7 {
    #[doc = "0: Output Disabled"]
    Dis = 0,
    #[doc = "1: Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    Pushpull = 1,
    #[doc = "2: Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    Od = 2,
    #[doc = "3: Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    Ts = 3,
}
impl From<Outcfg7> for u8 {
    #[inline(always)]
    fn from(variant: Outcfg7) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Outcfg7 {
    type Ux = u8;
}
impl crate::IsEnum for Outcfg7 {}
#[doc = "Field `OUTCFG7` reader - Pin IO mode selection for GPIO pin 7"]
pub type Outcfg7R = crate::FieldReader<Outcfg7>;
impl Outcfg7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Outcfg7 {
        match self.bits {
            0 => Outcfg7::Dis,
            1 => Outcfg7::Pushpull,
            2 => Outcfg7::Od,
            3 => Outcfg7::Ts,
            _ => unreachable!(),
        }
    }
    #[doc = "Output Disabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Outcfg7::Dis
    }
    #[doc = "Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == Outcfg7::Pushpull
    }
    #[doc = "Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == Outcfg7::Od
    }
    #[doc = "Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == Outcfg7::Ts
    }
}
#[doc = "Field `OUTCFG7` writer - Pin IO mode selection for GPIO pin 7"]
pub type Outcfg7W<'a, REG> = crate::FieldWriter<'a, REG, 2, Outcfg7, crate::Safe>;
impl<'a, REG> Outcfg7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Output Disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg7::Dis)
    }
    #[doc = "Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg7::Pushpull)
    }
    #[doc = "Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    #[inline(always)]
    pub fn od(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg7::Od)
    }
    #[doc = "Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    #[inline(always)]
    pub fn ts(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg7::Ts)
    }
}
#[doc = "Drive strength selection for GPIO 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ds7 {
    #[doc = "0: 0.1x output driver selected"]
    _0p1x = 0,
    #[doc = "1: 0.5x output driver selected"]
    _0p5x = 1,
    #[doc = "2: 0.75x output driver selected"]
    _0p75x = 2,
    #[doc = "3: 1.0x output driver selected"]
    _1p0x = 3,
}
impl From<Ds7> for u8 {
    #[inline(always)]
    fn from(variant: Ds7) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ds7 {
    type Ux = u8;
}
impl crate::IsEnum for Ds7 {}
#[doc = "Field `DS7` reader - Drive strength selection for GPIO 7"]
pub type Ds7R = crate::FieldReader<Ds7>;
impl Ds7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ds7 {
        match self.bits {
            0 => Ds7::_0p1x,
            1 => Ds7::_0p5x,
            2 => Ds7::_0p75x,
            3 => Ds7::_1p0x,
            _ => unreachable!(),
        }
    }
    #[doc = "0.1x output driver selected"]
    #[inline(always)]
    pub fn is_0p1x(&self) -> bool {
        *self == Ds7::_0p1x
    }
    #[doc = "0.5x output driver selected"]
    #[inline(always)]
    pub fn is_0p5x(&self) -> bool {
        *self == Ds7::_0p5x
    }
    #[doc = "0.75x output driver selected"]
    #[inline(always)]
    pub fn is_0p75x(&self) -> bool {
        *self == Ds7::_0p75x
    }
    #[doc = "1.0x output driver selected"]
    #[inline(always)]
    pub fn is_1p0x(&self) -> bool {
        *self == Ds7::_1p0x
    }
}
#[doc = "Field `DS7` writer - Drive strength selection for GPIO 7"]
pub type Ds7W<'a, REG> = crate::FieldWriter<'a, REG, 2, Ds7, crate::Safe>;
impl<'a, REG> Ds7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "0.1x output driver selected"]
    #[inline(always)]
    pub fn _0p1x(self) -> &'a mut crate::W<REG> {
        self.variant(Ds7::_0p1x)
    }
    #[doc = "0.5x output driver selected"]
    #[inline(always)]
    pub fn _0p5x(self) -> &'a mut crate::W<REG> {
        self.variant(Ds7::_0p5x)
    }
    #[doc = "0.75x output driver selected"]
    #[inline(always)]
    pub fn _0p75x(self) -> &'a mut crate::W<REG> {
        self.variant(Ds7::_0p75x)
    }
    #[doc = "1.0x output driver selected"]
    #[inline(always)]
    pub fn _1p0x(self) -> &'a mut crate::W<REG> {
        self.variant(Ds7::_1p0x)
    }
}
#[doc = "Field `SR7` reader - Configure the slew rate"]
pub type Sr7R = crate::BitReader;
#[doc = "Field `SR7` writer - Configure the slew rate"]
pub type Sr7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Pullup/Pulldown configuration for GPIO 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pullcfg7 {
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
impl From<Pullcfg7> for u8 {
    #[inline(always)]
    fn from(variant: Pullcfg7) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pullcfg7 {
    type Ux = u8;
}
impl crate::IsEnum for Pullcfg7 {}
#[doc = "Field `PULLCFG7` reader - Pullup/Pulldown configuration for GPIO 7"]
pub type Pullcfg7R = crate::FieldReader<Pullcfg7>;
impl Pullcfg7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pullcfg7 {
        match self.bits {
            0 => Pullcfg7::Dis,
            1 => Pullcfg7::Pd50k,
            2 => Pullcfg7::Pu15k,
            3 => Pullcfg7::Pu6k,
            4 => Pullcfg7::Pu12k,
            5 => Pullcfg7::Pu24k,
            6 => Pullcfg7::Pu50k,
            7 => Pullcfg7::Pu100k,
            _ => unreachable!(),
        }
    }
    #[doc = "No pullup or pulldown selected"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Pullcfg7::Dis
    }
    #[doc = "50K Pulldown selected"]
    #[inline(always)]
    pub fn is_pd50k(&self) -> bool {
        *self == Pullcfg7::Pd50k
    }
    #[doc = "1.5K Pullup selected"]
    #[inline(always)]
    pub fn is_pu15k(&self) -> bool {
        *self == Pullcfg7::Pu15k
    }
    #[doc = "6K Pullup selected"]
    #[inline(always)]
    pub fn is_pu6k(&self) -> bool {
        *self == Pullcfg7::Pu6k
    }
    #[doc = "12K Pullup selected"]
    #[inline(always)]
    pub fn is_pu12k(&self) -> bool {
        *self == Pullcfg7::Pu12k
    }
    #[doc = "24K Pullup selected"]
    #[inline(always)]
    pub fn is_pu24k(&self) -> bool {
        *self == Pullcfg7::Pu24k
    }
    #[doc = "50K Pullup selected"]
    #[inline(always)]
    pub fn is_pu50k(&self) -> bool {
        *self == Pullcfg7::Pu50k
    }
    #[doc = "100K Pullup selected"]
    #[inline(always)]
    pub fn is_pu100k(&self) -> bool {
        *self == Pullcfg7::Pu100k
    }
}
#[doc = "Field `PULLCFG7` writer - Pullup/Pulldown configuration for GPIO 7"]
pub type Pullcfg7W<'a, REG> = crate::FieldWriter<'a, REG, 3, Pullcfg7, crate::Safe>;
impl<'a, REG> Pullcfg7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No pullup or pulldown selected"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg7::Dis)
    }
    #[doc = "50K Pulldown selected"]
    #[inline(always)]
    pub fn pd50k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg7::Pd50k)
    }
    #[doc = "1.5K Pullup selected"]
    #[inline(always)]
    pub fn pu15k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg7::Pu15k)
    }
    #[doc = "6K Pullup selected"]
    #[inline(always)]
    pub fn pu6k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg7::Pu6k)
    }
    #[doc = "12K Pullup selected"]
    #[inline(always)]
    pub fn pu12k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg7::Pu12k)
    }
    #[doc = "24K Pullup selected"]
    #[inline(always)]
    pub fn pu24k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg7::Pu24k)
    }
    #[doc = "50K Pullup selected"]
    #[inline(always)]
    pub fn pu50k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg7::Pu50k)
    }
    #[doc = "100K Pullup selected"]
    #[inline(always)]
    pub fn pu100k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg7::Pu100k)
    }
}
#[doc = "IOMSTR/MSPI N Chip Select 7, DISP control signals DE, CSX, and CS. Polarity is determined by CE_POLARITY field\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ncesrc7 {
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
impl From<Ncesrc7> for u8 {
    #[inline(always)]
    fn from(variant: Ncesrc7) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ncesrc7 {
    type Ux = u8;
}
impl crate::IsEnum for Ncesrc7 {}
#[doc = "Field `NCESRC7` reader - IOMSTR/MSPI N Chip Select 7, DISP control signals DE, CSX, and CS. Polarity is determined by CE_POLARITY field"]
pub type Ncesrc7R = crate::FieldReader<Ncesrc7>;
impl Ncesrc7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ncesrc7> {
        match self.bits {
            0 => Some(Ncesrc7::Iom0ce0),
            1 => Some(Ncesrc7::Iom0ce1),
            2 => Some(Ncesrc7::Iom0ce2),
            3 => Some(Ncesrc7::Iom0ce3),
            4 => Some(Ncesrc7::Iom1ce0),
            5 => Some(Ncesrc7::Iom1ce1),
            6 => Some(Ncesrc7::Iom1ce2),
            7 => Some(Ncesrc7::Iom1ce3),
            8 => Some(Ncesrc7::Iom2ce0),
            9 => Some(Ncesrc7::Iom2ce1),
            10 => Some(Ncesrc7::Iom2ce2),
            11 => Some(Ncesrc7::Iom2ce3),
            12 => Some(Ncesrc7::Iom3ce0),
            13 => Some(Ncesrc7::Iom3ce1),
            14 => Some(Ncesrc7::Iom3ce2),
            15 => Some(Ncesrc7::Iom3ce3),
            16 => Some(Ncesrc7::Iom4ce0),
            17 => Some(Ncesrc7::Iom4ce1),
            18 => Some(Ncesrc7::Iom4ce2),
            19 => Some(Ncesrc7::Iom4ce3),
            20 => Some(Ncesrc7::Iom5ce0),
            21 => Some(Ncesrc7::Iom5ce1),
            22 => Some(Ncesrc7::Iom5ce2),
            23 => Some(Ncesrc7::Iom5ce3),
            24 => Some(Ncesrc7::Iom6ce0),
            25 => Some(Ncesrc7::Iom6ce1),
            26 => Some(Ncesrc7::Iom6ce2),
            27 => Some(Ncesrc7::Iom6ce3),
            28 => Some(Ncesrc7::Iom7ce0),
            29 => Some(Ncesrc7::Iom7ce1),
            30 => Some(Ncesrc7::Iom7ce2),
            31 => Some(Ncesrc7::Iom7ce3),
            32 => Some(Ncesrc7::Mspi0cen0),
            33 => Some(Ncesrc7::Mspi0cen1),
            34 => Some(Ncesrc7::Mspi1cen0),
            35 => Some(Ncesrc7::Mspi1cen1),
            36 => Some(Ncesrc7::Mspi2cen0),
            37 => Some(Ncesrc7::Mspi2cen1),
            38 => Some(Ncesrc7::DcDpiDe),
            39 => Some(Ncesrc7::DispContCsx),
            40 => Some(Ncesrc7::DcSpiCsN),
            41 => Some(Ncesrc7::DcQspiCsN),
            42 => Some(Ncesrc7::DcResx),
            _ => None,
        }
    }
    #[doc = "IOM 0 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom0ce0(&self) -> bool {
        *self == Ncesrc7::Iom0ce0
    }
    #[doc = "IOM 0 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom0ce1(&self) -> bool {
        *self == Ncesrc7::Iom0ce1
    }
    #[doc = "IOM 0 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom0ce2(&self) -> bool {
        *self == Ncesrc7::Iom0ce2
    }
    #[doc = "IOM 0 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom0ce3(&self) -> bool {
        *self == Ncesrc7::Iom0ce3
    }
    #[doc = "IOM 1 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom1ce0(&self) -> bool {
        *self == Ncesrc7::Iom1ce0
    }
    #[doc = "IOM 1 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom1ce1(&self) -> bool {
        *self == Ncesrc7::Iom1ce1
    }
    #[doc = "IOM 1 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom1ce2(&self) -> bool {
        *self == Ncesrc7::Iom1ce2
    }
    #[doc = "IOM 1 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom1ce3(&self) -> bool {
        *self == Ncesrc7::Iom1ce3
    }
    #[doc = "IOM 2 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom2ce0(&self) -> bool {
        *self == Ncesrc7::Iom2ce0
    }
    #[doc = "IOM 2 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom2ce1(&self) -> bool {
        *self == Ncesrc7::Iom2ce1
    }
    #[doc = "IOM 2 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom2ce2(&self) -> bool {
        *self == Ncesrc7::Iom2ce2
    }
    #[doc = "IOM 2 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom2ce3(&self) -> bool {
        *self == Ncesrc7::Iom2ce3
    }
    #[doc = "IOM 3 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom3ce0(&self) -> bool {
        *self == Ncesrc7::Iom3ce0
    }
    #[doc = "IOM 3 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom3ce1(&self) -> bool {
        *self == Ncesrc7::Iom3ce1
    }
    #[doc = "IOM 3 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom3ce2(&self) -> bool {
        *self == Ncesrc7::Iom3ce2
    }
    #[doc = "IOM 3 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom3ce3(&self) -> bool {
        *self == Ncesrc7::Iom3ce3
    }
    #[doc = "IOM 4 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom4ce0(&self) -> bool {
        *self == Ncesrc7::Iom4ce0
    }
    #[doc = "IOM 4 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom4ce1(&self) -> bool {
        *self == Ncesrc7::Iom4ce1
    }
    #[doc = "IOM 4 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom4ce2(&self) -> bool {
        *self == Ncesrc7::Iom4ce2
    }
    #[doc = "IOM 4 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom4ce3(&self) -> bool {
        *self == Ncesrc7::Iom4ce3
    }
    #[doc = "IOM 5 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom5ce0(&self) -> bool {
        *self == Ncesrc7::Iom5ce0
    }
    #[doc = "IOM 5 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom5ce1(&self) -> bool {
        *self == Ncesrc7::Iom5ce1
    }
    #[doc = "IOM 5 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom5ce2(&self) -> bool {
        *self == Ncesrc7::Iom5ce2
    }
    #[doc = "IOM 5 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom5ce3(&self) -> bool {
        *self == Ncesrc7::Iom5ce3
    }
    #[doc = "IOM 6 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom6ce0(&self) -> bool {
        *self == Ncesrc7::Iom6ce0
    }
    #[doc = "IOM 6 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom6ce1(&self) -> bool {
        *self == Ncesrc7::Iom6ce1
    }
    #[doc = "IOM 6 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom6ce2(&self) -> bool {
        *self == Ncesrc7::Iom6ce2
    }
    #[doc = "IOM 6 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom6ce3(&self) -> bool {
        *self == Ncesrc7::Iom6ce3
    }
    #[doc = "IOM 7 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom7ce0(&self) -> bool {
        *self == Ncesrc7::Iom7ce0
    }
    #[doc = "IOM 7 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom7ce1(&self) -> bool {
        *self == Ncesrc7::Iom7ce1
    }
    #[doc = "IOM 7 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom7ce2(&self) -> bool {
        *self == Ncesrc7::Iom7ce2
    }
    #[doc = "IOM 7 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom7ce3(&self) -> bool {
        *self == Ncesrc7::Iom7ce3
    }
    #[doc = "MSPI 0 NCE 0 module"]
    #[inline(always)]
    pub fn is_mspi0cen0(&self) -> bool {
        *self == Ncesrc7::Mspi0cen0
    }
    #[doc = "MSPI 0 NCE 1 module"]
    #[inline(always)]
    pub fn is_mspi0cen1(&self) -> bool {
        *self == Ncesrc7::Mspi0cen1
    }
    #[doc = "MSPI 1 NCE 0 module"]
    #[inline(always)]
    pub fn is_mspi1cen0(&self) -> bool {
        *self == Ncesrc7::Mspi1cen0
    }
    #[doc = "MSPI 1 NCE 1 module"]
    #[inline(always)]
    pub fn is_mspi1cen1(&self) -> bool {
        *self == Ncesrc7::Mspi1cen1
    }
    #[doc = "MSPI 2 NCE 0 module"]
    #[inline(always)]
    pub fn is_mspi2cen0(&self) -> bool {
        *self == Ncesrc7::Mspi2cen0
    }
    #[doc = "MSPI 2 NCE 1 module"]
    #[inline(always)]
    pub fn is_mspi2cen1(&self) -> bool {
        *self == Ncesrc7::Mspi2cen1
    }
    #[doc = "DC DPI DE module"]
    #[inline(always)]
    pub fn is_dc_dpi_de(&self) -> bool {
        *self == Ncesrc7::DcDpiDe
    }
    #[doc = "DISP CONT CSX module"]
    #[inline(always)]
    pub fn is_disp_cont_csx(&self) -> bool {
        *self == Ncesrc7::DispContCsx
    }
    #[doc = "DC SPI CS_N module"]
    #[inline(always)]
    pub fn is_dc_spi_cs_n(&self) -> bool {
        *self == Ncesrc7::DcSpiCsN
    }
    #[doc = "DC QSPI CS_N module"]
    #[inline(always)]
    pub fn is_dc_qspi_cs_n(&self) -> bool {
        *self == Ncesrc7::DcQspiCsN
    }
    #[doc = "DC module RESX"]
    #[inline(always)]
    pub fn is_dc_resx(&self) -> bool {
        *self == Ncesrc7::DcResx
    }
}
#[doc = "Field `NCESRC7` writer - IOMSTR/MSPI N Chip Select 7, DISP control signals DE, CSX, and CS. Polarity is determined by CE_POLARITY field"]
pub type Ncesrc7W<'a, REG> = crate::FieldWriter<'a, REG, 6, Ncesrc7>;
impl<'a, REG> Ncesrc7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "IOM 0 NCE 0 module"]
    #[inline(always)]
    pub fn iom0ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc7::Iom0ce0)
    }
    #[doc = "IOM 0 NCE 1 module"]
    #[inline(always)]
    pub fn iom0ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc7::Iom0ce1)
    }
    #[doc = "IOM 0 NCE 2 module"]
    #[inline(always)]
    pub fn iom0ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc7::Iom0ce2)
    }
    #[doc = "IOM 0 NCE 3 module"]
    #[inline(always)]
    pub fn iom0ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc7::Iom0ce3)
    }
    #[doc = "IOM 1 NCE 0 module"]
    #[inline(always)]
    pub fn iom1ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc7::Iom1ce0)
    }
    #[doc = "IOM 1 NCE 1 module"]
    #[inline(always)]
    pub fn iom1ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc7::Iom1ce1)
    }
    #[doc = "IOM 1 NCE 2 module"]
    #[inline(always)]
    pub fn iom1ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc7::Iom1ce2)
    }
    #[doc = "IOM 1 NCE 3 module"]
    #[inline(always)]
    pub fn iom1ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc7::Iom1ce3)
    }
    #[doc = "IOM 2 NCE 0 module"]
    #[inline(always)]
    pub fn iom2ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc7::Iom2ce0)
    }
    #[doc = "IOM 2 NCE 1 module"]
    #[inline(always)]
    pub fn iom2ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc7::Iom2ce1)
    }
    #[doc = "IOM 2 NCE 2 module"]
    #[inline(always)]
    pub fn iom2ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc7::Iom2ce2)
    }
    #[doc = "IOM 2 NCE 3 module"]
    #[inline(always)]
    pub fn iom2ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc7::Iom2ce3)
    }
    #[doc = "IOM 3 NCE 0 module"]
    #[inline(always)]
    pub fn iom3ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc7::Iom3ce0)
    }
    #[doc = "IOM 3 NCE 1 module"]
    #[inline(always)]
    pub fn iom3ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc7::Iom3ce1)
    }
    #[doc = "IOM 3 NCE 2 module"]
    #[inline(always)]
    pub fn iom3ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc7::Iom3ce2)
    }
    #[doc = "IOM 3 NCE 3 module"]
    #[inline(always)]
    pub fn iom3ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc7::Iom3ce3)
    }
    #[doc = "IOM 4 NCE 0 module"]
    #[inline(always)]
    pub fn iom4ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc7::Iom4ce0)
    }
    #[doc = "IOM 4 NCE 1 module"]
    #[inline(always)]
    pub fn iom4ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc7::Iom4ce1)
    }
    #[doc = "IOM 4 NCE 2 module"]
    #[inline(always)]
    pub fn iom4ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc7::Iom4ce2)
    }
    #[doc = "IOM 4 NCE 3 module"]
    #[inline(always)]
    pub fn iom4ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc7::Iom4ce3)
    }
    #[doc = "IOM 5 NCE 0 module"]
    #[inline(always)]
    pub fn iom5ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc7::Iom5ce0)
    }
    #[doc = "IOM 5 NCE 1 module"]
    #[inline(always)]
    pub fn iom5ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc7::Iom5ce1)
    }
    #[doc = "IOM 5 NCE 2 module"]
    #[inline(always)]
    pub fn iom5ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc7::Iom5ce2)
    }
    #[doc = "IOM 5 NCE 3 module"]
    #[inline(always)]
    pub fn iom5ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc7::Iom5ce3)
    }
    #[doc = "IOM 6 NCE 0 module"]
    #[inline(always)]
    pub fn iom6ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc7::Iom6ce0)
    }
    #[doc = "IOM 6 NCE 1 module"]
    #[inline(always)]
    pub fn iom6ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc7::Iom6ce1)
    }
    #[doc = "IOM 6 NCE 2 module"]
    #[inline(always)]
    pub fn iom6ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc7::Iom6ce2)
    }
    #[doc = "IOM 6 NCE 3 module"]
    #[inline(always)]
    pub fn iom6ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc7::Iom6ce3)
    }
    #[doc = "IOM 7 NCE 0 module"]
    #[inline(always)]
    pub fn iom7ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc7::Iom7ce0)
    }
    #[doc = "IOM 7 NCE 1 module"]
    #[inline(always)]
    pub fn iom7ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc7::Iom7ce1)
    }
    #[doc = "IOM 7 NCE 2 module"]
    #[inline(always)]
    pub fn iom7ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc7::Iom7ce2)
    }
    #[doc = "IOM 7 NCE 3 module"]
    #[inline(always)]
    pub fn iom7ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc7::Iom7ce3)
    }
    #[doc = "MSPI 0 NCE 0 module"]
    #[inline(always)]
    pub fn mspi0cen0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc7::Mspi0cen0)
    }
    #[doc = "MSPI 0 NCE 1 module"]
    #[inline(always)]
    pub fn mspi0cen1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc7::Mspi0cen1)
    }
    #[doc = "MSPI 1 NCE 0 module"]
    #[inline(always)]
    pub fn mspi1cen0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc7::Mspi1cen0)
    }
    #[doc = "MSPI 1 NCE 1 module"]
    #[inline(always)]
    pub fn mspi1cen1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc7::Mspi1cen1)
    }
    #[doc = "MSPI 2 NCE 0 module"]
    #[inline(always)]
    pub fn mspi2cen0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc7::Mspi2cen0)
    }
    #[doc = "MSPI 2 NCE 1 module"]
    #[inline(always)]
    pub fn mspi2cen1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc7::Mspi2cen1)
    }
    #[doc = "DC DPI DE module"]
    #[inline(always)]
    pub fn dc_dpi_de(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc7::DcDpiDe)
    }
    #[doc = "DISP CONT CSX module"]
    #[inline(always)]
    pub fn disp_cont_csx(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc7::DispContCsx)
    }
    #[doc = "DC SPI CS_N module"]
    #[inline(always)]
    pub fn dc_spi_cs_n(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc7::DcSpiCsN)
    }
    #[doc = "DC QSPI CS_N module"]
    #[inline(always)]
    pub fn dc_qspi_cs_n(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc7::DcQspiCsN)
    }
    #[doc = "DC module RESX"]
    #[inline(always)]
    pub fn dc_resx(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc7::DcResx)
    }
}
#[doc = "Polarity select for NCE for GPIO 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ncepol7 {
    #[doc = "0: Polarity is active low"]
    Low = 0,
    #[doc = "1: Polarity is active high"]
    High = 1,
}
impl From<Ncepol7> for bool {
    #[inline(always)]
    fn from(variant: Ncepol7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NCEPOL7` reader - Polarity select for NCE for GPIO 7"]
pub type Ncepol7R = crate::BitReader<Ncepol7>;
impl Ncepol7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ncepol7 {
        match self.bits {
            false => Ncepol7::Low,
            true => Ncepol7::High,
        }
    }
    #[doc = "Polarity is active low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Ncepol7::Low
    }
    #[doc = "Polarity is active high"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Ncepol7::High
    }
}
#[doc = "Field `NCEPOL7` writer - Polarity select for NCE for GPIO 7"]
pub type Ncepol7W<'a, REG> = crate::BitWriter<'a, REG, Ncepol7>;
impl<'a, REG> Ncepol7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Polarity is active low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Ncepol7::Low)
    }
    #[doc = "Polarity is active high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Ncepol7::High)
    }
}
#[doc = "Field `FIEN7` reader - Force input enable active regardless of function selected. Otherwise the selected function will enable the input only when needed"]
pub type Fien7R = crate::BitReader;
#[doc = "Field `FIEN7` writer - Force input enable active regardless of function selected. Otherwise the selected function will enable the input only when needed"]
pub type Fien7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FOEN7` reader - Force output enable active regardless of function selected. Otherwise the selected function will enable the output only when needed"]
pub type Foen7R = crate::BitReader;
#[doc = "Field `FOEN7` writer - Force output enable active regardless of function selected. Otherwise the selected function will enable the output only when needed"]
pub type Foen7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Function select for GPIO pin 7"]
    #[inline(always)]
    pub fn fncsel7(&self) -> Fncsel7R {
        Fncsel7R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Input enable for GPIO 7"]
    #[inline(always)]
    pub fn inpen7(&self) -> Inpen7R {
        Inpen7R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Return 0 for read data on GPIO 7"]
    #[inline(always)]
    pub fn rdzero7(&self) -> Rdzero7R {
        Rdzero7R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Interrupt enable for GPIO 7"]
    #[inline(always)]
    pub fn irpten7(&self) -> Irpten7R {
        Irpten7R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Pin IO mode selection for GPIO pin 7"]
    #[inline(always)]
    pub fn outcfg7(&self) -> Outcfg7R {
        Outcfg7R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Drive strength selection for GPIO 7"]
    #[inline(always)]
    pub fn ds7(&self) -> Ds7R {
        Ds7R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - Configure the slew rate"]
    #[inline(always)]
    pub fn sr7(&self) -> Sr7R {
        Sr7R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - Pullup/Pulldown configuration for GPIO 7"]
    #[inline(always)]
    pub fn pullcfg7(&self) -> Pullcfg7R {
        Pullcfg7R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:21 - IOMSTR/MSPI N Chip Select 7, DISP control signals DE, CSX, and CS. Polarity is determined by CE_POLARITY field"]
    #[inline(always)]
    pub fn ncesrc7(&self) -> Ncesrc7R {
        Ncesrc7R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 22 - Polarity select for NCE for GPIO 7"]
    #[inline(always)]
    pub fn ncepol7(&self) -> Ncepol7R {
        Ncepol7R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 26 - Force input enable active regardless of function selected. Otherwise the selected function will enable the input only when needed"]
    #[inline(always)]
    pub fn fien7(&self) -> Fien7R {
        Fien7R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Force output enable active regardless of function selected. Otherwise the selected function will enable the output only when needed"]
    #[inline(always)]
    pub fn foen7(&self) -> Foen7R {
        Foen7R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Function select for GPIO pin 7"]
    #[inline(always)]
    #[must_use]
    pub fn fncsel7(&mut self) -> Fncsel7W<Pincfg7Spec> {
        Fncsel7W::new(self, 0)
    }
    #[doc = "Bit 4 - Input enable for GPIO 7"]
    #[inline(always)]
    #[must_use]
    pub fn inpen7(&mut self) -> Inpen7W<Pincfg7Spec> {
        Inpen7W::new(self, 4)
    }
    #[doc = "Bit 5 - Return 0 for read data on GPIO 7"]
    #[inline(always)]
    #[must_use]
    pub fn rdzero7(&mut self) -> Rdzero7W<Pincfg7Spec> {
        Rdzero7W::new(self, 5)
    }
    #[doc = "Bits 6:7 - Interrupt enable for GPIO 7"]
    #[inline(always)]
    #[must_use]
    pub fn irpten7(&mut self) -> Irpten7W<Pincfg7Spec> {
        Irpten7W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Pin IO mode selection for GPIO pin 7"]
    #[inline(always)]
    #[must_use]
    pub fn outcfg7(&mut self) -> Outcfg7W<Pincfg7Spec> {
        Outcfg7W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Drive strength selection for GPIO 7"]
    #[inline(always)]
    #[must_use]
    pub fn ds7(&mut self) -> Ds7W<Pincfg7Spec> {
        Ds7W::new(self, 10)
    }
    #[doc = "Bit 12 - Configure the slew rate"]
    #[inline(always)]
    #[must_use]
    pub fn sr7(&mut self) -> Sr7W<Pincfg7Spec> {
        Sr7W::new(self, 12)
    }
    #[doc = "Bits 13:15 - Pullup/Pulldown configuration for GPIO 7"]
    #[inline(always)]
    #[must_use]
    pub fn pullcfg7(&mut self) -> Pullcfg7W<Pincfg7Spec> {
        Pullcfg7W::new(self, 13)
    }
    #[doc = "Bits 16:21 - IOMSTR/MSPI N Chip Select 7, DISP control signals DE, CSX, and CS. Polarity is determined by CE_POLARITY field"]
    #[inline(always)]
    #[must_use]
    pub fn ncesrc7(&mut self) -> Ncesrc7W<Pincfg7Spec> {
        Ncesrc7W::new(self, 16)
    }
    #[doc = "Bit 22 - Polarity select for NCE for GPIO 7"]
    #[inline(always)]
    #[must_use]
    pub fn ncepol7(&mut self) -> Ncepol7W<Pincfg7Spec> {
        Ncepol7W::new(self, 22)
    }
    #[doc = "Bit 26 - Force input enable active regardless of function selected. Otherwise the selected function will enable the input only when needed"]
    #[inline(always)]
    #[must_use]
    pub fn fien7(&mut self) -> Fien7W<Pincfg7Spec> {
        Fien7W::new(self, 26)
    }
    #[doc = "Bit 27 - Force output enable active regardless of function selected. Otherwise the selected function will enable the output only when needed"]
    #[inline(always)]
    #[must_use]
    pub fn foen7(&mut self) -> Foen7W<Pincfg7Spec> {
        Foen7W::new(self, 27)
    }
}
#[doc = "Controls the operation of GPIO pin 7.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pincfg7Spec;
impl crate::RegisterSpec for Pincfg7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pincfg7::R`](R) reader structure"]
impl crate::Readable for Pincfg7Spec {}
#[doc = "`write(|w| ..)` method takes [`pincfg7::W`](W) writer structure"]
impl crate::Writable for Pincfg7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PINCFG7 to value 0x03"]
impl crate::Resettable for Pincfg7Spec {
    const RESET_VALUE: u32 = 0x03;
}
