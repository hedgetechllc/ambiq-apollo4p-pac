#[doc = "Register `RXENTRIES` reader"]
pub type R = crate::R<RxentriesSpec>;
#[doc = "Register `RXENTRIES` writer"]
pub type W = crate::W<RxentriesSpec>;
#[doc = "Field `RXENTRIES` reader - Number of 32-bit words/entries in RX FIFO"]
pub type RxentriesR = crate::FieldReader;
#[doc = "Field `RXENTRIES` writer - Number of 32-bit words/entries in RX FIFO"]
pub type RxentriesW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Number of 32-bit words/entries in RX FIFO"]
    #[inline(always)]
    pub fn rxentries(&self) -> RxentriesR {
        RxentriesR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Number of 32-bit words/entries in RX FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn rxentries(&mut self) -> RxentriesW<RxentriesSpec> {
        RxentriesW::new(self, 0)
    }
}
#[doc = "Number of words in RX FIFO\n\nYou can [`read`](crate::Reg::read) this register and get [`rxentries::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxentries::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxentriesSpec;
impl crate::RegisterSpec for RxentriesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxentries::R`](R) reader structure"]
impl crate::Readable for RxentriesSpec {}
#[doc = "`write(|w| ..)` method takes [`rxentries::W`](W) writer structure"]
impl crate::Writable for RxentriesSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RXENTRIES to value 0"]
impl crate::Resettable for RxentriesSpec {
    const RESET_VALUE: u32 = 0;
}
