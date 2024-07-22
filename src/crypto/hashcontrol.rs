#[doc = "Register `HASHCONTROL` reader"]
pub type R = crate::R<HashcontrolSpec>;
#[doc = "Register `HASHCONTROL` writer"]
pub type W = crate::W<HashcontrolSpec>;
#[doc = "bits 1:0 of the HASH mode field. The hash mode field possible values are:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode01 {
    #[doc = "0: MD5 if present"]
    Md5 = 0,
    #[doc = "1: SHA-1"]
    Sha1 = 1,
    #[doc = "2: SHA-256"]
    Sha256 = 2,
}
impl From<Mode01> for u8 {
    #[inline(always)]
    fn from(variant: Mode01) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mode01 {
    type Ux = u8;
}
impl crate::IsEnum for Mode01 {}
#[doc = "Field `MODE01` reader - bits 1:0 of the HASH mode field. The hash mode field possible values are:"]
pub type Mode01R = crate::FieldReader<Mode01>;
impl Mode01R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Mode01> {
        match self.bits {
            0 => Some(Mode01::Md5),
            1 => Some(Mode01::Sha1),
            2 => Some(Mode01::Sha256),
            _ => None,
        }
    }
    #[doc = "MD5 if present"]
    #[inline(always)]
    pub fn is_md5(&self) -> bool {
        *self == Mode01::Md5
    }
    #[doc = "SHA-1"]
    #[inline(always)]
    pub fn is_sha_1(&self) -> bool {
        *self == Mode01::Sha1
    }
    #[doc = "SHA-256"]
    #[inline(always)]
    pub fn is_sha_256(&self) -> bool {
        *self == Mode01::Sha256
    }
}
#[doc = "Field `MODE01` writer - bits 1:0 of the HASH mode field. The hash mode field possible values are:"]
pub type Mode01W<'a, REG> = crate::FieldWriter<'a, REG, 2, Mode01>;
impl<'a, REG> Mode01W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "MD5 if present"]
    #[inline(always)]
    pub fn md5(self) -> &'a mut crate::W<REG> {
        self.variant(Mode01::Md5)
    }
    #[doc = "SHA-1"]
    #[inline(always)]
    pub fn sha_1(self) -> &'a mut crate::W<REG> {
        self.variant(Mode01::Sha1)
    }
    #[doc = "SHA-256"]
    #[inline(always)]
    pub fn sha_256(self) -> &'a mut crate::W<REG> {
        self.variant(Mode01::Sha256)
    }
}
#[doc = "Field `MODE3` reader - bit 3 of the HASH mode field. The hash mode field possible values are:4b0000 - MD5 if present 0x0001 SHA 1 4b0010 - SHA-256 4b1010 - SHA-224"]
pub type Mode3R = crate::BitReader;
#[doc = "Field `MODE3` writer - bit 3 of the HASH mode field. The hash mode field possible values are:4b0000 - MD5 if present 0x0001 SHA 1 4b0010 - SHA-256 4b1010 - SHA-224"]
pub type Mode3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - bits 1:0 of the HASH mode field. The hash mode field possible values are:"]
    #[inline(always)]
    pub fn mode01(&self) -> Mode01R {
        Mode01R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - bit 3 of the HASH mode field. The hash mode field possible values are:4b0000 - MD5 if present 0x0001 SHA 1 4b0010 - SHA-256 4b1010 - SHA-224"]
    #[inline(always)]
    pub fn mode3(&self) -> Mode3R {
        Mode3R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - bits 1:0 of the HASH mode field. The hash mode field possible values are:"]
    #[inline(always)]
    #[must_use]
    pub fn mode01(&mut self) -> Mode01W<HashcontrolSpec> {
        Mode01W::new(self, 0)
    }
    #[doc = "Bit 3 - bit 3 of the HASH mode field. The hash mode field possible values are:4b0000 - MD5 if present 0x0001 SHA 1 4b0010 - SHA-256 4b1010 - SHA-224"]
    #[inline(always)]
    #[must_use]
    pub fn mode3(&mut self) -> Mode3W<HashcontrolSpec> {
        Mode3W::new(self, 3)
    }
}
#[doc = "Selects which HASH mode to run\n\nYou can [`read`](crate::Reg::read) this register and get [`hashcontrol::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hashcontrol::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HashcontrolSpec;
impl crate::RegisterSpec for HashcontrolSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hashcontrol::R`](R) reader structure"]
impl crate::Readable for HashcontrolSpec {}
#[doc = "`write(|w| ..)` method takes [`hashcontrol::W`](W) writer structure"]
impl crate::Writable for HashcontrolSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HASHCONTROL to value 0"]
impl crate::Resettable for HashcontrolSpec {
    const RESET_VALUE: u32 = 0;
}
