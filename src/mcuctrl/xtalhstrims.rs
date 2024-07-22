#[doc = "Register `XTALHSTRIMS` reader"]
pub type R = crate::R<XtalhstrimsSpec>;
#[doc = "Register `XTALHSTRIMS` writer"]
pub type W = crate::W<XtalhstrimsSpec>;
#[doc = "Field `XTALHSCAP2TRIM` reader - xtalhs_cap2_trim"]
pub type Xtalhscap2trimR = crate::FieldReader;
#[doc = "Field `XTALHSCAP2TRIM` writer - xtalhs_cap2_trim"]
pub type Xtalhscap2trimW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `XTALHSCAPTRIM` reader - xtalhs_cap_trim"]
pub type XtalhscaptrimR = crate::FieldReader;
#[doc = "Field `XTALHSCAPTRIM` writer - xtalhs_cap_trim"]
pub type XtalhscaptrimW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `XTALHSDRIVETRIM` reader - xtalhs_drive_trim"]
pub type XtalhsdrivetrimR = crate::FieldReader;
#[doc = "Field `XTALHSDRIVETRIM` writer - xtalhs_drive_trim"]
pub type XtalhsdrivetrimW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `XTALHSDRIVERSTRENGTH` reader - xtalhs_driver_strength"]
pub type XtalhsdriverstrengthR = crate::FieldReader;
#[doc = "Field `XTALHSDRIVERSTRENGTH` writer - xtalhs_driver_strength"]
pub type XtalhsdriverstrengthW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `XTALHSIBIASCOMP2TRIM` reader - xtalhs_ibias_comp2_trim"]
pub type Xtalhsibiascomp2trimR = crate::FieldReader;
#[doc = "Field `XTALHSIBIASCOMP2TRIM` writer - xtalhs_ibias_comp2_trim"]
pub type Xtalhsibiascomp2trimW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `XTALHSIBIASCOMPTRIM` reader - xtalhs_ibias_comp_trim"]
pub type XtalhsibiascomptrimR = crate::FieldReader;
#[doc = "Field `XTALHSIBIASCOMPTRIM` writer - xtalhs_ibias_comp_trim"]
pub type XtalhsibiascomptrimW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `XTALHSIBIASTRIM` reader - xtalhs_ibias_trim"]
pub type XtalhsibiastrimR = crate::FieldReader;
#[doc = "Field `XTALHSIBIASTRIM` writer - xtalhs_ibias_trim"]
pub type XtalhsibiastrimW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `XTALHSRSTRIM` reader - xtalhs_rs_trim"]
pub type XtalhsrstrimR = crate::BitReader;
#[doc = "Field `XTALHSRSTRIM` writer - xtalhs_rs_trim"]
pub type XtalhsrstrimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XTALHSSPARE` reader - xtalhs_spare"]
pub type XtalhsspareR = crate::BitReader;
#[doc = "Field `XTALHSSPARE` writer - xtalhs_spare"]
pub type XtalhsspareW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - xtalhs_cap2_trim"]
    #[inline(always)]
    pub fn xtalhscap2trim(&self) -> Xtalhscap2trimR {
        Xtalhscap2trimR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:9 - xtalhs_cap_trim"]
    #[inline(always)]
    pub fn xtalhscaptrim(&self) -> XtalhscaptrimR {
        XtalhscaptrimR::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bits 10:11 - xtalhs_drive_trim"]
    #[inline(always)]
    pub fn xtalhsdrivetrim(&self) -> XtalhsdrivetrimR {
        XtalhsdrivetrimR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:14 - xtalhs_driver_strength"]
    #[inline(always)]
    pub fn xtalhsdriverstrength(&self) -> XtalhsdriverstrengthR {
        XtalhsdriverstrengthR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:16 - xtalhs_ibias_comp2_trim"]
    #[inline(always)]
    pub fn xtalhsibiascomp2trim(&self) -> Xtalhsibiascomp2trimR {
        Xtalhsibiascomp2trimR::new(((self.bits >> 15) & 3) as u8)
    }
    #[doc = "Bits 17:20 - xtalhs_ibias_comp_trim"]
    #[inline(always)]
    pub fn xtalhsibiascomptrim(&self) -> XtalhsibiascomptrimR {
        XtalhsibiascomptrimR::new(((self.bits >> 17) & 0x0f) as u8)
    }
    #[doc = "Bits 21:27 - xtalhs_ibias_trim"]
    #[inline(always)]
    pub fn xtalhsibiastrim(&self) -> XtalhsibiastrimR {
        XtalhsibiastrimR::new(((self.bits >> 21) & 0x7f) as u8)
    }
    #[doc = "Bit 28 - xtalhs_rs_trim"]
    #[inline(always)]
    pub fn xtalhsrstrim(&self) -> XtalhsrstrimR {
        XtalhsrstrimR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - xtalhs_spare"]
    #[inline(always)]
    pub fn xtalhsspare(&self) -> XtalhsspareR {
        XtalhsspareR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - xtalhs_cap2_trim"]
    #[inline(always)]
    #[must_use]
    pub fn xtalhscap2trim(&mut self) -> Xtalhscap2trimW<XtalhstrimsSpec> {
        Xtalhscap2trimW::new(self, 0)
    }
    #[doc = "Bits 6:9 - xtalhs_cap_trim"]
    #[inline(always)]
    #[must_use]
    pub fn xtalhscaptrim(&mut self) -> XtalhscaptrimW<XtalhstrimsSpec> {
        XtalhscaptrimW::new(self, 6)
    }
    #[doc = "Bits 10:11 - xtalhs_drive_trim"]
    #[inline(always)]
    #[must_use]
    pub fn xtalhsdrivetrim(&mut self) -> XtalhsdrivetrimW<XtalhstrimsSpec> {
        XtalhsdrivetrimW::new(self, 10)
    }
    #[doc = "Bits 12:14 - xtalhs_driver_strength"]
    #[inline(always)]
    #[must_use]
    pub fn xtalhsdriverstrength(&mut self) -> XtalhsdriverstrengthW<XtalhstrimsSpec> {
        XtalhsdriverstrengthW::new(self, 12)
    }
    #[doc = "Bits 15:16 - xtalhs_ibias_comp2_trim"]
    #[inline(always)]
    #[must_use]
    pub fn xtalhsibiascomp2trim(&mut self) -> Xtalhsibiascomp2trimW<XtalhstrimsSpec> {
        Xtalhsibiascomp2trimW::new(self, 15)
    }
    #[doc = "Bits 17:20 - xtalhs_ibias_comp_trim"]
    #[inline(always)]
    #[must_use]
    pub fn xtalhsibiascomptrim(&mut self) -> XtalhsibiascomptrimW<XtalhstrimsSpec> {
        XtalhsibiascomptrimW::new(self, 17)
    }
    #[doc = "Bits 21:27 - xtalhs_ibias_trim"]
    #[inline(always)]
    #[must_use]
    pub fn xtalhsibiastrim(&mut self) -> XtalhsibiastrimW<XtalhstrimsSpec> {
        XtalhsibiastrimW::new(self, 21)
    }
    #[doc = "Bit 28 - xtalhs_rs_trim"]
    #[inline(always)]
    #[must_use]
    pub fn xtalhsrstrim(&mut self) -> XtalhsrstrimW<XtalhstrimsSpec> {
        XtalhsrstrimW::new(self, 28)
    }
    #[doc = "Bit 29 - xtalhs_spare"]
    #[inline(always)]
    #[must_use]
    pub fn xtalhsspare(&mut self) -> XtalhsspareW<XtalhstrimsSpec> {
        XtalhsspareW::new(self, 29)
    }
}
#[doc = "XTALHS Trims\n\nYou can [`read`](crate::Reg::read) this register and get [`xtalhstrims::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xtalhstrims::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XtalhstrimsSpec;
impl crate::RegisterSpec for XtalhstrimsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xtalhstrims::R`](R) reader structure"]
impl crate::Readable for XtalhstrimsSpec {}
#[doc = "`write(|w| ..)` method takes [`xtalhstrims::W`](W) writer structure"]
impl crate::Writable for XtalhstrimsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets XTALHSTRIMS to value 0"]
impl crate::Resettable for XtalhstrimsSpec {
    const RESET_VALUE: u32 = 0;
}
