#[doc = "Register `CACHECFG` reader"]
pub type R = crate::R<CachecfgSpec>;
#[doc = "Register `CACHECFG` writer"]
pub type W = crate::W<CachecfgSpec>;
#[doc = "Field `ENABLE` reader - Enables the CM4 cache controller and enables power to the cache SRAMs. The ICACHE_ENABLE and DCACHE_ENABLE should be set to enable caching for each type of access."]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - Enables the CM4 cache controller and enables power to the cache SRAMs. The ICACHE_ENABLE and DCACHE_ENABLE should be set to enable caching for each type of access."]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LRU` reader - Sets the cache repleacment policy. 0=LRR (least recently replaced), 1=LRU (least recently used). LRR minimizes writes to the TAG SRAM."]
pub type LruR = crate::BitReader;
#[doc = "Field `LRU` writer - Sets the cache repleacment policy. 0=LRR (least recently replaced), 1=LRU (least recently used). LRR minimizes writes to the TAG SRAM."]
pub type LruW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NC0ENABLE` reader - Enable Non-cacheable region 0. See NCR0 registers to define the region."]
pub type Nc0enableR = crate::BitReader;
#[doc = "Field `NC0ENABLE` writer - Enable Non-cacheable region 0. See NCR0 registers to define the region."]
pub type Nc0enableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NC1ENABLE` reader - Enable Non-cacheable region 1. See NCR1 registers to define the region."]
pub type Nc1enableR = crate::BitReader;
#[doc = "Field `NC1ENABLE` writer - Enable Non-cacheable region 1. See NCR1 registers to define the region."]
pub type Nc1enableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Sets the cache configuration\n\nValue on reset: 5"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Config {
    #[doc = "4: Direct mapped, 128-bit linesize, 512 entries (4 SRAMs active)"]
    W1_128b512e = 4,
    #[doc = "5: Two-way set associative, 128-bit linesize, 512 entries (8 SRAMs active)"]
    W2_128b512e = 5,
    #[doc = "8: Direct mapped, 128-bit linesize, 1024 entries (8 SRAMs active)"]
    W1_128b1024e = 8,
    #[doc = "12: Direct mapped, 128-bit linesize, 2048 entries (4 SRAMs active)"]
    W1_128b2048e = 12,
    #[doc = "13: Two-way set associative, 128-bit linesize, 2048 entries (8 SRAMs active)"]
    W2_128b2048e = 13,
    #[doc = "14: Direct mapped, 128-bit linesize, 4096 entries (8 SRAMs active)"]
    W1_128b4096e = 14,
}
impl From<Config> for u8 {
    #[inline(always)]
    fn from(variant: Config) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Config {
    type Ux = u8;
}
impl crate::IsEnum for Config {}
#[doc = "Field `CONFIG` reader - Sets the cache configuration"]
pub type ConfigR = crate::FieldReader<Config>;
impl ConfigR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Config> {
        match self.bits {
            4 => Some(Config::W1_128b512e),
            5 => Some(Config::W2_128b512e),
            8 => Some(Config::W1_128b1024e),
            12 => Some(Config::W1_128b2048e),
            13 => Some(Config::W2_128b2048e),
            14 => Some(Config::W1_128b4096e),
            _ => None,
        }
    }
    #[doc = "Direct mapped, 128-bit linesize, 512 entries (4 SRAMs active)"]
    #[inline(always)]
    pub fn is_w1_128b_512e(&self) -> bool {
        *self == Config::W1_128b512e
    }
    #[doc = "Two-way set associative, 128-bit linesize, 512 entries (8 SRAMs active)"]
    #[inline(always)]
    pub fn is_w2_128b_512e(&self) -> bool {
        *self == Config::W2_128b512e
    }
    #[doc = "Direct mapped, 128-bit linesize, 1024 entries (8 SRAMs active)"]
    #[inline(always)]
    pub fn is_w1_128b_1024e(&self) -> bool {
        *self == Config::W1_128b1024e
    }
    #[doc = "Direct mapped, 128-bit linesize, 2048 entries (4 SRAMs active)"]
    #[inline(always)]
    pub fn is_w1_128b_2048e(&self) -> bool {
        *self == Config::W1_128b2048e
    }
    #[doc = "Two-way set associative, 128-bit linesize, 2048 entries (8 SRAMs active)"]
    #[inline(always)]
    pub fn is_w2_128b_2048e(&self) -> bool {
        *self == Config::W2_128b2048e
    }
    #[doc = "Direct mapped, 128-bit linesize, 4096 entries (8 SRAMs active)"]
    #[inline(always)]
    pub fn is_w1_128b_4096e(&self) -> bool {
        *self == Config::W1_128b4096e
    }
}
#[doc = "Field `CONFIG` writer - Sets the cache configuration"]
pub type ConfigW<'a, REG> = crate::FieldWriter<'a, REG, 4, Config>;
impl<'a, REG> ConfigW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Direct mapped, 128-bit linesize, 512 entries (4 SRAMs active)"]
    #[inline(always)]
    pub fn w1_128b_512e(self) -> &'a mut crate::W<REG> {
        self.variant(Config::W1_128b512e)
    }
    #[doc = "Two-way set associative, 128-bit linesize, 512 entries (8 SRAMs active)"]
    #[inline(always)]
    pub fn w2_128b_512e(self) -> &'a mut crate::W<REG> {
        self.variant(Config::W2_128b512e)
    }
    #[doc = "Direct mapped, 128-bit linesize, 1024 entries (8 SRAMs active)"]
    #[inline(always)]
    pub fn w1_128b_1024e(self) -> &'a mut crate::W<REG> {
        self.variant(Config::W1_128b1024e)
    }
    #[doc = "Direct mapped, 128-bit linesize, 2048 entries (4 SRAMs active)"]
    #[inline(always)]
    pub fn w1_128b_2048e(self) -> &'a mut crate::W<REG> {
        self.variant(Config::W1_128b2048e)
    }
    #[doc = "Two-way set associative, 128-bit linesize, 2048 entries (8 SRAMs active)"]
    #[inline(always)]
    pub fn w2_128b_2048e(self) -> &'a mut crate::W<REG> {
        self.variant(Config::W2_128b2048e)
    }
    #[doc = "Direct mapped, 128-bit linesize, 4096 entries (8 SRAMs active)"]
    #[inline(always)]
    pub fn w1_128b_4096e(self) -> &'a mut crate::W<REG> {
        self.variant(Config::W1_128b4096e)
    }
}
#[doc = "Field `IENABLE` reader - Enable CM4 Instruction Caching"]
pub type IenableR = crate::BitReader;
#[doc = "Field `IENABLE` writer - Enable CM4 Instruction Caching"]
pub type IenableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DENABLE` reader - Enable CM4 Data Caching."]
pub type DenableR = crate::BitReader;
#[doc = "Field `DENABLE` writer - Enable CM4 Data Caching."]
pub type DenableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKGATE` reader - Enable clock gating of cache TAG RAM. Software should enable this bit for optimal power efficiency."]
pub type ClkgateR = crate::BitReader;
#[doc = "Field `CLKGATE` writer - Enable clock gating of cache TAG RAM. Software should enable this bit for optimal power efficiency."]
pub type ClkgateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LS` reader - Enable LS (light sleep) of cache RAMs. Software should DISABLE this bit since cache activity is too high to benefit from LS usage."]
pub type LsR = crate::BitReader;
#[doc = "Field `LS` writer - Enable LS (light sleep) of cache RAMs. Software should DISABLE this bit since cache activity is too high to benefit from LS usage."]
pub type LsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NC1CACHELOCK` reader - Only valid when Cache Mode D is set. When high sets the mode of the the NC1 region such that all accesse to this region are cached in to the upper half of the cache. When set low then NCR1 is non cacheable."]
pub type Nc1cachelockR = crate::BitReader;
#[doc = "Field `NC1CACHELOCK` writer - Only valid when Cache Mode D is set. When high sets the mode of the the NC1 region such that all accesse to this region are cached in to the upper half of the cache. When set low then NCR1 is non cacheable."]
pub type Nc1cachelockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NC0CACHELOCK` reader - Only valid when Cache Mode D is set. When high sets the mode of the the NC0 region such that all accesse to this region are cached in to the lower half of the cache. When set low then NCR0 is non cacheable."]
pub type Nc0cachelockR = crate::BitReader;
#[doc = "Field `NC0CACHELOCK` writer - Only valid when Cache Mode D is set. When high sets the mode of the the NC0 region such that all accesse to this region are cached in to the lower half of the cache. When set low then NCR0 is non cacheable."]
pub type Nc0cachelockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATACLKGATE` reader - Enable aggressive clock gating of entire data array. This bit should be set to 1 for optimal power efficiency."]
pub type DataclkgateR = crate::BitReader;
#[doc = "Field `DATACLKGATE` writer - Enable aggressive clock gating of entire data array. This bit should be set to 1 for optimal power efficiency."]
pub type DataclkgateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLEMONITOR` reader - Enable Cache Monitoring Stats. Cache monitoring consumes additional power and should only be enabled when profiling code and counters will increment when this bit is set. Counter values will be retained when this is set to 0, allowing software to enable/disable counting for multiple code segments."]
pub type EnablemonitorR = crate::BitReader;
#[doc = "Field `ENABLEMONITOR` writer - Enable Cache Monitoring Stats. Cache monitoring consumes additional power and should only be enabled when profiling code and counters will increment when this bit is set. Counter values will be retained when this is set to 0, allowing software to enable/disable counting for multiple code segments."]
pub type EnablemonitorW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enables the CM4 cache controller and enables power to the cache SRAMs. The ICACHE_ENABLE and DCACHE_ENABLE should be set to enable caching for each type of access."]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Sets the cache repleacment policy. 0=LRR (least recently replaced), 1=LRU (least recently used). LRR minimizes writes to the TAG SRAM."]
    #[inline(always)]
    pub fn lru(&self) -> LruR {
        LruR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Non-cacheable region 0. See NCR0 registers to define the region."]
    #[inline(always)]
    pub fn nc0enable(&self) -> Nc0enableR {
        Nc0enableR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Non-cacheable region 1. See NCR1 registers to define the region."]
    #[inline(always)]
    pub fn nc1enable(&self) -> Nc1enableR {
        Nc1enableR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Sets the cache configuration"]
    #[inline(always)]
    pub fn config(&self) -> ConfigR {
        ConfigR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Enable CM4 Instruction Caching"]
    #[inline(always)]
    pub fn ienable(&self) -> IenableR {
        IenableR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable CM4 Data Caching."]
    #[inline(always)]
    pub fn denable(&self) -> DenableR {
        DenableR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable clock gating of cache TAG RAM. Software should enable this bit for optimal power efficiency."]
    #[inline(always)]
    pub fn clkgate(&self) -> ClkgateR {
        ClkgateR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable LS (light sleep) of cache RAMs. Software should DISABLE this bit since cache activity is too high to benefit from LS usage."]
    #[inline(always)]
    pub fn ls(&self) -> LsR {
        LsR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Only valid when Cache Mode D is set. When high sets the mode of the the NC1 region such that all accesse to this region are cached in to the upper half of the cache. When set low then NCR1 is non cacheable."]
    #[inline(always)]
    pub fn nc1cachelock(&self) -> Nc1cachelockR {
        Nc1cachelockR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Only valid when Cache Mode D is set. When high sets the mode of the the NC0 region such that all accesse to this region are cached in to the lower half of the cache. When set low then NCR0 is non cacheable."]
    #[inline(always)]
    pub fn nc0cachelock(&self) -> Nc0cachelockR {
        Nc0cachelockR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable aggressive clock gating of entire data array. This bit should be set to 1 for optimal power efficiency."]
    #[inline(always)]
    pub fn dataclkgate(&self) -> DataclkgateR {
        DataclkgateR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable Cache Monitoring Stats. Cache monitoring consumes additional power and should only be enabled when profiling code and counters will increment when this bit is set. Counter values will be retained when this is set to 0, allowing software to enable/disable counting for multiple code segments."]
    #[inline(always)]
    pub fn enablemonitor(&self) -> EnablemonitorR {
        EnablemonitorR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables the CM4 cache controller and enables power to the cache SRAMs. The ICACHE_ENABLE and DCACHE_ENABLE should be set to enable caching for each type of access."]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<CachecfgSpec> {
        EnableW::new(self, 0)
    }
    #[doc = "Bit 1 - Sets the cache repleacment policy. 0=LRR (least recently replaced), 1=LRU (least recently used). LRR minimizes writes to the TAG SRAM."]
    #[inline(always)]
    #[must_use]
    pub fn lru(&mut self) -> LruW<CachecfgSpec> {
        LruW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Non-cacheable region 0. See NCR0 registers to define the region."]
    #[inline(always)]
    #[must_use]
    pub fn nc0enable(&mut self) -> Nc0enableW<CachecfgSpec> {
        Nc0enableW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Non-cacheable region 1. See NCR1 registers to define the region."]
    #[inline(always)]
    #[must_use]
    pub fn nc1enable(&mut self) -> Nc1enableW<CachecfgSpec> {
        Nc1enableW::new(self, 3)
    }
    #[doc = "Bits 4:7 - Sets the cache configuration"]
    #[inline(always)]
    #[must_use]
    pub fn config(&mut self) -> ConfigW<CachecfgSpec> {
        ConfigW::new(self, 4)
    }
    #[doc = "Bit 8 - Enable CM4 Instruction Caching"]
    #[inline(always)]
    #[must_use]
    pub fn ienable(&mut self) -> IenableW<CachecfgSpec> {
        IenableW::new(self, 8)
    }
    #[doc = "Bit 9 - Enable CM4 Data Caching."]
    #[inline(always)]
    #[must_use]
    pub fn denable(&mut self) -> DenableW<CachecfgSpec> {
        DenableW::new(self, 9)
    }
    #[doc = "Bit 10 - Enable clock gating of cache TAG RAM. Software should enable this bit for optimal power efficiency."]
    #[inline(always)]
    #[must_use]
    pub fn clkgate(&mut self) -> ClkgateW<CachecfgSpec> {
        ClkgateW::new(self, 10)
    }
    #[doc = "Bit 11 - Enable LS (light sleep) of cache RAMs. Software should DISABLE this bit since cache activity is too high to benefit from LS usage."]
    #[inline(always)]
    #[must_use]
    pub fn ls(&mut self) -> LsW<CachecfgSpec> {
        LsW::new(self, 11)
    }
    #[doc = "Bit 12 - Only valid when Cache Mode D is set. When high sets the mode of the the NC1 region such that all accesse to this region are cached in to the upper half of the cache. When set low then NCR1 is non cacheable."]
    #[inline(always)]
    #[must_use]
    pub fn nc1cachelock(&mut self) -> Nc1cachelockW<CachecfgSpec> {
        Nc1cachelockW::new(self, 12)
    }
    #[doc = "Bit 13 - Only valid when Cache Mode D is set. When high sets the mode of the the NC0 region such that all accesse to this region are cached in to the lower half of the cache. When set low then NCR0 is non cacheable."]
    #[inline(always)]
    #[must_use]
    pub fn nc0cachelock(&mut self) -> Nc0cachelockW<CachecfgSpec> {
        Nc0cachelockW::new(self, 13)
    }
    #[doc = "Bit 20 - Enable aggressive clock gating of entire data array. This bit should be set to 1 for optimal power efficiency."]
    #[inline(always)]
    #[must_use]
    pub fn dataclkgate(&mut self) -> DataclkgateW<CachecfgSpec> {
        DataclkgateW::new(self, 20)
    }
    #[doc = "Bit 24 - Enable Cache Monitoring Stats. Cache monitoring consumes additional power and should only be enabled when profiling code and counters will increment when this bit is set. Counter values will be retained when this is set to 0, allowing software to enable/disable counting for multiple code segments."]
    #[inline(always)]
    #[must_use]
    pub fn enablemonitor(&mut self) -> EnablemonitorW<CachecfgSpec> {
        EnablemonitorW::new(self, 24)
    }
}
#[doc = "CM4 Cache Control\n\nYou can [`read`](crate::Reg::read) this register and get [`cachecfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cachecfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CachecfgSpec;
impl crate::RegisterSpec for CachecfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cachecfg::R`](R) reader structure"]
impl crate::Readable for CachecfgSpec {}
#[doc = "`write(|w| ..)` method takes [`cachecfg::W`](W) writer structure"]
impl crate::Writable for CachecfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CACHECFG to value 0x0010_3c50"]
impl crate::Resettable for CachecfgSpec {
    const RESET_VALUE: u32 = 0x0010_3c50;
}
