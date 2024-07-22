#[doc = "Register `PINCFG124` reader"]
pub type R = crate::R<Pincfg124Spec>;
#[doc = "Register `PINCFG124` writer"]
pub type W = crate::W<Pincfg124Spec>;
#[doc = "Function select for GPIO pin 124\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fncsel124 {
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
    Ct124 = 6,
    #[doc = "7: Reserved selection. Operation unknown if selected."]
    Reserved7 = 7,
    #[doc = "8: Observation bus bit 12"]
    Obsbus12 = 8,
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
impl From<Fncsel124> for u8 {
    #[inline(always)]
    fn from(variant: Fncsel124) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fncsel124 {
    type Ux = u8;
}
impl crate::IsEnum for Fncsel124 {}
#[doc = "Field `FNCSEL124` reader - Function select for GPIO pin 124"]
pub type Fncsel124R = crate::FieldReader<Fncsel124>;
impl Fncsel124R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fncsel124 {
        match self.bits {
            0 => Fncsel124::Reserved0,
            1 => Fncsel124::Reserved1,
            2 => Fncsel124::Reserved2,
            3 => Fncsel124::Gpio,
            4 => Fncsel124::Reserved4,
            5 => Fncsel124::Reserved5,
            6 => Fncsel124::Ct124,
            7 => Fncsel124::Reserved7,
            8 => Fncsel124::Obsbus12,
            9 => Fncsel124::Reserved9,
            10 => Fncsel124::Reserved10,
            11 => Fncsel124::Reserved11,
            12 => Fncsel124::Reserved12,
            13 => Fncsel124::Reserved13,
            14 => Fncsel124::Reserved14,
            15 => Fncsel124::Reserved15,
            _ => unreachable!(),
        }
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved0(&self) -> bool {
        *self == Fncsel124::Reserved0
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved1(&self) -> bool {
        *self == Fncsel124::Reserved1
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == Fncsel124::Reserved2
    }
    #[doc = "General purpose I/O"]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == Fncsel124::Gpio
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved4(&self) -> bool {
        *self == Fncsel124::Reserved4
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved5(&self) -> bool {
        *self == Fncsel124::Reserved5
    }
    #[doc = "Timer/Counter input or output; Selection of direction is done via CTIMER register settings."]
    #[inline(always)]
    pub fn is_ct124(&self) -> bool {
        *self == Fncsel124::Ct124
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved7(&self) -> bool {
        *self == Fncsel124::Reserved7
    }
    #[doc = "Observation bus bit 12"]
    #[inline(always)]
    pub fn is_obsbus12(&self) -> bool {
        *self == Fncsel124::Obsbus12
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved9(&self) -> bool {
        *self == Fncsel124::Reserved9
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved10(&self) -> bool {
        *self == Fncsel124::Reserved10
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved11(&self) -> bool {
        *self == Fncsel124::Reserved11
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved12(&self) -> bool {
        *self == Fncsel124::Reserved12
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved13(&self) -> bool {
        *self == Fncsel124::Reserved13
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved14(&self) -> bool {
        *self == Fncsel124::Reserved14
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved15(&self) -> bool {
        *self == Fncsel124::Reserved15
    }
}
#[doc = "Field `FNCSEL124` writer - Function select for GPIO pin 124"]
pub type Fncsel124W<'a, REG> = crate::FieldWriter<'a, REG, 4, Fncsel124, crate::Safe>;
impl<'a, REG> Fncsel124W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved0(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel124::Reserved0)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved1(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel124::Reserved1)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel124::Reserved2)
    }
    #[doc = "General purpose I/O"]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel124::Gpio)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved4(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel124::Reserved4)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved5(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel124::Reserved5)
    }
    #[doc = "Timer/Counter input or output; Selection of direction is done via CTIMER register settings."]
    #[inline(always)]
    pub fn ct124(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel124::Ct124)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved7(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel124::Reserved7)
    }
    #[doc = "Observation bus bit 12"]
    #[inline(always)]
    pub fn obsbus12(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel124::Obsbus12)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved9(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel124::Reserved9)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved10(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel124::Reserved10)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved11(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel124::Reserved11)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved12(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel124::Reserved12)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved13(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel124::Reserved13)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved14(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel124::Reserved14)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved15(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel124::Reserved15)
    }
}
#[doc = "Field `INPEN124` reader - Input enable for GPIO 124"]
pub type Inpen124R = crate::BitReader;
#[doc = "Field `INPEN124` writer - Input enable for GPIO 124"]
pub type Inpen124W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDZERO124` reader - Return 0 for read data on GPIO 124"]
pub type Rdzero124R = crate::BitReader;
#[doc = "Field `RDZERO124` writer - Return 0 for read data on GPIO 124"]
pub type Rdzero124W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Interrupt enable for GPIO 124\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Irpten124 {
    #[doc = "0: Interrupts are disabled for this GPIO"]
    Dis = 0,
    #[doc = "1: Interrupts are enabled for falling edge transition on this GPIO"]
    Intfall = 1,
    #[doc = "2: Interrupts are enabled for rising edge transitions on this GPIO"]
    Intrise = 2,
    #[doc = "3: Interrupts are enabled for any edge transition on this GPIO"]
    Intany = 3,
}
impl From<Irpten124> for u8 {
    #[inline(always)]
    fn from(variant: Irpten124) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Irpten124 {
    type Ux = u8;
}
impl crate::IsEnum for Irpten124 {}
#[doc = "Field `IRPTEN124` reader - Interrupt enable for GPIO 124"]
pub type Irpten124R = crate::FieldReader<Irpten124>;
impl Irpten124R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irpten124 {
        match self.bits {
            0 => Irpten124::Dis,
            1 => Irpten124::Intfall,
            2 => Irpten124::Intrise,
            3 => Irpten124::Intany,
            _ => unreachable!(),
        }
    }
    #[doc = "Interrupts are disabled for this GPIO"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Irpten124::Dis
    }
    #[doc = "Interrupts are enabled for falling edge transition on this GPIO"]
    #[inline(always)]
    pub fn is_intfall(&self) -> bool {
        *self == Irpten124::Intfall
    }
    #[doc = "Interrupts are enabled for rising edge transitions on this GPIO"]
    #[inline(always)]
    pub fn is_intrise(&self) -> bool {
        *self == Irpten124::Intrise
    }
    #[doc = "Interrupts are enabled for any edge transition on this GPIO"]
    #[inline(always)]
    pub fn is_intany(&self) -> bool {
        *self == Irpten124::Intany
    }
}
#[doc = "Field `IRPTEN124` writer - Interrupt enable for GPIO 124"]
pub type Irpten124W<'a, REG> = crate::FieldWriter<'a, REG, 2, Irpten124, crate::Safe>;
impl<'a, REG> Irpten124W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Interrupts are disabled for this GPIO"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten124::Dis)
    }
    #[doc = "Interrupts are enabled for falling edge transition on this GPIO"]
    #[inline(always)]
    pub fn intfall(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten124::Intfall)
    }
    #[doc = "Interrupts are enabled for rising edge transitions on this GPIO"]
    #[inline(always)]
    pub fn intrise(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten124::Intrise)
    }
    #[doc = "Interrupts are enabled for any edge transition on this GPIO"]
    #[inline(always)]
    pub fn intany(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten124::Intany)
    }
}
#[doc = "Pin IO mode selection for GPIO pin 124\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Outcfg124 {
    #[doc = "0: Output Disabled"]
    Dis = 0,
    #[doc = "1: Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    Pushpull = 1,
    #[doc = "2: Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    Od = 2,
    #[doc = "3: Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    Ts = 3,
}
impl From<Outcfg124> for u8 {
    #[inline(always)]
    fn from(variant: Outcfg124) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Outcfg124 {
    type Ux = u8;
}
impl crate::IsEnum for Outcfg124 {}
#[doc = "Field `OUTCFG124` reader - Pin IO mode selection for GPIO pin 124"]
pub type Outcfg124R = crate::FieldReader<Outcfg124>;
impl Outcfg124R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Outcfg124 {
        match self.bits {
            0 => Outcfg124::Dis,
            1 => Outcfg124::Pushpull,
            2 => Outcfg124::Od,
            3 => Outcfg124::Ts,
            _ => unreachable!(),
        }
    }
    #[doc = "Output Disabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Outcfg124::Dis
    }
    #[doc = "Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == Outcfg124::Pushpull
    }
    #[doc = "Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == Outcfg124::Od
    }
    #[doc = "Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == Outcfg124::Ts
    }
}
#[doc = "Field `OUTCFG124` writer - Pin IO mode selection for GPIO pin 124"]
pub type Outcfg124W<'a, REG> = crate::FieldWriter<'a, REG, 2, Outcfg124, crate::Safe>;
impl<'a, REG> Outcfg124W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Output Disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg124::Dis)
    }
    #[doc = "Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg124::Pushpull)
    }
    #[doc = "Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    #[inline(always)]
    pub fn od(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg124::Od)
    }
    #[doc = "Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    #[inline(always)]
    pub fn ts(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg124::Ts)
    }
}
impl R {
    #[doc = "Bits 0:3 - Function select for GPIO pin 124"]
    #[inline(always)]
    pub fn fncsel124(&self) -> Fncsel124R {
        Fncsel124R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Input enable for GPIO 124"]
    #[inline(always)]
    pub fn inpen124(&self) -> Inpen124R {
        Inpen124R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Return 0 for read data on GPIO 124"]
    #[inline(always)]
    pub fn rdzero124(&self) -> Rdzero124R {
        Rdzero124R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Interrupt enable for GPIO 124"]
    #[inline(always)]
    pub fn irpten124(&self) -> Irpten124R {
        Irpten124R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Pin IO mode selection for GPIO pin 124"]
    #[inline(always)]
    pub fn outcfg124(&self) -> Outcfg124R {
        Outcfg124R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Function select for GPIO pin 124"]
    #[inline(always)]
    #[must_use]
    pub fn fncsel124(&mut self) -> Fncsel124W<Pincfg124Spec> {
        Fncsel124W::new(self, 0)
    }
    #[doc = "Bit 4 - Input enable for GPIO 124"]
    #[inline(always)]
    #[must_use]
    pub fn inpen124(&mut self) -> Inpen124W<Pincfg124Spec> {
        Inpen124W::new(self, 4)
    }
    #[doc = "Bit 5 - Return 0 for read data on GPIO 124"]
    #[inline(always)]
    #[must_use]
    pub fn rdzero124(&mut self) -> Rdzero124W<Pincfg124Spec> {
        Rdzero124W::new(self, 5)
    }
    #[doc = "Bits 6:7 - Interrupt enable for GPIO 124"]
    #[inline(always)]
    #[must_use]
    pub fn irpten124(&mut self) -> Irpten124W<Pincfg124Spec> {
        Irpten124W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Pin IO mode selection for GPIO pin 124"]
    #[inline(always)]
    #[must_use]
    pub fn outcfg124(&mut self) -> Outcfg124W<Pincfg124Spec> {
        Outcfg124W::new(self, 8)
    }
}
#[doc = "Controls the operation of virtual GPIO pin 124.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg124::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg124::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pincfg124Spec;
impl crate::RegisterSpec for Pincfg124Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pincfg124::R`](R) reader structure"]
impl crate::Readable for Pincfg124Spec {}
#[doc = "`write(|w| ..)` method takes [`pincfg124::W`](W) writer structure"]
impl crate::Writable for Pincfg124Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PINCFG124 to value 0x03"]
impl crate::Resettable for Pincfg124Spec {
    const RESET_VALUE: u32 = 0x03;
}
