#[doc = "Register `PINCFG108` reader"]
pub type R = crate::R<Pincfg108Spec>;
#[doc = "Register `PINCFG108` writer"]
pub type W = crate::W<Pincfg108Spec>;
#[doc = "Function select for GPIO pin 108\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fncsel108 {
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
    Ct108 = 6,
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
impl From<Fncsel108> for u8 {
    #[inline(always)]
    fn from(variant: Fncsel108) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fncsel108 {
    type Ux = u8;
}
impl crate::IsEnum for Fncsel108 {}
#[doc = "Field `FNCSEL108` reader - Function select for GPIO pin 108"]
pub type Fncsel108R = crate::FieldReader<Fncsel108>;
impl Fncsel108R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fncsel108 {
        match self.bits {
            0 => Fncsel108::Reserved0,
            1 => Fncsel108::Reserved1,
            2 => Fncsel108::Reserved2,
            3 => Fncsel108::Gpio,
            4 => Fncsel108::Reserved4,
            5 => Fncsel108::Reserved5,
            6 => Fncsel108::Ct108,
            7 => Fncsel108::Reserved7,
            8 => Fncsel108::Obsbus12,
            9 => Fncsel108::Reserved9,
            10 => Fncsel108::Reserved10,
            11 => Fncsel108::Reserved11,
            12 => Fncsel108::Reserved12,
            13 => Fncsel108::Reserved13,
            14 => Fncsel108::Reserved14,
            15 => Fncsel108::Reserved15,
            _ => unreachable!(),
        }
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved0(&self) -> bool {
        *self == Fncsel108::Reserved0
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved1(&self) -> bool {
        *self == Fncsel108::Reserved1
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == Fncsel108::Reserved2
    }
    #[doc = "General purpose I/O"]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == Fncsel108::Gpio
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved4(&self) -> bool {
        *self == Fncsel108::Reserved4
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved5(&self) -> bool {
        *self == Fncsel108::Reserved5
    }
    #[doc = "Timer/Counter input or output; Selection of direction is done via CTIMER register settings."]
    #[inline(always)]
    pub fn is_ct108(&self) -> bool {
        *self == Fncsel108::Ct108
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved7(&self) -> bool {
        *self == Fncsel108::Reserved7
    }
    #[doc = "Observation bus bit 12"]
    #[inline(always)]
    pub fn is_obsbus12(&self) -> bool {
        *self == Fncsel108::Obsbus12
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved9(&self) -> bool {
        *self == Fncsel108::Reserved9
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved10(&self) -> bool {
        *self == Fncsel108::Reserved10
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved11(&self) -> bool {
        *self == Fncsel108::Reserved11
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved12(&self) -> bool {
        *self == Fncsel108::Reserved12
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved13(&self) -> bool {
        *self == Fncsel108::Reserved13
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved14(&self) -> bool {
        *self == Fncsel108::Reserved14
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved15(&self) -> bool {
        *self == Fncsel108::Reserved15
    }
}
#[doc = "Field `FNCSEL108` writer - Function select for GPIO pin 108"]
pub type Fncsel108W<'a, REG> = crate::FieldWriter<'a, REG, 4, Fncsel108, crate::Safe>;
impl<'a, REG> Fncsel108W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved0(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel108::Reserved0)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved1(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel108::Reserved1)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel108::Reserved2)
    }
    #[doc = "General purpose I/O"]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel108::Gpio)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved4(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel108::Reserved4)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved5(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel108::Reserved5)
    }
    #[doc = "Timer/Counter input or output; Selection of direction is done via CTIMER register settings."]
    #[inline(always)]
    pub fn ct108(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel108::Ct108)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved7(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel108::Reserved7)
    }
    #[doc = "Observation bus bit 12"]
    #[inline(always)]
    pub fn obsbus12(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel108::Obsbus12)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved9(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel108::Reserved9)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved10(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel108::Reserved10)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved11(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel108::Reserved11)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved12(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel108::Reserved12)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved13(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel108::Reserved13)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved14(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel108::Reserved14)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved15(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel108::Reserved15)
    }
}
#[doc = "Field `INPEN108` reader - Input enable for GPIO 108"]
pub type Inpen108R = crate::BitReader;
#[doc = "Field `INPEN108` writer - Input enable for GPIO 108"]
pub type Inpen108W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDZERO108` reader - Return 0 for read data on GPIO 108"]
pub type Rdzero108R = crate::BitReader;
#[doc = "Field `RDZERO108` writer - Return 0 for read data on GPIO 108"]
pub type Rdzero108W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Interrupt enable for GPIO 108\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Irpten108 {
    #[doc = "0: Interrupts are disabled for this GPIO"]
    Dis = 0,
    #[doc = "1: Interrupts are enabled for falling edge transition on this GPIO"]
    Intfall = 1,
    #[doc = "2: Interrupts are enabled for rising edge transitions on this GPIO"]
    Intrise = 2,
    #[doc = "3: Interrupts are enabled for any edge transition on this GPIO"]
    Intany = 3,
}
impl From<Irpten108> for u8 {
    #[inline(always)]
    fn from(variant: Irpten108) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Irpten108 {
    type Ux = u8;
}
impl crate::IsEnum for Irpten108 {}
#[doc = "Field `IRPTEN108` reader - Interrupt enable for GPIO 108"]
pub type Irpten108R = crate::FieldReader<Irpten108>;
impl Irpten108R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irpten108 {
        match self.bits {
            0 => Irpten108::Dis,
            1 => Irpten108::Intfall,
            2 => Irpten108::Intrise,
            3 => Irpten108::Intany,
            _ => unreachable!(),
        }
    }
    #[doc = "Interrupts are disabled for this GPIO"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Irpten108::Dis
    }
    #[doc = "Interrupts are enabled for falling edge transition on this GPIO"]
    #[inline(always)]
    pub fn is_intfall(&self) -> bool {
        *self == Irpten108::Intfall
    }
    #[doc = "Interrupts are enabled for rising edge transitions on this GPIO"]
    #[inline(always)]
    pub fn is_intrise(&self) -> bool {
        *self == Irpten108::Intrise
    }
    #[doc = "Interrupts are enabled for any edge transition on this GPIO"]
    #[inline(always)]
    pub fn is_intany(&self) -> bool {
        *self == Irpten108::Intany
    }
}
#[doc = "Field `IRPTEN108` writer - Interrupt enable for GPIO 108"]
pub type Irpten108W<'a, REG> = crate::FieldWriter<'a, REG, 2, Irpten108, crate::Safe>;
impl<'a, REG> Irpten108W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Interrupts are disabled for this GPIO"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten108::Dis)
    }
    #[doc = "Interrupts are enabled for falling edge transition on this GPIO"]
    #[inline(always)]
    pub fn intfall(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten108::Intfall)
    }
    #[doc = "Interrupts are enabled for rising edge transitions on this GPIO"]
    #[inline(always)]
    pub fn intrise(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten108::Intrise)
    }
    #[doc = "Interrupts are enabled for any edge transition on this GPIO"]
    #[inline(always)]
    pub fn intany(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten108::Intany)
    }
}
#[doc = "Pin IO mode selection for GPIO pin 108\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Outcfg108 {
    #[doc = "0: Output Disabled"]
    Dis = 0,
    #[doc = "1: Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    Pushpull = 1,
    #[doc = "2: Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    Od = 2,
    #[doc = "3: Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    Ts = 3,
}
impl From<Outcfg108> for u8 {
    #[inline(always)]
    fn from(variant: Outcfg108) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Outcfg108 {
    type Ux = u8;
}
impl crate::IsEnum for Outcfg108 {}
#[doc = "Field `OUTCFG108` reader - Pin IO mode selection for GPIO pin 108"]
pub type Outcfg108R = crate::FieldReader<Outcfg108>;
impl Outcfg108R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Outcfg108 {
        match self.bits {
            0 => Outcfg108::Dis,
            1 => Outcfg108::Pushpull,
            2 => Outcfg108::Od,
            3 => Outcfg108::Ts,
            _ => unreachable!(),
        }
    }
    #[doc = "Output Disabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Outcfg108::Dis
    }
    #[doc = "Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == Outcfg108::Pushpull
    }
    #[doc = "Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == Outcfg108::Od
    }
    #[doc = "Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == Outcfg108::Ts
    }
}
#[doc = "Field `OUTCFG108` writer - Pin IO mode selection for GPIO pin 108"]
pub type Outcfg108W<'a, REG> = crate::FieldWriter<'a, REG, 2, Outcfg108, crate::Safe>;
impl<'a, REG> Outcfg108W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Output Disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg108::Dis)
    }
    #[doc = "Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg108::Pushpull)
    }
    #[doc = "Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    #[inline(always)]
    pub fn od(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg108::Od)
    }
    #[doc = "Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    #[inline(always)]
    pub fn ts(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg108::Ts)
    }
}
impl R {
    #[doc = "Bits 0:3 - Function select for GPIO pin 108"]
    #[inline(always)]
    pub fn fncsel108(&self) -> Fncsel108R {
        Fncsel108R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Input enable for GPIO 108"]
    #[inline(always)]
    pub fn inpen108(&self) -> Inpen108R {
        Inpen108R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Return 0 for read data on GPIO 108"]
    #[inline(always)]
    pub fn rdzero108(&self) -> Rdzero108R {
        Rdzero108R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Interrupt enable for GPIO 108"]
    #[inline(always)]
    pub fn irpten108(&self) -> Irpten108R {
        Irpten108R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Pin IO mode selection for GPIO pin 108"]
    #[inline(always)]
    pub fn outcfg108(&self) -> Outcfg108R {
        Outcfg108R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Function select for GPIO pin 108"]
    #[inline(always)]
    #[must_use]
    pub fn fncsel108(&mut self) -> Fncsel108W<Pincfg108Spec> {
        Fncsel108W::new(self, 0)
    }
    #[doc = "Bit 4 - Input enable for GPIO 108"]
    #[inline(always)]
    #[must_use]
    pub fn inpen108(&mut self) -> Inpen108W<Pincfg108Spec> {
        Inpen108W::new(self, 4)
    }
    #[doc = "Bit 5 - Return 0 for read data on GPIO 108"]
    #[inline(always)]
    #[must_use]
    pub fn rdzero108(&mut self) -> Rdzero108W<Pincfg108Spec> {
        Rdzero108W::new(self, 5)
    }
    #[doc = "Bits 6:7 - Interrupt enable for GPIO 108"]
    #[inline(always)]
    #[must_use]
    pub fn irpten108(&mut self) -> Irpten108W<Pincfg108Spec> {
        Irpten108W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Pin IO mode selection for GPIO pin 108"]
    #[inline(always)]
    #[must_use]
    pub fn outcfg108(&mut self) -> Outcfg108W<Pincfg108Spec> {
        Outcfg108W::new(self, 8)
    }
}
#[doc = "Controls the operation of virtual GPIO pin 108.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg108::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg108::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pincfg108Spec;
impl crate::RegisterSpec for Pincfg108Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pincfg108::R`](R) reader structure"]
impl crate::Readable for Pincfg108Spec {}
#[doc = "`write(|w| ..)` method takes [`pincfg108::W`](W) writer structure"]
impl crate::Writable for Pincfg108Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PINCFG108 to value 0x03"]
impl crate::Resettable for Pincfg108Spec {
    const RESET_VALUE: u32 = 0x03;
}
