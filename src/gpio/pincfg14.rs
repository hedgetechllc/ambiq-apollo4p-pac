#[doc = "Register `PINCFG14` reader"]
pub type R = crate::R<Pincfg14Spec>;
#[doc = "Register `PINCFG14` writer"]
pub type W = crate::W<Pincfg14Spec>;
#[doc = "Function select for GPIO pin 14\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fncsel14 {
    #[doc = "0: Analog to Digital Converter SE IN5"]
    Adcse5 = 0,
    #[doc = "1: ADC trigger input"]
    Trig3 = 1,
    #[doc = "2: Reserved selection. Operation unknown if selected."]
    Reserved2 = 2,
    #[doc = "3: General purpose I/O"]
    Gpio = 3,
    #[doc = "4: MILLI Clock"]
    MilliClk = 4,
    #[doc = "5: UART receive input (UART 1)"]
    Uart1rx = 5,
    #[doc = "6: Timer/Counter input or output; Selection of direction is done via CTIMER register settings."]
    Ct14 = 6,
    #[doc = "7: IOMSTR/MSPI N Chip Select. Polarity is determined by CE_POLARITY field"]
    Nce14 = 7,
    #[doc = "8: Observation bus bit 14"]
    Obsbus14 = 8,
    #[doc = "9: Reserved selection. Operation unknown if selected."]
    Reserved9 = 9,
    #[doc = "10: I2S Data input (I2S Master/Slave 2)"]
    I2s0Sdin = 10,
    #[doc = "11: Fast PIO"]
    Fpio = 11,
    #[doc = "12: Reserved selection. Operation unknown if selected."]
    Reserved12 = 12,
    #[doc = "13: Internal function (Flash parallel load)"]
    FlloadAddr = 13,
    #[doc = "14: Internal function (MBIST)"]
    MdaTrstn = 14,
    #[doc = "15: Internal function (SCAN)"]
    Scanout2 = 15,
}
impl From<Fncsel14> for u8 {
    #[inline(always)]
    fn from(variant: Fncsel14) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fncsel14 {
    type Ux = u8;
}
impl crate::IsEnum for Fncsel14 {}
#[doc = "Field `FNCSEL14` reader - Function select for GPIO pin 14"]
pub type Fncsel14R = crate::FieldReader<Fncsel14>;
impl Fncsel14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fncsel14 {
        match self.bits {
            0 => Fncsel14::Adcse5,
            1 => Fncsel14::Trig3,
            2 => Fncsel14::Reserved2,
            3 => Fncsel14::Gpio,
            4 => Fncsel14::MilliClk,
            5 => Fncsel14::Uart1rx,
            6 => Fncsel14::Ct14,
            7 => Fncsel14::Nce14,
            8 => Fncsel14::Obsbus14,
            9 => Fncsel14::Reserved9,
            10 => Fncsel14::I2s0Sdin,
            11 => Fncsel14::Fpio,
            12 => Fncsel14::Reserved12,
            13 => Fncsel14::FlloadAddr,
            14 => Fncsel14::MdaTrstn,
            15 => Fncsel14::Scanout2,
            _ => unreachable!(),
        }
    }
    #[doc = "Analog to Digital Converter SE IN5"]
    #[inline(always)]
    pub fn is_adcse5(&self) -> bool {
        *self == Fncsel14::Adcse5
    }
    #[doc = "ADC trigger input"]
    #[inline(always)]
    pub fn is_trig3(&self) -> bool {
        *self == Fncsel14::Trig3
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == Fncsel14::Reserved2
    }
    #[doc = "General purpose I/O"]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == Fncsel14::Gpio
    }
    #[doc = "MILLI Clock"]
    #[inline(always)]
    pub fn is_milli_clk(&self) -> bool {
        *self == Fncsel14::MilliClk
    }
    #[doc = "UART receive input (UART 1)"]
    #[inline(always)]
    pub fn is_uart1rx(&self) -> bool {
        *self == Fncsel14::Uart1rx
    }
    #[doc = "Timer/Counter input or output; Selection of direction is done via CTIMER register settings."]
    #[inline(always)]
    pub fn is_ct14(&self) -> bool {
        *self == Fncsel14::Ct14
    }
    #[doc = "IOMSTR/MSPI N Chip Select. Polarity is determined by CE_POLARITY field"]
    #[inline(always)]
    pub fn is_nce14(&self) -> bool {
        *self == Fncsel14::Nce14
    }
    #[doc = "Observation bus bit 14"]
    #[inline(always)]
    pub fn is_obsbus14(&self) -> bool {
        *self == Fncsel14::Obsbus14
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved9(&self) -> bool {
        *self == Fncsel14::Reserved9
    }
    #[doc = "I2S Data input (I2S Master/Slave 2)"]
    #[inline(always)]
    pub fn is_i2s0_sdin(&self) -> bool {
        *self == Fncsel14::I2s0Sdin
    }
    #[doc = "Fast PIO"]
    #[inline(always)]
    pub fn is_fpio(&self) -> bool {
        *self == Fncsel14::Fpio
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved12(&self) -> bool {
        *self == Fncsel14::Reserved12
    }
    #[doc = "Internal function (Flash parallel load)"]
    #[inline(always)]
    pub fn is_flload_addr(&self) -> bool {
        *self == Fncsel14::FlloadAddr
    }
    #[doc = "Internal function (MBIST)"]
    #[inline(always)]
    pub fn is_mda_trstn(&self) -> bool {
        *self == Fncsel14::MdaTrstn
    }
    #[doc = "Internal function (SCAN)"]
    #[inline(always)]
    pub fn is_scanout2(&self) -> bool {
        *self == Fncsel14::Scanout2
    }
}
#[doc = "Field `FNCSEL14` writer - Function select for GPIO pin 14"]
pub type Fncsel14W<'a, REG> = crate::FieldWriter<'a, REG, 4, Fncsel14, crate::Safe>;
impl<'a, REG> Fncsel14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Analog to Digital Converter SE IN5"]
    #[inline(always)]
    pub fn adcse5(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel14::Adcse5)
    }
    #[doc = "ADC trigger input"]
    #[inline(always)]
    pub fn trig3(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel14::Trig3)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel14::Reserved2)
    }
    #[doc = "General purpose I/O"]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel14::Gpio)
    }
    #[doc = "MILLI Clock"]
    #[inline(always)]
    pub fn milli_clk(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel14::MilliClk)
    }
    #[doc = "UART receive input (UART 1)"]
    #[inline(always)]
    pub fn uart1rx(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel14::Uart1rx)
    }
    #[doc = "Timer/Counter input or output; Selection of direction is done via CTIMER register settings."]
    #[inline(always)]
    pub fn ct14(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel14::Ct14)
    }
    #[doc = "IOMSTR/MSPI N Chip Select. Polarity is determined by CE_POLARITY field"]
    #[inline(always)]
    pub fn nce14(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel14::Nce14)
    }
    #[doc = "Observation bus bit 14"]
    #[inline(always)]
    pub fn obsbus14(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel14::Obsbus14)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved9(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel14::Reserved9)
    }
    #[doc = "I2S Data input (I2S Master/Slave 2)"]
    #[inline(always)]
    pub fn i2s0_sdin(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel14::I2s0Sdin)
    }
    #[doc = "Fast PIO"]
    #[inline(always)]
    pub fn fpio(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel14::Fpio)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved12(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel14::Reserved12)
    }
    #[doc = "Internal function (Flash parallel load)"]
    #[inline(always)]
    pub fn flload_addr(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel14::FlloadAddr)
    }
    #[doc = "Internal function (MBIST)"]
    #[inline(always)]
    pub fn mda_trstn(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel14::MdaTrstn)
    }
    #[doc = "Internal function (SCAN)"]
    #[inline(always)]
    pub fn scanout2(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel14::Scanout2)
    }
}
#[doc = "Field `INPEN14` reader - Input enable for GPIO 14"]
pub type Inpen14R = crate::BitReader;
#[doc = "Field `INPEN14` writer - Input enable for GPIO 14"]
pub type Inpen14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDZERO14` reader - Return 0 for read data on GPIO 14"]
pub type Rdzero14R = crate::BitReader;
#[doc = "Field `RDZERO14` writer - Return 0 for read data on GPIO 14"]
pub type Rdzero14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Interrupt enable for GPIO 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Irpten14 {
    #[doc = "0: Interrupts are disabled for this GPIO"]
    Dis = 0,
    #[doc = "1: Interrupts are enabled for falling edge transition on this GPIO"]
    Intfall = 1,
    #[doc = "2: Interrupts are enabled for rising edge transitions on this GPIO"]
    Intrise = 2,
    #[doc = "3: Interrupts are enabled for any edge transition on this GPIO"]
    Intany = 3,
}
impl From<Irpten14> for u8 {
    #[inline(always)]
    fn from(variant: Irpten14) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Irpten14 {
    type Ux = u8;
}
impl crate::IsEnum for Irpten14 {}
#[doc = "Field `IRPTEN14` reader - Interrupt enable for GPIO 14"]
pub type Irpten14R = crate::FieldReader<Irpten14>;
impl Irpten14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irpten14 {
        match self.bits {
            0 => Irpten14::Dis,
            1 => Irpten14::Intfall,
            2 => Irpten14::Intrise,
            3 => Irpten14::Intany,
            _ => unreachable!(),
        }
    }
    #[doc = "Interrupts are disabled for this GPIO"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Irpten14::Dis
    }
    #[doc = "Interrupts are enabled for falling edge transition on this GPIO"]
    #[inline(always)]
    pub fn is_intfall(&self) -> bool {
        *self == Irpten14::Intfall
    }
    #[doc = "Interrupts are enabled for rising edge transitions on this GPIO"]
    #[inline(always)]
    pub fn is_intrise(&self) -> bool {
        *self == Irpten14::Intrise
    }
    #[doc = "Interrupts are enabled for any edge transition on this GPIO"]
    #[inline(always)]
    pub fn is_intany(&self) -> bool {
        *self == Irpten14::Intany
    }
}
#[doc = "Field `IRPTEN14` writer - Interrupt enable for GPIO 14"]
pub type Irpten14W<'a, REG> = crate::FieldWriter<'a, REG, 2, Irpten14, crate::Safe>;
impl<'a, REG> Irpten14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Interrupts are disabled for this GPIO"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten14::Dis)
    }
    #[doc = "Interrupts are enabled for falling edge transition on this GPIO"]
    #[inline(always)]
    pub fn intfall(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten14::Intfall)
    }
    #[doc = "Interrupts are enabled for rising edge transitions on this GPIO"]
    #[inline(always)]
    pub fn intrise(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten14::Intrise)
    }
    #[doc = "Interrupts are enabled for any edge transition on this GPIO"]
    #[inline(always)]
    pub fn intany(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten14::Intany)
    }
}
#[doc = "Pin IO mode selection for GPIO pin 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Outcfg14 {
    #[doc = "0: Output Disabled"]
    Dis = 0,
    #[doc = "1: Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    Pushpull = 1,
    #[doc = "2: Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    Od = 2,
    #[doc = "3: Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    Ts = 3,
}
impl From<Outcfg14> for u8 {
    #[inline(always)]
    fn from(variant: Outcfg14) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Outcfg14 {
    type Ux = u8;
}
impl crate::IsEnum for Outcfg14 {}
#[doc = "Field `OUTCFG14` reader - Pin IO mode selection for GPIO pin 14"]
pub type Outcfg14R = crate::FieldReader<Outcfg14>;
impl Outcfg14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Outcfg14 {
        match self.bits {
            0 => Outcfg14::Dis,
            1 => Outcfg14::Pushpull,
            2 => Outcfg14::Od,
            3 => Outcfg14::Ts,
            _ => unreachable!(),
        }
    }
    #[doc = "Output Disabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Outcfg14::Dis
    }
    #[doc = "Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == Outcfg14::Pushpull
    }
    #[doc = "Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == Outcfg14::Od
    }
    #[doc = "Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == Outcfg14::Ts
    }
}
#[doc = "Field `OUTCFG14` writer - Pin IO mode selection for GPIO pin 14"]
pub type Outcfg14W<'a, REG> = crate::FieldWriter<'a, REG, 2, Outcfg14, crate::Safe>;
impl<'a, REG> Outcfg14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Output Disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg14::Dis)
    }
    #[doc = "Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg14::Pushpull)
    }
    #[doc = "Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    #[inline(always)]
    pub fn od(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg14::Od)
    }
    #[doc = "Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    #[inline(always)]
    pub fn ts(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg14::Ts)
    }
}
#[doc = "Drive strength selection for GPIO 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ds14 {
    #[doc = "0: 0.1x output driver selected"]
    _0p1x = 0,
    #[doc = "1: 0.5x output driver selected"]
    _0p5x = 1,
}
impl From<Ds14> for u8 {
    #[inline(always)]
    fn from(variant: Ds14) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ds14 {
    type Ux = u8;
}
impl crate::IsEnum for Ds14 {}
#[doc = "Field `DS14` reader - Drive strength selection for GPIO 14"]
pub type Ds14R = crate::FieldReader<Ds14>;
impl Ds14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ds14> {
        match self.bits {
            0 => Some(Ds14::_0p1x),
            1 => Some(Ds14::_0p5x),
            _ => None,
        }
    }
    #[doc = "0.1x output driver selected"]
    #[inline(always)]
    pub fn is_0p1x(&self) -> bool {
        *self == Ds14::_0p1x
    }
    #[doc = "0.5x output driver selected"]
    #[inline(always)]
    pub fn is_0p5x(&self) -> bool {
        *self == Ds14::_0p5x
    }
}
#[doc = "Field `DS14` writer - Drive strength selection for GPIO 14"]
pub type Ds14W<'a, REG> = crate::FieldWriter<'a, REG, 2, Ds14>;
impl<'a, REG> Ds14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "0.1x output driver selected"]
    #[inline(always)]
    pub fn _0p1x(self) -> &'a mut crate::W<REG> {
        self.variant(Ds14::_0p1x)
    }
    #[doc = "0.5x output driver selected"]
    #[inline(always)]
    pub fn _0p5x(self) -> &'a mut crate::W<REG> {
        self.variant(Ds14::_0p5x)
    }
}
#[doc = "Field `SR14` reader - Configure the slew rate"]
pub type Sr14R = crate::BitReader;
#[doc = "Field `SR14` writer - Configure the slew rate"]
pub type Sr14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Pullup/Pulldown configuration for GPIO 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pullcfg14 {
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
impl From<Pullcfg14> for u8 {
    #[inline(always)]
    fn from(variant: Pullcfg14) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pullcfg14 {
    type Ux = u8;
}
impl crate::IsEnum for Pullcfg14 {}
#[doc = "Field `PULLCFG14` reader - Pullup/Pulldown configuration for GPIO 14"]
pub type Pullcfg14R = crate::FieldReader<Pullcfg14>;
impl Pullcfg14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pullcfg14 {
        match self.bits {
            0 => Pullcfg14::Dis,
            1 => Pullcfg14::Pd50k,
            2 => Pullcfg14::Pu15k,
            3 => Pullcfg14::Pu6k,
            4 => Pullcfg14::Pu12k,
            5 => Pullcfg14::Pu24k,
            6 => Pullcfg14::Pu50k,
            7 => Pullcfg14::Pu100k,
            _ => unreachable!(),
        }
    }
    #[doc = "No pullup or pulldown selected"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Pullcfg14::Dis
    }
    #[doc = "50K Pulldown selected"]
    #[inline(always)]
    pub fn is_pd50k(&self) -> bool {
        *self == Pullcfg14::Pd50k
    }
    #[doc = "1.5K Pullup selected"]
    #[inline(always)]
    pub fn is_pu15k(&self) -> bool {
        *self == Pullcfg14::Pu15k
    }
    #[doc = "6K Pullup selected"]
    #[inline(always)]
    pub fn is_pu6k(&self) -> bool {
        *self == Pullcfg14::Pu6k
    }
    #[doc = "12K Pullup selected"]
    #[inline(always)]
    pub fn is_pu12k(&self) -> bool {
        *self == Pullcfg14::Pu12k
    }
    #[doc = "24K Pullup selected"]
    #[inline(always)]
    pub fn is_pu24k(&self) -> bool {
        *self == Pullcfg14::Pu24k
    }
    #[doc = "50K Pullup selected"]
    #[inline(always)]
    pub fn is_pu50k(&self) -> bool {
        *self == Pullcfg14::Pu50k
    }
    #[doc = "100K Pullup selected"]
    #[inline(always)]
    pub fn is_pu100k(&self) -> bool {
        *self == Pullcfg14::Pu100k
    }
}
#[doc = "Field `PULLCFG14` writer - Pullup/Pulldown configuration for GPIO 14"]
pub type Pullcfg14W<'a, REG> = crate::FieldWriter<'a, REG, 3, Pullcfg14, crate::Safe>;
impl<'a, REG> Pullcfg14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No pullup or pulldown selected"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg14::Dis)
    }
    #[doc = "50K Pulldown selected"]
    #[inline(always)]
    pub fn pd50k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg14::Pd50k)
    }
    #[doc = "1.5K Pullup selected"]
    #[inline(always)]
    pub fn pu15k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg14::Pu15k)
    }
    #[doc = "6K Pullup selected"]
    #[inline(always)]
    pub fn pu6k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg14::Pu6k)
    }
    #[doc = "12K Pullup selected"]
    #[inline(always)]
    pub fn pu12k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg14::Pu12k)
    }
    #[doc = "24K Pullup selected"]
    #[inline(always)]
    pub fn pu24k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg14::Pu24k)
    }
    #[doc = "50K Pullup selected"]
    #[inline(always)]
    pub fn pu50k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg14::Pu50k)
    }
    #[doc = "100K Pullup selected"]
    #[inline(always)]
    pub fn pu100k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg14::Pu100k)
    }
}
#[doc = "IOMSTR/MSPI N Chip Select 14, DISP control signals DE, CSX, and CS. Polarity is determined by CE_POLARITY field\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ncesrc14 {
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
impl From<Ncesrc14> for u8 {
    #[inline(always)]
    fn from(variant: Ncesrc14) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ncesrc14 {
    type Ux = u8;
}
impl crate::IsEnum for Ncesrc14 {}
#[doc = "Field `NCESRC14` reader - IOMSTR/MSPI N Chip Select 14, DISP control signals DE, CSX, and CS. Polarity is determined by CE_POLARITY field"]
pub type Ncesrc14R = crate::FieldReader<Ncesrc14>;
impl Ncesrc14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ncesrc14> {
        match self.bits {
            0 => Some(Ncesrc14::Iom0ce0),
            1 => Some(Ncesrc14::Iom0ce1),
            2 => Some(Ncesrc14::Iom0ce2),
            3 => Some(Ncesrc14::Iom0ce3),
            4 => Some(Ncesrc14::Iom1ce0),
            5 => Some(Ncesrc14::Iom1ce1),
            6 => Some(Ncesrc14::Iom1ce2),
            7 => Some(Ncesrc14::Iom1ce3),
            8 => Some(Ncesrc14::Iom2ce0),
            9 => Some(Ncesrc14::Iom2ce1),
            10 => Some(Ncesrc14::Iom2ce2),
            11 => Some(Ncesrc14::Iom2ce3),
            12 => Some(Ncesrc14::Iom3ce0),
            13 => Some(Ncesrc14::Iom3ce1),
            14 => Some(Ncesrc14::Iom3ce2),
            15 => Some(Ncesrc14::Iom3ce3),
            16 => Some(Ncesrc14::Iom4ce0),
            17 => Some(Ncesrc14::Iom4ce1),
            18 => Some(Ncesrc14::Iom4ce2),
            19 => Some(Ncesrc14::Iom4ce3),
            20 => Some(Ncesrc14::Iom5ce0),
            21 => Some(Ncesrc14::Iom5ce1),
            22 => Some(Ncesrc14::Iom5ce2),
            23 => Some(Ncesrc14::Iom5ce3),
            24 => Some(Ncesrc14::Iom6ce0),
            25 => Some(Ncesrc14::Iom6ce1),
            26 => Some(Ncesrc14::Iom6ce2),
            27 => Some(Ncesrc14::Iom6ce3),
            28 => Some(Ncesrc14::Iom7ce0),
            29 => Some(Ncesrc14::Iom7ce1),
            30 => Some(Ncesrc14::Iom7ce2),
            31 => Some(Ncesrc14::Iom7ce3),
            32 => Some(Ncesrc14::Mspi0cen0),
            33 => Some(Ncesrc14::Mspi0cen1),
            34 => Some(Ncesrc14::Mspi1cen0),
            35 => Some(Ncesrc14::Mspi1cen1),
            36 => Some(Ncesrc14::Mspi2cen0),
            37 => Some(Ncesrc14::Mspi2cen1),
            38 => Some(Ncesrc14::DcDpiDe),
            39 => Some(Ncesrc14::DispContCsx),
            40 => Some(Ncesrc14::DcSpiCsN),
            41 => Some(Ncesrc14::DcQspiCsN),
            42 => Some(Ncesrc14::DcResx),
            _ => None,
        }
    }
    #[doc = "IOM 0 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom0ce0(&self) -> bool {
        *self == Ncesrc14::Iom0ce0
    }
    #[doc = "IOM 0 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom0ce1(&self) -> bool {
        *self == Ncesrc14::Iom0ce1
    }
    #[doc = "IOM 0 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom0ce2(&self) -> bool {
        *self == Ncesrc14::Iom0ce2
    }
    #[doc = "IOM 0 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom0ce3(&self) -> bool {
        *self == Ncesrc14::Iom0ce3
    }
    #[doc = "IOM 1 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom1ce0(&self) -> bool {
        *self == Ncesrc14::Iom1ce0
    }
    #[doc = "IOM 1 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom1ce1(&self) -> bool {
        *self == Ncesrc14::Iom1ce1
    }
    #[doc = "IOM 1 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom1ce2(&self) -> bool {
        *self == Ncesrc14::Iom1ce2
    }
    #[doc = "IOM 1 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom1ce3(&self) -> bool {
        *self == Ncesrc14::Iom1ce3
    }
    #[doc = "IOM 2 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom2ce0(&self) -> bool {
        *self == Ncesrc14::Iom2ce0
    }
    #[doc = "IOM 2 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom2ce1(&self) -> bool {
        *self == Ncesrc14::Iom2ce1
    }
    #[doc = "IOM 2 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom2ce2(&self) -> bool {
        *self == Ncesrc14::Iom2ce2
    }
    #[doc = "IOM 2 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom2ce3(&self) -> bool {
        *self == Ncesrc14::Iom2ce3
    }
    #[doc = "IOM 3 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom3ce0(&self) -> bool {
        *self == Ncesrc14::Iom3ce0
    }
    #[doc = "IOM 3 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom3ce1(&self) -> bool {
        *self == Ncesrc14::Iom3ce1
    }
    #[doc = "IOM 3 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom3ce2(&self) -> bool {
        *self == Ncesrc14::Iom3ce2
    }
    #[doc = "IOM 3 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom3ce3(&self) -> bool {
        *self == Ncesrc14::Iom3ce3
    }
    #[doc = "IOM 4 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom4ce0(&self) -> bool {
        *self == Ncesrc14::Iom4ce0
    }
    #[doc = "IOM 4 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom4ce1(&self) -> bool {
        *self == Ncesrc14::Iom4ce1
    }
    #[doc = "IOM 4 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom4ce2(&self) -> bool {
        *self == Ncesrc14::Iom4ce2
    }
    #[doc = "IOM 4 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom4ce3(&self) -> bool {
        *self == Ncesrc14::Iom4ce3
    }
    #[doc = "IOM 5 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom5ce0(&self) -> bool {
        *self == Ncesrc14::Iom5ce0
    }
    #[doc = "IOM 5 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom5ce1(&self) -> bool {
        *self == Ncesrc14::Iom5ce1
    }
    #[doc = "IOM 5 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom5ce2(&self) -> bool {
        *self == Ncesrc14::Iom5ce2
    }
    #[doc = "IOM 5 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom5ce3(&self) -> bool {
        *self == Ncesrc14::Iom5ce3
    }
    #[doc = "IOM 6 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom6ce0(&self) -> bool {
        *self == Ncesrc14::Iom6ce0
    }
    #[doc = "IOM 6 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom6ce1(&self) -> bool {
        *self == Ncesrc14::Iom6ce1
    }
    #[doc = "IOM 6 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom6ce2(&self) -> bool {
        *self == Ncesrc14::Iom6ce2
    }
    #[doc = "IOM 6 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom6ce3(&self) -> bool {
        *self == Ncesrc14::Iom6ce3
    }
    #[doc = "IOM 7 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom7ce0(&self) -> bool {
        *self == Ncesrc14::Iom7ce0
    }
    #[doc = "IOM 7 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom7ce1(&self) -> bool {
        *self == Ncesrc14::Iom7ce1
    }
    #[doc = "IOM 7 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom7ce2(&self) -> bool {
        *self == Ncesrc14::Iom7ce2
    }
    #[doc = "IOM 7 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom7ce3(&self) -> bool {
        *self == Ncesrc14::Iom7ce3
    }
    #[doc = "MSPI 0 NCE 0 module"]
    #[inline(always)]
    pub fn is_mspi0cen0(&self) -> bool {
        *self == Ncesrc14::Mspi0cen0
    }
    #[doc = "MSPI 0 NCE 1 module"]
    #[inline(always)]
    pub fn is_mspi0cen1(&self) -> bool {
        *self == Ncesrc14::Mspi0cen1
    }
    #[doc = "MSPI 1 NCE 0 module"]
    #[inline(always)]
    pub fn is_mspi1cen0(&self) -> bool {
        *self == Ncesrc14::Mspi1cen0
    }
    #[doc = "MSPI 1 NCE 1 module"]
    #[inline(always)]
    pub fn is_mspi1cen1(&self) -> bool {
        *self == Ncesrc14::Mspi1cen1
    }
    #[doc = "MSPI 2 NCE 0 module"]
    #[inline(always)]
    pub fn is_mspi2cen0(&self) -> bool {
        *self == Ncesrc14::Mspi2cen0
    }
    #[doc = "MSPI 2 NCE 1 module"]
    #[inline(always)]
    pub fn is_mspi2cen1(&self) -> bool {
        *self == Ncesrc14::Mspi2cen1
    }
    #[doc = "DC DPI DE module"]
    #[inline(always)]
    pub fn is_dc_dpi_de(&self) -> bool {
        *self == Ncesrc14::DcDpiDe
    }
    #[doc = "DISP CONT CSX module"]
    #[inline(always)]
    pub fn is_disp_cont_csx(&self) -> bool {
        *self == Ncesrc14::DispContCsx
    }
    #[doc = "DC SPI CS_N module"]
    #[inline(always)]
    pub fn is_dc_spi_cs_n(&self) -> bool {
        *self == Ncesrc14::DcSpiCsN
    }
    #[doc = "DC QSPI CS_N module"]
    #[inline(always)]
    pub fn is_dc_qspi_cs_n(&self) -> bool {
        *self == Ncesrc14::DcQspiCsN
    }
    #[doc = "DC module RESX"]
    #[inline(always)]
    pub fn is_dc_resx(&self) -> bool {
        *self == Ncesrc14::DcResx
    }
}
#[doc = "Field `NCESRC14` writer - IOMSTR/MSPI N Chip Select 14, DISP control signals DE, CSX, and CS. Polarity is determined by CE_POLARITY field"]
pub type Ncesrc14W<'a, REG> = crate::FieldWriter<'a, REG, 6, Ncesrc14>;
impl<'a, REG> Ncesrc14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "IOM 0 NCE 0 module"]
    #[inline(always)]
    pub fn iom0ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc14::Iom0ce0)
    }
    #[doc = "IOM 0 NCE 1 module"]
    #[inline(always)]
    pub fn iom0ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc14::Iom0ce1)
    }
    #[doc = "IOM 0 NCE 2 module"]
    #[inline(always)]
    pub fn iom0ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc14::Iom0ce2)
    }
    #[doc = "IOM 0 NCE 3 module"]
    #[inline(always)]
    pub fn iom0ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc14::Iom0ce3)
    }
    #[doc = "IOM 1 NCE 0 module"]
    #[inline(always)]
    pub fn iom1ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc14::Iom1ce0)
    }
    #[doc = "IOM 1 NCE 1 module"]
    #[inline(always)]
    pub fn iom1ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc14::Iom1ce1)
    }
    #[doc = "IOM 1 NCE 2 module"]
    #[inline(always)]
    pub fn iom1ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc14::Iom1ce2)
    }
    #[doc = "IOM 1 NCE 3 module"]
    #[inline(always)]
    pub fn iom1ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc14::Iom1ce3)
    }
    #[doc = "IOM 2 NCE 0 module"]
    #[inline(always)]
    pub fn iom2ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc14::Iom2ce0)
    }
    #[doc = "IOM 2 NCE 1 module"]
    #[inline(always)]
    pub fn iom2ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc14::Iom2ce1)
    }
    #[doc = "IOM 2 NCE 2 module"]
    #[inline(always)]
    pub fn iom2ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc14::Iom2ce2)
    }
    #[doc = "IOM 2 NCE 3 module"]
    #[inline(always)]
    pub fn iom2ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc14::Iom2ce3)
    }
    #[doc = "IOM 3 NCE 0 module"]
    #[inline(always)]
    pub fn iom3ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc14::Iom3ce0)
    }
    #[doc = "IOM 3 NCE 1 module"]
    #[inline(always)]
    pub fn iom3ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc14::Iom3ce1)
    }
    #[doc = "IOM 3 NCE 2 module"]
    #[inline(always)]
    pub fn iom3ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc14::Iom3ce2)
    }
    #[doc = "IOM 3 NCE 3 module"]
    #[inline(always)]
    pub fn iom3ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc14::Iom3ce3)
    }
    #[doc = "IOM 4 NCE 0 module"]
    #[inline(always)]
    pub fn iom4ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc14::Iom4ce0)
    }
    #[doc = "IOM 4 NCE 1 module"]
    #[inline(always)]
    pub fn iom4ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc14::Iom4ce1)
    }
    #[doc = "IOM 4 NCE 2 module"]
    #[inline(always)]
    pub fn iom4ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc14::Iom4ce2)
    }
    #[doc = "IOM 4 NCE 3 module"]
    #[inline(always)]
    pub fn iom4ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc14::Iom4ce3)
    }
    #[doc = "IOM 5 NCE 0 module"]
    #[inline(always)]
    pub fn iom5ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc14::Iom5ce0)
    }
    #[doc = "IOM 5 NCE 1 module"]
    #[inline(always)]
    pub fn iom5ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc14::Iom5ce1)
    }
    #[doc = "IOM 5 NCE 2 module"]
    #[inline(always)]
    pub fn iom5ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc14::Iom5ce2)
    }
    #[doc = "IOM 5 NCE 3 module"]
    #[inline(always)]
    pub fn iom5ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc14::Iom5ce3)
    }
    #[doc = "IOM 6 NCE 0 module"]
    #[inline(always)]
    pub fn iom6ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc14::Iom6ce0)
    }
    #[doc = "IOM 6 NCE 1 module"]
    #[inline(always)]
    pub fn iom6ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc14::Iom6ce1)
    }
    #[doc = "IOM 6 NCE 2 module"]
    #[inline(always)]
    pub fn iom6ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc14::Iom6ce2)
    }
    #[doc = "IOM 6 NCE 3 module"]
    #[inline(always)]
    pub fn iom6ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc14::Iom6ce3)
    }
    #[doc = "IOM 7 NCE 0 module"]
    #[inline(always)]
    pub fn iom7ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc14::Iom7ce0)
    }
    #[doc = "IOM 7 NCE 1 module"]
    #[inline(always)]
    pub fn iom7ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc14::Iom7ce1)
    }
    #[doc = "IOM 7 NCE 2 module"]
    #[inline(always)]
    pub fn iom7ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc14::Iom7ce2)
    }
    #[doc = "IOM 7 NCE 3 module"]
    #[inline(always)]
    pub fn iom7ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc14::Iom7ce3)
    }
    #[doc = "MSPI 0 NCE 0 module"]
    #[inline(always)]
    pub fn mspi0cen0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc14::Mspi0cen0)
    }
    #[doc = "MSPI 0 NCE 1 module"]
    #[inline(always)]
    pub fn mspi0cen1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc14::Mspi0cen1)
    }
    #[doc = "MSPI 1 NCE 0 module"]
    #[inline(always)]
    pub fn mspi1cen0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc14::Mspi1cen0)
    }
    #[doc = "MSPI 1 NCE 1 module"]
    #[inline(always)]
    pub fn mspi1cen1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc14::Mspi1cen1)
    }
    #[doc = "MSPI 2 NCE 0 module"]
    #[inline(always)]
    pub fn mspi2cen0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc14::Mspi2cen0)
    }
    #[doc = "MSPI 2 NCE 1 module"]
    #[inline(always)]
    pub fn mspi2cen1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc14::Mspi2cen1)
    }
    #[doc = "DC DPI DE module"]
    #[inline(always)]
    pub fn dc_dpi_de(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc14::DcDpiDe)
    }
    #[doc = "DISP CONT CSX module"]
    #[inline(always)]
    pub fn disp_cont_csx(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc14::DispContCsx)
    }
    #[doc = "DC SPI CS_N module"]
    #[inline(always)]
    pub fn dc_spi_cs_n(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc14::DcSpiCsN)
    }
    #[doc = "DC QSPI CS_N module"]
    #[inline(always)]
    pub fn dc_qspi_cs_n(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc14::DcQspiCsN)
    }
    #[doc = "DC module RESX"]
    #[inline(always)]
    pub fn dc_resx(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc14::DcResx)
    }
}
#[doc = "Polarity select for NCE for GPIO 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ncepol14 {
    #[doc = "0: Polarity is active low"]
    Low = 0,
    #[doc = "1: Polarity is active high"]
    High = 1,
}
impl From<Ncepol14> for bool {
    #[inline(always)]
    fn from(variant: Ncepol14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NCEPOL14` reader - Polarity select for NCE for GPIO 14"]
pub type Ncepol14R = crate::BitReader<Ncepol14>;
impl Ncepol14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ncepol14 {
        match self.bits {
            false => Ncepol14::Low,
            true => Ncepol14::High,
        }
    }
    #[doc = "Polarity is active low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Ncepol14::Low
    }
    #[doc = "Polarity is active high"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Ncepol14::High
    }
}
#[doc = "Field `NCEPOL14` writer - Polarity select for NCE for GPIO 14"]
pub type Ncepol14W<'a, REG> = crate::BitWriter<'a, REG, Ncepol14>;
impl<'a, REG> Ncepol14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Polarity is active low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Ncepol14::Low)
    }
    #[doc = "Polarity is active high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Ncepol14::High)
    }
}
#[doc = "Field `FIEN14` reader - Force input enable active regardless of function selected. Otherwise the selected function will enable the input only when needed"]
pub type Fien14R = crate::BitReader;
#[doc = "Field `FIEN14` writer - Force input enable active regardless of function selected. Otherwise the selected function will enable the input only when needed"]
pub type Fien14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FOEN14` reader - Force output enable active regardless of function selected. Otherwise the selected function will enable the output only when needed"]
pub type Foen14R = crate::BitReader;
#[doc = "Field `FOEN14` writer - Force output enable active regardless of function selected. Otherwise the selected function will enable the output only when needed"]
pub type Foen14W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Function select for GPIO pin 14"]
    #[inline(always)]
    pub fn fncsel14(&self) -> Fncsel14R {
        Fncsel14R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Input enable for GPIO 14"]
    #[inline(always)]
    pub fn inpen14(&self) -> Inpen14R {
        Inpen14R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Return 0 for read data on GPIO 14"]
    #[inline(always)]
    pub fn rdzero14(&self) -> Rdzero14R {
        Rdzero14R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Interrupt enable for GPIO 14"]
    #[inline(always)]
    pub fn irpten14(&self) -> Irpten14R {
        Irpten14R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Pin IO mode selection for GPIO pin 14"]
    #[inline(always)]
    pub fn outcfg14(&self) -> Outcfg14R {
        Outcfg14R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Drive strength selection for GPIO 14"]
    #[inline(always)]
    pub fn ds14(&self) -> Ds14R {
        Ds14R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - Configure the slew rate"]
    #[inline(always)]
    pub fn sr14(&self) -> Sr14R {
        Sr14R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - Pullup/Pulldown configuration for GPIO 14"]
    #[inline(always)]
    pub fn pullcfg14(&self) -> Pullcfg14R {
        Pullcfg14R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:21 - IOMSTR/MSPI N Chip Select 14, DISP control signals DE, CSX, and CS. Polarity is determined by CE_POLARITY field"]
    #[inline(always)]
    pub fn ncesrc14(&self) -> Ncesrc14R {
        Ncesrc14R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 22 - Polarity select for NCE for GPIO 14"]
    #[inline(always)]
    pub fn ncepol14(&self) -> Ncepol14R {
        Ncepol14R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 26 - Force input enable active regardless of function selected. Otherwise the selected function will enable the input only when needed"]
    #[inline(always)]
    pub fn fien14(&self) -> Fien14R {
        Fien14R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Force output enable active regardless of function selected. Otherwise the selected function will enable the output only when needed"]
    #[inline(always)]
    pub fn foen14(&self) -> Foen14R {
        Foen14R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Function select for GPIO pin 14"]
    #[inline(always)]
    #[must_use]
    pub fn fncsel14(&mut self) -> Fncsel14W<Pincfg14Spec> {
        Fncsel14W::new(self, 0)
    }
    #[doc = "Bit 4 - Input enable for GPIO 14"]
    #[inline(always)]
    #[must_use]
    pub fn inpen14(&mut self) -> Inpen14W<Pincfg14Spec> {
        Inpen14W::new(self, 4)
    }
    #[doc = "Bit 5 - Return 0 for read data on GPIO 14"]
    #[inline(always)]
    #[must_use]
    pub fn rdzero14(&mut self) -> Rdzero14W<Pincfg14Spec> {
        Rdzero14W::new(self, 5)
    }
    #[doc = "Bits 6:7 - Interrupt enable for GPIO 14"]
    #[inline(always)]
    #[must_use]
    pub fn irpten14(&mut self) -> Irpten14W<Pincfg14Spec> {
        Irpten14W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Pin IO mode selection for GPIO pin 14"]
    #[inline(always)]
    #[must_use]
    pub fn outcfg14(&mut self) -> Outcfg14W<Pincfg14Spec> {
        Outcfg14W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Drive strength selection for GPIO 14"]
    #[inline(always)]
    #[must_use]
    pub fn ds14(&mut self) -> Ds14W<Pincfg14Spec> {
        Ds14W::new(self, 10)
    }
    #[doc = "Bit 12 - Configure the slew rate"]
    #[inline(always)]
    #[must_use]
    pub fn sr14(&mut self) -> Sr14W<Pincfg14Spec> {
        Sr14W::new(self, 12)
    }
    #[doc = "Bits 13:15 - Pullup/Pulldown configuration for GPIO 14"]
    #[inline(always)]
    #[must_use]
    pub fn pullcfg14(&mut self) -> Pullcfg14W<Pincfg14Spec> {
        Pullcfg14W::new(self, 13)
    }
    #[doc = "Bits 16:21 - IOMSTR/MSPI N Chip Select 14, DISP control signals DE, CSX, and CS. Polarity is determined by CE_POLARITY field"]
    #[inline(always)]
    #[must_use]
    pub fn ncesrc14(&mut self) -> Ncesrc14W<Pincfg14Spec> {
        Ncesrc14W::new(self, 16)
    }
    #[doc = "Bit 22 - Polarity select for NCE for GPIO 14"]
    #[inline(always)]
    #[must_use]
    pub fn ncepol14(&mut self) -> Ncepol14W<Pincfg14Spec> {
        Ncepol14W::new(self, 22)
    }
    #[doc = "Bit 26 - Force input enable active regardless of function selected. Otherwise the selected function will enable the input only when needed"]
    #[inline(always)]
    #[must_use]
    pub fn fien14(&mut self) -> Fien14W<Pincfg14Spec> {
        Fien14W::new(self, 26)
    }
    #[doc = "Bit 27 - Force output enable active regardless of function selected. Otherwise the selected function will enable the output only when needed"]
    #[inline(always)]
    #[must_use]
    pub fn foen14(&mut self) -> Foen14W<Pincfg14Spec> {
        Foen14W::new(self, 27)
    }
}
#[doc = "Controls the operation of GPIO pin 14.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg14::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg14::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pincfg14Spec;
impl crate::RegisterSpec for Pincfg14Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pincfg14::R`](R) reader structure"]
impl crate::Readable for Pincfg14Spec {}
#[doc = "`write(|w| ..)` method takes [`pincfg14::W`](W) writer structure"]
impl crate::Writable for Pincfg14Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PINCFG14 to value 0x03"]
impl crate::Resettable for Pincfg14Spec {
    const RESET_VALUE: u32 = 0x03;
}
