#[doc = "Register `PINCFG22` reader"]
pub type R = crate::R<Pincfg22Spec>;
#[doc = "Register `PINCFG22` writer"]
pub type W = crate::W<Pincfg22Spec>;
#[doc = "Function select for GPIO pin 22\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fncsel22 {
    #[doc = "0: Serial I2C Master Clock output (IOM 7)"]
    M7scl = 0,
    #[doc = "1: Serial SPI Master Clock output (IOM 7)"]
    M7sck = 1,
    #[doc = "2: Serial Wire Output"]
    Swo = 2,
    #[doc = "3: General purpose I/O"]
    Gpio = 3,
    #[doc = "4: UART receive input (UART 0)"]
    Uart0rx = 4,
    #[doc = "5: UART receive input (UART 1)"]
    Uart1rx = 5,
    #[doc = "6: Timer/Counter input or output; Selection of direction is done via CTIMER register settings."]
    Ct22 = 6,
    #[doc = "7: IOMSTR/MSPI N Chip Select. Polarity is determined by CE_POLARITY field"]
    Nce22 = 7,
    #[doc = "8: Observation bus bit 6"]
    Obsbus6 = 8,
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
    Scanin3 = 15,
}
impl From<Fncsel22> for u8 {
    #[inline(always)]
    fn from(variant: Fncsel22) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fncsel22 {
    type Ux = u8;
}
impl crate::IsEnum for Fncsel22 {}
#[doc = "Field `FNCSEL22` reader - Function select for GPIO pin 22"]
pub type Fncsel22R = crate::FieldReader<Fncsel22>;
impl Fncsel22R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fncsel22 {
        match self.bits {
            0 => Fncsel22::M7scl,
            1 => Fncsel22::M7sck,
            2 => Fncsel22::Swo,
            3 => Fncsel22::Gpio,
            4 => Fncsel22::Uart0rx,
            5 => Fncsel22::Uart1rx,
            6 => Fncsel22::Ct22,
            7 => Fncsel22::Nce22,
            8 => Fncsel22::Obsbus6,
            9 => Fncsel22::Vcmpo,
            10 => Fncsel22::Reserved10,
            11 => Fncsel22::Fpio,
            12 => Fncsel22::Reserved12,
            13 => Fncsel22::Reserved13,
            14 => Fncsel22::Reserved14,
            15 => Fncsel22::Scanin3,
            _ => unreachable!(),
        }
    }
    #[doc = "Serial I2C Master Clock output (IOM 7)"]
    #[inline(always)]
    pub fn is_m7scl(&self) -> bool {
        *self == Fncsel22::M7scl
    }
    #[doc = "Serial SPI Master Clock output (IOM 7)"]
    #[inline(always)]
    pub fn is_m7sck(&self) -> bool {
        *self == Fncsel22::M7sck
    }
    #[doc = "Serial Wire Output"]
    #[inline(always)]
    pub fn is_swo(&self) -> bool {
        *self == Fncsel22::Swo
    }
    #[doc = "General purpose I/O"]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == Fncsel22::Gpio
    }
    #[doc = "UART receive input (UART 0)"]
    #[inline(always)]
    pub fn is_uart0rx(&self) -> bool {
        *self == Fncsel22::Uart0rx
    }
    #[doc = "UART receive input (UART 1)"]
    #[inline(always)]
    pub fn is_uart1rx(&self) -> bool {
        *self == Fncsel22::Uart1rx
    }
    #[doc = "Timer/Counter input or output; Selection of direction is done via CTIMER register settings."]
    #[inline(always)]
    pub fn is_ct22(&self) -> bool {
        *self == Fncsel22::Ct22
    }
    #[doc = "IOMSTR/MSPI N Chip Select. Polarity is determined by CE_POLARITY field"]
    #[inline(always)]
    pub fn is_nce22(&self) -> bool {
        *self == Fncsel22::Nce22
    }
    #[doc = "Observation bus bit 6"]
    #[inline(always)]
    pub fn is_obsbus6(&self) -> bool {
        *self == Fncsel22::Obsbus6
    }
    #[doc = "Output of the voltage comparator signal"]
    #[inline(always)]
    pub fn is_vcmpo(&self) -> bool {
        *self == Fncsel22::Vcmpo
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved10(&self) -> bool {
        *self == Fncsel22::Reserved10
    }
    #[doc = "Fast PIO"]
    #[inline(always)]
    pub fn is_fpio(&self) -> bool {
        *self == Fncsel22::Fpio
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved12(&self) -> bool {
        *self == Fncsel22::Reserved12
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved13(&self) -> bool {
        *self == Fncsel22::Reserved13
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved14(&self) -> bool {
        *self == Fncsel22::Reserved14
    }
    #[doc = "Internal function (SCAN)"]
    #[inline(always)]
    pub fn is_scanin3(&self) -> bool {
        *self == Fncsel22::Scanin3
    }
}
#[doc = "Field `FNCSEL22` writer - Function select for GPIO pin 22"]
pub type Fncsel22W<'a, REG> = crate::FieldWriter<'a, REG, 4, Fncsel22, crate::Safe>;
impl<'a, REG> Fncsel22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Serial I2C Master Clock output (IOM 7)"]
    #[inline(always)]
    pub fn m7scl(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel22::M7scl)
    }
    #[doc = "Serial SPI Master Clock output (IOM 7)"]
    #[inline(always)]
    pub fn m7sck(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel22::M7sck)
    }
    #[doc = "Serial Wire Output"]
    #[inline(always)]
    pub fn swo(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel22::Swo)
    }
    #[doc = "General purpose I/O"]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel22::Gpio)
    }
    #[doc = "UART receive input (UART 0)"]
    #[inline(always)]
    pub fn uart0rx(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel22::Uart0rx)
    }
    #[doc = "UART receive input (UART 1)"]
    #[inline(always)]
    pub fn uart1rx(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel22::Uart1rx)
    }
    #[doc = "Timer/Counter input or output; Selection of direction is done via CTIMER register settings."]
    #[inline(always)]
    pub fn ct22(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel22::Ct22)
    }
    #[doc = "IOMSTR/MSPI N Chip Select. Polarity is determined by CE_POLARITY field"]
    #[inline(always)]
    pub fn nce22(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel22::Nce22)
    }
    #[doc = "Observation bus bit 6"]
    #[inline(always)]
    pub fn obsbus6(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel22::Obsbus6)
    }
    #[doc = "Output of the voltage comparator signal"]
    #[inline(always)]
    pub fn vcmpo(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel22::Vcmpo)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved10(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel22::Reserved10)
    }
    #[doc = "Fast PIO"]
    #[inline(always)]
    pub fn fpio(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel22::Fpio)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved12(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel22::Reserved12)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved13(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel22::Reserved13)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved14(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel22::Reserved14)
    }
    #[doc = "Internal function (SCAN)"]
    #[inline(always)]
    pub fn scanin3(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel22::Scanin3)
    }
}
#[doc = "Field `INPEN22` reader - Input enable for GPIO 22"]
pub type Inpen22R = crate::BitReader;
#[doc = "Field `INPEN22` writer - Input enable for GPIO 22"]
pub type Inpen22W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDZERO22` reader - Return 0 for read data on GPIO 22"]
pub type Rdzero22R = crate::BitReader;
#[doc = "Field `RDZERO22` writer - Return 0 for read data on GPIO 22"]
pub type Rdzero22W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Interrupt enable for GPIO 22\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Irpten22 {
    #[doc = "0: Interrupts are disabled for this GPIO"]
    Dis = 0,
    #[doc = "1: Interrupts are enabled for falling edge transition on this GPIO"]
    Intfall = 1,
    #[doc = "2: Interrupts are enabled for rising edge transitions on this GPIO"]
    Intrise = 2,
    #[doc = "3: Interrupts are enabled for any edge transition on this GPIO"]
    Intany = 3,
}
impl From<Irpten22> for u8 {
    #[inline(always)]
    fn from(variant: Irpten22) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Irpten22 {
    type Ux = u8;
}
impl crate::IsEnum for Irpten22 {}
#[doc = "Field `IRPTEN22` reader - Interrupt enable for GPIO 22"]
pub type Irpten22R = crate::FieldReader<Irpten22>;
impl Irpten22R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irpten22 {
        match self.bits {
            0 => Irpten22::Dis,
            1 => Irpten22::Intfall,
            2 => Irpten22::Intrise,
            3 => Irpten22::Intany,
            _ => unreachable!(),
        }
    }
    #[doc = "Interrupts are disabled for this GPIO"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Irpten22::Dis
    }
    #[doc = "Interrupts are enabled for falling edge transition on this GPIO"]
    #[inline(always)]
    pub fn is_intfall(&self) -> bool {
        *self == Irpten22::Intfall
    }
    #[doc = "Interrupts are enabled for rising edge transitions on this GPIO"]
    #[inline(always)]
    pub fn is_intrise(&self) -> bool {
        *self == Irpten22::Intrise
    }
    #[doc = "Interrupts are enabled for any edge transition on this GPIO"]
    #[inline(always)]
    pub fn is_intany(&self) -> bool {
        *self == Irpten22::Intany
    }
}
#[doc = "Field `IRPTEN22` writer - Interrupt enable for GPIO 22"]
pub type Irpten22W<'a, REG> = crate::FieldWriter<'a, REG, 2, Irpten22, crate::Safe>;
impl<'a, REG> Irpten22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Interrupts are disabled for this GPIO"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten22::Dis)
    }
    #[doc = "Interrupts are enabled for falling edge transition on this GPIO"]
    #[inline(always)]
    pub fn intfall(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten22::Intfall)
    }
    #[doc = "Interrupts are enabled for rising edge transitions on this GPIO"]
    #[inline(always)]
    pub fn intrise(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten22::Intrise)
    }
    #[doc = "Interrupts are enabled for any edge transition on this GPIO"]
    #[inline(always)]
    pub fn intany(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten22::Intany)
    }
}
#[doc = "Pin IO mode selection for GPIO pin 22\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Outcfg22 {
    #[doc = "0: Output Disabled"]
    Dis = 0,
    #[doc = "1: Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    Pushpull = 1,
    #[doc = "2: Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    Od = 2,
    #[doc = "3: Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    Ts = 3,
}
impl From<Outcfg22> for u8 {
    #[inline(always)]
    fn from(variant: Outcfg22) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Outcfg22 {
    type Ux = u8;
}
impl crate::IsEnum for Outcfg22 {}
#[doc = "Field `OUTCFG22` reader - Pin IO mode selection for GPIO pin 22"]
pub type Outcfg22R = crate::FieldReader<Outcfg22>;
impl Outcfg22R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Outcfg22 {
        match self.bits {
            0 => Outcfg22::Dis,
            1 => Outcfg22::Pushpull,
            2 => Outcfg22::Od,
            3 => Outcfg22::Ts,
            _ => unreachable!(),
        }
    }
    #[doc = "Output Disabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Outcfg22::Dis
    }
    #[doc = "Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == Outcfg22::Pushpull
    }
    #[doc = "Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == Outcfg22::Od
    }
    #[doc = "Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == Outcfg22::Ts
    }
}
#[doc = "Field `OUTCFG22` writer - Pin IO mode selection for GPIO pin 22"]
pub type Outcfg22W<'a, REG> = crate::FieldWriter<'a, REG, 2, Outcfg22, crate::Safe>;
impl<'a, REG> Outcfg22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Output Disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg22::Dis)
    }
    #[doc = "Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg22::Pushpull)
    }
    #[doc = "Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    #[inline(always)]
    pub fn od(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg22::Od)
    }
    #[doc = "Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    #[inline(always)]
    pub fn ts(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg22::Ts)
    }
}
#[doc = "Drive strength selection for GPIO 22\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ds22 {
    #[doc = "0: 0.1x output driver selected"]
    _0p1x = 0,
    #[doc = "1: 0.5x output driver selected"]
    _0p5x = 1,
    #[doc = "2: 0.75x output driver selected"]
    _0p75x = 2,
    #[doc = "3: 1.0x output driver selected"]
    _1p0x = 3,
}
impl From<Ds22> for u8 {
    #[inline(always)]
    fn from(variant: Ds22) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ds22 {
    type Ux = u8;
}
impl crate::IsEnum for Ds22 {}
#[doc = "Field `DS22` reader - Drive strength selection for GPIO 22"]
pub type Ds22R = crate::FieldReader<Ds22>;
impl Ds22R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ds22 {
        match self.bits {
            0 => Ds22::_0p1x,
            1 => Ds22::_0p5x,
            2 => Ds22::_0p75x,
            3 => Ds22::_1p0x,
            _ => unreachable!(),
        }
    }
    #[doc = "0.1x output driver selected"]
    #[inline(always)]
    pub fn is_0p1x(&self) -> bool {
        *self == Ds22::_0p1x
    }
    #[doc = "0.5x output driver selected"]
    #[inline(always)]
    pub fn is_0p5x(&self) -> bool {
        *self == Ds22::_0p5x
    }
    #[doc = "0.75x output driver selected"]
    #[inline(always)]
    pub fn is_0p75x(&self) -> bool {
        *self == Ds22::_0p75x
    }
    #[doc = "1.0x output driver selected"]
    #[inline(always)]
    pub fn is_1p0x(&self) -> bool {
        *self == Ds22::_1p0x
    }
}
#[doc = "Field `DS22` writer - Drive strength selection for GPIO 22"]
pub type Ds22W<'a, REG> = crate::FieldWriter<'a, REG, 2, Ds22, crate::Safe>;
impl<'a, REG> Ds22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "0.1x output driver selected"]
    #[inline(always)]
    pub fn _0p1x(self) -> &'a mut crate::W<REG> {
        self.variant(Ds22::_0p1x)
    }
    #[doc = "0.5x output driver selected"]
    #[inline(always)]
    pub fn _0p5x(self) -> &'a mut crate::W<REG> {
        self.variant(Ds22::_0p5x)
    }
    #[doc = "0.75x output driver selected"]
    #[inline(always)]
    pub fn _0p75x(self) -> &'a mut crate::W<REG> {
        self.variant(Ds22::_0p75x)
    }
    #[doc = "1.0x output driver selected"]
    #[inline(always)]
    pub fn _1p0x(self) -> &'a mut crate::W<REG> {
        self.variant(Ds22::_1p0x)
    }
}
#[doc = "Field `SR22` reader - Configure the slew rate"]
pub type Sr22R = crate::BitReader;
#[doc = "Field `SR22` writer - Configure the slew rate"]
pub type Sr22W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Pullup/Pulldown configuration for GPIO 22\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pullcfg22 {
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
impl From<Pullcfg22> for u8 {
    #[inline(always)]
    fn from(variant: Pullcfg22) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pullcfg22 {
    type Ux = u8;
}
impl crate::IsEnum for Pullcfg22 {}
#[doc = "Field `PULLCFG22` reader - Pullup/Pulldown configuration for GPIO 22"]
pub type Pullcfg22R = crate::FieldReader<Pullcfg22>;
impl Pullcfg22R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pullcfg22 {
        match self.bits {
            0 => Pullcfg22::Dis,
            1 => Pullcfg22::Pd50k,
            2 => Pullcfg22::Pu15k,
            3 => Pullcfg22::Pu6k,
            4 => Pullcfg22::Pu12k,
            5 => Pullcfg22::Pu24k,
            6 => Pullcfg22::Pu50k,
            7 => Pullcfg22::Pu100k,
            _ => unreachable!(),
        }
    }
    #[doc = "No pullup or pulldown selected"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Pullcfg22::Dis
    }
    #[doc = "50K Pulldown selected"]
    #[inline(always)]
    pub fn is_pd50k(&self) -> bool {
        *self == Pullcfg22::Pd50k
    }
    #[doc = "1.5K Pullup selected"]
    #[inline(always)]
    pub fn is_pu15k(&self) -> bool {
        *self == Pullcfg22::Pu15k
    }
    #[doc = "6K Pullup selected"]
    #[inline(always)]
    pub fn is_pu6k(&self) -> bool {
        *self == Pullcfg22::Pu6k
    }
    #[doc = "12K Pullup selected"]
    #[inline(always)]
    pub fn is_pu12k(&self) -> bool {
        *self == Pullcfg22::Pu12k
    }
    #[doc = "24K Pullup selected"]
    #[inline(always)]
    pub fn is_pu24k(&self) -> bool {
        *self == Pullcfg22::Pu24k
    }
    #[doc = "50K Pullup selected"]
    #[inline(always)]
    pub fn is_pu50k(&self) -> bool {
        *self == Pullcfg22::Pu50k
    }
    #[doc = "100K Pullup selected"]
    #[inline(always)]
    pub fn is_pu100k(&self) -> bool {
        *self == Pullcfg22::Pu100k
    }
}
#[doc = "Field `PULLCFG22` writer - Pullup/Pulldown configuration for GPIO 22"]
pub type Pullcfg22W<'a, REG> = crate::FieldWriter<'a, REG, 3, Pullcfg22, crate::Safe>;
impl<'a, REG> Pullcfg22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No pullup or pulldown selected"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg22::Dis)
    }
    #[doc = "50K Pulldown selected"]
    #[inline(always)]
    pub fn pd50k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg22::Pd50k)
    }
    #[doc = "1.5K Pullup selected"]
    #[inline(always)]
    pub fn pu15k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg22::Pu15k)
    }
    #[doc = "6K Pullup selected"]
    #[inline(always)]
    pub fn pu6k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg22::Pu6k)
    }
    #[doc = "12K Pullup selected"]
    #[inline(always)]
    pub fn pu12k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg22::Pu12k)
    }
    #[doc = "24K Pullup selected"]
    #[inline(always)]
    pub fn pu24k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg22::Pu24k)
    }
    #[doc = "50K Pullup selected"]
    #[inline(always)]
    pub fn pu50k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg22::Pu50k)
    }
    #[doc = "100K Pullup selected"]
    #[inline(always)]
    pub fn pu100k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg22::Pu100k)
    }
}
#[doc = "IOMSTR/MSPI N Chip Select 22, DISP control signals DE, CSX, and CS. Polarity is determined by CE_POLARITY field\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ncesrc22 {
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
impl From<Ncesrc22> for u8 {
    #[inline(always)]
    fn from(variant: Ncesrc22) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ncesrc22 {
    type Ux = u8;
}
impl crate::IsEnum for Ncesrc22 {}
#[doc = "Field `NCESRC22` reader - IOMSTR/MSPI N Chip Select 22, DISP control signals DE, CSX, and CS. Polarity is determined by CE_POLARITY field"]
pub type Ncesrc22R = crate::FieldReader<Ncesrc22>;
impl Ncesrc22R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ncesrc22> {
        match self.bits {
            0 => Some(Ncesrc22::Iom0ce0),
            1 => Some(Ncesrc22::Iom0ce1),
            2 => Some(Ncesrc22::Iom0ce2),
            3 => Some(Ncesrc22::Iom0ce3),
            4 => Some(Ncesrc22::Iom1ce0),
            5 => Some(Ncesrc22::Iom1ce1),
            6 => Some(Ncesrc22::Iom1ce2),
            7 => Some(Ncesrc22::Iom1ce3),
            8 => Some(Ncesrc22::Iom2ce0),
            9 => Some(Ncesrc22::Iom2ce1),
            10 => Some(Ncesrc22::Iom2ce2),
            11 => Some(Ncesrc22::Iom2ce3),
            12 => Some(Ncesrc22::Iom3ce0),
            13 => Some(Ncesrc22::Iom3ce1),
            14 => Some(Ncesrc22::Iom3ce2),
            15 => Some(Ncesrc22::Iom3ce3),
            16 => Some(Ncesrc22::Iom4ce0),
            17 => Some(Ncesrc22::Iom4ce1),
            18 => Some(Ncesrc22::Iom4ce2),
            19 => Some(Ncesrc22::Iom4ce3),
            20 => Some(Ncesrc22::Iom5ce0),
            21 => Some(Ncesrc22::Iom5ce1),
            22 => Some(Ncesrc22::Iom5ce2),
            23 => Some(Ncesrc22::Iom5ce3),
            24 => Some(Ncesrc22::Iom6ce0),
            25 => Some(Ncesrc22::Iom6ce1),
            26 => Some(Ncesrc22::Iom6ce2),
            27 => Some(Ncesrc22::Iom6ce3),
            28 => Some(Ncesrc22::Iom7ce0),
            29 => Some(Ncesrc22::Iom7ce1),
            30 => Some(Ncesrc22::Iom7ce2),
            31 => Some(Ncesrc22::Iom7ce3),
            32 => Some(Ncesrc22::Mspi0cen0),
            33 => Some(Ncesrc22::Mspi0cen1),
            34 => Some(Ncesrc22::Mspi1cen0),
            35 => Some(Ncesrc22::Mspi1cen1),
            36 => Some(Ncesrc22::Mspi2cen0),
            37 => Some(Ncesrc22::Mspi2cen1),
            38 => Some(Ncesrc22::DcDpiDe),
            39 => Some(Ncesrc22::DispContCsx),
            40 => Some(Ncesrc22::DcSpiCsN),
            41 => Some(Ncesrc22::DcQspiCsN),
            42 => Some(Ncesrc22::DcResx),
            _ => None,
        }
    }
    #[doc = "IOM 0 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom0ce0(&self) -> bool {
        *self == Ncesrc22::Iom0ce0
    }
    #[doc = "IOM 0 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom0ce1(&self) -> bool {
        *self == Ncesrc22::Iom0ce1
    }
    #[doc = "IOM 0 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom0ce2(&self) -> bool {
        *self == Ncesrc22::Iom0ce2
    }
    #[doc = "IOM 0 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom0ce3(&self) -> bool {
        *self == Ncesrc22::Iom0ce3
    }
    #[doc = "IOM 1 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom1ce0(&self) -> bool {
        *self == Ncesrc22::Iom1ce0
    }
    #[doc = "IOM 1 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom1ce1(&self) -> bool {
        *self == Ncesrc22::Iom1ce1
    }
    #[doc = "IOM 1 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom1ce2(&self) -> bool {
        *self == Ncesrc22::Iom1ce2
    }
    #[doc = "IOM 1 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom1ce3(&self) -> bool {
        *self == Ncesrc22::Iom1ce3
    }
    #[doc = "IOM 2 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom2ce0(&self) -> bool {
        *self == Ncesrc22::Iom2ce0
    }
    #[doc = "IOM 2 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom2ce1(&self) -> bool {
        *self == Ncesrc22::Iom2ce1
    }
    #[doc = "IOM 2 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom2ce2(&self) -> bool {
        *self == Ncesrc22::Iom2ce2
    }
    #[doc = "IOM 2 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom2ce3(&self) -> bool {
        *self == Ncesrc22::Iom2ce3
    }
    #[doc = "IOM 3 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom3ce0(&self) -> bool {
        *self == Ncesrc22::Iom3ce0
    }
    #[doc = "IOM 3 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom3ce1(&self) -> bool {
        *self == Ncesrc22::Iom3ce1
    }
    #[doc = "IOM 3 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom3ce2(&self) -> bool {
        *self == Ncesrc22::Iom3ce2
    }
    #[doc = "IOM 3 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom3ce3(&self) -> bool {
        *self == Ncesrc22::Iom3ce3
    }
    #[doc = "IOM 4 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom4ce0(&self) -> bool {
        *self == Ncesrc22::Iom4ce0
    }
    #[doc = "IOM 4 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom4ce1(&self) -> bool {
        *self == Ncesrc22::Iom4ce1
    }
    #[doc = "IOM 4 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom4ce2(&self) -> bool {
        *self == Ncesrc22::Iom4ce2
    }
    #[doc = "IOM 4 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom4ce3(&self) -> bool {
        *self == Ncesrc22::Iom4ce3
    }
    #[doc = "IOM 5 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom5ce0(&self) -> bool {
        *self == Ncesrc22::Iom5ce0
    }
    #[doc = "IOM 5 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom5ce1(&self) -> bool {
        *self == Ncesrc22::Iom5ce1
    }
    #[doc = "IOM 5 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom5ce2(&self) -> bool {
        *self == Ncesrc22::Iom5ce2
    }
    #[doc = "IOM 5 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom5ce3(&self) -> bool {
        *self == Ncesrc22::Iom5ce3
    }
    #[doc = "IOM 6 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom6ce0(&self) -> bool {
        *self == Ncesrc22::Iom6ce0
    }
    #[doc = "IOM 6 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom6ce1(&self) -> bool {
        *self == Ncesrc22::Iom6ce1
    }
    #[doc = "IOM 6 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom6ce2(&self) -> bool {
        *self == Ncesrc22::Iom6ce2
    }
    #[doc = "IOM 6 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom6ce3(&self) -> bool {
        *self == Ncesrc22::Iom6ce3
    }
    #[doc = "IOM 7 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom7ce0(&self) -> bool {
        *self == Ncesrc22::Iom7ce0
    }
    #[doc = "IOM 7 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom7ce1(&self) -> bool {
        *self == Ncesrc22::Iom7ce1
    }
    #[doc = "IOM 7 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom7ce2(&self) -> bool {
        *self == Ncesrc22::Iom7ce2
    }
    #[doc = "IOM 7 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom7ce3(&self) -> bool {
        *self == Ncesrc22::Iom7ce3
    }
    #[doc = "MSPI 0 NCE 0 module"]
    #[inline(always)]
    pub fn is_mspi0cen0(&self) -> bool {
        *self == Ncesrc22::Mspi0cen0
    }
    #[doc = "MSPI 0 NCE 1 module"]
    #[inline(always)]
    pub fn is_mspi0cen1(&self) -> bool {
        *self == Ncesrc22::Mspi0cen1
    }
    #[doc = "MSPI 1 NCE 0 module"]
    #[inline(always)]
    pub fn is_mspi1cen0(&self) -> bool {
        *self == Ncesrc22::Mspi1cen0
    }
    #[doc = "MSPI 1 NCE 1 module"]
    #[inline(always)]
    pub fn is_mspi1cen1(&self) -> bool {
        *self == Ncesrc22::Mspi1cen1
    }
    #[doc = "MSPI 2 NCE 0 module"]
    #[inline(always)]
    pub fn is_mspi2cen0(&self) -> bool {
        *self == Ncesrc22::Mspi2cen0
    }
    #[doc = "MSPI 2 NCE 1 module"]
    #[inline(always)]
    pub fn is_mspi2cen1(&self) -> bool {
        *self == Ncesrc22::Mspi2cen1
    }
    #[doc = "DC DPI DE module"]
    #[inline(always)]
    pub fn is_dc_dpi_de(&self) -> bool {
        *self == Ncesrc22::DcDpiDe
    }
    #[doc = "DISP CONT CSX module"]
    #[inline(always)]
    pub fn is_disp_cont_csx(&self) -> bool {
        *self == Ncesrc22::DispContCsx
    }
    #[doc = "DC SPI CS_N module"]
    #[inline(always)]
    pub fn is_dc_spi_cs_n(&self) -> bool {
        *self == Ncesrc22::DcSpiCsN
    }
    #[doc = "DC QSPI CS_N module"]
    #[inline(always)]
    pub fn is_dc_qspi_cs_n(&self) -> bool {
        *self == Ncesrc22::DcQspiCsN
    }
    #[doc = "DC module RESX"]
    #[inline(always)]
    pub fn is_dc_resx(&self) -> bool {
        *self == Ncesrc22::DcResx
    }
}
#[doc = "Field `NCESRC22` writer - IOMSTR/MSPI N Chip Select 22, DISP control signals DE, CSX, and CS. Polarity is determined by CE_POLARITY field"]
pub type Ncesrc22W<'a, REG> = crate::FieldWriter<'a, REG, 6, Ncesrc22>;
impl<'a, REG> Ncesrc22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "IOM 0 NCE 0 module"]
    #[inline(always)]
    pub fn iom0ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc22::Iom0ce0)
    }
    #[doc = "IOM 0 NCE 1 module"]
    #[inline(always)]
    pub fn iom0ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc22::Iom0ce1)
    }
    #[doc = "IOM 0 NCE 2 module"]
    #[inline(always)]
    pub fn iom0ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc22::Iom0ce2)
    }
    #[doc = "IOM 0 NCE 3 module"]
    #[inline(always)]
    pub fn iom0ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc22::Iom0ce3)
    }
    #[doc = "IOM 1 NCE 0 module"]
    #[inline(always)]
    pub fn iom1ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc22::Iom1ce0)
    }
    #[doc = "IOM 1 NCE 1 module"]
    #[inline(always)]
    pub fn iom1ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc22::Iom1ce1)
    }
    #[doc = "IOM 1 NCE 2 module"]
    #[inline(always)]
    pub fn iom1ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc22::Iom1ce2)
    }
    #[doc = "IOM 1 NCE 3 module"]
    #[inline(always)]
    pub fn iom1ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc22::Iom1ce3)
    }
    #[doc = "IOM 2 NCE 0 module"]
    #[inline(always)]
    pub fn iom2ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc22::Iom2ce0)
    }
    #[doc = "IOM 2 NCE 1 module"]
    #[inline(always)]
    pub fn iom2ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc22::Iom2ce1)
    }
    #[doc = "IOM 2 NCE 2 module"]
    #[inline(always)]
    pub fn iom2ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc22::Iom2ce2)
    }
    #[doc = "IOM 2 NCE 3 module"]
    #[inline(always)]
    pub fn iom2ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc22::Iom2ce3)
    }
    #[doc = "IOM 3 NCE 0 module"]
    #[inline(always)]
    pub fn iom3ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc22::Iom3ce0)
    }
    #[doc = "IOM 3 NCE 1 module"]
    #[inline(always)]
    pub fn iom3ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc22::Iom3ce1)
    }
    #[doc = "IOM 3 NCE 2 module"]
    #[inline(always)]
    pub fn iom3ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc22::Iom3ce2)
    }
    #[doc = "IOM 3 NCE 3 module"]
    #[inline(always)]
    pub fn iom3ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc22::Iom3ce3)
    }
    #[doc = "IOM 4 NCE 0 module"]
    #[inline(always)]
    pub fn iom4ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc22::Iom4ce0)
    }
    #[doc = "IOM 4 NCE 1 module"]
    #[inline(always)]
    pub fn iom4ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc22::Iom4ce1)
    }
    #[doc = "IOM 4 NCE 2 module"]
    #[inline(always)]
    pub fn iom4ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc22::Iom4ce2)
    }
    #[doc = "IOM 4 NCE 3 module"]
    #[inline(always)]
    pub fn iom4ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc22::Iom4ce3)
    }
    #[doc = "IOM 5 NCE 0 module"]
    #[inline(always)]
    pub fn iom5ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc22::Iom5ce0)
    }
    #[doc = "IOM 5 NCE 1 module"]
    #[inline(always)]
    pub fn iom5ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc22::Iom5ce1)
    }
    #[doc = "IOM 5 NCE 2 module"]
    #[inline(always)]
    pub fn iom5ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc22::Iom5ce2)
    }
    #[doc = "IOM 5 NCE 3 module"]
    #[inline(always)]
    pub fn iom5ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc22::Iom5ce3)
    }
    #[doc = "IOM 6 NCE 0 module"]
    #[inline(always)]
    pub fn iom6ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc22::Iom6ce0)
    }
    #[doc = "IOM 6 NCE 1 module"]
    #[inline(always)]
    pub fn iom6ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc22::Iom6ce1)
    }
    #[doc = "IOM 6 NCE 2 module"]
    #[inline(always)]
    pub fn iom6ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc22::Iom6ce2)
    }
    #[doc = "IOM 6 NCE 3 module"]
    #[inline(always)]
    pub fn iom6ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc22::Iom6ce3)
    }
    #[doc = "IOM 7 NCE 0 module"]
    #[inline(always)]
    pub fn iom7ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc22::Iom7ce0)
    }
    #[doc = "IOM 7 NCE 1 module"]
    #[inline(always)]
    pub fn iom7ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc22::Iom7ce1)
    }
    #[doc = "IOM 7 NCE 2 module"]
    #[inline(always)]
    pub fn iom7ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc22::Iom7ce2)
    }
    #[doc = "IOM 7 NCE 3 module"]
    #[inline(always)]
    pub fn iom7ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc22::Iom7ce3)
    }
    #[doc = "MSPI 0 NCE 0 module"]
    #[inline(always)]
    pub fn mspi0cen0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc22::Mspi0cen0)
    }
    #[doc = "MSPI 0 NCE 1 module"]
    #[inline(always)]
    pub fn mspi0cen1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc22::Mspi0cen1)
    }
    #[doc = "MSPI 1 NCE 0 module"]
    #[inline(always)]
    pub fn mspi1cen0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc22::Mspi1cen0)
    }
    #[doc = "MSPI 1 NCE 1 module"]
    #[inline(always)]
    pub fn mspi1cen1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc22::Mspi1cen1)
    }
    #[doc = "MSPI 2 NCE 0 module"]
    #[inline(always)]
    pub fn mspi2cen0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc22::Mspi2cen0)
    }
    #[doc = "MSPI 2 NCE 1 module"]
    #[inline(always)]
    pub fn mspi2cen1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc22::Mspi2cen1)
    }
    #[doc = "DC DPI DE module"]
    #[inline(always)]
    pub fn dc_dpi_de(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc22::DcDpiDe)
    }
    #[doc = "DISP CONT CSX module"]
    #[inline(always)]
    pub fn disp_cont_csx(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc22::DispContCsx)
    }
    #[doc = "DC SPI CS_N module"]
    #[inline(always)]
    pub fn dc_spi_cs_n(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc22::DcSpiCsN)
    }
    #[doc = "DC QSPI CS_N module"]
    #[inline(always)]
    pub fn dc_qspi_cs_n(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc22::DcQspiCsN)
    }
    #[doc = "DC module RESX"]
    #[inline(always)]
    pub fn dc_resx(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc22::DcResx)
    }
}
#[doc = "Polarity select for NCE for GPIO 22\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ncepol22 {
    #[doc = "0: Polarity is active low"]
    Low = 0,
    #[doc = "1: Polarity is active high"]
    High = 1,
}
impl From<Ncepol22> for bool {
    #[inline(always)]
    fn from(variant: Ncepol22) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NCEPOL22` reader - Polarity select for NCE for GPIO 22"]
pub type Ncepol22R = crate::BitReader<Ncepol22>;
impl Ncepol22R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ncepol22 {
        match self.bits {
            false => Ncepol22::Low,
            true => Ncepol22::High,
        }
    }
    #[doc = "Polarity is active low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Ncepol22::Low
    }
    #[doc = "Polarity is active high"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Ncepol22::High
    }
}
#[doc = "Field `NCEPOL22` writer - Polarity select for NCE for GPIO 22"]
pub type Ncepol22W<'a, REG> = crate::BitWriter<'a, REG, Ncepol22>;
impl<'a, REG> Ncepol22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Polarity is active low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Ncepol22::Low)
    }
    #[doc = "Polarity is active high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Ncepol22::High)
    }
}
#[doc = "Field `FIEN22` reader - Force input enable active regardless of function selected. Otherwise the selected function will enable the input only when needed"]
pub type Fien22R = crate::BitReader;
#[doc = "Field `FIEN22` writer - Force input enable active regardless of function selected. Otherwise the selected function will enable the input only when needed"]
pub type Fien22W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FOEN22` reader - Force output enable active regardless of function selected. Otherwise the selected function will enable the output only when needed"]
pub type Foen22R = crate::BitReader;
#[doc = "Field `FOEN22` writer - Force output enable active regardless of function selected. Otherwise the selected function will enable the output only when needed"]
pub type Foen22W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Function select for GPIO pin 22"]
    #[inline(always)]
    pub fn fncsel22(&self) -> Fncsel22R {
        Fncsel22R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Input enable for GPIO 22"]
    #[inline(always)]
    pub fn inpen22(&self) -> Inpen22R {
        Inpen22R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Return 0 for read data on GPIO 22"]
    #[inline(always)]
    pub fn rdzero22(&self) -> Rdzero22R {
        Rdzero22R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Interrupt enable for GPIO 22"]
    #[inline(always)]
    pub fn irpten22(&self) -> Irpten22R {
        Irpten22R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Pin IO mode selection for GPIO pin 22"]
    #[inline(always)]
    pub fn outcfg22(&self) -> Outcfg22R {
        Outcfg22R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Drive strength selection for GPIO 22"]
    #[inline(always)]
    pub fn ds22(&self) -> Ds22R {
        Ds22R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - Configure the slew rate"]
    #[inline(always)]
    pub fn sr22(&self) -> Sr22R {
        Sr22R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - Pullup/Pulldown configuration for GPIO 22"]
    #[inline(always)]
    pub fn pullcfg22(&self) -> Pullcfg22R {
        Pullcfg22R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:21 - IOMSTR/MSPI N Chip Select 22, DISP control signals DE, CSX, and CS. Polarity is determined by CE_POLARITY field"]
    #[inline(always)]
    pub fn ncesrc22(&self) -> Ncesrc22R {
        Ncesrc22R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 22 - Polarity select for NCE for GPIO 22"]
    #[inline(always)]
    pub fn ncepol22(&self) -> Ncepol22R {
        Ncepol22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 26 - Force input enable active regardless of function selected. Otherwise the selected function will enable the input only when needed"]
    #[inline(always)]
    pub fn fien22(&self) -> Fien22R {
        Fien22R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Force output enable active regardless of function selected. Otherwise the selected function will enable the output only when needed"]
    #[inline(always)]
    pub fn foen22(&self) -> Foen22R {
        Foen22R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Function select for GPIO pin 22"]
    #[inline(always)]
    #[must_use]
    pub fn fncsel22(&mut self) -> Fncsel22W<Pincfg22Spec> {
        Fncsel22W::new(self, 0)
    }
    #[doc = "Bit 4 - Input enable for GPIO 22"]
    #[inline(always)]
    #[must_use]
    pub fn inpen22(&mut self) -> Inpen22W<Pincfg22Spec> {
        Inpen22W::new(self, 4)
    }
    #[doc = "Bit 5 - Return 0 for read data on GPIO 22"]
    #[inline(always)]
    #[must_use]
    pub fn rdzero22(&mut self) -> Rdzero22W<Pincfg22Spec> {
        Rdzero22W::new(self, 5)
    }
    #[doc = "Bits 6:7 - Interrupt enable for GPIO 22"]
    #[inline(always)]
    #[must_use]
    pub fn irpten22(&mut self) -> Irpten22W<Pincfg22Spec> {
        Irpten22W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Pin IO mode selection for GPIO pin 22"]
    #[inline(always)]
    #[must_use]
    pub fn outcfg22(&mut self) -> Outcfg22W<Pincfg22Spec> {
        Outcfg22W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Drive strength selection for GPIO 22"]
    #[inline(always)]
    #[must_use]
    pub fn ds22(&mut self) -> Ds22W<Pincfg22Spec> {
        Ds22W::new(self, 10)
    }
    #[doc = "Bit 12 - Configure the slew rate"]
    #[inline(always)]
    #[must_use]
    pub fn sr22(&mut self) -> Sr22W<Pincfg22Spec> {
        Sr22W::new(self, 12)
    }
    #[doc = "Bits 13:15 - Pullup/Pulldown configuration for GPIO 22"]
    #[inline(always)]
    #[must_use]
    pub fn pullcfg22(&mut self) -> Pullcfg22W<Pincfg22Spec> {
        Pullcfg22W::new(self, 13)
    }
    #[doc = "Bits 16:21 - IOMSTR/MSPI N Chip Select 22, DISP control signals DE, CSX, and CS. Polarity is determined by CE_POLARITY field"]
    #[inline(always)]
    #[must_use]
    pub fn ncesrc22(&mut self) -> Ncesrc22W<Pincfg22Spec> {
        Ncesrc22W::new(self, 16)
    }
    #[doc = "Bit 22 - Polarity select for NCE for GPIO 22"]
    #[inline(always)]
    #[must_use]
    pub fn ncepol22(&mut self) -> Ncepol22W<Pincfg22Spec> {
        Ncepol22W::new(self, 22)
    }
    #[doc = "Bit 26 - Force input enable active regardless of function selected. Otherwise the selected function will enable the input only when needed"]
    #[inline(always)]
    #[must_use]
    pub fn fien22(&mut self) -> Fien22W<Pincfg22Spec> {
        Fien22W::new(self, 26)
    }
    #[doc = "Bit 27 - Force output enable active regardless of function selected. Otherwise the selected function will enable the output only when needed"]
    #[inline(always)]
    #[must_use]
    pub fn foen22(&mut self) -> Foen22W<Pincfg22Spec> {
        Foen22W::new(self, 27)
    }
}
#[doc = "Controls the operation of GPIO pin 22.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg22::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg22::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pincfg22Spec;
impl crate::RegisterSpec for Pincfg22Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pincfg22::R`](R) reader structure"]
impl crate::Readable for Pincfg22Spec {}
#[doc = "`write(|w| ..)` method takes [`pincfg22::W`](W) writer structure"]
impl crate::Writable for Pincfg22Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PINCFG22 to value 0x03"]
impl crate::Resettable for Pincfg22Spec {
    const RESET_VALUE: u32 = 0x03;
}
