#[doc = "Register `HASHH8` reader"]
pub type R = crate::R<Hashh8Spec>;
#[doc = "Register `HASHH8` writer"]
pub type W = crate::W<Hashh8Spec>;
#[doc = "Field `HASHH8` reader - 1) Write initial Hash value."]
pub type Hashh8R = crate::FieldReader<u32>;
#[doc = "Field `HASHH8` writer - 1) Write initial Hash value."]
pub type Hashh8W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 1) Write initial Hash value."]
    #[inline(always)]
    pub fn hashh8(&self) -> Hashh8R {
        Hashh8R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 1) Write initial Hash value."]
    #[inline(always)]
    #[must_use]
    pub fn hashh8(&mut self) -> Hashh8W<Hashh8Spec> {
        Hashh8W::new(self, 0)
    }
}
#[doc = "H8 data. can only be written in the following HASH_CONTROL modes: SHA384 SHA512\n\nYou can [`read`](crate::Reg::read) this register and get [`hashh8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hashh8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hashh8Spec;
impl crate::RegisterSpec for Hashh8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hashh8::R`](R) reader structure"]
impl crate::Readable for Hashh8Spec {}
#[doc = "`write(|w| ..)` method takes [`hashh8::W`](W) writer structure"]
impl crate::Writable for Hashh8Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HASHH8 to value 0"]
impl crate::Resettable for Hashh8Spec {
    const RESET_VALUE: u32 = 0;
}
