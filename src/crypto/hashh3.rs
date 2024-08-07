#[doc = "Register `HASHH3` reader"]
pub type R = crate::R<Hashh3Spec>;
#[doc = "Register `HASHH3` writer"]
pub type W = crate::W<Hashh3Spec>;
#[doc = "Field `HASHH3` reader - 1) Write initial Hash value."]
pub type Hashh3R = crate::FieldReader<u32>;
#[doc = "Field `HASHH3` writer - 1) Write initial Hash value."]
pub type Hashh3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 1) Write initial Hash value."]
    #[inline(always)]
    pub fn hashh3(&self) -> Hashh3R {
        Hashh3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 1) Write initial Hash value."]
    #[inline(always)]
    #[must_use]
    pub fn hashh3(&mut self) -> Hashh3W<Hashh3Spec> {
        Hashh3W::new(self, 0)
    }
}
#[doc = "H3 data. can only be written in the following HASH_CONTROL modes: MD5 SHA1 SHA224 SHA256 SHA384 SHA512\n\nYou can [`read`](crate::Reg::read) this register and get [`hashh3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hashh3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hashh3Spec;
impl crate::RegisterSpec for Hashh3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hashh3::R`](R) reader structure"]
impl crate::Readable for Hashh3Spec {}
#[doc = "`write(|w| ..)` method takes [`hashh3::W`](W) writer structure"]
impl crate::Writable for Hashh3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HASHH3 to value 0"]
impl crate::Resettable for Hashh3Spec {
    const RESET_VALUE: u32 = 0;
}
