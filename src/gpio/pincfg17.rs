#[doc = "Register `PINCFG17` reader"]
pub type R = crate::R<Pincfg17Spec>;
#[doc = "Register `PINCFG17` writer"]
pub type W = crate::W<Pincfg17Spec>;
#[doc = "Function select for GPIO pin 17\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fncsel17 {
    #[doc = "0: Analog to Digital Converter SE IN2"]
    Adcse2 = 0,
    #[doc = "1: ADC trigger input"]
    Trig2 = 1,
    #[doc = "2: Bidirectional I2S Data. Operates in output mode in master mode and input mode for slave mode. (I2S Master/Slave 2)"]
    I2s1Data = 2,
    #[doc = "3: General purpose I/O"]
    Gpio = 3,
    #[doc = "4: MILLI Playback Data2"]
    MilliPbdata2 = 4,
    #[doc = "5: UART Request to Send (RTS) (UART 3)"]
    Uart3rts = 5,
    #[doc = "6: Timer/Counter input or output; Selection of direction is done via CTIMER register settings."]
    Ct17 = 6,
    #[doc = "7: IOMSTR/MSPI N Chip Select. Polarity is determined by CE_POLARITY field"]
    Nce17 = 7,
    #[doc = "8: Observation bus bit 1"]
    Obsbus1 = 8,
    #[doc = "9: I2S Data output (I2S Master/Slave 2)"]
    I2s1Sdout = 9,
    #[doc = "10: Reserved selection. Operation unknown if selected."]
    Reserved10 = 10,
    #[doc = "11: Fast PIO"]
    Fpio = 11,
    #[doc = "12: Reserved selection. Operation unknown if selected."]
    Reserved12 = 12,
    #[doc = "13: Internal function (Flash parallel load)"]
    FlloadStrb = 13,
    #[doc = "14: Internal function (MBIST)"]
    MdaTms = 14,
    #[doc = "15: Internal function (SCAN)"]
    OpcgClk = 15,
}
impl From<Fncsel17> for u8 {
    #[inline(always)]
    fn from(variant: Fncsel17) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fncsel17 {
    type Ux = u8;
}
impl crate::IsEnum for Fncsel17 {}
#[doc = "Field `FNCSEL17` reader - Function select for GPIO pin 17"]
pub type Fncsel17R = crate::FieldReader<Fncsel17>;
impl Fncsel17R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fncsel17 {
        match self.bits {
            0 => Fncsel17::Adcse2,
            1 => Fncsel17::Trig2,
            2 => Fncsel17::I2s1Data,
            3 => Fncsel17::Gpio,
            4 => Fncsel17::MilliPbdata2,
            5 => Fncsel17::Uart3rts,
            6 => Fncsel17::Ct17,
            7 => Fncsel17::Nce17,
            8 => Fncsel17::Obsbus1,
            9 => Fncsel17::I2s1Sdout,
            10 => Fncsel17::Reserved10,
            11 => Fncsel17::Fpio,
            12 => Fncsel17::Reserved12,
            13 => Fncsel17::FlloadStrb,
            14 => Fncsel17::MdaTms,
            15 => Fncsel17::OpcgClk,
            _ => unreachable!(),
        }
    }
    #[doc = "Analog to Digital Converter SE IN2"]
    #[inline(always)]
    pub fn is_adcse2(&self) -> bool {
        *self == Fncsel17::Adcse2
    }
    #[doc = "ADC trigger input"]
    #[inline(always)]
    pub fn is_trig2(&self) -> bool {
        *self == Fncsel17::Trig2
    }
    #[doc = "Bidirectional I2S Data. Operates in output mode in master mode and input mode for slave mode. (I2S Master/Slave 2)"]
    #[inline(always)]
    pub fn is_i2s1_data(&self) -> bool {
        *self == Fncsel17::I2s1Data
    }
    #[doc = "General purpose I/O"]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == Fncsel17::Gpio
    }
    #[doc = "MILLI Playback Data2"]
    #[inline(always)]
    pub fn is_milli_pbdata2(&self) -> bool {
        *self == Fncsel17::MilliPbdata2
    }
    #[doc = "UART Request to Send (RTS) (UART 3)"]
    #[inline(always)]
    pub fn is_uart3rts(&self) -> bool {
        *self == Fncsel17::Uart3rts
    }
    #[doc = "Timer/Counter input or output; Selection of direction is done via CTIMER register settings."]
    #[inline(always)]
    pub fn is_ct17(&self) -> bool {
        *self == Fncsel17::Ct17
    }
    #[doc = "IOMSTR/MSPI N Chip Select. Polarity is determined by CE_POLARITY field"]
    #[inline(always)]
    pub fn is_nce17(&self) -> bool {
        *self == Fncsel17::Nce17
    }
    #[doc = "Observation bus bit 1"]
    #[inline(always)]
    pub fn is_obsbus1(&self) -> bool {
        *self == Fncsel17::Obsbus1
    }
    #[doc = "I2S Data output (I2S Master/Slave 2)"]
    #[inline(always)]
    pub fn is_i2s1_sdout(&self) -> bool {
        *self == Fncsel17::I2s1Sdout
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved10(&self) -> bool {
        *self == Fncsel17::Reserved10
    }
    #[doc = "Fast PIO"]
    #[inline(always)]
    pub fn is_fpio(&self) -> bool {
        *self == Fncsel17::Fpio
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved12(&self) -> bool {
        *self == Fncsel17::Reserved12
    }
    #[doc = "Internal function (Flash parallel load)"]
    #[inline(always)]
    pub fn is_flload_strb(&self) -> bool {
        *self == Fncsel17::FlloadStrb
    }
    #[doc = "Internal function (MBIST)"]
    #[inline(always)]
    pub fn is_mda_tms(&self) -> bool {
        *self == Fncsel17::MdaTms
    }
    #[doc = "Internal function (SCAN)"]
    #[inline(always)]
    pub fn is_opcg_clk(&self) -> bool {
        *self == Fncsel17::OpcgClk
    }
}
#[doc = "Field `FNCSEL17` writer - Function select for GPIO pin 17"]
pub type Fncsel17W<'a, REG> = crate::FieldWriter<'a, REG, 4, Fncsel17, crate::Safe>;
impl<'a, REG> Fncsel17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Analog to Digital Converter SE IN2"]
    #[inline(always)]
    pub fn adcse2(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel17::Adcse2)
    }
    #[doc = "ADC trigger input"]
    #[inline(always)]
    pub fn trig2(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel17::Trig2)
    }
    #[doc = "Bidirectional I2S Data. Operates in output mode in master mode and input mode for slave mode. (I2S Master/Slave 2)"]
    #[inline(always)]
    pub fn i2s1_data(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel17::I2s1Data)
    }
    #[doc = "General purpose I/O"]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel17::Gpio)
    }
    #[doc = "MILLI Playback Data2"]
    #[inline(always)]
    pub fn milli_pbdata2(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel17::MilliPbdata2)
    }
    #[doc = "UART Request to Send (RTS) (UART 3)"]
    #[inline(always)]
    pub fn uart3rts(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel17::Uart3rts)
    }
    #[doc = "Timer/Counter input or output; Selection of direction is done via CTIMER register settings."]
    #[inline(always)]
    pub fn ct17(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel17::Ct17)
    }
    #[doc = "IOMSTR/MSPI N Chip Select. Polarity is determined by CE_POLARITY field"]
    #[inline(always)]
    pub fn nce17(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel17::Nce17)
    }
    #[doc = "Observation bus bit 1"]
    #[inline(always)]
    pub fn obsbus1(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel17::Obsbus1)
    }
    #[doc = "I2S Data output (I2S Master/Slave 2)"]
    #[inline(always)]
    pub fn i2s1_sdout(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel17::I2s1Sdout)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved10(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel17::Reserved10)
    }
    #[doc = "Fast PIO"]
    #[inline(always)]
    pub fn fpio(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel17::Fpio)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved12(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel17::Reserved12)
    }
    #[doc = "Internal function (Flash parallel load)"]
    #[inline(always)]
    pub fn flload_strb(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel17::FlloadStrb)
    }
    #[doc = "Internal function (MBIST)"]
    #[inline(always)]
    pub fn mda_tms(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel17::MdaTms)
    }
    #[doc = "Internal function (SCAN)"]
    #[inline(always)]
    pub fn opcg_clk(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel17::OpcgClk)
    }
}
#[doc = "Field `INPEN17` reader - Input enable for GPIO 17"]
pub type Inpen17R = crate::BitReader;
#[doc = "Field `INPEN17` writer - Input enable for GPIO 17"]
pub type Inpen17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDZERO17` reader - Return 0 for read data on GPIO 17"]
pub type Rdzero17R = crate::BitReader;
#[doc = "Field `RDZERO17` writer - Return 0 for read data on GPIO 17"]
pub type Rdzero17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Interrupt enable for GPIO 17\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Irpten17 {
    #[doc = "0: Interrupts are disabled for this GPIO"]
    Dis = 0,
    #[doc = "1: Interrupts are enabled for falling edge transition on this GPIO"]
    Intfall = 1,
    #[doc = "2: Interrupts are enabled for rising edge transitions on this GPIO"]
    Intrise = 2,
    #[doc = "3: Interrupts are enabled for any edge transition on this GPIO"]
    Intany = 3,
}
impl From<Irpten17> for u8 {
    #[inline(always)]
    fn from(variant: Irpten17) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Irpten17 {
    type Ux = u8;
}
impl crate::IsEnum for Irpten17 {}
#[doc = "Field `IRPTEN17` reader - Interrupt enable for GPIO 17"]
pub type Irpten17R = crate::FieldReader<Irpten17>;
impl Irpten17R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irpten17 {
        match self.bits {
            0 => Irpten17::Dis,
            1 => Irpten17::Intfall,
            2 => Irpten17::Intrise,
            3 => Irpten17::Intany,
            _ => unreachable!(),
        }
    }
    #[doc = "Interrupts are disabled for this GPIO"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Irpten17::Dis
    }
    #[doc = "Interrupts are enabled for falling edge transition on this GPIO"]
    #[inline(always)]
    pub fn is_intfall(&self) -> bool {
        *self == Irpten17::Intfall
    }
    #[doc = "Interrupts are enabled for rising edge transitions on this GPIO"]
    #[inline(always)]
    pub fn is_intrise(&self) -> bool {
        *self == Irpten17::Intrise
    }
    #[doc = "Interrupts are enabled for any edge transition on this GPIO"]
    #[inline(always)]
    pub fn is_intany(&self) -> bool {
        *self == Irpten17::Intany
    }
}
#[doc = "Field `IRPTEN17` writer - Interrupt enable for GPIO 17"]
pub type Irpten17W<'a, REG> = crate::FieldWriter<'a, REG, 2, Irpten17, crate::Safe>;
impl<'a, REG> Irpten17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Interrupts are disabled for this GPIO"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten17::Dis)
    }
    #[doc = "Interrupts are enabled for falling edge transition on this GPIO"]
    #[inline(always)]
    pub fn intfall(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten17::Intfall)
    }
    #[doc = "Interrupts are enabled for rising edge transitions on this GPIO"]
    #[inline(always)]
    pub fn intrise(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten17::Intrise)
    }
    #[doc = "Interrupts are enabled for any edge transition on this GPIO"]
    #[inline(always)]
    pub fn intany(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten17::Intany)
    }
}
#[doc = "Pin IO mode selection for GPIO pin 17\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Outcfg17 {
    #[doc = "0: Output Disabled"]
    Dis = 0,
    #[doc = "1: Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    Pushpull = 1,
    #[doc = "2: Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    Od = 2,
    #[doc = "3: Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    Ts = 3,
}
impl From<Outcfg17> for u8 {
    #[inline(always)]
    fn from(variant: Outcfg17) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Outcfg17 {
    type Ux = u8;
}
impl crate::IsEnum for Outcfg17 {}
#[doc = "Field `OUTCFG17` reader - Pin IO mode selection for GPIO pin 17"]
pub type Outcfg17R = crate::FieldReader<Outcfg17>;
impl Outcfg17R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Outcfg17 {
        match self.bits {
            0 => Outcfg17::Dis,
            1 => Outcfg17::Pushpull,
            2 => Outcfg17::Od,
            3 => Outcfg17::Ts,
            _ => unreachable!(),
        }
    }
    #[doc = "Output Disabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Outcfg17::Dis
    }
    #[doc = "Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == Outcfg17::Pushpull
    }
    #[doc = "Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == Outcfg17::Od
    }
    #[doc = "Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == Outcfg17::Ts
    }
}
#[doc = "Field `OUTCFG17` writer - Pin IO mode selection for GPIO pin 17"]
pub type Outcfg17W<'a, REG> = crate::FieldWriter<'a, REG, 2, Outcfg17, crate::Safe>;
impl<'a, REG> Outcfg17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Output Disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg17::Dis)
    }
    #[doc = "Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg17::Pushpull)
    }
    #[doc = "Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    #[inline(always)]
    pub fn od(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg17::Od)
    }
    #[doc = "Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    #[inline(always)]
    pub fn ts(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg17::Ts)
    }
}
#[doc = "Drive strength selection for GPIO 17\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ds17 {
    #[doc = "0: 0.1x output driver selected"]
    _0p1x = 0,
    #[doc = "1: 0.5x output driver selected"]
    _0p5x = 1,
}
impl From<Ds17> for u8 {
    #[inline(always)]
    fn from(variant: Ds17) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ds17 {
    type Ux = u8;
}
impl crate::IsEnum for Ds17 {}
#[doc = "Field `DS17` reader - Drive strength selection for GPIO 17"]
pub type Ds17R = crate::FieldReader<Ds17>;
impl Ds17R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ds17> {
        match self.bits {
            0 => Some(Ds17::_0p1x),
            1 => Some(Ds17::_0p5x),
            _ => None,
        }
    }
    #[doc = "0.1x output driver selected"]
    #[inline(always)]
    pub fn is_0p1x(&self) -> bool {
        *self == Ds17::_0p1x
    }
    #[doc = "0.5x output driver selected"]
    #[inline(always)]
    pub fn is_0p5x(&self) -> bool {
        *self == Ds17::_0p5x
    }
}
#[doc = "Field `DS17` writer - Drive strength selection for GPIO 17"]
pub type Ds17W<'a, REG> = crate::FieldWriter<'a, REG, 2, Ds17>;
impl<'a, REG> Ds17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "0.1x output driver selected"]
    #[inline(always)]
    pub fn _0p1x(self) -> &'a mut crate::W<REG> {
        self.variant(Ds17::_0p1x)
    }
    #[doc = "0.5x output driver selected"]
    #[inline(always)]
    pub fn _0p5x(self) -> &'a mut crate::W<REG> {
        self.variant(Ds17::_0p5x)
    }
}
#[doc = "Field `SR17` reader - Configure the slew rate"]
pub type Sr17R = crate::BitReader;
#[doc = "Field `SR17` writer - Configure the slew rate"]
pub type Sr17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Pullup/Pulldown configuration for GPIO 17\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pullcfg17 {
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
impl From<Pullcfg17> for u8 {
    #[inline(always)]
    fn from(variant: Pullcfg17) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pullcfg17 {
    type Ux = u8;
}
impl crate::IsEnum for Pullcfg17 {}
#[doc = "Field `PULLCFG17` reader - Pullup/Pulldown configuration for GPIO 17"]
pub type Pullcfg17R = crate::FieldReader<Pullcfg17>;
impl Pullcfg17R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pullcfg17 {
        match self.bits {
            0 => Pullcfg17::Dis,
            1 => Pullcfg17::Pd50k,
            2 => Pullcfg17::Pu15k,
            3 => Pullcfg17::Pu6k,
            4 => Pullcfg17::Pu12k,
            5 => Pullcfg17::Pu24k,
            6 => Pullcfg17::Pu50k,
            7 => Pullcfg17::Pu100k,
            _ => unreachable!(),
        }
    }
    #[doc = "No pullup or pulldown selected"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Pullcfg17::Dis
    }
    #[doc = "50K Pulldown selected"]
    #[inline(always)]
    pub fn is_pd50k(&self) -> bool {
        *self == Pullcfg17::Pd50k
    }
    #[doc = "1.5K Pullup selected"]
    #[inline(always)]
    pub fn is_pu15k(&self) -> bool {
        *self == Pullcfg17::Pu15k
    }
    #[doc = "6K Pullup selected"]
    #[inline(always)]
    pub fn is_pu6k(&self) -> bool {
        *self == Pullcfg17::Pu6k
    }
    #[doc = "12K Pullup selected"]
    #[inline(always)]
    pub fn is_pu12k(&self) -> bool {
        *self == Pullcfg17::Pu12k
    }
    #[doc = "24K Pullup selected"]
    #[inline(always)]
    pub fn is_pu24k(&self) -> bool {
        *self == Pullcfg17::Pu24k
    }
    #[doc = "50K Pullup selected"]
    #[inline(always)]
    pub fn is_pu50k(&self) -> bool {
        *self == Pullcfg17::Pu50k
    }
    #[doc = "100K Pullup selected"]
    #[inline(always)]
    pub fn is_pu100k(&self) -> bool {
        *self == Pullcfg17::Pu100k
    }
}
#[doc = "Field `PULLCFG17` writer - Pullup/Pulldown configuration for GPIO 17"]
pub type Pullcfg17W<'a, REG> = crate::FieldWriter<'a, REG, 3, Pullcfg17, crate::Safe>;
impl<'a, REG> Pullcfg17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No pullup or pulldown selected"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg17::Dis)
    }
    #[doc = "50K Pulldown selected"]
    #[inline(always)]
    pub fn pd50k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg17::Pd50k)
    }
    #[doc = "1.5K Pullup selected"]
    #[inline(always)]
    pub fn pu15k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg17::Pu15k)
    }
    #[doc = "6K Pullup selected"]
    #[inline(always)]
    pub fn pu6k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg17::Pu6k)
    }
    #[doc = "12K Pullup selected"]
    #[inline(always)]
    pub fn pu12k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg17::Pu12k)
    }
    #[doc = "24K Pullup selected"]
    #[inline(always)]
    pub fn pu24k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg17::Pu24k)
    }
    #[doc = "50K Pullup selected"]
    #[inline(always)]
    pub fn pu50k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg17::Pu50k)
    }
    #[doc = "100K Pullup selected"]
    #[inline(always)]
    pub fn pu100k(self) -> &'a mut crate::W<REG> {
        self.variant(Pullcfg17::Pu100k)
    }
}
#[doc = "IOMSTR/MSPI N Chip Select 17, DISP control signals DE, CSX, and CS. Polarity is determined by CE_POLARITY field\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ncesrc17 {
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
impl From<Ncesrc17> for u8 {
    #[inline(always)]
    fn from(variant: Ncesrc17) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ncesrc17 {
    type Ux = u8;
}
impl crate::IsEnum for Ncesrc17 {}
#[doc = "Field `NCESRC17` reader - IOMSTR/MSPI N Chip Select 17, DISP control signals DE, CSX, and CS. Polarity is determined by CE_POLARITY field"]
pub type Ncesrc17R = crate::FieldReader<Ncesrc17>;
impl Ncesrc17R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ncesrc17> {
        match self.bits {
            0 => Some(Ncesrc17::Iom0ce0),
            1 => Some(Ncesrc17::Iom0ce1),
            2 => Some(Ncesrc17::Iom0ce2),
            3 => Some(Ncesrc17::Iom0ce3),
            4 => Some(Ncesrc17::Iom1ce0),
            5 => Some(Ncesrc17::Iom1ce1),
            6 => Some(Ncesrc17::Iom1ce2),
            7 => Some(Ncesrc17::Iom1ce3),
            8 => Some(Ncesrc17::Iom2ce0),
            9 => Some(Ncesrc17::Iom2ce1),
            10 => Some(Ncesrc17::Iom2ce2),
            11 => Some(Ncesrc17::Iom2ce3),
            12 => Some(Ncesrc17::Iom3ce0),
            13 => Some(Ncesrc17::Iom3ce1),
            14 => Some(Ncesrc17::Iom3ce2),
            15 => Some(Ncesrc17::Iom3ce3),
            16 => Some(Ncesrc17::Iom4ce0),
            17 => Some(Ncesrc17::Iom4ce1),
            18 => Some(Ncesrc17::Iom4ce2),
            19 => Some(Ncesrc17::Iom4ce3),
            20 => Some(Ncesrc17::Iom5ce0),
            21 => Some(Ncesrc17::Iom5ce1),
            22 => Some(Ncesrc17::Iom5ce2),
            23 => Some(Ncesrc17::Iom5ce3),
            24 => Some(Ncesrc17::Iom6ce0),
            25 => Some(Ncesrc17::Iom6ce1),
            26 => Some(Ncesrc17::Iom6ce2),
            27 => Some(Ncesrc17::Iom6ce3),
            28 => Some(Ncesrc17::Iom7ce0),
            29 => Some(Ncesrc17::Iom7ce1),
            30 => Some(Ncesrc17::Iom7ce2),
            31 => Some(Ncesrc17::Iom7ce3),
            32 => Some(Ncesrc17::Mspi0cen0),
            33 => Some(Ncesrc17::Mspi0cen1),
            34 => Some(Ncesrc17::Mspi1cen0),
            35 => Some(Ncesrc17::Mspi1cen1),
            36 => Some(Ncesrc17::Mspi2cen0),
            37 => Some(Ncesrc17::Mspi2cen1),
            38 => Some(Ncesrc17::DcDpiDe),
            39 => Some(Ncesrc17::DispContCsx),
            40 => Some(Ncesrc17::DcSpiCsN),
            41 => Some(Ncesrc17::DcQspiCsN),
            42 => Some(Ncesrc17::DcResx),
            _ => None,
        }
    }
    #[doc = "IOM 0 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom0ce0(&self) -> bool {
        *self == Ncesrc17::Iom0ce0
    }
    #[doc = "IOM 0 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom0ce1(&self) -> bool {
        *self == Ncesrc17::Iom0ce1
    }
    #[doc = "IOM 0 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom0ce2(&self) -> bool {
        *self == Ncesrc17::Iom0ce2
    }
    #[doc = "IOM 0 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom0ce3(&self) -> bool {
        *self == Ncesrc17::Iom0ce3
    }
    #[doc = "IOM 1 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom1ce0(&self) -> bool {
        *self == Ncesrc17::Iom1ce0
    }
    #[doc = "IOM 1 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom1ce1(&self) -> bool {
        *self == Ncesrc17::Iom1ce1
    }
    #[doc = "IOM 1 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom1ce2(&self) -> bool {
        *self == Ncesrc17::Iom1ce2
    }
    #[doc = "IOM 1 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom1ce3(&self) -> bool {
        *self == Ncesrc17::Iom1ce3
    }
    #[doc = "IOM 2 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom2ce0(&self) -> bool {
        *self == Ncesrc17::Iom2ce0
    }
    #[doc = "IOM 2 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom2ce1(&self) -> bool {
        *self == Ncesrc17::Iom2ce1
    }
    #[doc = "IOM 2 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom2ce2(&self) -> bool {
        *self == Ncesrc17::Iom2ce2
    }
    #[doc = "IOM 2 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom2ce3(&self) -> bool {
        *self == Ncesrc17::Iom2ce3
    }
    #[doc = "IOM 3 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom3ce0(&self) -> bool {
        *self == Ncesrc17::Iom3ce0
    }
    #[doc = "IOM 3 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom3ce1(&self) -> bool {
        *self == Ncesrc17::Iom3ce1
    }
    #[doc = "IOM 3 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom3ce2(&self) -> bool {
        *self == Ncesrc17::Iom3ce2
    }
    #[doc = "IOM 3 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom3ce3(&self) -> bool {
        *self == Ncesrc17::Iom3ce3
    }
    #[doc = "IOM 4 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom4ce0(&self) -> bool {
        *self == Ncesrc17::Iom4ce0
    }
    #[doc = "IOM 4 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom4ce1(&self) -> bool {
        *self == Ncesrc17::Iom4ce1
    }
    #[doc = "IOM 4 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom4ce2(&self) -> bool {
        *self == Ncesrc17::Iom4ce2
    }
    #[doc = "IOM 4 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom4ce3(&self) -> bool {
        *self == Ncesrc17::Iom4ce3
    }
    #[doc = "IOM 5 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom5ce0(&self) -> bool {
        *self == Ncesrc17::Iom5ce0
    }
    #[doc = "IOM 5 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom5ce1(&self) -> bool {
        *self == Ncesrc17::Iom5ce1
    }
    #[doc = "IOM 5 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom5ce2(&self) -> bool {
        *self == Ncesrc17::Iom5ce2
    }
    #[doc = "IOM 5 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom5ce3(&self) -> bool {
        *self == Ncesrc17::Iom5ce3
    }
    #[doc = "IOM 6 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom6ce0(&self) -> bool {
        *self == Ncesrc17::Iom6ce0
    }
    #[doc = "IOM 6 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom6ce1(&self) -> bool {
        *self == Ncesrc17::Iom6ce1
    }
    #[doc = "IOM 6 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom6ce2(&self) -> bool {
        *self == Ncesrc17::Iom6ce2
    }
    #[doc = "IOM 6 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom6ce3(&self) -> bool {
        *self == Ncesrc17::Iom6ce3
    }
    #[doc = "IOM 7 NCE 0 module"]
    #[inline(always)]
    pub fn is_iom7ce0(&self) -> bool {
        *self == Ncesrc17::Iom7ce0
    }
    #[doc = "IOM 7 NCE 1 module"]
    #[inline(always)]
    pub fn is_iom7ce1(&self) -> bool {
        *self == Ncesrc17::Iom7ce1
    }
    #[doc = "IOM 7 NCE 2 module"]
    #[inline(always)]
    pub fn is_iom7ce2(&self) -> bool {
        *self == Ncesrc17::Iom7ce2
    }
    #[doc = "IOM 7 NCE 3 module"]
    #[inline(always)]
    pub fn is_iom7ce3(&self) -> bool {
        *self == Ncesrc17::Iom7ce3
    }
    #[doc = "MSPI 0 NCE 0 module"]
    #[inline(always)]
    pub fn is_mspi0cen0(&self) -> bool {
        *self == Ncesrc17::Mspi0cen0
    }
    #[doc = "MSPI 0 NCE 1 module"]
    #[inline(always)]
    pub fn is_mspi0cen1(&self) -> bool {
        *self == Ncesrc17::Mspi0cen1
    }
    #[doc = "MSPI 1 NCE 0 module"]
    #[inline(always)]
    pub fn is_mspi1cen0(&self) -> bool {
        *self == Ncesrc17::Mspi1cen0
    }
    #[doc = "MSPI 1 NCE 1 module"]
    #[inline(always)]
    pub fn is_mspi1cen1(&self) -> bool {
        *self == Ncesrc17::Mspi1cen1
    }
    #[doc = "MSPI 2 NCE 0 module"]
    #[inline(always)]
    pub fn is_mspi2cen0(&self) -> bool {
        *self == Ncesrc17::Mspi2cen0
    }
    #[doc = "MSPI 2 NCE 1 module"]
    #[inline(always)]
    pub fn is_mspi2cen1(&self) -> bool {
        *self == Ncesrc17::Mspi2cen1
    }
    #[doc = "DC DPI DE module"]
    #[inline(always)]
    pub fn is_dc_dpi_de(&self) -> bool {
        *self == Ncesrc17::DcDpiDe
    }
    #[doc = "DISP CONT CSX module"]
    #[inline(always)]
    pub fn is_disp_cont_csx(&self) -> bool {
        *self == Ncesrc17::DispContCsx
    }
    #[doc = "DC SPI CS_N module"]
    #[inline(always)]
    pub fn is_dc_spi_cs_n(&self) -> bool {
        *self == Ncesrc17::DcSpiCsN
    }
    #[doc = "DC QSPI CS_N module"]
    #[inline(always)]
    pub fn is_dc_qspi_cs_n(&self) -> bool {
        *self == Ncesrc17::DcQspiCsN
    }
    #[doc = "DC module RESX"]
    #[inline(always)]
    pub fn is_dc_resx(&self) -> bool {
        *self == Ncesrc17::DcResx
    }
}
#[doc = "Field `NCESRC17` writer - IOMSTR/MSPI N Chip Select 17, DISP control signals DE, CSX, and CS. Polarity is determined by CE_POLARITY field"]
pub type Ncesrc17W<'a, REG> = crate::FieldWriter<'a, REG, 6, Ncesrc17>;
impl<'a, REG> Ncesrc17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "IOM 0 NCE 0 module"]
    #[inline(always)]
    pub fn iom0ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc17::Iom0ce0)
    }
    #[doc = "IOM 0 NCE 1 module"]
    #[inline(always)]
    pub fn iom0ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc17::Iom0ce1)
    }
    #[doc = "IOM 0 NCE 2 module"]
    #[inline(always)]
    pub fn iom0ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc17::Iom0ce2)
    }
    #[doc = "IOM 0 NCE 3 module"]
    #[inline(always)]
    pub fn iom0ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc17::Iom0ce3)
    }
    #[doc = "IOM 1 NCE 0 module"]
    #[inline(always)]
    pub fn iom1ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc17::Iom1ce0)
    }
    #[doc = "IOM 1 NCE 1 module"]
    #[inline(always)]
    pub fn iom1ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc17::Iom1ce1)
    }
    #[doc = "IOM 1 NCE 2 module"]
    #[inline(always)]
    pub fn iom1ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc17::Iom1ce2)
    }
    #[doc = "IOM 1 NCE 3 module"]
    #[inline(always)]
    pub fn iom1ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc17::Iom1ce3)
    }
    #[doc = "IOM 2 NCE 0 module"]
    #[inline(always)]
    pub fn iom2ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc17::Iom2ce0)
    }
    #[doc = "IOM 2 NCE 1 module"]
    #[inline(always)]
    pub fn iom2ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc17::Iom2ce1)
    }
    #[doc = "IOM 2 NCE 2 module"]
    #[inline(always)]
    pub fn iom2ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc17::Iom2ce2)
    }
    #[doc = "IOM 2 NCE 3 module"]
    #[inline(always)]
    pub fn iom2ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc17::Iom2ce3)
    }
    #[doc = "IOM 3 NCE 0 module"]
    #[inline(always)]
    pub fn iom3ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc17::Iom3ce0)
    }
    #[doc = "IOM 3 NCE 1 module"]
    #[inline(always)]
    pub fn iom3ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc17::Iom3ce1)
    }
    #[doc = "IOM 3 NCE 2 module"]
    #[inline(always)]
    pub fn iom3ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc17::Iom3ce2)
    }
    #[doc = "IOM 3 NCE 3 module"]
    #[inline(always)]
    pub fn iom3ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc17::Iom3ce3)
    }
    #[doc = "IOM 4 NCE 0 module"]
    #[inline(always)]
    pub fn iom4ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc17::Iom4ce0)
    }
    #[doc = "IOM 4 NCE 1 module"]
    #[inline(always)]
    pub fn iom4ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc17::Iom4ce1)
    }
    #[doc = "IOM 4 NCE 2 module"]
    #[inline(always)]
    pub fn iom4ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc17::Iom4ce2)
    }
    #[doc = "IOM 4 NCE 3 module"]
    #[inline(always)]
    pub fn iom4ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc17::Iom4ce3)
    }
    #[doc = "IOM 5 NCE 0 module"]
    #[inline(always)]
    pub fn iom5ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc17::Iom5ce0)
    }
    #[doc = "IOM 5 NCE 1 module"]
    #[inline(always)]
    pub fn iom5ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc17::Iom5ce1)
    }
    #[doc = "IOM 5 NCE 2 module"]
    #[inline(always)]
    pub fn iom5ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc17::Iom5ce2)
    }
    #[doc = "IOM 5 NCE 3 module"]
    #[inline(always)]
    pub fn iom5ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc17::Iom5ce3)
    }
    #[doc = "IOM 6 NCE 0 module"]
    #[inline(always)]
    pub fn iom6ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc17::Iom6ce0)
    }
    #[doc = "IOM 6 NCE 1 module"]
    #[inline(always)]
    pub fn iom6ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc17::Iom6ce1)
    }
    #[doc = "IOM 6 NCE 2 module"]
    #[inline(always)]
    pub fn iom6ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc17::Iom6ce2)
    }
    #[doc = "IOM 6 NCE 3 module"]
    #[inline(always)]
    pub fn iom6ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc17::Iom6ce3)
    }
    #[doc = "IOM 7 NCE 0 module"]
    #[inline(always)]
    pub fn iom7ce0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc17::Iom7ce0)
    }
    #[doc = "IOM 7 NCE 1 module"]
    #[inline(always)]
    pub fn iom7ce1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc17::Iom7ce1)
    }
    #[doc = "IOM 7 NCE 2 module"]
    #[inline(always)]
    pub fn iom7ce2(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc17::Iom7ce2)
    }
    #[doc = "IOM 7 NCE 3 module"]
    #[inline(always)]
    pub fn iom7ce3(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc17::Iom7ce3)
    }
    #[doc = "MSPI 0 NCE 0 module"]
    #[inline(always)]
    pub fn mspi0cen0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc17::Mspi0cen0)
    }
    #[doc = "MSPI 0 NCE 1 module"]
    #[inline(always)]
    pub fn mspi0cen1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc17::Mspi0cen1)
    }
    #[doc = "MSPI 1 NCE 0 module"]
    #[inline(always)]
    pub fn mspi1cen0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc17::Mspi1cen0)
    }
    #[doc = "MSPI 1 NCE 1 module"]
    #[inline(always)]
    pub fn mspi1cen1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc17::Mspi1cen1)
    }
    #[doc = "MSPI 2 NCE 0 module"]
    #[inline(always)]
    pub fn mspi2cen0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc17::Mspi2cen0)
    }
    #[doc = "MSPI 2 NCE 1 module"]
    #[inline(always)]
    pub fn mspi2cen1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc17::Mspi2cen1)
    }
    #[doc = "DC DPI DE module"]
    #[inline(always)]
    pub fn dc_dpi_de(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc17::DcDpiDe)
    }
    #[doc = "DISP CONT CSX module"]
    #[inline(always)]
    pub fn disp_cont_csx(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc17::DispContCsx)
    }
    #[doc = "DC SPI CS_N module"]
    #[inline(always)]
    pub fn dc_spi_cs_n(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc17::DcSpiCsN)
    }
    #[doc = "DC QSPI CS_N module"]
    #[inline(always)]
    pub fn dc_qspi_cs_n(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc17::DcQspiCsN)
    }
    #[doc = "DC module RESX"]
    #[inline(always)]
    pub fn dc_resx(self) -> &'a mut crate::W<REG> {
        self.variant(Ncesrc17::DcResx)
    }
}
#[doc = "Polarity select for NCE for GPIO 17\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ncepol17 {
    #[doc = "0: Polarity is active low"]
    Low = 0,
    #[doc = "1: Polarity is active high"]
    High = 1,
}
impl From<Ncepol17> for bool {
    #[inline(always)]
    fn from(variant: Ncepol17) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NCEPOL17` reader - Polarity select for NCE for GPIO 17"]
pub type Ncepol17R = crate::BitReader<Ncepol17>;
impl Ncepol17R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ncepol17 {
        match self.bits {
            false => Ncepol17::Low,
            true => Ncepol17::High,
        }
    }
    #[doc = "Polarity is active low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Ncepol17::Low
    }
    #[doc = "Polarity is active high"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Ncepol17::High
    }
}
#[doc = "Field `NCEPOL17` writer - Polarity select for NCE for GPIO 17"]
pub type Ncepol17W<'a, REG> = crate::BitWriter<'a, REG, Ncepol17>;
impl<'a, REG> Ncepol17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Polarity is active low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Ncepol17::Low)
    }
    #[doc = "Polarity is active high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Ncepol17::High)
    }
}
#[doc = "Field `FIEN17` reader - Force input enable active regardless of function selected. Otherwise the selected function will enable the input only when needed"]
pub type Fien17R = crate::BitReader;
#[doc = "Field `FIEN17` writer - Force input enable active regardless of function selected. Otherwise the selected function will enable the input only when needed"]
pub type Fien17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FOEN17` reader - Force output enable active regardless of function selected. Otherwise the selected function will enable the output only when needed"]
pub type Foen17R = crate::BitReader;
#[doc = "Field `FOEN17` writer - Force output enable active regardless of function selected. Otherwise the selected function will enable the output only when needed"]
pub type Foen17W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Function select for GPIO pin 17"]
    #[inline(always)]
    pub fn fncsel17(&self) -> Fncsel17R {
        Fncsel17R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Input enable for GPIO 17"]
    #[inline(always)]
    pub fn inpen17(&self) -> Inpen17R {
        Inpen17R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Return 0 for read data on GPIO 17"]
    #[inline(always)]
    pub fn rdzero17(&self) -> Rdzero17R {
        Rdzero17R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Interrupt enable for GPIO 17"]
    #[inline(always)]
    pub fn irpten17(&self) -> Irpten17R {
        Irpten17R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Pin IO mode selection for GPIO pin 17"]
    #[inline(always)]
    pub fn outcfg17(&self) -> Outcfg17R {
        Outcfg17R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Drive strength selection for GPIO 17"]
    #[inline(always)]
    pub fn ds17(&self) -> Ds17R {
        Ds17R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - Configure the slew rate"]
    #[inline(always)]
    pub fn sr17(&self) -> Sr17R {
        Sr17R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - Pullup/Pulldown configuration for GPIO 17"]
    #[inline(always)]
    pub fn pullcfg17(&self) -> Pullcfg17R {
        Pullcfg17R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:21 - IOMSTR/MSPI N Chip Select 17, DISP control signals DE, CSX, and CS. Polarity is determined by CE_POLARITY field"]
    #[inline(always)]
    pub fn ncesrc17(&self) -> Ncesrc17R {
        Ncesrc17R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 22 - Polarity select for NCE for GPIO 17"]
    #[inline(always)]
    pub fn ncepol17(&self) -> Ncepol17R {
        Ncepol17R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 26 - Force input enable active regardless of function selected. Otherwise the selected function will enable the input only when needed"]
    #[inline(always)]
    pub fn fien17(&self) -> Fien17R {
        Fien17R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Force output enable active regardless of function selected. Otherwise the selected function will enable the output only when needed"]
    #[inline(always)]
    pub fn foen17(&self) -> Foen17R {
        Foen17R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Function select for GPIO pin 17"]
    #[inline(always)]
    #[must_use]
    pub fn fncsel17(&mut self) -> Fncsel17W<Pincfg17Spec> {
        Fncsel17W::new(self, 0)
    }
    #[doc = "Bit 4 - Input enable for GPIO 17"]
    #[inline(always)]
    #[must_use]
    pub fn inpen17(&mut self) -> Inpen17W<Pincfg17Spec> {
        Inpen17W::new(self, 4)
    }
    #[doc = "Bit 5 - Return 0 for read data on GPIO 17"]
    #[inline(always)]
    #[must_use]
    pub fn rdzero17(&mut self) -> Rdzero17W<Pincfg17Spec> {
        Rdzero17W::new(self, 5)
    }
    #[doc = "Bits 6:7 - Interrupt enable for GPIO 17"]
    #[inline(always)]
    #[must_use]
    pub fn irpten17(&mut self) -> Irpten17W<Pincfg17Spec> {
        Irpten17W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Pin IO mode selection for GPIO pin 17"]
    #[inline(always)]
    #[must_use]
    pub fn outcfg17(&mut self) -> Outcfg17W<Pincfg17Spec> {
        Outcfg17W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Drive strength selection for GPIO 17"]
    #[inline(always)]
    #[must_use]
    pub fn ds17(&mut self) -> Ds17W<Pincfg17Spec> {
        Ds17W::new(self, 10)
    }
    #[doc = "Bit 12 - Configure the slew rate"]
    #[inline(always)]
    #[must_use]
    pub fn sr17(&mut self) -> Sr17W<Pincfg17Spec> {
        Sr17W::new(self, 12)
    }
    #[doc = "Bits 13:15 - Pullup/Pulldown configuration for GPIO 17"]
    #[inline(always)]
    #[must_use]
    pub fn pullcfg17(&mut self) -> Pullcfg17W<Pincfg17Spec> {
        Pullcfg17W::new(self, 13)
    }
    #[doc = "Bits 16:21 - IOMSTR/MSPI N Chip Select 17, DISP control signals DE, CSX, and CS. Polarity is determined by CE_POLARITY field"]
    #[inline(always)]
    #[must_use]
    pub fn ncesrc17(&mut self) -> Ncesrc17W<Pincfg17Spec> {
        Ncesrc17W::new(self, 16)
    }
    #[doc = "Bit 22 - Polarity select for NCE for GPIO 17"]
    #[inline(always)]
    #[must_use]
    pub fn ncepol17(&mut self) -> Ncepol17W<Pincfg17Spec> {
        Ncepol17W::new(self, 22)
    }
    #[doc = "Bit 26 - Force input enable active regardless of function selected. Otherwise the selected function will enable the input only when needed"]
    #[inline(always)]
    #[must_use]
    pub fn fien17(&mut self) -> Fien17W<Pincfg17Spec> {
        Fien17W::new(self, 26)
    }
    #[doc = "Bit 27 - Force output enable active regardless of function selected. Otherwise the selected function will enable the output only when needed"]
    #[inline(always)]
    #[must_use]
    pub fn foen17(&mut self) -> Foen17W<Pincfg17Spec> {
        Foen17W::new(self, 27)
    }
}
#[doc = "Controls the operation of GPIO pin 17.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg17::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg17::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pincfg17Spec;
impl crate::RegisterSpec for Pincfg17Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pincfg17::R`](R) reader structure"]
impl crate::Readable for Pincfg17Spec {}
#[doc = "`write(|w| ..)` method takes [`pincfg17::W`](W) writer structure"]
impl crate::Writable for Pincfg17Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PINCFG17 to value 0x03"]
impl crate::Resettable for Pincfg17Spec {
    const RESET_VALUE: u32 = 0x03;
}
