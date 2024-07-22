#[doc = "Register `OBSDATA` reader"]
pub type R = crate::R<ObsdataSpec>;
#[doc = "Register `OBSDATA` writer"]
pub type W = crate::W<ObsdataSpec>;
#[doc = "Field `OBSDATA` reader - Sample of the data output on the GPIO observation port. May have async sampling issues, as the data is not synronized to the read operation. Intended for debug purposes only."]
pub type ObsdataR = crate::FieldReader<u16>;
#[doc = "Field `OBSDATA` writer - Sample of the data output on the GPIO observation port. May have async sampling issues, as the data is not synronized to the read operation. Intended for debug purposes only."]
pub type ObsdataW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Sample of the data output on the GPIO observation port. May have async sampling issues, as the data is not synronized to the read operation. Intended for debug purposes only."]
    #[inline(always)]
    pub fn obsdata(&self) -> ObsdataR {
        ObsdataR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Sample of the data output on the GPIO observation port. May have async sampling issues, as the data is not synronized to the read operation. Intended for debug purposes only."]
    #[inline(always)]
    #[must_use]
    pub fn obsdata(&mut self) -> ObsdataW<ObsdataSpec> {
        ObsdataW::new(self, 0)
    }
}
#[doc = "GPIO Observation mode sample\n\nYou can [`read`](crate::Reg::read) this register and get [`obsdata::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obsdata::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ObsdataSpec;
impl crate::RegisterSpec for ObsdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`obsdata::R`](R) reader structure"]
impl crate::Readable for ObsdataSpec {}
#[doc = "`write(|w| ..)` method takes [`obsdata::W`](W) writer structure"]
impl crate::Writable for ObsdataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OBSDATA to value 0"]
impl crate::Resettable for ObsdataSpec {
    const RESET_VALUE: u32 = 0;
}
