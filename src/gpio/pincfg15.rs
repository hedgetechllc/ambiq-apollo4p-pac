#[doc = "Register `PINCFG15` reader"]
pub type R = crate::R<Pincfg15Spec>;
#[doc = "Register `PINCFG15` writer"]
pub type W = crate::W<Pincfg15Spec>;
#[doc = "Function select for GPIO pin 15\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fncsel15 {
    #[doc = "0: Analog to Digital Converter SE IN4"]
    Adcse4 = 0,
    #[doc = "1: ADC trigger input"]
    Trig0 = 1,
    #[doc = "2: Reserved selection. Operation unknown if selected."]
    Reserved2 = 2,
    #[doc = "3: General purpose I/O"]
    Gpio = 3,
    #[doc = "4: MILLI Record Data"]
    MilliRecDat = 4,
    #[doc = "5: UART receive input (UART 3)"]
    Uart3rx = 5,
    #[doc = "6: Timer/Counter input or output; Selection of direction is done via CTIMER register settings."]
    Ct15 = 6,
    #[doc = "7: IOMSTR/MSPI N Chip Select. Polarity is determined by CE_POLARITY field"]
    Nce15 = 7,
    #[doc = "8: Observation bus bit 15"]
    Obsbus15 = 8,
    #[doc = "9: Reserved selection. Operation unknown if selected."]
    Reserved9 = 9,
    #[doc = "10: External Reference Clock"]
    RefclkExt = 10,
    #[doc = "11: Fast PIO"]
    Fpio = 11,
    #[doc = "12: Reserved selection. Operation unknown if selected."]
    Reserved12 = 12,
    #[doc = "13: Internal function (Flash parallel load)"]
    FlloadData = 13,
    #[doc = "14: Reserved selection. Operation unknown if selected."]
    Reserved14 = 14,
    #[doc = "15: Internal function (SCAN)"]
    Scanout1 = 15,
}
impl From<Fncsel15> for u8 {
    #[inline(always)]
    fn from(variant: Fncsel15) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fncsel15 {
    type Ux = u8;
}
impl crate::IsEnum for Fncsel15 {}
#[doc = "Field `FNCSEL15` reader - Function select for GPIO pin 15"]
pub type Fncsel15R = crate::FieldReader<Fncsel15>;
impl Fncsel15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fncsel15 {
        match self.bits {
            0 => Fncsel15::Adcse4,
            1 => Fncsel15::Trig0,
            2 => Fncsel15::Reserved2,
            3 => Fncsel15::Gpio,
            4 => Fncsel15::MilliRecDat,
            5 => Fncsel15::Uart3rx,
            6 => Fncsel15::Ct15,
            7 => Fncsel15::Nce15,
            8 => Fncsel15::Obsbus15,
            9 => Fncsel15::Reserved9,
            10 => Fncsel15::RefclkExt,
            11 => Fncsel15::Fpio,
            12 => Fncsel15::Reserved12,
            13 => Fncsel15::FlloadData,
            14 => Fncsel15::Reserved14,
            15 => Fncsel15::Scanout1,
            _ => unreachable!(),
        }
    }
    #[doc = "Analog to Digital Converter SE IN4"]
    #[inline(always)]
    pub fn is_adcse4(&self) -> bool {
        *self == Fncsel15::Adcse4
    }
    #[doc = "ADC trigger input"]
    #[inline(always)]
    pub fn is_trig0(&self) -> bool {
        *self == Fncsel15::Trig0
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == Fncsel15::Reserved2
    }
    #[doc = "General purpose I/O"]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == Fncsel15::Gpio
    }
    #[doc = "MILLI Record Data"]
    #[inline(always)]
    pub fn is_milli_rec_dat(&self) -> bool {
        *self == Fncsel15::MilliRecDat
    }
    #[doc = "UART receive input (UART 3)"]
    #[inline(always)]
    pub fn is_uart3rx(&self) -> bool {
        *self == Fncsel15::Uart3rx
    }
    #[doc = "Timer/Counter input or output; Selection of direction is done via CTIMER register settings."]
    #[inline(always)]
    pub fn is_ct15(&self) -> bool {
        *self == Fncsel15::Ct15
    }
    #[doc = "IOMSTR/MSPI N Chip Select. Polarity is determined by CE_POLARITY field"]
    #[inline(always)]
    pub fn is_nce15(&self) -> bool {
        *self == Fncsel15::Nce15
    }
    #[doc = "Observation bus bit 15"]
    #[inline(always)]
    pub fn is_obsbus15(&self) -> bool {
        *self == Fncsel15::Obsbus15
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved9(&self) -> bool {
        *self == Fncsel15::Reserved9
    }
    #[doc = "External Reference Clock"]
    #[inline(always)]
    pub fn is_refclk_ext(&self) -> bool {
        *self == Fncsel15::RefclkExt
    }
    #[doc = "Fast PIO"]
    #[inline(always)]
    pub fn is_fpio(&self) -> bool {
        *self == Fncsel15::Fpio
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved12(&self) -> bool {
        *self == Fncsel15::Reserved12
    }
    #[doc = "Internal function (Flash parallel load)"]
    #[inline(always)]
    pub fn is_flload_data(&self) -> bool {
        *self == Fncsel15::FlloadData
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved14(&self) -> bool {
        *self == Fncsel15::Reserved14
    }
    #[doc = "Internal function (SCAN)"]
    #[inline(always)]
    pub fn is_scanout1(&self) -> bool {
        *self == Fncsel15::Scanout1
    }
}
#[doc = "Field `FNCSEL15` writer - Function select for GPIO pin 15"]
pub type Fncsel15W<'a, REG> = crate::FieldWriter<'a, REG, 4, Fncsel15, crate::Safe>;
impl<'a, REG> Fncsel15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Analog to Digital Converter SE IN4"]
    #[inline(always)]
    pub fn adcse4(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel15::Adcse4)
    }
    #[doc = "ADC trigger input"]
    #[inline(always)]
    pub fn trig0(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel15::Trig0)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel15::Reserved2)
    }
    #[doc = "General purpose I/O"]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel15::Gpio)
    }
    #[doc = "MILLI Record Data"]
    #[inline(always)]
    pub fn milli_rec_dat(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel15::MilliRecDat)
    }
    #[doc = "UART receive input (UART 3)"]
    #[inline(always)]
    pub fn uart3rx(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel15::Uart3rx)
    }
    #[doc = "Timer/Counter input or output; Selection of direction is done via CTIMER register settings."]
    #[inline(always)]
    pub fn ct15(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel15::Ct15)
    }
    #[doc = "IOMSTR/MSPI N Chip Select. Polarity is determined by CE_POLARITY field"]
    #[inline(always)]
    pub fn nce15(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel15::Nce15)
    }
    #[doc = "Observation bus bit 15"]
    #[inline(always)]
    pub fn obsbus15(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel15::Obsbus15)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved9(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel15::Reserved9)
    }
    #[doc = "External Reference Clock"]
    #[inline(always)]
    pub fn refclk_ext(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel15::RefclkExt)
    }
    #[doc = "Fast PIO"]
    #[inline(always)]
    pub fn fpio(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel15::Fpio)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved12(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel15::Reserved12)
    }
    #[doc = "Internal function (Flash parallel load)"]
    #[inline(always)]
    pub fn flload_data(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel15::FlloadData)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved14(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel15::Reserved14)
    }
    #[doc = "Internal function (SCAN)"]
    #[inline(always)]
    pub fn scanout1(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel15::Scanout1)
    }
}
#[doc = "Field `INPEN15` reader - Input enable for GPIO 15"]
pub type Inpen15R = crate::BitReader;
#[doc = "Field `INPEN15` writer - Input enable for GPIO 15"]
pub type Inpen15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDZERO15` reader - Return 0 for read data on GPIO 15"]
pub type Rdzero15R = crate::BitReader;
#[doc = "Field `RDZERO15` writer - Return 0 for read data on GPIO 15"]
pub type Rdzero15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Interrupt enable for GPIO 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Irpten15 {
    #[doc = "0: Interrupts are disabled for this GPIO"]
    Dis = 0,
    #[doc = "1: Interrupts are enabled for falling edge transition on this GPIO"]
    Intfall = 1,
    #[doc = "2: Interrupts are enabled for rising edge transitions on this GPIO"]
    Intrise = 2,
    #[doc = "3: Interrupts are enabled for any edge transition on this GPIO"]
    Intany = 3,
}
impl From<Irpten15> for u8 {
    #[inline(always)]
    fn from(variant: Irpten15) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Irpten15 {
    type Ux = u8;
}
impl crate::IsEnum for Irpten15 {}
#[doc = "Field `IRPTEN15` reader - Interrupt enable for GPIO 15"]
pub type Irpten15R = crate::FieldReader<Irpten15>;
impl Irpten15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irpten15 {
        match self.bits {
            0 => Irpten15::Dis,
            1 => Irpten15::Intfall,
            2 => Irpten15::Intrise,
            3 => Irpten15::Intany,
            _ => unreachable!(),
        }
    }
    #[doc = "Interrupts are disabled for this GPIO"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Irpten15::Dis
    }
    #[doc = "Interrupts are enabled for falling edge transition on this GPIO"]
    #[inline(always)]
    pub fn is_intfall(&self) -> bool {
        *self == Irpten15::Intfall
    }
    #[doc = "Interrupts are enabled for rising edge transitions on this GPIO"]
    #[inline(always)]
    pub fn is_intrise(&self) -> bool {
        *self == Irpten15::Intrise
    }
    #[doc = "Interrupts are enabled for any edge transition on this GPIO"]
    #[inline(always)]
    pub fn is_intany(&self) -> bool {
        *self == Irpten15::Intany
    }
}
#[doc = "Field `IRPTEN15` writer - Interrupt enable for GPIO 15"]
pub type Irpten15W<'a, REG> = crate::FieldWriter<'a, REG, 2, Irpten15, crate::Safe>;
impl<'a, REG> Irpten15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Interrupts are disabled for this GPIO"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten15::Dis)
    }
    #[doc = "Interrupts are enabled for falling edge transition on this GPIO"]
    #[inline(always)]
    pub fn intfall(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten15::Intfall)
    }
    #[doc = "Interrupts are enabled for rising edge transitions on this GPIO"]
    #[inline(always)]
    pub fn intrise(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten15::Intrise)
    }
    #[doc = "Interrupts are enabled for any edge transition on this GPIO"]
    #[inline(always)]
    pub fn intany(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten15::Intany)
    }
}
#[doc = "Pin IO mode selection for GPIO pin 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Outcfg15 {
    #[doc = "0: Output Disabled"]
    Dis = 0,
    #[doc = "1: Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    Pushpull = 1,
    #[doc = "2: Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    Od = 2,
    #[doc = "3: Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    Ts = 3,
}
impl From<Outcfg15> for u8 {
    #[inline(always)]
    fn from(variant: Outcfg15) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Outcfg15 {
    type Ux = u8;
}
impl crate::IsEnum for Outcfg15 {}
#[doc = "Field `OUTCFG15` reader - Pin IO mode selection for GPIO pin 15"]
pub type Outcfg15R = crate::FieldReader<Outcfg15>;
impl Outcfg15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Outcfg15 {
        match self.bits {
            0 => Outcfg15::Dis,
            1 => Outcfg15::Pushpull,
            2 => Outcfg15::Od,
            3 => Outcfg15::Ts,
            _ => unreachable!(),
        }
    }
    #[doc = "Output Disabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Outcfg15::Dis
    }
    #[doc = "Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == Outcfg15::Pushpull
    }
    #[doc = "Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == Outcfg15::Od
    }
    #[doc = "Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == Outcfg15::Ts
    }
}
#[doc = "Field `OUTCFG15` writer - Pin IO mode selection for GPIO pin 15"]
pub type Outcfg15W<'a, REG> = crate::FieldWriter<'a, REG, 2, Outcfg15, crate::Safe>;
impl<'a, REG> Outcfg15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Output Disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg15::Dis)
    }
    #[doc = "Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg15::Pushpull)
    }
    #[doc = "Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    #[inline(always)]
    pub fn od(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg15::Od)
    }
    #[doc = "Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    #[inline(always)]
    pub fn ts(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg15::Ts)
    }
}
#[doc = "Drive strength selection for GPIO 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ds15 {
    #[doc = "0: 0.1x output driver selected"]
    _0p1x = 0,
    #[doc = "1: 0.5x output driver selected"]
    _0p5x = 1,
}
impl From<Ds15> for u8 {
    #[inline(always)]
    fn from(variant: Ds15) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ds15 {
    type Ux = u8;
}
impl crate::IsEnum for Ds15 {}
#[doc = "Field `DS15` reader - Drive strength selection for GPIO 15"]
pub type Ds15R = crate::FieldReader<Ds15>;
impl Ds15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ds15> {
        match self.bits {
            0 => Some(Ds15::_0p1x),
            1 => Some(Ds15::_0p5x),
            _ => None,
        }
    }
    #[doc = "0.1x output driver selected"]
    #[inline(always)]
    pub fn is_0p1x(&self) -> bool {
        *self == Ds15::_0p1x
    }
    #[doc = "0.5x output driver selected"]
    #[inline(always)]
    pub fn is_0p5x(&self) -> bool {
        *self == Ds15::_0p5x
    }
}
#[doc = "Field `DS15` writer - Drive strength selection for GPIO 15"]
pub type Ds15W<'a, REG> = crate::FieldWriter<'a, REG, 2, Ds15>;
impl<'a, REG> Ds15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "0.1x output driver selected"]
    #[inline(always)]
    pub fn _0p1x(self) -> &'a mut crate::W<REG> {
        self.variant(Ds15::_0p1x)
    }
    #[doc = "0.5x output driver selected"]
    #[inline(always)]
    pub fn _0p5x(self) -> &'a mut crate::W<REG> {
        self.variant(Ds15::_0p5x)
    }
}
#[doc = "Field `SR15` reader - Configure the slew rate"]
pub type Sr15R = crate::BitReader;
#[doc = "Field `SR15` writer - Configure the slew rate"]
pub type Sr15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Pullup/Pulldown configuration for GPIO 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pullcfg15 {
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
impl From<Pullcfg15> for u8 {
    #[inline(always)]
    fn from(variant: Pullcfg15) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pullcfg15 {
    type Ux = u8;
}
impl crate::IsEnum for Pullcfg15 {}
#[doc = "Field `PULLCFG15` reader - Pullup/Pulldown configuration for GPIO 15"]
pub type Pullcfg15R = crate::FieldReader<Pullcfg15>;
impl Pullcfg15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pullcfg15 {
        match self.bits {
            0 => Pullcfg15::Dis,
            1 => Pullcfg15::Pd50k,
            2 => Pullcfg15::Pu15k,
            3 => Pullcfg15::Pu6k,
            4 => Pullcfg15::Pu12k,
            5 => Pullcfg15::Pu24k,
            6 => Pullcfg15::Pu50k,
            7 => Pullcfg15::Pu100k,
            _ => unreachable!(),
        }
    }
    #[doc = "No pullup or pulldown selected"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Pullcfg15::Dis
    }
    #[doc = "50K Pulldown selected"]
    #[inline(always)]
    pub fn is_pd50k(&self) -> bool {
        *self == Pullcfg15::Pd50k
    }
    #[doc = "1.5K Pullup selected"]
    #[inline(always)]
    pub fn is_pu15k(&self) -> bool {
        *self == Pullcfg15::Pu15k
    }
    #[doc = "6K Pullup selected"]
    #[inline(always)]
    pub fn is_pu6k(&self) -> bool {
        *self == Pullcfg15::Pu6k
    }
    #[doc = "12K Pullup selected"]
    #[inline(always)]
    pub fn is_pu12k(&self) -> bool {
        *self == Pullcfg15::Pu12k
    }
    #[doc = "24K Pullup selected"]
    #[inline(always)]
    pub fn is_pu24k(&self) -> bool {
        *self == Pullcfg15::Pu24k
    }
    #[doc = "50K Pullup selected"]
    #[inline(always)]
    pub fn is_pu50k(&self) -> bool {
        *self == Pullcfg15::Pu50k
    }
    #[doc = "100K Pullup selected"]
    #[inline(always)]
    pub fn is_pu100k(&self) -> bool {
        *self == Pullcfg15::Pu100k
    }
}
#[doc = "Field `PULLCFG15` writer - Pullup/Pulldown configuration for GPIO 15"]
pub type Pullcfg15W<'a, REG> = crate::FieldWriter<'a, REG, 3, Pullcfg15, crate::Safe>;
impl<'a, REG> Pullcfg15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No pullup or pulldown selected"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg15::Dis)
    }
    #[doc = "50K Pulldown selected"]
    #[inline(always)]
    pub fn pd50k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg15::Pd50k)
    }
    #[doc = "1.5K Pullup selected"]
    #[inline(always)]
    pub fn pu15k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg15::Pu15k)
    }
    #[doc = "6K Pullup selected"]
    #[inline(always)]
    pub fn pu6k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg15::Pu6k)
    }
    #[doc = "12K Pullup selected"]
    #[inline(always)]
    pub fn pu12k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg15::Pu12k)
    }
    #[doc = "24K Pullup selected"]
    #[inline(always)]
    pub fn pu24k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg15::Pu24k)
    }
    #[doc = "50K Pullup selected"]
    #[inline(always)]
    pub fn pu50k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg15::Pu50k)
    }
    #[doc = "100K Pullup selected"]
    #[inline(always)]
    pub fn pu100k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg15::Pu100k)
    }
}
#[doc = "IOMSTR/MSPI N Chip Select 15, DISP control signals DE, CSX, and CS. Polarity is determined by CE_POLARITY field\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ncesrc15 {
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
impl From<Ncesrc15> for u8 {
    #[inline(always)]
    fn from(variant: Ncesrc15) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ncesrc15 {
    type Ux = u8;
}
impl crate::IsEnum for Ncesrc15 {}
#[doc = "Field `NCESRC15` reader - IOMSTR/MSPI N Chip Select 15, DISP control signals DE, CSX, and CS. Polarity is determined by CE_POLARITY field"]
pub type Ncesrc15R = crate::FieldReader<Ncesrc15>;
impl Ncesrc15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ncesrc15> {
        match self.bits {
            0 => Some(Ncesrc15::Iom0ce0),
            1 => Some(Ncesrc15::Iom0ce1),
            2 => Some(Ncesrc15::Iom0ce2),
            3 => Some(Ncesrc15::Iom0ce3),
            4 => Some(Ncesrc15::Iom1ce0),
            5 => Some(Ncesrc15::Iom1ce1),
            6 => Some(Ncesrc15::Iom1ce2),
            7 => Some(Ncesrc15::Iom1ce3),
            8 => Some(Ncesrc15::Iom2ce0),
            9 => Some(Ncesrc15::Iom2ce1),
            10 => Some(Ncesrc15::Iom2ce2),
            11 => Some(Ncesrc15::Iom2ce3),
            12 => Some(Ncesrc15::Iom3ce0),
            13 => Some(Ncesrc15::Iom3ce1),
            14 => Some(Ncesrc15::Iom3ce2),
            15 => Some(Ncesrc15::Iom3ce3),
            16 => Some(Ncesrc15::Iom4ce0),
            17 => Some(Ncesrc15::Iom4ce1),
            18 => Some(Ncesrc15::Iom4ce2),
            19 => Some(Ncesrc15::Iom4ce3),
            20 => Some(Ncesrc15::Iom5ce0),
            21 => Some(Ncesrc15::Iom5ce1),
            22 => Some(Ncesrc15::Iom5ce2),
            23 => Some(Ncesrc15::Iom5ce3),
            24 => Some(Ncesrc15::Iom6ce0),
            25 => Some(Ncesrc15::Iom6ce1),
            26 => Some(Ncesrc15::Iom6ce2),
            27 => Some(Ncesrc15::Iom6ce3),
            28 => Some(Ncesrc15::Iom7ce0),
            29 => Some(Ncesrc15::Iom7ce1),
            30 => Some(Ncesrc15::Iom7ce2),
            31 => Some(Ncesrc15::Iom7ce3),
            32 => Some(Ncesrc15::Mspi0cen0),
            33 => Some(Ncesrc15::Mspi0cen1),
            34 => Some(Ncesrc15::Mspi1cen0),
            35 => Some(Ncesrc15::Mspi1cen1),
            36 => Some(Ncesrc15::Mspi2cen0),
            37 => Some(Ncesrc15::Mspi2cen1),
            38 => Some(Ncesrc15::DcDpiDe),
            39 => Some(Ncesrc15::DispContCsx),
            40 => Some(Ncesrc15::DcSpiCsN),
            41 => Some(Ncesrc15::DcQspiCsN),
            42 => Some(Ncesrc15::DcResx),
            _ => None,
        }
    }
    #[doc = "IOM 0 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom0ce0(&self) -> bool {
        *self == Ncesrc15::Iom0ce0
    }
    #[doc = "IOM 0 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom0ce1(&self) -> bool {
        *self == Ncesrc15::Iom0ce1
    }
    #[doc = "IOM 0 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom0ce2(&self) -> bool {
        *self == Ncesrc15::Iom0ce2
    }
    #[doc = "IOM 0 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom0ce3(&self) -> bool {
        *self == Ncesrc15::Iom0ce3
    }
    #[doc = "IOM 1 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom1ce0(&self) -> bool {
        *self == Ncesrc15::Iom1ce0
    }
    #[doc = "IOM 1 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom1ce1(&self) -> bool {
        *self == Ncesrc15::Iom1ce1
    }
    #[doc = "IOM 1 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom1ce2(&self) -> bool {
        *self == Ncesrc15::Iom1ce2
    }
    #[doc = "IOM 1 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom1ce3(&self) -> bool {
        *self == Ncesrc15::Iom1ce3
    }
    #[doc = "IOM 2 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom2ce0(&self) -> bool {
        *self == Ncesrc15::Iom2ce0
    }
    #[doc = "IOM 2 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom2ce1(&self) -> bool {
        *self == Ncesrc15::Iom2ce1
    }
    #[doc = "IOM 2 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom2ce2(&self) -> bool {
        *self == Ncesrc15::Iom2ce2
    }
    #[doc = "IOM 2 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom2ce3(&self) -> bool {
        *self == Ncesrc15::Iom2ce3
    }
    #[doc = "IOM 3 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom3ce0(&self) -> bool {
        *self == Ncesrc15::Iom3ce0
    }
    #[doc = "IOM 3 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom3ce1(&self) -> bool {
        *self == Ncesrc15::Iom3ce1
    }
    #[doc = "IOM 3 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom3ce2(&self) -> bool {
        *self == Ncesrc15::Iom3ce2
    }
    #[doc = "IOM 3 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom3ce3(&self) -> bool {
        *self == Ncesrc15::Iom3ce3
    }
    #[doc = "IOM 4 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom4ce0(&self) -> bool {
        *self == Ncesrc15::Iom4ce0
    }
    #[doc = "IOM 4 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom4ce1(&self) -> bool {
        *self == Ncesrc15::Iom4ce1
    }
    #[doc = "IOM 4 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom4ce2(&self) -> bool {
        *self == Ncesrc15::Iom4ce2
    }
    #[doc = "IOM 4 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom4ce3(&self) -> bool {
        *self == Ncesrc15::Iom4ce3
    }
    #[doc = "IOM 5 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom5ce0(&self) -> bool {
        *self == Ncesrc15::Iom5ce0
    }
    #[doc = "IOM 5 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom5ce1(&self) -> bool {
        *self == Ncesrc15::Iom5ce1
    }
    #[doc = "IOM 5 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom5ce2(&self) -> bool {
        *self == Ncesrc15::Iom5ce2
    }
    #[doc = "IOM 5 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom5ce3(&self) -> bool {
        *self == Ncesrc15::Iom5ce3
    }
    #[doc = "IOM 6 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom6ce0(&self) -> bool {
        *self == Ncesrc15::Iom6ce0
    }
    #[doc = "IOM 6 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom6ce1(&self) -> bool {
        *self == Ncesrc15::Iom6ce1
    }
    #[doc = "IOM 6 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom6ce2(&self) -> bool {
        *self == Ncesrc15::Iom6ce2
    }
    #[doc = "IOM 6 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom6ce3(&self) -> bool {
        *self == Ncesrc15::Iom6ce3
    }
    #[doc = "IOM 7 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom7ce0(&self) -> bool {
        *self == Ncesrc15::Iom7ce0
    }
    #[doc = "IOM 7 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom7ce1(&self) -> bool {
        *self == Ncesrc15::Iom7ce1
    }
    #[doc = "IOM 7 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom7ce2(&self) -> bool {
        *self == Ncesrc15::Iom7ce2
    }
    #[doc = "IOM 7 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom7ce3(&self) -> bool {
        *self == Ncesrc15::Iom7ce3
    }
    #[doc = "MSPI 0 NCE 0 module"]
    #[inline(always)]
    pub fn is_mspi0cen0(&self) -> bool {
        *self == Ncesrc15::Mspi0cen0
    }
    #[doc = "MSPI 0 NCE 1 module"]
    #[inline(always)]
    pub fn is_mspi0cen1(&self) -> bool {
        *self == Ncesrc15::Mspi0cen1
    }
    #[doc = "MSPI 1 NCE 0 module"]
    #[inline(always)]
    pub fn is_mspi1cen0(&self) -> bool {
        *self == Ncesrc15::Mspi1cen0
    }
    #[doc = "MSPI 1 NCE 1 module"]
    #[inline(always)]
    pub fn is_mspi1cen1(&self) -> bool {
        *self == Ncesrc15::Mspi1cen1
    }
    #[doc = "MSPI 2 NCE 0 module"]
    #[inline(always)]
    pub fn is_mspi2cen0(&self) -> bool {
        *self == Ncesrc15::Mspi2cen0
    }
    #[doc = "MSPI 2 NCE 1 module"]
    #[inline(always)]
    pub fn is_mspi2cen1(&self) -> bool {
        *self == Ncesrc15::Mspi2cen1
    }
    #[doc = "DC DPI DE module"]
    #[inline(always)]
    pub fn is_dc_dpi_de(&self) -> bool {
        *self == Ncesrc15::DcDpiDe
    }
    #[doc = "DISP CONT CSX module"]
    #[inline(always)]
    pub fn is_disp_cont_csx(&self) -> bool {
        *self == Ncesrc15::DispContCsx
    }
    #[doc = "DC SPI CS_N module"]
    #[inline(always)]
    pub fn is_dc_spi_cs_n(&self) -> bool {
        *self == Ncesrc15::DcSpiCsN
    }
    #[doc = "DC QSPI CS_N module"]
    #[inline(always)]
    pub fn is_dc_qspi_cs_n(&self) -> bool {
        *self == Ncesrc15::DcQspiCsN
    }
    #[doc = "DC module RESX"]
    #[inline(always)]
    pub fn is_dc_resx(&self) -> bool {
        *self == Ncesrc15::DcResx
    }
}
#[doc = "Field `NCESRC15` writer - IOMSTR/MSPI N Chip Select 15, DISP control signals DE, CSX, and CS. Polarity is determined by CE_POLARITY field"]
pub type Ncesrc15W<'a, REG> = crate::FieldWriter<'a, REG, 6, Ncesrc15>;
impl<'a, REG> Ncesrc15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "IOM 0 NCE 0 module"]
    #[inline(always)]
    pub fn iom0ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc15::Iom0ce0)
    }
    #[doc = "IOM 0 NCE 1 module"]
    #[inline(always)]
    pub fn iom0ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc15::Iom0ce1)
    }
    #[doc = "IOM 0 NCE 2 module"]
    #[inline(always)]
    pub fn iom0ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc15::Iom0ce2)
    }
    #[doc = "IOM 0 NCE 3 module"]
    #[inline(always)]
    pub fn iom0ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc15::Iom0ce3)
    }
    #[doc = "IOM 1 NCE 0 module"]
    #[inline(always)]
    pub fn iom1ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc15::Iom1ce0)
    }
    #[doc = "IOM 1 NCE 1 module"]
    #[inline(always)]
    pub fn iom1ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc15::Iom1ce1)
    }
    #[doc = "IOM 1 NCE 2 module"]
    #[inline(always)]
    pub fn iom1ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc15::Iom1ce2)
    }
    #[doc = "IOM 1 NCE 3 module"]
    #[inline(always)]
    pub fn iom1ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc15::Iom1ce3)
    }
    #[doc = "IOM 2 NCE 0 module"]
    #[inline(always)]
    pub fn iom2ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc15::Iom2ce0)
    }
    #[doc = "IOM 2 NCE 1 module"]
    #[inline(always)]
    pub fn iom2ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc15::Iom2ce1)
    }
    #[doc = "IOM 2 NCE 2 module"]
    #[inline(always)]
    pub fn iom2ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc15::Iom2ce2)
    }
    #[doc = "IOM 2 NCE 3 module"]
    #[inline(always)]
    pub fn iom2ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc15::Iom2ce3)
    }
    #[doc = "IOM 3 NCE 0 module"]
    #[inline(always)]
    pub fn iom3ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc15::Iom3ce0)
    }
    #[doc = "IOM 3 NCE 1 module"]
    #[inline(always)]
    pub fn iom3ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc15::Iom3ce1)
    }
    #[doc = "IOM 3 NCE 2 module"]
    #[inline(always)]
    pub fn iom3ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc15::Iom3ce2)
    }
    #[doc = "IOM 3 NCE 3 module"]
    #[inline(always)]
    pub fn iom3ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc15::Iom3ce3)
    }
    #[doc = "IOM 4 NCE 0 module"]
    #[inline(always)]
    pub fn iom4ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc15::Iom4ce0)
    }
    #[doc = "IOM 4 NCE 1 module"]
    #[inline(always)]
    pub fn iom4ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc15::Iom4ce1)
    }
    #[doc = "IOM 4 NCE 2 module"]
    #[inline(always)]
    pub fn iom4ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc15::Iom4ce2)
    }
    #[doc = "IOM 4 NCE 3 module"]
    #[inline(always)]
    pub fn iom4ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc15::Iom4ce3)
    }
    #[doc = "IOM 5 NCE 0 module"]
    #[inline(always)]
    pub fn iom5ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc15::Iom5ce0)
    }
    #[doc = "IOM 5 NCE 1 module"]
    #[inline(always)]
    pub fn iom5ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc15::Iom5ce1)
    }
    #[doc = "IOM 5 NCE 2 module"]
    #[inline(always)]
    pub fn iom5ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc15::Iom5ce2)
    }
    #[doc = "IOM 5 NCE 3 module"]
    #[inline(always)]
    pub fn iom5ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc15::Iom5ce3)
    }
    #[doc = "IOM 6 NCE 0 module"]
    #[inline(always)]
    pub fn iom6ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc15::Iom6ce0)
    }
    #[doc = "IOM 6 NCE 1 module"]
    #[inline(always)]
    pub fn iom6ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc15::Iom6ce1)
    }
    #[doc = "IOM 6 NCE 2 module"]
    #[inline(always)]
    pub fn iom6ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc15::Iom6ce2)
    }
    #[doc = "IOM 6 NCE 3 module"]
    #[inline(always)]
    pub fn iom6ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc15::Iom6ce3)
    }
    #[doc = "IOM 7 NCE 0 module"]
    #[inline(always)]
    pub fn iom7ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc15::Iom7ce0)
    }
    #[doc = "IOM 7 NCE 1 module"]
    #[inline(always)]
    pub fn iom7ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc15::Iom7ce1)
    }
    #[doc = "IOM 7 NCE 2 module"]
    #[inline(always)]
    pub fn iom7ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc15::Iom7ce2)
    }
    #[doc = "IOM 7 NCE 3 module"]
    #[inline(always)]
    pub fn iom7ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc15::Iom7ce3)
    }
    #[doc = "MSPI 0 NCE 0 module"]
    #[inline(always)]
    pub fn mspi0cen0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc15::Mspi0cen0)
    }
    #[doc = "MSPI 0 NCE 1 module"]
    #[inline(always)]
    pub fn mspi0cen1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc15::Mspi0cen1)
    }
    #[doc = "MSPI 1 NCE 0 module"]
    #[inline(always)]
    pub fn mspi1cen0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc15::Mspi1cen0)
    }
    #[doc = "MSPI 1 NCE 1 module"]
    #[inline(always)]
    pub fn mspi1cen1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc15::Mspi1cen1)
    }
    #[doc = "MSPI 2 NCE 0 module"]
    #[inline(always)]
    pub fn mspi2cen0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc15::Mspi2cen0)
    }
    #[doc = "MSPI 2 NCE 1 module"]
    #[inline(always)]
    pub fn mspi2cen1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc15::Mspi2cen1)
    }
    #[doc = "DC DPI DE module"]
    #[inline(always)]
    pub fn dc_dpi_de(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc15::DcDpiDe)
    }
    #[doc = "DISP CONT CSX module"]
    #[inline(always)]
    pub fn disp_cont_csx(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc15::DispContCsx)
    }
    #[doc = "DC SPI CS_N module"]
    #[inline(always)]
    pub fn dc_spi_cs_n(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc15::DcSpiCsN)
    }
    #[doc = "DC QSPI CS_N module"]
    #[inline(always)]
    pub fn dc_qspi_cs_n(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc15::DcQspiCsN)
    }
    #[doc = "DC module RESX"]
    #[inline(always)]
    pub fn dc_resx(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc15::DcResx)
    }
}
#[doc = "Polarity select for NCE for GPIO 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ncepol15 {
    #[doc = "0: Polarity is active low"]
    Low = 0,
    #[doc = "1: Polarity is active high"]
    High = 1,
}
impl From<Ncepol15> for bool {
    #[inline(always)]
    fn from(variant: Ncepol15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NCEPOL15` reader - Polarity select for NCE for GPIO 15"]
pub type Ncepol15R = crate::BitReader<Ncepol15>;
impl Ncepol15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ncepol15 {
        match self.bits {
            false => Ncepol15::Low,
            true => Ncepol15::High,
        }
    }
    #[doc = "Polarity is active low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Ncepol15::Low
    }
    #[doc = "Polarity is active high"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Ncepol15::High
    }
}
#[doc = "Field `NCEPOL15` writer - Polarity select for NCE for GPIO 15"]
pub type Ncepol15W<'a, REG> = crate::BitWriter<'a, REG, Ncepol15>;
impl<'a, REG> Ncepol15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Polarity is active low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Ncepol15::Low)
    }
    #[doc = "Polarity is active high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Ncepol15::High)
    }
}
#[doc = "Field `FIEN15` reader - Force input enable active regardless of function selected. Otherwise the selected function will enable the input only when needed"]
pub type Fien15R = crate::BitReader;
#[doc = "Field `FIEN15` writer - Force input enable active regardless of function selected. Otherwise the selected function will enable the input only when needed"]
pub type Fien15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FOEN15` reader - Force output enable active regardless of function selected. Otherwise the selected function will enable the output only when needed"]
pub type Foen15R = crate::BitReader;
#[doc = "Field `FOEN15` writer - Force output enable active regardless of function selected. Otherwise the selected function will enable the output only when needed"]
pub type Foen15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Function select for GPIO pin 15"]
    #[inline(always)]
    pub fn fncsel15(&self) -> Fncsel15R {
        Fncsel15R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Input enable for GPIO 15"]
    #[inline(always)]
    pub fn inpen15(&self) -> Inpen15R {
        Inpen15R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Return 0 for read data on GPIO 15"]
    #[inline(always)]
    pub fn rdzero15(&self) -> Rdzero15R {
        Rdzero15R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Interrupt enable for GPIO 15"]
    #[inline(always)]
    pub fn irpten15(&self) -> Irpten15R {
        Irpten15R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Pin IO mode selection for GPIO pin 15"]
    #[inline(always)]
    pub fn outcfg15(&self) -> Outcfg15R {
        Outcfg15R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Drive strength selection for GPIO 15"]
    #[inline(always)]
    pub fn ds15(&self) -> Ds15R {
        Ds15R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - Configure the slew rate"]
    #[inline(always)]
    pub fn sr15(&self) -> Sr15R {
        Sr15R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - Pullup/Pulldown configuration for GPIO 15"]
    #[inline(always)]
    pub fn pullcfg15(&self) -> Pullcfg15R {
        Pullcfg15R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:21 - IOMSTR/MSPI N Chip Select 15, DISP control signals DE, CSX, and CS. Polarity is determined by CE_POLARITY field"]
    #[inline(always)]
    pub fn ncesrc15(&self) -> Ncesrc15R {
        Ncesrc15R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 22 - Polarity select for NCE for GPIO 15"]
    #[inline(always)]
    pub fn ncepol15(&self) -> Ncepol15R {
        Ncepol15R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 26 - Force input enable active regardless of function selected. Otherwise the selected function will enable the input only when needed"]
    #[inline(always)]
    pub fn fien15(&self) -> Fien15R {
        Fien15R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Force output enable active regardless of function selected. Otherwise the selected function will enable the output only when needed"]
    #[inline(always)]
    pub fn foen15(&self) -> Foen15R {
        Foen15R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Function select for GPIO pin 15"]
    #[inline(always)]
    #[must_use]
    pub fn fncsel15(&mut self) -> Fncsel15W<Pincfg15Spec> {
        Fncsel15W::new(self, 0)
    }
    #[doc = "Bit 4 - Input enable for GPIO 15"]
    #[inline(always)]
    #[must_use]
    pub fn inpen15(&mut self) -> Inpen15W<Pincfg15Spec> {
        Inpen15W::new(self, 4)
    }
    #[doc = "Bit 5 - Return 0 for read data on GPIO 15"]
    #[inline(always)]
    #[must_use]
    pub fn rdzero15(&mut self) -> Rdzero15W<Pincfg15Spec> {
        Rdzero15W::new(self, 5)
    }
    #[doc = "Bits 6:7 - Interrupt enable for GPIO 15"]
    #[inline(always)]
    #[must_use]
    pub fn irpten15(&mut self) -> Irpten15W<Pincfg15Spec> {
        Irpten15W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Pin IO mode selection for GPIO pin 15"]
    #[inline(always)]
    #[must_use]
    pub fn outcfg15(&mut self) -> Outcfg15W<Pincfg15Spec> {
        Outcfg15W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Drive strength selection for GPIO 15"]
    #[inline(always)]
    #[must_use]
    pub fn ds15(&mut self) -> Ds15W<Pincfg15Spec> {
        Ds15W::new(self, 10)
    }
    #[doc = "Bit 12 - Configure the slew rate"]
    #[inline(always)]
    #[must_use]
    pub fn sr15(&mut self) -> Sr15W<Pincfg15Spec> {
        Sr15W::new(self, 12)
    }
    #[doc = "Bits 13:15 - Pullup/Pulldown configuration for GPIO 15"]
    #[inline(always)]
    #[must_use]
    pub fn pullcfg15(&mut self) -> Pullcfg15W<Pincfg15Spec> {
        Pullcfg15W::new(self, 13)
    }
    #[doc = "Bits 16:21 - IOMSTR/MSPI N Chip Select 15, DISP control signals DE, CSX, and CS. Polarity is determined by CE_POLARITY field"]
    #[inline(always)]
    #[must_use]
    pub fn ncesrc15(&mut self) -> Ncesrc15W<Pincfg15Spec> {
        Ncesrc15W::new(self, 16)
    }
    #[doc = "Bit 22 - Polarity select for NCE for GPIO 15"]
    #[inline(always)]
    #[must_use]
    pub fn ncepol15(&mut self) -> Ncepol15W<Pincfg15Spec> {
        Ncepol15W::new(self, 22)
    }
    #[doc = "Bit 26 - Force input enable active regardless of function selected. Otherwise the selected function will enable the input only when needed"]
    #[inline(always)]
    #[must_use]
    pub fn fien15(&mut self) -> Fien15W<Pincfg15Spec> {
        Fien15W::new(self, 26)
    }
    #[doc = "Bit 27 - Force output enable active regardless of function selected. Otherwise the selected function will enable the output only when needed"]
    #[inline(always)]
    #[must_use]
    pub fn foen15(&mut self) -> Foen15W<Pincfg15Spec> {
        Foen15W::new(self, 27)
    }
}
#[doc = "Controls the operation of GPIO pin 15.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg15::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg15::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pincfg15Spec;
impl crate::RegisterSpec for Pincfg15Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pincfg15::R`](R) reader structure"]
impl crate::Readable for Pincfg15Spec {}
#[doc = "`write(|w| ..)` method takes [`pincfg15::W`](W) writer structure"]
impl crate::Writable for Pincfg15Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PINCFG15 to value 0x03"]
impl crate::Resettable for Pincfg15Spec {
    const RESET_VALUE: u32 = 0x03;
}
