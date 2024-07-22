#[doc = "Register `CHACHACLKENABLE` reader"]
pub type R = crate::R<ChachaclkenableSpec>;
#[doc = "Register `CHACHACLKENABLE` writer"]
pub type W = crate::W<ChachaclkenableSpec>;
#[doc = "Enable the CHACHA SALSA clock enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum En {
    #[doc = "1: the CHACHA SALSA clock is enabled."]
    ChachaE = 1,
    #[doc = "0: the CHACHA SALSA clock is disabled."]
    ChachaD = 0,
}
impl From<En> for bool {
    #[inline(always)]
    fn from(variant: En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN` reader - Enable the CHACHA SALSA clock enable."]
pub type EnR = crate::BitReader<En>;
impl EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> En {
        match self.bits {
            true => En::ChachaE,
            false => En::ChachaD,
        }
    }
    #[doc = "the CHACHA SALSA clock is enabled."]
    #[inline(always)]
    pub fn is_chacha_e(&self) -> bool {
        *self == En::ChachaE
    }
    #[doc = "the CHACHA SALSA clock is disabled."]
    #[inline(always)]
    pub fn is_chacha_d(&self) -> bool {
        *self == En::ChachaD
    }
}
#[doc = "Field `EN` writer - Enable the CHACHA SALSA clock enable."]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG, En>;
impl<'a, REG> EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "the CHACHA SALSA clock is enabled."]
    #[inline(always)]
    pub fn chacha_e(self) -> &'a mut crate::W<REG> {
        self.variant(En::ChachaE)
    }
    #[doc = "the CHACHA SALSA clock is disabled."]
    #[inline(always)]
    pub fn chacha_d(self) -> &'a mut crate::W<REG> {
        self.variant(En::ChachaD)
    }
}
impl R {
    #[doc = "Bit 0 - Enable the CHACHA SALSA clock enable."]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable the CHACHA SALSA clock enable."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EnW<ChachaclkenableSpec> {
        EnW::new(self, 0)
    }
}
#[doc = "CHACHA _SALSA clock enable register. Note: This is a special register, affected by internal logic. Test result of this register is NA.\n\nYou can [`read`](crate::Reg::read) this register and get [`chachaclkenable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chachaclkenable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChachaclkenableSpec;
impl crate::RegisterSpec for ChachaclkenableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chachaclkenable::R`](R) reader structure"]
impl crate::Readable for ChachaclkenableSpec {}
#[doc = "`write(|w| ..)` method takes [`chachaclkenable::W`](W) writer structure"]
impl crate::Writable for ChachaclkenableSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHACHACLKENABLE to value 0"]
impl crate::Resettable for ChachaclkenableSpec {
    const RESET_VALUE: u32 = 0;
}
