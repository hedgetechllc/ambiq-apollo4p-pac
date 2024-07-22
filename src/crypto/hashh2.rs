#[doc = "Register `HASHH2` reader"]
pub type R = crate::R<Hashh2Spec>;
#[doc = "Register `HASHH2` writer"]
pub type W = crate::W<Hashh2Spec>;
#[doc = "Field `HASHH2` reader - 1) Write initial Hash value."]
pub type Hashh2R = crate::FieldReader<u32>;
#[doc = "Field `HASHH2` writer - 1) Write initial Hash value."]
pub type Hashh2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 1) Write initial Hash value."]
    #[inline(always)]
    pub fn hashh2(&self) -> Hashh2R {
        Hashh2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 1) Write initial Hash value."]
    #[inline(always)]
    #[must_use]
    pub fn hashh2(&mut self) -> Hashh2W<Hashh2Spec> {
        Hashh2W::new(self, 0)
    }
}
#[doc = "H2 data. can only be written in the following HASH_CONTROL modes: MD5 SHA1 SHA224 SHA256 SHA384 SHA512\n\nYou can [`read`](crate::Reg::read) this register and get [`hashh2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hashh2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hashh2Spec;
impl crate::RegisterSpec for Hashh2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hashh2::R`](R) reader structure"]
impl crate::Readable for Hashh2Spec {}
#[doc = "`write(|w| ..)` method takes [`hashh2::W`](W) writer structure"]
impl crate::Writable for Hashh2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HASHH2 to value 0"]
impl crate::Resettable for Hashh2Spec {
    const RESET_VALUE: u32 = 0;
}
