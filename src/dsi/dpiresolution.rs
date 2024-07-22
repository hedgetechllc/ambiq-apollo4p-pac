#[doc = "Register `DPIRESOLUTION` reader"]
pub type R = crate::R<DpiresolutionSpec>;
#[doc = "Register `DPIRESOLUTION` writer"]
pub type W = crate::W<DpiresolutionSpec>;
#[doc = "Field `DPIRESOLUTION` reader - DPIRESOLUTION register description needed here."]
pub type DpiresolutionR = crate::FieldReader<u32>;
#[doc = "Field `DPIRESOLUTION` writer - DPIRESOLUTION register description needed here."]
pub type DpiresolutionW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - DPIRESOLUTION register description needed here."]
    #[inline(always)]
    pub fn dpiresolution(&self) -> DpiresolutionR {
        DpiresolutionR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DPIRESOLUTION register description needed here."]
    #[inline(always)]
    #[must_use]
    pub fn dpiresolution(&mut self) -> DpiresolutionW<DpiresolutionSpec> {
        DpiresolutionW::new(self, 0)
    }
}
#[doc = "Shows the horizontal address count in pixels\n\nYou can [`read`](crate::Reg::read) this register and get [`dpiresolution::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpiresolution::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpiresolutionSpec;
impl crate::RegisterSpec for DpiresolutionSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dpiresolution::R`](R) reader structure"]
impl crate::Readable for DpiresolutionSpec {}
#[doc = "`write(|w| ..)` method takes [`dpiresolution::W`](W) writer structure"]
impl crate::Writable for DpiresolutionSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DPIRESOLUTION to value 0"]
impl crate::Resettable for DpiresolutionSpec {
    const RESET_VALUE: u32 = 0;
}
