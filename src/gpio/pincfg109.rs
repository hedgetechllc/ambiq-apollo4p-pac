#[doc = "Register `PINCFG109` reader"]
pub type R = crate::R<Pincfg109Spec>;
#[doc = "Register `PINCFG109` writer"]
pub type W = crate::W<Pincfg109Spec>;
#[doc = "Function select for GPIO pin 109\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fncsel109 {
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
    Ct109 = 6,
    #[doc = "7: Reserved selection. Operation unknown if selected."]
    Reserved7 = 7,
    #[doc = "8: Observation bus bit 13"]
    Obsbus13 = 8,
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
impl From<Fncsel109> for u8 {
    #[inline(always)]
    fn from(variant: Fncsel109) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fncsel109 {
    type Ux = u8;
}
impl crate::IsEnum for Fncsel109 {}
#[doc = "Field `FNCSEL109` reader - Function select for GPIO pin 109"]
pub type Fncsel109R = crate::FieldReader<Fncsel109>;
impl Fncsel109R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fncsel109 {
        match self.bits {
            0 => Fncsel109::Reserved0,
            1 => Fncsel109::Reserved1,
            2 => Fncsel109::Reserved2,
            3 => Fncsel109::Gpio,
            4 => Fncsel109::Reserved4,
            5 => Fncsel109::Reserved5,
            6 => Fncsel109::Ct109,
            7 => Fncsel109::Reserved7,
            8 => Fncsel109::Obsbus13,
            9 => Fncsel109::Reserved9,
            10 => Fncsel109::Reserved10,
            11 => Fncsel109::Reserved11,
            12 => Fncsel109::Reserved12,
            13 => Fncsel109::Reserved13,
            14 => Fncsel109::Reserved14,
            15 => Fncsel109::Reserved15,
            _ => unreachable!(),
        }
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved0(&self) -> bool {
        *self == Fncsel109::Reserved0
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved1(&self) -> bool {
        *self == Fncsel109::Reserved1
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == Fncsel109::Reserved2
    }
    #[doc = "General purpose I/O"]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == Fncsel109::Gpio
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved4(&self) -> bool {
        *self == Fncsel109::Reserved4
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved5(&self) -> bool {
        *self == Fncsel109::Reserved5
    }
    #[doc = "Timer/Counter input or output; Selection of direction is done via CTIMER register settings."]
    #[inline(always)]
    pub fn is_ct109(&self) -> bool {
        *self == Fncsel109::Ct109
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved7(&self) -> bool {
        *self == Fncsel109::Reserved7
    }
    #[doc = "Observation bus bit 13"]
    #[inline(always)]
    pub fn is_obsbus13(&self) -> bool {
        *self == Fncsel109::Obsbus13
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved9(&self) -> bool {
        *self == Fncsel109::Reserved9
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved10(&self) -> bool {
        *self == Fncsel109::Reserved10
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved11(&self) -> bool {
        *self == Fncsel109::Reserved11
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved12(&self) -> bool {
        *self == Fncsel109::Reserved12
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved13(&self) -> bool {
        *self == Fncsel109::Reserved13
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved14(&self) -> bool {
        *self == Fncsel109::Reserved14
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved15(&self) -> bool {
        *self == Fncsel109::Reserved15
    }
}
#[doc = "Field `FNCSEL109` writer - Function select for GPIO pin 109"]
pub type Fncsel109W<'a, REG> = crate::FieldWriter<'a, REG, 4, Fncsel109, crate::Safe>;
impl<'a, REG> Fncsel109W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved0(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel109::Reserved0)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved1(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel109::Reserved1)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel109::Reserved2)
    }
    #[doc = "General purpose I/O"]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel109::Gpio)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved4(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel109::Reserved4)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved5(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel109::Reserved5)
    }
    #[doc = "Timer/Counter input or output; Selection of direction is done via CTIMER register settings."]
    #[inline(always)]
    pub fn ct109(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel109::Ct109)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved7(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel109::Reserved7)
    }
    #[doc = "Observation bus bit 13"]
    #[inline(always)]
    pub fn obsbus13(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel109::Obsbus13)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved9(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel109::Reserved9)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved10(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel109::Reserved10)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved11(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel109::Reserved11)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved12(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel109::Reserved12)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved13(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel109::Reserved13)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved14(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel109::Reserved14)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved15(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel109::Reserved15)
    }
}
#[doc = "Field `INPEN109` reader - Input enable for GPIO 109"]
pub type Inpen109R = crate::BitReader;
#[doc = "Field `INPEN109` writer - Input enable for GPIO 109"]
pub type Inpen109W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDZERO109` reader - Return 0 for read data on GPIO 109"]
pub type Rdzero109R = crate::BitReader;
#[doc = "Field `RDZERO109` writer - Return 0 for read data on GPIO 109"]
pub type Rdzero109W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Interrupt enable for GPIO 109\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Irpten109 {
    #[doc = "0: Interrupts are disabled for this GPIO"]
    Dis = 0,
    #[doc = "1: Interrupts are enabled for falling edge transition on this GPIO"]
    Intfall = 1,
    #[doc = "2: Interrupts are enabled for rising edge transitions on this GPIO"]
    Intrise = 2,
    #[doc = "3: Interrupts are enabled for any edge transition on this GPIO"]
    Intany = 3,
}
impl From<Irpten109> for u8 {
    #[inline(always)]
    fn from(variant: Irpten109) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Irpten109 {
    type Ux = u8;
}
impl crate::IsEnum for Irpten109 {}
#[doc = "Field `IRPTEN109` reader - Interrupt enable for GPIO 109"]
pub type Irpten109R = crate::FieldReader<Irpten109>;
impl Irpten109R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irpten109 {
        match self.bits {
            0 => Irpten109::Dis,
            1 => Irpten109::Intfall,
            2 => Irpten109::Intrise,
            3 => Irpten109::Intany,
            _ => unreachable!(),
        }
    }
    #[doc = "Interrupts are disabled for this GPIO"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Irpten109::Dis
    }
    #[doc = "Interrupts are enabled for falling edge transition on this GPIO"]
    #[inline(always)]
    pub fn is_intfall(&self) -> bool {
        *self == Irpten109::Intfall
    }
    #[doc = "Interrupts are enabled for rising edge transitions on this GPIO"]
    #[inline(always)]
    pub fn is_intrise(&self) -> bool {
        *self == Irpten109::Intrise
    }
    #[doc = "Interrupts are enabled for any edge transition on this GPIO"]
    #[inline(always)]
    pub fn is_intany(&self) -> bool {
        *self == Irpten109::Intany
    }
}
#[doc = "Field `IRPTEN109` writer - Interrupt enable for GPIO 109"]
pub type Irpten109W<'a, REG> = crate::FieldWriter<'a, REG, 2, Irpten109, crate::Safe>;
impl<'a, REG> Irpten109W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Interrupts are disabled for this GPIO"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten109::Dis)
    }
    #[doc = "Interrupts are enabled for falling edge transition on this GPIO"]
    #[inline(always)]
    pub fn intfall(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten109::Intfall)
    }
    #[doc = "Interrupts are enabled for rising edge transitions on this GPIO"]
    #[inline(always)]
    pub fn intrise(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten109::Intrise)
    }
    #[doc = "Interrupts are enabled for any edge transition on this GPIO"]
    #[inline(always)]
    pub fn intany(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten109::Intany)
    }
}
#[doc = "Pin IO mode selection for GPIO pin 109\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Outcfg109 {
    #[doc = "0: Output Disabled"]
    Dis = 0,
    #[doc = "1: Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    Pushpull = 1,
    #[doc = "2: Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    Od = 2,
    #[doc = "3: Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    Ts = 3,
}
impl From<Outcfg109> for u8 {
    #[inline(always)]
    fn from(variant: Outcfg109) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Outcfg109 {
    type Ux = u8;
}
impl crate::IsEnum for Outcfg109 {}
#[doc = "Field `OUTCFG109` reader - Pin IO mode selection for GPIO pin 109"]
pub type Outcfg109R = crate::FieldReader<Outcfg109>;
impl Outcfg109R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Outcfg109 {
        match self.bits {
            0 => Outcfg109::Dis,
            1 => Outcfg109::Pushpull,
            2 => Outcfg109::Od,
            3 => Outcfg109::Ts,
            _ => unreachable!(),
        }
    }
    #[doc = "Output Disabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Outcfg109::Dis
    }
    #[doc = "Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == Outcfg109::Pushpull
    }
    #[doc = "Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == Outcfg109::Od
    }
    #[doc = "Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == Outcfg109::Ts
    }
}
#[doc = "Field `OUTCFG109` writer - Pin IO mode selection for GPIO pin 109"]
pub type Outcfg109W<'a, REG> = crate::FieldWriter<'a, REG, 2, Outcfg109, crate::Safe>;
impl<'a, REG> Outcfg109W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Output Disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg109::Dis)
    }
    #[doc = "Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg109::Pushpull)
    }
    #[doc = "Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    #[inline(always)]
    pub fn od(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg109::Od)
    }
    #[doc = "Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    #[inline(always)]
    pub fn ts(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg109::Ts)
    }
}
impl R {
    #[doc = "Bits 0:3 - Function select for GPIO pin 109"]
    #[inline(always)]
    pub fn fncsel109(&self) -> Fncsel109R {
        Fncsel109R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Input enable for GPIO 109"]
    #[inline(always)]
    pub fn inpen109(&self) -> Inpen109R {
        Inpen109R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Return 0 for read data on GPIO 109"]
    #[inline(always)]
    pub fn rdzero109(&self) -> Rdzero109R {
        Rdzero109R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Interrupt enable for GPIO 109"]
    #[inline(always)]
    pub fn irpten109(&self) -> Irpten109R {
        Irpten109R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Pin IO mode selection for GPIO pin 109"]
    #[inline(always)]
    pub fn outcfg109(&self) -> Outcfg109R {
        Outcfg109R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Function select for GPIO pin 109"]
    #[inline(always)]
    #[must_use]
    pub fn fncsel109(&mut self) -> Fncsel109W<Pincfg109Spec> {
        Fncsel109W::new(self, 0)
    }
    #[doc = "Bit 4 - Input enable for GPIO 109"]
    #[inline(always)]
    #[must_use]
    pub fn inpen109(&mut self) -> Inpen109W<Pincfg109Spec> {
        Inpen109W::new(self, 4)
    }
    #[doc = "Bit 5 - Return 0 for read data on GPIO 109"]
    #[inline(always)]
    #[must_use]
    pub fn rdzero109(&mut self) -> Rdzero109W<Pincfg109Spec> {
        Rdzero109W::new(self, 5)
    }
    #[doc = "Bits 6:7 - Interrupt enable for GPIO 109"]
    #[inline(always)]
    #[must_use]
    pub fn irpten109(&mut self) -> Irpten109W<Pincfg109Spec> {
        Irpten109W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Pin IO mode selection for GPIO pin 109"]
    #[inline(always)]
    #[must_use]
    pub fn outcfg109(&mut self) -> Outcfg109W<Pincfg109Spec> {
        Outcfg109W::new(self, 8)
    }
}
#[doc = "Controls the operation of virtual GPIO pin 109.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg109::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg109::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pincfg109Spec;
impl crate::RegisterSpec for Pincfg109Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pincfg109::R`](R) reader structure"]
impl crate::Readable for Pincfg109Spec {}
#[doc = "`write(|w| ..)` method takes [`pincfg109::W`](W) writer structure"]
impl crate::Writable for Pincfg109Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PINCFG109 to value 0x03"]
impl crate::Resettable for Pincfg109Spec {
    const RESET_VALUE: u32 = 0x03;
}
