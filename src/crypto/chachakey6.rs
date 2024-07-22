#[doc = "Register `CHACHAKEY6` reader"]
pub type R = crate::R<Chachakey6Spec>;
#[doc = "Register `CHACHAKEY6` writer"]
pub type W = crate::W<Chachakey6Spec>;
#[doc = "Field `CHACHAKEY6` reader - bits 63:32 of CHACHA Key"]
pub type Chachakey6R = crate::FieldReader<u32>;
#[doc = "Field `CHACHAKEY6` writer - bits 63:32 of CHACHA Key"]
pub type Chachakey6W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - bits 63:32 of CHACHA Key"]
    #[inline(always)]
    pub fn chachakey6(&self) -> Chachakey6R {
        Chachakey6R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - bits 63:32 of CHACHA Key"]
    #[inline(always)]
    #[must_use]
    pub fn chachakey6(&mut self) -> Chachakey6W<Chachakey6Spec> {
        Chachakey6W::new(self, 0)
    }
}
#[doc = "bits 63:32 of CHACHA Key\n\nYou can [`read`](crate::Reg::read) this register and get [`chachakey6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chachakey6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Chachakey6Spec;
impl crate::RegisterSpec for Chachakey6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chachakey6::R`](R) reader structure"]
impl crate::Readable for Chachakey6Spec {}
#[doc = "`write(|w| ..)` method takes [`chachakey6::W`](W) writer structure"]
impl crate::Writable for Chachakey6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHACHAKEY6 to value 0"]
impl crate::Resettable for Chachakey6Spec {
    const RESET_VALUE: u32 = 0;
}
