#[doc = "Register `SHADOWVALID` reader"]
pub type R = crate::R<ShadowvalidSpec>;
#[doc = "Register `SHADOWVALID` writer"]
pub type W = crate::W<ShadowvalidSpec>;
#[doc = "Indicates whether the shadow registers contain valid data from the Flash Information Space.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Valid {
    #[doc = "1: Flash information space contains valid data."]
    Valid = 1,
    #[doc = "0: Default value."]
    Default = 0,
}
impl From<Valid> for bool {
    #[inline(always)]
    fn from(variant: Valid) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VALID` reader - Indicates whether the shadow registers contain valid data from the Flash Information Space."]
pub type ValidR = crate::BitReader<Valid>;
impl ValidR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Valid {
        match self.bits {
            true => Valid::Valid,
            false => Valid::Default,
        }
    }
    #[doc = "Flash information space contains valid data."]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == Valid::Valid
    }
    #[doc = "Default value."]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == Valid::Default
    }
}
#[doc = "Field `VALID` writer - Indicates whether the shadow registers contain valid data from the Flash Information Space."]
pub type ValidW<'a, REG> = crate::BitWriter<'a, REG, Valid>;
impl<'a, REG> ValidW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flash information space contains valid data."]
    #[inline(always)]
    pub fn valid(self) -> &'a mut crate::W<REG> {
        self.variant(Valid::Valid)
    }
    #[doc = "Default value."]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(Valid::Default)
    }
}
#[doc = "Indicates whether the bootloader should sleep or deep sleep if no image loaded.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bldsleep {
    #[doc = "1: Bootloader will go to deep sleep if no flash image loaded"]
    Deepsleep = 1,
    #[doc = "0: Bootloader will go to normalsleep if no flash image loaded"]
    Sleep = 0,
}
impl From<Bldsleep> for bool {
    #[inline(always)]
    fn from(variant: Bldsleep) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BLDSLEEP` reader - Indicates whether the bootloader should sleep or deep sleep if no image loaded."]
pub type BldsleepR = crate::BitReader<Bldsleep>;
impl BldsleepR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bldsleep {
        match self.bits {
            true => Bldsleep::Deepsleep,
            false => Bldsleep::Sleep,
        }
    }
    #[doc = "Bootloader will go to deep sleep if no flash image loaded"]
    #[inline(always)]
    pub fn is_deepsleep(&self) -> bool {
        *self == Bldsleep::Deepsleep
    }
    #[doc = "Bootloader will go to normalsleep if no flash image loaded"]
    #[inline(always)]
    pub fn is_sleep(&self) -> bool {
        *self == Bldsleep::Sleep
    }
}
#[doc = "Field `BLDSLEEP` writer - Indicates whether the bootloader should sleep or deep sleep if no image loaded."]
pub type BldsleepW<'a, REG> = crate::BitWriter<'a, REG, Bldsleep>;
impl<'a, REG> BldsleepW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bootloader will go to deep sleep if no flash image loaded"]
    #[inline(always)]
    pub fn deepsleep(self) -> &'a mut crate::W<REG> {
        self.variant(Bldsleep::Deepsleep)
    }
    #[doc = "Bootloader will go to normalsleep if no flash image loaded"]
    #[inline(always)]
    pub fn sleep(self) -> &'a mut crate::W<REG> {
        self.variant(Bldsleep::Sleep)
    }
}
#[doc = "Indicates whether info0 contains valid data\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Info0valid {
    #[doc = "1: Flash info0 (customer) space contains valid data."]
    Valid = 1,
    #[doc = "0: Default value."]
    Default = 0,
}
impl From<Info0valid> for bool {
    #[inline(always)]
    fn from(variant: Info0valid) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INFO0VALID` reader - Indicates whether info0 contains valid data"]
pub type Info0validR = crate::BitReader<Info0valid>;
impl Info0validR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Info0valid {
        match self.bits {
            true => Info0valid::Valid,
            false => Info0valid::Default,
        }
    }
    #[doc = "Flash info0 (customer) space contains valid data."]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == Info0valid::Valid
    }
    #[doc = "Default value."]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == Info0valid::Default
    }
}
#[doc = "Field `INFO0VALID` writer - Indicates whether info0 contains valid data"]
pub type Info0validW<'a, REG> = crate::BitWriter<'a, REG, Info0valid>;
impl<'a, REG> Info0validW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flash info0 (customer) space contains valid data."]
    #[inline(always)]
    pub fn valid(self) -> &'a mut crate::W<REG> {
        self.variant(Info0valid::Valid)
    }
    #[doc = "Default value."]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(Info0valid::Default)
    }
}
impl R {
    #[doc = "Bit 0 - Indicates whether the shadow registers contain valid data from the Flash Information Space."]
    #[inline(always)]
    pub fn valid(&self) -> ValidR {
        ValidR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Indicates whether the bootloader should sleep or deep sleep if no image loaded."]
    #[inline(always)]
    pub fn bldsleep(&self) -> BldsleepR {
        BldsleepR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Indicates whether info0 contains valid data"]
    #[inline(always)]
    pub fn info0valid(&self) -> Info0validR {
        Info0validR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Indicates whether the shadow registers contain valid data from the Flash Information Space."]
    #[inline(always)]
    #[must_use]
    pub fn valid(&mut self) -> ValidW<ShadowvalidSpec> {
        ValidW::new(self, 0)
    }
    #[doc = "Bit 1 - Indicates whether the bootloader should sleep or deep sleep if no image loaded."]
    #[inline(always)]
    #[must_use]
    pub fn bldsleep(&mut self) -> BldsleepW<ShadowvalidSpec> {
        BldsleepW::new(self, 1)
    }
    #[doc = "Bit 2 - Indicates whether info0 contains valid data"]
    #[inline(always)]
    #[must_use]
    pub fn info0valid(&mut self) -> Info0validW<ShadowvalidSpec> {
        Info0validW::new(self, 2)
    }
}
#[doc = "Register to indicate whether the shadow registers have been successfully loaded from the Flash Information Space.\n\nYou can [`read`](crate::Reg::read) this register and get [`shadowvalid::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shadowvalid::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ShadowvalidSpec;
impl crate::RegisterSpec for ShadowvalidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`shadowvalid::R`](R) reader structure"]
impl crate::Readable for ShadowvalidSpec {}
#[doc = "`write(|w| ..)` method takes [`shadowvalid::W`](W) writer structure"]
impl crate::Writable for ShadowvalidSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SHADOWVALID to value 0x07"]
impl crate::Resettable for ShadowvalidSpec {
    const RESET_VALUE: u32 = 0x07;
}
