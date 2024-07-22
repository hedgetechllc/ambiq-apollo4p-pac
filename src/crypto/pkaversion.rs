#[doc = "Register `PKAVERSION` reader"]
pub type R = crate::R<PkaversionSpec>;
#[doc = "Register `PKAVERSION` writer"]
pub type W = crate::W<PkaversionSpec>;
#[doc = "Field `PKAVERSION` reader - This is the PKA version"]
pub type PkaversionR = crate::FieldReader<u32>;
#[doc = "Field `PKAVERSION` writer - This is the PKA version"]
pub type PkaversionW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - This is the PKA version"]
    #[inline(always)]
    pub fn pkaversion(&self) -> PkaversionR {
        PkaversionR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - This is the PKA version"]
    #[inline(always)]
    #[must_use]
    pub fn pkaversion(&mut self) -> PkaversionW<PkaversionSpec> {
        PkaversionW::new(self, 0)
    }
}
#[doc = "This register holds the pka version\n\nYou can [`read`](crate::Reg::read) this register and get [`pkaversion::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pkaversion::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PkaversionSpec;
impl crate::RegisterSpec for PkaversionSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pkaversion::R`](R) reader structure"]
impl crate::Readable for PkaversionSpec {}
#[doc = "`write(|w| ..)` method takes [`pkaversion::W`](W) writer structure"]
impl crate::Writable for PkaversionSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PKAVERSION to value 0x1611_0215"]
impl crate::Resettable for PkaversionSpec {
    const RESET_VALUE: u32 = 0x1611_0215;
}
