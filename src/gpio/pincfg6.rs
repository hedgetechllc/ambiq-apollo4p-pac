#[doc = "Register `PINCFG6` reader"]
pub type R = crate::R<Pincfg6Spec>;
#[doc = "Register `PINCFG6` writer"]
pub type W = crate::W<Pincfg6Spec>;
#[doc = "Function select for GPIO pin 6\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fncsel6 {
    #[doc = "0: Serial I2C Master Data I/O (I2C Mode) Serial SPI Master Data I/O (SPI 3 wire mode) (IOM 0)"]
    M0sdawir3 = 0,
    #[doc = "1: Serial SPI Master MOSI output (IOM 0)"]
    M0mosi = 1,
    #[doc = "2: Bidirectional I2S Data. Operates in output mode in master mode and input mode for slave mode. (I2S Master/Slave 2)"]
    I2s0Data = 2,
    #[doc = "3: General purpose I/O"]
    Gpio = 3,
    #[doc = "4: UART Clear to Send (CTS) (UART 0)"]
    Uart0cts = 4,
    #[doc = "5: UART Clear to Send (CTS) (UART 1)"]
    Uart1cts = 5,
    #[doc = "6: Timer/Counter input or output; Selection of direction is done via CTIMER register settings."]
    Ct6 = 6,
    #[doc = "7: IOMSTR/MSPI N Chip Select. Polarity is determined by CE_POLARITY field"]
    Nce6 = 7,
    #[doc = "8: Observation bus bit 6"]
    Obsbus6 = 8,
    #[doc = "9: I2S Data output (I2S Master/Slave 2)"]
    I2s0Sdout = 9,
    #[doc = "10: I2S Data output (I2S Master/Slave 2)"]
    I2s1Sdout = 10,
    #[doc = "11: Fast PIO"]
    Fpio = 11,
    #[doc = "12: Reserved selection. Operation unknown if selected."]
    Reserved12 = 12,
    #[doc = "13: Reserved selection. Operation unknown if selected."]
    Reserved13 = 13,
    #[doc = "14: Reserved selection. Operation unknown if selected."]
    Reserved14 = 14,
    #[doc = "15: Internal function (SCAN)"]
    Scanin6 = 15,
}
impl From<Fncsel6> for u8 {
    #[inline(always)]
    fn from(variant: Fncsel6) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fncsel6 {
    type Ux = u8;
}
impl crate::IsEnum for Fncsel6 {}
#[doc = "Field `FNCSEL6` reader - Function select for GPIO pin 6"]
pub type Fncsel6R = crate::FieldReader<Fncsel6>;
impl Fncsel6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fncsel6 {
        match self.bits {
            0 => Fncsel6::M0sdawir3,
            1 => Fncsel6::M0mosi,
            2 => Fncsel6::I2s0Data,
            3 => Fncsel6::Gpio,
            4 => Fncsel6::Uart0cts,
            5 => Fncsel6::Uart1cts,
            6 => Fncsel6::Ct6,
            7 => Fncsel6::Nce6,
            8 => Fncsel6::Obsbus6,
            9 => Fncsel6::I2s0Sdout,
            10 => Fncsel6::I2s1Sdout,
            11 => Fncsel6::Fpio,
            12 => Fncsel6::Reserved12,
            13 => Fncsel6::Reserved13,
            14 => Fncsel6::Reserved14,
            15 => Fncsel6::Scanin6,
            _ => unreachable!(),
        }
    }
    #[doc = "Serial I2C Master Data I/O (I2C Mode) Serial SPI Master Data I/O (SPI 3 wire mode) (IOM 0)"]
    #[inline(always)]
    pub fn is_m0sdawir3(&self) -> bool {
        *self == Fncsel6::M0sdawir3
    }
    #[doc = "Serial SPI Master MOSI output (IOM 0)"]
    #[inline(always)]
    pub fn is_m0mosi(&self) -> bool {
        *self == Fncsel6::M0mosi
    }
    #[doc = "Bidirectional I2S Data. Operates in output mode in master mode and input mode for slave mode. (I2S Master/Slave 2)"]
    #[inline(always)]
    pub fn is_i2s0_data(&self) -> bool {
        *self == Fncsel6::I2s0Data
    }
    #[doc = "General purpose I/O"]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == Fncsel6::Gpio
    }
    #[doc = "UART Clear to Send (CTS) (UART 0)"]
    #[inline(always)]
    pub fn is_uart0cts(&self) -> bool {
        *self == Fncsel6::Uart0cts
    }
    #[doc = "UART Clear to Send (CTS) (UART 1)"]
    #[inline(always)]
    pub fn is_uart1cts(&self) -> bool {
        *self == Fncsel6::Uart1cts
    }
    #[doc = "Timer/Counter input or output; Selection of direction is done via CTIMER register settings."]
    #[inline(always)]
    pub fn is_ct6(&self) -> bool {
        *self == Fncsel6::Ct6
    }
    #[doc = "IOMSTR/MSPI N Chip Select. Polarity is determined by CE_POLARITY field"]
    #[inline(always)]
    pub fn is_nce6(&self) -> bool {
        *self == Fncsel6::Nce6
    }
    #[doc = "Observation bus bit 6"]
    #[inline(always)]
    pub fn is_obsbus6(&self) -> bool {
        *self == Fncsel6::Obsbus6
    }
    #[doc = "I2S Data output (I2S Master/Slave 2)"]
    #[inline(always)]
    pub fn is_i2s0_sdout(&self) -> bool {
        *self == Fncsel6::I2s0Sdout
    }
    #[doc = "I2S Data output (I2S Master/Slave 2)"]
    #[inline(always)]
    pub fn is_i2s1_sdout(&self) -> bool {
        *self == Fncsel6::I2s1Sdout
    }
    #[doc = "Fast PIO"]
    #[inline(always)]
    pub fn is_fpio(&self) -> bool {
        *self == Fncsel6::Fpio
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved12(&self) -> bool {
        *self == Fncsel6::Reserved12
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved13(&self) -> bool {
        *self == Fncsel6::Reserved13
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved14(&self) -> bool {
        *self == Fncsel6::Reserved14
    }
    #[doc = "Internal function (SCAN)"]
    #[inline(always)]
    pub fn is_scanin6(&self) -> bool {
        *self == Fncsel6::Scanin6
    }
}
#[doc = "Field `FNCSEL6` writer - Function select for GPIO pin 6"]
pub type Fncsel6W<'a, REG> = crate::FieldWriter<'a, REG, 4, Fncsel6, crate::Safe>;
impl<'a, REG> Fncsel6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Serial I2C Master Data I/O (I2C Mode) Serial SPI Master Data I/O (SPI 3 wire mode) (IOM 0)"]
    #[inline(always)]
    pub fn m0sdawir3(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel6::M0sdawir3)
    }
    #[doc = "Serial SPI Master MOSI output (IOM 0)"]
    #[inline(always)]
    pub fn m0mosi(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel6::M0mosi)
    }
    #[doc = "Bidirectional I2S Data. Operates in output mode in master mode and input mode for slave mode. (I2S Master/Slave 2)"]
    #[inline(always)]
    pub fn i2s0_data(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel6::I2s0Data)
    }
    #[doc = "General purpose I/O"]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel6::Gpio)
    }
    #[doc = "UART Clear to Send (CTS) (UART 0)"]
    #[inline(always)]
    pub fn uart0cts(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel6::Uart0cts)
    }
    #[doc = "UART Clear to Send (CTS) (UART 1)"]
    #[inline(always)]
    pub fn uart1cts(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel6::Uart1cts)
    }
    #[doc = "Timer/Counter input or output; Selection of direction is done via CTIMER register settings."]
    #[inline(always)]
    pub fn ct6(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel6::Ct6)
    }
    #[doc = "IOMSTR/MSPI N Chip Select. Polarity is determined by CE_POLARITY field"]
    #[inline(always)]
    pub fn nce6(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel6::Nce6)
    }
    #[doc = "Observation bus bit 6"]
    #[inline(always)]
    pub fn obsbus6(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel6::Obsbus6)
    }
    #[doc = "I2S Data output (I2S Master/Slave 2)"]
    #[inline(always)]
    pub fn i2s0_sdout(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel6::I2s0Sdout)
    }
    #[doc = "I2S Data output (I2S Master/Slave 2)"]
    #[inline(always)]
    pub fn i2s1_sdout(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel6::I2s1Sdout)
    }
    #[doc = "Fast PIO"]
    #[inline(always)]
    pub fn fpio(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel6::Fpio)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved12(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel6::Reserved12)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved13(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel6::Reserved13)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved14(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel6::Reserved14)
    }
    #[doc = "Internal function (SCAN)"]
    #[inline(always)]
    pub fn scanin6(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel6::Scanin6)
    }
}
#[doc = "Field `INPEN6` reader - Input enable for GPIO 6"]
pub type Inpen6R = crate::BitReader;
#[doc = "Field `INPEN6` writer - Input enable for GPIO 6"]
pub type Inpen6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDZERO6` reader - Return 0 for read data on GPIO 6"]
pub type Rdzero6R = crate::BitReader;
#[doc = "Field `RDZERO6` writer - Return 0 for read data on GPIO 6"]
pub type Rdzero6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Interrupt enable for GPIO 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Irpten6 {
    #[doc = "0: Interrupts are disabled for this GPIO"]
    Dis = 0,
    #[doc = "1: Interrupts are enabled for falling edge transition on this GPIO"]
    Intfall = 1,
    #[doc = "2: Interrupts are enabled for rising edge transitions on this GPIO"]
    Intrise = 2,
    #[doc = "3: Interrupts are enabled for any edge transition on this GPIO"]
    Intany = 3,
}
impl From<Irpten6> for u8 {
    #[inline(always)]
    fn from(variant: Irpten6) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Irpten6 {
    type Ux = u8;
}
impl crate::IsEnum for Irpten6 {}
#[doc = "Field `IRPTEN6` reader - Interrupt enable for GPIO 6"]
pub type Irpten6R = crate::FieldReader<Irpten6>;
impl Irpten6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irpten6 {
        match self.bits {
            0 => Irpten6::Dis,
            1 => Irpten6::Intfall,
            2 => Irpten6::Intrise,
            3 => Irpten6::Intany,
            _ => unreachable!(),
        }
    }
    #[doc = "Interrupts are disabled for this GPIO"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Irpten6::Dis
    }
    #[doc = "Interrupts are enabled for falling edge transition on this GPIO"]
    #[inline(always)]
    pub fn is_intfall(&self) -> bool {
        *self == Irpten6::Intfall
    }
    #[doc = "Interrupts are enabled for rising edge transitions on this GPIO"]
    #[inline(always)]
    pub fn is_intrise(&self) -> bool {
        *self == Irpten6::Intrise
    }
    #[doc = "Interrupts are enabled for any edge transition on this GPIO"]
    #[inline(always)]
    pub fn is_intany(&self) -> bool {
        *self == Irpten6::Intany
    }
}
#[doc = "Field `IRPTEN6` writer - Interrupt enable for GPIO 6"]
pub type Irpten6W<'a, REG> = crate::FieldWriter<'a, REG, 2, Irpten6, crate::Safe>;
impl<'a, REG> Irpten6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Interrupts are disabled for this GPIO"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten6::Dis)
    }
    #[doc = "Interrupts are enabled for falling edge transition on this GPIO"]
    #[inline(always)]
    pub fn intfall(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten6::Intfall)
    }
    #[doc = "Interrupts are enabled for rising edge transitions on this GPIO"]
    #[inline(always)]
    pub fn intrise(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten6::Intrise)
    }
    #[doc = "Interrupts are enabled for any edge transition on this GPIO"]
    #[inline(always)]
    pub fn intany(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten6::Intany)
    }
}
#[doc = "Pin IO mode selection for GPIO pin 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Outcfg6 {
    #[doc = "0: Output Disabled"]
    Dis = 0,
    #[doc = "1: Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    Pushpull = 1,
    #[doc = "2: Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    Od = 2,
    #[doc = "3: Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    Ts = 3,
}
impl From<Outcfg6> for u8 {
    #[inline(always)]
    fn from(variant: Outcfg6) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Outcfg6 {
    type Ux = u8;
}
impl crate::IsEnum for Outcfg6 {}
#[doc = "Field `OUTCFG6` reader - Pin IO mode selection for GPIO pin 6"]
pub type Outcfg6R = crate::FieldReader<Outcfg6>;
impl Outcfg6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Outcfg6 {
        match self.bits {
            0 => Outcfg6::Dis,
            1 => Outcfg6::Pushpull,
            2 => Outcfg6::Od,
            3 => Outcfg6::Ts,
            _ => unreachable!(),
        }
    }
    #[doc = "Output Disabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Outcfg6::Dis
    }
    #[doc = "Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == Outcfg6::Pushpull
    }
    #[doc = "Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == Outcfg6::Od
    }
    #[doc = "Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == Outcfg6::Ts
    }
}
#[doc = "Field `OUTCFG6` writer - Pin IO mode selection for GPIO pin 6"]
pub type Outcfg6W<'a, REG> = crate::FieldWriter<'a, REG, 2, Outcfg6, crate::Safe>;
impl<'a, REG> Outcfg6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Output Disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg6::Dis)
    }
    #[doc = "Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg6::Pushpull)
    }
    #[doc = "Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    #[inline(always)]
    pub fn od(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg6::Od)
    }
    #[doc = "Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    #[inline(always)]
    pub fn ts(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg6::Ts)
    }
}
#[doc = "Drive strength selection for GPIO 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ds6 {
    #[doc = "0: 0.1x output driver selected"]
    _0p1x = 0,
    #[doc = "1: 0.5x output driver selected"]
    _0p5x = 1,
    #[doc = "2: 0.75x output driver selected"]
    _0p75x = 2,
    #[doc = "3: 1.0x output driver selected"]
    _1p0x = 3,
}
impl From<Ds6> for u8 {
    #[inline(always)]
    fn from(variant: Ds6) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ds6 {
    type Ux = u8;
}
impl crate::IsEnum for Ds6 {}
#[doc = "Field `DS6` reader - Drive strength selection for GPIO 6"]
pub type Ds6R = crate::FieldReader<Ds6>;
impl Ds6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ds6 {
        match self.bits {
            0 => Ds6::_0p1x,
            1 => Ds6::_0p5x,
            2 => Ds6::_0p75x,
            3 => Ds6::_1p0x,
            _ => unreachable!(),
        }
    }
    #[doc = "0.1x output driver selected"]
    #[inline(always)]
    pub fn is_0p1x(&self) -> bool {
        *self == Ds6::_0p1x
    }
    #[doc = "0.5x output driver selected"]
    #[inline(always)]
    pub fn is_0p5x(&self) -> bool {
        *self == Ds6::_0p5x
    }
    #[doc = "0.75x output driver selected"]
    #[inline(always)]
    pub fn is_0p75x(&self) -> bool {
        *self == Ds6::_0p75x
    }
    #[doc = "1.0x output driver selected"]
    #[inline(always)]
    pub fn is_1p0x(&self) -> bool {
        *self == Ds6::_1p0x
    }
}
#[doc = "Field `DS6` writer - Drive strength selection for GPIO 6"]
pub type Ds6W<'a, REG> = crate::FieldWriter<'a, REG, 2, Ds6, crate::Safe>;
impl<'a, REG> Ds6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "0.1x output driver selected"]
    #[inline(always)]
    pub fn _0p1x(self) -> &'a mut crate::W<REG> {
        self.variant(Ds6::_0p1x)
    }
    #[doc = "0.5x output driver selected"]
    #[inline(always)]
    pub fn _0p5x(self) -> &'a mut crate::W<REG> {
        self.variant(Ds6::_0p5x)
    }
    #[doc = "0.75x output driver selected"]
    #[inline(always)]
    pub fn _0p75x(self) -> &'a mut crate::W<REG> {
        self.variant(Ds6::_0p75x)
    }
    #[doc = "1.0x output driver selected"]
    #[inline(always)]
    pub fn _1p0x(self) -> &'a mut crate::W<REG> {
        self.variant(Ds6::_1p0x)
    }
}
#[doc = "Field `SR6` reader - Configure the slew rate"]
pub type Sr6R = crate::BitReader;
#[doc = "Field `SR6` writer - Configure the slew rate"]
pub type Sr6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Pullup/Pulldown configuration for GPIO 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pullcfg6 {
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
impl From<Pullcfg6> for u8 {
    #[inline(always)]
    fn from(variant: Pullcfg6) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pullcfg6 {
    type Ux = u8;
}
impl crate::IsEnum for Pullcfg6 {}
#[doc = "Field `PULLCFG6` reader - Pullup/Pulldown configuration for GPIO 6"]
pub type Pullcfg6R = crate::FieldReader<Pullcfg6>;
impl Pullcfg6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pullcfg6 {
        match self.bits {
            0 => Pullcfg6::Dis,
            1 => Pullcfg6::Pd50k,
            2 => Pullcfg6::Pu15k,
            3 => Pullcfg6::Pu6k,
            4 => Pullcfg6::Pu12k,
            5 => Pullcfg6::Pu24k,
            6 => Pullcfg6::Pu50k,
            7 => Pullcfg6::Pu100k,
            _ => unreachable!(),
        }
    }
    #[doc = "No pullup or pulldown selected"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Pullcfg6::Dis
    }
    #[doc = "50K Pulldown selected"]
    #[inline(always)]
    pub fn is_pd50k(&self) -> bool {
        *self == Pullcfg6::Pd50k
    }
    #[doc = "1.5K Pullup selected"]
    #[inline(always)]
    pub fn is_pu15k(&self) -> bool {
        *self == Pullcfg6::Pu15k
    }
    #[doc = "6K Pullup selected"]
    #[inline(always)]
    pub fn is_pu6k(&self) -> bool {
        *self == Pullcfg6::Pu6k
    }
    #[doc = "12K Pullup selected"]
    #[inline(always)]
    pub fn is_pu12k(&self) -> bool {
        *self == Pullcfg6::Pu12k
    }
    #[doc = "24K Pullup selected"]
    #[inline(always)]
    pub fn is_pu24k(&self) -> bool {
        *self == Pullcfg6::Pu24k
    }
    #[doc = "50K Pullup selected"]
    #[inline(always)]
    pub fn is_pu50k(&self) -> bool {
        *self == Pullcfg6::Pu50k
    }
    #[doc = "100K Pullup selected"]
    #[inline(always)]
    pub fn is_pu100k(&self) -> bool {
        *self == Pullcfg6::Pu100k
    }
}
#[doc = "Field `PULLCFG6` writer - Pullup/Pulldown configuration for GPIO 6"]
pub type Pullcfg6W<'a, REG> = crate::FieldWriter<'a, REG, 3, Pullcfg6, crate::Safe>;
impl<'a, REG> Pullcfg6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No pullup or pulldown selected"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg6::Dis)
    }
    #[doc = "50K Pulldown selected"]
    #[inline(always)]
    pub fn pd50k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg6::Pd50k)
    }
    #[doc = "1.5K Pullup selected"]
    #[inline(always)]
    pub fn pu15k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg6::Pu15k)
    }
    #[doc = "6K Pullup selected"]
    #[inline(always)]
    pub fn pu6k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg6::Pu6k)
    }
    #[doc = "12K Pullup selected"]
    #[inline(always)]
    pub fn pu12k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg6::Pu12k)
    }
    #[doc = "24K Pullup selected"]
    #[inline(always)]
    pub fn pu24k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg6::Pu24k)
    }
    #[doc = "50K Pullup selected"]
    #[inline(always)]
    pub fn pu50k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg6::Pu50k)
    }
    #[doc = "100K Pullup selected"]
    #[inline(always)]
    pub fn pu100k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg6::Pu100k)
    }
}
#[doc = "IOMSTR/MSPI N Chip Select 6, DISP control signals DE, CSX, and CS. Polarity is determined by CE_POLARITY field\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ncesrc6 {
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
impl From<Ncesrc6> for u8 {
    #[inline(always)]
    fn from(variant: Ncesrc6) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ncesrc6 {
    type Ux = u8;
}
impl crate::IsEnum for Ncesrc6 {}
#[doc = "Field `NCESRC6` reader - IOMSTR/MSPI N Chip Select 6, DISP control signals DE, CSX, and CS. Polarity is determined by CE_POLARITY field"]
pub type Ncesrc6R = crate::FieldReader<Ncesrc6>;
impl Ncesrc6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ncesrc6> {
        match self.bits {
            0 => Some(Ncesrc6::Iom0ce0),
            1 => Some(Ncesrc6::Iom0ce1),
            2 => Some(Ncesrc6::Iom0ce2),
            3 => Some(Ncesrc6::Iom0ce3),
            4 => Some(Ncesrc6::Iom1ce0),
            5 => Some(Ncesrc6::Iom1ce1),
            6 => Some(Ncesrc6::Iom1ce2),
            7 => Some(Ncesrc6::Iom1ce3),
            8 => Some(Ncesrc6::Iom2ce0),
            9 => Some(Ncesrc6::Iom2ce1),
            10 => Some(Ncesrc6::Iom2ce2),
            11 => Some(Ncesrc6::Iom2ce3),
            12 => Some(Ncesrc6::Iom3ce0),
            13 => Some(Ncesrc6::Iom3ce1),
            14 => Some(Ncesrc6::Iom3ce2),
            15 => Some(Ncesrc6::Iom3ce3),
            16 => Some(Ncesrc6::Iom4ce0),
            17 => Some(Ncesrc6::Iom4ce1),
            18 => Some(Ncesrc6::Iom4ce2),
            19 => Some(Ncesrc6::Iom4ce3),
            20 => Some(Ncesrc6::Iom5ce0),
            21 => Some(Ncesrc6::Iom5ce1),
            22 => Some(Ncesrc6::Iom5ce2),
            23 => Some(Ncesrc6::Iom5ce3),
            24 => Some(Ncesrc6::Iom6ce0),
            25 => Some(Ncesrc6::Iom6ce1),
            26 => Some(Ncesrc6::Iom6ce2),
            27 => Some(Ncesrc6::Iom6ce3),
            28 => Some(Ncesrc6::Iom7ce0),
            29 => Some(Ncesrc6::Iom7ce1),
            30 => Some(Ncesrc6::Iom7ce2),
            31 => Some(Ncesrc6::Iom7ce3),
            32 => Some(Ncesrc6::Mspi0cen0),
            33 => Some(Ncesrc6::Mspi0cen1),
            34 => Some(Ncesrc6::Mspi1cen0),
            35 => Some(Ncesrc6::Mspi1cen1),
            36 => Some(Ncesrc6::Mspi2cen0),
            37 => Some(Ncesrc6::Mspi2cen1),
            38 => Some(Ncesrc6::DcDpiDe),
            39 => Some(Ncesrc6::DispContCsx),
            40 => Some(Ncesrc6::DcSpiCsN),
            41 => Some(Ncesrc6::DcQspiCsN),
            42 => Some(Ncesrc6::DcResx),
            _ => None,
        }
    }
    #[doc = "IOM 0 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom0ce0(&self) -> bool {
        *self == Ncesrc6::Iom0ce0
    }
    #[doc = "IOM 0 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom0ce1(&self) -> bool {
        *self == Ncesrc6::Iom0ce1
    }
    #[doc = "IOM 0 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom0ce2(&self) -> bool {
        *self == Ncesrc6::Iom0ce2
    }
    #[doc = "IOM 0 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom0ce3(&self) -> bool {
        *self == Ncesrc6::Iom0ce3
    }
    #[doc = "IOM 1 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom1ce0(&self) -> bool {
        *self == Ncesrc6::Iom1ce0
    }
    #[doc = "IOM 1 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom1ce1(&self) -> bool {
        *self == Ncesrc6::Iom1ce1
    }
    #[doc = "IOM 1 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom1ce2(&self) -> bool {
        *self == Ncesrc6::Iom1ce2
    }
    #[doc = "IOM 1 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom1ce3(&self) -> bool {
        *self == Ncesrc6::Iom1ce3
    }
    #[doc = "IOM 2 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom2ce0(&self) -> bool {
        *self == Ncesrc6::Iom2ce0
    }
    #[doc = "IOM 2 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom2ce1(&self) -> bool {
        *self == Ncesrc6::Iom2ce1
    }
    #[doc = "IOM 2 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom2ce2(&self) -> bool {
        *self == Ncesrc6::Iom2ce2
    }
    #[doc = "IOM 2 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom2ce3(&self) -> bool {
        *self == Ncesrc6::Iom2ce3
    }
    #[doc = "IOM 3 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom3ce0(&self) -> bool {
        *self == Ncesrc6::Iom3ce0
    }
    #[doc = "IOM 3 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom3ce1(&self) -> bool {
        *self == Ncesrc6::Iom3ce1
    }
    #[doc = "IOM 3 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom3ce2(&self) -> bool {
        *self == Ncesrc6::Iom3ce2
    }
    #[doc = "IOM 3 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom3ce3(&self) -> bool {
        *self == Ncesrc6::Iom3ce3
    }
    #[doc = "IOM 4 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom4ce0(&self) -> bool {
        *self == Ncesrc6::Iom4ce0
    }
    #[doc = "IOM 4 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom4ce1(&self) -> bool {
        *self == Ncesrc6::Iom4ce1
    }
    #[doc = "IOM 4 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom4ce2(&self) -> bool {
        *self == Ncesrc6::Iom4ce2
    }
    #[doc = "IOM 4 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom4ce3(&self) -> bool {
        *self == Ncesrc6::Iom4ce3
    }
    #[doc = "IOM 5 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom5ce0(&self) -> bool {
        *self == Ncesrc6::Iom5ce0
    }
    #[doc = "IOM 5 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom5ce1(&self) -> bool {
        *self == Ncesrc6::Iom5ce1
    }
    #[doc = "IOM 5 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom5ce2(&self) -> bool {
        *self == Ncesrc6::Iom5ce2
    }
    #[doc = "IOM 5 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom5ce3(&self) -> bool {
        *self == Ncesrc6::Iom5ce3
    }
    #[doc = "IOM 6 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom6ce0(&self) -> bool {
        *self == Ncesrc6::Iom6ce0
    }
    #[doc = "IOM 6 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom6ce1(&self) -> bool {
        *self == Ncesrc6::Iom6ce1
    }
    #[doc = "IOM 6 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom6ce2(&self) -> bool {
        *self == Ncesrc6::Iom6ce2
    }
    #[doc = "IOM 6 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom6ce3(&self) -> bool {
        *self == Ncesrc6::Iom6ce3
    }
    #[doc = "IOM 7 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom7ce0(&self) -> bool {
        *self == Ncesrc6::Iom7ce0
    }
    #[doc = "IOM 7 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom7ce1(&self) -> bool {
        *self == Ncesrc6::Iom7ce1
    }
    #[doc = "IOM 7 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom7ce2(&self) -> bool {
        *self == Ncesrc6::Iom7ce2
    }
    #[doc = "IOM 7 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom7ce3(&self) -> bool {
        *self == Ncesrc6::Iom7ce3
    }
    #[doc = "MSPI 0 NCE 0 module"]
    #[inline(always)]
    pub fn is_mspi0cen0(&self) -> bool {
        *self == Ncesrc6::Mspi0cen0
    }
    #[doc = "MSPI 0 NCE 1 module"]
    #[inline(always)]
    pub fn is_mspi0cen1(&self) -> bool {
        *self == Ncesrc6::Mspi0cen1
    }
    #[doc = "MSPI 1 NCE 0 module"]
    #[inline(always)]
    pub fn is_mspi1cen0(&self) -> bool {
        *self == Ncesrc6::Mspi1cen0
    }
    #[doc = "MSPI 1 NCE 1 module"]
    #[inline(always)]
    pub fn is_mspi1cen1(&self) -> bool {
        *self == Ncesrc6::Mspi1cen1
    }
    #[doc = "MSPI 2 NCE 0 module"]
    #[inline(always)]
    pub fn is_mspi2cen0(&self) -> bool {
        *self == Ncesrc6::Mspi2cen0
    }
    #[doc = "MSPI 2 NCE 1 module"]
    #[inline(always)]
    pub fn is_mspi2cen1(&self) -> bool {
        *self == Ncesrc6::Mspi2cen1
    }
    #[doc = "DC DPI DE module"]
    #[inline(always)]
    pub fn is_dc_dpi_de(&self) -> bool {
        *self == Ncesrc6::DcDpiDe
    }
    #[doc = "DISP CONT CSX module"]
    #[inline(always)]
    pub fn is_disp_cont_csx(&self) -> bool {
        *self == Ncesrc6::DispContCsx
    }
    #[doc = "DC SPI CS_N module"]
    #[inline(always)]
    pub fn is_dc_spi_cs_n(&self) -> bool {
        *self == Ncesrc6::DcSpiCsN
    }
    #[doc = "DC QSPI CS_N module"]
    #[inline(always)]
    pub fn is_dc_qspi_cs_n(&self) -> bool {
        *self == Ncesrc6::DcQspiCsN
    }
    #[doc = "DC module RESX"]
    #[inline(always)]
    pub fn is_dc_resx(&self) -> bool {
        *self == Ncesrc6::DcResx
    }
}
#[doc = "Field `NCESRC6` writer - IOMSTR/MSPI N Chip Select 6, DISP control signals DE, CSX, and CS. Polarity is determined by CE_POLARITY field"]
pub type Ncesrc6W<'a, REG> = crate::FieldWriter<'a, REG, 6, Ncesrc6>;
impl<'a, REG> Ncesrc6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "IOM 0 NCE 0 module"]
    #[inline(always)]
    pub fn iom0ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc6::Iom0ce0)
    }
    #[doc = "IOM 0 NCE 1 module"]
    #[inline(always)]
    pub fn iom0ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc6::Iom0ce1)
    }
    #[doc = "IOM 0 NCE 2 module"]
    #[inline(always)]
    pub fn iom0ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc6::Iom0ce2)
    }
    #[doc = "IOM 0 NCE 3 module"]
    #[inline(always)]
    pub fn iom0ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc6::Iom0ce3)
    }
    #[doc = "IOM 1 NCE 0 module"]
    #[inline(always)]
    pub fn iom1ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc6::Iom1ce0)
    }
    #[doc = "IOM 1 NCE 1 module"]
    #[inline(always)]
    pub fn iom1ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc6::Iom1ce1)
    }
    #[doc = "IOM 1 NCE 2 module"]
    #[inline(always)]
    pub fn iom1ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc6::Iom1ce2)
    }
    #[doc = "IOM 1 NCE 3 module"]
    #[inline(always)]
    pub fn iom1ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc6::Iom1ce3)
    }
    #[doc = "IOM 2 NCE 0 module"]
    #[inline(always)]
    pub fn iom2ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc6::Iom2ce0)
    }
    #[doc = "IOM 2 NCE 1 module"]
    #[inline(always)]
    pub fn iom2ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc6::Iom2ce1)
    }
    #[doc = "IOM 2 NCE 2 module"]
    #[inline(always)]
    pub fn iom2ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc6::Iom2ce2)
    }
    #[doc = "IOM 2 NCE 3 module"]
    #[inline(always)]
    pub fn iom2ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc6::Iom2ce3)
    }
    #[doc = "IOM 3 NCE 0 module"]
    #[inline(always)]
    pub fn iom3ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc6::Iom3ce0)
    }
    #[doc = "IOM 3 NCE 1 module"]
    #[inline(always)]
    pub fn iom3ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc6::Iom3ce1)
    }
    #[doc = "IOM 3 NCE 2 module"]
    #[inline(always)]
    pub fn iom3ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc6::Iom3ce2)
    }
    #[doc = "IOM 3 NCE 3 module"]
    #[inline(always)]
    pub fn iom3ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc6::Iom3ce3)
    }
    #[doc = "IOM 4 NCE 0 module"]
    #[inline(always)]
    pub fn iom4ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc6::Iom4ce0)
    }
    #[doc = "IOM 4 NCE 1 module"]
    #[inline(always)]
    pub fn iom4ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc6::Iom4ce1)
    }
    #[doc = "IOM 4 NCE 2 module"]
    #[inline(always)]
    pub fn iom4ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc6::Iom4ce2)
    }
    #[doc = "IOM 4 NCE 3 module"]
    #[inline(always)]
    pub fn iom4ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc6::Iom4ce3)
    }
    #[doc = "IOM 5 NCE 0 module"]
    #[inline(always)]
    pub fn iom5ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc6::Iom5ce0)
    }
    #[doc = "IOM 5 NCE 1 module"]
    #[inline(always)]
    pub fn iom5ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc6::Iom5ce1)
    }
    #[doc = "IOM 5 NCE 2 module"]
    #[inline(always)]
    pub fn iom5ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc6::Iom5ce2)
    }
    #[doc = "IOM 5 NCE 3 module"]
    #[inline(always)]
    pub fn iom5ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc6::Iom5ce3)
    }
    #[doc = "IOM 6 NCE 0 module"]
    #[inline(always)]
    pub fn iom6ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc6::Iom6ce0)
    }
    #[doc = "IOM 6 NCE 1 module"]
    #[inline(always)]
    pub fn iom6ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc6::Iom6ce1)
    }
    #[doc = "IOM 6 NCE 2 module"]
    #[inline(always)]
    pub fn iom6ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc6::Iom6ce2)
    }
    #[doc = "IOM 6 NCE 3 module"]
    #[inline(always)]
    pub fn iom6ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc6::Iom6ce3)
    }
    #[doc = "IOM 7 NCE 0 module"]
    #[inline(always)]
    pub fn iom7ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc6::Iom7ce0)
    }
    #[doc = "IOM 7 NCE 1 module"]
    #[inline(always)]
    pub fn iom7ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc6::Iom7ce1)
    }
    #[doc = "IOM 7 NCE 2 module"]
    #[inline(always)]
    pub fn iom7ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc6::Iom7ce2)
    }
    #[doc = "IOM 7 NCE 3 module"]
    #[inline(always)]
    pub fn iom7ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc6::Iom7ce3)
    }
    #[doc = "MSPI 0 NCE 0 module"]
    #[inline(always)]
    pub fn mspi0cen0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc6::Mspi0cen0)
    }
    #[doc = "MSPI 0 NCE 1 module"]
    #[inline(always)]
    pub fn mspi0cen1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc6::Mspi0cen1)
    }
    #[doc = "MSPI 1 NCE 0 module"]
    #[inline(always)]
    pub fn mspi1cen0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc6::Mspi1cen0)
    }
    #[doc = "MSPI 1 NCE 1 module"]
    #[inline(always)]
    pub fn mspi1cen1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc6::Mspi1cen1)
    }
    #[doc = "MSPI 2 NCE 0 module"]
    #[inline(always)]
    pub fn mspi2cen0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc6::Mspi2cen0)
    }
    #[doc = "MSPI 2 NCE 1 module"]
    #[inline(always)]
    pub fn mspi2cen1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc6::Mspi2cen1)
    }
    #[doc = "DC DPI DE module"]
    #[inline(always)]
    pub fn dc_dpi_de(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc6::DcDpiDe)
    }
    #[doc = "DISP CONT CSX module"]
    #[inline(always)]
    pub fn disp_cont_csx(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc6::DispContCsx)
    }
    #[doc = "DC SPI CS_N module"]
    #[inline(always)]
    pub fn dc_spi_cs_n(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc6::DcSpiCsN)
    }
    #[doc = "DC QSPI CS_N module"]
    #[inline(always)]
    pub fn dc_qspi_cs_n(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc6::DcQspiCsN)
    }
    #[doc = "DC module RESX"]
    #[inline(always)]
    pub fn dc_resx(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc6::DcResx)
    }
}
#[doc = "Polarity select for NCE for GPIO 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ncepol6 {
    #[doc = "0: Polarity is active low"]
    Low = 0,
    #[doc = "1: Polarity is active high"]
    High = 1,
}
impl From<Ncepol6> for bool {
    #[inline(always)]
    fn from(variant: Ncepol6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NCEPOL6` reader - Polarity select for NCE for GPIO 6"]
pub type Ncepol6R = crate::BitReader<Ncepol6>;
impl Ncepol6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ncepol6 {
        match self.bits {
            false => Ncepol6::Low,
            true => Ncepol6::High,
        }
    }
    #[doc = "Polarity is active low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Ncepol6::Low
    }
    #[doc = "Polarity is active high"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Ncepol6::High
    }
}
#[doc = "Field `NCEPOL6` writer - Polarity select for NCE for GPIO 6"]
pub type Ncepol6W<'a, REG> = crate::BitWriter<'a, REG, Ncepol6>;
impl<'a, REG> Ncepol6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Polarity is active low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Ncepol6::Low)
    }
    #[doc = "Polarity is active high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Ncepol6::High)
    }
}
#[doc = "Field `FIEN6` reader - Force input enable active regardless of function selected. Otherwise the selected function will enable the input only when needed"]
pub type Fien6R = crate::BitReader;
#[doc = "Field `FIEN6` writer - Force input enable active regardless of function selected. Otherwise the selected function will enable the input only when needed"]
pub type Fien6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FOEN6` reader - Force output enable active regardless of function selected. Otherwise the selected function will enable the output only when needed"]
pub type Foen6R = crate::BitReader;
#[doc = "Field `FOEN6` writer - Force output enable active regardless of function selected. Otherwise the selected function will enable the output only when needed"]
pub type Foen6W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Function select for GPIO pin 6"]
    #[inline(always)]
    pub fn fncsel6(&self) -> Fncsel6R {
        Fncsel6R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Input enable for GPIO 6"]
    #[inline(always)]
    pub fn inpen6(&self) -> Inpen6R {
        Inpen6R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Return 0 for read data on GPIO 6"]
    #[inline(always)]
    pub fn rdzero6(&self) -> Rdzero6R {
        Rdzero6R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Interrupt enable for GPIO 6"]
    #[inline(always)]
    pub fn irpten6(&self) -> Irpten6R {
        Irpten6R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Pin IO mode selection for GPIO pin 6"]
    #[inline(always)]
    pub fn outcfg6(&self) -> Outcfg6R {
        Outcfg6R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Drive strength selection for GPIO 6"]
    #[inline(always)]
    pub fn ds6(&self) -> Ds6R {
        Ds6R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - Configure the slew rate"]
    #[inline(always)]
    pub fn sr6(&self) -> Sr6R {
        Sr6R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - Pullup/Pulldown configuration for GPIO 6"]
    #[inline(always)]
    pub fn pullcfg6(&self) -> Pullcfg6R {
        Pullcfg6R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:21 - IOMSTR/MSPI N Chip Select 6, DISP control signals DE, CSX, and CS. Polarity is determined by CE_POLARITY field"]
    #[inline(always)]
    pub fn ncesrc6(&self) -> Ncesrc6R {
        Ncesrc6R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 22 - Polarity select for NCE for GPIO 6"]
    #[inline(always)]
    pub fn ncepol6(&self) -> Ncepol6R {
        Ncepol6R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 26 - Force input enable active regardless of function selected. Otherwise the selected function will enable the input only when needed"]
    #[inline(always)]
    pub fn fien6(&self) -> Fien6R {
        Fien6R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Force output enable active regardless of function selected. Otherwise the selected function will enable the output only when needed"]
    #[inline(always)]
    pub fn foen6(&self) -> Foen6R {
        Foen6R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Function select for GPIO pin 6"]
    #[inline(always)]
    #[must_use]
    pub fn fncsel6(&mut self) -> Fncsel6W<Pincfg6Spec> {
        Fncsel6W::new(self, 0)
    }
    #[doc = "Bit 4 - Input enable for GPIO 6"]
    #[inline(always)]
    #[must_use]
    pub fn inpen6(&mut self) -> Inpen6W<Pincfg6Spec> {
        Inpen6W::new(self, 4)
    }
    #[doc = "Bit 5 - Return 0 for read data on GPIO 6"]
    #[inline(always)]
    #[must_use]
    pub fn rdzero6(&mut self) -> Rdzero6W<Pincfg6Spec> {
        Rdzero6W::new(self, 5)
    }
    #[doc = "Bits 6:7 - Interrupt enable for GPIO 6"]
    #[inline(always)]
    #[must_use]
    pub fn irpten6(&mut self) -> Irpten6W<Pincfg6Spec> {
        Irpten6W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Pin IO mode selection for GPIO pin 6"]
    #[inline(always)]
    #[must_use]
    pub fn outcfg6(&mut self) -> Outcfg6W<Pincfg6Spec> {
        Outcfg6W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Drive strength selection for GPIO 6"]
    #[inline(always)]
    #[must_use]
    pub fn ds6(&mut self) -> Ds6W<Pincfg6Spec> {
        Ds6W::new(self, 10)
    }
    #[doc = "Bit 12 - Configure the slew rate"]
    #[inline(always)]
    #[must_use]
    pub fn sr6(&mut self) -> Sr6W<Pincfg6Spec> {
        Sr6W::new(self, 12)
    }
    #[doc = "Bits 13:15 - Pullup/Pulldown configuration for GPIO 6"]
    #[inline(always)]
    #[must_use]
    pub fn pullcfg6(&mut self) -> Pullcfg6W<Pincfg6Spec> {
        Pullcfg6W::new(self, 13)
    }
    #[doc = "Bits 16:21 - IOMSTR/MSPI N Chip Select 6, DISP control signals DE, CSX, and CS. Polarity is determined by CE_POLARITY field"]
    #[inline(always)]
    #[must_use]
    pub fn ncesrc6(&mut self) -> Ncesrc6W<Pincfg6Spec> {
        Ncesrc6W::new(self, 16)
    }
    #[doc = "Bit 22 - Polarity select for NCE for GPIO 6"]
    #[inline(always)]
    #[must_use]
    pub fn ncepol6(&mut self) -> Ncepol6W<Pincfg6Spec> {
        Ncepol6W::new(self, 22)
    }
    #[doc = "Bit 26 - Force input enable active regardless of function selected. Otherwise the selected function will enable the input only when needed"]
    #[inline(always)]
    #[must_use]
    pub fn fien6(&mut self) -> Fien6W<Pincfg6Spec> {
        Fien6W::new(self, 26)
    }
    #[doc = "Bit 27 - Force output enable active regardless of function selected. Otherwise the selected function will enable the output only when needed"]
    #[inline(always)]
    #[must_use]
    pub fn foen6(&mut self) -> Foen6W<Pincfg6Spec> {
        Foen6W::new(self, 27)
    }
}
#[doc = "Controls the operation of GPIO pin 6.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pincfg6Spec;
impl crate::RegisterSpec for Pincfg6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pincfg6::R`](R) reader structure"]
impl crate::Readable for Pincfg6Spec {}
#[doc = "`write(|w| ..)` method takes [`pincfg6::W`](W) writer structure"]
impl crate::Writable for Pincfg6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PINCFG6 to value 0x03"]
impl crate::Resettable for Pincfg6Spec {
    const RESET_VALUE: u32 = 0x03;
}
