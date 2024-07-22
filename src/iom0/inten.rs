#[doc = "Register `INTEN` reader"]
pub type R = crate::R<IntenSpec>;
#[doc = "Register `INTEN` writer"]
pub type W = crate::W<IntenSpec>;
#[doc = "Field `CMDCMP` reader - Command Complete interrupt. Asserted when the current operation has completed. For repeated commands, this will only be asserted when the final repeated command is completed."]
pub type CmdcmpR = crate::BitReader;
#[doc = "Field `CMDCMP` writer - Command Complete interrupt. Asserted when the current operation has completed. For repeated commands, this will only be asserted when the final repeated command is completed."]
pub type CmdcmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `THR` reader - FIFO Threshold interrupt. For write operations, asserted when the number of free bytes in the write FIFO equals or exceeds the WTHR field. For read operations, asserted when the number of valid bytes in the read FIFO equals of exceeds the value set in the RTHR field."]
pub type ThrR = crate::BitReader;
#[doc = "Field `THR` writer - FIFO Threshold interrupt. For write operations, asserted when the number of free bytes in the write FIFO equals or exceeds the WTHR field. For read operations, asserted when the number of valid bytes in the read FIFO equals of exceeds the value set in the RTHR field."]
pub type ThrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FUNDFL` reader - Read FIFO Underflow interrupt. This occurs when software tries to pop from an empty fifo."]
pub type FundflR = crate::BitReader;
#[doc = "Field `FUNDFL` writer - Read FIFO Underflow interrupt. This occurs when software tries to pop from an empty fifo."]
pub type FundflW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FOVFL` reader - Write FIFO Overflow interrupt. This occurs when software tries to write to a full fifo. The current operation does not stop."]
pub type FovflR = crate::BitReader;
#[doc = "Field `FOVFL` writer - Write FIFO Overflow interrupt. This occurs when software tries to write to a full fifo. The current operation does not stop."]
pub type FovflW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAK` reader - I2C NAK interrupt. Asserted when an unexpected NAK has been received on the I2C bus."]
pub type NakR = crate::BitReader;
#[doc = "Field `NAK` writer - I2C NAK interrupt. Asserted when an unexpected NAK has been received on the I2C bus."]
pub type NakW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IACC` reader - illegal FIFO access interrupt. Asserted when there is a overflow or underflow event"]
pub type IaccR = crate::BitReader;
#[doc = "Field `IACC` writer - illegal FIFO access interrupt. Asserted when there is a overflow or underflow event"]
pub type IaccW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICMD` reader - illegal command interrupt. Asserted when a command is written when an active command is in progress."]
pub type IcmdR = crate::BitReader;
#[doc = "Field `ICMD` writer - illegal command interrupt. Asserted when a command is written when an active command is in progress."]
pub type IcmdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `START` reader - START command interrupt. Asserted when another master on the bus has signaled a START command."]
pub type StartR = crate::BitReader;
#[doc = "Field `START` writer - START command interrupt. Asserted when another master on the bus has signaled a START command."]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOP` reader - STOP command interrupt. Asserted when another master on the bus has signaled a STOP command."]
pub type StopR = crate::BitReader;
#[doc = "Field `STOP` writer - STOP command interrupt. Asserted when another master on the bus has signaled a STOP command."]
pub type StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARB` reader - Arbitration loss interrupt. Asserted when arbitration is enabled and has been lost to another master on the bus."]
pub type ArbR = crate::BitReader;
#[doc = "Field `ARB` writer - Arbitration loss interrupt. Asserted when arbitration is enabled and has been lost to another master on the bus."]
pub type ArbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCMP` reader - DMA Complete. Processing of the DMA operation has completed and the DMA submodule is returned into the idle state"]
pub type DcmpR = crate::BitReader;
#[doc = "Field `DCMP` writer - DMA Complete. Processing of the DMA operation has completed and the DMA submodule is returned into the idle state"]
pub type DcmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DERR` reader - DMA Error encountered during the processing of the DMA command. The DMA error could occur when the memory access specified in the DMA operation is not available or incorrectly specified."]
pub type DerrR = crate::BitReader;
#[doc = "Field `DERR` writer - DMA Error encountered during the processing of the DMA command. The DMA error could occur when the memory access specified in the DMA operation is not available or incorrectly specified."]
pub type DerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CQPAUSED` reader - Command queue is paused due to an active event enabled in the PAUSEEN register. The interrupt is posted when the event is enabled within the PAUSEEN register, the mask is active in the CQIRQMASK field and the event occurs."]
pub type CqpausedR = crate::BitReader;
#[doc = "Field `CQPAUSED` writer - Command queue is paused due to an active event enabled in the PAUSEEN register. The interrupt is posted when the event is enabled within the PAUSEEN register, the mask is active in the CQIRQMASK field and the event occurs."]
pub type CqpausedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CQUPD` reader - CQ write operation performed a register write with the register address bit 0 set to 1. The low address bits in the CQ address fields are unused and bit 0 can be used to trigger an interrupt to indicate when this register write is performed by the CQ operation."]
pub type CqupdR = crate::BitReader;
#[doc = "Field `CQUPD` writer - CQ write operation performed a register write with the register address bit 0 set to 1. The low address bits in the CQ address fields are unused and bit 0 can be used to trigger an interrupt to indicate when this register write is performed by the CQ operation."]
pub type CqupdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CQERR` reader - Error during command queue operations"]
pub type CqerrR = crate::BitReader;
#[doc = "Field `CQERR` writer - Error during command queue operations"]
pub type CqerrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Command Complete interrupt. Asserted when the current operation has completed. For repeated commands, this will only be asserted when the final repeated command is completed."]
    #[inline(always)]
    pub fn cmdcmp(&self) -> CmdcmpR {
        CmdcmpR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FIFO Threshold interrupt. For write operations, asserted when the number of free bytes in the write FIFO equals or exceeds the WTHR field. For read operations, asserted when the number of valid bytes in the read FIFO equals of exceeds the value set in the RTHR field."]
    #[inline(always)]
    pub fn thr(&self) -> ThrR {
        ThrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Read FIFO Underflow interrupt. This occurs when software tries to pop from an empty fifo."]
    #[inline(always)]
    pub fn fundfl(&self) -> FundflR {
        FundflR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write FIFO Overflow interrupt. This occurs when software tries to write to a full fifo. The current operation does not stop."]
    #[inline(always)]
    pub fn fovfl(&self) -> FovflR {
        FovflR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - I2C NAK interrupt. Asserted when an unexpected NAK has been received on the I2C bus."]
    #[inline(always)]
    pub fn nak(&self) -> NakR {
        NakR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - illegal FIFO access interrupt. Asserted when there is a overflow or underflow event"]
    #[inline(always)]
    pub fn iacc(&self) -> IaccR {
        IaccR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - illegal command interrupt. Asserted when a command is written when an active command is in progress."]
    #[inline(always)]
    pub fn icmd(&self) -> IcmdR {
        IcmdR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - START command interrupt. Asserted when another master on the bus has signaled a START command."]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - STOP command interrupt. Asserted when another master on the bus has signaled a STOP command."]
    #[inline(always)]
    pub fn stop(&self) -> StopR {
        StopR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Arbitration loss interrupt. Asserted when arbitration is enabled and has been lost to another master on the bus."]
    #[inline(always)]
    pub fn arb(&self) -> ArbR {
        ArbR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DMA Complete. Processing of the DMA operation has completed and the DMA submodule is returned into the idle state"]
    #[inline(always)]
    pub fn dcmp(&self) -> DcmpR {
        DcmpR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DMA Error encountered during the processing of the DMA command. The DMA error could occur when the memory access specified in the DMA operation is not available or incorrectly specified."]
    #[inline(always)]
    pub fn derr(&self) -> DerrR {
        DerrR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Command queue is paused due to an active event enabled in the PAUSEEN register. The interrupt is posted when the event is enabled within the PAUSEEN register, the mask is active in the CQIRQMASK field and the event occurs."]
    #[inline(always)]
    pub fn cqpaused(&self) -> CqpausedR {
        CqpausedR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - CQ write operation performed a register write with the register address bit 0 set to 1. The low address bits in the CQ address fields are unused and bit 0 can be used to trigger an interrupt to indicate when this register write is performed by the CQ operation."]
    #[inline(always)]
    pub fn cqupd(&self) -> CqupdR {
        CqupdR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Error during command queue operations"]
    #[inline(always)]
    pub fn cqerr(&self) -> CqerrR {
        CqerrR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Command Complete interrupt. Asserted when the current operation has completed. For repeated commands, this will only be asserted when the final repeated command is completed."]
    #[inline(always)]
    #[must_use]
    pub fn cmdcmp(&mut self) -> CmdcmpW<IntenSpec> {
        CmdcmpW::new(self, 0)
    }
    #[doc = "Bit 1 - FIFO Threshold interrupt. For write operations, asserted when the number of free bytes in the write FIFO equals or exceeds the WTHR field. For read operations, asserted when the number of valid bytes in the read FIFO equals of exceeds the value set in the RTHR field."]
    #[inline(always)]
    #[must_use]
    pub fn thr(&mut self) -> ThrW<IntenSpec> {
        ThrW::new(self, 1)
    }
    #[doc = "Bit 2 - Read FIFO Underflow interrupt. This occurs when software tries to pop from an empty fifo."]
    #[inline(always)]
    #[must_use]
    pub fn fundfl(&mut self) -> FundflW<IntenSpec> {
        FundflW::new(self, 2)
    }
    #[doc = "Bit 3 - Write FIFO Overflow interrupt. This occurs when software tries to write to a full fifo. The current operation does not stop."]
    #[inline(always)]
    #[must_use]
    pub fn fovfl(&mut self) -> FovflW<IntenSpec> {
        FovflW::new(self, 3)
    }
    #[doc = "Bit 4 - I2C NAK interrupt. Asserted when an unexpected NAK has been received on the I2C bus."]
    #[inline(always)]
    #[must_use]
    pub fn nak(&mut self) -> NakW<IntenSpec> {
        NakW::new(self, 4)
    }
    #[doc = "Bit 5 - illegal FIFO access interrupt. Asserted when there is a overflow or underflow event"]
    #[inline(always)]
    #[must_use]
    pub fn iacc(&mut self) -> IaccW<IntenSpec> {
        IaccW::new(self, 5)
    }
    #[doc = "Bit 6 - illegal command interrupt. Asserted when a command is written when an active command is in progress."]
    #[inline(always)]
    #[must_use]
    pub fn icmd(&mut self) -> IcmdW<IntenSpec> {
        IcmdW::new(self, 6)
    }
    #[doc = "Bit 7 - START command interrupt. Asserted when another master on the bus has signaled a START command."]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> StartW<IntenSpec> {
        StartW::new(self, 7)
    }
    #[doc = "Bit 8 - STOP command interrupt. Asserted when another master on the bus has signaled a STOP command."]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> StopW<IntenSpec> {
        StopW::new(self, 8)
    }
    #[doc = "Bit 9 - Arbitration loss interrupt. Asserted when arbitration is enabled and has been lost to another master on the bus."]
    #[inline(always)]
    #[must_use]
    pub fn arb(&mut self) -> ArbW<IntenSpec> {
        ArbW::new(self, 9)
    }
    #[doc = "Bit 10 - DMA Complete. Processing of the DMA operation has completed and the DMA submodule is returned into the idle state"]
    #[inline(always)]
    #[must_use]
    pub fn dcmp(&mut self) -> DcmpW<IntenSpec> {
        DcmpW::new(self, 10)
    }
    #[doc = "Bit 11 - DMA Error encountered during the processing of the DMA command. The DMA error could occur when the memory access specified in the DMA operation is not available or incorrectly specified."]
    #[inline(always)]
    #[must_use]
    pub fn derr(&mut self) -> DerrW<IntenSpec> {
        DerrW::new(self, 11)
    }
    #[doc = "Bit 12 - Command queue is paused due to an active event enabled in the PAUSEEN register. The interrupt is posted when the event is enabled within the PAUSEEN register, the mask is active in the CQIRQMASK field and the event occurs."]
    #[inline(always)]
    #[must_use]
    pub fn cqpaused(&mut self) -> CqpausedW<IntenSpec> {
        CqpausedW::new(self, 12)
    }
    #[doc = "Bit 13 - CQ write operation performed a register write with the register address bit 0 set to 1. The low address bits in the CQ address fields are unused and bit 0 can be used to trigger an interrupt to indicate when this register write is performed by the CQ operation."]
    #[inline(always)]
    #[must_use]
    pub fn cqupd(&mut self) -> CqupdW<IntenSpec> {
        CqupdW::new(self, 13)
    }
    #[doc = "Bit 14 - Error during command queue operations"]
    #[inline(always)]
    #[must_use]
    pub fn cqerr(&mut self) -> CqerrW<IntenSpec> {
        CqerrW::new(self, 14)
    }
}
#[doc = "Set bits in this register to allow this module to generate the corresponding interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`inten::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenSpec;
impl crate::RegisterSpec for IntenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inten::R`](R) reader structure"]
impl crate::Readable for IntenSpec {}
#[doc = "`write(|w| ..)` method takes [`inten::W`](W) writer structure"]
impl crate::Writable for IntenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for IntenSpec {
    const RESET_VALUE: u32 = 0;
}
