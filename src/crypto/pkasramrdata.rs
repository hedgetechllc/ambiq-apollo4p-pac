#[doc = "Register `PKASRAMRDATA` reader"]
pub type R = crate::R<PkasramrdataSpec>;
#[doc = "Register `PKASRAMRDATA` writer"]
pub type W = crate::W<PkasramrdataSpec>;
#[doc = "Field `PKASRAMRDATA` reader - 32 bit read from PKA SRAM: read - triggers the SRAM read DMA address automatically incremented"]
pub type PkasramrdataR = crate::FieldReader<u32>;
#[doc = "Field `PKASRAMRDATA` writer - 32 bit read from PKA SRAM: read - triggers the SRAM read DMA address automatically incremented"]
pub type PkasramrdataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 32 bit read from PKA SRAM: read - triggers the SRAM read DMA address automatically incremented"]
    #[inline(always)]
    pub fn pkasramrdata(&self) -> PkasramrdataR {
        PkasramrdataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 32 bit read from PKA SRAM: read - triggers the SRAM read DMA address automatically incremented"]
    #[inline(always)]
    #[must_use]
    pub fn pkasramrdata(&mut self) -> PkasramrdataW<PkasramrdataSpec> {
        PkasramrdataW::new(self, 0)
    }
}
#[doc = "Read data from PKA SRAM.\n\nYou can [`read`](crate::Reg::read) this register and get [`pkasramrdata::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pkasramrdata::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PkasramrdataSpec;
impl crate::RegisterSpec for PkasramrdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pkasramrdata::R`](R) reader structure"]
impl crate::Readable for PkasramrdataSpec {}
#[doc = "`write(|w| ..)` method takes [`pkasramrdata::W`](W) writer structure"]
impl crate::Writable for PkasramrdataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PKASRAMRDATA to value 0"]
impl crate::Resettable for PkasramrdataSpec {
    const RESET_VALUE: u32 = 0;
}
