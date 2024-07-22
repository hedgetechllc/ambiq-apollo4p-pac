#[doc = "Register `SSRAMPWRST` reader"]
pub type R = crate::R<SsrampwrstSpec>;
#[doc = "Register `SSRAMPWRST` writer"]
pub type W = crate::W<SsrampwrstSpec>;
#[doc = "Field `SSRAMPWRST` reader - Each bit corresponds to one 1M SSRAM group. Power Status- 1:ON, 0:OFF See SSRAMPWREN PWRENSSRAM for bit mapping. For example, a value of 2 in this field would mean that SSRAM1 is powered up and SSRAM0 is powered down."]
pub type SsrampwrstR = crate::FieldReader;
#[doc = "Field `SSRAMPWRST` writer - Each bit corresponds to one 1M SSRAM group. Power Status- 1:ON, 0:OFF See SSRAMPWREN PWRENSSRAM for bit mapping. For example, a value of 2 in this field would mean that SSRAM1 is powered up and SSRAM0 is powered down."]
pub type SsrampwrstW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Each bit corresponds to one 1M SSRAM group. Power Status- 1:ON, 0:OFF See SSRAMPWREN PWRENSSRAM for bit mapping. For example, a value of 2 in this field would mean that SSRAM1 is powered up and SSRAM0 is powered down."]
    #[inline(always)]
    pub fn ssrampwrst(&self) -> SsrampwrstR {
        SsrampwrstR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Each bit corresponds to one 1M SSRAM group. Power Status- 1:ON, 0:OFF See SSRAMPWREN PWRENSSRAM for bit mapping. For example, a value of 2 in this field would mean that SSRAM1 is powered up and SSRAM0 is powered down."]
    #[inline(always)]
    #[must_use]
    pub fn ssrampwrst(&mut self) -> SsrampwrstW<SsrampwrstSpec> {
        SsrampwrstW::new(self, 0)
    }
}
#[doc = "It provides the power status for shared sram banks. The status here should reflect the enable provided by the SSRAMPWREN register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ssrampwrst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssrampwrst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SsrampwrstSpec;
impl crate::RegisterSpec for SsrampwrstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ssrampwrst::R`](R) reader structure"]
impl crate::Readable for SsrampwrstSpec {}
#[doc = "`write(|w| ..)` method takes [`ssrampwrst::W`](W) writer structure"]
impl crate::Writable for SsrampwrstSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SSRAMPWRST to value 0x03"]
impl crate::Resettable for SsrampwrstSpec {
    const RESET_VALUE: u32 = 0x03;
}
