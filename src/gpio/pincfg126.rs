#[doc = "Register `PINCFG126` reader"]
pub type R = crate::R<Pincfg126Spec>;
#[doc = "Register `PINCFG126` writer"]
pub type W = crate::W<Pincfg126Spec>;
#[doc = "Function select for GPIO pin 126\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fncsel126 {
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
    Ct126 = 6,
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
impl From<Fncsel126> for u8 {
    #[inline(always)]
    fn from(variant: Fncsel126) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fncsel126 {
    type Ux = u8;
}
impl crate::IsEnum for Fncsel126 {}
#[doc = "Field `FNCSEL126` reader - Function select for GPIO pin 126"]
pub type Fncsel126R = crate::FieldReader<Fncsel126>;
impl Fncsel126R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fncsel126 {
        match self.bits {
            0 => Fncsel126::Reserved0,
            1 => Fncsel126::Reserved1,
            2 => Fncsel126::Reserved2,
            3 => Fncsel126::Gpio,
            4 => Fncsel126::Reserved4,
            5 => Fncsel126::Reserved5,
            6 => Fncsel126::Ct126,
            7 => Fncsel126::Reserved7,
            8 => Fncsel126::Obsbus14,
            9 => Fncsel126::Reserved9,
            10 => Fncsel126::Reserved10,
            11 => Fncsel126::Reserved11,
            12 => Fncsel126::Reserved12,
            13 => Fncsel126::Reserved13,
            14 => Fncsel126::Reserved14,
            15 => Fncsel126::Reserved15,
            _ => unreachable!(),
        }
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved0(&self) -> bool {
        *self == Fncsel126::Reserved0
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved1(&self) -> bool {
        *self == Fncsel126::Reserved1
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == Fncsel126::Reserved2
    }
    #[doc = "General purpose I/O"]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == Fncsel126::Gpio
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved4(&self) -> bool {
        *self == Fncsel126::Reserved4
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved5(&self) -> bool {
        *self == Fncsel126::Reserved5
    }
    #[doc = "Timer/Counter input or output; Selection of direction is done via CTIMER register settings."]
    #[inline(always)]
    pub fn is_ct126(&self) -> bool {
        *self == Fncsel126::Ct126
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved7(&self) -> bool {
        *self == Fncsel126::Reserved7
    }
    #[doc = "Observation bus bit 14"]
    #[inline(always)]
    pub fn is_obsbus14(&self) -> bool {
        *self == Fncsel126::Obsbus14
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved9(&self) -> bool {
        *self == Fncsel126::Reserved9
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved10(&self) -> bool {
        *self == Fncsel126::Reserved10
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved11(&self) -> bool {
        *self == Fncsel126::Reserved11
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved12(&self) -> bool {
        *self == Fncsel126::Reserved12
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved13(&self) -> bool {
        *self == Fncsel126::Reserved13
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved14(&self) -> bool {
        *self == Fncsel126::Reserved14
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved15(&self) -> bool {
        *self == Fncsel126::Reserved15
    }
}
#[doc = "Field `FNCSEL126` writer - Function select for GPIO pin 126"]
pub type Fncsel126W<'a, REG> = crate::FieldWriter<'a, REG, 4, Fncsel126, crate::Safe>;
impl<'a, REG> Fncsel126W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved0(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel126::Reserved0)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved1(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel126::Reserved1)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel126::Reserved2)
    }
    #[doc = "General purpose I/O"]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel126::Gpio)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved4(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel126::Reserved4)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved5(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel126::Reserved5)
    }
    #[doc = "Timer/Counter input or output; Selection of direction is done via CTIMER register settings."]
    #[inline(always)]
    pub fn ct126(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel126::Ct126)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved7(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel126::Reserved7)
    }
    #[doc = "Observation bus bit 14"]
    #[inline(always)]
    pub fn obsbus14(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel126::Obsbus14)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved9(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel126::Reserved9)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved10(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel126::Reserved10)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved11(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel126::Reserved11)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved12(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel126::Reserved12)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved13(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel126::Reserved13)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved14(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel126::Reserved14)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved15(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel126::Reserved15)
    }
}
#[doc = "Field `INPEN126` reader - Input enable for GPIO 126"]
pub type Inpen126R = crate::BitReader;
#[doc = "Field `INPEN126` writer - Input enable for GPIO 126"]
pub type Inpen126W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDZERO126` reader - Return 0 for read data on GPIO 126"]
pub type Rdzero126R = crate::BitReader;
#[doc = "Field `RDZERO126` writer - Return 0 for read data on GPIO 126"]
pub type Rdzero126W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Interrupt enable for GPIO 126\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Irpten126 {
    #[doc = "0: Interrupts are disabled for this GPIO"]
    Dis = 0,
    #[doc = "1: Interrupts are enabled for falling edge transition on this GPIO"]
    Intfall = 1,
    #[doc = "2: Interrupts are enabled for rising edge transitions on this GPIO"]
    Intrise = 2,
    #[doc = "3: Interrupts are enabled for any edge transition on this GPIO"]
    Intany = 3,
}
impl From<Irpten126> for u8 {
    #[inline(always)]
    fn from(variant: Irpten126) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Irpten126 {
    type Ux = u8;
}
impl crate::IsEnum for Irpten126 {}
#[doc = "Field `IRPTEN126` reader - Interrupt enable for GPIO 126"]
pub type Irpten126R = crate::FieldReader<Irpten126>;
impl Irpten126R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irpten126 {
        match self.bits {
            0 => Irpten126::Dis,
            1 => Irpten126::Intfall,
            2 => Irpten126::Intrise,
            3 => Irpten126::Intany,
            _ => unreachable!(),
        }
    }
    #[doc = "Interrupts are disabled for this GPIO"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Irpten126::Dis
    }
    #[doc = "Interrupts are enabled for falling edge transition on this GPIO"]
    #[inline(always)]
    pub fn is_intfall(&self) -> bool {
        *self == Irpten126::Intfall
    }
    #[doc = "Interrupts are enabled for rising edge transitions on this GPIO"]
    #[inline(always)]
    pub fn is_intrise(&self) -> bool {
        *self == Irpten126::Intrise
    }
    #[doc = "Interrupts are enabled for any edge transition on this GPIO"]
    #[inline(always)]
    pub fn is_intany(&self) -> bool {
        *self == Irpten126::Intany
    }
}
#[doc = "Field `IRPTEN126` writer - Interrupt enable for GPIO 126"]
pub type Irpten126W<'a, REG> = crate::FieldWriter<'a, REG, 2, Irpten126, crate::Safe>;
impl<'a, REG> Irpten126W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Interrupts are disabled for this GPIO"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten126::Dis)
    }
    #[doc = "Interrupts are enabled for falling edge transition on this GPIO"]
    #[inline(always)]
    pub fn intfall(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten126::Intfall)
    }
    #[doc = "Interrupts are enabled for rising edge transitions on this GPIO"]
    #[inline(always)]
    pub fn intrise(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten126::Intrise)
    }
    #[doc = "Interrupts are enabled for any edge transition on this GPIO"]
    #[inline(always)]
    pub fn intany(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten126::Intany)
    }
}
#[doc = "Pin IO mode selection for GPIO pin 126\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Outcfg126 {
    #[doc = "0: Output Disabled"]
    Dis = 0,
    #[doc = "1: Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    Pushpull = 1,
    #[doc = "2: Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    Od = 2,
    #[doc = "3: Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    Ts = 3,
}
impl From<Outcfg126> for u8 {
    #[inline(always)]
    fn from(variant: Outcfg126) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Outcfg126 {
    type Ux = u8;
}
impl crate::IsEnum for Outcfg126 {}
#[doc = "Field `OUTCFG126` reader - Pin IO mode selection for GPIO pin 126"]
pub type Outcfg126R = crate::FieldReader<Outcfg126>;
impl Outcfg126R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Outcfg126 {
        match self.bits {
            0 => Outcfg126::Dis,
            1 => Outcfg126::Pushpull,
            2 => Outcfg126::Od,
            3 => Outcfg126::Ts,
            _ => unreachable!(),
        }
    }
    #[doc = "Output Disabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Outcfg126::Dis
    }
    #[doc = "Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == Outcfg126::Pushpull
    }
    #[doc = "Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == Outcfg126::Od
    }
    #[doc = "Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == Outcfg126::Ts
    }
}
#[doc = "Field `OUTCFG126` writer - Pin IO mode selection for GPIO pin 126"]
pub type Outcfg126W<'a, REG> = crate::FieldWriter<'a, REG, 2, Outcfg126, crate::Safe>;
impl<'a, REG> Outcfg126W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Output Disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg126::Dis)
    }
    #[doc = "Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg126::Pushpull)
    }
    #[doc = "Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    #[inline(always)]
    pub fn od(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg126::Od)
    }
    #[doc = "Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    #[inline(always)]
    pub fn ts(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg126::Ts)
    }
}
impl R {
    #[doc = "Bits 0:3 - Function select for GPIO pin 126"]
    #[inline(always)]
    pub fn fncsel126(&self) -> Fncsel126R {
        Fncsel126R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Input enable for GPIO 126"]
    #[inline(always)]
    pub fn inpen126(&self) -> Inpen126R {
        Inpen126R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Return 0 for read data on GPIO 126"]
    #[inline(always)]
    pub fn rdzero126(&self) -> Rdzero126R {
        Rdzero126R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Interrupt enable for GPIO 126"]
    #[inline(always)]
    pub fn irpten126(&self) -> Irpten126R {
        Irpten126R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Pin IO mode selection for GPIO pin 126"]
    #[inline(always)]
    pub fn outcfg126(&self) -> Outcfg126R {
        Outcfg126R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Function select for GPIO pin 126"]
    #[inline(always)]
    #[must_use]
    pub fn fncsel126(&mut self) -> Fncsel126W<Pincfg126Spec> {
        Fncsel126W::new(self, 0)
    }
    #[doc = "Bit 4 - Input enable for GPIO 126"]
    #[inline(always)]
    #[must_use]
    pub fn inpen126(&mut self) -> Inpen126W<Pincfg126Spec> {
        Inpen126W::new(self, 4)
    }
    #[doc = "Bit 5 - Return 0 for read data on GPIO 126"]
    #[inline(always)]
    #[must_use]
    pub fn rdzero126(&mut self) -> Rdzero126W<Pincfg126Spec> {
        Rdzero126W::new(self, 5)
    }
    #[doc = "Bits 6:7 - Interrupt enable for GPIO 126"]
    #[inline(always)]
    #[must_use]
    pub fn irpten126(&mut self) -> Irpten126W<Pincfg126Spec> {
        Irpten126W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Pin IO mode selection for GPIO pin 126"]
    #[inline(always)]
    #[must_use]
    pub fn outcfg126(&mut self) -> Outcfg126W<Pincfg126Spec> {
        Outcfg126W::new(self, 8)
    }
}
#[doc = "Controls the operation of virtual GPIO pin 126.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg126::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg126::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pincfg126Spec;
impl crate::RegisterSpec for Pincfg126Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pincfg126::R`](R) reader structure"]
impl crate::Readable for Pincfg126Spec {}
#[doc = "`write(|w| ..)` method takes [`pincfg126::W`](W) writer structure"]
impl crate::Writable for Pincfg126Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PINCFG126 to value 0x03"]
impl crate::Resettable for Pincfg126Spec {
    const RESET_VALUE: u32 = 0x03;
}
