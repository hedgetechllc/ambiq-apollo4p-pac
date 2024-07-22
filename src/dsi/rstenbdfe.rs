#[doc = "Register `RSTENBDFE` reader"]
pub type R = crate::R<RstenbdfeSpec>;
#[doc = "Register `RSTENBDFE` writer"]
pub type W = crate::W<RstenbdfeSpec>;
#[doc = "Field `ENABLE` reader - This field provides the reset (enable) to the DFE."]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - This field provides the reset (enable) to the DFE."]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This field provides the reset (enable) to the DFE."]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This field provides the reset (enable) to the DFE."]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<RstenbdfeSpec> {
        EnableW::new(self, 0)
    }
}
#[doc = "This field provides the reset (enable) to the DFE\n\nYou can [`read`](crate::Reg::read) this register and get [`rstenbdfe::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstenbdfe::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RstenbdfeSpec;
impl crate::RegisterSpec for RstenbdfeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rstenbdfe::R`](R) reader structure"]
impl crate::Readable for RstenbdfeSpec {}
#[doc = "`write(|w| ..)` method takes [`rstenbdfe::W`](W) writer structure"]
impl crate::Writable for RstenbdfeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RSTENBDFE to value 0"]
impl crate::Resettable for RstenbdfeSpec {
    const RESET_VALUE: u32 = 0;
}
