#[doc = "Register `RXCHANID` reader"]
pub type R = crate::R<RxchanidSpec>;
#[doc = "Register `RXCHANID` writer"]
pub type W = crate::W<RxchanidSpec>;
#[doc = "Field `RXCHANID` reader - Channel ID value 0-255."]
pub type RxchanidR = crate::FieldReader;
#[doc = "Field `RXCHANID` writer - Channel ID value 0-255."]
pub type RxchanidW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Channel ID value 0-255."]
    #[inline(always)]
    pub fn rxchanid(&self) -> RxchanidR {
        RxchanidR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Channel ID value 0-255."]
    #[inline(always)]
    #[must_use]
    pub fn rxchanid(&mut self) -> RxchanidW<RxchanidSpec> {
        RxchanidW::new(self, 0)
    }
}
#[doc = "Read only received channel identification register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxchanid::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxchanid::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxchanidSpec;
impl crate::RegisterSpec for RxchanidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxchanid::R`](R) reader structure"]
impl crate::Readable for RxchanidSpec {}
#[doc = "`write(|w| ..)` method takes [`rxchanid::W`](W) writer structure"]
impl crate::Writable for RxchanidSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RXCHANID to value 0"]
impl crate::Resettable for RxchanidSpec {
    const RESET_VALUE: u32 = 0;
}
