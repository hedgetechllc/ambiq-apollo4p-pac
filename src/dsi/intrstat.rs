#[doc = "Register `INTRSTAT` reader"]
pub type R = crate::R<IntrstatSpec>;
#[doc = "Register `INTRSTAT` writer"]
pub type W = crate::W<IntrstatSpec>;
#[doc = "Field `RXSOTERROR` reader - (RW1C) Set to 1 if a start of transmission sequence error is reported in the Acknowledge packet by the display device"]
pub type RxsoterrorR = crate::BitReader;
#[doc = "Field `RXSOTERROR` writer - (RW1C) Set to 1 if a start of transmission sequence error is reported in the Acknowledge packet by the display device"]
pub type RxsoterrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXSOTSYNCERROR` reader - (RW1C) Set to 1 if synchronisation error occurrence in the start of transmission sequence is reported in the acknowledge packet by the display device"]
pub type RxsotsyncerrorR = crate::BitReader;
#[doc = "Field `RXSOTSYNCERROR` writer - (RW1C) Set to 1 if synchronisation error occurrence in the start of transmission sequence is reported in the acknowledge packet by the display device"]
pub type RxsotsyncerrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXEOTSYNCERROR` reader - (RW1C) Set to 1 if End of transmission synchronisation Error is reported in the acknowledgment packet by the display device"]
pub type RxeotsyncerrorR = crate::BitReader;
#[doc = "Field `RXEOTSYNCERROR` writer - (RW1C) Set to 1 if End of transmission synchronisation Error is reported in the acknowledgment packet by the display device"]
pub type RxeotsyncerrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXESCAPEMODE` reader - (RW1C) Entry Error; Set to 1 if Escape Mode Entry command is not understandable by the display device and is reported in the Acknowledge packet by the display device."]
pub type RxescapemodeR = crate::BitReader;
#[doc = "Field `RXESCAPEMODE` writer - (RW1C) Entry Error; Set to 1 if Escape Mode Entry command is not understandable by the display device and is reported in the Acknowledge packet by the display device."]
pub type RxescapemodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXLPTXSYNCERR` reader - (RW1C) Rx LP tx sync error; Set to 1 if Low power transmission sync error occurs in the display device and is reported in the Acknowledge packet by the display device"]
pub type RxlptxsyncerrR = crate::BitReader;
#[doc = "Field `RXLPTXSYNCERR` writer - (RW1C) Rx LP tx sync error; Set to 1 if Low power transmission sync error occurs in the display device and is reported in the Acknowledge packet by the display device"]
pub type RxlptxsyncerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXPERIPHERAL` reader - (RW1C) Rx Peripheral timeout Error; Set to 1 if the high speed receive timer value or LP Tx timer value are expired, display device is reported in the Acknowledge packet"]
pub type RxperipheralR = crate::BitReader;
#[doc = "Field `RXPERIPHERAL` writer - (RW1C) Rx Peripheral timeout Error; Set to 1 if the high speed receive timer value or LP Tx timer value are expired, display device is reported in the Acknowledge packet"]
pub type RxperipheralW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFALSECNTRL` reader - (RW1C) RxFalse Control Error; Set to 1 if a control error is reported in the acknowledge packet by the display device"]
pub type RxfalsecntrlR = crate::BitReader;
#[doc = "Field `RXFALSECNTRL` writer - (RW1C) RxFalse Control Error; Set to 1 if a control error is reported in the acknowledge packet by the display device"]
pub type RxfalsecntrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RxECCS` reader - (RW1C) RRxECC single bit error; Set to 1 if ECC syndrome was computed and corrected for one bit error is reported in the Acknowledge packet by the display device"]
pub type RxEccsR = crate::BitReader;
#[doc = "Field `RxECCS` writer - (RW1C) RRxECC single bit error; Set to 1 if ECC syndrome was computed and corrected for one bit error is reported in the Acknowledge packet by the display device"]
pub type RxEccsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RxECCM` reader - (RW1C) RxECC multibit error; Set to 1 if there is no ECC correction for the packet or there are more than 2 bit errors in the packet isreported in the Acknowledge packet by the display device"]
pub type RxEccmR = crate::BitReader;
#[doc = "Field `RxECCM` writer - (RW1C) RxECC multibit error; Set to 1 if there is no ECC correction for the packet or there are more than 2 bit errors in the packet isreported in the Acknowledge packet by the display device"]
pub type RxEccmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXCHECKSUM` reader - (RW1C) Set to 1 if the computed CRC differs from the received CRC value and is reported in the acknowledge packet by the display device"]
pub type RxchecksumR = crate::BitReader;
#[doc = "Field `RXCHECKSUM` writer - (RW1C) Set to 1 if the computed CRC differs from the received CRC value and is reported in the acknowledge packet by the display device"]
pub type RxchecksumW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RxDSINR` reader - (RW1C) RxDSI data type not recognised; Set to 1 if the data type is not recognised by the display device is reported in the Acknowledge packet by the display device"]
pub type RxDsinrR = crate::BitReader;
#[doc = "Field `RxDSINR` writer - (RW1C) RxDSI data type not recognised; Set to 1 if the data type is not recognised by the display device is reported in the Acknowledge packet by the display device"]
pub type RxDsinrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RxDSIDI` reader - (RW1C) RxDSI VC ID invalid; Set to 1 if the virtual channel ID is invalid by the display device is reported in the Acknowledge packet by the display device"]
pub type RxDsidiR = crate::BitReader;
#[doc = "Field `RxDSIDI` writer - (RW1C) RxDSI VC ID invalid; Set to 1 if the virtual channel ID is invalid by the display device is reported in the Acknowledge packet by the display device"]
pub type RxDsidiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFALSECNTRL` reader - (RW1C) TxFalse Control Error; Set to 1 if a control error is observed on the lanes by the Arasan_DSI_host"]
pub type TxfalsecntrlR = crate::BitReader;
#[doc = "Field `TXFALSECNTRL` writer - (RW1C) TxFalse Control Error; Set to 1 if a control error is observed on the lanes by the Arasan_DSI_host"]
pub type TxfalsecntrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXECCS` reader - (RW1C) Set to 1 if ECC syndrome was computed and is corrected for one bit error during the reception of packets by the Arasan_DSI_host."]
pub type TxeccsR = crate::BitReader;
#[doc = "Field `TXECCS` writer - (RW1C) Set to 1 if ECC syndrome was computed and is corrected for one bit error during the reception of packets by the Arasan_DSI_host."]
pub type TxeccsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXECCM` reader - (RW1C) Set to 1 if there is no ECC correction for the packet or there are more than 2 bit errors in the packet received by Arasan_DSI_host."]
pub type TxeccmR = crate::BitReader;
#[doc = "Field `TXECCM` writer - (RW1C) Set to 1 if there is no ECC correction for the packet or there are more than 2 bit errors in the packet received by Arasan_DSI_host."]
pub type TxeccmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXCHECKSUM` reader - (RW1C) Txchecksum error; Set to 1 if the computed CRC differs from the received CRC value during the reception of packets by Arasan_DSI host"]
pub type TxchecksumR = crate::BitReader;
#[doc = "Field `TXCHECKSUM` writer - (RW1C) Txchecksum error; Set to 1 if the computed CRC differs from the received CRC value during the reception of packets by Arasan_DSI host"]
pub type TxchecksumW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TxDSIN` reader - (RW1C) TxDSI data type not recognised; Set to 1 if the received data type is not recognised"]
pub type TxDsinR = crate::BitReader;
#[doc = "Field `TxDSIN` writer - (RW1C) TxDSI data type not recognised; Set to 1 if the received data type is not recognised"]
pub type TxDsinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TxDSII` reader - (RW1C) TxDSI VC ID invalid; Set to 1 if the received virtual channel ID is invalid"]
pub type TxDsiiR = crate::BitReader;
#[doc = "Field `TxDSII` writer - (RW1C) TxDSI VC ID invalid; Set to 1 if the received virtual channel ID is invalid"]
pub type TxDsiiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HIGHC` reader - (RW1C) High contention;Set to 1 if a LP high fault is registered by at the D-PHY contention detector. If this interrupt is set device should be re-enumerated"]
pub type HighcR = crate::BitReader;
#[doc = "Field `HIGHC` writer - (RW1C) High contention;Set to 1 if a LP high fault is registered by at the D-PHY contention detector. If this interrupt is set device should be re-enumerated"]
pub type HighcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOWC` reader - (RW1C) Low contention; Set to 1 if a LP low fault is registered by at the D-PHY contention detector. If this interrupt is set device should be re-enumerated"]
pub type LowcR = crate::BitReader;
#[doc = "Field `LOWC` writer - (RW1C) Low contention; Set to 1 if a LP low fault is registered by at the D-PHY contention detector. If this interrupt is set device should be re-enumerated"]
pub type LowcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFOEMPTY` reader - (RW1C) Set to 1 if all FIFOs are empty"]
pub type FifoemptyR = crate::BitReader;
#[doc = "Field `FIFOEMPTY` writer - (RW1C) Set to 1 if all FIFOs are empty"]
pub type FifoemptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSTXTIMEOUT` reader - (RW1C) Set if a high speed transmission prevails for more than the expected count value this interrupt is raised"]
pub type HstxtimeoutR = crate::BitReader;
#[doc = "Field `HSTXTIMEOUT` writer - (RW1C) Set if a high speed transmission prevails for more than the expected count value this interrupt is raised"]
pub type HstxtimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPRXTIMEOUT` reader - (RW1C) Set if a low power reception count expires this interrupt is generated"]
pub type LprxtimeoutR = crate::BitReader;
#[doc = "Field `LPRXTIMEOUT` writer - (RW1C) Set if a low power reception count expires this interrupt is generated"]
pub type LprxtimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TURNARNDACK` reader - (RW1C) Turn around acknowledge. Set if a turn around acknowledgement sequence is timeout not received from the display device"]
pub type TurnarndackR = crate::BitReader;
#[doc = "Field `TURNARNDACK` writer - (RW1C) Turn around acknowledge. Set if a turn around acknowledgement sequence is timeout not received from the display device"]
pub type TurnarndackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACKWNOERR` reader - (RW1C) T ACK_with No_error; Set if acknowledge trigger message is received with out any error."]
pub type AckwnoerrR = crate::BitReader;
#[doc = "Field `ACKWNOERR` writer - (RW1C) T ACK_with No_error; Set if acknowledge trigger message is received with out any error."]
pub type AckwnoerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXINVALID` reader - (RW1C) Rx Invalid; Set if acknowledge short packet shows an invalid transmission count"]
pub type RxinvalidR = crate::BitReader;
#[doc = "Field `RXINVALID` writer - (RW1C) Rx Invalid; Set if acknowledge short packet shows an invalid transmission count"]
pub type RxinvalidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXDSIPROT` reader - (RW1C) Rx DSI protocol violation; Set if acknowledge short packet shows DSI protocol violation error"]
pub type RxdsiprotR = crate::BitReader;
#[doc = "Field `RXDSIPROT` writer - (RW1C) Rx DSI protocol violation; Set if acknowledge short packet shows DSI protocol violation error"]
pub type RxdsiprotW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPECIALPACK` reader - (RW1C) Special packet command sent; Set to confirm the transmission of the DPI event specific commands set in the dpi control and dpi data"]
pub type SpecialpackR = crate::BitReader;
#[doc = "Field `SPECIALPACK` writer - (RW1C) Special packet command sent; Set to confirm the transmission of the DPI event specific commands set in the dpi control and dpi data"]
pub type SpecialpackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INITDONE` reader - (RW1C) Set 1 indicates that the DSI initialization is done. DSI Tx is ready to accept the DPI or DBI or Generic transfer"]
pub type InitdoneR = crate::BitReader;
#[doc = "Field `INITDONE` writer - (RW1C) Set 1 indicates that the DSI initialization is done. DSI Tx is ready to accept the DPI or DBI or Generic transfer"]
pub type InitdoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXCNT` reader - (RW1C) Rx Contention; Set to 1 if contention detected in the display"]
pub type RxcntR = crate::BitReader;
#[doc = "Field `RXCNT` writer - (RW1C) Rx Contention; Set to 1 if contention detected in the display"]
pub type RxcntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPILINETO` reader - (RW1C) DPI line time out. Set to 1 indicates that the line time out during the DPI transfer"]
pub type DpilinetoR = crate::BitReader;
#[doc = "Field `DPILINETO` writer - (RW1C) DPI line time out. Set to 1 indicates that the line time out during the DPI transfer"]
pub type DpilinetoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPIPRGERR` reader - (RW1C) Set to 1 indicates that the error in DPI parameters programming"]
pub type DpiprgerrR = crate::BitReader;
#[doc = "Field `DPIPRGERR` writer - (RW1C) Set to 1 indicates that the error in DPI parameters programming"]
pub type DpiprgerrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - (RW1C) Set to 1 if a start of transmission sequence error is reported in the Acknowledge packet by the display device"]
    #[inline(always)]
    pub fn rxsoterror(&self) -> RxsoterrorR {
        RxsoterrorR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - (RW1C) Set to 1 if synchronisation error occurrence in the start of transmission sequence is reported in the acknowledge packet by the display device"]
    #[inline(always)]
    pub fn rxsotsyncerror(&self) -> RxsotsyncerrorR {
        RxsotsyncerrorR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - (RW1C) Set to 1 if End of transmission synchronisation Error is reported in the acknowledgment packet by the display device"]
    #[inline(always)]
    pub fn rxeotsyncerror(&self) -> RxeotsyncerrorR {
        RxeotsyncerrorR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - (RW1C) Entry Error; Set to 1 if Escape Mode Entry command is not understandable by the display device and is reported in the Acknowledge packet by the display device."]
    #[inline(always)]
    pub fn rxescapemode(&self) -> RxescapemodeR {
        RxescapemodeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - (RW1C) Rx LP tx sync error; Set to 1 if Low power transmission sync error occurs in the display device and is reported in the Acknowledge packet by the display device"]
    #[inline(always)]
    pub fn rxlptxsyncerr(&self) -> RxlptxsyncerrR {
        RxlptxsyncerrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - (RW1C) Rx Peripheral timeout Error; Set to 1 if the high speed receive timer value or LP Tx timer value are expired, display device is reported in the Acknowledge packet"]
    #[inline(always)]
    pub fn rxperipheral(&self) -> RxperipheralR {
        RxperipheralR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - (RW1C) RxFalse Control Error; Set to 1 if a control error is reported in the acknowledge packet by the display device"]
    #[inline(always)]
    pub fn rxfalsecntrl(&self) -> RxfalsecntrlR {
        RxfalsecntrlR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - (RW1C) RRxECC single bit error; Set to 1 if ECC syndrome was computed and corrected for one bit error is reported in the Acknowledge packet by the display device"]
    #[inline(always)]
    pub fn rx_eccs(&self) -> RxEccsR {
        RxEccsR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - (RW1C) RxECC multibit error; Set to 1 if there is no ECC correction for the packet or there are more than 2 bit errors in the packet isreported in the Acknowledge packet by the display device"]
    #[inline(always)]
    pub fn rx_eccm(&self) -> RxEccmR {
        RxEccmR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - (RW1C) Set to 1 if the computed CRC differs from the received CRC value and is reported in the acknowledge packet by the display device"]
    #[inline(always)]
    pub fn rxchecksum(&self) -> RxchecksumR {
        RxchecksumR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - (RW1C) RxDSI data type not recognised; Set to 1 if the data type is not recognised by the display device is reported in the Acknowledge packet by the display device"]
    #[inline(always)]
    pub fn rx_dsinr(&self) -> RxDsinrR {
        RxDsinrR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - (RW1C) RxDSI VC ID invalid; Set to 1 if the virtual channel ID is invalid by the display device is reported in the Acknowledge packet by the display device"]
    #[inline(always)]
    pub fn rx_dsidi(&self) -> RxDsidiR {
        RxDsidiR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - (RW1C) TxFalse Control Error; Set to 1 if a control error is observed on the lanes by the Arasan_DSI_host"]
    #[inline(always)]
    pub fn txfalsecntrl(&self) -> TxfalsecntrlR {
        TxfalsecntrlR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - (RW1C) Set to 1 if ECC syndrome was computed and is corrected for one bit error during the reception of packets by the Arasan_DSI_host."]
    #[inline(always)]
    pub fn txeccs(&self) -> TxeccsR {
        TxeccsR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - (RW1C) Set to 1 if there is no ECC correction for the packet or there are more than 2 bit errors in the packet received by Arasan_DSI_host."]
    #[inline(always)]
    pub fn txeccm(&self) -> TxeccmR {
        TxeccmR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - (RW1C) Txchecksum error; Set to 1 if the computed CRC differs from the received CRC value during the reception of packets by Arasan_DSI host"]
    #[inline(always)]
    pub fn txchecksum(&self) -> TxchecksumR {
        TxchecksumR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - (RW1C) TxDSI data type not recognised; Set to 1 if the received data type is not recognised"]
    #[inline(always)]
    pub fn tx_dsin(&self) -> TxDsinR {
        TxDsinR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - (RW1C) TxDSI VC ID invalid; Set to 1 if the received virtual channel ID is invalid"]
    #[inline(always)]
    pub fn tx_dsii(&self) -> TxDsiiR {
        TxDsiiR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - (RW1C) High contention;Set to 1 if a LP high fault is registered by at the D-PHY contention detector. If this interrupt is set device should be re-enumerated"]
    #[inline(always)]
    pub fn highc(&self) -> HighcR {
        HighcR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - (RW1C) Low contention; Set to 1 if a LP low fault is registered by at the D-PHY contention detector. If this interrupt is set device should be re-enumerated"]
    #[inline(always)]
    pub fn lowc(&self) -> LowcR {
        LowcR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - (RW1C) Set to 1 if all FIFOs are empty"]
    #[inline(always)]
    pub fn fifoempty(&self) -> FifoemptyR {
        FifoemptyR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - (RW1C) Set if a high speed transmission prevails for more than the expected count value this interrupt is raised"]
    #[inline(always)]
    pub fn hstxtimeout(&self) -> HstxtimeoutR {
        HstxtimeoutR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - (RW1C) Set if a low power reception count expires this interrupt is generated"]
    #[inline(always)]
    pub fn lprxtimeout(&self) -> LprxtimeoutR {
        LprxtimeoutR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - (RW1C) Turn around acknowledge. Set if a turn around acknowledgement sequence is timeout not received from the display device"]
    #[inline(always)]
    pub fn turnarndack(&self) -> TurnarndackR {
        TurnarndackR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - (RW1C) T ACK_with No_error; Set if acknowledge trigger message is received with out any error."]
    #[inline(always)]
    pub fn ackwnoerr(&self) -> AckwnoerrR {
        AckwnoerrR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - (RW1C) Rx Invalid; Set if acknowledge short packet shows an invalid transmission count"]
    #[inline(always)]
    pub fn rxinvalid(&self) -> RxinvalidR {
        RxinvalidR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - (RW1C) Rx DSI protocol violation; Set if acknowledge short packet shows DSI protocol violation error"]
    #[inline(always)]
    pub fn rxdsiprot(&self) -> RxdsiprotR {
        RxdsiprotR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - (RW1C) Special packet command sent; Set to confirm the transmission of the DPI event specific commands set in the dpi control and dpi data"]
    #[inline(always)]
    pub fn specialpack(&self) -> SpecialpackR {
        SpecialpackR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - (RW1C) Set 1 indicates that the DSI initialization is done. DSI Tx is ready to accept the DPI or DBI or Generic transfer"]
    #[inline(always)]
    pub fn initdone(&self) -> InitdoneR {
        InitdoneR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - (RW1C) Rx Contention; Set to 1 if contention detected in the display"]
    #[inline(always)]
    pub fn rxcnt(&self) -> RxcntR {
        RxcntR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - (RW1C) DPI line time out. Set to 1 indicates that the line time out during the DPI transfer"]
    #[inline(always)]
    pub fn dpilineto(&self) -> DpilinetoR {
        DpilinetoR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - (RW1C) Set to 1 indicates that the error in DPI parameters programming"]
    #[inline(always)]
    pub fn dpiprgerr(&self) -> DpiprgerrR {
        DpiprgerrR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - (RW1C) Set to 1 if a start of transmission sequence error is reported in the Acknowledge packet by the display device"]
    #[inline(always)]
    #[must_use]
    pub fn rxsoterror(&mut self) -> RxsoterrorW<IntrstatSpec> {
        RxsoterrorW::new(self, 0)
    }
    #[doc = "Bit 1 - (RW1C) Set to 1 if synchronisation error occurrence in the start of transmission sequence is reported in the acknowledge packet by the display device"]
    #[inline(always)]
    #[must_use]
    pub fn rxsotsyncerror(&mut self) -> RxsotsyncerrorW<IntrstatSpec> {
        RxsotsyncerrorW::new(self, 1)
    }
    #[doc = "Bit 2 - (RW1C) Set to 1 if End of transmission synchronisation Error is reported in the acknowledgment packet by the display device"]
    #[inline(always)]
    #[must_use]
    pub fn rxeotsyncerror(&mut self) -> RxeotsyncerrorW<IntrstatSpec> {
        RxeotsyncerrorW::new(self, 2)
    }
    #[doc = "Bit 3 - (RW1C) Entry Error; Set to 1 if Escape Mode Entry command is not understandable by the display device and is reported in the Acknowledge packet by the display device."]
    #[inline(always)]
    #[must_use]
    pub fn rxescapemode(&mut self) -> RxescapemodeW<IntrstatSpec> {
        RxescapemodeW::new(self, 3)
    }
    #[doc = "Bit 4 - (RW1C) Rx LP tx sync error; Set to 1 if Low power transmission sync error occurs in the display device and is reported in the Acknowledge packet by the display device"]
    #[inline(always)]
    #[must_use]
    pub fn rxlptxsyncerr(&mut self) -> RxlptxsyncerrW<IntrstatSpec> {
        RxlptxsyncerrW::new(self, 4)
    }
    #[doc = "Bit 5 - (RW1C) Rx Peripheral timeout Error; Set to 1 if the high speed receive timer value or LP Tx timer value are expired, display device is reported in the Acknowledge packet"]
    #[inline(always)]
    #[must_use]
    pub fn rxperipheral(&mut self) -> RxperipheralW<IntrstatSpec> {
        RxperipheralW::new(self, 5)
    }
    #[doc = "Bit 6 - (RW1C) RxFalse Control Error; Set to 1 if a control error is reported in the acknowledge packet by the display device"]
    #[inline(always)]
    #[must_use]
    pub fn rxfalsecntrl(&mut self) -> RxfalsecntrlW<IntrstatSpec> {
        RxfalsecntrlW::new(self, 6)
    }
    #[doc = "Bit 7 - (RW1C) RRxECC single bit error; Set to 1 if ECC syndrome was computed and corrected for one bit error is reported in the Acknowledge packet by the display device"]
    #[inline(always)]
    #[must_use]
    pub fn rx_eccs(&mut self) -> RxEccsW<IntrstatSpec> {
        RxEccsW::new(self, 7)
    }
    #[doc = "Bit 8 - (RW1C) RxECC multibit error; Set to 1 if there is no ECC correction for the packet or there are more than 2 bit errors in the packet isreported in the Acknowledge packet by the display device"]
    #[inline(always)]
    #[must_use]
    pub fn rx_eccm(&mut self) -> RxEccmW<IntrstatSpec> {
        RxEccmW::new(self, 8)
    }
    #[doc = "Bit 9 - (RW1C) Set to 1 if the computed CRC differs from the received CRC value and is reported in the acknowledge packet by the display device"]
    #[inline(always)]
    #[must_use]
    pub fn rxchecksum(&mut self) -> RxchecksumW<IntrstatSpec> {
        RxchecksumW::new(self, 9)
    }
    #[doc = "Bit 10 - (RW1C) RxDSI data type not recognised; Set to 1 if the data type is not recognised by the display device is reported in the Acknowledge packet by the display device"]
    #[inline(always)]
    #[must_use]
    pub fn rx_dsinr(&mut self) -> RxDsinrW<IntrstatSpec> {
        RxDsinrW::new(self, 10)
    }
    #[doc = "Bit 11 - (RW1C) RxDSI VC ID invalid; Set to 1 if the virtual channel ID is invalid by the display device is reported in the Acknowledge packet by the display device"]
    #[inline(always)]
    #[must_use]
    pub fn rx_dsidi(&mut self) -> RxDsidiW<IntrstatSpec> {
        RxDsidiW::new(self, 11)
    }
    #[doc = "Bit 12 - (RW1C) TxFalse Control Error; Set to 1 if a control error is observed on the lanes by the Arasan_DSI_host"]
    #[inline(always)]
    #[must_use]
    pub fn txfalsecntrl(&mut self) -> TxfalsecntrlW<IntrstatSpec> {
        TxfalsecntrlW::new(self, 12)
    }
    #[doc = "Bit 13 - (RW1C) Set to 1 if ECC syndrome was computed and is corrected for one bit error during the reception of packets by the Arasan_DSI_host."]
    #[inline(always)]
    #[must_use]
    pub fn txeccs(&mut self) -> TxeccsW<IntrstatSpec> {
        TxeccsW::new(self, 13)
    }
    #[doc = "Bit 14 - (RW1C) Set to 1 if there is no ECC correction for the packet or there are more than 2 bit errors in the packet received by Arasan_DSI_host."]
    #[inline(always)]
    #[must_use]
    pub fn txeccm(&mut self) -> TxeccmW<IntrstatSpec> {
        TxeccmW::new(self, 14)
    }
    #[doc = "Bit 15 - (RW1C) Txchecksum error; Set to 1 if the computed CRC differs from the received CRC value during the reception of packets by Arasan_DSI host"]
    #[inline(always)]
    #[must_use]
    pub fn txchecksum(&mut self) -> TxchecksumW<IntrstatSpec> {
        TxchecksumW::new(self, 15)
    }
    #[doc = "Bit 16 - (RW1C) TxDSI data type not recognised; Set to 1 if the received data type is not recognised"]
    #[inline(always)]
    #[must_use]
    pub fn tx_dsin(&mut self) -> TxDsinW<IntrstatSpec> {
        TxDsinW::new(self, 16)
    }
    #[doc = "Bit 17 - (RW1C) TxDSI VC ID invalid; Set to 1 if the received virtual channel ID is invalid"]
    #[inline(always)]
    #[must_use]
    pub fn tx_dsii(&mut self) -> TxDsiiW<IntrstatSpec> {
        TxDsiiW::new(self, 17)
    }
    #[doc = "Bit 18 - (RW1C) High contention;Set to 1 if a LP high fault is registered by at the D-PHY contention detector. If this interrupt is set device should be re-enumerated"]
    #[inline(always)]
    #[must_use]
    pub fn highc(&mut self) -> HighcW<IntrstatSpec> {
        HighcW::new(self, 18)
    }
    #[doc = "Bit 19 - (RW1C) Low contention; Set to 1 if a LP low fault is registered by at the D-PHY contention detector. If this interrupt is set device should be re-enumerated"]
    #[inline(always)]
    #[must_use]
    pub fn lowc(&mut self) -> LowcW<IntrstatSpec> {
        LowcW::new(self, 19)
    }
    #[doc = "Bit 20 - (RW1C) Set to 1 if all FIFOs are empty"]
    #[inline(always)]
    #[must_use]
    pub fn fifoempty(&mut self) -> FifoemptyW<IntrstatSpec> {
        FifoemptyW::new(self, 20)
    }
    #[doc = "Bit 21 - (RW1C) Set if a high speed transmission prevails for more than the expected count value this interrupt is raised"]
    #[inline(always)]
    #[must_use]
    pub fn hstxtimeout(&mut self) -> HstxtimeoutW<IntrstatSpec> {
        HstxtimeoutW::new(self, 21)
    }
    #[doc = "Bit 22 - (RW1C) Set if a low power reception count expires this interrupt is generated"]
    #[inline(always)]
    #[must_use]
    pub fn lprxtimeout(&mut self) -> LprxtimeoutW<IntrstatSpec> {
        LprxtimeoutW::new(self, 22)
    }
    #[doc = "Bit 23 - (RW1C) Turn around acknowledge. Set if a turn around acknowledgement sequence is timeout not received from the display device"]
    #[inline(always)]
    #[must_use]
    pub fn turnarndack(&mut self) -> TurnarndackW<IntrstatSpec> {
        TurnarndackW::new(self, 23)
    }
    #[doc = "Bit 24 - (RW1C) T ACK_with No_error; Set if acknowledge trigger message is received with out any error."]
    #[inline(always)]
    #[must_use]
    pub fn ackwnoerr(&mut self) -> AckwnoerrW<IntrstatSpec> {
        AckwnoerrW::new(self, 24)
    }
    #[doc = "Bit 25 - (RW1C) Rx Invalid; Set if acknowledge short packet shows an invalid transmission count"]
    #[inline(always)]
    #[must_use]
    pub fn rxinvalid(&mut self) -> RxinvalidW<IntrstatSpec> {
        RxinvalidW::new(self, 25)
    }
    #[doc = "Bit 26 - (RW1C) Rx DSI protocol violation; Set if acknowledge short packet shows DSI protocol violation error"]
    #[inline(always)]
    #[must_use]
    pub fn rxdsiprot(&mut self) -> RxdsiprotW<IntrstatSpec> {
        RxdsiprotW::new(self, 26)
    }
    #[doc = "Bit 27 - (RW1C) Special packet command sent; Set to confirm the transmission of the DPI event specific commands set in the dpi control and dpi data"]
    #[inline(always)]
    #[must_use]
    pub fn specialpack(&mut self) -> SpecialpackW<IntrstatSpec> {
        SpecialpackW::new(self, 27)
    }
    #[doc = "Bit 28 - (RW1C) Set 1 indicates that the DSI initialization is done. DSI Tx is ready to accept the DPI or DBI or Generic transfer"]
    #[inline(always)]
    #[must_use]
    pub fn initdone(&mut self) -> InitdoneW<IntrstatSpec> {
        InitdoneW::new(self, 28)
    }
    #[doc = "Bit 29 - (RW1C) Rx Contention; Set to 1 if contention detected in the display"]
    #[inline(always)]
    #[must_use]
    pub fn rxcnt(&mut self) -> RxcntW<IntrstatSpec> {
        RxcntW::new(self, 29)
    }
    #[doc = "Bit 30 - (RW1C) DPI line time out. Set to 1 indicates that the line time out during the DPI transfer"]
    #[inline(always)]
    #[must_use]
    pub fn dpilineto(&mut self) -> DpilinetoW<IntrstatSpec> {
        DpilinetoW::new(self, 30)
    }
    #[doc = "Bit 31 - (RW1C) Set to 1 indicates that the error in DPI parameters programming"]
    #[inline(always)]
    #[must_use]
    pub fn dpiprgerr(&mut self) -> DpiprgerrW<IntrstatSpec> {
        DpiprgerrW::new(self, 31)
    }
}
#[doc = "The interrupt status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`intrstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intrstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrstatSpec;
impl crate::RegisterSpec for IntrstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intrstat::R`](R) reader structure"]
impl crate::Readable for IntrstatSpec {}
#[doc = "`write(|w| ..)` method takes [`intrstat::W`](W) writer structure"]
impl crate::Writable for IntrstatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTRSTAT to value 0"]
impl crate::Resettable for IntrstatSpec {
    const RESET_VALUE: u32 = 0;
}
