#[doc = "Register `PINCFG105` reader"]
pub type R = crate::R<Pincfg105Spec>;
#[doc = "Register `PINCFG105` writer"]
pub type W = crate::W<Pincfg105Spec>;
#[doc = "Function select for GPIO pin 105\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fncsel105 {
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
    Ct105 = 6,
    #[doc = "7: Reserved selection. Operation unknown if selected."]
    Reserved7 = 7,
    #[doc = "8: Observation bus bit 9"]
    Obsbus9 = 8,
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
impl From<Fncsel105> for u8 {
    #[inline(always)]
    fn from(variant: Fncsel105) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fncsel105 {
    type Ux = u8;
}
impl crate::IsEnum for Fncsel105 {}
#[doc = "Field `FNCSEL105` reader - Function select for GPIO pin 105"]
pub type Fncsel105R = crate::FieldReader<Fncsel105>;
impl Fncsel105R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fncsel105 {
        match self.bits {
            0 => Fncsel105::Reserved0,
            1 => Fncsel105::Reserved1,
            2 => Fncsel105::Reserved2,
            3 => Fncsel105::Gpio,
            4 => Fncsel105::Reserved4,
            5 => Fncsel105::Reserved5,
            6 => Fncsel105::Ct105,
            7 => Fncsel105::Reserved7,
            8 => Fncsel105::Obsbus9,
            9 => Fncsel105::Reserved9,
            10 => Fncsel105::Reserved10,
            11 => Fncsel105::Reserved11,
            12 => Fncsel105::Reserved12,
            13 => Fncsel105::Reserved13,
            14 => Fncsel105::Reserved14,
            15 => Fncsel105::Reserved15,
            _ => unreachable!(),
        }
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved0(&self) -> bool {
        *self == Fncsel105::Reserved0
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved1(&self) -> bool {
        *self == Fncsel105::Reserved1
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == Fncsel105::Reserved2
    }
    #[doc = "General purpose I/O"]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == Fncsel105::Gpio
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved4(&self) -> bool {
        *self == Fncsel105::Reserved4
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved5(&self) -> bool {
        *self == Fncsel105::Reserved5
    }
    #[doc = "Timer/Counter input or output; Selection of direction is done via CTIMER register settings."]
    #[inline(always)]
    pub fn is_ct105(&self) -> bool {
        *self == Fncsel105::Ct105
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved7(&self) -> bool {
        *self == Fncsel105::Reserved7
    }
    #[doc = "Observation bus bit 9"]
    #[inline(always)]
    pub fn is_obsbus9(&self) -> bool {
        *self == Fncsel105::Obsbus9
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved9(&self) -> bool {
        *self == Fncsel105::Reserved9
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved10(&self) -> bool {
        *self == Fncsel105::Reserved10
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved11(&self) -> bool {
        *self == Fncsel105::Reserved11
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved12(&self) -> bool {
        *self == Fncsel105::Reserved12
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved13(&self) -> bool {
        *self == Fncsel105::Reserved13
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved14(&self) -> bool {
        *self == Fncsel105::Reserved14
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved15(&self) -> bool {
        *self == Fncsel105::Reserved15
    }
}
#[doc = "Field `FNCSEL105` writer - Function select for GPIO pin 105"]
pub type Fncsel105W<'a, REG> = crate::FieldWriter<'a, REG, 4, Fncsel105, crate::Safe>;
impl<'a, REG> Fncsel105W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved0(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel105::Reserved0)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved1(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel105::Reserved1)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel105::Reserved2)
    }
    #[doc = "General purpose I/O"]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel105::Gpio)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved4(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel105::Reserved4)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved5(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel105::Reserved5)
    }
    #[doc = "Timer/Counter input or output; Selection of direction is done via CTIMER register settings."]
    #[inline(always)]
    pub fn ct105(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel105::Ct105)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved7(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel105::Reserved7)
    }
    #[doc = "Observation bus bit 9"]
    #[inline(always)]
    pub fn obsbus9(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel105::Obsbus9)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved9(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel105::Reserved9)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved10(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel105::Reserved10)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved11(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel105::Reserved11)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved12(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel105::Reserved12)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved13(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel105::Reserved13)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved14(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel105::Reserved14)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved15(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel105::Reserved15)
    }
}
#[doc = "Field `INPEN105` reader - Input enable for GPIO 105"]
pub type Inpen105R = crate::BitReader;
#[doc = "Field `INPEN105` writer - Input enable for GPIO 105"]
pub type Inpen105W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDZERO105` reader - Return 0 for read data on GPIO 105"]
pub type Rdzero105R = crate::BitReader;
#[doc = "Field `RDZERO105` writer - Return 0 for read data on GPIO 105"]
pub type Rdzero105W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Interrupt enable for GPIO 105\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Irpten105 {
    #[doc = "0: Interrupts are disabled for this GPIO"]
    Dis = 0,
    #[doc = "1: Interrupts are enabled for falling edge transition on this GPIO"]
    Intfall = 1,
    #[doc = "2: Interrupts are enabled for rising edge transitions on this GPIO"]
    Intrise = 2,
    #[doc = "3: Interrupts are enabled for any edge transition on this GPIO"]
    Intany = 3,
}
impl From<Irpten105> for u8 {
    #[inline(always)]
    fn from(variant: Irpten105) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Irpten105 {
    type Ux = u8;
}
impl crate::IsEnum for Irpten105 {}
#[doc = "Field `IRPTEN105` reader - Interrupt enable for GPIO 105"]
pub type Irpten105R = crate::FieldReader<Irpten105>;
impl Irpten105R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irpten105 {
        match self.bits {
            0 => Irpten105::Dis,
            1 => Irpten105::Intfall,
            2 => Irpten105::Intrise,
            3 => Irpten105::Intany,
            _ => unreachable!(),
        }
    }
    #[doc = "Interrupts are disabled for this GPIO"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Irpten105::Dis
    }
    #[doc = "Interrupts are enabled for falling edge transition on this GPIO"]
    #[inline(always)]
    pub fn is_intfall(&self) -> bool {
        *self == Irpten105::Intfall
    }
    #[doc = "Interrupts are enabled for rising edge transitions on this GPIO"]
    #[inline(always)]
    pub fn is_intrise(&self) -> bool {
        *self == Irpten105::Intrise
    }
    #[doc = "Interrupts are enabled for any edge transition on this GPIO"]
    #[inline(always)]
    pub fn is_intany(&self) -> bool {
        *self == Irpten105::Intany
    }
}
#[doc = "Field `IRPTEN105` writer - Interrupt enable for GPIO 105"]
pub type Irpten105W<'a, REG> = crate::FieldWriter<'a, REG, 2, Irpten105, crate::Safe>;
impl<'a, REG> Irpten105W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Interrupts are disabled for this GPIO"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten105::Dis)
    }
    #[doc = "Interrupts are enabled for falling edge transition on this GPIO"]
    #[inline(always)]
    pub fn intfall(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten105::Intfall)
    }
    #[doc = "Interrupts are enabled for rising edge transitions on this GPIO"]
    #[inline(always)]
    pub fn intrise(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten105::Intrise)
    }
    #[doc = "Interrupts are enabled for any edge transition on this GPIO"]
    #[inline(always)]
    pub fn intany(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten105::Intany)
    }
}
#[doc = "Pin IO mode selection for GPIO pin 105\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Outcfg105 {
    #[doc = "0: Output Disabled"]
    Dis = 0,
    #[doc = "1: Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    Pushpull = 1,
    #[doc = "2: Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    Od = 2,
    #[doc = "3: Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    Ts = 3,
}
impl From<Outcfg105> for u8 {
    #[inline(always)]
    fn from(variant: Outcfg105) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Outcfg105 {
    type Ux = u8;
}
impl crate::IsEnum for Outcfg105 {}
#[doc = "Field `OUTCFG105` reader - Pin IO mode selection for GPIO pin 105"]
pub type Outcfg105R = crate::FieldReader<Outcfg105>;
impl Outcfg105R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Outcfg105 {
        match self.bits {
            0 => Outcfg105::Dis,
            1 => Outcfg105::Pushpull,
            2 => Outcfg105::Od,
            3 => Outcfg105::Ts,
            _ => unreachable!(),
        }
    }
    #[doc = "Output Disabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Outcfg105::Dis
    }
    #[doc = "Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == Outcfg105::Pushpull
    }
    #[doc = "Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == Outcfg105::Od
    }
    #[doc = "Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == Outcfg105::Ts
    }
}
#[doc = "Field `OUTCFG105` writer - Pin IO mode selection for GPIO pin 105"]
pub type Outcfg105W<'a, REG> = crate::FieldWriter<'a, REG, 2, Outcfg105, crate::Safe>;
impl<'a, REG> Outcfg105W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Output Disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg105::Dis)
    }
    #[doc = "Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg105::Pushpull)
    }
    #[doc = "Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    #[inline(always)]
    pub fn od(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg105::Od)
    }
    #[doc = "Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    #[inline(always)]
    pub fn ts(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg105::Ts)
    }
}
impl R {
    #[doc = "Bits 0:3 - Function select for GPIO pin 105"]
    #[inline(always)]
    pub fn fncsel105(&self) -> Fncsel105R {
        Fncsel105R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Input enable for GPIO 105"]
    #[inline(always)]
    pub fn inpen105(&self) -> Inpen105R {
        Inpen105R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Return 0 for read data on GPIO 105"]
    #[inline(always)]
    pub fn rdzero105(&self) -> Rdzero105R {
        Rdzero105R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Interrupt enable for GPIO 105"]
    #[inline(always)]
    pub fn irpten105(&self) -> Irpten105R {
        Irpten105R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Pin IO mode selection for GPIO pin 105"]
    #[inline(always)]
    pub fn outcfg105(&self) -> Outcfg105R {
        Outcfg105R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Function select for GPIO pin 105"]
    #[inline(always)]
    #[must_use]
    pub fn fncsel105(&mut self) -> Fncsel105W<Pincfg105Spec> {
        Fncsel105W::new(self, 0)
    }
    #[doc = "Bit 4 - Input enable for GPIO 105"]
    #[inline(always)]
    #[must_use]
    pub fn inpen105(&mut self) -> Inpen105W<Pincfg105Spec> {
        Inpen105W::new(self, 4)
    }
    #[doc = "Bit 5 - Return 0 for read data on GPIO 105"]
    #[inline(always)]
    #[must_use]
    pub fn rdzero105(&mut self) -> Rdzero105W<Pincfg105Spec> {
        Rdzero105W::new(self, 5)
    }
    #[doc = "Bits 6:7 - Interrupt enable for GPIO 105"]
    #[inline(always)]
    #[must_use]
    pub fn irpten105(&mut self) -> Irpten105W<Pincfg105Spec> {
        Irpten105W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Pin IO mode selection for GPIO pin 105"]
    #[inline(always)]
    #[must_use]
    pub fn outcfg105(&mut self) -> Outcfg105W<Pincfg105Spec> {
        Outcfg105W::new(self, 8)
    }
}
#[doc = "Controls the operation of virtual GPIO pin 105.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg105::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg105::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pincfg105Spec;
impl crate::RegisterSpec for Pincfg105Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pincfg105::R`](R) reader structure"]
impl crate::Readable for Pincfg105Spec {}
#[doc = "`write(|w| ..)` method takes [`pincfg105::W`](W) writer structure"]
impl crate::Writable for Pincfg105Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PINCFG105 to value 0x03"]
impl crate::Resettable for Pincfg105Spec {
    const RESET_VALUE: u32 = 0x03;
}
