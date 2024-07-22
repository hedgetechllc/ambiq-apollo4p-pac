#[doc = "Register `HASHH1` reader"]
pub type R = crate::R<Hashh1Spec>;
#[doc = "Register `HASHH1` writer"]
pub type W = crate::W<Hashh1Spec>;
#[doc = "Field `HASHH1` reader - 1) Write initial Hash value."]
pub type Hashh1R = crate::FieldReader<u32>;
#[doc = "Field `HASHH1` writer - 1) Write initial Hash value."]
pub type Hashh1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 1) Write initial Hash value."]
    #[inline(always)]
    pub fn hashh1(&self) -> Hashh1R {
        Hashh1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 1) Write initial Hash value."]
    #[inline(always)]
    #[must_use]
    pub fn hashh1(&mut self) -> Hashh1W<Hashh1Spec> {
        Hashh1W::new(self, 0)
    }
}
#[doc = "H1 data. can only be written in the following HASH_CONTROL modes: MD5 SHA1 SHA224 SHA256 SHA384 SHA512\n\nYou can [`read`](crate::Reg::read) this register and get [`hashh1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hashh1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hashh1Spec;
impl crate::RegisterSpec for Hashh1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hashh1::R`](R) reader structure"]
impl crate::Readable for Hashh1Spec {}
#[doc = "`write(|w| ..)` method takes [`hashh1::W`](W) writer structure"]
impl crate::Writable for Hashh1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HASHH1 to value 0"]
impl crate::Resettable for Hashh1Spec {
    const RESET_VALUE: u32 = 0;
}
