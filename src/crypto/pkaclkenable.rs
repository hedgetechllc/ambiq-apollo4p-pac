#[doc = "Register `PKACLKENABLE` reader"]
pub type R = crate::R<PkaclkenableSpec>;
#[doc = "Register `PKACLKENABLE` writer"]
pub type W = crate::W<PkaclkenableSpec>;
#[doc = "Enable the PKA clock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum En {
    #[doc = "1: the PKA clock is enabled."]
    PkaE = 1,
    #[doc = "0: the PKA clock is disabled."]
    PkaD = 0,
}
impl From<En> for bool {
    #[inline(always)]
    fn from(variant: En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN` reader - Enable the PKA clock."]
pub type EnR = crate::BitReader<En>;
impl EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> En {
        match self.bits {
            true => En::PkaE,
            false => En::PkaD,
        }
    }
    #[doc = "the PKA clock is enabled."]
    #[inline(always)]
    pub fn is_pka_e(&self) -> bool {
        *self == En::PkaE
    }
    #[doc = "the PKA clock is disabled."]
    #[inline(always)]
    pub fn is_pka_d(&self) -> bool {
        *self == En::PkaD
    }
}
#[doc = "Field `EN` writer - Enable the PKA clock."]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG, En>;
impl<'a, REG> EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "the PKA clock is enabled."]
    #[inline(always)]
    pub fn pka_e(self) -> &'a mut crate::W<REG> {
        self.variant(En::PkaE)
    }
    #[doc = "the PKA clock is disabled."]
    #[inline(always)]
    pub fn pka_d(self) -> &'a mut crate::W<REG> {
        self.variant(En::PkaD)
    }
}
impl R {
    #[doc = "Bit 0 - Enable the PKA clock."]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable the PKA clock."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EnW<PkaclkenableSpec> {
        EnW::new(self, 0)
    }
}
#[doc = "The PKA clock enable register. Note: This is a special register, affected by internal logic. Test result of this register is NA.\n\nYou can [`read`](crate::Reg::read) this register and get [`pkaclkenable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pkaclkenable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PkaclkenableSpec;
impl crate::RegisterSpec for PkaclkenableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pkaclkenable::R`](R) reader structure"]
impl crate::Readable for PkaclkenableSpec {}
#[doc = "`write(|w| ..)` method takes [`pkaclkenable::W`](W) writer structure"]
impl crate::Writable for PkaclkenableSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PKACLKENABLE to value 0"]
impl crate::Resettable for PkaclkenableSpec {
    const RESET_VALUE: u32 = 0;
}
