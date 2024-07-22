#[doc = "Register `IPBIRPT` reader"]
pub type R = crate::R<IpbirptSpec>;
#[doc = "Register `IPBIRPT` writer"]
pub type W = crate::W<IpbirptSpec>;
#[doc = "Field `RXFFM` reader - Receive FIFO interrupt mask. Will assert interrupt when = 1 and RXFFI is asserted"]
pub type RxffmR = crate::BitReader;
#[doc = "Field `RXFFM` writer - Receive FIFO interrupt mask. Will assert interrupt when = 1 and RXFFI is asserted"]
pub type RxffmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFFM` reader - Transmit FIFO interrupt mask. Will assert interrupt when = 1 and TXFFI is asserted"]
pub type TxffmR = crate::BitReader;
#[doc = "Field `TXFFM` writer - Transmit FIFO interrupt mask. Will assert interrupt when = 1 and TXFFI is asserted"]
pub type TxffmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFM` reader - Receive FIFO interrupt mask. Will assert interrupt when = 1 and RXFI is asserted"]
pub type RxfmR = crate::BitReader;
#[doc = "Field `RXFM` writer - Receive FIFO interrupt mask. Will assert interrupt when = 1 and RXFI is asserted"]
pub type RxfmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXEM` reader - Transmit FIFO interrupt mask. Will assert interrupt when = 1 and TXEI is asserted"]
pub type TxemR = crate::BitReader;
#[doc = "Field `TXEM` writer - Transmit FIFO interrupt mask. Will assert interrupt when = 1 and TXEI is asserted"]
pub type TxemW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXDMAM` reader - Receive FIFO interrupt mask. Will assert interrupt when = 1 and cimdmareq_rx is asserted"]
pub type RxdmamR = crate::BitReader;
#[doc = "Field `RXDMAM` writer - Receive FIFO interrupt mask. Will assert interrupt when = 1 and cimdmareq_rx is asserted"]
pub type RxdmamW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXDMAM` reader - Transmit FIFO interrupt mask. Will assert interrupt when = 1 and cimdmareq_tx is asserted"]
pub type TxdmamR = crate::BitReader;
#[doc = "Field `TXDMAM` writer - Transmit FIFO interrupt mask. Will assert interrupt when = 1 and cimdmareq_tx is asserted"]
pub type TxdmamW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFFI` reader - Receive fifo high limit interrupt"]
pub type RxffiR = crate::BitReader;
#[doc = "Field `RXFFI` writer - Receive fifo high limit interrupt"]
pub type RxffiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFFI` reader - Transmit fifo low limit interrupt"]
pub type TxffiR = crate::BitReader;
#[doc = "Field `TXFFI` writer - Transmit fifo low limit interrupt"]
pub type TxffiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFI` reader - RX Full interrupt. RX unit attempted to write to a full FIFO"]
pub type RxfiR = crate::BitReader;
#[doc = "Field `RXFI` writer - RX Full interrupt. RX unit attempted to write to a full FIFO"]
pub type RxfiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXEI` reader - TX Empty interrupt. TX unit attempted to read an empty FIFO"]
pub type TxeiR = crate::BitReader;
#[doc = "Field `TXEI` writer - TX Empty interrupt. TX unit attempted to read an empty FIFO"]
pub type TxeiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXDMAI` reader - RX dma interrupt"]
pub type RxdmaiR = crate::BitReader;
#[doc = "Field `RXDMAI` writer - RX dma interrupt"]
pub type RxdmaiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXDMAI` reader - TX dma interrupt"]
pub type TxdmaiR = crate::BitReader;
#[doc = "Field `TXDMAI` writer - TX dma interrupt"]
pub type TxdmaiW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Receive FIFO interrupt mask. Will assert interrupt when = 1 and RXFFI is asserted"]
    #[inline(always)]
    pub fn rxffm(&self) -> RxffmR {
        RxffmR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit FIFO interrupt mask. Will assert interrupt when = 1 and TXFFI is asserted"]
    #[inline(always)]
    pub fn txffm(&self) -> TxffmR {
        TxffmR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive FIFO interrupt mask. Will assert interrupt when = 1 and RXFI is asserted"]
    #[inline(always)]
    pub fn rxfm(&self) -> RxfmR {
        RxfmR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit FIFO interrupt mask. Will assert interrupt when = 1 and TXEI is asserted"]
    #[inline(always)]
    pub fn txem(&self) -> TxemR {
        TxemR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive FIFO interrupt mask. Will assert interrupt when = 1 and cimdmareq_rx is asserted"]
    #[inline(always)]
    pub fn rxdmam(&self) -> RxdmamR {
        RxdmamR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit FIFO interrupt mask. Will assert interrupt when = 1 and cimdmareq_tx is asserted"]
    #[inline(always)]
    pub fn txdmam(&self) -> TxdmamR {
        TxdmamR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 16 - Receive fifo high limit interrupt"]
    #[inline(always)]
    pub fn rxffi(&self) -> RxffiR {
        RxffiR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Transmit fifo low limit interrupt"]
    #[inline(always)]
    pub fn txffi(&self) -> TxffiR {
        TxffiR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - RX Full interrupt. RX unit attempted to write to a full FIFO"]
    #[inline(always)]
    pub fn rxfi(&self) -> RxfiR {
        RxfiR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - TX Empty interrupt. TX unit attempted to read an empty FIFO"]
    #[inline(always)]
    pub fn txei(&self) -> TxeiR {
        TxeiR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - RX dma interrupt"]
    #[inline(always)]
    pub fn rxdmai(&self) -> RxdmaiR {
        RxdmaiR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - TX dma interrupt"]
    #[inline(always)]
    pub fn txdmai(&self) -> TxdmaiR {
        TxdmaiR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive FIFO interrupt mask. Will assert interrupt when = 1 and RXFFI is asserted"]
    #[inline(always)]
    #[must_use]
    pub fn rxffm(&mut self) -> RxffmW<IpbirptSpec> {
        RxffmW::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit FIFO interrupt mask. Will assert interrupt when = 1 and TXFFI is asserted"]
    #[inline(always)]
    #[must_use]
    pub fn txffm(&mut self) -> TxffmW<IpbirptSpec> {
        TxffmW::new(self, 1)
    }
    #[doc = "Bit 2 - Receive FIFO interrupt mask. Will assert interrupt when = 1 and RXFI is asserted"]
    #[inline(always)]
    #[must_use]
    pub fn rxfm(&mut self) -> RxfmW<IpbirptSpec> {
        RxfmW::new(self, 2)
    }
    #[doc = "Bit 3 - Transmit FIFO interrupt mask. Will assert interrupt when = 1 and TXEI is asserted"]
    #[inline(always)]
    #[must_use]
    pub fn txem(&mut self) -> TxemW<IpbirptSpec> {
        TxemW::new(self, 3)
    }
    #[doc = "Bit 4 - Receive FIFO interrupt mask. Will assert interrupt when = 1 and cimdmareq_rx is asserted"]
    #[inline(always)]
    #[must_use]
    pub fn rxdmam(&mut self) -> RxdmamW<IpbirptSpec> {
        RxdmamW::new(self, 4)
    }
    #[doc = "Bit 5 - Transmit FIFO interrupt mask. Will assert interrupt when = 1 and cimdmareq_tx is asserted"]
    #[inline(always)]
    #[must_use]
    pub fn txdmam(&mut self) -> TxdmamW<IpbirptSpec> {
        TxdmamW::new(self, 5)
    }
    #[doc = "Bit 16 - Receive fifo high limit interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rxffi(&mut self) -> RxffiW<IpbirptSpec> {
        RxffiW::new(self, 16)
    }
    #[doc = "Bit 17 - Transmit fifo low limit interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn txffi(&mut self) -> TxffiW<IpbirptSpec> {
        TxffiW::new(self, 17)
    }
    #[doc = "Bit 18 - RX Full interrupt. RX unit attempted to write to a full FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn rxfi(&mut self) -> RxfiW<IpbirptSpec> {
        RxfiW::new(self, 18)
    }
    #[doc = "Bit 19 - TX Empty interrupt. TX unit attempted to read an empty FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn txei(&mut self) -> TxeiW<IpbirptSpec> {
        TxeiW::new(self, 19)
    }
    #[doc = "Bit 20 - RX dma interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rxdmai(&mut self) -> RxdmaiW<IpbirptSpec> {
        RxdmaiW::new(self, 20)
    }
    #[doc = "Bit 21 - TX dma interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn txdmai(&mut self) -> TxdmaiW<IpbirptSpec> {
        TxdmaiW::new(self, 21)
    }
}
#[doc = "Additional mask and status registers for the IPB core.\n\nYou can [`read`](crate::Reg::read) this register and get [`ipbirpt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipbirpt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IpbirptSpec;
impl crate::RegisterSpec for IpbirptSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipbirpt::R`](R) reader structure"]
impl crate::Readable for IpbirptSpec {}
#[doc = "`write(|w| ..)` method takes [`ipbirpt::W`](W) writer structure"]
impl crate::Writable for IpbirptSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IPBIRPT to value 0"]
impl crate::Resettable for IpbirptSpec {
    const RESET_VALUE: u32 = 0;
}
