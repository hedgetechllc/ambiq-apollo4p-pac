#[doc = "Register `PINCFG4` reader"]
pub type R = crate::R<Pincfg4Spec>;
#[doc = "Register `PINCFG4` writer"]
pub type W = crate::W<Pincfg4Spec>;
#[doc = "Function select for GPIO pin 4\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fncsel4 {
    #[doc = "0: Serial Wire Debug Trace Output 3"]
    Swtrace3 = 0,
    #[doc = "1: Configurable Slave Interrupt"]
    Slint = 1,
    #[doc = "2: 32kHZ from analog"]
    _32khzXt = 2,
    #[doc = "3: General purpose I/O"]
    Gpio = 3,
    #[doc = "4: UART Request to Send (RTS) (UART 0)"]
    Uart0rts = 4,
    #[doc = "5: UART Request to Send (RTS) (UART 1)"]
    Uart1rts = 5,
    #[doc = "6: Timer/Counter input or output; Selection of direction is done via CTIMER register settings."]
    Ct4 = 6,
    #[doc = "7: IOMSTR/MSPI N Chip Select. Polarity is determined by CE_POLARITY field"]
    Nce4 = 7,
    #[doc = "8: Observation bus bit 4"]
    Obsbus4 = 8,
    #[doc = "9: I2S Data input (I2S Master/Slave 2)"]
    I2s0Sdin = 9,
    #[doc = "10: I2S Data input (I2S Master/Slave 2)"]
    I2s1Sdin = 10,
    #[doc = "11: Fast PIO"]
    Fpio = 11,
    #[doc = "12: Internal function (Flash Bist)"]
    FlbTdo = 12,
    #[doc = "13: Internal function (Flash parallel load)"]
    FlloadDir = 13,
    #[doc = "14: Internal function (MBIST)"]
    MdaTdo = 14,
    #[doc = "15: Internal function (SCAN)"]
    OpcgTrig = 15,
}
impl From<Fncsel4> for u8 {
    #[inline(always)]
    fn from(variant: Fncsel4) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fncsel4 {
    type Ux = u8;
}
impl crate::IsEnum for Fncsel4 {}
#[doc = "Field `FNCSEL4` reader - Function select for GPIO pin 4"]
pub type Fncsel4R = crate::FieldReader<Fncsel4>;
impl Fncsel4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fncsel4 {
        match self.bits {
            0 => Fncsel4::Swtrace3,
            1 => Fncsel4::Slint,
            2 => Fncsel4::_32khzXt,
            3 => Fncsel4::Gpio,
            4 => Fncsel4::Uart0rts,
            5 => Fncsel4::Uart1rts,
            6 => Fncsel4::Ct4,
            7 => Fncsel4::Nce4,
            8 => Fncsel4::Obsbus4,
            9 => Fncsel4::I2s0Sdin,
            10 => Fncsel4::I2s1Sdin,
            11 => Fncsel4::Fpio,
            12 => Fncsel4::FlbTdo,
            13 => Fncsel4::FlloadDir,
            14 => Fncsel4::MdaTdo,
            15 => Fncsel4::OpcgTrig,
            _ => unreachable!(),
        }
    }
    #[doc = "Serial Wire Debug Trace Output 3"]
    #[inline(always)]
    pub fn is_swtrace3(&self) -> bool {
        *self == Fncsel4::Swtrace3
    }
    #[doc = "Configurable Slave Interrupt"]
    #[inline(always)]
    pub fn is_slint(&self) -> bool {
        *self == Fncsel4::Slint
    }
    #[doc = "32kHZ from analog"]
    #[inline(always)]
    pub fn is_32khz_xt(&self) -> bool {
        *self == Fncsel4::_32khzXt
    }
    #[doc = "General purpose I/O"]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == Fncsel4::Gpio
    }
    #[doc = "UART Request to Send (RTS) (UART 0)"]
    #[inline(always)]
    pub fn is_uart0rts(&self) -> bool {
        *self == Fncsel4::Uart0rts
    }
    #[doc = "UART Request to Send (RTS) (UART 1)"]
    #[inline(always)]
    pub fn is_uart1rts(&self) -> bool {
        *self == Fncsel4::Uart1rts
    }
    #[doc = "Timer/Counter input or output; Selection of direction is done via CTIMER register settings."]
    #[inline(always)]
    pub fn is_ct4(&self) -> bool {
        *self == Fncsel4::Ct4
    }
    #[doc = "IOMSTR/MSPI N Chip Select. Polarity is determined by CE_POLARITY field"]
    #[inline(always)]
    pub fn is_nce4(&self) -> bool {
        *self == Fncsel4::Nce4
    }
    #[doc = "Observation bus bit 4"]
    #[inline(always)]
    pub fn is_obsbus4(&self) -> bool {
        *self == Fncsel4::Obsbus4
    }
    #[doc = "I2S Data input (I2S Master/Slave 2)"]
    #[inline(always)]
    pub fn is_i2s0_sdin(&self) -> bool {
        *self == Fncsel4::I2s0Sdin
    }
    #[doc = "I2S Data input (I2S Master/Slave 2)"]
    #[inline(always)]
    pub fn is_i2s1_sdin(&self) -> bool {
        *self == Fncsel4::I2s1Sdin
    }
    #[doc = "Fast PIO"]
    #[inline(always)]
    pub fn is_fpio(&self) -> bool {
        *self == Fncsel4::Fpio
    }
    #[doc = "Internal function (Flash Bist)"]
    #[inline(always)]
    pub fn is_flb_tdo(&self) -> bool {
        *self == Fncsel4::FlbTdo
    }
    #[doc = "Internal function (Flash parallel load)"]
    #[inline(always)]
    pub fn is_flload_dir(&self) -> bool {
        *self == Fncsel4::FlloadDir
    }
    #[doc = "Internal function (MBIST)"]
    #[inline(always)]
    pub fn is_mda_tdo(&self) -> bool {
        *self == Fncsel4::MdaTdo
    }
    #[doc = "Internal function (SCAN)"]
    #[inline(always)]
    pub fn is_opcg_trig(&self) -> bool {
        *self == Fncsel4::OpcgTrig
    }
}
#[doc = "Field `FNCSEL4` writer - Function select for GPIO pin 4"]
pub type Fncsel4W<'a, REG> = crate::FieldWriter<'a, REG, 4, Fncsel4, crate::Safe>;
impl<'a, REG> Fncsel4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Serial Wire Debug Trace Output 3"]
    #[inline(always)]
    pub fn swtrace3(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel4::Swtrace3)
    }
    #[doc = "Configurable Slave Interrupt"]
    #[inline(always)]
    pub fn slint(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel4::Slint)
    }
    #[doc = "32kHZ from analog"]
    #[inline(always)]
    pub fn _32khz_xt(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel4::_32khzXt)
    }
    #[doc = "General purpose I/O"]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel4::Gpio)
    }
    #[doc = "UART Request to Send (RTS) (UART 0)"]
    #[inline(always)]
    pub fn uart0rts(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel4::Uart0rts)
    }
    #[doc = "UART Request to Send (RTS) (UART 1)"]
    #[inline(always)]
    pub fn uart1rts(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel4::Uart1rts)
    }
    #[doc = "Timer/Counter input or output; Selection of direction is done via CTIMER register settings."]
    #[inline(always)]
    pub fn ct4(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel4::Ct4)
    }
    #[doc = "IOMSTR/MSPI N Chip Select. Polarity is determined by CE_POLARITY field"]
    #[inline(always)]
    pub fn nce4(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel4::Nce4)
    }
    #[doc = "Observation bus bit 4"]
    #[inline(always)]
    pub fn obsbus4(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel4::Obsbus4)
    }
    #[doc = "I2S Data input (I2S Master/Slave 2)"]
    #[inline(always)]
    pub fn i2s0_sdin(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel4::I2s0Sdin)
    }
    #[doc = "I2S Data input (I2S Master/Slave 2)"]
    #[inline(always)]
    pub fn i2s1_sdin(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel4::I2s1Sdin)
    }
    #[doc = "Fast PIO"]
    #[inline(always)]
    pub fn fpio(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel4::Fpio)
    }
    #[doc = "Internal function (Flash Bist)"]
    #[inline(always)]
    pub fn flb_tdo(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel4::FlbTdo)
    }
    #[doc = "Internal function (Flash parallel load)"]
    #[inline(always)]
    pub fn flload_dir(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel4::FlloadDir)
    }
    #[doc = "Internal function (MBIST)"]
    #[inline(always)]
    pub fn mda_tdo(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel4::MdaTdo)
    }
    #[doc = "Internal function (SCAN)"]
    #[inline(always)]
    pub fn opcg_trig(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel4::OpcgTrig)
    }
}
#[doc = "Field `INPEN4` reader - Input enable for GPIO 4"]
pub type Inpen4R = crate::BitReader;
#[doc = "Field `INPEN4` writer - Input enable for GPIO 4"]
pub type Inpen4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDZERO4` reader - Return 0 for read data on GPIO 4"]
pub type Rdzero4R = crate::BitReader;
#[doc = "Field `RDZERO4` writer - Return 0 for read data on GPIO 4"]
pub type Rdzero4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Interrupt enable for GPIO 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Irpten4 {
    #[doc = "0: Interrupts are disabled for this GPIO"]
    Dis = 0,
    #[doc = "1: Interrupts are enabled for falling edge transition on this GPIO"]
    Intfall = 1,
    #[doc = "2: Interrupts are enabled for rising edge transitions on this GPIO"]
    Intrise = 2,
    #[doc = "3: Interrupts are enabled for any edge transition on this GPIO"]
    Intany = 3,
}
impl From<Irpten4> for u8 {
    #[inline(always)]
    fn from(variant: Irpten4) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Irpten4 {
    type Ux = u8;
}
impl crate::IsEnum for Irpten4 {}
#[doc = "Field `IRPTEN4` reader - Interrupt enable for GPIO 4"]
pub type Irpten4R = crate::FieldReader<Irpten4>;
impl Irpten4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irpten4 {
        match self.bits {
            0 => Irpten4::Dis,
            1 => Irpten4::Intfall,
            2 => Irpten4::Intrise,
            3 => Irpten4::Intany,
            _ => unreachable!(),
        }
    }
    #[doc = "Interrupts are disabled for this GPIO"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Irpten4::Dis
    }
    #[doc = "Interrupts are enabled for falling edge transition on this GPIO"]
    #[inline(always)]
    pub fn is_intfall(&self) -> bool {
        *self == Irpten4::Intfall
    }
    #[doc = "Interrupts are enabled for rising edge transitions on this GPIO"]
    #[inline(always)]
    pub fn is_intrise(&self) -> bool {
        *self == Irpten4::Intrise
    }
    #[doc = "Interrupts are enabled for any edge transition on this GPIO"]
    #[inline(always)]
    pub fn is_intany(&self) -> bool {
        *self == Irpten4::Intany
    }
}
#[doc = "Field `IRPTEN4` writer - Interrupt enable for GPIO 4"]
pub type Irpten4W<'a, REG> = crate::FieldWriter<'a, REG, 2, Irpten4, crate::Safe>;
impl<'a, REG> Irpten4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Interrupts are disabled for this GPIO"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten4::Dis)
    }
    #[doc = "Interrupts are enabled for falling edge transition on this GPIO"]
    #[inline(always)]
    pub fn intfall(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten4::Intfall)
    }
    #[doc = "Interrupts are enabled for rising edge transitions on this GPIO"]
    #[inline(always)]
    pub fn intrise(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten4::Intrise)
    }
    #[doc = "Interrupts are enabled for any edge transition on this GPIO"]
    #[inline(always)]
    pub fn intany(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten4::Intany)
    }
}
#[doc = "Pin IO mode selection for GPIO pin 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Outcfg4 {
    #[doc = "0: Output Disabled"]
    Dis = 0,
    #[doc = "1: Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    Pushpull = 1,
    #[doc = "2: Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    Od = 2,
    #[doc = "3: Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    Ts = 3,
}
impl From<Outcfg4> for u8 {
    #[inline(always)]
    fn from(variant: Outcfg4) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Outcfg4 {
    type Ux = u8;
}
impl crate::IsEnum for Outcfg4 {}
#[doc = "Field `OUTCFG4` reader - Pin IO mode selection for GPIO pin 4"]
pub type Outcfg4R = crate::FieldReader<Outcfg4>;
impl Outcfg4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Outcfg4 {
        match self.bits {
            0 => Outcfg4::Dis,
            1 => Outcfg4::Pushpull,
            2 => Outcfg4::Od,
            3 => Outcfg4::Ts,
            _ => unreachable!(),
        }
    }
    #[doc = "Output Disabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Outcfg4::Dis
    }
    #[doc = "Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == Outcfg4::Pushpull
    }
    #[doc = "Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == Outcfg4::Od
    }
    #[doc = "Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == Outcfg4::Ts
    }
}
#[doc = "Field `OUTCFG4` writer - Pin IO mode selection for GPIO pin 4"]
pub type Outcfg4W<'a, REG> = crate::FieldWriter<'a, REG, 2, Outcfg4, crate::Safe>;
impl<'a, REG> Outcfg4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Output Disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg4::Dis)
    }
    #[doc = "Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg4::Pushpull)
    }
    #[doc = "Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    #[inline(always)]
    pub fn od(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg4::Od)
    }
    #[doc = "Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    #[inline(always)]
    pub fn ts(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg4::Ts)
    }
}
#[doc = "Drive strength selection for GPIO 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ds4 {
    #[doc = "0: 0.1x output driver selected"]
    _0p1x = 0,
    #[doc = "1: 0.5x output driver selected"]
    _0p5x = 1,
}
impl From<Ds4> for u8 {
    #[inline(always)]
    fn from(variant: Ds4) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ds4 {
    type Ux = u8;
}
impl crate::IsEnum for Ds4 {}
#[doc = "Field `DS4` reader - Drive strength selection for GPIO 4"]
pub type Ds4R = crate::FieldReader<Ds4>;
impl Ds4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ds4> {
        match self.bits {
            0 => Some(Ds4::_0p1x),
            1 => Some(Ds4::_0p5x),
            _ => None,
        }
    }
    #[doc = "0.1x output driver selected"]
    #[inline(always)]
    pub fn is_0p1x(&self) -> bool {
        *self == Ds4::_0p1x
    }
    #[doc = "0.5x output driver selected"]
    #[inline(always)]
    pub fn is_0p5x(&self) -> bool {
        *self == Ds4::_0p5x
    }
}
#[doc = "Field `DS4` writer - Drive strength selection for GPIO 4"]
pub type Ds4W<'a, REG> = crate::FieldWriter<'a, REG, 2, Ds4>;
impl<'a, REG> Ds4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "0.1x output driver selected"]
    #[inline(always)]
    pub fn _0p1x(self) -> &'a mut crate::W<REG> {
        self.variant(Ds4::_0p1x)
    }
    #[doc = "0.5x output driver selected"]
    #[inline(always)]
    pub fn _0p5x(self) -> &'a mut crate::W<REG> {
        self.variant(Ds4::_0p5x)
    }
}
#[doc = "Field `SR4` reader - Configure the slew rate"]
pub type Sr4R = crate::BitReader;
#[doc = "Field `SR4` writer - Configure the slew rate"]
pub type Sr4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Pullup/Pulldown configuration for GPIO 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pullcfg4 {
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
impl From<Pullcfg4> for u8 {
    #[inline(always)]
    fn from(variant: Pullcfg4) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pullcfg4 {
    type Ux = u8;
}
impl crate::IsEnum for Pullcfg4 {}
#[doc = "Field `PULLCFG4` reader - Pullup/Pulldown configuration for GPIO 4"]
pub type Pullcfg4R = crate::FieldReader<Pullcfg4>;
impl Pullcfg4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pullcfg4 {
        match self.bits {
            0 => Pullcfg4::Dis,
            1 => Pullcfg4::Pd50k,
            2 => Pullcfg4::Pu15k,
            3 => Pullcfg4::Pu6k,
            4 => Pullcfg4::Pu12k,
            5 => Pullcfg4::Pu24k,
            6 => Pullcfg4::Pu50k,
            7 => Pullcfg4::Pu100k,
            _ => unreachable!(),
        }
    }
    #[doc = "No pullup or pulldown selected"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Pullcfg4::Dis
    }
    #[doc = "50K Pulldown selected"]
    #[inline(always)]
    pub fn is_pd50k(&self) -> bool {
        *self == Pullcfg4::Pd50k
    }
    #[doc = "1.5K Pullup selected"]
    #[inline(always)]
    pub fn is_pu15k(&self) -> bool {
        *self == Pullcfg4::Pu15k
    }
    #[doc = "6K Pullup selected"]
    #[inline(always)]
    pub fn is_pu6k(&self) -> bool {
        *self == Pullcfg4::Pu6k
    }
    #[doc = "12K Pullup selected"]
    #[inline(always)]
    pub fn is_pu12k(&self) -> bool {
        *self == Pullcfg4::Pu12k
    }
    #[doc = "24K Pullup selected"]
    #[inline(always)]
    pub fn is_pu24k(&self) -> bool {
        *self == Pullcfg4::Pu24k
    }
    #[doc = "50K Pullup selected"]
    #[inline(always)]
    pub fn is_pu50k(&self) -> bool {
        *self == Pullcfg4::Pu50k
    }
    #[doc = "100K Pullup selected"]
    #[inline(always)]
    pub fn is_pu100k(&self) -> bool {
        *self == Pullcfg4::Pu100k
    }
}
#[doc = "Field `PULLCFG4` writer - Pullup/Pulldown configuration for GPIO 4"]
pub type Pullcfg4W<'a, REG> = crate::FieldWriter<'a, REG, 3, Pullcfg4, crate::Safe>;
impl<'a, REG> Pullcfg4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No pullup or pulldown selected"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg4::Dis)
    }
    #[doc = "50K Pulldown selected"]
    #[inline(always)]
    pub fn pd50k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg4::Pd50k)
    }
    #[doc = "1.5K Pullup selected"]
    #[inline(always)]
    pub fn pu15k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg4::Pu15k)
    }
    #[doc = "6K Pullup selected"]
    #[inline(always)]
    pub fn pu6k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg4::Pu6k)
    }
    #[doc = "12K Pullup selected"]
    #[inline(always)]
    pub fn pu12k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg4::Pu12k)
    }
    #[doc = "24K Pullup selected"]
    #[inline(always)]
    pub fn pu24k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg4::Pu24k)
    }
    #[doc = "50K Pullup selected"]
    #[inline(always)]
    pub fn pu50k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg4::Pu50k)
    }
    #[doc = "100K Pullup selected"]
    #[inline(always)]
    pub fn pu100k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg4::Pu100k)
    }
}
#[doc = "IOMSTR/MSPI N Chip Select 4, DISP control signals DE, CSX, and CS. Polarity is determined by CE_POLARITY field\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ncesrc4 {
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
impl From<Ncesrc4> for u8 {
    #[inline(always)]
    fn from(variant: Ncesrc4) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ncesrc4 {
    type Ux = u8;
}
impl crate::IsEnum for Ncesrc4 {}
#[doc = "Field `NCESRC4` reader - IOMSTR/MSPI N Chip Select 4, DISP control signals DE, CSX, and CS. Polarity is determined by CE_POLARITY field"]
pub type Ncesrc4R = crate::FieldReader<Ncesrc4>;
impl Ncesrc4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ncesrc4> {
        match self.bits {
            0 => Some(Ncesrc4::Iom0ce0),
            1 => Some(Ncesrc4::Iom0ce1),
            2 => Some(Ncesrc4::Iom0ce2),
            3 => Some(Ncesrc4::Iom0ce3),
            4 => Some(Ncesrc4::Iom1ce0),
            5 => Some(Ncesrc4::Iom1ce1),
            6 => Some(Ncesrc4::Iom1ce2),
            7 => Some(Ncesrc4::Iom1ce3),
            8 => Some(Ncesrc4::Iom2ce0),
            9 => Some(Ncesrc4::Iom2ce1),
            10 => Some(Ncesrc4::Iom2ce2),
            11 => Some(Ncesrc4::Iom2ce3),
            12 => Some(Ncesrc4::Iom3ce0),
            13 => Some(Ncesrc4::Iom3ce1),
            14 => Some(Ncesrc4::Iom3ce2),
            15 => Some(Ncesrc4::Iom3ce3),
            16 => Some(Ncesrc4::Iom4ce0),
            17 => Some(Ncesrc4::Iom4ce1),
            18 => Some(Ncesrc4::Iom4ce2),
            19 => Some(Ncesrc4::Iom4ce3),
            20 => Some(Ncesrc4::Iom5ce0),
            21 => Some(Ncesrc4::Iom5ce1),
            22 => Some(Ncesrc4::Iom5ce2),
            23 => Some(Ncesrc4::Iom5ce3),
            24 => Some(Ncesrc4::Iom6ce0),
            25 => Some(Ncesrc4::Iom6ce1),
            26 => Some(Ncesrc4::Iom6ce2),
            27 => Some(Ncesrc4::Iom6ce3),
            28 => Some(Ncesrc4::Iom7ce0),
            29 => Some(Ncesrc4::Iom7ce1),
            30 => Some(Ncesrc4::Iom7ce2),
            31 => Some(Ncesrc4::Iom7ce3),
            32 => Some(Ncesrc4::Mspi0cen0),
            33 => Some(Ncesrc4::Mspi0cen1),
            34 => Some(Ncesrc4::Mspi1cen0),
            35 => Some(Ncesrc4::Mspi1cen1),
            36 => Some(Ncesrc4::Mspi2cen0),
            37 => Some(Ncesrc4::Mspi2cen1),
            38 => Some(Ncesrc4::DcDpiDe),
            39 => Some(Ncesrc4::DispContCsx),
            40 => Some(Ncesrc4::DcSpiCsN),
            41 => Some(Ncesrc4::DcQspiCsN),
            42 => Some(Ncesrc4::DcResx),
            _ => None,
        }
    }
    #[doc = "IOM 0 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom0ce0(&self) -> bool {
        *self == Ncesrc4::Iom0ce0
    }
    #[doc = "IOM 0 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom0ce1(&self) -> bool {
        *self == Ncesrc4::Iom0ce1
    }
    #[doc = "IOM 0 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom0ce2(&self) -> bool {
        *self == Ncesrc4::Iom0ce2
    }
    #[doc = "IOM 0 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom0ce3(&self) -> bool {
        *self == Ncesrc4::Iom0ce3
    }
    #[doc = "IOM 1 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom1ce0(&self) -> bool {
        *self == Ncesrc4::Iom1ce0
    }
    #[doc = "IOM 1 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom1ce1(&self) -> bool {
        *self == Ncesrc4::Iom1ce1
    }
    #[doc = "IOM 1 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom1ce2(&self) -> bool {
        *self == Ncesrc4::Iom1ce2
    }
    #[doc = "IOM 1 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom1ce3(&self) -> bool {
        *self == Ncesrc4::Iom1ce3
    }
    #[doc = "IOM 2 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom2ce0(&self) -> bool {
        *self == Ncesrc4::Iom2ce0
    }
    #[doc = "IOM 2 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom2ce1(&self) -> bool {
        *self == Ncesrc4::Iom2ce1
    }
    #[doc = "IOM 2 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom2ce2(&self) -> bool {
        *self == Ncesrc4::Iom2ce2
    }
    #[doc = "IOM 2 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom2ce3(&self) -> bool {
        *self == Ncesrc4::Iom2ce3
    }
    #[doc = "IOM 3 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom3ce0(&self) -> bool {
        *self == Ncesrc4::Iom3ce0
    }
    #[doc = "IOM 3 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom3ce1(&self) -> bool {
        *self == Ncesrc4::Iom3ce1
    }
    #[doc = "IOM 3 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom3ce2(&self) -> bool {
        *self == Ncesrc4::Iom3ce2
    }
    #[doc = "IOM 3 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom3ce3(&self) -> bool {
        *self == Ncesrc4::Iom3ce3
    }
    #[doc = "IOM 4 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom4ce0(&self) -> bool {
        *self == Ncesrc4::Iom4ce0
    }
    #[doc = "IOM 4 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom4ce1(&self) -> bool {
        *self == Ncesrc4::Iom4ce1
    }
    #[doc = "IOM 4 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom4ce2(&self) -> bool {
        *self == Ncesrc4::Iom4ce2
    }
    #[doc = "IOM 4 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom4ce3(&self) -> bool {
        *self == Ncesrc4::Iom4ce3
    }
    #[doc = "IOM 5 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom5ce0(&self) -> bool {
        *self == Ncesrc4::Iom5ce0
    }
    #[doc = "IOM 5 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom5ce1(&self) -> bool {
        *self == Ncesrc4::Iom5ce1
    }
    #[doc = "IOM 5 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom5ce2(&self) -> bool {
        *self == Ncesrc4::Iom5ce2
    }
    #[doc = "IOM 5 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom5ce3(&self) -> bool {
        *self == Ncesrc4::Iom5ce3
    }
    #[doc = "IOM 6 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom6ce0(&self) -> bool {
        *self == Ncesrc4::Iom6ce0
    }
    #[doc = "IOM 6 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom6ce1(&self) -> bool {
        *self == Ncesrc4::Iom6ce1
    }
    #[doc = "IOM 6 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom6ce2(&self) -> bool {
        *self == Ncesrc4::Iom6ce2
    }
    #[doc = "IOM 6 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom6ce3(&self) -> bool {
        *self == Ncesrc4::Iom6ce3
    }
    #[doc = "IOM 7 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom7ce0(&self) -> bool {
        *self == Ncesrc4::Iom7ce0
    }
    #[doc = "IOM 7 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom7ce1(&self) -> bool {
        *self == Ncesrc4::Iom7ce1
    }
    #[doc = "IOM 7 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom7ce2(&self) -> bool {
        *self == Ncesrc4::Iom7ce2
    }
    #[doc = "IOM 7 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom7ce3(&self) -> bool {
        *self == Ncesrc4::Iom7ce3
    }
    #[doc = "MSPI 0 NCE 0 module"]
    #[inline(always)]
    pub fn is_mspi0cen0(&self) -> bool {
        *self == Ncesrc4::Mspi0cen0
    }
    #[doc = "MSPI 0 NCE 1 module"]
    #[inline(always)]
    pub fn is_mspi0cen1(&self) -> bool {
        *self == Ncesrc4::Mspi0cen1
    }
    #[doc = "MSPI 1 NCE 0 module"]
    #[inline(always)]
    pub fn is_mspi1cen0(&self) -> bool {
        *self == Ncesrc4::Mspi1cen0
    }
    #[doc = "MSPI 1 NCE 1 module"]
    #[inline(always)]
    pub fn is_mspi1cen1(&self) -> bool {
        *self == Ncesrc4::Mspi1cen1
    }
    #[doc = "MSPI 2 NCE 0 module"]
    #[inline(always)]
    pub fn is_mspi2cen0(&self) -> bool {
        *self == Ncesrc4::Mspi2cen0
    }
    #[doc = "MSPI 2 NCE 1 module"]
    #[inline(always)]
    pub fn is_mspi2cen1(&self) -> bool {
        *self == Ncesrc4::Mspi2cen1
    }
    #[doc = "DC DPI DE module"]
    #[inline(always)]
    pub fn is_dc_dpi_de(&self) -> bool {
        *self == Ncesrc4::DcDpiDe
    }
    #[doc = "DISP CONT CSX module"]
    #[inline(always)]
    pub fn is_disp_cont_csx(&self) -> bool {
        *self == Ncesrc4::DispContCsx
    }
    #[doc = "DC SPI CS_N module"]
    #[inline(always)]
    pub fn is_dc_spi_cs_n(&self) -> bool {
        *self == Ncesrc4::DcSpiCsN
    }
    #[doc = "DC QSPI CS_N module"]
    #[inline(always)]
    pub fn is_dc_qspi_cs_n(&self) -> bool {
        *self == Ncesrc4::DcQspiCsN
    }
    #[doc = "DC module RESX"]
    #[inline(always)]
    pub fn is_dc_resx(&self) -> bool {
        *self == Ncesrc4::DcResx
    }
}
#[doc = "Field `NCESRC4` writer - IOMSTR/MSPI N Chip Select 4, DISP control signals DE, CSX, and CS. Polarity is determined by CE_POLARITY field"]
pub type Ncesrc4W<'a, REG> = crate::FieldWriter<'a, REG, 6, Ncesrc4>;
impl<'a, REG> Ncesrc4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "IOM 0 NCE 0 module"]
    #[inline(always)]
    pub fn iom0ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc4::Iom0ce0)
    }
    #[doc = "IOM 0 NCE 1 module"]
    #[inline(always)]
    pub fn iom0ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc4::Iom0ce1)
    }
    #[doc = "IOM 0 NCE 2 module"]
    #[inline(always)]
    pub fn iom0ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc4::Iom0ce2)
    }
    #[doc = "IOM 0 NCE 3 module"]
    #[inline(always)]
    pub fn iom0ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc4::Iom0ce3)
    }
    #[doc = "IOM 1 NCE 0 module"]
    #[inline(always)]
    pub fn iom1ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc4::Iom1ce0)
    }
    #[doc = "IOM 1 NCE 1 module"]
    #[inline(always)]
    pub fn iom1ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc4::Iom1ce1)
    }
    #[doc = "IOM 1 NCE 2 module"]
    #[inline(always)]
    pub fn iom1ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc4::Iom1ce2)
    }
    #[doc = "IOM 1 NCE 3 module"]
    #[inline(always)]
    pub fn iom1ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc4::Iom1ce3)
    }
    #[doc = "IOM 2 NCE 0 module"]
    #[inline(always)]
    pub fn iom2ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc4::Iom2ce0)
    }
    #[doc = "IOM 2 NCE 1 module"]
    #[inline(always)]
    pub fn iom2ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc4::Iom2ce1)
    }
    #[doc = "IOM 2 NCE 2 module"]
    #[inline(always)]
    pub fn iom2ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc4::Iom2ce2)
    }
    #[doc = "IOM 2 NCE 3 module"]
    #[inline(always)]
    pub fn iom2ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc4::Iom2ce3)
    }
    #[doc = "IOM 3 NCE 0 module"]
    #[inline(always)]
    pub fn iom3ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc4::Iom3ce0)
    }
    #[doc = "IOM 3 NCE 1 module"]
    #[inline(always)]
    pub fn iom3ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc4::Iom3ce1)
    }
    #[doc = "IOM 3 NCE 2 module"]
    #[inline(always)]
    pub fn iom3ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc4::Iom3ce2)
    }
    #[doc = "IOM 3 NCE 3 module"]
    #[inline(always)]
    pub fn iom3ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc4::Iom3ce3)
    }
    #[doc = "IOM 4 NCE 0 module"]
    #[inline(always)]
    pub fn iom4ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc4::Iom4ce0)
    }
    #[doc = "IOM 4 NCE 1 module"]
    #[inline(always)]
    pub fn iom4ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc4::Iom4ce1)
    }
    #[doc = "IOM 4 NCE 2 module"]
    #[inline(always)]
    pub fn iom4ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc4::Iom4ce2)
    }
    #[doc = "IOM 4 NCE 3 module"]
    #[inline(always)]
    pub fn iom4ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc4::Iom4ce3)
    }
    #[doc = "IOM 5 NCE 0 module"]
    #[inline(always)]
    pub fn iom5ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc4::Iom5ce0)
    }
    #[doc = "IOM 5 NCE 1 module"]
    #[inline(always)]
    pub fn iom5ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc4::Iom5ce1)
    }
    #[doc = "IOM 5 NCE 2 module"]
    #[inline(always)]
    pub fn iom5ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc4::Iom5ce2)
    }
    #[doc = "IOM 5 NCE 3 module"]
    #[inline(always)]
    pub fn iom5ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc4::Iom5ce3)
    }
    #[doc = "IOM 6 NCE 0 module"]
    #[inline(always)]
    pub fn iom6ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc4::Iom6ce0)
    }
    #[doc = "IOM 6 NCE 1 module"]
    #[inline(always)]
    pub fn iom6ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc4::Iom6ce1)
    }
    #[doc = "IOM 6 NCE 2 module"]
    #[inline(always)]
    pub fn iom6ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc4::Iom6ce2)
    }
    #[doc = "IOM 6 NCE 3 module"]
    #[inline(always)]
    pub fn iom6ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc4::Iom6ce3)
    }
    #[doc = "IOM 7 NCE 0 module"]
    #[inline(always)]
    pub fn iom7ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc4::Iom7ce0)
    }
    #[doc = "IOM 7 NCE 1 module"]
    #[inline(always)]
    pub fn iom7ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc4::Iom7ce1)
    }
    #[doc = "IOM 7 NCE 2 module"]
    #[inline(always)]
    pub fn iom7ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc4::Iom7ce2)
    }
    #[doc = "IOM 7 NCE 3 module"]
    #[inline(always)]
    pub fn iom7ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc4::Iom7ce3)
    }
    #[doc = "MSPI 0 NCE 0 module"]
    #[inline(always)]
    pub fn mspi0cen0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc4::Mspi0cen0)
    }
    #[doc = "MSPI 0 NCE 1 module"]
    #[inline(always)]
    pub fn mspi0cen1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc4::Mspi0cen1)
    }
    #[doc = "MSPI 1 NCE 0 module"]
    #[inline(always)]
    pub fn mspi1cen0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc4::Mspi1cen0)
    }
    #[doc = "MSPI 1 NCE 1 module"]
    #[inline(always)]
    pub fn mspi1cen1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc4::Mspi1cen1)
    }
    #[doc = "MSPI 2 NCE 0 module"]
    #[inline(always)]
    pub fn mspi2cen0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc4::Mspi2cen0)
    }
    #[doc = "MSPI 2 NCE 1 module"]
    #[inline(always)]
    pub fn mspi2cen1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc4::Mspi2cen1)
    }
    #[doc = "DC DPI DE module"]
    #[inline(always)]
    pub fn dc_dpi_de(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc4::DcDpiDe)
    }
    #[doc = "DISP CONT CSX module"]
    #[inline(always)]
    pub fn disp_cont_csx(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc4::DispContCsx)
    }
    #[doc = "DC SPI CS_N module"]
    #[inline(always)]
    pub fn dc_spi_cs_n(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc4::DcSpiCsN)
    }
    #[doc = "DC QSPI CS_N module"]
    #[inline(always)]
    pub fn dc_qspi_cs_n(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc4::DcQspiCsN)
    }
    #[doc = "DC module RESX"]
    #[inline(always)]
    pub fn dc_resx(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc4::DcResx)
    }
}
#[doc = "Polarity select for NCE for GPIO 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ncepol4 {
    #[doc = "0: Polarity is active low"]
    Low = 0,
    #[doc = "1: Polarity is active high"]
    High = 1,
}
impl From<Ncepol4> for bool {
    #[inline(always)]
    fn from(variant: Ncepol4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NCEPOL4` reader - Polarity select for NCE for GPIO 4"]
pub type Ncepol4R = crate::BitReader<Ncepol4>;
impl Ncepol4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ncepol4 {
        match self.bits {
            false => Ncepol4::Low,
            true => Ncepol4::High,
        }
    }
    #[doc = "Polarity is active low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Ncepol4::Low
    }
    #[doc = "Polarity is active high"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Ncepol4::High
    }
}
#[doc = "Field `NCEPOL4` writer - Polarity select for NCE for GPIO 4"]
pub type Ncepol4W<'a, REG> = crate::BitWriter<'a, REG, Ncepol4>;
impl<'a, REG> Ncepol4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Polarity is active low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Ncepol4::Low)
    }
    #[doc = "Polarity is active high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Ncepol4::High)
    }
}
#[doc = "Field `FIEN4` reader - Force input enable active regardless of function selected. Otherwise the selected function will enable the input only when needed"]
pub type Fien4R = crate::BitReader;
#[doc = "Field `FIEN4` writer - Force input enable active regardless of function selected. Otherwise the selected function will enable the input only when needed"]
pub type Fien4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FOEN4` reader - Force output enable active regardless of function selected. Otherwise the selected function will enable the output only when needed"]
pub type Foen4R = crate::BitReader;
#[doc = "Field `FOEN4` writer - Force output enable active regardless of function selected. Otherwise the selected function will enable the output only when needed"]
pub type Foen4W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Function select for GPIO pin 4"]
    #[inline(always)]
    pub fn fncsel4(&self) -> Fncsel4R {
        Fncsel4R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Input enable for GPIO 4"]
    #[inline(always)]
    pub fn inpen4(&self) -> Inpen4R {
        Inpen4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Return 0 for read data on GPIO 4"]
    #[inline(always)]
    pub fn rdzero4(&self) -> Rdzero4R {
        Rdzero4R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Interrupt enable for GPIO 4"]
    #[inline(always)]
    pub fn irpten4(&self) -> Irpten4R {
        Irpten4R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Pin IO mode selection for GPIO pin 4"]
    #[inline(always)]
    pub fn outcfg4(&self) -> Outcfg4R {
        Outcfg4R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Drive strength selection for GPIO 4"]
    #[inline(always)]
    pub fn ds4(&self) -> Ds4R {
        Ds4R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - Configure the slew rate"]
    #[inline(always)]
    pub fn sr4(&self) -> Sr4R {
        Sr4R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - Pullup/Pulldown configuration for GPIO 4"]
    #[inline(always)]
    pub fn pullcfg4(&self) -> Pullcfg4R {
        Pullcfg4R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:21 - IOMSTR/MSPI N Chip Select 4, DISP control signals DE, CSX, and CS. Polarity is determined by CE_POLARITY field"]
    #[inline(always)]
    pub fn ncesrc4(&self) -> Ncesrc4R {
        Ncesrc4R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 22 - Polarity select for NCE for GPIO 4"]
    #[inline(always)]
    pub fn ncepol4(&self) -> Ncepol4R {
        Ncepol4R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 26 - Force input enable active regardless of function selected. Otherwise the selected function will enable the input only when needed"]
    #[inline(always)]
    pub fn fien4(&self) -> Fien4R {
        Fien4R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Force output enable active regardless of function selected. Otherwise the selected function will enable the output only when needed"]
    #[inline(always)]
    pub fn foen4(&self) -> Foen4R {
        Foen4R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Function select for GPIO pin 4"]
    #[inline(always)]
    #[must_use]
    pub fn fncsel4(&mut self) -> Fncsel4W<Pincfg4Spec> {
        Fncsel4W::new(self, 0)
    }
    #[doc = "Bit 4 - Input enable for GPIO 4"]
    #[inline(always)]
    #[must_use]
    pub fn inpen4(&mut self) -> Inpen4W<Pincfg4Spec> {
        Inpen4W::new(self, 4)
    }
    #[doc = "Bit 5 - Return 0 for read data on GPIO 4"]
    #[inline(always)]
    #[must_use]
    pub fn rdzero4(&mut self) -> Rdzero4W<Pincfg4Spec> {
        Rdzero4W::new(self, 5)
    }
    #[doc = "Bits 6:7 - Interrupt enable for GPIO 4"]
    #[inline(always)]
    #[must_use]
    pub fn irpten4(&mut self) -> Irpten4W<Pincfg4Spec> {
        Irpten4W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Pin IO mode selection for GPIO pin 4"]
    #[inline(always)]
    #[must_use]
    pub fn outcfg4(&mut self) -> Outcfg4W<Pincfg4Spec> {
        Outcfg4W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Drive strength selection for GPIO 4"]
    #[inline(always)]
    #[must_use]
    pub fn ds4(&mut self) -> Ds4W<Pincfg4Spec> {
        Ds4W::new(self, 10)
    }
    #[doc = "Bit 12 - Configure the slew rate"]
    #[inline(always)]
    #[must_use]
    pub fn sr4(&mut self) -> Sr4W<Pincfg4Spec> {
        Sr4W::new(self, 12)
    }
    #[doc = "Bits 13:15 - Pullup/Pulldown configuration for GPIO 4"]
    #[inline(always)]
    #[must_use]
    pub fn pullcfg4(&mut self) -> Pullcfg4W<Pincfg4Spec> {
        Pullcfg4W::new(self, 13)
    }
    #[doc = "Bits 16:21 - IOMSTR/MSPI N Chip Select 4, DISP control signals DE, CSX, and CS. Polarity is determined by CE_POLARITY field"]
    #[inline(always)]
    #[must_use]
    pub fn ncesrc4(&mut self) -> Ncesrc4W<Pincfg4Spec> {
        Ncesrc4W::new(self, 16)
    }
    #[doc = "Bit 22 - Polarity select for NCE for GPIO 4"]
    #[inline(always)]
    #[must_use]
    pub fn ncepol4(&mut self) -> Ncepol4W<Pincfg4Spec> {
        Ncepol4W::new(self, 22)
    }
    #[doc = "Bit 26 - Force input enable active regardless of function selected. Otherwise the selected function will enable the input only when needed"]
    #[inline(always)]
    #[must_use]
    pub fn fien4(&mut self) -> Fien4W<Pincfg4Spec> {
        Fien4W::new(self, 26)
    }
    #[doc = "Bit 27 - Force output enable active regardless of function selected. Otherwise the selected function will enable the output only when needed"]
    #[inline(always)]
    #[must_use]
    pub fn foen4(&mut self) -> Foen4W<Pincfg4Spec> {
        Foen4W::new(self, 27)
    }
}
#[doc = "Controls the operation of GPIO pin 4.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pincfg4Spec;
impl crate::RegisterSpec for Pincfg4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pincfg4::R`](R) reader structure"]
impl crate::Readable for Pincfg4Spec {}
#[doc = "`write(|w| ..)` method takes [`pincfg4::W`](W) writer structure"]
impl crate::Writable for Pincfg4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PINCFG4 to value 0x03"]
impl crate::Resettable for Pincfg4Spec {
    const RESET_VALUE: u32 = 0x03;
}
