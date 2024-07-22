#[doc = "Register `PINCFG123` reader"]
pub type R = crate::R<Pincfg123Spec>;
#[doc = "Register `PINCFG123` writer"]
pub type W = crate::W<Pincfg123Spec>;
#[doc = "Function select for GPIO pin 123\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fncsel123 {
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
    Ct123 = 6,
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
impl From<Fncsel123> for u8 {
    #[inline(always)]
    fn from(variant: Fncsel123) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fncsel123 {
    type Ux = u8;
}
impl crate::IsEnum for Fncsel123 {}
#[doc = "Field `FNCSEL123` reader - Function select for GPIO pin 123"]
pub type Fncsel123R = crate::FieldReader<Fncsel123>;
impl Fncsel123R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fncsel123 {
        match self.bits {
            0 => Fncsel123::Reserved0,
            1 => Fncsel123::Reserved1,
            2 => Fncsel123::Reserved2,
            3 => Fncsel123::Gpio,
            4 => Fncsel123::Reserved4,
            5 => Fncsel123::Reserved5,
            6 => Fncsel123::Ct123,
            7 => Fncsel123::Reserved7,
            8 => Fncsel123::Obsbus11,
            9 => Fncsel123::Reserved9,
            10 => Fncsel123::Reserved10,
            11 => Fncsel123::Reserved11,
            12 => Fncsel123::Reserved12,
            13 => Fncsel123::Reserved13,
            14 => Fncsel123::Reserved14,
            15 => Fncsel123::Reserved15,
            _ => unreachable!(),
        }
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved0(&self) -> bool {
        *self == Fncsel123::Reserved0
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved1(&self) -> bool {
        *self == Fncsel123::Reserved1
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == Fncsel123::Reserved2
    }
    #[doc = "General purpose I/O"]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == Fncsel123::Gpio
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved4(&self) -> bool {
        *self == Fncsel123::Reserved4
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved5(&self) -> bool {
        *self == Fncsel123::Reserved5
    }
    #[doc = "Timer/Counter input or output; Selection of direction is done via CTIMER register settings."]
    #[inline(always)]
    pub fn is_ct123(&self) -> bool {
        *self == Fncsel123::Ct123
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved7(&self) -> bool {
        *self == Fncsel123::Reserved7
    }
    #[doc = "Observation bus bit 11"]
    #[inline(always)]
    pub fn is_obsbus11(&self) -> bool {
        *self == Fncsel123::Obsbus11
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved9(&self) -> bool {
        *self == Fncsel123::Reserved9
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved10(&self) -> bool {
        *self == Fncsel123::Reserved10
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved11(&self) -> bool {
        *self == Fncsel123::Reserved11
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved12(&self) -> bool {
        *self == Fncsel123::Reserved12
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved13(&self) -> bool {
        *self == Fncsel123::Reserved13
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved14(&self) -> bool {
        *self == Fncsel123::Reserved14
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved15(&self) -> bool {
        *self == Fncsel123::Reserved15
    }
}
#[doc = "Field `FNCSEL123` writer - Function select for GPIO pin 123"]
pub type Fncsel123W<'a, REG> = crate::FieldWriter<'a, REG, 4, Fncsel123, crate::Safe>;
impl<'a, REG> Fncsel123W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved0(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel123::Reserved0)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved1(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel123::Reserved1)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel123::Reserved2)
    }
    #[doc = "General purpose I/O"]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel123::Gpio)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved4(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel123::Reserved4)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved5(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel123::Reserved5)
    }
    #[doc = "Timer/Counter input or output; Selection of direction is done via CTIMER register settings."]
    #[inline(always)]
    pub fn ct123(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel123::Ct123)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved7(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel123::Reserved7)
    }
    #[doc = "Observation bus bit 11"]
    #[inline(always)]
    pub fn obsbus11(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel123::Obsbus11)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved9(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel123::Reserved9)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved10(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel123::Reserved10)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved11(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel123::Reserved11)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved12(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel123::Reserved12)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved13(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel123::Reserved13)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved14(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel123::Reserved14)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved15(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel123::Reserved15)
    }
}
#[doc = "Field `INPEN123` reader - Input enable for GPIO 123"]
pub type Inpen123R = crate::BitReader;
#[doc = "Field `INPEN123` writer - Input enable for GPIO 123"]
pub type Inpen123W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDZERO123` reader - Return 0 for read data on GPIO 123"]
pub type Rdzero123R = crate::BitReader;
#[doc = "Field `RDZERO123` writer - Return 0 for read data on GPIO 123"]
pub type Rdzero123W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Interrupt enable for GPIO 123\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Irpten123 {
    #[doc = "0: Interrupts are disabled for this GPIO"]
    Dis = 0,
    #[doc = "1: Interrupts are enabled for falling edge transition on this GPIO"]
    Intfall = 1,
    #[doc = "2: Interrupts are enabled for rising edge transitions on this GPIO"]
    Intrise = 2,
    #[doc = "3: Interrupts are enabled for any edge transition on this GPIO"]
    Intany = 3,
}
impl From<Irpten123> for u8 {
    #[inline(always)]
    fn from(variant: Irpten123) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Irpten123 {
    type Ux = u8;
}
impl crate::IsEnum for Irpten123 {}
#[doc = "Field `IRPTEN123` reader - Interrupt enable for GPIO 123"]
pub type Irpten123R = crate::FieldReader<Irpten123>;
impl Irpten123R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irpten123 {
        match self.bits {
            0 => Irpten123::Dis,
            1 => Irpten123::Intfall,
            2 => Irpten123::Intrise,
            3 => Irpten123::Intany,
            _ => unreachable!(),
        }
    }
    #[doc = "Interrupts are disabled for this GPIO"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Irpten123::Dis
    }
    #[doc = "Interrupts are enabled for falling edge transition on this GPIO"]
    #[inline(always)]
    pub fn is_intfall(&self) -> bool {
        *self == Irpten123::Intfall
    }
    #[doc = "Interrupts are enabled for rising edge transitions on this GPIO"]
    #[inline(always)]
    pub fn is_intrise(&self) -> bool {
        *self == Irpten123::Intrise
    }
    #[doc = "Interrupts are enabled for any edge transition on this GPIO"]
    #[inline(always)]
    pub fn is_intany(&self) -> bool {
        *self == Irpten123::Intany
    }
}
#[doc = "Field `IRPTEN123` writer - Interrupt enable for GPIO 123"]
pub type Irpten123W<'a, REG> = crate::FieldWriter<'a, REG, 2, Irpten123, crate::Safe>;
impl<'a, REG> Irpten123W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Interrupts are disabled for this GPIO"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten123::Dis)
    }
    #[doc = "Interrupts are enabled for falling edge transition on this GPIO"]
    #[inline(always)]
    pub fn intfall(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten123::Intfall)
    }
    #[doc = "Interrupts are enabled for rising edge transitions on this GPIO"]
    #[inline(always)]
    pub fn intrise(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten123::Intrise)
    }
    #[doc = "Interrupts are enabled for any edge transition on this GPIO"]
    #[inline(always)]
    pub fn intany(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten123::Intany)
    }
}
#[doc = "Pin IO mode selection for GPIO pin 123\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Outcfg123 {
    #[doc = "0: Output Disabled"]
    Dis = 0,
    #[doc = "1: Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    Pushpull = 1,
    #[doc = "2: Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    Od = 2,
    #[doc = "3: Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    Ts = 3,
}
impl From<Outcfg123> for u8 {
    #[inline(always)]
    fn from(variant: Outcfg123) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Outcfg123 {
    type Ux = u8;
}
impl crate::IsEnum for Outcfg123 {}
#[doc = "Field `OUTCFG123` reader - Pin IO mode selection for GPIO pin 123"]
pub type Outcfg123R = crate::FieldReader<Outcfg123>;
impl Outcfg123R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Outcfg123 {
        match self.bits {
            0 => Outcfg123::Dis,
            1 => Outcfg123::Pushpull,
            2 => Outcfg123::Od,
            3 => Outcfg123::Ts,
            _ => unreachable!(),
        }
    }
    #[doc = "Output Disabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Outcfg123::Dis
    }
    #[doc = "Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == Outcfg123::Pushpull
    }
    #[doc = "Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == Outcfg123::Od
    }
    #[doc = "Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == Outcfg123::Ts
    }
}
#[doc = "Field `OUTCFG123` writer - Pin IO mode selection for GPIO pin 123"]
pub type Outcfg123W<'a, REG> = crate::FieldWriter<'a, REG, 2, Outcfg123, crate::Safe>;
impl<'a, REG> Outcfg123W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Output Disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg123::Dis)
    }
    #[doc = "Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg123::Pushpull)
    }
    #[doc = "Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    #[inline(always)]
    pub fn od(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg123::Od)
    }
    #[doc = "Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    #[inline(always)]
    pub fn ts(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg123::Ts)
    }
}
impl R {
    #[doc = "Bits 0:3 - Function select for GPIO pin 123"]
    #[inline(always)]
    pub fn fncsel123(&self) -> Fncsel123R {
        Fncsel123R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Input enable for GPIO 123"]
    #[inline(always)]
    pub fn inpen123(&self) -> Inpen123R {
        Inpen123R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Return 0 for read data on GPIO 123"]
    #[inline(always)]
    pub fn rdzero123(&self) -> Rdzero123R {
        Rdzero123R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Interrupt enable for GPIO 123"]
    #[inline(always)]
    pub fn irpten123(&self) -> Irpten123R {
        Irpten123R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Pin IO mode selection for GPIO pin 123"]
    #[inline(always)]
    pub fn outcfg123(&self) -> Outcfg123R {
        Outcfg123R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Function select for GPIO pin 123"]
    #[inline(always)]
    #[must_use]
    pub fn fncsel123(&mut self) -> Fncsel123W<Pincfg123Spec> {
        Fncsel123W::new(self, 0)
    }
    #[doc = "Bit 4 - Input enable for GPIO 123"]
    #[inline(always)]
    #[must_use]
    pub fn inpen123(&mut self) -> Inpen123W<Pincfg123Spec> {
        Inpen123W::new(self, 4)
    }
    #[doc = "Bit 5 - Return 0 for read data on GPIO 123"]
    #[inline(always)]
    #[must_use]
    pub fn rdzero123(&mut self) -> Rdzero123W<Pincfg123Spec> {
        Rdzero123W::new(self, 5)
    }
    #[doc = "Bits 6:7 - Interrupt enable for GPIO 123"]
    #[inline(always)]
    #[must_use]
    pub fn irpten123(&mut self) -> Irpten123W<Pincfg123Spec> {
        Irpten123W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Pin IO mode selection for GPIO pin 123"]
    #[inline(always)]
    #[must_use]
    pub fn outcfg123(&mut self) -> Outcfg123W<Pincfg123Spec> {
        Outcfg123W::new(self, 8)
    }
}
#[doc = "Controls the operation of virtual GPIO pin 123.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg123::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg123::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pincfg123Spec;
impl crate::RegisterSpec for Pincfg123Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pincfg123::R`](R) reader structure"]
impl crate::Readable for Pincfg123Spec {}
#[doc = "`write(|w| ..)` method takes [`pincfg123::W`](W) writer structure"]
impl crate::Writable for Pincfg123Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PINCFG123 to value 0x03"]
impl crate::Resettable for Pincfg123Spec {
    const RESET_VALUE: u32 = 0x03;
}
