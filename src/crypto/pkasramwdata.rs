#[doc = "Register `PKASRAMWDATA` reader"]
pub type R = crate::R<PkasramwdataSpec>;
#[doc = "Register `PKASRAMWDATA` writer"]
pub type W = crate::W<PkasramwdataSpec>;
#[doc = "Field `PKASRAMWDATA` reader - 32 bit write to PKA SRAM: triggers the SRAM write DMA address automatically incremented"]
pub type PkasramwdataR = crate::FieldReader<u32>;
#[doc = "Field `PKASRAMWDATA` writer - 32 bit write to PKA SRAM: triggers the SRAM write DMA address automatically incremented"]
pub type PkasramwdataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 32 bit write to PKA SRAM: triggers the SRAM write DMA address automatically incremented"]
    #[inline(always)]
    pub fn pkasramwdata(&self) -> PkasramwdataR {
        PkasramwdataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 32 bit write to PKA SRAM: triggers the SRAM write DMA address automatically incremented"]
    #[inline(always)]
    #[must_use]
    pub fn pkasramwdata(&mut self) -> PkasramwdataW<PkasramwdataSpec> {
        PkasramwdataW::new(self, 0)
    }
}
#[doc = "Write data to PKA SRAM.\n\nYou can [`read`](crate::Reg::read) this register and get [`pkasramwdata::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pkasramwdata::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PkasramwdataSpec;
impl crate::RegisterSpec for PkasramwdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pkasramwdata::R`](R) reader structure"]
impl crate::Readable for PkasramwdataSpec {}
#[doc = "`write(|w| ..)` method takes [`pkasramwdata::W`](W) writer structure"]
impl crate::Writable for PkasramwdataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PKASRAMWDATA to value 0"]
impl crate::Resettable for PkasramwdataSpec {
    const RESET_VALUE: u32 = 0;
}
