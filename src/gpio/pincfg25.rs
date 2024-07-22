#[doc = "Register `PINCFG25` reader"]
pub type R = crate::R<Pincfg25Spec>;
#[doc = "Register `PINCFG25` writer"]
pub type W = crate::W<Pincfg25Spec>;
#[doc = "Function select for GPIO pin 25\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fncsel25 {
    #[doc = "0: Serial I2C Master Clock output (IOM 2)"]
    M2scl = 0,
    #[doc = "1: Serial SPI Master Clock output (IOM 2)"]
    M2sck = 1,
    #[doc = "2: Reserved selection. Operation unknown if selected."]
    Reserved2 = 2,
    #[doc = "3: General purpose I/O"]
    Gpio = 3,
    #[doc = "4: External LFRC Clock"]
    LfrcExt = 4,
    #[doc = "5: JTAG tms input"]
    DspTms = 5,
    #[doc = "6: Timer/Counter input or output; Selection of direction is done via CTIMER register settings."]
    Ct25 = 6,
    #[doc = "7: IOMSTR/MSPI N Chip Select. Polarity is determined by CE_POLARITY field"]
    Nce25 = 7,
    #[doc = "8: Observation bus bit 9"]
    Obsbus9 = 8,
    #[doc = "9: Reserved selection. Operation unknown if selected."]
    Reserved9 = 9,
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
    Scanin8 = 15,
}
impl From<Fncsel25> for u8 {
    #[inline(always)]
    fn from(variant: Fncsel25) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fncsel25 {
    type Ux = u8;
}
impl crate::IsEnum for Fncsel25 {}
#[doc = "Field `FNCSEL25` reader - Function select for GPIO pin 25"]
pub type Fncsel25R = crate::FieldReader<Fncsel25>;
impl Fncsel25R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fncsel25 {
        match self.bits {
            0 => Fncsel25::M2scl,
            1 => Fncsel25::M2sck,
            2 => Fncsel25::Reserved2,
            3 => Fncsel25::Gpio,
            4 => Fncsel25::LfrcExt,
            5 => Fncsel25::DspTms,
            6 => Fncsel25::Ct25,
            7 => Fncsel25::Nce25,
            8 => Fncsel25::Obsbus9,
            9 => Fncsel25::Reserved9,
            10 => Fncsel25::Reserved10,
            11 => Fncsel25::Fpio,
            12 => Fncsel25::Reserved12,
            13 => Fncsel25::Reserved13,
            14 => Fncsel25::Reserved14,
            15 => Fncsel25::Scanin8,
            _ => unreachable!(),
        }
    }
    #[doc = "Serial I2C Master Clock output (IOM 2)"]
    #[inline(always)]
    pub fn is_m2scl(&self) -> bool {
        *self == Fncsel25::M2scl
    }
    #[doc = "Serial SPI Master Clock output (IOM 2)"]
    #[inline(always)]
    pub fn is_m2sck(&self) -> bool {
        *self == Fncsel25::M2sck
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == Fncsel25::Reserved2
    }
    #[doc = "General purpose I/O"]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == Fncsel25::Gpio
    }
    #[doc = "External LFRC Clock"]
    #[inline(always)]
    pub fn is_lfrc_ext(&self) -> bool {
        *self == Fncsel25::LfrcExt
    }
    #[doc = "JTAG tms input"]
    #[inline(always)]
    pub fn is_dsp_tms(&self) -> bool {
        *self == Fncsel25::DspTms
    }
    #[doc = "Timer/Counter input or output; Selection of direction is done via CTIMER register settings."]
    #[inline(always)]
    pub fn is_ct25(&self) -> bool {
        *self == Fncsel25::Ct25
    }
    #[doc = "IOMSTR/MSPI N Chip Select. Polarity is determined by CE_POLARITY field"]
    #[inline(always)]
    pub fn is_nce25(&self) -> bool {
        *self == Fncsel25::Nce25
    }
    #[doc = "Observation bus bit 9"]
    #[inline(always)]
    pub fn is_obsbus9(&self) -> bool {
        *self == Fncsel25::Obsbus9
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved9(&self) -> bool {
        *self == Fncsel25::Reserved9
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved10(&self) -> bool {
        *self == Fncsel25::Reserved10
    }
    #[doc = "Fast PIO"]
    #[inline(always)]
    pub fn is_fpio(&self) -> bool {
        *self == Fncsel25::Fpio
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved12(&self) -> bool {
        *self == Fncsel25::Reserved12
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved13(&self) -> bool {
        *self == Fncsel25::Reserved13
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved14(&self) -> bool {
        *self == Fncsel25::Reserved14
    }
    #[doc = "Internal function (SCAN)"]
    #[inline(always)]
    pub fn is_scanin8(&self) -> bool {
        *self == Fncsel25::Scanin8
    }
}
#[doc = "Field `FNCSEL25` writer - Function select for GPIO pin 25"]
pub type Fncsel25W<'a, REG> = crate::FieldWriter<'a, REG, 4, Fncsel25, crate::Safe>;
impl<'a, REG> Fncsel25W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Serial I2C Master Clock output (IOM 2)"]
    #[inline(always)]
    pub fn m2scl(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel25::M2scl)
    }
    #[doc = "Serial SPI Master Clock output (IOM 2)"]
    #[inline(always)]
    pub fn m2sck(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel25::M2sck)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel25::Reserved2)
    }
    #[doc = "General purpose I/O"]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel25::Gpio)
    }
    #[doc = "External LFRC Clock"]
    #[inline(always)]
    pub fn lfrc_ext(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel25::LfrcExt)
    }
    #[doc = "JTAG tms input"]
    #[inline(always)]
    pub fn dsp_tms(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel25::DspTms)
    }
    #[doc = "Timer/Counter input or output; Selection of direction is done via CTIMER register settings."]
    #[inline(always)]
    pub fn ct25(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel25::Ct25)
    }
    #[doc = "IOMSTR/MSPI N Chip Select. Polarity is determined by CE_POLARITY field"]
    #[inline(always)]
    pub fn nce25(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel25::Nce25)
    }
    #[doc = "Observation bus bit 9"]
    #[inline(always)]
    pub fn obsbus9(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel25::Obsbus9)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved9(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel25::Reserved9)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved10(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel25::Reserved10)
    }
    #[doc = "Fast PIO"]
    #[inline(always)]
    pub fn fpio(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel25::Fpio)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved12(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel25::Reserved12)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved13(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel25::Reserved13)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved14(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel25::Reserved14)
    }
    #[doc = "Internal function (SCAN)"]
    #[inline(always)]
    pub fn scanin8(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel25::Scanin8)
    }
}
#[doc = "Field `INPEN25` reader - Input enable for GPIO 25"]
pub type Inpen25R = crate::BitReader;
#[doc = "Field `INPEN25` writer - Input enable for GPIO 25"]
pub type Inpen25W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDZERO25` reader - Return 0 for read data on GPIO 25"]
pub type Rdzero25R = crate::BitReader;
#[doc = "Field `RDZERO25` writer - Return 0 for read data on GPIO 25"]
pub type Rdzero25W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Interrupt enable for GPIO 25\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Irpten25 {
    #[doc = "0: Interrupts are disabled for this GPIO"]
    Dis = 0,
    #[doc = "1: Interrupts are enabled for falling edge transition on this GPIO"]
    Intfall = 1,
    #[doc = "2: Interrupts are enabled for rising edge transitions on this GPIO"]
    Intrise = 2,
    #[doc = "3: Interrupts are enabled for any edge transition on this GPIO"]
    Intany = 3,
}
impl From<Irpten25> for u8 {
    #[inline(always)]
    fn from(variant: Irpten25) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Irpten25 {
    type Ux = u8;
}
impl crate::IsEnum for Irpten25 {}
#[doc = "Field `IRPTEN25` reader - Interrupt enable for GPIO 25"]
pub type Irpten25R = crate::FieldReader<Irpten25>;
impl Irpten25R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irpten25 {
        match self.bits {
            0 => Irpten25::Dis,
            1 => Irpten25::Intfall,
            2 => Irpten25::Intrise,
            3 => Irpten25::Intany,
            _ => unreachable!(),
        }
    }
    #[doc = "Interrupts are disabled for this GPIO"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Irpten25::Dis
    }
    #[doc = "Interrupts are enabled for falling edge transition on this GPIO"]
    #[inline(always)]
    pub fn is_intfall(&self) -> bool {
        *self == Irpten25::Intfall
    }
    #[doc = "Interrupts are enabled for rising edge transitions on this GPIO"]
    #[inline(always)]
    pub fn is_intrise(&self) -> bool {
        *self == Irpten25::Intrise
    }
    #[doc = "Interrupts are enabled for any edge transition on this GPIO"]
    #[inline(always)]
    pub fn is_intany(&self) -> bool {
        *self == Irpten25::Intany
    }
}
#[doc = "Field `IRPTEN25` writer - Interrupt enable for GPIO 25"]
pub type Irpten25W<'a, REG> = crate::FieldWriter<'a, REG, 2, Irpten25, crate::Safe>;
impl<'a, REG> Irpten25W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Interrupts are disabled for this GPIO"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten25::Dis)
    }
    #[doc = "Interrupts are enabled for falling edge transition on this GPIO"]
    #[inline(always)]
    pub fn intfall(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten25::Intfall)
    }
    #[doc = "Interrupts are enabled for rising edge transitions on this GPIO"]
    #[inline(always)]
    pub fn intrise(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten25::Intrise)
    }
    #[doc = "Interrupts are enabled for any edge transition on this GPIO"]
    #[inline(always)]
    pub fn intany(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten25::Intany)
    }
}
#[doc = "Pin IO mode selection for GPIO pin 25\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Outcfg25 {
    #[doc = "0: Output Disabled"]
    Dis = 0,
    #[doc = "1: Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    Pushpull = 1,
    #[doc = "2: Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    Od = 2,
    #[doc = "3: Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    Ts = 3,
}
impl From<Outcfg25> for u8 {
    #[inline(always)]
    fn from(variant: Outcfg25) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Outcfg25 {
    type Ux = u8;
}
impl crate::IsEnum for Outcfg25 {}
#[doc = "Field `OUTCFG25` reader - Pin IO mode selection for GPIO pin 25"]
pub type Outcfg25R = crate::FieldReader<Outcfg25>;
impl Outcfg25R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Outcfg25 {
        match self.bits {
            0 => Outcfg25::Dis,
            1 => Outcfg25::Pushpull,
            2 => Outcfg25::Od,
            3 => Outcfg25::Ts,
            _ => unreachable!(),
        }
    }
    #[doc = "Output Disabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Outcfg25::Dis
    }
    #[doc = "Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == Outcfg25::Pushpull
    }
    #[doc = "Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == Outcfg25::Od
    }
    #[doc = "Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == Outcfg25::Ts
    }
}
#[doc = "Field `OUTCFG25` writer - Pin IO mode selection for GPIO pin 25"]
pub type Outcfg25W<'a, REG> = crate::FieldWriter<'a, REG, 2, Outcfg25, crate::Safe>;
impl<'a, REG> Outcfg25W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Output Disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg25::Dis)
    }
    #[doc = "Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg25::Pushpull)
    }
    #[doc = "Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    #[inline(always)]
    pub fn od(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg25::Od)
    }
    #[doc = "Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    #[inline(always)]
    pub fn ts(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg25::Ts)
    }
}
#[doc = "Drive strength selection for GPIO 25\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ds25 {
    #[doc = "0: 0.1x output driver selected"]
    _0p1x = 0,
    #[doc = "1: 0.5x output driver selected"]
    _0p5x = 1,
    #[doc = "2: 0.75x output driver selected"]
    _0p75x = 2,
    #[doc = "3: 1.0x output driver selected"]
    _1p0x = 3,
}
impl From<Ds25> for u8 {
    #[inline(always)]
    fn from(variant: Ds25) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ds25 {
    type Ux = u8;
}
impl crate::IsEnum for Ds25 {}
#[doc = "Field `DS25` reader - Drive strength selection for GPIO 25"]
pub type Ds25R = crate::FieldReader<Ds25>;
impl Ds25R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ds25 {
        match self.bits {
            0 => Ds25::_0p1x,
            1 => Ds25::_0p5x,
            2 => Ds25::_0p75x,
            3 => Ds25::_1p0x,
            _ => unreachable!(),
        }
    }
    #[doc = "0.1x output driver selected"]
    #[inline(always)]
    pub fn is_0p1x(&self) -> bool {
        *self == Ds25::_0p1x
    }
    #[doc = "0.5x output driver selected"]
    #[inline(always)]
    pub fn is_0p5x(&self) -> bool {
        *self == Ds25::_0p5x
    }
    #[doc = "0.75x output driver selected"]
    #[inline(always)]
    pub fn is_0p75x(&self) -> bool {
        *self == Ds25::_0p75x
    }
    #[doc = "1.0x output driver selected"]
    #[inline(always)]
    pub fn is_1p0x(&self) -> bool {
        *self == Ds25::_1p0x
    }
}
#[doc = "Field `DS25` writer - Drive strength selection for GPIO 25"]
pub type Ds25W<'a, REG> = crate::FieldWriter<'a, REG, 2, Ds25, crate::Safe>;
impl<'a, REG> Ds25W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "0.1x output driver selected"]
    #[inline(always)]
    pub fn _0p1x(self) -> &'a mut crate::W<REG> {
        self.variant(Ds25::_0p1x)
    }
    #[doc = "0.5x output driver selected"]
    #[inline(always)]
    pub fn _0p5x(self) -> &'a mut crate::W<REG> {
        self.variant(Ds25::_0p5x)
    }
    #[doc = "0.75x output driver selected"]
    #[inline(always)]
    pub fn _0p75x(self) -> &'a mut crate::W<REG> {
        self.variant(Ds25::_0p75x)
    }
    #[doc = "1.0x output driver selected"]
    #[inline(always)]
    pub fn _1p0x(self) -> &'a mut crate::W<REG> {
        self.variant(Ds25::_1p0x)
    }
}
#[doc = "Field `SR25` reader - Configure the slew rate"]
pub type Sr25R = crate::BitReader;
#[doc = "Field `SR25` writer - Configure the slew rate"]
pub type Sr25W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Pullup/Pulldown configuration for GPIO 25\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pullcfg25 {
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
impl From<Pullcfg25> for u8 {
    #[inline(always)]
    fn from(variant: Pullcfg25) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pullcfg25 {
    type Ux = u8;
}
impl crate::IsEnum for Pullcfg25 {}
#[doc = "Field `PULLCFG25` reader - Pullup/Pulldown configuration for GPIO 25"]
pub type Pullcfg25R = crate::FieldReader<Pullcfg25>;
impl Pullcfg25R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pullcfg25 {
        match self.bits {
            0 => Pullcfg25::Dis,
            1 => Pullcfg25::Pd50k,
            2 => Pullcfg25::Pu15k,
            3 => Pullcfg25::Pu6k,
            4 => Pullcfg25::Pu12k,
            5 => Pullcfg25::Pu24k,
            6 => Pullcfg25::Pu50k,
            7 => Pullcfg25::Pu100k,
            _ => unreachable!(),
        }
    }
    #[doc = "No pullup or pulldown selected"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Pullcfg25::Dis
    }
    #[doc = "50K Pulldown selected"]
    #[inline(always)]
    pub fn is_pd50k(&self) -> bool {
        *self == Pullcfg25::Pd50k
    }
    #[doc = "1.5K Pullup selected"]
    #[inline(always)]
    pub fn is_pu15k(&self) -> bool {
        *self == Pullcfg25::Pu15k
    }
    #[doc = "6K Pullup selected"]
    #[inline(always)]
    pub fn is_pu6k(&self) -> bool {
        *self == Pullcfg25::Pu6k
    }
    #[doc = "12K Pullup selected"]
    #[inline(always)]
    pub fn is_pu12k(&self) -> bool {
        *self == Pullcfg25::Pu12k
    }
    #[doc = "24K Pullup selected"]
    #[inline(always)]
    pub fn is_pu24k(&self) -> bool {
        *self == Pullcfg25::Pu24k
    }
    #[doc = "50K Pullup selected"]
    #[inline(always)]
    pub fn is_pu50k(&self) -> bool {
        *self == Pullcfg25::Pu50k
    }
    #[doc = "100K Pullup selected"]
    #[inline(always)]
    pub fn is_pu100k(&self) -> bool {
        *self == Pullcfg25::Pu100k
    }
}
#[doc = "Field `PULLCFG25` writer - Pullup/Pulldown configuration for GPIO 25"]
pub type Pullcfg25W<'a, REG> = crate::FieldWriter<'a, REG, 3, Pullcfg25, crate::Safe>;
impl<'a, REG> Pullcfg25W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No pullup or pulldown selected"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg25::Dis)
    }
    #[doc = "50K Pulldown selected"]
    #[inline(always)]
    pub fn pd50k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg25::Pd50k)
    }
    #[doc = "1.5K Pullup selected"]
    #[inline(always)]
    pub fn pu15k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg25::Pu15k)
    }
    #[doc = "6K Pullup selected"]
    #[inline(always)]
    pub fn pu6k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg25::Pu6k)
    }
    #[doc = "12K Pullup selected"]
    #[inline(always)]
    pub fn pu12k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg25::Pu12k)
    }
    #[doc = "24K Pullup selected"]
    #[inline(always)]
    pub fn pu24k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg25::Pu24k)
    }
    #[doc = "50K Pullup selected"]
    #[inline(always)]
    pub fn pu50k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg25::Pu50k)
    }
    #[doc = "100K Pullup selected"]
    #[inline(always)]
    pub fn pu100k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg25::Pu100k)
    }
}
#[doc = "IOMSTR/MSPI N Chip Select 25, DISP control signals DE, CSX, and CS. Polarity is determined by CE_POLARITY field\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ncesrc25 {
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
impl From<Ncesrc25> for u8 {
    #[inline(always)]
    fn from(variant: Ncesrc25) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ncesrc25 {
    type Ux = u8;
}
impl crate::IsEnum for Ncesrc25 {}
#[doc = "Field `NCESRC25` reader - IOMSTR/MSPI N Chip Select 25, DISP control signals DE, CSX, and CS. Polarity is determined by CE_POLARITY field"]
pub type Ncesrc25R = crate::FieldReader<Ncesrc25>;
impl Ncesrc25R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ncesrc25> {
        match self.bits {
            0 => Some(Ncesrc25::Iom0ce0),
            1 => Some(Ncesrc25::Iom0ce1),
            2 => Some(Ncesrc25::Iom0ce2),
            3 => Some(Ncesrc25::Iom0ce3),
            4 => Some(Ncesrc25::Iom1ce0),
            5 => Some(Ncesrc25::Iom1ce1),
            6 => Some(Ncesrc25::Iom1ce2),
            7 => Some(Ncesrc25::Iom1ce3),
            8 => Some(Ncesrc25::Iom2ce0),
            9 => Some(Ncesrc25::Iom2ce1),
            10 => Some(Ncesrc25::Iom2ce2),
            11 => Some(Ncesrc25::Iom2ce3),
            12 => Some(Ncesrc25::Iom3ce0),
            13 => Some(Ncesrc25::Iom3ce1),
            14 => Some(Ncesrc25::Iom3ce2),
            15 => Some(Ncesrc25::Iom3ce3),
            16 => Some(Ncesrc25::Iom4ce0),
            17 => Some(Ncesrc25::Iom4ce1),
            18 => Some(Ncesrc25::Iom4ce2),
            19 => Some(Ncesrc25::Iom4ce3),
            20 => Some(Ncesrc25::Iom5ce0),
            21 => Some(Ncesrc25::Iom5ce1),
            22 => Some(Ncesrc25::Iom5ce2),
            23 => Some(Ncesrc25::Iom5ce3),
            24 => Some(Ncesrc25::Iom6ce0),
            25 => Some(Ncesrc25::Iom6ce1),
            26 => Some(Ncesrc25::Iom6ce2),
            27 => Some(Ncesrc25::Iom6ce3),
            28 => Some(Ncesrc25::Iom7ce0),
            29 => Some(Ncesrc25::Iom7ce1),
            30 => Some(Ncesrc25::Iom7ce2),
            31 => Some(Ncesrc25::Iom7ce3),
            32 => Some(Ncesrc25::Mspi0cen0),
            33 => Some(Ncesrc25::Mspi0cen1),
            34 => Some(Ncesrc25::Mspi1cen0),
            35 => Some(Ncesrc25::Mspi1cen1),
            36 => Some(Ncesrc25::Mspi2cen0),
            37 => Some(Ncesrc25::Mspi2cen1),
            38 => Some(Ncesrc25::DcDpiDe),
            39 => Some(Ncesrc25::DispContCsx),
            40 => Some(Ncesrc25::DcSpiCsN),
            41 => Some(Ncesrc25::DcQspiCsN),
            42 => Some(Ncesrc25::DcResx),
            _ => None,
        }
    }
    #[doc = "IOM 0 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom0ce0(&self) -> bool {
        *self == Ncesrc25::Iom0ce0
    }
    #[doc = "IOM 0 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom0ce1(&self) -> bool {
        *self == Ncesrc25::Iom0ce1
    }
    #[doc = "IOM 0 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom0ce2(&self) -> bool {
        *self == Ncesrc25::Iom0ce2
    }
    #[doc = "IOM 0 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom0ce3(&self) -> bool {
        *self == Ncesrc25::Iom0ce3
    }
    #[doc = "IOM 1 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom1ce0(&self) -> bool {
        *self == Ncesrc25::Iom1ce0
    }
    #[doc = "IOM 1 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom1ce1(&self) -> bool {
        *self == Ncesrc25::Iom1ce1
    }
    #[doc = "IOM 1 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom1ce2(&self) -> bool {
        *self == Ncesrc25::Iom1ce2
    }
    #[doc = "IOM 1 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom1ce3(&self) -> bool {
        *self == Ncesrc25::Iom1ce3
    }
    #[doc = "IOM 2 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom2ce0(&self) -> bool {
        *self == Ncesrc25::Iom2ce0
    }
    #[doc = "IOM 2 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom2ce1(&self) -> bool {
        *self == Ncesrc25::Iom2ce1
    }
    #[doc = "IOM 2 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom2ce2(&self) -> bool {
        *self == Ncesrc25::Iom2ce2
    }
    #[doc = "IOM 2 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom2ce3(&self) -> bool {
        *self == Ncesrc25::Iom2ce3
    }
    #[doc = "IOM 3 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom3ce0(&self) -> bool {
        *self == Ncesrc25::Iom3ce0
    }
    #[doc = "IOM 3 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom3ce1(&self) -> bool {
        *self == Ncesrc25::Iom3ce1
    }
    #[doc = "IOM 3 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom3ce2(&self) -> bool {
        *self == Ncesrc25::Iom3ce2
    }
    #[doc = "IOM 3 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom3ce3(&self) -> bool {
        *self == Ncesrc25::Iom3ce3
    }
    #[doc = "IOM 4 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom4ce0(&self) -> bool {
        *self == Ncesrc25::Iom4ce0
    }
    #[doc = "IOM 4 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom4ce1(&self) -> bool {
        *self == Ncesrc25::Iom4ce1
    }
    #[doc = "IOM 4 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom4ce2(&self) -> bool {
        *self == Ncesrc25::Iom4ce2
    }
    #[doc = "IOM 4 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom4ce3(&self) -> bool {
        *self == Ncesrc25::Iom4ce3
    }
    #[doc = "IOM 5 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom5ce0(&self) -> bool {
        *self == Ncesrc25::Iom5ce0
    }
    #[doc = "IOM 5 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom5ce1(&self) -> bool {
        *self == Ncesrc25::Iom5ce1
    }
    #[doc = "IOM 5 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom5ce2(&self) -> bool {
        *self == Ncesrc25::Iom5ce2
    }
    #[doc = "IOM 5 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom5ce3(&self) -> bool {
        *self == Ncesrc25::Iom5ce3
    }
    #[doc = "IOM 6 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom6ce0(&self) -> bool {
        *self == Ncesrc25::Iom6ce0
    }
    #[doc = "IOM 6 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom6ce1(&self) -> bool {
        *self == Ncesrc25::Iom6ce1
    }
    #[doc = "IOM 6 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom6ce2(&self) -> bool {
        *self == Ncesrc25::Iom6ce2
    }
    #[doc = "IOM 6 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom6ce3(&self) -> bool {
        *self == Ncesrc25::Iom6ce3
    }
    #[doc = "IOM 7 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom7ce0(&self) -> bool {
        *self == Ncesrc25::Iom7ce0
    }
    #[doc = "IOM 7 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom7ce1(&self) -> bool {
        *self == Ncesrc25::Iom7ce1
    }
    #[doc = "IOM 7 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom7ce2(&self) -> bool {
        *self == Ncesrc25::Iom7ce2
    }
    #[doc = "IOM 7 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom7ce3(&self) -> bool {
        *self == Ncesrc25::Iom7ce3
    }
    #[doc = "MSPI 0 NCE 0 module"]
    #[inline(always)]
    pub fn is_mspi0cen0(&self) -> bool {
        *self == Ncesrc25::Mspi0cen0
    }
    #[doc = "MSPI 0 NCE 1 module"]
    #[inline(always)]
    pub fn is_mspi0cen1(&self) -> bool {
        *self == Ncesrc25::Mspi0cen1
    }
    #[doc = "MSPI 1 NCE 0 module"]
    #[inline(always)]
    pub fn is_mspi1cen0(&self) -> bool {
        *self == Ncesrc25::Mspi1cen0
    }
    #[doc = "MSPI 1 NCE 1 module"]
    #[inline(always)]
    pub fn is_mspi1cen1(&self) -> bool {
        *self == Ncesrc25::Mspi1cen1
    }
    #[doc = "MSPI 2 NCE 0 module"]
    #[inline(always)]
    pub fn is_mspi2cen0(&self) -> bool {
        *self == Ncesrc25::Mspi2cen0
    }
    #[doc = "MSPI 2 NCE 1 module"]
    #[inline(always)]
    pub fn is_mspi2cen1(&self) -> bool {
        *self == Ncesrc25::Mspi2cen1
    }
    #[doc = "DC DPI DE module"]
    #[inline(always)]
    pub fn is_dc_dpi_de(&self) -> bool {
        *self == Ncesrc25::DcDpiDe
    }
    #[doc = "DISP CONT CSX module"]
    #[inline(always)]
    pub fn is_disp_cont_csx(&self) -> bool {
        *self == Ncesrc25::DispContCsx
    }
    #[doc = "DC SPI CS_N module"]
    #[inline(always)]
    pub fn is_dc_spi_cs_n(&self) -> bool {
        *self == Ncesrc25::DcSpiCsN
    }
    #[doc = "DC QSPI CS_N module"]
    #[inline(always)]
    pub fn is_dc_qspi_cs_n(&self) -> bool {
        *self == Ncesrc25::DcQspiCsN
    }
    #[doc = "DC module RESX"]
    #[inline(always)]
    pub fn is_dc_resx(&self) -> bool {
        *self == Ncesrc25::DcResx
    }
}
#[doc = "Field `NCESRC25` writer - IOMSTR/MSPI N Chip Select 25, DISP control signals DE, CSX, and CS. Polarity is determined by CE_POLARITY field"]
pub type Ncesrc25W<'a, REG> = crate::FieldWriter<'a, REG, 6, Ncesrc25>;
impl<'a, REG> Ncesrc25W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "IOM 0 NCE 0 module"]
    #[inline(always)]
    pub fn iom0ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc25::Iom0ce0)
    }
    #[doc = "IOM 0 NCE 1 module"]
    #[inline(always)]
    pub fn iom0ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc25::Iom0ce1)
    }
    #[doc = "IOM 0 NCE 2 module"]
    #[inline(always)]
    pub fn iom0ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc25::Iom0ce2)
    }
    #[doc = "IOM 0 NCE 3 module"]
    #[inline(always)]
    pub fn iom0ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc25::Iom0ce3)
    }
    #[doc = "IOM 1 NCE 0 module"]
    #[inline(always)]
    pub fn iom1ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc25::Iom1ce0)
    }
    #[doc = "IOM 1 NCE 1 module"]
    #[inline(always)]
    pub fn iom1ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc25::Iom1ce1)
    }
    #[doc = "IOM 1 NCE 2 module"]
    #[inline(always)]
    pub fn iom1ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc25::Iom1ce2)
    }
    #[doc = "IOM 1 NCE 3 module"]
    #[inline(always)]
    pub fn iom1ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc25::Iom1ce3)
    }
    #[doc = "IOM 2 NCE 0 module"]
    #[inline(always)]
    pub fn iom2ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc25::Iom2ce0)
    }
    #[doc = "IOM 2 NCE 1 module"]
    #[inline(always)]
    pub fn iom2ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc25::Iom2ce1)
    }
    #[doc = "IOM 2 NCE 2 module"]
    #[inline(always)]
    pub fn iom2ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc25::Iom2ce2)
    }
    #[doc = "IOM 2 NCE 3 module"]
    #[inline(always)]
    pub fn iom2ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc25::Iom2ce3)
    }
    #[doc = "IOM 3 NCE 0 module"]
    #[inline(always)]
    pub fn iom3ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc25::Iom3ce0)
    }
    #[doc = "IOM 3 NCE 1 module"]
    #[inline(always)]
    pub fn iom3ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc25::Iom3ce1)
    }
    #[doc = "IOM 3 NCE 2 module"]
    #[inline(always)]
    pub fn iom3ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc25::Iom3ce2)
    }
    #[doc = "IOM 3 NCE 3 module"]
    #[inline(always)]
    pub fn iom3ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc25::Iom3ce3)
    }
    #[doc = "IOM 4 NCE 0 module"]
    #[inline(always)]
    pub fn iom4ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc25::Iom4ce0)
    }
    #[doc = "IOM 4 NCE 1 module"]
    #[inline(always)]
    pub fn iom4ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc25::Iom4ce1)
    }
    #[doc = "IOM 4 NCE 2 module"]
    #[inline(always)]
    pub fn iom4ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc25::Iom4ce2)
    }
    #[doc = "IOM 4 NCE 3 module"]
    #[inline(always)]
    pub fn iom4ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc25::Iom4ce3)
    }
    #[doc = "IOM 5 NCE 0 module"]
    #[inline(always)]
    pub fn iom5ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc25::Iom5ce0)
    }
    #[doc = "IOM 5 NCE 1 module"]
    #[inline(always)]
    pub fn iom5ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc25::Iom5ce1)
    }
    #[doc = "IOM 5 NCE 2 module"]
    #[inline(always)]
    pub fn iom5ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc25::Iom5ce2)
    }
    #[doc = "IOM 5 NCE 3 module"]
    #[inline(always)]
    pub fn iom5ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc25::Iom5ce3)
    }
    #[doc = "IOM 6 NCE 0 module"]
    #[inline(always)]
    pub fn iom6ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc25::Iom6ce0)
    }
    #[doc = "IOM 6 NCE 1 module"]
    #[inline(always)]
    pub fn iom6ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc25::Iom6ce1)
    }
    #[doc = "IOM 6 NCE 2 module"]
    #[inline(always)]
    pub fn iom6ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc25::Iom6ce2)
    }
    #[doc = "IOM 6 NCE 3 module"]
    #[inline(always)]
    pub fn iom6ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc25::Iom6ce3)
    }
    #[doc = "IOM 7 NCE 0 module"]
    #[inline(always)]
    pub fn iom7ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc25::Iom7ce0)
    }
    #[doc = "IOM 7 NCE 1 module"]
    #[inline(always)]
    pub fn iom7ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc25::Iom7ce1)
    }
    #[doc = "IOM 7 NCE 2 module"]
    #[inline(always)]
    pub fn iom7ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc25::Iom7ce2)
    }
    #[doc = "IOM 7 NCE 3 module"]
    #[inline(always)]
    pub fn iom7ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc25::Iom7ce3)
    }
    #[doc = "MSPI 0 NCE 0 module"]
    #[inline(always)]
    pub fn mspi0cen0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc25::Mspi0cen0)
    }
    #[doc = "MSPI 0 NCE 1 module"]
    #[inline(always)]
    pub fn mspi0cen1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc25::Mspi0cen1)
    }
    #[doc = "MSPI 1 NCE 0 module"]
    #[inline(always)]
    pub fn mspi1cen0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc25::Mspi1cen0)
    }
    #[doc = "MSPI 1 NCE 1 module"]
    #[inline(always)]
    pub fn mspi1cen1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc25::Mspi1cen1)
    }
    #[doc = "MSPI 2 NCE 0 module"]
    #[inline(always)]
    pub fn mspi2cen0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc25::Mspi2cen0)
    }
    #[doc = "MSPI 2 NCE 1 module"]
    #[inline(always)]
    pub fn mspi2cen1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc25::Mspi2cen1)
    }
    #[doc = "DC DPI DE module"]
    #[inline(always)]
    pub fn dc_dpi_de(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc25::DcDpiDe)
    }
    #[doc = "DISP CONT CSX module"]
    #[inline(always)]
    pub fn disp_cont_csx(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc25::DispContCsx)
    }
    #[doc = "DC SPI CS_N module"]
    #[inline(always)]
    pub fn dc_spi_cs_n(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc25::DcSpiCsN)
    }
    #[doc = "DC QSPI CS_N module"]
    #[inline(always)]
    pub fn dc_qspi_cs_n(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc25::DcQspiCsN)
    }
    #[doc = "DC module RESX"]
    #[inline(always)]
    pub fn dc_resx(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc25::DcResx)
    }
}
#[doc = "Polarity select for NCE for GPIO 25\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ncepol25 {
    #[doc = "0: Polarity is active low"]
    Low = 0,
    #[doc = "1: Polarity is active high"]
    High = 1,
}
impl From<Ncepol25> for bool {
    #[inline(always)]
    fn from(variant: Ncepol25) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NCEPOL25` reader - Polarity select for NCE for GPIO 25"]
pub type Ncepol25R = crate::BitReader<Ncepol25>;
impl Ncepol25R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ncepol25 {
        match self.bits {
            false => Ncepol25::Low,
            true => Ncepol25::High,
        }
    }
    #[doc = "Polarity is active low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Ncepol25::Low
    }
    #[doc = "Polarity is active high"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Ncepol25::High
    }
}
#[doc = "Field `NCEPOL25` writer - Polarity select for NCE for GPIO 25"]
pub type Ncepol25W<'a, REG> = crate::BitWriter<'a, REG, Ncepol25>;
impl<'a, REG> Ncepol25W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Polarity is active low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Ncepol25::Low)
    }
    #[doc = "Polarity is active high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Ncepol25::High)
    }
}
#[doc = "Field `FIEN25` reader - Force input enable active regardless of function selected. Otherwise the selected function will enable the input only when needed"]
pub type Fien25R = crate::BitReader;
#[doc = "Field `FIEN25` writer - Force input enable active regardless of function selected. Otherwise the selected function will enable the input only when needed"]
pub type Fien25W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FOEN25` reader - Force output enable active regardless of function selected. Otherwise the selected function will enable the output only when needed"]
pub type Foen25R = crate::BitReader;
#[doc = "Field `FOEN25` writer - Force output enable active regardless of function selected. Otherwise the selected function will enable the output only when needed"]
pub type Foen25W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Function select for GPIO pin 25"]
    #[inline(always)]
    pub fn fncsel25(&self) -> Fncsel25R {
        Fncsel25R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Input enable for GPIO 25"]
    #[inline(always)]
    pub fn inpen25(&self) -> Inpen25R {
        Inpen25R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Return 0 for read data on GPIO 25"]
    #[inline(always)]
    pub fn rdzero25(&self) -> Rdzero25R {
        Rdzero25R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Interrupt enable for GPIO 25"]
    #[inline(always)]
    pub fn irpten25(&self) -> Irpten25R {
        Irpten25R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Pin IO mode selection for GPIO pin 25"]
    #[inline(always)]
    pub fn outcfg25(&self) -> Outcfg25R {
        Outcfg25R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Drive strength selection for GPIO 25"]
    #[inline(always)]
    pub fn ds25(&self) -> Ds25R {
        Ds25R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - Configure the slew rate"]
    #[inline(always)]
    pub fn sr25(&self) -> Sr25R {
        Sr25R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - Pullup/Pulldown configuration for GPIO 25"]
    #[inline(always)]
    pub fn pullcfg25(&self) -> Pullcfg25R {
        Pullcfg25R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:21 - IOMSTR/MSPI N Chip Select 25, DISP control signals DE, CSX, and CS. Polarity is determined by CE_POLARITY field"]
    #[inline(always)]
    pub fn ncesrc25(&self) -> Ncesrc25R {
        Ncesrc25R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 22 - Polarity select for NCE for GPIO 25"]
    #[inline(always)]
    pub fn ncepol25(&self) -> Ncepol25R {
        Ncepol25R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 26 - Force input enable active regardless of function selected. Otherwise the selected function will enable the input only when needed"]
    #[inline(always)]
    pub fn fien25(&self) -> Fien25R {
        Fien25R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Force output enable active regardless of function selected. Otherwise the selected function will enable the output only when needed"]
    #[inline(always)]
    pub fn foen25(&self) -> Foen25R {
        Foen25R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Function select for GPIO pin 25"]
    #[inline(always)]
    #[must_use]
    pub fn fncsel25(&mut self) -> Fncsel25W<Pincfg25Spec> {
        Fncsel25W::new(self, 0)
    }
    #[doc = "Bit 4 - Input enable for GPIO 25"]
    #[inline(always)]
    #[must_use]
    pub fn inpen25(&mut self) -> Inpen25W<Pincfg25Spec> {
        Inpen25W::new(self, 4)
    }
    #[doc = "Bit 5 - Return 0 for read data on GPIO 25"]
    #[inline(always)]
    #[must_use]
    pub fn rdzero25(&mut self) -> Rdzero25W<Pincfg25Spec> {
        Rdzero25W::new(self, 5)
    }
    #[doc = "Bits 6:7 - Interrupt enable for GPIO 25"]
    #[inline(always)]
    #[must_use]
    pub fn irpten25(&mut self) -> Irpten25W<Pincfg25Spec> {
        Irpten25W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Pin IO mode selection for GPIO pin 25"]
    #[inline(always)]
    #[must_use]
    pub fn outcfg25(&mut self) -> Outcfg25W<Pincfg25Spec> {
        Outcfg25W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Drive strength selection for GPIO 25"]
    #[inline(always)]
    #[must_use]
    pub fn ds25(&mut self) -> Ds25W<Pincfg25Spec> {
        Ds25W::new(self, 10)
    }
    #[doc = "Bit 12 - Configure the slew rate"]
    #[inline(always)]
    #[must_use]
    pub fn sr25(&mut self) -> Sr25W<Pincfg25Spec> {
        Sr25W::new(self, 12)
    }
    #[doc = "Bits 13:15 - Pullup/Pulldown configuration for GPIO 25"]
    #[inline(always)]
    #[must_use]
    pub fn pullcfg25(&mut self) -> Pullcfg25W<Pincfg25Spec> {
        Pullcfg25W::new(self, 13)
    }
    #[doc = "Bits 16:21 - IOMSTR/MSPI N Chip Select 25, DISP control signals DE, CSX, and CS. Polarity is determined by CE_POLARITY field"]
    #[inline(always)]
    #[must_use]
    pub fn ncesrc25(&mut self) -> Ncesrc25W<Pincfg25Spec> {
        Ncesrc25W::new(self, 16)
    }
    #[doc = "Bit 22 - Polarity select for NCE for GPIO 25"]
    #[inline(always)]
    #[must_use]
    pub fn ncepol25(&mut self) -> Ncepol25W<Pincfg25Spec> {
        Ncepol25W::new(self, 22)
    }
    #[doc = "Bit 26 - Force input enable active regardless of function selected. Otherwise the selected function will enable the input only when needed"]
    #[inline(always)]
    #[must_use]
    pub fn fien25(&mut self) -> Fien25W<Pincfg25Spec> {
        Fien25W::new(self, 26)
    }
    #[doc = "Bit 27 - Force output enable active regardless of function selected. Otherwise the selected function will enable the output only when needed"]
    #[inline(always)]
    #[must_use]
    pub fn foen25(&mut self) -> Foen25W<Pincfg25Spec> {
        Foen25W::new(self, 27)
    }
}
#[doc = "Controls the operation of GPIO pin 25.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg25::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg25::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pincfg25Spec;
impl crate::RegisterSpec for Pincfg25Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pincfg25::R`](R) reader structure"]
impl crate::Readable for Pincfg25Spec {}
#[doc = "`write(|w| ..)` method takes [`pincfg25::W`](W) writer structure"]
impl crate::Writable for Pincfg25Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PINCFG25 to value 0x03"]
impl crate::Resettable for Pincfg25Spec {
    const RESET_VALUE: u32 = 0x03;
}
