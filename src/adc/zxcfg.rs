#[doc = "Register `ZXCFG` reader"]
pub type R = crate::R<ZxcfgSpec>;
#[doc = "Register `ZXCFG` writer"]
pub type W = crate::W<ZxcfgSpec>;
#[doc = "Field `ZXEN` reader - Enable the ZX comparator"]
pub type ZxenR = crate::BitReader;
#[doc = "Field `ZXEN` writer - Enable the ZX comparator"]
pub type ZxenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ZXCHANSEL` reader - Select which slots to use for zero crossing measurement. 0 enables zero crossing detection on slots 0 and 2. 1 enables zero crossing detection on slots 1 and 3."]
pub type ZxchanselR = crate::BitReader;
#[doc = "Field `ZXCHANSEL` writer - Select which slots to use for zero crossing measurement. 0 enables zero crossing detection on slots 0 and 2. 1 enables zero crossing detection on slots 1 and 3."]
pub type ZxchanselW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable the ZX comparator"]
    #[inline(always)]
    pub fn zxen(&self) -> ZxenR {
        ZxenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Select which slots to use for zero crossing measurement. 0 enables zero crossing detection on slots 0 and 2. 1 enables zero crossing detection on slots 1 and 3."]
    #[inline(always)]
    pub fn zxchansel(&self) -> ZxchanselR {
        ZxchanselR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable the ZX comparator"]
    #[inline(always)]
    #[must_use]
    pub fn zxen(&mut self) -> ZxenW<ZxcfgSpec> {
        ZxenW::new(self, 0)
    }
    #[doc = "Bit 4 - Select which slots to use for zero crossing measurement. 0 enables zero crossing detection on slots 0 and 2. 1 enables zero crossing detection on slots 1 and 3."]
    #[inline(always)]
    #[must_use]
    pub fn zxchansel(&mut self) -> ZxchanselW<ZxcfgSpec> {
        ZxchanselW::new(self, 4)
    }
}
#[doc = "Zero Crossing Comparator Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`zxcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`zxcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ZxcfgSpec;
impl crate::RegisterSpec for ZxcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`zxcfg::R`](R) reader structure"]
impl crate::Readable for ZxcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`zxcfg::W`](W) writer structure"]
impl crate::Writable for ZxcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ZXCFG to value 0"]
impl crate::Resettable for ZxcfgSpec {
    const RESET_VALUE: u32 = 0;
}
