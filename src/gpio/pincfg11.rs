#[doc = "Register `PINCFG11` reader"]
pub type R = crate::R<Pincfg11Spec>;
#[doc = "Register `PINCFG11` writer"]
pub type W = crate::W<Pincfg11Spec>;
#[doc = "Function select for GPIO pin 11\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fncsel11 {
    #[doc = "0: Voltage comparator input 1"]
    Cmpin1 = 0,
    #[doc = "1: ADC trigger input"]
    Trig0 = 1,
    #[doc = "2: Bidirectional I2S Bit clock. Operates in output mode in master mode and input mode for slave mode. (I2S Master/Slave 2)"]
    I2s0Clk = 2,
    #[doc = "3: General purpose I/O"]
    Gpio = 3,
    #[doc = "4: UART receive input (UART 2)"]
    Uart2rx = 4,
    #[doc = "5: UART receive input (UART 3)"]
    Uart3rx = 5,
    #[doc = "6: Timer/Counter input or output; Selection of direction is done via CTIMER register settings."]
    Ct11 = 6,
    #[doc = "7: IOMSTR/MSPI N Chip Select. Polarity is determined by CE_POLARITY field"]
    Nce11 = 7,
    #[doc = "8: Observation bus bit 11"]
    Obsbus11 = 8,
    #[doc = "9: Reserved selection. Operation unknown if selected."]
    Reserved9 = 9,
    #[doc = "10: Reserved selection. Operation unknown if selected."]
    Reserved10 = 10,
    #[doc = "11: Fast PIO"]
    Fpio = 11,
    #[doc = "12: Internal function (Flash Bist)"]
    FlbTclk = 12,
    #[doc = "13: Internal function (Flash parallel load)"]
    FlloadAddr = 13,
    #[doc = "14: Internal function (MBIST)"]
    MdaTck = 14,
    #[doc = "15: Internal function (SCAN)"]
    Scanin0 = 15,
}
impl From<Fncsel11> for u8 {
    #[inline(always)]
    fn from(variant: Fncsel11) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fncsel11 {
    type Ux = u8;
}
impl crate::IsEnum for Fncsel11 {}
#[doc = "Field `FNCSEL11` reader - Function select for GPIO pin 11"]
pub type Fncsel11R = crate::FieldReader<Fncsel11>;
impl Fncsel11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fncsel11 {
        match self.bits {
            0 => Fncsel11::Cmpin1,
            1 => Fncsel11::Trig0,
            2 => Fncsel11::I2s0Clk,
            3 => Fncsel11::Gpio,
            4 => Fncsel11::Uart2rx,
            5 => Fncsel11::Uart3rx,
            6 => Fncsel11::Ct11,
            7 => Fncsel11::Nce11,
            8 => Fncsel11::Obsbus11,
            9 => Fncsel11::Reserved9,
            10 => Fncsel11::Reserved10,
            11 => Fncsel11::Fpio,
            12 => Fncsel11::FlbTclk,
            13 => Fncsel11::FlloadAddr,
            14 => Fncsel11::MdaTck,
            15 => Fncsel11::Scanin0,
            _ => unreachable!(),
        }
    }
    #[doc = "Voltage comparator input 1"]
    #[inline(always)]
    pub fn is_cmpin1(&self) -> bool {
        *self == Fncsel11::Cmpin1
    }
    #[doc = "ADC trigger input"]
    #[inline(always)]
    pub fn is_trig0(&self) -> bool {
        *self == Fncsel11::Trig0
    }
    #[doc = "Bidirectional I2S Bit clock. Operates in output mode in master mode and input mode for slave mode. (I2S Master/Slave 2)"]
    #[inline(always)]
    pub fn is_i2s0_clk(&self) -> bool {
        *self == Fncsel11::I2s0Clk
    }
    #[doc = "General purpose I/O"]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == Fncsel11::Gpio
    }
    #[doc = "UART receive input (UART 2)"]
    #[inline(always)]
    pub fn is_uart2rx(&self) -> bool {
        *self == Fncsel11::Uart2rx
    }
    #[doc = "UART receive input (UART 3)"]
    #[inline(always)]
    pub fn is_uart3rx(&self) -> bool {
        *self == Fncsel11::Uart3rx
    }
    #[doc = "Timer/Counter input or output; Selection of direction is done via CTIMER register settings."]
    #[inline(always)]
    pub fn is_ct11(&self) -> bool {
        *self == Fncsel11::Ct11
    }
    #[doc = "IOMSTR/MSPI N Chip Select. Polarity is determined by CE_POLARITY field"]
    #[inline(always)]
    pub fn is_nce11(&self) -> bool {
        *self == Fncsel11::Nce11
    }
    #[doc = "Observation bus bit 11"]
    #[inline(always)]
    pub fn is_obsbus11(&self) -> bool {
        *self == Fncsel11::Obsbus11
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved9(&self) -> bool {
        *self == Fncsel11::Reserved9
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved10(&self) -> bool {
        *self == Fncsel11::Reserved10
    }
    #[doc = "Fast PIO"]
    #[inline(always)]
    pub fn is_fpio(&self) -> bool {
        *self == Fncsel11::Fpio
    }
    #[doc = "Internal function (Flash Bist)"]
    #[inline(always)]
    pub fn is_flb_tclk(&self) -> bool {
        *self == Fncsel11::FlbTclk
    }
    #[doc = "Internal function (Flash parallel load)"]
    #[inline(always)]
    pub fn is_flload_addr(&self) -> bool {
        *self == Fncsel11::FlloadAddr
    }
    #[doc = "Internal function (MBIST)"]
    #[inline(always)]
    pub fn is_mda_tck(&self) -> bool {
        *self == Fncsel11::MdaTck
    }
    #[doc = "Internal function (SCAN)"]
    #[inline(always)]
    pub fn is_scanin0(&self) -> bool {
        *self == Fncsel11::Scanin0
    }
}
#[doc = "Field `FNCSEL11` writer - Function select for GPIO pin 11"]
pub type Fncsel11W<'a, REG> = crate::FieldWriter<'a, REG, 4, Fncsel11, crate::Safe>;
impl<'a, REG> Fncsel11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Voltage comparator input 1"]
    #[inline(always)]
    pub fn cmpin1(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel11::Cmpin1)
    }
    #[doc = "ADC trigger input"]
    #[inline(always)]
    pub fn trig0(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel11::Trig0)
    }
    #[doc = "Bidirectional I2S Bit clock. Operates in output mode in master mode and input mode for slave mode. (I2S Master/Slave 2)"]
    #[inline(always)]
    pub fn i2s0_clk(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel11::I2s0Clk)
    }
    #[doc = "General purpose I/O"]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel11::Gpio)
    }
    #[doc = "UART receive input (UART 2)"]
    #[inline(always)]
    pub fn uart2rx(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel11::Uart2rx)
    }
    #[doc = "UART receive input (UART 3)"]
    #[inline(always)]
    pub fn uart3rx(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel11::Uart3rx)
    }
    #[doc = "Timer/Counter input or output; Selection of direction is done via CTIMER register settings."]
    #[inline(always)]
    pub fn ct11(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel11::Ct11)
    }
    #[doc = "IOMSTR/MSPI N Chip Select. Polarity is determined by CE_POLARITY field"]
    #[inline(always)]
    pub fn nce11(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel11::Nce11)
    }
    #[doc = "Observation bus bit 11"]
    #[inline(always)]
    pub fn obsbus11(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel11::Obsbus11)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved9(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel11::Reserved9)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved10(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel11::Reserved10)
    }
    #[doc = "Fast PIO"]
    #[inline(always)]
    pub fn fpio(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel11::Fpio)
    }
    #[doc = "Internal function (Flash Bist)"]
    #[inline(always)]
    pub fn flb_tclk(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel11::FlbTclk)
    }
    #[doc = "Internal function (Flash parallel load)"]
    #[inline(always)]
    pub fn flload_addr(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel11::FlloadAddr)
    }
    #[doc = "Internal function (MBIST)"]
    #[inline(always)]
    pub fn mda_tck(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel11::MdaTck)
    }
    #[doc = "Internal function (SCAN)"]
    #[inline(always)]
    pub fn scanin0(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel11::Scanin0)
    }
}
#[doc = "Field `INPEN11` reader - Input enable for GPIO 11"]
pub type Inpen11R = crate::BitReader;
#[doc = "Field `INPEN11` writer - Input enable for GPIO 11"]
pub type Inpen11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDZERO11` reader - Return 0 for read data on GPIO 11"]
pub type Rdzero11R = crate::BitReader;
#[doc = "Field `RDZERO11` writer - Return 0 for read data on GPIO 11"]
pub type Rdzero11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Interrupt enable for GPIO 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Irpten11 {
    #[doc = "0: Interrupts are disabled for this GPIO"]
    Dis = 0,
    #[doc = "1: Interrupts are enabled for falling edge transition on this GPIO"]
    Intfall = 1,
    #[doc = "2: Interrupts are enabled for rising edge transitions on this GPIO"]
    Intrise = 2,
    #[doc = "3: Interrupts are enabled for any edge transition on this GPIO"]
    Intany = 3,
}
impl From<Irpten11> for u8 {
    #[inline(always)]
    fn from(variant: Irpten11) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Irpten11 {
    type Ux = u8;
}
impl crate::IsEnum for Irpten11 {}
#[doc = "Field `IRPTEN11` reader - Interrupt enable for GPIO 11"]
pub type Irpten11R = crate::FieldReader<Irpten11>;
impl Irpten11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irpten11 {
        match self.bits {
            0 => Irpten11::Dis,
            1 => Irpten11::Intfall,
            2 => Irpten11::Intrise,
            3 => Irpten11::Intany,
            _ => unreachable!(),
        }
    }
    #[doc = "Interrupts are disabled for this GPIO"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Irpten11::Dis
    }
    #[doc = "Interrupts are enabled for falling edge transition on this GPIO"]
    #[inline(always)]
    pub fn is_intfall(&self) -> bool {
        *self == Irpten11::Intfall
    }
    #[doc = "Interrupts are enabled for rising edge transitions on this GPIO"]
    #[inline(always)]
    pub fn is_intrise(&self) -> bool {
        *self == Irpten11::Intrise
    }
    #[doc = "Interrupts are enabled for any edge transition on this GPIO"]
    #[inline(always)]
    pub fn is_intany(&self) -> bool {
        *self == Irpten11::Intany
    }
}
#[doc = "Field `IRPTEN11` writer - Interrupt enable for GPIO 11"]
pub type Irpten11W<'a, REG> = crate::FieldWriter<'a, REG, 2, Irpten11, crate::Safe>;
impl<'a, REG> Irpten11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Interrupts are disabled for this GPIO"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten11::Dis)
    }
    #[doc = "Interrupts are enabled for falling edge transition on this GPIO"]
    #[inline(always)]
    pub fn intfall(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten11::Intfall)
    }
    #[doc = "Interrupts are enabled for rising edge transitions on this GPIO"]
    #[inline(always)]
    pub fn intrise(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten11::Intrise)
    }
    #[doc = "Interrupts are enabled for any edge transition on this GPIO"]
    #[inline(always)]
    pub fn intany(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten11::Intany)
    }
}
#[doc = "Pin IO mode selection for GPIO pin 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Outcfg11 {
    #[doc = "0: Output Disabled"]
    Dis = 0,
    #[doc = "1: Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    Pushpull = 1,
    #[doc = "2: Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    Od = 2,
    #[doc = "3: Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    Ts = 3,
}
impl From<Outcfg11> for u8 {
    #[inline(always)]
    fn from(variant: Outcfg11) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Outcfg11 {
    type Ux = u8;
}
impl crate::IsEnum for Outcfg11 {}
#[doc = "Field `OUTCFG11` reader - Pin IO mode selection for GPIO pin 11"]
pub type Outcfg11R = crate::FieldReader<Outcfg11>;
impl Outcfg11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Outcfg11 {
        match self.bits {
            0 => Outcfg11::Dis,
            1 => Outcfg11::Pushpull,
            2 => Outcfg11::Od,
            3 => Outcfg11::Ts,
            _ => unreachable!(),
        }
    }
    #[doc = "Output Disabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Outcfg11::Dis
    }
    #[doc = "Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == Outcfg11::Pushpull
    }
    #[doc = "Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == Outcfg11::Od
    }
    #[doc = "Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == Outcfg11::Ts
    }
}
#[doc = "Field `OUTCFG11` writer - Pin IO mode selection for GPIO pin 11"]
pub type Outcfg11W<'a, REG> = crate::FieldWriter<'a, REG, 2, Outcfg11, crate::Safe>;
impl<'a, REG> Outcfg11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Output Disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg11::Dis)
    }
    #[doc = "Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg11::Pushpull)
    }
    #[doc = "Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    #[inline(always)]
    pub fn od(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg11::Od)
    }
    #[doc = "Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    #[inline(always)]
    pub fn ts(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg11::Ts)
    }
}
#[doc = "Drive strength selection for GPIO 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ds11 {
    #[doc = "0: 0.1x output driver selected"]
    _0p1x = 0,
    #[doc = "1: 0.5x output driver selected"]
    _0p5x = 1,
}
impl From<Ds11> for u8 {
    #[inline(always)]
    fn from(variant: Ds11) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ds11 {
    type Ux = u8;
}
impl crate::IsEnum for Ds11 {}
#[doc = "Field `DS11` reader - Drive strength selection for GPIO 11"]
pub type Ds11R = crate::FieldReader<Ds11>;
impl Ds11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ds11> {
        match self.bits {
            0 => Some(Ds11::_0p1x),
            1 => Some(Ds11::_0p5x),
            _ => None,
        }
    }
    #[doc = "0.1x output driver selected"]
    #[inline(always)]
    pub fn is_0p1x(&self) -> bool {
        *self == Ds11::_0p1x
    }
    #[doc = "0.5x output driver selected"]
    #[inline(always)]
    pub fn is_0p5x(&self) -> bool {
        *self == Ds11::_0p5x
    }
}
#[doc = "Field `DS11` writer - Drive strength selection for GPIO 11"]
pub type Ds11W<'a, REG> = crate::FieldWriter<'a, REG, 2, Ds11>;
impl<'a, REG> Ds11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "0.1x output driver selected"]
    #[inline(always)]
    pub fn _0p1x(self) -> &'a mut crate::W<REG> {
        self.variant(Ds11::_0p1x)
    }
    #[doc = "0.5x output driver selected"]
    #[inline(always)]
    pub fn _0p5x(self) -> &'a mut crate::W<REG> {
        self.variant(Ds11::_0p5x)
    }
}
#[doc = "Field `SR11` reader - Configure the slew rate"]
pub type Sr11R = crate::BitReader;
#[doc = "Field `SR11` writer - Configure the slew rate"]
pub type Sr11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Pullup/Pulldown configuration for GPIO 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pullcfg11 {
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
impl From<Pullcfg11> for u8 {
    #[inline(always)]
    fn from(variant: Pullcfg11) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pullcfg11 {
    type Ux = u8;
}
impl crate::IsEnum for Pullcfg11 {}
#[doc = "Field `PULLCFG11` reader - Pullup/Pulldown configuration for GPIO 11"]
pub type Pullcfg11R = crate::FieldReader<Pullcfg11>;
impl Pullcfg11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pullcfg11 {
        match self.bits {
            0 => Pullcfg11::Dis,
            1 => Pullcfg11::Pd50k,
            2 => Pullcfg11::Pu15k,
            3 => Pullcfg11::Pu6k,
            4 => Pullcfg11::Pu12k,
            5 => Pullcfg11::Pu24k,
            6 => Pullcfg11::Pu50k,
            7 => Pullcfg11::Pu100k,
            _ => unreachable!(),
        }
    }
    #[doc = "No pullup or pulldown selected"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Pullcfg11::Dis
    }
    #[doc = "50K Pulldown selected"]
    #[inline(always)]
    pub fn is_pd50k(&self) -> bool {
        *self == Pullcfg11::Pd50k
    }
    #[doc = "1.5K Pullup selected"]
    #[inline(always)]
    pub fn is_pu15k(&self) -> bool {
        *self == Pullcfg11::Pu15k
    }
    #[doc = "6K Pullup selected"]
    #[inline(always)]
    pub fn is_pu6k(&self) -> bool {
        *self == Pullcfg11::Pu6k
    }
    #[doc = "12K Pullup selected"]
    #[inline(always)]
    pub fn is_pu12k(&self) -> bool {
        *self == Pullcfg11::Pu12k
    }
    #[doc = "24K Pullup selected"]
    #[inline(always)]
    pub fn is_pu24k(&self) -> bool {
        *self == Pullcfg11::Pu24k
    }
    #[doc = "50K Pullup selected"]
    #[inline(always)]
    pub fn is_pu50k(&self) -> bool {
        *self == Pullcfg11::Pu50k
    }
    #[doc = "100K Pullup selected"]
    #[inline(always)]
    pub fn is_pu100k(&self) -> bool {
        *self == Pullcfg11::Pu100k
    }
}
#[doc = "Field `PULLCFG11` writer - Pullup/Pulldown configuration for GPIO 11"]
pub type Pullcfg11W<'a, REG> = crate::FieldWriter<'a, REG, 3, Pullcfg11, crate::Safe>;
impl<'a, REG> Pullcfg11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No pullup or pulldown selected"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg11::Dis)
    }
    #[doc = "50K Pulldown selected"]
    #[inline(always)]
    pub fn pd50k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg11::Pd50k)
    }
    #[doc = "1.5K Pullup selected"]
    #[inline(always)]
    pub fn pu15k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg11::Pu15k)
    }
    #[doc = "6K Pullup selected"]
    #[inline(always)]
    pub fn pu6k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg11::Pu6k)
    }
    #[doc = "12K Pullup selected"]
    #[inline(always)]
    pub fn pu12k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg11::Pu12k)
    }
    #[doc = "24K Pullup selected"]
    #[inline(always)]
    pub fn pu24k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg11::Pu24k)
    }
    #[doc = "50K Pullup selected"]
    #[inline(always)]
    pub fn pu50k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg11::Pu50k)
    }
    #[doc = "100K Pullup selected"]
    #[inline(always)]
    pub fn pu100k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg11::Pu100k)
    }
}
#[doc = "IOMSTR/MSPI N Chip Select 11, DISP control signals DE, CSX, and CS. Polarity is determined by CE_POLARITY field\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ncesrc11 {
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
impl From<Ncesrc11> for u8 {
    #[inline(always)]
    fn from(variant: Ncesrc11) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ncesrc11 {
    type Ux = u8;
}
impl crate::IsEnum for Ncesrc11 {}
#[doc = "Field `NCESRC11` reader - IOMSTR/MSPI N Chip Select 11, DISP control signals DE, CSX, and CS. Polarity is determined by CE_POLARITY field"]
pub type Ncesrc11R = crate::FieldReader<Ncesrc11>;
impl Ncesrc11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ncesrc11> {
        match self.bits {
            0 => Some(Ncesrc11::Iom0ce0),
            1 => Some(Ncesrc11::Iom0ce1),
            2 => Some(Ncesrc11::Iom0ce2),
            3 => Some(Ncesrc11::Iom0ce3),
            4 => Some(Ncesrc11::Iom1ce0),
            5 => Some(Ncesrc11::Iom1ce1),
            6 => Some(Ncesrc11::Iom1ce2),
            7 => Some(Ncesrc11::Iom1ce3),
            8 => Some(Ncesrc11::Iom2ce0),
            9 => Some(Ncesrc11::Iom2ce1),
            10 => Some(Ncesrc11::Iom2ce2),
            11 => Some(Ncesrc11::Iom2ce3),
            12 => Some(Ncesrc11::Iom3ce0),
            13 => Some(Ncesrc11::Iom3ce1),
            14 => Some(Ncesrc11::Iom3ce2),
            15 => Some(Ncesrc11::Iom3ce3),
            16 => Some(Ncesrc11::Iom4ce0),
            17 => Some(Ncesrc11::Iom4ce1),
            18 => Some(Ncesrc11::Iom4ce2),
            19 => Some(Ncesrc11::Iom4ce3),
            20 => Some(Ncesrc11::Iom5ce0),
            21 => Some(Ncesrc11::Iom5ce1),
            22 => Some(Ncesrc11::Iom5ce2),
            23 => Some(Ncesrc11::Iom5ce3),
            24 => Some(Ncesrc11::Iom6ce0),
            25 => Some(Ncesrc11::Iom6ce1),
            26 => Some(Ncesrc11::Iom6ce2),
            27 => Some(Ncesrc11::Iom6ce3),
            28 => Some(Ncesrc11::Iom7ce0),
            29 => Some(Ncesrc11::Iom7ce1),
            30 => Some(Ncesrc11::Iom7ce2),
            31 => Some(Ncesrc11::Iom7ce3),
            32 => Some(Ncesrc11::Mspi0cen0),
            33 => Some(Ncesrc11::Mspi0cen1),
            34 => Some(Ncesrc11::Mspi1cen0),
            35 => Some(Ncesrc11::Mspi1cen1),
            36 => Some(Ncesrc11::Mspi2cen0),
            37 => Some(Ncesrc11::Mspi2cen1),
            38 => Some(Ncesrc11::DcDpiDe),
            39 => Some(Ncesrc11::DispContCsx),
            40 => Some(Ncesrc11::DcSpiCsN),
            41 => Some(Ncesrc11::DcQspiCsN),
            42 => Some(Ncesrc11::DcResx),
            _ => None,
        }
    }
    #[doc = "IOM 0 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom0ce0(&self) -> bool {
        *self == Ncesrc11::Iom0ce0
    }
    #[doc = "IOM 0 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom0ce1(&self) -> bool {
        *self == Ncesrc11::Iom0ce1
    }
    #[doc = "IOM 0 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom0ce2(&self) -> bool {
        *self == Ncesrc11::Iom0ce2
    }
    #[doc = "IOM 0 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom0ce3(&self) -> bool {
        *self == Ncesrc11::Iom0ce3
    }
    #[doc = "IOM 1 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom1ce0(&self) -> bool {
        *self == Ncesrc11::Iom1ce0
    }
    #[doc = "IOM 1 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom1ce1(&self) -> bool {
        *self == Ncesrc11::Iom1ce1
    }
    #[doc = "IOM 1 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom1ce2(&self) -> bool {
        *self == Ncesrc11::Iom1ce2
    }
    #[doc = "IOM 1 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom1ce3(&self) -> bool {
        *self == Ncesrc11::Iom1ce3
    }
    #[doc = "IOM 2 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom2ce0(&self) -> bool {
        *self == Ncesrc11::Iom2ce0
    }
    #[doc = "IOM 2 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom2ce1(&self) -> bool {
        *self == Ncesrc11::Iom2ce1
    }
    #[doc = "IOM 2 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom2ce2(&self) -> bool {
        *self == Ncesrc11::Iom2ce2
    }
    #[doc = "IOM 2 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom2ce3(&self) -> bool {
        *self == Ncesrc11::Iom2ce3
    }
    #[doc = "IOM 3 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom3ce0(&self) -> bool {
        *self == Ncesrc11::Iom3ce0
    }
    #[doc = "IOM 3 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom3ce1(&self) -> bool {
        *self == Ncesrc11::Iom3ce1
    }
    #[doc = "IOM 3 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom3ce2(&self) -> bool {
        *self == Ncesrc11::Iom3ce2
    }
    #[doc = "IOM 3 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom3ce3(&self) -> bool {
        *self == Ncesrc11::Iom3ce3
    }
    #[doc = "IOM 4 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom4ce0(&self) -> bool {
        *self == Ncesrc11::Iom4ce0
    }
    #[doc = "IOM 4 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom4ce1(&self) -> bool {
        *self == Ncesrc11::Iom4ce1
    }
    #[doc = "IOM 4 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom4ce2(&self) -> bool {
        *self == Ncesrc11::Iom4ce2
    }
    #[doc = "IOM 4 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom4ce3(&self) -> bool {
        *self == Ncesrc11::Iom4ce3
    }
    #[doc = "IOM 5 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom5ce0(&self) -> bool {
        *self == Ncesrc11::Iom5ce0
    }
    #[doc = "IOM 5 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom5ce1(&self) -> bool {
        *self == Ncesrc11::Iom5ce1
    }
    #[doc = "IOM 5 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom5ce2(&self) -> bool {
        *self == Ncesrc11::Iom5ce2
    }
    #[doc = "IOM 5 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom5ce3(&self) -> bool {
        *self == Ncesrc11::Iom5ce3
    }
    #[doc = "IOM 6 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom6ce0(&self) -> bool {
        *self == Ncesrc11::Iom6ce0
    }
    #[doc = "IOM 6 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom6ce1(&self) -> bool {
        *self == Ncesrc11::Iom6ce1
    }
    #[doc = "IOM 6 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom6ce2(&self) -> bool {
        *self == Ncesrc11::Iom6ce2
    }
    #[doc = "IOM 6 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom6ce3(&self) -> bool {
        *self == Ncesrc11::Iom6ce3
    }
    #[doc = "IOM 7 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom7ce0(&self) -> bool {
        *self == Ncesrc11::Iom7ce0
    }
    #[doc = "IOM 7 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom7ce1(&self) -> bool {
        *self == Ncesrc11::Iom7ce1
    }
    #[doc = "IOM 7 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom7ce2(&self) -> bool {
        *self == Ncesrc11::Iom7ce2
    }
    #[doc = "IOM 7 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom7ce3(&self) -> bool {
        *self == Ncesrc11::Iom7ce3
    }
    #[doc = "MSPI 0 NCE 0 module"]
    #[inline(always)]
    pub fn is_mspi0cen0(&self) -> bool {
        *self == Ncesrc11::Mspi0cen0
    }
    #[doc = "MSPI 0 NCE 1 module"]
    #[inline(always)]
    pub fn is_mspi0cen1(&self) -> bool {
        *self == Ncesrc11::Mspi0cen1
    }
    #[doc = "MSPI 1 NCE 0 module"]
    #[inline(always)]
    pub fn is_mspi1cen0(&self) -> bool {
        *self == Ncesrc11::Mspi1cen0
    }
    #[doc = "MSPI 1 NCE 1 module"]
    #[inline(always)]
    pub fn is_mspi1cen1(&self) -> bool {
        *self == Ncesrc11::Mspi1cen1
    }
    #[doc = "MSPI 2 NCE 0 module"]
    #[inline(always)]
    pub fn is_mspi2cen0(&self) -> bool {
        *self == Ncesrc11::Mspi2cen0
    }
    #[doc = "MSPI 2 NCE 1 module"]
    #[inline(always)]
    pub fn is_mspi2cen1(&self) -> bool {
        *self == Ncesrc11::Mspi2cen1
    }
    #[doc = "DC DPI DE module"]
    #[inline(always)]
    pub fn is_dc_dpi_de(&self) -> bool {
        *self == Ncesrc11::DcDpiDe
    }
    #[doc = "DISP CONT CSX module"]
    #[inline(always)]
    pub fn is_disp_cont_csx(&self) -> bool {
        *self == Ncesrc11::DispContCsx
    }
    #[doc = "DC SPI CS_N module"]
    #[inline(always)]
    pub fn is_dc_spi_cs_n(&self) -> bool {
        *self == Ncesrc11::DcSpiCsN
    }
    #[doc = "DC QSPI CS_N module"]
    #[inline(always)]
    pub fn is_dc_qspi_cs_n(&self) -> bool {
        *self == Ncesrc11::DcQspiCsN
    }
    #[doc = "DC module RESX"]
    #[inline(always)]
    pub fn is_dc_resx(&self) -> bool {
        *self == Ncesrc11::DcResx
    }
}
#[doc = "Field `NCESRC11` writer - IOMSTR/MSPI N Chip Select 11, DISP control signals DE, CSX, and CS. Polarity is determined by CE_POLARITY field"]
pub type Ncesrc11W<'a, REG> = crate::FieldWriter<'a, REG, 6, Ncesrc11>;
impl<'a, REG> Ncesrc11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "IOM 0 NCE 0 module"]
    #[inline(always)]
    pub fn iom0ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc11::Iom0ce0)
    }
    #[doc = "IOM 0 NCE 1 module"]
    #[inline(always)]
    pub fn iom0ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc11::Iom0ce1)
    }
    #[doc = "IOM 0 NCE 2 module"]
    #[inline(always)]
    pub fn iom0ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc11::Iom0ce2)
    }
    #[doc = "IOM 0 NCE 3 module"]
    #[inline(always)]
    pub fn iom0ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc11::Iom0ce3)
    }
    #[doc = "IOM 1 NCE 0 module"]
    #[inline(always)]
    pub fn iom1ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc11::Iom1ce0)
    }
    #[doc = "IOM 1 NCE 1 module"]
    #[inline(always)]
    pub fn iom1ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc11::Iom1ce1)
    }
    #[doc = "IOM 1 NCE 2 module"]
    #[inline(always)]
    pub fn iom1ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc11::Iom1ce2)
    }
    #[doc = "IOM 1 NCE 3 module"]
    #[inline(always)]
    pub fn iom1ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc11::Iom1ce3)
    }
    #[doc = "IOM 2 NCE 0 module"]
    #[inline(always)]
    pub fn iom2ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc11::Iom2ce0)
    }
    #[doc = "IOM 2 NCE 1 module"]
    #[inline(always)]
    pub fn iom2ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc11::Iom2ce1)
    }
    #[doc = "IOM 2 NCE 2 module"]
    #[inline(always)]
    pub fn iom2ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc11::Iom2ce2)
    }
    #[doc = "IOM 2 NCE 3 module"]
    #[inline(always)]
    pub fn iom2ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc11::Iom2ce3)
    }
    #[doc = "IOM 3 NCE 0 module"]
    #[inline(always)]
    pub fn iom3ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc11::Iom3ce0)
    }
    #[doc = "IOM 3 NCE 1 module"]
    #[inline(always)]
    pub fn iom3ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc11::Iom3ce1)
    }
    #[doc = "IOM 3 NCE 2 module"]
    #[inline(always)]
    pub fn iom3ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc11::Iom3ce2)
    }
    #[doc = "IOM 3 NCE 3 module"]
    #[inline(always)]
    pub fn iom3ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc11::Iom3ce3)
    }
    #[doc = "IOM 4 NCE 0 module"]
    #[inline(always)]
    pub fn iom4ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc11::Iom4ce0)
    }
    #[doc = "IOM 4 NCE 1 module"]
    #[inline(always)]
    pub fn iom4ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc11::Iom4ce1)
    }
    #[doc = "IOM 4 NCE 2 module"]
    #[inline(always)]
    pub fn iom4ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc11::Iom4ce2)
    }
    #[doc = "IOM 4 NCE 3 module"]
    #[inline(always)]
    pub fn iom4ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc11::Iom4ce3)
    }
    #[doc = "IOM 5 NCE 0 module"]
    #[inline(always)]
    pub fn iom5ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc11::Iom5ce0)
    }
    #[doc = "IOM 5 NCE 1 module"]
    #[inline(always)]
    pub fn iom5ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc11::Iom5ce1)
    }
    #[doc = "IOM 5 NCE 2 module"]
    #[inline(always)]
    pub fn iom5ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc11::Iom5ce2)
    }
    #[doc = "IOM 5 NCE 3 module"]
    #[inline(always)]
    pub fn iom5ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc11::Iom5ce3)
    }
    #[doc = "IOM 6 NCE 0 module"]
    #[inline(always)]
    pub fn iom6ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc11::Iom6ce0)
    }
    #[doc = "IOM 6 NCE 1 module"]
    #[inline(always)]
    pub fn iom6ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc11::Iom6ce1)
    }
    #[doc = "IOM 6 NCE 2 module"]
    #[inline(always)]
    pub fn iom6ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc11::Iom6ce2)
    }
    #[doc = "IOM 6 NCE 3 module"]
    #[inline(always)]
    pub fn iom6ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc11::Iom6ce3)
    }
    #[doc = "IOM 7 NCE 0 module"]
    #[inline(always)]
    pub fn iom7ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc11::Iom7ce0)
    }
    #[doc = "IOM 7 NCE 1 module"]
    #[inline(always)]
    pub fn iom7ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc11::Iom7ce1)
    }
    #[doc = "IOM 7 NCE 2 module"]
    #[inline(always)]
    pub fn iom7ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc11::Iom7ce2)
    }
    #[doc = "IOM 7 NCE 3 module"]
    #[inline(always)]
    pub fn iom7ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc11::Iom7ce3)
    }
    #[doc = "MSPI 0 NCE 0 module"]
    #[inline(always)]
    pub fn mspi0cen0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc11::Mspi0cen0)
    }
    #[doc = "MSPI 0 NCE 1 module"]
    #[inline(always)]
    pub fn mspi0cen1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc11::Mspi0cen1)
    }
    #[doc = "MSPI 1 NCE 0 module"]
    #[inline(always)]
    pub fn mspi1cen0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc11::Mspi1cen0)
    }
    #[doc = "MSPI 1 NCE 1 module"]
    #[inline(always)]
    pub fn mspi1cen1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc11::Mspi1cen1)
    }
    #[doc = "MSPI 2 NCE 0 module"]
    #[inline(always)]
    pub fn mspi2cen0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc11::Mspi2cen0)
    }
    #[doc = "MSPI 2 NCE 1 module"]
    #[inline(always)]
    pub fn mspi2cen1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc11::Mspi2cen1)
    }
    #[doc = "DC DPI DE module"]
    #[inline(always)]
    pub fn dc_dpi_de(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc11::DcDpiDe)
    }
    #[doc = "DISP CONT CSX module"]
    #[inline(always)]
    pub fn disp_cont_csx(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc11::DispContCsx)
    }
    #[doc = "DC SPI CS_N module"]
    #[inline(always)]
    pub fn dc_spi_cs_n(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc11::DcSpiCsN)
    }
    #[doc = "DC QSPI CS_N module"]
    #[inline(always)]
    pub fn dc_qspi_cs_n(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc11::DcQspiCsN)
    }
    #[doc = "DC module RESX"]
    #[inline(always)]
    pub fn dc_resx(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc11::DcResx)
    }
}
#[doc = "Polarity select for NCE for GPIO 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ncepol11 {
    #[doc = "0: Polarity is active low"]
    Low = 0,
    #[doc = "1: Polarity is active high"]
    High = 1,
}
impl From<Ncepol11> for bool {
    #[inline(always)]
    fn from(variant: Ncepol11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NCEPOL11` reader - Polarity select for NCE for GPIO 11"]
pub type Ncepol11R = crate::BitReader<Ncepol11>;
impl Ncepol11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ncepol11 {
        match self.bits {
            false => Ncepol11::Low,
            true => Ncepol11::High,
        }
    }
    #[doc = "Polarity is active low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Ncepol11::Low
    }
    #[doc = "Polarity is active high"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Ncepol11::High
    }
}
#[doc = "Field `NCEPOL11` writer - Polarity select for NCE for GPIO 11"]
pub type Ncepol11W<'a, REG> = crate::BitWriter<'a, REG, Ncepol11>;
impl<'a, REG> Ncepol11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Polarity is active low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Ncepol11::Low)
    }
    #[doc = "Polarity is active high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Ncepol11::High)
    }
}
#[doc = "Field `FIEN11` reader - Force input enable active regardless of function selected. Otherwise the selected function will enable the input only when needed"]
pub type Fien11R = crate::BitReader;
#[doc = "Field `FIEN11` writer - Force input enable active regardless of function selected. Otherwise the selected function will enable the input only when needed"]
pub type Fien11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FOEN11` reader - Force output enable active regardless of function selected. Otherwise the selected function will enable the output only when needed"]
pub type Foen11R = crate::BitReader;
#[doc = "Field `FOEN11` writer - Force output enable active regardless of function selected. Otherwise the selected function will enable the output only when needed"]
pub type Foen11W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Function select for GPIO pin 11"]
    #[inline(always)]
    pub fn fncsel11(&self) -> Fncsel11R {
        Fncsel11R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Input enable for GPIO 11"]
    #[inline(always)]
    pub fn inpen11(&self) -> Inpen11R {
        Inpen11R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Return 0 for read data on GPIO 11"]
    #[inline(always)]
    pub fn rdzero11(&self) -> Rdzero11R {
        Rdzero11R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Interrupt enable for GPIO 11"]
    #[inline(always)]
    pub fn irpten11(&self) -> Irpten11R {
        Irpten11R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Pin IO mode selection for GPIO pin 11"]
    #[inline(always)]
    pub fn outcfg11(&self) -> Outcfg11R {
        Outcfg11R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Drive strength selection for GPIO 11"]
    #[inline(always)]
    pub fn ds11(&self) -> Ds11R {
        Ds11R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - Configure the slew rate"]
    #[inline(always)]
    pub fn sr11(&self) -> Sr11R {
        Sr11R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - Pullup/Pulldown configuration for GPIO 11"]
    #[inline(always)]
    pub fn pullcfg11(&self) -> Pullcfg11R {
        Pullcfg11R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:21 - IOMSTR/MSPI N Chip Select 11, DISP control signals DE, CSX, and CS. Polarity is determined by CE_POLARITY field"]
    #[inline(always)]
    pub fn ncesrc11(&self) -> Ncesrc11R {
        Ncesrc11R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 22 - Polarity select for NCE for GPIO 11"]
    #[inline(always)]
    pub fn ncepol11(&self) -> Ncepol11R {
        Ncepol11R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 26 - Force input enable active regardless of function selected. Otherwise the selected function will enable the input only when needed"]
    #[inline(always)]
    pub fn fien11(&self) -> Fien11R {
        Fien11R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Force output enable active regardless of function selected. Otherwise the selected function will enable the output only when needed"]
    #[inline(always)]
    pub fn foen11(&self) -> Foen11R {
        Foen11R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Function select for GPIO pin 11"]
    #[inline(always)]
    #[must_use]
    pub fn fncsel11(&mut self) -> Fncsel11W<Pincfg11Spec> {
        Fncsel11W::new(self, 0)
    }
    #[doc = "Bit 4 - Input enable for GPIO 11"]
    #[inline(always)]
    #[must_use]
    pub fn inpen11(&mut self) -> Inpen11W<Pincfg11Spec> {
        Inpen11W::new(self, 4)
    }
    #[doc = "Bit 5 - Return 0 for read data on GPIO 11"]
    #[inline(always)]
    #[must_use]
    pub fn rdzero11(&mut self) -> Rdzero11W<Pincfg11Spec> {
        Rdzero11W::new(self, 5)
    }
    #[doc = "Bits 6:7 - Interrupt enable for GPIO 11"]
    #[inline(always)]
    #[must_use]
    pub fn irpten11(&mut self) -> Irpten11W<Pincfg11Spec> {
        Irpten11W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Pin IO mode selection for GPIO pin 11"]
    #[inline(always)]
    #[must_use]
    pub fn outcfg11(&mut self) -> Outcfg11W<Pincfg11Spec> {
        Outcfg11W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Drive strength selection for GPIO 11"]
    #[inline(always)]
    #[must_use]
    pub fn ds11(&mut self) -> Ds11W<Pincfg11Spec> {
        Ds11W::new(self, 10)
    }
    #[doc = "Bit 12 - Configure the slew rate"]
    #[inline(always)]
    #[must_use]
    pub fn sr11(&mut self) -> Sr11W<Pincfg11Spec> {
        Sr11W::new(self, 12)
    }
    #[doc = "Bits 13:15 - Pullup/Pulldown configuration for GPIO 11"]
    #[inline(always)]
    #[must_use]
    pub fn pullcfg11(&mut self) -> Pullcfg11W<Pincfg11Spec> {
        Pullcfg11W::new(self, 13)
    }
    #[doc = "Bits 16:21 - IOMSTR/MSPI N Chip Select 11, DISP control signals DE, CSX, and CS. Polarity is determined by CE_POLARITY field"]
    #[inline(always)]
    #[must_use]
    pub fn ncesrc11(&mut self) -> Ncesrc11W<Pincfg11Spec> {
        Ncesrc11W::new(self, 16)
    }
    #[doc = "Bit 22 - Polarity select for NCE for GPIO 11"]
    #[inline(always)]
    #[must_use]
    pub fn ncepol11(&mut self) -> Ncepol11W<Pincfg11Spec> {
        Ncepol11W::new(self, 22)
    }
    #[doc = "Bit 26 - Force input enable active regardless of function selected. Otherwise the selected function will enable the input only when needed"]
    #[inline(always)]
    #[must_use]
    pub fn fien11(&mut self) -> Fien11W<Pincfg11Spec> {
        Fien11W::new(self, 26)
    }
    #[doc = "Bit 27 - Force output enable active regardless of function selected. Otherwise the selected function will enable the output only when needed"]
    #[inline(always)]
    #[must_use]
    pub fn foen11(&mut self) -> Foen11W<Pincfg11Spec> {
        Foen11W::new(self, 27)
    }
}
#[doc = "Controls the operation of GPIO pin 11.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg11::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg11::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pincfg11Spec;
impl crate::RegisterSpec for Pincfg11Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pincfg11::R`](R) reader structure"]
impl crate::Readable for Pincfg11Spec {}
#[doc = "`write(|w| ..)` method takes [`pincfg11::W`](W) writer structure"]
impl crate::Writable for Pincfg11Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PINCFG11 to value 0x03"]
impl crate::Resettable for Pincfg11Spec {
    const RESET_VALUE: u32 = 0x03;
}
