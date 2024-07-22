#[doc = "Register `PINCFG75` reader"]
pub type R = crate::R<Pincfg75Spec>;
#[doc = "Register `PINCFG75` writer"]
pub type W = crate::W<Pincfg75Spec>;
#[doc = "Function select for GPIO pin 75\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fncsel75 {
    #[doc = "0: MSPI Master 2 Interface Signal"]
    Mspi2_1 = 0,
    #[doc = "1: 32kHZ from analog"]
    _32khzXt = 1,
    #[doc = "2: Display SPI Data1"]
    DispQspiD1 = 2,
    #[doc = "3: General purpose I/O"]
    Gpio = 3,
    #[doc = "4: UART receive input (UART 2)"]
    Uart2rx = 4,
    #[doc = "5: Display Data 11"]
    DispD11 = 5,
    #[doc = "6: Timer/Counter input or output; Selection of direction is done via CTIMER register settings."]
    Ct75 = 6,
    #[doc = "7: IOMSTR/MSPI N Chip Select. Polarity is determined by CE_POLARITY field"]
    Nce75 = 7,
    #[doc = "8: Observation bus bit 11"]
    Obsbus11 = 8,
    #[doc = "9: Display SPI DCx"]
    DispSpiDcx = 9,
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
impl From<Fncsel75> for u8 {
    #[inline(always)]
    fn from(variant: Fncsel75) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fncsel75 {
    type Ux = u8;
}
impl crate::IsEnum for Fncsel75 {}
#[doc = "Field `FNCSEL75` reader - Function select for GPIO pin 75"]
pub type Fncsel75R = crate::FieldReader<Fncsel75>;
impl Fncsel75R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fncsel75 {
        match self.bits {
            0 => Fncsel75::Mspi2_1,
            1 => Fncsel75::_32khzXt,
            2 => Fncsel75::DispQspiD1,
            3 => Fncsel75::Gpio,
            4 => Fncsel75::Uart2rx,
            5 => Fncsel75::DispD11,
            6 => Fncsel75::Ct75,
            7 => Fncsel75::Nce75,
            8 => Fncsel75::Obsbus11,
            9 => Fncsel75::DispSpiDcx,
            10 => Fncsel75::Reserved10,
            11 => Fncsel75::Fpio,
            12 => Fncsel75::Reserved12,
            13 => Fncsel75::Reserved13,
            14 => Fncsel75::Reserved14,
            15 => Fncsel75::Reserved15,
            _ => unreachable!(),
        }
    }
    #[doc = "MSPI Master 2 Interface Signal"]
    #[inline(always)]
    pub fn is_mspi2_1(&self) -> bool {
        *self == Fncsel75::Mspi2_1
    }
    #[doc = "32kHZ from analog"]
    #[inline(always)]
    pub fn is_32khz_xt(&self) -> bool {
        *self == Fncsel75::_32khzXt
    }
    #[doc = "Display SPI Data1"]
    #[inline(always)]
    pub fn is_disp_qspi_d1(&self) -> bool {
        *self == Fncsel75::DispQspiD1
    }
    #[doc = "General purpose I/O"]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == Fncsel75::Gpio
    }
    #[doc = "UART receive input (UART 2)"]
    #[inline(always)]
    pub fn is_uart2rx(&self) -> bool {
        *self == Fncsel75::Uart2rx
    }
    #[doc = "Display Data 11"]
    #[inline(always)]
    pub fn is_disp_d11(&self) -> bool {
        *self == Fncsel75::DispD11
    }
    #[doc = "Timer/Counter input or output; Selection of direction is done via CTIMER register settings."]
    #[inline(always)]
    pub fn is_ct75(&self) -> bool {
        *self == Fncsel75::Ct75
    }
    #[doc = "IOMSTR/MSPI N Chip Select. Polarity is determined by CE_POLARITY field"]
    #[inline(always)]
    pub fn is_nce75(&self) -> bool {
        *self == Fncsel75::Nce75
    }
    #[doc = "Observation bus bit 11"]
    #[inline(always)]
    pub fn is_obsbus11(&self) -> bool {
        *self == Fncsel75::Obsbus11
    }
    #[doc = "Display SPI DCx"]
    #[inline(always)]
    pub fn is_disp_spi_dcx(&self) -> bool {
        *self == Fncsel75::DispSpiDcx
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved10(&self) -> bool {
        *self == Fncsel75::Reserved10
    }
    #[doc = "Fast PIO"]
    #[inline(always)]
    pub fn is_fpio(&self) -> bool {
        *self == Fncsel75::Fpio
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved12(&self) -> bool {
        *self == Fncsel75::Reserved12
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved13(&self) -> bool {
        *self == Fncsel75::Reserved13
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved14(&self) -> bool {
        *self == Fncsel75::Reserved14
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved15(&self) -> bool {
        *self == Fncsel75::Reserved15
    }
}
#[doc = "Field `FNCSEL75` writer - Function select for GPIO pin 75"]
pub type Fncsel75W<'a, REG> = crate::FieldWriter<'a, REG, 4, Fncsel75, crate::Safe>;
impl<'a, REG> Fncsel75W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "MSPI Master 2 Interface Signal"]
    #[inline(always)]
    pub fn mspi2_1(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel75::Mspi2_1)
    }
    #[doc = "32kHZ from analog"]
    #[inline(always)]
    pub fn _32khz_xt(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel75::_32khzXt)
    }
    #[doc = "Display SPI Data1"]
    #[inline(always)]
    pub fn disp_qspi_d1(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel75::DispQspiD1)
    }
    #[doc = "General purpose I/O"]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel75::Gpio)
    }
    #[doc = "UART receive input (UART 2)"]
    #[inline(always)]
    pub fn uart2rx(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel75::Uart2rx)
    }
    #[doc = "Display Data 11"]
    #[inline(always)]
    pub fn disp_d11(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel75::DispD11)
    }
    #[doc = "Timer/Counter input or output; Selection of direction is done via CTIMER register settings."]
    #[inline(always)]
    pub fn ct75(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel75::Ct75)
    }
    #[doc = "IOMSTR/MSPI N Chip Select. Polarity is determined by CE_POLARITY field"]
    #[inline(always)]
    pub fn nce75(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel75::Nce75)
    }
    #[doc = "Observation bus bit 11"]
    #[inline(always)]
    pub fn obsbus11(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel75::Obsbus11)
    }
    #[doc = "Display SPI DCx"]
    #[inline(always)]
    pub fn disp_spi_dcx(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel75::DispSpiDcx)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved10(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel75::Reserved10)
    }
    #[doc = "Fast PIO"]
    #[inline(always)]
    pub fn fpio(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel75::Fpio)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved12(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel75::Reserved12)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved13(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel75::Reserved13)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved14(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel75::Reserved14)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved15(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel75::Reserved15)
    }
}
#[doc = "Field `INPEN75` reader - Input enable for GPIO 75"]
pub type Inpen75R = crate::BitReader;
#[doc = "Field `INPEN75` writer - Input enable for GPIO 75"]
pub type Inpen75W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDZERO75` reader - Return 0 for read data on GPIO 75"]
pub type Rdzero75R = crate::BitReader;
#[doc = "Field `RDZERO75` writer - Return 0 for read data on GPIO 75"]
pub type Rdzero75W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Interrupt enable for GPIO 75\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Irpten75 {
    #[doc = "0: Interrupts are disabled for this GPIO"]
    Dis = 0,
    #[doc = "1: Interrupts are enabled for falling edge transition on this GPIO"]
    Intfall = 1,
    #[doc = "2: Interrupts are enabled for rising edge transitions on this GPIO"]
    Intrise = 2,
    #[doc = "3: Interrupts are enabled for any edge transition on this GPIO"]
    Intany = 3,
}
impl From<Irpten75> for u8 {
    #[inline(always)]
    fn from(variant: Irpten75) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Irpten75 {
    type Ux = u8;
}
impl crate::IsEnum for Irpten75 {}
#[doc = "Field `IRPTEN75` reader - Interrupt enable for GPIO 75"]
pub type Irpten75R = crate::FieldReader<Irpten75>;
impl Irpten75R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irpten75 {
        match self.bits {
            0 => Irpten75::Dis,
            1 => Irpten75::Intfall,
            2 => Irpten75::Intrise,
            3 => Irpten75::Intany,
            _ => unreachable!(),
        }
    }
    #[doc = "Interrupts are disabled for this GPIO"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Irpten75::Dis
    }
    #[doc = "Interrupts are enabled for falling edge transition on this GPIO"]
    #[inline(always)]
    pub fn is_intfall(&self) -> bool {
        *self == Irpten75::Intfall
    }
    #[doc = "Interrupts are enabled for rising edge transitions on this GPIO"]
    #[inline(always)]
    pub fn is_intrise(&self) -> bool {
        *self == Irpten75::Intrise
    }
    #[doc = "Interrupts are enabled for any edge transition on this GPIO"]
    #[inline(always)]
    pub fn is_intany(&self) -> bool {
        *self == Irpten75::Intany
    }
}
#[doc = "Field `IRPTEN75` writer - Interrupt enable for GPIO 75"]
pub type Irpten75W<'a, REG> = crate::FieldWriter<'a, REG, 2, Irpten75, crate::Safe>;
impl<'a, REG> Irpten75W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Interrupts are disabled for this GPIO"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten75::Dis)
    }
    #[doc = "Interrupts are enabled for falling edge transition on this GPIO"]
    #[inline(always)]
    pub fn intfall(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten75::Intfall)
    }
    #[doc = "Interrupts are enabled for rising edge transitions on this GPIO"]
    #[inline(always)]
    pub fn intrise(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten75::Intrise)
    }
    #[doc = "Interrupts are enabled for any edge transition on this GPIO"]
    #[inline(always)]
    pub fn intany(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten75::Intany)
    }
}
#[doc = "Pin IO mode selection for GPIO pin 75\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Outcfg75 {
    #[doc = "0: Output Disabled"]
    Dis = 0,
    #[doc = "1: Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    Pushpull = 1,
    #[doc = "2: Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    Od = 2,
    #[doc = "3: Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    Ts = 3,
}
impl From<Outcfg75> for u8 {
    #[inline(always)]
    fn from(variant: Outcfg75) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Outcfg75 {
    type Ux = u8;
}
impl crate::IsEnum for Outcfg75 {}
#[doc = "Field `OUTCFG75` reader - Pin IO mode selection for GPIO pin 75"]
pub type Outcfg75R = crate::FieldReader<Outcfg75>;
impl Outcfg75R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Outcfg75 {
        match self.bits {
            0 => Outcfg75::Dis,
            1 => Outcfg75::Pushpull,
            2 => Outcfg75::Od,
            3 => Outcfg75::Ts,
            _ => unreachable!(),
        }
    }
    #[doc = "Output Disabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Outcfg75::Dis
    }
    #[doc = "Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == Outcfg75::Pushpull
    }
    #[doc = "Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == Outcfg75::Od
    }
    #[doc = "Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == Outcfg75::Ts
    }
}
#[doc = "Field `OUTCFG75` writer - Pin IO mode selection for GPIO pin 75"]
pub type Outcfg75W<'a, REG> = crate::FieldWriter<'a, REG, 2, Outcfg75, crate::Safe>;
impl<'a, REG> Outcfg75W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Output Disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg75::Dis)
    }
    #[doc = "Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg75::Pushpull)
    }
    #[doc = "Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    #[inline(always)]
    pub fn od(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg75::Od)
    }
    #[doc = "Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    #[inline(always)]
    pub fn ts(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg75::Ts)
    }
}
#[doc = "Drive strength selection for GPIO 75\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ds75 {
    #[doc = "0: 0.1x output driver selected"]
    _0p1x = 0,
    #[doc = "1: 0.5x output driver selected"]
    _0p5x = 1,
    #[doc = "2: 0.75x output driver selected"]
    _0p75x = 2,
    #[doc = "3: 1.0x output driver selected"]
    _1p0x = 3,
}
impl From<Ds75> for u8 {
    #[inline(always)]
    fn from(variant: Ds75) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ds75 {
    type Ux = u8;
}
impl crate::IsEnum for Ds75 {}
#[doc = "Field `DS75` reader - Drive strength selection for GPIO 75"]
pub type Ds75R = crate::FieldReader<Ds75>;
impl Ds75R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ds75 {
        match self.bits {
            0 => Ds75::_0p1x,
            1 => Ds75::_0p5x,
            2 => Ds75::_0p75x,
            3 => Ds75::_1p0x,
            _ => unreachable!(),
        }
    }
    #[doc = "0.1x output driver selected"]
    #[inline(always)]
    pub fn is_0p1x(&self) -> bool {
        *self == Ds75::_0p1x
    }
    #[doc = "0.5x output driver selected"]
    #[inline(always)]
    pub fn is_0p5x(&self) -> bool {
        *self == Ds75::_0p5x
    }
    #[doc = "0.75x output driver selected"]
    #[inline(always)]
    pub fn is_0p75x(&self) -> bool {
        *self == Ds75::_0p75x
    }
    #[doc = "1.0x output driver selected"]
    #[inline(always)]
    pub fn is_1p0x(&self) -> bool {
        *self == Ds75::_1p0x
    }
}
#[doc = "Field `DS75` writer - Drive strength selection for GPIO 75"]
pub type Ds75W<'a, REG> = crate::FieldWriter<'a, REG, 2, Ds75, crate::Safe>;
impl<'a, REG> Ds75W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "0.1x output driver selected"]
    #[inline(always)]
    pub fn _0p1x(self) -> &'a mut crate::W<REG> {
        self.variant(Ds75::_0p1x)
    }
    #[doc = "0.5x output driver selected"]
    #[inline(always)]
    pub fn _0p5x(self) -> &'a mut crate::W<REG> {
        self.variant(Ds75::_0p5x)
    }
    #[doc = "0.75x output driver selected"]
    #[inline(always)]
    pub fn _0p75x(self) -> &'a mut crate::W<REG> {
        self.variant(Ds75::_0p75x)
    }
    #[doc = "1.0x output driver selected"]
    #[inline(always)]
    pub fn _1p0x(self) -> &'a mut crate::W<REG> {
        self.variant(Ds75::_1p0x)
    }
}
#[doc = "Field `SR75` reader - Configure the slew rate"]
pub type Sr75R = crate::BitReader;
#[doc = "Field `SR75` writer - Configure the slew rate"]
pub type Sr75W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Pullup/Pulldown configuration for GPIO 75\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pullcfg75 {
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
impl From<Pullcfg75> for u8 {
    #[inline(always)]
    fn from(variant: Pullcfg75) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pullcfg75 {
    type Ux = u8;
}
impl crate::IsEnum for Pullcfg75 {}
#[doc = "Field `PULLCFG75` reader - Pullup/Pulldown configuration for GPIO 75"]
pub type Pullcfg75R = crate::FieldReader<Pullcfg75>;
impl Pullcfg75R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pullcfg75 {
        match self.bits {
            0 => Pullcfg75::Dis,
            1 => Pullcfg75::Pd50k,
            2 => Pullcfg75::Pu15k,
            3 => Pullcfg75::Pu6k,
            4 => Pullcfg75::Pu12k,
            5 => Pullcfg75::Pu24k,
            6 => Pullcfg75::Pu50k,
            7 => Pullcfg75::Pu100k,
            _ => unreachable!(),
        }
    }
    #[doc = "No pullup or pulldown selected"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Pullcfg75::Dis
    }
    #[doc = "50K Pulldown selected"]
    #[inline(always)]
    pub fn is_pd50k(&self) -> bool {
        *self == Pullcfg75::Pd50k
    }
    #[doc = "1.5K Pullup selected"]
    #[inline(always)]
    pub fn is_pu15k(&self) -> bool {
        *self == Pullcfg75::Pu15k
    }
    #[doc = "6K Pullup selected"]
    #[inline(always)]
    pub fn is_pu6k(&self) -> bool {
        *self == Pullcfg75::Pu6k
    }
    #[doc = "12K Pullup selected"]
    #[inline(always)]
    pub fn is_pu12k(&self) -> bool {
        *self == Pullcfg75::Pu12k
    }
    #[doc = "24K Pullup selected"]
    #[inline(always)]
    pub fn is_pu24k(&self) -> bool {
        *self == Pullcfg75::Pu24k
    }
    #[doc = "50K Pullup selected"]
    #[inline(always)]
    pub fn is_pu50k(&self) -> bool {
        *self == Pullcfg75::Pu50k
    }
    #[doc = "100K Pullup selected"]
    #[inline(always)]
    pub fn is_pu100k(&self) -> bool {
        *self == Pullcfg75::Pu100k
    }
}
#[doc = "Field `PULLCFG75` writer - Pullup/Pulldown configuration for GPIO 75"]
pub type Pullcfg75W<'a, REG> = crate::FieldWriter<'a, REG, 3, Pullcfg75, crate::Safe>;
impl<'a, REG> Pullcfg75W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No pullup or pulldown selected"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg75::Dis)
    }
    #[doc = "50K Pulldown selected"]
    #[inline(always)]
    pub fn pd50k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg75::Pd50k)
    }
    #[doc = "1.5K Pullup selected"]
    #[inline(always)]
    pub fn pu15k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg75::Pu15k)
    }
    #[doc = "6K Pullup selected"]
    #[inline(always)]
    pub fn pu6k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg75::Pu6k)
    }
    #[doc = "12K Pullup selected"]
    #[inline(always)]
    pub fn pu12k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg75::Pu12k)
    }
    #[doc = "24K Pullup selected"]
    #[inline(always)]
    pub fn pu24k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg75::Pu24k)
    }
    #[doc = "50K Pullup selected"]
    #[inline(always)]
    pub fn pu50k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg75::Pu50k)
    }
    #[doc = "100K Pullup selected"]
    #[inline(always)]
    pub fn pu100k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg75::Pu100k)
    }
}
#[doc = "IOMSTR/MSPI N Chip Select 75, DISP control signals DE, CSX, and CS. Polarity is determined by CE_POLARITY field\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ncesrc75 {
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
impl From<Ncesrc75> for u8 {
    #[inline(always)]
    fn from(variant: Ncesrc75) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ncesrc75 {
    type Ux = u8;
}
impl crate::IsEnum for Ncesrc75 {}
#[doc = "Field `NCESRC75` reader - IOMSTR/MSPI N Chip Select 75, DISP control signals DE, CSX, and CS. Polarity is determined by CE_POLARITY field"]
pub type Ncesrc75R = crate::FieldReader<Ncesrc75>;
impl Ncesrc75R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ncesrc75> {
        match self.bits {
            0 => Some(Ncesrc75::Iom0ce0),
            1 => Some(Ncesrc75::Iom0ce1),
            2 => Some(Ncesrc75::Iom0ce2),
            3 => Some(Ncesrc75::Iom0ce3),
            4 => Some(Ncesrc75::Iom1ce0),
            5 => Some(Ncesrc75::Iom1ce1),
            6 => Some(Ncesrc75::Iom1ce2),
            7 => Some(Ncesrc75::Iom1ce3),
            8 => Some(Ncesrc75::Iom2ce0),
            9 => Some(Ncesrc75::Iom2ce1),
            10 => Some(Ncesrc75::Iom2ce2),
            11 => Some(Ncesrc75::Iom2ce3),
            12 => Some(Ncesrc75::Iom3ce0),
            13 => Some(Ncesrc75::Iom3ce1),
            14 => Some(Ncesrc75::Iom3ce2),
            15 => Some(Ncesrc75::Iom3ce3),
            16 => Some(Ncesrc75::Iom4ce0),
            17 => Some(Ncesrc75::Iom4ce1),
            18 => Some(Ncesrc75::Iom4ce2),
            19 => Some(Ncesrc75::Iom4ce3),
            20 => Some(Ncesrc75::Iom5ce0),
            21 => Some(Ncesrc75::Iom5ce1),
            22 => Some(Ncesrc75::Iom5ce2),
            23 => Some(Ncesrc75::Iom5ce3),
            24 => Some(Ncesrc75::Iom6ce0),
            25 => Some(Ncesrc75::Iom6ce1),
            26 => Some(Ncesrc75::Iom6ce2),
            27 => Some(Ncesrc75::Iom6ce3),
            28 => Some(Ncesrc75::Iom7ce0),
            29 => Some(Ncesrc75::Iom7ce1),
            30 => Some(Ncesrc75::Iom7ce2),
            31 => Some(Ncesrc75::Iom7ce3),
            32 => Some(Ncesrc75::Mspi0cen0),
            33 => Some(Ncesrc75::Mspi0cen1),
            34 => Some(Ncesrc75::Mspi1cen0),
            35 => Some(Ncesrc75::Mspi1cen1),
            36 => Some(Ncesrc75::Mspi2cen0),
            37 => Some(Ncesrc75::Mspi2cen1),
            38 => Some(Ncesrc75::DcDpiDe),
            39 => Some(Ncesrc75::DispContCsx),
            40 => Some(Ncesrc75::DcSpiCsN),
            41 => Some(Ncesrc75::DcQspiCsN),
            42 => Some(Ncesrc75::DcResx),
            _ => None,
        }
    }
    #[doc = "IOM 0 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom0ce0(&self) -> bool {
        *self == Ncesrc75::Iom0ce0
    }
    #[doc = "IOM 0 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom0ce1(&self) -> bool {
        *self == Ncesrc75::Iom0ce1
    }
    #[doc = "IOM 0 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom0ce2(&self) -> bool {
        *self == Ncesrc75::Iom0ce2
    }
    #[doc = "IOM 0 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom0ce3(&self) -> bool {
        *self == Ncesrc75::Iom0ce3
    }
    #[doc = "IOM 1 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom1ce0(&self) -> bool {
        *self == Ncesrc75::Iom1ce0
    }
    #[doc = "IOM 1 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom1ce1(&self) -> bool {
        *self == Ncesrc75::Iom1ce1
    }
    #[doc = "IOM 1 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom1ce2(&self) -> bool {
        *self == Ncesrc75::Iom1ce2
    }
    #[doc = "IOM 1 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom1ce3(&self) -> bool {
        *self == Ncesrc75::Iom1ce3
    }
    #[doc = "IOM 2 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom2ce0(&self) -> bool {
        *self == Ncesrc75::Iom2ce0
    }
    #[doc = "IOM 2 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom2ce1(&self) -> bool {
        *self == Ncesrc75::Iom2ce1
    }
    #[doc = "IOM 2 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom2ce2(&self) -> bool {
        *self == Ncesrc75::Iom2ce2
    }
    #[doc = "IOM 2 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom2ce3(&self) -> bool {
        *self == Ncesrc75::Iom2ce3
    }
    #[doc = "IOM 3 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom3ce0(&self) -> bool {
        *self == Ncesrc75::Iom3ce0
    }
    #[doc = "IOM 3 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom3ce1(&self) -> bool {
        *self == Ncesrc75::Iom3ce1
    }
    #[doc = "IOM 3 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom3ce2(&self) -> bool {
        *self == Ncesrc75::Iom3ce2
    }
    #[doc = "IOM 3 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom3ce3(&self) -> bool {
        *self == Ncesrc75::Iom3ce3
    }
    #[doc = "IOM 4 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom4ce0(&self) -> bool {
        *self == Ncesrc75::Iom4ce0
    }
    #[doc = "IOM 4 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom4ce1(&self) -> bool {
        *self == Ncesrc75::Iom4ce1
    }
    #[doc = "IOM 4 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom4ce2(&self) -> bool {
        *self == Ncesrc75::Iom4ce2
    }
    #[doc = "IOM 4 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom4ce3(&self) -> bool {
        *self == Ncesrc75::Iom4ce3
    }
    #[doc = "IOM 5 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom5ce0(&self) -> bool {
        *self == Ncesrc75::Iom5ce0
    }
    #[doc = "IOM 5 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom5ce1(&self) -> bool {
        *self == Ncesrc75::Iom5ce1
    }
    #[doc = "IOM 5 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom5ce2(&self) -> bool {
        *self == Ncesrc75::Iom5ce2
    }
    #[doc = "IOM 5 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom5ce3(&self) -> bool {
        *self == Ncesrc75::Iom5ce3
    }
    #[doc = "IOM 6 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom6ce0(&self) -> bool {
        *self == Ncesrc75::Iom6ce0
    }
    #[doc = "IOM 6 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom6ce1(&self) -> bool {
        *self == Ncesrc75::Iom6ce1
    }
    #[doc = "IOM 6 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom6ce2(&self) -> bool {
        *self == Ncesrc75::Iom6ce2
    }
    #[doc = "IOM 6 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom6ce3(&self) -> bool {
        *self == Ncesrc75::Iom6ce3
    }
    #[doc = "IOM 7 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom7ce0(&self) -> bool {
        *self == Ncesrc75::Iom7ce0
    }
    #[doc = "IOM 7 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom7ce1(&self) -> bool {
        *self == Ncesrc75::Iom7ce1
    }
    #[doc = "IOM 7 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom7ce2(&self) -> bool {
        *self == Ncesrc75::Iom7ce2
    }
    #[doc = "IOM 7 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom7ce3(&self) -> bool {
        *self == Ncesrc75::Iom7ce3
    }
    #[doc = "MSPI 0 NCE 0 module"]
    #[inline(always)]
    pub fn is_mspi0cen0(&self) -> bool {
        *self == Ncesrc75::Mspi0cen0
    }
    #[doc = "MSPI 0 NCE 1 module"]
    #[inline(always)]
    pub fn is_mspi0cen1(&self) -> bool {
        *self == Ncesrc75::Mspi0cen1
    }
    #[doc = "MSPI 1 NCE 0 module"]
    #[inline(always)]
    pub fn is_mspi1cen0(&self) -> bool {
        *self == Ncesrc75::Mspi1cen0
    }
    #[doc = "MSPI 1 NCE 1 module"]
    #[inline(always)]
    pub fn is_mspi1cen1(&self) -> bool {
        *self == Ncesrc75::Mspi1cen1
    }
    #[doc = "MSPI 2 NCE 0 module"]
    #[inline(always)]
    pub fn is_mspi2cen0(&self) -> bool {
        *self == Ncesrc75::Mspi2cen0
    }
    #[doc = "MSPI 2 NCE 1 module"]
    #[inline(always)]
    pub fn is_mspi2cen1(&self) -> bool {
        *self == Ncesrc75::Mspi2cen1
    }
    #[doc = "DC DPI DE module"]
    #[inline(always)]
    pub fn is_dc_dpi_de(&self) -> bool {
        *self == Ncesrc75::DcDpiDe
    }
    #[doc = "DISP CONT CSX module"]
    #[inline(always)]
    pub fn is_disp_cont_csx(&self) -> bool {
        *self == Ncesrc75::DispContCsx
    }
    #[doc = "DC SPI CS_N module"]
    #[inline(always)]
    pub fn is_dc_spi_cs_n(&self) -> bool {
        *self == Ncesrc75::DcSpiCsN
    }
    #[doc = "DC QSPI CS_N module"]
    #[inline(always)]
    pub fn is_dc_qspi_cs_n(&self) -> bool {
        *self == Ncesrc75::DcQspiCsN
    }
    #[doc = "DC module RESX"]
    #[inline(always)]
    pub fn is_dc_resx(&self) -> bool {
        *self == Ncesrc75::DcResx
    }
}
#[doc = "Field `NCESRC75` writer - IOMSTR/MSPI N Chip Select 75, DISP control signals DE, CSX, and CS. Polarity is determined by CE_POLARITY field"]
pub type Ncesrc75W<'a, REG> = crate::FieldWriter<'a, REG, 6, Ncesrc75>;
impl<'a, REG> Ncesrc75W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "IOM 0 NCE 0 module"]
    #[inline(always)]
    pub fn iom0ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc75::Iom0ce0)
    }
    #[doc = "IOM 0 NCE 1 module"]
    #[inline(always)]
    pub fn iom0ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc75::Iom0ce1)
    }
    #[doc = "IOM 0 NCE 2 module"]
    #[inline(always)]
    pub fn iom0ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc75::Iom0ce2)
    }
    #[doc = "IOM 0 NCE 3 module"]
    #[inline(always)]
    pub fn iom0ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc75::Iom0ce3)
    }
    #[doc = "IOM 1 NCE 0 module"]
    #[inline(always)]
    pub fn iom1ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc75::Iom1ce0)
    }
    #[doc = "IOM 1 NCE 1 module"]
    #[inline(always)]
    pub fn iom1ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc75::Iom1ce1)
    }
    #[doc = "IOM 1 NCE 2 module"]
    #[inline(always)]
    pub fn iom1ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc75::Iom1ce2)
    }
    #[doc = "IOM 1 NCE 3 module"]
    #[inline(always)]
    pub fn iom1ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc75::Iom1ce3)
    }
    #[doc = "IOM 2 NCE 0 module"]
    #[inline(always)]
    pub fn iom2ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc75::Iom2ce0)
    }
    #[doc = "IOM 2 NCE 1 module"]
    #[inline(always)]
    pub fn iom2ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc75::Iom2ce1)
    }
    #[doc = "IOM 2 NCE 2 module"]
    #[inline(always)]
    pub fn iom2ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc75::Iom2ce2)
    }
    #[doc = "IOM 2 NCE 3 module"]
    #[inline(always)]
    pub fn iom2ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc75::Iom2ce3)
    }
    #[doc = "IOM 3 NCE 0 module"]
    #[inline(always)]
    pub fn iom3ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc75::Iom3ce0)
    }
    #[doc = "IOM 3 NCE 1 module"]
    #[inline(always)]
    pub fn iom3ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc75::Iom3ce1)
    }
    #[doc = "IOM 3 NCE 2 module"]
    #[inline(always)]
    pub fn iom3ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc75::Iom3ce2)
    }
    #[doc = "IOM 3 NCE 3 module"]
    #[inline(always)]
    pub fn iom3ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc75::Iom3ce3)
    }
    #[doc = "IOM 4 NCE 0 module"]
    #[inline(always)]
    pub fn iom4ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc75::Iom4ce0)
    }
    #[doc = "IOM 4 NCE 1 module"]
    #[inline(always)]
    pub fn iom4ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc75::Iom4ce1)
    }
    #[doc = "IOM 4 NCE 2 module"]
    #[inline(always)]
    pub fn iom4ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc75::Iom4ce2)
    }
    #[doc = "IOM 4 NCE 3 module"]
    #[inline(always)]
    pub fn iom4ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc75::Iom4ce3)
    }
    #[doc = "IOM 5 NCE 0 module"]
    #[inline(always)]
    pub fn iom5ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc75::Iom5ce0)
    }
    #[doc = "IOM 5 NCE 1 module"]
    #[inline(always)]
    pub fn iom5ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc75::Iom5ce1)
    }
    #[doc = "IOM 5 NCE 2 module"]
    #[inline(always)]
    pub fn iom5ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc75::Iom5ce2)
    }
    #[doc = "IOM 5 NCE 3 module"]
    #[inline(always)]
    pub fn iom5ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc75::Iom5ce3)
    }
    #[doc = "IOM 6 NCE 0 module"]
    #[inline(always)]
    pub fn iom6ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc75::Iom6ce0)
    }
    #[doc = "IOM 6 NCE 1 module"]
    #[inline(always)]
    pub fn iom6ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc75::Iom6ce1)
    }
    #[doc = "IOM 6 NCE 2 module"]
    #[inline(always)]
    pub fn iom6ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc75::Iom6ce2)
    }
    #[doc = "IOM 6 NCE 3 module"]
    #[inline(always)]
    pub fn iom6ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc75::Iom6ce3)
    }
    #[doc = "IOM 7 NCE 0 module"]
    #[inline(always)]
    pub fn iom7ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc75::Iom7ce0)
    }
    #[doc = "IOM 7 NCE 1 module"]
    #[inline(always)]
    pub fn iom7ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc75::Iom7ce1)
    }
    #[doc = "IOM 7 NCE 2 module"]
    #[inline(always)]
    pub fn iom7ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc75::Iom7ce2)
    }
    #[doc = "IOM 7 NCE 3 module"]
    #[inline(always)]
    pub fn iom7ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc75::Iom7ce3)
    }
    #[doc = "MSPI 0 NCE 0 module"]
    #[inline(always)]
    pub fn mspi0cen0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc75::Mspi0cen0)
    }
    #[doc = "MSPI 0 NCE 1 module"]
    #[inline(always)]
    pub fn mspi0cen1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc75::Mspi0cen1)
    }
    #[doc = "MSPI 1 NCE 0 module"]
    #[inline(always)]
    pub fn mspi1cen0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc75::Mspi1cen0)
    }
    #[doc = "MSPI 1 NCE 1 module"]
    #[inline(always)]
    pub fn mspi1cen1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc75::Mspi1cen1)
    }
    #[doc = "MSPI 2 NCE 0 module"]
    #[inline(always)]
    pub fn mspi2cen0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc75::Mspi2cen0)
    }
    #[doc = "MSPI 2 NCE 1 module"]
    #[inline(always)]
    pub fn mspi2cen1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc75::Mspi2cen1)
    }
    #[doc = "DC DPI DE module"]
    #[inline(always)]
    pub fn dc_dpi_de(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc75::DcDpiDe)
    }
    #[doc = "DISP CONT CSX module"]
    #[inline(always)]
    pub fn disp_cont_csx(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc75::DispContCsx)
    }
    #[doc = "DC SPI CS_N module"]
    #[inline(always)]
    pub fn dc_spi_cs_n(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc75::DcSpiCsN)
    }
    #[doc = "DC QSPI CS_N module"]
    #[inline(always)]
    pub fn dc_qspi_cs_n(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc75::DcQspiCsN)
    }
    #[doc = "DC module RESX"]
    #[inline(always)]
    pub fn dc_resx(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc75::DcResx)
    }
}
#[doc = "Polarity select for NCE for GPIO 75\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ncepol75 {
    #[doc = "0: Polarity is active low"]
    Low = 0,
    #[doc = "1: Polarity is active high"]
    High = 1,
}
impl From<Ncepol75> for bool {
    #[inline(always)]
    fn from(variant: Ncepol75) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NCEPOL75` reader - Polarity select for NCE for GPIO 75"]
pub type Ncepol75R = crate::BitReader<Ncepol75>;
impl Ncepol75R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ncepol75 {
        match self.bits {
            false => Ncepol75::Low,
            true => Ncepol75::High,
        }
    }
    #[doc = "Polarity is active low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Ncepol75::Low
    }
    #[doc = "Polarity is active high"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Ncepol75::High
    }
}
#[doc = "Field `NCEPOL75` writer - Polarity select for NCE for GPIO 75"]
pub type Ncepol75W<'a, REG> = crate::BitWriter<'a, REG, Ncepol75>;
impl<'a, REG> Ncepol75W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Polarity is active low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Ncepol75::Low)
    }
    #[doc = "Polarity is active high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Ncepol75::High)
    }
}
#[doc = "Field `FIEN75` reader - Force input enable active regardless of function selected. Otherwise the selected function will enable the input only when needed"]
pub type Fien75R = crate::BitReader;
#[doc = "Field `FIEN75` writer - Force input enable active regardless of function selected. Otherwise the selected function will enable the input only when needed"]
pub type Fien75W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FOEN75` reader - Force output enable active regardless of function selected. Otherwise the selected function will enable the output only when needed"]
pub type Foen75R = crate::BitReader;
#[doc = "Field `FOEN75` writer - Force output enable active regardless of function selected. Otherwise the selected function will enable the output only when needed"]
pub type Foen75W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Function select for GPIO pin 75"]
    #[inline(always)]
    pub fn fncsel75(&self) -> Fncsel75R {
        Fncsel75R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Input enable for GPIO 75"]
    #[inline(always)]
    pub fn inpen75(&self) -> Inpen75R {
        Inpen75R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Return 0 for read data on GPIO 75"]
    #[inline(always)]
    pub fn rdzero75(&self) -> Rdzero75R {
        Rdzero75R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Interrupt enable for GPIO 75"]
    #[inline(always)]
    pub fn irpten75(&self) -> Irpten75R {
        Irpten75R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Pin IO mode selection for GPIO pin 75"]
    #[inline(always)]
    pub fn outcfg75(&self) -> Outcfg75R {
        Outcfg75R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Drive strength selection for GPIO 75"]
    #[inline(always)]
    pub fn ds75(&self) -> Ds75R {
        Ds75R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - Configure the slew rate"]
    #[inline(always)]
    pub fn sr75(&self) -> Sr75R {
        Sr75R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - Pullup/Pulldown configuration for GPIO 75"]
    #[inline(always)]
    pub fn pullcfg75(&self) -> Pullcfg75R {
        Pullcfg75R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:21 - IOMSTR/MSPI N Chip Select 75, DISP control signals DE, CSX, and CS. Polarity is determined by CE_POLARITY field"]
    #[inline(always)]
    pub fn ncesrc75(&self) -> Ncesrc75R {
        Ncesrc75R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 22 - Polarity select for NCE for GPIO 75"]
    #[inline(always)]
    pub fn ncepol75(&self) -> Ncepol75R {
        Ncepol75R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 26 - Force input enable active regardless of function selected. Otherwise the selected function will enable the input only when needed"]
    #[inline(always)]
    pub fn fien75(&self) -> Fien75R {
        Fien75R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Force output enable active regardless of function selected. Otherwise the selected function will enable the output only when needed"]
    #[inline(always)]
    pub fn foen75(&self) -> Foen75R {
        Foen75R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Function select for GPIO pin 75"]
    #[inline(always)]
    #[must_use]
    pub fn fncsel75(&mut self) -> Fncsel75W<Pincfg75Spec> {
        Fncsel75W::new(self, 0)
    }
    #[doc = "Bit 4 - Input enable for GPIO 75"]
    #[inline(always)]
    #[must_use]
    pub fn inpen75(&mut self) -> Inpen75W<Pincfg75Spec> {
        Inpen75W::new(self, 4)
    }
    #[doc = "Bit 5 - Return 0 for read data on GPIO 75"]
    #[inline(always)]
    #[must_use]
    pub fn rdzero75(&mut self) -> Rdzero75W<Pincfg75Spec> {
        Rdzero75W::new(self, 5)
    }
    #[doc = "Bits 6:7 - Interrupt enable for GPIO 75"]
    #[inline(always)]
    #[must_use]
    pub fn irpten75(&mut self) -> Irpten75W<Pincfg75Spec> {
        Irpten75W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Pin IO mode selection for GPIO pin 75"]
    #[inline(always)]
    #[must_use]
    pub fn outcfg75(&mut self) -> Outcfg75W<Pincfg75Spec> {
        Outcfg75W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Drive strength selection for GPIO 75"]
    #[inline(always)]
    #[must_use]
    pub fn ds75(&mut self) -> Ds75W<Pincfg75Spec> {
        Ds75W::new(self, 10)
    }
    #[doc = "Bit 12 - Configure the slew rate"]
    #[inline(always)]
    #[must_use]
    pub fn sr75(&mut self) -> Sr75W<Pincfg75Spec> {
        Sr75W::new(self, 12)
    }
    #[doc = "Bits 13:15 - Pullup/Pulldown configuration for GPIO 75"]
    #[inline(always)]
    #[must_use]
    pub fn pullcfg75(&mut self) -> Pullcfg75W<Pincfg75Spec> {
        Pullcfg75W::new(self, 13)
    }
    #[doc = "Bits 16:21 - IOMSTR/MSPI N Chip Select 75, DISP control signals DE, CSX, and CS. Polarity is determined by CE_POLARITY field"]
    #[inline(always)]
    #[must_use]
    pub fn ncesrc75(&mut self) -> Ncesrc75W<Pincfg75Spec> {
        Ncesrc75W::new(self, 16)
    }
    #[doc = "Bit 22 - Polarity select for NCE for GPIO 75"]
    #[inline(always)]
    #[must_use]
    pub fn ncepol75(&mut self) -> Ncepol75W<Pincfg75Spec> {
        Ncepol75W::new(self, 22)
    }
    #[doc = "Bit 26 - Force input enable active regardless of function selected. Otherwise the selected function will enable the input only when needed"]
    #[inline(always)]
    #[must_use]
    pub fn fien75(&mut self) -> Fien75W<Pincfg75Spec> {
        Fien75W::new(self, 26)
    }
    #[doc = "Bit 27 - Force output enable active regardless of function selected. Otherwise the selected function will enable the output only when needed"]
    #[inline(always)]
    #[must_use]
    pub fn foen75(&mut self) -> Foen75W<Pincfg75Spec> {
        Foen75W::new(self, 27)
    }
}
#[doc = "Controls the operation of GPIO pin 75.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg75::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg75::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pincfg75Spec;
impl crate::RegisterSpec for Pincfg75Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pincfg75::R`](R) reader structure"]
impl crate::Readable for Pincfg75Spec {}
#[doc = "`write(|w| ..)` method takes [`pincfg75::W`](W) writer structure"]
impl crate::Writable for Pincfg75Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PINCFG75 to value 0x03"]
impl crate::Resettable for Pincfg75Spec {
    const RESET_VALUE: u32 = 0x03;
}
