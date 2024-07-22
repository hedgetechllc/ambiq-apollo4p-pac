#[doc = "Register `DMON3` reader"]
pub type R = crate::R<Dmon3Spec>;
#[doc = "Register `DMON3` writer"]
pub type W = crate::W<Dmon3Spec>;
#[doc = "Field `DLINE` reader - Cache hits from line cache"]
pub type DlineR = crate::FieldReader<u32>;
#[doc = "Field `DLINE` writer - Cache hits from line cache"]
pub type DlineW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Cache hits from line cache"]
    #[inline(always)]
    pub fn dline(&self) -> DlineR {
        DlineR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Cache hits from line cache"]
    #[inline(always)]
    #[must_use]
    pub fn dline(&mut self) -> DlineW<Dmon3Spec> {
        DlineW::new(self, 0)
    }
}
#[doc = "Data Cache Line Hits\n\nYou can [`read`](crate::Reg::read) this register and get [`dmon3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmon3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dmon3Spec;
impl crate::RegisterSpec for Dmon3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmon3::R`](R) reader structure"]
impl crate::Readable for Dmon3Spec {}
#[doc = "`write(|w| ..)` method takes [`dmon3::W`](W) writer structure"]
impl crate::Writable for Dmon3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMON3 to value 0"]
impl crate::Resettable for Dmon3Spec {
    const RESET_VALUE: u32 = 0;
}
