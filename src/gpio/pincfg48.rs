#[doc = "Register `PINCFG48` reader"]
pub type R = crate::R<Pincfg48Spec>;
#[doc = "Register `PINCFG48` writer"]
pub type W = crate::W<Pincfg48Spec>;
#[doc = "Function select for GPIO pin 48\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fncsel48 {
    #[doc = "0: Serial I2C Master Data I/O (I2C Mode) Serial SPI Master Data I/O (SPI 3 wire mode) (IOM 5)"]
    M5sdawir3 = 0,
    #[doc = "1: Serial SPI Master MOSI output (IOM 5)"]
    M5mosi = 1,
    #[doc = "2: Bidirectional I2S Data. Operates in output mode in master mode and input mode for slave mode. (I2S Master/Slave 2)"]
    I2s1Data = 2,
    #[doc = "3: General purpose I/O"]
    Gpio = 3,
    #[doc = "4: UART receive input (UART 2)"]
    Uart2rx = 4,
    #[doc = "5: UART receive input (UART 3)"]
    Uart3rx = 5,
    #[doc = "6: Timer/Counter input or output; Selection of direction is done via CTIMER register settings."]
    Ct48 = 6,
    #[doc = "7: IOMSTR/MSPI N Chip Select. Polarity is determined by CE_POLARITY field"]
    Nce48 = 7,
    #[doc = "8: Observation bus bit 0"]
    Obsbus0 = 8,
    #[doc = "9: I2S Data output (I2S Master/Slave 2)"]
    I2s1Sdout = 9,
    #[doc = "10: I2S Data output (I2S Master/Slave 2)"]
    I2s0Sdout = 10,
    #[doc = "11: Fast PIO"]
    Fpio = 11,
    #[doc = "12: Reserved selection. Operation unknown if selected."]
    Reserved12 = 12,
    #[doc = "13: Reserved selection. Operation unknown if selected."]
    Reserved13 = 13,
    #[doc = "14: Reserved selection. Operation unknown if selected."]
    Reserved14 = 14,
    #[doc = "15: Reserved selection. Operation unknown if selected."]
    Reserved15 = 15,
}
impl From<Fncsel48> for u8 {
    #[inline(always)]
    fn from(variant: Fncsel48) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fncsel48 {
    type Ux = u8;
}
impl crate::IsEnum for Fncsel48 {}
#[doc = "Field `FNCSEL48` reader - Function select for GPIO pin 48"]
pub type Fncsel48R = crate::FieldReader<Fncsel48>;
impl Fncsel48R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fncsel48 {
        match self.bits {
            0 => Fncsel48::M5sdawir3,
            1 => Fncsel48::M5mosi,
            2 => Fncsel48::I2s1Data,
            3 => Fncsel48::Gpio,
            4 => Fncsel48::Uart2rx,
            5 => Fncsel48::Uart3rx,
            6 => Fncsel48::Ct48,
            7 => Fncsel48::Nce48,
            8 => Fncsel48::Obsbus0,
            9 => Fncsel48::I2s1Sdout,
            10 => Fncsel48::I2s0Sdout,
            11 => Fncsel48::Fpio,
            12 => Fncsel48::Reserved12,
            13 => Fncsel48::Reserved13,
            14 => Fncsel48::Reserved14,
            15 => Fncsel48::Reserved15,
            _ => unreachable!(),
        }
    }
    #[doc = "Serial I2C Master Data I/O (I2C Mode) Serial SPI Master Data I/O (SPI 3 wire mode) (IOM 5)"]
    #[inline(always)]
    pub fn is_m5sdawir3(&self) -> bool {
        *self == Fncsel48::M5sdawir3
    }
    #[doc = "Serial SPI Master MOSI output (IOM 5)"]
    #[inline(always)]
    pub fn is_m5mosi(&self) -> bool {
        *self == Fncsel48::M5mosi
    }
    #[doc = "Bidirectional I2S Data. Operates in output mode in master mode and input mode for slave mode. (I2S Master/Slave 2)"]
    #[inline(always)]
    pub fn is_i2s1_data(&self) -> bool {
        *self == Fncsel48::I2s1Data
    }
    #[doc = "General purpose I/O"]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == Fncsel48::Gpio
    }
    #[doc = "UART receive input (UART 2)"]
    #[inline(always)]
    pub fn is_uart2rx(&self) -> bool {
        *self == Fncsel48::Uart2rx
    }
    #[doc = "UART receive input (UART 3)"]
    #[inline(always)]
    pub fn is_uart3rx(&self) -> bool {
        *self == Fncsel48::Uart3rx
    }
    #[doc = "Timer/Counter input or output; Selection of direction is done via CTIMER register settings."]
    #[inline(always)]
    pub fn is_ct48(&self) -> bool {
        *self == Fncsel48::Ct48
    }
    #[doc = "IOMSTR/MSPI N Chip Select. Polarity is determined by CE_POLARITY field"]
    #[inline(always)]
    pub fn is_nce48(&self) -> bool {
        *self == Fncsel48::Nce48
    }
    #[doc = "Observation bus bit 0"]
    #[inline(always)]
    pub fn is_obsbus0(&self) -> bool {
        *self == Fncsel48::Obsbus0
    }
    #[doc = "I2S Data output (I2S Master/Slave 2)"]
    #[inline(always)]
    pub fn is_i2s1_sdout(&self) -> bool {
        *self == Fncsel48::I2s1Sdout
    }
    #[doc = "I2S Data output (I2S Master/Slave 2)"]
    #[inline(always)]
    pub fn is_i2s0_sdout(&self) -> bool {
        *self == Fncsel48::I2s0Sdout
    }
    #[doc = "Fast PIO"]
    #[inline(always)]
    pub fn is_fpio(&self) -> bool {
        *self == Fncsel48::Fpio
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved12(&self) -> bool {
        *self == Fncsel48::Reserved12
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved13(&self) -> bool {
        *self == Fncsel48::Reserved13
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved14(&self) -> bool {
        *self == Fncsel48::Reserved14
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved15(&self) -> bool {
        *self == Fncsel48::Reserved15
    }
}
#[doc = "Field `FNCSEL48` writer - Function select for GPIO pin 48"]
pub type Fncsel48W<'a, REG> = crate::FieldWriter<'a, REG, 4, Fncsel48, crate::Safe>;
impl<'a, REG> Fncsel48W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Serial I2C Master Data I/O (I2C Mode) Serial SPI Master Data I/O (SPI 3 wire mode) (IOM 5)"]
    #[inline(always)]
    pub fn m5sdawir3(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel48::M5sdawir3)
    }
    #[doc = "Serial SPI Master MOSI output (IOM 5)"]
    #[inline(always)]
    pub fn m5mosi(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel48::M5mosi)
    }
    #[doc = "Bidirectional I2S Data. Operates in output mode in master mode and input mode for slave mode. (I2S Master/Slave 2)"]
    #[inline(always)]
    pub fn i2s1_data(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel48::I2s1Data)
    }
    #[doc = "General purpose I/O"]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel48::Gpio)
    }
    #[doc = "UART receive input (UART 2)"]
    #[inline(always)]
    pub fn uart2rx(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel48::Uart2rx)
    }
    #[doc = "UART receive input (UART 3)"]
    #[inline(always)]
    pub fn uart3rx(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel48::Uart3rx)
    }
    #[doc = "Timer/Counter input or output; Selection of direction is done via CTIMER register settings."]
    #[inline(always)]
    pub fn ct48(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel48::Ct48)
    }
    #[doc = "IOMSTR/MSPI N Chip Select. Polarity is determined by CE_POLARITY field"]
    #[inline(always)]
    pub fn nce48(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel48::Nce48)
    }
    #[doc = "Observation bus bit 0"]
    #[inline(always)]
    pub fn obsbus0(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel48::Obsbus0)
    }
    #[doc = "I2S Data output (I2S Master/Slave 2)"]
    #[inline(always)]
    pub fn i2s1_sdout(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel48::I2s1Sdout)
    }
    #[doc = "I2S Data output (I2S Master/Slave 2)"]
    #[inline(always)]
    pub fn i2s0_sdout(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel48::I2s0Sdout)
    }
    #[doc = "Fast PIO"]
    #[inline(always)]
    pub fn fpio(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel48::Fpio)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved12(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel48::Reserved12)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved13(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel48::Reserved13)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved14(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel48::Reserved14)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved15(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel48::Reserved15)
    }
}
#[doc = "Field `INPEN48` reader - Input enable for GPIO 48"]
pub type Inpen48R = crate::BitReader;
#[doc = "Field `INPEN48` writer - Input enable for GPIO 48"]
pub type Inpen48W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDZERO48` reader - Return 0 for read data on GPIO 48"]
pub type Rdzero48R = crate::BitReader;
#[doc = "Field `RDZERO48` writer - Return 0 for read data on GPIO 48"]
pub type Rdzero48W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Interrupt enable for GPIO 48\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Irpten48 {
    #[doc = "0: Interrupts are disabled for this GPIO"]
    Dis = 0,
    #[doc = "1: Interrupts are enabled for falling edge transition on this GPIO"]
    Intfall = 1,
    #[doc = "2: Interrupts are enabled for rising edge transitions on this GPIO"]
    Intrise = 2,
    #[doc = "3: Interrupts are enabled for any edge transition on this GPIO"]
    Intany = 3,
}
impl From<Irpten48> for u8 {
    #[inline(always)]
    fn from(variant: Irpten48) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Irpten48 {
    type Ux = u8;
}
impl crate::IsEnum for Irpten48 {}
#[doc = "Field `IRPTEN48` reader - Interrupt enable for GPIO 48"]
pub type Irpten48R = crate::FieldReader<Irpten48>;
impl Irpten48R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irpten48 {
        match self.bits {
            0 => Irpten48::Dis,
            1 => Irpten48::Intfall,
            2 => Irpten48::Intrise,
            3 => Irpten48::Intany,
            _ => unreachable!(),
        }
    }
    #[doc = "Interrupts are disabled for this GPIO"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Irpten48::Dis
    }
    #[doc = "Interrupts are enabled for falling edge transition on this GPIO"]
    #[inline(always)]
    pub fn is_intfall(&self) -> bool {
        *self == Irpten48::Intfall
    }
    #[doc = "Interrupts are enabled for rising edge transitions on this GPIO"]
    #[inline(always)]
    pub fn is_intrise(&self) -> bool {
        *self == Irpten48::Intrise
    }
    #[doc = "Interrupts are enabled for any edge transition on this GPIO"]
    #[inline(always)]
    pub fn is_intany(&self) -> bool {
        *self == Irpten48::Intany
    }
}
#[doc = "Field `IRPTEN48` writer - Interrupt enable for GPIO 48"]
pub type Irpten48W<'a, REG> = crate::FieldWriter<'a, REG, 2, Irpten48, crate::Safe>;
impl<'a, REG> Irpten48W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Interrupts are disabled for this GPIO"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten48::Dis)
    }
    #[doc = "Interrupts are enabled for falling edge transition on this GPIO"]
    #[inline(always)]
    pub fn intfall(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten48::Intfall)
    }
    #[doc = "Interrupts are enabled for rising edge transitions on this GPIO"]
    #[inline(always)]
    pub fn intrise(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten48::Intrise)
    }
    #[doc = "Interrupts are enabled for any edge transition on this GPIO"]
    #[inline(always)]
    pub fn intany(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten48::Intany)
    }
}
#[doc = "Pin IO mode selection for GPIO pin 48\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Outcfg48 {
    #[doc = "0: Output Disabled"]
    Dis = 0,
    #[doc = "1: Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    Pushpull = 1,
    #[doc = "2: Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    Od = 2,
    #[doc = "3: Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    Ts = 3,
}
impl From<Outcfg48> for u8 {
    #[inline(always)]
    fn from(variant: Outcfg48) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Outcfg48 {
    type Ux = u8;
}
impl crate::IsEnum for Outcfg48 {}
#[doc = "Field `OUTCFG48` reader - Pin IO mode selection for GPIO pin 48"]
pub type Outcfg48R = crate::FieldReader<Outcfg48>;
impl Outcfg48R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Outcfg48 {
        match self.bits {
            0 => Outcfg48::Dis,
            1 => Outcfg48::Pushpull,
            2 => Outcfg48::Od,
            3 => Outcfg48::Ts,
            _ => unreachable!(),
        }
    }
    #[doc = "Output Disabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Outcfg48::Dis
    }
    #[doc = "Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == Outcfg48::Pushpull
    }
    #[doc = "Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == Outcfg48::Od
    }
    #[doc = "Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == Outcfg48::Ts
    }
}
#[doc = "Field `OUTCFG48` writer - Pin IO mode selection for GPIO pin 48"]
pub type Outcfg48W<'a, REG> = crate::FieldWriter<'a, REG, 2, Outcfg48, crate::Safe>;
impl<'a, REG> Outcfg48W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Output Disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg48::Dis)
    }
    #[doc = "Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg48::Pushpull)
    }
    #[doc = "Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    #[inline(always)]
    pub fn od(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg48::Od)
    }
    #[doc = "Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    #[inline(always)]
    pub fn ts(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg48::Ts)
    }
}
#[doc = "Drive strength selection for GPIO 48\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ds48 {
    #[doc = "0: 0.1x output driver selected"]
    _0p1x = 0,
    #[doc = "1: 0.5x output driver selected"]
    _0p5x = 1,
    #[doc = "2: 0.75x output driver selected"]
    _0p75x = 2,
    #[doc = "3: 1.0x output driver selected"]
    _1p0x = 3,
}
impl From<Ds48> for u8 {
    #[inline(always)]
    fn from(variant: Ds48) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ds48 {
    type Ux = u8;
}
impl crate::IsEnum for Ds48 {}
#[doc = "Field `DS48` reader - Drive strength selection for GPIO 48"]
pub type Ds48R = crate::FieldReader<Ds48>;
impl Ds48R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ds48 {
        match self.bits {
            0 => Ds48::_0p1x,
            1 => Ds48::_0p5x,
            2 => Ds48::_0p75x,
            3 => Ds48::_1p0x,
            _ => unreachable!(),
        }
    }
    #[doc = "0.1x output driver selected"]
    #[inline(always)]
    pub fn is_0p1x(&self) -> bool {
        *self == Ds48::_0p1x
    }
    #[doc = "0.5x output driver selected"]
    #[inline(always)]
    pub fn is_0p5x(&self) -> bool {
        *self == Ds48::_0p5x
    }
    #[doc = "0.75x output driver selected"]
    #[inline(always)]
    pub fn is_0p75x(&self) -> bool {
        *self == Ds48::_0p75x
    }
    #[doc = "1.0x output driver selected"]
    #[inline(always)]
    pub fn is_1p0x(&self) -> bool {
        *self == Ds48::_1p0x
    }
}
#[doc = "Field `DS48` writer - Drive strength selection for GPIO 48"]
pub type Ds48W<'a, REG> = crate::FieldWriter<'a, REG, 2, Ds48, crate::Safe>;
impl<'a, REG> Ds48W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "0.1x output driver selected"]
    #[inline(always)]
    pub fn _0p1x(self) -> &'a mut crate::W<REG> {
        self.variant(Ds48::_0p1x)
    }
    #[doc = "0.5x output driver selected"]
    #[inline(always)]
    pub fn _0p5x(self) -> &'a mut crate::W<REG> {
        self.variant(Ds48::_0p5x)
    }
    #[doc = "0.75x output driver selected"]
    #[inline(always)]
    pub fn _0p75x(self) -> &'a mut crate::W<REG> {
        self.variant(Ds48::_0p75x)
    }
    #[doc = "1.0x output driver selected"]
    #[inline(always)]
    pub fn _1p0x(self) -> &'a mut crate::W<REG> {
        self.variant(Ds48::_1p0x)
    }
}
#[doc = "Field `SR48` reader - Configure the slew rate"]
pub type Sr48R = crate::BitReader;
#[doc = "Field `SR48` writer - Configure the slew rate"]
pub type Sr48W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Pullup/Pulldown configuration for GPIO 48\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pullcfg48 {
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
impl From<Pullcfg48> for u8 {
    #[inline(always)]
    fn from(variant: Pullcfg48) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pullcfg48 {
    type Ux = u8;
}
impl crate::IsEnum for Pullcfg48 {}
#[doc = "Field `PULLCFG48` reader - Pullup/Pulldown configuration for GPIO 48"]
pub type Pullcfg48R = crate::FieldReader<Pullcfg48>;
impl Pullcfg48R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pullcfg48 {
        match self.bits {
            0 => Pullcfg48::Dis,
            1 => Pullcfg48::Pd50k,
            2 => Pullcfg48::Pu15k,
            3 => Pullcfg48::Pu6k,
            4 => Pullcfg48::Pu12k,
            5 => Pullcfg48::Pu24k,
            6 => Pullcfg48::Pu50k,
            7 => Pullcfg48::Pu100k,
            _ => unreachable!(),
        }
    }
    #[doc = "No pullup or pulldown selected"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Pullcfg48::Dis
    }
    #[doc = "50K Pulldown selected"]
    #[inline(always)]
    pub fn is_pd50k(&self) -> bool {
        *self == Pullcfg48::Pd50k
    }
    #[doc = "1.5K Pullup selected"]
    #[inline(always)]
    pub fn is_pu15k(&self) -> bool {
        *self == Pullcfg48::Pu15k
    }
    #[doc = "6K Pullup selected"]
    #[inline(always)]
    pub fn is_pu6k(&self) -> bool {
        *self == Pullcfg48::Pu6k
    }
    #[doc = "12K Pullup selected"]
    #[inline(always)]
    pub fn is_pu12k(&self) -> bool {
        *self == Pullcfg48::Pu12k
    }
    #[doc = "24K Pullup selected"]
    #[inline(always)]
    pub fn is_pu24k(&self) -> bool {
        *self == Pullcfg48::Pu24k
    }
    #[doc = "50K Pullup selected"]
    #[inline(always)]
    pub fn is_pu50k(&self) -> bool {
        *self == Pullcfg48::Pu50k
    }
    #[doc = "100K Pullup selected"]
    #[inline(always)]
    pub fn is_pu100k(&self) -> bool {
        *self == Pullcfg48::Pu100k
    }
}
#[doc = "Field `PULLCFG48` writer - Pullup/Pulldown configuration for GPIO 48"]
pub type Pullcfg48W<'a, REG> = crate::FieldWriter<'a, REG, 3, Pullcfg48, crate::Safe>;
impl<'a, REG> Pullcfg48W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No pullup or pulldown selected"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg48::Dis)
    }
    #[doc = "50K Pulldown selected"]
    #[inline(always)]
    pub fn pd50k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg48::Pd50k)
    }
    #[doc = "1.5K Pullup selected"]
    #[inline(always)]
    pub fn pu15k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg48::Pu15k)
    }
    #[doc = "6K Pullup selected"]
    #[inline(always)]
    pub fn pu6k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg48::Pu6k)
    }
    #[doc = "12K Pullup selected"]
    #[inline(always)]
    pub fn pu12k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg48::Pu12k)
    }
    #[doc = "24K Pullup selected"]
    #[inline(always)]
    pub fn pu24k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg48::Pu24k)
    }
    #[doc = "50K Pullup selected"]
    #[inline(always)]
    pub fn pu50k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg48::Pu50k)
    }
    #[doc = "100K Pullup selected"]
    #[inline(always)]
    pub fn pu100k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg48::Pu100k)
    }
}
#[doc = "IOMSTR/MSPI N Chip Select 48, DISP control signals DE, CSX, and CS. Polarity is determined by CE_POLARITY field\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ncesrc48 {
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
impl From<Ncesrc48> for u8 {
    #[inline(always)]
    fn from(variant: Ncesrc48) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ncesrc48 {
    type Ux = u8;
}
impl crate::IsEnum for Ncesrc48 {}
#[doc = "Field `NCESRC48` reader - IOMSTR/MSPI N Chip Select 48, DISP control signals DE, CSX, and CS. Polarity is determined by CE_POLARITY field"]
pub type Ncesrc48R = crate::FieldReader<Ncesrc48>;
impl Ncesrc48R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ncesrc48> {
        match self.bits {
            0 => Some(Ncesrc48::Iom0ce0),
            1 => Some(Ncesrc48::Iom0ce1),
            2 => Some(Ncesrc48::Iom0ce2),
            3 => Some(Ncesrc48::Iom0ce3),
            4 => Some(Ncesrc48::Iom1ce0),
            5 => Some(Ncesrc48::Iom1ce1),
            6 => Some(Ncesrc48::Iom1ce2),
            7 => Some(Ncesrc48::Iom1ce3),
            8 => Some(Ncesrc48::Iom2ce0),
            9 => Some(Ncesrc48::Iom2ce1),
            10 => Some(Ncesrc48::Iom2ce2),
            11 => Some(Ncesrc48::Iom2ce3),
            12 => Some(Ncesrc48::Iom3ce0),
            13 => Some(Ncesrc48::Iom3ce1),
            14 => Some(Ncesrc48::Iom3ce2),
            15 => Some(Ncesrc48::Iom3ce3),
            16 => Some(Ncesrc48::Iom4ce0),
            17 => Some(Ncesrc48::Iom4ce1),
            18 => Some(Ncesrc48::Iom4ce2),
            19 => Some(Ncesrc48::Iom4ce3),
            20 => Some(Ncesrc48::Iom5ce0),
            21 => Some(Ncesrc48::Iom5ce1),
            22 => Some(Ncesrc48::Iom5ce2),
            23 => Some(Ncesrc48::Iom5ce3),
            24 => Some(Ncesrc48::Iom6ce0),
            25 => Some(Ncesrc48::Iom6ce1),
            26 => Some(Ncesrc48::Iom6ce2),
            27 => Some(Ncesrc48::Iom6ce3),
            28 => Some(Ncesrc48::Iom7ce0),
            29 => Some(Ncesrc48::Iom7ce1),
            30 => Some(Ncesrc48::Iom7ce2),
            31 => Some(Ncesrc48::Iom7ce3),
            32 => Some(Ncesrc48::Mspi0cen0),
            33 => Some(Ncesrc48::Mspi0cen1),
            34 => Some(Ncesrc48::Mspi1cen0),
            35 => Some(Ncesrc48::Mspi1cen1),
            36 => Some(Ncesrc48::Mspi2cen0),
            37 => Some(Ncesrc48::Mspi2cen1),
            38 => Some(Ncesrc48::DcDpiDe),
            39 => Some(Ncesrc48::DispContCsx),
            40 => Some(Ncesrc48::DcSpiCsN),
            41 => Some(Ncesrc48::DcQspiCsN),
            42 => Some(Ncesrc48::DcResx),
            _ => None,
        }
    }
    #[doc = "IOM 0 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom0ce0(&self) -> bool {
        *self == Ncesrc48::Iom0ce0
    }
    #[doc = "IOM 0 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom0ce1(&self) -> bool {
        *self == Ncesrc48::Iom0ce1
    }
    #[doc = "IOM 0 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom0ce2(&self) -> bool {
        *self == Ncesrc48::Iom0ce2
    }
    #[doc = "IOM 0 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom0ce3(&self) -> bool {
        *self == Ncesrc48::Iom0ce3
    }
    #[doc = "IOM 1 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom1ce0(&self) -> bool {
        *self == Ncesrc48::Iom1ce0
    }
    #[doc = "IOM 1 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom1ce1(&self) -> bool {
        *self == Ncesrc48::Iom1ce1
    }
    #[doc = "IOM 1 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom1ce2(&self) -> bool {
        *self == Ncesrc48::Iom1ce2
    }
    #[doc = "IOM 1 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom1ce3(&self) -> bool {
        *self == Ncesrc48::Iom1ce3
    }
    #[doc = "IOM 2 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom2ce0(&self) -> bool {
        *self == Ncesrc48::Iom2ce0
    }
    #[doc = "IOM 2 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom2ce1(&self) -> bool {
        *self == Ncesrc48::Iom2ce1
    }
    #[doc = "IOM 2 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom2ce2(&self) -> bool {
        *self == Ncesrc48::Iom2ce2
    }
    #[doc = "IOM 2 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom2ce3(&self) -> bool {
        *self == Ncesrc48::Iom2ce3
    }
    #[doc = "IOM 3 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom3ce0(&self) -> bool {
        *self == Ncesrc48::Iom3ce0
    }
    #[doc = "IOM 3 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom3ce1(&self) -> bool {
        *self == Ncesrc48::Iom3ce1
    }
    #[doc = "IOM 3 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom3ce2(&self) -> bool {
        *self == Ncesrc48::Iom3ce2
    }
    #[doc = "IOM 3 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom3ce3(&self) -> bool {
        *self == Ncesrc48::Iom3ce3
    }
    #[doc = "IOM 4 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom4ce0(&self) -> bool {
        *self == Ncesrc48::Iom4ce0
    }
    #[doc = "IOM 4 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom4ce1(&self) -> bool {
        *self == Ncesrc48::Iom4ce1
    }
    #[doc = "IOM 4 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom4ce2(&self) -> bool {
        *self == Ncesrc48::Iom4ce2
    }
    #[doc = "IOM 4 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom4ce3(&self) -> bool {
        *self == Ncesrc48::Iom4ce3
    }
    #[doc = "IOM 5 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom5ce0(&self) -> bool {
        *self == Ncesrc48::Iom5ce0
    }
    #[doc = "IOM 5 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom5ce1(&self) -> bool {
        *self == Ncesrc48::Iom5ce1
    }
    #[doc = "IOM 5 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom5ce2(&self) -> bool {
        *self == Ncesrc48::Iom5ce2
    }
    #[doc = "IOM 5 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom5ce3(&self) -> bool {
        *self == Ncesrc48::Iom5ce3
    }
    #[doc = "IOM 6 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom6ce0(&self) -> bool {
        *self == Ncesrc48::Iom6ce0
    }
    #[doc = "IOM 6 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom6ce1(&self) -> bool {
        *self == Ncesrc48::Iom6ce1
    }
    #[doc = "IOM 6 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom6ce2(&self) -> bool {
        *self == Ncesrc48::Iom6ce2
    }
    #[doc = "IOM 6 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom6ce3(&self) -> bool {
        *self == Ncesrc48::Iom6ce3
    }
    #[doc = "IOM 7 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom7ce0(&self) -> bool {
        *self == Ncesrc48::Iom7ce0
    }
    #[doc = "IOM 7 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom7ce1(&self) -> bool {
        *self == Ncesrc48::Iom7ce1
    }
    #[doc = "IOM 7 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom7ce2(&self) -> bool {
        *self == Ncesrc48::Iom7ce2
    }
    #[doc = "IOM 7 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom7ce3(&self) -> bool {
        *self == Ncesrc48::Iom7ce3
    }
    #[doc = "MSPI 0 NCE 0 module"]
    #[inline(always)]
    pub fn is_mspi0cen0(&self) -> bool {
        *self == Ncesrc48::Mspi0cen0
    }
    #[doc = "MSPI 0 NCE 1 module"]
    #[inline(always)]
    pub fn is_mspi0cen1(&self) -> bool {
        *self == Ncesrc48::Mspi0cen1
    }
    #[doc = "MSPI 1 NCE 0 module"]
    #[inline(always)]
    pub fn is_mspi1cen0(&self) -> bool {
        *self == Ncesrc48::Mspi1cen0
    }
    #[doc = "MSPI 1 NCE 1 module"]
    #[inline(always)]
    pub fn is_mspi1cen1(&self) -> bool {
        *self == Ncesrc48::Mspi1cen1
    }
    #[doc = "MSPI 2 NCE 0 module"]
    #[inline(always)]
    pub fn is_mspi2cen0(&self) -> bool {
        *self == Ncesrc48::Mspi2cen0
    }
    #[doc = "MSPI 2 NCE 1 module"]
    #[inline(always)]
    pub fn is_mspi2cen1(&self) -> bool {
        *self == Ncesrc48::Mspi2cen1
    }
    #[doc = "DC DPI DE module"]
    #[inline(always)]
    pub fn is_dc_dpi_de(&self) -> bool {
        *self == Ncesrc48::DcDpiDe
    }
    #[doc = "DISP CONT CSX module"]
    #[inline(always)]
    pub fn is_disp_cont_csx(&self) -> bool {
        *self == Ncesrc48::DispContCsx
    }
    #[doc = "DC SPI CS_N module"]
    #[inline(always)]
    pub fn is_dc_spi_cs_n(&self) -> bool {
        *self == Ncesrc48::DcSpiCsN
    }
    #[doc = "DC QSPI CS_N module"]
    #[inline(always)]
    pub fn is_dc_qspi_cs_n(&self) -> bool {
        *self == Ncesrc48::DcQspiCsN
    }
    #[doc = "DC module RESX"]
    #[inline(always)]
    pub fn is_dc_resx(&self) -> bool {
        *self == Ncesrc48::DcResx
    }
}
#[doc = "Field `NCESRC48` writer - IOMSTR/MSPI N Chip Select 48, DISP control signals DE, CSX, and CS. Polarity is determined by CE_POLARITY field"]
pub type Ncesrc48W<'a, REG> = crate::FieldWriter<'a, REG, 6, Ncesrc48>;
impl<'a, REG> Ncesrc48W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "IOM 0 NCE 0 module"]
    #[inline(always)]
    pub fn iom0ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc48::Iom0ce0)
    }
    #[doc = "IOM 0 NCE 1 module"]
    #[inline(always)]
    pub fn iom0ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc48::Iom0ce1)
    }
    #[doc = "IOM 0 NCE 2 module"]
    #[inline(always)]
    pub fn iom0ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc48::Iom0ce2)
    }
    #[doc = "IOM 0 NCE 3 module"]
    #[inline(always)]
    pub fn iom0ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc48::Iom0ce3)
    }
    #[doc = "IOM 1 NCE 0 module"]
    #[inline(always)]
    pub fn iom1ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc48::Iom1ce0)
    }
    #[doc = "IOM 1 NCE 1 module"]
    #[inline(always)]
    pub fn iom1ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc48::Iom1ce1)
    }
    #[doc = "IOM 1 NCE 2 module"]
    #[inline(always)]
    pub fn iom1ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc48::Iom1ce2)
    }
    #[doc = "IOM 1 NCE 3 module"]
    #[inline(always)]
    pub fn iom1ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc48::Iom1ce3)
    }
    #[doc = "IOM 2 NCE 0 module"]
    #[inline(always)]
    pub fn iom2ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc48::Iom2ce0)
    }
    #[doc = "IOM 2 NCE 1 module"]
    #[inline(always)]
    pub fn iom2ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc48::Iom2ce1)
    }
    #[doc = "IOM 2 NCE 2 module"]
    #[inline(always)]
    pub fn iom2ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc48::Iom2ce2)
    }
    #[doc = "IOM 2 NCE 3 module"]
    #[inline(always)]
    pub fn iom2ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc48::Iom2ce3)
    }
    #[doc = "IOM 3 NCE 0 module"]
    #[inline(always)]
    pub fn iom3ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc48::Iom3ce0)
    }
    #[doc = "IOM 3 NCE 1 module"]
    #[inline(always)]
    pub fn iom3ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc48::Iom3ce1)
    }
    #[doc = "IOM 3 NCE 2 module"]
    #[inline(always)]
    pub fn iom3ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc48::Iom3ce2)
    }
    #[doc = "IOM 3 NCE 3 module"]
    #[inline(always)]
    pub fn iom3ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc48::Iom3ce3)
    }
    #[doc = "IOM 4 NCE 0 module"]
    #[inline(always)]
    pub fn iom4ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc48::Iom4ce0)
    }
    #[doc = "IOM 4 NCE 1 module"]
    #[inline(always)]
    pub fn iom4ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc48::Iom4ce1)
    }
    #[doc = "IOM 4 NCE 2 module"]
    #[inline(always)]
    pub fn iom4ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc48::Iom4ce2)
    }
    #[doc = "IOM 4 NCE 3 module"]
    #[inline(always)]
    pub fn iom4ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc48::Iom4ce3)
    }
    #[doc = "IOM 5 NCE 0 module"]
    #[inline(always)]
    pub fn iom5ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc48::Iom5ce0)
    }
    #[doc = "IOM 5 NCE 1 module"]
    #[inline(always)]
    pub fn iom5ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc48::Iom5ce1)
    }
    #[doc = "IOM 5 NCE 2 module"]
    #[inline(always)]
    pub fn iom5ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc48::Iom5ce2)
    }
    #[doc = "IOM 5 NCE 3 module"]
    #[inline(always)]
    pub fn iom5ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc48::Iom5ce3)
    }
    #[doc = "IOM 6 NCE 0 module"]
    #[inline(always)]
    pub fn iom6ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc48::Iom6ce0)
    }
    #[doc = "IOM 6 NCE 1 module"]
    #[inline(always)]
    pub fn iom6ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc48::Iom6ce1)
    }
    #[doc = "IOM 6 NCE 2 module"]
    #[inline(always)]
    pub fn iom6ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc48::Iom6ce2)
    }
    #[doc = "IOM 6 NCE 3 module"]
    #[inline(always)]
    pub fn iom6ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc48::Iom6ce3)
    }
    #[doc = "IOM 7 NCE 0 module"]
    #[inline(always)]
    pub fn iom7ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc48::Iom7ce0)
    }
    #[doc = "IOM 7 NCE 1 module"]
    #[inline(always)]
    pub fn iom7ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc48::Iom7ce1)
    }
    #[doc = "IOM 7 NCE 2 module"]
    #[inline(always)]
    pub fn iom7ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc48::Iom7ce2)
    }
    #[doc = "IOM 7 NCE 3 module"]
    #[inline(always)]
    pub fn iom7ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc48::Iom7ce3)
    }
    #[doc = "MSPI 0 NCE 0 module"]
    #[inline(always)]
    pub fn mspi0cen0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc48::Mspi0cen0)
    }
    #[doc = "MSPI 0 NCE 1 module"]
    #[inline(always)]
    pub fn mspi0cen1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc48::Mspi0cen1)
    }
    #[doc = "MSPI 1 NCE 0 module"]
    #[inline(always)]
    pub fn mspi1cen0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc48::Mspi1cen0)
    }
    #[doc = "MSPI 1 NCE 1 module"]
    #[inline(always)]
    pub fn mspi1cen1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc48::Mspi1cen1)
    }
    #[doc = "MSPI 2 NCE 0 module"]
    #[inline(always)]
    pub fn mspi2cen0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc48::Mspi2cen0)
    }
    #[doc = "MSPI 2 NCE 1 module"]
    #[inline(always)]
    pub fn mspi2cen1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc48::Mspi2cen1)
    }
    #[doc = "DC DPI DE module"]
    #[inline(always)]
    pub fn dc_dpi_de(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc48::DcDpiDe)
    }
    #[doc = "DISP CONT CSX module"]
    #[inline(always)]
    pub fn disp_cont_csx(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc48::DispContCsx)
    }
    #[doc = "DC SPI CS_N module"]
    #[inline(always)]
    pub fn dc_spi_cs_n(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc48::DcSpiCsN)
    }
    #[doc = "DC QSPI CS_N module"]
    #[inline(always)]
    pub fn dc_qspi_cs_n(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc48::DcQspiCsN)
    }
    #[doc = "DC module RESX"]
    #[inline(always)]
    pub fn dc_resx(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc48::DcResx)
    }
}
#[doc = "Polarity select for NCE for GPIO 48\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ncepol48 {
    #[doc = "0: Polarity is active low"]
    Low = 0,
    #[doc = "1: Polarity is active high"]
    High = 1,
}
impl From<Ncepol48> for bool {
    #[inline(always)]
    fn from(variant: Ncepol48) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NCEPOL48` reader - Polarity select for NCE for GPIO 48"]
pub type Ncepol48R = crate::BitReader<Ncepol48>;
impl Ncepol48R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ncepol48 {
        match self.bits {
            false => Ncepol48::Low,
            true => Ncepol48::High,
        }
    }
    #[doc = "Polarity is active low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Ncepol48::Low
    }
    #[doc = "Polarity is active high"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Ncepol48::High
    }
}
#[doc = "Field `NCEPOL48` writer - Polarity select for NCE for GPIO 48"]
pub type Ncepol48W<'a, REG> = crate::BitWriter<'a, REG, Ncepol48>;
impl<'a, REG> Ncepol48W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Polarity is active low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Ncepol48::Low)
    }
    #[doc = "Polarity is active high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Ncepol48::High)
    }
}
#[doc = "Field `FIEN48` reader - Force input enable active regardless of function selected. Otherwise the selected function will enable the input only when needed"]
pub type Fien48R = crate::BitReader;
#[doc = "Field `FIEN48` writer - Force input enable active regardless of function selected. Otherwise the selected function will enable the input only when needed"]
pub type Fien48W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FOEN48` reader - Force output enable active regardless of function selected. Otherwise the selected function will enable the output only when needed"]
pub type Foen48R = crate::BitReader;
#[doc = "Field `FOEN48` writer - Force output enable active regardless of function selected. Otherwise the selected function will enable the output only when needed"]
pub type Foen48W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Function select for GPIO pin 48"]
    #[inline(always)]
    pub fn fncsel48(&self) -> Fncsel48R {
        Fncsel48R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Input enable for GPIO 48"]
    #[inline(always)]
    pub fn inpen48(&self) -> Inpen48R {
        Inpen48R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Return 0 for read data on GPIO 48"]
    #[inline(always)]
    pub fn rdzero48(&self) -> Rdzero48R {
        Rdzero48R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Interrupt enable for GPIO 48"]
    #[inline(always)]
    pub fn irpten48(&self) -> Irpten48R {
        Irpten48R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Pin IO mode selection for GPIO pin 48"]
    #[inline(always)]
    pub fn outcfg48(&self) -> Outcfg48R {
        Outcfg48R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Drive strength selection for GPIO 48"]
    #[inline(always)]
    pub fn ds48(&self) -> Ds48R {
        Ds48R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - Configure the slew rate"]
    #[inline(always)]
    pub fn sr48(&self) -> Sr48R {
        Sr48R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - Pullup/Pulldown configuration for GPIO 48"]
    #[inline(always)]
    pub fn pullcfg48(&self) -> Pullcfg48R {
        Pullcfg48R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:21 - IOMSTR/MSPI N Chip Select 48, DISP control signals DE, CSX, and CS. Polarity is determined by CE_POLARITY field"]
    #[inline(always)]
    pub fn ncesrc48(&self) -> Ncesrc48R {
        Ncesrc48R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 22 - Polarity select for NCE for GPIO 48"]
    #[inline(always)]
    pub fn ncepol48(&self) -> Ncepol48R {
        Ncepol48R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 26 - Force input enable active regardless of function selected. Otherwise the selected function will enable the input only when needed"]
    #[inline(always)]
    pub fn fien48(&self) -> Fien48R {
        Fien48R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Force output enable active regardless of function selected. Otherwise the selected function will enable the output only when needed"]
    #[inline(always)]
    pub fn foen48(&self) -> Foen48R {
        Foen48R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Function select for GPIO pin 48"]
    #[inline(always)]
    #[must_use]
    pub fn fncsel48(&mut self) -> Fncsel48W<Pincfg48Spec> {
        Fncsel48W::new(self, 0)
    }
    #[doc = "Bit 4 - Input enable for GPIO 48"]
    #[inline(always)]
    #[must_use]
    pub fn inpen48(&mut self) -> Inpen48W<Pincfg48Spec> {
        Inpen48W::new(self, 4)
    }
    #[doc = "Bit 5 - Return 0 for read data on GPIO 48"]
    #[inline(always)]
    #[must_use]
    pub fn rdzero48(&mut self) -> Rdzero48W<Pincfg48Spec> {
        Rdzero48W::new(self, 5)
    }
    #[doc = "Bits 6:7 - Interrupt enable for GPIO 48"]
    #[inline(always)]
    #[must_use]
    pub fn irpten48(&mut self) -> Irpten48W<Pincfg48Spec> {
        Irpten48W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Pin IO mode selection for GPIO pin 48"]
    #[inline(always)]
    #[must_use]
    pub fn outcfg48(&mut self) -> Outcfg48W<Pincfg48Spec> {
        Outcfg48W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Drive strength selection for GPIO 48"]
    #[inline(always)]
    #[must_use]
    pub fn ds48(&mut self) -> Ds48W<Pincfg48Spec> {
        Ds48W::new(self, 10)
    }
    #[doc = "Bit 12 - Configure the slew rate"]
    #[inline(always)]
    #[must_use]
    pub fn sr48(&mut self) -> Sr48W<Pincfg48Spec> {
        Sr48W::new(self, 12)
    }
    #[doc = "Bits 13:15 - Pullup/Pulldown configuration for GPIO 48"]
    #[inline(always)]
    #[must_use]
    pub fn pullcfg48(&mut self) -> Pullcfg48W<Pincfg48Spec> {
        Pullcfg48W::new(self, 13)
    }
    #[doc = "Bits 16:21 - IOMSTR/MSPI N Chip Select 48, DISP control signals DE, CSX, and CS. Polarity is determined by CE_POLARITY field"]
    #[inline(always)]
    #[must_use]
    pub fn ncesrc48(&mut self) -> Ncesrc48W<Pincfg48Spec> {
        Ncesrc48W::new(self, 16)
    }
    #[doc = "Bit 22 - Polarity select for NCE for GPIO 48"]
    #[inline(always)]
    #[must_use]
    pub fn ncepol48(&mut self) -> Ncepol48W<Pincfg48Spec> {
        Ncepol48W::new(self, 22)
    }
    #[doc = "Bit 26 - Force input enable active regardless of function selected. Otherwise the selected function will enable the input only when needed"]
    #[inline(always)]
    #[must_use]
    pub fn fien48(&mut self) -> Fien48W<Pincfg48Spec> {
        Fien48W::new(self, 26)
    }
    #[doc = "Bit 27 - Force output enable active regardless of function selected. Otherwise the selected function will enable the output only when needed"]
    #[inline(always)]
    #[must_use]
    pub fn foen48(&mut self) -> Foen48W<Pincfg48Spec> {
        Foen48W::new(self, 27)
    }
}
#[doc = "Controls the operation of GPIO pin 48.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg48::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg48::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pincfg48Spec;
impl crate::RegisterSpec for Pincfg48Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pincfg48::R`](R) reader structure"]
impl crate::Readable for Pincfg48Spec {}
#[doc = "`write(|w| ..)` method takes [`pincfg48::W`](W) writer structure"]
impl crate::Writable for Pincfg48Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PINCFG48 to value 0x03"]
impl crate::Resettable for Pincfg48Spec {
    const RESET_VALUE: u32 = 0x03;
}
