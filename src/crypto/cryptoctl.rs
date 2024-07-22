#[doc = "Register `CRYPTOCTL` reader"]
pub type R = crate::R<CryptoctlSpec>;
#[doc = "Register `CRYPTOCTL` writer"]
pub type W = crate::W<CryptoctlSpec>;
#[doc = "Determines the active cryptographic engine:\n\nValue on reset: 15"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode {
    #[doc = "0: bypass"]
    Bypass = 0,
    #[doc = "1: aes"]
    Aes = 1,
    #[doc = "2: aes to hash"]
    AesToHash = 2,
    #[doc = "3: aes and hash"]
    AesAndHash = 3,
    #[doc = "4: des"]
    Des = 4,
    #[doc = "5: des to hash"]
    DesToHash = 5,
    #[doc = "6: des and hash"]
    DesAndHash = 6,
    #[doc = "7: hash"]
    Hash = 7,
    #[doc = "9: aes mac and bypass"]
    AesMacAndBypass = 9,
    #[doc = "10: aes to hash and _dout"]
    AesToHashAndDout = 10,
    #[doc = "11: reserved1"]
    Reserved1 = 11,
    #[doc = "8: reserved2"]
    Reserved2 = 8,
}
impl From<Mode> for u8 {
    #[inline(always)]
    fn from(variant: Mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mode {
    type Ux = u8;
}
impl crate::IsEnum for Mode {}
#[doc = "Field `MODE` reader - Determines the active cryptographic engine:"]
pub type ModeR = crate::FieldReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Mode> {
        match self.bits {
            0 => Some(Mode::Bypass),
            1 => Some(Mode::Aes),
            2 => Some(Mode::AesToHash),
            3 => Some(Mode::AesAndHash),
            4 => Some(Mode::Des),
            5 => Some(Mode::DesToHash),
            6 => Some(Mode::DesAndHash),
            7 => Some(Mode::Hash),
            9 => Some(Mode::AesMacAndBypass),
            10 => Some(Mode::AesToHashAndDout),
            11 => Some(Mode::Reserved1),
            8 => Some(Mode::Reserved2),
            _ => None,
        }
    }
    #[doc = "bypass"]
    #[inline(always)]
    pub fn is_bypass(&self) -> bool {
        *self == Mode::Bypass
    }
    #[doc = "aes"]
    #[inline(always)]
    pub fn is_aes(&self) -> bool {
        *self == Mode::Aes
    }
    #[doc = "aes to hash"]
    #[inline(always)]
    pub fn is_aes_to_hash(&self) -> bool {
        *self == Mode::AesToHash
    }
    #[doc = "aes and hash"]
    #[inline(always)]
    pub fn is_aes_and_hash(&self) -> bool {
        *self == Mode::AesAndHash
    }
    #[doc = "des"]
    #[inline(always)]
    pub fn is_des(&self) -> bool {
        *self == Mode::Des
    }
    #[doc = "des to hash"]
    #[inline(always)]
    pub fn is_des_to_hash(&self) -> bool {
        *self == Mode::DesToHash
    }
    #[doc = "des and hash"]
    #[inline(always)]
    pub fn is_des_and_hash(&self) -> bool {
        *self == Mode::DesAndHash
    }
    #[doc = "hash"]
    #[inline(always)]
    pub fn is_hash(&self) -> bool {
        *self == Mode::Hash
    }
    #[doc = "aes mac and bypass"]
    #[inline(always)]
    pub fn is_aes_mac_and_bypass(&self) -> bool {
        *self == Mode::AesMacAndBypass
    }
    #[doc = "aes to hash and _dout"]
    #[inline(always)]
    pub fn is_aes_to_hash_and_dout(&self) -> bool {
        *self == Mode::AesToHashAndDout
    }
    #[doc = "reserved1"]
    #[inline(always)]
    pub fn is_reserved1(&self) -> bool {
        *self == Mode::Reserved1
    }
    #[doc = "reserved2"]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == Mode::Reserved2
    }
}
#[doc = "Field `MODE` writer - Determines the active cryptographic engine:"]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 5, Mode>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "bypass"]
    #[inline(always)]
    pub fn bypass(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Bypass)
    }
    #[doc = "aes"]
    #[inline(always)]
    pub fn aes(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Aes)
    }
    #[doc = "aes to hash"]
    #[inline(always)]
    pub fn aes_to_hash(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::AesToHash)
    }
    #[doc = "aes and hash"]
    #[inline(always)]
    pub fn aes_and_hash(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::AesAndHash)
    }
    #[doc = "des"]
    #[inline(always)]
    pub fn des(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Des)
    }
    #[doc = "des to hash"]
    #[inline(always)]
    pub fn des_to_hash(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::DesToHash)
    }
    #[doc = "des and hash"]
    #[inline(always)]
    pub fn des_and_hash(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::DesAndHash)
    }
    #[doc = "hash"]
    #[inline(always)]
    pub fn hash(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Hash)
    }
    #[doc = "aes mac and bypass"]
    #[inline(always)]
    pub fn aes_mac_and_bypass(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::AesMacAndBypass)
    }
    #[doc = "aes to hash and _dout"]
    #[inline(always)]
    pub fn aes_to_hash_and_dout(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::AesToHashAndDout)
    }
    #[doc = "reserved1"]
    #[inline(always)]
    pub fn reserved1(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Reserved1)
    }
    #[doc = "reserved2"]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Reserved2)
    }
}
impl R {
    #[doc = "Bits 0:4 - Determines the active cryptographic engine:"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Determines the active cryptographic engine:"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> ModeW<CryptoctlSpec> {
        ModeW::new(self, 0)
    }
}
#[doc = "Defines the cryptographic flow.\n\nYou can [`read`](crate::Reg::read) this register and get [`cryptoctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cryptoctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CryptoctlSpec;
impl crate::RegisterSpec for CryptoctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cryptoctl::R`](R) reader structure"]
impl crate::Readable for CryptoctlSpec {}
#[doc = "`write(|w| ..)` method takes [`cryptoctl::W`](W) writer structure"]
impl crate::Writable for CryptoctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRYPTOCTL to value 0x0f"]
impl crate::Resettable for CryptoctlSpec {
    const RESET_VALUE: u32 = 0x0f;
}
