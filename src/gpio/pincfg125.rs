#[doc = "Register `PINCFG125` reader"]
pub type R = crate::R<Pincfg125Spec>;
#[doc = "Register `PINCFG125` writer"]
pub type W = crate::W<Pincfg125Spec>;
#[doc = "Function select for GPIO pin 125\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fncsel125 {
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
    Ct125 = 6,
    #[doc = "7: Reserved selection. Operation unknown if selected."]
    Reserved7 = 7,
    #[doc = "8: Observation bus bit 13"]
    Obsbus13 = 8,
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
impl From<Fncsel125> for u8 {
    #[inline(always)]
    fn from(variant: Fncsel125) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fncsel125 {
    type Ux = u8;
}
impl crate::IsEnum for Fncsel125 {}
#[doc = "Field `FNCSEL125` reader - Function select for GPIO pin 125"]
pub type Fncsel125R = crate::FieldReader<Fncsel125>;
impl Fncsel125R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fncsel125 {
        match self.bits {
            0 => Fncsel125::Reserved0,
            1 => Fncsel125::Reserved1,
            2 => Fncsel125::Reserved2,
            3 => Fncsel125::Gpio,
            4 => Fncsel125::Reserved4,
            5 => Fncsel125::Reserved5,
            6 => Fncsel125::Ct125,
            7 => Fncsel125::Reserved7,
            8 => Fncsel125::Obsbus13,
            9 => Fncsel125::Reserved9,
            10 => Fncsel125::Reserved10,
            11 => Fncsel125::Reserved11,
            12 => Fncsel125::Reserved12,
            13 => Fncsel125::Reserved13,
            14 => Fncsel125::Reserved14,
            15 => Fncsel125::Reserved15,
            _ => unreachable!(),
        }
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved0(&self) -> bool {
        *self == Fncsel125::Reserved0
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved1(&self) -> bool {
        *self == Fncsel125::Reserved1
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == Fncsel125::Reserved2
    }
    #[doc = "General purpose I/O"]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == Fncsel125::Gpio
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved4(&self) -> bool {
        *self == Fncsel125::Reserved4
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved5(&self) -> bool {
        *self == Fncsel125::Reserved5
    }
    #[doc = "Timer/Counter input or output; Selection of direction is done via CTIMER register settings."]
    #[inline(always)]
    pub fn is_ct125(&self) -> bool {
        *self == Fncsel125::Ct125
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved7(&self) -> bool {
        *self == Fncsel125::Reserved7
    }
    #[doc = "Observation bus bit 13"]
    #[inline(always)]
    pub fn is_obsbus13(&self) -> bool {
        *self == Fncsel125::Obsbus13
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved9(&self) -> bool {
        *self == Fncsel125::Reserved9
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved10(&self) -> bool {
        *self == Fncsel125::Reserved10
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved11(&self) -> bool {
        *self == Fncsel125::Reserved11
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved12(&self) -> bool {
        *self == Fncsel125::Reserved12
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved13(&self) -> bool {
        *self == Fncsel125::Reserved13
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved14(&self) -> bool {
        *self == Fncsel125::Reserved14
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved15(&self) -> bool {
        *self == Fncsel125::Reserved15
    }
}
#[doc = "Field `FNCSEL125` writer - Function select for GPIO pin 125"]
pub type Fncsel125W<'a, REG> = crate::FieldWriter<'a, REG, 4, Fncsel125, crate::Safe>;
impl<'a, REG> Fncsel125W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved0(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel125::Reserved0)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved1(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel125::Reserved1)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel125::Reserved2)
    }
    #[doc = "General purpose I/O"]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel125::Gpio)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved4(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel125::Reserved4)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved5(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel125::Reserved5)
    }
    #[doc = "Timer/Counter input or output; Selection of direction is done via CTIMER register settings."]
    #[inline(always)]
    pub fn ct125(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel125::Ct125)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved7(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel125::Reserved7)
    }
    #[doc = "Observation bus bit 13"]
    #[inline(always)]
    pub fn obsbus13(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel125::Obsbus13)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved9(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel125::Reserved9)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved10(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel125::Reserved10)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved11(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel125::Reserved11)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved12(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel125::Reserved12)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved13(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel125::Reserved13)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved14(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel125::Reserved14)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved15(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel125::Reserved15)
    }
}
#[doc = "Field `INPEN125` reader - Input enable for GPIO 125"]
pub type Inpen125R = crate::BitReader;
#[doc = "Field `INPEN125` writer - Input enable for GPIO 125"]
pub type Inpen125W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDZERO125` reader - Return 0 for read data on GPIO 125"]
pub type Rdzero125R = crate::BitReader;
#[doc = "Field `RDZERO125` writer - Return 0 for read data on GPIO 125"]
pub type Rdzero125W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Interrupt enable for GPIO 125\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Irpten125 {
    #[doc = "0: Interrupts are disabled for this GPIO"]
    Dis = 0,
    #[doc = "1: Interrupts are enabled for falling edge transition on this GPIO"]
    Intfall = 1,
    #[doc = "2: Interrupts are enabled for rising edge transitions on this GPIO"]
    Intrise = 2,
    #[doc = "3: Interrupts are enabled for any edge transition on this GPIO"]
    Intany = 3,
}
impl From<Irpten125> for u8 {
    #[inline(always)]
    fn from(variant: Irpten125) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Irpten125 {
    type Ux = u8;
}
impl crate::IsEnum for Irpten125 {}
#[doc = "Field `IRPTEN125` reader - Interrupt enable for GPIO 125"]
pub type Irpten125R = crate::FieldReader<Irpten125>;
impl Irpten125R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irpten125 {
        match self.bits {
            0 => Irpten125::Dis,
            1 => Irpten125::Intfall,
            2 => Irpten125::Intrise,
            3 => Irpten125::Intany,
            _ => unreachable!(),
        }
    }
    #[doc = "Interrupts are disabled for this GPIO"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Irpten125::Dis
    }
    #[doc = "Interrupts are enabled for falling edge transition on this GPIO"]
    #[inline(always)]
    pub fn is_intfall(&self) -> bool {
        *self == Irpten125::Intfall
    }
    #[doc = "Interrupts are enabled for rising edge transitions on this GPIO"]
    #[inline(always)]
    pub fn is_intrise(&self) -> bool {
        *self == Irpten125::Intrise
    }
    #[doc = "Interrupts are enabled for any edge transition on this GPIO"]
    #[inline(always)]
    pub fn is_intany(&self) -> bool {
        *self == Irpten125::Intany
    }
}
#[doc = "Field `IRPTEN125` writer - Interrupt enable for GPIO 125"]
pub type Irpten125W<'a, REG> = crate::FieldWriter<'a, REG, 2, Irpten125, crate::Safe>;
impl<'a, REG> Irpten125W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Interrupts are disabled for this GPIO"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten125::Dis)
    }
    #[doc = "Interrupts are enabled for falling edge transition on this GPIO"]
    #[inline(always)]
    pub fn intfall(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten125::Intfall)
    }
    #[doc = "Interrupts are enabled for rising edge transitions on this GPIO"]
    #[inline(always)]
    pub fn intrise(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten125::Intrise)
    }
    #[doc = "Interrupts are enabled for any edge transition on this GPIO"]
    #[inline(always)]
    pub fn intany(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten125::Intany)
    }
}
#[doc = "Pin IO mode selection for GPIO pin 125\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Outcfg125 {
    #[doc = "0: Output Disabled"]
    Dis = 0,
    #[doc = "1: Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    Pushpull = 1,
    #[doc = "2: Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    Od = 2,
    #[doc = "3: Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    Ts = 3,
}
impl From<Outcfg125> for u8 {
    #[inline(always)]
    fn from(variant: Outcfg125) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Outcfg125 {
    type Ux = u8;
}
impl crate::IsEnum for Outcfg125 {}
#[doc = "Field `OUTCFG125` reader - Pin IO mode selection for GPIO pin 125"]
pub type Outcfg125R = crate::FieldReader<Outcfg125>;
impl Outcfg125R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Outcfg125 {
        match self.bits {
            0 => Outcfg125::Dis,
            1 => Outcfg125::Pushpull,
            2 => Outcfg125::Od,
            3 => Outcfg125::Ts,
            _ => unreachable!(),
        }
    }
    #[doc = "Output Disabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Outcfg125::Dis
    }
    #[doc = "Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == Outcfg125::Pushpull
    }
    #[doc = "Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == Outcfg125::Od
    }
    #[doc = "Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == Outcfg125::Ts
    }
}
#[doc = "Field `OUTCFG125` writer - Pin IO mode selection for GPIO pin 125"]
pub type Outcfg125W<'a, REG> = crate::FieldWriter<'a, REG, 2, Outcfg125, crate::Safe>;
impl<'a, REG> Outcfg125W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Output Disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg125::Dis)
    }
    #[doc = "Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg125::Pushpull)
    }
    #[doc = "Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    #[inline(always)]
    pub fn od(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg125::Od)
    }
    #[doc = "Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    #[inline(always)]
    pub fn ts(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg125::Ts)
    }
}
impl R {
    #[doc = "Bits 0:3 - Function select for GPIO pin 125"]
    #[inline(always)]
    pub fn fncsel125(&self) -> Fncsel125R {
        Fncsel125R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Input enable for GPIO 125"]
    #[inline(always)]
    pub fn inpen125(&self) -> Inpen125R {
        Inpen125R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Return 0 for read data on GPIO 125"]
    #[inline(always)]
    pub fn rdzero125(&self) -> Rdzero125R {
        Rdzero125R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Interrupt enable for GPIO 125"]
    #[inline(always)]
    pub fn irpten125(&self) -> Irpten125R {
        Irpten125R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Pin IO mode selection for GPIO pin 125"]
    #[inline(always)]
    pub fn outcfg125(&self) -> Outcfg125R {
        Outcfg125R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Function select for GPIO pin 125"]
    #[inline(always)]
    #[must_use]
    pub fn fncsel125(&mut self) -> Fncsel125W<Pincfg125Spec> {
        Fncsel125W::new(self, 0)
    }
    #[doc = "Bit 4 - Input enable for GPIO 125"]
    #[inline(always)]
    #[must_use]
    pub fn inpen125(&mut self) -> Inpen125W<Pincfg125Spec> {
        Inpen125W::new(self, 4)
    }
    #[doc = "Bit 5 - Return 0 for read data on GPIO 125"]
    #[inline(always)]
    #[must_use]
    pub fn rdzero125(&mut self) -> Rdzero125W<Pincfg125Spec> {
        Rdzero125W::new(self, 5)
    }
    #[doc = "Bits 6:7 - Interrupt enable for GPIO 125"]
    #[inline(always)]
    #[must_use]
    pub fn irpten125(&mut self) -> Irpten125W<Pincfg125Spec> {
        Irpten125W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Pin IO mode selection for GPIO pin 125"]
    #[inline(always)]
    #[must_use]
    pub fn outcfg125(&mut self) -> Outcfg125W<Pincfg125Spec> {
        Outcfg125W::new(self, 8)
    }
}
#[doc = "Controls the operation of virtual GPIO pin 125.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg125::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg125::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pincfg125Spec;
impl crate::RegisterSpec for Pincfg125Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pincfg125::R`](R) reader structure"]
impl crate::Readable for Pincfg125Spec {}
#[doc = "`write(|w| ..)` method takes [`pincfg125::W`](W) writer structure"]
impl crate::Writable for Pincfg125Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PINCFG125 to value 0x03"]
impl crate::Resettable for Pincfg125Spec {
    const RESET_VALUE: u32 = 0x03;
}
