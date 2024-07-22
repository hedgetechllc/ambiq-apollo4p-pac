#[doc = "Register `CHACHAFORPOLYKEY5` reader"]
pub type R = crate::R<Chachaforpolykey5Spec>;
#[doc = "Register `CHACHAFORPOLYKEY5` writer"]
pub type W = crate::W<Chachaforpolykey5Spec>;
#[doc = "Field `CHACHAFORPOLYKEY5` reader - bits 95:64 of CHACHA_FOR_POLY_KEY"]
pub type Chachaforpolykey5R = crate::FieldReader<u32>;
#[doc = "Field `CHACHAFORPOLYKEY5` writer - bits 95:64 of CHACHA_FOR_POLY_KEY"]
pub type Chachaforpolykey5W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - bits 95:64 of CHACHA_FOR_POLY_KEY"]
    #[inline(always)]
    pub fn chachaforpolykey5(&self) -> Chachaforpolykey5R {
        Chachaforpolykey5R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - bits 95:64 of CHACHA_FOR_POLY_KEY"]
    #[inline(always)]
    #[must_use]
    pub fn chachaforpolykey5(&mut self) -> Chachaforpolykey5W<Chachaforpolykey5Spec> {
        Chachaforpolykey5W::new(self, 0)
    }
}
#[doc = "bits 95:64 of CHACHA_FOR_POLY_KEY\n\nYou can [`read`](crate::Reg::read) this register and get [`chachaforpolykey5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chachaforpolykey5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Chachaforpolykey5Spec;
impl crate::RegisterSpec for Chachaforpolykey5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chachaforpolykey5::R`](R) reader structure"]
impl crate::Readable for Chachaforpolykey5Spec {}
#[doc = "`write(|w| ..)` method takes [`chachaforpolykey5::W`](W) writer structure"]
impl crate::Writable for Chachaforpolykey5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHACHAFORPOLYKEY5 to value 0"]
impl crate::Resettable for Chachaforpolykey5Spec {
    const RESET_VALUE: u32 = 0;
}
