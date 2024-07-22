#[doc = "Register `PINCFG70` reader"]
pub type R = crate::R<Pincfg70Spec>;
#[doc = "Register `PINCFG70` writer"]
pub type W = crate::W<Pincfg70Spec>;
#[doc = "Function select for GPIO pin 70\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fncsel70 {
    #[doc = "0: MSPI Master 0 Interface Signal"]
    Mspi0_6 = 0,
    #[doc = "1: 32kHZ from analog"]
    _32khzXt = 1,
    #[doc = "2: Serial Wire Debug Trace Output 0"]
    Swtrace0 = 2,
    #[doc = "3: General purpose I/O"]
    Gpio = 3,
    #[doc = "4: UART Request to Send (RTS) (UART 0)"]
    Uart0rts = 4,
    #[doc = "5: Display Data 6"]
    DispD6 = 5,
    #[doc = "6: Timer/Counter input or output; Selection of direction is done via CTIMER register settings."]
    Ct70 = 6,
    #[doc = "7: IOMSTR/MSPI N Chip Select. Polarity is determined by CE_POLARITY field"]
    Nce70 = 7,
    #[doc = "8: Observation bus bit 6"]
    Obsbus6 = 8,
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
impl From<Fncsel70> for u8 {
    #[inline(always)]
    fn from(variant: Fncsel70) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fncsel70 {
    type Ux = u8;
}
impl crate::IsEnum for Fncsel70 {}
#[doc = "Field `FNCSEL70` reader - Function select for GPIO pin 70"]
pub type Fncsel70R = crate::FieldReader<Fncsel70>;
impl Fncsel70R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fncsel70 {
        match self.bits {
            0 => Fncsel70::Mspi0_6,
            1 => Fncsel70::_32khzXt,
            2 => Fncsel70::Swtrace0,
            3 => Fncsel70::Gpio,
            4 => Fncsel70::Uart0rts,
            5 => Fncsel70::DispD6,
            6 => Fncsel70::Ct70,
            7 => Fncsel70::Nce70,
            8 => Fncsel70::Obsbus6,
            9 => Fncsel70::Reserved9,
            10 => Fncsel70::Reserved10,
            11 => Fncsel70::Fpio,
            12 => Fncsel70::Reserved12,
            13 => Fncsel70::Reserved13,
            14 => Fncsel70::Reserved14,
            15 => Fncsel70::Reserved15,
            _ => unreachable!(),
        }
    }
    #[doc = "MSPI Master 0 Interface Signal"]
    #[inline(always)]
    pub fn is_mspi0_6(&self) -> bool {
        *self == Fncsel70::Mspi0_6
    }
    #[doc = "32kHZ from analog"]
    #[inline(always)]
    pub fn is_32khz_xt(&self) -> bool {
        *self == Fncsel70::_32khzXt
    }
    #[doc = "Serial Wire Debug Trace Output 0"]
    #[inline(always)]
    pub fn is_swtrace0(&self) -> bool {
        *self == Fncsel70::Swtrace0
    }
    #[doc = "General purpose I/O"]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == Fncsel70::Gpio
    }
    #[doc = "UART Request to Send (RTS) (UART 0)"]
    #[inline(always)]
    pub fn is_uart0rts(&self) -> bool {
        *self == Fncsel70::Uart0rts
    }
    #[doc = "Display Data 6"]
    #[inline(always)]
    pub fn is_disp_d6(&self) -> bool {
        *self == Fncsel70::DispD6
    }
    #[doc = "Timer/Counter input or output; Selection of direction is done via CTIMER register settings."]
    #[inline(always)]
    pub fn is_ct70(&self) -> bool {
        *self == Fncsel70::Ct70
    }
    #[doc = "IOMSTR/MSPI N Chip Select. Polarity is determined by CE_POLARITY field"]
    #[inline(always)]
    pub fn is_nce70(&self) -> bool {
        *self == Fncsel70::Nce70
    }
    #[doc = "Observation bus bit 6"]
    #[inline(always)]
    pub fn is_obsbus6(&self) -> bool {
        *self == Fncsel70::Obsbus6
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved9(&self) -> bool {
        *self == Fncsel70::Reserved9
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved10(&self) -> bool {
        *self == Fncsel70::Reserved10
    }
    #[doc = "Fast PIO"]
    #[inline(always)]
    pub fn is_fpio(&self) -> bool {
        *self == Fncsel70::Fpio
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved12(&self) -> bool {
        *self == Fncsel70::Reserved12
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved13(&self) -> bool {
        *self == Fncsel70::Reserved13
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved14(&self) -> bool {
        *self == Fncsel70::Reserved14
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved15(&self) -> bool {
        *self == Fncsel70::Reserved15
    }
}
#[doc = "Field `FNCSEL70` writer - Function select for GPIO pin 70"]
pub type Fncsel70W<'a, REG> = crate::FieldWriter<'a, REG, 4, Fncsel70, crate::Safe>;
impl<'a, REG> Fncsel70W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "MSPI Master 0 Interface Signal"]
    #[inline(always)]
    pub fn mspi0_6(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel70::Mspi0_6)
    }
    #[doc = "32kHZ from analog"]
    #[inline(always)]
    pub fn _32khz_xt(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel70::_32khzXt)
    }
    #[doc = "Serial Wire Debug Trace Output 0"]
    #[inline(always)]
    pub fn swtrace0(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel70::Swtrace0)
    }
    #[doc = "General purpose I/O"]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel70::Gpio)
    }
    #[doc = "UART Request to Send (RTS) (UART 0)"]
    #[inline(always)]
    pub fn uart0rts(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel70::Uart0rts)
    }
    #[doc = "Display Data 6"]
    #[inline(always)]
    pub fn disp_d6(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel70::DispD6)
    }
    #[doc = "Timer/Counter input or output; Selection of direction is done via CTIMER register settings."]
    #[inline(always)]
    pub fn ct70(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel70::Ct70)
    }
    #[doc = "IOMSTR/MSPI N Chip Select. Polarity is determined by CE_POLARITY field"]
    #[inline(always)]
    pub fn nce70(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel70::Nce70)
    }
    #[doc = "Observation bus bit 6"]
    #[inline(always)]
    pub fn obsbus6(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel70::Obsbus6)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved9(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel70::Reserved9)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved10(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel70::Reserved10)
    }
    #[doc = "Fast PIO"]
    #[inline(always)]
    pub fn fpio(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel70::Fpio)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved12(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel70::Reserved12)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved13(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel70::Reserved13)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved14(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel70::Reserved14)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved15(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel70::Reserved15)
    }
}
#[doc = "Field `INPEN70` reader - Input enable for GPIO 70"]
pub type Inpen70R = crate::BitReader;
#[doc = "Field `INPEN70` writer - Input enable for GPIO 70"]
pub type Inpen70W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDZERO70` reader - Return 0 for read data on GPIO 70"]
pub type Rdzero70R = crate::BitReader;
#[doc = "Field `RDZERO70` writer - Return 0 for read data on GPIO 70"]
pub type Rdzero70W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Interrupt enable for GPIO 70\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Irpten70 {
    #[doc = "0: Interrupts are disabled for this GPIO"]
    Dis = 0,
    #[doc = "1: Interrupts are enabled for falling edge transition on this GPIO"]
    Intfall = 1,
    #[doc = "2: Interrupts are enabled for rising edge transitions on this GPIO"]
    Intrise = 2,
    #[doc = "3: Interrupts are enabled for any edge transition on this GPIO"]
    Intany = 3,
}
impl From<Irpten70> for u8 {
    #[inline(always)]
    fn from(variant: Irpten70) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Irpten70 {
    type Ux = u8;
}
impl crate::IsEnum for Irpten70 {}
#[doc = "Field `IRPTEN70` reader - Interrupt enable for GPIO 70"]
pub type Irpten70R = crate::FieldReader<Irpten70>;
impl Irpten70R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irpten70 {
        match self.bits {
            0 => Irpten70::Dis,
            1 => Irpten70::Intfall,
            2 => Irpten70::Intrise,
            3 => Irpten70::Intany,
            _ => unreachable!(),
        }
    }
    #[doc = "Interrupts are disabled for this GPIO"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Irpten70::Dis
    }
    #[doc = "Interrupts are enabled for falling edge transition on this GPIO"]
    #[inline(always)]
    pub fn is_intfall(&self) -> bool {
        *self == Irpten70::Intfall
    }
    #[doc = "Interrupts are enabled for rising edge transitions on this GPIO"]
    #[inline(always)]
    pub fn is_intrise(&self) -> bool {
        *self == Irpten70::Intrise
    }
    #[doc = "Interrupts are enabled for any edge transition on this GPIO"]
    #[inline(always)]
    pub fn is_intany(&self) -> bool {
        *self == Irpten70::Intany
    }
}
#[doc = "Field `IRPTEN70` writer - Interrupt enable for GPIO 70"]
pub type Irpten70W<'a, REG> = crate::FieldWriter<'a, REG, 2, Irpten70, crate::Safe>;
impl<'a, REG> Irpten70W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Interrupts are disabled for this GPIO"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten70::Dis)
    }
    #[doc = "Interrupts are enabled for falling edge transition on this GPIO"]
    #[inline(always)]
    pub fn intfall(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten70::Intfall)
    }
    #[doc = "Interrupts are enabled for rising edge transitions on this GPIO"]
    #[inline(always)]
    pub fn intrise(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten70::Intrise)
    }
    #[doc = "Interrupts are enabled for any edge transition on this GPIO"]
    #[inline(always)]
    pub fn intany(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten70::Intany)
    }
}
#[doc = "Pin IO mode selection for GPIO pin 70\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Outcfg70 {
    #[doc = "0: Output Disabled"]
    Dis = 0,
    #[doc = "1: Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    Pushpull = 1,
    #[doc = "2: Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    Od = 2,
    #[doc = "3: Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    Ts = 3,
}
impl From<Outcfg70> for u8 {
    #[inline(always)]
    fn from(variant: Outcfg70) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Outcfg70 {
    type Ux = u8;
}
impl crate::IsEnum for Outcfg70 {}
#[doc = "Field `OUTCFG70` reader - Pin IO mode selection for GPIO pin 70"]
pub type Outcfg70R = crate::FieldReader<Outcfg70>;
impl Outcfg70R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Outcfg70 {
        match self.bits {
            0 => Outcfg70::Dis,
            1 => Outcfg70::Pushpull,
            2 => Outcfg70::Od,
            3 => Outcfg70::Ts,
            _ => unreachable!(),
        }
    }
    #[doc = "Output Disabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Outcfg70::Dis
    }
    #[doc = "Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == Outcfg70::Pushpull
    }
    #[doc = "Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == Outcfg70::Od
    }
    #[doc = "Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == Outcfg70::Ts
    }
}
#[doc = "Field `OUTCFG70` writer - Pin IO mode selection for GPIO pin 70"]
pub type Outcfg70W<'a, REG> = crate::FieldWriter<'a, REG, 2, Outcfg70, crate::Safe>;
impl<'a, REG> Outcfg70W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Output Disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg70::Dis)
    }
    #[doc = "Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg70::Pushpull)
    }
    #[doc = "Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    #[inline(always)]
    pub fn od(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg70::Od)
    }
    #[doc = "Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    #[inline(always)]
    pub fn ts(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg70::Ts)
    }
}
#[doc = "Drive strength selection for GPIO 70\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ds70 {
    #[doc = "0: 0.1x output driver selected"]
    _0p1x = 0,
    #[doc = "1: 0.5x output driver selected"]
    _0p5x = 1,
    #[doc = "2: 0.75x output driver selected"]
    _0p75x = 2,
    #[doc = "3: 1.0x output driver selected"]
    _1p0x = 3,
}
impl From<Ds70> for u8 {
    #[inline(always)]
    fn from(variant: Ds70) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ds70 {
    type Ux = u8;
}
impl crate::IsEnum for Ds70 {}
#[doc = "Field `DS70` reader - Drive strength selection for GPIO 70"]
pub type Ds70R = crate::FieldReader<Ds70>;
impl Ds70R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ds70 {
        match self.bits {
            0 => Ds70::_0p1x,
            1 => Ds70::_0p5x,
            2 => Ds70::_0p75x,
            3 => Ds70::_1p0x,
            _ => unreachable!(),
        }
    }
    #[doc = "0.1x output driver selected"]
    #[inline(always)]
    pub fn is_0p1x(&self) -> bool {
        *self == Ds70::_0p1x
    }
    #[doc = "0.5x output driver selected"]
    #[inline(always)]
    pub fn is_0p5x(&self) -> bool {
        *self == Ds70::_0p5x
    }
    #[doc = "0.75x output driver selected"]
    #[inline(always)]
    pub fn is_0p75x(&self) -> bool {
        *self == Ds70::_0p75x
    }
    #[doc = "1.0x output driver selected"]
    #[inline(always)]
    pub fn is_1p0x(&self) -> bool {
        *self == Ds70::_1p0x
    }
}
#[doc = "Field `DS70` writer - Drive strength selection for GPIO 70"]
pub type Ds70W<'a, REG> = crate::FieldWriter<'a, REG, 2, Ds70, crate::Safe>;
impl<'a, REG> Ds70W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "0.1x output driver selected"]
    #[inline(always)]
    pub fn _0p1x(self) -> &'a mut crate::W<REG> {
        self.variant(Ds70::_0p1x)
    }
    #[doc = "0.5x output driver selected"]
    #[inline(always)]
    pub fn _0p5x(self) -> &'a mut crate::W<REG> {
        self.variant(Ds70::_0p5x)
    }
    #[doc = "0.75x output driver selected"]
    #[inline(always)]
    pub fn _0p75x(self) -> &'a mut crate::W<REG> {
        self.variant(Ds70::_0p75x)
    }
    #[doc = "1.0x output driver selected"]
    #[inline(always)]
    pub fn _1p0x(self) -> &'a mut crate::W<REG> {
        self.variant(Ds70::_1p0x)
    }
}
#[doc = "Field `SR70` reader - Configure the slew rate"]
pub type Sr70R = crate::BitReader;
#[doc = "Field `SR70` writer - Configure the slew rate"]
pub type Sr70W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Pullup/Pulldown configuration for GPIO 70\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pullcfg70 {
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
impl From<Pullcfg70> for u8 {
    #[inline(always)]
    fn from(variant: Pullcfg70) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pullcfg70 {
    type Ux = u8;
}
impl crate::IsEnum for Pullcfg70 {}
#[doc = "Field `PULLCFG70` reader - Pullup/Pulldown configuration for GPIO 70"]
pub type Pullcfg70R = crate::FieldReader<Pullcfg70>;
impl Pullcfg70R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pullcfg70 {
        match self.bits {
            0 => Pullcfg70::Dis,
            1 => Pullcfg70::Pd50k,
            2 => Pullcfg70::Pu15k,
            3 => Pullcfg70::Pu6k,
            4 => Pullcfg70::Pu12k,
            5 => Pullcfg70::Pu24k,
            6 => Pullcfg70::Pu50k,
            7 => Pullcfg70::Pu100k,
            _ => unreachable!(),
        }
    }
    #[doc = "No pullup or pulldown selected"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Pullcfg70::Dis
    }
    #[doc = "50K Pulldown selected"]
    #[inline(always)]
    pub fn is_pd50k(&self) -> bool {
        *self == Pullcfg70::Pd50k
    }
    #[doc = "1.5K Pullup selected"]
    #[inline(always)]
    pub fn is_pu15k(&self) -> bool {
        *self == Pullcfg70::Pu15k
    }
    #[doc = "6K Pullup selected"]
    #[inline(always)]
    pub fn is_pu6k(&self) -> bool {
        *self == Pullcfg70::Pu6k
    }
    #[doc = "12K Pullup selected"]
    #[inline(always)]
    pub fn is_pu12k(&self) -> bool {
        *self == Pullcfg70::Pu12k
    }
    #[doc = "24K Pullup selected"]
    #[inline(always)]
    pub fn is_pu24k(&self) -> bool {
        *self == Pullcfg70::Pu24k
    }
    #[doc = "50K Pullup selected"]
    #[inline(always)]
    pub fn is_pu50k(&self) -> bool {
        *self == Pullcfg70::Pu50k
    }
    #[doc = "100K Pullup selected"]
    #[inline(always)]
    pub fn is_pu100k(&self) -> bool {
        *self == Pullcfg70::Pu100k
    }
}
#[doc = "Field `PULLCFG70` writer - Pullup/Pulldown configuration for GPIO 70"]
pub type Pullcfg70W<'a, REG> = crate::FieldWriter<'a, REG, 3, Pullcfg70, crate::Safe>;
impl<'a, REG> Pullcfg70W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No pullup or pulldown selected"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg70::Dis)
    }
    #[doc = "50K Pulldown selected"]
    #[inline(always)]
    pub fn pd50k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg70::Pd50k)
    }
    #[doc = "1.5K Pullup selected"]
    #[inline(always)]
    pub fn pu15k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg70::Pu15k)
    }
    #[doc = "6K Pullup selected"]
    #[inline(always)]
    pub fn pu6k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg70::Pu6k)
    }
    #[doc = "12K Pullup selected"]
    #[inline(always)]
    pub fn pu12k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg70::Pu12k)
    }
    #[doc = "24K Pullup selected"]
    #[inline(always)]
    pub fn pu24k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg70::Pu24k)
    }
    #[doc = "50K Pullup selected"]
    #[inline(always)]
    pub fn pu50k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg70::Pu50k)
    }
    #[doc = "100K Pullup selected"]
    #[inline(always)]
    pub fn pu100k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg70::Pu100k)
    }
}
#[doc = "IOMSTR/MSPI N Chip Select 70, DISP control signals DE, CSX, and CS. Polarity is determined by CE_POLARITY field\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ncesrc70 {
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
impl From<Ncesrc70> for u8 {
    #[inline(always)]
    fn from(variant: Ncesrc70) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ncesrc70 {
    type Ux = u8;
}
impl crate::IsEnum for Ncesrc70 {}
#[doc = "Field `NCESRC70` reader - IOMSTR/MSPI N Chip Select 70, DISP control signals DE, CSX, and CS. Polarity is determined by CE_POLARITY field"]
pub type Ncesrc70R = crate::FieldReader<Ncesrc70>;
impl Ncesrc70R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ncesrc70> {
        match self.bits {
            0 => Some(Ncesrc70::Iom0ce0),
            1 => Some(Ncesrc70::Iom0ce1),
            2 => Some(Ncesrc70::Iom0ce2),
            3 => Some(Ncesrc70::Iom0ce3),
            4 => Some(Ncesrc70::Iom1ce0),
            5 => Some(Ncesrc70::Iom1ce1),
            6 => Some(Ncesrc70::Iom1ce2),
            7 => Some(Ncesrc70::Iom1ce3),
            8 => Some(Ncesrc70::Iom2ce0),
            9 => Some(Ncesrc70::Iom2ce1),
            10 => Some(Ncesrc70::Iom2ce2),
            11 => Some(Ncesrc70::Iom2ce3),
            12 => Some(Ncesrc70::Iom3ce0),
            13 => Some(Ncesrc70::Iom3ce1),
            14 => Some(Ncesrc70::Iom3ce2),
            15 => Some(Ncesrc70::Iom3ce3),
            16 => Some(Ncesrc70::Iom4ce0),
            17 => Some(Ncesrc70::Iom4ce1),
            18 => Some(Ncesrc70::Iom4ce2),
            19 => Some(Ncesrc70::Iom4ce3),
            20 => Some(Ncesrc70::Iom5ce0),
            21 => Some(Ncesrc70::Iom5ce1),
            22 => Some(Ncesrc70::Iom5ce2),
            23 => Some(Ncesrc70::Iom5ce3),
            24 => Some(Ncesrc70::Iom6ce0),
            25 => Some(Ncesrc70::Iom6ce1),
            26 => Some(Ncesrc70::Iom6ce2),
            27 => Some(Ncesrc70::Iom6ce3),
            28 => Some(Ncesrc70::Iom7ce0),
            29 => Some(Ncesrc70::Iom7ce1),
            30 => Some(Ncesrc70::Iom7ce2),
            31 => Some(Ncesrc70::Iom7ce3),
            32 => Some(Ncesrc70::Mspi0cen0),
            33 => Some(Ncesrc70::Mspi0cen1),
            34 => Some(Ncesrc70::Mspi1cen0),
            35 => Some(Ncesrc70::Mspi1cen1),
            36 => Some(Ncesrc70::Mspi2cen0),
            37 => Some(Ncesrc70::Mspi2cen1),
            38 => Some(Ncesrc70::DcDpiDe),
            39 => Some(Ncesrc70::DispContCsx),
            40 => Some(Ncesrc70::DcSpiCsN),
            41 => Some(Ncesrc70::DcQspiCsN),
            42 => Some(Ncesrc70::DcResx),
            _ => None,
        }
    }
    #[doc = "IOM 0 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom0ce0(&self) -> bool {
        *self == Ncesrc70::Iom0ce0
    }
    #[doc = "IOM 0 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom0ce1(&self) -> bool {
        *self == Ncesrc70::Iom0ce1
    }
    #[doc = "IOM 0 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom0ce2(&self) -> bool {
        *self == Ncesrc70::Iom0ce2
    }
    #[doc = "IOM 0 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom0ce3(&self) -> bool {
        *self == Ncesrc70::Iom0ce3
    }
    #[doc = "IOM 1 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom1ce0(&self) -> bool {
        *self == Ncesrc70::Iom1ce0
    }
    #[doc = "IOM 1 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom1ce1(&self) -> bool {
        *self == Ncesrc70::Iom1ce1
    }
    #[doc = "IOM 1 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom1ce2(&self) -> bool {
        *self == Ncesrc70::Iom1ce2
    }
    #[doc = "IOM 1 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom1ce3(&self) -> bool {
        *self == Ncesrc70::Iom1ce3
    }
    #[doc = "IOM 2 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom2ce0(&self) -> bool {
        *self == Ncesrc70::Iom2ce0
    }
    #[doc = "IOM 2 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom2ce1(&self) -> bool {
        *self == Ncesrc70::Iom2ce1
    }
    #[doc = "IOM 2 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom2ce2(&self) -> bool {
        *self == Ncesrc70::Iom2ce2
    }
    #[doc = "IOM 2 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom2ce3(&self) -> bool {
        *self == Ncesrc70::Iom2ce3
    }
    #[doc = "IOM 3 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom3ce0(&self) -> bool {
        *self == Ncesrc70::Iom3ce0
    }
    #[doc = "IOM 3 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom3ce1(&self) -> bool {
        *self == Ncesrc70::Iom3ce1
    }
    #[doc = "IOM 3 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom3ce2(&self) -> bool {
        *self == Ncesrc70::Iom3ce2
    }
    #[doc = "IOM 3 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom3ce3(&self) -> bool {
        *self == Ncesrc70::Iom3ce3
    }
    #[doc = "IOM 4 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom4ce0(&self) -> bool {
        *self == Ncesrc70::Iom4ce0
    }
    #[doc = "IOM 4 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom4ce1(&self) -> bool {
        *self == Ncesrc70::Iom4ce1
    }
    #[doc = "IOM 4 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom4ce2(&self) -> bool {
        *self == Ncesrc70::Iom4ce2
    }
    #[doc = "IOM 4 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom4ce3(&self) -> bool {
        *self == Ncesrc70::Iom4ce3
    }
    #[doc = "IOM 5 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom5ce0(&self) -> bool {
        *self == Ncesrc70::Iom5ce0
    }
    #[doc = "IOM 5 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom5ce1(&self) -> bool {
        *self == Ncesrc70::Iom5ce1
    }
    #[doc = "IOM 5 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom5ce2(&self) -> bool {
        *self == Ncesrc70::Iom5ce2
    }
    #[doc = "IOM 5 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom5ce3(&self) -> bool {
        *self == Ncesrc70::Iom5ce3
    }
    #[doc = "IOM 6 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom6ce0(&self) -> bool {
        *self == Ncesrc70::Iom6ce0
    }
    #[doc = "IOM 6 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom6ce1(&self) -> bool {
        *self == Ncesrc70::Iom6ce1
    }
    #[doc = "IOM 6 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom6ce2(&self) -> bool {
        *self == Ncesrc70::Iom6ce2
    }
    #[doc = "IOM 6 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom6ce3(&self) -> bool {
        *self == Ncesrc70::Iom6ce3
    }
    #[doc = "IOM 7 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom7ce0(&self) -> bool {
        *self == Ncesrc70::Iom7ce0
    }
    #[doc = "IOM 7 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom7ce1(&self) -> bool {
        *self == Ncesrc70::Iom7ce1
    }
    #[doc = "IOM 7 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom7ce2(&self) -> bool {
        *self == Ncesrc70::Iom7ce2
    }
    #[doc = "IOM 7 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom7ce3(&self) -> bool {
        *self == Ncesrc70::Iom7ce3
    }
    #[doc = "MSPI 0 NCE 0 module"]
    #[inline(always)]
    pub fn is_mspi0cen0(&self) -> bool {
        *self == Ncesrc70::Mspi0cen0
    }
    #[doc = "MSPI 0 NCE 1 module"]
    #[inline(always)]
    pub fn is_mspi0cen1(&self) -> bool {
        *self == Ncesrc70::Mspi0cen1
    }
    #[doc = "MSPI 1 NCE 0 module"]
    #[inline(always)]
    pub fn is_mspi1cen0(&self) -> bool {
        *self == Ncesrc70::Mspi1cen0
    }
    #[doc = "MSPI 1 NCE 1 module"]
    #[inline(always)]
    pub fn is_mspi1cen1(&self) -> bool {
        *self == Ncesrc70::Mspi1cen1
    }
    #[doc = "MSPI 2 NCE 0 module"]
    #[inline(always)]
    pub fn is_mspi2cen0(&self) -> bool {
        *self == Ncesrc70::Mspi2cen0
    }
    #[doc = "MSPI 2 NCE 1 module"]
    #[inline(always)]
    pub fn is_mspi2cen1(&self) -> bool {
        *self == Ncesrc70::Mspi2cen1
    }
    #[doc = "DC DPI DE module"]
    #[inline(always)]
    pub fn is_dc_dpi_de(&self) -> bool {
        *self == Ncesrc70::DcDpiDe
    }
    #[doc = "DISP CONT CSX module"]
    #[inline(always)]
    pub fn is_disp_cont_csx(&self) -> bool {
        *self == Ncesrc70::DispContCsx
    }
    #[doc = "DC SPI CS_N module"]
    #[inline(always)]
    pub fn is_dc_spi_cs_n(&self) -> bool {
        *self == Ncesrc70::DcSpiCsN
    }
    #[doc = "DC QSPI CS_N module"]
    #[inline(always)]
    pub fn is_dc_qspi_cs_n(&self) -> bool {
        *self == Ncesrc70::DcQspiCsN
    }
    #[doc = "DC module RESX"]
    #[inline(always)]
    pub fn is_dc_resx(&self) -> bool {
        *self == Ncesrc70::DcResx
    }
}
#[doc = "Field `NCESRC70` writer - IOMSTR/MSPI N Chip Select 70, DISP control signals DE, CSX, and CS. Polarity is determined by CE_POLARITY field"]
pub type Ncesrc70W<'a, REG> = crate::FieldWriter<'a, REG, 6, Ncesrc70>;
impl<'a, REG> Ncesrc70W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "IOM 0 NCE 0 module"]
    #[inline(always)]
    pub fn iom0ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc70::Iom0ce0)
    }
    #[doc = "IOM 0 NCE 1 module"]
    #[inline(always)]
    pub fn iom0ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc70::Iom0ce1)
    }
    #[doc = "IOM 0 NCE 2 module"]
    #[inline(always)]
    pub fn iom0ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc70::Iom0ce2)
    }
    #[doc = "IOM 0 NCE 3 module"]
    #[inline(always)]
    pub fn iom0ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc70::Iom0ce3)
    }
    #[doc = "IOM 1 NCE 0 module"]
    #[inline(always)]
    pub fn iom1ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc70::Iom1ce0)
    }
    #[doc = "IOM 1 NCE 1 module"]
    #[inline(always)]
    pub fn iom1ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc70::Iom1ce1)
    }
    #[doc = "IOM 1 NCE 2 module"]
    #[inline(always)]
    pub fn iom1ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc70::Iom1ce2)
    }
    #[doc = "IOM 1 NCE 3 module"]
    #[inline(always)]
    pub fn iom1ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc70::Iom1ce3)
    }
    #[doc = "IOM 2 NCE 0 module"]
    #[inline(always)]
    pub fn iom2ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc70::Iom2ce0)
    }
    #[doc = "IOM 2 NCE 1 module"]
    #[inline(always)]
    pub fn iom2ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc70::Iom2ce1)
    }
    #[doc = "IOM 2 NCE 2 module"]
    #[inline(always)]
    pub fn iom2ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc70::Iom2ce2)
    }
    #[doc = "IOM 2 NCE 3 module"]
    #[inline(always)]
    pub fn iom2ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc70::Iom2ce3)
    }
    #[doc = "IOM 3 NCE 0 module"]
    #[inline(always)]
    pub fn iom3ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc70::Iom3ce0)
    }
    #[doc = "IOM 3 NCE 1 module"]
    #[inline(always)]
    pub fn iom3ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc70::Iom3ce1)
    }
    #[doc = "IOM 3 NCE 2 module"]
    #[inline(always)]
    pub fn iom3ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc70::Iom3ce2)
    }
    #[doc = "IOM 3 NCE 3 module"]
    #[inline(always)]
    pub fn iom3ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc70::Iom3ce3)
    }
    #[doc = "IOM 4 NCE 0 module"]
    #[inline(always)]
    pub fn iom4ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc70::Iom4ce0)
    }
    #[doc = "IOM 4 NCE 1 module"]
    #[inline(always)]
    pub fn iom4ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc70::Iom4ce1)
    }
    #[doc = "IOM 4 NCE 2 module"]
    #[inline(always)]
    pub fn iom4ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc70::Iom4ce2)
    }
    #[doc = "IOM 4 NCE 3 module"]
    #[inline(always)]
    pub fn iom4ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc70::Iom4ce3)
    }
    #[doc = "IOM 5 NCE 0 module"]
    #[inline(always)]
    pub fn iom5ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc70::Iom5ce0)
    }
    #[doc = "IOM 5 NCE 1 module"]
    #[inline(always)]
    pub fn iom5ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc70::Iom5ce1)
    }
    #[doc = "IOM 5 NCE 2 module"]
    #[inline(always)]
    pub fn iom5ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc70::Iom5ce2)
    }
    #[doc = "IOM 5 NCE 3 module"]
    #[inline(always)]
    pub fn iom5ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc70::Iom5ce3)
    }
    #[doc = "IOM 6 NCE 0 module"]
    #[inline(always)]
    pub fn iom6ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc70::Iom6ce0)
    }
    #[doc = "IOM 6 NCE 1 module"]
    #[inline(always)]
    pub fn iom6ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc70::Iom6ce1)
    }
    #[doc = "IOM 6 NCE 2 module"]
    #[inline(always)]
    pub fn iom6ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc70::Iom6ce2)
    }
    #[doc = "IOM 6 NCE 3 module"]
    #[inline(always)]
    pub fn iom6ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc70::Iom6ce3)
    }
    #[doc = "IOM 7 NCE 0 module"]
    #[inline(always)]
    pub fn iom7ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc70::Iom7ce0)
    }
    #[doc = "IOM 7 NCE 1 module"]
    #[inline(always)]
    pub fn iom7ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc70::Iom7ce1)
    }
    #[doc = "IOM 7 NCE 2 module"]
    #[inline(always)]
    pub fn iom7ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc70::Iom7ce2)
    }
    #[doc = "IOM 7 NCE 3 module"]
    #[inline(always)]
    pub fn iom7ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc70::Iom7ce3)
    }
    #[doc = "MSPI 0 NCE 0 module"]
    #[inline(always)]
    pub fn mspi0cen0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc70::Mspi0cen0)
    }
    #[doc = "MSPI 0 NCE 1 module"]
    #[inline(always)]
    pub fn mspi0cen1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc70::Mspi0cen1)
    }
    #[doc = "MSPI 1 NCE 0 module"]
    #[inline(always)]
    pub fn mspi1cen0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc70::Mspi1cen0)
    }
    #[doc = "MSPI 1 NCE 1 module"]
    #[inline(always)]
    pub fn mspi1cen1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc70::Mspi1cen1)
    }
    #[doc = "MSPI 2 NCE 0 module"]
    #[inline(always)]
    pub fn mspi2cen0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc70::Mspi2cen0)
    }
    #[doc = "MSPI 2 NCE 1 module"]
    #[inline(always)]
    pub fn mspi2cen1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc70::Mspi2cen1)
    }
    #[doc = "DC DPI DE module"]
    #[inline(always)]
    pub fn dc_dpi_de(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc70::DcDpiDe)
    }
    #[doc = "DISP CONT CSX module"]
    #[inline(always)]
    pub fn disp_cont_csx(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc70::DispContCsx)
    }
    #[doc = "DC SPI CS_N module"]
    #[inline(always)]
    pub fn dc_spi_cs_n(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc70::DcSpiCsN)
    }
    #[doc = "DC QSPI CS_N module"]
    #[inline(always)]
    pub fn dc_qspi_cs_n(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc70::DcQspiCsN)
    }
    #[doc = "DC module RESX"]
    #[inline(always)]
    pub fn dc_resx(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc70::DcResx)
    }
}
#[doc = "Polarity select for NCE for GPIO 70\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ncepol70 {
    #[doc = "0: Polarity is active low"]
    Low = 0,
    #[doc = "1: Polarity is active high"]
    High = 1,
}
impl From<Ncepol70> for bool {
    #[inline(always)]
    fn from(variant: Ncepol70) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NCEPOL70` reader - Polarity select for NCE for GPIO 70"]
pub type Ncepol70R = crate::BitReader<Ncepol70>;
impl Ncepol70R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ncepol70 {
        match self.bits {
            false => Ncepol70::Low,
            true => Ncepol70::High,
        }
    }
    #[doc = "Polarity is active low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Ncepol70::Low
    }
    #[doc = "Polarity is active high"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Ncepol70::High
    }
}
#[doc = "Field `NCEPOL70` writer - Polarity select for NCE for GPIO 70"]
pub type Ncepol70W<'a, REG> = crate::BitWriter<'a, REG, Ncepol70>;
impl<'a, REG> Ncepol70W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Polarity is active low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Ncepol70::Low)
    }
    #[doc = "Polarity is active high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Ncepol70::High)
    }
}
#[doc = "Field `FIEN70` reader - Force input enable active regardless of function selected. Otherwise the selected function will enable the input only when needed"]
pub type Fien70R = crate::BitReader;
#[doc = "Field `FIEN70` writer - Force input enable active regardless of function selected. Otherwise the selected function will enable the input only when needed"]
pub type Fien70W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FOEN70` reader - Force output enable active regardless of function selected. Otherwise the selected function will enable the output only when needed"]
pub type Foen70R = crate::BitReader;
#[doc = "Field `FOEN70` writer - Force output enable active regardless of function selected. Otherwise the selected function will enable the output only when needed"]
pub type Foen70W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Function select for GPIO pin 70"]
    #[inline(always)]
    pub fn fncsel70(&self) -> Fncsel70R {
        Fncsel70R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Input enable for GPIO 70"]
    #[inline(always)]
    pub fn inpen70(&self) -> Inpen70R {
        Inpen70R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Return 0 for read data on GPIO 70"]
    #[inline(always)]
    pub fn rdzero70(&self) -> Rdzero70R {
        Rdzero70R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Interrupt enable for GPIO 70"]
    #[inline(always)]
    pub fn irpten70(&self) -> Irpten70R {
        Irpten70R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Pin IO mode selection for GPIO pin 70"]
    #[inline(always)]
    pub fn outcfg70(&self) -> Outcfg70R {
        Outcfg70R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Drive strength selection for GPIO 70"]
    #[inline(always)]
    pub fn ds70(&self) -> Ds70R {
        Ds70R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - Configure the slew rate"]
    #[inline(always)]
    pub fn sr70(&self) -> Sr70R {
        Sr70R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - Pullup/Pulldown configuration for GPIO 70"]
    #[inline(always)]
    pub fn pullcfg70(&self) -> Pullcfg70R {
        Pullcfg70R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:21 - IOMSTR/MSPI N Chip Select 70, DISP control signals DE, CSX, and CS. Polarity is determined by CE_POLARITY field"]
    #[inline(always)]
    pub fn ncesrc70(&self) -> Ncesrc70R {
        Ncesrc70R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 22 - Polarity select for NCE for GPIO 70"]
    #[inline(always)]
    pub fn ncepol70(&self) -> Ncepol70R {
        Ncepol70R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 26 - Force input enable active regardless of function selected. Otherwise the selected function will enable the input only when needed"]
    #[inline(always)]
    pub fn fien70(&self) -> Fien70R {
        Fien70R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Force output enable active regardless of function selected. Otherwise the selected function will enable the output only when needed"]
    #[inline(always)]
    pub fn foen70(&self) -> Foen70R {
        Foen70R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Function select for GPIO pin 70"]
    #[inline(always)]
    #[must_use]
    pub fn fncsel70(&mut self) -> Fncsel70W<Pincfg70Spec> {
        Fncsel70W::new(self, 0)
    }
    #[doc = "Bit 4 - Input enable for GPIO 70"]
    #[inline(always)]
    #[must_use]
    pub fn inpen70(&mut self) -> Inpen70W<Pincfg70Spec> {
        Inpen70W::new(self, 4)
    }
    #[doc = "Bit 5 - Return 0 for read data on GPIO 70"]
    #[inline(always)]
    #[must_use]
    pub fn rdzero70(&mut self) -> Rdzero70W<Pincfg70Spec> {
        Rdzero70W::new(self, 5)
    }
    #[doc = "Bits 6:7 - Interrupt enable for GPIO 70"]
    #[inline(always)]
    #[must_use]
    pub fn irpten70(&mut self) -> Irpten70W<Pincfg70Spec> {
        Irpten70W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Pin IO mode selection for GPIO pin 70"]
    #[inline(always)]
    #[must_use]
    pub fn outcfg70(&mut self) -> Outcfg70W<Pincfg70Spec> {
        Outcfg70W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Drive strength selection for GPIO 70"]
    #[inline(always)]
    #[must_use]
    pub fn ds70(&mut self) -> Ds70W<Pincfg70Spec> {
        Ds70W::new(self, 10)
    }
    #[doc = "Bit 12 - Configure the slew rate"]
    #[inline(always)]
    #[must_use]
    pub fn sr70(&mut self) -> Sr70W<Pincfg70Spec> {
        Sr70W::new(self, 12)
    }
    #[doc = "Bits 13:15 - Pullup/Pulldown configuration for GPIO 70"]
    #[inline(always)]
    #[must_use]
    pub fn pullcfg70(&mut self) -> Pullcfg70W<Pincfg70Spec> {
        Pullcfg70W::new(self, 13)
    }
    #[doc = "Bits 16:21 - IOMSTR/MSPI N Chip Select 70, DISP control signals DE, CSX, and CS. Polarity is determined by CE_POLARITY field"]
    #[inline(always)]
    #[must_use]
    pub fn ncesrc70(&mut self) -> Ncesrc70W<Pincfg70Spec> {
        Ncesrc70W::new(self, 16)
    }
    #[doc = "Bit 22 - Polarity select for NCE for GPIO 70"]
    #[inline(always)]
    #[must_use]
    pub fn ncepol70(&mut self) -> Ncepol70W<Pincfg70Spec> {
        Ncepol70W::new(self, 22)
    }
    #[doc = "Bit 26 - Force input enable active regardless of function selected. Otherwise the selected function will enable the input only when needed"]
    #[inline(always)]
    #[must_use]
    pub fn fien70(&mut self) -> Fien70W<Pincfg70Spec> {
        Fien70W::new(self, 26)
    }
    #[doc = "Bit 27 - Force output enable active regardless of function selected. Otherwise the selected function will enable the output only when needed"]
    #[inline(always)]
    #[must_use]
    pub fn foen70(&mut self) -> Foen70W<Pincfg70Spec> {
        Foen70W::new(self, 27)
    }
}
#[doc = "Controls the operation of GPIO pin 70.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg70::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg70::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pincfg70Spec;
impl crate::RegisterSpec for Pincfg70Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pincfg70::R`](R) reader structure"]
impl crate::Readable for Pincfg70Spec {}
#[doc = "`write(|w| ..)` method takes [`pincfg70::W`](W) writer structure"]
impl crate::Writable for Pincfg70Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PINCFG70 to value 0x03"]
impl crate::Resettable for Pincfg70Spec {
    const RESET_VALUE: u32 = 0x03;
}
