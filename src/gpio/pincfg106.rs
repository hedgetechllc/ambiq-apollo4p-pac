#[doc = "Register `PINCFG106` reader"]
pub type R = crate::R<Pincfg106Spec>;
#[doc = "Register `PINCFG106` writer"]
pub type W = crate::W<Pincfg106Spec>;
#[doc = "Function select for GPIO pin 106\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fncsel106 {
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
    Ct106 = 6,
    #[doc = "7: Reserved selection. Operation unknown if selected."]
    Reserved7 = 7,
    #[doc = "8: Observation bus bit 10"]
    Obsbus10 = 8,
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
impl From<Fncsel106> for u8 {
    #[inline(always)]
    fn from(variant: Fncsel106) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fncsel106 {
    type Ux = u8;
}
impl crate::IsEnum for Fncsel106 {}
#[doc = "Field `FNCSEL106` reader - Function select for GPIO pin 106"]
pub type Fncsel106R = crate::FieldReader<Fncsel106>;
impl Fncsel106R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fncsel106 {
        match self.bits {
            0 => Fncsel106::Reserved0,
            1 => Fncsel106::Reserved1,
            2 => Fncsel106::Reserved2,
            3 => Fncsel106::Gpio,
            4 => Fncsel106::Reserved4,
            5 => Fncsel106::Reserved5,
            6 => Fncsel106::Ct106,
            7 => Fncsel106::Reserved7,
            8 => Fncsel106::Obsbus10,
            9 => Fncsel106::Reserved9,
            10 => Fncsel106::Reserved10,
            11 => Fncsel106::Reserved11,
            12 => Fncsel106::Reserved12,
            13 => Fncsel106::Reserved13,
            14 => Fncsel106::Reserved14,
            15 => Fncsel106::Reserved15,
            _ => unreachable!(),
        }
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved0(&self) -> bool {
        *self == Fncsel106::Reserved0
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved1(&self) -> bool {
        *self == Fncsel106::Reserved1
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == Fncsel106::Reserved2
    }
    #[doc = "General purpose I/O"]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == Fncsel106::Gpio
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved4(&self) -> bool {
        *self == Fncsel106::Reserved4
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved5(&self) -> bool {
        *self == Fncsel106::Reserved5
    }
    #[doc = "Timer/Counter input or output; Selection of direction is done via CTIMER register settings."]
    #[inline(always)]
    pub fn is_ct106(&self) -> bool {
        *self == Fncsel106::Ct106
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved7(&self) -> bool {
        *self == Fncsel106::Reserved7
    }
    #[doc = "Observation bus bit 10"]
    #[inline(always)]
    pub fn is_obsbus10(&self) -> bool {
        *self == Fncsel106::Obsbus10
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved9(&self) -> bool {
        *self == Fncsel106::Reserved9
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved10(&self) -> bool {
        *self == Fncsel106::Reserved10
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved11(&self) -> bool {
        *self == Fncsel106::Reserved11
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved12(&self) -> bool {
        *self == Fncsel106::Reserved12
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved13(&self) -> bool {
        *self == Fncsel106::Reserved13
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved14(&self) -> bool {
        *self == Fncsel106::Reserved14
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved15(&self) -> bool {
        *self == Fncsel106::Reserved15
    }
}
#[doc = "Field `FNCSEL106` writer - Function select for GPIO pin 106"]
pub type Fncsel106W<'a, REG> = crate::FieldWriter<'a, REG, 4, Fncsel106, crate::Safe>;
impl<'a, REG> Fncsel106W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved0(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel106::Reserved0)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved1(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel106::Reserved1)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel106::Reserved2)
    }
    #[doc = "General purpose I/O"]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel106::Gpio)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved4(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel106::Reserved4)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved5(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel106::Reserved5)
    }
    #[doc = "Timer/Counter input or output; Selection of direction is done via CTIMER register settings."]
    #[inline(always)]
    pub fn ct106(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel106::Ct106)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved7(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel106::Reserved7)
    }
    #[doc = "Observation bus bit 10"]
    #[inline(always)]
    pub fn obsbus10(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel106::Obsbus10)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved9(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel106::Reserved9)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved10(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel106::Reserved10)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved11(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel106::Reserved11)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved12(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel106::Reserved12)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved13(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel106::Reserved13)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved14(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel106::Reserved14)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved15(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel106::Reserved15)
    }
}
#[doc = "Field `INPEN106` reader - Input enable for GPIO 106"]
pub type Inpen106R = crate::BitReader;
#[doc = "Field `INPEN106` writer - Input enable for GPIO 106"]
pub type Inpen106W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDZERO106` reader - Return 0 for read data on GPIO 106"]
pub type Rdzero106R = crate::BitReader;
#[doc = "Field `RDZERO106` writer - Return 0 for read data on GPIO 106"]
pub type Rdzero106W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Interrupt enable for GPIO 106\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Irpten106 {
    #[doc = "0: Interrupts are disabled for this GPIO"]
    Dis = 0,
    #[doc = "1: Interrupts are enabled for falling edge transition on this GPIO"]
    Intfall = 1,
    #[doc = "2: Interrupts are enabled for rising edge transitions on this GPIO"]
    Intrise = 2,
    #[doc = "3: Interrupts are enabled for any edge transition on this GPIO"]
    Intany = 3,
}
impl From<Irpten106> for u8 {
    #[inline(always)]
    fn from(variant: Irpten106) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Irpten106 {
    type Ux = u8;
}
impl crate::IsEnum for Irpten106 {}
#[doc = "Field `IRPTEN106` reader - Interrupt enable for GPIO 106"]
pub type Irpten106R = crate::FieldReader<Irpten106>;
impl Irpten106R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irpten106 {
        match self.bits {
            0 => Irpten106::Dis,
            1 => Irpten106::Intfall,
            2 => Irpten106::Intrise,
            3 => Irpten106::Intany,
            _ => unreachable!(),
        }
    }
    #[doc = "Interrupts are disabled for this GPIO"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Irpten106::Dis
    }
    #[doc = "Interrupts are enabled for falling edge transition on this GPIO"]
    #[inline(always)]
    pub fn is_intfall(&self) -> bool {
        *self == Irpten106::Intfall
    }
    #[doc = "Interrupts are enabled for rising edge transitions on this GPIO"]
    #[inline(always)]
    pub fn is_intrise(&self) -> bool {
        *self == Irpten106::Intrise
    }
    #[doc = "Interrupts are enabled for any edge transition on this GPIO"]
    #[inline(always)]
    pub fn is_intany(&self) -> bool {
        *self == Irpten106::Intany
    }
}
#[doc = "Field `IRPTEN106` writer - Interrupt enable for GPIO 106"]
pub type Irpten106W<'a, REG> = crate::FieldWriter<'a, REG, 2, Irpten106, crate::Safe>;
impl<'a, REG> Irpten106W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Interrupts are disabled for this GPIO"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten106::Dis)
    }
    #[doc = "Interrupts are enabled for falling edge transition on this GPIO"]
    #[inline(always)]
    pub fn intfall(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten106::Intfall)
    }
    #[doc = "Interrupts are enabled for rising edge transitions on this GPIO"]
    #[inline(always)]
    pub fn intrise(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten106::Intrise)
    }
    #[doc = "Interrupts are enabled for any edge transition on this GPIO"]
    #[inline(always)]
    pub fn intany(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten106::Intany)
    }
}
#[doc = "Pin IO mode selection for GPIO pin 106\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Outcfg106 {
    #[doc = "0: Output Disabled"]
    Dis = 0,
    #[doc = "1: Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    Pushpull = 1,
    #[doc = "2: Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    Od = 2,
    #[doc = "3: Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    Ts = 3,
}
impl From<Outcfg106> for u8 {
    #[inline(always)]
    fn from(variant: Outcfg106) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Outcfg106 {
    type Ux = u8;
}
impl crate::IsEnum for Outcfg106 {}
#[doc = "Field `OUTCFG106` reader - Pin IO mode selection for GPIO pin 106"]
pub type Outcfg106R = crate::FieldReader<Outcfg106>;
impl Outcfg106R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Outcfg106 {
        match self.bits {
            0 => Outcfg106::Dis,
            1 => Outcfg106::Pushpull,
            2 => Outcfg106::Od,
            3 => Outcfg106::Ts,
            _ => unreachable!(),
        }
    }
    #[doc = "Output Disabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Outcfg106::Dis
    }
    #[doc = "Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == Outcfg106::Pushpull
    }
    #[doc = "Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == Outcfg106::Od
    }
    #[doc = "Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == Outcfg106::Ts
    }
}
#[doc = "Field `OUTCFG106` writer - Pin IO mode selection for GPIO pin 106"]
pub type Outcfg106W<'a, REG> = crate::FieldWriter<'a, REG, 2, Outcfg106, crate::Safe>;
impl<'a, REG> Outcfg106W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Output Disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg106::Dis)
    }
    #[doc = "Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg106::Pushpull)
    }
    #[doc = "Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    #[inline(always)]
    pub fn od(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg106::Od)
    }
    #[doc = "Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    #[inline(always)]
    pub fn ts(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg106::Ts)
    }
}
impl R {
    #[doc = "Bits 0:3 - Function select for GPIO pin 106"]
    #[inline(always)]
    pub fn fncsel106(&self) -> Fncsel106R {
        Fncsel106R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Input enable for GPIO 106"]
    #[inline(always)]
    pub fn inpen106(&self) -> Inpen106R {
        Inpen106R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Return 0 for read data on GPIO 106"]
    #[inline(always)]
    pub fn rdzero106(&self) -> Rdzero106R {
        Rdzero106R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Interrupt enable for GPIO 106"]
    #[inline(always)]
    pub fn irpten106(&self) -> Irpten106R {
        Irpten106R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Pin IO mode selection for GPIO pin 106"]
    #[inline(always)]
    pub fn outcfg106(&self) -> Outcfg106R {
        Outcfg106R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Function select for GPIO pin 106"]
    #[inline(always)]
    #[must_use]
    pub fn fncsel106(&mut self) -> Fncsel106W<Pincfg106Spec> {
        Fncsel106W::new(self, 0)
    }
    #[doc = "Bit 4 - Input enable for GPIO 106"]
    #[inline(always)]
    #[must_use]
    pub fn inpen106(&mut self) -> Inpen106W<Pincfg106Spec> {
        Inpen106W::new(self, 4)
    }
    #[doc = "Bit 5 - Return 0 for read data on GPIO 106"]
    #[inline(always)]
    #[must_use]
    pub fn rdzero106(&mut self) -> Rdzero106W<Pincfg106Spec> {
        Rdzero106W::new(self, 5)
    }
    #[doc = "Bits 6:7 - Interrupt enable for GPIO 106"]
    #[inline(always)]
    #[must_use]
    pub fn irpten106(&mut self) -> Irpten106W<Pincfg106Spec> {
        Irpten106W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Pin IO mode selection for GPIO pin 106"]
    #[inline(always)]
    #[must_use]
    pub fn outcfg106(&mut self) -> Outcfg106W<Pincfg106Spec> {
        Outcfg106W::new(self, 8)
    }
}
#[doc = "Controls the operation of virtual GPIO pin 106.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg106::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg106::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pincfg106Spec;
impl crate::RegisterSpec for Pincfg106Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pincfg106::R`](R) reader structure"]
impl crate::Readable for Pincfg106Spec {}
#[doc = "`write(|w| ..)` method takes [`pincfg106::W`](W) writer structure"]
impl crate::Writable for Pincfg106Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PINCFG106 to value 0x03"]
impl crate::Resettable for Pincfg106Spec {
    const RESET_VALUE: u32 = 0x03;
}
