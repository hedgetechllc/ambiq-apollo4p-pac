#[doc = "Register `CHACHAIV1` reader"]
pub type R = crate::R<Chachaiv1Spec>;
#[doc = "Register `CHACHAIV1` writer"]
pub type W = crate::W<Chachaiv1Spec>;
#[doc = "Field `CHACHAIV1` reader - bits 31:0 of CHACHA_IV1 register"]
pub type Chachaiv1R = crate::FieldReader<u32>;
#[doc = "Field `CHACHAIV1` writer - bits 31:0 of CHACHA_IV1 register"]
pub type Chachaiv1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - bits 31:0 of CHACHA_IV1 register"]
    #[inline(always)]
    pub fn chachaiv1(&self) -> Chachaiv1R {
        Chachaiv1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - bits 31:0 of CHACHA_IV1 register"]
    #[inline(always)]
    #[must_use]
    pub fn chachaiv1(&mut self) -> Chachaiv1W<Chachaiv1Spec> {
        Chachaiv1W::new(self, 0)
    }
}
#[doc = "bits 31:0 of CHACHA_IV1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`chachaiv1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chachaiv1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Chachaiv1Spec;
impl crate::RegisterSpec for Chachaiv1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chachaiv1::R`](R) reader structure"]
impl crate::Readable for Chachaiv1Spec {}
#[doc = "`write(|w| ..)` method takes [`chachaiv1::W`](W) writer structure"]
impl crate::Writable for Chachaiv1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHACHAIV1 to value 0"]
impl crate::Resettable for Chachaiv1Spec {
    const RESET_VALUE: u32 = 0;
}
