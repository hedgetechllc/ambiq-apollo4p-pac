#[doc = "Register `PINCFG120` reader"]
pub type R = crate::R<Pincfg120Spec>;
#[doc = "Register `PINCFG120` writer"]
pub type W = crate::W<Pincfg120Spec>;
#[doc = "Function select for GPIO pin 120\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fncsel120 {
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
    Ct120 = 6,
    #[doc = "7: Reserved selection. Operation unknown if selected."]
    Reserved7 = 7,
    #[doc = "8: Observation bus bit 8"]
    Obsbus8 = 8,
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
impl From<Fncsel120> for u8 {
    #[inline(always)]
    fn from(variant: Fncsel120) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fncsel120 {
    type Ux = u8;
}
impl crate::IsEnum for Fncsel120 {}
#[doc = "Field `FNCSEL120` reader - Function select for GPIO pin 120"]
pub type Fncsel120R = crate::FieldReader<Fncsel120>;
impl Fncsel120R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fncsel120 {
        match self.bits {
            0 => Fncsel120::Reserved0,
            1 => Fncsel120::Reserved1,
            2 => Fncsel120::Reserved2,
            3 => Fncsel120::Gpio,
            4 => Fncsel120::Reserved4,
            5 => Fncsel120::Reserved5,
            6 => Fncsel120::Ct120,
            7 => Fncsel120::Reserved7,
            8 => Fncsel120::Obsbus8,
            9 => Fncsel120::Reserved9,
            10 => Fncsel120::Reserved10,
            11 => Fncsel120::Reserved11,
            12 => Fncsel120::Reserved12,
            13 => Fncsel120::Reserved13,
            14 => Fncsel120::Reserved14,
            15 => Fncsel120::Reserved15,
            _ => unreachable!(),
        }
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved0(&self) -> bool {
        *self == Fncsel120::Reserved0
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved1(&self) -> bool {
        *self == Fncsel120::Reserved1
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == Fncsel120::Reserved2
    }
    #[doc = "General purpose I/O"]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == Fncsel120::Gpio
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved4(&self) -> bool {
        *self == Fncsel120::Reserved4
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved5(&self) -> bool {
        *self == Fncsel120::Reserved5
    }
    #[doc = "Timer/Counter input or output; Selection of direction is done via CTIMER register settings."]
    #[inline(always)]
    pub fn is_ct120(&self) -> bool {
        *self == Fncsel120::Ct120
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved7(&self) -> bool {
        *self == Fncsel120::Reserved7
    }
    #[doc = "Observation bus bit 8"]
    #[inline(always)]
    pub fn is_obsbus8(&self) -> bool {
        *self == Fncsel120::Obsbus8
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved9(&self) -> bool {
        *self == Fncsel120::Reserved9
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved10(&self) -> bool {
        *self == Fncsel120::Reserved10
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved11(&self) -> bool {
        *self == Fncsel120::Reserved11
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved12(&self) -> bool {
        *self == Fncsel120::Reserved12
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved13(&self) -> bool {
        *self == Fncsel120::Reserved13
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved14(&self) -> bool {
        *self == Fncsel120::Reserved14
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved15(&self) -> bool {
        *self == Fncsel120::Reserved15
    }
}
#[doc = "Field `FNCSEL120` writer - Function select for GPIO pin 120"]
pub type Fncsel120W<'a, REG> = crate::FieldWriter<'a, REG, 4, Fncsel120, crate::Safe>;
impl<'a, REG> Fncsel120W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved0(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel120::Reserved0)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved1(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel120::Reserved1)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel120::Reserved2)
    }
    #[doc = "General purpose I/O"]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel120::Gpio)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved4(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel120::Reserved4)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved5(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel120::Reserved5)
    }
    #[doc = "Timer/Counter input or output; Selection of direction is done via CTIMER register settings."]
    #[inline(always)]
    pub fn ct120(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel120::Ct120)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved7(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel120::Reserved7)
    }
    #[doc = "Observation bus bit 8"]
    #[inline(always)]
    pub fn obsbus8(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel120::Obsbus8)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved9(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel120::Reserved9)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved10(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel120::Reserved10)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved11(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel120::Reserved11)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved12(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel120::Reserved12)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved13(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel120::Reserved13)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved14(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel120::Reserved14)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved15(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel120::Reserved15)
    }
}
#[doc = "Field `INPEN120` reader - Input enable for GPIO 120"]
pub type Inpen120R = crate::BitReader;
#[doc = "Field `INPEN120` writer - Input enable for GPIO 120"]
pub type Inpen120W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDZERO120` reader - Return 0 for read data on GPIO 120"]
pub type Rdzero120R = crate::BitReader;
#[doc = "Field `RDZERO120` writer - Return 0 for read data on GPIO 120"]
pub type Rdzero120W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Interrupt enable for GPIO 120\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Irpten120 {
    #[doc = "0: Interrupts are disabled for this GPIO"]
    Dis = 0,
    #[doc = "1: Interrupts are enabled for falling edge transition on this GPIO"]
    Intfall = 1,
    #[doc = "2: Interrupts are enabled for rising edge transitions on this GPIO"]
    Intrise = 2,
    #[doc = "3: Interrupts are enabled for any edge transition on this GPIO"]
    Intany = 3,
}
impl From<Irpten120> for u8 {
    #[inline(always)]
    fn from(variant: Irpten120) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Irpten120 {
    type Ux = u8;
}
impl crate::IsEnum for Irpten120 {}
#[doc = "Field `IRPTEN120` reader - Interrupt enable for GPIO 120"]
pub type Irpten120R = crate::FieldReader<Irpten120>;
impl Irpten120R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irpten120 {
        match self.bits {
            0 => Irpten120::Dis,
            1 => Irpten120::Intfall,
            2 => Irpten120::Intrise,
            3 => Irpten120::Intany,
            _ => unreachable!(),
        }
    }
    #[doc = "Interrupts are disabled for this GPIO"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Irpten120::Dis
    }
    #[doc = "Interrupts are enabled for falling edge transition on this GPIO"]
    #[inline(always)]
    pub fn is_intfall(&self) -> bool {
        *self == Irpten120::Intfall
    }
    #[doc = "Interrupts are enabled for rising edge transitions on this GPIO"]
    #[inline(always)]
    pub fn is_intrise(&self) -> bool {
        *self == Irpten120::Intrise
    }
    #[doc = "Interrupts are enabled for any edge transition on this GPIO"]
    #[inline(always)]
    pub fn is_intany(&self) -> bool {
        *self == Irpten120::Intany
    }
}
#[doc = "Field `IRPTEN120` writer - Interrupt enable for GPIO 120"]
pub type Irpten120W<'a, REG> = crate::FieldWriter<'a, REG, 2, Irpten120, crate::Safe>;
impl<'a, REG> Irpten120W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Interrupts are disabled for this GPIO"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten120::Dis)
    }
    #[doc = "Interrupts are enabled for falling edge transition on this GPIO"]
    #[inline(always)]
    pub fn intfall(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten120::Intfall)
    }
    #[doc = "Interrupts are enabled for rising edge transitions on this GPIO"]
    #[inline(always)]
    pub fn intrise(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten120::Intrise)
    }
    #[doc = "Interrupts are enabled for any edge transition on this GPIO"]
    #[inline(always)]
    pub fn intany(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten120::Intany)
    }
}
#[doc = "Pin IO mode selection for GPIO pin 120\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Outcfg120 {
    #[doc = "0: Output Disabled"]
    Dis = 0,
    #[doc = "1: Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    Pushpull = 1,
    #[doc = "2: Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    Od = 2,
    #[doc = "3: Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    Ts = 3,
}
impl From<Outcfg120> for u8 {
    #[inline(always)]
    fn from(variant: Outcfg120) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Outcfg120 {
    type Ux = u8;
}
impl crate::IsEnum for Outcfg120 {}
#[doc = "Field `OUTCFG120` reader - Pin IO mode selection for GPIO pin 120"]
pub type Outcfg120R = crate::FieldReader<Outcfg120>;
impl Outcfg120R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Outcfg120 {
        match self.bits {
            0 => Outcfg120::Dis,
            1 => Outcfg120::Pushpull,
            2 => Outcfg120::Od,
            3 => Outcfg120::Ts,
            _ => unreachable!(),
        }
    }
    #[doc = "Output Disabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Outcfg120::Dis
    }
    #[doc = "Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == Outcfg120::Pushpull
    }
    #[doc = "Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == Outcfg120::Od
    }
    #[doc = "Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == Outcfg120::Ts
    }
}
#[doc = "Field `OUTCFG120` writer - Pin IO mode selection for GPIO pin 120"]
pub type Outcfg120W<'a, REG> = crate::FieldWriter<'a, REG, 2, Outcfg120, crate::Safe>;
impl<'a, REG> Outcfg120W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Output Disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg120::Dis)
    }
    #[doc = "Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg120::Pushpull)
    }
    #[doc = "Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    #[inline(always)]
    pub fn od(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg120::Od)
    }
    #[doc = "Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    #[inline(always)]
    pub fn ts(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg120::Ts)
    }
}
impl R {
    #[doc = "Bits 0:3 - Function select for GPIO pin 120"]
    #[inline(always)]
    pub fn fncsel120(&self) -> Fncsel120R {
        Fncsel120R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Input enable for GPIO 120"]
    #[inline(always)]
    pub fn inpen120(&self) -> Inpen120R {
        Inpen120R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Return 0 for read data on GPIO 120"]
    #[inline(always)]
    pub fn rdzero120(&self) -> Rdzero120R {
        Rdzero120R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Interrupt enable for GPIO 120"]
    #[inline(always)]
    pub fn irpten120(&self) -> Irpten120R {
        Irpten120R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Pin IO mode selection for GPIO pin 120"]
    #[inline(always)]
    pub fn outcfg120(&self) -> Outcfg120R {
        Outcfg120R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Function select for GPIO pin 120"]
    #[inline(always)]
    #[must_use]
    pub fn fncsel120(&mut self) -> Fncsel120W<Pincfg120Spec> {
        Fncsel120W::new(self, 0)
    }
    #[doc = "Bit 4 - Input enable for GPIO 120"]
    #[inline(always)]
    #[must_use]
    pub fn inpen120(&mut self) -> Inpen120W<Pincfg120Spec> {
        Inpen120W::new(self, 4)
    }
    #[doc = "Bit 5 - Return 0 for read data on GPIO 120"]
    #[inline(always)]
    #[must_use]
    pub fn rdzero120(&mut self) -> Rdzero120W<Pincfg120Spec> {
        Rdzero120W::new(self, 5)
    }
    #[doc = "Bits 6:7 - Interrupt enable for GPIO 120"]
    #[inline(always)]
    #[must_use]
    pub fn irpten120(&mut self) -> Irpten120W<Pincfg120Spec> {
        Irpten120W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Pin IO mode selection for GPIO pin 120"]
    #[inline(always)]
    #[must_use]
    pub fn outcfg120(&mut self) -> Outcfg120W<Pincfg120Spec> {
        Outcfg120W::new(self, 8)
    }
}
#[doc = "Controls the operation of virtual GPIO pin 120.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg120::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg120::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pincfg120Spec;
impl crate::RegisterSpec for Pincfg120Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pincfg120::R`](R) reader structure"]
impl crate::Readable for Pincfg120Spec {}
#[doc = "`write(|w| ..)` method takes [`pincfg120::W`](W) writer structure"]
impl crate::Writable for Pincfg120Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PINCFG120 to value 0x03"]
impl crate::Resettable for Pincfg120Spec {
    const RESET_VALUE: u32 = 0x03;
}
