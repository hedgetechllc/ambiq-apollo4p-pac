#[doc = "Register `PWDKEY` reader"]
pub type R = crate::R<PwdkeySpec>;
#[doc = "Register `PWDKEY` writer"]
pub type W = crate::W<PwdkeySpec>;
#[doc = "Key register value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Pwdkey {
    #[doc = "55: Key value to unlock the register."]
    Key = 55,
}
impl From<Pwdkey> for u32 {
    #[inline(always)]
    fn from(variant: Pwdkey) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pwdkey {
    type Ux = u32;
}
impl crate::IsEnum for Pwdkey {}
#[doc = "Field `PWDKEY` reader - Key register value."]
pub type PwdkeyR = crate::FieldReader<Pwdkey>;
impl PwdkeyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pwdkey> {
        match self.bits {
            55 => Some(Pwdkey::Key),
            _ => None,
        }
    }
    #[doc = "Key value to unlock the register."]
    #[inline(always)]
    pub fn is_key(&self) -> bool {
        *self == Pwdkey::Key
    }
}
#[doc = "Field `PWDKEY` writer - Key register value."]
pub type PwdkeyW<'a, REG> = crate::FieldWriter<'a, REG, 32, Pwdkey>;
impl<'a, REG> PwdkeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Key value to unlock the register."]
    #[inline(always)]
    pub fn key(self) -> &'a mut crate::W<REG> {
        self.variant(Pwdkey::Key)
    }
}
impl R {
    #[doc = "Bits 0:31 - Key register value."]
    #[inline(always)]
    pub fn pwdkey(&self) -> PwdkeyR {
        PwdkeyR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Key register value."]
    #[inline(always)]
    #[must_use]
    pub fn pwdkey(&mut self) -> PwdkeyW<PwdkeySpec> {
        PwdkeyW::new(self, 0)
    }
}
#[doc = "Write a value of 0x37 to unlock, write any other value to lock. This register also indicates lock status when read. When in the unlccked state (i.e. 0x37 has been written), it reads as 1. When in the locked state, it reads as 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`pwdkey::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwdkey::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwdkeySpec;
impl crate::RegisterSpec for PwdkeySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwdkey::R`](R) reader structure"]
impl crate::Readable for PwdkeySpec {}
#[doc = "`write(|w| ..)` method takes [`pwdkey::W`](W) writer structure"]
impl crate::Writable for PwdkeySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWDKEY to value 0"]
impl crate::Resettable for PwdkeySpec {
    const RESET_VALUE: u32 = 0;
}
