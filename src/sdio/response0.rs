#[doc = "Register `RESPONSE0` reader"]
pub type R = crate::R<Response0Spec>;
#[doc = "Register `RESPONSE0` writer"]
pub type W = crate::W<Response0Spec>;
#[doc = "Field `CMDRESP0` reader - R\\[\\]
refers to a bit range within the response data as transmitted on the SD Bus, REP\\[\\]
refers to a bit range within the Response register."]
pub type Cmdresp0R = crate::FieldReader<u32>;
#[doc = "Field `CMDRESP0` writer - R\\[\\]
refers to a bit range within the response data as transmitted on the SD Bus, REP\\[\\]
refers to a bit range within the Response register."]
pub type Cmdresp0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - R\\[\\]
refers to a bit range within the response data as transmitted on the SD Bus, REP\\[\\]
refers to a bit range within the Response register."]
    #[inline(always)]
    pub fn cmdresp0(&self) -> Cmdresp0R {
        Cmdresp0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - R\\[\\]
refers to a bit range within the response data as transmitted on the SD Bus, REP\\[\\]
refers to a bit range within the Response register."]
    #[inline(always)]
    #[must_use]
    pub fn cmdresp0(&mut self) -> Cmdresp0W<Response0Spec> {
        Cmdresp0W::new(self, 0)
    }
}
#[doc = "Response0\n\nYou can [`read`](crate::Reg::read) this register and get [`response0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`response0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Response0Spec;
impl crate::RegisterSpec for Response0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`response0::R`](R) reader structure"]
impl crate::Readable for Response0Spec {}
#[doc = "`write(|w| ..)` method takes [`response0::W`](W) writer structure"]
impl crate::Writable for Response0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RESPONSE0 to value 0"]
impl crate::Resettable for Response0Spec {
    const RESET_VALUE: u32 = 0;
}
