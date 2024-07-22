#[doc = "Register `CHACHAIV0` reader"]
pub type R = crate::R<Chachaiv0Spec>;
#[doc = "Register `CHACHAIV0` writer"]
pub type W = crate::W<Chachaiv0Spec>;
#[doc = "Field `CHACHAIV0` reader - bits 31:0 of CHACHA_IV0 register"]
pub type Chachaiv0R = crate::FieldReader<u32>;
#[doc = "Field `CHACHAIV0` writer - bits 31:0 of CHACHA_IV0 register"]
pub type Chachaiv0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - bits 31:0 of CHACHA_IV0 register"]
    #[inline(always)]
    pub fn chachaiv0(&self) -> Chachaiv0R {
        Chachaiv0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - bits 31:0 of CHACHA_IV0 register"]
    #[inline(always)]
    #[must_use]
    pub fn chachaiv0(&mut self) -> Chachaiv0W<Chachaiv0Spec> {
        Chachaiv0W::new(self, 0)
    }
}
#[doc = "bits 31:0 of CHACHA_IV0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`chachaiv0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chachaiv0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Chachaiv0Spec;
impl crate::RegisterSpec for Chachaiv0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chachaiv0::R`](R) reader structure"]
impl crate::Readable for Chachaiv0Spec {}
#[doc = "`write(|w| ..)` method takes [`chachaiv0::W`](W) writer structure"]
impl crate::Writable for Chachaiv0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHACHAIV0 to value 0"]
impl crate::Resettable for Chachaiv0Spec {
    const RESET_VALUE: u32 = 0;
}
