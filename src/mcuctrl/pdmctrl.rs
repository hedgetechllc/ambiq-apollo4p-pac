#[doc = "Register `PDMCTRL` reader"]
pub type R = crate::R<PdmctrlSpec>;
#[doc = "Register `PDMCTRL` writer"]
pub type W = crate::W<PdmctrlSpec>;
#[doc = "Field `PDMGLOBALEN` reader - PDM global enable to allow all PDMs to have synchronized interface clocks and FIFO sampling."]
pub type PdmglobalenR = crate::BitReader;
#[doc = "Field `PDMGLOBALEN` writer - PDM global enable to allow all PDMs to have synchronized interface clocks and FIFO sampling."]
pub type PdmglobalenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PDM global enable to allow all PDMs to have synchronized interface clocks and FIFO sampling."]
    #[inline(always)]
    pub fn pdmglobalen(&self) -> PdmglobalenR {
        PdmglobalenR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PDM global enable to allow all PDMs to have synchronized interface clocks and FIFO sampling."]
    #[inline(always)]
    #[must_use]
    pub fn pdmglobalen(&mut self) -> PdmglobalenW<PdmctrlSpec> {
        PdmglobalenW::new(self, 0)
    }
}
#[doc = "PDM Control\n\nYou can [`read`](crate::Reg::read) this register and get [`pdmctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdmctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdmctrlSpec;
impl crate::RegisterSpec for PdmctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdmctrl::R`](R) reader structure"]
impl crate::Readable for PdmctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`pdmctrl::W`](W) writer structure"]
impl crate::Writable for PdmctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PDMCTRL to value 0x01"]
impl crate::Resettable for PdmctrlSpec {
    const RESET_VALUE: u32 = 0x01;
}
