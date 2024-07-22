#[doc = "Register `SATCFG` reader"]
pub type R = crate::R<SatcfgSpec>;
#[doc = "Register `SATCFG` writer"]
pub type W = crate::W<SatcfgSpec>;
#[doc = "Field `SATEN` reader - Enable the saturation comparator"]
pub type SatenR = crate::BitReader;
#[doc = "Field `SATEN` writer - Enable the saturation comparator"]
pub type SatenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SATCHANSEL` reader - Select which slots to use for saturation measurement. 0 enables saturation on slots 0 and 2. 1 enables saturation on slots 1 and 3."]
pub type SatchanselR = crate::BitReader;
#[doc = "Field `SATCHANSEL` writer - Select which slots to use for saturation measurement. 0 enables saturation on slots 0 and 2. 1 enables saturation on slots 1 and 3."]
pub type SatchanselW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable the saturation comparator"]
    #[inline(always)]
    pub fn saten(&self) -> SatenR {
        SatenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Select which slots to use for saturation measurement. 0 enables saturation on slots 0 and 2. 1 enables saturation on slots 1 and 3."]
    #[inline(always)]
    pub fn satchansel(&self) -> SatchanselR {
        SatchanselR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable the saturation comparator"]
    #[inline(always)]
    #[must_use]
    pub fn saten(&mut self) -> SatenW<SatcfgSpec> {
        SatenW::new(self, 0)
    }
    #[doc = "Bit 4 - Select which slots to use for saturation measurement. 0 enables saturation on slots 0 and 2. 1 enables saturation on slots 1 and 3."]
    #[inline(always)]
    #[must_use]
    pub fn satchansel(&mut self) -> SatchanselW<SatcfgSpec> {
        SatchanselW::new(self, 4)
    }
}
#[doc = "Saturation Comparator Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`satcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`satcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SatcfgSpec;
impl crate::RegisterSpec for SatcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`satcfg::R`](R) reader structure"]
impl crate::Readable for SatcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`satcfg::W`](W) writer structure"]
impl crate::Writable for SatcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SATCFG to value 0"]
impl crate::Resettable for SatcfgSpec {
    const RESET_VALUE: u32 = 0;
}
