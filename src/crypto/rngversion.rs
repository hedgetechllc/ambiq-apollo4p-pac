#[doc = "Register `RNGVERSION` reader"]
pub type R = crate::R<RngversionSpec>;
#[doc = "Register `RNGVERSION` writer"]
pub type W = crate::W<RngversionSpec>;
#[doc = "EHR width selection.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ehrwidth192 {
    #[doc = "0: 128 bit EHR"]
    _128Ehr = 0,
    #[doc = "1: 192 bit EHR"]
    _192Ehr = 1,
}
impl From<Ehrwidth192> for bool {
    #[inline(always)]
    fn from(variant: Ehrwidth192) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EHRWIDTH192` reader - EHR width selection."]
pub type Ehrwidth192R = crate::BitReader<Ehrwidth192>;
impl Ehrwidth192R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ehrwidth192 {
        match self.bits {
            false => Ehrwidth192::_128Ehr,
            true => Ehrwidth192::_192Ehr,
        }
    }
    #[doc = "128 bit EHR"]
    #[inline(always)]
    pub fn is_128_ehr(&self) -> bool {
        *self == Ehrwidth192::_128Ehr
    }
    #[doc = "192 bit EHR"]
    #[inline(always)]
    pub fn is_192_ehr(&self) -> bool {
        *self == Ehrwidth192::_192Ehr
    }
}
#[doc = "Field `EHRWIDTH192` writer - EHR width selection."]
pub type Ehrwidth192W<'a, REG> = crate::BitWriter<'a, REG, Ehrwidth192>;
impl<'a, REG> Ehrwidth192W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "128 bit EHR"]
    #[inline(always)]
    pub fn _128_ehr(self) -> &'a mut crate::W<REG> {
        self.variant(Ehrwidth192::_128Ehr)
    }
    #[doc = "192 bit EHR"]
    #[inline(always)]
    pub fn _192_ehr(self) -> &'a mut crate::W<REG> {
        self.variant(Ehrwidth192::_192Ehr)
    }
}
#[doc = "CRNGT exists.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Crngtexists {
    #[doc = "0: does not exist"]
    NoExist = 0,
    #[doc = "1: exists"]
    Exists = 1,
}
impl From<Crngtexists> for bool {
    #[inline(always)]
    fn from(variant: Crngtexists) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRNGTEXISTS` reader - CRNGT exists."]
pub type CrngtexistsR = crate::BitReader<Crngtexists>;
impl CrngtexistsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Crngtexists {
        match self.bits {
            false => Crngtexists::NoExist,
            true => Crngtexists::Exists,
        }
    }
    #[doc = "does not exist"]
    #[inline(always)]
    pub fn is_no_exist(&self) -> bool {
        *self == Crngtexists::NoExist
    }
    #[doc = "exists"]
    #[inline(always)]
    pub fn is_exists(&self) -> bool {
        *self == Crngtexists::Exists
    }
}
#[doc = "Field `CRNGTEXISTS` writer - CRNGT exists."]
pub type CrngtexistsW<'a, REG> = crate::BitWriter<'a, REG, Crngtexists>;
impl<'a, REG> CrngtexistsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "does not exist"]
    #[inline(always)]
    pub fn no_exist(self) -> &'a mut crate::W<REG> {
        self.variant(Crngtexists::NoExist)
    }
    #[doc = "exists"]
    #[inline(always)]
    pub fn exists(self) -> &'a mut crate::W<REG> {
        self.variant(Crngtexists::Exists)
    }
}
#[doc = "Auto correct exists.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Autocorrexists {
    #[doc = "0: does not exist"]
    NoExist = 0,
    #[doc = "1: exists"]
    Exists = 1,
}
impl From<Autocorrexists> for bool {
    #[inline(always)]
    fn from(variant: Autocorrexists) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUTOCORREXISTS` reader - Auto correct exists."]
pub type AutocorrexistsR = crate::BitReader<Autocorrexists>;
impl AutocorrexistsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Autocorrexists {
        match self.bits {
            false => Autocorrexists::NoExist,
            true => Autocorrexists::Exists,
        }
    }
    #[doc = "does not exist"]
    #[inline(always)]
    pub fn is_no_exist(&self) -> bool {
        *self == Autocorrexists::NoExist
    }
    #[doc = "exists"]
    #[inline(always)]
    pub fn is_exists(&self) -> bool {
        *self == Autocorrexists::Exists
    }
}
#[doc = "Field `AUTOCORREXISTS` writer - Auto correct exists."]
pub type AutocorrexistsW<'a, REG> = crate::BitWriter<'a, REG, Autocorrexists>;
impl<'a, REG> AutocorrexistsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "does not exist"]
    #[inline(always)]
    pub fn no_exist(self) -> &'a mut crate::W<REG> {
        self.variant(Autocorrexists::NoExist)
    }
    #[doc = "exists"]
    #[inline(always)]
    pub fn exists(self) -> &'a mut crate::W<REG> {
        self.variant(Autocorrexists::Exists)
    }
}
#[doc = "TRNG tests bypass enable.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trngtestsbypassen {
    #[doc = "0: trng tests bypass not enabled"]
    TrngNe = 0,
    #[doc = "1: trng tests bypass enabled"]
    TrngE = 1,
}
impl From<Trngtestsbypassen> for bool {
    #[inline(always)]
    fn from(variant: Trngtestsbypassen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRNGTESTSBYPASSEN` reader - TRNG tests bypass enable."]
pub type TrngtestsbypassenR = crate::BitReader<Trngtestsbypassen>;
impl TrngtestsbypassenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trngtestsbypassen {
        match self.bits {
            false => Trngtestsbypassen::TrngNe,
            true => Trngtestsbypassen::TrngE,
        }
    }
    #[doc = "trng tests bypass not enabled"]
    #[inline(always)]
    pub fn is_trng_ne(&self) -> bool {
        *self == Trngtestsbypassen::TrngNe
    }
    #[doc = "trng tests bypass enabled"]
    #[inline(always)]
    pub fn is_trng_e(&self) -> bool {
        *self == Trngtestsbypassen::TrngE
    }
}
#[doc = "Field `TRNGTESTSBYPASSEN` writer - TRNG tests bypass enable."]
pub type TrngtestsbypassenW<'a, REG> = crate::BitWriter<'a, REG, Trngtestsbypassen>;
impl<'a, REG> TrngtestsbypassenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "trng tests bypass not enabled"]
    #[inline(always)]
    pub fn trng_ne(self) -> &'a mut crate::W<REG> {
        self.variant(Trngtestsbypassen::TrngNe)
    }
    #[doc = "trng tests bypass enabled"]
    #[inline(always)]
    pub fn trng_e(self) -> &'a mut crate::W<REG> {
        self.variant(Trngtestsbypassen::TrngE)
    }
}
#[doc = "PRNG Exists.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Prngexists {
    #[doc = "0: does not exist"]
    NoExist = 0,
    #[doc = "1: exists"]
    Exists = 1,
}
impl From<Prngexists> for bool {
    #[inline(always)]
    fn from(variant: Prngexists) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRNGEXISTS` reader - PRNG Exists."]
pub type PrngexistsR = crate::BitReader<Prngexists>;
impl PrngexistsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Prngexists {
        match self.bits {
            false => Prngexists::NoExist,
            true => Prngexists::Exists,
        }
    }
    #[doc = "does not exist"]
    #[inline(always)]
    pub fn is_no_exist(&self) -> bool {
        *self == Prngexists::NoExist
    }
    #[doc = "exists"]
    #[inline(always)]
    pub fn is_exists(&self) -> bool {
        *self == Prngexists::Exists
    }
}
#[doc = "Field `PRNGEXISTS` writer - PRNG Exists."]
pub type PrngexistsW<'a, REG> = crate::BitWriter<'a, REG, Prngexists>;
impl<'a, REG> PrngexistsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "does not exist"]
    #[inline(always)]
    pub fn no_exist(self) -> &'a mut crate::W<REG> {
        self.variant(Prngexists::NoExist)
    }
    #[doc = "exists"]
    #[inline(always)]
    pub fn exists(self) -> &'a mut crate::W<REG> {
        self.variant(Prngexists::Exists)
    }
}
#[doc = "KAT exists.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Katexists {
    #[doc = "0: does not exist"]
    NoExist = 0,
    #[doc = "1: exists"]
    Exists = 1,
}
impl From<Katexists> for bool {
    #[inline(always)]
    fn from(variant: Katexists) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `KATEXISTS` reader - KAT exists."]
pub type KatexistsR = crate::BitReader<Katexists>;
impl KatexistsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Katexists {
        match self.bits {
            false => Katexists::NoExist,
            true => Katexists::Exists,
        }
    }
    #[doc = "does not exist"]
    #[inline(always)]
    pub fn is_no_exist(&self) -> bool {
        *self == Katexists::NoExist
    }
    #[doc = "exists"]
    #[inline(always)]
    pub fn is_exists(&self) -> bool {
        *self == Katexists::Exists
    }
}
#[doc = "Field `KATEXISTS` writer - KAT exists."]
pub type KatexistsW<'a, REG> = crate::BitWriter<'a, REG, Katexists>;
impl<'a, REG> KatexistsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "does not exist"]
    #[inline(always)]
    pub fn no_exist(self) -> &'a mut crate::W<REG> {
        self.variant(Katexists::NoExist)
    }
    #[doc = "exists"]
    #[inline(always)]
    pub fn exists(self) -> &'a mut crate::W<REG> {
        self.variant(Katexists::Exists)
    }
}
#[doc = "Reseeding exists.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reseedingexists {
    #[doc = "1: exists"]
    Exists = 1,
    #[doc = "0: Reseed does not exists"]
    Noreseed = 0,
}
impl From<Reseedingexists> for bool {
    #[inline(always)]
    fn from(variant: Reseedingexists) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESEEDINGEXISTS` reader - Reseeding exists."]
pub type ReseedingexistsR = crate::BitReader<Reseedingexists>;
impl ReseedingexistsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Reseedingexists {
        match self.bits {
            true => Reseedingexists::Exists,
            false => Reseedingexists::Noreseed,
        }
    }
    #[doc = "exists"]
    #[inline(always)]
    pub fn is_exists(&self) -> bool {
        *self == Reseedingexists::Exists
    }
    #[doc = "Reseed does not exists"]
    #[inline(always)]
    pub fn is_noreseed(&self) -> bool {
        *self == Reseedingexists::Noreseed
    }
}
#[doc = "Field `RESEEDINGEXISTS` writer - Reseeding exists."]
pub type ReseedingexistsW<'a, REG> = crate::BitWriter<'a, REG, Reseedingexists>;
impl<'a, REG> ReseedingexistsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "exists"]
    #[inline(always)]
    pub fn exists(self) -> &'a mut crate::W<REG> {
        self.variant(Reseedingexists::Exists)
    }
    #[doc = "Reseed does not exists"]
    #[inline(always)]
    pub fn noreseed(self) -> &'a mut crate::W<REG> {
        self.variant(Reseedingexists::Noreseed)
    }
}
#[doc = "RNG use 5 (or 20) SBOX AES\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rnguse5sboxes {
    #[doc = "0: 20 SBOX AES"]
    _20SboxAes = 0,
    #[doc = "1: 5 SBOX AES"]
    _5SboxAes = 1,
}
impl From<Rnguse5sboxes> for bool {
    #[inline(always)]
    fn from(variant: Rnguse5sboxes) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RNGUSE5SBOXES` reader - RNG use 5 (or 20) SBOX AES"]
pub type Rnguse5sboxesR = crate::BitReader<Rnguse5sboxes>;
impl Rnguse5sboxesR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rnguse5sboxes {
        match self.bits {
            false => Rnguse5sboxes::_20SboxAes,
            true => Rnguse5sboxes::_5SboxAes,
        }
    }
    #[doc = "20 SBOX AES"]
    #[inline(always)]
    pub fn is_20_sbox_aes(&self) -> bool {
        *self == Rnguse5sboxes::_20SboxAes
    }
    #[doc = "5 SBOX AES"]
    #[inline(always)]
    pub fn is_5_sbox_aes(&self) -> bool {
        *self == Rnguse5sboxes::_5SboxAes
    }
}
#[doc = "Field `RNGUSE5SBOXES` writer - RNG use 5 (or 20) SBOX AES"]
pub type Rnguse5sboxesW<'a, REG> = crate::BitWriter<'a, REG, Rnguse5sboxes>;
impl<'a, REG> Rnguse5sboxesW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "20 SBOX AES"]
    #[inline(always)]
    pub fn _20_sbox_aes(self) -> &'a mut crate::W<REG> {
        self.variant(Rnguse5sboxes::_20SboxAes)
    }
    #[doc = "5 SBOX AES"]
    #[inline(always)]
    pub fn _5_sbox_aes(self) -> &'a mut crate::W<REG> {
        self.variant(Rnguse5sboxes::_5SboxAes)
    }
}
impl R {
    #[doc = "Bit 0 - EHR width selection."]
    #[inline(always)]
    pub fn ehrwidth192(&self) -> Ehrwidth192R {
        Ehrwidth192R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CRNGT exists."]
    #[inline(always)]
    pub fn crngtexists(&self) -> CrngtexistsR {
        CrngtexistsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Auto correct exists."]
    #[inline(always)]
    pub fn autocorrexists(&self) -> AutocorrexistsR {
        AutocorrexistsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TRNG tests bypass enable."]
    #[inline(always)]
    pub fn trngtestsbypassen(&self) -> TrngtestsbypassenR {
        TrngtestsbypassenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PRNG Exists."]
    #[inline(always)]
    pub fn prngexists(&self) -> PrngexistsR {
        PrngexistsR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - KAT exists."]
    #[inline(always)]
    pub fn katexists(&self) -> KatexistsR {
        KatexistsR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Reseeding exists."]
    #[inline(always)]
    pub fn reseedingexists(&self) -> ReseedingexistsR {
        ReseedingexistsR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RNG use 5 (or 20) SBOX AES"]
    #[inline(always)]
    pub fn rnguse5sboxes(&self) -> Rnguse5sboxesR {
        Rnguse5sboxesR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EHR width selection."]
    #[inline(always)]
    #[must_use]
    pub fn ehrwidth192(&mut self) -> Ehrwidth192W<RngversionSpec> {
        Ehrwidth192W::new(self, 0)
    }
    #[doc = "Bit 1 - CRNGT exists."]
    #[inline(always)]
    #[must_use]
    pub fn crngtexists(&mut self) -> CrngtexistsW<RngversionSpec> {
        CrngtexistsW::new(self, 1)
    }
    #[doc = "Bit 2 - Auto correct exists."]
    #[inline(always)]
    #[must_use]
    pub fn autocorrexists(&mut self) -> AutocorrexistsW<RngversionSpec> {
        AutocorrexistsW::new(self, 2)
    }
    #[doc = "Bit 3 - TRNG tests bypass enable."]
    #[inline(always)]
    #[must_use]
    pub fn trngtestsbypassen(&mut self) -> TrngtestsbypassenW<RngversionSpec> {
        TrngtestsbypassenW::new(self, 3)
    }
    #[doc = "Bit 4 - PRNG Exists."]
    #[inline(always)]
    #[must_use]
    pub fn prngexists(&mut self) -> PrngexistsW<RngversionSpec> {
        PrngexistsW::new(self, 4)
    }
    #[doc = "Bit 5 - KAT exists."]
    #[inline(always)]
    #[must_use]
    pub fn katexists(&mut self) -> KatexistsW<RngversionSpec> {
        KatexistsW::new(self, 5)
    }
    #[doc = "Bit 6 - Reseeding exists."]
    #[inline(always)]
    #[must_use]
    pub fn reseedingexists(&mut self) -> ReseedingexistsW<RngversionSpec> {
        ReseedingexistsW::new(self, 6)
    }
    #[doc = "Bit 7 - RNG use 5 (or 20) SBOX AES"]
    #[inline(always)]
    #[must_use]
    pub fn rnguse5sboxes(&mut self) -> Rnguse5sboxesW<RngversionSpec> {
        Rnguse5sboxesW::new(self, 7)
    }
}
#[doc = "This register holds the RNG version\n\nYou can [`read`](crate::Reg::read) this register and get [`rngversion::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rngversion::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RngversionSpec;
impl crate::RegisterSpec for RngversionSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rngversion::R`](R) reader structure"]
impl crate::Readable for RngversionSpec {}
#[doc = "`write(|w| ..)` method takes [`rngversion::W`](W) writer structure"]
impl crate::Writable for RngversionSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RNGVERSION to value 0x0f"]
impl crate::Resettable for RngversionSpec {
    const RESET_VALUE: u32 = 0x0f;
}
