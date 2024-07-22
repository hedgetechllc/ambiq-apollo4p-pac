#[doc = "Register `SWPOR` reader"]
pub type R = crate::R<SwporSpec>;
#[doc = "Register `SWPOR` writer"]
pub type W = crate::W<SwporSpec>;
#[doc = "0xD4 generates a software POR reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Swporkey {
    #[doc = "212: Writing 0xD4 key value generates a software POR reset."]
    Keyvalue = 212,
    #[doc = "0: Default value."]
    Default = 0,
}
impl From<Swporkey> for u8 {
    #[inline(always)]
    fn from(variant: Swporkey) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Swporkey {
    type Ux = u8;
}
impl crate::IsEnum for Swporkey {}
#[doc = "Field `SWPORKEY` reader - 0xD4 generates a software POR reset."]
pub type SwporkeyR = crate::FieldReader<Swporkey>;
impl SwporkeyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Swporkey> {
        match self.bits {
            212 => Some(Swporkey::Keyvalue),
            0 => Some(Swporkey::Default),
            _ => None,
        }
    }
    #[doc = "Writing 0xD4 key value generates a software POR reset."]
    #[inline(always)]
    pub fn is_keyvalue(&self) -> bool {
        *self == Swporkey::Keyvalue
    }
    #[doc = "Default value."]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == Swporkey::Default
    }
}
#[doc = "Field `SWPORKEY` writer - 0xD4 generates a software POR reset."]
pub type SwporkeyW<'a, REG> = crate::FieldWriter<'a, REG, 8, Swporkey>;
impl<'a, REG> SwporkeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Writing 0xD4 key value generates a software POR reset."]
    #[inline(always)]
    pub fn keyvalue(self) -> &'a mut crate::W<REG> {
        self.variant(Swporkey::Keyvalue)
    }
    #[doc = "Default value."]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(Swporkey::Default)
    }
}
impl R {
    #[doc = "Bits 0:7 - 0xD4 generates a software POR reset."]
    #[inline(always)]
    pub fn swporkey(&self) -> SwporkeyR {
        SwporkeyR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 0xD4 generates a software POR reset."]
    #[inline(always)]
    #[must_use]
    pub fn swporkey(&mut self) -> SwporkeyW<SwporSpec> {
        SwporkeyW::new(self, 0)
    }
}
#[doc = "This is the software POR reset. Writing the key value to this register will trigger a POR to the system. This will cause a reset to all blocks except for registers in clock gen, RTC, power management unit, the stimer, and the power management unit.\n\nYou can [`read`](crate::Reg::read) this register and get [`swpor::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swpor::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwporSpec;
impl crate::RegisterSpec for SwporSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swpor::R`](R) reader structure"]
impl crate::Readable for SwporSpec {}
#[doc = "`write(|w| ..)` method takes [`swpor::W`](W) writer structure"]
impl crate::Writable for SwporSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWPOR to value 0"]
impl crate::Resettable for SwporSpec {
    const RESET_VALUE: u32 = 0;
}
