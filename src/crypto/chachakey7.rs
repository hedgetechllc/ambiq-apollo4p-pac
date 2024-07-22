#[doc = "Register `CHACHAKEY7` reader"]
pub type R = crate::R<Chachakey7Spec>;
#[doc = "Register `CHACHAKEY7` writer"]
pub type W = crate::W<Chachakey7Spec>;
#[doc = "Field `CHACHAKEY7` reader - bits 31:0 of CHACHA Key"]
pub type Chachakey7R = crate::FieldReader<u32>;
#[doc = "Field `CHACHAKEY7` writer - bits 31:0 of CHACHA Key"]
pub type Chachakey7W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - bits 31:0 of CHACHA Key"]
    #[inline(always)]
    pub fn chachakey7(&self) -> Chachakey7R {
        Chachakey7R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - bits 31:0 of CHACHA Key"]
    #[inline(always)]
    #[must_use]
    pub fn chachakey7(&mut self) -> Chachakey7W<Chachakey7Spec> {
        Chachakey7W::new(self, 0)
    }
}
#[doc = "bits 31:0 of CHACHA Key\n\nYou can [`read`](crate::Reg::read) this register and get [`chachakey7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chachakey7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Chachakey7Spec;
impl crate::RegisterSpec for Chachakey7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chachakey7::R`](R) reader structure"]
impl crate::Readable for Chachakey7Spec {}
#[doc = "`write(|w| ..)` method takes [`chachakey7::W`](W) writer structure"]
impl crate::Writable for Chachakey7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHACHAKEY7 to value 0"]
impl crate::Resettable for Chachakey7Spec {
    const RESET_VALUE: u32 = 0;
}
