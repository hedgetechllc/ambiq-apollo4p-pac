#[doc = "Register `HSTXTIMEOUT` reader"]
pub type R = crate::R<HstxtimeoutSpec>;
#[doc = "Register `HSTXTIMEOUT` writer"]
pub type W = crate::W<HstxtimeoutSpec>;
#[doc = "Field `MAXDURTOCNT` reader - The maximum duration allowed for the DSI host to remain in high speed mode for a transmission. If the counter expires, processor is interrupted with HS_Tx_timeout interrupt"]
pub type MaxdurtocntR = crate::FieldReader<u32>;
#[doc = "Field `MAXDURTOCNT` writer - The maximum duration allowed for the DSI host to remain in high speed mode for a transmission. If the counter expires, processor is interrupted with HS_Tx_timeout interrupt"]
pub type MaxdurtocntW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - The maximum duration allowed for the DSI host to remain in high speed mode for a transmission. If the counter expires, processor is interrupted with HS_Tx_timeout interrupt"]
    #[inline(always)]
    pub fn maxdurtocnt(&self) -> MaxdurtocntR {
        MaxdurtocntR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - The maximum duration allowed for the DSI host to remain in high speed mode for a transmission. If the counter expires, processor is interrupted with HS_Tx_timeout interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn maxdurtocnt(&mut self) -> MaxdurtocntW<HstxtimeoutSpec> {
        MaxdurtocntW::new(self, 0)
    }
}
#[doc = "Maximum duration allow for the DSi host to remain in High speed mode for transmission.\n\nYou can [`read`](crate::Reg::read) this register and get [`hstxtimeout::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstxtimeout::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HstxtimeoutSpec;
impl crate::RegisterSpec for HstxtimeoutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hstxtimeout::R`](R) reader structure"]
impl crate::Readable for HstxtimeoutSpec {}
#[doc = "`write(|w| ..)` method takes [`hstxtimeout::W`](W) writer structure"]
impl crate::Writable for HstxtimeoutSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HSTXTIMEOUT to value 0xffff"]
impl crate::Resettable for HstxtimeoutSpec {
    const RESET_VALUE: u32 = 0xffff;
}
