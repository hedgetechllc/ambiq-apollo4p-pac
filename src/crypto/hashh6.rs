#[doc = "Register `HASHH6` reader"]
pub type R = crate::R<Hashh6Spec>;
#[doc = "Register `HASHH6` writer"]
pub type W = crate::W<Hashh6Spec>;
#[doc = "Field `HASHH6` reader - 1) Write initial Hash value."]
pub type Hashh6R = crate::FieldReader<u32>;
#[doc = "Field `HASHH6` writer - 1) Write initial Hash value."]
pub type Hashh6W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 1) Write initial Hash value."]
    #[inline(always)]
    pub fn hashh6(&self) -> Hashh6R {
        Hashh6R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 1) Write initial Hash value."]
    #[inline(always)]
    #[must_use]
    pub fn hashh6(&mut self) -> Hashh6W<Hashh6Spec> {
        Hashh6W::new(self, 0)
    }
}
#[doc = "H6 data. can only be written in the following HASH_CONTROL modes: SHA224 SHA256 SHA384 SHA512\n\nYou can [`read`](crate::Reg::read) this register and get [`hashh6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hashh6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hashh6Spec;
impl crate::RegisterSpec for Hashh6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hashh6::R`](R) reader structure"]
impl crate::Readable for Hashh6Spec {}
#[doc = "`write(|w| ..)` method takes [`hashh6::W`](W) writer structure"]
impl crate::Writable for Hashh6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HASHH6 to value 0"]
impl crate::Resettable for Hashh6Spec {
    const RESET_VALUE: u32 = 0;
}
