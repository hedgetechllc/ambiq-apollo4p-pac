#[doc = "Register `PINCFG76` reader"]
pub type R = crate::R<Pincfg76Spec>;
#[doc = "Register `PINCFG76` writer"]
pub type W = crate::W<Pincfg76Spec>;
#[doc = "Function select for GPIO pin 76\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fncsel76 {
    #[doc = "0: MSPI Master 2 Interface Signal"]
    Mspi2_2 = 0,
    #[doc = "1: 32kHZ from analog"]
    _32khzXt = 1,
    #[doc = "2: Display SPI Data2"]
    DispQspiD2 = 2,
    #[doc = "3: General purpose I/O"]
    Gpio = 3,
    #[doc = "4: UART Request to Send (RTS) (UART 0)"]
    Uart0rts = 4,
    #[doc = "5: Display Data 12"]
    DispD12 = 5,
    #[doc = "6: Timer/Counter input or output; Selection of direction is done via CTIMER register settings."]
    Ct76 = 6,
    #[doc = "7: IOMSTR/MSPI N Chip Select. Polarity is determined by CE_POLARITY field"]
    Nce76 = 7,
    #[doc = "8: Observation bus bit 12"]
    Obsbus12 = 8,
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
impl From<Fncsel76> for u8 {
    #[inline(always)]
    fn from(variant: Fncsel76) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fncsel76 {
    type Ux = u8;
}
impl crate::IsEnum for Fncsel76 {}
#[doc = "Field `FNCSEL76` reader - Function select for GPIO pin 76"]
pub type Fncsel76R = crate::FieldReader<Fncsel76>;
impl Fncsel76R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fncsel76 {
        match self.bits {
            0 => Fncsel76::Mspi2_2,
            1 => Fncsel76::_32khzXt,
            2 => Fncsel76::DispQspiD2,
            3 => Fncsel76::Gpio,
            4 => Fncsel76::Uart0rts,
            5 => Fncsel76::DispD12,
            6 => Fncsel76::Ct76,
            7 => Fncsel76::Nce76,
            8 => Fncsel76::Obsbus12,
            9 => Fncsel76::Reserved9,
            10 => Fncsel76::Reserved10,
            11 => Fncsel76::Fpio,
            12 => Fncsel76::Reserved12,
            13 => Fncsel76::Reserved13,
            14 => Fncsel76::Reserved14,
            15 => Fncsel76::Reserved15,
            _ => unreachable!(),
        }
    }
    #[doc = "MSPI Master 2 Interface Signal"]
    #[inline(always)]
    pub fn is_mspi2_2(&self) -> bool {
        *self == Fncsel76::Mspi2_2
    }
    #[doc = "32kHZ from analog"]
    #[inline(always)]
    pub fn is_32khz_xt(&self) -> bool {
        *self == Fncsel76::_32khzXt
    }
    #[doc = "Display SPI Data2"]
    #[inline(always)]
    pub fn is_disp_qspi_d2(&self) -> bool {
        *self == Fncsel76::DispQspiD2
    }
    #[doc = "General purpose I/O"]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == Fncsel76::Gpio
    }
    #[doc = "UART Request to Send (RTS) (UART 0)"]
    #[inline(always)]
    pub fn is_uart0rts(&self) -> bool {
        *self == Fncsel76::Uart0rts
    }
    #[doc = "Display Data 12"]
    #[inline(always)]
    pub fn is_disp_d12(&self) -> bool {
        *self == Fncsel76::DispD12
    }
    #[doc = "Timer/Counter input or output; Selection of direction is done via CTIMER register settings."]
    #[inline(always)]
    pub fn is_ct76(&self) -> bool {
        *self == Fncsel76::Ct76
    }
    #[doc = "IOMSTR/MSPI N Chip Select. Polarity is determined by CE_POLARITY field"]
    #[inline(always)]
    pub fn is_nce76(&self) -> bool {
        *self == Fncsel76::Nce76
    }
    #[doc = "Observation bus bit 12"]
    #[inline(always)]
    pub fn is_obsbus12(&self) -> bool {
        *self == Fncsel76::Obsbus12
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved9(&self) -> bool {
        *self == Fncsel76::Reserved9
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved10(&self) -> bool {
        *self == Fncsel76::Reserved10
    }
    #[doc = "Fast PIO"]
    #[inline(always)]
    pub fn is_fpio(&self) -> bool {
        *self == Fncsel76::Fpio
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved12(&self) -> bool {
        *self == Fncsel76::Reserved12
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved13(&self) -> bool {
        *self == Fncsel76::Reserved13
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved14(&self) -> bool {
        *self == Fncsel76::Reserved14
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved15(&self) -> bool {
        *self == Fncsel76::Reserved15
    }
}
#[doc = "Field `FNCSEL76` writer - Function select for GPIO pin 76"]
pub type Fncsel76W<'a, REG> = crate::FieldWriter<'a, REG, 4, Fncsel76, crate::Safe>;
impl<'a, REG> Fncsel76W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "MSPI Master 2 Interface Signal"]
    #[inline(always)]
    pub fn mspi2_2(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel76::Mspi2_2)
    }
    #[doc = "32kHZ from analog"]
    #[inline(always)]
    pub fn _32khz_xt(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel76::_32khzXt)
    }
    #[doc = "Display SPI Data2"]
    #[inline(always)]
    pub fn disp_qspi_d2(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel76::DispQspiD2)
    }
    #[doc = "General purpose I/O"]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel76::Gpio)
    }
    #[doc = "UART Request to Send (RTS) (UART 0)"]
    #[inline(always)]
    pub fn uart0rts(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel76::Uart0rts)
    }
    #[doc = "Display Data 12"]
    #[inline(always)]
    pub fn disp_d12(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel76::DispD12)
    }
    #[doc = "Timer/Counter input or output; Selection of direction is done via CTIMER register settings."]
    #[inline(always)]
    pub fn ct76(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel76::Ct76)
    }
    #[doc = "IOMSTR/MSPI N Chip Select. Polarity is determined by CE_POLARITY field"]
    #[inline(always)]
    pub fn nce76(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel76::Nce76)
    }
    #[doc = "Observation bus bit 12"]
    #[inline(always)]
    pub fn obsbus12(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel76::Obsbus12)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved9(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel76::Reserved9)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved10(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel76::Reserved10)
    }
    #[doc = "Fast PIO"]
    #[inline(always)]
    pub fn fpio(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel76::Fpio)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved12(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel76::Reserved12)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved13(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel76::Reserved13)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved14(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel76::Reserved14)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved15(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel76::Reserved15)
    }
}
#[doc = "Field `INPEN76` reader - Input enable for GPIO 76"]
pub type Inpen76R = crate::BitReader;
#[doc = "Field `INPEN76` writer - Input enable for GPIO 76"]
pub type Inpen76W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDZERO76` reader - Return 0 for read data on GPIO 76"]
pub type Rdzero76R = crate::BitReader;
#[doc = "Field `RDZERO76` writer - Return 0 for read data on GPIO 76"]
pub type Rdzero76W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Interrupt enable for GPIO 76\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Irpten76 {
    #[doc = "0: Interrupts are disabled for this GPIO"]
    Dis = 0,
    #[doc = "1: Interrupts are enabled for falling edge transition on this GPIO"]
    Intfall = 1,
    #[doc = "2: Interrupts are enabled for rising edge transitions on this GPIO"]
    Intrise = 2,
    #[doc = "3: Interrupts are enabled for any edge transition on this GPIO"]
    Intany = 3,
}
impl From<Irpten76> for u8 {
    #[inline(always)]
    fn from(variant: Irpten76) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Irpten76 {
    type Ux = u8;
}
impl crate::IsEnum for Irpten76 {}
#[doc = "Field `IRPTEN76` reader - Interrupt enable for GPIO 76"]
pub type Irpten76R = crate::FieldReader<Irpten76>;
impl Irpten76R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irpten76 {
        match self.bits {
            0 => Irpten76::Dis,
            1 => Irpten76::Intfall,
            2 => Irpten76::Intrise,
            3 => Irpten76::Intany,
            _ => unreachable!(),
        }
    }
    #[doc = "Interrupts are disabled for this GPIO"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Irpten76::Dis
    }
    #[doc = "Interrupts are enabled for falling edge transition on this GPIO"]
    #[inline(always)]
    pub fn is_intfall(&self) -> bool {
        *self == Irpten76::Intfall
    }
    #[doc = "Interrupts are enabled for rising edge transitions on this GPIO"]
    #[inline(always)]
    pub fn is_intrise(&self) -> bool {
        *self == Irpten76::Intrise
    }
    #[doc = "Interrupts are enabled for any edge transition on this GPIO"]
    #[inline(always)]
    pub fn is_intany(&self) -> bool {
        *self == Irpten76::Intany
    }
}
#[doc = "Field `IRPTEN76` writer - Interrupt enable for GPIO 76"]
pub type Irpten76W<'a, REG> = crate::FieldWriter<'a, REG, 2, Irpten76, crate::Safe>;
impl<'a, REG> Irpten76W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Interrupts are disabled for this GPIO"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten76::Dis)
    }
    #[doc = "Interrupts are enabled for falling edge transition on this GPIO"]
    #[inline(always)]
    pub fn intfall(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten76::Intfall)
    }
    #[doc = "Interrupts are enabled for rising edge transitions on this GPIO"]
    #[inline(always)]
    pub fn intrise(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten76::Intrise)
    }
    #[doc = "Interrupts are enabled for any edge transition on this GPIO"]
    #[inline(always)]
    pub fn intany(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten76::Intany)
    }
}
#[doc = "Pin IO mode selection for GPIO pin 76\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Outcfg76 {
    #[doc = "0: Output Disabled"]
    Dis = 0,
    #[doc = "1: Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    Pushpull = 1,
    #[doc = "2: Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    Od = 2,
    #[doc = "3: Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    Ts = 3,
}
impl From<Outcfg76> for u8 {
    #[inline(always)]
    fn from(variant: Outcfg76) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Outcfg76 {
    type Ux = u8;
}
impl crate::IsEnum for Outcfg76 {}
#[doc = "Field `OUTCFG76` reader - Pin IO mode selection for GPIO pin 76"]
pub type Outcfg76R = crate::FieldReader<Outcfg76>;
impl Outcfg76R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Outcfg76 {
        match self.bits {
            0 => Outcfg76::Dis,
            1 => Outcfg76::Pushpull,
            2 => Outcfg76::Od,
            3 => Outcfg76::Ts,
            _ => unreachable!(),
        }
    }
    #[doc = "Output Disabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Outcfg76::Dis
    }
    #[doc = "Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == Outcfg76::Pushpull
    }
    #[doc = "Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == Outcfg76::Od
    }
    #[doc = "Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == Outcfg76::Ts
    }
}
#[doc = "Field `OUTCFG76` writer - Pin IO mode selection for GPIO pin 76"]
pub type Outcfg76W<'a, REG> = crate::FieldWriter<'a, REG, 2, Outcfg76, crate::Safe>;
impl<'a, REG> Outcfg76W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Output Disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg76::Dis)
    }
    #[doc = "Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg76::Pushpull)
    }
    #[doc = "Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    #[inline(always)]
    pub fn od(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg76::Od)
    }
    #[doc = "Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    #[inline(always)]
    pub fn ts(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg76::Ts)
    }
}
#[doc = "Drive strength selection for GPIO 76\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ds76 {
    #[doc = "0: 0.1x output driver selected"]
    _0p1x = 0,
    #[doc = "1: 0.5x output driver selected"]
    _0p5x = 1,
    #[doc = "2: 0.75x output driver selected"]
    _0p75x = 2,
    #[doc = "3: 1.0x output driver selected"]
    _1p0x = 3,
}
impl From<Ds76> for u8 {
    #[inline(always)]
    fn from(variant: Ds76) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ds76 {
    type Ux = u8;
}
impl crate::IsEnum for Ds76 {}
#[doc = "Field `DS76` reader - Drive strength selection for GPIO 76"]
pub type Ds76R = crate::FieldReader<Ds76>;
impl Ds76R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ds76 {
        match self.bits {
            0 => Ds76::_0p1x,
            1 => Ds76::_0p5x,
            2 => Ds76::_0p75x,
            3 => Ds76::_1p0x,
            _ => unreachable!(),
        }
    }
    #[doc = "0.1x output driver selected"]
    #[inline(always)]
    pub fn is_0p1x(&self) -> bool {
        *self == Ds76::_0p1x
    }
    #[doc = "0.5x output driver selected"]
    #[inline(always)]
    pub fn is_0p5x(&self) -> bool {
        *self == Ds76::_0p5x
    }
    #[doc = "0.75x output driver selected"]
    #[inline(always)]
    pub fn is_0p75x(&self) -> bool {
        *self == Ds76::_0p75x
    }
    #[doc = "1.0x output driver selected"]
    #[inline(always)]
    pub fn is_1p0x(&self) -> bool {
        *self == Ds76::_1p0x
    }
}
#[doc = "Field `DS76` writer - Drive strength selection for GPIO 76"]
pub type Ds76W<'a, REG> = crate::FieldWriter<'a, REG, 2, Ds76, crate::Safe>;
impl<'a, REG> Ds76W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "0.1x output driver selected"]
    #[inline(always)]
    pub fn _0p1x(self) -> &'a mut crate::W<REG> {
        self.variant(Ds76::_0p1x)
    }
    #[doc = "0.5x output driver selected"]
    #[inline(always)]
    pub fn _0p5x(self) -> &'a mut crate::W<REG> {
        self.variant(Ds76::_0p5x)
    }
    #[doc = "0.75x output driver selected"]
    #[inline(always)]
    pub fn _0p75x(self) -> &'a mut crate::W<REG> {
        self.variant(Ds76::_0p75x)
    }
    #[doc = "1.0x output driver selected"]
    #[inline(always)]
    pub fn _1p0x(self) -> &'a mut crate::W<REG> {
        self.variant(Ds76::_1p0x)
    }
}
#[doc = "Field `SR76` reader - Configure the slew rate"]
pub type Sr76R = crate::BitReader;
#[doc = "Field `SR76` writer - Configure the slew rate"]
pub type Sr76W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Pullup/Pulldown configuration for GPIO 76\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pullcfg76 {
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
impl From<Pullcfg76> for u8 {
    #[inline(always)]
    fn from(variant: Pullcfg76) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pullcfg76 {
    type Ux = u8;
}
impl crate::IsEnum for Pullcfg76 {}
#[doc = "Field `PULLCFG76` reader - Pullup/Pulldown configuration for GPIO 76"]
pub type Pullcfg76R = crate::FieldReader<Pullcfg76>;
impl Pullcfg76R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pullcfg76 {
        match self.bits {
            0 => Pullcfg76::Dis,
            1 => Pullcfg76::Pd50k,
            2 => Pullcfg76::Pu15k,
            3 => Pullcfg76::Pu6k,
            4 => Pullcfg76::Pu12k,
            5 => Pullcfg76::Pu24k,
            6 => Pullcfg76::Pu50k,
            7 => Pullcfg76::Pu100k,
            _ => unreachable!(),
        }
    }
    #[doc = "No pullup or pulldown selected"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Pullcfg76::Dis
    }
    #[doc = "50K Pulldown selected"]
    #[inline(always)]
    pub fn is_pd50k(&self) -> bool {
        *self == Pullcfg76::Pd50k
    }
    #[doc = "1.5K Pullup selected"]
    #[inline(always)]
    pub fn is_pu15k(&self) -> bool {
        *self == Pullcfg76::Pu15k
    }
    #[doc = "6K Pullup selected"]
    #[inline(always)]
    pub fn is_pu6k(&self) -> bool {
        *self == Pullcfg76::Pu6k
    }
    #[doc = "12K Pullup selected"]
    #[inline(always)]
    pub fn is_pu12k(&self) -> bool {
        *self == Pullcfg76::Pu12k
    }
    #[doc = "24K Pullup selected"]
    #[inline(always)]
    pub fn is_pu24k(&self) -> bool {
        *self == Pullcfg76::Pu24k
    }
    #[doc = "50K Pullup selected"]
    #[inline(always)]
    pub fn is_pu50k(&self) -> bool {
        *self == Pullcfg76::Pu50k
    }
    #[doc = "100K Pullup selected"]
    #[inline(always)]
    pub fn is_pu100k(&self) -> bool {
        *self == Pullcfg76::Pu100k
    }
}
#[doc = "Field `PULLCFG76` writer - Pullup/Pulldown configuration for GPIO 76"]
pub type Pullcfg76W<'a, REG> = crate::FieldWriter<'a, REG, 3, Pullcfg76, crate::Safe>;
impl<'a, REG> Pullcfg76W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No pullup or pulldown selected"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg76::Dis)
    }
    #[doc = "50K Pulldown selected"]
    #[inline(always)]
    pub fn pd50k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg76::Pd50k)
    }
    #[doc = "1.5K Pullup selected"]
    #[inline(always)]
    pub fn pu15k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg76::Pu15k)
    }
    #[doc = "6K Pullup selected"]
    #[inline(always)]
    pub fn pu6k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg76::Pu6k)
    }
    #[doc = "12K Pullup selected"]
    #[inline(always)]
    pub fn pu12k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg76::Pu12k)
    }
    #[doc = "24K Pullup selected"]
    #[inline(always)]
    pub fn pu24k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg76::Pu24k)
    }
    #[doc = "50K Pullup selected"]
    #[inline(always)]
    pub fn pu50k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg76::Pu50k)
    }
    #[doc = "100K Pullup selected"]
    #[inline(always)]
    pub fn pu100k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg76::Pu100k)
    }
}
#[doc = "IOMSTR/MSPI N Chip Select 76, DISP control signals DE, CSX, and CS. Polarity is determined by CE_POLARITY field\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ncesrc76 {
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
impl From<Ncesrc76> for u8 {
    #[inline(always)]
    fn from(variant: Ncesrc76) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ncesrc76 {
    type Ux = u8;
}
impl crate::IsEnum for Ncesrc76 {}
#[doc = "Field `NCESRC76` reader - IOMSTR/MSPI N Chip Select 76, DISP control signals DE, CSX, and CS. Polarity is determined by CE_POLARITY field"]
pub type Ncesrc76R = crate::FieldReader<Ncesrc76>;
impl Ncesrc76R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ncesrc76> {
        match self.bits {
            0 => Some(Ncesrc76::Iom0ce0),
            1 => Some(Ncesrc76::Iom0ce1),
            2 => Some(Ncesrc76::Iom0ce2),
            3 => Some(Ncesrc76::Iom0ce3),
            4 => Some(Ncesrc76::Iom1ce0),
            5 => Some(Ncesrc76::Iom1ce1),
            6 => Some(Ncesrc76::Iom1ce2),
            7 => Some(Ncesrc76::Iom1ce3),
            8 => Some(Ncesrc76::Iom2ce0),
            9 => Some(Ncesrc76::Iom2ce1),
            10 => Some(Ncesrc76::Iom2ce2),
            11 => Some(Ncesrc76::Iom2ce3),
            12 => Some(Ncesrc76::Iom3ce0),
            13 => Some(Ncesrc76::Iom3ce1),
            14 => Some(Ncesrc76::Iom3ce2),
            15 => Some(Ncesrc76::Iom3ce3),
            16 => Some(Ncesrc76::Iom4ce0),
            17 => Some(Ncesrc76::Iom4ce1),
            18 => Some(Ncesrc76::Iom4ce2),
            19 => Some(Ncesrc76::Iom4ce3),
            20 => Some(Ncesrc76::Iom5ce0),
            21 => Some(Ncesrc76::Iom5ce1),
            22 => Some(Ncesrc76::Iom5ce2),
            23 => Some(Ncesrc76::Iom5ce3),
            24 => Some(Ncesrc76::Iom6ce0),
            25 => Some(Ncesrc76::Iom6ce1),
            26 => Some(Ncesrc76::Iom6ce2),
            27 => Some(Ncesrc76::Iom6ce3),
            28 => Some(Ncesrc76::Iom7ce0),
            29 => Some(Ncesrc76::Iom7ce1),
            30 => Some(Ncesrc76::Iom7ce2),
            31 => Some(Ncesrc76::Iom7ce3),
            32 => Some(Ncesrc76::Mspi0cen0),
            33 => Some(Ncesrc76::Mspi0cen1),
            34 => Some(Ncesrc76::Mspi1cen0),
            35 => Some(Ncesrc76::Mspi1cen1),
            36 => Some(Ncesrc76::Mspi2cen0),
            37 => Some(Ncesrc76::Mspi2cen1),
            38 => Some(Ncesrc76::DcDpiDe),
            39 => Some(Ncesrc76::DispContCsx),
            40 => Some(Ncesrc76::DcSpiCsN),
            41 => Some(Ncesrc76::DcQspiCsN),
            42 => Some(Ncesrc76::DcResx),
            _ => None,
        }
    }
    #[doc = "IOM 0 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom0ce0(&self) -> bool {
        *self == Ncesrc76::Iom0ce0
    }
    #[doc = "IOM 0 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom0ce1(&self) -> bool {
        *self == Ncesrc76::Iom0ce1
    }
    #[doc = "IOM 0 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom0ce2(&self) -> bool {
        *self == Ncesrc76::Iom0ce2
    }
    #[doc = "IOM 0 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom0ce3(&self) -> bool {
        *self == Ncesrc76::Iom0ce3
    }
    #[doc = "IOM 1 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom1ce0(&self) -> bool {
        *self == Ncesrc76::Iom1ce0
    }
    #[doc = "IOM 1 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom1ce1(&self) -> bool {
        *self == Ncesrc76::Iom1ce1
    }
    #[doc = "IOM 1 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom1ce2(&self) -> bool {
        *self == Ncesrc76::Iom1ce2
    }
    #[doc = "IOM 1 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom1ce3(&self) -> bool {
        *self == Ncesrc76::Iom1ce3
    }
    #[doc = "IOM 2 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom2ce0(&self) -> bool {
        *self == Ncesrc76::Iom2ce0
    }
    #[doc = "IOM 2 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom2ce1(&self) -> bool {
        *self == Ncesrc76::Iom2ce1
    }
    #[doc = "IOM 2 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom2ce2(&self) -> bool {
        *self == Ncesrc76::Iom2ce2
    }
    #[doc = "IOM 2 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom2ce3(&self) -> bool {
        *self == Ncesrc76::Iom2ce3
    }
    #[doc = "IOM 3 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom3ce0(&self) -> bool {
        *self == Ncesrc76::Iom3ce0
    }
    #[doc = "IOM 3 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom3ce1(&self) -> bool {
        *self == Ncesrc76::Iom3ce1
    }
    #[doc = "IOM 3 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom3ce2(&self) -> bool {
        *self == Ncesrc76::Iom3ce2
    }
    #[doc = "IOM 3 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom3ce3(&self) -> bool {
        *self == Ncesrc76::Iom3ce3
    }
    #[doc = "IOM 4 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom4ce0(&self) -> bool {
        *self == Ncesrc76::Iom4ce0
    }
    #[doc = "IOM 4 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom4ce1(&self) -> bool {
        *self == Ncesrc76::Iom4ce1
    }
    #[doc = "IOM 4 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom4ce2(&self) -> bool {
        *self == Ncesrc76::Iom4ce2
    }
    #[doc = "IOM 4 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom4ce3(&self) -> bool {
        *self == Ncesrc76::Iom4ce3
    }
    #[doc = "IOM 5 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom5ce0(&self) -> bool {
        *self == Ncesrc76::Iom5ce0
    }
    #[doc = "IOM 5 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom5ce1(&self) -> bool {
        *self == Ncesrc76::Iom5ce1
    }
    #[doc = "IOM 5 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom5ce2(&self) -> bool {
        *self == Ncesrc76::Iom5ce2
    }
    #[doc = "IOM 5 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom5ce3(&self) -> bool {
        *self == Ncesrc76::Iom5ce3
    }
    #[doc = "IOM 6 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom6ce0(&self) -> bool {
        *self == Ncesrc76::Iom6ce0
    }
    #[doc = "IOM 6 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom6ce1(&self) -> bool {
        *self == Ncesrc76::Iom6ce1
    }
    #[doc = "IOM 6 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom6ce2(&self) -> bool {
        *self == Ncesrc76::Iom6ce2
    }
    #[doc = "IOM 6 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom6ce3(&self) -> bool {
        *self == Ncesrc76::Iom6ce3
    }
    #[doc = "IOM 7 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom7ce0(&self) -> bool {
        *self == Ncesrc76::Iom7ce0
    }
    #[doc = "IOM 7 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom7ce1(&self) -> bool {
        *self == Ncesrc76::Iom7ce1
    }
    #[doc = "IOM 7 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom7ce2(&self) -> bool {
        *self == Ncesrc76::Iom7ce2
    }
    #[doc = "IOM 7 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom7ce3(&self) -> bool {
        *self == Ncesrc76::Iom7ce3
    }
    #[doc = "MSPI 0 NCE 0 module"]
    #[inline(always)]
    pub fn is_mspi0cen0(&self) -> bool {
        *self == Ncesrc76::Mspi0cen0
    }
    #[doc = "MSPI 0 NCE 1 module"]
    #[inline(always)]
    pub fn is_mspi0cen1(&self) -> bool {
        *self == Ncesrc76::Mspi0cen1
    }
    #[doc = "MSPI 1 NCE 0 module"]
    #[inline(always)]
    pub fn is_mspi1cen0(&self) -> bool {
        *self == Ncesrc76::Mspi1cen0
    }
    #[doc = "MSPI 1 NCE 1 module"]
    #[inline(always)]
    pub fn is_mspi1cen1(&self) -> bool {
        *self == Ncesrc76::Mspi1cen1
    }
    #[doc = "MSPI 2 NCE 0 module"]
    #[inline(always)]
    pub fn is_mspi2cen0(&self) -> bool {
        *self == Ncesrc76::Mspi2cen0
    }
    #[doc = "MSPI 2 NCE 1 module"]
    #[inline(always)]
    pub fn is_mspi2cen1(&self) -> bool {
        *self == Ncesrc76::Mspi2cen1
    }
    #[doc = "DC DPI DE module"]
    #[inline(always)]
    pub fn is_dc_dpi_de(&self) -> bool {
        *self == Ncesrc76::DcDpiDe
    }
    #[doc = "DISP CONT CSX module"]
    #[inline(always)]
    pub fn is_disp_cont_csx(&self) -> bool {
        *self == Ncesrc76::DispContCsx
    }
    #[doc = "DC SPI CS_N module"]
    #[inline(always)]
    pub fn is_dc_spi_cs_n(&self) -> bool {
        *self == Ncesrc76::DcSpiCsN
    }
    #[doc = "DC QSPI CS_N module"]
    #[inline(always)]
    pub fn is_dc_qspi_cs_n(&self) -> bool {
        *self == Ncesrc76::DcQspiCsN
    }
    #[doc = "DC module RESX"]
    #[inline(always)]
    pub fn is_dc_resx(&self) -> bool {
        *self == Ncesrc76::DcResx
    }
}
#[doc = "Field `NCESRC76` writer - IOMSTR/MSPI N Chip Select 76, DISP control signals DE, CSX, and CS. Polarity is determined by CE_POLARITY field"]
pub type Ncesrc76W<'a, REG> = crate::FieldWriter<'a, REG, 6, Ncesrc76>;
impl<'a, REG> Ncesrc76W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "IOM 0 NCE 0 module"]
    #[inline(always)]
    pub fn iom0ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc76::Iom0ce0)
    }
    #[doc = "IOM 0 NCE 1 module"]
    #[inline(always)]
    pub fn iom0ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc76::Iom0ce1)
    }
    #[doc = "IOM 0 NCE 2 module"]
    #[inline(always)]
    pub fn iom0ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc76::Iom0ce2)
    }
    #[doc = "IOM 0 NCE 3 module"]
    #[inline(always)]
    pub fn iom0ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc76::Iom0ce3)
    }
    #[doc = "IOM 1 NCE 0 module"]
    #[inline(always)]
    pub fn iom1ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc76::Iom1ce0)
    }
    #[doc = "IOM 1 NCE 1 module"]
    #[inline(always)]
    pub fn iom1ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc76::Iom1ce1)
    }
    #[doc = "IOM 1 NCE 2 module"]
    #[inline(always)]
    pub fn iom1ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc76::Iom1ce2)
    }
    #[doc = "IOM 1 NCE 3 module"]
    #[inline(always)]
    pub fn iom1ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc76::Iom1ce3)
    }
    #[doc = "IOM 2 NCE 0 module"]
    #[inline(always)]
    pub fn iom2ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc76::Iom2ce0)
    }
    #[doc = "IOM 2 NCE 1 module"]
    #[inline(always)]
    pub fn iom2ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc76::Iom2ce1)
    }
    #[doc = "IOM 2 NCE 2 module"]
    #[inline(always)]
    pub fn iom2ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc76::Iom2ce2)
    }
    #[doc = "IOM 2 NCE 3 module"]
    #[inline(always)]
    pub fn iom2ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc76::Iom2ce3)
    }
    #[doc = "IOM 3 NCE 0 module"]
    #[inline(always)]
    pub fn iom3ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc76::Iom3ce0)
    }
    #[doc = "IOM 3 NCE 1 module"]
    #[inline(always)]
    pub fn iom3ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc76::Iom3ce1)
    }
    #[doc = "IOM 3 NCE 2 module"]
    #[inline(always)]
    pub fn iom3ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc76::Iom3ce2)
    }
    #[doc = "IOM 3 NCE 3 module"]
    #[inline(always)]
    pub fn iom3ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc76::Iom3ce3)
    }
    #[doc = "IOM 4 NCE 0 module"]
    #[inline(always)]
    pub fn iom4ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc76::Iom4ce0)
    }
    #[doc = "IOM 4 NCE 1 module"]
    #[inline(always)]
    pub fn iom4ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc76::Iom4ce1)
    }
    #[doc = "IOM 4 NCE 2 module"]
    #[inline(always)]
    pub fn iom4ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc76::Iom4ce2)
    }
    #[doc = "IOM 4 NCE 3 module"]
    #[inline(always)]
    pub fn iom4ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc76::Iom4ce3)
    }
    #[doc = "IOM 5 NCE 0 module"]
    #[inline(always)]
    pub fn iom5ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc76::Iom5ce0)
    }
    #[doc = "IOM 5 NCE 1 module"]
    #[inline(always)]
    pub fn iom5ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc76::Iom5ce1)
    }
    #[doc = "IOM 5 NCE 2 module"]
    #[inline(always)]
    pub fn iom5ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc76::Iom5ce2)
    }
    #[doc = "IOM 5 NCE 3 module"]
    #[inline(always)]
    pub fn iom5ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc76::Iom5ce3)
    }
    #[doc = "IOM 6 NCE 0 module"]
    #[inline(always)]
    pub fn iom6ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc76::Iom6ce0)
    }
    #[doc = "IOM 6 NCE 1 module"]
    #[inline(always)]
    pub fn iom6ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc76::Iom6ce1)
    }
    #[doc = "IOM 6 NCE 2 module"]
    #[inline(always)]
    pub fn iom6ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc76::Iom6ce2)
    }
    #[doc = "IOM 6 NCE 3 module"]
    #[inline(always)]
    pub fn iom6ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc76::Iom6ce3)
    }
    #[doc = "IOM 7 NCE 0 module"]
    #[inline(always)]
    pub fn iom7ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc76::Iom7ce0)
    }
    #[doc = "IOM 7 NCE 1 module"]
    #[inline(always)]
    pub fn iom7ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc76::Iom7ce1)
    }
    #[doc = "IOM 7 NCE 2 module"]
    #[inline(always)]
    pub fn iom7ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc76::Iom7ce2)
    }
    #[doc = "IOM 7 NCE 3 module"]
    #[inline(always)]
    pub fn iom7ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc76::Iom7ce3)
    }
    #[doc = "MSPI 0 NCE 0 module"]
    #[inline(always)]
    pub fn mspi0cen0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc76::Mspi0cen0)
    }
    #[doc = "MSPI 0 NCE 1 module"]
    #[inline(always)]
    pub fn mspi0cen1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc76::Mspi0cen1)
    }
    #[doc = "MSPI 1 NCE 0 module"]
    #[inline(always)]
    pub fn mspi1cen0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc76::Mspi1cen0)
    }
    #[doc = "MSPI 1 NCE 1 module"]
    #[inline(always)]
    pub fn mspi1cen1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc76::Mspi1cen1)
    }
    #[doc = "MSPI 2 NCE 0 module"]
    #[inline(always)]
    pub fn mspi2cen0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc76::Mspi2cen0)
    }
    #[doc = "MSPI 2 NCE 1 module"]
    #[inline(always)]
    pub fn mspi2cen1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc76::Mspi2cen1)
    }
    #[doc = "DC DPI DE module"]
    #[inline(always)]
    pub fn dc_dpi_de(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc76::DcDpiDe)
    }
    #[doc = "DISP CONT CSX module"]
    #[inline(always)]
    pub fn disp_cont_csx(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc76::DispContCsx)
    }
    #[doc = "DC SPI CS_N module"]
    #[inline(always)]
    pub fn dc_spi_cs_n(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc76::DcSpiCsN)
    }
    #[doc = "DC QSPI CS_N module"]
    #[inline(always)]
    pub fn dc_qspi_cs_n(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc76::DcQspiCsN)
    }
    #[doc = "DC module RESX"]
    #[inline(always)]
    pub fn dc_resx(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc76::DcResx)
    }
}
#[doc = "Polarity select for NCE for GPIO 76\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ncepol76 {
    #[doc = "0: Polarity is active low"]
    Low = 0,
    #[doc = "1: Polarity is active high"]
    High = 1,
}
impl From<Ncepol76> for bool {
    #[inline(always)]
    fn from(variant: Ncepol76) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NCEPOL76` reader - Polarity select for NCE for GPIO 76"]
pub type Ncepol76R = crate::BitReader<Ncepol76>;
impl Ncepol76R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ncepol76 {
        match self.bits {
            false => Ncepol76::Low,
            true => Ncepol76::High,
        }
    }
    #[doc = "Polarity is active low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Ncepol76::Low
    }
    #[doc = "Polarity is active high"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Ncepol76::High
    }
}
#[doc = "Field `NCEPOL76` writer - Polarity select for NCE for GPIO 76"]
pub type Ncepol76W<'a, REG> = crate::BitWriter<'a, REG, Ncepol76>;
impl<'a, REG> Ncepol76W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Polarity is active low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Ncepol76::Low)
    }
    #[doc = "Polarity is active high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Ncepol76::High)
    }
}
#[doc = "Field `FIEN76` reader - Force input enable active regardless of function selected. Otherwise the selected function will enable the input only when needed"]
pub type Fien76R = crate::BitReader;
#[doc = "Field `FIEN76` writer - Force input enable active regardless of function selected. Otherwise the selected function will enable the input only when needed"]
pub type Fien76W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FOEN76` reader - Force output enable active regardless of function selected. Otherwise the selected function will enable the output only when needed"]
pub type Foen76R = crate::BitReader;
#[doc = "Field `FOEN76` writer - Force output enable active regardless of function selected. Otherwise the selected function will enable the output only when needed"]
pub type Foen76W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Function select for GPIO pin 76"]
    #[inline(always)]
    pub fn fncsel76(&self) -> Fncsel76R {
        Fncsel76R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Input enable for GPIO 76"]
    #[inline(always)]
    pub fn inpen76(&self) -> Inpen76R {
        Inpen76R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Return 0 for read data on GPIO 76"]
    #[inline(always)]
    pub fn rdzero76(&self) -> Rdzero76R {
        Rdzero76R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Interrupt enable for GPIO 76"]
    #[inline(always)]
    pub fn irpten76(&self) -> Irpten76R {
        Irpten76R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Pin IO mode selection for GPIO pin 76"]
    #[inline(always)]
    pub fn outcfg76(&self) -> Outcfg76R {
        Outcfg76R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Drive strength selection for GPIO 76"]
    #[inline(always)]
    pub fn ds76(&self) -> Ds76R {
        Ds76R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - Configure the slew rate"]
    #[inline(always)]
    pub fn sr76(&self) -> Sr76R {
        Sr76R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - Pullup/Pulldown configuration for GPIO 76"]
    #[inline(always)]
    pub fn pullcfg76(&self) -> Pullcfg76R {
        Pullcfg76R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:21 - IOMSTR/MSPI N Chip Select 76, DISP control signals DE, CSX, and CS. Polarity is determined by CE_POLARITY field"]
    #[inline(always)]
    pub fn ncesrc76(&self) -> Ncesrc76R {
        Ncesrc76R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 22 - Polarity select for NCE for GPIO 76"]
    #[inline(always)]
    pub fn ncepol76(&self) -> Ncepol76R {
        Ncepol76R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 26 - Force input enable active regardless of function selected. Otherwise the selected function will enable the input only when needed"]
    #[inline(always)]
    pub fn fien76(&self) -> Fien76R {
        Fien76R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Force output enable active regardless of function selected. Otherwise the selected function will enable the output only when needed"]
    #[inline(always)]
    pub fn foen76(&self) -> Foen76R {
        Foen76R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Function select for GPIO pin 76"]
    #[inline(always)]
    #[must_use]
    pub fn fncsel76(&mut self) -> Fncsel76W<Pincfg76Spec> {
        Fncsel76W::new(self, 0)
    }
    #[doc = "Bit 4 - Input enable for GPIO 76"]
    #[inline(always)]
    #[must_use]
    pub fn inpen76(&mut self) -> Inpen76W<Pincfg76Spec> {
        Inpen76W::new(self, 4)
    }
    #[doc = "Bit 5 - Return 0 for read data on GPIO 76"]
    #[inline(always)]
    #[must_use]
    pub fn rdzero76(&mut self) -> Rdzero76W<Pincfg76Spec> {
        Rdzero76W::new(self, 5)
    }
    #[doc = "Bits 6:7 - Interrupt enable for GPIO 76"]
    #[inline(always)]
    #[must_use]
    pub fn irpten76(&mut self) -> Irpten76W<Pincfg76Spec> {
        Irpten76W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Pin IO mode selection for GPIO pin 76"]
    #[inline(always)]
    #[must_use]
    pub fn outcfg76(&mut self) -> Outcfg76W<Pincfg76Spec> {
        Outcfg76W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Drive strength selection for GPIO 76"]
    #[inline(always)]
    #[must_use]
    pub fn ds76(&mut self) -> Ds76W<Pincfg76Spec> {
        Ds76W::new(self, 10)
    }
    #[doc = "Bit 12 - Configure the slew rate"]
    #[inline(always)]
    #[must_use]
    pub fn sr76(&mut self) -> Sr76W<Pincfg76Spec> {
        Sr76W::new(self, 12)
    }
    #[doc = "Bits 13:15 - Pullup/Pulldown configuration for GPIO 76"]
    #[inline(always)]
    #[must_use]
    pub fn pullcfg76(&mut self) -> Pullcfg76W<Pincfg76Spec> {
        Pullcfg76W::new(self, 13)
    }
    #[doc = "Bits 16:21 - IOMSTR/MSPI N Chip Select 76, DISP control signals DE, CSX, and CS. Polarity is determined by CE_POLARITY field"]
    #[inline(always)]
    #[must_use]
    pub fn ncesrc76(&mut self) -> Ncesrc76W<Pincfg76Spec> {
        Ncesrc76W::new(self, 16)
    }
    #[doc = "Bit 22 - Polarity select for NCE for GPIO 76"]
    #[inline(always)]
    #[must_use]
    pub fn ncepol76(&mut self) -> Ncepol76W<Pincfg76Spec> {
        Ncepol76W::new(self, 22)
    }
    #[doc = "Bit 26 - Force input enable active regardless of function selected. Otherwise the selected function will enable the input only when needed"]
    #[inline(always)]
    #[must_use]
    pub fn fien76(&mut self) -> Fien76W<Pincfg76Spec> {
        Fien76W::new(self, 26)
    }
    #[doc = "Bit 27 - Force output enable active regardless of function selected. Otherwise the selected function will enable the output only when needed"]
    #[inline(always)]
    #[must_use]
    pub fn foen76(&mut self) -> Foen76W<Pincfg76Spec> {
        Foen76W::new(self, 27)
    }
}
#[doc = "Controls the operation of GPIO pin 76.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg76::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg76::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pincfg76Spec;
impl crate::RegisterSpec for Pincfg76Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pincfg76::R`](R) reader structure"]
impl crate::Readable for Pincfg76Spec {}
#[doc = "`write(|w| ..)` method takes [`pincfg76::W`](W) writer structure"]
impl crate::Writable for Pincfg76Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PINCFG76 to value 0x03"]
impl crate::Resettable for Pincfg76Spec {
    const RESET_VALUE: u32 = 0x03;
}
