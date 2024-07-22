#[doc = "Register `CHACHAFORPOLYKEY4` reader"]
pub type R = crate::R<Chachaforpolykey4Spec>;
#[doc = "Register `CHACHAFORPOLYKEY4` writer"]
pub type W = crate::W<Chachaforpolykey4Spec>;
#[doc = "Field `CHACHAFORPOLYKEY4` reader - bits 127:96 of CHACHA_FOR_POLY_KEY"]
pub type Chachaforpolykey4R = crate::FieldReader<u32>;
#[doc = "Field `CHACHAFORPOLYKEY4` writer - bits 127:96 of CHACHA_FOR_POLY_KEY"]
pub type Chachaforpolykey4W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - bits 127:96 of CHACHA_FOR_POLY_KEY"]
    #[inline(always)]
    pub fn chachaforpolykey4(&self) -> Chachaforpolykey4R {
        Chachaforpolykey4R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - bits 127:96 of CHACHA_FOR_POLY_KEY"]
    #[inline(always)]
    #[must_use]
    pub fn chachaforpolykey4(&mut self) -> Chachaforpolykey4W<Chachaforpolykey4Spec> {
        Chachaforpolykey4W::new(self, 0)
    }
}
#[doc = "bits 127:96 of CHACHA_FOR_POLY_KEY\n\nYou can [`read`](crate::Reg::read) this register and get [`chachaforpolykey4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chachaforpolykey4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Chachaforpolykey4Spec;
impl crate::RegisterSpec for Chachaforpolykey4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chachaforpolykey4::R`](R) reader structure"]
impl crate::Readable for Chachaforpolykey4Spec {}
#[doc = "`write(|w| ..)` method takes [`chachaforpolykey4::W`](W) writer structure"]
impl crate::Writable for Chachaforpolykey4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHACHAFORPOLYKEY4 to value 0"]
impl crate::Resettable for Chachaforpolykey4Spec {
    const RESET_VALUE: u32 = 0;
}
