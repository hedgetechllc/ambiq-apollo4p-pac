#[doc = "Register `CHACHAFORPOLYKEY3` reader"]
pub type R = crate::R<Chachaforpolykey3Spec>;
#[doc = "Register `CHACHAFORPOLYKEY3` writer"]
pub type W = crate::W<Chachaforpolykey3Spec>;
#[doc = "Field `CHACHAFORPOLYKEY3` reader - bits 159:128 of CHACHA_FOR_POLY_KEY"]
pub type Chachaforpolykey3R = crate::FieldReader<u32>;
#[doc = "Field `CHACHAFORPOLYKEY3` writer - bits 159:128 of CHACHA_FOR_POLY_KEY"]
pub type Chachaforpolykey3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - bits 159:128 of CHACHA_FOR_POLY_KEY"]
    #[inline(always)]
    pub fn chachaforpolykey3(&self) -> Chachaforpolykey3R {
        Chachaforpolykey3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - bits 159:128 of CHACHA_FOR_POLY_KEY"]
    #[inline(always)]
    #[must_use]
    pub fn chachaforpolykey3(&mut self) -> Chachaforpolykey3W<Chachaforpolykey3Spec> {
        Chachaforpolykey3W::new(self, 0)
    }
}
#[doc = "bits159:128 of CHACHA_FOR_POLY_KEY\n\nYou can [`read`](crate::Reg::read) this register and get [`chachaforpolykey3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chachaforpolykey3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Chachaforpolykey3Spec;
impl crate::RegisterSpec for Chachaforpolykey3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chachaforpolykey3::R`](R) reader structure"]
impl crate::Readable for Chachaforpolykey3Spec {}
#[doc = "`write(|w| ..)` method takes [`chachaforpolykey3::W`](W) writer structure"]
impl crate::Writable for Chachaforpolykey3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHACHAFORPOLYKEY3 to value 0"]
impl crate::Resettable for Chachaforpolykey3Spec {
    const RESET_VALUE: u32 = 0;
}
