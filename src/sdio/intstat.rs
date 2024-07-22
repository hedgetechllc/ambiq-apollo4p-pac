#[doc = "Register `INTSTAT` reader"]
pub type R = crate::R<IntstatSpec>;
#[doc = "Register `INTSTAT` writer"]
pub type W = crate::W<IntstatSpec>;
#[doc = "This bit is set when we get the end bit of the command response (Except Auto CMD12 and Auto CMD23) Note: Command Time-out Error has higher priority than Command Complete. If both are set to 1, it can be considered that the response was not received correctly.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Commandcomplete {
    #[doc = "0: No Command Complete"]
    Nocmp = 0,
    #[doc = "1: Command Complete"]
    Cmdcmp = 1,
}
impl From<Commandcomplete> for bool {
    #[inline(always)]
    fn from(variant: Commandcomplete) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMMANDCOMPLETE` reader - This bit is set when we get the end bit of the command response (Except Auto CMD12 and Auto CMD23) Note: Command Time-out Error has higher priority than Command Complete. If both are set to 1, it can be considered that the response was not received correctly."]
pub type CommandcompleteR = crate::BitReader<Commandcomplete>;
impl CommandcompleteR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Commandcomplete {
        match self.bits {
            false => Commandcomplete::Nocmp,
            true => Commandcomplete::Cmdcmp,
        }
    }
    #[doc = "No Command Complete"]
    #[inline(always)]
    pub fn is_nocmp(&self) -> bool {
        *self == Commandcomplete::Nocmp
    }
    #[doc = "Command Complete"]
    #[inline(always)]
    pub fn is_cmdcmp(&self) -> bool {
        *self == Commandcomplete::Cmdcmp
    }
}
#[doc = "Field `COMMANDCOMPLETE` writer - This bit is set when we get the end bit of the command response (Except Auto CMD12 and Auto CMD23) Note: Command Time-out Error has higher priority than Command Complete. If both are set to 1, it can be considered that the response was not received correctly."]
pub type CommandcompleteW<'a, REG> = crate::BitWriter<'a, REG, Commandcomplete>;
impl<'a, REG> CommandcompleteW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Command Complete"]
    #[inline(always)]
    pub fn nocmp(self) -> &'a mut crate::W<REG> {
        self.variant(Commandcomplete::Nocmp)
    }
    #[doc = "Command Complete"]
    #[inline(always)]
    pub fn cmdcmp(self) -> &'a mut crate::W<REG> {
        self.variant(Commandcomplete::Cmdcmp)
    }
}
#[doc = "This bit is set when a read / write transaction is completed. Read Transaction: This bit is set at the falling edge of Read Transfer Active Status. There are two cases in which the Interrupt is generated. The first is when a data transfer is completed as specified by data length (After the last data has been read to the Host System). The second is when data has stopped at the block gap and completed the data transfer by setting the Stop At Block Gap Request in the Block Gap Control Register (After valid data has been read to the Host System). Write Transaction: This bit is set at the falling edge of the DAT Line Active Status. There are two cases in which the Interrupt is generated. The first is when the last data is written to the card as specified by data length and Busy signal is released. The second is when data transfers are stopped at the block gap by setting Stop At Block Gap Request in the Block Gap Control Register and data transfers completed. (After valid data is written to the SD card and the busy signal is released). Note: Transfer Complete has higher priority than Data Time-out Error. If both bits are set to 1, the data transfer can be considered complete Note: While performing tuning procedure (Execute Tuning is set to 1), Transfer Complete is not set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Transfercomplete {
    #[doc = "0: No Data Transfer Complete"]
    Nodata = 0,
    #[doc = "1: Data Transfer Complete"]
    Complete = 1,
}
impl From<Transfercomplete> for bool {
    #[inline(always)]
    fn from(variant: Transfercomplete) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRANSFERCOMPLETE` reader - This bit is set when a read / write transaction is completed. Read Transaction: This bit is set at the falling edge of Read Transfer Active Status. There are two cases in which the Interrupt is generated. The first is when a data transfer is completed as specified by data length (After the last data has been read to the Host System). The second is when data has stopped at the block gap and completed the data transfer by setting the Stop At Block Gap Request in the Block Gap Control Register (After valid data has been read to the Host System). Write Transaction: This bit is set at the falling edge of the DAT Line Active Status. There are two cases in which the Interrupt is generated. The first is when the last data is written to the card as specified by data length and Busy signal is released. The second is when data transfers are stopped at the block gap by setting Stop At Block Gap Request in the Block Gap Control Register and data transfers completed. (After valid data is written to the SD card and the busy signal is released). Note: Transfer Complete has higher priority than Data Time-out Error. If both bits are set to 1, the data transfer can be considered complete Note: While performing tuning procedure (Execute Tuning is set to 1), Transfer Complete is not set to 1"]
pub type TransfercompleteR = crate::BitReader<Transfercomplete>;
impl TransfercompleteR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Transfercomplete {
        match self.bits {
            false => Transfercomplete::Nodata,
            true => Transfercomplete::Complete,
        }
    }
    #[doc = "No Data Transfer Complete"]
    #[inline(always)]
    pub fn is_nodata(&self) -> bool {
        *self == Transfercomplete::Nodata
    }
    #[doc = "Data Transfer Complete"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == Transfercomplete::Complete
    }
}
#[doc = "Field `TRANSFERCOMPLETE` writer - This bit is set when a read / write transaction is completed. Read Transaction: This bit is set at the falling edge of Read Transfer Active Status. There are two cases in which the Interrupt is generated. The first is when a data transfer is completed as specified by data length (After the last data has been read to the Host System). The second is when data has stopped at the block gap and completed the data transfer by setting the Stop At Block Gap Request in the Block Gap Control Register (After valid data has been read to the Host System). Write Transaction: This bit is set at the falling edge of the DAT Line Active Status. There are two cases in which the Interrupt is generated. The first is when the last data is written to the card as specified by data length and Busy signal is released. The second is when data transfers are stopped at the block gap by setting Stop At Block Gap Request in the Block Gap Control Register and data transfers completed. (After valid data is written to the SD card and the busy signal is released). Note: Transfer Complete has higher priority than Data Time-out Error. If both bits are set to 1, the data transfer can be considered complete Note: While performing tuning procedure (Execute Tuning is set to 1), Transfer Complete is not set to 1"]
pub type TransfercompleteW<'a, REG> = crate::BitWriter<'a, REG, Transfercomplete>;
impl<'a, REG> TransfercompleteW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Data Transfer Complete"]
    #[inline(always)]
    pub fn nodata(self) -> &'a mut crate::W<REG> {
        self.variant(Transfercomplete::Nodata)
    }
    #[doc = "Data Transfer Complete"]
    #[inline(always)]
    pub fn complete(self) -> &'a mut crate::W<REG> {
        self.variant(Transfercomplete::Complete)
    }
}
#[doc = "If the Stop At Block Gap Request in the Block Gap Control Register is set, this bit is set. Read Transaction: This bit is set at the falling edge of the DAT Line Active Status (When the transaction is stopped at SD Bus timing. The Read Wait must be supported inorder to use this function). Write Transaction: This bit is set at the falling edge of Write Transfer Active Status (After getting CRC status at SD Bus timing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Blockgapevent {
    #[doc = "0: No Block Gap Event"]
    Noevent = 0,
    #[doc = "1: Transaction stopped at Block Gap"]
    Stopped = 1,
}
impl From<Blockgapevent> for bool {
    #[inline(always)]
    fn from(variant: Blockgapevent) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BLOCKGAPEVENT` reader - If the Stop At Block Gap Request in the Block Gap Control Register is set, this bit is set. Read Transaction: This bit is set at the falling edge of the DAT Line Active Status (When the transaction is stopped at SD Bus timing. The Read Wait must be supported inorder to use this function). Write Transaction: This bit is set at the falling edge of Write Transfer Active Status (After getting CRC status at SD Bus timing)."]
pub type BlockgapeventR = crate::BitReader<Blockgapevent>;
impl BlockgapeventR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Blockgapevent {
        match self.bits {
            false => Blockgapevent::Noevent,
            true => Blockgapevent::Stopped,
        }
    }
    #[doc = "No Block Gap Event"]
    #[inline(always)]
    pub fn is_noevent(&self) -> bool {
        *self == Blockgapevent::Noevent
    }
    #[doc = "Transaction stopped at Block Gap"]
    #[inline(always)]
    pub fn is_stopped(&self) -> bool {
        *self == Blockgapevent::Stopped
    }
}
#[doc = "Field `BLOCKGAPEVENT` writer - If the Stop At Block Gap Request in the Block Gap Control Register is set, this bit is set. Read Transaction: This bit is set at the falling edge of the DAT Line Active Status (When the transaction is stopped at SD Bus timing. The Read Wait must be supported inorder to use this function). Write Transaction: This bit is set at the falling edge of Write Transfer Active Status (After getting CRC status at SD Bus timing)."]
pub type BlockgapeventW<'a, REG> = crate::BitWriter<'a, REG, Blockgapevent>;
impl<'a, REG> BlockgapeventW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Block Gap Event"]
    #[inline(always)]
    pub fn noevent(self) -> &'a mut crate::W<REG> {
        self.variant(Blockgapevent::Noevent)
    }
    #[doc = "Transaction stopped at Block Gap"]
    #[inline(always)]
    pub fn stopped(self) -> &'a mut crate::W<REG> {
        self.variant(Blockgapevent::Stopped)
    }
}
#[doc = "This status is set if the HC detects the Host DMA Buffer Boundary in the Block Size regiser.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmainterrupt {
    #[doc = "0: No DMA Interrupt"]
    Noint = 0,
    #[doc = "1: DMA Interrupt is Generated"]
    Int = 1,
}
impl From<Dmainterrupt> for bool {
    #[inline(always)]
    fn from(variant: Dmainterrupt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAINTERRUPT` reader - This status is set if the HC detects the Host DMA Buffer Boundary in the Block Size regiser."]
pub type DmainterruptR = crate::BitReader<Dmainterrupt>;
impl DmainterruptR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmainterrupt {
        match self.bits {
            false => Dmainterrupt::Noint,
            true => Dmainterrupt::Int,
        }
    }
    #[doc = "No DMA Interrupt"]
    #[inline(always)]
    pub fn is_noint(&self) -> bool {
        *self == Dmainterrupt::Noint
    }
    #[doc = "DMA Interrupt is Generated"]
    #[inline(always)]
    pub fn is_int(&self) -> bool {
        *self == Dmainterrupt::Int
    }
}
#[doc = "Field `DMAINTERRUPT` writer - This status is set if the HC detects the Host DMA Buffer Boundary in the Block Size regiser."]
pub type DmainterruptW<'a, REG> = crate::BitWriter<'a, REG, Dmainterrupt>;
impl<'a, REG> DmainterruptW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No DMA Interrupt"]
    #[inline(always)]
    pub fn noint(self) -> &'a mut crate::W<REG> {
        self.variant(Dmainterrupt::Noint)
    }
    #[doc = "DMA Interrupt is Generated"]
    #[inline(always)]
    pub fn int(self) -> &'a mut crate::W<REG> {
        self.variant(Dmainterrupt::Int)
    }
}
#[doc = "This status is set if the Buffer Write Enable changes from 0 to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bufferwriteready {
    #[doc = "0: Not Ready to Write Buffer."]
    Notready = 0,
    #[doc = "1: Ready to Write Buffer."]
    Ready = 1,
}
impl From<Bufferwriteready> for bool {
    #[inline(always)]
    fn from(variant: Bufferwriteready) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUFFERWRITEREADY` reader - This status is set if the Buffer Write Enable changes from 0 to 1."]
pub type BufferwritereadyR = crate::BitReader<Bufferwriteready>;
impl BufferwritereadyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bufferwriteready {
        match self.bits {
            false => Bufferwriteready::Notready,
            true => Bufferwriteready::Ready,
        }
    }
    #[doc = "Not Ready to Write Buffer."]
    #[inline(always)]
    pub fn is_notready(&self) -> bool {
        *self == Bufferwriteready::Notready
    }
    #[doc = "Ready to Write Buffer."]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == Bufferwriteready::Ready
    }
}
#[doc = "Field `BUFFERWRITEREADY` writer - This status is set if the Buffer Write Enable changes from 0 to 1."]
pub type BufferwritereadyW<'a, REG> = crate::BitWriter<'a, REG, Bufferwriteready>;
impl<'a, REG> BufferwritereadyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not Ready to Write Buffer."]
    #[inline(always)]
    pub fn notready(self) -> &'a mut crate::W<REG> {
        self.variant(Bufferwriteready::Notready)
    }
    #[doc = "Ready to Write Buffer."]
    #[inline(always)]
    pub fn ready(self) -> &'a mut crate::W<REG> {
        self.variant(Bufferwriteready::Ready)
    }
}
#[doc = "This status is set if the Buffer Read Enable changes from 0 to 1. Buffer Read Ready is set to 1 for every CMD19 execution in tuning procedure.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bufferreadready {
    #[doc = "0: Not Ready to read Buffer."]
    Noready = 0,
    #[doc = "1: Ready to read Buffer."]
    Ready = 1,
}
impl From<Bufferreadready> for bool {
    #[inline(always)]
    fn from(variant: Bufferreadready) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUFFERREADREADY` reader - This status is set if the Buffer Read Enable changes from 0 to 1. Buffer Read Ready is set to 1 for every CMD19 execution in tuning procedure."]
pub type BufferreadreadyR = crate::BitReader<Bufferreadready>;
impl BufferreadreadyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bufferreadready {
        match self.bits {
            false => Bufferreadready::Noready,
            true => Bufferreadready::Ready,
        }
    }
    #[doc = "Not Ready to read Buffer."]
    #[inline(always)]
    pub fn is_noready(&self) -> bool {
        *self == Bufferreadready::Noready
    }
    #[doc = "Ready to read Buffer."]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == Bufferreadready::Ready
    }
}
#[doc = "Field `BUFFERREADREADY` writer - This status is set if the Buffer Read Enable changes from 0 to 1. Buffer Read Ready is set to 1 for every CMD19 execution in tuning procedure."]
pub type BufferreadreadyW<'a, REG> = crate::BitWriter<'a, REG, Bufferreadready>;
impl<'a, REG> BufferreadreadyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not Ready to read Buffer."]
    #[inline(always)]
    pub fn noready(self) -> &'a mut crate::W<REG> {
        self.variant(Bufferreadready::Noready)
    }
    #[doc = "Ready to read Buffer."]
    #[inline(always)]
    pub fn ready(self) -> &'a mut crate::W<REG> {
        self.variant(Bufferreadready::Ready)
    }
}
#[doc = "This status is set if the Card Inserted in the Present State register changes from 0 to 1. When the HD writes this bit to 1 to clear this status the status of the Card Inserted in the Present State register should be confirmed. Because the card detect may possibly be changed when the HD clear this bit an Interrupt event may not be generated.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cardinsertion {
    #[doc = "0: Card State Stable or Debouncing"]
    Stable = 0,
    #[doc = "1: Card Inserted"]
    Inserted = 1,
}
impl From<Cardinsertion> for bool {
    #[inline(always)]
    fn from(variant: Cardinsertion) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CARDINSERTION` reader - This status is set if the Card Inserted in the Present State register changes from 0 to 1. When the HD writes this bit to 1 to clear this status the status of the Card Inserted in the Present State register should be confirmed. Because the card detect may possibly be changed when the HD clear this bit an Interrupt event may not be generated."]
pub type CardinsertionR = crate::BitReader<Cardinsertion>;
impl CardinsertionR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cardinsertion {
        match self.bits {
            false => Cardinsertion::Stable,
            true => Cardinsertion::Inserted,
        }
    }
    #[doc = "Card State Stable or Debouncing"]
    #[inline(always)]
    pub fn is_stable(&self) -> bool {
        *self == Cardinsertion::Stable
    }
    #[doc = "Card Inserted"]
    #[inline(always)]
    pub fn is_inserted(&self) -> bool {
        *self == Cardinsertion::Inserted
    }
}
#[doc = "Field `CARDINSERTION` writer - This status is set if the Card Inserted in the Present State register changes from 0 to 1. When the HD writes this bit to 1 to clear this status the status of the Card Inserted in the Present State register should be confirmed. Because the card detect may possibly be changed when the HD clear this bit an Interrupt event may not be generated."]
pub type CardinsertionW<'a, REG> = crate::BitWriter<'a, REG, Cardinsertion>;
impl<'a, REG> CardinsertionW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Card State Stable or Debouncing"]
    #[inline(always)]
    pub fn stable(self) -> &'a mut crate::W<REG> {
        self.variant(Cardinsertion::Stable)
    }
    #[doc = "Card Inserted"]
    #[inline(always)]
    pub fn inserted(self) -> &'a mut crate::W<REG> {
        self.variant(Cardinsertion::Inserted)
    }
}
#[doc = "This status is set if the Card Inserted in the Present State register changes from 1 to 0. When the HD writes this bit to 1 to clear this status the status of the Card Inserted in the Present State register should be confirmed. Because the card detect may possibly be changed when the HD clear this bit an Interrupt event may not be generated.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cardremoval {
    #[doc = "0: Card State Stable or Debouncing"]
    Stable = 0,
    #[doc = "1: Card Removed"]
    Removed = 1,
}
impl From<Cardremoval> for bool {
    #[inline(always)]
    fn from(variant: Cardremoval) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CARDREMOVAL` reader - This status is set if the Card Inserted in the Present State register changes from 1 to 0. When the HD writes this bit to 1 to clear this status the status of the Card Inserted in the Present State register should be confirmed. Because the card detect may possibly be changed when the HD clear this bit an Interrupt event may not be generated."]
pub type CardremovalR = crate::BitReader<Cardremoval>;
impl CardremovalR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cardremoval {
        match self.bits {
            false => Cardremoval::Stable,
            true => Cardremoval::Removed,
        }
    }
    #[doc = "Card State Stable or Debouncing"]
    #[inline(always)]
    pub fn is_stable(&self) -> bool {
        *self == Cardremoval::Stable
    }
    #[doc = "Card Removed"]
    #[inline(always)]
    pub fn is_removed(&self) -> bool {
        *self == Cardremoval::Removed
    }
}
#[doc = "Field `CARDREMOVAL` writer - This status is set if the Card Inserted in the Present State register changes from 1 to 0. When the HD writes this bit to 1 to clear this status the status of the Card Inserted in the Present State register should be confirmed. Because the card detect may possibly be changed when the HD clear this bit an Interrupt event may not be generated."]
pub type CardremovalW<'a, REG> = crate::BitWriter<'a, REG, Cardremoval>;
impl<'a, REG> CardremovalW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Card State Stable or Debouncing"]
    #[inline(always)]
    pub fn stable(self) -> &'a mut crate::W<REG> {
        self.variant(Cardremoval::Stable)
    }
    #[doc = "Card Removed"]
    #[inline(always)]
    pub fn removed(self) -> &'a mut crate::W<REG> {
        self.variant(Cardremoval::Removed)
    }
}
#[doc = "Writing this bit to 1 does not clear this bit. It is cleared by resetting the SD card interrupt factor. In 1-bit mode, the HC shall detect the Card Interrupt without SD Clock to support wakeup. In 4-bit mode, the card interrupt signal is sampled during the interrupt cycle, so there are some sample delays between the interrupt signal from the card and the interrupt to the Host system. when this status has been set and the HD needs to start this interrupt service, Card Interrupt Status Enable in the Normal Interrupt Status register shall be set to 0 in order to clear the card interrupt statuses latched in the HC and stop driving the Host System. After completion of the card interrupt service (the reset factor in the SD card and the interrupt signal may not be asserted), set Card Interrupt Status Enable to 1 and start sampling the interrupt signal again. Interrupt detected by DAT\\[1\\]
is supported when there is a card per slot. In case of shared bus, interrupt pins are used to detect interrupts. If 000b is set to Interrupt Pin Select in the Shared Bus Control register, this status is effective. Non-zero value is set to Interrupt Pin Select, INT_A, INT_B or INT_C is then used to device interrupts.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cardinterrupt {
    #[doc = "0: No Card Interrupt"]
    Noint = 0,
    #[doc = "1: Generate Card Interrupt"]
    Int = 1,
}
impl From<Cardinterrupt> for bool {
    #[inline(always)]
    fn from(variant: Cardinterrupt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CARDINTERRUPT` reader - Writing this bit to 1 does not clear this bit. It is cleared by resetting the SD card interrupt factor. In 1-bit mode, the HC shall detect the Card Interrupt without SD Clock to support wakeup. In 4-bit mode, the card interrupt signal is sampled during the interrupt cycle, so there are some sample delays between the interrupt signal from the card and the interrupt to the Host system. when this status has been set and the HD needs to start this interrupt service, Card Interrupt Status Enable in the Normal Interrupt Status register shall be set to 0 in order to clear the card interrupt statuses latched in the HC and stop driving the Host System. After completion of the card interrupt service (the reset factor in the SD card and the interrupt signal may not be asserted), set Card Interrupt Status Enable to 1 and start sampling the interrupt signal again. Interrupt detected by DAT\\[1\\]
is supported when there is a card per slot. In case of shared bus, interrupt pins are used to detect interrupts. If 000b is set to Interrupt Pin Select in the Shared Bus Control register, this status is effective. Non-zero value is set to Interrupt Pin Select, INT_A, INT_B or INT_C is then used to device interrupts."]
pub type CardinterruptR = crate::BitReader<Cardinterrupt>;
impl CardinterruptR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cardinterrupt {
        match self.bits {
            false => Cardinterrupt::Noint,
            true => Cardinterrupt::Int,
        }
    }
    #[doc = "No Card Interrupt"]
    #[inline(always)]
    pub fn is_noint(&self) -> bool {
        *self == Cardinterrupt::Noint
    }
    #[doc = "Generate Card Interrupt"]
    #[inline(always)]
    pub fn is_int(&self) -> bool {
        *self == Cardinterrupt::Int
    }
}
#[doc = "Field `CARDINTERRUPT` writer - Writing this bit to 1 does not clear this bit. It is cleared by resetting the SD card interrupt factor. In 1-bit mode, the HC shall detect the Card Interrupt without SD Clock to support wakeup. In 4-bit mode, the card interrupt signal is sampled during the interrupt cycle, so there are some sample delays between the interrupt signal from the card and the interrupt to the Host system. when this status has been set and the HD needs to start this interrupt service, Card Interrupt Status Enable in the Normal Interrupt Status register shall be set to 0 in order to clear the card interrupt statuses latched in the HC and stop driving the Host System. After completion of the card interrupt service (the reset factor in the SD card and the interrupt signal may not be asserted), set Card Interrupt Status Enable to 1 and start sampling the interrupt signal again. Interrupt detected by DAT\\[1\\]
is supported when there is a card per slot. In case of shared bus, interrupt pins are used to detect interrupts. If 000b is set to Interrupt Pin Select in the Shared Bus Control register, this status is effective. Non-zero value is set to Interrupt Pin Select, INT_A, INT_B or INT_C is then used to device interrupts."]
pub type CardinterruptW<'a, REG> = crate::BitWriter<'a, REG, Cardinterrupt>;
impl<'a, REG> CardinterruptW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Card Interrupt"]
    #[inline(always)]
    pub fn noint(self) -> &'a mut crate::W<REG> {
        self.variant(Cardinterrupt::Noint)
    }
    #[doc = "Generate Card Interrupt"]
    #[inline(always)]
    pub fn int(self) -> &'a mut crate::W<REG> {
        self.variant(Cardinterrupt::Int)
    }
}
#[doc = "Field `INTA` reader - This status is set if INT_A is enabled and INT_A# pin is in low level. Writing this bit to 1 does not clear this bit. It is cleared by resetting the INT_A interrupt factor"]
pub type IntaR = crate::BitReader;
#[doc = "Field `INTA` writer - This status is set if INT_A is enabled and INT_A# pin is in low level. Writing this bit to 1 does not clear this bit. It is cleared by resetting the INT_A interrupt factor"]
pub type IntaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTB` reader - This status is set if INT_B is enabled and INT_B# pin is in low level. Writing this bit to 1 does not clear this bit. It is cleared by resetting the INT_B interrupt factor"]
pub type IntbR = crate::BitReader;
#[doc = "Field `INTB` writer - This status is set if INT_B is enabled and INT_B# pin is in low level. Writing this bit to 1 does not clear this bit. It is cleared by resetting the INT_B interrupt factor"]
pub type IntbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTC` reader - This status is set if INT_C is enabled and INT_C# pin is in low level. Writing this bit to 1 does not clear this bit. It is cleared by resetting the INT_C interrupt factor"]
pub type IntcR = crate::BitReader;
#[doc = "Field `INTC` writer - This status is set if INT_C is enabled and INT_C# pin is in low level. Writing this bit to 1 does not clear this bit. It is cleared by resetting the INT_C interrupt factor"]
pub type IntcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "This status is set if Re-Tuning Request in the Present State register changes from 0 to 1. Host Controller requests Host Driver to perform re-tuning for next data transfer. Current data transfer (not large block count) can be completed without re-tuning.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Retuningevent {
    #[doc = "1: ReTuning should be performed"]
    Retune = 1,
    #[doc = "0: ReTuning is not required"]
    Noretune = 0,
}
impl From<Retuningevent> for bool {
    #[inline(always)]
    fn from(variant: Retuningevent) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RETUNINGEVENT` reader - This status is set if Re-Tuning Request in the Present State register changes from 0 to 1. Host Controller requests Host Driver to perform re-tuning for next data transfer. Current data transfer (not large block count) can be completed without re-tuning."]
pub type RetuningeventR = crate::BitReader<Retuningevent>;
impl RetuningeventR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Retuningevent {
        match self.bits {
            true => Retuningevent::Retune,
            false => Retuningevent::Noretune,
        }
    }
    #[doc = "ReTuning should be performed"]
    #[inline(always)]
    pub fn is_retune(&self) -> bool {
        *self == Retuningevent::Retune
    }
    #[doc = "ReTuning is not required"]
    #[inline(always)]
    pub fn is_noretune(&self) -> bool {
        *self == Retuningevent::Noretune
    }
}
#[doc = "Field `RETUNINGEVENT` writer - This status is set if Re-Tuning Request in the Present State register changes from 0 to 1. Host Controller requests Host Driver to perform re-tuning for next data transfer. Current data transfer (not large block count) can be completed without re-tuning."]
pub type RetuningeventW<'a, REG> = crate::BitWriter<'a, REG, Retuningevent>;
impl<'a, REG> RetuningeventW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ReTuning should be performed"]
    #[inline(always)]
    pub fn retune(self) -> &'a mut crate::W<REG> {
        self.variant(Retuningevent::Retune)
    }
    #[doc = "ReTuning is not required"]
    #[inline(always)]
    pub fn noretune(self) -> &'a mut crate::W<REG> {
        self.variant(Retuningevent::Noretune)
    }
}
#[doc = "This status is set if the boot acknowledge is received from device.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bootackrcv {
    #[doc = "0: Boot ack is not received."]
    Noack = 0,
    #[doc = "1: Boot ack is received."]
    Ack = 1,
}
impl From<Bootackrcv> for bool {
    #[inline(always)]
    fn from(variant: Bootackrcv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BOOTACKRCV` reader - This status is set if the boot acknowledge is received from device."]
pub type BootackrcvR = crate::BitReader<Bootackrcv>;
impl BootackrcvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bootackrcv {
        match self.bits {
            false => Bootackrcv::Noack,
            true => Bootackrcv::Ack,
        }
    }
    #[doc = "Boot ack is not received."]
    #[inline(always)]
    pub fn is_noack(&self) -> bool {
        *self == Bootackrcv::Noack
    }
    #[doc = "Boot ack is received."]
    #[inline(always)]
    pub fn is_ack(&self) -> bool {
        *self == Bootackrcv::Ack
    }
}
#[doc = "Field `BOOTACKRCV` writer - This status is set if the boot acknowledge is received from device."]
pub type BootackrcvW<'a, REG> = crate::BitWriter<'a, REG, Bootackrcv>;
impl<'a, REG> BootackrcvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Boot ack is not received."]
    #[inline(always)]
    pub fn noack(self) -> &'a mut crate::W<REG> {
        self.variant(Bootackrcv::Noack)
    }
    #[doc = "Boot ack is received."]
    #[inline(always)]
    pub fn ack(self) -> &'a mut crate::W<REG> {
        self.variant(Bootackrcv::Ack)
    }
}
#[doc = "Interrupt This status is set if the boot operation get terminated\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bootterminate {
    #[doc = "0: Boot operation is not terminated."]
    Ok = 0,
    #[doc = "1: Boot operation is terminated"]
    Bootterm = 1,
}
impl From<Bootterminate> for bool {
    #[inline(always)]
    fn from(variant: Bootterminate) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BOOTTERMINATE` reader - Interrupt This status is set if the boot operation get terminated"]
pub type BootterminateR = crate::BitReader<Bootterminate>;
impl BootterminateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bootterminate {
        match self.bits {
            false => Bootterminate::Ok,
            true => Bootterminate::Bootterm,
        }
    }
    #[doc = "Boot operation is not terminated."]
    #[inline(always)]
    pub fn is_ok(&self) -> bool {
        *self == Bootterminate::Ok
    }
    #[doc = "Boot operation is terminated"]
    #[inline(always)]
    pub fn is_bootterm(&self) -> bool {
        *self == Bootterminate::Bootterm
    }
}
#[doc = "Field `BOOTTERMINATE` writer - Interrupt This status is set if the boot operation get terminated"]
pub type BootterminateW<'a, REG> = crate::BitWriter<'a, REG, Bootterminate>;
impl<'a, REG> BootterminateW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Boot operation is not terminated."]
    #[inline(always)]
    pub fn ok(self) -> &'a mut crate::W<REG> {
        self.variant(Bootterminate::Ok)
    }
    #[doc = "Boot operation is terminated"]
    #[inline(always)]
    pub fn bootterm(self) -> &'a mut crate::W<REG> {
        self.variant(Bootterminate::Bootterm)
    }
}
#[doc = "If any of the bits in the Error Interrupt Status Register are set, then this bit is set. Therefore the HD can test for an error by checking this bit first.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Errorinterrupt {
    #[doc = "0: No Error."]
    Noerror = 0,
    #[doc = "1: Error."]
    Error = 1,
}
impl From<Errorinterrupt> for bool {
    #[inline(always)]
    fn from(variant: Errorinterrupt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRORINTERRUPT` reader - If any of the bits in the Error Interrupt Status Register are set, then this bit is set. Therefore the HD can test for an error by checking this bit first."]
pub type ErrorinterruptR = crate::BitReader<Errorinterrupt>;
impl ErrorinterruptR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Errorinterrupt {
        match self.bits {
            false => Errorinterrupt::Noerror,
            true => Errorinterrupt::Error,
        }
    }
    #[doc = "No Error."]
    #[inline(always)]
    pub fn is_noerror(&self) -> bool {
        *self == Errorinterrupt::Noerror
    }
    #[doc = "Error."]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == Errorinterrupt::Error
    }
}
#[doc = "Field `ERRORINTERRUPT` writer - If any of the bits in the Error Interrupt Status Register are set, then this bit is set. Therefore the HD can test for an error by checking this bit first."]
pub type ErrorinterruptW<'a, REG> = crate::BitWriter<'a, REG, Errorinterrupt>;
impl<'a, REG> ErrorinterruptW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Error."]
    #[inline(always)]
    pub fn noerror(self) -> &'a mut crate::W<REG> {
        self.variant(Errorinterrupt::Noerror)
    }
    #[doc = "Error."]
    #[inline(always)]
    pub fn error(self) -> &'a mut crate::W<REG> {
        self.variant(Errorinterrupt::Error)
    }
}
#[doc = "Command CRC Error is generated in two cases. 1. If a response is returned and the Command Time-out Error is set to 0, this bit is set to 1 when detecting a CRT error in the command response 2. The HC detects a CMD line conflict by monitoring the CMD line when a command is issued. If the HC drives the CMD line to 1 level, but detects 0 level on the CMD line at the next SDCLK edge, then the HC shall abort the command (Stop driving CMD line) and set this bit to 1. The Command Timeout Error shall also be set to 1 to distinguish CMD line conflict.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Commandtimeouterror {
    #[doc = "0: No Error"]
    Noerror = 0,
    #[doc = "1: CRC Error Generated"]
    Error = 1,
}
impl From<Commandtimeouterror> for bool {
    #[inline(always)]
    fn from(variant: Commandtimeouterror) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMMANDTIMEOUTERROR` reader - Command CRC Error is generated in two cases. 1. If a response is returned and the Command Time-out Error is set to 0, this bit is set to 1 when detecting a CRT error in the command response 2. The HC detects a CMD line conflict by monitoring the CMD line when a command is issued. If the HC drives the CMD line to 1 level, but detects 0 level on the CMD line at the next SDCLK edge, then the HC shall abort the command (Stop driving CMD line) and set this bit to 1. The Command Timeout Error shall also be set to 1 to distinguish CMD line conflict."]
pub type CommandtimeouterrorR = crate::BitReader<Commandtimeouterror>;
impl CommandtimeouterrorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Commandtimeouterror {
        match self.bits {
            false => Commandtimeouterror::Noerror,
            true => Commandtimeouterror::Error,
        }
    }
    #[doc = "No Error"]
    #[inline(always)]
    pub fn is_noerror(&self) -> bool {
        *self == Commandtimeouterror::Noerror
    }
    #[doc = "CRC Error Generated"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == Commandtimeouterror::Error
    }
}
#[doc = "Field `COMMANDTIMEOUTERROR` writer - Command CRC Error is generated in two cases. 1. If a response is returned and the Command Time-out Error is set to 0, this bit is set to 1 when detecting a CRT error in the command response 2. The HC detects a CMD line conflict by monitoring the CMD line when a command is issued. If the HC drives the CMD line to 1 level, but detects 0 level on the CMD line at the next SDCLK edge, then the HC shall abort the command (Stop driving CMD line) and set this bit to 1. The Command Timeout Error shall also be set to 1 to distinguish CMD line conflict."]
pub type CommandtimeouterrorW<'a, REG> = crate::BitWriter<'a, REG, Commandtimeouterror>;
impl<'a, REG> CommandtimeouterrorW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Error"]
    #[inline(always)]
    pub fn noerror(self) -> &'a mut crate::W<REG> {
        self.variant(Commandtimeouterror::Noerror)
    }
    #[doc = "CRC Error Generated"]
    #[inline(always)]
    pub fn error(self) -> &'a mut crate::W<REG> {
        self.variant(Commandtimeouterror::Error)
    }
}
#[doc = "Occurs when detecting that the end bit of a command response is 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Commandcrcerror {
    #[doc = "0: No Error"]
    Noerror = 0,
    #[doc = "1: End Bit Error Generated"]
    Error = 1,
}
impl From<Commandcrcerror> for bool {
    #[inline(always)]
    fn from(variant: Commandcrcerror) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMMANDCRCERROR` reader - Occurs when detecting that the end bit of a command response is 0."]
pub type CommandcrcerrorR = crate::BitReader<Commandcrcerror>;
impl CommandcrcerrorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Commandcrcerror {
        match self.bits {
            false => Commandcrcerror::Noerror,
            true => Commandcrcerror::Error,
        }
    }
    #[doc = "No Error"]
    #[inline(always)]
    pub fn is_noerror(&self) -> bool {
        *self == Commandcrcerror::Noerror
    }
    #[doc = "End Bit Error Generated"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == Commandcrcerror::Error
    }
}
#[doc = "Field `COMMANDCRCERROR` writer - Occurs when detecting that the end bit of a command response is 0."]
pub type CommandcrcerrorW<'a, REG> = crate::BitWriter<'a, REG, Commandcrcerror>;
impl<'a, REG> CommandcrcerrorW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Error"]
    #[inline(always)]
    pub fn noerror(self) -> &'a mut crate::W<REG> {
        self.variant(Commandcrcerror::Noerror)
    }
    #[doc = "End Bit Error Generated"]
    #[inline(always)]
    pub fn error(self) -> &'a mut crate::W<REG> {
        self.variant(Commandcrcerror::Error)
    }
}
#[doc = "Occurs only if the no response is returned within 64 SDCLK cycles from the end bit of the command. If the HC detects a CMD line conflict, in which case Command CRC Error shall also be set. This bit shall be set without waiting for 64 SDCLK cycles because the command will be aborted by the HC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Commandendbiterror {
    #[doc = "0: No Error"]
    Noerror = 0,
    #[doc = "1: Timeout"]
    Error = 1,
}
impl From<Commandendbiterror> for bool {
    #[inline(always)]
    fn from(variant: Commandendbiterror) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMMANDENDBITERROR` reader - Occurs only if the no response is returned within 64 SDCLK cycles from the end bit of the command. If the HC detects a CMD line conflict, in which case Command CRC Error shall also be set. This bit shall be set without waiting for 64 SDCLK cycles because the command will be aborted by the HC."]
pub type CommandendbiterrorR = crate::BitReader<Commandendbiterror>;
impl CommandendbiterrorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Commandendbiterror {
        match self.bits {
            false => Commandendbiterror::Noerror,
            true => Commandendbiterror::Error,
        }
    }
    #[doc = "No Error"]
    #[inline(always)]
    pub fn is_noerror(&self) -> bool {
        *self == Commandendbiterror::Noerror
    }
    #[doc = "Timeout"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == Commandendbiterror::Error
    }
}
#[doc = "Field `COMMANDENDBITERROR` writer - Occurs only if the no response is returned within 64 SDCLK cycles from the end bit of the command. If the HC detects a CMD line conflict, in which case Command CRC Error shall also be set. This bit shall be set without waiting for 64 SDCLK cycles because the command will be aborted by the HC."]
pub type CommandendbiterrorW<'a, REG> = crate::BitWriter<'a, REG, Commandendbiterror>;
impl<'a, REG> CommandendbiterrorW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Error"]
    #[inline(always)]
    pub fn noerror(self) -> &'a mut crate::W<REG> {
        self.variant(Commandendbiterror::Noerror)
    }
    #[doc = "Timeout"]
    #[inline(always)]
    pub fn error(self) -> &'a mut crate::W<REG> {
        self.variant(Commandendbiterror::Error)
    }
}
#[doc = "Occurs if a Command Index error occurs in the Command Response.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Commandindexerror {
    #[doc = "0: No Error"]
    Noerror = 0,
    #[doc = "1: Error"]
    Error = 1,
}
impl From<Commandindexerror> for bool {
    #[inline(always)]
    fn from(variant: Commandindexerror) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMMANDINDEXERROR` reader - Occurs if a Command Index error occurs in the Command Response."]
pub type CommandindexerrorR = crate::BitReader<Commandindexerror>;
impl CommandindexerrorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Commandindexerror {
        match self.bits {
            false => Commandindexerror::Noerror,
            true => Commandindexerror::Error,
        }
    }
    #[doc = "No Error"]
    #[inline(always)]
    pub fn is_noerror(&self) -> bool {
        *self == Commandindexerror::Noerror
    }
    #[doc = "Error"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == Commandindexerror::Error
    }
}
#[doc = "Field `COMMANDINDEXERROR` writer - Occurs if a Command Index error occurs in the Command Response."]
pub type CommandindexerrorW<'a, REG> = crate::BitWriter<'a, REG, Commandindexerror>;
impl<'a, REG> CommandindexerrorW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Error"]
    #[inline(always)]
    pub fn noerror(self) -> &'a mut crate::W<REG> {
        self.variant(Commandindexerror::Noerror)
    }
    #[doc = "Error"]
    #[inline(always)]
    pub fn error(self) -> &'a mut crate::W<REG> {
        self.variant(Commandindexerror::Error)
    }
}
#[doc = "Occurs when detecting one of following timeout conditions. 1. Busy Timeout for R1b, R5b type. 2. Busy Timeout after Write CRC status 3. Write CRC status Timeout 4. Read Data Timeout\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Datatimeouterror {
    #[doc = "0: No Error"]
    Noerror = 0,
    #[doc = "1: Timeout"]
    Error = 1,
}
impl From<Datatimeouterror> for bool {
    #[inline(always)]
    fn from(variant: Datatimeouterror) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATATIMEOUTERROR` reader - Occurs when detecting one of following timeout conditions. 1. Busy Timeout for R1b, R5b type. 2. Busy Timeout after Write CRC status 3. Write CRC status Timeout 4. Read Data Timeout"]
pub type DatatimeouterrorR = crate::BitReader<Datatimeouterror>;
impl DatatimeouterrorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Datatimeouterror {
        match self.bits {
            false => Datatimeouterror::Noerror,
            true => Datatimeouterror::Error,
        }
    }
    #[doc = "No Error"]
    #[inline(always)]
    pub fn is_noerror(&self) -> bool {
        *self == Datatimeouterror::Noerror
    }
    #[doc = "Timeout"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == Datatimeouterror::Error
    }
}
#[doc = "Field `DATATIMEOUTERROR` writer - Occurs when detecting one of following timeout conditions. 1. Busy Timeout for R1b, R5b type. 2. Busy Timeout after Write CRC status 3. Write CRC status Timeout 4. Read Data Timeout"]
pub type DatatimeouterrorW<'a, REG> = crate::BitWriter<'a, REG, Datatimeouterror>;
impl<'a, REG> DatatimeouterrorW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Error"]
    #[inline(always)]
    pub fn noerror(self) -> &'a mut crate::W<REG> {
        self.variant(Datatimeouterror::Noerror)
    }
    #[doc = "Timeout"]
    #[inline(always)]
    pub fn error(self) -> &'a mut crate::W<REG> {
        self.variant(Datatimeouterror::Error)
    }
}
#[doc = "Occurs when detecting CRC error when transferring read data which uses the DAT line or when detecting the Write CRC Status having a value of other than 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Datacrcerror {
    #[doc = "0: No Error"]
    Noerror = 0,
    #[doc = "1: Error"]
    Error = 1,
}
impl From<Datacrcerror> for bool {
    #[inline(always)]
    fn from(variant: Datacrcerror) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATACRCERROR` reader - Occurs when detecting CRC error when transferring read data which uses the DAT line or when detecting the Write CRC Status having a value of other than 0."]
pub type DatacrcerrorR = crate::BitReader<Datacrcerror>;
impl DatacrcerrorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Datacrcerror {
        match self.bits {
            false => Datacrcerror::Noerror,
            true => Datacrcerror::Error,
        }
    }
    #[doc = "No Error"]
    #[inline(always)]
    pub fn is_noerror(&self) -> bool {
        *self == Datacrcerror::Noerror
    }
    #[doc = "Error"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == Datacrcerror::Error
    }
}
#[doc = "Field `DATACRCERROR` writer - Occurs when detecting CRC error when transferring read data which uses the DAT line or when detecting the Write CRC Status having a value of other than 0."]
pub type DatacrcerrorW<'a, REG> = crate::BitWriter<'a, REG, Datacrcerror>;
impl<'a, REG> DatacrcerrorW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Error"]
    #[inline(always)]
    pub fn noerror(self) -> &'a mut crate::W<REG> {
        self.variant(Datacrcerror::Noerror)
    }
    #[doc = "Error"]
    #[inline(always)]
    pub fn error(self) -> &'a mut crate::W<REG> {
        self.variant(Datacrcerror::Error)
    }
}
#[doc = "Occurs when detecting 0 at the end bit position of read data which uses the DAT line or the end bit position of the CRC status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dataendbiterror {
    #[doc = "0: No Error"]
    Noerror = 0,
    #[doc = "1: Error"]
    Error = 1,
}
impl From<Dataendbiterror> for bool {
    #[inline(always)]
    fn from(variant: Dataendbiterror) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATAENDBITERROR` reader - Occurs when detecting 0 at the end bit position of read data which uses the DAT line or the end bit position of the CRC status."]
pub type DataendbiterrorR = crate::BitReader<Dataendbiterror>;
impl DataendbiterrorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dataendbiterror {
        match self.bits {
            false => Dataendbiterror::Noerror,
            true => Dataendbiterror::Error,
        }
    }
    #[doc = "No Error"]
    #[inline(always)]
    pub fn is_noerror(&self) -> bool {
        *self == Dataendbiterror::Noerror
    }
    #[doc = "Error"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == Dataendbiterror::Error
    }
}
#[doc = "Field `DATAENDBITERROR` writer - Occurs when detecting 0 at the end bit position of read data which uses the DAT line or the end bit position of the CRC status."]
pub type DataendbiterrorW<'a, REG> = crate::BitWriter<'a, REG, Dataendbiterror>;
impl<'a, REG> DataendbiterrorW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Error"]
    #[inline(always)]
    pub fn noerror(self) -> &'a mut crate::W<REG> {
        self.variant(Dataendbiterror::Noerror)
    }
    #[doc = "Error"]
    #[inline(always)]
    pub fn error(self) -> &'a mut crate::W<REG> {
        self.variant(Dataendbiterror::Error)
    }
}
#[doc = "By setting the SD Bus Power bit in the Power Control Register, the HC is requested to supply power for the SD Bus. If the HC supports the Current Limit Function, it can be protected from an Illegal card by stopping power supply to the card in which case this bit indicates a failure status. Reading 1 means the HC is not supplying power to SD card due to some failure. Reading 0 means that the HC is supplying power and no error has occurred. This bit shall always set to be 0, if the HC does not support this function.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Currentlimiterror {
    #[doc = "0: No Error"]
    Noerror = 0,
    #[doc = "1: Power Fail"]
    Error = 1,
}
impl From<Currentlimiterror> for bool {
    #[inline(always)]
    fn from(variant: Currentlimiterror) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CURRENTLIMITERROR` reader - By setting the SD Bus Power bit in the Power Control Register, the HC is requested to supply power for the SD Bus. If the HC supports the Current Limit Function, it can be protected from an Illegal card by stopping power supply to the card in which case this bit indicates a failure status. Reading 1 means the HC is not supplying power to SD card due to some failure. Reading 0 means that the HC is supplying power and no error has occurred. This bit shall always set to be 0, if the HC does not support this function."]
pub type CurrentlimiterrorR = crate::BitReader<Currentlimiterror>;
impl CurrentlimiterrorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Currentlimiterror {
        match self.bits {
            false => Currentlimiterror::Noerror,
            true => Currentlimiterror::Error,
        }
    }
    #[doc = "No Error"]
    #[inline(always)]
    pub fn is_noerror(&self) -> bool {
        *self == Currentlimiterror::Noerror
    }
    #[doc = "Power Fail"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == Currentlimiterror::Error
    }
}
#[doc = "Field `CURRENTLIMITERROR` writer - By setting the SD Bus Power bit in the Power Control Register, the HC is requested to supply power for the SD Bus. If the HC supports the Current Limit Function, it can be protected from an Illegal card by stopping power supply to the card in which case this bit indicates a failure status. Reading 1 means the HC is not supplying power to SD card due to some failure. Reading 0 means that the HC is supplying power and no error has occurred. This bit shall always set to be 0, if the HC does not support this function."]
pub type CurrentlimiterrorW<'a, REG> = crate::BitWriter<'a, REG, Currentlimiterror>;
impl<'a, REG> CurrentlimiterrorW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Error"]
    #[inline(always)]
    pub fn noerror(self) -> &'a mut crate::W<REG> {
        self.variant(Currentlimiterror::Noerror)
    }
    #[doc = "Power Fail"]
    #[inline(always)]
    pub fn error(self) -> &'a mut crate::W<REG> {
        self.variant(Currentlimiterror::Error)
    }
}
#[doc = "Auto CMD12 and Auto CMD23 use this error status. This bit is set when detecting that one of the bits D00-D04 in Auto CMD Error Status register has changed from 0 to 1. In case of Auto CMD12, this bit is set to 1, not only when the errors in Auto CMD12 occur but also when Auto CMD12 is not executed due to the previous command error.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Autocmderror {
    #[doc = "0: No Error"]
    Noerror = 0,
    #[doc = "1: Error"]
    Error = 1,
}
impl From<Autocmderror> for bool {
    #[inline(always)]
    fn from(variant: Autocmderror) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUTOCMDERROR` reader - Auto CMD12 and Auto CMD23 use this error status. This bit is set when detecting that one of the bits D00-D04 in Auto CMD Error Status register has changed from 0 to 1. In case of Auto CMD12, this bit is set to 1, not only when the errors in Auto CMD12 occur but also when Auto CMD12 is not executed due to the previous command error."]
pub type AutocmderrorR = crate::BitReader<Autocmderror>;
impl AutocmderrorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Autocmderror {
        match self.bits {
            false => Autocmderror::Noerror,
            true => Autocmderror::Error,
        }
    }
    #[doc = "No Error"]
    #[inline(always)]
    pub fn is_noerror(&self) -> bool {
        *self == Autocmderror::Noerror
    }
    #[doc = "Error"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == Autocmderror::Error
    }
}
#[doc = "Field `AUTOCMDERROR` writer - Auto CMD12 and Auto CMD23 use this error status. This bit is set when detecting that one of the bits D00-D04 in Auto CMD Error Status register has changed from 0 to 1. In case of Auto CMD12, this bit is set to 1, not only when the errors in Auto CMD12 occur but also when Auto CMD12 is not executed due to the previous command error."]
pub type AutocmderrorW<'a, REG> = crate::BitWriter<'a, REG, Autocmderror>;
impl<'a, REG> AutocmderrorW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Error"]
    #[inline(always)]
    pub fn noerror(self) -> &'a mut crate::W<REG> {
        self.variant(Autocmderror::Noerror)
    }
    #[doc = "Error"]
    #[inline(always)]
    pub fn error(self) -> &'a mut crate::W<REG> {
        self.variant(Autocmderror::Error)
    }
}
#[doc = "This bit is set when the Host Controller detects errors during ADMA based data transfer. The state of the ADMA at an error occurrence is saved in the ADMA Error Status Register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Admaerror {
    #[doc = "1: Error"]
    Error = 1,
    #[doc = "0: No error"]
    Noerror = 0,
}
impl From<Admaerror> for bool {
    #[inline(always)]
    fn from(variant: Admaerror) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADMAERROR` reader - This bit is set when the Host Controller detects errors during ADMA based data transfer. The state of the ADMA at an error occurrence is saved in the ADMA Error Status Register."]
pub type AdmaerrorR = crate::BitReader<Admaerror>;
impl AdmaerrorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Admaerror {
        match self.bits {
            true => Admaerror::Error,
            false => Admaerror::Noerror,
        }
    }
    #[doc = "Error"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == Admaerror::Error
    }
    #[doc = "No error"]
    #[inline(always)]
    pub fn is_noerror(&self) -> bool {
        *self == Admaerror::Noerror
    }
}
#[doc = "Field `ADMAERROR` writer - This bit is set when the Host Controller detects errors during ADMA based data transfer. The state of the ADMA at an error occurrence is saved in the ADMA Error Status Register."]
pub type AdmaerrorW<'a, REG> = crate::BitWriter<'a, REG, Admaerror>;
impl<'a, REG> AdmaerrorW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Error"]
    #[inline(always)]
    pub fn error(self) -> &'a mut crate::W<REG> {
        self.variant(Admaerror::Error)
    }
    #[doc = "No error"]
    #[inline(always)]
    pub fn noerror(self) -> &'a mut crate::W<REG> {
        self.variant(Admaerror::Noerror)
    }
}
#[doc = "Occurs when detecting error in aximst_bresp or aximst_rresp\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tgtresperr {
    #[doc = "0: no error"]
    Noerror = 0,
    #[doc = "1: error"]
    Error = 1,
}
impl From<Tgtresperr> for bool {
    #[inline(always)]
    fn from(variant: Tgtresperr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TGTRESPERR` reader - Occurs when detecting error in aximst_bresp or aximst_rresp"]
pub type TgtresperrR = crate::BitReader<Tgtresperr>;
impl TgtresperrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tgtresperr {
        match self.bits {
            false => Tgtresperr::Noerror,
            true => Tgtresperr::Error,
        }
    }
    #[doc = "no error"]
    #[inline(always)]
    pub fn is_noerror(&self) -> bool {
        *self == Tgtresperr::Noerror
    }
    #[doc = "error"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == Tgtresperr::Error
    }
}
#[doc = "Field `TGTRESPERR` writer - Occurs when detecting error in aximst_bresp or aximst_rresp"]
pub type TgtresperrW<'a, REG> = crate::BitWriter<'a, REG, Tgtresperr>;
impl<'a, REG> TgtresperrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no error"]
    #[inline(always)]
    pub fn noerror(self) -> &'a mut crate::W<REG> {
        self.variant(Tgtresperr::Noerror)
    }
    #[doc = "error"]
    #[inline(always)]
    pub fn error(self) -> &'a mut crate::W<REG> {
        self.variant(Tgtresperr::Error)
    }
}
#[doc = "Vendor specific error status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Vnderrstat {
    #[doc = "1: Ready"]
    Ready = 1,
    #[doc = "0: Not Ready"]
    Notready = 0,
}
impl From<Vnderrstat> for u8 {
    #[inline(always)]
    fn from(variant: Vnderrstat) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Vnderrstat {
    type Ux = u8;
}
impl crate::IsEnum for Vnderrstat {}
#[doc = "Field `VNDERRSTAT` reader - Vendor specific error status."]
pub type VnderrstatR = crate::FieldReader<Vnderrstat>;
impl VnderrstatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Vnderrstat> {
        match self.bits {
            1 => Some(Vnderrstat::Ready),
            0 => Some(Vnderrstat::Notready),
            _ => None,
        }
    }
    #[doc = "Ready"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == Vnderrstat::Ready
    }
    #[doc = "Not Ready"]
    #[inline(always)]
    pub fn is_notready(&self) -> bool {
        *self == Vnderrstat::Notready
    }
}
#[doc = "Field `VNDERRSTAT` writer - Vendor specific error status."]
pub type VnderrstatW<'a, REG> = crate::FieldWriter<'a, REG, 3, Vnderrstat>;
impl<'a, REG> VnderrstatW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Ready"]
    #[inline(always)]
    pub fn ready(self) -> &'a mut crate::W<REG> {
        self.variant(Vnderrstat::Ready)
    }
    #[doc = "Not Ready"]
    #[inline(always)]
    pub fn notready(self) -> &'a mut crate::W<REG> {
        self.variant(Vnderrstat::Notready)
    }
}
impl R {
    #[doc = "Bit 0 - This bit is set when we get the end bit of the command response (Except Auto CMD12 and Auto CMD23) Note: Command Time-out Error has higher priority than Command Complete. If both are set to 1, it can be considered that the response was not received correctly."]
    #[inline(always)]
    pub fn commandcomplete(&self) -> CommandcompleteR {
        CommandcompleteR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit is set when a read / write transaction is completed. Read Transaction: This bit is set at the falling edge of Read Transfer Active Status. There are two cases in which the Interrupt is generated. The first is when a data transfer is completed as specified by data length (After the last data has been read to the Host System). The second is when data has stopped at the block gap and completed the data transfer by setting the Stop At Block Gap Request in the Block Gap Control Register (After valid data has been read to the Host System). Write Transaction: This bit is set at the falling edge of the DAT Line Active Status. There are two cases in which the Interrupt is generated. The first is when the last data is written to the card as specified by data length and Busy signal is released. The second is when data transfers are stopped at the block gap by setting Stop At Block Gap Request in the Block Gap Control Register and data transfers completed. (After valid data is written to the SD card and the busy signal is released). Note: Transfer Complete has higher priority than Data Time-out Error. If both bits are set to 1, the data transfer can be considered complete Note: While performing tuning procedure (Execute Tuning is set to 1), Transfer Complete is not set to 1"]
    #[inline(always)]
    pub fn transfercomplete(&self) -> TransfercompleteR {
        TransfercompleteR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - If the Stop At Block Gap Request in the Block Gap Control Register is set, this bit is set. Read Transaction: This bit is set at the falling edge of the DAT Line Active Status (When the transaction is stopped at SD Bus timing. The Read Wait must be supported inorder to use this function). Write Transaction: This bit is set at the falling edge of Write Transfer Active Status (After getting CRC status at SD Bus timing)."]
    #[inline(always)]
    pub fn blockgapevent(&self) -> BlockgapeventR {
        BlockgapeventR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This status is set if the HC detects the Host DMA Buffer Boundary in the Block Size regiser."]
    #[inline(always)]
    pub fn dmainterrupt(&self) -> DmainterruptR {
        DmainterruptR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This status is set if the Buffer Write Enable changes from 0 to 1."]
    #[inline(always)]
    pub fn bufferwriteready(&self) -> BufferwritereadyR {
        BufferwritereadyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This status is set if the Buffer Read Enable changes from 0 to 1. Buffer Read Ready is set to 1 for every CMD19 execution in tuning procedure."]
    #[inline(always)]
    pub fn bufferreadready(&self) -> BufferreadreadyR {
        BufferreadreadyR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This status is set if the Card Inserted in the Present State register changes from 0 to 1. When the HD writes this bit to 1 to clear this status the status of the Card Inserted in the Present State register should be confirmed. Because the card detect may possibly be changed when the HD clear this bit an Interrupt event may not be generated."]
    #[inline(always)]
    pub fn cardinsertion(&self) -> CardinsertionR {
        CardinsertionR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This status is set if the Card Inserted in the Present State register changes from 1 to 0. When the HD writes this bit to 1 to clear this status the status of the Card Inserted in the Present State register should be confirmed. Because the card detect may possibly be changed when the HD clear this bit an Interrupt event may not be generated."]
    #[inline(always)]
    pub fn cardremoval(&self) -> CardremovalR {
        CardremovalR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Writing this bit to 1 does not clear this bit. It is cleared by resetting the SD card interrupt factor. In 1-bit mode, the HC shall detect the Card Interrupt without SD Clock to support wakeup. In 4-bit mode, the card interrupt signal is sampled during the interrupt cycle, so there are some sample delays between the interrupt signal from the card and the interrupt to the Host system. when this status has been set and the HD needs to start this interrupt service, Card Interrupt Status Enable in the Normal Interrupt Status register shall be set to 0 in order to clear the card interrupt statuses latched in the HC and stop driving the Host System. After completion of the card interrupt service (the reset factor in the SD card and the interrupt signal may not be asserted), set Card Interrupt Status Enable to 1 and start sampling the interrupt signal again. Interrupt detected by DAT\\[1\\]
is supported when there is a card per slot. In case of shared bus, interrupt pins are used to detect interrupts. If 000b is set to Interrupt Pin Select in the Shared Bus Control register, this status is effective. Non-zero value is set to Interrupt Pin Select, INT_A, INT_B or INT_C is then used to device interrupts."]
    #[inline(always)]
    pub fn cardinterrupt(&self) -> CardinterruptR {
        CardinterruptR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - This status is set if INT_A is enabled and INT_A# pin is in low level. Writing this bit to 1 does not clear this bit. It is cleared by resetting the INT_A interrupt factor"]
    #[inline(always)]
    pub fn inta(&self) -> IntaR {
        IntaR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - This status is set if INT_B is enabled and INT_B# pin is in low level. Writing this bit to 1 does not clear this bit. It is cleared by resetting the INT_B interrupt factor"]
    #[inline(always)]
    pub fn intb(&self) -> IntbR {
        IntbR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - This status is set if INT_C is enabled and INT_C# pin is in low level. Writing this bit to 1 does not clear this bit. It is cleared by resetting the INT_C interrupt factor"]
    #[inline(always)]
    pub fn intc(&self) -> IntcR {
        IntcR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - This status is set if Re-Tuning Request in the Present State register changes from 0 to 1. Host Controller requests Host Driver to perform re-tuning for next data transfer. Current data transfer (not large block count) can be completed without re-tuning."]
    #[inline(always)]
    pub fn retuningevent(&self) -> RetuningeventR {
        RetuningeventR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - This status is set if the boot acknowledge is received from device."]
    #[inline(always)]
    pub fn bootackrcv(&self) -> BootackrcvR {
        BootackrcvR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Interrupt This status is set if the boot operation get terminated"]
    #[inline(always)]
    pub fn bootterminate(&self) -> BootterminateR {
        BootterminateR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - If any of the bits in the Error Interrupt Status Register are set, then this bit is set. Therefore the HD can test for an error by checking this bit first."]
    #[inline(always)]
    pub fn errorinterrupt(&self) -> ErrorinterruptR {
        ErrorinterruptR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Command CRC Error is generated in two cases. 1. If a response is returned and the Command Time-out Error is set to 0, this bit is set to 1 when detecting a CRT error in the command response 2. The HC detects a CMD line conflict by monitoring the CMD line when a command is issued. If the HC drives the CMD line to 1 level, but detects 0 level on the CMD line at the next SDCLK edge, then the HC shall abort the command (Stop driving CMD line) and set this bit to 1. The Command Timeout Error shall also be set to 1 to distinguish CMD line conflict."]
    #[inline(always)]
    pub fn commandtimeouterror(&self) -> CommandtimeouterrorR {
        CommandtimeouterrorR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Occurs when detecting that the end bit of a command response is 0."]
    #[inline(always)]
    pub fn commandcrcerror(&self) -> CommandcrcerrorR {
        CommandcrcerrorR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Occurs only if the no response is returned within 64 SDCLK cycles from the end bit of the command. If the HC detects a CMD line conflict, in which case Command CRC Error shall also be set. This bit shall be set without waiting for 64 SDCLK cycles because the command will be aborted by the HC."]
    #[inline(always)]
    pub fn commandendbiterror(&self) -> CommandendbiterrorR {
        CommandendbiterrorR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Occurs if a Command Index error occurs in the Command Response."]
    #[inline(always)]
    pub fn commandindexerror(&self) -> CommandindexerrorR {
        CommandindexerrorR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Occurs when detecting one of following timeout conditions. 1. Busy Timeout for R1b, R5b type. 2. Busy Timeout after Write CRC status 3. Write CRC status Timeout 4. Read Data Timeout"]
    #[inline(always)]
    pub fn datatimeouterror(&self) -> DatatimeouterrorR {
        DatatimeouterrorR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Occurs when detecting CRC error when transferring read data which uses the DAT line or when detecting the Write CRC Status having a value of other than 0."]
    #[inline(always)]
    pub fn datacrcerror(&self) -> DatacrcerrorR {
        DatacrcerrorR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Occurs when detecting 0 at the end bit position of read data which uses the DAT line or the end bit position of the CRC status."]
    #[inline(always)]
    pub fn dataendbiterror(&self) -> DataendbiterrorR {
        DataendbiterrorR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - By setting the SD Bus Power bit in the Power Control Register, the HC is requested to supply power for the SD Bus. If the HC supports the Current Limit Function, it can be protected from an Illegal card by stopping power supply to the card in which case this bit indicates a failure status. Reading 1 means the HC is not supplying power to SD card due to some failure. Reading 0 means that the HC is supplying power and no error has occurred. This bit shall always set to be 0, if the HC does not support this function."]
    #[inline(always)]
    pub fn currentlimiterror(&self) -> CurrentlimiterrorR {
        CurrentlimiterrorR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Auto CMD12 and Auto CMD23 use this error status. This bit is set when detecting that one of the bits D00-D04 in Auto CMD Error Status register has changed from 0 to 1. In case of Auto CMD12, this bit is set to 1, not only when the errors in Auto CMD12 occur but also when Auto CMD12 is not executed due to the previous command error."]
    #[inline(always)]
    pub fn autocmderror(&self) -> AutocmderrorR {
        AutocmderrorR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - This bit is set when the Host Controller detects errors during ADMA based data transfer. The state of the ADMA at an error occurrence is saved in the ADMA Error Status Register."]
    #[inline(always)]
    pub fn admaerror(&self) -> AdmaerrorR {
        AdmaerrorR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 28 - Occurs when detecting error in aximst_bresp or aximst_rresp"]
    #[inline(always)]
    pub fn tgtresperr(&self) -> TgtresperrR {
        TgtresperrR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:31 - Vendor specific error status."]
    #[inline(always)]
    pub fn vnderrstat(&self) -> VnderrstatR {
        VnderrstatR::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - This bit is set when we get the end bit of the command response (Except Auto CMD12 and Auto CMD23) Note: Command Time-out Error has higher priority than Command Complete. If both are set to 1, it can be considered that the response was not received correctly."]
    #[inline(always)]
    #[must_use]
    pub fn commandcomplete(&mut self) -> CommandcompleteW<IntstatSpec> {
        CommandcompleteW::new(self, 0)
    }
    #[doc = "Bit 1 - This bit is set when a read / write transaction is completed. Read Transaction: This bit is set at the falling edge of Read Transfer Active Status. There are two cases in which the Interrupt is generated. The first is when a data transfer is completed as specified by data length (After the last data has been read to the Host System). The second is when data has stopped at the block gap and completed the data transfer by setting the Stop At Block Gap Request in the Block Gap Control Register (After valid data has been read to the Host System). Write Transaction: This bit is set at the falling edge of the DAT Line Active Status. There are two cases in which the Interrupt is generated. The first is when the last data is written to the card as specified by data length and Busy signal is released. The second is when data transfers are stopped at the block gap by setting Stop At Block Gap Request in the Block Gap Control Register and data transfers completed. (After valid data is written to the SD card and the busy signal is released). Note: Transfer Complete has higher priority than Data Time-out Error. If both bits are set to 1, the data transfer can be considered complete Note: While performing tuning procedure (Execute Tuning is set to 1), Transfer Complete is not set to 1"]
    #[inline(always)]
    #[must_use]
    pub fn transfercomplete(&mut self) -> TransfercompleteW<IntstatSpec> {
        TransfercompleteW::new(self, 1)
    }
    #[doc = "Bit 2 - If the Stop At Block Gap Request in the Block Gap Control Register is set, this bit is set. Read Transaction: This bit is set at the falling edge of the DAT Line Active Status (When the transaction is stopped at SD Bus timing. The Read Wait must be supported inorder to use this function). Write Transaction: This bit is set at the falling edge of Write Transfer Active Status (After getting CRC status at SD Bus timing)."]
    #[inline(always)]
    #[must_use]
    pub fn blockgapevent(&mut self) -> BlockgapeventW<IntstatSpec> {
        BlockgapeventW::new(self, 2)
    }
    #[doc = "Bit 3 - This status is set if the HC detects the Host DMA Buffer Boundary in the Block Size regiser."]
    #[inline(always)]
    #[must_use]
    pub fn dmainterrupt(&mut self) -> DmainterruptW<IntstatSpec> {
        DmainterruptW::new(self, 3)
    }
    #[doc = "Bit 4 - This status is set if the Buffer Write Enable changes from 0 to 1."]
    #[inline(always)]
    #[must_use]
    pub fn bufferwriteready(&mut self) -> BufferwritereadyW<IntstatSpec> {
        BufferwritereadyW::new(self, 4)
    }
    #[doc = "Bit 5 - This status is set if the Buffer Read Enable changes from 0 to 1. Buffer Read Ready is set to 1 for every CMD19 execution in tuning procedure."]
    #[inline(always)]
    #[must_use]
    pub fn bufferreadready(&mut self) -> BufferreadreadyW<IntstatSpec> {
        BufferreadreadyW::new(self, 5)
    }
    #[doc = "Bit 6 - This status is set if the Card Inserted in the Present State register changes from 0 to 1. When the HD writes this bit to 1 to clear this status the status of the Card Inserted in the Present State register should be confirmed. Because the card detect may possibly be changed when the HD clear this bit an Interrupt event may not be generated."]
    #[inline(always)]
    #[must_use]
    pub fn cardinsertion(&mut self) -> CardinsertionW<IntstatSpec> {
        CardinsertionW::new(self, 6)
    }
    #[doc = "Bit 7 - This status is set if the Card Inserted in the Present State register changes from 1 to 0. When the HD writes this bit to 1 to clear this status the status of the Card Inserted in the Present State register should be confirmed. Because the card detect may possibly be changed when the HD clear this bit an Interrupt event may not be generated."]
    #[inline(always)]
    #[must_use]
    pub fn cardremoval(&mut self) -> CardremovalW<IntstatSpec> {
        CardremovalW::new(self, 7)
    }
    #[doc = "Bit 8 - Writing this bit to 1 does not clear this bit. It is cleared by resetting the SD card interrupt factor. In 1-bit mode, the HC shall detect the Card Interrupt without SD Clock to support wakeup. In 4-bit mode, the card interrupt signal is sampled during the interrupt cycle, so there are some sample delays between the interrupt signal from the card and the interrupt to the Host system. when this status has been set and the HD needs to start this interrupt service, Card Interrupt Status Enable in the Normal Interrupt Status register shall be set to 0 in order to clear the card interrupt statuses latched in the HC and stop driving the Host System. After completion of the card interrupt service (the reset factor in the SD card and the interrupt signal may not be asserted), set Card Interrupt Status Enable to 1 and start sampling the interrupt signal again. Interrupt detected by DAT\\[1\\]
is supported when there is a card per slot. In case of shared bus, interrupt pins are used to detect interrupts. If 000b is set to Interrupt Pin Select in the Shared Bus Control register, this status is effective. Non-zero value is set to Interrupt Pin Select, INT_A, INT_B or INT_C is then used to device interrupts."]
    #[inline(always)]
    #[must_use]
    pub fn cardinterrupt(&mut self) -> CardinterruptW<IntstatSpec> {
        CardinterruptW::new(self, 8)
    }
    #[doc = "Bit 9 - This status is set if INT_A is enabled and INT_A# pin is in low level. Writing this bit to 1 does not clear this bit. It is cleared by resetting the INT_A interrupt factor"]
    #[inline(always)]
    #[must_use]
    pub fn inta(&mut self) -> IntaW<IntstatSpec> {
        IntaW::new(self, 9)
    }
    #[doc = "Bit 10 - This status is set if INT_B is enabled and INT_B# pin is in low level. Writing this bit to 1 does not clear this bit. It is cleared by resetting the INT_B interrupt factor"]
    #[inline(always)]
    #[must_use]
    pub fn intb(&mut self) -> IntbW<IntstatSpec> {
        IntbW::new(self, 10)
    }
    #[doc = "Bit 11 - This status is set if INT_C is enabled and INT_C# pin is in low level. Writing this bit to 1 does not clear this bit. It is cleared by resetting the INT_C interrupt factor"]
    #[inline(always)]
    #[must_use]
    pub fn intc(&mut self) -> IntcW<IntstatSpec> {
        IntcW::new(self, 11)
    }
    #[doc = "Bit 12 - This status is set if Re-Tuning Request in the Present State register changes from 0 to 1. Host Controller requests Host Driver to perform re-tuning for next data transfer. Current data transfer (not large block count) can be completed without re-tuning."]
    #[inline(always)]
    #[must_use]
    pub fn retuningevent(&mut self) -> RetuningeventW<IntstatSpec> {
        RetuningeventW::new(self, 12)
    }
    #[doc = "Bit 13 - This status is set if the boot acknowledge is received from device."]
    #[inline(always)]
    #[must_use]
    pub fn bootackrcv(&mut self) -> BootackrcvW<IntstatSpec> {
        BootackrcvW::new(self, 13)
    }
    #[doc = "Bit 14 - Interrupt This status is set if the boot operation get terminated"]
    #[inline(always)]
    #[must_use]
    pub fn bootterminate(&mut self) -> BootterminateW<IntstatSpec> {
        BootterminateW::new(self, 14)
    }
    #[doc = "Bit 15 - If any of the bits in the Error Interrupt Status Register are set, then this bit is set. Therefore the HD can test for an error by checking this bit first."]
    #[inline(always)]
    #[must_use]
    pub fn errorinterrupt(&mut self) -> ErrorinterruptW<IntstatSpec> {
        ErrorinterruptW::new(self, 15)
    }
    #[doc = "Bit 16 - Command CRC Error is generated in two cases. 1. If a response is returned and the Command Time-out Error is set to 0, this bit is set to 1 when detecting a CRT error in the command response 2. The HC detects a CMD line conflict by monitoring the CMD line when a command is issued. If the HC drives the CMD line to 1 level, but detects 0 level on the CMD line at the next SDCLK edge, then the HC shall abort the command (Stop driving CMD line) and set this bit to 1. The Command Timeout Error shall also be set to 1 to distinguish CMD line conflict."]
    #[inline(always)]
    #[must_use]
    pub fn commandtimeouterror(&mut self) -> CommandtimeouterrorW<IntstatSpec> {
        CommandtimeouterrorW::new(self, 16)
    }
    #[doc = "Bit 17 - Occurs when detecting that the end bit of a command response is 0."]
    #[inline(always)]
    #[must_use]
    pub fn commandcrcerror(&mut self) -> CommandcrcerrorW<IntstatSpec> {
        CommandcrcerrorW::new(self, 17)
    }
    #[doc = "Bit 18 - Occurs only if the no response is returned within 64 SDCLK cycles from the end bit of the command. If the HC detects a CMD line conflict, in which case Command CRC Error shall also be set. This bit shall be set without waiting for 64 SDCLK cycles because the command will be aborted by the HC."]
    #[inline(always)]
    #[must_use]
    pub fn commandendbiterror(&mut self) -> CommandendbiterrorW<IntstatSpec> {
        CommandendbiterrorW::new(self, 18)
    }
    #[doc = "Bit 19 - Occurs if a Command Index error occurs in the Command Response."]
    #[inline(always)]
    #[must_use]
    pub fn commandindexerror(&mut self) -> CommandindexerrorW<IntstatSpec> {
        CommandindexerrorW::new(self, 19)
    }
    #[doc = "Bit 20 - Occurs when detecting one of following timeout conditions. 1. Busy Timeout for R1b, R5b type. 2. Busy Timeout after Write CRC status 3. Write CRC status Timeout 4. Read Data Timeout"]
    #[inline(always)]
    #[must_use]
    pub fn datatimeouterror(&mut self) -> DatatimeouterrorW<IntstatSpec> {
        DatatimeouterrorW::new(self, 20)
    }
    #[doc = "Bit 21 - Occurs when detecting CRC error when transferring read data which uses the DAT line or when detecting the Write CRC Status having a value of other than 0."]
    #[inline(always)]
    #[must_use]
    pub fn datacrcerror(&mut self) -> DatacrcerrorW<IntstatSpec> {
        DatacrcerrorW::new(self, 21)
    }
    #[doc = "Bit 22 - Occurs when detecting 0 at the end bit position of read data which uses the DAT line or the end bit position of the CRC status."]
    #[inline(always)]
    #[must_use]
    pub fn dataendbiterror(&mut self) -> DataendbiterrorW<IntstatSpec> {
        DataendbiterrorW::new(self, 22)
    }
    #[doc = "Bit 23 - By setting the SD Bus Power bit in the Power Control Register, the HC is requested to supply power for the SD Bus. If the HC supports the Current Limit Function, it can be protected from an Illegal card by stopping power supply to the card in which case this bit indicates a failure status. Reading 1 means the HC is not supplying power to SD card due to some failure. Reading 0 means that the HC is supplying power and no error has occurred. This bit shall always set to be 0, if the HC does not support this function."]
    #[inline(always)]
    #[must_use]
    pub fn currentlimiterror(&mut self) -> CurrentlimiterrorW<IntstatSpec> {
        CurrentlimiterrorW::new(self, 23)
    }
    #[doc = "Bit 24 - Auto CMD12 and Auto CMD23 use this error status. This bit is set when detecting that one of the bits D00-D04 in Auto CMD Error Status register has changed from 0 to 1. In case of Auto CMD12, this bit is set to 1, not only when the errors in Auto CMD12 occur but also when Auto CMD12 is not executed due to the previous command error."]
    #[inline(always)]
    #[must_use]
    pub fn autocmderror(&mut self) -> AutocmderrorW<IntstatSpec> {
        AutocmderrorW::new(self, 24)
    }
    #[doc = "Bit 25 - This bit is set when the Host Controller detects errors during ADMA based data transfer. The state of the ADMA at an error occurrence is saved in the ADMA Error Status Register."]
    #[inline(always)]
    #[must_use]
    pub fn admaerror(&mut self) -> AdmaerrorW<IntstatSpec> {
        AdmaerrorW::new(self, 25)
    }
    #[doc = "Bit 28 - Occurs when detecting error in aximst_bresp or aximst_rresp"]
    #[inline(always)]
    #[must_use]
    pub fn tgtresperr(&mut self) -> TgtresperrW<IntstatSpec> {
        TgtresperrW::new(self, 28)
    }
    #[doc = "Bits 29:31 - Vendor specific error status."]
    #[inline(always)]
    #[must_use]
    pub fn vnderrstat(&mut self) -> VnderrstatW<IntstatSpec> {
        VnderrstatW::new(self, 29)
    }
}
#[doc = "Interrupt enable\n\nYou can [`read`](crate::Reg::read) this register and get [`intstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntstatSpec;
impl crate::RegisterSpec for IntstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intstat::R`](R) reader structure"]
impl crate::Readable for IntstatSpec {}
#[doc = "`write(|w| ..)` method takes [`intstat::W`](W) writer structure"]
impl crate::Writable for IntstatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTSTAT to value 0"]
impl crate::Resettable for IntstatSpec {
    const RESET_VALUE: u32 = 0;
}
