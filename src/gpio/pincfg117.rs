#[doc = "Register `PINCFG117` reader"]
pub type R = crate::R<Pincfg117Spec>;
#[doc = "Register `PINCFG117` writer"]
pub type W = crate::W<Pincfg117Spec>;
#[doc = "Function select for GPIO pin 117\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fncsel117 {
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
    Ct117 = 6,
    #[doc = "7: Reserved selection. Operation unknown if selected."]
    Reserved7 = 7,
    #[doc = "8: Observation bus bit 5"]
    Obsbus5 = 8,
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
impl From<Fncsel117> for u8 {
    #[inline(always)]
    fn from(variant: Fncsel117) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fncsel117 {
    type Ux = u8;
}
impl crate::IsEnum for Fncsel117 {}
#[doc = "Field `FNCSEL117` reader - Function select for GPIO pin 117"]
pub type Fncsel117R = crate::FieldReader<Fncsel117>;
impl Fncsel117R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fncsel117 {
        match self.bits {
            0 => Fncsel117::Reserved0,
            1 => Fncsel117::Reserved1,
            2 => Fncsel117::Reserved2,
            3 => Fncsel117::Gpio,
            4 => Fncsel117::Reserved4,
            5 => Fncsel117::Reserved5,
            6 => Fncsel117::Ct117,
            7 => Fncsel117::Reserved7,
            8 => Fncsel117::Obsbus5,
            9 => Fncsel117::Reserved9,
            10 => Fncsel117::Reserved10,
            11 => Fncsel117::Reserved11,
            12 => Fncsel117::Reserved12,
            13 => Fncsel117::Reserved13,
            14 => Fncsel117::Reserved14,
            15 => Fncsel117::Reserved15,
            _ => unreachable!(),
        }
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved0(&self) -> bool {
        *self == Fncsel117::Reserved0
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved1(&self) -> bool {
        *self == Fncsel117::Reserved1
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == Fncsel117::Reserved2
    }
    #[doc = "General purpose I/O"]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == Fncsel117::Gpio
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved4(&self) -> bool {
        *self == Fncsel117::Reserved4
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved5(&self) -> bool {
        *self == Fncsel117::Reserved5
    }
    #[doc = "Timer/Counter input or output; Selection of direction is done via CTIMER register settings."]
    #[inline(always)]
    pub fn is_ct117(&self) -> bool {
        *self == Fncsel117::Ct117
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved7(&self) -> bool {
        *self == Fncsel117::Reserved7
    }
    #[doc = "Observation bus bit 5"]
    #[inline(always)]
    pub fn is_obsbus5(&self) -> bool {
        *self == Fncsel117::Obsbus5
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved9(&self) -> bool {
        *self == Fncsel117::Reserved9
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved10(&self) -> bool {
        *self == Fncsel117::Reserved10
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved11(&self) -> bool {
        *self == Fncsel117::Reserved11
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved12(&self) -> bool {
        *self == Fncsel117::Reserved12
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved13(&self) -> bool {
        *self == Fncsel117::Reserved13
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved14(&self) -> bool {
        *self == Fncsel117::Reserved14
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved15(&self) -> bool {
        *self == Fncsel117::Reserved15
    }
}
#[doc = "Field `FNCSEL117` writer - Function select for GPIO pin 117"]
pub type Fncsel117W<'a, REG> = crate::FieldWriter<'a, REG, 4, Fncsel117, crate::Safe>;
impl<'a, REG> Fncsel117W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved0(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel117::Reserved0)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved1(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel117::Reserved1)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel117::Reserved2)
    }
    #[doc = "General purpose I/O"]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel117::Gpio)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved4(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel117::Reserved4)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved5(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel117::Reserved5)
    }
    #[doc = "Timer/Counter input or output; Selection of direction is done via CTIMER register settings."]
    #[inline(always)]
    pub fn ct117(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel117::Ct117)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved7(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel117::Reserved7)
    }
    #[doc = "Observation bus bit 5"]
    #[inline(always)]
    pub fn obsbus5(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel117::Obsbus5)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved9(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel117::Reserved9)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved10(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel117::Reserved10)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved11(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel117::Reserved11)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved12(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel117::Reserved12)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved13(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel117::Reserved13)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved14(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel117::Reserved14)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved15(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel117::Reserved15)
    }
}
#[doc = "Field `INPEN117` reader - Input enable for GPIO 117"]
pub type Inpen117R = crate::BitReader;
#[doc = "Field `INPEN117` writer - Input enable for GPIO 117"]
pub type Inpen117W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDZERO117` reader - Return 0 for read data on GPIO 117"]
pub type Rdzero117R = crate::BitReader;
#[doc = "Field `RDZERO117` writer - Return 0 for read data on GPIO 117"]
pub type Rdzero117W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Interrupt enable for GPIO 117\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Irpten117 {
    #[doc = "0: Interrupts are disabled for this GPIO"]
    Dis = 0,
    #[doc = "1: Interrupts are enabled for falling edge transition on this GPIO"]
    Intfall = 1,
    #[doc = "2: Interrupts are enabled for rising edge transitions on this GPIO"]
    Intrise = 2,
    #[doc = "3: Interrupts are enabled for any edge transition on this GPIO"]
    Intany = 3,
}
impl From<Irpten117> for u8 {
    #[inline(always)]
    fn from(variant: Irpten117) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Irpten117 {
    type Ux = u8;
}
impl crate::IsEnum for Irpten117 {}
#[doc = "Field `IRPTEN117` reader - Interrupt enable for GPIO 117"]
pub type Irpten117R = crate::FieldReader<Irpten117>;
impl Irpten117R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irpten117 {
        match self.bits {
            0 => Irpten117::Dis,
            1 => Irpten117::Intfall,
            2 => Irpten117::Intrise,
            3 => Irpten117::Intany,
            _ => unreachable!(),
        }
    }
    #[doc = "Interrupts are disabled for this GPIO"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Irpten117::Dis
    }
    #[doc = "Interrupts are enabled for falling edge transition on this GPIO"]
    #[inline(always)]
    pub fn is_intfall(&self) -> bool {
        *self == Irpten117::Intfall
    }
    #[doc = "Interrupts are enabled for rising edge transitions on this GPIO"]
    #[inline(always)]
    pub fn is_intrise(&self) -> bool {
        *self == Irpten117::Intrise
    }
    #[doc = "Interrupts are enabled for any edge transition on this GPIO"]
    #[inline(always)]
    pub fn is_intany(&self) -> bool {
        *self == Irpten117::Intany
    }
}
#[doc = "Field `IRPTEN117` writer - Interrupt enable for GPIO 117"]
pub type Irpten117W<'a, REG> = crate::FieldWriter<'a, REG, 2, Irpten117, crate::Safe>;
impl<'a, REG> Irpten117W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Interrupts are disabled for this GPIO"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten117::Dis)
    }
    #[doc = "Interrupts are enabled for falling edge transition on this GPIO"]
    #[inline(always)]
    pub fn intfall(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten117::Intfall)
    }
    #[doc = "Interrupts are enabled for rising edge transitions on this GPIO"]
    #[inline(always)]
    pub fn intrise(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten117::Intrise)
    }
    #[doc = "Interrupts are enabled for any edge transition on this GPIO"]
    #[inline(always)]
    pub fn intany(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten117::Intany)
    }
}
#[doc = "Pin IO mode selection for GPIO pin 117\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Outcfg117 {
    #[doc = "0: Output Disabled"]
    Dis = 0,
    #[doc = "1: Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    Pushpull = 1,
    #[doc = "2: Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    Od = 2,
    #[doc = "3: Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    Ts = 3,
}
impl From<Outcfg117> for u8 {
    #[inline(always)]
    fn from(variant: Outcfg117) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Outcfg117 {
    type Ux = u8;
}
impl crate::IsEnum for Outcfg117 {}
#[doc = "Field `OUTCFG117` reader - Pin IO mode selection for GPIO pin 117"]
pub type Outcfg117R = crate::FieldReader<Outcfg117>;
impl Outcfg117R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Outcfg117 {
        match self.bits {
            0 => Outcfg117::Dis,
            1 => Outcfg117::Pushpull,
            2 => Outcfg117::Od,
            3 => Outcfg117::Ts,
            _ => unreachable!(),
        }
    }
    #[doc = "Output Disabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Outcfg117::Dis
    }
    #[doc = "Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == Outcfg117::Pushpull
    }
    #[doc = "Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == Outcfg117::Od
    }
    #[doc = "Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == Outcfg117::Ts
    }
}
#[doc = "Field `OUTCFG117` writer - Pin IO mode selection for GPIO pin 117"]
pub type Outcfg117W<'a, REG> = crate::FieldWriter<'a, REG, 2, Outcfg117, crate::Safe>;
impl<'a, REG> Outcfg117W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Output Disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg117::Dis)
    }
    #[doc = "Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg117::Pushpull)
    }
    #[doc = "Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    #[inline(always)]
    pub fn od(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg117::Od)
    }
    #[doc = "Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    #[inline(always)]
    pub fn ts(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg117::Ts)
    }
}
impl R {
    #[doc = "Bits 0:3 - Function select for GPIO pin 117"]
    #[inline(always)]
    pub fn fncsel117(&self) -> Fncsel117R {
        Fncsel117R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Input enable for GPIO 117"]
    #[inline(always)]
    pub fn inpen117(&self) -> Inpen117R {
        Inpen117R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Return 0 for read data on GPIO 117"]
    #[inline(always)]
    pub fn rdzero117(&self) -> Rdzero117R {
        Rdzero117R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Interrupt enable for GPIO 117"]
    #[inline(always)]
    pub fn irpten117(&self) -> Irpten117R {
        Irpten117R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Pin IO mode selection for GPIO pin 117"]
    #[inline(always)]
    pub fn outcfg117(&self) -> Outcfg117R {
        Outcfg117R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Function select for GPIO pin 117"]
    #[inline(always)]
    #[must_use]
    pub fn fncsel117(&mut self) -> Fncsel117W<Pincfg117Spec> {
        Fncsel117W::new(self, 0)
    }
    #[doc = "Bit 4 - Input enable for GPIO 117"]
    #[inline(always)]
    #[must_use]
    pub fn inpen117(&mut self) -> Inpen117W<Pincfg117Spec> {
        Inpen117W::new(self, 4)
    }
    #[doc = "Bit 5 - Return 0 for read data on GPIO 117"]
    #[inline(always)]
    #[must_use]
    pub fn rdzero117(&mut self) -> Rdzero117W<Pincfg117Spec> {
        Rdzero117W::new(self, 5)
    }
    #[doc = "Bits 6:7 - Interrupt enable for GPIO 117"]
    #[inline(always)]
    #[must_use]
    pub fn irpten117(&mut self) -> Irpten117W<Pincfg117Spec> {
        Irpten117W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Pin IO mode selection for GPIO pin 117"]
    #[inline(always)]
    #[must_use]
    pub fn outcfg117(&mut self) -> Outcfg117W<Pincfg117Spec> {
        Outcfg117W::new(self, 8)
    }
}
#[doc = "Controls the operation of virtual GPIO pin 117.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg117::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg117::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pincfg117Spec;
impl crate::RegisterSpec for Pincfg117Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pincfg117::R`](R) reader structure"]
impl crate::Readable for Pincfg117Spec {}
#[doc = "`write(|w| ..)` method takes [`pincfg117::W`](W) writer structure"]
impl crate::Writable for Pincfg117Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PINCFG117 to value 0x03"]
impl crate::Resettable for Pincfg117Spec {
    const RESET_VALUE: u32 = 0x03;
}
