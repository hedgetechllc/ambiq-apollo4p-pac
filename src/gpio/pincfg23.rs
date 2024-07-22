#[doc = "Register `PINCFG23` reader"]
pub type R = crate::R<Pincfg23Spec>;
#[doc = "Register `PINCFG23` writer"]
pub type W = crate::W<Pincfg23Spec>;
#[doc = "Function select for GPIO pin 23\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fncsel23 {
    #[doc = "0: Serial I2C Master Data I/O (I2C Mode) Serial SPI Master Data I/O (SPI 3 wire mode) (IOM 7)"]
    M7sdawir3 = 0,
    #[doc = "1: Serial SPI Master MOSI output (IOM 7)"]
    M7mosi = 1,
    #[doc = "2: Serial Wire Output"]
    Swo = 2,
    #[doc = "3: General purpose I/O"]
    Gpio = 3,
    #[doc = "4: UART receive input (UART 2)"]
    Uart2rx = 4,
    #[doc = "5: UART receive input (UART 3)"]
    Uart3rx = 5,
    #[doc = "6: Timer/Counter input or output; Selection of direction is done via CTIMER register settings."]
    Ct23 = 6,
    #[doc = "7: IOMSTR/MSPI N Chip Select. Polarity is determined by CE_POLARITY field"]
    Nce23 = 7,
    #[doc = "8: Observation bus bit 7"]
    Obsbus7 = 8,
    #[doc = "9: Output of the voltage comparator signal"]
    Vcmpo = 9,
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
    Scanout6 = 15,
}
impl From<Fncsel23> for u8 {
    #[inline(always)]
    fn from(variant: Fncsel23) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fncsel23 {
    type Ux = u8;
}
impl crate::IsEnum for Fncsel23 {}
#[doc = "Field `FNCSEL23` reader - Function select for GPIO pin 23"]
pub type Fncsel23R = crate::FieldReader<Fncsel23>;
impl Fncsel23R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fncsel23 {
        match self.bits {
            0 => Fncsel23::M7sdawir3,
            1 => Fncsel23::M7mosi,
            2 => Fncsel23::Swo,
            3 => Fncsel23::Gpio,
            4 => Fncsel23::Uart2rx,
            5 => Fncsel23::Uart3rx,
            6 => Fncsel23::Ct23,
            7 => Fncsel23::Nce23,
            8 => Fncsel23::Obsbus7,
            9 => Fncsel23::Vcmpo,
            10 => Fncsel23::Reserved10,
            11 => Fncsel23::Fpio,
            12 => Fncsel23::Reserved12,
            13 => Fncsel23::Reserved13,
            14 => Fncsel23::Reserved14,
            15 => Fncsel23::Scanout6,
            _ => unreachable!(),
        }
    }
    #[doc = "Serial I2C Master Data I/O (I2C Mode) Serial SPI Master Data I/O (SPI 3 wire mode) (IOM 7)"]
    #[inline(always)]
    pub fn is_m7sdawir3(&self) -> bool {
        *self == Fncsel23::M7sdawir3
    }
    #[doc = "Serial SPI Master MOSI output (IOM 7)"]
    #[inline(always)]
    pub fn is_m7mosi(&self) -> bool {
        *self == Fncsel23::M7mosi
    }
    #[doc = "Serial Wire Output"]
    #[inline(always)]
    pub fn is_swo(&self) -> bool {
        *self == Fncsel23::Swo
    }
    #[doc = "General purpose I/O"]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == Fncsel23::Gpio
    }
    #[doc = "UART receive input (UART 2)"]
    #[inline(always)]
    pub fn is_uart2rx(&self) -> bool {
        *self == Fncsel23::Uart2rx
    }
    #[doc = "UART receive input (UART 3)"]
    #[inline(always)]
    pub fn is_uart3rx(&self) -> bool {
        *self == Fncsel23::Uart3rx
    }
    #[doc = "Timer/Counter input or output; Selection of direction is done via CTIMER register settings."]
    #[inline(always)]
    pub fn is_ct23(&self) -> bool {
        *self == Fncsel23::Ct23
    }
    #[doc = "IOMSTR/MSPI N Chip Select. Polarity is determined by CE_POLARITY field"]
    #[inline(always)]
    pub fn is_nce23(&self) -> bool {
        *self == Fncsel23::Nce23
    }
    #[doc = "Observation bus bit 7"]
    #[inline(always)]
    pub fn is_obsbus7(&self) -> bool {
        *self == Fncsel23::Obsbus7
    }
    #[doc = "Output of the voltage comparator signal"]
    #[inline(always)]
    pub fn is_vcmpo(&self) -> bool {
        *self == Fncsel23::Vcmpo
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved10(&self) -> bool {
        *self == Fncsel23::Reserved10
    }
    #[doc = "Fast PIO"]
    #[inline(always)]
    pub fn is_fpio(&self) -> bool {
        *self == Fncsel23::Fpio
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved12(&self) -> bool {
        *self == Fncsel23::Reserved12
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved13(&self) -> bool {
        *self == Fncsel23::Reserved13
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved14(&self) -> bool {
        *self == Fncsel23::Reserved14
    }
    #[doc = "Internal function (SCAN)"]
    #[inline(always)]
    pub fn is_scanout6(&self) -> bool {
        *self == Fncsel23::Scanout6
    }
}
#[doc = "Field `FNCSEL23` writer - Function select for GPIO pin 23"]
pub type Fncsel23W<'a, REG> = crate::FieldWriter<'a, REG, 4, Fncsel23, crate::Safe>;
impl<'a, REG> Fncsel23W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Serial I2C Master Data I/O (I2C Mode) Serial SPI Master Data I/O (SPI 3 wire mode) (IOM 7)"]
    #[inline(always)]
    pub fn m7sdawir3(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel23::M7sdawir3)
    }
    #[doc = "Serial SPI Master MOSI output (IOM 7)"]
    #[inline(always)]
    pub fn m7mosi(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel23::M7mosi)
    }
    #[doc = "Serial Wire Output"]
    #[inline(always)]
    pub fn swo(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel23::Swo)
    }
    #[doc = "General purpose I/O"]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel23::Gpio)
    }
    #[doc = "UART receive input (UART 2)"]
    #[inline(always)]
    pub fn uart2rx(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel23::Uart2rx)
    }
    #[doc = "UART receive input (UART 3)"]
    #[inline(always)]
    pub fn uart3rx(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel23::Uart3rx)
    }
    #[doc = "Timer/Counter input or output; Selection of direction is done via CTIMER register settings."]
    #[inline(always)]
    pub fn ct23(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel23::Ct23)
    }
    #[doc = "IOMSTR/MSPI N Chip Select. Polarity is determined by CE_POLARITY field"]
    #[inline(always)]
    pub fn nce23(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel23::Nce23)
    }
    #[doc = "Observation bus bit 7"]
    #[inline(always)]
    pub fn obsbus7(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel23::Obsbus7)
    }
    #[doc = "Output of the voltage comparator signal"]
    #[inline(always)]
    pub fn vcmpo(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel23::Vcmpo)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved10(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel23::Reserved10)
    }
    #[doc = "Fast PIO"]
    #[inline(always)]
    pub fn fpio(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel23::Fpio)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved12(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel23::Reserved12)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved13(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel23::Reserved13)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved14(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel23::Reserved14)
    }
    #[doc = "Internal function (SCAN)"]
    #[inline(always)]
    pub fn scanout6(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel23::Scanout6)
    }
}
#[doc = "Field `INPEN23` reader - Input enable for GPIO 23"]
pub type Inpen23R = crate::BitReader;
#[doc = "Field `INPEN23` writer - Input enable for GPIO 23"]
pub type Inpen23W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDZERO23` reader - Return 0 for read data on GPIO 23"]
pub type Rdzero23R = crate::BitReader;
#[doc = "Field `RDZERO23` writer - Return 0 for read data on GPIO 23"]
pub type Rdzero23W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Interrupt enable for GPIO 23\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Irpten23 {
    #[doc = "0: Interrupts are disabled for this GPIO"]
    Dis = 0,
    #[doc = "1: Interrupts are enabled for falling edge transition on this GPIO"]
    Intfall = 1,
    #[doc = "2: Interrupts are enabled for rising edge transitions on this GPIO"]
    Intrise = 2,
    #[doc = "3: Interrupts are enabled for any edge transition on this GPIO"]
    Intany = 3,
}
impl From<Irpten23> for u8 {
    #[inline(always)]
    fn from(variant: Irpten23) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Irpten23 {
    type Ux = u8;
}
impl crate::IsEnum for Irpten23 {}
#[doc = "Field `IRPTEN23` reader - Interrupt enable for GPIO 23"]
pub type Irpten23R = crate::FieldReader<Irpten23>;
impl Irpten23R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irpten23 {
        match self.bits {
            0 => Irpten23::Dis,
            1 => Irpten23::Intfall,
            2 => Irpten23::Intrise,
            3 => Irpten23::Intany,
            _ => unreachable!(),
        }
    }
    #[doc = "Interrupts are disabled for this GPIO"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Irpten23::Dis
    }
    #[doc = "Interrupts are enabled for falling edge transition on this GPIO"]
    #[inline(always)]
    pub fn is_intfall(&self) -> bool {
        *self == Irpten23::Intfall
    }
    #[doc = "Interrupts are enabled for rising edge transitions on this GPIO"]
    #[inline(always)]
    pub fn is_intrise(&self) -> bool {
        *self == Irpten23::Intrise
    }
    #[doc = "Interrupts are enabled for any edge transition on this GPIO"]
    #[inline(always)]
    pub fn is_intany(&self) -> bool {
        *self == Irpten23::Intany
    }
}
#[doc = "Field `IRPTEN23` writer - Interrupt enable for GPIO 23"]
pub type Irpten23W<'a, REG> = crate::FieldWriter<'a, REG, 2, Irpten23, crate::Safe>;
impl<'a, REG> Irpten23W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Interrupts are disabled for this GPIO"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten23::Dis)
    }
    #[doc = "Interrupts are enabled for falling edge transition on this GPIO"]
    #[inline(always)]
    pub fn intfall(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten23::Intfall)
    }
    #[doc = "Interrupts are enabled for rising edge transitions on this GPIO"]
    #[inline(always)]
    pub fn intrise(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten23::Intrise)
    }
    #[doc = "Interrupts are enabled for any edge transition on this GPIO"]
    #[inline(always)]
    pub fn intany(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten23::Intany)
    }
}
#[doc = "Pin IO mode selection for GPIO pin 23\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Outcfg23 {
    #[doc = "0: Output Disabled"]
    Dis = 0,
    #[doc = "1: Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    Pushpull = 1,
    #[doc = "2: Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    Od = 2,
    #[doc = "3: Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    Ts = 3,
}
impl From<Outcfg23> for u8 {
    #[inline(always)]
    fn from(variant: Outcfg23) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Outcfg23 {
    type Ux = u8;
}
impl crate::IsEnum for Outcfg23 {}
#[doc = "Field `OUTCFG23` reader - Pin IO mode selection for GPIO pin 23"]
pub type Outcfg23R = crate::FieldReader<Outcfg23>;
impl Outcfg23R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Outcfg23 {
        match self.bits {
            0 => Outcfg23::Dis,
            1 => Outcfg23::Pushpull,
            2 => Outcfg23::Od,
            3 => Outcfg23::Ts,
            _ => unreachable!(),
        }
    }
    #[doc = "Output Disabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Outcfg23::Dis
    }
    #[doc = "Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == Outcfg23::Pushpull
    }
    #[doc = "Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == Outcfg23::Od
    }
    #[doc = "Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == Outcfg23::Ts
    }
}
#[doc = "Field `OUTCFG23` writer - Pin IO mode selection for GPIO pin 23"]
pub type Outcfg23W<'a, REG> = crate::FieldWriter<'a, REG, 2, Outcfg23, crate::Safe>;
impl<'a, REG> Outcfg23W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Output Disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg23::Dis)
    }
    #[doc = "Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg23::Pushpull)
    }
    #[doc = "Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    #[inline(always)]
    pub fn od(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg23::Od)
    }
    #[doc = "Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    #[inline(always)]
    pub fn ts(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg23::Ts)
    }
}
#[doc = "Drive strength selection for GPIO 23\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ds23 {
    #[doc = "0: 0.1x output driver selected"]
    _0p1x = 0,
    #[doc = "1: 0.5x output driver selected"]
    _0p5x = 1,
    #[doc = "2: 0.75x output driver selected"]
    _0p75x = 2,
    #[doc = "3: 1.0x output driver selected"]
    _1p0x = 3,
}
impl From<Ds23> for u8 {
    #[inline(always)]
    fn from(variant: Ds23) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ds23 {
    type Ux = u8;
}
impl crate::IsEnum for Ds23 {}
#[doc = "Field `DS23` reader - Drive strength selection for GPIO 23"]
pub type Ds23R = crate::FieldReader<Ds23>;
impl Ds23R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ds23 {
        match self.bits {
            0 => Ds23::_0p1x,
            1 => Ds23::_0p5x,
            2 => Ds23::_0p75x,
            3 => Ds23::_1p0x,
            _ => unreachable!(),
        }
    }
    #[doc = "0.1x output driver selected"]
    #[inline(always)]
    pub fn is_0p1x(&self) -> bool {
        *self == Ds23::_0p1x
    }
    #[doc = "0.5x output driver selected"]
    #[inline(always)]
    pub fn is_0p5x(&self) -> bool {
        *self == Ds23::_0p5x
    }
    #[doc = "0.75x output driver selected"]
    #[inline(always)]
    pub fn is_0p75x(&self) -> bool {
        *self == Ds23::_0p75x
    }
    #[doc = "1.0x output driver selected"]
    #[inline(always)]
    pub fn is_1p0x(&self) -> bool {
        *self == Ds23::_1p0x
    }
}
#[doc = "Field `DS23` writer - Drive strength selection for GPIO 23"]
pub type Ds23W<'a, REG> = crate::FieldWriter<'a, REG, 2, Ds23, crate::Safe>;
impl<'a, REG> Ds23W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "0.1x output driver selected"]
    #[inline(always)]
    pub fn _0p1x(self) -> &'a mut crate::W<REG> {
        self.variant(Ds23::_0p1x)
    }
    #[doc = "0.5x output driver selected"]
    #[inline(always)]
    pub fn _0p5x(self) -> &'a mut crate::W<REG> {
        self.variant(Ds23::_0p5x)
    }
    #[doc = "0.75x output driver selected"]
    #[inline(always)]
    pub fn _0p75x(self) -> &'a mut crate::W<REG> {
        self.variant(Ds23::_0p75x)
    }
    #[doc = "1.0x output driver selected"]
    #[inline(always)]
    pub fn _1p0x(self) -> &'a mut crate::W<REG> {
        self.variant(Ds23::_1p0x)
    }
}
#[doc = "Field `SR23` reader - Configure the slew rate"]
pub type Sr23R = crate::BitReader;
#[doc = "Field `SR23` writer - Configure the slew rate"]
pub type Sr23W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Pullup/Pulldown configuration for GPIO 23\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pullcfg23 {
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
impl From<Pullcfg23> for u8 {
    #[inline(always)]
    fn from(variant: Pullcfg23) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pullcfg23 {
    type Ux = u8;
}
impl crate::IsEnum for Pullcfg23 {}
#[doc = "Field `PULLCFG23` reader - Pullup/Pulldown configuration for GPIO 23"]
pub type Pullcfg23R = crate::FieldReader<Pullcfg23>;
impl Pullcfg23R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pullcfg23 {
        match self.bits {
            0 => Pullcfg23::Dis,
            1 => Pullcfg23::Pd50k,
            2 => Pullcfg23::Pu15k,
            3 => Pullcfg23::Pu6k,
            4 => Pullcfg23::Pu12k,
            5 => Pullcfg23::Pu24k,
            6 => Pullcfg23::Pu50k,
            7 => Pullcfg23::Pu100k,
            _ => unreachable!(),
        }
    }
    #[doc = "No pullup or pulldown selected"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Pullcfg23::Dis
    }
    #[doc = "50K Pulldown selected"]
    #[inline(always)]
    pub fn is_pd50k(&self) -> bool {
        *self == Pullcfg23::Pd50k
    }
    #[doc = "1.5K Pullup selected"]
    #[inline(always)]
    pub fn is_pu15k(&self) -> bool {
        *self == Pullcfg23::Pu15k
    }
    #[doc = "6K Pullup selected"]
    #[inline(always)]
    pub fn is_pu6k(&self) -> bool {
        *self == Pullcfg23::Pu6k
    }
    #[doc = "12K Pullup selected"]
    #[inline(always)]
    pub fn is_pu12k(&self) -> bool {
        *self == Pullcfg23::Pu12k
    }
    #[doc = "24K Pullup selected"]
    #[inline(always)]
    pub fn is_pu24k(&self) -> bool {
        *self == Pullcfg23::Pu24k
    }
    #[doc = "50K Pullup selected"]
    #[inline(always)]
    pub fn is_pu50k(&self) -> bool {
        *self == Pullcfg23::Pu50k
    }
    #[doc = "100K Pullup selected"]
    #[inline(always)]
    pub fn is_pu100k(&self) -> bool {
        *self == Pullcfg23::Pu100k
    }
}
#[doc = "Field `PULLCFG23` writer - Pullup/Pulldown configuration for GPIO 23"]
pub type Pullcfg23W<'a, REG> = crate::FieldWriter<'a, REG, 3, Pullcfg23, crate::Safe>;
impl<'a, REG> Pullcfg23W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No pullup or pulldown selected"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg23::Dis)
    }
    #[doc = "50K Pulldown selected"]
    #[inline(always)]
    pub fn pd50k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg23::Pd50k)
    }
    #[doc = "1.5K Pullup selected"]
    #[inline(always)]
    pub fn pu15k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg23::Pu15k)
    }
    #[doc = "6K Pullup selected"]
    #[inline(always)]
    pub fn pu6k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg23::Pu6k)
    }
    #[doc = "12K Pullup selected"]
    #[inline(always)]
    pub fn pu12k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg23::Pu12k)
    }
    #[doc = "24K Pullup selected"]
    #[inline(always)]
    pub fn pu24k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg23::Pu24k)
    }
    #[doc = "50K Pullup selected"]
    #[inline(always)]
    pub fn pu50k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg23::Pu50k)
    }
    #[doc = "100K Pullup selected"]
    #[inline(always)]
    pub fn pu100k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg23::Pu100k)
    }
}
#[doc = "IOMSTR/MSPI N Chip Select 23, DISP control signals DE, CSX, and CS. Polarity is determined by CE_POLARITY field\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ncesrc23 {
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
impl From<Ncesrc23> for u8 {
    #[inline(always)]
    fn from(variant: Ncesrc23) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ncesrc23 {
    type Ux = u8;
}
impl crate::IsEnum for Ncesrc23 {}
#[doc = "Field `NCESRC23` reader - IOMSTR/MSPI N Chip Select 23, DISP control signals DE, CSX, and CS. Polarity is determined by CE_POLARITY field"]
pub type Ncesrc23R = crate::FieldReader<Ncesrc23>;
impl Ncesrc23R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ncesrc23> {
        match self.bits {
            0 => Some(Ncesrc23::Iom0ce0),
            1 => Some(Ncesrc23::Iom0ce1),
            2 => Some(Ncesrc23::Iom0ce2),
            3 => Some(Ncesrc23::Iom0ce3),
            4 => Some(Ncesrc23::Iom1ce0),
            5 => Some(Ncesrc23::Iom1ce1),
            6 => Some(Ncesrc23::Iom1ce2),
            7 => Some(Ncesrc23::Iom1ce3),
            8 => Some(Ncesrc23::Iom2ce0),
            9 => Some(Ncesrc23::Iom2ce1),
            10 => Some(Ncesrc23::Iom2ce2),
            11 => Some(Ncesrc23::Iom2ce3),
            12 => Some(Ncesrc23::Iom3ce0),
            13 => Some(Ncesrc23::Iom3ce1),
            14 => Some(Ncesrc23::Iom3ce2),
            15 => Some(Ncesrc23::Iom3ce3),
            16 => Some(Ncesrc23::Iom4ce0),
            17 => Some(Ncesrc23::Iom4ce1),
            18 => Some(Ncesrc23::Iom4ce2),
            19 => Some(Ncesrc23::Iom4ce3),
            20 => Some(Ncesrc23::Iom5ce0),
            21 => Some(Ncesrc23::Iom5ce1),
            22 => Some(Ncesrc23::Iom5ce2),
            23 => Some(Ncesrc23::Iom5ce3),
            24 => Some(Ncesrc23::Iom6ce0),
            25 => Some(Ncesrc23::Iom6ce1),
            26 => Some(Ncesrc23::Iom6ce2),
            27 => Some(Ncesrc23::Iom6ce3),
            28 => Some(Ncesrc23::Iom7ce0),
            29 => Some(Ncesrc23::Iom7ce1),
            30 => Some(Ncesrc23::Iom7ce2),
            31 => Some(Ncesrc23::Iom7ce3),
            32 => Some(Ncesrc23::Mspi0cen0),
            33 => Some(Ncesrc23::Mspi0cen1),
            34 => Some(Ncesrc23::Mspi1cen0),
            35 => Some(Ncesrc23::Mspi1cen1),
            36 => Some(Ncesrc23::Mspi2cen0),
            37 => Some(Ncesrc23::Mspi2cen1),
            38 => Some(Ncesrc23::DcDpiDe),
            39 => Some(Ncesrc23::DispContCsx),
            40 => Some(Ncesrc23::DcSpiCsN),
            41 => Some(Ncesrc23::DcQspiCsN),
            42 => Some(Ncesrc23::DcResx),
            _ => None,
        }
    }
    #[doc = "IOM 0 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom0ce0(&self) -> bool {
        *self == Ncesrc23::Iom0ce0
    }
    #[doc = "IOM 0 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom0ce1(&self) -> bool {
        *self == Ncesrc23::Iom0ce1
    }
    #[doc = "IOM 0 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom0ce2(&self) -> bool {
        *self == Ncesrc23::Iom0ce2
    }
    #[doc = "IOM 0 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom0ce3(&self) -> bool {
        *self == Ncesrc23::Iom0ce3
    }
    #[doc = "IOM 1 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom1ce0(&self) -> bool {
        *self == Ncesrc23::Iom1ce0
    }
    #[doc = "IOM 1 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom1ce1(&self) -> bool {
        *self == Ncesrc23::Iom1ce1
    }
    #[doc = "IOM 1 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom1ce2(&self) -> bool {
        *self == Ncesrc23::Iom1ce2
    }
    #[doc = "IOM 1 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom1ce3(&self) -> bool {
        *self == Ncesrc23::Iom1ce3
    }
    #[doc = "IOM 2 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom2ce0(&self) -> bool {
        *self == Ncesrc23::Iom2ce0
    }
    #[doc = "IOM 2 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom2ce1(&self) -> bool {
        *self == Ncesrc23::Iom2ce1
    }
    #[doc = "IOM 2 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom2ce2(&self) -> bool {
        *self == Ncesrc23::Iom2ce2
    }
    #[doc = "IOM 2 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom2ce3(&self) -> bool {
        *self == Ncesrc23::Iom2ce3
    }
    #[doc = "IOM 3 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom3ce0(&self) -> bool {
        *self == Ncesrc23::Iom3ce0
    }
    #[doc = "IOM 3 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom3ce1(&self) -> bool {
        *self == Ncesrc23::Iom3ce1
    }
    #[doc = "IOM 3 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom3ce2(&self) -> bool {
        *self == Ncesrc23::Iom3ce2
    }
    #[doc = "IOM 3 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom3ce3(&self) -> bool {
        *self == Ncesrc23::Iom3ce3
    }
    #[doc = "IOM 4 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom4ce0(&self) -> bool {
        *self == Ncesrc23::Iom4ce0
    }
    #[doc = "IOM 4 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom4ce1(&self) -> bool {
        *self == Ncesrc23::Iom4ce1
    }
    #[doc = "IOM 4 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom4ce2(&self) -> bool {
        *self == Ncesrc23::Iom4ce2
    }
    #[doc = "IOM 4 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom4ce3(&self) -> bool {
        *self == Ncesrc23::Iom4ce3
    }
    #[doc = "IOM 5 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom5ce0(&self) -> bool {
        *self == Ncesrc23::Iom5ce0
    }
    #[doc = "IOM 5 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom5ce1(&self) -> bool {
        *self == Ncesrc23::Iom5ce1
    }
    #[doc = "IOM 5 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom5ce2(&self) -> bool {
        *self == Ncesrc23::Iom5ce2
    }
    #[doc = "IOM 5 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom5ce3(&self) -> bool {
        *self == Ncesrc23::Iom5ce3
    }
    #[doc = "IOM 6 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom6ce0(&self) -> bool {
        *self == Ncesrc23::Iom6ce0
    }
    #[doc = "IOM 6 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom6ce1(&self) -> bool {
        *self == Ncesrc23::Iom6ce1
    }
    #[doc = "IOM 6 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom6ce2(&self) -> bool {
        *self == Ncesrc23::Iom6ce2
    }
    #[doc = "IOM 6 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom6ce3(&self) -> bool {
        *self == Ncesrc23::Iom6ce3
    }
    #[doc = "IOM 7 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom7ce0(&self) -> bool {
        *self == Ncesrc23::Iom7ce0
    }
    #[doc = "IOM 7 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom7ce1(&self) -> bool {
        *self == Ncesrc23::Iom7ce1
    }
    #[doc = "IOM 7 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom7ce2(&self) -> bool {
        *self == Ncesrc23::Iom7ce2
    }
    #[doc = "IOM 7 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom7ce3(&self) -> bool {
        *self == Ncesrc23::Iom7ce3
    }
    #[doc = "MSPI 0 NCE 0 module"]
    #[inline(always)]
    pub fn is_mspi0cen0(&self) -> bool {
        *self == Ncesrc23::Mspi0cen0
    }
    #[doc = "MSPI 0 NCE 1 module"]
    #[inline(always)]
    pub fn is_mspi0cen1(&self) -> bool {
        *self == Ncesrc23::Mspi0cen1
    }
    #[doc = "MSPI 1 NCE 0 module"]
    #[inline(always)]
    pub fn is_mspi1cen0(&self) -> bool {
        *self == Ncesrc23::Mspi1cen0
    }
    #[doc = "MSPI 1 NCE 1 module"]
    #[inline(always)]
    pub fn is_mspi1cen1(&self) -> bool {
        *self == Ncesrc23::Mspi1cen1
    }
    #[doc = "MSPI 2 NCE 0 module"]
    #[inline(always)]
    pub fn is_mspi2cen0(&self) -> bool {
        *self == Ncesrc23::Mspi2cen0
    }
    #[doc = "MSPI 2 NCE 1 module"]
    #[inline(always)]
    pub fn is_mspi2cen1(&self) -> bool {
        *self == Ncesrc23::Mspi2cen1
    }
    #[doc = "DC DPI DE module"]
    #[inline(always)]
    pub fn is_dc_dpi_de(&self) -> bool {
        *self == Ncesrc23::DcDpiDe
    }
    #[doc = "DISP CONT CSX module"]
    #[inline(always)]
    pub fn is_disp_cont_csx(&self) -> bool {
        *self == Ncesrc23::DispContCsx
    }
    #[doc = "DC SPI CS_N module"]
    #[inline(always)]
    pub fn is_dc_spi_cs_n(&self) -> bool {
        *self == Ncesrc23::DcSpiCsN
    }
    #[doc = "DC QSPI CS_N module"]
    #[inline(always)]
    pub fn is_dc_qspi_cs_n(&self) -> bool {
        *self == Ncesrc23::DcQspiCsN
    }
    #[doc = "DC module RESX"]
    #[inline(always)]
    pub fn is_dc_resx(&self) -> bool {
        *self == Ncesrc23::DcResx
    }
}
#[doc = "Field `NCESRC23` writer - IOMSTR/MSPI N Chip Select 23, DISP control signals DE, CSX, and CS. Polarity is determined by CE_POLARITY field"]
pub type Ncesrc23W<'a, REG> = crate::FieldWriter<'a, REG, 6, Ncesrc23>;
impl<'a, REG> Ncesrc23W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "IOM 0 NCE 0 module"]
    #[inline(always)]
    pub fn iom0ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc23::Iom0ce0)
    }
    #[doc = "IOM 0 NCE 1 module"]
    #[inline(always)]
    pub fn iom0ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc23::Iom0ce1)
    }
    #[doc = "IOM 0 NCE 2 module"]
    #[inline(always)]
    pub fn iom0ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc23::Iom0ce2)
    }
    #[doc = "IOM 0 NCE 3 module"]
    #[inline(always)]
    pub fn iom0ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc23::Iom0ce3)
    }
    #[doc = "IOM 1 NCE 0 module"]
    #[inline(always)]
    pub fn iom1ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc23::Iom1ce0)
    }
    #[doc = "IOM 1 NCE 1 module"]
    #[inline(always)]
    pub fn iom1ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc23::Iom1ce1)
    }
    #[doc = "IOM 1 NCE 2 module"]
    #[inline(always)]
    pub fn iom1ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc23::Iom1ce2)
    }
    #[doc = "IOM 1 NCE 3 module"]
    #[inline(always)]
    pub fn iom1ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc23::Iom1ce3)
    }
    #[doc = "IOM 2 NCE 0 module"]
    #[inline(always)]
    pub fn iom2ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc23::Iom2ce0)
    }
    #[doc = "IOM 2 NCE 1 module"]
    #[inline(always)]
    pub fn iom2ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc23::Iom2ce1)
    }
    #[doc = "IOM 2 NCE 2 module"]
    #[inline(always)]
    pub fn iom2ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc23::Iom2ce2)
    }
    #[doc = "IOM 2 NCE 3 module"]
    #[inline(always)]
    pub fn iom2ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc23::Iom2ce3)
    }
    #[doc = "IOM 3 NCE 0 module"]
    #[inline(always)]
    pub fn iom3ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc23::Iom3ce0)
    }
    #[doc = "IOM 3 NCE 1 module"]
    #[inline(always)]
    pub fn iom3ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc23::Iom3ce1)
    }
    #[doc = "IOM 3 NCE 2 module"]
    #[inline(always)]
    pub fn iom3ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc23::Iom3ce2)
    }
    #[doc = "IOM 3 NCE 3 module"]
    #[inline(always)]
    pub fn iom3ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc23::Iom3ce3)
    }
    #[doc = "IOM 4 NCE 0 module"]
    #[inline(always)]
    pub fn iom4ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc23::Iom4ce0)
    }
    #[doc = "IOM 4 NCE 1 module"]
    #[inline(always)]
    pub fn iom4ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc23::Iom4ce1)
    }
    #[doc = "IOM 4 NCE 2 module"]
    #[inline(always)]
    pub fn iom4ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc23::Iom4ce2)
    }
    #[doc = "IOM 4 NCE 3 module"]
    #[inline(always)]
    pub fn iom4ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc23::Iom4ce3)
    }
    #[doc = "IOM 5 NCE 0 module"]
    #[inline(always)]
    pub fn iom5ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc23::Iom5ce0)
    }
    #[doc = "IOM 5 NCE 1 module"]
    #[inline(always)]
    pub fn iom5ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc23::Iom5ce1)
    }
    #[doc = "IOM 5 NCE 2 module"]
    #[inline(always)]
    pub fn iom5ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc23::Iom5ce2)
    }
    #[doc = "IOM 5 NCE 3 module"]
    #[inline(always)]
    pub fn iom5ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc23::Iom5ce3)
    }
    #[doc = "IOM 6 NCE 0 module"]
    #[inline(always)]
    pub fn iom6ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc23::Iom6ce0)
    }
    #[doc = "IOM 6 NCE 1 module"]
    #[inline(always)]
    pub fn iom6ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc23::Iom6ce1)
    }
    #[doc = "IOM 6 NCE 2 module"]
    #[inline(always)]
    pub fn iom6ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc23::Iom6ce2)
    }
    #[doc = "IOM 6 NCE 3 module"]
    #[inline(always)]
    pub fn iom6ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc23::Iom6ce3)
    }
    #[doc = "IOM 7 NCE 0 module"]
    #[inline(always)]
    pub fn iom7ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc23::Iom7ce0)
    }
    #[doc = "IOM 7 NCE 1 module"]
    #[inline(always)]
    pub fn iom7ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc23::Iom7ce1)
    }
    #[doc = "IOM 7 NCE 2 module"]
    #[inline(always)]
    pub fn iom7ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc23::Iom7ce2)
    }
    #[doc = "IOM 7 NCE 3 module"]
    #[inline(always)]
    pub fn iom7ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc23::Iom7ce3)
    }
    #[doc = "MSPI 0 NCE 0 module"]
    #[inline(always)]
    pub fn mspi0cen0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc23::Mspi0cen0)
    }
    #[doc = "MSPI 0 NCE 1 module"]
    #[inline(always)]
    pub fn mspi0cen1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc23::Mspi0cen1)
    }
    #[doc = "MSPI 1 NCE 0 module"]
    #[inline(always)]
    pub fn mspi1cen0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc23::Mspi1cen0)
    }
    #[doc = "MSPI 1 NCE 1 module"]
    #[inline(always)]
    pub fn mspi1cen1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc23::Mspi1cen1)
    }
    #[doc = "MSPI 2 NCE 0 module"]
    #[inline(always)]
    pub fn mspi2cen0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc23::Mspi2cen0)
    }
    #[doc = "MSPI 2 NCE 1 module"]
    #[inline(always)]
    pub fn mspi2cen1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc23::Mspi2cen1)
    }
    #[doc = "DC DPI DE module"]
    #[inline(always)]
    pub fn dc_dpi_de(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc23::DcDpiDe)
    }
    #[doc = "DISP CONT CSX module"]
    #[inline(always)]
    pub fn disp_cont_csx(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc23::DispContCsx)
    }
    #[doc = "DC SPI CS_N module"]
    #[inline(always)]
    pub fn dc_spi_cs_n(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc23::DcSpiCsN)
    }
    #[doc = "DC QSPI CS_N module"]
    #[inline(always)]
    pub fn dc_qspi_cs_n(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc23::DcQspiCsN)
    }
    #[doc = "DC module RESX"]
    #[inline(always)]
    pub fn dc_resx(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc23::DcResx)
    }
}
#[doc = "Polarity select for NCE for GPIO 23\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ncepol23 {
    #[doc = "0: Polarity is active low"]
    Low = 0,
    #[doc = "1: Polarity is active high"]
    High = 1,
}
impl From<Ncepol23> for bool {
    #[inline(always)]
    fn from(variant: Ncepol23) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NCEPOL23` reader - Polarity select for NCE for GPIO 23"]
pub type Ncepol23R = crate::BitReader<Ncepol23>;
impl Ncepol23R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ncepol23 {
        match self.bits {
            false => Ncepol23::Low,
            true => Ncepol23::High,
        }
    }
    #[doc = "Polarity is active low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Ncepol23::Low
    }
    #[doc = "Polarity is active high"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Ncepol23::High
    }
}
#[doc = "Field `NCEPOL23` writer - Polarity select for NCE for GPIO 23"]
pub type Ncepol23W<'a, REG> = crate::BitWriter<'a, REG, Ncepol23>;
impl<'a, REG> Ncepol23W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Polarity is active low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Ncepol23::Low)
    }
    #[doc = "Polarity is active high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Ncepol23::High)
    }
}
#[doc = "Field `FIEN23` reader - Force input enable active regardless of function selected. Otherwise the selected function will enable the input only when needed"]
pub type Fien23R = crate::BitReader;
#[doc = "Field `FIEN23` writer - Force input enable active regardless of function selected. Otherwise the selected function will enable the input only when needed"]
pub type Fien23W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FOEN23` reader - Force output enable active regardless of function selected. Otherwise the selected function will enable the output only when needed"]
pub type Foen23R = crate::BitReader;
#[doc = "Field `FOEN23` writer - Force output enable active regardless of function selected. Otherwise the selected function will enable the output only when needed"]
pub type Foen23W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Function select for GPIO pin 23"]
    #[inline(always)]
    pub fn fncsel23(&self) -> Fncsel23R {
        Fncsel23R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Input enable for GPIO 23"]
    #[inline(always)]
    pub fn inpen23(&self) -> Inpen23R {
        Inpen23R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Return 0 for read data on GPIO 23"]
    #[inline(always)]
    pub fn rdzero23(&self) -> Rdzero23R {
        Rdzero23R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Interrupt enable for GPIO 23"]
    #[inline(always)]
    pub fn irpten23(&self) -> Irpten23R {
        Irpten23R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Pin IO mode selection for GPIO pin 23"]
    #[inline(always)]
    pub fn outcfg23(&self) -> Outcfg23R {
        Outcfg23R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Drive strength selection for GPIO 23"]
    #[inline(always)]
    pub fn ds23(&self) -> Ds23R {
        Ds23R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - Configure the slew rate"]
    #[inline(always)]
    pub fn sr23(&self) -> Sr23R {
        Sr23R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - Pullup/Pulldown configuration for GPIO 23"]
    #[inline(always)]
    pub fn pullcfg23(&self) -> Pullcfg23R {
        Pullcfg23R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:21 - IOMSTR/MSPI N Chip Select 23, DISP control signals DE, CSX, and CS. Polarity is determined by CE_POLARITY field"]
    #[inline(always)]
    pub fn ncesrc23(&self) -> Ncesrc23R {
        Ncesrc23R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 22 - Polarity select for NCE for GPIO 23"]
    #[inline(always)]
    pub fn ncepol23(&self) -> Ncepol23R {
        Ncepol23R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 26 - Force input enable active regardless of function selected. Otherwise the selected function will enable the input only when needed"]
    #[inline(always)]
    pub fn fien23(&self) -> Fien23R {
        Fien23R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Force output enable active regardless of function selected. Otherwise the selected function will enable the output only when needed"]
    #[inline(always)]
    pub fn foen23(&self) -> Foen23R {
        Foen23R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Function select for GPIO pin 23"]
    #[inline(always)]
    #[must_use]
    pub fn fncsel23(&mut self) -> Fncsel23W<Pincfg23Spec> {
        Fncsel23W::new(self, 0)
    }
    #[doc = "Bit 4 - Input enable for GPIO 23"]
    #[inline(always)]
    #[must_use]
    pub fn inpen23(&mut self) -> Inpen23W<Pincfg23Spec> {
        Inpen23W::new(self, 4)
    }
    #[doc = "Bit 5 - Return 0 for read data on GPIO 23"]
    #[inline(always)]
    #[must_use]
    pub fn rdzero23(&mut self) -> Rdzero23W<Pincfg23Spec> {
        Rdzero23W::new(self, 5)
    }
    #[doc = "Bits 6:7 - Interrupt enable for GPIO 23"]
    #[inline(always)]
    #[must_use]
    pub fn irpten23(&mut self) -> Irpten23W<Pincfg23Spec> {
        Irpten23W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Pin IO mode selection for GPIO pin 23"]
    #[inline(always)]
    #[must_use]
    pub fn outcfg23(&mut self) -> Outcfg23W<Pincfg23Spec> {
        Outcfg23W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Drive strength selection for GPIO 23"]
    #[inline(always)]
    #[must_use]
    pub fn ds23(&mut self) -> Ds23W<Pincfg23Spec> {
        Ds23W::new(self, 10)
    }
    #[doc = "Bit 12 - Configure the slew rate"]
    #[inline(always)]
    #[must_use]
    pub fn sr23(&mut self) -> Sr23W<Pincfg23Spec> {
        Sr23W::new(self, 12)
    }
    #[doc = "Bits 13:15 - Pullup/Pulldown configuration for GPIO 23"]
    #[inline(always)]
    #[must_use]
    pub fn pullcfg23(&mut self) -> Pullcfg23W<Pincfg23Spec> {
        Pullcfg23W::new(self, 13)
    }
    #[doc = "Bits 16:21 - IOMSTR/MSPI N Chip Select 23, DISP control signals DE, CSX, and CS. Polarity is determined by CE_POLARITY field"]
    #[inline(always)]
    #[must_use]
    pub fn ncesrc23(&mut self) -> Ncesrc23W<Pincfg23Spec> {
        Ncesrc23W::new(self, 16)
    }
    #[doc = "Bit 22 - Polarity select for NCE for GPIO 23"]
    #[inline(always)]
    #[must_use]
    pub fn ncepol23(&mut self) -> Ncepol23W<Pincfg23Spec> {
        Ncepol23W::new(self, 22)
    }
    #[doc = "Bit 26 - Force input enable active regardless of function selected. Otherwise the selected function will enable the input only when needed"]
    #[inline(always)]
    #[must_use]
    pub fn fien23(&mut self) -> Fien23W<Pincfg23Spec> {
        Fien23W::new(self, 26)
    }
    #[doc = "Bit 27 - Force output enable active regardless of function selected. Otherwise the selected function will enable the output only when needed"]
    #[inline(always)]
    #[must_use]
    pub fn foen23(&mut self) -> Foen23W<Pincfg23Spec> {
        Foen23W::new(self, 27)
    }
}
#[doc = "Controls the operation of GPIO pin 23.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg23::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg23::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pincfg23Spec;
impl crate::RegisterSpec for Pincfg23Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pincfg23::R`](R) reader structure"]
impl crate::Readable for Pincfg23Spec {}
#[doc = "`write(|w| ..)` method takes [`pincfg23::W`](W) writer structure"]
impl crate::Writable for Pincfg23Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PINCFG23 to value 0x03"]
impl crate::Resettable for Pincfg23Spec {
    const RESET_VALUE: u32 = 0x03;
}
