#[doc = "Register `SWPOI` reader"]
pub type R = crate::R<SwpoiSpec>;
#[doc = "Register `SWPOI` writer"]
pub type W = crate::W<SwpoiSpec>;
#[doc = "0x1B generates a software POI reset. This is a write-only register. Reading from this register will yield only all 0s.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Swpoikey {
    #[doc = "27: Writing 0x1B key value generates a software POI reset."]
    Keyvalue = 27,
    #[doc = "0: Default value."]
    Default = 0,
}
impl From<Swpoikey> for u8 {
    #[inline(always)]
    fn from(variant: Swpoikey) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Swpoikey {
    type Ux = u8;
}
impl crate::IsEnum for Swpoikey {}
#[doc = "Field `SWPOIKEY` reader - 0x1B generates a software POI reset. This is a write-only register. Reading from this register will yield only all 0s."]
pub type SwpoikeyR = crate::FieldReader<Swpoikey>;
impl SwpoikeyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Swpoikey> {
        match self.bits {
            27 => Some(Swpoikey::Keyvalue),
            0 => Some(Swpoikey::Default),
            _ => None,
        }
    }
    #[doc = "Writing 0x1B key value generates a software POI reset."]
    #[inline(always)]
    pub fn is_keyvalue(&self) -> bool {
        *self == Swpoikey::Keyvalue
    }
    #[doc = "Default value."]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == Swpoikey::Default
    }
}
#[doc = "Field `SWPOIKEY` writer - 0x1B generates a software POI reset. This is a write-only register. Reading from this register will yield only all 0s."]
pub type SwpoikeyW<'a, REG> = crate::FieldWriter<'a, REG, 8, Swpoikey>;
impl<'a, REG> SwpoikeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Writing 0x1B key value generates a software POI reset."]
    #[inline(always)]
    pub fn keyvalue(self) -> &'a mut crate::W<REG> {
        self.variant(Swpoikey::Keyvalue)
    }
    #[doc = "Default value."]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(Swpoikey::Default)
    }
}
impl R {
    #[doc = "Bits 0:7 - 0x1B generates a software POI reset. This is a write-only register. Reading from this register will yield only all 0s."]
    #[inline(always)]
    pub fn swpoikey(&self) -> SwpoikeyR {
        SwpoikeyR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 0x1B generates a software POI reset. This is a write-only register. Reading from this register will yield only all 0s."]
    #[inline(always)]
    #[must_use]
    pub fn swpoikey(&mut self) -> SwpoikeyW<SwpoiSpec> {
        SwpoikeyW::new(self, 0)
    }
}
#[doc = "This is the software POI reset. writing the key value to this register will trigger a POI to the system. This will cause a reset to all blocks except for registers in clock gen, RTC and the stimer.\n\nYou can [`read`](crate::Reg::read) this register and get [`swpoi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swpoi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwpoiSpec;
impl crate::RegisterSpec for SwpoiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swpoi::R`](R) reader structure"]
impl crate::Readable for SwpoiSpec {}
#[doc = "`write(|w| ..)` method takes [`swpoi::W`](W) writer structure"]
impl crate::Writable for SwpoiSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWPOI to value 0"]
impl crate::Resettable for SwpoiSpec {
    const RESET_VALUE: u32 = 0;
}
