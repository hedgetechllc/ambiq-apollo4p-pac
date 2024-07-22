#[doc = "Register `HASHH7` reader"]
pub type R = crate::R<Hashh7Spec>;
#[doc = "Register `HASHH7` writer"]
pub type W = crate::W<Hashh7Spec>;
#[doc = "Field `HASHH7` reader - 1) Write initial Hash value."]
pub type Hashh7R = crate::FieldReader<u32>;
#[doc = "Field `HASHH7` writer - 1) Write initial Hash value."]
pub type Hashh7W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 1) Write initial Hash value."]
    #[inline(always)]
    pub fn hashh7(&self) -> Hashh7R {
        Hashh7R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 1) Write initial Hash value."]
    #[inline(always)]
    #[must_use]
    pub fn hashh7(&mut self) -> Hashh7W<Hashh7Spec> {
        Hashh7W::new(self, 0)
    }
}
#[doc = "H7 data. can only be written in the following HASH_CONTROL modes: SHA224 SHA256 SHA384 SHA512\n\nYou can [`read`](crate::Reg::read) this register and get [`hashh7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hashh7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hashh7Spec;
impl crate::RegisterSpec for Hashh7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hashh7::R`](R) reader structure"]
impl crate::Readable for Hashh7Spec {}
#[doc = "`write(|w| ..)` method takes [`hashh7::W`](W) writer structure"]
impl crate::Writable for Hashh7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HASHH7 to value 0"]
impl crate::Resettable for Hashh7Spec {
    const RESET_VALUE: u32 = 0;
}
