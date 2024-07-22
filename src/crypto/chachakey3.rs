#[doc = "Register `CHACHAKEY3` reader"]
pub type R = crate::R<Chachakey3Spec>;
#[doc = "Register `CHACHAKEY3` writer"]
pub type W = crate::W<Chachakey3Spec>;
#[doc = "Field `CHACHAKEY3` reader - bits 159:128 of CHACHA Key"]
pub type Chachakey3R = crate::FieldReader<u32>;
#[doc = "Field `CHACHAKEY3` writer - bits 159:128 of CHACHA Key"]
pub type Chachakey3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - bits 159:128 of CHACHA Key"]
    #[inline(always)]
    pub fn chachakey3(&self) -> Chachakey3R {
        Chachakey3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - bits 159:128 of CHACHA Key"]
    #[inline(always)]
    #[must_use]
    pub fn chachakey3(&mut self) -> Chachakey3W<Chachakey3Spec> {
        Chachakey3W::new(self, 0)
    }
}
#[doc = "bits159:128 of CHACHA Key\n\nYou can [`read`](crate::Reg::read) this register and get [`chachakey3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chachakey3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Chachakey3Spec;
impl crate::RegisterSpec for Chachakey3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chachakey3::R`](R) reader structure"]
impl crate::Readable for Chachakey3Spec {}
#[doc = "`write(|w| ..)` method takes [`chachakey3::W`](W) writer structure"]
impl crate::Writable for Chachakey3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHACHAKEY3 to value 0"]
impl crate::Resettable for Chachakey3Spec {
    const RESET_VALUE: u32 = 0;
}
