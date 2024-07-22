#[doc = "Register `PINCFG118` reader"]
pub type R = crate::R<Pincfg118Spec>;
#[doc = "Register `PINCFG118` writer"]
pub type W = crate::W<Pincfg118Spec>;
#[doc = "Function select for GPIO pin 118\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fncsel118 {
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
    Ct118 = 6,
    #[doc = "7: Reserved selection. Operation unknown if selected."]
    Reserved7 = 7,
    #[doc = "8: Observation bus bit 6"]
    Obsbus6 = 8,
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
impl From<Fncsel118> for u8 {
    #[inline(always)]
    fn from(variant: Fncsel118) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fncsel118 {
    type Ux = u8;
}
impl crate::IsEnum for Fncsel118 {}
#[doc = "Field `FNCSEL118` reader - Function select for GPIO pin 118"]
pub type Fncsel118R = crate::FieldReader<Fncsel118>;
impl Fncsel118R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fncsel118 {
        match self.bits {
            0 => Fncsel118::Reserved0,
            1 => Fncsel118::Reserved1,
            2 => Fncsel118::Reserved2,
            3 => Fncsel118::Gpio,
            4 => Fncsel118::Reserved4,
            5 => Fncsel118::Reserved5,
            6 => Fncsel118::Ct118,
            7 => Fncsel118::Reserved7,
            8 => Fncsel118::Obsbus6,
            9 => Fncsel118::Reserved9,
            10 => Fncsel118::Reserved10,
            11 => Fncsel118::Reserved11,
            12 => Fncsel118::Reserved12,
            13 => Fncsel118::Reserved13,
            14 => Fncsel118::Reserved14,
            15 => Fncsel118::Reserved15,
            _ => unreachable!(),
        }
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved0(&self) -> bool {
        *self == Fncsel118::Reserved0
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved1(&self) -> bool {
        *self == Fncsel118::Reserved1
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == Fncsel118::Reserved2
    }
    #[doc = "General purpose I/O"]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == Fncsel118::Gpio
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved4(&self) -> bool {
        *self == Fncsel118::Reserved4
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved5(&self) -> bool {
        *self == Fncsel118::Reserved5
    }
    #[doc = "Timer/Counter input or output; Selection of direction is done via CTIMER register settings."]
    #[inline(always)]
    pub fn is_ct118(&self) -> bool {
        *self == Fncsel118::Ct118
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved7(&self) -> bool {
        *self == Fncsel118::Reserved7
    }
    #[doc = "Observation bus bit 6"]
    #[inline(always)]
    pub fn is_obsbus6(&self) -> bool {
        *self == Fncsel118::Obsbus6
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved9(&self) -> bool {
        *self == Fncsel118::Reserved9
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved10(&self) -> bool {
        *self == Fncsel118::Reserved10
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved11(&self) -> bool {
        *self == Fncsel118::Reserved11
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved12(&self) -> bool {
        *self == Fncsel118::Reserved12
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved13(&self) -> bool {
        *self == Fncsel118::Reserved13
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved14(&self) -> bool {
        *self == Fncsel118::Reserved14
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved15(&self) -> bool {
        *self == Fncsel118::Reserved15
    }
}
#[doc = "Field `FNCSEL118` writer - Function select for GPIO pin 118"]
pub type Fncsel118W<'a, REG> = crate::FieldWriter<'a, REG, 4, Fncsel118, crate::Safe>;
impl<'a, REG> Fncsel118W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved0(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel118::Reserved0)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved1(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel118::Reserved1)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel118::Reserved2)
    }
    #[doc = "General purpose I/O"]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel118::Gpio)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved4(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel118::Reserved4)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved5(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel118::Reserved5)
    }
    #[doc = "Timer/Counter input or output; Selection of direction is done via CTIMER register settings."]
    #[inline(always)]
    pub fn ct118(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel118::Ct118)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved7(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel118::Reserved7)
    }
    #[doc = "Observation bus bit 6"]
    #[inline(always)]
    pub fn obsbus6(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel118::Obsbus6)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved9(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel118::Reserved9)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved10(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel118::Reserved10)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved11(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel118::Reserved11)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved12(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel118::Reserved12)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved13(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel118::Reserved13)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved14(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel118::Reserved14)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved15(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel118::Reserved15)
    }
}
#[doc = "Field `INPEN118` reader - Input enable for GPIO 118"]
pub type Inpen118R = crate::BitReader;
#[doc = "Field `INPEN118` writer - Input enable for GPIO 118"]
pub type Inpen118W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDZERO118` reader - Return 0 for read data on GPIO 118"]
pub type Rdzero118R = crate::BitReader;
#[doc = "Field `RDZERO118` writer - Return 0 for read data on GPIO 118"]
pub type Rdzero118W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Interrupt enable for GPIO 118\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Irpten118 {
    #[doc = "0: Interrupts are disabled for this GPIO"]
    Dis = 0,
    #[doc = "1: Interrupts are enabled for falling edge transition on this GPIO"]
    Intfall = 1,
    #[doc = "2: Interrupts are enabled for rising edge transitions on this GPIO"]
    Intrise = 2,
    #[doc = "3: Interrupts are enabled for any edge transition on this GPIO"]
    Intany = 3,
}
impl From<Irpten118> for u8 {
    #[inline(always)]
    fn from(variant: Irpten118) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Irpten118 {
    type Ux = u8;
}
impl crate::IsEnum for Irpten118 {}
#[doc = "Field `IRPTEN118` reader - Interrupt enable for GPIO 118"]
pub type Irpten118R = crate::FieldReader<Irpten118>;
impl Irpten118R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irpten118 {
        match self.bits {
            0 => Irpten118::Dis,
            1 => Irpten118::Intfall,
            2 => Irpten118::Intrise,
            3 => Irpten118::Intany,
            _ => unreachable!(),
        }
    }
    #[doc = "Interrupts are disabled for this GPIO"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Irpten118::Dis
    }
    #[doc = "Interrupts are enabled for falling edge transition on this GPIO"]
    #[inline(always)]
    pub fn is_intfall(&self) -> bool {
        *self == Irpten118::Intfall
    }
    #[doc = "Interrupts are enabled for rising edge transitions on this GPIO"]
    #[inline(always)]
    pub fn is_intrise(&self) -> bool {
        *self == Irpten118::Intrise
    }
    #[doc = "Interrupts are enabled for any edge transition on this GPIO"]
    #[inline(always)]
    pub fn is_intany(&self) -> bool {
        *self == Irpten118::Intany
    }
}
#[doc = "Field `IRPTEN118` writer - Interrupt enable for GPIO 118"]
pub type Irpten118W<'a, REG> = crate::FieldWriter<'a, REG, 2, Irpten118, crate::Safe>;
impl<'a, REG> Irpten118W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Interrupts are disabled for this GPIO"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten118::Dis)
    }
    #[doc = "Interrupts are enabled for falling edge transition on this GPIO"]
    #[inline(always)]
    pub fn intfall(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten118::Intfall)
    }
    #[doc = "Interrupts are enabled for rising edge transitions on this GPIO"]
    #[inline(always)]
    pub fn intrise(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten118::Intrise)
    }
    #[doc = "Interrupts are enabled for any edge transition on this GPIO"]
    #[inline(always)]
    pub fn intany(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten118::Intany)
    }
}
#[doc = "Pin IO mode selection for GPIO pin 118\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Outcfg118 {
    #[doc = "0: Output Disabled"]
    Dis = 0,
    #[doc = "1: Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    Pushpull = 1,
    #[doc = "2: Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    Od = 2,
    #[doc = "3: Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    Ts = 3,
}
impl From<Outcfg118> for u8 {
    #[inline(always)]
    fn from(variant: Outcfg118) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Outcfg118 {
    type Ux = u8;
}
impl crate::IsEnum for Outcfg118 {}
#[doc = "Field `OUTCFG118` reader - Pin IO mode selection for GPIO pin 118"]
pub type Outcfg118R = crate::FieldReader<Outcfg118>;
impl Outcfg118R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Outcfg118 {
        match self.bits {
            0 => Outcfg118::Dis,
            1 => Outcfg118::Pushpull,
            2 => Outcfg118::Od,
            3 => Outcfg118::Ts,
            _ => unreachable!(),
        }
    }
    #[doc = "Output Disabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Outcfg118::Dis
    }
    #[doc = "Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == Outcfg118::Pushpull
    }
    #[doc = "Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == Outcfg118::Od
    }
    #[doc = "Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == Outcfg118::Ts
    }
}
#[doc = "Field `OUTCFG118` writer - Pin IO mode selection for GPIO pin 118"]
pub type Outcfg118W<'a, REG> = crate::FieldWriter<'a, REG, 2, Outcfg118, crate::Safe>;
impl<'a, REG> Outcfg118W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Output Disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg118::Dis)
    }
    #[doc = "Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg118::Pushpull)
    }
    #[doc = "Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    #[inline(always)]
    pub fn od(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg118::Od)
    }
    #[doc = "Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    #[inline(always)]
    pub fn ts(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg118::Ts)
    }
}
impl R {
    #[doc = "Bits 0:3 - Function select for GPIO pin 118"]
    #[inline(always)]
    pub fn fncsel118(&self) -> Fncsel118R {
        Fncsel118R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Input enable for GPIO 118"]
    #[inline(always)]
    pub fn inpen118(&self) -> Inpen118R {
        Inpen118R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Return 0 for read data on GPIO 118"]
    #[inline(always)]
    pub fn rdzero118(&self) -> Rdzero118R {
        Rdzero118R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Interrupt enable for GPIO 118"]
    #[inline(always)]
    pub fn irpten118(&self) -> Irpten118R {
        Irpten118R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Pin IO mode selection for GPIO pin 118"]
    #[inline(always)]
    pub fn outcfg118(&self) -> Outcfg118R {
        Outcfg118R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Function select for GPIO pin 118"]
    #[inline(always)]
    #[must_use]
    pub fn fncsel118(&mut self) -> Fncsel118W<Pincfg118Spec> {
        Fncsel118W::new(self, 0)
    }
    #[doc = "Bit 4 - Input enable for GPIO 118"]
    #[inline(always)]
    #[must_use]
    pub fn inpen118(&mut self) -> Inpen118W<Pincfg118Spec> {
        Inpen118W::new(self, 4)
    }
    #[doc = "Bit 5 - Return 0 for read data on GPIO 118"]
    #[inline(always)]
    #[must_use]
    pub fn rdzero118(&mut self) -> Rdzero118W<Pincfg118Spec> {
        Rdzero118W::new(self, 5)
    }
    #[doc = "Bits 6:7 - Interrupt enable for GPIO 118"]
    #[inline(always)]
    #[must_use]
    pub fn irpten118(&mut self) -> Irpten118W<Pincfg118Spec> {
        Irpten118W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Pin IO mode selection for GPIO pin 118"]
    #[inline(always)]
    #[must_use]
    pub fn outcfg118(&mut self) -> Outcfg118W<Pincfg118Spec> {
        Outcfg118W::new(self, 8)
    }
}
#[doc = "Controls the operation of virtual GPIO pin 118.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg118::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg118::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pincfg118Spec;
impl crate::RegisterSpec for Pincfg118Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pincfg118::R`](R) reader structure"]
impl crate::Readable for Pincfg118Spec {}
#[doc = "`write(|w| ..)` method takes [`pincfg118::W`](W) writer structure"]
impl crate::Writable for Pincfg118Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PINCFG118 to value 0x03"]
impl crate::Resettable for Pincfg118Spec {
    const RESET_VALUE: u32 = 0x03;
}
