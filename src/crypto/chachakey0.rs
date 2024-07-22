#[doc = "Register `CHACHAKEY0` reader"]
pub type R = crate::R<Chachakey0Spec>;
#[doc = "Register `CHACHAKEY0` writer"]
pub type W = crate::W<Chachakey0Spec>;
#[doc = "Field `CHACHAKEY0` reader - bits 255:224 of CHACHA Key"]
pub type Chachakey0R = crate::FieldReader<u32>;
#[doc = "Field `CHACHAKEY0` writer - bits 255:224 of CHACHA Key"]
pub type Chachakey0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - bits 255:224 of CHACHA Key"]
    #[inline(always)]
    pub fn chachakey0(&self) -> Chachakey0R {
        Chachakey0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - bits 255:224 of CHACHA Key"]
    #[inline(always)]
    #[must_use]
    pub fn chachakey0(&mut self) -> Chachakey0W<Chachakey0Spec> {
        Chachakey0W::new(self, 0)
    }
}
#[doc = "bits 255:224 of CHACHA Key\n\nYou can [`read`](crate::Reg::read) this register and get [`chachakey0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chachakey0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Chachakey0Spec;
impl crate::RegisterSpec for Chachakey0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chachakey0::R`](R) reader structure"]
impl crate::Readable for Chachakey0Spec {}
#[doc = "`write(|w| ..)` method takes [`chachakey0::W`](W) writer structure"]
impl crate::Writable for Chachakey0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHACHAKEY0 to value 0"]
impl crate::Resettable for Chachakey0Spec {
    const RESET_VALUE: u32 = 0;
}
