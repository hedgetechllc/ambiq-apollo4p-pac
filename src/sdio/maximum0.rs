#[doc = "Register `MAXIMUM0` reader"]
pub type R = crate::R<Maximum0Spec>;
#[doc = "Register `MAXIMUM0` writer"]
pub type W = crate::W<Maximum0Spec>;
#[doc = "Field `ALLBITSRSVD` reader - The entire 32-bits of this register are reserved, do not read or write."]
pub type AllbitsrsvdR = crate::FieldReader<u32>;
#[doc = "Field `ALLBITSRSVD` writer - The entire 32-bits of this register are reserved, do not read or write."]
pub type AllbitsrsvdW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - The entire 32-bits of this register are reserved, do not read or write."]
    #[inline(always)]
    pub fn allbitsrsvd(&self) -> AllbitsrsvdR {
        AllbitsrsvdR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The entire 32-bits of this register are reserved, do not read or write."]
    #[inline(always)]
    #[must_use]
    pub fn allbitsrsvd(&mut self) -> AllbitsrsvdW<Maximum0Spec> {
        AllbitsrsvdW::new(self, 0)
    }
}
#[doc = "Maximum current capabilities\n\nYou can [`read`](crate::Reg::read) this register and get [`maximum0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maximum0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Maximum0Spec;
impl crate::RegisterSpec for Maximum0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`maximum0::R`](R) reader structure"]
impl crate::Readable for Maximum0Spec {}
#[doc = "`write(|w| ..)` method takes [`maximum0::W`](W) writer structure"]
impl crate::Writable for Maximum0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MAXIMUM0 to value 0"]
impl crate::Resettable for Maximum0Spec {
    const RESET_VALUE: u32 = 0;
}
