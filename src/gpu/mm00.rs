#[doc = "Register `MM00` reader"]
pub type R = crate::R<Mm00Spec>;
#[doc = "Register `MM00` writer"]
pub type W = crate::W<Mm00Spec>;
#[doc = "Field `MTX` reader - (0,0). matrix floating point element."]
pub type MtxR = crate::FieldReader<u32>;
#[doc = "Field `MTX` writer - (0,0). matrix floating point element."]
pub type MtxW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - (0,0). matrix floating point element."]
    #[inline(always)]
    pub fn mtx(&self) -> MtxR {
        MtxR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - (0,0). matrix floating point element."]
    #[inline(always)]
    #[must_use]
    pub fn mtx(&mut self) -> MtxW<Mm00Spec> {
        MtxW::new(self, 0)
    }
}
#[doc = "matrix floating point element.\n\nYou can [`read`](crate::Reg::read) this register and get [`mm00::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mm00::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mm00Spec;
impl crate::RegisterSpec for Mm00Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mm00::R`](R) reader structure"]
impl crate::Readable for Mm00Spec {}
#[doc = "`write(|w| ..)` method takes [`mm00::W`](W) writer structure"]
impl crate::Writable for Mm00Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MM00 to value 0"]
impl crate::Resettable for Mm00Spec {
    const RESET_VALUE: u32 = 0;
}
