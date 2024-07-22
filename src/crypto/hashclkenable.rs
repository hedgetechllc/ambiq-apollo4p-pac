#[doc = "Register `HASHCLKENABLE` reader"]
pub type R = crate::R<HashclkenableSpec>;
#[doc = "Register `HASHCLKENABLE` writer"]
pub type W = crate::W<HashclkenableSpec>;
#[doc = "Enable the hash clock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum En {
    #[doc = "1: the HASH clock is enabled."]
    HashE = 1,
    #[doc = "0: the HASH clock is disabled."]
    HashD = 0,
}
impl From<En> for bool {
    #[inline(always)]
    fn from(variant: En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN` reader - Enable the hash clock."]
pub type EnR = crate::BitReader<En>;
impl EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> En {
        match self.bits {
            true => En::HashE,
            false => En::HashD,
        }
    }
    #[doc = "the HASH clock is enabled."]
    #[inline(always)]
    pub fn is_hash_e(&self) -> bool {
        *self == En::HashE
    }
    #[doc = "the HASH clock is disabled."]
    #[inline(always)]
    pub fn is_hash_d(&self) -> bool {
        *self == En::HashD
    }
}
#[doc = "Field `EN` writer - Enable the hash clock."]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG, En>;
impl<'a, REG> EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "the HASH clock is enabled."]
    #[inline(always)]
    pub fn hash_e(self) -> &'a mut crate::W<REG> {
        self.variant(En::HashE)
    }
    #[doc = "the HASH clock is disabled."]
    #[inline(always)]
    pub fn hash_d(self) -> &'a mut crate::W<REG> {
        self.variant(En::HashD)
    }
}
impl R {
    #[doc = "Bit 0 - Enable the hash clock."]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable the hash clock."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EnW<HashclkenableSpec> {
        EnW::new(self, 0)
    }
}
#[doc = "The HASH clock enable register. Note: This is a special register, affected by internal logic. Test result of this register is NA.\n\nYou can [`read`](crate::Reg::read) this register and get [`hashclkenable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hashclkenable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HashclkenableSpec;
impl crate::RegisterSpec for HashclkenableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hashclkenable::R`](R) reader structure"]
impl crate::Readable for HashclkenableSpec {}
#[doc = "`write(|w| ..)` method takes [`hashclkenable::W`](W) writer structure"]
impl crate::Writable for HashclkenableSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HASHCLKENABLE to value 0"]
impl crate::Resettable for HashclkenableSpec {
    const RESET_VALUE: u32 = 0;
}
