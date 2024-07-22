#[doc = "Register `TXCHANID` reader"]
pub type R = crate::R<TxchanidSpec>;
#[doc = "Register `TXCHANID` writer"]
pub type W = crate::W<TxchanidSpec>;
#[doc = "Field `TXCHANID` reader - Channel ID value 0-255."]
pub type TxchanidR = crate::FieldReader;
#[doc = "Field `TXCHANID` writer - Channel ID value 0-255."]
pub type TxchanidW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Channel ID value 0-255."]
    #[inline(always)]
    pub fn txchanid(&self) -> TxchanidR {
        TxchanidR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Channel ID value 0-255."]
    #[inline(always)]
    #[must_use]
    pub fn txchanid(&mut self) -> TxchanidW<TxchanidSpec> {
        TxchanidW::new(self, 0)
    }
}
#[doc = "Channel ID used for the next audio sample to be written to the data transmission register\n\nYou can [`read`](crate::Reg::read) this register and get [`txchanid::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txchanid::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxchanidSpec;
impl crate::RegisterSpec for TxchanidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txchanid::R`](R) reader structure"]
impl crate::Readable for TxchanidSpec {}
#[doc = "`write(|w| ..)` method takes [`txchanid::W`](W) writer structure"]
impl crate::Writable for TxchanidSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXCHANID to value 0"]
impl crate::Resettable for TxchanidSpec {
    const RESET_VALUE: u32 = 0;
}
