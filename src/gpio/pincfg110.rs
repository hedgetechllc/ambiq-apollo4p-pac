#[doc = "Register `PINCFG110` reader"]
pub type R = crate::R<Pincfg110Spec>;
#[doc = "Register `PINCFG110` writer"]
pub type W = crate::W<Pincfg110Spec>;
#[doc = "Function select for GPIO pin 110\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fncsel110 {
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
    Ct110 = 6,
    #[doc = "7: Reserved selection. Operation unknown if selected."]
    Reserved7 = 7,
    #[doc = "8: Observation bus bit 14"]
    Obsbus14 = 8,
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
impl From<Fncsel110> for u8 {
    #[inline(always)]
    fn from(variant: Fncsel110) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fncsel110 {
    type Ux = u8;
}
impl crate::IsEnum for Fncsel110 {}
#[doc = "Field `FNCSEL110` reader - Function select for GPIO pin 110"]
pub type Fncsel110R = crate::FieldReader<Fncsel110>;
impl Fncsel110R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fncsel110 {
        match self.bits {
            0 => Fncsel110::Reserved0,
            1 => Fncsel110::Reserved1,
            2 => Fncsel110::Reserved2,
            3 => Fncsel110::Gpio,
            4 => Fncsel110::Reserved4,
            5 => Fncsel110::Reserved5,
            6 => Fncsel110::Ct110,
            7 => Fncsel110::Reserved7,
            8 => Fncsel110::Obsbus14,
            9 => Fncsel110::Reserved9,
            10 => Fncsel110::Reserved10,
            11 => Fncsel110::Reserved11,
            12 => Fncsel110::Reserved12,
            13 => Fncsel110::Reserved13,
            14 => Fncsel110::Reserved14,
            15 => Fncsel110::Reserved15,
            _ => unreachable!(),
        }
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved0(&self) -> bool {
        *self == Fncsel110::Reserved0
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved1(&self) -> bool {
        *self == Fncsel110::Reserved1
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == Fncsel110::Reserved2
    }
    #[doc = "General purpose I/O"]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == Fncsel110::Gpio
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved4(&self) -> bool {
        *self == Fncsel110::Reserved4
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved5(&self) -> bool {
        *self == Fncsel110::Reserved5
    }
    #[doc = "Timer/Counter input or output; Selection of direction is done via CTIMER register settings."]
    #[inline(always)]
    pub fn is_ct110(&self) -> bool {
        *self == Fncsel110::Ct110
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved7(&self) -> bool {
        *self == Fncsel110::Reserved7
    }
    #[doc = "Observation bus bit 14"]
    #[inline(always)]
    pub fn is_obsbus14(&self) -> bool {
        *self == Fncsel110::Obsbus14
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved9(&self) -> bool {
        *self == Fncsel110::Reserved9
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved10(&self) -> bool {
        *self == Fncsel110::Reserved10
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved11(&self) -> bool {
        *self == Fncsel110::Reserved11
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved12(&self) -> bool {
        *self == Fncsel110::Reserved12
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved13(&self) -> bool {
        *self == Fncsel110::Reserved13
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved14(&self) -> bool {
        *self == Fncsel110::Reserved14
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved15(&self) -> bool {
        *self == Fncsel110::Reserved15
    }
}
#[doc = "Field `FNCSEL110` writer - Function select for GPIO pin 110"]
pub type Fncsel110W<'a, REG> = crate::FieldWriter<'a, REG, 4, Fncsel110, crate::Safe>;
impl<'a, REG> Fncsel110W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved0(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel110::Reserved0)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved1(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel110::Reserved1)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel110::Reserved2)
    }
    #[doc = "General purpose I/O"]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel110::Gpio)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved4(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel110::Reserved4)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved5(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel110::Reserved5)
    }
    #[doc = "Timer/Counter input or output; Selection of direction is done via CTIMER register settings."]
    #[inline(always)]
    pub fn ct110(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel110::Ct110)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved7(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel110::Reserved7)
    }
    #[doc = "Observation bus bit 14"]
    #[inline(always)]
    pub fn obsbus14(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel110::Obsbus14)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved9(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel110::Reserved9)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved10(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel110::Reserved10)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved11(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel110::Reserved11)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved12(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel110::Reserved12)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved13(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel110::Reserved13)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved14(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel110::Reserved14)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved15(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel110::Reserved15)
    }
}
#[doc = "Field `INPEN110` reader - Input enable for GPIO 110"]
pub type Inpen110R = crate::BitReader;
#[doc = "Field `INPEN110` writer - Input enable for GPIO 110"]
pub type Inpen110W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDZERO110` reader - Return 0 for read data on GPIO 110"]
pub type Rdzero110R = crate::BitReader;
#[doc = "Field `RDZERO110` writer - Return 0 for read data on GPIO 110"]
pub type Rdzero110W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Interrupt enable for GPIO 110\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Irpten110 {
    #[doc = "0: Interrupts are disabled for this GPIO"]
    Dis = 0,
    #[doc = "1: Interrupts are enabled for falling edge transition on this GPIO"]
    Intfall = 1,
    #[doc = "2: Interrupts are enabled for rising edge transitions on this GPIO"]
    Intrise = 2,
    #[doc = "3: Interrupts are enabled for any edge transition on this GPIO"]
    Intany = 3,
}
impl From<Irpten110> for u8 {
    #[inline(always)]
    fn from(variant: Irpten110) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Irpten110 {
    type Ux = u8;
}
impl crate::IsEnum for Irpten110 {}
#[doc = "Field `IRPTEN110` reader - Interrupt enable for GPIO 110"]
pub type Irpten110R = crate::FieldReader<Irpten110>;
impl Irpten110R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irpten110 {
        match self.bits {
            0 => Irpten110::Dis,
            1 => Irpten110::Intfall,
            2 => Irpten110::Intrise,
            3 => Irpten110::Intany,
            _ => unreachable!(),
        }
    }
    #[doc = "Interrupts are disabled for this GPIO"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Irpten110::Dis
    }
    #[doc = "Interrupts are enabled for falling edge transition on this GPIO"]
    #[inline(always)]
    pub fn is_intfall(&self) -> bool {
        *self == Irpten110::Intfall
    }
    #[doc = "Interrupts are enabled for rising edge transitions on this GPIO"]
    #[inline(always)]
    pub fn is_intrise(&self) -> bool {
        *self == Irpten110::Intrise
    }
    #[doc = "Interrupts are enabled for any edge transition on this GPIO"]
    #[inline(always)]
    pub fn is_intany(&self) -> bool {
        *self == Irpten110::Intany
    }
}
#[doc = "Field `IRPTEN110` writer - Interrupt enable for GPIO 110"]
pub type Irpten110W<'a, REG> = crate::FieldWriter<'a, REG, 2, Irpten110, crate::Safe>;
impl<'a, REG> Irpten110W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Interrupts are disabled for this GPIO"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten110::Dis)
    }
    #[doc = "Interrupts are enabled for falling edge transition on this GPIO"]
    #[inline(always)]
    pub fn intfall(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten110::Intfall)
    }
    #[doc = "Interrupts are enabled for rising edge transitions on this GPIO"]
    #[inline(always)]
    pub fn intrise(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten110::Intrise)
    }
    #[doc = "Interrupts are enabled for any edge transition on this GPIO"]
    #[inline(always)]
    pub fn intany(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten110::Intany)
    }
}
#[doc = "Pin IO mode selection for GPIO pin 110\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Outcfg110 {
    #[doc = "0: Output Disabled"]
    Dis = 0,
    #[doc = "1: Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    Pushpull = 1,
    #[doc = "2: Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    Od = 2,
    #[doc = "3: Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    Ts = 3,
}
impl From<Outcfg110> for u8 {
    #[inline(always)]
    fn from(variant: Outcfg110) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Outcfg110 {
    type Ux = u8;
}
impl crate::IsEnum for Outcfg110 {}
#[doc = "Field `OUTCFG110` reader - Pin IO mode selection for GPIO pin 110"]
pub type Outcfg110R = crate::FieldReader<Outcfg110>;
impl Outcfg110R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Outcfg110 {
        match self.bits {
            0 => Outcfg110::Dis,
            1 => Outcfg110::Pushpull,
            2 => Outcfg110::Od,
            3 => Outcfg110::Ts,
            _ => unreachable!(),
        }
    }
    #[doc = "Output Disabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Outcfg110::Dis
    }
    #[doc = "Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == Outcfg110::Pushpull
    }
    #[doc = "Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == Outcfg110::Od
    }
    #[doc = "Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == Outcfg110::Ts
    }
}
#[doc = "Field `OUTCFG110` writer - Pin IO mode selection for GPIO pin 110"]
pub type Outcfg110W<'a, REG> = crate::FieldWriter<'a, REG, 2, Outcfg110, crate::Safe>;
impl<'a, REG> Outcfg110W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Output Disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg110::Dis)
    }
    #[doc = "Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg110::Pushpull)
    }
    #[doc = "Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    #[inline(always)]
    pub fn od(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg110::Od)
    }
    #[doc = "Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    #[inline(always)]
    pub fn ts(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg110::Ts)
    }
}
impl R {
    #[doc = "Bits 0:3 - Function select for GPIO pin 110"]
    #[inline(always)]
    pub fn fncsel110(&self) -> Fncsel110R {
        Fncsel110R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Input enable for GPIO 110"]
    #[inline(always)]
    pub fn inpen110(&self) -> Inpen110R {
        Inpen110R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Return 0 for read data on GPIO 110"]
    #[inline(always)]
    pub fn rdzero110(&self) -> Rdzero110R {
        Rdzero110R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Interrupt enable for GPIO 110"]
    #[inline(always)]
    pub fn irpten110(&self) -> Irpten110R {
        Irpten110R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Pin IO mode selection for GPIO pin 110"]
    #[inline(always)]
    pub fn outcfg110(&self) -> Outcfg110R {
        Outcfg110R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Function select for GPIO pin 110"]
    #[inline(always)]
    #[must_use]
    pub fn fncsel110(&mut self) -> Fncsel110W<Pincfg110Spec> {
        Fncsel110W::new(self, 0)
    }
    #[doc = "Bit 4 - Input enable for GPIO 110"]
    #[inline(always)]
    #[must_use]
    pub fn inpen110(&mut self) -> Inpen110W<Pincfg110Spec> {
        Inpen110W::new(self, 4)
    }
    #[doc = "Bit 5 - Return 0 for read data on GPIO 110"]
    #[inline(always)]
    #[must_use]
    pub fn rdzero110(&mut self) -> Rdzero110W<Pincfg110Spec> {
        Rdzero110W::new(self, 5)
    }
    #[doc = "Bits 6:7 - Interrupt enable for GPIO 110"]
    #[inline(always)]
    #[must_use]
    pub fn irpten110(&mut self) -> Irpten110W<Pincfg110Spec> {
        Irpten110W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Pin IO mode selection for GPIO pin 110"]
    #[inline(always)]
    #[must_use]
    pub fn outcfg110(&mut self) -> Outcfg110W<Pincfg110Spec> {
        Outcfg110W::new(self, 8)
    }
}
#[doc = "Controls the operation of virtual GPIO pin 110.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg110::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg110::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pincfg110Spec;
impl crate::RegisterSpec for Pincfg110Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pincfg110::R`](R) reader structure"]
impl crate::Readable for Pincfg110Spec {}
#[doc = "`write(|w| ..)` method takes [`pincfg110::W`](W) writer structure"]
impl crate::Writable for Pincfg110Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PINCFG110 to value 0x03"]
impl crate::Resettable for Pincfg110Spec {
    const RESET_VALUE: u32 = 0x03;
}
