#[doc = "Register `CHACHAFORPOLYKEY1` reader"]
pub type R = crate::R<Chachaforpolykey1Spec>;
#[doc = "Register `CHACHAFORPOLYKEY1` writer"]
pub type W = crate::W<Chachaforpolykey1Spec>;
#[doc = "Field `CHACHAFORPOLYKEY1` reader - bits 223:192 of CHACHA_FOR_POLY_KEY"]
pub type Chachaforpolykey1R = crate::FieldReader<u32>;
#[doc = "Field `CHACHAFORPOLYKEY1` writer - bits 223:192 of CHACHA_FOR_POLY_KEY"]
pub type Chachaforpolykey1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - bits 223:192 of CHACHA_FOR_POLY_KEY"]
    #[inline(always)]
    pub fn chachaforpolykey1(&self) -> Chachaforpolykey1R {
        Chachaforpolykey1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - bits 223:192 of CHACHA_FOR_POLY_KEY"]
    #[inline(always)]
    #[must_use]
    pub fn chachaforpolykey1(&mut self) -> Chachaforpolykey1W<Chachaforpolykey1Spec> {
        Chachaforpolykey1W::new(self, 0)
    }
}
#[doc = "bits 223:192 of CHACHA_FOR_POLY_KEY\n\nYou can [`read`](crate::Reg::read) this register and get [`chachaforpolykey1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chachaforpolykey1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Chachaforpolykey1Spec;
impl crate::RegisterSpec for Chachaforpolykey1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chachaforpolykey1::R`](R) reader structure"]
impl crate::Readable for Chachaforpolykey1Spec {}
#[doc = "`write(|w| ..)` method takes [`chachaforpolykey1::W`](W) writer structure"]
impl crate::Writable for Chachaforpolykey1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHACHAFORPOLYKEY1 to value 0"]
impl crate::Resettable for Chachaforpolykey1Spec {
    const RESET_VALUE: u32 = 0;
}
