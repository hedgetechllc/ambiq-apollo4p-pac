#[doc = "Register `IMON2` reader"]
pub type R = crate::R<Imon2Spec>;
#[doc = "Register `IMON2` writer"]
pub type W = crate::W<Imon2Spec>;
#[doc = "Field `IHIT` reader - Cache hits from lookup operations"]
pub type IhitR = crate::FieldReader<u32>;
#[doc = "Field `IHIT` writer - Cache hits from lookup operations"]
pub type IhitW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Cache hits from lookup operations"]
    #[inline(always)]
    pub fn ihit(&self) -> IhitR {
        IhitR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Cache hits from lookup operations"]
    #[inline(always)]
    #[must_use]
    pub fn ihit(&mut self) -> IhitW<Imon2Spec> {
        IhitW::new(self, 0)
    }
}
#[doc = "Instruction Cache Hits\n\nYou can [`read`](crate::Reg::read) this register and get [`imon2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imon2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Imon2Spec;
impl crate::RegisterSpec for Imon2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imon2::R`](R) reader structure"]
impl crate::Readable for Imon2Spec {}
#[doc = "`write(|w| ..)` method takes [`imon2::W`](W) writer structure"]
impl crate::Writable for Imon2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IMON2 to value 0"]
impl crate::Resettable for Imon2Spec {
    const RESET_VALUE: u32 = 0;
}
