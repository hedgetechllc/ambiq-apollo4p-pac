#[doc = "Register `TXFIFOSTATUS` reader"]
pub type R = crate::R<TxfifostatusSpec>;
#[doc = "Register `TXFIFOSTATUS` writer"]
pub type W = crate::W<TxfifostatusSpec>;
#[doc = "Field `TXFIFOCNT` reader - The count of the number of samples currently in the transmit FIFO."]
pub type TxfifocntR = crate::FieldReader<u32>;
#[doc = "Field `TXFIFOCNT` writer - The count of the number of samples currently in the transmit FIFO."]
pub type TxfifocntW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
#[doc = "Field `TXFIFOFULL` reader - Transmit FIFO full bit. a 1 indicates the transmit FIFO is full."]
pub type TxfifofullR = crate::BitReader;
#[doc = "Field `TXFIFOFULL` writer - Transmit FIFO full bit. a 1 indicates the transmit FIFO is full."]
pub type TxfifofullW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:27 - The count of the number of samples currently in the transmit FIFO."]
    #[inline(always)]
    pub fn txfifocnt(&self) -> TxfifocntR {
        TxfifocntR::new(self.bits & 0x0fff_ffff)
    }
    #[doc = "Bit 28 - Transmit FIFO full bit. a 1 indicates the transmit FIFO is full."]
    #[inline(always)]
    pub fn txfifofull(&self) -> TxfifofullR {
        TxfifofullR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:27 - The count of the number of samples currently in the transmit FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn txfifocnt(&mut self) -> TxfifocntW<TxfifostatusSpec> {
        TxfifocntW::new(self, 0)
    }
    #[doc = "Bit 28 - Transmit FIFO full bit. a 1 indicates the transmit FIFO is full."]
    #[inline(always)]
    #[must_use]
    pub fn txfifofull(&mut self) -> TxfifofullW<TxfifostatusSpec> {
        TxfifofullW::new(self, 28)
    }
}
#[doc = "Holds the number of samples currently in the transmit FIFO, and the full condition flag\n\nYou can [`read`](crate::Reg::read) this register and get [`txfifostatus::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txfifostatus::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxfifostatusSpec;
impl crate::RegisterSpec for TxfifostatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txfifostatus::R`](R) reader structure"]
impl crate::Readable for TxfifostatusSpec {}
#[doc = "`write(|w| ..)` method takes [`txfifostatus::W`](W) writer structure"]
impl crate::Writable for TxfifostatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXFIFOSTATUS to value 0"]
impl crate::Resettable for TxfifostatusSpec {
    const RESET_VALUE: u32 = 0;
}
