#[doc = "Register `CHACHAFORPOLYKEY7` reader"]
pub type R = crate::R<Chachaforpolykey7Spec>;
#[doc = "Register `CHACHAFORPOLYKEY7` writer"]
pub type W = crate::W<Chachaforpolykey7Spec>;
#[doc = "Field `CHACHAFORPOLYKEY7` reader - bits 31:0 of CHACHA_FOR_POLY_KEY"]
pub type Chachaforpolykey7R = crate::FieldReader<u32>;
#[doc = "Field `CHACHAFORPOLYKEY7` writer - bits 31:0 of CHACHA_FOR_POLY_KEY"]
pub type Chachaforpolykey7W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - bits 31:0 of CHACHA_FOR_POLY_KEY"]
    #[inline(always)]
    pub fn chachaforpolykey7(&self) -> Chachaforpolykey7R {
        Chachaforpolykey7R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - bits 31:0 of CHACHA_FOR_POLY_KEY"]
    #[inline(always)]
    #[must_use]
    pub fn chachaforpolykey7(&mut self) -> Chachaforpolykey7W<Chachaforpolykey7Spec> {
        Chachaforpolykey7W::new(self, 0)
    }
}
#[doc = "bits 31:0 of CHACHA_FOR_POLY_KEY\n\nYou can [`read`](crate::Reg::read) this register and get [`chachaforpolykey7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chachaforpolykey7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Chachaforpolykey7Spec;
impl crate::RegisterSpec for Chachaforpolykey7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chachaforpolykey7::R`](R) reader structure"]
impl crate::Readable for Chachaforpolykey7Spec {}
#[doc = "`write(|w| ..)` method takes [`chachaforpolykey7::W`](W) writer structure"]
impl crate::Writable for Chachaforpolykey7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHACHAFORPOLYKEY7 to value 0"]
impl crate::Resettable for Chachaforpolykey7Spec {
    const RESET_VALUE: u32 = 0;
}
