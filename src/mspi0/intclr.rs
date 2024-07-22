#[doc = "Register `INTCLR` reader"]
pub type R = crate::R<IntclrSpec>;
#[doc = "Register `INTCLR` writer"]
pub type W = crate::W<IntclrSpec>;
#[doc = "Field `CMDCMP` reader - Transfer complete. Note that DMA and CQ operations are layered, so CMDCMP, DCMP, and CQ* can all be signaled simultaneously."]
pub type CmdcmpR = crate::BitReader;
#[doc = "Field `CMDCMP` writer - Transfer complete. Note that DMA and CQ operations are layered, so CMDCMP, DCMP, and CQ* can all be signaled simultaneously."]
pub type CmdcmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXE` reader - Transmit FIFO empty."]
pub type TxeR = crate::BitReader;
#[doc = "Field `TXE` writer - Transmit FIFO empty."]
pub type TxeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXO` reader - Transmit FIFO Overflow (only occurs when SW writes to a full FIFO)."]
pub type TxoR = crate::BitReader;
#[doc = "Field `TXO` writer - Transmit FIFO Overflow (only occurs when SW writes to a full FIFO)."]
pub type TxoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXU` reader - Receive FIFO underflow (only occurs when SW reads from an empty FIFO)"]
pub type RxuR = crate::BitReader;
#[doc = "Field `RXU` writer - Receive FIFO underflow (only occurs when SW reads from an empty FIFO)"]
pub type RxuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXO` reader - Receive FIFO overflow (cannot happen in MSPI design -- MSPI bus pins will stall)"]
pub type RxoR = crate::BitReader;
#[doc = "Field `RXO` writer - Receive FIFO overflow (cannot happen in MSPI design -- MSPI bus pins will stall)"]
pub type RxoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXF` reader - Receive FIFO full"]
pub type RxfR = crate::BitReader;
#[doc = "Field `RXF` writer - Receive FIFO full"]
pub type RxfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCMP` reader - DMA Complete Interrupt"]
pub type DcmpR = crate::BitReader;
#[doc = "Field `DCMP` writer - DMA Complete Interrupt"]
pub type DcmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DERR` reader - DMA Error Interrupt"]
pub type DerrR = crate::BitReader;
#[doc = "Field `DERR` writer - DMA Error Interrupt"]
pub type DerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CQCMP` reader - Command Queue Complete Interrupt"]
pub type CqcmpR = crate::BitReader;
#[doc = "Field `CQCMP` writer - Command Queue Complete Interrupt"]
pub type CqcmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CQUPD` reader - Command Queue Update Interrupt. Issued whenever the CQ performs an operation where address bit\\[0\\]
is set. Useful for triggering CURIDX interrupts."]
pub type CqupdR = crate::BitReader;
#[doc = "Field `CQUPD` writer - Command Queue Update Interrupt. Issued whenever the CQ performs an operation where address bit\\[0\\]
is set. Useful for triggering CURIDX interrupts."]
pub type CqupdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CQPAUSED` reader - Command Queue is Paused."]
pub type CqpausedR = crate::BitReader;
#[doc = "Field `CQPAUSED` writer - Command Queue is Paused."]
pub type CqpausedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CQERR` reader - Command Queue Error Interrupt"]
pub type CqerrR = crate::BitReader;
#[doc = "Field `CQERR` writer - Command Queue Error Interrupt"]
pub type CqerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCRERR` reader - Scrambling Alignment Error. Scrambling operations must be aligned to word (4-byte) start address."]
pub type ScrerrR = crate::BitReader;
#[doc = "Field `SCRERR` writer - Scrambling Alignment Error. Scrambling operations must be aligned to word (4-byte) start address."]
pub type ScrerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APBDMAERR` reader - MSPI is dma target as well as dma source, which may result in deadlock."]
pub type ApbdmaerrR = crate::BitReader;
#[doc = "Field `APBDMAERR` writer - MSPI is dma target as well as dma source, which may result in deadlock."]
pub type ApbdmaerrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transfer complete. Note that DMA and CQ operations are layered, so CMDCMP, DCMP, and CQ* can all be signaled simultaneously."]
    #[inline(always)]
    pub fn cmdcmp(&self) -> CmdcmpR {
        CmdcmpR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit FIFO empty."]
    #[inline(always)]
    pub fn txe(&self) -> TxeR {
        TxeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit FIFO Overflow (only occurs when SW writes to a full FIFO)."]
    #[inline(always)]
    pub fn txo(&self) -> TxoR {
        TxoR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive FIFO underflow (only occurs when SW reads from an empty FIFO)"]
    #[inline(always)]
    pub fn rxu(&self) -> RxuR {
        RxuR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive FIFO overflow (cannot happen in MSPI design -- MSPI bus pins will stall)"]
    #[inline(always)]
    pub fn rxo(&self) -> RxoR {
        RxoR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Receive FIFO full"]
    #[inline(always)]
    pub fn rxf(&self) -> RxfR {
        RxfR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DMA Complete Interrupt"]
    #[inline(always)]
    pub fn dcmp(&self) -> DcmpR {
        DcmpR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DMA Error Interrupt"]
    #[inline(always)]
    pub fn derr(&self) -> DerrR {
        DerrR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Command Queue Complete Interrupt"]
    #[inline(always)]
    pub fn cqcmp(&self) -> CqcmpR {
        CqcmpR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Command Queue Update Interrupt. Issued whenever the CQ performs an operation where address bit\\[0\\]
is set. Useful for triggering CURIDX interrupts."]
    #[inline(always)]
    pub fn cqupd(&self) -> CqupdR {
        CqupdR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Command Queue is Paused."]
    #[inline(always)]
    pub fn cqpaused(&self) -> CqpausedR {
        CqpausedR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Command Queue Error Interrupt"]
    #[inline(always)]
    pub fn cqerr(&self) -> CqerrR {
        CqerrR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Scrambling Alignment Error. Scrambling operations must be aligned to word (4-byte) start address."]
    #[inline(always)]
    pub fn screrr(&self) -> ScrerrR {
        ScrerrR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - MSPI is dma target as well as dma source, which may result in deadlock."]
    #[inline(always)]
    pub fn apbdmaerr(&self) -> ApbdmaerrR {
        ApbdmaerrR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer complete. Note that DMA and CQ operations are layered, so CMDCMP, DCMP, and CQ* can all be signaled simultaneously."]
    #[inline(always)]
    #[must_use]
    pub fn cmdcmp(&mut self) -> CmdcmpW<IntclrSpec> {
        CmdcmpW::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit FIFO empty."]
    #[inline(always)]
    #[must_use]
    pub fn txe(&mut self) -> TxeW<IntclrSpec> {
        TxeW::new(self, 1)
    }
    #[doc = "Bit 2 - Transmit FIFO Overflow (only occurs when SW writes to a full FIFO)."]
    #[inline(always)]
    #[must_use]
    pub fn txo(&mut self) -> TxoW<IntclrSpec> {
        TxoW::new(self, 2)
    }
    #[doc = "Bit 3 - Receive FIFO underflow (only occurs when SW reads from an empty FIFO)"]
    #[inline(always)]
    #[must_use]
    pub fn rxu(&mut self) -> RxuW<IntclrSpec> {
        RxuW::new(self, 3)
    }
    #[doc = "Bit 4 - Receive FIFO overflow (cannot happen in MSPI design -- MSPI bus pins will stall)"]
    #[inline(always)]
    #[must_use]
    pub fn rxo(&mut self) -> RxoW<IntclrSpec> {
        RxoW::new(self, 4)
    }
    #[doc = "Bit 5 - Receive FIFO full"]
    #[inline(always)]
    #[must_use]
    pub fn rxf(&mut self) -> RxfW<IntclrSpec> {
        RxfW::new(self, 5)
    }
    #[doc = "Bit 6 - DMA Complete Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn dcmp(&mut self) -> DcmpW<IntclrSpec> {
        DcmpW::new(self, 6)
    }
    #[doc = "Bit 7 - DMA Error Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn derr(&mut self) -> DerrW<IntclrSpec> {
        DerrW::new(self, 7)
    }
    #[doc = "Bit 8 - Command Queue Complete Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn cqcmp(&mut self) -> CqcmpW<IntclrSpec> {
        CqcmpW::new(self, 8)
    }
    #[doc = "Bit 9 - Command Queue Update Interrupt. Issued whenever the CQ performs an operation where address bit\\[0\\]
is set. Useful for triggering CURIDX interrupts."]
    #[inline(always)]
    #[must_use]
    pub fn cqupd(&mut self) -> CqupdW<IntclrSpec> {
        CqupdW::new(self, 9)
    }
    #[doc = "Bit 10 - Command Queue is Paused."]
    #[inline(always)]
    #[must_use]
    pub fn cqpaused(&mut self) -> CqpausedW<IntclrSpec> {
        CqpausedW::new(self, 10)
    }
    #[doc = "Bit 11 - Command Queue Error Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn cqerr(&mut self) -> CqerrW<IntclrSpec> {
        CqerrW::new(self, 11)
    }
    #[doc = "Bit 12 - Scrambling Alignment Error. Scrambling operations must be aligned to word (4-byte) start address."]
    #[inline(always)]
    #[must_use]
    pub fn screrr(&mut self) -> ScrerrW<IntclrSpec> {
        ScrerrW::new(self, 12)
    }
    #[doc = "Bit 13 - MSPI is dma target as well as dma source, which may result in deadlock."]
    #[inline(always)]
    #[must_use]
    pub fn apbdmaerr(&mut self) -> ApbdmaerrW<IntclrSpec> {
        ApbdmaerrW::new(self, 13)
    }
}
#[doc = "Write a 1 to a bit in this register to clear the interrupt status associated with that bit.\n\nYou can [`read`](crate::Reg::read) this register and get [`intclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntclrSpec;
impl crate::RegisterSpec for IntclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intclr::R`](R) reader structure"]
impl crate::Readable for IntclrSpec {}
#[doc = "`write(|w| ..)` method takes [`intclr::W`](W) writer structure"]
impl crate::Writable for IntclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTCLR to value 0"]
impl crate::Resettable for IntclrSpec {
    const RESET_VALUE: u32 = 0;
}
