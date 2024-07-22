#[doc = "Register `DMATOTCOUNT` reader"]
pub type R = crate::R<DmatotcountSpec>;
#[doc = "Register `DMATOTCOUNT` writer"]
pub type W = crate::W<DmatotcountSpec>;
#[doc = "Field `TOTCOUNT` reader - Total Transfer Count. The transfer count must be a multiple of the THR setting to avoid DMA overruns."]
pub type TotcountR = crate::FieldReader<u32>;
#[doc = "Field `TOTCOUNT` writer - Total Transfer Count. The transfer count must be a multiple of the THR setting to avoid DMA overruns."]
pub type TotcountW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - Total Transfer Count. The transfer count must be a multiple of the THR setting to avoid DMA overruns."]
    #[inline(always)]
    pub fn totcount(&self) -> TotcountR {
        TotcountR::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - Total Transfer Count. The transfer count must be a multiple of the THR setting to avoid DMA overruns."]
    #[inline(always)]
    #[must_use]
    pub fn totcount(&mut self) -> TotcountW<DmatotcountSpec> {
        TotcountW::new(self, 0)
    }
}
#[doc = "DMA Total Transfer Count\n\nYou can [`read`](crate::Reg::read) this register and get [`dmatotcount::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmatotcount::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmatotcountSpec;
impl crate::RegisterSpec for DmatotcountSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmatotcount::R`](R) reader structure"]
impl crate::Readable for DmatotcountSpec {}
#[doc = "`write(|w| ..)` method takes [`dmatotcount::W`](W) writer structure"]
impl crate::Writable for DmatotcountSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMATOTCOUNT to value 0"]
impl crate::Resettable for DmatotcountSpec {
    const RESET_VALUE: u32 = 0;
}
