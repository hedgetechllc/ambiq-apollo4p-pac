#[doc = "Register `PINCFG114` reader"]
pub type R = crate::R<Pincfg114Spec>;
#[doc = "Register `PINCFG114` writer"]
pub type W = crate::W<Pincfg114Spec>;
#[doc = "Function select for GPIO pin 114\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fncsel114 {
    #[doc = "0: Reserved selection. Operation unknown if selected."]
    Reserved0 = 0,
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
    Ct114 = 6,
    #[doc = "7: Reserved selection. Operation unknown if selected."]
    Reserved7 = 7,
    #[doc = "8: Observation bus bit 2"]
    Obsbus2 = 8,
    #[doc = "9: Reserved selection. Operation unknown if selected."]
    Reserved9 = 9,
    #[doc = "10: Reserved selection. Operation unknown if selected."]
    Reserved10 = 10,
    #[doc = "11: Reserved selection. Operation unknown if selected."]
    Reserved11 = 11,
    #[doc = "12: Reserved selection. Operation unknown if selected."]
    Reserved12 = 12,
    #[doc = "13: Reserved selection. Operation unknown if selected."]
    Reserved13 = 13,
    #[doc = "14: Reserved selection. Operation unknown if selected."]
    Reserved14 = 14,
    #[doc = "15: Reserved selection. Operation unknown if selected."]
    Reserved15 = 15,
}
impl From<Fncsel114> for u8 {
    #[inline(always)]
    fn from(variant: Fncsel114) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fncsel114 {
    type Ux = u8;
}
impl crate::IsEnum for Fncsel114 {}
#[doc = "Field `FNCSEL114` reader - Function select for GPIO pin 114"]
pub type Fncsel114R = crate::FieldReader<Fncsel114>;
impl Fncsel114R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fncsel114 {
        match self.bits {
            0 => Fncsel114::Reserved0,
            1 => Fncsel114::Reserved1,
            2 => Fncsel114::Reserved2,
            3 => Fncsel114::Gpio,
            4 => Fncsel114::Reserved4,
            5 => Fncsel114::Reserved5,
            6 => Fncsel114::Ct114,
            7 => Fncsel114::Reserved7,
            8 => Fncsel114::Obsbus2,
            9 => Fncsel114::Reserved9,
            10 => Fncsel114::Reserved10,
            11 => Fncsel114::Reserved11,
            12 => Fncsel114::Reserved12,
            13 => Fncsel114::Reserved13,
            14 => Fncsel114::Reserved14,
            15 => Fncsel114::Reserved15,
            _ => unreachable!(),
        }
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved0(&self) -> bool {
        *self == Fncsel114::Reserved0
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved1(&self) -> bool {
        *self == Fncsel114::Reserved1
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == Fncsel114::Reserved2
    }
    #[doc = "General purpose I/O"]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == Fncsel114::Gpio
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved4(&self) -> bool {
        *self == Fncsel114::Reserved4
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved5(&self) -> bool {
        *self == Fncsel114::Reserved5
    }
    #[doc = "Timer/Counter input or output; Selection of direction is done via CTIMER register settings."]
    #[inline(always)]
    pub fn is_ct114(&self) -> bool {
        *self == Fncsel114::Ct114
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved7(&self) -> bool {
        *self == Fncsel114::Reserved7
    }
    #[doc = "Observation bus bit 2"]
    #[inline(always)]
    pub fn is_obsbus2(&self) -> bool {
        *self == Fncsel114::Obsbus2
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved9(&self) -> bool {
        *self == Fncsel114::Reserved9
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved10(&self) -> bool {
        *self == Fncsel114::Reserved10
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved11(&self) -> bool {
        *self == Fncsel114::Reserved11
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved12(&self) -> bool {
        *self == Fncsel114::Reserved12
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved13(&self) -> bool {
        *self == Fncsel114::Reserved13
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved14(&self) -> bool {
        *self == Fncsel114::Reserved14
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved15(&self) -> bool {
        *self == Fncsel114::Reserved15
    }
}
#[doc = "Field `FNCSEL114` writer - Function select for GPIO pin 114"]
pub type Fncsel114W<'a, REG> = crate::FieldWriter<'a, REG, 4, Fncsel114, crate::Safe>;
impl<'a, REG> Fncsel114W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved0(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel114::Reserved0)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved1(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel114::Reserved1)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel114::Reserved2)
    }
    #[doc = "General purpose I/O"]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel114::Gpio)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved4(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel114::Reserved4)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved5(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel114::Reserved5)
    }
    #[doc = "Timer/Counter input or output; Selection of direction is done via CTIMER register settings."]
    #[inline(always)]
    pub fn ct114(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel114::Ct114)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved7(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel114::Reserved7)
    }
    #[doc = "Observation bus bit 2"]
    #[inline(always)]
    pub fn obsbus2(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel114::Obsbus2)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved9(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel114::Reserved9)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved10(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel114::Reserved10)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved11(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel114::Reserved11)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved12(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel114::Reserved12)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved13(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel114::Reserved13)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved14(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel114::Reserved14)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved15(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel114::Reserved15)
    }
}
#[doc = "Field `INPEN114` reader - Input enable for GPIO 114"]
pub type Inpen114R = crate::BitReader;
#[doc = "Field `INPEN114` writer - Input enable for GPIO 114"]
pub type Inpen114W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDZERO114` reader - Return 0 for read data on GPIO 114"]
pub type Rdzero114R = crate::BitReader;
#[doc = "Field `RDZERO114` writer - Return 0 for read data on GPIO 114"]
pub type Rdzero114W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Interrupt enable for GPIO 114\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Irpten114 {
    #[doc = "0: Interrupts are disabled for this GPIO"]
    Dis = 0,
    #[doc = "1: Interrupts are enabled for falling edge transition on this GPIO"]
    Intfall = 1,
    #[doc = "2: Interrupts are enabled for rising edge transitions on this GPIO"]
    Intrise = 2,
    #[doc = "3: Interrupts are enabled for any edge transition on this GPIO"]
    Intany = 3,
}
impl From<Irpten114> for u8 {
    #[inline(always)]
    fn from(variant: Irpten114) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Irpten114 {
    type Ux = u8;
}
impl crate::IsEnum for Irpten114 {}
#[doc = "Field `IRPTEN114` reader - Interrupt enable for GPIO 114"]
pub type Irpten114R = crate::FieldReader<Irpten114>;
impl Irpten114R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irpten114 {
        match self.bits {
            0 => Irpten114::Dis,
            1 => Irpten114::Intfall,
            2 => Irpten114::Intrise,
            3 => Irpten114::Intany,
            _ => unreachable!(),
        }
    }
    #[doc = "Interrupts are disabled for this GPIO"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Irpten114::Dis
    }
    #[doc = "Interrupts are enabled for falling edge transition on this GPIO"]
    #[inline(always)]
    pub fn is_intfall(&self) -> bool {
        *self == Irpten114::Intfall
    }
    #[doc = "Interrupts are enabled for rising edge transitions on this GPIO"]
    #[inline(always)]
    pub fn is_intrise(&self) -> bool {
        *self == Irpten114::Intrise
    }
    #[doc = "Interrupts are enabled for any edge transition on this GPIO"]
    #[inline(always)]
    pub fn is_intany(&self) -> bool {
        *self == Irpten114::Intany
    }
}
#[doc = "Field `IRPTEN114` writer - Interrupt enable for GPIO 114"]
pub type Irpten114W<'a, REG> = crate::FieldWriter<'a, REG, 2, Irpten114, crate::Safe>;
impl<'a, REG> Irpten114W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Interrupts are disabled for this GPIO"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten114::Dis)
    }
    #[doc = "Interrupts are enabled for falling edge transition on this GPIO"]
    #[inline(always)]
    pub fn intfall(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten114::Intfall)
    }
    #[doc = "Interrupts are enabled for rising edge transitions on this GPIO"]
    #[inline(always)]
    pub fn intrise(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten114::Intrise)
    }
    #[doc = "Interrupts are enabled for any edge transition on this GPIO"]
    #[inline(always)]
    pub fn intany(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten114::Intany)
    }
}
#[doc = "Pin IO mode selection for GPIO pin 114\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Outcfg114 {
    #[doc = "0: Output Disabled"]
    Dis = 0,
    #[doc = "1: Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    Pushpull = 1,
    #[doc = "2: Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    Od = 2,
    #[doc = "3: Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    Ts = 3,
}
impl From<Outcfg114> for u8 {
    #[inline(always)]
    fn from(variant: Outcfg114) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Outcfg114 {
    type Ux = u8;
}
impl crate::IsEnum for Outcfg114 {}
#[doc = "Field `OUTCFG114` reader - Pin IO mode selection for GPIO pin 114"]
pub type Outcfg114R = crate::FieldReader<Outcfg114>;
impl Outcfg114R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Outcfg114 {
        match self.bits {
            0 => Outcfg114::Dis,
            1 => Outcfg114::Pushpull,
            2 => Outcfg114::Od,
            3 => Outcfg114::Ts,
            _ => unreachable!(),
        }
    }
    #[doc = "Output Disabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Outcfg114::Dis
    }
    #[doc = "Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == Outcfg114::Pushpull
    }
    #[doc = "Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == Outcfg114::Od
    }
    #[doc = "Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == Outcfg114::Ts
    }
}
#[doc = "Field `OUTCFG114` writer - Pin IO mode selection for GPIO pin 114"]
pub type Outcfg114W<'a, REG> = crate::FieldWriter<'a, REG, 2, Outcfg114, crate::Safe>;
impl<'a, REG> Outcfg114W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Output Disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg114::Dis)
    }
    #[doc = "Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg114::Pushpull)
    }
    #[doc = "Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    #[inline(always)]
    pub fn od(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg114::Od)
    }
    #[doc = "Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    #[inline(always)]
    pub fn ts(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg114::Ts)
    }
}
impl R {
    #[doc = "Bits 0:3 - Function select for GPIO pin 114"]
    #[inline(always)]
    pub fn fncsel114(&self) -> Fncsel114R {
        Fncsel114R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Input enable for GPIO 114"]
    #[inline(always)]
    pub fn inpen114(&self) -> Inpen114R {
        Inpen114R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Return 0 for read data on GPIO 114"]
    #[inline(always)]
    pub fn rdzero114(&self) -> Rdzero114R {
        Rdzero114R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Interrupt enable for GPIO 114"]
    #[inline(always)]
    pub fn irpten114(&self) -> Irpten114R {
        Irpten114R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Pin IO mode selection for GPIO pin 114"]
    #[inline(always)]
    pub fn outcfg114(&self) -> Outcfg114R {
        Outcfg114R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Function select for GPIO pin 114"]
    #[inline(always)]
    #[must_use]
    pub fn fncsel114(&mut self) -> Fncsel114W<Pincfg114Spec> {
        Fncsel114W::new(self, 0)
    }
    #[doc = "Bit 4 - Input enable for GPIO 114"]
    #[inline(always)]
    #[must_use]
    pub fn inpen114(&mut self) -> Inpen114W<Pincfg114Spec> {
        Inpen114W::new(self, 4)
    }
    #[doc = "Bit 5 - Return 0 for read data on GPIO 114"]
    #[inline(always)]
    #[must_use]
    pub fn rdzero114(&mut self) -> Rdzero114W<Pincfg114Spec> {
        Rdzero114W::new(self, 5)
    }
    #[doc = "Bits 6:7 - Interrupt enable for GPIO 114"]
    #[inline(always)]
    #[must_use]
    pub fn irpten114(&mut self) -> Irpten114W<Pincfg114Spec> {
        Irpten114W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Pin IO mode selection for GPIO pin 114"]
    #[inline(always)]
    #[must_use]
    pub fn outcfg114(&mut self) -> Outcfg114W<Pincfg114Spec> {
        Outcfg114W::new(self, 8)
    }
}
#[doc = "Controls the operation of virtual GPIO pin 114.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg114::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg114::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pincfg114Spec;
impl crate::RegisterSpec for Pincfg114Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pincfg114::R`](R) reader structure"]
impl crate::Readable for Pincfg114Spec {}
#[doc = "`write(|w| ..)` method takes [`pincfg114::W`](W) writer structure"]
impl crate::Writable for Pincfg114Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PINCFG114 to value 0x03"]
impl crate::Resettable for Pincfg114Spec {
    const RESET_VALUE: u32 = 0x03;
}
