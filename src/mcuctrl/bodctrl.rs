#[doc = "Register `BODCTRL` reader"]
pub type R = crate::R<BodctrlSpec>;
#[doc = "Register `BODCTRL` writer"]
pub type W = crate::W<BodctrlSpec>;
#[doc = "Field `BODLPWD` reader - BODL Power Down."]
pub type BodlpwdR = crate::BitReader;
#[doc = "Field `BODLPWD` writer - BODL Power Down."]
pub type BodlpwdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BODHPWD` reader - BODH Power Down."]
pub type BodhpwdR = crate::BitReader;
#[doc = "Field `BODHPWD` writer - BODH Power Down."]
pub type BodhpwdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BODCPWD` reader - BODC Power Down."]
pub type BodcpwdR = crate::BitReader;
#[doc = "Field `BODCPWD` writer - BODC Power Down."]
pub type BodcpwdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BODFPWD` reader - BODF Power Down."]
pub type BodfpwdR = crate::BitReader;
#[doc = "Field `BODFPWD` writer - BODF Power Down."]
pub type BodfpwdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BODSPWD` reader - BODS Power Down."]
pub type BodspwdR = crate::BitReader;
#[doc = "Field `BODSPWD` writer - BODS Power Down."]
pub type BodspwdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BODCLVPWD` reader - BODC_LV Power Down."]
pub type BodclvpwdR = crate::BitReader;
#[doc = "Field `BODCLVPWD` writer - BODC_LV Power Down."]
pub type BodclvpwdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BODLVREFSEL` reader - BODL External Reference Select. Note: the SWE mux select in PWRSEQ2SWE must be set for this to take effect."]
pub type BodlvrefselR = crate::BitReader;
#[doc = "Field `BODLVREFSEL` writer - BODL External Reference Select. Note: the SWE mux select in PWRSEQ2SWE must be set for this to take effect."]
pub type BodlvrefselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BODHVREFSEL` reader - BODH External Reference Select. Note: the SWE mux select in PWRSEQ2SWE must be set for this to take effect."]
pub type BodhvrefselR = crate::BitReader;
#[doc = "Field `BODHVREFSEL` writer - BODH External Reference Select. Note: the SWE mux select in PWRSEQ2SWE must be set for this to take effect."]
pub type BodhvrefselW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - BODL Power Down."]
    #[inline(always)]
    pub fn bodlpwd(&self) -> BodlpwdR {
        BodlpwdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - BODH Power Down."]
    #[inline(always)]
    pub fn bodhpwd(&self) -> BodhpwdR {
        BodhpwdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - BODC Power Down."]
    #[inline(always)]
    pub fn bodcpwd(&self) -> BodcpwdR {
        BodcpwdR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - BODF Power Down."]
    #[inline(always)]
    pub fn bodfpwd(&self) -> BodfpwdR {
        BodfpwdR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - BODS Power Down."]
    #[inline(always)]
    pub fn bodspwd(&self) -> BodspwdR {
        BodspwdR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - BODC_LV Power Down."]
    #[inline(always)]
    pub fn bodclvpwd(&self) -> BodclvpwdR {
        BodclvpwdR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - BODL External Reference Select. Note: the SWE mux select in PWRSEQ2SWE must be set for this to take effect."]
    #[inline(always)]
    pub fn bodlvrefsel(&self) -> BodlvrefselR {
        BodlvrefselR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - BODH External Reference Select. Note: the SWE mux select in PWRSEQ2SWE must be set for this to take effect."]
    #[inline(always)]
    pub fn bodhvrefsel(&self) -> BodhvrefselR {
        BodhvrefselR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BODL Power Down."]
    #[inline(always)]
    #[must_use]
    pub fn bodlpwd(&mut self) -> BodlpwdW<BodctrlSpec> {
        BodlpwdW::new(self, 0)
    }
    #[doc = "Bit 1 - BODH Power Down."]
    #[inline(always)]
    #[must_use]
    pub fn bodhpwd(&mut self) -> BodhpwdW<BodctrlSpec> {
        BodhpwdW::new(self, 1)
    }
    #[doc = "Bit 2 - BODC Power Down."]
    #[inline(always)]
    #[must_use]
    pub fn bodcpwd(&mut self) -> BodcpwdW<BodctrlSpec> {
        BodcpwdW::new(self, 2)
    }
    #[doc = "Bit 3 - BODF Power Down."]
    #[inline(always)]
    #[must_use]
    pub fn bodfpwd(&mut self) -> BodfpwdW<BodctrlSpec> {
        BodfpwdW::new(self, 3)
    }
    #[doc = "Bit 4 - BODS Power Down."]
    #[inline(always)]
    #[must_use]
    pub fn bodspwd(&mut self) -> BodspwdW<BodctrlSpec> {
        BodspwdW::new(self, 4)
    }
    #[doc = "Bit 5 - BODC_LV Power Down."]
    #[inline(always)]
    #[must_use]
    pub fn bodclvpwd(&mut self) -> BodclvpwdW<BodctrlSpec> {
        BodclvpwdW::new(self, 5)
    }
    #[doc = "Bit 6 - BODL External Reference Select. Note: the SWE mux select in PWRSEQ2SWE must be set for this to take effect."]
    #[inline(always)]
    #[must_use]
    pub fn bodlvrefsel(&mut self) -> BodlvrefselW<BodctrlSpec> {
        BodlvrefselW::new(self, 6)
    }
    #[doc = "Bit 7 - BODH External Reference Select. Note: the SWE mux select in PWRSEQ2SWE must be set for this to take effect."]
    #[inline(always)]
    #[must_use]
    pub fn bodhvrefsel(&mut self) -> BodhvrefselW<BodctrlSpec> {
        BodhvrefselW::new(self, 7)
    }
}
#[doc = "BOD control\n\nYou can [`read`](crate::Reg::read) this register and get [`bodctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bodctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BodctrlSpec;
impl crate::RegisterSpec for BodctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bodctrl::R`](R) reader structure"]
impl crate::Readable for BodctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`bodctrl::W`](W) writer structure"]
impl crate::Writable for BodctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BODCTRL to value 0"]
impl crate::Resettable for BodctrlSpec {
    const RESET_VALUE: u32 = 0;
}
