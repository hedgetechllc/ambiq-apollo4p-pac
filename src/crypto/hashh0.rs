#[doc = "Register `HASHH0` reader"]
pub type R = crate::R<Hashh0Spec>;
#[doc = "Register `HASHH0` writer"]
pub type W = crate::W<Hashh0Spec>;
#[doc = "Field `HASHH0` reader - 1) Write initial Hash value."]
pub type Hashh0R = crate::FieldReader<u32>;
#[doc = "Field `HASHH0` writer - 1) Write initial Hash value."]
pub type Hashh0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 1) Write initial Hash value."]
    #[inline(always)]
    pub fn hashh0(&self) -> Hashh0R {
        Hashh0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 1) Write initial Hash value."]
    #[inline(always)]
    #[must_use]
    pub fn hashh0(&mut self) -> Hashh0W<Hashh0Spec> {
        Hashh0W::new(self, 0)
    }
}
#[doc = "H0 data. can only be written in the following HASH_CONTROL modes: MD5 SHA1 SHA224 SHA256 SHA384 SHA512\n\nYou can [`read`](crate::Reg::read) this register and get [`hashh0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hashh0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hashh0Spec;
impl crate::RegisterSpec for Hashh0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hashh0::R`](R) reader structure"]
impl crate::Readable for Hashh0Spec {}
#[doc = "`write(|w| ..)` method takes [`hashh0::W`](W) writer structure"]
impl crate::Writable for Hashh0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HASHH0 to value 0"]
impl crate::Resettable for Hashh0Spec {
    const RESET_VALUE: u32 = 0;
}
