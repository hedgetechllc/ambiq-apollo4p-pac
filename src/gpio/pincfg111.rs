#[doc = "Register `PINCFG111` reader"]
pub type R = crate::R<Pincfg111Spec>;
#[doc = "Register `PINCFG111` writer"]
pub type W = crate::W<Pincfg111Spec>;
#[doc = "Function select for GPIO pin 111\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fncsel111 {
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
    Ct111 = 6,
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
impl From<Fncsel111> for u8 {
    #[inline(always)]
    fn from(variant: Fncsel111) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fncsel111 {
    type Ux = u8;
}
impl crate::IsEnum for Fncsel111 {}
#[doc = "Field `FNCSEL111` reader - Function select for GPIO pin 111"]
pub type Fncsel111R = crate::FieldReader<Fncsel111>;
impl Fncsel111R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fncsel111 {
        match self.bits {
            0 => Fncsel111::Reserved0,
            1 => Fncsel111::Reserved1,
            2 => Fncsel111::Reserved2,
            3 => Fncsel111::Gpio,
            4 => Fncsel111::Reserved4,
            5 => Fncsel111::Reserved5,
            6 => Fncsel111::Ct111,
            7 => Fncsel111::Reserved7,
            8 => Fncsel111::Obsbus15,
            9 => Fncsel111::Reserved9,
            10 => Fncsel111::Reserved10,
            11 => Fncsel111::Reserved11,
            12 => Fncsel111::Reserved12,
            13 => Fncsel111::Reserved13,
            14 => Fncsel111::Reserved14,
            15 => Fncsel111::Reserved15,
            _ => unreachable!(),
        }
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved0(&self) -> bool {
        *self == Fncsel111::Reserved0
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved1(&self) -> bool {
        *self == Fncsel111::Reserved1
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == Fncsel111::Reserved2
    }
    #[doc = "General purpose I/O"]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == Fncsel111::Gpio
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved4(&self) -> bool {
        *self == Fncsel111::Reserved4
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved5(&self) -> bool {
        *self == Fncsel111::Reserved5
    }
    #[doc = "Timer/Counter input or output; Selection of direction is done via CTIMER register settings."]
    #[inline(always)]
    pub fn is_ct111(&self) -> bool {
        *self == Fncsel111::Ct111
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved7(&self) -> bool {
        *self == Fncsel111::Reserved7
    }
    #[doc = "Observation bus bit 15"]
    #[inline(always)]
    pub fn is_obsbus15(&self) -> bool {
        *self == Fncsel111::Obsbus15
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved9(&self) -> bool {
        *self == Fncsel111::Reserved9
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved10(&self) -> bool {
        *self == Fncsel111::Reserved10
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved11(&self) -> bool {
        *self == Fncsel111::Reserved11
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved12(&self) -> bool {
        *self == Fncsel111::Reserved12
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved13(&self) -> bool {
        *self == Fncsel111::Reserved13
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved14(&self) -> bool {
        *self == Fncsel111::Reserved14
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved15(&self) -> bool {
        *self == Fncsel111::Reserved15
    }
}
#[doc = "Field `FNCSEL111` writer - Function select for GPIO pin 111"]
pub type Fncsel111W<'a, REG> = crate::FieldWriter<'a, REG, 4, Fncsel111, crate::Safe>;
impl<'a, REG> Fncsel111W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved0(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel111::Reserved0)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved1(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel111::Reserved1)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel111::Reserved2)
    }
    #[doc = "General purpose I/O"]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel111::Gpio)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved4(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel111::Reserved4)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved5(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel111::Reserved5)
    }
    #[doc = "Timer/Counter input or output; Selection of direction is done via CTIMER register settings."]
    #[inline(always)]
    pub fn ct111(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel111::Ct111)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved7(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel111::Reserved7)
    }
    #[doc = "Observation bus bit 15"]
    #[inline(always)]
    pub fn obsbus15(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel111::Obsbus15)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved9(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel111::Reserved9)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved10(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel111::Reserved10)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved11(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel111::Reserved11)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved12(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel111::Reserved12)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved13(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel111::Reserved13)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved14(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel111::Reserved14)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved15(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel111::Reserved15)
    }
}
#[doc = "Field `INPEN111` reader - Input enable for GPIO 111"]
pub type Inpen111R = crate::BitReader;
#[doc = "Field `INPEN111` writer - Input enable for GPIO 111"]
pub type Inpen111W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDZERO111` reader - Return 0 for read data on GPIO 111"]
pub type Rdzero111R = crate::BitReader;
#[doc = "Field `RDZERO111` writer - Return 0 for read data on GPIO 111"]
pub type Rdzero111W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Interrupt enable for GPIO 111\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Irpten111 {
    #[doc = "0: Interrupts are disabled for this GPIO"]
    Dis = 0,
    #[doc = "1: Interrupts are enabled for falling edge transition on this GPIO"]
    Intfall = 1,
    #[doc = "2: Interrupts are enabled for rising edge transitions on this GPIO"]
    Intrise = 2,
    #[doc = "3: Interrupts are enabled for any edge transition on this GPIO"]
    Intany = 3,
}
impl From<Irpten111> for u8 {
    #[inline(always)]
    fn from(variant: Irpten111) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Irpten111 {
    type Ux = u8;
}
impl crate::IsEnum for Irpten111 {}
#[doc = "Field `IRPTEN111` reader - Interrupt enable for GPIO 111"]
pub type Irpten111R = crate::FieldReader<Irpten111>;
impl Irpten111R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irpten111 {
        match self.bits {
            0 => Irpten111::Dis,
            1 => Irpten111::Intfall,
            2 => Irpten111::Intrise,
            3 => Irpten111::Intany,
            _ => unreachable!(),
        }
    }
    #[doc = "Interrupts are disabled for this GPIO"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Irpten111::Dis
    }
    #[doc = "Interrupts are enabled for falling edge transition on this GPIO"]
    #[inline(always)]
    pub fn is_intfall(&self) -> bool {
        *self == Irpten111::Intfall
    }
    #[doc = "Interrupts are enabled for rising edge transitions on this GPIO"]
    #[inline(always)]
    pub fn is_intrise(&self) -> bool {
        *self == Irpten111::Intrise
    }
    #[doc = "Interrupts are enabled for any edge transition on this GPIO"]
    #[inline(always)]
    pub fn is_intany(&self) -> bool {
        *self == Irpten111::Intany
    }
}
#[doc = "Field `IRPTEN111` writer - Interrupt enable for GPIO 111"]
pub type Irpten111W<'a, REG> = crate::FieldWriter<'a, REG, 2, Irpten111, crate::Safe>;
impl<'a, REG> Irpten111W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Interrupts are disabled for this GPIO"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten111::Dis)
    }
    #[doc = "Interrupts are enabled for falling edge transition on this GPIO"]
    #[inline(always)]
    pub fn intfall(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten111::Intfall)
    }
    #[doc = "Interrupts are enabled for rising edge transitions on this GPIO"]
    #[inline(always)]
    pub fn intrise(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten111::Intrise)
    }
    #[doc = "Interrupts are enabled for any edge transition on this GPIO"]
    #[inline(always)]
    pub fn intany(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten111::Intany)
    }
}
#[doc = "Pin IO mode selection for GPIO pin 111\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Outcfg111 {
    #[doc = "0: Output Disabled"]
    Dis = 0,
    #[doc = "1: Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    Pushpull = 1,
    #[doc = "2: Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    Od = 2,
    #[doc = "3: Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    Ts = 3,
}
impl From<Outcfg111> for u8 {
    #[inline(always)]
    fn from(variant: Outcfg111) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Outcfg111 {
    type Ux = u8;
}
impl crate::IsEnum for Outcfg111 {}
#[doc = "Field `OUTCFG111` reader - Pin IO mode selection for GPIO pin 111"]
pub type Outcfg111R = crate::FieldReader<Outcfg111>;
impl Outcfg111R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Outcfg111 {
        match self.bits {
            0 => Outcfg111::Dis,
            1 => Outcfg111::Pushpull,
            2 => Outcfg111::Od,
            3 => Outcfg111::Ts,
            _ => unreachable!(),
        }
    }
    #[doc = "Output Disabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Outcfg111::Dis
    }
    #[doc = "Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == Outcfg111::Pushpull
    }
    #[doc = "Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == Outcfg111::Od
    }
    #[doc = "Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == Outcfg111::Ts
    }
}
#[doc = "Field `OUTCFG111` writer - Pin IO mode selection for GPIO pin 111"]
pub type Outcfg111W<'a, REG> = crate::FieldWriter<'a, REG, 2, Outcfg111, crate::Safe>;
impl<'a, REG> Outcfg111W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Output Disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg111::Dis)
    }
    #[doc = "Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg111::Pushpull)
    }
    #[doc = "Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    #[inline(always)]
    pub fn od(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg111::Od)
    }
    #[doc = "Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    #[inline(always)]
    pub fn ts(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg111::Ts)
    }
}
impl R {
    #[doc = "Bits 0:3 - Function select for GPIO pin 111"]
    #[inline(always)]
    pub fn fncsel111(&self) -> Fncsel111R {
        Fncsel111R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Input enable for GPIO 111"]
    #[inline(always)]
    pub fn inpen111(&self) -> Inpen111R {
        Inpen111R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Return 0 for read data on GPIO 111"]
    #[inline(always)]
    pub fn rdzero111(&self) -> Rdzero111R {
        Rdzero111R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Interrupt enable for GPIO 111"]
    #[inline(always)]
    pub fn irpten111(&self) -> Irpten111R {
        Irpten111R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Pin IO mode selection for GPIO pin 111"]
    #[inline(always)]
    pub fn outcfg111(&self) -> Outcfg111R {
        Outcfg111R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Function select for GPIO pin 111"]
    #[inline(always)]
    #[must_use]
    pub fn fncsel111(&mut self) -> Fncsel111W<Pincfg111Spec> {
        Fncsel111W::new(self, 0)
    }
    #[doc = "Bit 4 - Input enable for GPIO 111"]
    #[inline(always)]
    #[must_use]
    pub fn inpen111(&mut self) -> Inpen111W<Pincfg111Spec> {
        Inpen111W::new(self, 4)
    }
    #[doc = "Bit 5 - Return 0 for read data on GPIO 111"]
    #[inline(always)]
    #[must_use]
    pub fn rdzero111(&mut self) -> Rdzero111W<Pincfg111Spec> {
        Rdzero111W::new(self, 5)
    }
    #[doc = "Bits 6:7 - Interrupt enable for GPIO 111"]
    #[inline(always)]
    #[must_use]
    pub fn irpten111(&mut self) -> Irpten111W<Pincfg111Spec> {
        Irpten111W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Pin IO mode selection for GPIO pin 111"]
    #[inline(always)]
    #[must_use]
    pub fn outcfg111(&mut self) -> Outcfg111W<Pincfg111Spec> {
        Outcfg111W::new(self, 8)
    }
}
#[doc = "Controls the operation of virtual GPIO pin 111.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg111::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg111::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pincfg111Spec;
impl crate::RegisterSpec for Pincfg111Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pincfg111::R`](R) reader structure"]
impl crate::Readable for Pincfg111Spec {}
#[doc = "`write(|w| ..)` method takes [`pincfg111::W`](W) writer structure"]
impl crate::Writable for Pincfg111Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PINCFG111 to value 0x03"]
impl crate::Resettable for Pincfg111Spec {
    const RESET_VALUE: u32 = 0x03;
}
