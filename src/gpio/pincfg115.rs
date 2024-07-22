#[doc = "Register `PINCFG115` reader"]
pub type R = crate::R<Pincfg115Spec>;
#[doc = "Register `PINCFG115` writer"]
pub type W = crate::W<Pincfg115Spec>;
#[doc = "Function select for GPIO pin 115\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fncsel115 {
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
    Ct115 = 6,
    #[doc = "7: Reserved selection. Operation unknown if selected."]
    Reserved7 = 7,
    #[doc = "8: Observation bus bit 3"]
    Obsbus3 = 8,
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
impl From<Fncsel115> for u8 {
    #[inline(always)]
    fn from(variant: Fncsel115) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fncsel115 {
    type Ux = u8;
}
impl crate::IsEnum for Fncsel115 {}
#[doc = "Field `FNCSEL115` reader - Function select for GPIO pin 115"]
pub type Fncsel115R = crate::FieldReader<Fncsel115>;
impl Fncsel115R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fncsel115 {
        match self.bits {
            0 => Fncsel115::Reserved0,
            1 => Fncsel115::Reserved1,
            2 => Fncsel115::Reserved2,
            3 => Fncsel115::Gpio,
            4 => Fncsel115::Reserved4,
            5 => Fncsel115::Reserved5,
            6 => Fncsel115::Ct115,
            7 => Fncsel115::Reserved7,
            8 => Fncsel115::Obsbus3,
            9 => Fncsel115::Reserved9,
            10 => Fncsel115::Reserved10,
            11 => Fncsel115::Reserved11,
            12 => Fncsel115::Reserved12,
            13 => Fncsel115::Reserved13,
            14 => Fncsel115::Reserved14,
            15 => Fncsel115::Reserved15,
            _ => unreachable!(),
        }
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved0(&self) -> bool {
        *self == Fncsel115::Reserved0
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved1(&self) -> bool {
        *self == Fncsel115::Reserved1
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == Fncsel115::Reserved2
    }
    #[doc = "General purpose I/O"]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == Fncsel115::Gpio
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved4(&self) -> bool {
        *self == Fncsel115::Reserved4
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved5(&self) -> bool {
        *self == Fncsel115::Reserved5
    }
    #[doc = "Timer/Counter input or output; Selection of direction is done via CTIMER register settings."]
    #[inline(always)]
    pub fn is_ct115(&self) -> bool {
        *self == Fncsel115::Ct115
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved7(&self) -> bool {
        *self == Fncsel115::Reserved7
    }
    #[doc = "Observation bus bit 3"]
    #[inline(always)]
    pub fn is_obsbus3(&self) -> bool {
        *self == Fncsel115::Obsbus3
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved9(&self) -> bool {
        *self == Fncsel115::Reserved9
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved10(&self) -> bool {
        *self == Fncsel115::Reserved10
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved11(&self) -> bool {
        *self == Fncsel115::Reserved11
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved12(&self) -> bool {
        *self == Fncsel115::Reserved12
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved13(&self) -> bool {
        *self == Fncsel115::Reserved13
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved14(&self) -> bool {
        *self == Fncsel115::Reserved14
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved15(&self) -> bool {
        *self == Fncsel115::Reserved15
    }
}
#[doc = "Field `FNCSEL115` writer - Function select for GPIO pin 115"]
pub type Fncsel115W<'a, REG> = crate::FieldWriter<'a, REG, 4, Fncsel115, crate::Safe>;
impl<'a, REG> Fncsel115W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved0(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel115::Reserved0)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved1(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel115::Reserved1)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel115::Reserved2)
    }
    #[doc = "General purpose I/O"]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel115::Gpio)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved4(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel115::Reserved4)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved5(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel115::Reserved5)
    }
    #[doc = "Timer/Counter input or output; Selection of direction is done via CTIMER register settings."]
    #[inline(always)]
    pub fn ct115(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel115::Ct115)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved7(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel115::Reserved7)
    }
    #[doc = "Observation bus bit 3"]
    #[inline(always)]
    pub fn obsbus3(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel115::Obsbus3)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved9(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel115::Reserved9)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved10(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel115::Reserved10)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved11(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel115::Reserved11)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved12(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel115::Reserved12)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved13(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel115::Reserved13)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved14(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel115::Reserved14)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved15(self) -> &'a mut crate::W<REG> {
        self.variant(Fncsel115::Reserved15)
    }
}
#[doc = "Field `INPEN115` reader - Input enable for GPIO 115"]
pub type Inpen115R = crate::BitReader;
#[doc = "Field `INPEN115` writer - Input enable for GPIO 115"]
pub type Inpen115W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDZERO115` reader - Return 0 for read data on GPIO 115"]
pub type Rdzero115R = crate::BitReader;
#[doc = "Field `RDZERO115` writer - Return 0 for read data on GPIO 115"]
pub type Rdzero115W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Interrupt enable for GPIO 115\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Irpten115 {
    #[doc = "0: Interrupts are disabled for this GPIO"]
    Dis = 0,
    #[doc = "1: Interrupts are enabled for falling edge transition on this GPIO"]
    Intfall = 1,
    #[doc = "2: Interrupts are enabled for rising edge transitions on this GPIO"]
    Intrise = 2,
    #[doc = "3: Interrupts are enabled for any edge transition on this GPIO"]
    Intany = 3,
}
impl From<Irpten115> for u8 {
    #[inline(always)]
    fn from(variant: Irpten115) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Irpten115 {
    type Ux = u8;
}
impl crate::IsEnum for Irpten115 {}
#[doc = "Field `IRPTEN115` reader - Interrupt enable for GPIO 115"]
pub type Irpten115R = crate::FieldReader<Irpten115>;
impl Irpten115R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irpten115 {
        match self.bits {
            0 => Irpten115::Dis,
            1 => Irpten115::Intfall,
            2 => Irpten115::Intrise,
            3 => Irpten115::Intany,
            _ => unreachable!(),
        }
    }
    #[doc = "Interrupts are disabled for this GPIO"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Irpten115::Dis
    }
    #[doc = "Interrupts are enabled for falling edge transition on this GPIO"]
    #[inline(always)]
    pub fn is_intfall(&self) -> bool {
        *self == Irpten115::Intfall
    }
    #[doc = "Interrupts are enabled for rising edge transitions on this GPIO"]
    #[inline(always)]
    pub fn is_intrise(&self) -> bool {
        *self == Irpten115::Intrise
    }
    #[doc = "Interrupts are enabled for any edge transition on this GPIO"]
    #[inline(always)]
    pub fn is_intany(&self) -> bool {
        *self == Irpten115::Intany
    }
}
#[doc = "Field `IRPTEN115` writer - Interrupt enable for GPIO 115"]
pub type Irpten115W<'a, REG> = crate::FieldWriter<'a, REG, 2, Irpten115, crate::Safe>;
impl<'a, REG> Irpten115W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Interrupts are disabled for this GPIO"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten115::Dis)
    }
    #[doc = "Interrupts are enabled for falling edge transition on this GPIO"]
    #[inline(always)]
    pub fn intfall(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten115::Intfall)
    }
    #[doc = "Interrupts are enabled for rising edge transitions on this GPIO"]
    #[inline(always)]
    pub fn intrise(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten115::Intrise)
    }
    #[doc = "Interrupts are enabled for any edge transition on this GPIO"]
    #[inline(always)]
    pub fn intany(self) -> &'a mut crate::W<REG> {
        self.variant(Irpten115::Intany)
    }
}
#[doc = "Pin IO mode selection for GPIO pin 115\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Outcfg115 {
    #[doc = "0: Output Disabled"]
    Dis = 0,
    #[doc = "1: Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    Pushpull = 1,
    #[doc = "2: Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    Od = 2,
    #[doc = "3: Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    Ts = 3,
}
impl From<Outcfg115> for u8 {
    #[inline(always)]
    fn from(variant: Outcfg115) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Outcfg115 {
    type Ux = u8;
}
impl crate::IsEnum for Outcfg115 {}
#[doc = "Field `OUTCFG115` reader - Pin IO mode selection for GPIO pin 115"]
pub type Outcfg115R = crate::FieldReader<Outcfg115>;
impl Outcfg115R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Outcfg115 {
        match self.bits {
            0 => Outcfg115::Dis,
            1 => Outcfg115::Pushpull,
            2 => Outcfg115::Od,
            3 => Outcfg115::Ts,
            _ => unreachable!(),
        }
    }
    #[doc = "Output Disabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Outcfg115::Dis
    }
    #[doc = "Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == Outcfg115::Pushpull
    }
    #[doc = "Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == Outcfg115::Od
    }
    #[doc = "Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == Outcfg115::Ts
    }
}
#[doc = "Field `OUTCFG115` writer - Pin IO mode selection for GPIO pin 115"]
pub type Outcfg115W<'a, REG> = crate::FieldWriter<'a, REG, 2, Outcfg115, crate::Safe>;
impl<'a, REG> Outcfg115W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Output Disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg115::Dis)
    }
    #[doc = "Output configured in push pull mode. Will drive 0 and 1 values on pin."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg115::Pushpull)
    }
    #[doc = "Output configured in open drain mode. Will only drive pin low, tristate otherwise."]
    #[inline(always)]
    pub fn od(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg115::Od)
    }
    #[doc = "Output configured in Tristate-able push pull mode. Will drive 0, 1 of HiZ on pin."]
    #[inline(always)]
    pub fn ts(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg115::Ts)
    }
}
impl R {
    #[doc = "Bits 0:3 - Function select for GPIO pin 115"]
    #[inline(always)]
    pub fn fncsel115(&self) -> Fncsel115R {
        Fncsel115R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Input enable for GPIO 115"]
    #[inline(always)]
    pub fn inpen115(&self) -> Inpen115R {
        Inpen115R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Return 0 for read data on GPIO 115"]
    #[inline(always)]
    pub fn rdzero115(&self) -> Rdzero115R {
        Rdzero115R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Interrupt enable for GPIO 115"]
    #[inline(always)]
    pub fn irpten115(&self) -> Irpten115R {
        Irpten115R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Pin IO mode selection for GPIO pin 115"]
    #[inline(always)]
    pub fn outcfg115(&self) -> Outcfg115R {
        Outcfg115R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Function select for GPIO pin 115"]
    #[inline(always)]
    #[must_use]
    pub fn fncsel115(&mut self) -> Fncsel115W<Pincfg115Spec> {
        Fncsel115W::new(self, 0)
    }
    #[doc = "Bit 4 - Input enable for GPIO 115"]
    #[inline(always)]
    #[must_use]
    pub fn inpen115(&mut self) -> Inpen115W<Pincfg115Spec> {
        Inpen115W::new(self, 4)
    }
    #[doc = "Bit 5 - Return 0 for read data on GPIO 115"]
    #[inline(always)]
    #[must_use]
    pub fn rdzero115(&mut self) -> Rdzero115W<Pincfg115Spec> {
        Rdzero115W::new(self, 5)
    }
    #[doc = "Bits 6:7 - Interrupt enable for GPIO 115"]
    #[inline(always)]
    #[must_use]
    pub fn irpten115(&mut self) -> Irpten115W<Pincfg115Spec> {
        Irpten115W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Pin IO mode selection for GPIO pin 115"]
    #[inline(always)]
    #[must_use]
    pub fn outcfg115(&mut self) -> Outcfg115W<Pincfg115Spec> {
        Outcfg115W::new(self, 8)
    }
}
#[doc = "Controls the operation of virtual GPIO pin 115.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg115::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg115::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pincfg115Spec;
impl crate::RegisterSpec for Pincfg115Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pincfg115::R`](R) reader structure"]
impl crate::Readable for Pincfg115Spec {}
#[doc = "`write(|w| ..)` method takes [`pincfg115::W`](W) writer structure"]
impl crate::Writable for Pincfg115Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PINCFG115 to value 0x03"]
impl crate::Resettable for Pincfg115Spec {
    const RESET_VALUE: u32 = 0x03;
}
