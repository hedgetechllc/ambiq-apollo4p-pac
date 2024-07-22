#[doc = "Register `HASHH4` reader"]
pub type R = crate::R<Hashh4Spec>;
#[doc = "Register `HASHH4` writer"]
pub type W = crate::W<Hashh4Spec>;
#[doc = "Field `HASHH4` reader - 1) Write initial Hash value."]
pub type Hashh4R = crate::FieldReader<u32>;
#[doc = "Field `HASHH4` writer - 1) Write initial Hash value."]
pub type Hashh4W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 1) Write initial Hash value."]
    #[inline(always)]
    pub fn hashh4(&self) -> Hashh4R {
        Hashh4R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 1) Write initial Hash value."]
    #[inline(always)]
    #[must_use]
    pub fn hashh4(&mut self) -> Hashh4W<Hashh4Spec> {
        Hashh4W::new(self, 0)
    }
}
#[doc = "H4 data. can only be written in the following HASH_CONTROL modes: SHA1 SHA224 SHA256 SHA384 SHA512\n\nYou can [`read`](crate::Reg::read) this register and get [`hashh4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hashh4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hashh4Spec;
impl crate::RegisterSpec for Hashh4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hashh4::R`](R) reader structure"]
impl crate::Readable for Hashh4Spec {}
#[doc = "`write(|w| ..)` method takes [`hashh4::W`](W) writer structure"]
impl crate::Writable for Hashh4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HASHH4 to value 0"]
impl crate::Resettable for Hashh4Spec {
    const RESET_VALUE: u32 = 0;
}
