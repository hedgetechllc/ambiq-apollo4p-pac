#[doc = "Register `CHACHAFORPOLYKEY2` reader"]
pub type R = crate::R<Chachaforpolykey2Spec>;
#[doc = "Register `CHACHAFORPOLYKEY2` writer"]
pub type W = crate::W<Chachaforpolykey2Spec>;
#[doc = "Field `CHACHAFORPOLYKEY2` reader - bits191:160 of CHACHA_FOR_POLY_KEY"]
pub type Chachaforpolykey2R = crate::FieldReader<u32>;
#[doc = "Field `CHACHAFORPOLYKEY2` writer - bits191:160 of CHACHA_FOR_POLY_KEY"]
pub type Chachaforpolykey2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - bits191:160 of CHACHA_FOR_POLY_KEY"]
    #[inline(always)]
    pub fn chachaforpolykey2(&self) -> Chachaforpolykey2R {
        Chachaforpolykey2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - bits191:160 of CHACHA_FOR_POLY_KEY"]
    #[inline(always)]
    #[must_use]
    pub fn chachaforpolykey2(&mut self) -> Chachaforpolykey2W<Chachaforpolykey2Spec> {
        Chachaforpolykey2W::new(self, 0)
    }
}
#[doc = "bits191:160 of CHACHA_FOR_POLY_KEY\n\nYou can [`read`](crate::Reg::read) this register and get [`chachaforpolykey2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chachaforpolykey2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Chachaforpolykey2Spec;
impl crate::RegisterSpec for Chachaforpolykey2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chachaforpolykey2::R`](R) reader structure"]
impl crate::Readable for Chachaforpolykey2Spec {}
#[doc = "`write(|w| ..)` method takes [`chachaforpolykey2::W`](W) writer structure"]
impl crate::Writable for Chachaforpolykey2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHACHAFORPOLYKEY2 to value 0"]
impl crate::Resettable for Chachaforpolykey2Spec {
    const RESET_VALUE: u32 = 0;
}
