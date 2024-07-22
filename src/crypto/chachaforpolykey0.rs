#[doc = "Register `CHACHAFORPOLYKEY0` reader"]
pub type R = crate::R<Chachaforpolykey0Spec>;
#[doc = "Register `CHACHAFORPOLYKEY0` writer"]
pub type W = crate::W<Chachaforpolykey0Spec>;
#[doc = "Field `CHACHAFORPOLYKEY0` reader - bits 255:224 of CHACHA_FOR_POLY_KEY"]
pub type Chachaforpolykey0R = crate::FieldReader<u32>;
#[doc = "Field `CHACHAFORPOLYKEY0` writer - bits 255:224 of CHACHA_FOR_POLY_KEY"]
pub type Chachaforpolykey0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - bits 255:224 of CHACHA_FOR_POLY_KEY"]
    #[inline(always)]
    pub fn chachaforpolykey0(&self) -> Chachaforpolykey0R {
        Chachaforpolykey0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - bits 255:224 of CHACHA_FOR_POLY_KEY"]
    #[inline(always)]
    #[must_use]
    pub fn chachaforpolykey0(&mut self) -> Chachaforpolykey0W<Chachaforpolykey0Spec> {
        Chachaforpolykey0W::new(self, 0)
    }
}
#[doc = "bits 255:224 of CHACHA_FOR_POLY_KEY\n\nYou can [`read`](crate::Reg::read) this register and get [`chachaforpolykey0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chachaforpolykey0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Chachaforpolykey0Spec;
impl crate::RegisterSpec for Chachaforpolykey0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chachaforpolykey0::R`](R) reader structure"]
impl crate::Readable for Chachaforpolykey0Spec {}
#[doc = "`write(|w| ..)` method takes [`chachaforpolykey0::W`](W) writer structure"]
impl crate::Writable for Chachaforpolykey0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHACHAFORPOLYKEY0 to value 0"]
impl crate::Resettable for Chachaforpolykey0Spec {
    const RESET_VALUE: u32 = 0;
}
