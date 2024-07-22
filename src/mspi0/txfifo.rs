#[doc = "Register `TXFIFO` reader"]
pub type R = crate::R<TxfifoSpec>;
#[doc = "Register `TXFIFO` writer"]
pub type W = crate::W<TxfifoSpec>;
#[doc = "Field `TXFIFO` reader - Data to be transmitted. Data should normally be aligned to the LSB (pad the upper bits with zeros) unless BIGENDIAN is set."]
pub type TxfifoR = crate::FieldReader<u32>;
#[doc = "Field `TXFIFO` writer - Data to be transmitted. Data should normally be aligned to the LSB (pad the upper bits with zeros) unless BIGENDIAN is set."]
pub type TxfifoW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Data to be transmitted. Data should normally be aligned to the LSB (pad the upper bits with zeros) unless BIGENDIAN is set."]
    #[inline(always)]
    pub fn txfifo(&self) -> TxfifoR {
        TxfifoR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data to be transmitted. Data should normally be aligned to the LSB (pad the upper bits with zeros) unless BIGENDIAN is set."]
    #[inline(always)]
    #[must_use]
    pub fn txfifo(&mut self) -> TxfifoW<TxfifoSpec> {
        TxfifoW::new(self, 0)
    }
}
#[doc = "TX Data FIFO\n\nYou can [`read`](crate::Reg::read) this register and get [`txfifo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txfifo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxfifoSpec;
impl crate::RegisterSpec for TxfifoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txfifo::R`](R) reader structure"]
impl crate::Readable for TxfifoSpec {}
#[doc = "`write(|w| ..)` method takes [`txfifo::W`](W) writer structure"]
impl crate::Writable for TxfifoSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXFIFO to value 0"]
impl crate::Resettable for TxfifoSpec {
    const RESET_VALUE: u32 = 0;
}
