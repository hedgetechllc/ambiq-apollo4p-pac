#[doc = "Register `CHACHAKEY4` reader"]
pub type R = crate::R<Chachakey4Spec>;
#[doc = "Register `CHACHAKEY4` writer"]
pub type W = crate::W<Chachakey4Spec>;
#[doc = "Field `CHACHAKEY4` reader - bits 127:96 of CHACHA Key"]
pub type Chachakey4R = crate::FieldReader<u32>;
#[doc = "Field `CHACHAKEY4` writer - bits 127:96 of CHACHA Key"]
pub type Chachakey4W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - bits 127:96 of CHACHA Key"]
    #[inline(always)]
    pub fn chachakey4(&self) -> Chachakey4R {
        Chachakey4R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - bits 127:96 of CHACHA Key"]
    #[inline(always)]
    #[must_use]
    pub fn chachakey4(&mut self) -> Chachakey4W<Chachakey4Spec> {
        Chachakey4W::new(self, 0)
    }
}
#[doc = "bits 127:96 of CHACHA Key\n\nYou can [`read`](crate::Reg::read) this register and get [`chachakey4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chachakey4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Chachakey4Spec;
impl crate::RegisterSpec for Chachakey4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chachakey4::R`](R) reader structure"]
impl crate::Readable for Chachakey4Spec {}
#[doc = "`write(|w| ..)` method takes [`chachakey4::W`](W) writer structure"]
impl crate::Writable for Chachakey4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHACHAKEY4 to value 0"]
impl crate::Resettable for Chachakey4Spec {
    const RESET_VALUE: u32 = 0;
}
