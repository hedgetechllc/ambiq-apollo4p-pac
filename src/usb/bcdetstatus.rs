#[doc = "Register `BCDETSTATUS` reader"]
pub type R = crate::R<BcdetstatusSpec>;
#[doc = "Register `BCDETSTATUS` writer"]
pub type W = crate::W<BcdetstatusSpec>;
#[doc = "Field `DPATTACHED` reader - Data pin attachment detected"]
pub type DpattachedR = crate::BitReader;
#[doc = "Field `DPATTACHED` writer - Data pin attachment detected"]
pub type DpattachedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPDETECTED` reader - Charging port detected"]
pub type CpdetectedR = crate::BitReader;
#[doc = "Field `CPDETECTED` writer - Charging port detected"]
pub type CpdetectedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCPDETECTED` reader - Dedicated charging port detected"]
pub type DcpdetectedR = crate::BitReader;
#[doc = "Field `DCPDETECTED` writer - Dedicated charging port detected"]
pub type DcpdetectedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPCOMPOUT` reader - DP comparator output"]
pub type DpcompoutR = crate::BitReader;
#[doc = "Field `DPCOMPOUT` writer - DP comparator output"]
pub type DpcompoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMCOMPOUT` reader - DM comparator output"]
pub type DmcompoutR = crate::BitReader;
#[doc = "Field `DMCOMPOUT` writer - DM comparator output"]
pub type DmcompoutW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Data pin attachment detected"]
    #[inline(always)]
    pub fn dpattached(&self) -> DpattachedR {
        DpattachedR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Charging port detected"]
    #[inline(always)]
    pub fn cpdetected(&self) -> CpdetectedR {
        CpdetectedR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Dedicated charging port detected"]
    #[inline(always)]
    pub fn dcpdetected(&self) -> DcpdetectedR {
        DcpdetectedR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - DP comparator output"]
    #[inline(always)]
    pub fn dpcompout(&self) -> DpcompoutR {
        DpcompoutR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DM comparator output"]
    #[inline(always)]
    pub fn dmcompout(&self) -> DmcompoutR {
        DmcompoutR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Data pin attachment detected"]
    #[inline(always)]
    #[must_use]
    pub fn dpattached(&mut self) -> DpattachedW<BcdetstatusSpec> {
        DpattachedW::new(self, 0)
    }
    #[doc = "Bit 1 - Charging port detected"]
    #[inline(always)]
    #[must_use]
    pub fn cpdetected(&mut self) -> CpdetectedW<BcdetstatusSpec> {
        CpdetectedW::new(self, 1)
    }
    #[doc = "Bit 2 - Dedicated charging port detected"]
    #[inline(always)]
    #[must_use]
    pub fn dcpdetected(&mut self) -> DcpdetectedW<BcdetstatusSpec> {
        DcpdetectedW::new(self, 2)
    }
    #[doc = "Bit 4 - DP comparator output"]
    #[inline(always)]
    #[must_use]
    pub fn dpcompout(&mut self) -> DpcompoutW<BcdetstatusSpec> {
        DpcompoutW::new(self, 4)
    }
    #[doc = "Bit 5 - DM comparator output"]
    #[inline(always)]
    #[must_use]
    pub fn dmcompout(&mut self) -> DmcompoutW<BcdetstatusSpec> {
        DmcompoutW::new(self, 5)
    }
}
#[doc = "USB Battery Charge Detenction Registers\n\nYou can [`read`](crate::Reg::read) this register and get [`bcdetstatus::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcdetstatus::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BcdetstatusSpec;
impl crate::RegisterSpec for BcdetstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bcdetstatus::R`](R) reader structure"]
impl crate::Readable for BcdetstatusSpec {}
#[doc = "`write(|w| ..)` method takes [`bcdetstatus::W`](W) writer structure"]
impl crate::Writable for BcdetstatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BCDETSTATUS to value 0"]
impl crate::Resettable for BcdetstatusSpec {
    const RESET_VALUE: u32 = 0;
}
