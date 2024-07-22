#[doc = "Register `HASHPARAM` reader"]
pub type R = crate::R<HashparamSpec>;
#[doc = "Register `HASHPARAM` writer"]
pub type W = crate::W<HashparamSpec>;
#[doc = "Field `CW` reader - Indicates the number of concurrent words the hash is using to compute signature. 1 - One concurrent w(t). 2 - Two concurrent w(t)."]
pub type CwR = crate::FieldReader;
#[doc = "Field `CW` writer - Indicates the number of concurrent words the hash is using to compute signature. 1 - One concurrent w(t). 2 - Two concurrent w(t)."]
pub type CwW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CH` reader - Indicate if Hi adders are present for each Hi value or 1 adder is shared for all Hi. 0 - One Hi value is updated at a time 1 - All Hi values are updated at the same time."]
pub type ChR = crate::FieldReader;
#[doc = "Field `CH` writer - Indicate if Hi adders are present for each Hi value or 1 adder is shared for all Hi. 0 - One Hi value is updated at a time 1 - All Hi values are updated at the same time."]
pub type ChW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DW` reader - Determine the granularity of word size. 0 - 32 bit word data. 1 - 64 bit word data."]
pub type DwR = crate::FieldReader;
#[doc = "Field `DW` writer - Determine the granularity of word size. 0 - 32 bit word data. 1 - 64 bit word data."]
pub type DwW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SHA512EXISTS` reader - Indicate if SHA-512 is present in the design. By default SHA-1 and SHA-256 are present. 0 - SHA-1 and SHA-256 are present only 1 - SHA-1 and all SHA-2 are present (SHA-256 SHA-512)."]
pub type Sha512existsR = crate::BitReader;
#[doc = "Field `SHA512EXISTS` writer - Indicate if SHA-512 is present in the design. By default SHA-1 and SHA-256 are present. 0 - SHA-1 and SHA-256 are present only 1 - SHA-1 and all SHA-2 are present (SHA-256 SHA-512)."]
pub type Sha512existsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADEXISTS` reader - Indicate if pad block is present in the design. 0 - pad function is not supported by hardware. 1 - pad function is supported by hardware."]
pub type PadexistsR = crate::BitReader;
#[doc = "Field `PADEXISTS` writer - Indicate if pad block is present in the design. 0 - pad function is not supported by hardware. 1 - pad function is supported by hardware."]
pub type PadexistsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MD5EXISTS` reader - Indicate if MD5 is present in HW"]
pub type Md5existsR = crate::BitReader;
#[doc = "Field `MD5EXISTS` writer - Indicate if MD5 is present in HW"]
pub type Md5existsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HMACEXISTS` reader - Indicate if HMAC logic is present in the design"]
pub type HmacexistsR = crate::BitReader;
#[doc = "Field `HMACEXISTS` writer - Indicate if HMAC logic is present in the design"]
pub type HmacexistsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHA256EXISTS` reader - Indicate if SHA-256 is present in the design"]
pub type Sha256existsR = crate::BitReader;
#[doc = "Field `SHA256EXISTS` writer - Indicate if SHA-256 is present in the design"]
pub type Sha256existsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HASHCOMPAREEXISTS` reader - Indicate if COMPARE digest logic is present in the design"]
pub type HashcompareexistsR = crate::BitReader;
#[doc = "Field `HASHCOMPAREEXISTS` writer - Indicate if COMPARE digest logic is present in the design"]
pub type HashcompareexistsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUMPHASHTODOUTEXISTS` reader - Indicate if HASH to dout is present in the design"]
pub type DumphashtodoutexistsR = crate::BitReader;
#[doc = "Field `DUMPHASHTODOUTEXISTS` writer - Indicate if HASH to dout is present in the design"]
pub type DumphashtodoutexistsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Indicates the number of concurrent words the hash is using to compute signature. 1 - One concurrent w(t). 2 - Two concurrent w(t)."]
    #[inline(always)]
    pub fn cw(&self) -> CwR {
        CwR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Indicate if Hi adders are present for each Hi value or 1 adder is shared for all Hi. 0 - One Hi value is updated at a time 1 - All Hi values are updated at the same time."]
    #[inline(always)]
    pub fn ch(&self) -> ChR {
        ChR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Determine the granularity of word size. 0 - 32 bit word data. 1 - 64 bit word data."]
    #[inline(always)]
    pub fn dw(&self) -> DwR {
        DwR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - Indicate if SHA-512 is present in the design. By default SHA-1 and SHA-256 are present. 0 - SHA-1 and SHA-256 are present only 1 - SHA-1 and all SHA-2 are present (SHA-256 SHA-512)."]
    #[inline(always)]
    pub fn sha512exists(&self) -> Sha512existsR {
        Sha512existsR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Indicate if pad block is present in the design. 0 - pad function is not supported by hardware. 1 - pad function is supported by hardware."]
    #[inline(always)]
    pub fn padexists(&self) -> PadexistsR {
        PadexistsR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Indicate if MD5 is present in HW"]
    #[inline(always)]
    pub fn md5exists(&self) -> Md5existsR {
        Md5existsR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Indicate if HMAC logic is present in the design"]
    #[inline(always)]
    pub fn hmacexists(&self) -> HmacexistsR {
        HmacexistsR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Indicate if SHA-256 is present in the design"]
    #[inline(always)]
    pub fn sha256exists(&self) -> Sha256existsR {
        Sha256existsR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Indicate if COMPARE digest logic is present in the design"]
    #[inline(always)]
    pub fn hashcompareexists(&self) -> HashcompareexistsR {
        HashcompareexistsR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Indicate if HASH to dout is present in the design"]
    #[inline(always)]
    pub fn dumphashtodoutexists(&self) -> DumphashtodoutexistsR {
        DumphashtodoutexistsR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Indicates the number of concurrent words the hash is using to compute signature. 1 - One concurrent w(t). 2 - Two concurrent w(t)."]
    #[inline(always)]
    #[must_use]
    pub fn cw(&mut self) -> CwW<HashparamSpec> {
        CwW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Indicate if Hi adders are present for each Hi value or 1 adder is shared for all Hi. 0 - One Hi value is updated at a time 1 - All Hi values are updated at the same time."]
    #[inline(always)]
    #[must_use]
    pub fn ch(&mut self) -> ChW<HashparamSpec> {
        ChW::new(self, 4)
    }
    #[doc = "Bits 8:11 - Determine the granularity of word size. 0 - 32 bit word data. 1 - 64 bit word data."]
    #[inline(always)]
    #[must_use]
    pub fn dw(&mut self) -> DwW<HashparamSpec> {
        DwW::new(self, 8)
    }
    #[doc = "Bit 12 - Indicate if SHA-512 is present in the design. By default SHA-1 and SHA-256 are present. 0 - SHA-1 and SHA-256 are present only 1 - SHA-1 and all SHA-2 are present (SHA-256 SHA-512)."]
    #[inline(always)]
    #[must_use]
    pub fn sha512exists(&mut self) -> Sha512existsW<HashparamSpec> {
        Sha512existsW::new(self, 12)
    }
    #[doc = "Bit 13 - Indicate if pad block is present in the design. 0 - pad function is not supported by hardware. 1 - pad function is supported by hardware."]
    #[inline(always)]
    #[must_use]
    pub fn padexists(&mut self) -> PadexistsW<HashparamSpec> {
        PadexistsW::new(self, 13)
    }
    #[doc = "Bit 14 - Indicate if MD5 is present in HW"]
    #[inline(always)]
    #[must_use]
    pub fn md5exists(&mut self) -> Md5existsW<HashparamSpec> {
        Md5existsW::new(self, 14)
    }
    #[doc = "Bit 15 - Indicate if HMAC logic is present in the design"]
    #[inline(always)]
    #[must_use]
    pub fn hmacexists(&mut self) -> HmacexistsW<HashparamSpec> {
        HmacexistsW::new(self, 15)
    }
    #[doc = "Bit 16 - Indicate if SHA-256 is present in the design"]
    #[inline(always)]
    #[must_use]
    pub fn sha256exists(&mut self) -> Sha256existsW<HashparamSpec> {
        Sha256existsW::new(self, 16)
    }
    #[doc = "Bit 17 - Indicate if COMPARE digest logic is present in the design"]
    #[inline(always)]
    #[must_use]
    pub fn hashcompareexists(&mut self) -> HashcompareexistsW<HashparamSpec> {
        HashcompareexistsW::new(self, 17)
    }
    #[doc = "Bit 18 - Indicate if HASH to dout is present in the design"]
    #[inline(always)]
    #[must_use]
    pub fn dumphashtodoutexists(&mut self) -> DumphashtodoutexistsW<HashparamSpec> {
        DumphashtodoutexistsW::new(self, 18)
    }
}
#[doc = "HASH_PARAM Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`hashparam::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hashparam::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HashparamSpec;
impl crate::RegisterSpec for HashparamSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hashparam::R`](R) reader structure"]
impl crate::Readable for HashparamSpec {}
#[doc = "`write(|w| ..)` method takes [`hashparam::W`](W) writer structure"]
impl crate::Writable for HashparamSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HASHPARAM to value 0x0001_2001"]
impl crate::Resettable for HashparamSpec {
    const RESET_VALUE: u32 = 0x0001_2001;
}
