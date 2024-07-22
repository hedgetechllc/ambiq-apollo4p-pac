#[doc = "Register `RESPONSE3` reader"]
pub type R = crate::R<Response3Spec>;
#[doc = "Register `RESPONSE3` writer"]
pub type W = crate::W<Response3Spec>;
#[doc = "Field `CMDRESP3` reader - R\\[\\]
refers to a bit range within the response data as transmitted on the SD Bus, REP\\[\\]
refers to a bit range within the Response register."]
pub type Cmdresp3R = crate::FieldReader<u32>;
#[doc = "Field `CMDRESP3` writer - R\\[\\]
refers to a bit range within the response data as transmitted on the SD Bus, REP\\[\\]
refers to a bit range within the Response register."]
pub type Cmdresp3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - R\\[\\]
refers to a bit range within the response data as transmitted on the SD Bus, REP\\[\\]
refers to a bit range within the Response register."]
    #[inline(always)]
    pub fn cmdresp3(&self) -> Cmdresp3R {
        Cmdresp3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - R\\[\\]
refers to a bit range within the response data as transmitted on the SD Bus, REP\\[\\]
refers to a bit range within the Response register."]
    #[inline(always)]
    #[must_use]
    pub fn cmdresp3(&mut self) -> Cmdresp3W<Response3Spec> {
        Cmdresp3W::new(self, 0)
    }
}
#[doc = "Response3\n\nYou can [`read`](crate::Reg::read) this register and get [`response3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`response3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Response3Spec;
impl crate::RegisterSpec for Response3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`response3::R`](R) reader structure"]
impl crate::Readable for Response3Spec {}
#[doc = "`write(|w| ..)` method takes [`response3::W`](W) writer structure"]
impl crate::Writable for Response3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RESPONSE3 to value 0"]
impl crate::Resettable for Response3Spec {
    const RESET_VALUE: u32 = 0;
}
