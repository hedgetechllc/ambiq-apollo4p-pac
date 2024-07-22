#[doc = "Register `IDX1` reader"]
pub type R = crate::R<Idx1Spec>;
#[doc = "Register `IDX1` writer"]
pub type W = crate::W<Idx1Spec>;
#[doc = "Field `MAXPAYLOAD` reader - Maximum Payload transmitted in a single transaction. The value set can be up to 1024 bytes but is subject to the constraints placed by the USB Specification on packet sizes for Bulk, Interrupt and Isochronous transfers in Fullspeed and High-speed operations. The total amount of data represented by MAXPAYLOAD x (PKTSPLITOPTION + 1) must not exceed the FIFO size for the OUT endpoint, and should not exceed half the FIFO size if double-buffering is required. Note: The value written here (multiplied by m in the case of high-bandwidth Isochronous transfers) must match the value given in the wMaxPacketSize field of the Standard Endpoint Descriptor for the associated endpoint (see USB Specification Revision 2.0, Chapter 9). A mismatch could cause nexpected results."]
pub type MaxpayloadR = crate::FieldReader<u16>;
#[doc = "Field `MAXPAYLOAD` writer - Maximum Payload transmitted in a single transaction. The value set can be up to 1024 bytes but is subject to the constraints placed by the USB Specification on packet sizes for Bulk, Interrupt and Isochronous transfers in Fullspeed and High-speed operations. The total amount of data represented by MAXPAYLOAD x (PKTSPLITOPTION + 1) must not exceed the FIFO size for the OUT endpoint, and should not exceed half the FIFO size if double-buffering is required. Note: The value written here (multiplied by m in the case of high-bandwidth Isochronous transfers) must match the value given in the wMaxPacketSize field of the Standard Endpoint Descriptor for the associated endpoint (see USB Specification Revision 2.0, Chapter 9). A mismatch could cause nexpected results."]
pub type MaxpayloadW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `PKTSPLITOPTION` reader - Packet Split Option. When IDX1_ISO = 1, this bit serves as the MAXPAYLOAD multiplier for Isochronous OUT transfers. When IDX1_ISO = 0, this bit serves as the MAXPAYLOAD multiplier for Bulk IN transfers.If IDX1_ISO = 0x1, this field sets the multiplier for Isochronous transfers. For Isochronous endpoints operating in High-Speed mode and with the High-bandwidth option enabled, PKTSPLITOPTION may be either 2 or 3 (corresponding to this field's bit 0 set or bit 1 set, respectively, and bits\\[4:2\\]
are ignored) and it specifies the maximum number of such transactions that can take place in a single microframe. If either bit 0 or bit 1 is non-zero, the USB Controller will automatically combine the separate USB packets received in any microframe into a single packet within the OUT FIFO. The maximum payload for each transaction is 1024 bytes, so this allows up to 3072 bytes to be received in each microframe. (For Isochronous transfers in full-speed mode or if high-bandwidth is not enabled, bits 0 and 1 are ignored.)If IDX1_ISO = 0x0, this field sets the multiplier for Bulk transfers. The field setting can be up to 31, and the multiplier, m, is (field setting + 1). The multiplier defines the number of USB packets of the specified payload which are to be combined into a single data packet within the FIFO."]
pub type PktsplitoptionR = crate::FieldReader;
#[doc = "Field `PKTSPLITOPTION` writer - Packet Split Option. When IDX1_ISO = 1, this bit serves as the MAXPAYLOAD multiplier for Isochronous OUT transfers. When IDX1_ISO = 0, this bit serves as the MAXPAYLOAD multiplier for Bulk IN transfers.If IDX1_ISO = 0x1, this field sets the multiplier for Isochronous transfers. For Isochronous endpoints operating in High-Speed mode and with the High-bandwidth option enabled, PKTSPLITOPTION may be either 2 or 3 (corresponding to this field's bit 0 set or bit 1 set, respectively, and bits\\[4:2\\]
are ignored) and it specifies the maximum number of such transactions that can take place in a single microframe. If either bit 0 or bit 1 is non-zero, the USB Controller will automatically combine the separate USB packets received in any microframe into a single packet within the OUT FIFO. The maximum payload for each transaction is 1024 bytes, so this allows up to 3072 bytes to be received in each microframe. (For Isochronous transfers in full-speed mode or if high-bandwidth is not enabled, bits 0 and 1 are ignored.)If IDX1_ISO = 0x0, this field sets the multiplier for Bulk transfers. The field setting can be up to 31, and the multiplier, m, is (field setting + 1). The multiplier defines the number of USB packets of the specified payload which are to be combined into a single data packet within the FIFO."]
pub type PktsplitoptionW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `OutPktRdy` reader - OUT Packet Ready. This bit is set when a data packet has been received. Clear this bit when the packet has been unloaded from the OUT FIFO. An interrupt is generated (if enabled) when the bit is set."]
pub type OutPktRdyR = crate::BitReader;
#[doc = "Field `OutPktRdy` writer - OUT Packet Ready. This bit is set when a data packet has been received. Clear this bit when the packet has been unloaded from the OUT FIFO. An interrupt is generated (if enabled) when the bit is set."]
pub type OutPktRdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFOFull` reader - FIFO Full. When set, this bit indicates that no more packets can be loaded into the OUT FIFO."]
pub type FifofullR = crate::BitReader;
#[doc = "Field `FIFOFull` writer - FIFO Full. When set, this bit indicates that no more packets can be loaded into the OUT FIFO."]
pub type FifofullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OverRun` reader - Overrun Condition. Indicates an overrun.If IDX1_ISO = 0x1 (ISO mode), this bit is set if an OUT packet arrives while FIFOFull is set, i.e., the OUT packet cannot be loaded into the OUT FIFO. The CPU should clear this bit.If IDX1_ISO = 0x0 (Bulk mode), this field always returns zero. This field is only valid when the endpoint is operating in ISO mode."]
pub type OverRunR = crate::BitReader;
#[doc = "Field `OverRun` writer - Overrun Condition. Indicates an overrun.If IDX1_ISO = 0x1 (ISO mode), this bit is set if an OUT packet arrives while FIFOFull is set, i.e., the OUT packet cannot be loaded into the OUT FIFO. The CPU should clear this bit.If IDX1_ISO = 0x0 (Bulk mode), this field always returns zero. This field is only valid when the endpoint is operating in ISO mode."]
pub type OverRunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DataError` reader - Data Error. Indicates a CRC error.If IDX1_ISO = 0x1 (ISO mode), this bit is set at the same time that OutPktRdy is set if the data packet has a CRC error. It is cleared when OutPktRdy is cleared.If IDX1_ISO = 0x0 (Bulk mode), this field always returns zero. This field is only valid when the endpoint is operating in ISO mode."]
pub type DataErrorR = crate::BitReader;
#[doc = "Field `DataError` writer - Data Error. Indicates a CRC error.If IDX1_ISO = 0x1 (ISO mode), this bit is set at the same time that OutPktRdy is set if the data packet has a CRC error. It is cleared when OutPktRdy is cleared.If IDX1_ISO = 0x0 (Bulk mode), this field always returns zero. This field is only valid when the endpoint is operating in ISO mode."]
pub type DataErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FlushFIFO` reader - Flush FIFO. Set this bit to flush the next packet to be read from the endpoint OUT FIFO. The FIFO pointer is reset and the OutPktRdy bit is cleared. FlushFIFO should only be used when OutPktRdy is set. At other times, it may cause data to be corrupted. If the FIFO is double-buffered, FlushFIFO may need to be set twice to completely clear the FIFO."]
pub type FlushFifoR = crate::BitReader;
#[doc = "Field `FlushFIFO` writer - Flush FIFO. Set this bit to flush the next packet to be read from the endpoint OUT FIFO. The FIFO pointer is reset and the OutPktRdy bit is cleared. FlushFIFO should only be used when OutPktRdy is set. At other times, it may cause data to be corrupted. If the FIFO is double-buffered, FlushFIFO may need to be set twice to completely clear the FIFO."]
pub type FlushFifoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SendStall` reader - Send Stall. Issues a STALL handshake to a DATA packet.If IDX1_ISO = 0x1, this bit has no effect when the endpoint is being used for Isochronous transfers.If IDX1_ISO = 0x0, this field enables Stall Handshakes for Bulk/Interrupt transactions. Set this bit to issue a STALL handshake to a DATA packet. Clear this bit to terminate the stall condition."]
pub type SendStallR = crate::BitReader;
#[doc = "Field `SendStall` writer - Send Stall. Issues a STALL handshake to a DATA packet.If IDX1_ISO = 0x1, this bit has no effect when the endpoint is being used for Isochronous transfers.If IDX1_ISO = 0x0, this field enables Stall Handshakes for Bulk/Interrupt transactions. Set this bit to issue a STALL handshake to a DATA packet. Clear this bit to terminate the stall condition."]
pub type SendStallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SentStall` reader - Sent Stall. This bit is set when a STALL handshake is transmitted. The CPU should clear this bit."]
pub type SentStallR = crate::BitReader;
#[doc = "Field `SentStall` writer - Sent Stall. This bit is set when a STALL handshake is transmitted. The CPU should clear this bit."]
pub type SentStallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ClrDataTog` reader - Clear Data Toggle. Set this bit to reset the endpoint data toggle to 0."]
pub type ClrDataTogR = crate::BitReader;
#[doc = "Field `ClrDataTog` writer - Clear Data Toggle. Set this bit to reset the endpoint data toggle to 0."]
pub type ClrDataTogW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IncompRx` reader - Incomplete Receive. This bit is set in a high-bandwidth Isochronous transfer if the packet in the OUT FIFO is incomplete because parts of the data were not received. It is cleared when OutPktRdy is cleared. Note: In anything other than a high-bandwidth Isochronous transfer, this bit will always return 0."]
pub type IncompRxR = crate::BitReader;
#[doc = "Field `IncompRx` writer - Incomplete Receive. This bit is set in a high-bandwidth Isochronous transfer if the packet in the OUT FIFO is incomplete because parts of the data were not received. It is cleared when OutPktRdy is cleared. Note: In anything other than a high-bandwidth Isochronous transfer, this bit will always return 0."]
pub type IncompRxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Double Packet Buffer Disable. This bit is used to control the use of Double Packet Buffering. It is ignored when Dynamic FIFO sizing is enabled. Clearing this bit does NOT necessarily enable Double Packet Buffering but rather allows Double Packet Buffering to be determined by the Endpoint's IDX2_OUTFIFOSZ setting and MAXPAYLOAD size relationship. Default is enabled.\n\nValue on reset: 0"]
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
#[doc = "Field `DPktBufDis` reader - Double Packet Buffer Disable. This bit is used to control the use of Double Packet Buffering. It is ignored when Dynamic FIFO sizing is enabled. Clearing this bit does NOT necessarily enable Double Packet Buffering but rather allows Double Packet Buffering to be determined by the Endpoint's IDX2_OUTFIFOSZ setting and MAXPAYLOAD size relationship. Default is enabled."]
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
#[doc = "Field `DPktBufDis` writer - Double Packet Buffer Disable. This bit is used to control the use of Double Packet Buffering. It is ignored when Dynamic FIFO sizing is enabled. Clearing this bit does NOT necessarily enable Double Packet Buffering but rather allows Double Packet Buffering to be determined by the Endpoint's IDX2_OUTFIFOSZ setting and MAXPAYLOAD size relationship. Default is enabled."]
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
#[doc = "Field `DisNye` reader - Disable NYET Handshakes / PID Error. For Bulk/Interrupt transactions, this bit disable the sending of NYET handshakes. For Bulk/Interrupt transactions, indicates PID errors.If IDX1_ISO = 0x1, this field is read-only and, when set, indicates a PID error in the received packet for Isochronous transfers.If IDX1_ISO = 0x0, this field disables NYET Handshakes for Bulk/Interrupt transactions. Set this bit to disable the sending of NYET handshakes. When set, all successfully received OUT packets are ACK'd including at the point at which the FIFO becomes full.Note: This bit has an effect only in High-speed mode, when it should be set for all Interrupt endpoints."]
pub type DisNyeR = crate::BitReader;
#[doc = "Field `DisNye` writer - Disable NYET Handshakes / PID Error. For Bulk/Interrupt transactions, this bit disable the sending of NYET handshakes. For Bulk/Interrupt transactions, indicates PID errors.If IDX1_ISO = 0x1, this field is read-only and, when set, indicates a PID error in the received packet for Isochronous transfers.If IDX1_ISO = 0x0, this field disables NYET Handshakes for Bulk/Interrupt transactions. Set this bit to disable the sending of NYET handshakes. When set, all successfully received OUT packets are ACK'd including at the point at which the FIFO becomes full.Note: This bit has an effect only in High-speed mode, when it should be set for all Interrupt endpoints."]
pub type DisNyeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Isochronous Transfers. The CPU sets this bit to enable the OUT endpoint for either Isochronous transfers (ISO mode) or for Bulk/Interrupt transfers.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iso {
    #[doc = "0: Clear to enable the OUT endpoint for Bulk/Interrupt transfers."]
    BulkInt = 0,
    #[doc = "1: Set to enable the OUT endpoint for Isochronous transfers."]
    Iso = 1,
}
impl From<Iso> for bool {
    #[inline(always)]
    fn from(variant: Iso) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISO` reader - Isochronous Transfers. The CPU sets this bit to enable the OUT endpoint for either Isochronous transfers (ISO mode) or for Bulk/Interrupt transfers."]
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
    #[doc = "Clear to enable the OUT endpoint for Bulk/Interrupt transfers."]
    #[inline(always)]
    pub fn is_bulk_int(&self) -> bool {
        *self == Iso::BulkInt
    }
    #[doc = "Set to enable the OUT endpoint for Isochronous transfers."]
    #[inline(always)]
    pub fn is_iso(&self) -> bool {
        *self == Iso::Iso
    }
}
#[doc = "Field `ISO` writer - Isochronous Transfers. The CPU sets this bit to enable the OUT endpoint for either Isochronous transfers (ISO mode) or for Bulk/Interrupt transfers."]
pub type IsoW<'a, REG> = crate::BitWriter<'a, REG, Iso>;
impl<'a, REG> IsoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear to enable the OUT endpoint for Bulk/Interrupt transfers."]
    #[inline(always)]
    pub fn bulk_int(self) -> &'a mut crate::W<REG> {
        self.variant(Iso::BulkInt)
    }
    #[doc = "Set to enable the OUT endpoint for Isochronous transfers."]
    #[inline(always)]
    pub fn iso(self) -> &'a mut crate::W<REG> {
        self.variant(Iso::Iso)
    }
}
#[doc = "Automatically Clear OutPktRdy.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AutoClear {
    #[doc = "0: OutPktRdy field will not be automatically cleared when a packet of MAXPAYLOAD data size is unloaded from the OUT FIFO."]
    NoAutoclr = 0,
    #[doc = "1: OutPktRdy field will be automatically cleared when a packet of MAXPAYLOAD data size is unloaded from the OUT FIFO. When packets of less than the maximum packet size are unloaded, OutPktRdy must be cleared manually. Note: Should not be set for high bandwidth Isochronous endpoints."]
    Autoclr = 1,
}
impl From<AutoClear> for bool {
    #[inline(always)]
    fn from(variant: AutoClear) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AutoClear` reader - Automatically Clear OutPktRdy."]
pub type AutoClearR = crate::BitReader<AutoClear>;
impl AutoClearR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AutoClear {
        match self.bits {
            false => AutoClear::NoAutoclr,
            true => AutoClear::Autoclr,
        }
    }
    #[doc = "OutPktRdy field will not be automatically cleared when a packet of MAXPAYLOAD data size is unloaded from the OUT FIFO."]
    #[inline(always)]
    pub fn is_no_autoclr(&self) -> bool {
        *self == AutoClear::NoAutoclr
    }
    #[doc = "OutPktRdy field will be automatically cleared when a packet of MAXPAYLOAD data size is unloaded from the OUT FIFO. When packets of less than the maximum packet size are unloaded, OutPktRdy must be cleared manually. Note: Should not be set for high bandwidth Isochronous endpoints."]
    #[inline(always)]
    pub fn is_autoclr(&self) -> bool {
        *self == AutoClear::Autoclr
    }
}
#[doc = "Field `AutoClear` writer - Automatically Clear OutPktRdy."]
pub type AutoClearW<'a, REG> = crate::BitWriter<'a, REG, AutoClear>;
impl<'a, REG> AutoClearW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "OutPktRdy field will not be automatically cleared when a packet of MAXPAYLOAD data size is unloaded from the OUT FIFO."]
    #[inline(always)]
    pub fn no_autoclr(self) -> &'a mut crate::W<REG> {
        self.variant(AutoClear::NoAutoclr)
    }
    #[doc = "OutPktRdy field will be automatically cleared when a packet of MAXPAYLOAD data size is unloaded from the OUT FIFO. When packets of less than the maximum packet size are unloaded, OutPktRdy must be cleared manually. Note: Should not be set for high bandwidth Isochronous endpoints."]
    #[inline(always)]
    pub fn autoclr(self) -> &'a mut crate::W<REG> {
        self.variant(AutoClear::Autoclr)
    }
}
impl R {
    #[doc = "Bits 0:10 - Maximum Payload transmitted in a single transaction. The value set can be up to 1024 bytes but is subject to the constraints placed by the USB Specification on packet sizes for Bulk, Interrupt and Isochronous transfers in Fullspeed and High-speed operations. The total amount of data represented by MAXPAYLOAD x (PKTSPLITOPTION + 1) must not exceed the FIFO size for the OUT endpoint, and should not exceed half the FIFO size if double-buffering is required. Note: The value written here (multiplied by m in the case of high-bandwidth Isochronous transfers) must match the value given in the wMaxPacketSize field of the Standard Endpoint Descriptor for the associated endpoint (see USB Specification Revision 2.0, Chapter 9). A mismatch could cause nexpected results."]
    #[inline(always)]
    pub fn maxpayload(&self) -> MaxpayloadR {
        MaxpayloadR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:15 - Packet Split Option. When IDX1_ISO = 1, this bit serves as the MAXPAYLOAD multiplier for Isochronous OUT transfers. When IDX1_ISO = 0, this bit serves as the MAXPAYLOAD multiplier for Bulk IN transfers.If IDX1_ISO = 0x1, this field sets the multiplier for Isochronous transfers. For Isochronous endpoints operating in High-Speed mode and with the High-bandwidth option enabled, PKTSPLITOPTION may be either 2 or 3 (corresponding to this field's bit 0 set or bit 1 set, respectively, and bits\\[4:2\\]
are ignored) and it specifies the maximum number of such transactions that can take place in a single microframe. If either bit 0 or bit 1 is non-zero, the USB Controller will automatically combine the separate USB packets received in any microframe into a single packet within the OUT FIFO. The maximum payload for each transaction is 1024 bytes, so this allows up to 3072 bytes to be received in each microframe. (For Isochronous transfers in full-speed mode or if high-bandwidth is not enabled, bits 0 and 1 are ignored.)If IDX1_ISO = 0x0, this field sets the multiplier for Bulk transfers. The field setting can be up to 31, and the multiplier, m, is (field setting + 1). The multiplier defines the number of USB packets of the specified payload which are to be combined into a single data packet within the FIFO."]
    #[inline(always)]
    pub fn pktsplitoption(&self) -> PktsplitoptionR {
        PktsplitoptionR::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bit 16 - OUT Packet Ready. This bit is set when a data packet has been received. Clear this bit when the packet has been unloaded from the OUT FIFO. An interrupt is generated (if enabled) when the bit is set."]
    #[inline(always)]
    pub fn out_pkt_rdy(&self) -> OutPktRdyR {
        OutPktRdyR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - FIFO Full. When set, this bit indicates that no more packets can be loaded into the OUT FIFO."]
    #[inline(always)]
    pub fn fifofull(&self) -> FifofullR {
        FifofullR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Overrun Condition. Indicates an overrun.If IDX1_ISO = 0x1 (ISO mode), this bit is set if an OUT packet arrives while FIFOFull is set, i.e., the OUT packet cannot be loaded into the OUT FIFO. The CPU should clear this bit.If IDX1_ISO = 0x0 (Bulk mode), this field always returns zero. This field is only valid when the endpoint is operating in ISO mode."]
    #[inline(always)]
    pub fn over_run(&self) -> OverRunR {
        OverRunR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Data Error. Indicates a CRC error.If IDX1_ISO = 0x1 (ISO mode), this bit is set at the same time that OutPktRdy is set if the data packet has a CRC error. It is cleared when OutPktRdy is cleared.If IDX1_ISO = 0x0 (Bulk mode), this field always returns zero. This field is only valid when the endpoint is operating in ISO mode."]
    #[inline(always)]
    pub fn data_error(&self) -> DataErrorR {
        DataErrorR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Flush FIFO. Set this bit to flush the next packet to be read from the endpoint OUT FIFO. The FIFO pointer is reset and the OutPktRdy bit is cleared. FlushFIFO should only be used when OutPktRdy is set. At other times, it may cause data to be corrupted. If the FIFO is double-buffered, FlushFIFO may need to be set twice to completely clear the FIFO."]
    #[inline(always)]
    pub fn flush_fifo(&self) -> FlushFifoR {
        FlushFifoR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Send Stall. Issues a STALL handshake to a DATA packet.If IDX1_ISO = 0x1, this bit has no effect when the endpoint is being used for Isochronous transfers.If IDX1_ISO = 0x0, this field enables Stall Handshakes for Bulk/Interrupt transactions. Set this bit to issue a STALL handshake to a DATA packet. Clear this bit to terminate the stall condition."]
    #[inline(always)]
    pub fn send_stall(&self) -> SendStallR {
        SendStallR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Sent Stall. This bit is set when a STALL handshake is transmitted. The CPU should clear this bit."]
    #[inline(always)]
    pub fn sent_stall(&self) -> SentStallR {
        SentStallR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Clear Data Toggle. Set this bit to reset the endpoint data toggle to 0."]
    #[inline(always)]
    pub fn clr_data_tog(&self) -> ClrDataTogR {
        ClrDataTogR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Incomplete Receive. This bit is set in a high-bandwidth Isochronous transfer if the packet in the OUT FIFO is incomplete because parts of the data were not received. It is cleared when OutPktRdy is cleared. Note: In anything other than a high-bandwidth Isochronous transfer, this bit will always return 0."]
    #[inline(always)]
    pub fn incomp_rx(&self) -> IncompRxR {
        IncompRxR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Double Packet Buffer Disable. This bit is used to control the use of Double Packet Buffering. It is ignored when Dynamic FIFO sizing is enabled. Clearing this bit does NOT necessarily enable Double Packet Buffering but rather allows Double Packet Buffering to be determined by the Endpoint's IDX2_OUTFIFOSZ setting and MAXPAYLOAD size relationship. Default is enabled."]
    #[inline(always)]
    pub fn dpkt_buf_dis(&self) -> DpktBufDisR {
        DpktBufDisR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 28 - Disable NYET Handshakes / PID Error. For Bulk/Interrupt transactions, this bit disable the sending of NYET handshakes. For Bulk/Interrupt transactions, indicates PID errors.If IDX1_ISO = 0x1, this field is read-only and, when set, indicates a PID error in the received packet for Isochronous transfers.If IDX1_ISO = 0x0, this field disables NYET Handshakes for Bulk/Interrupt transactions. Set this bit to disable the sending of NYET handshakes. When set, all successfully received OUT packets are ACK'd including at the point at which the FIFO becomes full.Note: This bit has an effect only in High-speed mode, when it should be set for all Interrupt endpoints."]
    #[inline(always)]
    pub fn dis_nye(&self) -> DisNyeR {
        DisNyeR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - Isochronous Transfers. The CPU sets this bit to enable the OUT endpoint for either Isochronous transfers (ISO mode) or for Bulk/Interrupt transfers."]
    #[inline(always)]
    pub fn iso(&self) -> IsoR {
        IsoR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Automatically Clear OutPktRdy."]
    #[inline(always)]
    pub fn auto_clear(&self) -> AutoClearR {
        AutoClearR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10 - Maximum Payload transmitted in a single transaction. The value set can be up to 1024 bytes but is subject to the constraints placed by the USB Specification on packet sizes for Bulk, Interrupt and Isochronous transfers in Fullspeed and High-speed operations. The total amount of data represented by MAXPAYLOAD x (PKTSPLITOPTION + 1) must not exceed the FIFO size for the OUT endpoint, and should not exceed half the FIFO size if double-buffering is required. Note: The value written here (multiplied by m in the case of high-bandwidth Isochronous transfers) must match the value given in the wMaxPacketSize field of the Standard Endpoint Descriptor for the associated endpoint (see USB Specification Revision 2.0, Chapter 9). A mismatch could cause nexpected results."]
    #[inline(always)]
    #[must_use]
    pub fn maxpayload(&mut self) -> MaxpayloadW<Idx1Spec> {
        MaxpayloadW::new(self, 0)
    }
    #[doc = "Bits 11:15 - Packet Split Option. When IDX1_ISO = 1, this bit serves as the MAXPAYLOAD multiplier for Isochronous OUT transfers. When IDX1_ISO = 0, this bit serves as the MAXPAYLOAD multiplier for Bulk IN transfers.If IDX1_ISO = 0x1, this field sets the multiplier for Isochronous transfers. For Isochronous endpoints operating in High-Speed mode and with the High-bandwidth option enabled, PKTSPLITOPTION may be either 2 or 3 (corresponding to this field's bit 0 set or bit 1 set, respectively, and bits\\[4:2\\]
are ignored) and it specifies the maximum number of such transactions that can take place in a single microframe. If either bit 0 or bit 1 is non-zero, the USB Controller will automatically combine the separate USB packets received in any microframe into a single packet within the OUT FIFO. The maximum payload for each transaction is 1024 bytes, so this allows up to 3072 bytes to be received in each microframe. (For Isochronous transfers in full-speed mode or if high-bandwidth is not enabled, bits 0 and 1 are ignored.)If IDX1_ISO = 0x0, this field sets the multiplier for Bulk transfers. The field setting can be up to 31, and the multiplier, m, is (field setting + 1). The multiplier defines the number of USB packets of the specified payload which are to be combined into a single data packet within the FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn pktsplitoption(&mut self) -> PktsplitoptionW<Idx1Spec> {
        PktsplitoptionW::new(self, 11)
    }
    #[doc = "Bit 16 - OUT Packet Ready. This bit is set when a data packet has been received. Clear this bit when the packet has been unloaded from the OUT FIFO. An interrupt is generated (if enabled) when the bit is set."]
    #[inline(always)]
    #[must_use]
    pub fn out_pkt_rdy(&mut self) -> OutPktRdyW<Idx1Spec> {
        OutPktRdyW::new(self, 16)
    }
    #[doc = "Bit 17 - FIFO Full. When set, this bit indicates that no more packets can be loaded into the OUT FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn fifofull(&mut self) -> FifofullW<Idx1Spec> {
        FifofullW::new(self, 17)
    }
    #[doc = "Bit 18 - Overrun Condition. Indicates an overrun.If IDX1_ISO = 0x1 (ISO mode), this bit is set if an OUT packet arrives while FIFOFull is set, i.e., the OUT packet cannot be loaded into the OUT FIFO. The CPU should clear this bit.If IDX1_ISO = 0x0 (Bulk mode), this field always returns zero. This field is only valid when the endpoint is operating in ISO mode."]
    #[inline(always)]
    #[must_use]
    pub fn over_run(&mut self) -> OverRunW<Idx1Spec> {
        OverRunW::new(self, 18)
    }
    #[doc = "Bit 19 - Data Error. Indicates a CRC error.If IDX1_ISO = 0x1 (ISO mode), this bit is set at the same time that OutPktRdy is set if the data packet has a CRC error. It is cleared when OutPktRdy is cleared.If IDX1_ISO = 0x0 (Bulk mode), this field always returns zero. This field is only valid when the endpoint is operating in ISO mode."]
    #[inline(always)]
    #[must_use]
    pub fn data_error(&mut self) -> DataErrorW<Idx1Spec> {
        DataErrorW::new(self, 19)
    }
    #[doc = "Bit 20 - Flush FIFO. Set this bit to flush the next packet to be read from the endpoint OUT FIFO. The FIFO pointer is reset and the OutPktRdy bit is cleared. FlushFIFO should only be used when OutPktRdy is set. At other times, it may cause data to be corrupted. If the FIFO is double-buffered, FlushFIFO may need to be set twice to completely clear the FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn flush_fifo(&mut self) -> FlushFifoW<Idx1Spec> {
        FlushFifoW::new(self, 20)
    }
    #[doc = "Bit 21 - Send Stall. Issues a STALL handshake to a DATA packet.If IDX1_ISO = 0x1, this bit has no effect when the endpoint is being used for Isochronous transfers.If IDX1_ISO = 0x0, this field enables Stall Handshakes for Bulk/Interrupt transactions. Set this bit to issue a STALL handshake to a DATA packet. Clear this bit to terminate the stall condition."]
    #[inline(always)]
    #[must_use]
    pub fn send_stall(&mut self) -> SendStallW<Idx1Spec> {
        SendStallW::new(self, 21)
    }
    #[doc = "Bit 22 - Sent Stall. This bit is set when a STALL handshake is transmitted. The CPU should clear this bit."]
    #[inline(always)]
    #[must_use]
    pub fn sent_stall(&mut self) -> SentStallW<Idx1Spec> {
        SentStallW::new(self, 22)
    }
    #[doc = "Bit 23 - Clear Data Toggle. Set this bit to reset the endpoint data toggle to 0."]
    #[inline(always)]
    #[must_use]
    pub fn clr_data_tog(&mut self) -> ClrDataTogW<Idx1Spec> {
        ClrDataTogW::new(self, 23)
    }
    #[doc = "Bit 24 - Incomplete Receive. This bit is set in a high-bandwidth Isochronous transfer if the packet in the OUT FIFO is incomplete because parts of the data were not received. It is cleared when OutPktRdy is cleared. Note: In anything other than a high-bandwidth Isochronous transfer, this bit will always return 0."]
    #[inline(always)]
    #[must_use]
    pub fn incomp_rx(&mut self) -> IncompRxW<Idx1Spec> {
        IncompRxW::new(self, 24)
    }
    #[doc = "Bit 25 - Double Packet Buffer Disable. This bit is used to control the use of Double Packet Buffering. It is ignored when Dynamic FIFO sizing is enabled. Clearing this bit does NOT necessarily enable Double Packet Buffering but rather allows Double Packet Buffering to be determined by the Endpoint's IDX2_OUTFIFOSZ setting and MAXPAYLOAD size relationship. Default is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn dpkt_buf_dis(&mut self) -> DpktBufDisW<Idx1Spec> {
        DpktBufDisW::new(self, 25)
    }
    #[doc = "Bit 28 - Disable NYET Handshakes / PID Error. For Bulk/Interrupt transactions, this bit disable the sending of NYET handshakes. For Bulk/Interrupt transactions, indicates PID errors.If IDX1_ISO = 0x1, this field is read-only and, when set, indicates a PID error in the received packet for Isochronous transfers.If IDX1_ISO = 0x0, this field disables NYET Handshakes for Bulk/Interrupt transactions. Set this bit to disable the sending of NYET handshakes. When set, all successfully received OUT packets are ACK'd including at the point at which the FIFO becomes full.Note: This bit has an effect only in High-speed mode, when it should be set for all Interrupt endpoints."]
    #[inline(always)]
    #[must_use]
    pub fn dis_nye(&mut self) -> DisNyeW<Idx1Spec> {
        DisNyeW::new(self, 28)
    }
    #[doc = "Bit 30 - Isochronous Transfers. The CPU sets this bit to enable the OUT endpoint for either Isochronous transfers (ISO mode) or for Bulk/Interrupt transfers."]
    #[inline(always)]
    #[must_use]
    pub fn iso(&mut self) -> IsoW<Idx1Spec> {
        IsoW::new(self, 30)
    }
    #[doc = "Bit 31 - Automatically Clear OutPktRdy."]
    #[inline(always)]
    #[must_use]
    pub fn auto_clear(&mut self) -> AutoClearW<Idx1Spec> {
        AutoClearW::new(self, 31)
    }
}
#[doc = "Provides control and status bits for OUT transactions through the currently-selected endpoint. It is reset to 0. The value returned when this register is read reflects the status of an endpoint specified by setting the endpoint index in the CFG3_ENDPOINT field. Also, the MAXPAYLOAD field defines the maximum amount of data that can be transferred through the selected OUT endpoint in a single operation. There is a MAXPAYLOAD for each OUT endpoint (except Endpoint 0). Note that the action initiated by setting a field in one of the IDXn registers when the selected endpoint has not been configured is undefined.\n\nYou can [`read`](crate::Reg::read) this register and get [`idx1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idx1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Idx1Spec;
impl crate::RegisterSpec for Idx1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idx1::R`](R) reader structure"]
impl crate::Readable for Idx1Spec {}
#[doc = "`write(|w| ..)` method takes [`idx1::W`](W) writer structure"]
impl crate::Writable for Idx1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IDX1 to value 0"]
impl crate::Resettable for Idx1Spec {
    const RESET_VALUE: u32 = 0;
}
