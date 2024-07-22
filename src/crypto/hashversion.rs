#[doc = "Register `HASHVERSION` reader"]
pub type R = crate::R<HashversionSpec>;
#[doc = "Register `HASHVERSION` writer"]
pub type W = crate::W<HashversionSpec>;
#[doc = "Field `FIXES` reader - Fixes field."]
pub type FixesR = crate::FieldReader;
#[doc = "Field `FIXES` writer - Fixes field."]
pub type FixesW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MINORVERSIONNUMBER` reader - minor version number"]
pub type MinorversionnumberR = crate::FieldReader;
#[doc = "Field `MINORVERSIONNUMBER` writer - minor version number"]
pub type MinorversionnumberW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MAJORVERSIONNUMBER` reader - major version number"]
pub type MajorversionnumberR = crate::FieldReader;
#[doc = "Field `MAJORVERSIONNUMBER` writer - major version number"]
pub type MajorversionnumberW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:7 - Fixes field."]
    #[inline(always)]
    pub fn fixes(&self) -> FixesR {
        FixesR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - minor version number"]
    #[inline(always)]
    pub fn minorversionnumber(&self) -> MinorversionnumberR {
        MinorversionnumberR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - major version number"]
    #[inline(always)]
    pub fn majorversionnumber(&self) -> MajorversionnumberR {
        MajorversionnumberR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Fixes field."]
    #[inline(always)]
    #[must_use]
    pub fn fixes(&mut self) -> FixesW<HashversionSpec> {
        FixesW::new(self, 0)
    }
    #[doc = "Bits 8:11 - minor version number"]
    #[inline(always)]
    #[must_use]
    pub fn minorversionnumber(&mut self) -> MinorversionnumberW<HashversionSpec> {
        MinorversionnumberW::new(self, 8)
    }
    #[doc = "Bits 12:15 - major version number"]
    #[inline(always)]
    #[must_use]
    pub fn majorversionnumber(&mut self) -> MajorversionnumberW<HashversionSpec> {
        MajorversionnumberW::new(self, 12)
    }
}
#[doc = "HASH VERSION Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hashversion::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hashversion::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HashversionSpec;
impl crate::RegisterSpec for HashversionSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hashversion::R`](R) reader structure"]
impl crate::Readable for HashversionSpec {}
#[doc = "`write(|w| ..)` method takes [`hashversion::W`](W) writer structure"]
impl crate::Writable for HashversionSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HASHVERSION to value 0"]
impl crate::Resettable for HashversionSpec {
    const RESET_VALUE: u32 = 0;
}
