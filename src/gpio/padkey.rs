#[doc = "Register `PADKEY` reader"]
pub type R = crate::R<PadkeySpec>;
#[doc = "Register `PADKEY` writer"]
pub type W = crate::W<PadkeySpec>;
#[doc = "Key register value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Padkey {
    #[doc = "115: Key value to unlock the register."]
    Key = 115,
}
impl From<Padkey> for u32 {
    #[inline(always)]
    fn from(variant: Padkey) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Padkey {
    type Ux = u32;
}
impl crate::IsEnum for Padkey {}
#[doc = "Field `PADKEY` reader - Key register value."]
pub type PadkeyR = crate::FieldReader<Padkey>;
impl PadkeyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Padkey> {
        match self.bits {
            115 => Some(Padkey::Key),
            _ => None,
        }
    }
    #[doc = "Key value to unlock the register."]
    #[inline(always)]
    pub fn is_key(&self) -> bool {
        *self == Padkey::Key
    }
}
#[doc = "Field `PADKEY` writer - Key register value."]
pub type PadkeyW<'a, REG> = crate::FieldWriter<'a, REG, 32, Padkey>;
impl<'a, REG> PadkeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Key value to unlock the register."]
    #[inline(always)]
    pub fn key(self) -> &'a mut crate::W<REG> {
        self.variant(Padkey::Key)
    }
}
impl R {
    #[doc = "Bits 0:31 - Key register value."]
    #[inline(always)]
    pub fn padkey(&self) -> PadkeyR {
        PadkeyR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Key register value."]
    #[inline(always)]
    #[must_use]
    pub fn padkey(&mut self) -> PadkeyW<PadkeySpec> {
        PadkeyW::new(self, 0)
    }
}
#[doc = "Lock state of the PINCFG and GPIO configuration registers. Write a value of 0x73 to unlock write access to the PAD and GPIO.\n\nYou can [`read`](crate::Reg::read) this register and get [`padkey::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`padkey::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PadkeySpec;
impl crate::RegisterSpec for PadkeySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`padkey::R`](R) reader structure"]
impl crate::Readable for PadkeySpec {}
#[doc = "`write(|w| ..)` method takes [`padkey::W`](W) writer structure"]
impl crate::Writable for PadkeySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PADKEY to value 0"]
impl crate::Resettable for PadkeySpec {
    const RESET_VALUE: u32 = 0;
}
