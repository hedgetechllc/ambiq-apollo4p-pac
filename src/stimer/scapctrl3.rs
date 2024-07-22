#[doc = "Register `SCAPCTRL3` reader"]
pub type R = crate::R<Scapctrl3Spec>;
#[doc = "Register `SCAPCTRL3` writer"]
pub type W = crate::W<Scapctrl3Spec>;
#[doc = "Field `STSEL3` reader - STIMER Capture 3 Select."]
pub type Stsel3R = crate::FieldReader;
#[doc = "Field `STSEL3` writer - STIMER Capture 3 Select."]
pub type Stsel3W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "STIMER Capture 3 Polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stpol3 {
    #[doc = "0: Capture on low to high GPIO transition"]
    Caplh = 0,
    #[doc = "1: Capture on high to low GPIO transition"]
    Caphl = 1,
}
impl From<Stpol3> for bool {
    #[inline(always)]
    fn from(variant: Stpol3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STPOL3` reader - STIMER Capture 3 Polarity."]
pub type Stpol3R = crate::BitReader<Stpol3>;
impl Stpol3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stpol3 {
        match self.bits {
            false => Stpol3::Caplh,
            true => Stpol3::Caphl,
        }
    }
    #[doc = "Capture on low to high GPIO transition"]
    #[inline(always)]
    pub fn is_caplh(&self) -> bool {
        *self == Stpol3::Caplh
    }
    #[doc = "Capture on high to low GPIO transition"]
    #[inline(always)]
    pub fn is_caphl(&self) -> bool {
        *self == Stpol3::Caphl
    }
}
#[doc = "Field `STPOL3` writer - STIMER Capture 3 Polarity."]
pub type Stpol3W<'a, REG> = crate::BitWriter<'a, REG, Stpol3>;
impl<'a, REG> Stpol3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Capture on low to high GPIO transition"]
    #[inline(always)]
    pub fn caplh(self) -> &'a mut crate::W<REG> {
        self.variant(Stpol3::Caplh)
    }
    #[doc = "Capture on high to low GPIO transition"]
    #[inline(always)]
    pub fn caphl(self) -> &'a mut crate::W<REG> {
        self.variant(Stpol3::Caphl)
    }
}
#[doc = "Selects whether capture 3 is enabled for the specified capture register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Capture3 {
    #[doc = "0: Capture function disabled."]
    Disable = 0,
    #[doc = "1: Capture function enabled."]
    Enable = 1,
}
impl From<Capture3> for bool {
    #[inline(always)]
    fn from(variant: Capture3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPTURE3` reader - Selects whether capture 3 is enabled for the specified capture register."]
pub type Capture3R = crate::BitReader<Capture3>;
impl Capture3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Capture3 {
        match self.bits {
            false => Capture3::Disable,
            true => Capture3::Enable,
        }
    }
    #[doc = "Capture function disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Capture3::Disable
    }
    #[doc = "Capture function enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Capture3::Enable
    }
}
#[doc = "Field `CAPTURE3` writer - Selects whether capture 3 is enabled for the specified capture register."]
pub type Capture3W<'a, REG> = crate::BitWriter<'a, REG, Capture3>;
impl<'a, REG> Capture3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Capture function disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Capture3::Disable)
    }
    #[doc = "Capture function enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Capture3::Enable)
    }
}
impl R {
    #[doc = "Bits 0:6 - STIMER Capture 3 Select."]
    #[inline(always)]
    pub fn stsel3(&self) -> Stsel3R {
        Stsel3R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 8 - STIMER Capture 3 Polarity."]
    #[inline(always)]
    pub fn stpol3(&self) -> Stpol3R {
        Stpol3R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Selects whether capture 3 is enabled for the specified capture register."]
    #[inline(always)]
    pub fn capture3(&self) -> Capture3R {
        Capture3R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - STIMER Capture 3 Select."]
    #[inline(always)]
    #[must_use]
    pub fn stsel3(&mut self) -> Stsel3W<Scapctrl3Spec> {
        Stsel3W::new(self, 0)
    }
    #[doc = "Bit 8 - STIMER Capture 3 Polarity."]
    #[inline(always)]
    #[must_use]
    pub fn stpol3(&mut self) -> Stpol3W<Scapctrl3Spec> {
        Stpol3W::new(self, 8)
    }
    #[doc = "Bit 9 - Selects whether capture 3 is enabled for the specified capture register."]
    #[inline(always)]
    #[must_use]
    pub fn capture3(&mut self) -> Capture3W<Scapctrl3Spec> {
        Capture3W::new(self, 9)
    }
}
#[doc = "The STIMER Capture Control Register controls each of the 4 capture registers. It selects their GPIO pin number for a trigger source, enables a capture operation and sets the input polarity for the capture. NOTE: 8-bit writes can control individual capture registers atomically.\n\nYou can [`read`](crate::Reg::read) this register and get [`scapctrl3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scapctrl3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scapctrl3Spec;
impl crate::RegisterSpec for Scapctrl3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scapctrl3::R`](R) reader structure"]
impl crate::Readable for Scapctrl3Spec {}
#[doc = "`write(|w| ..)` method takes [`scapctrl3::W`](W) writer structure"]
impl crate::Writable for Scapctrl3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCAPCTRL3 to value 0x7f"]
impl crate::Resettable for Scapctrl3Spec {
    const RESET_VALUE: u32 = 0x7f;
}
