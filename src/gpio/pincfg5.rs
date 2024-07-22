#[doc = "Register `PINCFG5` reader"]
pub type R = crate::R<Pincfg5Spec>;
#[doc = "Register `PINCFG5` writer"]
pub type W = crate::W<Pincfg5Spec>;
#[doc = "Function select for GPIO pin 5\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fncsel5 {
    #[doc = "0: Serial I2C Master Clock output (IOM 0)"]
    M0scl = 0,
    #[doc = "1: Serial SPI Master Clock output (IOM 0)"]
    M0sck = 1,
    #[doc = "2: Bidirectional I2S Bit clock. Operates in output mode in master mode and input mode for slave mode. (I2S Master/Slave 2)"]
    I2s0Clk = 2,
    #[doc = "3: General purpose I/O"]
    Gpio = 3,
    #[doc = "4: UART Request to Send (RTS) (UART 2)"]
    Uart2rts = 4,
    #[doc = "5: UART Request to Send (RTS) (UART 3)"]
    Uart3rts = 5,
    #[doc = "6: Timer/Counter input or output; Selection of direction is done via CTIMER register settings."]
    Ct5 = 6,
    #[doc = "7: IOMSTR/MSPI N Chip Select. Polarity is determined by CE_POLARITY field"]
    Nce5 = 7,
    #[doc = "8: Observation bus bit 5"]
    Obsbus5 = 8,
    #[doc = "9: Reserved selection. Operation unknown if selected."]
    Reserved9 = 9,
    #[doc = "10: Bidirectional I2S Bit clock. Operates in output mode in master mode and input mode for slave mode. (I2S Master/Slave 2)"]
    I2s1Clk = 10,
    #[doc = "11: Fast PIO"]
    Fpio = 11,
    #[doc = "12: Internal function (Flash Bist)"]
    FlbTdi = 12,
    #[doc = "13: Internal function (Flash parallel load)"]
    FlloadData = 13,
    #[doc = "14: Internal function (MBIST)"]
    MdaSrst = 14,
    #[doc = "15: Internal function (SCAN)"]
    DftIso = 15,
}
impl From<Fncsel5> for u8 {
    #[inline(always)]
    fn from(variant: Fncsel5) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fncsel5 {
    type Ux = u8;
}
impl crate::IsEnum for Fncsel5 {}
#[doc = "Field `FNCSEL5` reader - Function select for GPIO pin 5"]
pub type Fncsel5R = crate::FieldReader<Fncsel5>;
impl Fncsel5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fncsel5 {
        match self.bits {
            0 => Fncsel5::M0scl,
            1 => Fncsel5::M0sck,
            2 => Fncsel5::I2s0Clk,
            3 => Fncsel5::Gpio,
            4 => Fncsel5::Uart2rts,
            5 => Fncsel5::Uart3rts,
            6 => Fncsel5::Ct5,
            7 => Fncsel5::Nce5,
            8 => Fncsel5::Obsbus5,
            9 => Fncsel5::Reserved9,
            10 => Fncsel5::I2s1Clk,
            11 => Fncsel5::Fpio,
            12 => Fncsel5::FlbTdi,
            13 => Fncsel5::FlloadData,
            14 => Fncsel5::MdaSrst,
            15 => Fncsel5::DftIso,
            _ => unreachable!(),
        }
    }
    #[doc = "Serial I2C Master Clock output (IOM 0)"]
    #[inline(always)]
    pub fn is_m0scl(&self) -> bool {
        *self == Fncsel5::M0scl
    }
    #[doc = "Serial SPI Master Clock output (IOM 0)"]
    #[inline(always)]
    pub fn is_m0sck(&self) -> bool {
        *self == Fncsel5::M0sck
    }
    #[doc = "Bidirectional I2S Bit clock. Operates in output mode in master mode and input mode for slave mode. (I2S Master/Slave 2)"]
    #[inline(always)]
    pub fn is_i2s0_clk(&self) -> bool {
        *self == Fncsel5::I2s0Clk
    }
    #[doc = "General purpose I/O"]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == Fncsel5::Gpio
    }
    #[doc = "UART Request to Send (RTS) (UART 2)"]
    #[inline(always)]
    pub fn is_uart2rts(&self) -> bool {
        *self == Fncsel5::Uart2rts
    }
    #[doc = "UART Request to Send (RTS) (UART 3)"]
    #[inline(always)]
    pub fn is_uart3rts(&self) -> bool {
        *self == Fncsel5::Uart3rts
    }
    #[doc = "Timer/Counter input or output; Selection of direction is done via CTIMER register settings."]
    #[inline(always)]
    pub fn is_ct5(&self) -> bool {
        *self == Fncsel5::Ct5
    }
    #[doc = "IOMSTR/MSPI N Chip Select. Polarity is determined by CE_POLARITY field"]
    #[inline(always)]
    pub fn is_nce5(&self) -> bool {
        *self == Fncsel5::Nce5
    }
    #[doc = "Observation bus bit 5"]
    #[inline(always)]
    pub fn is_obsbus5(&self) -> bool {
        *self == Fncsel5::Obsbus5
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved9(&self) -> bool {
        *self == Fncsel5::Reserved9
    }
    #[doc = "Bidirectional I2S Bit clock. Operates in output mode in master mode and input mode for slave mode. (I2S Master/Slave 2)"]
    #[inline(always)]
    pub fn is_i2s1_clk(&self) -> bool {
        *self == Fncsel5::I2s1Clk
    }
    #[doc = "Fast PIO"]
    #[inline(always)]
    pub fn is_fpio(&self) -> bool {
        *self == Fncsel5::Fpio
    }
    #[doc = "Internal function (Flash Bist)"]
    #[inline(always)]
    pub fn is_flb_tdi(&self) -> bool {
        *self == Fncsel5::FlbTdi
    }
    #[doc = "Internal function (Flash parallel load)"]
    #[inline(always)]
    pub fn is_flload_data(&self) -> bool {
        *self == Fncsel5::FlloadData
    }
    #[doc = "Internal function (MBIST)"]
    #[inline(always)]
    pub fn is_mda_srst(&self) -> bool {
        *self == Fncsel5::MdaSrst
    }
    #[doc = "Internal function (SCAN)"]
    #[inline(always)]
    pub fn is_dft_iso(&self) -> bool {
        *self == Fncsel5::DftIso
    }
}
#[doc = "Field `FNCSEL5` writer - Function select for GPIO pin 5"]
pub type Fncsel5W<'a, REG> = crate::FieldWriter<'a, REG, 4, Fncsel5, crate::Safe>;
impl<'a, REG> Fncsel5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Serial I2C Master Clock output (IOM 0)"]
    #[inline(always)]
    pub fn m0scl(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel5::M0scl)
    }
    #[doc = "Serial SPI Master Clock output (IOM 0)"]
    #[inline(always)]
    pub fn m0sck(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel5::M0sck)
    }
    #[doc = "Bidirectional I2S Bit clock. Operates in output mode in master mode and input mode for slave mode. (I2S Master/Slave 2)"]
    #[inline(always)]
    pub fn i2s0_clk(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel5::I2s0Clk)
    }
    #[doc = "General purpose I/O"]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel5::Gpio)
    }
    #[doc = "UART Request to Send (RTS) (UART 2)"]
    #[inline(always)]
    pub fn uart2rts(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel5::Uart2rts)
    }
    #[doc = "UART Request to Send (RTS) (UART 3)"]
    #[inline(always)]
    pub fn uart3rts(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel5::Uart3rts)
    }
    #[doc = "Timer/Counter input or output; Selection of direction is done via CTIMER register settings."]
    #[inline(always)]
    pub fn ct5(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel5::Ct5)
    }
    #[doc = "IOMSTR/MSPI N Chip Select. Polarity is determined by CE_POLARITY field"]
    #[inline(always)]
    pub fn nce5(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel5::Nce5)
    }
    #[doc = "Observation bus bit 5"]
    #[inline(always)]
    pub fn obsbus5(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel5::Obsbus5)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved9(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel5::Reserved9)
    }
    #[doc = "Bidirectional I2S Bit clock. Operates in output mode in master mode and input mode for slave mode. (I2S Master/Slave 2)"]
    #[inline(always)]
    pub fn i2s1_clk(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel5::I2s1Clk)
    }
    #[doc = "Fast PIO"]
    #[inline(always)]
    pub fn fpio(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel5::Fpio)
    }
    #[doc = "Internal function (Flash Bist)"]
    #[inline(always)]
    pub fn flb_tdi(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel5::FlbTdi)
    }
    #[doc = "Internal function (Flash parallel load)"]
    #[inline(always)]
    pub fn flload_data(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel5::FlloadData)
    }
    #[doc = "Internal function (MBIST)"]
    #[inline(always)]
    pub fn mda_srst(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel5::MdaSrst)
    }
    #[doc = "Internal function (SCAN)"]
    #[inline(always)]
    pub fn dft_iso(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel5::DftIso)
    }
}
#[doc = "Field `INPEN5` reader - Input enable for GPIO 5"]
pub type Inpen5R = crate::BitReader;
#[doc = "Field `INPEN5` writer - Input enable for GPIO 5"]
pub type Inpen5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDZERO5` reader - Return 0 for read data on GPIO 5"]
pub type Rdzero5R = crate::BitReader;
#[doc = "Field `RDZERO5` writer - Return 0 for read data on GPIO 5"]
pub type Rdzero5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Interrupt enable for GPIO 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Irpten5 {
    #[doc = "0: Interrupts are disabled for this GPIO"]
    Dis = 0,
    #[doc = "1: Interrupts are enabled for falling edge transition on this GPIO"]
    Intfall = 1,
    #[doc = "2: Interrupts are enabled for rising edge transitions on this GPIO"]
    Intrise = 2,
    #[doc = "3: Interrupts are enabled for any edge transition on this GPIO"]
    Intany = 3,
}
impl From<Irpten5> for u8 {
    #[inline(always)]
    fn from(variant: Irpten5) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Irpten5 {
    type Ux = u8;
}
impl crate::IsEnum for Irpten5 {}
#[doc = "Field `IRPTEN5` reader - Interrupt enable for GPIO 5"]
pub type Irpten5R = crate::FieldReader<Irpten5>;
impl Irpten5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irpten5 {
        match self.bits {
            0 => Irpten5::Dis,
            1 => Irpten5::Intfall,
            2 => Irpten5::Intrise,
            3 => Irpten5::Intany,
            _ => unreachable!(),
        }
    }
    #[doc = "Interrupts are disabled for this GPIO"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Irpten5::Dis
    }
    #[doc = "Interrupts are enabled for falling edge transition on this GPIO"]
    #[inline(always)]
    pub fn is_intfall(&self) -> bool {
        *self == Irpten5::Intfall
    }
    #[doc = "Interrupts are enabled for rising edge transitions on this GPIO"]
    #[inline(always)]
    pub fn is_intrise(&self) -> bool {
        *self == Irpten5::Intrise
    }
    #[doc = "Interrupts are enabled for any edge transition on this GPIO"]
    #[inline(always)]
    pub fn is_intany(&self) -> bool {
        *self == Irpten5::Intany
    }
}
#[doc = "Field `IRPTEN5` writer - Interrupt enable for GPIO 5"]
pub type Irpten5W<'a, REG> = crate::FieldWriter<'a, REG, 2, Irpten5, crate::Safe>;
impl<'a, REG> Irpten5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Interrupts are disabled for this GPIO"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten5::Dis)
    }
    #[doc = "Interrupts are enabled for falling edge transition on this GPIO"]
    #[inline(always)]
    pub fn intfall(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten5::Intfall)
    }
    #[doc = "Interrupts are enabled for rising edge transitions on this GPIO"]
    #[inline(always)]
    pub fn intrise(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten5::Intrise)
    }
    #[doc = "Interrupts are enabled for any edge transition on this GPIO"]
    #[inline(always)]
    pub fn intany(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten5::Intany)
    }
}
#[doc = "Pin IO mode selection for GPIO pin 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Outcfg5 {
    #[doc = "0: Output Disabled"]
    Dis = 0,
    #[doc = "1: Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    Pushpull = 1,
    #[doc = "2: Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    Od = 2,
    #[doc = "3: Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    Ts = 3,
}
impl From<Outcfg5> for u8 {
    #[inline(always)]
    fn from(variant: Outcfg5) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Outcfg5 {
    type Ux = u8;
}
impl crate::IsEnum for Outcfg5 {}
#[doc = "Field `OUTCFG5` reader - Pin IO mode selection for GPIO pin 5"]
pub type Outcfg5R = crate::FieldReader<Outcfg5>;
impl Outcfg5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Outcfg5 {
        match self.bits {
            0 => Outcfg5::Dis,
            1 => Outcfg5::Pushpull,
            2 => Outcfg5::Od,
            3 => Outcfg5::Ts,
            _ => unreachable!(),
        }
    }
    #[doc = "Output Disabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Outcfg5::Dis
    }
    #[doc = "Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == Outcfg5::Pushpull
    }
    #[doc = "Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == Outcfg5::Od
    }
    #[doc = "Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == Outcfg5::Ts
    }
}
#[doc = "Field `OUTCFG5` writer - Pin IO mode selection for GPIO pin 5"]
pub type Outcfg5W<'a, REG> = crate::FieldWriter<'a, REG, 2, Outcfg5, crate::Safe>;
impl<'a, REG> Outcfg5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Output Disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg5::Dis)
    }
    #[doc = "Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg5::Pushpull)
    }
    #[doc = "Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    #[inline(always)]
    pub fn od(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg5::Od)
    }
    #[doc = "Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    #[inline(always)]
    pub fn ts(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg5::Ts)
    }
}
#[doc = "Drive strength selection for GPIO 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ds5 {
    #[doc = "0: 0.1x output driver selected"]
    _0p1x = 0,
    #[doc = "1: 0.5x output driver selected"]
    _0p5x = 1,
    #[doc = "2: 0.75x output driver selected"]
    _0p75x = 2,
    #[doc = "3: 1.0x output driver selected"]
    _1p0x = 3,
}
impl From<Ds5> for u8 {
    #[inline(always)]
    fn from(variant: Ds5) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ds5 {
    type Ux = u8;
}
impl crate::IsEnum for Ds5 {}
#[doc = "Field `DS5` reader - Drive strength selection for GPIO 5"]
pub type Ds5R = crate::FieldReader<Ds5>;
impl Ds5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ds5 {
        match self.bits {
            0 => Ds5::_0p1x,
            1 => Ds5::_0p5x,
            2 => Ds5::_0p75x,
            3 => Ds5::_1p0x,
            _ => unreachable!(),
        }
    }
    #[doc = "0.1x output driver selected"]
    #[inline(always)]
    pub fn is_0p1x(&self) -> bool {
        *self == Ds5::_0p1x
    }
    #[doc = "0.5x output driver selected"]
    #[inline(always)]
    pub fn is_0p5x(&self) -> bool {
        *self == Ds5::_0p5x
    }
    #[doc = "0.75x output driver selected"]
    #[inline(always)]
    pub fn is_0p75x(&self) -> bool {
        *self == Ds5::_0p75x
    }
    #[doc = "1.0x output driver selected"]
    #[inline(always)]
    pub fn is_1p0x(&self) -> bool {
        *self == Ds5::_1p0x
    }
}
#[doc = "Field `DS5` writer - Drive strength selection for GPIO 5"]
pub type Ds5W<'a, REG> = crate::FieldWriter<'a, REG, 2, Ds5, crate::Safe>;
impl<'a, REG> Ds5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "0.1x output driver selected"]
    #[inline(always)]
    pub fn _0p1x(self) -> &'a mut crate::W<REG> {
        self.variant(Ds5::_0p1x)
    }
    #[doc = "0.5x output driver selected"]
    #[inline(always)]
    pub fn _0p5x(self) -> &'a mut crate::W<REG> {
        self.variant(Ds5::_0p5x)
    }
    #[doc = "0.75x output driver selected"]
    #[inline(always)]
    pub fn _0p75x(self) -> &'a mut crate::W<REG> {
        self.variant(Ds5::_0p75x)
    }
    #[doc = "1.0x output driver selected"]
    #[inline(always)]
    pub fn _1p0x(self) -> &'a mut crate::W<REG> {
        self.variant(Ds5::_1p0x)
    }
}
#[doc = "Field `SR5` reader - Configure the slew rate"]
pub type Sr5R = crate::BitReader;
#[doc = "Field `SR5` writer - Configure the slew rate"]
pub type Sr5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Pullup/Pulldown configuration for GPIO 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pullcfg5 {
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
impl From<Pullcfg5> for u8 {
    #[inline(always)]
    fn from(variant: Pullcfg5) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pullcfg5 {
    type Ux = u8;
}
impl crate::IsEnum for Pullcfg5 {}
#[doc = "Field `PULLCFG5` reader - Pullup/Pulldown configuration for GPIO 5"]
pub type Pullcfg5R = crate::FieldReader<Pullcfg5>;
impl Pullcfg5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pullcfg5 {
        match self.bits {
            0 => Pullcfg5::Dis,
            1 => Pullcfg5::Pd50k,
            2 => Pullcfg5::Pu15k,
            3 => Pullcfg5::Pu6k,
            4 => Pullcfg5::Pu12k,
            5 => Pullcfg5::Pu24k,
            6 => Pullcfg5::Pu50k,
            7 => Pullcfg5::Pu100k,
            _ => unreachable!(),
        }
    }
    #[doc = "No pullup or pulldown selected"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Pullcfg5::Dis
    }
    #[doc = "50K Pulldown selected"]
    #[inline(always)]
    pub fn is_pd50k(&self) -> bool {
        *self == Pullcfg5::Pd50k
    }
    #[doc = "1.5K Pullup selected"]
    #[inline(always)]
    pub fn is_pu15k(&self) -> bool {
        *self == Pullcfg5::Pu15k
    }
    #[doc = "6K Pullup selected"]
    #[inline(always)]
    pub fn is_pu6k(&self) -> bool {
        *self == Pullcfg5::Pu6k
    }
    #[doc = "12K Pullup selected"]
    #[inline(always)]
    pub fn is_pu12k(&self) -> bool {
        *self == Pullcfg5::Pu12k
    }
    #[doc = "24K Pullup selected"]
    #[inline(always)]
    pub fn is_pu24k(&self) -> bool {
        *self == Pullcfg5::Pu24k
    }
    #[doc = "50K Pullup selected"]
    #[inline(always)]
    pub fn is_pu50k(&self) -> bool {
        *self == Pullcfg5::Pu50k
    }
    #[doc = "100K Pullup selected"]
    #[inline(always)]
    pub fn is_pu100k(&self) -> bool {
        *self == Pullcfg5::Pu100k
    }
}
#[doc = "Field `PULLCFG5` writer - Pullup/Pulldown configuration for GPIO 5"]
pub type Pullcfg5W<'a, REG> = crate::FieldWriter<'a, REG, 3, Pullcfg5, crate::Safe>;
impl<'a, REG> Pullcfg5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No pullup or pulldown selected"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg5::Dis)
    }
    #[doc = "50K Pulldown selected"]
    #[inline(always)]
    pub fn pd50k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg5::Pd50k)
    }
    #[doc = "1.5K Pullup selected"]
    #[inline(always)]
    pub fn pu15k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg5::Pu15k)
    }
    #[doc = "6K Pullup selected"]
    #[inline(always)]
    pub fn pu6k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg5::Pu6k)
    }
    #[doc = "12K Pullup selected"]
    #[inline(always)]
    pub fn pu12k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg5::Pu12k)
    }
    #[doc = "24K Pullup selected"]
    #[inline(always)]
    pub fn pu24k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg5::Pu24k)
    }
    #[doc = "50K Pullup selected"]
    #[inline(always)]
    pub fn pu50k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg5::Pu50k)
    }
    #[doc = "100K Pullup selected"]
    #[inline(always)]
    pub fn pu100k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg5::Pu100k)
    }
}
#[doc = "IOMSTR/MSPI N Chip Select 5, DISP control signals DE, CSX, and CS. Polarity is determined by CE_POLARITY field\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ncesrc5 {
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
impl From<Ncesrc5> for u8 {
    #[inline(always)]
    fn from(variant: Ncesrc5) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ncesrc5 {
    type Ux = u8;
}
impl crate::IsEnum for Ncesrc5 {}
#[doc = "Field `NCESRC5` reader - IOMSTR/MSPI N Chip Select 5, DISP control signals DE, CSX, and CS. Polarity is determined by CE_POLARITY field"]
pub type Ncesrc5R = crate::FieldReader<Ncesrc5>;
impl Ncesrc5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ncesrc5> {
        match self.bits {
            0 => Some(Ncesrc5::Iom0ce0),
            1 => Some(Ncesrc5::Iom0ce1),
            2 => Some(Ncesrc5::Iom0ce2),
            3 => Some(Ncesrc5::Iom0ce3),
            4 => Some(Ncesrc5::Iom1ce0),
            5 => Some(Ncesrc5::Iom1ce1),
            6 => Some(Ncesrc5::Iom1ce2),
            7 => Some(Ncesrc5::Iom1ce3),
            8 => Some(Ncesrc5::Iom2ce0),
            9 => Some(Ncesrc5::Iom2ce1),
            10 => Some(Ncesrc5::Iom2ce2),
            11 => Some(Ncesrc5::Iom2ce3),
            12 => Some(Ncesrc5::Iom3ce0),
            13 => Some(Ncesrc5::Iom3ce1),
            14 => Some(Ncesrc5::Iom3ce2),
            15 => Some(Ncesrc5::Iom3ce3),
            16 => Some(Ncesrc5::Iom4ce0),
            17 => Some(Ncesrc5::Iom4ce1),
            18 => Some(Ncesrc5::Iom4ce2),
            19 => Some(Ncesrc5::Iom4ce3),
            20 => Some(Ncesrc5::Iom5ce0),
            21 => Some(Ncesrc5::Iom5ce1),
            22 => Some(Ncesrc5::Iom5ce2),
            23 => Some(Ncesrc5::Iom5ce3),
            24 => Some(Ncesrc5::Iom6ce0),
            25 => Some(Ncesrc5::Iom6ce1),
            26 => Some(Ncesrc5::Iom6ce2),
            27 => Some(Ncesrc5::Iom6ce3),
            28 => Some(Ncesrc5::Iom7ce0),
            29 => Some(Ncesrc5::Iom7ce1),
            30 => Some(Ncesrc5::Iom7ce2),
            31 => Some(Ncesrc5::Iom7ce3),
            32 => Some(Ncesrc5::Mspi0cen0),
            33 => Some(Ncesrc5::Mspi0cen1),
            34 => Some(Ncesrc5::Mspi1cen0),
            35 => Some(Ncesrc5::Mspi1cen1),
            36 => Some(Ncesrc5::Mspi2cen0),
            37 => Some(Ncesrc5::Mspi2cen1),
            38 => Some(Ncesrc5::DcDpiDe),
            39 => Some(Ncesrc5::DispContCsx),
            40 => Some(Ncesrc5::DcSpiCsN),
            41 => Some(Ncesrc5::DcQspiCsN),
            42 => Some(Ncesrc5::DcResx),
            _ => None,
        }
    }
    #[doc = "IOM 0 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom0ce0(&self) -> bool {
        *self == Ncesrc5::Iom0ce0
    }
    #[doc = "IOM 0 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom0ce1(&self) -> bool {
        *self == Ncesrc5::Iom0ce1
    }
    #[doc = "IOM 0 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom0ce2(&self) -> bool {
        *self == Ncesrc5::Iom0ce2
    }
    #[doc = "IOM 0 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom0ce3(&self) -> bool {
        *self == Ncesrc5::Iom0ce3
    }
    #[doc = "IOM 1 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom1ce0(&self) -> bool {
        *self == Ncesrc5::Iom1ce0
    }
    #[doc = "IOM 1 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom1ce1(&self) -> bool {
        *self == Ncesrc5::Iom1ce1
    }
    #[doc = "IOM 1 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom1ce2(&self) -> bool {
        *self == Ncesrc5::Iom1ce2
    }
    #[doc = "IOM 1 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom1ce3(&self) -> bool {
        *self == Ncesrc5::Iom1ce3
    }
    #[doc = "IOM 2 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom2ce0(&self) -> bool {
        *self == Ncesrc5::Iom2ce0
    }
    #[doc = "IOM 2 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom2ce1(&self) -> bool {
        *self == Ncesrc5::Iom2ce1
    }
    #[doc = "IOM 2 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom2ce2(&self) -> bool {
        *self == Ncesrc5::Iom2ce2
    }
    #[doc = "IOM 2 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom2ce3(&self) -> bool {
        *self == Ncesrc5::Iom2ce3
    }
    #[doc = "IOM 3 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom3ce0(&self) -> bool {
        *self == Ncesrc5::Iom3ce0
    }
    #[doc = "IOM 3 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom3ce1(&self) -> bool {
        *self == Ncesrc5::Iom3ce1
    }
    #[doc = "IOM 3 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom3ce2(&self) -> bool {
        *self == Ncesrc5::Iom3ce2
    }
    #[doc = "IOM 3 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom3ce3(&self) -> bool {
        *self == Ncesrc5::Iom3ce3
    }
    #[doc = "IOM 4 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom4ce0(&self) -> bool {
        *self == Ncesrc5::Iom4ce0
    }
    #[doc = "IOM 4 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom4ce1(&self) -> bool {
        *self == Ncesrc5::Iom4ce1
    }
    #[doc = "IOM 4 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom4ce2(&self) -> bool {
        *self == Ncesrc5::Iom4ce2
    }
    #[doc = "IOM 4 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom4ce3(&self) -> bool {
        *self == Ncesrc5::Iom4ce3
    }
    #[doc = "IOM 5 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom5ce0(&self) -> bool {
        *self == Ncesrc5::Iom5ce0
    }
    #[doc = "IOM 5 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom5ce1(&self) -> bool {
        *self == Ncesrc5::Iom5ce1
    }
    #[doc = "IOM 5 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom5ce2(&self) -> bool {
        *self == Ncesrc5::Iom5ce2
    }
    #[doc = "IOM 5 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom5ce3(&self) -> bool {
        *self == Ncesrc5::Iom5ce3
    }
    #[doc = "IOM 6 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom6ce0(&self) -> bool {
        *self == Ncesrc5::Iom6ce0
    }
    #[doc = "IOM 6 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom6ce1(&self) -> bool {
        *self == Ncesrc5::Iom6ce1
    }
    #[doc = "IOM 6 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom6ce2(&self) -> bool {
        *self == Ncesrc5::Iom6ce2
    }
    #[doc = "IOM 6 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom6ce3(&self) -> bool {
        *self == Ncesrc5::Iom6ce3
    }
    #[doc = "IOM 7 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom7ce0(&self) -> bool {
        *self == Ncesrc5::Iom7ce0
    }
    #[doc = "IOM 7 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom7ce1(&self) -> bool {
        *self == Ncesrc5::Iom7ce1
    }
    #[doc = "IOM 7 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom7ce2(&self) -> bool {
        *self == Ncesrc5::Iom7ce2
    }
    #[doc = "IOM 7 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom7ce3(&self) -> bool {
        *self == Ncesrc5::Iom7ce3
    }
    #[doc = "MSPI 0 NCE 0 module"]
    #[inline(always)]
    pub fn is_mspi0cen0(&self) -> bool {
        *self == Ncesrc5::Mspi0cen0
    }
    #[doc = "MSPI 0 NCE 1 module"]
    #[inline(always)]
    pub fn is_mspi0cen1(&self) -> bool {
        *self == Ncesrc5::Mspi0cen1
    }
    #[doc = "MSPI 1 NCE 0 module"]
    #[inline(always)]
    pub fn is_mspi1cen0(&self) -> bool {
        *self == Ncesrc5::Mspi1cen0
    }
    #[doc = "MSPI 1 NCE 1 module"]
    #[inline(always)]
    pub fn is_mspi1cen1(&self) -> bool {
        *self == Ncesrc5::Mspi1cen1
    }
    #[doc = "MSPI 2 NCE 0 module"]
    #[inline(always)]
    pub fn is_mspi2cen0(&self) -> bool {
        *self == Ncesrc5::Mspi2cen0
    }
    #[doc = "MSPI 2 NCE 1 module"]
    #[inline(always)]
    pub fn is_mspi2cen1(&self) -> bool {
        *self == Ncesrc5::Mspi2cen1
    }
    #[doc = "DC DPI DE module"]
    #[inline(always)]
    pub fn is_dc_dpi_de(&self) -> bool {
        *self == Ncesrc5::DcDpiDe
    }
    #[doc = "DISP CONT CSX module"]
    #[inline(always)]
    pub fn is_disp_cont_csx(&self) -> bool {
        *self == Ncesrc5::DispContCsx
    }
    #[doc = "DC SPI CS_N module"]
    #[inline(always)]
    pub fn is_dc_spi_cs_n(&self) -> bool {
        *self == Ncesrc5::DcSpiCsN
    }
    #[doc = "DC QSPI CS_N module"]
    #[inline(always)]
    pub fn is_dc_qspi_cs_n(&self) -> bool {
        *self == Ncesrc5::DcQspiCsN
    }
    #[doc = "DC module RESX"]
    #[inline(always)]
    pub fn is_dc_resx(&self) -> bool {
        *self == Ncesrc5::DcResx
    }
}
#[doc = "Field `NCESRC5` writer - IOMSTR/MSPI N Chip Select 5, DISP control signals DE, CSX, and CS. Polarity is determined by CE_POLARITY field"]
pub type Ncesrc5W<'a, REG> = crate::FieldWriter<'a, REG, 6, Ncesrc5>;
impl<'a, REG> Ncesrc5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "IOM 0 NCE 0 module"]
    #[inline(always)]
    pub fn iom0ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc5::Iom0ce0)
    }
    #[doc = "IOM 0 NCE 1 module"]
    #[inline(always)]
    pub fn iom0ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc5::Iom0ce1)
    }
    #[doc = "IOM 0 NCE 2 module"]
    #[inline(always)]
    pub fn iom0ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc5::Iom0ce2)
    }
    #[doc = "IOM 0 NCE 3 module"]
    #[inline(always)]
    pub fn iom0ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc5::Iom0ce3)
    }
    #[doc = "IOM 1 NCE 0 module"]
    #[inline(always)]
    pub fn iom1ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc5::Iom1ce0)
    }
    #[doc = "IOM 1 NCE 1 module"]
    #[inline(always)]
    pub fn iom1ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc5::Iom1ce1)
    }
    #[doc = "IOM 1 NCE 2 module"]
    #[inline(always)]
    pub fn iom1ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc5::Iom1ce2)
    }
    #[doc = "IOM 1 NCE 3 module"]
    #[inline(always)]
    pub fn iom1ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc5::Iom1ce3)
    }
    #[doc = "IOM 2 NCE 0 module"]
    #[inline(always)]
    pub fn iom2ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc5::Iom2ce0)
    }
    #[doc = "IOM 2 NCE 1 module"]
    #[inline(always)]
    pub fn iom2ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc5::Iom2ce1)
    }
    #[doc = "IOM 2 NCE 2 module"]
    #[inline(always)]
    pub fn iom2ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc5::Iom2ce2)
    }
    #[doc = "IOM 2 NCE 3 module"]
    #[inline(always)]
    pub fn iom2ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc5::Iom2ce3)
    }
    #[doc = "IOM 3 NCE 0 module"]
    #[inline(always)]
    pub fn iom3ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc5::Iom3ce0)
    }
    #[doc = "IOM 3 NCE 1 module"]
    #[inline(always)]
    pub fn iom3ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc5::Iom3ce1)
    }
    #[doc = "IOM 3 NCE 2 module"]
    #[inline(always)]
    pub fn iom3ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc5::Iom3ce2)
    }
    #[doc = "IOM 3 NCE 3 module"]
    #[inline(always)]
    pub fn iom3ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc5::Iom3ce3)
    }
    #[doc = "IOM 4 NCE 0 module"]
    #[inline(always)]
    pub fn iom4ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc5::Iom4ce0)
    }
    #[doc = "IOM 4 NCE 1 module"]
    #[inline(always)]
    pub fn iom4ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc5::Iom4ce1)
    }
    #[doc = "IOM 4 NCE 2 module"]
    #[inline(always)]
    pub fn iom4ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc5::Iom4ce2)
    }
    #[doc = "IOM 4 NCE 3 module"]
    #[inline(always)]
    pub fn iom4ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc5::Iom4ce3)
    }
    #[doc = "IOM 5 NCE 0 module"]
    #[inline(always)]
    pub fn iom5ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc5::Iom5ce0)
    }
    #[doc = "IOM 5 NCE 1 module"]
    #[inline(always)]
    pub fn iom5ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc5::Iom5ce1)
    }
    #[doc = "IOM 5 NCE 2 module"]
    #[inline(always)]
    pub fn iom5ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc5::Iom5ce2)
    }
    #[doc = "IOM 5 NCE 3 module"]
    #[inline(always)]
    pub fn iom5ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc5::Iom5ce3)
    }
    #[doc = "IOM 6 NCE 0 module"]
    #[inline(always)]
    pub fn iom6ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc5::Iom6ce0)
    }
    #[doc = "IOM 6 NCE 1 module"]
    #[inline(always)]
    pub fn iom6ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc5::Iom6ce1)
    }
    #[doc = "IOM 6 NCE 2 module"]
    #[inline(always)]
    pub fn iom6ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc5::Iom6ce2)
    }
    #[doc = "IOM 6 NCE 3 module"]
    #[inline(always)]
    pub fn iom6ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc5::Iom6ce3)
    }
    #[doc = "IOM 7 NCE 0 module"]
    #[inline(always)]
    pub fn iom7ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc5::Iom7ce0)
    }
    #[doc = "IOM 7 NCE 1 module"]
    #[inline(always)]
    pub fn iom7ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc5::Iom7ce1)
    }
    #[doc = "IOM 7 NCE 2 module"]
    #[inline(always)]
    pub fn iom7ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc5::Iom7ce2)
    }
    #[doc = "IOM 7 NCE 3 module"]
    #[inline(always)]
    pub fn iom7ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc5::Iom7ce3)
    }
    #[doc = "MSPI 0 NCE 0 module"]
    #[inline(always)]
    pub fn mspi0cen0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc5::Mspi0cen0)
    }
    #[doc = "MSPI 0 NCE 1 module"]
    #[inline(always)]
    pub fn mspi0cen1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc5::Mspi0cen1)
    }
    #[doc = "MSPI 1 NCE 0 module"]
    #[inline(always)]
    pub fn mspi1cen0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc5::Mspi1cen0)
    }
    #[doc = "MSPI 1 NCE 1 module"]
    #[inline(always)]
    pub fn mspi1cen1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc5::Mspi1cen1)
    }
    #[doc = "MSPI 2 NCE 0 module"]
    #[inline(always)]
    pub fn mspi2cen0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc5::Mspi2cen0)
    }
    #[doc = "MSPI 2 NCE 1 module"]
    #[inline(always)]
    pub fn mspi2cen1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc5::Mspi2cen1)
    }
    #[doc = "DC DPI DE module"]
    #[inline(always)]
    pub fn dc_dpi_de(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc5::DcDpiDe)
    }
    #[doc = "DISP CONT CSX module"]
    #[inline(always)]
    pub fn disp_cont_csx(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc5::DispContCsx)
    }
    #[doc = "DC SPI CS_N module"]
    #[inline(always)]
    pub fn dc_spi_cs_n(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc5::DcSpiCsN)
    }
    #[doc = "DC QSPI CS_N module"]
    #[inline(always)]
    pub fn dc_qspi_cs_n(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc5::DcQspiCsN)
    }
    #[doc = "DC module RESX"]
    #[inline(always)]
    pub fn dc_resx(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc5::DcResx)
    }
}
#[doc = "Polarity select for NCE for GPIO 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ncepol5 {
    #[doc = "0: Polarity is active low"]
    Low = 0,
    #[doc = "1: Polarity is active high"]
    High = 1,
}
impl From<Ncepol5> for bool {
    #[inline(always)]
    fn from(variant: Ncepol5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NCEPOL5` reader - Polarity select for NCE for GPIO 5"]
pub type Ncepol5R = crate::BitReader<Ncepol5>;
impl Ncepol5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ncepol5 {
        match self.bits {
            false => Ncepol5::Low,
            true => Ncepol5::High,
        }
    }
    #[doc = "Polarity is active low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Ncepol5::Low
    }
    #[doc = "Polarity is active high"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Ncepol5::High
    }
}
#[doc = "Field `NCEPOL5` writer - Polarity select for NCE for GPIO 5"]
pub type Ncepol5W<'a, REG> = crate::BitWriter<'a, REG, Ncepol5>;
impl<'a, REG> Ncepol5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Polarity is active low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Ncepol5::Low)
    }
    #[doc = "Polarity is active high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Ncepol5::High)
    }
}
#[doc = "Field `FIEN5` reader - Force input enable active regardless of function selected. Otherwise the selected function will enable the input only when needed"]
pub type Fien5R = crate::BitReader;
#[doc = "Field `FIEN5` writer - Force input enable active regardless of function selected. Otherwise the selected function will enable the input only when needed"]
pub type Fien5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FOEN5` reader - Force output enable active regardless of function selected. Otherwise the selected function will enable the output only when needed"]
pub type Foen5R = crate::BitReader;
#[doc = "Field `FOEN5` writer - Force output enable active regardless of function selected. Otherwise the selected function will enable the output only when needed"]
pub type Foen5W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Function select for GPIO pin 5"]
    #[inline(always)]
    pub fn fncsel5(&self) -> Fncsel5R {
        Fncsel5R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Input enable for GPIO 5"]
    #[inline(always)]
    pub fn inpen5(&self) -> Inpen5R {
        Inpen5R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Return 0 for read data on GPIO 5"]
    #[inline(always)]
    pub fn rdzero5(&self) -> Rdzero5R {
        Rdzero5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Interrupt enable for GPIO 5"]
    #[inline(always)]
    pub fn irpten5(&self) -> Irpten5R {
        Irpten5R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Pin IO mode selection for GPIO pin 5"]
    #[inline(always)]
    pub fn outcfg5(&self) -> Outcfg5R {
        Outcfg5R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Drive strength selection for GPIO 5"]
    #[inline(always)]
    pub fn ds5(&self) -> Ds5R {
        Ds5R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - Configure the slew rate"]
    #[inline(always)]
    pub fn sr5(&self) -> Sr5R {
        Sr5R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - Pullup/Pulldown configuration for GPIO 5"]
    #[inline(always)]
    pub fn pullcfg5(&self) -> Pullcfg5R {
        Pullcfg5R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:21 - IOMSTR/MSPI N Chip Select 5, DISP control signals DE, CSX, and CS. Polarity is determined by CE_POLARITY field"]
    #[inline(always)]
    pub fn ncesrc5(&self) -> Ncesrc5R {
        Ncesrc5R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 22 - Polarity select for NCE for GPIO 5"]
    #[inline(always)]
    pub fn ncepol5(&self) -> Ncepol5R {
        Ncepol5R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 26 - Force input enable active regardless of function selected. Otherwise the selected function will enable the input only when needed"]
    #[inline(always)]
    pub fn fien5(&self) -> Fien5R {
        Fien5R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Force output enable active regardless of function selected. Otherwise the selected function will enable the output only when needed"]
    #[inline(always)]
    pub fn foen5(&self) -> Foen5R {
        Foen5R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Function select for GPIO pin 5"]
    #[inline(always)]
    #[must_use]
    pub fn fncsel5(&mut self) -> Fncsel5W<Pincfg5Spec> {
        Fncsel5W::new(self, 0)
    }
    #[doc = "Bit 4 - Input enable for GPIO 5"]
    #[inline(always)]
    #[must_use]
    pub fn inpen5(&mut self) -> Inpen5W<Pincfg5Spec> {
        Inpen5W::new(self, 4)
    }
    #[doc = "Bit 5 - Return 0 for read data on GPIO 5"]
    #[inline(always)]
    #[must_use]
    pub fn rdzero5(&mut self) -> Rdzero5W<Pincfg5Spec> {
        Rdzero5W::new(self, 5)
    }
    #[doc = "Bits 6:7 - Interrupt enable for GPIO 5"]
    #[inline(always)]
    #[must_use]
    pub fn irpten5(&mut self) -> Irpten5W<Pincfg5Spec> {
        Irpten5W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Pin IO mode selection for GPIO pin 5"]
    #[inline(always)]
    #[must_use]
    pub fn outcfg5(&mut self) -> Outcfg5W<Pincfg5Spec> {
        Outcfg5W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Drive strength selection for GPIO 5"]
    #[inline(always)]
    #[must_use]
    pub fn ds5(&mut self) -> Ds5W<Pincfg5Spec> {
        Ds5W::new(self, 10)
    }
    #[doc = "Bit 12 - Configure the slew rate"]
    #[inline(always)]
    #[must_use]
    pub fn sr5(&mut self) -> Sr5W<Pincfg5Spec> {
        Sr5W::new(self, 12)
    }
    #[doc = "Bits 13:15 - Pullup/Pulldown configuration for GPIO 5"]
    #[inline(always)]
    #[must_use]
    pub fn pullcfg5(&mut self) -> Pullcfg5W<Pincfg5Spec> {
        Pullcfg5W::new(self, 13)
    }
    #[doc = "Bits 16:21 - IOMSTR/MSPI N Chip Select 5, DISP control signals DE, CSX, and CS. Polarity is determined by CE_POLARITY field"]
    #[inline(always)]
    #[must_use]
    pub fn ncesrc5(&mut self) -> Ncesrc5W<Pincfg5Spec> {
        Ncesrc5W::new(self, 16)
    }
    #[doc = "Bit 22 - Polarity select for NCE for GPIO 5"]
    #[inline(always)]
    #[must_use]
    pub fn ncepol5(&mut self) -> Ncepol5W<Pincfg5Spec> {
        Ncepol5W::new(self, 22)
    }
    #[doc = "Bit 26 - Force input enable active regardless of function selected. Otherwise the selected function will enable the input only when needed"]
    #[inline(always)]
    #[must_use]
    pub fn fien5(&mut self) -> Fien5W<Pincfg5Spec> {
        Fien5W::new(self, 26)
    }
    #[doc = "Bit 27 - Force output enable active regardless of function selected. Otherwise the selected function will enable the output only when needed"]
    #[inline(always)]
    #[must_use]
    pub fn foen5(&mut self) -> Foen5W<Pincfg5Spec> {
        Foen5W::new(self, 27)
    }
}
#[doc = "Controls the operation of GPIO pin 5.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pincfg5Spec;
impl crate::RegisterSpec for Pincfg5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pincfg5::R`](R) reader structure"]
impl crate::Readable for Pincfg5Spec {}
#[doc = "`write(|w| ..)` method takes [`pincfg5::W`](W) writer structure"]
impl crate::Writable for Pincfg5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PINCFG5 to value 0x03"]
impl crate::Resettable for Pincfg5Spec {
    const RESET_VALUE: u32 = 0x03;
}
