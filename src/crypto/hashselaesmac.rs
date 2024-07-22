#[doc = "Register `HASHSELAESMAC` reader"]
pub type R = crate::R<HashselaesmacSpec>;
#[doc = "Register `HASHSELAESMAC` writer"]
pub type W = crate::W<HashselaesmacSpec>;
#[doc = "Hash or AES MAC module select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hashselaesmac {
    #[doc = "0: select the hash module"]
    HashMod = 0,
    #[doc = "1: select the AES mac module"]
    MacMod = 1,
}
impl From<Hashselaesmac> for bool {
    #[inline(always)]
    fn from(variant: Hashselaesmac) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HASHSELAESMAC` reader - Hash or AES MAC module select."]
pub type HashselaesmacR = crate::BitReader<Hashselaesmac>;
impl HashselaesmacR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hashselaesmac {
        match self.bits {
            false => Hashselaesmac::HashMod,
            true => Hashselaesmac::MacMod,
        }
    }
    #[doc = "select the hash module"]
    #[inline(always)]
    pub fn is_hash_mod(&self) -> bool {
        *self == Hashselaesmac::HashMod
    }
    #[doc = "select the AES mac module"]
    #[inline(always)]
    pub fn is_mac_mod(&self) -> bool {
        *self == Hashselaesmac::MacMod
    }
}
#[doc = "Field `HASHSELAESMAC` writer - Hash or AES MAC module select."]
pub type HashselaesmacW<'a, REG> = crate::BitWriter<'a, REG, Hashselaesmac>;
impl<'a, REG> HashselaesmacW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "select the hash module"]
    #[inline(always)]
    pub fn hash_mod(self) -> &'a mut crate::W<REG> {
        self.variant(Hashselaesmac::HashMod)
    }
    #[doc = "select the AES mac module"]
    #[inline(always)]
    pub fn mac_mod(self) -> &'a mut crate::W<REG> {
        self.variant(Hashselaesmac::MacMod)
    }
}
#[doc = "GHASH select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ghashsel {
    #[doc = "0: select the hash module"]
    HashMod = 0,
    #[doc = "1: select the ghash module"]
    GhashMod = 1,
}
impl From<Ghashsel> for bool {
    #[inline(always)]
    fn from(variant: Ghashsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GHASHSEL` reader - GHASH select."]
pub type GhashselR = crate::BitReader<Ghashsel>;
impl GhashselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ghashsel {
        match self.bits {
            false => Ghashsel::HashMod,
            true => Ghashsel::GhashMod,
        }
    }
    #[doc = "select the hash module"]
    #[inline(always)]
    pub fn is_hash_mod(&self) -> bool {
        *self == Ghashsel::HashMod
    }
    #[doc = "select the ghash module"]
    #[inline(always)]
    pub fn is_ghash_mod(&self) -> bool {
        *self == Ghashsel::GhashMod
    }
}
#[doc = "Field `GHASHSEL` writer - GHASH select."]
pub type GhashselW<'a, REG> = crate::BitWriter<'a, REG, Ghashsel>;
impl<'a, REG> GhashselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "select the hash module"]
    #[inline(always)]
    pub fn hash_mod(self) -> &'a mut crate::W<REG> {
        self.variant(Ghashsel::HashMod)
    }
    #[doc = "select the ghash module"]
    #[inline(always)]
    pub fn ghash_mod(self) -> &'a mut crate::W<REG> {
        self.variant(Ghashsel::GhashMod)
    }
}
impl R {
    #[doc = "Bit 0 - Hash or AES MAC module select."]
    #[inline(always)]
    pub fn hashselaesmac(&self) -> HashselaesmacR {
        HashselaesmacR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GHASH select."]
    #[inline(always)]
    pub fn ghashsel(&self) -> GhashselR {
        GhashselR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Hash or AES MAC module select."]
    #[inline(always)]
    #[must_use]
    pub fn hashselaesmac(&mut self) -> HashselaesmacW<HashselaesmacSpec> {
        HashselaesmacW::new(self, 0)
    }
    #[doc = "Bit 1 - GHASH select."]
    #[inline(always)]
    #[must_use]
    pub fn ghashsel(&mut self) -> GhashselW<HashselaesmacSpec> {
        GhashselW::new(self, 1)
    }
}
#[doc = "select the AES MAC module rather than the hash module\n\nYou can [`read`](crate::Reg::read) this register and get [`hashselaesmac::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hashselaesmac::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HashselaesmacSpec;
impl crate::RegisterSpec for HashselaesmacSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hashselaesmac::R`](R) reader structure"]
impl crate::Readable for HashselaesmacSpec {}
#[doc = "`write(|w| ..)` method takes [`hashselaesmac::W`](W) writer structure"]
impl crate::Writable for HashselaesmacSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HASHSELAESMAC to value 0"]
impl crate::Resettable for HashselaesmacSpec {
    const RESET_VALUE: u32 = 0;
}
