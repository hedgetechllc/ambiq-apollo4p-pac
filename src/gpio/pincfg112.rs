#[doc = "Register `PINCFG112` reader"]
pub type R = crate::R<Pincfg112Spec>;
#[doc = "Register `PINCFG112` writer"]
pub type W = crate::W<Pincfg112Spec>;
#[doc = "Function select for GPIO pin 112\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fncsel112 {
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
    Ct112 = 6,
    #[doc = "7: Reserved selection. Operation unknown if selected."]
    Reserved7 = 7,
    #[doc = "8: Observation bus bit 0"]
    Obsbus0 = 8,
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
impl From<Fncsel112> for u8 {
    #[inline(always)]
    fn from(variant: Fncsel112) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fncsel112 {
    type Ux = u8;
}
impl crate::IsEnum for Fncsel112 {}
#[doc = "Field `FNCSEL112` reader - Function select for GPIO pin 112"]
pub type Fncsel112R = crate::FieldReader<Fncsel112>;
impl Fncsel112R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fncsel112 {
        match self.bits {
            0 => Fncsel112::Reserved0,
            1 => Fncsel112::Reserved1,
            2 => Fncsel112::Reserved2,
            3 => Fncsel112::Gpio,
            4 => Fncsel112::Reserved4,
            5 => Fncsel112::Reserved5,
            6 => Fncsel112::Ct112,
            7 => Fncsel112::Reserved7,
            8 => Fncsel112::Obsbus0,
            9 => Fncsel112::Reserved9,
            10 => Fncsel112::Reserved10,
            11 => Fncsel112::Reserved11,
            12 => Fncsel112::Reserved12,
            13 => Fncsel112::Reserved13,
            14 => Fncsel112::Reserved14,
            15 => Fncsel112::Reserved15,
            _ => unreachable!(),
        }
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved0(&self) -> bool {
        *self == Fncsel112::Reserved0
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved1(&self) -> bool {
        *self == Fncsel112::Reserved1
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == Fncsel112::Reserved2
    }
    #[doc = "General purpose I/O"]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == Fncsel112::Gpio
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved4(&self) -> bool {
        *self == Fncsel112::Reserved4
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved5(&self) -> bool {
        *self == Fncsel112::Reserved5
    }
    #[doc = "Timer/Counter input or output; Selection of direction is done via CTIMER register settings."]
    #[inline(always)]
    pub fn is_ct112(&self) -> bool {
        *self == Fncsel112::Ct112
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved7(&self) -> bool {
        *self == Fncsel112::Reserved7
    }
    #[doc = "Observation bus bit 0"]
    #[inline(always)]
    pub fn is_obsbus0(&self) -> bool {
        *self == Fncsel112::Obsbus0
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved9(&self) -> bool {
        *self == Fncsel112::Reserved9
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved10(&self) -> bool {
        *self == Fncsel112::Reserved10
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved11(&self) -> bool {
        *self == Fncsel112::Reserved11
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved12(&self) -> bool {
        *self == Fncsel112::Reserved12
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved13(&self) -> bool {
        *self == Fncsel112::Reserved13
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved14(&self) -> bool {
        *self == Fncsel112::Reserved14
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved15(&self) -> bool {
        *self == Fncsel112::Reserved15
    }
}
#[doc = "Field `FNCSEL112` writer - Function select for GPIO pin 112"]
pub type Fncsel112W<'a, REG> = crate::FieldWriter<'a, REG, 4, Fncsel112, crate::Safe>;
impl<'a, REG> Fncsel112W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved0(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel112::Reserved0)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved1(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel112::Reserved1)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel112::Reserved2)
    }
    #[doc = "General purpose I/O"]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel112::Gpio)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved4(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel112::Reserved4)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved5(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel112::Reserved5)
    }
    #[doc = "Timer/Counter input or output; Selection of direction is done via CTIMER register settings."]
    #[inline(always)]
    pub fn ct112(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel112::Ct112)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved7(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel112::Reserved7)
    }
    #[doc = "Observation bus bit 0"]
    #[inline(always)]
    pub fn obsbus0(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel112::Obsbus0)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved9(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel112::Reserved9)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved10(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel112::Reserved10)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved11(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel112::Reserved11)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved12(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel112::Reserved12)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved13(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel112::Reserved13)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved14(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel112::Reserved14)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved15(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel112::Reserved15)
    }
}
#[doc = "Field `INPEN112` reader - Input enable for GPIO 112"]
pub type Inpen112R = crate::BitReader;
#[doc = "Field `INPEN112` writer - Input enable for GPIO 112"]
pub type Inpen112W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDZERO112` reader - Return 0 for read data on GPIO 112"]
pub type Rdzero112R = crate::BitReader;
#[doc = "Field `RDZERO112` writer - Return 0 for read data on GPIO 112"]
pub type Rdzero112W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Interrupt enable for GPIO 112\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Irpten112 {
    #[doc = "0: Interrupts are disabled for this GPIO"]
    Dis = 0,
    #[doc = "1: Interrupts are enabled for falling edge transition on this GPIO"]
    Intfall = 1,
    #[doc = "2: Interrupts are enabled for rising edge transitions on this GPIO"]
    Intrise = 2,
    #[doc = "3: Interrupts are enabled for any edge transition on this GPIO"]
    Intany = 3,
}
impl From<Irpten112> for u8 {
    #[inline(always)]
    fn from(variant: Irpten112) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Irpten112 {
    type Ux = u8;
}
impl crate::IsEnum for Irpten112 {}
#[doc = "Field `IRPTEN112` reader - Interrupt enable for GPIO 112"]
pub type Irpten112R = crate::FieldReader<Irpten112>;
impl Irpten112R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irpten112 {
        match self.bits {
            0 => Irpten112::Dis,
            1 => Irpten112::Intfall,
            2 => Irpten112::Intrise,
            3 => Irpten112::Intany,
            _ => unreachable!(),
        }
    }
    #[doc = "Interrupts are disabled for this GPIO"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Irpten112::Dis
    }
    #[doc = "Interrupts are enabled for falling edge transition on this GPIO"]
    #[inline(always)]
    pub fn is_intfall(&self) -> bool {
        *self == Irpten112::Intfall
    }
    #[doc = "Interrupts are enabled for rising edge transitions on this GPIO"]
    #[inline(always)]
    pub fn is_intrise(&self) -> bool {
        *self == Irpten112::Intrise
    }
    #[doc = "Interrupts are enabled for any edge transition on this GPIO"]
    #[inline(always)]
    pub fn is_intany(&self) -> bool {
        *self == Irpten112::Intany
    }
}
#[doc = "Field `IRPTEN112` writer - Interrupt enable for GPIO 112"]
pub type Irpten112W<'a, REG> = crate::FieldWriter<'a, REG, 2, Irpten112, crate::Safe>;
impl<'a, REG> Irpten112W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Interrupts are disabled for this GPIO"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten112::Dis)
    }
    #[doc = "Interrupts are enabled for falling edge transition on this GPIO"]
    #[inline(always)]
    pub fn intfall(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten112::Intfall)
    }
    #[doc = "Interrupts are enabled for rising edge transitions on this GPIO"]
    #[inline(always)]
    pub fn intrise(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten112::Intrise)
    }
    #[doc = "Interrupts are enabled for any edge transition on this GPIO"]
    #[inline(always)]
    pub fn intany(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten112::Intany)
    }
}
#[doc = "Pin IO mode selection for GPIO pin 112\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Outcfg112 {
    #[doc = "0: Output Disabled"]
    Dis = 0,
    #[doc = "1: Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    Pushpull = 1,
    #[doc = "2: Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    Od = 2,
    #[doc = "3: Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    Ts = 3,
}
impl From<Outcfg112> for u8 {
    #[inline(always)]
    fn from(variant: Outcfg112) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Outcfg112 {
    type Ux = u8;
}
impl crate::IsEnum for Outcfg112 {}
#[doc = "Field `OUTCFG112` reader - Pin IO mode selection for GPIO pin 112"]
pub type Outcfg112R = crate::FieldReader<Outcfg112>;
impl Outcfg112R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Outcfg112 {
        match self.bits {
            0 => Outcfg112::Dis,
            1 => Outcfg112::Pushpull,
            2 => Outcfg112::Od,
            3 => Outcfg112::Ts,
            _ => unreachable!(),
        }
    }
    #[doc = "Output Disabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Outcfg112::Dis
    }
    #[doc = "Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == Outcfg112::Pushpull
    }
    #[doc = "Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == Outcfg112::Od
    }
    #[doc = "Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == Outcfg112::Ts
    }
}
#[doc = "Field `OUTCFG112` writer - Pin IO mode selection for GPIO pin 112"]
pub type Outcfg112W<'a, REG> = crate::FieldWriter<'a, REG, 2, Outcfg112, crate::Safe>;
impl<'a, REG> Outcfg112W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Output Disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg112::Dis)
    }
    #[doc = "Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg112::Pushpull)
    }
    #[doc = "Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    #[inline(always)]
    pub fn od(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg112::Od)
    }
    #[doc = "Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    #[inline(always)]
    pub fn ts(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg112::Ts)
    }
}
impl R {
    #[doc = "Bits 0:3 - Function select for GPIO pin 112"]
    #[inline(always)]
    pub fn fncsel112(&self) -> Fncsel112R {
        Fncsel112R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Input enable for GPIO 112"]
    #[inline(always)]
    pub fn inpen112(&self) -> Inpen112R {
        Inpen112R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Return 0 for read data on GPIO 112"]
    #[inline(always)]
    pub fn rdzero112(&self) -> Rdzero112R {
        Rdzero112R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Interrupt enable for GPIO 112"]
    #[inline(always)]
    pub fn irpten112(&self) -> Irpten112R {
        Irpten112R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Pin IO mode selection for GPIO pin 112"]
    #[inline(always)]
    pub fn outcfg112(&self) -> Outcfg112R {
        Outcfg112R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Function select for GPIO pin 112"]
    #[inline(always)]
    #[must_use]
    pub fn fncsel112(&mut self) -> Fncsel112W<Pincfg112Spec> {
        Fncsel112W::new(self, 0)
    }
    #[doc = "Bit 4 - Input enable for GPIO 112"]
    #[inline(always)]
    #[must_use]
    pub fn inpen112(&mut self) -> Inpen112W<Pincfg112Spec> {
        Inpen112W::new(self, 4)
    }
    #[doc = "Bit 5 - Return 0 for read data on GPIO 112"]
    #[inline(always)]
    #[must_use]
    pub fn rdzero112(&mut self) -> Rdzero112W<Pincfg112Spec> {
        Rdzero112W::new(self, 5)
    }
    #[doc = "Bits 6:7 - Interrupt enable for GPIO 112"]
    #[inline(always)]
    #[must_use]
    pub fn irpten112(&mut self) -> Irpten112W<Pincfg112Spec> {
        Irpten112W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Pin IO mode selection for GPIO pin 112"]
    #[inline(always)]
    #[must_use]
    pub fn outcfg112(&mut self) -> Outcfg112W<Pincfg112Spec> {
        Outcfg112W::new(self, 8)
    }
}
#[doc = "Controls the operation of virtual GPIO pin 112.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg112::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg112::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pincfg112Spec;
impl crate::RegisterSpec for Pincfg112Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pincfg112::R`](R) reader structure"]
impl crate::Readable for Pincfg112Spec {}
#[doc = "`write(|w| ..)` method takes [`pincfg112::W`](W) writer structure"]
impl crate::Writable for Pincfg112Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PINCFG112 to value 0x03"]
impl crate::Resettable for Pincfg112Spec {
    const RESET_VALUE: u32 = 0x03;
}
