#[doc = "Register `HASHH5` reader"]
pub type R = crate::R<Hashh5Spec>;
#[doc = "Register `HASHH5` writer"]
pub type W = crate::W<Hashh5Spec>;
#[doc = "Field `HASHH5` reader - 1) Write initial Hash value."]
pub type Hashh5R = crate::FieldReader<u32>;
#[doc = "Field `HASHH5` writer - 1) Write initial Hash value."]
pub type Hashh5W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 1) Write initial Hash value."]
    #[inline(always)]
    pub fn hashh5(&self) -> Hashh5R {
        Hashh5R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 1) Write initial Hash value."]
    #[inline(always)]
    #[must_use]
    pub fn hashh5(&mut self) -> Hashh5W<Hashh5Spec> {
        Hashh5W::new(self, 0)
    }
}
#[doc = "H5 data. can only be written in the following HASH_CONTROL modes: SHA224 SHA256 SHA384 SHA512\n\nYou can [`read`](crate::Reg::read) this register and get [`hashh5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hashh5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hashh5Spec;
impl crate::RegisterSpec for Hashh5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hashh5::R`](R) reader structure"]
impl crate::Readable for Hashh5Spec {}
#[doc = "`write(|w| ..)` method takes [`hashh5::W`](W) writer structure"]
impl crate::Writable for Hashh5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HASHH5 to value 0"]
impl crate::Resettable for Hashh5Spec {
    const RESET_VALUE: u32 = 0;
}
