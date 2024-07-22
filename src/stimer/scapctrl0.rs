#[doc = "Register `SCAPCTRL0` reader"]
pub type R = crate::R<Scapctrl0Spec>;
#[doc = "Register `SCAPCTRL0` writer"]
pub type W = crate::W<Scapctrl0Spec>;
#[doc = "Field `STSEL0` reader - STIMER Capture 0 Select."]
pub type Stsel0R = crate::FieldReader;
#[doc = "Field `STSEL0` writer - STIMER Capture 0 Select."]
pub type Stsel0W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "STIMER Capture 0 Polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stpol0 {
    #[doc = "0: Capture on low to high GPIO transition"]
    Caplh = 0,
    #[doc = "1: Capture on high to low GPIO transition"]
    Caphl = 1,
}
impl From<Stpol0> for bool {
    #[inline(always)]
    fn from(variant: Stpol0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STPOL0` reader - STIMER Capture 0 Polarity."]
pub type Stpol0R = crate::BitReader<Stpol0>;
impl Stpol0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stpol0 {
        match self.bits {
            false => Stpol0::Caplh,
            true => Stpol0::Caphl,
        }
    }
    #[doc = "Capture on low to high GPIO transition"]
    #[inline(always)]
    pub fn is_caplh(&self) -> bool {
        *self == Stpol0::Caplh
    }
    #[doc = "Capture on high to low GPIO transition"]
    #[inline(always)]
    pub fn is_caphl(&self) -> bool {
        *self == Stpol0::Caphl
    }
}
#[doc = "Field `STPOL0` writer - STIMER Capture 0 Polarity."]
pub type Stpol0W<'a, REG> = crate::BitWriter<'a, REG, Stpol0>;
impl<'a, REG> Stpol0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Capture on low to high GPIO transition"]
    #[inline(always)]
    pub fn caplh(self) -> &'a mut crate::W<REG> {
        self.variant(Stpol0::Caplh)
    }
    #[doc = "Capture on high to low GPIO transition"]
    #[inline(always)]
    pub fn caphl(self) -> &'a mut crate::W<REG> {
        self.variant(Stpol0::Caphl)
    }
}
#[doc = "Selects whether capture 0 is enabled for the specified capture register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Capture0 {
    #[doc = "0: Capture function disabled."]
    Disable = 0,
    #[doc = "1: Capture function enabled."]
    Enable = 1,
}
impl From<Capture0> for bool {
    #[inline(always)]
    fn from(variant: Capture0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPTURE0` reader - Selects whether capture 0 is enabled for the specified capture register."]
pub type Capture0R = crate::BitReader<Capture0>;
impl Capture0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Capture0 {
        match self.bits {
            false => Capture0::Disable,
            true => Capture0::Enable,
        }
    }
    #[doc = "Capture function disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Capture0::Disable
    }
    #[doc = "Capture function enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Capture0::Enable
    }
}
#[doc = "Field `CAPTURE0` writer - Selects whether capture 0 is enabled for the specified capture register."]
pub type Capture0W<'a, REG> = crate::BitWriter<'a, REG, Capture0>;
impl<'a, REG> Capture0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Capture function disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Capture0::Disable)
    }
    #[doc = "Capture function enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Capture0::Enable)
    }
}
impl R {
    #[doc = "Bits 0:6 - STIMER Capture 0 Select."]
    #[inline(always)]
    pub fn stsel0(&self) -> Stsel0R {
        Stsel0R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 8 - STIMER Capture 0 Polarity."]
    #[inline(always)]
    pub fn stpol0(&self) -> Stpol0R {
        Stpol0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Selects whether capture 0 is enabled for the specified capture register."]
    #[inline(always)]
    pub fn capture0(&self) -> Capture0R {
        Capture0R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - STIMER Capture 0 Select."]
    #[inline(always)]
    #[must_use]
    pub fn stsel0(&mut self) -> Stsel0W<Scapctrl0Spec> {
        Stsel0W::new(self, 0)
    }
    #[doc = "Bit 8 - STIMER Capture 0 Polarity."]
    #[inline(always)]
    #[must_use]
    pub fn stpol0(&mut self) -> Stpol0W<Scapctrl0Spec> {
        Stpol0W::new(self, 8)
    }
    #[doc = "Bit 9 - Selects whether capture 0 is enabled for the specified capture register."]
    #[inline(always)]
    #[must_use]
    pub fn capture0(&mut self) -> Capture0W<Scapctrl0Spec> {
        Capture0W::new(self, 9)
    }
}
#[doc = "The STIMER Capture Control Register controls each of the 4 capture registers. It selects their GPIO pin number for a trigger source, enables a capture operation and sets the input polarity for the capture. NOTE: 8-bit writes can control individual capture registers atomically.\n\nYou can [`read`](crate::Reg::read) this register and get [`scapctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scapctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scapctrl0Spec;
impl crate::RegisterSpec for Scapctrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scapctrl0::R`](R) reader structure"]
impl crate::Readable for Scapctrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`scapctrl0::W`](W) writer structure"]
impl crate::Writable for Scapctrl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCAPCTRL0 to value 0x7f"]
impl crate::Resettable for Scapctrl0Spec {
    const RESET_VALUE: u32 = 0x7f;
}
