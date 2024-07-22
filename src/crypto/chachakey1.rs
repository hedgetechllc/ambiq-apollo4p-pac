#[doc = "Register `CHACHAKEY1` reader"]
pub type R = crate::R<Chachakey1Spec>;
#[doc = "Register `CHACHAKEY1` writer"]
pub type W = crate::W<Chachakey1Spec>;
#[doc = "Field `CHACHAKEY1` reader - bits 223:192 of CHACHA Key"]
pub type Chachakey1R = crate::FieldReader<u32>;
#[doc = "Field `CHACHAKEY1` writer - bits 223:192 of CHACHA Key"]
pub type Chachakey1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - bits 223:192 of CHACHA Key"]
    #[inline(always)]
    pub fn chachakey1(&self) -> Chachakey1R {
        Chachakey1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - bits 223:192 of CHACHA Key"]
    #[inline(always)]
    #[must_use]
    pub fn chachakey1(&mut self) -> Chachakey1W<Chachakey1Spec> {
        Chachakey1W::new(self, 0)
    }
}
#[doc = "bits 223:192 of CHACHA Key\n\nYou can [`read`](crate::Reg::read) this register and get [`chachakey1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chachakey1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Chachakey1Spec;
impl crate::RegisterSpec for Chachakey1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chachakey1::R`](R) reader structure"]
impl crate::Readable for Chachakey1Spec {}
#[doc = "`write(|w| ..)` method takes [`chachakey1::W`](W) writer structure"]
impl crate::Writable for Chachakey1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHACHAKEY1 to value 0"]
impl crate::Resettable for Chachakey1Spec {
    const RESET_VALUE: u32 = 0;
}
