#[doc = "Register `LPRXTO` reader"]
pub type R = crate::R<LprxtoSpec>;
#[doc = "Register `LPRXTO` writer"]
pub type W = crate::W<LprxtoSpec>;
#[doc = "Field `TOCHKRVS` reader - Timeout value to be checked for reverse communication. If the counter expires, processor is interrupted with LP_Rx_timeout interrupt.The timeout value is protocol specific. Time out value is calculated from txclkesc(50ns)."]
pub type TochkrvsR = crate::FieldReader<u32>;
#[doc = "Field `TOCHKRVS` writer - Timeout value to be checked for reverse communication. If the counter expires, processor is interrupted with LP_Rx_timeout interrupt.The timeout value is protocol specific. Time out value is calculated from txclkesc(50ns)."]
pub type TochkrvsW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - Timeout value to be checked for reverse communication. If the counter expires, processor is interrupted with LP_Rx_timeout interrupt.The timeout value is protocol specific. Time out value is calculated from txclkesc(50ns)."]
    #[inline(always)]
    pub fn tochkrvs(&self) -> TochkrvsR {
        TochkrvsR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Timeout value to be checked for reverse communication. If the counter expires, processor is interrupted with LP_Rx_timeout interrupt.The timeout value is protocol specific. Time out value is calculated from txclkesc(50ns)."]
    #[inline(always)]
    #[must_use]
    pub fn tochkrvs(&mut self) -> TochkrvsW<LprxtoSpec> {
        TochkrvsW::new(self, 0)
    }
}
#[doc = "Timeout value to be checked for reverse communicationl\n\nYou can [`read`](crate::Reg::read) this register and get [`lprxto::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lprxto::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LprxtoSpec;
impl crate::RegisterSpec for LprxtoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lprxto::R`](R) reader structure"]
impl crate::Readable for LprxtoSpec {}
#[doc = "`write(|w| ..)` method takes [`lprxto::W`](W) writer structure"]
impl crate::Writable for LprxtoSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LPRXTO to value 0xffff"]
impl crate::Resettable for LprxtoSpec {
    const RESET_VALUE: u32 = 0xffff;
}
