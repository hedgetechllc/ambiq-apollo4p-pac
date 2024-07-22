#[doc = "Register `RESPONSE1` reader"]
pub type R = crate::R<Response1Spec>;
#[doc = "Register `RESPONSE1` writer"]
pub type W = crate::W<Response1Spec>;
#[doc = "Field `CMDRESP1` reader - R\\[\\]
refers to a bit range within the response data as transmitted on the SD Bus, REP\\[\\]
refers to a bit range within the Response register."]
pub type Cmdresp1R = crate::FieldReader<u32>;
#[doc = "Field `CMDRESP1` writer - R\\[\\]
refers to a bit range within the response data as transmitted on the SD Bus, REP\\[\\]
refers to a bit range within the Response register."]
pub type Cmdresp1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - R\\[\\]
refers to a bit range within the response data as transmitted on the SD Bus, REP\\[\\]
refers to a bit range within the Response register."]
    #[inline(always)]
    pub fn cmdresp1(&self) -> Cmdresp1R {
        Cmdresp1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - R\\[\\]
refers to a bit range within the response data as transmitted on the SD Bus, REP\\[\\]
refers to a bit range within the Response register."]
    #[inline(always)]
    #[must_use]
    pub fn cmdresp1(&mut self) -> Cmdresp1W<Response1Spec> {
        Cmdresp1W::new(self, 0)
    }
}
#[doc = "Response1\n\nYou can [`read`](crate::Reg::read) this register and get [`response1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`response1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Response1Spec;
impl crate::RegisterSpec for Response1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`response1::R`](R) reader structure"]
impl crate::Readable for Response1Spec {}
#[doc = "`write(|w| ..)` method takes [`response1::W`](W) writer structure"]
impl crate::Writable for Response1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RESPONSE1 to value 0"]
impl crate::Resettable for Response1Spec {
    const RESET_VALUE: u32 = 0;
}
