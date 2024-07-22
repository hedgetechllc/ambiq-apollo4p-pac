#[doc = "Register `PINCFG119` reader"]
pub type R = crate::R<Pincfg119Spec>;
#[doc = "Register `PINCFG119` writer"]
pub type W = crate::W<Pincfg119Spec>;
#[doc = "Function select for GPIO pin 119\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fncsel119 {
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
    Ct119 = 6,
    #[doc = "7: Reserved selection. Operation unknown if selected."]
    Reserved7 = 7,
    #[doc = "8: Observation bus bit 7"]
    Obsbus7 = 8,
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
impl From<Fncsel119> for u8 {
    #[inline(always)]
    fn from(variant: Fncsel119) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fncsel119 {
    type Ux = u8;
}
impl crate::IsEnum for Fncsel119 {}
#[doc = "Field `FNCSEL119` reader - Function select for GPIO pin 119"]
pub type Fncsel119R = crate::FieldReader<Fncsel119>;
impl Fncsel119R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fncsel119 {
        match self.bits {
            0 => Fncsel119::Reserved0,
            1 => Fncsel119::Reserved1,
            2 => Fncsel119::Reserved2,
            3 => Fncsel119::Gpio,
            4 => Fncsel119::Reserved4,
            5 => Fncsel119::Reserved5,
            6 => Fncsel119::Ct119,
            7 => Fncsel119::Reserved7,
            8 => Fncsel119::Obsbus7,
            9 => Fncsel119::Reserved9,
            10 => Fncsel119::Reserved10,
            11 => Fncsel119::Reserved11,
            12 => Fncsel119::Reserved12,
            13 => Fncsel119::Reserved13,
            14 => Fncsel119::Reserved14,
            15 => Fncsel119::Reserved15,
            _ => unreachable!(),
        }
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved0(&self) -> bool {
        *self == Fncsel119::Reserved0
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved1(&self) -> bool {
        *self == Fncsel119::Reserved1
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == Fncsel119::Reserved2
    }
    #[doc = "General purpose I/O"]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == Fncsel119::Gpio
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved4(&self) -> bool {
        *self == Fncsel119::Reserved4
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved5(&self) -> bool {
        *self == Fncsel119::Reserved5
    }
    #[doc = "Timer/Counter input or output; Selection of direction is done via CTIMER register settings."]
    #[inline(always)]
    pub fn is_ct119(&self) -> bool {
        *self == Fncsel119::Ct119
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved7(&self) -> bool {
        *self == Fncsel119::Reserved7
    }
    #[doc = "Observation bus bit 7"]
    #[inline(always)]
    pub fn is_obsbus7(&self) -> bool {
        *self == Fncsel119::Obsbus7
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved9(&self) -> bool {
        *self == Fncsel119::Reserved9
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved10(&self) -> bool {
        *self == Fncsel119::Reserved10
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved11(&self) -> bool {
        *self == Fncsel119::Reserved11
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved12(&self) -> bool {
        *self == Fncsel119::Reserved12
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved13(&self) -> bool {
        *self == Fncsel119::Reserved13
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved14(&self) -> bool {
        *self == Fncsel119::Reserved14
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved15(&self) -> bool {
        *self == Fncsel119::Reserved15
    }
}
#[doc = "Field `FNCSEL119` writer - Function select for GPIO pin 119"]
pub type Fncsel119W<'a, REG> = crate::FieldWriter<'a, REG, 4, Fncsel119, crate::Safe>;
impl<'a, REG> Fncsel119W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved0(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel119::Reserved0)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved1(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel119::Reserved1)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel119::Reserved2)
    }
    #[doc = "General purpose I/O"]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel119::Gpio)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved4(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel119::Reserved4)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved5(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel119::Reserved5)
    }
    #[doc = "Timer/Counter input or output; Selection of direction is done via CTIMER register settings."]
    #[inline(always)]
    pub fn ct119(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel119::Ct119)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved7(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel119::Reserved7)
    }
    #[doc = "Observation bus bit 7"]
    #[inline(always)]
    pub fn obsbus7(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel119::Obsbus7)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved9(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel119::Reserved9)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved10(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel119::Reserved10)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved11(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel119::Reserved11)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved12(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel119::Reserved12)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved13(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel119::Reserved13)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved14(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel119::Reserved14)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved15(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel119::Reserved15)
    }
}
#[doc = "Field `INPEN119` reader - Input enable for GPIO 119"]
pub type Inpen119R = crate::BitReader;
#[doc = "Field `INPEN119` writer - Input enable for GPIO 119"]
pub type Inpen119W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDZERO119` reader - Return 0 for read data on GPIO 119"]
pub type Rdzero119R = crate::BitReader;
#[doc = "Field `RDZERO119` writer - Return 0 for read data on GPIO 119"]
pub type Rdzero119W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Interrupt enable for GPIO 119\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Irpten119 {
    #[doc = "0: Interrupts are disabled for this GPIO"]
    Dis = 0,
    #[doc = "1: Interrupts are enabled for falling edge transition on this GPIO"]
    Intfall = 1,
    #[doc = "2: Interrupts are enabled for rising edge transitions on this GPIO"]
    Intrise = 2,
    #[doc = "3: Interrupts are enabled for any edge transition on this GPIO"]
    Intany = 3,
}
impl From<Irpten119> for u8 {
    #[inline(always)]
    fn from(variant: Irpten119) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Irpten119 {
    type Ux = u8;
}
impl crate::IsEnum for Irpten119 {}
#[doc = "Field `IRPTEN119` reader - Interrupt enable for GPIO 119"]
pub type Irpten119R = crate::FieldReader<Irpten119>;
impl Irpten119R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irpten119 {
        match self.bits {
            0 => Irpten119::Dis,
            1 => Irpten119::Intfall,
            2 => Irpten119::Intrise,
            3 => Irpten119::Intany,
            _ => unreachable!(),
        }
    }
    #[doc = "Interrupts are disabled for this GPIO"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Irpten119::Dis
    }
    #[doc = "Interrupts are enabled for falling edge transition on this GPIO"]
    #[inline(always)]
    pub fn is_intfall(&self) -> bool {
        *self == Irpten119::Intfall
    }
    #[doc = "Interrupts are enabled for rising edge transitions on this GPIO"]
    #[inline(always)]
    pub fn is_intrise(&self) -> bool {
        *self == Irpten119::Intrise
    }
    #[doc = "Interrupts are enabled for any edge transition on this GPIO"]
    #[inline(always)]
    pub fn is_intany(&self) -> bool {
        *self == Irpten119::Intany
    }
}
#[doc = "Field `IRPTEN119` writer - Interrupt enable for GPIO 119"]
pub type Irpten119W<'a, REG> = crate::FieldWriter<'a, REG, 2, Irpten119, crate::Safe>;
impl<'a, REG> Irpten119W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Interrupts are disabled for this GPIO"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten119::Dis)
    }
    #[doc = "Interrupts are enabled for falling edge transition on this GPIO"]
    #[inline(always)]
    pub fn intfall(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten119::Intfall)
    }
    #[doc = "Interrupts are enabled for rising edge transitions on this GPIO"]
    #[inline(always)]
    pub fn intrise(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten119::Intrise)
    }
    #[doc = "Interrupts are enabled for any edge transition on this GPIO"]
    #[inline(always)]
    pub fn intany(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten119::Intany)
    }
}
#[doc = "Pin IO mode selection for GPIO pin 119\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Outcfg119 {
    #[doc = "0: Output Disabled"]
    Dis = 0,
    #[doc = "1: Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    Pushpull = 1,
    #[doc = "2: Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    Od = 2,
    #[doc = "3: Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    Ts = 3,
}
impl From<Outcfg119> for u8 {
    #[inline(always)]
    fn from(variant: Outcfg119) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Outcfg119 {
    type Ux = u8;
}
impl crate::IsEnum for Outcfg119 {}
#[doc = "Field `OUTCFG119` reader - Pin IO mode selection for GPIO pin 119"]
pub type Outcfg119R = crate::FieldReader<Outcfg119>;
impl Outcfg119R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Outcfg119 {
        match self.bits {
            0 => Outcfg119::Dis,
            1 => Outcfg119::Pushpull,
            2 => Outcfg119::Od,
            3 => Outcfg119::Ts,
            _ => unreachable!(),
        }
    }
    #[doc = "Output Disabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Outcfg119::Dis
    }
    #[doc = "Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == Outcfg119::Pushpull
    }
    #[doc = "Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == Outcfg119::Od
    }
    #[doc = "Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == Outcfg119::Ts
    }
}
#[doc = "Field `OUTCFG119` writer - Pin IO mode selection for GPIO pin 119"]
pub type Outcfg119W<'a, REG> = crate::FieldWriter<'a, REG, 2, Outcfg119, crate::Safe>;
impl<'a, REG> Outcfg119W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Output Disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg119::Dis)
    }
    #[doc = "Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg119::Pushpull)
    }
    #[doc = "Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    #[inline(always)]
    pub fn od(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg119::Od)
    }
    #[doc = "Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    #[inline(always)]
    pub fn ts(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg119::Ts)
    }
}
impl R {
    #[doc = "Bits 0:3 - Function select for GPIO pin 119"]
    #[inline(always)]
    pub fn fncsel119(&self) -> Fncsel119R {
        Fncsel119R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Input enable for GPIO 119"]
    #[inline(always)]
    pub fn inpen119(&self) -> Inpen119R {
        Inpen119R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Return 0 for read data on GPIO 119"]
    #[inline(always)]
    pub fn rdzero119(&self) -> Rdzero119R {
        Rdzero119R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Interrupt enable for GPIO 119"]
    #[inline(always)]
    pub fn irpten119(&self) -> Irpten119R {
        Irpten119R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Pin IO mode selection for GPIO pin 119"]
    #[inline(always)]
    pub fn outcfg119(&self) -> Outcfg119R {
        Outcfg119R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Function select for GPIO pin 119"]
    #[inline(always)]
    #[must_use]
    pub fn fncsel119(&mut self) -> Fncsel119W<Pincfg119Spec> {
        Fncsel119W::new(self, 0)
    }
    #[doc = "Bit 4 - Input enable for GPIO 119"]
    #[inline(always)]
    #[must_use]
    pub fn inpen119(&mut self) -> Inpen119W<Pincfg119Spec> {
        Inpen119W::new(self, 4)
    }
    #[doc = "Bit 5 - Return 0 for read data on GPIO 119"]
    #[inline(always)]
    #[must_use]
    pub fn rdzero119(&mut self) -> Rdzero119W<Pincfg119Spec> {
        Rdzero119W::new(self, 5)
    }
    #[doc = "Bits 6:7 - Interrupt enable for GPIO 119"]
    #[inline(always)]
    #[must_use]
    pub fn irpten119(&mut self) -> Irpten119W<Pincfg119Spec> {
        Irpten119W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Pin IO mode selection for GPIO pin 119"]
    #[inline(always)]
    #[must_use]
    pub fn outcfg119(&mut self) -> Outcfg119W<Pincfg119Spec> {
        Outcfg119W::new(self, 8)
    }
}
#[doc = "Controls the operation of virtual GPIO pin 119.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg119::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg119::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pincfg119Spec;
impl crate::RegisterSpec for Pincfg119Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pincfg119::R`](R) reader structure"]
impl crate::Readable for Pincfg119Spec {}
#[doc = "`write(|w| ..)` method takes [`pincfg119::W`](W) writer structure"]
impl crate::Writable for Pincfg119Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PINCFG119 to value 0x03"]
impl crate::Resettable for Pincfg119Spec {
    const RESET_VALUE: u32 = 0x03;
}
