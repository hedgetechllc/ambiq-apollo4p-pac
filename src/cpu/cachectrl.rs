#[doc = "Register `CACHECTRL` reader"]
pub type R = crate::R<CachectrlSpec>;
#[doc = "Register `CACHECTRL` writer"]
pub type W = crate::W<CachectrlSpec>;
#[doc = "Field `INVALIDATE` reader - Writing a 1 to this bitfield invalidates the CM4 cache contents."]
pub type InvalidateR = crate::BitReader;
#[doc = "Field `INVALIDATE` writer - Writing a 1 to this bitfield invalidates the CM4 cache contents."]
pub type InvalidateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Reset Cache Statistics. When written to a 1, the cache monitor counters will be cleared. The monitor counters can be reset only when the CACHECFG.ENABLE_MONITOR bit is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Resetstat {
    #[doc = "1: Clear Cache Stats"]
    Clear = 1,
    #[doc = "0: default Cache Stats"]
    Default = 0,
}
impl From<Resetstat> for bool {
    #[inline(always)]
    fn from(variant: Resetstat) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESETSTAT` reader - Reset Cache Statistics. When written to a 1, the cache monitor counters will be cleared. The monitor counters can be reset only when the CACHECFG.ENABLE_MONITOR bit is set."]
pub type ResetstatR = crate::BitReader<Resetstat>;
impl ResetstatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Resetstat {
        match self.bits {
            true => Resetstat::Clear,
            false => Resetstat::Default,
        }
    }
    #[doc = "Clear Cache Stats"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Resetstat::Clear
    }
    #[doc = "default Cache Stats"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == Resetstat::Default
    }
}
#[doc = "Field `RESETSTAT` writer - Reset Cache Statistics. When written to a 1, the cache monitor counters will be cleared. The monitor counters can be reset only when the CACHECFG.ENABLE_MONITOR bit is set."]
pub type ResetstatW<'a, REG> = crate::BitWriter<'a, REG, Resetstat>;
impl<'a, REG> ResetstatW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Cache Stats"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Resetstat::Clear)
    }
    #[doc = "default Cache Stats"]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(Resetstat::Default)
    }
}
#[doc = "Field `CACHEREADY` reader - Cache Ready Status (enabled and not processing an invalidate operation)"]
pub type CachereadyR = crate::BitReader;
#[doc = "Field `CACHEREADY` writer - Cache Ready Status (enabled and not processing an invalidate operation)"]
pub type CachereadyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Writing a 1 to this bitfield invalidates the CM4 cache contents."]
    #[inline(always)]
    pub fn invalidate(&self) -> InvalidateR {
        InvalidateR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reset Cache Statistics. When written to a 1, the cache monitor counters will be cleared. The monitor counters can be reset only when the CACHECFG.ENABLE_MONITOR bit is set."]
    #[inline(always)]
    pub fn resetstat(&self) -> ResetstatR {
        ResetstatR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Cache Ready Status (enabled and not processing an invalidate operation)"]
    #[inline(always)]
    pub fn cacheready(&self) -> CachereadyR {
        CachereadyR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Writing a 1 to this bitfield invalidates the CM4 cache contents."]
    #[inline(always)]
    #[must_use]
    pub fn invalidate(&mut self) -> InvalidateW<CachectrlSpec> {
        InvalidateW::new(self, 0)
    }
    #[doc = "Bit 1 - Reset Cache Statistics. When written to a 1, the cache monitor counters will be cleared. The monitor counters can be reset only when the CACHECFG.ENABLE_MONITOR bit is set."]
    #[inline(always)]
    #[must_use]
    pub fn resetstat(&mut self) -> ResetstatW<CachectrlSpec> {
        ResetstatW::new(self, 1)
    }
    #[doc = "Bit 2 - Cache Ready Status (enabled and not processing an invalidate operation)"]
    #[inline(always)]
    #[must_use]
    pub fn cacheready(&mut self) -> CachereadyW<CachectrlSpec> {
        CachereadyW::new(self, 2)
    }
}
#[doc = "Cache Control\n\nYou can [`read`](crate::Reg::read) this register and get [`cachectrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cachectrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CachectrlSpec;
impl crate::RegisterSpec for CachectrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cachectrl::R`](R) reader structure"]
impl crate::Readable for CachectrlSpec {}
#[doc = "`write(|w| ..)` method takes [`cachectrl::W`](W) writer structure"]
impl crate::Writable for CachectrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CACHECTRL to value 0"]
impl crate::Resettable for CachectrlSpec {
    const RESET_VALUE: u32 = 0;
}
