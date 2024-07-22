#[doc = "Register `IMON3` reader"]
pub type R = crate::R<Imon3Spec>;
#[doc = "Register `IMON3` writer"]
pub type W = crate::W<Imon3Spec>;
#[doc = "Field `ILINE` reader - Cache hits from line cache"]
pub type IlineR = crate::FieldReader<u32>;
#[doc = "Field `ILINE` writer - Cache hits from line cache"]
pub type IlineW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Cache hits from line cache"]
    #[inline(always)]
    pub fn iline(&self) -> IlineR {
        IlineR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Cache hits from line cache"]
    #[inline(always)]
    #[must_use]
    pub fn iline(&mut self) -> IlineW<Imon3Spec> {
        IlineW::new(self, 0)
    }
}
#[doc = "Instruction Cache Line Hits\n\nYou can [`read`](crate::Reg::read) this register and get [`imon3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imon3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Imon3Spec;
impl crate::RegisterSpec for Imon3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imon3::R`](R) reader structure"]
impl crate::Readable for Imon3Spec {}
#[doc = "`write(|w| ..)` method takes [`imon3::W`](W) writer structure"]
impl crate::Writable for Imon3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IMON3 to value 0"]
impl crate::Resettable for Imon3Spec {
    const RESET_VALUE: u32 = 0;
}
