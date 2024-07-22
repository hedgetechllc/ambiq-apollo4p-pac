#[doc = "Register `PINCFG96` reader"]
pub type R = crate::R<Pincfg96Spec>;
#[doc = "Register `PINCFG96` writer"]
pub type W = crate::W<Pincfg96Spec>;
#[doc = "Function select for GPIO pin 96\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fncsel96 {
    #[doc = "0: MSPI Master 1 Interface Signal"]
    Mspi1_1 = 0,
    #[doc = "1: Reserved selection. Operation unknown if selected."]
    Reserved1 = 1,
    #[doc = "2: Reserved selection. Operation unknown if selected."]
    Reserved2 = 2,
    #[doc = "3: General purpose I/O"]
    Gpio = 3,
    #[doc = "4: Reserved selection. Operation unknown if selected."]
    Reserved4 = 4,
    #[doc = "5: Reserved selection. Operation unknown if selected."]
    Reserved5 = 5,
    #[doc = "6: Timer/Counter input or output; Selection of direction is done via CTIMER register settings."]
    Ct96 = 6,
    #[doc = "7: IOMSTR/MSPI N Chip Select. Polarity is determined by CE_POLARITY field"]
    Nce96 = 7,
    #[doc = "8: Observation bus bit 0"]
    Obsbus0 = 8,
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
    #[doc = "15: Reserved selection. Operation unknown if selected."]
    Reserved15 = 15,
}
impl From<Fncsel96> for u8 {
    #[inline(always)]
    fn from(variant: Fncsel96) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fncsel96 {
    type Ux = u8;
}
impl crate::IsEnum for Fncsel96 {}
#[doc = "Field `FNCSEL96` reader - Function select for GPIO pin 96"]
pub type Fncsel96R = crate::FieldReader<Fncsel96>;
impl Fncsel96R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fncsel96 {
        match self.bits {
            0 => Fncsel96::Mspi1_1,
            1 => Fncsel96::Reserved1,
            2 => Fncsel96::Reserved2,
            3 => Fncsel96::Gpio,
            4 => Fncsel96::Reserved4,
            5 => Fncsel96::Reserved5,
            6 => Fncsel96::Ct96,
            7 => Fncsel96::Nce96,
            8 => Fncsel96::Obsbus0,
            9 => Fncsel96::Reserved9,
            10 => Fncsel96::Reserved10,
            11 => Fncsel96::Fpio,
            12 => Fncsel96::Reserved12,
            13 => Fncsel96::Reserved13,
            14 => Fncsel96::Reserved14,
            15 => Fncsel96::Reserved15,
            _ => unreachable!(),
        }
    }
    #[doc = "MSPI Master 1 Interface Signal"]
    #[inline(always)]
    pub fn is_mspi1_1(&self) -> bool {
        *self == Fncsel96::Mspi1_1
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved1(&self) -> bool {
        *self == Fncsel96::Reserved1
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == Fncsel96::Reserved2
    }
    #[doc = "General purpose I/O"]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == Fncsel96::Gpio
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved4(&self) -> bool {
        *self == Fncsel96::Reserved4
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved5(&self) -> bool {
        *self == Fncsel96::Reserved5
    }
    #[doc = "Timer/Counter input or output; Selection of direction is done via CTIMER register settings."]
    #[inline(always)]
    pub fn is_ct96(&self) -> bool {
        *self == Fncsel96::Ct96
    }
    #[doc = "IOMSTR/MSPI N Chip Select. Polarity is determined by CE_POLARITY field"]
    #[inline(always)]
    pub fn is_nce96(&self) -> bool {
        *self == Fncsel96::Nce96
    }
    #[doc = "Observation bus bit 0"]
    #[inline(always)]
    pub fn is_obsbus0(&self) -> bool {
        *self == Fncsel96::Obsbus0
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved9(&self) -> bool {
        *self == Fncsel96::Reserved9
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved10(&self) -> bool {
        *self == Fncsel96::Reserved10
    }
    #[doc = "Fast PIO"]
    #[inline(always)]
    pub fn is_fpio(&self) -> bool {
        *self == Fncsel96::Fpio
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved12(&self) -> bool {
        *self == Fncsel96::Reserved12
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved13(&self) -> bool {
        *self == Fncsel96::Reserved13
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved14(&self) -> bool {
        *self == Fncsel96::Reserved14
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved15(&self) -> bool {
        *self == Fncsel96::Reserved15
    }
}
#[doc = "Field `FNCSEL96` writer - Function select for GPIO pin 96"]
pub type Fncsel96W<'a, REG> = crate::FieldWriter<'a, REG, 4, Fncsel96, crate::Safe>;
impl<'a, REG> Fncsel96W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "MSPI Master 1 Interface Signal"]
    #[inline(always)]
    pub fn mspi1_1(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel96::Mspi1_1)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved1(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel96::Reserved1)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel96::Reserved2)
    }
    #[doc = "General purpose I/O"]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel96::Gpio)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved4(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel96::Reserved4)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved5(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel96::Reserved5)
    }
    #[doc = "Timer/Counter input or output; Selection of direction is done via CTIMER register settings."]
    #[inline(always)]
    pub fn ct96(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel96::Ct96)
    }
    #[doc = "IOMSTR/MSPI N Chip Select. Polarity is determined by CE_POLARITY field"]
    #[inline(always)]
    pub fn nce96(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel96::Nce96)
    }
    #[doc = "Observation bus bit 0"]
    #[inline(always)]
    pub fn obsbus0(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel96::Obsbus0)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved9(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel96::Reserved9)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved10(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel96::Reserved10)
    }
    #[doc = "Fast PIO"]
    #[inline(always)]
    pub fn fpio(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel96::Fpio)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved12(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel96::Reserved12)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved13(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel96::Reserved13)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved14(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel96::Reserved14)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved15(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel96::Reserved15)
    }
}
#[doc = "Field `INPEN96` reader - Input enable for GPIO 96"]
pub type Inpen96R = crate::BitReader;
#[doc = "Field `INPEN96` writer - Input enable for GPIO 96"]
pub type Inpen96W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDZERO96` reader - Return 0 for read data on GPIO 96"]
pub type Rdzero96R = crate::BitReader;
#[doc = "Field `RDZERO96` writer - Return 0 for read data on GPIO 96"]
pub type Rdzero96W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Interrupt enable for GPIO 96\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Irpten96 {
    #[doc = "0: Interrupts are disabled for this GPIO"]
    Dis = 0,
    #[doc = "1: Interrupts are enabled for falling edge transition on this GPIO"]
    Intfall = 1,
    #[doc = "2: Interrupts are enabled for rising edge transitions on this GPIO"]
    Intrise = 2,
    #[doc = "3: Interrupts are enabled for any edge transition on this GPIO"]
    Intany = 3,
}
impl From<Irpten96> for u8 {
    #[inline(always)]
    fn from(variant: Irpten96) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Irpten96 {
    type Ux = u8;
}
impl crate::IsEnum for Irpten96 {}
#[doc = "Field `IRPTEN96` reader - Interrupt enable for GPIO 96"]
pub type Irpten96R = crate::FieldReader<Irpten96>;
impl Irpten96R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irpten96 {
        match self.bits {
            0 => Irpten96::Dis,
            1 => Irpten96::Intfall,
            2 => Irpten96::Intrise,
            3 => Irpten96::Intany,
            _ => unreachable!(),
        }
    }
    #[doc = "Interrupts are disabled for this GPIO"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Irpten96::Dis
    }
    #[doc = "Interrupts are enabled for falling edge transition on this GPIO"]
    #[inline(always)]
    pub fn is_intfall(&self) -> bool {
        *self == Irpten96::Intfall
    }
    #[doc = "Interrupts are enabled for rising edge transitions on this GPIO"]
    #[inline(always)]
    pub fn is_intrise(&self) -> bool {
        *self == Irpten96::Intrise
    }
    #[doc = "Interrupts are enabled for any edge transition on this GPIO"]
    #[inline(always)]
    pub fn is_intany(&self) -> bool {
        *self == Irpten96::Intany
    }
}
#[doc = "Field `IRPTEN96` writer - Interrupt enable for GPIO 96"]
pub type Irpten96W<'a, REG> = crate::FieldWriter<'a, REG, 2, Irpten96, crate::Safe>;
impl<'a, REG> Irpten96W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Interrupts are disabled for this GPIO"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten96::Dis)
    }
    #[doc = "Interrupts are enabled for falling edge transition on this GPIO"]
    #[inline(always)]
    pub fn intfall(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten96::Intfall)
    }
    #[doc = "Interrupts are enabled for rising edge transitions on this GPIO"]
    #[inline(always)]
    pub fn intrise(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten96::Intrise)
    }
    #[doc = "Interrupts are enabled for any edge transition on this GPIO"]
    #[inline(always)]
    pub fn intany(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten96::Intany)
    }
}
#[doc = "Pin IO mode selection for GPIO pin 96\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Outcfg96 {
    #[doc = "0: Output Disabled"]
    Dis = 0,
    #[doc = "1: Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    Pushpull = 1,
    #[doc = "2: Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    Od = 2,
    #[doc = "3: Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    Ts = 3,
}
impl From<Outcfg96> for u8 {
    #[inline(always)]
    fn from(variant: Outcfg96) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Outcfg96 {
    type Ux = u8;
}
impl crate::IsEnum for Outcfg96 {}
#[doc = "Field `OUTCFG96` reader - Pin IO mode selection for GPIO pin 96"]
pub type Outcfg96R = crate::FieldReader<Outcfg96>;
impl Outcfg96R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Outcfg96 {
        match self.bits {
            0 => Outcfg96::Dis,
            1 => Outcfg96::Pushpull,
            2 => Outcfg96::Od,
            3 => Outcfg96::Ts,
            _ => unreachable!(),
        }
    }
    #[doc = "Output Disabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Outcfg96::Dis
    }
    #[doc = "Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == Outcfg96::Pushpull
    }
    #[doc = "Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == Outcfg96::Od
    }
    #[doc = "Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == Outcfg96::Ts
    }
}
#[doc = "Field `OUTCFG96` writer - Pin IO mode selection for GPIO pin 96"]
pub type Outcfg96W<'a, REG> = crate::FieldWriter<'a, REG, 2, Outcfg96, crate::Safe>;
impl<'a, REG> Outcfg96W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Output Disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg96::Dis)
    }
    #[doc = "Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg96::Pushpull)
    }
    #[doc = "Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    #[inline(always)]
    pub fn od(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg96::Od)
    }
    #[doc = "Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    #[inline(always)]
    pub fn ts(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg96::Ts)
    }
}
#[doc = "Drive strength selection for GPIO 96\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ds96 {
    #[doc = "0: 0.1x output driver selected"]
    _0p1x = 0,
    #[doc = "1: 0.5x output driver selected"]
    _0p5x = 1,
}
impl From<Ds96> for u8 {
    #[inline(always)]
    fn from(variant: Ds96) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ds96 {
    type Ux = u8;
}
impl crate::IsEnum for Ds96 {}
#[doc = "Field `DS96` reader - Drive strength selection for GPIO 96"]
pub type Ds96R = crate::FieldReader<Ds96>;
impl Ds96R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ds96> {
        match self.bits {
            0 => Some(Ds96::_0p1x),
            1 => Some(Ds96::_0p5x),
            _ => None,
        }
    }
    #[doc = "0.1x output driver selected"]
    #[inline(always)]
    pub fn is_0p1x(&self) -> bool {
        *self == Ds96::_0p1x
    }
    #[doc = "0.5x output driver selected"]
    #[inline(always)]
    pub fn is_0p5x(&self) -> bool {
        *self == Ds96::_0p5x
    }
}
#[doc = "Field `DS96` writer - Drive strength selection for GPIO 96"]
pub type Ds96W<'a, REG> = crate::FieldWriter<'a, REG, 2, Ds96>;
impl<'a, REG> Ds96W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "0.1x output driver selected"]
    #[inline(always)]
    pub fn _0p1x(self) -> &'a mut crate::W<REG> {
        self.variant(Ds96::_0p1x)
    }
    #[doc = "0.5x output driver selected"]
    #[inline(always)]
    pub fn _0p5x(self) -> &'a mut crate::W<REG> {
        self.variant(Ds96::_0p5x)
    }
}
#[doc = "Field `SR96` reader - Configure the slew rate"]
pub type Sr96R = crate::BitReader;
#[doc = "Field `SR96` writer - Configure the slew rate"]
pub type Sr96W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Pullup/Pulldown configuration for GPIO 96\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pullcfg96 {
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
impl From<Pullcfg96> for u8 {
    #[inline(always)]
    fn from(variant: Pullcfg96) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pullcfg96 {
    type Ux = u8;
}
impl crate::IsEnum for Pullcfg96 {}
#[doc = "Field `PULLCFG96` reader - Pullup/Pulldown configuration for GPIO 96"]
pub type Pullcfg96R = crate::FieldReader<Pullcfg96>;
impl Pullcfg96R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pullcfg96 {
        match self.bits {
            0 => Pullcfg96::Dis,
            1 => Pullcfg96::Pd50k,
            2 => Pullcfg96::Pu15k,
            3 => Pullcfg96::Pu6k,
            4 => Pullcfg96::Pu12k,
            5 => Pullcfg96::Pu24k,
            6 => Pullcfg96::Pu50k,
            7 => Pullcfg96::Pu100k,
            _ => unreachable!(),
        }
    }
    #[doc = "No pullup or pulldown selected"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Pullcfg96::Dis
    }
    #[doc = "50K Pulldown selected"]
    #[inline(always)]
    pub fn is_pd50k(&self) -> bool {
        *self == Pullcfg96::Pd50k
    }
    #[doc = "1.5K Pullup selected"]
    #[inline(always)]
    pub fn is_pu15k(&self) -> bool {
        *self == Pullcfg96::Pu15k
    }
    #[doc = "6K Pullup selected"]
    #[inline(always)]
    pub fn is_pu6k(&self) -> bool {
        *self == Pullcfg96::Pu6k
    }
    #[doc = "12K Pullup selected"]
    #[inline(always)]
    pub fn is_pu12k(&self) -> bool {
        *self == Pullcfg96::Pu12k
    }
    #[doc = "24K Pullup selected"]
    #[inline(always)]
    pub fn is_pu24k(&self) -> bool {
        *self == Pullcfg96::Pu24k
    }
    #[doc = "50K Pullup selected"]
    #[inline(always)]
    pub fn is_pu50k(&self) -> bool {
        *self == Pullcfg96::Pu50k
    }
    #[doc = "100K Pullup selected"]
    #[inline(always)]
    pub fn is_pu100k(&self) -> bool {
        *self == Pullcfg96::Pu100k
    }
}
#[doc = "Field `PULLCFG96` writer - Pullup/Pulldown configuration for GPIO 96"]
pub type Pullcfg96W<'a, REG> = crate::FieldWriter<'a, REG, 3, Pullcfg96, crate::Safe>;
impl<'a, REG> Pullcfg96W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No pullup or pulldown selected"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg96::Dis)
    }
    #[doc = "50K Pulldown selected"]
    #[inline(always)]
    pub fn pd50k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg96::Pd50k)
    }
    #[doc = "1.5K Pullup selected"]
    #[inline(always)]
    pub fn pu15k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg96::Pu15k)
    }
    #[doc = "6K Pullup selected"]
    #[inline(always)]
    pub fn pu6k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg96::Pu6k)
    }
    #[doc = "12K Pullup selected"]
    #[inline(always)]
    pub fn pu12k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg96::Pu12k)
    }
    #[doc = "24K Pullup selected"]
    #[inline(always)]
    pub fn pu24k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg96::Pu24k)
    }
    #[doc = "50K Pullup selected"]
    #[inline(always)]
    pub fn pu50k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg96::Pu50k)
    }
    #[doc = "100K Pullup selected"]
    #[inline(always)]
    pub fn pu100k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg96::Pu100k)
    }
}
#[doc = "IOMSTR/MSPI N Chip Select 96, DISP control signals DE, CSX, and CS. Polarity is determined by CE_POLARITY field\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ncesrc96 {
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
impl From<Ncesrc96> for u8 {
    #[inline(always)]
    fn from(variant: Ncesrc96) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ncesrc96 {
    type Ux = u8;
}
impl crate::IsEnum for Ncesrc96 {}
#[doc = "Field `NCESRC96` reader - IOMSTR/MSPI N Chip Select 96, DISP control signals DE, CSX, and CS. Polarity is determined by CE_POLARITY field"]
pub type Ncesrc96R = crate::FieldReader<Ncesrc96>;
impl Ncesrc96R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ncesrc96> {
        match self.bits {
            0 => Some(Ncesrc96::Iom0ce0),
            1 => Some(Ncesrc96::Iom0ce1),
            2 => Some(Ncesrc96::Iom0ce2),
            3 => Some(Ncesrc96::Iom0ce3),
            4 => Some(Ncesrc96::Iom1ce0),
            5 => Some(Ncesrc96::Iom1ce1),
            6 => Some(Ncesrc96::Iom1ce2),
            7 => Some(Ncesrc96::Iom1ce3),
            8 => Some(Ncesrc96::Iom2ce0),
            9 => Some(Ncesrc96::Iom2ce1),
            10 => Some(Ncesrc96::Iom2ce2),
            11 => Some(Ncesrc96::Iom2ce3),
            12 => Some(Ncesrc96::Iom3ce0),
            13 => Some(Ncesrc96::Iom3ce1),
            14 => Some(Ncesrc96::Iom3ce2),
            15 => Some(Ncesrc96::Iom3ce3),
            16 => Some(Ncesrc96::Iom4ce0),
            17 => Some(Ncesrc96::Iom4ce1),
            18 => Some(Ncesrc96::Iom4ce2),
            19 => Some(Ncesrc96::Iom4ce3),
            20 => Some(Ncesrc96::Iom5ce0),
            21 => Some(Ncesrc96::Iom5ce1),
            22 => Some(Ncesrc96::Iom5ce2),
            23 => Some(Ncesrc96::Iom5ce3),
            24 => Some(Ncesrc96::Iom6ce0),
            25 => Some(Ncesrc96::Iom6ce1),
            26 => Some(Ncesrc96::Iom6ce2),
            27 => Some(Ncesrc96::Iom6ce3),
            28 => Some(Ncesrc96::Iom7ce0),
            29 => Some(Ncesrc96::Iom7ce1),
            30 => Some(Ncesrc96::Iom7ce2),
            31 => Some(Ncesrc96::Iom7ce3),
            32 => Some(Ncesrc96::Mspi0cen0),
            33 => Some(Ncesrc96::Mspi0cen1),
            34 => Some(Ncesrc96::Mspi1cen0),
            35 => Some(Ncesrc96::Mspi1cen1),
            36 => Some(Ncesrc96::Mspi2cen0),
            37 => Some(Ncesrc96::Mspi2cen1),
            38 => Some(Ncesrc96::DcDpiDe),
            39 => Some(Ncesrc96::DispContCsx),
            40 => Some(Ncesrc96::DcSpiCsN),
            41 => Some(Ncesrc96::DcQspiCsN),
            42 => Some(Ncesrc96::DcResx),
            _ => None,
        }
    }
    #[doc = "IOM 0 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom0ce0(&self) -> bool {
        *self == Ncesrc96::Iom0ce0
    }
    #[doc = "IOM 0 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom0ce1(&self) -> bool {
        *self == Ncesrc96::Iom0ce1
    }
    #[doc = "IOM 0 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom0ce2(&self) -> bool {
        *self == Ncesrc96::Iom0ce2
    }
    #[doc = "IOM 0 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom0ce3(&self) -> bool {
        *self == Ncesrc96::Iom0ce3
    }
    #[doc = "IOM 1 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom1ce0(&self) -> bool {
        *self == Ncesrc96::Iom1ce0
    }
    #[doc = "IOM 1 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom1ce1(&self) -> bool {
        *self == Ncesrc96::Iom1ce1
    }
    #[doc = "IOM 1 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom1ce2(&self) -> bool {
        *self == Ncesrc96::Iom1ce2
    }
    #[doc = "IOM 1 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom1ce3(&self) -> bool {
        *self == Ncesrc96::Iom1ce3
    }
    #[doc = "IOM 2 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom2ce0(&self) -> bool {
        *self == Ncesrc96::Iom2ce0
    }
    #[doc = "IOM 2 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom2ce1(&self) -> bool {
        *self == Ncesrc96::Iom2ce1
    }
    #[doc = "IOM 2 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom2ce2(&self) -> bool {
        *self == Ncesrc96::Iom2ce2
    }
    #[doc = "IOM 2 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom2ce3(&self) -> bool {
        *self == Ncesrc96::Iom2ce3
    }
    #[doc = "IOM 3 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom3ce0(&self) -> bool {
        *self == Ncesrc96::Iom3ce0
    }
    #[doc = "IOM 3 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom3ce1(&self) -> bool {
        *self == Ncesrc96::Iom3ce1
    }
    #[doc = "IOM 3 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom3ce2(&self) -> bool {
        *self == Ncesrc96::Iom3ce2
    }
    #[doc = "IOM 3 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom3ce3(&self) -> bool {
        *self == Ncesrc96::Iom3ce3
    }
    #[doc = "IOM 4 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom4ce0(&self) -> bool {
        *self == Ncesrc96::Iom4ce0
    }
    #[doc = "IOM 4 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom4ce1(&self) -> bool {
        *self == Ncesrc96::Iom4ce1
    }
    #[doc = "IOM 4 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom4ce2(&self) -> bool {
        *self == Ncesrc96::Iom4ce2
    }
    #[doc = "IOM 4 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom4ce3(&self) -> bool {
        *self == Ncesrc96::Iom4ce3
    }
    #[doc = "IOM 5 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom5ce0(&self) -> bool {
        *self == Ncesrc96::Iom5ce0
    }
    #[doc = "IOM 5 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom5ce1(&self) -> bool {
        *self == Ncesrc96::Iom5ce1
    }
    #[doc = "IOM 5 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom5ce2(&self) -> bool {
        *self == Ncesrc96::Iom5ce2
    }
    #[doc = "IOM 5 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom5ce3(&self) -> bool {
        *self == Ncesrc96::Iom5ce3
    }
    #[doc = "IOM 6 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom6ce0(&self) -> bool {
        *self == Ncesrc96::Iom6ce0
    }
    #[doc = "IOM 6 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom6ce1(&self) -> bool {
        *self == Ncesrc96::Iom6ce1
    }
    #[doc = "IOM 6 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom6ce2(&self) -> bool {
        *self == Ncesrc96::Iom6ce2
    }
    #[doc = "IOM 6 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom6ce3(&self) -> bool {
        *self == Ncesrc96::Iom6ce3
    }
    #[doc = "IOM 7 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom7ce0(&self) -> bool {
        *self == Ncesrc96::Iom7ce0
    }
    #[doc = "IOM 7 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom7ce1(&self) -> bool {
        *self == Ncesrc96::Iom7ce1
    }
    #[doc = "IOM 7 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom7ce2(&self) -> bool {
        *self == Ncesrc96::Iom7ce2
    }
    #[doc = "IOM 7 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom7ce3(&self) -> bool {
        *self == Ncesrc96::Iom7ce3
    }
    #[doc = "MSPI 0 NCE 0 module"]
    #[inline(always)]
    pub fn is_mspi0cen0(&self) -> bool {
        *self == Ncesrc96::Mspi0cen0
    }
    #[doc = "MSPI 0 NCE 1 module"]
    #[inline(always)]
    pub fn is_mspi0cen1(&self) -> bool {
        *self == Ncesrc96::Mspi0cen1
    }
    #[doc = "MSPI 1 NCE 0 module"]
    #[inline(always)]
    pub fn is_mspi1cen0(&self) -> bool {
        *self == Ncesrc96::Mspi1cen0
    }
    #[doc = "MSPI 1 NCE 1 module"]
    #[inline(always)]
    pub fn is_mspi1cen1(&self) -> bool {
        *self == Ncesrc96::Mspi1cen1
    }
    #[doc = "MSPI 2 NCE 0 module"]
    #[inline(always)]
    pub fn is_mspi2cen0(&self) -> bool {
        *self == Ncesrc96::Mspi2cen0
    }
    #[doc = "MSPI 2 NCE 1 module"]
    #[inline(always)]
    pub fn is_mspi2cen1(&self) -> bool {
        *self == Ncesrc96::Mspi2cen1
    }
    #[doc = "DC DPI DE module"]
    #[inline(always)]
    pub fn is_dc_dpi_de(&self) -> bool {
        *self == Ncesrc96::DcDpiDe
    }
    #[doc = "DISP CONT CSX module"]
    #[inline(always)]
    pub fn is_disp_cont_csx(&self) -> bool {
        *self == Ncesrc96::DispContCsx
    }
    #[doc = "DC SPI CS_N module"]
    #[inline(always)]
    pub fn is_dc_spi_cs_n(&self) -> bool {
        *self == Ncesrc96::DcSpiCsN
    }
    #[doc = "DC QSPI CS_N module"]
    #[inline(always)]
    pub fn is_dc_qspi_cs_n(&self) -> bool {
        *self == Ncesrc96::DcQspiCsN
    }
    #[doc = "DC module RESX"]
    #[inline(always)]
    pub fn is_dc_resx(&self) -> bool {
        *self == Ncesrc96::DcResx
    }
}
#[doc = "Field `NCESRC96` writer - IOMSTR/MSPI N Chip Select 96, DISP control signals DE, CSX, and CS. Polarity is determined by CE_POLARITY field"]
pub type Ncesrc96W<'a, REG> = crate::FieldWriter<'a, REG, 6, Ncesrc96>;
impl<'a, REG> Ncesrc96W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "IOM 0 NCE 0 module"]
    #[inline(always)]
    pub fn iom0ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc96::Iom0ce0)
    }
    #[doc = "IOM 0 NCE 1 module"]
    #[inline(always)]
    pub fn iom0ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc96::Iom0ce1)
    }
    #[doc = "IOM 0 NCE 2 module"]
    #[inline(always)]
    pub fn iom0ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc96::Iom0ce2)
    }
    #[doc = "IOM 0 NCE 3 module"]
    #[inline(always)]
    pub fn iom0ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc96::Iom0ce3)
    }
    #[doc = "IOM 1 NCE 0 module"]
    #[inline(always)]
    pub fn iom1ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc96::Iom1ce0)
    }
    #[doc = "IOM 1 NCE 1 module"]
    #[inline(always)]
    pub fn iom1ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc96::Iom1ce1)
    }
    #[doc = "IOM 1 NCE 2 module"]
    #[inline(always)]
    pub fn iom1ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc96::Iom1ce2)
    }
    #[doc = "IOM 1 NCE 3 module"]
    #[inline(always)]
    pub fn iom1ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc96::Iom1ce3)
    }
    #[doc = "IOM 2 NCE 0 module"]
    #[inline(always)]
    pub fn iom2ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc96::Iom2ce0)
    }
    #[doc = "IOM 2 NCE 1 module"]
    #[inline(always)]
    pub fn iom2ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc96::Iom2ce1)
    }
    #[doc = "IOM 2 NCE 2 module"]
    #[inline(always)]
    pub fn iom2ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc96::Iom2ce2)
    }
    #[doc = "IOM 2 NCE 3 module"]
    #[inline(always)]
    pub fn iom2ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc96::Iom2ce3)
    }
    #[doc = "IOM 3 NCE 0 module"]
    #[inline(always)]
    pub fn iom3ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc96::Iom3ce0)
    }
    #[doc = "IOM 3 NCE 1 module"]
    #[inline(always)]
    pub fn iom3ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc96::Iom3ce1)
    }
    #[doc = "IOM 3 NCE 2 module"]
    #[inline(always)]
    pub fn iom3ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc96::Iom3ce2)
    }
    #[doc = "IOM 3 NCE 3 module"]
    #[inline(always)]
    pub fn iom3ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc96::Iom3ce3)
    }
    #[doc = "IOM 4 NCE 0 module"]
    #[inline(always)]
    pub fn iom4ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc96::Iom4ce0)
    }
    #[doc = "IOM 4 NCE 1 module"]
    #[inline(always)]
    pub fn iom4ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc96::Iom4ce1)
    }
    #[doc = "IOM 4 NCE 2 module"]
    #[inline(always)]
    pub fn iom4ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc96::Iom4ce2)
    }
    #[doc = "IOM 4 NCE 3 module"]
    #[inline(always)]
    pub fn iom4ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc96::Iom4ce3)
    }
    #[doc = "IOM 5 NCE 0 module"]
    #[inline(always)]
    pub fn iom5ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc96::Iom5ce0)
    }
    #[doc = "IOM 5 NCE 1 module"]
    #[inline(always)]
    pub fn iom5ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc96::Iom5ce1)
    }
    #[doc = "IOM 5 NCE 2 module"]
    #[inline(always)]
    pub fn iom5ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc96::Iom5ce2)
    }
    #[doc = "IOM 5 NCE 3 module"]
    #[inline(always)]
    pub fn iom5ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc96::Iom5ce3)
    }
    #[doc = "IOM 6 NCE 0 module"]
    #[inline(always)]
    pub fn iom6ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc96::Iom6ce0)
    }
    #[doc = "IOM 6 NCE 1 module"]
    #[inline(always)]
    pub fn iom6ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc96::Iom6ce1)
    }
    #[doc = "IOM 6 NCE 2 module"]
    #[inline(always)]
    pub fn iom6ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc96::Iom6ce2)
    }
    #[doc = "IOM 6 NCE 3 module"]
    #[inline(always)]
    pub fn iom6ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc96::Iom6ce3)
    }
    #[doc = "IOM 7 NCE 0 module"]
    #[inline(always)]
    pub fn iom7ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc96::Iom7ce0)
    }
    #[doc = "IOM 7 NCE 1 module"]
    #[inline(always)]
    pub fn iom7ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc96::Iom7ce1)
    }
    #[doc = "IOM 7 NCE 2 module"]
    #[inline(always)]
    pub fn iom7ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc96::Iom7ce2)
    }
    #[doc = "IOM 7 NCE 3 module"]
    #[inline(always)]
    pub fn iom7ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc96::Iom7ce3)
    }
    #[doc = "MSPI 0 NCE 0 module"]
    #[inline(always)]
    pub fn mspi0cen0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc96::Mspi0cen0)
    }
    #[doc = "MSPI 0 NCE 1 module"]
    #[inline(always)]
    pub fn mspi0cen1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc96::Mspi0cen1)
    }
    #[doc = "MSPI 1 NCE 0 module"]
    #[inline(always)]
    pub fn mspi1cen0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc96::Mspi1cen0)
    }
    #[doc = "MSPI 1 NCE 1 module"]
    #[inline(always)]
    pub fn mspi1cen1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc96::Mspi1cen1)
    }
    #[doc = "MSPI 2 NCE 0 module"]
    #[inline(always)]
    pub fn mspi2cen0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc96::Mspi2cen0)
    }
    #[doc = "MSPI 2 NCE 1 module"]
    #[inline(always)]
    pub fn mspi2cen1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc96::Mspi2cen1)
    }
    #[doc = "DC DPI DE module"]
    #[inline(always)]
    pub fn dc_dpi_de(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc96::DcDpiDe)
    }
    #[doc = "DISP CONT CSX module"]
    #[inline(always)]
    pub fn disp_cont_csx(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc96::DispContCsx)
    }
    #[doc = "DC SPI CS_N module"]
    #[inline(always)]
    pub fn dc_spi_cs_n(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc96::DcSpiCsN)
    }
    #[doc = "DC QSPI CS_N module"]
    #[inline(always)]
    pub fn dc_qspi_cs_n(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc96::DcQspiCsN)
    }
    #[doc = "DC module RESX"]
    #[inline(always)]
    pub fn dc_resx(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc96::DcResx)
    }
}
#[doc = "Polarity select for NCE for GPIO 96\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ncepol96 {
    #[doc = "0: Polarity is active low"]
    Low = 0,
    #[doc = "1: Polarity is active high"]
    High = 1,
}
impl From<Ncepol96> for bool {
    #[inline(always)]
    fn from(variant: Ncepol96) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NCEPOL96` reader - Polarity select for NCE for GPIO 96"]
pub type Ncepol96R = crate::BitReader<Ncepol96>;
impl Ncepol96R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ncepol96 {
        match self.bits {
            false => Ncepol96::Low,
            true => Ncepol96::High,
        }
    }
    #[doc = "Polarity is active low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Ncepol96::Low
    }
    #[doc = "Polarity is active high"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Ncepol96::High
    }
}
#[doc = "Field `NCEPOL96` writer - Polarity select for NCE for GPIO 96"]
pub type Ncepol96W<'a, REG> = crate::BitWriter<'a, REG, Ncepol96>;
impl<'a, REG> Ncepol96W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Polarity is active low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Ncepol96::Low)
    }
    #[doc = "Polarity is active high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Ncepol96::High)
    }
}
#[doc = "Field `FIEN96` reader - Force input enable active regardless of function selected. Otherwise the selected function will enable the input only when needed"]
pub type Fien96R = crate::BitReader;
#[doc = "Field `FIEN96` writer - Force input enable active regardless of function selected. Otherwise the selected function will enable the input only when needed"]
pub type Fien96W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FOEN96` reader - Force output enable active regardless of function selected. Otherwise the selected function will enable the output only when needed"]
pub type Foen96R = crate::BitReader;
#[doc = "Field `FOEN96` writer - Force output enable active regardless of function selected. Otherwise the selected function will enable the output only when needed"]
pub type Foen96W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Function select for GPIO pin 96"]
    #[inline(always)]
    pub fn fncsel96(&self) -> Fncsel96R {
        Fncsel96R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Input enable for GPIO 96"]
    #[inline(always)]
    pub fn inpen96(&self) -> Inpen96R {
        Inpen96R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Return 0 for read data on GPIO 96"]
    #[inline(always)]
    pub fn rdzero96(&self) -> Rdzero96R {
        Rdzero96R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Interrupt enable for GPIO 96"]
    #[inline(always)]
    pub fn irpten96(&self) -> Irpten96R {
        Irpten96R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Pin IO mode selection for GPIO pin 96"]
    #[inline(always)]
    pub fn outcfg96(&self) -> Outcfg96R {
        Outcfg96R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Drive strength selection for GPIO 96"]
    #[inline(always)]
    pub fn ds96(&self) -> Ds96R {
        Ds96R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - Configure the slew rate"]
    #[inline(always)]
    pub fn sr96(&self) -> Sr96R {
        Sr96R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - Pullup/Pulldown configuration for GPIO 96"]
    #[inline(always)]
    pub fn pullcfg96(&self) -> Pullcfg96R {
        Pullcfg96R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:21 - IOMSTR/MSPI N Chip Select 96, DISP control signals DE, CSX, and CS. Polarity is determined by CE_POLARITY field"]
    #[inline(always)]
    pub fn ncesrc96(&self) -> Ncesrc96R {
        Ncesrc96R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 22 - Polarity select for NCE for GPIO 96"]
    #[inline(always)]
    pub fn ncepol96(&self) -> Ncepol96R {
        Ncepol96R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 26 - Force input enable active regardless of function selected. Otherwise the selected function will enable the input only when needed"]
    #[inline(always)]
    pub fn fien96(&self) -> Fien96R {
        Fien96R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Force output enable active regardless of function selected. Otherwise the selected function will enable the output only when needed"]
    #[inline(always)]
    pub fn foen96(&self) -> Foen96R {
        Foen96R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Function select for GPIO pin 96"]
    #[inline(always)]
    #[must_use]
    pub fn fncsel96(&mut self) -> Fncsel96W<Pincfg96Spec> {
        Fncsel96W::new(self, 0)
    }
    #[doc = "Bit 4 - Input enable for GPIO 96"]
    #[inline(always)]
    #[must_use]
    pub fn inpen96(&mut self) -> Inpen96W<Pincfg96Spec> {
        Inpen96W::new(self, 4)
    }
    #[doc = "Bit 5 - Return 0 for read data on GPIO 96"]
    #[inline(always)]
    #[must_use]
    pub fn rdzero96(&mut self) -> Rdzero96W<Pincfg96Spec> {
        Rdzero96W::new(self, 5)
    }
    #[doc = "Bits 6:7 - Interrupt enable for GPIO 96"]
    #[inline(always)]
    #[must_use]
    pub fn irpten96(&mut self) -> Irpten96W<Pincfg96Spec> {
        Irpten96W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Pin IO mode selection for GPIO pin 96"]
    #[inline(always)]
    #[must_use]
    pub fn outcfg96(&mut self) -> Outcfg96W<Pincfg96Spec> {
        Outcfg96W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Drive strength selection for GPIO 96"]
    #[inline(always)]
    #[must_use]
    pub fn ds96(&mut self) -> Ds96W<Pincfg96Spec> {
        Ds96W::new(self, 10)
    }
    #[doc = "Bit 12 - Configure the slew rate"]
    #[inline(always)]
    #[must_use]
    pub fn sr96(&mut self) -> Sr96W<Pincfg96Spec> {
        Sr96W::new(self, 12)
    }
    #[doc = "Bits 13:15 - Pullup/Pulldown configuration for GPIO 96"]
    #[inline(always)]
    #[must_use]
    pub fn pullcfg96(&mut self) -> Pullcfg96W<Pincfg96Spec> {
        Pullcfg96W::new(self, 13)
    }
    #[doc = "Bits 16:21 - IOMSTR/MSPI N Chip Select 96, DISP control signals DE, CSX, and CS. Polarity is determined by CE_POLARITY field"]
    #[inline(always)]
    #[must_use]
    pub fn ncesrc96(&mut self) -> Ncesrc96W<Pincfg96Spec> {
        Ncesrc96W::new(self, 16)
    }
    #[doc = "Bit 22 - Polarity select for NCE for GPIO 96"]
    #[inline(always)]
    #[must_use]
    pub fn ncepol96(&mut self) -> Ncepol96W<Pincfg96Spec> {
        Ncepol96W::new(self, 22)
    }
    #[doc = "Bit 26 - Force input enable active regardless of function selected. Otherwise the selected function will enable the input only when needed"]
    #[inline(always)]
    #[must_use]
    pub fn fien96(&mut self) -> Fien96W<Pincfg96Spec> {
        Fien96W::new(self, 26)
    }
    #[doc = "Bit 27 - Force output enable active regardless of function selected. Otherwise the selected function will enable the output only when needed"]
    #[inline(always)]
    #[must_use]
    pub fn foen96(&mut self) -> Foen96W<Pincfg96Spec> {
        Foen96W::new(self, 27)
    }
}
#[doc = "Controls the operation of GPIO pin 96.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg96::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg96::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pincfg96Spec;
impl crate::RegisterSpec for Pincfg96Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pincfg96::R`](R) reader structure"]
impl crate::Readable for Pincfg96Spec {}
#[doc = "`write(|w| ..)` method takes [`pincfg96::W`](W) writer structure"]
impl crate::Writable for Pincfg96Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PINCFG96 to value 0x03"]
impl crate::Resettable for Pincfg96Spec {
    const RESET_VALUE: u32 = 0x03;
}
