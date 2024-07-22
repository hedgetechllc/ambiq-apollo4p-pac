#[doc = "Register `HOSTCCISIDLE` reader"]
pub type R = crate::R<HostccisidleSpec>;
#[doc = "Register `HOSTCCISIDLE` writer"]
pub type W = crate::W<HostccisidleSpec>;
#[doc = "Field `HOSTCCISIDLE` reader - Read if CC is idle."]
pub type HostccisidleR = crate::BitReader;
#[doc = "Field `HOSTCCISIDLE` writer - Read if CC is idle."]
pub type HostccisidleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOSTCCISIDLEEVENT` reader - The event that indicates that CC is idle."]
pub type HostccisidleeventR = crate::BitReader;
#[doc = "Field `HOSTCCISIDLEEVENT` writer - The event that indicates that CC is idle."]
pub type HostccisidleeventW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYMISBUSY` reader - symetric flow is busy"]
pub type SymisbusyR = crate::BitReader;
#[doc = "Field `SYMISBUSY` writer - symetric flow is busy"]
pub type SymisbusyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHBISIDLE` reader - ahb stste machine is idle"]
pub type AhbisidleR = crate::BitReader;
#[doc = "Field `AHBISIDLE` writer - ahb stste machine is idle"]
pub type AhbisidleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NVMARBISIDLE` reader - nvm arbiter is idle"]
pub type NvmarbisidleR = crate::BitReader;
#[doc = "Field `NVMARBISIDLE` writer - nvm arbiter is idle"]
pub type NvmarbisidleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NVMISIDLE` reader - nvm is idle"]
pub type NvmisidleR = crate::BitReader;
#[doc = "Field `NVMISIDLE` writer - nvm is idle"]
pub type NvmisidleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FATALWR` reader - fatal write"]
pub type FatalwrR = crate::BitReader;
#[doc = "Field `FATALWR` writer - fatal write"]
pub type FatalwrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RNGISIDLE` reader - rng is idle"]
pub type RngisidleR = crate::BitReader;
#[doc = "Field `RNGISIDLE` writer - rng is idle"]
pub type RngisidleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PKAISIDLE` reader - pka is idle"]
pub type PkaisidleR = crate::BitReader;
#[doc = "Field `PKAISIDLE` writer - pka is idle"]
pub type PkaisidleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRYPTOISIDLE` reader - crypto flow is done"]
pub type CryptoisidleR = crate::BitReader;
#[doc = "Field `CRYPTOISIDLE` writer - crypto flow is done"]
pub type CryptoisidleW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Read if CC is idle."]
    #[inline(always)]
    pub fn hostccisidle(&self) -> HostccisidleR {
        HostccisidleR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The event that indicates that CC is idle."]
    #[inline(always)]
    pub fn hostccisidleevent(&self) -> HostccisidleeventR {
        HostccisidleeventR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - symetric flow is busy"]
    #[inline(always)]
    pub fn symisbusy(&self) -> SymisbusyR {
        SymisbusyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ahb stste machine is idle"]
    #[inline(always)]
    pub fn ahbisidle(&self) -> AhbisidleR {
        AhbisidleR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - nvm arbiter is idle"]
    #[inline(always)]
    pub fn nvmarbisidle(&self) -> NvmarbisidleR {
        NvmarbisidleR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - nvm is idle"]
    #[inline(always)]
    pub fn nvmisidle(&self) -> NvmisidleR {
        NvmisidleR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - fatal write"]
    #[inline(always)]
    pub fn fatalwr(&self) -> FatalwrR {
        FatalwrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - rng is idle"]
    #[inline(always)]
    pub fn rngisidle(&self) -> RngisidleR {
        RngisidleR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - pka is idle"]
    #[inline(always)]
    pub fn pkaisidle(&self) -> PkaisidleR {
        PkaisidleR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - crypto flow is done"]
    #[inline(always)]
    pub fn cryptoisidle(&self) -> CryptoisidleR {
        CryptoisidleR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Read if CC is idle."]
    #[inline(always)]
    #[must_use]
    pub fn hostccisidle(&mut self) -> HostccisidleW<HostccisidleSpec> {
        HostccisidleW::new(self, 0)
    }
    #[doc = "Bit 1 - The event that indicates that CC is idle."]
    #[inline(always)]
    #[must_use]
    pub fn hostccisidleevent(&mut self) -> HostccisidleeventW<HostccisidleSpec> {
        HostccisidleeventW::new(self, 1)
    }
    #[doc = "Bit 2 - symetric flow is busy"]
    #[inline(always)]
    #[must_use]
    pub fn symisbusy(&mut self) -> SymisbusyW<HostccisidleSpec> {
        SymisbusyW::new(self, 2)
    }
    #[doc = "Bit 3 - ahb stste machine is idle"]
    #[inline(always)]
    #[must_use]
    pub fn ahbisidle(&mut self) -> AhbisidleW<HostccisidleSpec> {
        AhbisidleW::new(self, 3)
    }
    #[doc = "Bit 4 - nvm arbiter is idle"]
    #[inline(always)]
    #[must_use]
    pub fn nvmarbisidle(&mut self) -> NvmarbisidleW<HostccisidleSpec> {
        NvmarbisidleW::new(self, 4)
    }
    #[doc = "Bit 5 - nvm is idle"]
    #[inline(always)]
    #[must_use]
    pub fn nvmisidle(&mut self) -> NvmisidleW<HostccisidleSpec> {
        NvmisidleW::new(self, 5)
    }
    #[doc = "Bit 6 - fatal write"]
    #[inline(always)]
    #[must_use]
    pub fn fatalwr(&mut self) -> FatalwrW<HostccisidleSpec> {
        FatalwrW::new(self, 6)
    }
    #[doc = "Bit 7 - rng is idle"]
    #[inline(always)]
    #[must_use]
    pub fn rngisidle(&mut self) -> RngisidleW<HostccisidleSpec> {
        RngisidleW::new(self, 7)
    }
    #[doc = "Bit 8 - pka is idle"]
    #[inline(always)]
    #[must_use]
    pub fn pkaisidle(&mut self) -> PkaisidleW<HostccisidleSpec> {
        PkaisidleW::new(self, 8)
    }
    #[doc = "Bit 9 - crypto flow is done"]
    #[inline(always)]
    #[must_use]
    pub fn cryptoisidle(&mut self) -> CryptoisidleW<HostccisidleSpec> {
        CryptoisidleW::new(self, 9)
    }
}
#[doc = "This register holds the idle indication of CC . Note: This is a special register, affected by internal logic. Test result of this register is NA.\n\nYou can [`read`](crate::Reg::read) this register and get [`hostccisidle::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hostccisidle::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HostccisidleSpec;
impl crate::RegisterSpec for HostccisidleSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hostccisidle::R`](R) reader structure"]
impl crate::Readable for HostccisidleSpec {}
#[doc = "`write(|w| ..)` method takes [`hostccisidle::W`](W) writer structure"]
impl crate::Writable for HostccisidleSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HOSTCCISIDLE to value 0x03b9"]
impl crate::Resettable for HostccisidleSpec {
    const RESET_VALUE: u32 = 0x03b9;
}
