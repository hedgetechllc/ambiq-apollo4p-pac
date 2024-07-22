#[doc = "Register `MRAMPWRCTRL` reader"]
pub type R = crate::R<MrampwrctrlSpec>;
#[doc = "Register `MRAMPWRCTRL` writer"]
pub type W = crate::W<MrampwrctrlSpec>;
#[doc = "Field `MRAMLPREN` reader - MRAM low power mode enable"]
pub type MramlprenR = crate::BitReader;
#[doc = "Field `MRAMLPREN` writer - MRAM low power mode enable"]
pub type MramlprenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MRAMSLPEN` reader - MRAM sleep mode enable"]
pub type MramslpenR = crate::BitReader;
#[doc = "Field `MRAMSLPEN` writer - MRAM sleep mode enable"]
pub type MramslpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MRAMPWRCTRL` reader - MRAM low power mode control. When set to 1, tmc_lpr and tmc_slp are driven by the value of MRAMLPREN and MRAMSLPEN of this register."]
pub type MrampwrctrlR = crate::BitReader;
#[doc = "Field `MRAMPWRCTRL` writer - MRAM low power mode control. When set to 1, tmc_lpr and tmc_slp are driven by the value of MRAMLPREN and MRAMSLPEN of this register."]
pub type MrampwrctrlW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - MRAM low power mode enable"]
    #[inline(always)]
    pub fn mramlpren(&self) -> MramlprenR {
        MramlprenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MRAM sleep mode enable"]
    #[inline(always)]
    pub fn mramslpen(&self) -> MramslpenR {
        MramslpenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MRAM low power mode control. When set to 1, tmc_lpr and tmc_slp are driven by the value of MRAMLPREN and MRAMSLPEN of this register."]
    #[inline(always)]
    pub fn mrampwrctrl(&self) -> MrampwrctrlR {
        MrampwrctrlR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MRAM low power mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn mramlpren(&mut self) -> MramlprenW<MrampwrctrlSpec> {
        MramlprenW::new(self, 0)
    }
    #[doc = "Bit 1 - MRAM sleep mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn mramslpen(&mut self) -> MramslpenW<MrampwrctrlSpec> {
        MramslpenW::new(self, 1)
    }
    #[doc = "Bit 2 - MRAM low power mode control. When set to 1, tmc_lpr and tmc_slp are driven by the value of MRAMLPREN and MRAMSLPEN of this register."]
    #[inline(always)]
    #[must_use]
    pub fn mrampwrctrl(&mut self) -> MrampwrctrlW<MrampwrctrlSpec> {
        MrampwrctrlW::new(self, 2)
    }
}
#[doc = "MRAM Power Control\n\nYou can [`read`](crate::Reg::read) this register and get [`mrampwrctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mrampwrctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MrampwrctrlSpec;
impl crate::RegisterSpec for MrampwrctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mrampwrctrl::R`](R) reader structure"]
impl crate::Readable for MrampwrctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`mrampwrctrl::W`](W) writer structure"]
impl crate::Writable for MrampwrctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MRAMPWRCTRL to value 0"]
impl crate::Resettable for MrampwrctrlSpec {
    const RESET_VALUE: u32 = 0;
}
