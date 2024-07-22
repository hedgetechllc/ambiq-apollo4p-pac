#[doc = "Register `DMON2` reader"]
pub type R = crate::R<Dmon2Spec>;
#[doc = "Register `DMON2` writer"]
pub type W = crate::W<Dmon2Spec>;
#[doc = "Field `DHIT` reader - Cache hits from lookup operations."]
pub type DhitR = crate::FieldReader<u32>;
#[doc = "Field `DHIT` writer - Cache hits from lookup operations."]
pub type DhitW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Cache hits from lookup operations."]
    #[inline(always)]
    pub fn dhit(&self) -> DhitR {
        DhitR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Cache hits from lookup operations."]
    #[inline(always)]
    #[must_use]
    pub fn dhit(&mut self) -> DhitW<Dmon2Spec> {
        DhitW::new(self, 0)
    }
}
#[doc = "Data Cache Hits\n\nYou can [`read`](crate::Reg::read) this register and get [`dmon2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmon2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dmon2Spec;
impl crate::RegisterSpec for Dmon2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmon2::R`](R) reader structure"]
impl crate::Readable for Dmon2Spec {}
#[doc = "`write(|w| ..)` method takes [`dmon2::W`](W) writer structure"]
impl crate::Writable for Dmon2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMON2 to value 0"]
impl crate::Resettable for Dmon2Spec {
    const RESET_VALUE: u32 = 0;
}
