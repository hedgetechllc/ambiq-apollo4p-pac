#[doc = "Register `TIMEOUT2` reader"]
pub type R = crate::R<Timeout2Spec>;
#[doc = "Register `TIMEOUT2` writer"]
pub type W = crate::W<Timeout2Spec>;
#[doc = "Field `CTHRSTN` reader - Configurable delay from the end of High Speed resume signaling to enabling UTM normal operating mode. Default value of 0x32 corresponds to a delay of 3us. This programmed delay is equivalent to the number of 60MHz clock cycles * 4."]
pub type CthrstnR = crate::FieldReader<u16>;
#[doc = "Field `CTHRSTN` writer - Configurable delay from the end of High Speed resume signaling to enabling UTM normal operating mode. Default value of 0x32 corresponds to a delay of 3us. This programmed delay is equivalent to the number of 60MHz clock cycles * 4."]
pub type CthrstnW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Configurable delay from the end of High Speed resume signaling to enabling UTM normal operating mode. Default value of 0x32 corresponds to a delay of 3us. This programmed delay is equivalent to the number of 60MHz clock cycles * 4."]
    #[inline(always)]
    pub fn cthrstn(&self) -> CthrstnR {
        CthrstnR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Configurable delay from the end of High Speed resume signaling to enabling UTM normal operating mode. Default value of 0x32 corresponds to a delay of 3us. This programmed delay is equivalent to the number of 60MHz clock cycles * 4."]
    #[inline(always)]
    #[must_use]
    pub fn cthrstn(&mut self) -> CthrstnW<Timeout2Spec> {
        CthrstnW::new(self, 0)
    }
}
#[doc = "Holds the configurable delay from the end of High Speed resume signal to enable UTM normal operating mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`timeout2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timeout2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Timeout2Spec;
impl crate::RegisterSpec for Timeout2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timeout2::R`](R) reader structure"]
impl crate::Readable for Timeout2Spec {}
#[doc = "`write(|w| ..)` method takes [`timeout2::W`](W) writer structure"]
impl crate::Writable for Timeout2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMEOUT2 to value 0x32"]
impl crate::Resettable for Timeout2Spec {
    const RESET_VALUE: u32 = 0x32;
}
