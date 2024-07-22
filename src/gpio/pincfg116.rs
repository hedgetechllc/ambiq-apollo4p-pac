#[doc = "Register `PINCFG116` reader"]
pub type R = crate::R<Pincfg116Spec>;
#[doc = "Register `PINCFG116` writer"]
pub type W = crate::W<Pincfg116Spec>;
#[doc = "Function select for GPIO pin 116\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fncsel116 {
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
    Ct116 = 6,
    #[doc = "7: Reserved selection. Operation unknown if selected."]
    Reserved7 = 7,
    #[doc = "8: Observation bus bit 4"]
    Obsbus4 = 8,
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
impl From<Fncsel116> for u8 {
    #[inline(always)]
    fn from(variant: Fncsel116) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fncsel116 {
    type Ux = u8;
}
impl crate::IsEnum for Fncsel116 {}
#[doc = "Field `FNCSEL116` reader - Function select for GPIO pin 116"]
pub type Fncsel116R = crate::FieldReader<Fncsel116>;
impl Fncsel116R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fncsel116 {
        match self.bits {
            0 => Fncsel116::Reserved0,
            1 => Fncsel116::Reserved1,
            2 => Fncsel116::Reserved2,
            3 => Fncsel116::Gpio,
            4 => Fncsel116::Reserved4,
            5 => Fncsel116::Reserved5,
            6 => Fncsel116::Ct116,
            7 => Fncsel116::Reserved7,
            8 => Fncsel116::Obsbus4,
            9 => Fncsel116::Reserved9,
            10 => Fncsel116::Reserved10,
            11 => Fncsel116::Reserved11,
            12 => Fncsel116::Reserved12,
            13 => Fncsel116::Reserved13,
            14 => Fncsel116::Reserved14,
            15 => Fncsel116::Reserved15,
            _ => unreachable!(),
        }
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved0(&self) -> bool {
        *self == Fncsel116::Reserved0
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved1(&self) -> bool {
        *self == Fncsel116::Reserved1
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == Fncsel116::Reserved2
    }
    #[doc = "General purpose I/O"]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == Fncsel116::Gpio
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved4(&self) -> bool {
        *self == Fncsel116::Reserved4
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved5(&self) -> bool {
        *self == Fncsel116::Reserved5
    }
    #[doc = "Timer/Counter input or output; Selection of direction is done via CTIMER register settings."]
    #[inline(always)]
    pub fn is_ct116(&self) -> bool {
        *self == Fncsel116::Ct116
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved7(&self) -> bool {
        *self == Fncsel116::Reserved7
    }
    #[doc = "Observation bus bit 4"]
    #[inline(always)]
    pub fn is_obsbus4(&self) -> bool {
        *self == Fncsel116::Obsbus4
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved9(&self) -> bool {
        *self == Fncsel116::Reserved9
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved10(&self) -> bool {
        *self == Fncsel116::Reserved10
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved11(&self) -> bool {
        *self == Fncsel116::Reserved11
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved12(&self) -> bool {
        *self == Fncsel116::Reserved12
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved13(&self) -> bool {
        *self == Fncsel116::Reserved13
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved14(&self) -> bool {
        *self == Fncsel116::Reserved14
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved15(&self) -> bool {
        *self == Fncsel116::Reserved15
    }
}
#[doc = "Field `FNCSEL116` writer - Function select for GPIO pin 116"]
pub type Fncsel116W<'a, REG> = crate::FieldWriter<'a, REG, 4, Fncsel116, crate::Safe>;
impl<'a, REG> Fncsel116W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved0(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel116::Reserved0)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved1(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel116::Reserved1)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel116::Reserved2)
    }
    #[doc = "General purpose I/O"]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel116::Gpio)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved4(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel116::Reserved4)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved5(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel116::Reserved5)
    }
    #[doc = "Timer/Counter input or output; Selection of direction is done via CTIMER register settings."]
    #[inline(always)]
    pub fn ct116(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel116::Ct116)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved7(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel116::Reserved7)
    }
    #[doc = "Observation bus bit 4"]
    #[inline(always)]
    pub fn obsbus4(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel116::Obsbus4)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved9(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel116::Reserved9)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved10(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel116::Reserved10)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved11(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel116::Reserved11)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved12(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel116::Reserved12)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved13(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel116::Reserved13)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved14(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel116::Reserved14)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved15(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel116::Reserved15)
    }
}
#[doc = "Field `INPEN116` reader - Input enable for GPIO 116"]
pub type Inpen116R = crate::BitReader;
#[doc = "Field `INPEN116` writer - Input enable for GPIO 116"]
pub type Inpen116W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDZERO116` reader - Return 0 for read data on GPIO 116"]
pub type Rdzero116R = crate::BitReader;
#[doc = "Field `RDZERO116` writer - Return 0 for read data on GPIO 116"]
pub type Rdzero116W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Interrupt enable for GPIO 116\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Irpten116 {
    #[doc = "0: Interrupts are disabled for this GPIO"]
    Dis = 0,
    #[doc = "1: Interrupts are enabled for falling edge transition on this GPIO"]
    Intfall = 1,
    #[doc = "2: Interrupts are enabled for rising edge transitions on this GPIO"]
    Intrise = 2,
    #[doc = "3: Interrupts are enabled for any edge transition on this GPIO"]
    Intany = 3,
}
impl From<Irpten116> for u8 {
    #[inline(always)]
    fn from(variant: Irpten116) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Irpten116 {
    type Ux = u8;
}
impl crate::IsEnum for Irpten116 {}
#[doc = "Field `IRPTEN116` reader - Interrupt enable for GPIO 116"]
pub type Irpten116R = crate::FieldReader<Irpten116>;
impl Irpten116R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irpten116 {
        match self.bits {
            0 => Irpten116::Dis,
            1 => Irpten116::Intfall,
            2 => Irpten116::Intrise,
            3 => Irpten116::Intany,
            _ => unreachable!(),
        }
    }
    #[doc = "Interrupts are disabled for this GPIO"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Irpten116::Dis
    }
    #[doc = "Interrupts are enabled for falling edge transition on this GPIO"]
    #[inline(always)]
    pub fn is_intfall(&self) -> bool {
        *self == Irpten116::Intfall
    }
    #[doc = "Interrupts are enabled for rising edge transitions on this GPIO"]
    #[inline(always)]
    pub fn is_intrise(&self) -> bool {
        *self == Irpten116::Intrise
    }
    #[doc = "Interrupts are enabled for any edge transition on this GPIO"]
    #[inline(always)]
    pub fn is_intany(&self) -> bool {
        *self == Irpten116::Intany
    }
}
#[doc = "Field `IRPTEN116` writer - Interrupt enable for GPIO 116"]
pub type Irpten116W<'a, REG> = crate::FieldWriter<'a, REG, 2, Irpten116, crate::Safe>;
impl<'a, REG> Irpten116W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Interrupts are disabled for this GPIO"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten116::Dis)
    }
    #[doc = "Interrupts are enabled for falling edge transition on this GPIO"]
    #[inline(always)]
    pub fn intfall(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten116::Intfall)
    }
    #[doc = "Interrupts are enabled for rising edge transitions on this GPIO"]
    #[inline(always)]
    pub fn intrise(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten116::Intrise)
    }
    #[doc = "Interrupts are enabled for any edge transition on this GPIO"]
    #[inline(always)]
    pub fn intany(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten116::Intany)
    }
}
#[doc = "Pin IO mode selection for GPIO pin 116\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Outcfg116 {
    #[doc = "0: Output Disabled"]
    Dis = 0,
    #[doc = "1: Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    Pushpull = 1,
    #[doc = "2: Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    Od = 2,
    #[doc = "3: Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    Ts = 3,
}
impl From<Outcfg116> for u8 {
    #[inline(always)]
    fn from(variant: Outcfg116) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Outcfg116 {
    type Ux = u8;
}
impl crate::IsEnum for Outcfg116 {}
#[doc = "Field `OUTCFG116` reader - Pin IO mode selection for GPIO pin 116"]
pub type Outcfg116R = crate::FieldReader<Outcfg116>;
impl Outcfg116R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Outcfg116 {
        match self.bits {
            0 => Outcfg116::Dis,
            1 => Outcfg116::Pushpull,
            2 => Outcfg116::Od,
            3 => Outcfg116::Ts,
            _ => unreachable!(),
        }
    }
    #[doc = "Output Disabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Outcfg116::Dis
    }
    #[doc = "Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == Outcfg116::Pushpull
    }
    #[doc = "Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == Outcfg116::Od
    }
    #[doc = "Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == Outcfg116::Ts
    }
}
#[doc = "Field `OUTCFG116` writer - Pin IO mode selection for GPIO pin 116"]
pub type Outcfg116W<'a, REG> = crate::FieldWriter<'a, REG, 2, Outcfg116, crate::Safe>;
impl<'a, REG> Outcfg116W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Output Disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg116::Dis)
    }
    #[doc = "Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg116::Pushpull)
    }
    #[doc = "Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    #[inline(always)]
    pub fn od(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg116::Od)
    }
    #[doc = "Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    #[inline(always)]
    pub fn ts(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg116::Ts)
    }
}
impl R {
    #[doc = "Bits 0:3 - Function select for GPIO pin 116"]
    #[inline(always)]
    pub fn fncsel116(&self) -> Fncsel116R {
        Fncsel116R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Input enable for GPIO 116"]
    #[inline(always)]
    pub fn inpen116(&self) -> Inpen116R {
        Inpen116R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Return 0 for read data on GPIO 116"]
    #[inline(always)]
    pub fn rdzero116(&self) -> Rdzero116R {
        Rdzero116R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Interrupt enable for GPIO 116"]
    #[inline(always)]
    pub fn irpten116(&self) -> Irpten116R {
        Irpten116R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Pin IO mode selection for GPIO pin 116"]
    #[inline(always)]
    pub fn outcfg116(&self) -> Outcfg116R {
        Outcfg116R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Function select for GPIO pin 116"]
    #[inline(always)]
    #[must_use]
    pub fn fncsel116(&mut self) -> Fncsel116W<Pincfg116Spec> {
        Fncsel116W::new(self, 0)
    }
    #[doc = "Bit 4 - Input enable for GPIO 116"]
    #[inline(always)]
    #[must_use]
    pub fn inpen116(&mut self) -> Inpen116W<Pincfg116Spec> {
        Inpen116W::new(self, 4)
    }
    #[doc = "Bit 5 - Return 0 for read data on GPIO 116"]
    #[inline(always)]
    #[must_use]
    pub fn rdzero116(&mut self) -> Rdzero116W<Pincfg116Spec> {
        Rdzero116W::new(self, 5)
    }
    #[doc = "Bits 6:7 - Interrupt enable for GPIO 116"]
    #[inline(always)]
    #[must_use]
    pub fn irpten116(&mut self) -> Irpten116W<Pincfg116Spec> {
        Irpten116W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Pin IO mode selection for GPIO pin 116"]
    #[inline(always)]
    #[must_use]
    pub fn outcfg116(&mut self) -> Outcfg116W<Pincfg116Spec> {
        Outcfg116W::new(self, 8)
    }
}
#[doc = "Controls the operation of virtual GPIO pin 116.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg116::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg116::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pincfg116Spec;
impl crate::RegisterSpec for Pincfg116Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pincfg116::R`](R) reader structure"]
impl crate::Readable for Pincfg116Spec {}
#[doc = "`write(|w| ..)` method takes [`pincfg116::W`](W) writer structure"]
impl crate::Writable for Pincfg116Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PINCFG116 to value 0x03"]
impl crate::Resettable for Pincfg116Spec {
    const RESET_VALUE: u32 = 0x03;
}
