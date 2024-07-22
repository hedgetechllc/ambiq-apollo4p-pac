#[doc = "Register `PINCFG107` reader"]
pub type R = crate::R<Pincfg107Spec>;
#[doc = "Register `PINCFG107` writer"]
pub type W = crate::W<Pincfg107Spec>;
#[doc = "Function select for GPIO pin 107\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fncsel107 {
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
    Ct107 = 6,
    #[doc = "7: Reserved selection. Operation unknown if selected."]
    Reserved7 = 7,
    #[doc = "8: Observation bus bit 11"]
    Obsbus11 = 8,
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
impl From<Fncsel107> for u8 {
    #[inline(always)]
    fn from(variant: Fncsel107) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fncsel107 {
    type Ux = u8;
}
impl crate::IsEnum for Fncsel107 {}
#[doc = "Field `FNCSEL107` reader - Function select for GPIO pin 107"]
pub type Fncsel107R = crate::FieldReader<Fncsel107>;
impl Fncsel107R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fncsel107 {
        match self.bits {
            0 => Fncsel107::Reserved0,
            1 => Fncsel107::Reserved1,
            2 => Fncsel107::Reserved2,
            3 => Fncsel107::Gpio,
            4 => Fncsel107::Reserved4,
            5 => Fncsel107::Reserved5,
            6 => Fncsel107::Ct107,
            7 => Fncsel107::Reserved7,
            8 => Fncsel107::Obsbus11,
            9 => Fncsel107::Reserved9,
            10 => Fncsel107::Reserved10,
            11 => Fncsel107::Reserved11,
            12 => Fncsel107::Reserved12,
            13 => Fncsel107::Reserved13,
            14 => Fncsel107::Reserved14,
            15 => Fncsel107::Reserved15,
            _ => unreachable!(),
        }
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved0(&self) -> bool {
        *self == Fncsel107::Reserved0
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved1(&self) -> bool {
        *self == Fncsel107::Reserved1
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == Fncsel107::Reserved2
    }
    #[doc = "General purpose I/O"]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == Fncsel107::Gpio
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved4(&self) -> bool {
        *self == Fncsel107::Reserved4
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved5(&self) -> bool {
        *self == Fncsel107::Reserved5
    }
    #[doc = "Timer/Counter input or output; Selection of direction is done via CTIMER register settings."]
    #[inline(always)]
    pub fn is_ct107(&self) -> bool {
        *self == Fncsel107::Ct107
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved7(&self) -> bool {
        *self == Fncsel107::Reserved7
    }
    #[doc = "Observation bus bit 11"]
    #[inline(always)]
    pub fn is_obsbus11(&self) -> bool {
        *self == Fncsel107::Obsbus11
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved9(&self) -> bool {
        *self == Fncsel107::Reserved9
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved10(&self) -> bool {
        *self == Fncsel107::Reserved10
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved11(&self) -> bool {
        *self == Fncsel107::Reserved11
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved12(&self) -> bool {
        *self == Fncsel107::Reserved12
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved13(&self) -> bool {
        *self == Fncsel107::Reserved13
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved14(&self) -> bool {
        *self == Fncsel107::Reserved14
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved15(&self) -> bool {
        *self == Fncsel107::Reserved15
    }
}
#[doc = "Field `FNCSEL107` writer - Function select for GPIO pin 107"]
pub type Fncsel107W<'a, REG> = crate::FieldWriter<'a, REG, 4, Fncsel107, crate::Safe>;
impl<'a, REG> Fncsel107W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved0(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel107::Reserved0)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved1(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel107::Reserved1)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel107::Reserved2)
    }
    #[doc = "General purpose I/O"]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel107::Gpio)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved4(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel107::Reserved4)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved5(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel107::Reserved5)
    }
    #[doc = "Timer/Counter input or output; Selection of direction is done via CTIMER register settings."]
    #[inline(always)]
    pub fn ct107(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel107::Ct107)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved7(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel107::Reserved7)
    }
    #[doc = "Observation bus bit 11"]
    #[inline(always)]
    pub fn obsbus11(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel107::Obsbus11)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved9(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel107::Reserved9)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved10(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel107::Reserved10)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved11(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel107::Reserved11)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved12(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel107::Reserved12)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved13(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel107::Reserved13)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved14(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel107::Reserved14)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved15(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel107::Reserved15)
    }
}
#[doc = "Field `INPEN107` reader - Input enable for GPIO 107"]
pub type Inpen107R = crate::BitReader;
#[doc = "Field `INPEN107` writer - Input enable for GPIO 107"]
pub type Inpen107W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDZERO107` reader - Return 0 for read data on GPIO 107"]
pub type Rdzero107R = crate::BitReader;
#[doc = "Field `RDZERO107` writer - Return 0 for read data on GPIO 107"]
pub type Rdzero107W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Interrupt enable for GPIO 107\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Irpten107 {
    #[doc = "0: Interrupts are disabled for this GPIO"]
    Dis = 0,
    #[doc = "1: Interrupts are enabled for falling edge transition on this GPIO"]
    Intfall = 1,
    #[doc = "2: Interrupts are enabled for rising edge transitions on this GPIO"]
    Intrise = 2,
    #[doc = "3: Interrupts are enabled for any edge transition on this GPIO"]
    Intany = 3,
}
impl From<Irpten107> for u8 {
    #[inline(always)]
    fn from(variant: Irpten107) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Irpten107 {
    type Ux = u8;
}
impl crate::IsEnum for Irpten107 {}
#[doc = "Field `IRPTEN107` reader - Interrupt enable for GPIO 107"]
pub type Irpten107R = crate::FieldReader<Irpten107>;
impl Irpten107R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irpten107 {
        match self.bits {
            0 => Irpten107::Dis,
            1 => Irpten107::Intfall,
            2 => Irpten107::Intrise,
            3 => Irpten107::Intany,
            _ => unreachable!(),
        }
    }
    #[doc = "Interrupts are disabled for this GPIO"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Irpten107::Dis
    }
    #[doc = "Interrupts are enabled for falling edge transition on this GPIO"]
    #[inline(always)]
    pub fn is_intfall(&self) -> bool {
        *self == Irpten107::Intfall
    }
    #[doc = "Interrupts are enabled for rising edge transitions on this GPIO"]
    #[inline(always)]
    pub fn is_intrise(&self) -> bool {
        *self == Irpten107::Intrise
    }
    #[doc = "Interrupts are enabled for any edge transition on this GPIO"]
    #[inline(always)]
    pub fn is_intany(&self) -> bool {
        *self == Irpten107::Intany
    }
}
#[doc = "Field `IRPTEN107` writer - Interrupt enable for GPIO 107"]
pub type Irpten107W<'a, REG> = crate::FieldWriter<'a, REG, 2, Irpten107, crate::Safe>;
impl<'a, REG> Irpten107W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Interrupts are disabled for this GPIO"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten107::Dis)
    }
    #[doc = "Interrupts are enabled for falling edge transition on this GPIO"]
    #[inline(always)]
    pub fn intfall(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten107::Intfall)
    }
    #[doc = "Interrupts are enabled for rising edge transitions on this GPIO"]
    #[inline(always)]
    pub fn intrise(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten107::Intrise)
    }
    #[doc = "Interrupts are enabled for any edge transition on this GPIO"]
    #[inline(always)]
    pub fn intany(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten107::Intany)
    }
}
#[doc = "Pin IO mode selection for GPIO pin 107\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Outcfg107 {
    #[doc = "0: Output Disabled"]
    Dis = 0,
    #[doc = "1: Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    Pushpull = 1,
    #[doc = "2: Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    Od = 2,
    #[doc = "3: Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    Ts = 3,
}
impl From<Outcfg107> for u8 {
    #[inline(always)]
    fn from(variant: Outcfg107) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Outcfg107 {
    type Ux = u8;
}
impl crate::IsEnum for Outcfg107 {}
#[doc = "Field `OUTCFG107` reader - Pin IO mode selection for GPIO pin 107"]
pub type Outcfg107R = crate::FieldReader<Outcfg107>;
impl Outcfg107R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Outcfg107 {
        match self.bits {
            0 => Outcfg107::Dis,
            1 => Outcfg107::Pushpull,
            2 => Outcfg107::Od,
            3 => Outcfg107::Ts,
            _ => unreachable!(),
        }
    }
    #[doc = "Output Disabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Outcfg107::Dis
    }
    #[doc = "Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == Outcfg107::Pushpull
    }
    #[doc = "Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == Outcfg107::Od
    }
    #[doc = "Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == Outcfg107::Ts
    }
}
#[doc = "Field `OUTCFG107` writer - Pin IO mode selection for GPIO pin 107"]
pub type Outcfg107W<'a, REG> = crate::FieldWriter<'a, REG, 2, Outcfg107, crate::Safe>;
impl<'a, REG> Outcfg107W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Output Disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg107::Dis)
    }
    #[doc = "Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg107::Pushpull)
    }
    #[doc = "Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    #[inline(always)]
    pub fn od(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg107::Od)
    }
    #[doc = "Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    #[inline(always)]
    pub fn ts(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg107::Ts)
    }
}
impl R {
    #[doc = "Bits 0:3 - Function select for GPIO pin 107"]
    #[inline(always)]
    pub fn fncsel107(&self) -> Fncsel107R {
        Fncsel107R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Input enable for GPIO 107"]
    #[inline(always)]
    pub fn inpen107(&self) -> Inpen107R {
        Inpen107R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Return 0 for read data on GPIO 107"]
    #[inline(always)]
    pub fn rdzero107(&self) -> Rdzero107R {
        Rdzero107R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Interrupt enable for GPIO 107"]
    #[inline(always)]
    pub fn irpten107(&self) -> Irpten107R {
        Irpten107R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Pin IO mode selection for GPIO pin 107"]
    #[inline(always)]
    pub fn outcfg107(&self) -> Outcfg107R {
        Outcfg107R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Function select for GPIO pin 107"]
    #[inline(always)]
    #[must_use]
    pub fn fncsel107(&mut self) -> Fncsel107W<Pincfg107Spec> {
        Fncsel107W::new(self, 0)
    }
    #[doc = "Bit 4 - Input enable for GPIO 107"]
    #[inline(always)]
    #[must_use]
    pub fn inpen107(&mut self) -> Inpen107W<Pincfg107Spec> {
        Inpen107W::new(self, 4)
    }
    #[doc = "Bit 5 - Return 0 for read data on GPIO 107"]
    #[inline(always)]
    #[must_use]
    pub fn rdzero107(&mut self) -> Rdzero107W<Pincfg107Spec> {
        Rdzero107W::new(self, 5)
    }
    #[doc = "Bits 6:7 - Interrupt enable for GPIO 107"]
    #[inline(always)]
    #[must_use]
    pub fn irpten107(&mut self) -> Irpten107W<Pincfg107Spec> {
        Irpten107W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Pin IO mode selection for GPIO pin 107"]
    #[inline(always)]
    #[must_use]
    pub fn outcfg107(&mut self) -> Outcfg107W<Pincfg107Spec> {
        Outcfg107W::new(self, 8)
    }
}
#[doc = "Controls the operation of virtual GPIO pin 107.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg107::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg107::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pincfg107Spec;
impl crate::RegisterSpec for Pincfg107Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pincfg107::R`](R) reader structure"]
impl crate::Readable for Pincfg107Spec {}
#[doc = "`write(|w| ..)` method takes [`pincfg107::W`](W) writer structure"]
impl crate::Writable for Pincfg107Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PINCFG107 to value 0x03"]
impl crate::Resettable for Pincfg107Spec {
    const RESET_VALUE: u32 = 0x03;
}
