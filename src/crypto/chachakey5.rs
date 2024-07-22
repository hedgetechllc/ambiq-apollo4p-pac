#[doc = "Register `CHACHAKEY5` reader"]
pub type R = crate::R<Chachakey5Spec>;
#[doc = "Register `CHACHAKEY5` writer"]
pub type W = crate::W<Chachakey5Spec>;
#[doc = "Field `CHACHAKEY5` reader - bits 95:64 of CHACHA Key"]
pub type Chachakey5R = crate::FieldReader<u32>;
#[doc = "Field `CHACHAKEY5` writer - bits 95:64 of CHACHA Key"]
pub type Chachakey5W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - bits 95:64 of CHACHA Key"]
    #[inline(always)]
    pub fn chachakey5(&self) -> Chachakey5R {
        Chachakey5R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - bits 95:64 of CHACHA Key"]
    #[inline(always)]
    #[must_use]
    pub fn chachakey5(&mut self) -> Chachakey5W<Chachakey5Spec> {
        Chachakey5W::new(self, 0)
    }
}
#[doc = "bits 95:64 of CHACHA Key\n\nYou can [`read`](crate::Reg::read) this register and get [`chachakey5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chachakey5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Chachakey5Spec;
impl crate::RegisterSpec for Chachakey5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chachakey5::R`](R) reader structure"]
impl crate::Readable for Chachakey5Spec {}
#[doc = "`write(|w| ..)` method takes [`chachakey5::W`](W) writer structure"]
impl crate::Writable for Chachakey5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHACHAKEY5 to value 0"]
impl crate::Resettable for Chachakey5Spec {
    const RESET_VALUE: u32 = 0;
}
