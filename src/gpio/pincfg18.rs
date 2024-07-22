#[doc = "Register `PINCFG18` reader"]
pub type R = crate::R<Pincfg18Spec>;
#[doc = "Register `PINCFG18` writer"]
pub type W = crate::W<Pincfg18Spec>;
#[doc = "Function select for GPIO pin 18\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fncsel18 {
    #[doc = "0: Analog to Digital Converter SE IN1"]
    Adcse1 = 0,
    #[doc = "1: Ambiq Analog test I/O - Unbuffered"]
    Anatest2 = 1,
    #[doc = "2: Bidirectional I2S L/R clock. Operates in output mode in master mode and input mode for slave mode. (I2S Master/Slave 2)"]
    I2s1Ws = 2,
    #[doc = "3: General purpose I/O"]
    Gpio = 3,
    #[doc = "4: UART Clear to Send (CTS) (UART 0)"]
    Uart0cts = 4,
    #[doc = "5: UART Clear to Send (CTS) (UART 1)"]
    Uart1cts = 5,
    #[doc = "6: Timer/Counter input or output; Selection of direction is done via CTIMER register settings."]
    Ct18 = 6,
    #[doc = "7: IOMSTR/MSPI N Chip Select. Polarity is determined by CE_POLARITY field"]
    Nce18 = 7,
    #[doc = "8: Observation bus bit 2"]
    Obsbus2 = 8,
    #[doc = "9: Reserved selection. Operation unknown if selected."]
    Reserved9 = 9,
    #[doc = "10: Reserved selection. Operation unknown if selected."]
    Reserved10 = 10,
    #[doc = "11: Fast PIO"]
    Fpio = 11,
    #[doc = "12: Internal function (Flash Bist)"]
    FlbTms = 12,
    #[doc = "13: Internal function (Flash parallel load)"]
    FlloadData = 13,
    #[doc = "14: Internal function (MBIST)"]
    MdaHfrcExt = 14,
    #[doc = "15: Internal function (SCAN)"]
    Scanin1 = 15,
}
impl From<Fncsel18> for u8 {
    #[inline(always)]
    fn from(variant: Fncsel18) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fncsel18 {
    type Ux = u8;
}
impl crate::IsEnum for Fncsel18 {}
#[doc = "Field `FNCSEL18` reader - Function select for GPIO pin 18"]
pub type Fncsel18R = crate::FieldReader<Fncsel18>;
impl Fncsel18R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fncsel18 {
        match self.bits {
            0 => Fncsel18::Adcse1,
            1 => Fncsel18::Anatest2,
            2 => Fncsel18::I2s1Ws,
            3 => Fncsel18::Gpio,
            4 => Fncsel18::Uart0cts,
            5 => Fncsel18::Uart1cts,
            6 => Fncsel18::Ct18,
            7 => Fncsel18::Nce18,
            8 => Fncsel18::Obsbus2,
            9 => Fncsel18::Reserved9,
            10 => Fncsel18::Reserved10,
            11 => Fncsel18::Fpio,
            12 => Fncsel18::FlbTms,
            13 => Fncsel18::FlloadData,
            14 => Fncsel18::MdaHfrcExt,
            15 => Fncsel18::Scanin1,
            _ => unreachable!(),
        }
    }
    #[doc = "Analog to Digital Converter SE IN1"]
    #[inline(always)]
    pub fn is_adcse1(&self) -> bool {
        *self == Fncsel18::Adcse1
    }
    #[doc = "Ambiq Analog test I/O - Unbuffered"]
    #[inline(always)]
    pub fn is_anatest2(&self) -> bool {
        *self == Fncsel18::Anatest2
    }
    #[doc = "Bidirectional I2S L/R clock. Operates in output mode in master mode and input mode for slave mode. (I2S Master/Slave 2)"]
    #[inline(always)]
    pub fn is_i2s1_ws(&self) -> bool {
        *self == Fncsel18::I2s1Ws
    }
    #[doc = "General purpose I/O"]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == Fncsel18::Gpio
    }
    #[doc = "UART Clear to Send (CTS) (UART 0)"]
    #[inline(always)]
    pub fn is_uart0cts(&self) -> bool {
        *self == Fncsel18::Uart0cts
    }
    #[doc = "UART Clear to Send (CTS) (UART 1)"]
    #[inline(always)]
    pub fn is_uart1cts(&self) -> bool {
        *self == Fncsel18::Uart1cts
    }
    #[doc = "Timer/Counter input or output; Selection of direction is done via CTIMER register settings."]
    #[inline(always)]
    pub fn is_ct18(&self) -> bool {
        *self == Fncsel18::Ct18
    }
    #[doc = "IOMSTR/MSPI N Chip Select. Polarity is determined by CE_POLARITY field"]
    #[inline(always)]
    pub fn is_nce18(&self) -> bool {
        *self == Fncsel18::Nce18
    }
    #[doc = "Observation bus bit 2"]
    #[inline(always)]
    pub fn is_obsbus2(&self) -> bool {
        *self == Fncsel18::Obsbus2
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved9(&self) -> bool {
        *self == Fncsel18::Reserved9
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved10(&self) -> bool {
        *self == Fncsel18::Reserved10
    }
    #[doc = "Fast PIO"]
    #[inline(always)]
    pub fn is_fpio(&self) -> bool {
        *self == Fncsel18::Fpio
    }
    #[doc = "Internal function (Flash Bist)"]
    #[inline(always)]
    pub fn is_flb_tms(&self) -> bool {
        *self == Fncsel18::FlbTms
    }
    #[doc = "Internal function (Flash parallel load)"]
    #[inline(always)]
    pub fn is_flload_data(&self) -> bool {
        *self == Fncsel18::FlloadData
    }
    #[doc = "Internal function (MBIST)"]
    #[inline(always)]
    pub fn is_mda_hfrc_ext(&self) -> bool {
        *self == Fncsel18::MdaHfrcExt
    }
    #[doc = "Internal function (SCAN)"]
    #[inline(always)]
    pub fn is_scanin1(&self) -> bool {
        *self == Fncsel18::Scanin1
    }
}
#[doc = "Field `FNCSEL18` writer - Function select for GPIO pin 18"]
pub type Fncsel18W<'a, REG> = crate::FieldWriter<'a, REG, 4, Fncsel18, crate::Safe>;
impl<'a, REG> Fncsel18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Analog to Digital Converter SE IN1"]
    #[inline(always)]
    pub fn adcse1(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel18::Adcse1)
    }
    #[doc = "Ambiq Analog test I/O - Unbuffered"]
    #[inline(always)]
    pub fn anatest2(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel18::Anatest2)
    }
    #[doc = "Bidirectional I2S L/R clock. Operates in output mode in master mode and input mode for slave mode. (I2S Master/Slave 2)"]
    #[inline(always)]
    pub fn i2s1_ws(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel18::I2s1Ws)
    }
    #[doc = "General purpose I/O"]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel18::Gpio)
    }
    #[doc = "UART Clear to Send (CTS) (UART 0)"]
    #[inline(always)]
    pub fn uart0cts(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel18::Uart0cts)
    }
    #[doc = "UART Clear to Send (CTS) (UART 1)"]
    #[inline(always)]
    pub fn uart1cts(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel18::Uart1cts)
    }
    #[doc = "Timer/Counter input or output; Selection of direction is done via CTIMER register settings."]
    #[inline(always)]
    pub fn ct18(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel18::Ct18)
    }
    #[doc = "IOMSTR/MSPI N Chip Select. Polarity is determined by CE_POLARITY field"]
    #[inline(always)]
    pub fn nce18(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel18::Nce18)
    }
    #[doc = "Observation bus bit 2"]
    #[inline(always)]
    pub fn obsbus2(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel18::Obsbus2)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved9(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel18::Reserved9)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved10(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel18::Reserved10)
    }
    #[doc = "Fast PIO"]
    #[inline(always)]
    pub fn fpio(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel18::Fpio)
    }
    #[doc = "Internal function (Flash Bist)"]
    #[inline(always)]
    pub fn flb_tms(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel18::FlbTms)
    }
    #[doc = "Internal function (Flash parallel load)"]
    #[inline(always)]
    pub fn flload_data(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel18::FlloadData)
    }
    #[doc = "Internal function (MBIST)"]
    #[inline(always)]
    pub fn mda_hfrc_ext(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel18::MdaHfrcExt)
    }
    #[doc = "Internal function (SCAN)"]
    #[inline(always)]
    pub fn scanin1(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel18::Scanin1)
    }
}
#[doc = "Field `INPEN18` reader - Input enable for GPIO 18"]
pub type Inpen18R = crate::BitReader;
#[doc = "Field `INPEN18` writer - Input enable for GPIO 18"]
pub type Inpen18W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDZERO18` reader - Return 0 for read data on GPIO 18"]
pub type Rdzero18R = crate::BitReader;
#[doc = "Field `RDZERO18` writer - Return 0 for read data on GPIO 18"]
pub type Rdzero18W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Interrupt enable for GPIO 18\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Irpten18 {
    #[doc = "0: Interrupts are disabled for this GPIO"]
    Dis = 0,
    #[doc = "1: Interrupts are enabled for falling edge transition on this GPIO"]
    Intfall = 1,
    #[doc = "2: Interrupts are enabled for rising edge transitions on this GPIO"]
    Intrise = 2,
    #[doc = "3: Interrupts are enabled for any edge transition on this GPIO"]
    Intany = 3,
}
impl From<Irpten18> for u8 {
    #[inline(always)]
    fn from(variant: Irpten18) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Irpten18 {
    type Ux = u8;
}
impl crate::IsEnum for Irpten18 {}
#[doc = "Field `IRPTEN18` reader - Interrupt enable for GPIO 18"]
pub type Irpten18R = crate::FieldReader<Irpten18>;
impl Irpten18R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irpten18 {
        match self.bits {
            0 => Irpten18::Dis,
            1 => Irpten18::Intfall,
            2 => Irpten18::Intrise,
            3 => Irpten18::Intany,
            _ => unreachable!(),
        }
    }
    #[doc = "Interrupts are disabled for this GPIO"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Irpten18::Dis
    }
    #[doc = "Interrupts are enabled for falling edge transition on this GPIO"]
    #[inline(always)]
    pub fn is_intfall(&self) -> bool {
        *self == Irpten18::Intfall
    }
    #[doc = "Interrupts are enabled for rising edge transitions on this GPIO"]
    #[inline(always)]
    pub fn is_intrise(&self) -> bool {
        *self == Irpten18::Intrise
    }
    #[doc = "Interrupts are enabled for any edge transition on this GPIO"]
    #[inline(always)]
    pub fn is_intany(&self) -> bool {
        *self == Irpten18::Intany
    }
}
#[doc = "Field `IRPTEN18` writer - Interrupt enable for GPIO 18"]
pub type Irpten18W<'a, REG> = crate::FieldWriter<'a, REG, 2, Irpten18, crate::Safe>;
impl<'a, REG> Irpten18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Interrupts are disabled for this GPIO"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten18::Dis)
    }
    #[doc = "Interrupts are enabled for falling edge transition on this GPIO"]
    #[inline(always)]
    pub fn intfall(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten18::Intfall)
    }
    #[doc = "Interrupts are enabled for rising edge transitions on this GPIO"]
    #[inline(always)]
    pub fn intrise(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten18::Intrise)
    }
    #[doc = "Interrupts are enabled for any edge transition on this GPIO"]
    #[inline(always)]
    pub fn intany(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten18::Intany)
    }
}
#[doc = "Pin IO mode selection for GPIO pin 18\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Outcfg18 {
    #[doc = "0: Output Disabled"]
    Dis = 0,
    #[doc = "1: Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    Pushpull = 1,
    #[doc = "2: Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    Od = 2,
    #[doc = "3: Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    Ts = 3,
}
impl From<Outcfg18> for u8 {
    #[inline(always)]
    fn from(variant: Outcfg18) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Outcfg18 {
    type Ux = u8;
}
impl crate::IsEnum for Outcfg18 {}
#[doc = "Field `OUTCFG18` reader - Pin IO mode selection for GPIO pin 18"]
pub type Outcfg18R = crate::FieldReader<Outcfg18>;
impl Outcfg18R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Outcfg18 {
        match self.bits {
            0 => Outcfg18::Dis,
            1 => Outcfg18::Pushpull,
            2 => Outcfg18::Od,
            3 => Outcfg18::Ts,
            _ => unreachable!(),
        }
    }
    #[doc = "Output Disabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Outcfg18::Dis
    }
    #[doc = "Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == Outcfg18::Pushpull
    }
    #[doc = "Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == Outcfg18::Od
    }
    #[doc = "Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == Outcfg18::Ts
    }
}
#[doc = "Field `OUTCFG18` writer - Pin IO mode selection for GPIO pin 18"]
pub type Outcfg18W<'a, REG> = crate::FieldWriter<'a, REG, 2, Outcfg18, crate::Safe>;
impl<'a, REG> Outcfg18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Output Disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg18::Dis)
    }
    #[doc = "Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg18::Pushpull)
    }
    #[doc = "Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    #[inline(always)]
    pub fn od(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg18::Od)
    }
    #[doc = "Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    #[inline(always)]
    pub fn ts(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg18::Ts)
    }
}
#[doc = "Drive strength selection for GPIO 18\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ds18 {
    #[doc = "0: 0.1x output driver selected"]
    _0p1x = 0,
    #[doc = "1: 0.5x output driver selected"]
    _0p5x = 1,
}
impl From<Ds18> for u8 {
    #[inline(always)]
    fn from(variant: Ds18) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ds18 {
    type Ux = u8;
}
impl crate::IsEnum for Ds18 {}
#[doc = "Field `DS18` reader - Drive strength selection for GPIO 18"]
pub type Ds18R = crate::FieldReader<Ds18>;
impl Ds18R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ds18> {
        match self.bits {
            0 => Some(Ds18::_0p1x),
            1 => Some(Ds18::_0p5x),
            _ => None,
        }
    }
    #[doc = "0.1x output driver selected"]
    #[inline(always)]
    pub fn is_0p1x(&self) -> bool {
        *self == Ds18::_0p1x
    }
    #[doc = "0.5x output driver selected"]
    #[inline(always)]
    pub fn is_0p5x(&self) -> bool {
        *self == Ds18::_0p5x
    }
}
#[doc = "Field `DS18` writer - Drive strength selection for GPIO 18"]
pub type Ds18W<'a, REG> = crate::FieldWriter<'a, REG, 2, Ds18>;
impl<'a, REG> Ds18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "0.1x output driver selected"]
    #[inline(always)]
    pub fn _0p1x(self) -> &'a mut crate::W<REG> {
        self.variant(Ds18::_0p1x)
    }
    #[doc = "0.5x output driver selected"]
    #[inline(always)]
    pub fn _0p5x(self) -> &'a mut crate::W<REG> {
        self.variant(Ds18::_0p5x)
    }
}
#[doc = "Field `SR18` reader - Configure the slew rate"]
pub type Sr18R = crate::BitReader;
#[doc = "Field `SR18` writer - Configure the slew rate"]
pub type Sr18W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Pullup/Pulldown configuration for GPIO 18\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pullcfg18 {
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
impl From<Pullcfg18> for u8 {
    #[inline(always)]
    fn from(variant: Pullcfg18) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pullcfg18 {
    type Ux = u8;
}
impl crate::IsEnum for Pullcfg18 {}
#[doc = "Field `PULLCFG18` reader - Pullup/Pulldown configuration for GPIO 18"]
pub type Pullcfg18R = crate::FieldReader<Pullcfg18>;
impl Pullcfg18R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pullcfg18 {
        match self.bits {
            0 => Pullcfg18::Dis,
            1 => Pullcfg18::Pd50k,
            2 => Pullcfg18::Pu15k,
            3 => Pullcfg18::Pu6k,
            4 => Pullcfg18::Pu12k,
            5 => Pullcfg18::Pu24k,
            6 => Pullcfg18::Pu50k,
            7 => Pullcfg18::Pu100k,
            _ => unreachable!(),
        }
    }
    #[doc = "No pullup or pulldown selected"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Pullcfg18::Dis
    }
    #[doc = "50K Pulldown selected"]
    #[inline(always)]
    pub fn is_pd50k(&self) -> bool {
        *self == Pullcfg18::Pd50k
    }
    #[doc = "1.5K Pullup selected"]
    #[inline(always)]
    pub fn is_pu15k(&self) -> bool {
        *self == Pullcfg18::Pu15k
    }
    #[doc = "6K Pullup selected"]
    #[inline(always)]
    pub fn is_pu6k(&self) -> bool {
        *self == Pullcfg18::Pu6k
    }
    #[doc = "12K Pullup selected"]
    #[inline(always)]
    pub fn is_pu12k(&self) -> bool {
        *self == Pullcfg18::Pu12k
    }
    #[doc = "24K Pullup selected"]
    #[inline(always)]
    pub fn is_pu24k(&self) -> bool {
        *self == Pullcfg18::Pu24k
    }
    #[doc = "50K Pullup selected"]
    #[inline(always)]
    pub fn is_pu50k(&self) -> bool {
        *self == Pullcfg18::Pu50k
    }
    #[doc = "100K Pullup selected"]
    #[inline(always)]
    pub fn is_pu100k(&self) -> bool {
        *self == Pullcfg18::Pu100k
    }
}
#[doc = "Field `PULLCFG18` writer - Pullup/Pulldown configuration for GPIO 18"]
pub type Pullcfg18W<'a, REG> = crate::FieldWriter<'a, REG, 3, Pullcfg18, crate::Safe>;
impl<'a, REG> Pullcfg18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No pullup or pulldown selected"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg18::Dis)
    }
    #[doc = "50K Pulldown selected"]
    #[inline(always)]
    pub fn pd50k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg18::Pd50k)
    }
    #[doc = "1.5K Pullup selected"]
    #[inline(always)]
    pub fn pu15k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg18::Pu15k)
    }
    #[doc = "6K Pullup selected"]
    #[inline(always)]
    pub fn pu6k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg18::Pu6k)
    }
    #[doc = "12K Pullup selected"]
    #[inline(always)]
    pub fn pu12k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg18::Pu12k)
    }
    #[doc = "24K Pullup selected"]
    #[inline(always)]
    pub fn pu24k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg18::Pu24k)
    }
    #[doc = "50K Pullup selected"]
    #[inline(always)]
    pub fn pu50k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg18::Pu50k)
    }
    #[doc = "100K Pullup selected"]
    #[inline(always)]
    pub fn pu100k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg18::Pu100k)
    }
}
#[doc = "IOMSTR/MSPI N Chip Select 18, DISP control signals DE, CSX, and CS. Polarity is determined by CE_POLARITY field\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ncesrc18 {
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
impl From<Ncesrc18> for u8 {
    #[inline(always)]
    fn from(variant: Ncesrc18) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ncesrc18 {
    type Ux = u8;
}
impl crate::IsEnum for Ncesrc18 {}
#[doc = "Field `NCESRC18` reader - IOMSTR/MSPI N Chip Select 18, DISP control signals DE, CSX, and CS. Polarity is determined by CE_POLARITY field"]
pub type Ncesrc18R = crate::FieldReader<Ncesrc18>;
impl Ncesrc18R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ncesrc18> {
        match self.bits {
            0 => Some(Ncesrc18::Iom0ce0),
            1 => Some(Ncesrc18::Iom0ce1),
            2 => Some(Ncesrc18::Iom0ce2),
            3 => Some(Ncesrc18::Iom0ce3),
            4 => Some(Ncesrc18::Iom1ce0),
            5 => Some(Ncesrc18::Iom1ce1),
            6 => Some(Ncesrc18::Iom1ce2),
            7 => Some(Ncesrc18::Iom1ce3),
            8 => Some(Ncesrc18::Iom2ce0),
            9 => Some(Ncesrc18::Iom2ce1),
            10 => Some(Ncesrc18::Iom2ce2),
            11 => Some(Ncesrc18::Iom2ce3),
            12 => Some(Ncesrc18::Iom3ce0),
            13 => Some(Ncesrc18::Iom3ce1),
            14 => Some(Ncesrc18::Iom3ce2),
            15 => Some(Ncesrc18::Iom3ce3),
            16 => Some(Ncesrc18::Iom4ce0),
            17 => Some(Ncesrc18::Iom4ce1),
            18 => Some(Ncesrc18::Iom4ce2),
            19 => Some(Ncesrc18::Iom4ce3),
            20 => Some(Ncesrc18::Iom5ce0),
            21 => Some(Ncesrc18::Iom5ce1),
            22 => Some(Ncesrc18::Iom5ce2),
            23 => Some(Ncesrc18::Iom5ce3),
            24 => Some(Ncesrc18::Iom6ce0),
            25 => Some(Ncesrc18::Iom6ce1),
            26 => Some(Ncesrc18::Iom6ce2),
            27 => Some(Ncesrc18::Iom6ce3),
            28 => Some(Ncesrc18::Iom7ce0),
            29 => Some(Ncesrc18::Iom7ce1),
            30 => Some(Ncesrc18::Iom7ce2),
            31 => Some(Ncesrc18::Iom7ce3),
            32 => Some(Ncesrc18::Mspi0cen0),
            33 => Some(Ncesrc18::Mspi0cen1),
            34 => Some(Ncesrc18::Mspi1cen0),
            35 => Some(Ncesrc18::Mspi1cen1),
            36 => Some(Ncesrc18::Mspi2cen0),
            37 => Some(Ncesrc18::Mspi2cen1),
            38 => Some(Ncesrc18::DcDpiDe),
            39 => Some(Ncesrc18::DispContCsx),
            40 => Some(Ncesrc18::DcSpiCsN),
            41 => Some(Ncesrc18::DcQspiCsN),
            42 => Some(Ncesrc18::DcResx),
            _ => None,
        }
    }
    #[doc = "IOM 0 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom0ce0(&self) -> bool {
        *self == Ncesrc18::Iom0ce0
    }
    #[doc = "IOM 0 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom0ce1(&self) -> bool {
        *self == Ncesrc18::Iom0ce1
    }
    #[doc = "IOM 0 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom0ce2(&self) -> bool {
        *self == Ncesrc18::Iom0ce2
    }
    #[doc = "IOM 0 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom0ce3(&self) -> bool {
        *self == Ncesrc18::Iom0ce3
    }
    #[doc = "IOM 1 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom1ce0(&self) -> bool {
        *self == Ncesrc18::Iom1ce0
    }
    #[doc = "IOM 1 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom1ce1(&self) -> bool {
        *self == Ncesrc18::Iom1ce1
    }
    #[doc = "IOM 1 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom1ce2(&self) -> bool {
        *self == Ncesrc18::Iom1ce2
    }
    #[doc = "IOM 1 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom1ce3(&self) -> bool {
        *self == Ncesrc18::Iom1ce3
    }
    #[doc = "IOM 2 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom2ce0(&self) -> bool {
        *self == Ncesrc18::Iom2ce0
    }
    #[doc = "IOM 2 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom2ce1(&self) -> bool {
        *self == Ncesrc18::Iom2ce1
    }
    #[doc = "IOM 2 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom2ce2(&self) -> bool {
        *self == Ncesrc18::Iom2ce2
    }
    #[doc = "IOM 2 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom2ce3(&self) -> bool {
        *self == Ncesrc18::Iom2ce3
    }
    #[doc = "IOM 3 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom3ce0(&self) -> bool {
        *self == Ncesrc18::Iom3ce0
    }
    #[doc = "IOM 3 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom3ce1(&self) -> bool {
        *self == Ncesrc18::Iom3ce1
    }
    #[doc = "IOM 3 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom3ce2(&self) -> bool {
        *self == Ncesrc18::Iom3ce2
    }
    #[doc = "IOM 3 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom3ce3(&self) -> bool {
        *self == Ncesrc18::Iom3ce3
    }
    #[doc = "IOM 4 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom4ce0(&self) -> bool {
        *self == Ncesrc18::Iom4ce0
    }
    #[doc = "IOM 4 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom4ce1(&self) -> bool {
        *self == Ncesrc18::Iom4ce1
    }
    #[doc = "IOM 4 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom4ce2(&self) -> bool {
        *self == Ncesrc18::Iom4ce2
    }
    #[doc = "IOM 4 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom4ce3(&self) -> bool {
        *self == Ncesrc18::Iom4ce3
    }
    #[doc = "IOM 5 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom5ce0(&self) -> bool {
        *self == Ncesrc18::Iom5ce0
    }
    #[doc = "IOM 5 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom5ce1(&self) -> bool {
        *self == Ncesrc18::Iom5ce1
    }
    #[doc = "IOM 5 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom5ce2(&self) -> bool {
        *self == Ncesrc18::Iom5ce2
    }
    #[doc = "IOM 5 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom5ce3(&self) -> bool {
        *self == Ncesrc18::Iom5ce3
    }
    #[doc = "IOM 6 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom6ce0(&self) -> bool {
        *self == Ncesrc18::Iom6ce0
    }
    #[doc = "IOM 6 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom6ce1(&self) -> bool {
        *self == Ncesrc18::Iom6ce1
    }
    #[doc = "IOM 6 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom6ce2(&self) -> bool {
        *self == Ncesrc18::Iom6ce2
    }
    #[doc = "IOM 6 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom6ce3(&self) -> bool {
        *self == Ncesrc18::Iom6ce3
    }
    #[doc = "IOM 7 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom7ce0(&self) -> bool {
        *self == Ncesrc18::Iom7ce0
    }
    #[doc = "IOM 7 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom7ce1(&self) -> bool {
        *self == Ncesrc18::Iom7ce1
    }
    #[doc = "IOM 7 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom7ce2(&self) -> bool {
        *self == Ncesrc18::Iom7ce2
    }
    #[doc = "IOM 7 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom7ce3(&self) -> bool {
        *self == Ncesrc18::Iom7ce3
    }
    #[doc = "MSPI 0 NCE 0 module"]
    #[inline(always)]
    pub fn is_mspi0cen0(&self) -> bool {
        *self == Ncesrc18::Mspi0cen0
    }
    #[doc = "MSPI 0 NCE 1 module"]
    #[inline(always)]
    pub fn is_mspi0cen1(&self) -> bool {
        *self == Ncesrc18::Mspi0cen1
    }
    #[doc = "MSPI 1 NCE 0 module"]
    #[inline(always)]
    pub fn is_mspi1cen0(&self) -> bool {
        *self == Ncesrc18::Mspi1cen0
    }
    #[doc = "MSPI 1 NCE 1 module"]
    #[inline(always)]
    pub fn is_mspi1cen1(&self) -> bool {
        *self == Ncesrc18::Mspi1cen1
    }
    #[doc = "MSPI 2 NCE 0 module"]
    #[inline(always)]
    pub fn is_mspi2cen0(&self) -> bool {
        *self == Ncesrc18::Mspi2cen0
    }
    #[doc = "MSPI 2 NCE 1 module"]
    #[inline(always)]
    pub fn is_mspi2cen1(&self) -> bool {
        *self == Ncesrc18::Mspi2cen1
    }
    #[doc = "DC DPI DE module"]
    #[inline(always)]
    pub fn is_dc_dpi_de(&self) -> bool {
        *self == Ncesrc18::DcDpiDe
    }
    #[doc = "DISP CONT CSX module"]
    #[inline(always)]
    pub fn is_disp_cont_csx(&self) -> bool {
        *self == Ncesrc18::DispContCsx
    }
    #[doc = "DC SPI CS_N module"]
    #[inline(always)]
    pub fn is_dc_spi_cs_n(&self) -> bool {
        *self == Ncesrc18::DcSpiCsN
    }
    #[doc = "DC QSPI CS_N module"]
    #[inline(always)]
    pub fn is_dc_qspi_cs_n(&self) -> bool {
        *self == Ncesrc18::DcQspiCsN
    }
    #[doc = "DC module RESX"]
    #[inline(always)]
    pub fn is_dc_resx(&self) -> bool {
        *self == Ncesrc18::DcResx
    }
}
#[doc = "Field `NCESRC18` writer - IOMSTR/MSPI N Chip Select 18, DISP control signals DE, CSX, and CS. Polarity is determined by CE_POLARITY field"]
pub type Ncesrc18W<'a, REG> = crate::FieldWriter<'a, REG, 6, Ncesrc18>;
impl<'a, REG> Ncesrc18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "IOM 0 NCE 0 module"]
    #[inline(always)]
    pub fn iom0ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc18::Iom0ce0)
    }
    #[doc = "IOM 0 NCE 1 module"]
    #[inline(always)]
    pub fn iom0ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc18::Iom0ce1)
    }
    #[doc = "IOM 0 NCE 2 module"]
    #[inline(always)]
    pub fn iom0ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc18::Iom0ce2)
    }
    #[doc = "IOM 0 NCE 3 module"]
    #[inline(always)]
    pub fn iom0ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc18::Iom0ce3)
    }
    #[doc = "IOM 1 NCE 0 module"]
    #[inline(always)]
    pub fn iom1ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc18::Iom1ce0)
    }
    #[doc = "IOM 1 NCE 1 module"]
    #[inline(always)]
    pub fn iom1ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc18::Iom1ce1)
    }
    #[doc = "IOM 1 NCE 2 module"]
    #[inline(always)]
    pub fn iom1ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc18::Iom1ce2)
    }
    #[doc = "IOM 1 NCE 3 module"]
    #[inline(always)]
    pub fn iom1ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc18::Iom1ce3)
    }
    #[doc = "IOM 2 NCE 0 module"]
    #[inline(always)]
    pub fn iom2ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc18::Iom2ce0)
    }
    #[doc = "IOM 2 NCE 1 module"]
    #[inline(always)]
    pub fn iom2ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc18::Iom2ce1)
    }
    #[doc = "IOM 2 NCE 2 module"]
    #[inline(always)]
    pub fn iom2ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc18::Iom2ce2)
    }
    #[doc = "IOM 2 NCE 3 module"]
    #[inline(always)]
    pub fn iom2ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc18::Iom2ce3)
    }
    #[doc = "IOM 3 NCE 0 module"]
    #[inline(always)]
    pub fn iom3ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc18::Iom3ce0)
    }
    #[doc = "IOM 3 NCE 1 module"]
    #[inline(always)]
    pub fn iom3ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc18::Iom3ce1)
    }
    #[doc = "IOM 3 NCE 2 module"]
    #[inline(always)]
    pub fn iom3ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc18::Iom3ce2)
    }
    #[doc = "IOM 3 NCE 3 module"]
    #[inline(always)]
    pub fn iom3ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc18::Iom3ce3)
    }
    #[doc = "IOM 4 NCE 0 module"]
    #[inline(always)]
    pub fn iom4ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc18::Iom4ce0)
    }
    #[doc = "IOM 4 NCE 1 module"]
    #[inline(always)]
    pub fn iom4ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc18::Iom4ce1)
    }
    #[doc = "IOM 4 NCE 2 module"]
    #[inline(always)]
    pub fn iom4ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc18::Iom4ce2)
    }
    #[doc = "IOM 4 NCE 3 module"]
    #[inline(always)]
    pub fn iom4ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc18::Iom4ce3)
    }
    #[doc = "IOM 5 NCE 0 module"]
    #[inline(always)]
    pub fn iom5ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc18::Iom5ce0)
    }
    #[doc = "IOM 5 NCE 1 module"]
    #[inline(always)]
    pub fn iom5ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc18::Iom5ce1)
    }
    #[doc = "IOM 5 NCE 2 module"]
    #[inline(always)]
    pub fn iom5ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc18::Iom5ce2)
    }
    #[doc = "IOM 5 NCE 3 module"]
    #[inline(always)]
    pub fn iom5ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc18::Iom5ce3)
    }
    #[doc = "IOM 6 NCE 0 module"]
    #[inline(always)]
    pub fn iom6ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc18::Iom6ce0)
    }
    #[doc = "IOM 6 NCE 1 module"]
    #[inline(always)]
    pub fn iom6ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc18::Iom6ce1)
    }
    #[doc = "IOM 6 NCE 2 module"]
    #[inline(always)]
    pub fn iom6ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc18::Iom6ce2)
    }
    #[doc = "IOM 6 NCE 3 module"]
    #[inline(always)]
    pub fn iom6ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc18::Iom6ce3)
    }
    #[doc = "IOM 7 NCE 0 module"]
    #[inline(always)]
    pub fn iom7ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc18::Iom7ce0)
    }
    #[doc = "IOM 7 NCE 1 module"]
    #[inline(always)]
    pub fn iom7ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc18::Iom7ce1)
    }
    #[doc = "IOM 7 NCE 2 module"]
    #[inline(always)]
    pub fn iom7ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc18::Iom7ce2)
    }
    #[doc = "IOM 7 NCE 3 module"]
    #[inline(always)]
    pub fn iom7ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc18::Iom7ce3)
    }
    #[doc = "MSPI 0 NCE 0 module"]
    #[inline(always)]
    pub fn mspi0cen0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc18::Mspi0cen0)
    }
    #[doc = "MSPI 0 NCE 1 module"]
    #[inline(always)]
    pub fn mspi0cen1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc18::Mspi0cen1)
    }
    #[doc = "MSPI 1 NCE 0 module"]
    #[inline(always)]
    pub fn mspi1cen0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc18::Mspi1cen0)
    }
    #[doc = "MSPI 1 NCE 1 module"]
    #[inline(always)]
    pub fn mspi1cen1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc18::Mspi1cen1)
    }
    #[doc = "MSPI 2 NCE 0 module"]
    #[inline(always)]
    pub fn mspi2cen0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc18::Mspi2cen0)
    }
    #[doc = "MSPI 2 NCE 1 module"]
    #[inline(always)]
    pub fn mspi2cen1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc18::Mspi2cen1)
    }
    #[doc = "DC DPI DE module"]
    #[inline(always)]
    pub fn dc_dpi_de(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc18::DcDpiDe)
    }
    #[doc = "DISP CONT CSX module"]
    #[inline(always)]
    pub fn disp_cont_csx(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc18::DispContCsx)
    }
    #[doc = "DC SPI CS_N module"]
    #[inline(always)]
    pub fn dc_spi_cs_n(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc18::DcSpiCsN)
    }
    #[doc = "DC QSPI CS_N module"]
    #[inline(always)]
    pub fn dc_qspi_cs_n(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc18::DcQspiCsN)
    }
    #[doc = "DC module RESX"]
    #[inline(always)]
    pub fn dc_resx(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc18::DcResx)
    }
}
#[doc = "Polarity select for NCE for GPIO 18\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ncepol18 {
    #[doc = "0: Polarity is active low"]
    Low = 0,
    #[doc = "1: Polarity is active high"]
    High = 1,
}
impl From<Ncepol18> for bool {
    #[inline(always)]
    fn from(variant: Ncepol18) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NCEPOL18` reader - Polarity select for NCE for GPIO 18"]
pub type Ncepol18R = crate::BitReader<Ncepol18>;
impl Ncepol18R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ncepol18 {
        match self.bits {
            false => Ncepol18::Low,
            true => Ncepol18::High,
        }
    }
    #[doc = "Polarity is active low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Ncepol18::Low
    }
    #[doc = "Polarity is active high"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Ncepol18::High
    }
}
#[doc = "Field `NCEPOL18` writer - Polarity select for NCE for GPIO 18"]
pub type Ncepol18W<'a, REG> = crate::BitWriter<'a, REG, Ncepol18>;
impl<'a, REG> Ncepol18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Polarity is active low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Ncepol18::Low)
    }
    #[doc = "Polarity is active high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Ncepol18::High)
    }
}
#[doc = "Field `FIEN18` reader - Force input enable active regardless of function selected. Otherwise the selected function will enable the input only when needed"]
pub type Fien18R = crate::BitReader;
#[doc = "Field `FIEN18` writer - Force input enable active regardless of function selected. Otherwise the selected function will enable the input only when needed"]
pub type Fien18W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FOEN18` reader - Force output enable active regardless of function selected. Otherwise the selected function will enable the output only when needed"]
pub type Foen18R = crate::BitReader;
#[doc = "Field `FOEN18` writer - Force output enable active regardless of function selected. Otherwise the selected function will enable the output only when needed"]
pub type Foen18W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Function select for GPIO pin 18"]
    #[inline(always)]
    pub fn fncsel18(&self) -> Fncsel18R {
        Fncsel18R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Input enable for GPIO 18"]
    #[inline(always)]
    pub fn inpen18(&self) -> Inpen18R {
        Inpen18R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Return 0 for read data on GPIO 18"]
    #[inline(always)]
    pub fn rdzero18(&self) -> Rdzero18R {
        Rdzero18R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Interrupt enable for GPIO 18"]
    #[inline(always)]
    pub fn irpten18(&self) -> Irpten18R {
        Irpten18R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Pin IO mode selection for GPIO pin 18"]
    #[inline(always)]
    pub fn outcfg18(&self) -> Outcfg18R {
        Outcfg18R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Drive strength selection for GPIO 18"]
    #[inline(always)]
    pub fn ds18(&self) -> Ds18R {
        Ds18R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - Configure the slew rate"]
    #[inline(always)]
    pub fn sr18(&self) -> Sr18R {
        Sr18R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - Pullup/Pulldown configuration for GPIO 18"]
    #[inline(always)]
    pub fn pullcfg18(&self) -> Pullcfg18R {
        Pullcfg18R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:21 - IOMSTR/MSPI N Chip Select 18, DISP control signals DE, CSX, and CS. Polarity is determined by CE_POLARITY field"]
    #[inline(always)]
    pub fn ncesrc18(&self) -> Ncesrc18R {
        Ncesrc18R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 22 - Polarity select for NCE for GPIO 18"]
    #[inline(always)]
    pub fn ncepol18(&self) -> Ncepol18R {
        Ncepol18R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 26 - Force input enable active regardless of function selected. Otherwise the selected function will enable the input only when needed"]
    #[inline(always)]
    pub fn fien18(&self) -> Fien18R {
        Fien18R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Force output enable active regardless of function selected. Otherwise the selected function will enable the output only when needed"]
    #[inline(always)]
    pub fn foen18(&self) -> Foen18R {
        Foen18R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Function select for GPIO pin 18"]
    #[inline(always)]
    #[must_use]
    pub fn fncsel18(&mut self) -> Fncsel18W<Pincfg18Spec> {
        Fncsel18W::new(self, 0)
    }
    #[doc = "Bit 4 - Input enable for GPIO 18"]
    #[inline(always)]
    #[must_use]
    pub fn inpen18(&mut self) -> Inpen18W<Pincfg18Spec> {
        Inpen18W::new(self, 4)
    }
    #[doc = "Bit 5 - Return 0 for read data on GPIO 18"]
    #[inline(always)]
    #[must_use]
    pub fn rdzero18(&mut self) -> Rdzero18W<Pincfg18Spec> {
        Rdzero18W::new(self, 5)
    }
    #[doc = "Bits 6:7 - Interrupt enable for GPIO 18"]
    #[inline(always)]
    #[must_use]
    pub fn irpten18(&mut self) -> Irpten18W<Pincfg18Spec> {
        Irpten18W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Pin IO mode selection for GPIO pin 18"]
    #[inline(always)]
    #[must_use]
    pub fn outcfg18(&mut self) -> Outcfg18W<Pincfg18Spec> {
        Outcfg18W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Drive strength selection for GPIO 18"]
    #[inline(always)]
    #[must_use]
    pub fn ds18(&mut self) -> Ds18W<Pincfg18Spec> {
        Ds18W::new(self, 10)
    }
    #[doc = "Bit 12 - Configure the slew rate"]
    #[inline(always)]
    #[must_use]
    pub fn sr18(&mut self) -> Sr18W<Pincfg18Spec> {
        Sr18W::new(self, 12)
    }
    #[doc = "Bits 13:15 - Pullup/Pulldown configuration for GPIO 18"]
    #[inline(always)]
    #[must_use]
    pub fn pullcfg18(&mut self) -> Pullcfg18W<Pincfg18Spec> {
        Pullcfg18W::new(self, 13)
    }
    #[doc = "Bits 16:21 - IOMSTR/MSPI N Chip Select 18, DISP control signals DE, CSX, and CS. Polarity is determined by CE_POLARITY field"]
    #[inline(always)]
    #[must_use]
    pub fn ncesrc18(&mut self) -> Ncesrc18W<Pincfg18Spec> {
        Ncesrc18W::new(self, 16)
    }
    #[doc = "Bit 22 - Polarity select for NCE for GPIO 18"]
    #[inline(always)]
    #[must_use]
    pub fn ncepol18(&mut self) -> Ncepol18W<Pincfg18Spec> {
        Ncepol18W::new(self, 22)
    }
    #[doc = "Bit 26 - Force input enable active regardless of function selected. Otherwise the selected function will enable the input only when needed"]
    #[inline(always)]
    #[must_use]
    pub fn fien18(&mut self) -> Fien18W<Pincfg18Spec> {
        Fien18W::new(self, 26)
    }
    #[doc = "Bit 27 - Force output enable active regardless of function selected. Otherwise the selected function will enable the output only when needed"]
    #[inline(always)]
    #[must_use]
    pub fn foen18(&mut self) -> Foen18W<Pincfg18Spec> {
        Foen18W::new(self, 27)
    }
}
#[doc = "Controls the operation of GPIO pin 18.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg18::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg18::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pincfg18Spec;
impl crate::RegisterSpec for Pincfg18Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pincfg18::R`](R) reader structure"]
impl crate::Readable for Pincfg18Spec {}
#[doc = "`write(|w| ..)` method takes [`pincfg18::W`](W) writer structure"]
impl crate::Writable for Pincfg18Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PINCFG18 to value 0x03"]
impl crate::Resettable for Pincfg18Spec {
    const RESET_VALUE: u32 = 0x03;
}
