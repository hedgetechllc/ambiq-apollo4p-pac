#[doc = "Register `RESPONSE2` reader"]
pub type R = crate::R<Response2Spec>;
#[doc = "Register `RESPONSE2` writer"]
pub type W = crate::W<Response2Spec>;
#[doc = "Field `CMDRESP2` reader - R\\[\\]
refers to a bit range within the response data as transmitted on the SD Bus, REP\\[\\]
refers to a bit range within the Response register."]
pub type Cmdresp2R = crate::FieldReader<u32>;
#[doc = "Field `CMDRESP2` writer - R\\[\\]
refers to a bit range within the response data as transmitted on the SD Bus, REP\\[\\]
refers to a bit range within the Response register."]
pub type Cmdresp2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - R\\[\\]
refers to a bit range within the response data as transmitted on the SD Bus, REP\\[\\]
refers to a bit range within the Response register."]
    #[inline(always)]
    pub fn cmdresp2(&self) -> Cmdresp2R {
        Cmdresp2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - R\\[\\]
refers to a bit range within the response data as transmitted on the SD Bus, REP\\[\\]
refers to a bit range within the Response register."]
    #[inline(always)]
    #[must_use]
    pub fn cmdresp2(&mut self) -> Cmdresp2W<Response2Spec> {
        Cmdresp2W::new(self, 0)
    }
}
#[doc = "Response2\n\nYou can [`read`](crate::Reg::read) this register and get [`response2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`response2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Response2Spec;
impl crate::RegisterSpec for Response2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`response2::R`](R) reader structure"]
impl crate::Readable for Response2Spec {}
#[doc = "`write(|w| ..)` method takes [`response2::W`](W) writer structure"]
impl crate::Writable for Response2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RESPONSE2 to value 0"]
impl crate::Resettable for Response2Spec {
    const RESET_VALUE: u32 = 0;
}
