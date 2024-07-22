#[doc = "Register `SCAPCTRL2` reader"]
pub type R = crate::R<Scapctrl2Spec>;
#[doc = "Register `SCAPCTRL2` writer"]
pub type W = crate::W<Scapctrl2Spec>;
#[doc = "Field `STSEL2` reader - STIMER Capture 2 Select."]
pub type Stsel2R = crate::FieldReader;
#[doc = "Field `STSEL2` writer - STIMER Capture 2 Select."]
pub type Stsel2W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "STIMER Capture 2 Polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stpol2 {
    #[doc = "0: Capture on low to high GPIO transition"]
    Caplh = 0,
    #[doc = "1: Capture on high to low GPIO transition"]
    Caphl = 1,
}
impl From<Stpol2> for bool {
    #[inline(always)]
    fn from(variant: Stpol2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STPOL2` reader - STIMER Capture 2 Polarity."]
pub type Stpol2R = crate::BitReader<Stpol2>;
impl Stpol2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stpol2 {
        match self.bits {
            false => Stpol2::Caplh,
            true => Stpol2::Caphl,
        }
    }
    #[doc = "Capture on low to high GPIO transition"]
    #[inline(always)]
    pub fn is_caplh(&self) -> bool {
        *self == Stpol2::Caplh
    }
    #[doc = "Capture on high to low GPIO transition"]
    #[inline(always)]
    pub fn is_caphl(&self) -> bool {
        *self == Stpol2::Caphl
    }
}
#[doc = "Field `STPOL2` writer - STIMER Capture 2 Polarity."]
pub type Stpol2W<'a, REG> = crate::BitWriter<'a, REG, Stpol2>;
impl<'a, REG> Stpol2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Capture on low to high GPIO transition"]
    #[inline(always)]
    pub fn caplh(self) -> &'a mut crate::W<REG> {
        self.variant(Stpol2::Caplh)
    }
    #[doc = "Capture on high to low GPIO transition"]
    #[inline(always)]
    pub fn caphl(self) -> &'a mut crate::W<REG> {
        self.variant(Stpol2::Caphl)
    }
}
#[doc = "Selects whether capture 2 is enabled for the specified capture register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Capture2 {
    #[doc = "0: Capture function disabled."]
    Disable = 0,
    #[doc = "1: Capture function enabled."]
    Enable = 1,
}
impl From<Capture2> for bool {
    #[inline(always)]
    fn from(variant: Capture2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPTURE2` reader - Selects whether capture 2 is enabled for the specified capture register."]
pub type Capture2R = crate::BitReader<Capture2>;
impl Capture2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Capture2 {
        match self.bits {
            false => Capture2::Disable,
            true => Capture2::Enable,
        }
    }
    #[doc = "Capture function disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Capture2::Disable
    }
    #[doc = "Capture function enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Capture2::Enable
    }
}
#[doc = "Field `CAPTURE2` writer - Selects whether capture 2 is enabled for the specified capture register."]
pub type Capture2W<'a, REG> = crate::BitWriter<'a, REG, Capture2>;
impl<'a, REG> Capture2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Capture function disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Capture2::Disable)
    }
    #[doc = "Capture function enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Capture2::Enable)
    }
}
impl R {
    #[doc = "Bits 0:6 - STIMER Capture 2 Select."]
    #[inline(always)]
    pub fn stsel2(&self) -> Stsel2R {
        Stsel2R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 8 - STIMER Capture 2 Polarity."]
    #[inline(always)]
    pub fn stpol2(&self) -> Stpol2R {
        Stpol2R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Selects whether capture 2 is enabled for the specified capture register."]
    #[inline(always)]
    pub fn capture2(&self) -> Capture2R {
        Capture2R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - STIMER Capture 2 Select."]
    #[inline(always)]
    #[must_use]
    pub fn stsel2(&mut self) -> Stsel2W<Scapctrl2Spec> {
        Stsel2W::new(self, 0)
    }
    #[doc = "Bit 8 - STIMER Capture 2 Polarity."]
    #[inline(always)]
    #[must_use]
    pub fn stpol2(&mut self) -> Stpol2W<Scapctrl2Spec> {
        Stpol2W::new(self, 8)
    }
    #[doc = "Bit 9 - Selects whether capture 2 is enabled for the specified capture register."]
    #[inline(always)]
    #[must_use]
    pub fn capture2(&mut self) -> Capture2W<Scapctrl2Spec> {
        Capture2W::new(self, 9)
    }
}
#[doc = "The STIMER Capture Control Register controls each of the 4 capture registers. It selects their GPIO pin number for a trigger source, enables a capture operation and sets the input polarity for the capture. NOTE: 8-bit writes can control individual capture registers atomically.\n\nYou can [`read`](crate::Reg::read) this register and get [`scapctrl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scapctrl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scapctrl2Spec;
impl crate::RegisterSpec for Scapctrl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scapctrl2::R`](R) reader structure"]
impl crate::Readable for Scapctrl2Spec {}
#[doc = "`write(|w| ..)` method takes [`scapctrl2::W`](W) writer structure"]
impl crate::Writable for Scapctrl2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCAPCTRL2 to value 0x7f"]
impl crate::Resettable for Scapctrl2Spec {
    const RESET_VALUE: u32 = 0x7f;
}
