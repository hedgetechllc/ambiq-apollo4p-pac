#[doc = "Register `TXENTRIES` reader"]
pub type R = crate::R<TxentriesSpec>;
#[doc = "Register `TXENTRIES` writer"]
pub type W = crate::W<TxentriesSpec>;
#[doc = "Field `TXENTRIES` reader - Number of 32-bit words/entries in TX FIFO"]
pub type TxentriesR = crate::FieldReader;
#[doc = "Field `TXENTRIES` writer - Number of 32-bit words/entries in TX FIFO"]
pub type TxentriesW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Number of 32-bit words/entries in TX FIFO"]
    #[inline(always)]
    pub fn txentries(&self) -> TxentriesR {
        TxentriesR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Number of 32-bit words/entries in TX FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn txentries(&mut self) -> TxentriesW<TxentriesSpec> {
        TxentriesW::new(self, 0)
    }
}
#[doc = "Number of words in TX FIFO\n\nYou can [`read`](crate::Reg::read) this register and get [`txentries::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txentries::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxentriesSpec;
impl crate::RegisterSpec for TxentriesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txentries::R`](R) reader structure"]
impl crate::Readable for TxentriesSpec {}
#[doc = "`write(|w| ..)` method takes [`txentries::W`](W) writer structure"]
impl crate::Writable for TxentriesSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXENTRIES to value 0"]
impl crate::Resettable for TxentriesSpec {
    const RESET_VALUE: u32 = 0;
}
