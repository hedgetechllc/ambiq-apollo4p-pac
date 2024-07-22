#[doc = "Register `PKAMONREAD` reader"]
pub type R = crate::R<PkamonreadSpec>;
#[doc = "Register `PKAMONREAD` writer"]
pub type W = crate::W<PkamonreadSpec>;
#[doc = "Field `PKAMONREAD` reader - This is the PKA monitor bus register output"]
pub type PkamonreadR = crate::FieldReader<u32>;
#[doc = "Field `PKAMONREAD` writer - This is the PKA monitor bus register output"]
pub type PkamonreadW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - This is the PKA monitor bus register output"]
    #[inline(always)]
    pub fn pkamonread(&self) -> PkamonreadR {
        PkamonreadR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - This is the PKA monitor bus register output"]
    #[inline(always)]
    #[must_use]
    pub fn pkamonread(&mut self) -> PkamonreadW<PkamonreadSpec> {
        PkamonreadW::new(self, 0)
    }
}
#[doc = "The PKA monitor bus register.\n\nYou can [`read`](crate::Reg::read) this register and get [`pkamonread::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pkamonread::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PkamonreadSpec;
impl crate::RegisterSpec for PkamonreadSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pkamonread::R`](R) reader structure"]
impl crate::Readable for PkamonreadSpec {}
#[doc = "`write(|w| ..)` method takes [`pkamonread::W`](W) writer structure"]
impl crate::Writable for PkamonreadSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PKAMONREAD to value 0"]
impl crate::Resettable for PkamonreadSpec {
    const RESET_VALUE: u32 = 0;
}
