#[doc = "Register `PINCFG127` reader"]
pub type R = crate::R<Pincfg127Spec>;
#[doc = "Register `PINCFG127` writer"]
pub type W = crate::W<Pincfg127Spec>;
#[doc = "Function select for GPIO pin 127\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fncsel127 {
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
    Ct127 = 6,
    #[doc = "7: Reserved selection. Operation unknown if selected."]
    Reserved7 = 7,
    #[doc = "8: Observation bus bit 15"]
    Obsbus15 = 8,
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
impl From<Fncsel127> for u8 {
    #[inline(always)]
    fn from(variant: Fncsel127) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fncsel127 {
    type Ux = u8;
}
impl crate::IsEnum for Fncsel127 {}
#[doc = "Field `FNCSEL127` reader - Function select for GPIO pin 127"]
pub type Fncsel127R = crate::FieldReader<Fncsel127>;
impl Fncsel127R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fncsel127 {
        match self.bits {
            0 => Fncsel127::Reserved0,
            1 => Fncsel127::Reserved1,
            2 => Fncsel127::Reserved2,
            3 => Fncsel127::Gpio,
            4 => Fncsel127::Reserved4,
            5 => Fncsel127::Reserved5,
            6 => Fncsel127::Ct127,
            7 => Fncsel127::Reserved7,
            8 => Fncsel127::Obsbus15,
            9 => Fncsel127::Reserved9,
            10 => Fncsel127::Reserved10,
            11 => Fncsel127::Reserved11,
            12 => Fncsel127::Reserved12,
            13 => Fncsel127::Reserved13,
            14 => Fncsel127::Reserved14,
            15 => Fncsel127::Reserved15,
            _ => unreachable!(),
        }
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved0(&self) -> bool {
        *self == Fncsel127::Reserved0
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved1(&self) -> bool {
        *self == Fncsel127::Reserved1
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == Fncsel127::Reserved2
    }
    #[doc = "General purpose I/O"]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == Fncsel127::Gpio
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved4(&self) -> bool {
        *self == Fncsel127::Reserved4
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved5(&self) -> bool {
        *self == Fncsel127::Reserved5
    }
    #[doc = "Timer/Counter input or output; Selection of direction is done via CTIMER register settings."]
    #[inline(always)]
    pub fn is_ct127(&self) -> bool {
        *self == Fncsel127::Ct127
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved7(&self) -> bool {
        *self == Fncsel127::Reserved7
    }
    #[doc = "Observation bus bit 15"]
    #[inline(always)]
    pub fn is_obsbus15(&self) -> bool {
        *self == Fncsel127::Obsbus15
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved9(&self) -> bool {
        *self == Fncsel127::Reserved9
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved10(&self) -> bool {
        *self == Fncsel127::Reserved10
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved11(&self) -> bool {
        *self == Fncsel127::Reserved11
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved12(&self) -> bool {
        *self == Fncsel127::Reserved12
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved13(&self) -> bool {
        *self == Fncsel127::Reserved13
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved14(&self) -> bool {
        *self == Fncsel127::Reserved14
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved15(&self) -> bool {
        *self == Fncsel127::Reserved15
    }
}
#[doc = "Field `FNCSEL127` writer - Function select for GPIO pin 127"]
pub type Fncsel127W<'a, REG> = crate::FieldWriter<'a, REG, 4, Fncsel127, crate::Safe>;
impl<'a, REG> Fncsel127W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved0(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel127::Reserved0)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved1(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel127::Reserved1)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel127::Reserved2)
    }
    #[doc = "General purpose I/O"]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel127::Gpio)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved4(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel127::Reserved4)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved5(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel127::Reserved5)
    }
    #[doc = "Timer/Counter input or output; Selection of direction is done via CTIMER register settings."]
    #[inline(always)]
    pub fn ct127(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel127::Ct127)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved7(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel127::Reserved7)
    }
    #[doc = "Observation bus bit 15"]
    #[inline(always)]
    pub fn obsbus15(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel127::Obsbus15)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved9(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel127::Reserved9)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved10(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel127::Reserved10)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved11(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel127::Reserved11)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved12(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel127::Reserved12)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved13(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel127::Reserved13)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved14(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel127::Reserved14)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved15(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel127::Reserved15)
    }
}
#[doc = "Field `INPEN127` reader - Input enable for GPIO 127"]
pub type Inpen127R = crate::BitReader;
#[doc = "Field `INPEN127` writer - Input enable for GPIO 127"]
pub type Inpen127W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDZERO127` reader - Return 0 for read data on GPIO 127"]
pub type Rdzero127R = crate::BitReader;
#[doc = "Field `RDZERO127` writer - Return 0 for read data on GPIO 127"]
pub type Rdzero127W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Interrupt enable for GPIO 127\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Irpten127 {
    #[doc = "0: Interrupts are disabled for this GPIO"]
    Dis = 0,
    #[doc = "1: Interrupts are enabled for falling edge transition on this GPIO"]
    Intfall = 1,
    #[doc = "2: Interrupts are enabled for rising edge transitions on this GPIO"]
    Intrise = 2,
    #[doc = "3: Interrupts are enabled for any edge transition on this GPIO"]
    Intany = 3,
}
impl From<Irpten127> for u8 {
    #[inline(always)]
    fn from(variant: Irpten127) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Irpten127 {
    type Ux = u8;
}
impl crate::IsEnum for Irpten127 {}
#[doc = "Field `IRPTEN127` reader - Interrupt enable for GPIO 127"]
pub type Irpten127R = crate::FieldReader<Irpten127>;
impl Irpten127R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irpten127 {
        match self.bits {
            0 => Irpten127::Dis,
            1 => Irpten127::Intfall,
            2 => Irpten127::Intrise,
            3 => Irpten127::Intany,
            _ => unreachable!(),
        }
    }
    #[doc = "Interrupts are disabled for this GPIO"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Irpten127::Dis
    }
    #[doc = "Interrupts are enabled for falling edge transition on this GPIO"]
    #[inline(always)]
    pub fn is_intfall(&self) -> bool {
        *self == Irpten127::Intfall
    }
    #[doc = "Interrupts are enabled for rising edge transitions on this GPIO"]
    #[inline(always)]
    pub fn is_intrise(&self) -> bool {
        *self == Irpten127::Intrise
    }
    #[doc = "Interrupts are enabled for any edge transition on this GPIO"]
    #[inline(always)]
    pub fn is_intany(&self) -> bool {
        *self == Irpten127::Intany
    }
}
#[doc = "Field `IRPTEN127` writer - Interrupt enable for GPIO 127"]
pub type Irpten127W<'a, REG> = crate::FieldWriter<'a, REG, 2, Irpten127, crate::Safe>;
impl<'a, REG> Irpten127W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Interrupts are disabled for this GPIO"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten127::Dis)
    }
    #[doc = "Interrupts are enabled for falling edge transition on this GPIO"]
    #[inline(always)]
    pub fn intfall(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten127::Intfall)
    }
    #[doc = "Interrupts are enabled for rising edge transitions on this GPIO"]
    #[inline(always)]
    pub fn intrise(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten127::Intrise)
    }
    #[doc = "Interrupts are enabled for any edge transition on this GPIO"]
    #[inline(always)]
    pub fn intany(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten127::Intany)
    }
}
#[doc = "Pin IO mode selection for GPIO pin 127\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Outcfg127 {
    #[doc = "0: Output Disabled"]
    Dis = 0,
    #[doc = "1: Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    Pushpull = 1,
    #[doc = "2: Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    Od = 2,
    #[doc = "3: Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    Ts = 3,
}
impl From<Outcfg127> for u8 {
    #[inline(always)]
    fn from(variant: Outcfg127) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Outcfg127 {
    type Ux = u8;
}
impl crate::IsEnum for Outcfg127 {}
#[doc = "Field `OUTCFG127` reader - Pin IO mode selection for GPIO pin 127"]
pub type Outcfg127R = crate::FieldReader<Outcfg127>;
impl Outcfg127R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Outcfg127 {
        match self.bits {
            0 => Outcfg127::Dis,
            1 => Outcfg127::Pushpull,
            2 => Outcfg127::Od,
            3 => Outcfg127::Ts,
            _ => unreachable!(),
        }
    }
    #[doc = "Output Disabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Outcfg127::Dis
    }
    #[doc = "Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == Outcfg127::Pushpull
    }
    #[doc = "Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == Outcfg127::Od
    }
    #[doc = "Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == Outcfg127::Ts
    }
}
#[doc = "Field `OUTCFG127` writer - Pin IO mode selection for GPIO pin 127"]
pub type Outcfg127W<'a, REG> = crate::FieldWriter<'a, REG, 2, Outcfg127, crate::Safe>;
impl<'a, REG> Outcfg127W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Output Disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg127::Dis)
    }
    #[doc = "Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg127::Pushpull)
    }
    #[doc = "Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    #[inline(always)]
    pub fn od(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg127::Od)
    }
    #[doc = "Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    #[inline(always)]
    pub fn ts(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg127::Ts)
    }
}
impl R {
    #[doc = "Bits 0:3 - Function select for GPIO pin 127"]
    #[inline(always)]
    pub fn fncsel127(&self) -> Fncsel127R {
        Fncsel127R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Input enable for GPIO 127"]
    #[inline(always)]
    pub fn inpen127(&self) -> Inpen127R {
        Inpen127R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Return 0 for read data on GPIO 127"]
    #[inline(always)]
    pub fn rdzero127(&self) -> Rdzero127R {
        Rdzero127R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Interrupt enable for GPIO 127"]
    #[inline(always)]
    pub fn irpten127(&self) -> Irpten127R {
        Irpten127R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Pin IO mode selection for GPIO pin 127"]
    #[inline(always)]
    pub fn outcfg127(&self) -> Outcfg127R {
        Outcfg127R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Function select for GPIO pin 127"]
    #[inline(always)]
    #[must_use]
    pub fn fncsel127(&mut self) -> Fncsel127W<Pincfg127Spec> {
        Fncsel127W::new(self, 0)
    }
    #[doc = "Bit 4 - Input enable for GPIO 127"]
    #[inline(always)]
    #[must_use]
    pub fn inpen127(&mut self) -> Inpen127W<Pincfg127Spec> {
        Inpen127W::new(self, 4)
    }
    #[doc = "Bit 5 - Return 0 for read data on GPIO 127"]
    #[inline(always)]
    #[must_use]
    pub fn rdzero127(&mut self) -> Rdzero127W<Pincfg127Spec> {
        Rdzero127W::new(self, 5)
    }
    #[doc = "Bits 6:7 - Interrupt enable for GPIO 127"]
    #[inline(always)]
    #[must_use]
    pub fn irpten127(&mut self) -> Irpten127W<Pincfg127Spec> {
        Irpten127W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Pin IO mode selection for GPIO pin 127"]
    #[inline(always)]
    #[must_use]
    pub fn outcfg127(&mut self) -> Outcfg127W<Pincfg127Spec> {
        Outcfg127W::new(self, 8)
    }
}
#[doc = "Controls the operation of virtual GPIO pin 127.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg127::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg127::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pincfg127Spec;
impl crate::RegisterSpec for Pincfg127Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pincfg127::R`](R) reader structure"]
impl crate::Readable for Pincfg127Spec {}
#[doc = "`write(|w| ..)` method takes [`pincfg127::W`](W) writer structure"]
impl crate::Writable for Pincfg127Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PINCFG127 to value 0x03"]
impl crate::Resettable for Pincfg127Spec {
    const RESET_VALUE: u32 = 0x03;
}
