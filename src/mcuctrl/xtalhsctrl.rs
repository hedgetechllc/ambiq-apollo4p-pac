#[doc = "Register `XTALHSCTRL` reader"]
pub type R = crate::R<XtalhsctrlSpec>;
#[doc = "Register `XTALHSCTRL` writer"]
pub type W = crate::W<XtalhsctrlSpec>;
#[doc = "Field `XTALHSPDNB` reader - xtalhs_pdnb"]
pub type XtalhspdnbR = crate::BitReader;
#[doc = "Field `XTALHSPDNB` writer - xtalhs_pdnb"]
pub type XtalhspdnbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XTALHSCOMPPDNB` reader - xtalhs_comp_pdnb"]
pub type XtalhscomppdnbR = crate::BitReader;
#[doc = "Field `XTALHSCOMPPDNB` writer - xtalhs_comp_pdnb"]
pub type XtalhscomppdnbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XTALHSCOMPSEL` reader - xtalhs_comp_sel"]
pub type XtalhscompselR = crate::BitReader;
#[doc = "Field `XTALHSCOMPSEL` writer - xtalhs_comp_sel"]
pub type XtalhscompselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XTALHSIBSTENABLE` reader - xtalhs_ibst_enable"]
pub type XtalhsibstenableR = crate::BitReader;
#[doc = "Field `XTALHSIBSTENABLE` writer - xtalhs_ibst_enable"]
pub type XtalhsibstenableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XTALHSINJECTIONENABLE` reader - xtalhs_injection_enable"]
pub type XtalhsinjectionenableR = crate::BitReader;
#[doc = "Field `XTALHSINJECTIONENABLE` writer - xtalhs_injection_enable"]
pub type XtalhsinjectionenableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XTALHSPDNPNIMPROVE` reader - xtalhs_pdn_pn_improve"]
pub type XtalhspdnpnimproveR = crate::BitReader;
#[doc = "Field `XTALHSPDNPNIMPROVE` writer - xtalhs_pdn_pn_improve"]
pub type XtalhspdnpnimproveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XTALHSSELRCOM` reader - xtalhs_sel_rcom"]
pub type XtalhsselrcomR = crate::BitReader;
#[doc = "Field `XTALHSSELRCOM` writer - xtalhs_sel_rcom"]
pub type XtalhsselrcomW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XTALHSPADOUTEN` reader - xtalhs_padout_en"]
pub type XtalhspadoutenR = crate::BitReader;
#[doc = "Field `XTALHSPADOUTEN` writer - xtalhs_padout_en"]
pub type XtalhspadoutenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XTALHSEXTERNALCLOCK` reader - xtalhs_external_clock"]
pub type XtalhsexternalclockR = crate::BitReader;
#[doc = "Field `XTALHSEXTERNALCLOCK` writer - xtalhs_external_clock"]
pub type XtalhsexternalclockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - xtalhs_pdnb"]
    #[inline(always)]
    pub fn xtalhspdnb(&self) -> XtalhspdnbR {
        XtalhspdnbR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - xtalhs_comp_pdnb"]
    #[inline(always)]
    pub fn xtalhscomppdnb(&self) -> XtalhscomppdnbR {
        XtalhscomppdnbR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - xtalhs_comp_sel"]
    #[inline(always)]
    pub fn xtalhscompsel(&self) -> XtalhscompselR {
        XtalhscompselR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - xtalhs_ibst_enable"]
    #[inline(always)]
    pub fn xtalhsibstenable(&self) -> XtalhsibstenableR {
        XtalhsibstenableR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - xtalhs_injection_enable"]
    #[inline(always)]
    pub fn xtalhsinjectionenable(&self) -> XtalhsinjectionenableR {
        XtalhsinjectionenableR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - xtalhs_pdn_pn_improve"]
    #[inline(always)]
    pub fn xtalhspdnpnimprove(&self) -> XtalhspdnpnimproveR {
        XtalhspdnpnimproveR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - xtalhs_sel_rcom"]
    #[inline(always)]
    pub fn xtalhsselrcom(&self) -> XtalhsselrcomR {
        XtalhsselrcomR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - xtalhs_padout_en"]
    #[inline(always)]
    pub fn xtalhspadouten(&self) -> XtalhspadoutenR {
        XtalhspadoutenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - xtalhs_external_clock"]
    #[inline(always)]
    pub fn xtalhsexternalclock(&self) -> XtalhsexternalclockR {
        XtalhsexternalclockR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - xtalhs_pdnb"]
    #[inline(always)]
    #[must_use]
    pub fn xtalhspdnb(&mut self) -> XtalhspdnbW<XtalhsctrlSpec> {
        XtalhspdnbW::new(self, 0)
    }
    #[doc = "Bit 1 - xtalhs_comp_pdnb"]
    #[inline(always)]
    #[must_use]
    pub fn xtalhscomppdnb(&mut self) -> XtalhscomppdnbW<XtalhsctrlSpec> {
        XtalhscomppdnbW::new(self, 1)
    }
    #[doc = "Bit 2 - xtalhs_comp_sel"]
    #[inline(always)]
    #[must_use]
    pub fn xtalhscompsel(&mut self) -> XtalhscompselW<XtalhsctrlSpec> {
        XtalhscompselW::new(self, 2)
    }
    #[doc = "Bit 3 - xtalhs_ibst_enable"]
    #[inline(always)]
    #[must_use]
    pub fn xtalhsibstenable(&mut self) -> XtalhsibstenableW<XtalhsctrlSpec> {
        XtalhsibstenableW::new(self, 3)
    }
    #[doc = "Bit 4 - xtalhs_injection_enable"]
    #[inline(always)]
    #[must_use]
    pub fn xtalhsinjectionenable(&mut self) -> XtalhsinjectionenableW<XtalhsctrlSpec> {
        XtalhsinjectionenableW::new(self, 4)
    }
    #[doc = "Bit 5 - xtalhs_pdn_pn_improve"]
    #[inline(always)]
    #[must_use]
    pub fn xtalhspdnpnimprove(&mut self) -> XtalhspdnpnimproveW<XtalhsctrlSpec> {
        XtalhspdnpnimproveW::new(self, 5)
    }
    #[doc = "Bit 6 - xtalhs_sel_rcom"]
    #[inline(always)]
    #[must_use]
    pub fn xtalhsselrcom(&mut self) -> XtalhsselrcomW<XtalhsctrlSpec> {
        XtalhsselrcomW::new(self, 6)
    }
    #[doc = "Bit 7 - xtalhs_padout_en"]
    #[inline(always)]
    #[must_use]
    pub fn xtalhspadouten(&mut self) -> XtalhspadoutenW<XtalhsctrlSpec> {
        XtalhspadoutenW::new(self, 7)
    }
    #[doc = "Bit 8 - xtalhs_external_clock"]
    #[inline(always)]
    #[must_use]
    pub fn xtalhsexternalclock(&mut self) -> XtalhsexternalclockW<XtalhsctrlSpec> {
        XtalhsexternalclockW::new(self, 8)
    }
}
#[doc = "XTALHS Control\n\nYou can [`read`](crate::Reg::read) this register and get [`xtalhsctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xtalhsctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XtalhsctrlSpec;
impl crate::RegisterSpec for XtalhsctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xtalhsctrl::R`](R) reader structure"]
impl crate::Readable for XtalhsctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`xtalhsctrl::W`](W) writer structure"]
impl crate::Writable for XtalhsctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets XTALHSCTRL to value 0"]
impl crate::Resettable for XtalhsctrlSpec {
    const RESET_VALUE: u32 = 0;
}
