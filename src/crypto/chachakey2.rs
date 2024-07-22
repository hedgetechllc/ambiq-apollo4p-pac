#[doc = "Register `CHACHAKEY2` reader"]
pub type R = crate::R<Chachakey2Spec>;
#[doc = "Register `CHACHAKEY2` writer"]
pub type W = crate::W<Chachakey2Spec>;
#[doc = "Field `CHACHAKEY2` reader - bits191:160 of CHACHA Key"]
pub type Chachakey2R = crate::FieldReader<u32>;
#[doc = "Field `CHACHAKEY2` writer - bits191:160 of CHACHA Key"]
pub type Chachakey2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - bits191:160 of CHACHA Key"]
    #[inline(always)]
    pub fn chachakey2(&self) -> Chachakey2R {
        Chachakey2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - bits191:160 of CHACHA Key"]
    #[inline(always)]
    #[must_use]
    pub fn chachakey2(&mut self) -> Chachakey2W<Chachakey2Spec> {
        Chachakey2W::new(self, 0)
    }
}
#[doc = "bits 191:160 of CHACHA Key\n\nYou can [`read`](crate::Reg::read) this register and get [`chachakey2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chachakey2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Chachakey2Spec;
impl crate::RegisterSpec for Chachakey2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chachakey2::R`](R) reader structure"]
impl crate::Readable for Chachakey2Spec {}
#[doc = "`write(|w| ..)` method takes [`chachakey2::W`](W) writer structure"]
impl crate::Writable for Chachakey2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHACHAKEY2 to value 0"]
impl crate::Resettable for Chachakey2Spec {
    const RESET_VALUE: u32 = 0;
}
