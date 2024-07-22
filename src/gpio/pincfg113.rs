#[doc = "Register `PINCFG113` reader"]
pub type R = crate::R<Pincfg113Spec>;
#[doc = "Register `PINCFG113` writer"]
pub type W = crate::W<Pincfg113Spec>;
#[doc = "Function select for GPIO pin 113\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fncsel113 {
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
    Ct113 = 6,
    #[doc = "7: Reserved selection. Operation unknown if selected."]
    Reserved7 = 7,
    #[doc = "8: Observation bus bit 1"]
    Obsbus1 = 8,
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
impl From<Fncsel113> for u8 {
    #[inline(always)]
    fn from(variant: Fncsel113) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fncsel113 {
    type Ux = u8;
}
impl crate::IsEnum for Fncsel113 {}
#[doc = "Field `FNCSEL113` reader - Function select for GPIO pin 113"]
pub type Fncsel113R = crate::FieldReader<Fncsel113>;
impl Fncsel113R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fncsel113 {
        match self.bits {
            0 => Fncsel113::Reserved0,
            1 => Fncsel113::Reserved1,
            2 => Fncsel113::Reserved2,
            3 => Fncsel113::Gpio,
            4 => Fncsel113::Reserved4,
            5 => Fncsel113::Reserved5,
            6 => Fncsel113::Ct113,
            7 => Fncsel113::Reserved7,
            8 => Fncsel113::Obsbus1,
            9 => Fncsel113::Reserved9,
            10 => Fncsel113::Reserved10,
            11 => Fncsel113::Reserved11,
            12 => Fncsel113::Reserved12,
            13 => Fncsel113::Reserved13,
            14 => Fncsel113::Reserved14,
            15 => Fncsel113::Reserved15,
            _ => unreachable!(),
        }
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved0(&self) -> bool {
        *self == Fncsel113::Reserved0
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved1(&self) -> bool {
        *self == Fncsel113::Reserved1
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == Fncsel113::Reserved2
    }
    #[doc = "General purpose I/O"]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == Fncsel113::Gpio
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved4(&self) -> bool {
        *self == Fncsel113::Reserved4
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved5(&self) -> bool {
        *self == Fncsel113::Reserved5
    }
    #[doc = "Timer/Counter input or output; Selection of direction is done via CTIMER register settings."]
    #[inline(always)]
    pub fn is_ct113(&self) -> bool {
        *self == Fncsel113::Ct113
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved7(&self) -> bool {
        *self == Fncsel113::Reserved7
    }
    #[doc = "Observation bus bit 1"]
    #[inline(always)]
    pub fn is_obsbus1(&self) -> bool {
        *self == Fncsel113::Obsbus1
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved9(&self) -> bool {
        *self == Fncsel113::Reserved9
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved10(&self) -> bool {
        *self == Fncsel113::Reserved10
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved11(&self) -> bool {
        *self == Fncsel113::Reserved11
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved12(&self) -> bool {
        *self == Fncsel113::Reserved12
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved13(&self) -> bool {
        *self == Fncsel113::Reserved13
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved14(&self) -> bool {
        *self == Fncsel113::Reserved14
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved15(&self) -> bool {
        *self == Fncsel113::Reserved15
    }
}
#[doc = "Field `FNCSEL113` writer - Function select for GPIO pin 113"]
pub type Fncsel113W<'a, REG> = crate::FieldWriter<'a, REG, 4, Fncsel113, crate::Safe>;
impl<'a, REG> Fncsel113W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved0(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel113::Reserved0)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved1(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel113::Reserved1)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel113::Reserved2)
    }
    #[doc = "General purpose I/O"]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel113::Gpio)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved4(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel113::Reserved4)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved5(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel113::Reserved5)
    }
    #[doc = "Timer/Counter input or output; Selection of direction is done via CTIMER register settings."]
    #[inline(always)]
    pub fn ct113(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel113::Ct113)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved7(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel113::Reserved7)
    }
    #[doc = "Observation bus bit 1"]
    #[inline(always)]
    pub fn obsbus1(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel113::Obsbus1)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved9(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel113::Reserved9)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved10(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel113::Reserved10)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved11(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel113::Reserved11)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved12(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel113::Reserved12)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved13(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel113::Reserved13)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved14(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel113::Reserved14)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved15(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel113::Reserved15)
    }
}
#[doc = "Field `INPEN113` reader - Input enable for GPIO 113"]
pub type Inpen113R = crate::BitReader;
#[doc = "Field `INPEN113` writer - Input enable for GPIO 113"]
pub type Inpen113W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDZERO113` reader - Return 0 for read data on GPIO 113"]
pub type Rdzero113R = crate::BitReader;
#[doc = "Field `RDZERO113` writer - Return 0 for read data on GPIO 113"]
pub type Rdzero113W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Interrupt enable for GPIO 113\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Irpten113 {
    #[doc = "0: Interrupts are disabled for this GPIO"]
    Dis = 0,
    #[doc = "1: Interrupts are enabled for falling edge transition on this GPIO"]
    Intfall = 1,
    #[doc = "2: Interrupts are enabled for rising edge transitions on this GPIO"]
    Intrise = 2,
    #[doc = "3: Interrupts are enabled for any edge transition on this GPIO"]
    Intany = 3,
}
impl From<Irpten113> for u8 {
    #[inline(always)]
    fn from(variant: Irpten113) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Irpten113 {
    type Ux = u8;
}
impl crate::IsEnum for Irpten113 {}
#[doc = "Field `IRPTEN113` reader - Interrupt enable for GPIO 113"]
pub type Irpten113R = crate::FieldReader<Irpten113>;
impl Irpten113R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irpten113 {
        match self.bits {
            0 => Irpten113::Dis,
            1 => Irpten113::Intfall,
            2 => Irpten113::Intrise,
            3 => Irpten113::Intany,
            _ => unreachable!(),
        }
    }
    #[doc = "Interrupts are disabled for this GPIO"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Irpten113::Dis
    }
    #[doc = "Interrupts are enabled for falling edge transition on this GPIO"]
    #[inline(always)]
    pub fn is_intfall(&self) -> bool {
        *self == Irpten113::Intfall
    }
    #[doc = "Interrupts are enabled for rising edge transitions on this GPIO"]
    #[inline(always)]
    pub fn is_intrise(&self) -> bool {
        *self == Irpten113::Intrise
    }
    #[doc = "Interrupts are enabled for any edge transition on this GPIO"]
    #[inline(always)]
    pub fn is_intany(&self) -> bool {
        *self == Irpten113::Intany
    }
}
#[doc = "Field `IRPTEN113` writer - Interrupt enable for GPIO 113"]
pub type Irpten113W<'a, REG> = crate::FieldWriter<'a, REG, 2, Irpten113, crate::Safe>;
impl<'a, REG> Irpten113W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Interrupts are disabled for this GPIO"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten113::Dis)
    }
    #[doc = "Interrupts are enabled for falling edge transition on this GPIO"]
    #[inline(always)]
    pub fn intfall(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten113::Intfall)
    }
    #[doc = "Interrupts are enabled for rising edge transitions on this GPIO"]
    #[inline(always)]
    pub fn intrise(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten113::Intrise)
    }
    #[doc = "Interrupts are enabled for any edge transition on this GPIO"]
    #[inline(always)]
    pub fn intany(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten113::Intany)
    }
}
#[doc = "Pin IO mode selection for GPIO pin 113\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Outcfg113 {
    #[doc = "0: Output Disabled"]
    Dis = 0,
    #[doc = "1: Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    Pushpull = 1,
    #[doc = "2: Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    Od = 2,
    #[doc = "3: Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    Ts = 3,
}
impl From<Outcfg113> for u8 {
    #[inline(always)]
    fn from(variant: Outcfg113) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Outcfg113 {
    type Ux = u8;
}
impl crate::IsEnum for Outcfg113 {}
#[doc = "Field `OUTCFG113` reader - Pin IO mode selection for GPIO pin 113"]
pub type Outcfg113R = crate::FieldReader<Outcfg113>;
impl Outcfg113R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Outcfg113 {
        match self.bits {
            0 => Outcfg113::Dis,
            1 => Outcfg113::Pushpull,
            2 => Outcfg113::Od,
            3 => Outcfg113::Ts,
            _ => unreachable!(),
        }
    }
    #[doc = "Output Disabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Outcfg113::Dis
    }
    #[doc = "Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == Outcfg113::Pushpull
    }
    #[doc = "Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == Outcfg113::Od
    }
    #[doc = "Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == Outcfg113::Ts
    }
}
#[doc = "Field `OUTCFG113` writer - Pin IO mode selection for GPIO pin 113"]
pub type Outcfg113W<'a, REG> = crate::FieldWriter<'a, REG, 2, Outcfg113, crate::Safe>;
impl<'a, REG> Outcfg113W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Output Disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg113::Dis)
    }
    #[doc = "Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg113::Pushpull)
    }
    #[doc = "Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    #[inline(always)]
    pub fn od(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg113::Od)
    }
    #[doc = "Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    #[inline(always)]
    pub fn ts(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg113::Ts)
    }
}
impl R {
    #[doc = "Bits 0:3 - Function select for GPIO pin 113"]
    #[inline(always)]
    pub fn fncsel113(&self) -> Fncsel113R {
        Fncsel113R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Input enable for GPIO 113"]
    #[inline(always)]
    pub fn inpen113(&self) -> Inpen113R {
        Inpen113R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Return 0 for read data on GPIO 113"]
    #[inline(always)]
    pub fn rdzero113(&self) -> Rdzero113R {
        Rdzero113R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Interrupt enable for GPIO 113"]
    #[inline(always)]
    pub fn irpten113(&self) -> Irpten113R {
        Irpten113R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Pin IO mode selection for GPIO pin 113"]
    #[inline(always)]
    pub fn outcfg113(&self) -> Outcfg113R {
        Outcfg113R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Function select for GPIO pin 113"]
    #[inline(always)]
    #[must_use]
    pub fn fncsel113(&mut self) -> Fncsel113W<Pincfg113Spec> {
        Fncsel113W::new(self, 0)
    }
    #[doc = "Bit 4 - Input enable for GPIO 113"]
    #[inline(always)]
    #[must_use]
    pub fn inpen113(&mut self) -> Inpen113W<Pincfg113Spec> {
        Inpen113W::new(self, 4)
    }
    #[doc = "Bit 5 - Return 0 for read data on GPIO 113"]
    #[inline(always)]
    #[must_use]
    pub fn rdzero113(&mut self) -> Rdzero113W<Pincfg113Spec> {
        Rdzero113W::new(self, 5)
    }
    #[doc = "Bits 6:7 - Interrupt enable for GPIO 113"]
    #[inline(always)]
    #[must_use]
    pub fn irpten113(&mut self) -> Irpten113W<Pincfg113Spec> {
        Irpten113W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Pin IO mode selection for GPIO pin 113"]
    #[inline(always)]
    #[must_use]
    pub fn outcfg113(&mut self) -> Outcfg113W<Pincfg113Spec> {
        Outcfg113W::new(self, 8)
    }
}
#[doc = "Controls the operation of virtual GPIO pin 113.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg113::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg113::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pincfg113Spec;
impl crate::RegisterSpec for Pincfg113Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pincfg113::R`](R) reader structure"]
impl crate::Readable for Pincfg113Spec {}
#[doc = "`write(|w| ..)` method takes [`pincfg113::W`](W) writer structure"]
impl crate::Writable for Pincfg113Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PINCFG113 to value 0x03"]
impl crate::Resettable for Pincfg113Spec {
    const RESET_VALUE: u32 = 0x03;
}
