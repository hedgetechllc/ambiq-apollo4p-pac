#[doc = "Register `SCAPCTRL1` reader"]
pub type R = crate::R<Scapctrl1Spec>;
#[doc = "Register `SCAPCTRL1` writer"]
pub type W = crate::W<Scapctrl1Spec>;
#[doc = "Field `STSEL1` reader - STIMER Capture 1 Select."]
pub type Stsel1R = crate::FieldReader;
#[doc = "Field `STSEL1` writer - STIMER Capture 1 Select."]
pub type Stsel1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "STIMER Capture 1 Polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stpol1 {
    #[doc = "0: Capture on low to high GPIO transition"]
    Caplh = 0,
    #[doc = "1: Capture on high to low GPIO transition"]
    Caphl = 1,
}
impl From<Stpol1> for bool {
    #[inline(always)]
    fn from(variant: Stpol1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STPOL1` reader - STIMER Capture 1 Polarity."]
pub type Stpol1R = crate::BitReader<Stpol1>;
impl Stpol1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stpol1 {
        match self.bits {
            false => Stpol1::Caplh,
            true => Stpol1::Caphl,
        }
    }
    #[doc = "Capture on low to high GPIO transition"]
    #[inline(always)]
    pub fn is_caplh(&self) -> bool {
        *self == Stpol1::Caplh
    }
    #[doc = "Capture on high to low GPIO transition"]
    #[inline(always)]
    pub fn is_caphl(&self) -> bool {
        *self == Stpol1::Caphl
    }
}
#[doc = "Field `STPOL1` writer - STIMER Capture 1 Polarity."]
pub type Stpol1W<'a, REG> = crate::BitWriter<'a, REG, Stpol1>;
impl<'a, REG> Stpol1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Capture on low to high GPIO transition"]
    #[inline(always)]
    pub fn caplh(self) -> &'a mut crate::W<REG> {
        self.variant(Stpol1::Caplh)
    }
    #[doc = "Capture on high to low GPIO transition"]
    #[inline(always)]
    pub fn caphl(self) -> &'a mut crate::W<REG> {
        self.variant(Stpol1::Caphl)
    }
}
#[doc = "Selects whether capture 1 is enabled for the specified capture register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Capture1 {
    #[doc = "0: Capture function disabled."]
    Disable = 0,
    #[doc = "1: Capture function enabled."]
    Enable = 1,
}
impl From<Capture1> for bool {
    #[inline(always)]
    fn from(variant: Capture1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPTURE1` reader - Selects whether capture 1 is enabled for the specified capture register."]
pub type Capture1R = crate::BitReader<Capture1>;
impl Capture1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Capture1 {
        match self.bits {
            false => Capture1::Disable,
            true => Capture1::Enable,
        }
    }
    #[doc = "Capture function disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Capture1::Disable
    }
    #[doc = "Capture function enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Capture1::Enable
    }
}
#[doc = "Field `CAPTURE1` writer - Selects whether capture 1 is enabled for the specified capture register."]
pub type Capture1W<'a, REG> = crate::BitWriter<'a, REG, Capture1>;
impl<'a, REG> Capture1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Capture function disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Capture1::Disable)
    }
    #[doc = "Capture function enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Capture1::Enable)
    }
}
impl R {
    #[doc = "Bits 0:6 - STIMER Capture 1 Select."]
    #[inline(always)]
    pub fn stsel1(&self) -> Stsel1R {
        Stsel1R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 8 - STIMER Capture 1 Polarity."]
    #[inline(always)]
    pub fn stpol1(&self) -> Stpol1R {
        Stpol1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Selects whether capture 1 is enabled for the specified capture register."]
    #[inline(always)]
    pub fn capture1(&self) -> Capture1R {
        Capture1R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - STIMER Capture 1 Select."]
    #[inline(always)]
    #[must_use]
    pub fn stsel1(&mut self) -> Stsel1W<Scapctrl1Spec> {
        Stsel1W::new(self, 0)
    }
    #[doc = "Bit 8 - STIMER Capture 1 Polarity."]
    #[inline(always)]
    #[must_use]
    pub fn stpol1(&mut self) -> Stpol1W<Scapctrl1Spec> {
        Stpol1W::new(self, 8)
    }
    #[doc = "Bit 9 - Selects whether capture 1 is enabled for the specified capture register."]
    #[inline(always)]
    #[must_use]
    pub fn capture1(&mut self) -> Capture1W<Scapctrl1Spec> {
        Capture1W::new(self, 9)
    }
}
#[doc = "The STIMER Capture Control Register controls each of the 4 capture registers. It selects their GPIO pin number for a trigger source, enables a capture operation and sets the input polarity for the capture. NOTE: 8-bit writes can control individual capture registers atomically.\n\nYou can [`read`](crate::Reg::read) this register and get [`scapctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scapctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scapctrl1Spec;
impl crate::RegisterSpec for Scapctrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scapctrl1::R`](R) reader structure"]
impl crate::Readable for Scapctrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`scapctrl1::W`](W) writer structure"]
impl crate::Writable for Scapctrl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCAPCTRL1 to value 0x7f"]
impl crate::Resettable for Scapctrl1Spec {
    const RESET_VALUE: u32 = 0x7f;
}
