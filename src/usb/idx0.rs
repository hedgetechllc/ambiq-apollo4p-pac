#[doc = "Register `IDX0` reader"]
pub type R = crate::R<Idx0Spec>;
#[doc = "Register `IDX0` writer"]
pub type W = crate::W<Idx0Spec>;
#[doc = "Field `MAXPAYLOAD` reader - Maximum Payload transmitted in a single transaction. The total amount of data represented by MAXPAYLOAD x (PKTSPLITOPTION + 1) must not exceed the FIFO size for the IN endpoint, and should not exceed half the FIFO size if double-buffering is required. Note: The value written here (multiplied by PKTSPLITOPTION + 1 in the case of high-bandwidth Isochronous transfers) must match the value given in the wMaxPacketSize field of the Standard Endpoint Descriptor for the associated endpoint (see USB Specification Revision 2.0, Chapter 9). A mismatch could cause unexpected results."]
pub type MaxpayloadR = crate::FieldReader<u16>;
#[doc = "Field `MAXPAYLOAD` writer - Maximum Payload transmitted in a single transaction. The total amount of data represented by MAXPAYLOAD x (PKTSPLITOPTION + 1) must not exceed the FIFO size for the IN endpoint, and should not exceed half the FIFO size if double-buffering is required. Note: The value written here (multiplied by PKTSPLITOPTION + 1 in the case of high-bandwidth Isochronous transfers) must match the value given in the wMaxPacketSize field of the Standard Endpoint Descriptor for the associated endpoint (see USB Specification Revision 2.0, Chapter 9). A mismatch could cause unexpected results."]
pub type MaxpayloadW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `PKTSPLITOPTION` reader - Packet Split Option. When IDX0_ISO = 1, this bit serves as the MAXPAYLOAD multiplier for Isochronous IN transfers. When IDX0_ISO = 0, this bit serves as the MAXPAYLOAD multiplier for Bulk IN transfers.If IDX0_ISO = 0x1, this field sets the multiplier for Isochronous transfers. For Isochronous endpoints operating in High-Speed mode and with the High-bandwidth option enabled, PKTSPLITOPTION may be either 2 or 3 (corresponding to this field's bit 0 set or bit 1 set, respectively, and bits\\[4:2\\]
are ignored) and it specifies the maximum number of such transactions that can take place in a single microframe. If either bit 0 or bit 1 is non-zero, the USB Controller will automatically split any data packet written to the FIFO into up to 2 or 3 'USB' packets, each containing the specified MAXPAYLOAD (or less). The maximum payload for each transaction is 1024 bytes, so this allows up to 3072 bytes to be transmitted in each microframe. (For Isochronous transfers in full-speed mode or if High-bandwidth is not enabled, bits 0 and 1 are ignored.)If IDX0_ISO = 0x0, this field sets the multiplier for Bulk transfers. The field setting can be up to 31, and the multiplier, m, is (field setting + 1). The multiplier defines the maximum number of 'USB' packets of the specified MAXPAYLOAD data bytes into which a single data packet placed in the FIFO should be split prior to transfer. The data packet is required to be an exact multiple of the MAXPAYLOAD, which is itself required to be either 8, 16, 32, 64 or (in the case of High Speed transfers) 512 bytes."]
pub type PktsplitoptionR = crate::FieldReader;
#[doc = "Field `PKTSPLITOPTION` writer - Packet Split Option. When IDX0_ISO = 1, this bit serves as the MAXPAYLOAD multiplier for Isochronous IN transfers. When IDX0_ISO = 0, this bit serves as the MAXPAYLOAD multiplier for Bulk IN transfers.If IDX0_ISO = 0x1, this field sets the multiplier for Isochronous transfers. For Isochronous endpoints operating in High-Speed mode and with the High-bandwidth option enabled, PKTSPLITOPTION may be either 2 or 3 (corresponding to this field's bit 0 set or bit 1 set, respectively, and bits\\[4:2\\]
are ignored) and it specifies the maximum number of such transactions that can take place in a single microframe. If either bit 0 or bit 1 is non-zero, the USB Controller will automatically split any data packet written to the FIFO into up to 2 or 3 'USB' packets, each containing the specified MAXPAYLOAD (or less). The maximum payload for each transaction is 1024 bytes, so this allows up to 3072 bytes to be transmitted in each microframe. (For Isochronous transfers in full-speed mode or if High-bandwidth is not enabled, bits 0 and 1 are ignored.)If IDX0_ISO = 0x0, this field sets the multiplier for Bulk transfers. The field setting can be up to 31, and the multiplier, m, is (field setting + 1). The multiplier defines the maximum number of 'USB' packets of the specified MAXPAYLOAD data bytes into which a single data packet placed in the FIFO should be split prior to transfer. The data packet is required to be an exact multiple of the MAXPAYLOAD, which is itself required to be either 8, 16, 32, 64 or (in the case of High Speed transfers) 512 bytes."]
pub type PktsplitoptionW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `InPktRdyOutPktRdy` reader - IN Packet Ready / OUT Packet Ready. When CFG3_ENDPOINT > 0, this bit serves as the InPktRdy field. When CFG3_ENDPOINT = 0, this bit serves as the OutPkyRdy bit.If CFG3_ENDPOINT = 0x1-0x5, this bit serves as the InPktRdy field. Set this bit after loading a data packet into the FIFO. It is cleared automatically when a data packet has been transmitted. If the FIFO is double-buffered, it is also automatically cleared when there is space for a second packet in the FIFO. An interrupt is generate (if enabled) when the bit is cleared.If CFG3_ENDPOINT = 0x0, this bit serves as the OutPkyRdy bit. This bit is set when a data packet has been received. An interrupt is generate when this bit is set (if enabled). The CPU clears this bit by setting the ServicedOutPktRdy bit."]
pub type InPktRdyOutPktRdyR = crate::BitReader;
#[doc = "Field `InPktRdyOutPktRdy` writer - IN Packet Ready / OUT Packet Ready. When CFG3_ENDPOINT > 0, this bit serves as the InPktRdy field. When CFG3_ENDPOINT = 0, this bit serves as the OutPkyRdy bit.If CFG3_ENDPOINT = 0x1-0x5, this bit serves as the InPktRdy field. Set this bit after loading a data packet into the FIFO. It is cleared automatically when a data packet has been transmitted. If the FIFO is double-buffered, it is also automatically cleared when there is space for a second packet in the FIFO. An interrupt is generate (if enabled) when the bit is cleared.If CFG3_ENDPOINT = 0x0, this bit serves as the OutPkyRdy bit. This bit is set when a data packet has been received. An interrupt is generate when this bit is set (if enabled). The CPU clears this bit by setting the ServicedOutPktRdy bit."]
pub type InPktRdyOutPktRdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFONotEmptyInPktRdy` reader - FIFO Not Empty / IN Packet Ready. When CFG3_ENDPOINT = 1 to 5, this bit serves as the FIFONotEmpty field. When CFG3_ENDPOINT = 0, this bit serves as the InPktRdy bit.If CFG3_ENDPOINT = 0x1-0x5, this bit serves as the FIFONotEmpty field. It is set when there is at least 1 packet in the IN FIFO.If CFG3_ENDPOINT = 0x0, this bit serves as the InPktRdy bit. Set this bit after loading a data packet into the FIFO. It is cleared automatically when the data packet has been transmitted. An interrupt is generated when this bit is cleared (if enabled)."]
pub type FifonotEmptyInPktRdyR = crate::BitReader;
#[doc = "Field `FIFONotEmptyInPktRdy` writer - FIFO Not Empty / IN Packet Ready. When CFG3_ENDPOINT = 1 to 5, this bit serves as the FIFONotEmpty field. When CFG3_ENDPOINT = 0, this bit serves as the InPktRdy bit.If CFG3_ENDPOINT = 0x1-0x5, this bit serves as the FIFONotEmpty field. It is set when there is at least 1 packet in the IN FIFO.If CFG3_ENDPOINT = 0x0, this bit serves as the InPktRdy bit. Set this bit after loading a data packet into the FIFO. It is cleared automatically when the data packet has been transmitted. An interrupt is generated when this bit is cleared (if enabled)."]
pub type FifonotEmptyInPktRdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UnderRunSentStall` reader - Under Run / Sent Stall. When CFG3_ENDPOINT = 1 to 5, this bit serves as the UnderRun field. When CFG3_ENDPOINT = 0, this bit serves as the SentStall field.If CFG3_ENDPOINT = 0x1-0x5, this bit serves as the UnderRun field. In ISO mode this bit is set when a zero length data packet is sent after receiving an IN token with the InPktRdy bit not set. In Bulk/Interrupt mode, this bit is set when a NAK is returned in response to an IN token. The CPU should clear this bit.If CFG3_ENDPOINT = 0x0, this bit serves as the SentStall field. This bit is set when a STALL handshake is transmitted. The CPU should clear this bit."]
pub type UnderRunSentStallR = crate::BitReader;
#[doc = "Field `UnderRunSentStall` writer - Under Run / Sent Stall. When CFG3_ENDPOINT = 1 to 5, this bit serves as the UnderRun field. When CFG3_ENDPOINT = 0, this bit serves as the SentStall field.If CFG3_ENDPOINT = 0x1-0x5, this bit serves as the UnderRun field. In ISO mode this bit is set when a zero length data packet is sent after receiving an IN token with the InPktRdy bit not set. In Bulk/Interrupt mode, this bit is set when a NAK is returned in response to an IN token. The CPU should clear this bit.If CFG3_ENDPOINT = 0x0, this bit serves as the SentStall field. This bit is set when a STALL handshake is transmitted. The CPU should clear this bit."]
pub type UnderRunSentStallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FlushFIFODataEnd` reader - When CFG3_ENDPOINT = 1 to 5, this bit serves as the FlushFIFO field. When CFG3_ENDPOINT = 0, this bit serves as the DataEnd bit.If CFG3_ENDPOINT = 0x1-0x5, this bit serves as the FlushFIFO field. Setting this bit flushes the next packet to be transmitted from the endpoint IN FIFO. The FIFO pointer is reset and the InPktRdy bit is cleared. May be set simultaneously with InPktRdy to abort the packet that has just been loaded into the FIFO.Note 1: FlushFIFO should only be set when InPktRdy is set (at other times, it may cause data corruption).Note 2: If the FIFO contains two packets, FlushFIFO may need to be set twice to completely clear the FIFO.If CFG3_ENDPOINT = 0x0, this bit serves as the DataEnd bit. Set this bit when (1) setting InPktRdy for the last data packet, (2) clearing OutPktRdy after unloading the last data packet, or (3) setting InPktRdy for a zero length data packet.It is cleared automatically."]
pub type FlushFifodataEndR = crate::BitReader;
#[doc = "Field `FlushFIFODataEnd` writer - When CFG3_ENDPOINT = 1 to 5, this bit serves as the FlushFIFO field. When CFG3_ENDPOINT = 0, this bit serves as the DataEnd bit.If CFG3_ENDPOINT = 0x1-0x5, this bit serves as the FlushFIFO field. Setting this bit flushes the next packet to be transmitted from the endpoint IN FIFO. The FIFO pointer is reset and the InPktRdy bit is cleared. May be set simultaneously with InPktRdy to abort the packet that has just been loaded into the FIFO.Note 1: FlushFIFO should only be set when InPktRdy is set (at other times, it may cause data corruption).Note 2: If the FIFO contains two packets, FlushFIFO may need to be set twice to completely clear the FIFO.If CFG3_ENDPOINT = 0x0, this bit serves as the DataEnd bit. Set this bit when (1) setting InPktRdy for the last data packet, (2) clearing OutPktRdy after unloading the last data packet, or (3) setting InPktRdy for a zero length data packet.It is cleared automatically."]
pub type FlushFifodataEndW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SendStallSetupEnd` reader - When CFG3_ENDPOINT = 1 to 5, this bit serves as the SendStall field. When CFG3_ENDPOINT = 0, this bit serves as the SetupEnd field.If CFG3_ENDPOINT = 0x1-0x5, this bit serves as the SendStall field. Setting this bit issues a STALL handshake to an IN token. The CPU clears this bit to terminate the stall condition.Note: This bit has no effect when the endpoint is being used for Isochronous transfers.If CFG3_ENDPOINT = 0x0, this bit serves as the SetupEnd field. It is set when a control transaction ends before the DataEnd bit has been set. An interrupt will be generated and the FIFO flushed at this time. The bit is cleared by the CPU writing a 1 to the ServicedSetupEnd bit."]
pub type SendStallSetupEndR = crate::BitReader;
#[doc = "Field `SendStallSetupEnd` writer - When CFG3_ENDPOINT = 1 to 5, this bit serves as the SendStall field. When CFG3_ENDPOINT = 0, this bit serves as the SetupEnd field.If CFG3_ENDPOINT = 0x1-0x5, this bit serves as the SendStall field. Setting this bit issues a STALL handshake to an IN token. The CPU clears this bit to terminate the stall condition.Note: This bit has no effect when the endpoint is being used for Isochronous transfers.If CFG3_ENDPOINT = 0x0, this bit serves as the SetupEnd field. It is set when a control transaction ends before the DataEnd bit has been set. An interrupt will be generated and the FIFO flushed at this time. The bit is cleared by the CPU writing a 1 to the ServicedSetupEnd bit."]
pub type SendStallSetupEndW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SentStallSendStall` reader - Sent Stall / Send Stall. When CFG3_ENDPOINT = 1 to 5, this bit serves as the SentStall field. When CFG3_ENDPOINT = 0, this bit serves as the SendStall function.If CFG3_ENDPOINT = 0x1-0x5, this bit serves as the SentStall field. It is set when a STALL handshake is transmitted. The FIFO is flushed and the InPktRdy bit is cleared. The CPU should clear this bit.If CFG3_ENDPOINT = 0x0, this bit serves as the SendStall field. The CPU sets this bit to terminate the current transaction. The STALL handshake will be transmitted and then this bit will be cleared automatically. Note: This behavior differs from that of the SendStall bits associated with additional IN/OUT endpoints, which need to be cleared by the CPU."]
pub type SentStallSendStallR = crate::BitReader;
#[doc = "Field `SentStallSendStall` writer - Sent Stall / Send Stall. When CFG3_ENDPOINT = 1 to 5, this bit serves as the SentStall field. When CFG3_ENDPOINT = 0, this bit serves as the SendStall function.If CFG3_ENDPOINT = 0x1-0x5, this bit serves as the SentStall field. It is set when a STALL handshake is transmitted. The FIFO is flushed and the InPktRdy bit is cleared. The CPU should clear this bit.If CFG3_ENDPOINT = 0x0, this bit serves as the SendStall field. The CPU sets this bit to terminate the current transaction. The STALL handshake will be transmitted and then this bit will be cleared automatically. Note: This behavior differs from that of the SendStall bits associated with additional IN/OUT endpoints, which need to be cleared by the CPU."]
pub type SentStallSendStallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ClrDataTogServicedOutPktRdy` reader - Clear Data Toggle / Serviced OUT Packet Ready. When CFG3_ENDPOINT = 1 to 5, this bit serves as the ClrDataTog field. When CFG3_ENDPOINT = 0, this bit serves as the ServicedOutPktReady field.If CFG3_ENDPOINT = 0x1-0x5, this bit serves as the ClrDataTog field. Setting this bit resets the endpoint IN data toggle to 0.If CFG3_ENDPOINT = 0x0, this bit serves as the ServicedOutPktReady field. The CPU writes a 1 to this bit to clear the OutPktRdy bit. This bit is cleared automatically."]
pub type ClrDataTogServicedOutPktRdyR = crate::BitReader;
#[doc = "Field `ClrDataTogServicedOutPktRdy` writer - Clear Data Toggle / Serviced OUT Packet Ready. When CFG3_ENDPOINT = 1 to 5, this bit serves as the ClrDataTog field. When CFG3_ENDPOINT = 0, this bit serves as the ServicedOutPktReady field.If CFG3_ENDPOINT = 0x1-0x5, this bit serves as the ClrDataTog field. Setting this bit resets the endpoint IN data toggle to 0.If CFG3_ENDPOINT = 0x0, this bit serves as the ServicedOutPktReady field. The CPU writes a 1 to this bit to clear the OutPktRdy bit. This bit is cleared automatically."]
pub type ClrDataTogServicedOutPktRdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Incomplete Transmission / Service Setup End. When CFG3_ENDPOINT = 1 to 5, this bit serves as the IncompTx field. When CFG3_ENDPOINT = 0, this bit serves as the ServiceSetupEnd field.If CFG3_ENDPOINT = 0x1-0x5, then this bit serves as the IncompTx field. If the endpoint is being used for high-bandwidth Isochronous transfers, this bit is set to indicate when a large packet has been split into 2 or 3 packets for transmission but insufficient IN tokens have been received to send all the parts. The remainder of the current packet is then flushed from the FIFO (but any second packet in the FIFO will remain). Note: In anything other than a high bandwidth Isochronous transfer, this bit will always return 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IncompTxServiceSetupEnd {
    #[doc = "0: Packet has NOT been split into multiple packets for transmission."]
    NoPacketSplit = 0,
    #[doc = "1: A large packet has been split into 2 or 3 packets for transmission but insufficient IN tokens have been received to send all the parts.If CFG3_ENDPOINT = 0x0, this bit serves as the ServiceSetupEnd field."]
    PacketSplit = 1,
}
impl From<IncompTxServiceSetupEnd> for bool {
    #[inline(always)]
    fn from(variant: IncompTxServiceSetupEnd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IncompTxServiceSetupEnd` reader - Incomplete Transmission / Service Setup End. When CFG3_ENDPOINT = 1 to 5, this bit serves as the IncompTx field. When CFG3_ENDPOINT = 0, this bit serves as the ServiceSetupEnd field.If CFG3_ENDPOINT = 0x1-0x5, then this bit serves as the IncompTx field. If the endpoint is being used for high-bandwidth Isochronous transfers, this bit is set to indicate when a large packet has been split into 2 or 3 packets for transmission but insufficient IN tokens have been received to send all the parts. The remainder of the current packet is then flushed from the FIFO (but any second packet in the FIFO will remain). Note: In anything other than a high bandwidth Isochronous transfer, this bit will always return 0."]
pub type IncompTxServiceSetupEndR = crate::BitReader<IncompTxServiceSetupEnd>;
impl IncompTxServiceSetupEndR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IncompTxServiceSetupEnd {
        match self.bits {
            false => IncompTxServiceSetupEnd::NoPacketSplit,
            true => IncompTxServiceSetupEnd::PacketSplit,
        }
    }
    #[doc = "Packet has NOT been split into multiple packets for transmission."]
    #[inline(always)]
    pub fn is_no_packet_split(&self) -> bool {
        *self == IncompTxServiceSetupEnd::NoPacketSplit
    }
    #[doc = "A large packet has been split into 2 or 3 packets for transmission but insufficient IN tokens have been received to send all the parts.If CFG3_ENDPOINT = 0x0, this bit serves as the ServiceSetupEnd field."]
    #[inline(always)]
    pub fn is_packet_split(&self) -> bool {
        *self == IncompTxServiceSetupEnd::PacketSplit
    }
}
#[doc = "Field `IncompTxServiceSetupEnd` writer - Incomplete Transmission / Service Setup End. When CFG3_ENDPOINT = 1 to 5, this bit serves as the IncompTx field. When CFG3_ENDPOINT = 0, this bit serves as the ServiceSetupEnd field.If CFG3_ENDPOINT = 0x1-0x5, then this bit serves as the IncompTx field. If the endpoint is being used for high-bandwidth Isochronous transfers, this bit is set to indicate when a large packet has been split into 2 or 3 packets for transmission but insufficient IN tokens have been received to send all the parts. The remainder of the current packet is then flushed from the FIFO (but any second packet in the FIFO will remain). Note: In anything other than a high bandwidth Isochronous transfer, this bit will always return 0."]
pub type IncompTxServiceSetupEndW<'a, REG> = crate::BitWriter<'a, REG, IncompTxServiceSetupEnd>;
impl<'a, REG> IncompTxServiceSetupEndW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Packet has NOT been split into multiple packets for transmission."]
    #[inline(always)]
    pub fn no_packet_split(self) -> &'a mut crate::W<REG> {
        self.variant(IncompTxServiceSetupEnd::NoPacketSplit)
    }
    #[doc = "A large packet has been split into 2 or 3 packets for transmission but insufficient IN tokens have been received to send all the parts.If CFG3_ENDPOINT = 0x0, this bit serves as the ServiceSetupEnd field."]
    #[inline(always)]
    pub fn packet_split(self) -> &'a mut crate::W<REG> {
        self.variant(IncompTxServiceSetupEnd::PacketSplit)
    }
}
#[doc = "Field `D0` reader - Unused, always return 0."]
pub type D0R = crate::BitReader;
#[doc = "Field `D0` writer - Unused, always return 0."]
pub type D0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Double Packet Buffer Disable. This bit is used to control the use of Double Packet Buffering. It is ignored when Dynamic FIFO sizing is enabled. Clearing this bit does NOT necessarily enable Double Packet Buffering but rather allows Double Packet Buffering to be determined by the Endpoint's IDX2_INFIFOSZ setting and MAXPAYLOAD size relationship. Default is enabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DpktBufDis {
    #[doc = "0: Clear to allow Double Packet Buffering."]
    EnDpb = 0,
    #[doc = "1: Set to disable Double Packet Buffering regardless of the End Point FIFO size and MAXPAYLOAD size relationship."]
    DisDpb = 1,
}
impl From<DpktBufDis> for bool {
    #[inline(always)]
    fn from(variant: DpktBufDis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DPktBufDis` reader - Double Packet Buffer Disable. This bit is used to control the use of Double Packet Buffering. It is ignored when Dynamic FIFO sizing is enabled. Clearing this bit does NOT necessarily enable Double Packet Buffering but rather allows Double Packet Buffering to be determined by the Endpoint's IDX2_INFIFOSZ setting and MAXPAYLOAD size relationship. Default is enabled."]
pub type DpktBufDisR = crate::BitReader<DpktBufDis>;
impl DpktBufDisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DpktBufDis {
        match self.bits {
            false => DpktBufDis::EnDpb,
            true => DpktBufDis::DisDpb,
        }
    }
    #[doc = "Clear to allow Double Packet Buffering."]
    #[inline(always)]
    pub fn is_en_dpb(&self) -> bool {
        *self == DpktBufDis::EnDpb
    }
    #[doc = "Set to disable Double Packet Buffering regardless of the End Point FIFO size and MAXPAYLOAD size relationship."]
    #[inline(always)]
    pub fn is_dis_dpb(&self) -> bool {
        *self == DpktBufDis::DisDpb
    }
}
#[doc = "Field `DPktBufDis` writer - Double Packet Buffer Disable. This bit is used to control the use of Double Packet Buffering. It is ignored when Dynamic FIFO sizing is enabled. Clearing this bit does NOT necessarily enable Double Packet Buffering but rather allows Double Packet Buffering to be determined by the Endpoint's IDX2_INFIFOSZ setting and MAXPAYLOAD size relationship. Default is enabled."]
pub type DpktBufDisW<'a, REG> = crate::BitWriter<'a, REG, DpktBufDis>;
impl<'a, REG> DpktBufDisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear to allow Double Packet Buffering."]
    #[inline(always)]
    pub fn en_dpb(self) -> &'a mut crate::W<REG> {
        self.variant(DpktBufDis::EnDpb)
    }
    #[doc = "Set to disable Double Packet Buffering regardless of the End Point FIFO size and MAXPAYLOAD size relationship."]
    #[inline(always)]
    pub fn dis_dpb(self) -> &'a mut crate::W<REG> {
        self.variant(DpktBufDis::DisDpb)
    }
}
#[doc = "Force Data Toggle. The CPU sets this bit to force the endpoint's IN data toggle to switch after each data packet is sent regardless of whether an ACK was received. This can be used by Interrupt IN endpoints that are used to communicate rate feedback for Isochronous endpoints.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FrcDataTog {
    #[doc = "0: Keep cleared to not force data toggle."]
    NoForceToggle = 0,
    #[doc = "1: Set to force the endpoint's IN data toggle to switch after each data packet is sent."]
    ForceToggle = 1,
}
impl From<FrcDataTog> for bool {
    #[inline(always)]
    fn from(variant: FrcDataTog) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FrcDataTog` reader - Force Data Toggle. The CPU sets this bit to force the endpoint's IN data toggle to switch after each data packet is sent regardless of whether an ACK was received. This can be used by Interrupt IN endpoints that are used to communicate rate feedback for Isochronous endpoints."]
pub type FrcDataTogR = crate::BitReader<FrcDataTog>;
impl FrcDataTogR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FrcDataTog {
        match self.bits {
            false => FrcDataTog::NoForceToggle,
            true => FrcDataTog::ForceToggle,
        }
    }
    #[doc = "Keep cleared to not force data toggle."]
    #[inline(always)]
    pub fn is_no_force_toggle(&self) -> bool {
        *self == FrcDataTog::NoForceToggle
    }
    #[doc = "Set to force the endpoint's IN data toggle to switch after each data packet is sent."]
    #[inline(always)]
    pub fn is_force_toggle(&self) -> bool {
        *self == FrcDataTog::ForceToggle
    }
}
#[doc = "Field `FrcDataTog` writer - Force Data Toggle. The CPU sets this bit to force the endpoint's IN data toggle to switch after each data packet is sent regardless of whether an ACK was received. This can be used by Interrupt IN endpoints that are used to communicate rate feedback for Isochronous endpoints."]
pub type FrcDataTogW<'a, REG> = crate::BitWriter<'a, REG, FrcDataTog>;
impl<'a, REG> FrcDataTogW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Keep cleared to not force data toggle."]
    #[inline(always)]
    pub fn no_force_toggle(self) -> &'a mut crate::W<REG> {
        self.variant(FrcDataTog::NoForceToggle)
    }
    #[doc = "Set to force the endpoint's IN data toggle to switch after each data packet is sent."]
    #[inline(always)]
    pub fn force_toggle(self) -> &'a mut crate::W<REG> {
        self.variant(FrcDataTog::ForceToggle)
    }
}
#[doc = "OUT/IN Mode. The CPU sets this bit to enable the endpoint direction as IN or OUT. Note: Only valid where the endpoint FIFO is used for both IN and OUT transactions, otherwise ignored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    #[doc = "0: Clear to enable the OUT direction for endpoint."]
    Out = 0,
    #[doc = "1: Set to enable the IN direction for endpoint."]
    In = 1,
}
impl From<Mode> for bool {
    #[inline(always)]
    fn from(variant: Mode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Mode` reader - OUT/IN Mode. The CPU sets this bit to enable the endpoint direction as IN or OUT. Note: Only valid where the endpoint FIFO is used for both IN and OUT transactions, otherwise ignored."]
pub type ModeR = crate::BitReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode {
        match self.bits {
            false => Mode::Out,
            true => Mode::In,
        }
    }
    #[doc = "Clear to enable the OUT direction for endpoint."]
    #[inline(always)]
    pub fn is_out(&self) -> bool {
        *self == Mode::Out
    }
    #[doc = "Set to enable the IN direction for endpoint."]
    #[inline(always)]
    pub fn is_in(&self) -> bool {
        *self == Mode::In
    }
}
#[doc = "Field `Mode` writer - OUT/IN Mode. The CPU sets this bit to enable the endpoint direction as IN or OUT. Note: Only valid where the endpoint FIFO is used for both IN and OUT transactions, otherwise ignored."]
pub type ModeW<'a, REG> = crate::BitWriter<'a, REG, Mode>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear to enable the OUT direction for endpoint."]
    #[inline(always)]
    pub fn out(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Out)
    }
    #[doc = "Set to enable the IN direction for endpoint."]
    #[inline(always)]
    pub fn in_(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::In)
    }
}
#[doc = "Isochronous Transfers. The CPU sets this bit to enable the IN endpoint for Isochronous transfers (ISO mode) or for Bulk/Interrupt transfers.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iso {
    #[doc = "0: Clear to enable the IN endpoint for Bulk/Interrupt transfers."]
    BulkInt = 0,
    #[doc = "1: Set to enable the IN endpoint for Isochronous transfers."]
    Iso = 1,
}
impl From<Iso> for bool {
    #[inline(always)]
    fn from(variant: Iso) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISO` reader - Isochronous Transfers. The CPU sets this bit to enable the IN endpoint for Isochronous transfers (ISO mode) or for Bulk/Interrupt transfers."]
pub type IsoR = crate::BitReader<Iso>;
impl IsoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Iso {
        match self.bits {
            false => Iso::BulkInt,
            true => Iso::Iso,
        }
    }
    #[doc = "Clear to enable the IN endpoint for Bulk/Interrupt transfers."]
    #[inline(always)]
    pub fn is_bulk_int(&self) -> bool {
        *self == Iso::BulkInt
    }
    #[doc = "Set to enable the IN endpoint for Isochronous transfers."]
    #[inline(always)]
    pub fn is_iso(&self) -> bool {
        *self == Iso::Iso
    }
}
#[doc = "Field `ISO` writer - Isochronous Transfers. The CPU sets this bit to enable the IN endpoint for Isochronous transfers (ISO mode) or for Bulk/Interrupt transfers."]
pub type IsoW<'a, REG> = crate::BitWriter<'a, REG, Iso>;
impl<'a, REG> IsoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear to enable the IN endpoint for Bulk/Interrupt transfers."]
    #[inline(always)]
    pub fn bulk_int(self) -> &'a mut crate::W<REG> {
        self.variant(Iso::BulkInt)
    }
    #[doc = "Set to enable the IN endpoint for Isochronous transfers."]
    #[inline(always)]
    pub fn iso(self) -> &'a mut crate::W<REG> {
        self.variant(Iso::Iso)
    }
}
#[doc = "Automatically Set InPktRdy. When set, the FIFONotEmptyInPktRdy field (for IN Endpoint 0) or InPktRdyOutPktRdy field (for IN Endpoint 1-5) in this register will be automatically set when data of the maximum packet size (set in MAXPAYLOAD field) is loaded into the IN FIFO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AutoSet {
    #[doc = "0: InPktRdy field is not automatically set when MAXPAYLOAD data size is loaded into the IN FIFO."]
    NoAutoset = 0,
    #[doc = "1: Applicable InPktRdy field is automatically set when MAXPAYLOAD data size is loaded into the IN FIFO. If a packet of less than the maximum packet size is loaded, InPktRdy will have to be set manually. Note: Should not be set for high-bandwidth Isochronous endpoints."]
    Autoset = 1,
}
impl From<AutoSet> for bool {
    #[inline(always)]
    fn from(variant: AutoSet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AutoSet` reader - Automatically Set InPktRdy. When set, the FIFONotEmptyInPktRdy field (for IN Endpoint 0) or InPktRdyOutPktRdy field (for IN Endpoint 1-5) in this register will be automatically set when data of the maximum packet size (set in MAXPAYLOAD field) is loaded into the IN FIFO."]
pub type AutoSetR = crate::BitReader<AutoSet>;
impl AutoSetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AutoSet {
        match self.bits {
            false => AutoSet::NoAutoset,
            true => AutoSet::Autoset,
        }
    }
    #[doc = "InPktRdy field is not automatically set when MAXPAYLOAD data size is loaded into the IN FIFO."]
    #[inline(always)]
    pub fn is_no_autoset(&self) -> bool {
        *self == AutoSet::NoAutoset
    }
    #[doc = "Applicable InPktRdy field is automatically set when MAXPAYLOAD data size is loaded into the IN FIFO. If a packet of less than the maximum packet size is loaded, InPktRdy will have to be set manually. Note: Should not be set for high-bandwidth Isochronous endpoints."]
    #[inline(always)]
    pub fn is_autoset(&self) -> bool {
        *self == AutoSet::Autoset
    }
}
#[doc = "Field `AutoSet` writer - Automatically Set InPktRdy. When set, the FIFONotEmptyInPktRdy field (for IN Endpoint 0) or InPktRdyOutPktRdy field (for IN Endpoint 1-5) in this register will be automatically set when data of the maximum packet size (set in MAXPAYLOAD field) is loaded into the IN FIFO."]
pub type AutoSetW<'a, REG> = crate::BitWriter<'a, REG, AutoSet>;
impl<'a, REG> AutoSetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "InPktRdy field is not automatically set when MAXPAYLOAD data size is loaded into the IN FIFO."]
    #[inline(always)]
    pub fn no_autoset(self) -> &'a mut crate::W<REG> {
        self.variant(AutoSet::NoAutoset)
    }
    #[doc = "Applicable InPktRdy field is automatically set when MAXPAYLOAD data size is loaded into the IN FIFO. If a packet of less than the maximum packet size is loaded, InPktRdy will have to be set manually. Note: Should not be set for high-bandwidth Isochronous endpoints."]
    #[inline(always)]
    pub fn autoset(self) -> &'a mut crate::W<REG> {
        self.variant(AutoSet::Autoset)
    }
}
impl R {
    #[doc = "Bits 0:10 - Maximum Payload transmitted in a single transaction. The total amount of data represented by MAXPAYLOAD x (PKTSPLITOPTION + 1) must not exceed the FIFO size for the IN endpoint, and should not exceed half the FIFO size if double-buffering is required. Note: The value written here (multiplied by PKTSPLITOPTION + 1 in the case of high-bandwidth Isochronous transfers) must match the value given in the wMaxPacketSize field of the Standard Endpoint Descriptor for the associated endpoint (see USB Specification Revision 2.0, Chapter 9). A mismatch could cause unexpected results."]
    #[inline(always)]
    pub fn maxpayload(&self) -> MaxpayloadR {
        MaxpayloadR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:15 - Packet Split Option. When IDX0_ISO = 1, this bit serves as the MAXPAYLOAD multiplier for Isochronous IN transfers. When IDX0_ISO = 0, this bit serves as the MAXPAYLOAD multiplier for Bulk IN transfers.If IDX0_ISO = 0x1, this field sets the multiplier for Isochronous transfers. For Isochronous endpoints operating in High-Speed mode and with the High-bandwidth option enabled, PKTSPLITOPTION may be either 2 or 3 (corresponding to this field's bit 0 set or bit 1 set, respectively, and bits\\[4:2\\]
are ignored) and it specifies the maximum number of such transactions that can take place in a single microframe. If either bit 0 or bit 1 is non-zero, the USB Controller will automatically split any data packet written to the FIFO into up to 2 or 3 'USB' packets, each containing the specified MAXPAYLOAD (or less). The maximum payload for each transaction is 1024 bytes, so this allows up to 3072 bytes to be transmitted in each microframe. (For Isochronous transfers in full-speed mode or if High-bandwidth is not enabled, bits 0 and 1 are ignored.)If IDX0_ISO = 0x0, this field sets the multiplier for Bulk transfers. The field setting can be up to 31, and the multiplier, m, is (field setting + 1). The multiplier defines the maximum number of 'USB' packets of the specified MAXPAYLOAD data bytes into which a single data packet placed in the FIFO should be split prior to transfer. The data packet is required to be an exact multiple of the MAXPAYLOAD, which is itself required to be either 8, 16, 32, 64 or (in the case of High Speed transfers) 512 bytes."]
    #[inline(always)]
    pub fn pktsplitoption(&self) -> PktsplitoptionR {
        PktsplitoptionR::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bit 16 - IN Packet Ready / OUT Packet Ready. When CFG3_ENDPOINT > 0, this bit serves as the InPktRdy field. When CFG3_ENDPOINT = 0, this bit serves as the OutPkyRdy bit.If CFG3_ENDPOINT = 0x1-0x5, this bit serves as the InPktRdy field. Set this bit after loading a data packet into the FIFO. It is cleared automatically when a data packet has been transmitted. If the FIFO is double-buffered, it is also automatically cleared when there is space for a second packet in the FIFO. An interrupt is generate (if enabled) when the bit is cleared.If CFG3_ENDPOINT = 0x0, this bit serves as the OutPkyRdy bit. This bit is set when a data packet has been received. An interrupt is generate when this bit is set (if enabled). The CPU clears this bit by setting the ServicedOutPktRdy bit."]
    #[inline(always)]
    pub fn in_pkt_rdy_out_pkt_rdy(&self) -> InPktRdyOutPktRdyR {
        InPktRdyOutPktRdyR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - FIFO Not Empty / IN Packet Ready. When CFG3_ENDPOINT = 1 to 5, this bit serves as the FIFONotEmpty field. When CFG3_ENDPOINT = 0, this bit serves as the InPktRdy bit.If CFG3_ENDPOINT = 0x1-0x5, this bit serves as the FIFONotEmpty field. It is set when there is at least 1 packet in the IN FIFO.If CFG3_ENDPOINT = 0x0, this bit serves as the InPktRdy bit. Set this bit after loading a data packet into the FIFO. It is cleared automatically when the data packet has been transmitted. An interrupt is generated when this bit is cleared (if enabled)."]
    #[inline(always)]
    pub fn fifonot_empty_in_pkt_rdy(&self) -> FifonotEmptyInPktRdyR {
        FifonotEmptyInPktRdyR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Under Run / Sent Stall. When CFG3_ENDPOINT = 1 to 5, this bit serves as the UnderRun field. When CFG3_ENDPOINT = 0, this bit serves as the SentStall field.If CFG3_ENDPOINT = 0x1-0x5, this bit serves as the UnderRun field. In ISO mode this bit is set when a zero length data packet is sent after receiving an IN token with the InPktRdy bit not set. In Bulk/Interrupt mode, this bit is set when a NAK is returned in response to an IN token. The CPU should clear this bit.If CFG3_ENDPOINT = 0x0, this bit serves as the SentStall field. This bit is set when a STALL handshake is transmitted. The CPU should clear this bit."]
    #[inline(always)]
    pub fn under_run_sent_stall(&self) -> UnderRunSentStallR {
        UnderRunSentStallR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - When CFG3_ENDPOINT = 1 to 5, this bit serves as the FlushFIFO field. When CFG3_ENDPOINT = 0, this bit serves as the DataEnd bit.If CFG3_ENDPOINT = 0x1-0x5, this bit serves as the FlushFIFO field. Setting this bit flushes the next packet to be transmitted from the endpoint IN FIFO. The FIFO pointer is reset and the InPktRdy bit is cleared. May be set simultaneously with InPktRdy to abort the packet that has just been loaded into the FIFO.Note 1: FlushFIFO should only be set when InPktRdy is set (at other times, it may cause data corruption).Note 2: If the FIFO contains two packets, FlushFIFO may need to be set twice to completely clear the FIFO.If CFG3_ENDPOINT = 0x0, this bit serves as the DataEnd bit. Set this bit when (1) setting InPktRdy for the last data packet, (2) clearing OutPktRdy after unloading the last data packet, or (3) setting InPktRdy for a zero length data packet.It is cleared automatically."]
    #[inline(always)]
    pub fn flush_fifodata_end(&self) -> FlushFifodataEndR {
        FlushFifodataEndR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - When CFG3_ENDPOINT = 1 to 5, this bit serves as the SendStall field. When CFG3_ENDPOINT = 0, this bit serves as the SetupEnd field.If CFG3_ENDPOINT = 0x1-0x5, this bit serves as the SendStall field. Setting this bit issues a STALL handshake to an IN token. The CPU clears this bit to terminate the stall condition.Note: This bit has no effect when the endpoint is being used for Isochronous transfers.If CFG3_ENDPOINT = 0x0, this bit serves as the SetupEnd field. It is set when a control transaction ends before the DataEnd bit has been set. An interrupt will be generated and the FIFO flushed at this time. The bit is cleared by the CPU writing a 1 to the ServicedSetupEnd bit."]
    #[inline(always)]
    pub fn send_stall_setup_end(&self) -> SendStallSetupEndR {
        SendStallSetupEndR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Sent Stall / Send Stall. When CFG3_ENDPOINT = 1 to 5, this bit serves as the SentStall field. When CFG3_ENDPOINT = 0, this bit serves as the SendStall function.If CFG3_ENDPOINT = 0x1-0x5, this bit serves as the SentStall field. It is set when a STALL handshake is transmitted. The FIFO is flushed and the InPktRdy bit is cleared. The CPU should clear this bit.If CFG3_ENDPOINT = 0x0, this bit serves as the SendStall field. The CPU sets this bit to terminate the current transaction. The STALL handshake will be transmitted and then this bit will be cleared automatically. Note: This behavior differs from that of the SendStall bits associated with additional IN/OUT endpoints, which need to be cleared by the CPU."]
    #[inline(always)]
    pub fn sent_stall_send_stall(&self) -> SentStallSendStallR {
        SentStallSendStallR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Clear Data Toggle / Serviced OUT Packet Ready. When CFG3_ENDPOINT = 1 to 5, this bit serves as the ClrDataTog field. When CFG3_ENDPOINT = 0, this bit serves as the ServicedOutPktReady field.If CFG3_ENDPOINT = 0x1-0x5, this bit serves as the ClrDataTog field. Setting this bit resets the endpoint IN data toggle to 0.If CFG3_ENDPOINT = 0x0, this bit serves as the ServicedOutPktReady field. The CPU writes a 1 to this bit to clear the OutPktRdy bit. This bit is cleared automatically."]
    #[inline(always)]
    pub fn clr_data_tog_serviced_out_pkt_rdy(&self) -> ClrDataTogServicedOutPktRdyR {
        ClrDataTogServicedOutPktRdyR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Incomplete Transmission / Service Setup End. When CFG3_ENDPOINT = 1 to 5, this bit serves as the IncompTx field. When CFG3_ENDPOINT = 0, this bit serves as the ServiceSetupEnd field.If CFG3_ENDPOINT = 0x1-0x5, then this bit serves as the IncompTx field. If the endpoint is being used for high-bandwidth Isochronous transfers, this bit is set to indicate when a large packet has been split into 2 or 3 packets for transmission but insufficient IN tokens have been received to send all the parts. The remainder of the current packet is then flushed from the FIFO (but any second packet in the FIFO will remain). Note: In anything other than a high bandwidth Isochronous transfer, this bit will always return 0."]
    #[inline(always)]
    pub fn incomp_tx_service_setup_end(&self) -> IncompTxServiceSetupEndR {
        IncompTxServiceSetupEndR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Unused, always return 0."]
    #[inline(always)]
    pub fn d0(&self) -> D0R {
        D0R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Double Packet Buffer Disable. This bit is used to control the use of Double Packet Buffering. It is ignored when Dynamic FIFO sizing is enabled. Clearing this bit does NOT necessarily enable Double Packet Buffering but rather allows Double Packet Buffering to be determined by the Endpoint's IDX2_INFIFOSZ setting and MAXPAYLOAD size relationship. Default is enabled."]
    #[inline(always)]
    pub fn dpkt_buf_dis(&self) -> DpktBufDisR {
        DpktBufDisR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - Force Data Toggle. The CPU sets this bit to force the endpoint's IN data toggle to switch after each data packet is sent regardless of whether an ACK was received. This can be used by Interrupt IN endpoints that are used to communicate rate feedback for Isochronous endpoints."]
    #[inline(always)]
    pub fn frc_data_tog(&self) -> FrcDataTogR {
        FrcDataTogR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 29 - OUT/IN Mode. The CPU sets this bit to enable the endpoint direction as IN or OUT. Note: Only valid where the endpoint FIFO is used for both IN and OUT transactions, otherwise ignored."]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Isochronous Transfers. The CPU sets this bit to enable the IN endpoint for Isochronous transfers (ISO mode) or for Bulk/Interrupt transfers."]
    #[inline(always)]
    pub fn iso(&self) -> IsoR {
        IsoR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Automatically Set InPktRdy. When set, the FIFONotEmptyInPktRdy field (for IN Endpoint 0) or InPktRdyOutPktRdy field (for IN Endpoint 1-5) in this register will be automatically set when data of the maximum packet size (set in MAXPAYLOAD field) is loaded into the IN FIFO."]
    #[inline(always)]
    pub fn auto_set(&self) -> AutoSetR {
        AutoSetR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10 - Maximum Payload transmitted in a single transaction. The total amount of data represented by MAXPAYLOAD x (PKTSPLITOPTION + 1) must not exceed the FIFO size for the IN endpoint, and should not exceed half the FIFO size if double-buffering is required. Note: The value written here (multiplied by PKTSPLITOPTION + 1 in the case of high-bandwidth Isochronous transfers) must match the value given in the wMaxPacketSize field of the Standard Endpoint Descriptor for the associated endpoint (see USB Specification Revision 2.0, Chapter 9). A mismatch could cause unexpected results."]
    #[inline(always)]
    #[must_use]
    pub fn maxpayload(&mut self) -> MaxpayloadW<Idx0Spec> {
        MaxpayloadW::new(self, 0)
    }
    #[doc = "Bits 11:15 - Packet Split Option. When IDX0_ISO = 1, this bit serves as the MAXPAYLOAD multiplier for Isochronous IN transfers. When IDX0_ISO = 0, this bit serves as the MAXPAYLOAD multiplier for Bulk IN transfers.If IDX0_ISO = 0x1, this field sets the multiplier for Isochronous transfers. For Isochronous endpoints operating in High-Speed mode and with the High-bandwidth option enabled, PKTSPLITOPTION may be either 2 or 3 (corresponding to this field's bit 0 set or bit 1 set, respectively, and bits\\[4:2\\]
are ignored) and it specifies the maximum number of such transactions that can take place in a single microframe. If either bit 0 or bit 1 is non-zero, the USB Controller will automatically split any data packet written to the FIFO into up to 2 or 3 'USB' packets, each containing the specified MAXPAYLOAD (or less). The maximum payload for each transaction is 1024 bytes, so this allows up to 3072 bytes to be transmitted in each microframe. (For Isochronous transfers in full-speed mode or if High-bandwidth is not enabled, bits 0 and 1 are ignored.)If IDX0_ISO = 0x0, this field sets the multiplier for Bulk transfers. The field setting can be up to 31, and the multiplier, m, is (field setting + 1). The multiplier defines the maximum number of 'USB' packets of the specified MAXPAYLOAD data bytes into which a single data packet placed in the FIFO should be split prior to transfer. The data packet is required to be an exact multiple of the MAXPAYLOAD, which is itself required to be either 8, 16, 32, 64 or (in the case of High Speed transfers) 512 bytes."]
    #[inline(always)]
    #[must_use]
    pub fn pktsplitoption(&mut self) -> PktsplitoptionW<Idx0Spec> {
        PktsplitoptionW::new(self, 11)
    }
    #[doc = "Bit 16 - IN Packet Ready / OUT Packet Ready. When CFG3_ENDPOINT > 0, this bit serves as the InPktRdy field. When CFG3_ENDPOINT = 0, this bit serves as the OutPkyRdy bit.If CFG3_ENDPOINT = 0x1-0x5, this bit serves as the InPktRdy field. Set this bit after loading a data packet into the FIFO. It is cleared automatically when a data packet has been transmitted. If the FIFO is double-buffered, it is also automatically cleared when there is space for a second packet in the FIFO. An interrupt is generate (if enabled) when the bit is cleared.If CFG3_ENDPOINT = 0x0, this bit serves as the OutPkyRdy bit. This bit is set when a data packet has been received. An interrupt is generate when this bit is set (if enabled). The CPU clears this bit by setting the ServicedOutPktRdy bit."]
    #[inline(always)]
    #[must_use]
    pub fn in_pkt_rdy_out_pkt_rdy(&mut self) -> InPktRdyOutPktRdyW<Idx0Spec> {
        InPktRdyOutPktRdyW::new(self, 16)
    }
    #[doc = "Bit 17 - FIFO Not Empty / IN Packet Ready. When CFG3_ENDPOINT = 1 to 5, this bit serves as the FIFONotEmpty field. When CFG3_ENDPOINT = 0, this bit serves as the InPktRdy bit.If CFG3_ENDPOINT = 0x1-0x5, this bit serves as the FIFONotEmpty field. It is set when there is at least 1 packet in the IN FIFO.If CFG3_ENDPOINT = 0x0, this bit serves as the InPktRdy bit. Set this bit after loading a data packet into the FIFO. It is cleared automatically when the data packet has been transmitted. An interrupt is generated when this bit is cleared (if enabled)."]
    #[inline(always)]
    #[must_use]
    pub fn fifonot_empty_in_pkt_rdy(&mut self) -> FifonotEmptyInPktRdyW<Idx0Spec> {
        FifonotEmptyInPktRdyW::new(self, 17)
    }
    #[doc = "Bit 18 - Under Run / Sent Stall. When CFG3_ENDPOINT = 1 to 5, this bit serves as the UnderRun field. When CFG3_ENDPOINT = 0, this bit serves as the SentStall field.If CFG3_ENDPOINT = 0x1-0x5, this bit serves as the UnderRun field. In ISO mode this bit is set when a zero length data packet is sent after receiving an IN token with the InPktRdy bit not set. In Bulk/Interrupt mode, this bit is set when a NAK is returned in response to an IN token. The CPU should clear this bit.If CFG3_ENDPOINT = 0x0, this bit serves as the SentStall field. This bit is set when a STALL handshake is transmitted. The CPU should clear this bit."]
    #[inline(always)]
    #[must_use]
    pub fn under_run_sent_stall(&mut self) -> UnderRunSentStallW<Idx0Spec> {
        UnderRunSentStallW::new(self, 18)
    }
    #[doc = "Bit 19 - When CFG3_ENDPOINT = 1 to 5, this bit serves as the FlushFIFO field. When CFG3_ENDPOINT = 0, this bit serves as the DataEnd bit.If CFG3_ENDPOINT = 0x1-0x5, this bit serves as the FlushFIFO field. Setting this bit flushes the next packet to be transmitted from the endpoint IN FIFO. The FIFO pointer is reset and the InPktRdy bit is cleared. May be set simultaneously with InPktRdy to abort the packet that has just been loaded into the FIFO.Note 1: FlushFIFO should only be set when InPktRdy is set (at other times, it may cause data corruption).Note 2: If the FIFO contains two packets, FlushFIFO may need to be set twice to completely clear the FIFO.If CFG3_ENDPOINT = 0x0, this bit serves as the DataEnd bit. Set this bit when (1) setting InPktRdy for the last data packet, (2) clearing OutPktRdy after unloading the last data packet, or (3) setting InPktRdy for a zero length data packet.It is cleared automatically."]
    #[inline(always)]
    #[must_use]
    pub fn flush_fifodata_end(&mut self) -> FlushFifodataEndW<Idx0Spec> {
        FlushFifodataEndW::new(self, 19)
    }
    #[doc = "Bit 20 - When CFG3_ENDPOINT = 1 to 5, this bit serves as the SendStall field. When CFG3_ENDPOINT = 0, this bit serves as the SetupEnd field.If CFG3_ENDPOINT = 0x1-0x5, this bit serves as the SendStall field. Setting this bit issues a STALL handshake to an IN token. The CPU clears this bit to terminate the stall condition.Note: This bit has no effect when the endpoint is being used for Isochronous transfers.If CFG3_ENDPOINT = 0x0, this bit serves as the SetupEnd field. It is set when a control transaction ends before the DataEnd bit has been set. An interrupt will be generated and the FIFO flushed at this time. The bit is cleared by the CPU writing a 1 to the ServicedSetupEnd bit."]
    #[inline(always)]
    #[must_use]
    pub fn send_stall_setup_end(&mut self) -> SendStallSetupEndW<Idx0Spec> {
        SendStallSetupEndW::new(self, 20)
    }
    #[doc = "Bit 21 - Sent Stall / Send Stall. When CFG3_ENDPOINT = 1 to 5, this bit serves as the SentStall field. When CFG3_ENDPOINT = 0, this bit serves as the SendStall function.If CFG3_ENDPOINT = 0x1-0x5, this bit serves as the SentStall field. It is set when a STALL handshake is transmitted. The FIFO is flushed and the InPktRdy bit is cleared. The CPU should clear this bit.If CFG3_ENDPOINT = 0x0, this bit serves as the SendStall field. The CPU sets this bit to terminate the current transaction. The STALL handshake will be transmitted and then this bit will be cleared automatically. Note: This behavior differs from that of the SendStall bits associated with additional IN/OUT endpoints, which need to be cleared by the CPU."]
    #[inline(always)]
    #[must_use]
    pub fn sent_stall_send_stall(&mut self) -> SentStallSendStallW<Idx0Spec> {
        SentStallSendStallW::new(self, 21)
    }
    #[doc = "Bit 22 - Clear Data Toggle / Serviced OUT Packet Ready. When CFG3_ENDPOINT = 1 to 5, this bit serves as the ClrDataTog field. When CFG3_ENDPOINT = 0, this bit serves as the ServicedOutPktReady field.If CFG3_ENDPOINT = 0x1-0x5, this bit serves as the ClrDataTog field. Setting this bit resets the endpoint IN data toggle to 0.If CFG3_ENDPOINT = 0x0, this bit serves as the ServicedOutPktReady field. The CPU writes a 1 to this bit to clear the OutPktRdy bit. This bit is cleared automatically."]
    #[inline(always)]
    #[must_use]
    pub fn clr_data_tog_serviced_out_pkt_rdy(&mut self) -> ClrDataTogServicedOutPktRdyW<Idx0Spec> {
        ClrDataTogServicedOutPktRdyW::new(self, 22)
    }
    #[doc = "Bit 23 - Incomplete Transmission / Service Setup End. When CFG3_ENDPOINT = 1 to 5, this bit serves as the IncompTx field. When CFG3_ENDPOINT = 0, this bit serves as the ServiceSetupEnd field.If CFG3_ENDPOINT = 0x1-0x5, then this bit serves as the IncompTx field. If the endpoint is being used for high-bandwidth Isochronous transfers, this bit is set to indicate when a large packet has been split into 2 or 3 packets for transmission but insufficient IN tokens have been received to send all the parts. The remainder of the current packet is then flushed from the FIFO (but any second packet in the FIFO will remain). Note: In anything other than a high bandwidth Isochronous transfer, this bit will always return 0."]
    #[inline(always)]
    #[must_use]
    pub fn incomp_tx_service_setup_end(&mut self) -> IncompTxServiceSetupEndW<Idx0Spec> {
        IncompTxServiceSetupEndW::new(self, 23)
    }
    #[doc = "Bit 24 - Unused, always return 0."]
    #[inline(always)]
    #[must_use]
    pub fn d0(&mut self) -> D0W<Idx0Spec> {
        D0W::new(self, 24)
    }
    #[doc = "Bit 25 - Double Packet Buffer Disable. This bit is used to control the use of Double Packet Buffering. It is ignored when Dynamic FIFO sizing is enabled. Clearing this bit does NOT necessarily enable Double Packet Buffering but rather allows Double Packet Buffering to be determined by the Endpoint's IDX2_INFIFOSZ setting and MAXPAYLOAD size relationship. Default is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn dpkt_buf_dis(&mut self) -> DpktBufDisW<Idx0Spec> {
        DpktBufDisW::new(self, 25)
    }
    #[doc = "Bit 27 - Force Data Toggle. The CPU sets this bit to force the endpoint's IN data toggle to switch after each data packet is sent regardless of whether an ACK was received. This can be used by Interrupt IN endpoints that are used to communicate rate feedback for Isochronous endpoints."]
    #[inline(always)]
    #[must_use]
    pub fn frc_data_tog(&mut self) -> FrcDataTogW<Idx0Spec> {
        FrcDataTogW::new(self, 27)
    }
    #[doc = "Bit 29 - OUT/IN Mode. The CPU sets this bit to enable the endpoint direction as IN or OUT. Note: Only valid where the endpoint FIFO is used for both IN and OUT transactions, otherwise ignored."]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> ModeW<Idx0Spec> {
        ModeW::new(self, 29)
    }
    #[doc = "Bit 30 - Isochronous Transfers. The CPU sets this bit to enable the IN endpoint for Isochronous transfers (ISO mode) or for Bulk/Interrupt transfers."]
    #[inline(always)]
    #[must_use]
    pub fn iso(&mut self) -> IsoW<Idx0Spec> {
        IsoW::new(self, 30)
    }
    #[doc = "Bit 31 - Automatically Set InPktRdy. When set, the FIFONotEmptyInPktRdy field (for IN Endpoint 0) or InPktRdyOutPktRdy field (for IN Endpoint 1-5) in this register will be automatically set when data of the maximum packet size (set in MAXPAYLOAD field) is loaded into the IN FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn auto_set(&mut self) -> AutoSetW<Idx0Spec> {
        AutoSetW::new(self, 31)
    }
}
#[doc = "Provides additional control and status for IN transactions through the currently-selected endpoint. (To avoid CMSIS conflicts, the address here includes an additional offset of 0x1000. Access to this register must take this into account.) The value returned when this register is read reflects the status of an endpoint specified by setting the endpoint index in the CFG3_ENDPOINT field. When the endpoint index (CFG3_ENDPOINT) = 0, this field provides status and control of Endpoint 0. Also, the MAXPAYLOAD field defines the maximum amount of data that can be transferred through the selected IN endpoint in a single operation. There is a MAXPAYLOAD for each IN endpoint (except Endpoint 0). Note that the action initiated by setting a field in one of the IDXn registers when the selected endpoint has not been configured is undefined.\n\nYou can [`read`](crate::Reg::read) this register and get [`idx0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idx0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Idx0Spec;
impl crate::RegisterSpec for Idx0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idx0::R`](R) reader structure"]
impl crate::Readable for Idx0Spec {}
#[doc = "`write(|w| ..)` method takes [`idx0::W`](W) writer structure"]
impl crate::Writable for Idx0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IDX0 to value 0"]
impl crate::Resettable for Idx0Spec {
    const RESET_VALUE: u32 = 0;
}
