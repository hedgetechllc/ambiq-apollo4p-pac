#[doc = "Register `INTREN` reader"]
pub type R = crate::R<IntrenSpec>;
#[doc = "Register `INTREN` writer"]
pub type W = crate::W<IntrenSpec>;
#[doc = "Field `RXSOTERROR` reader - RX start of transmission; set to enable the interrupt for start of transmission"]
pub type RxsoterrorR = crate::BitReader;
#[doc = "Field `RXSOTERROR` writer - RX start of transmission; set to enable the interrupt for start of transmission"]
pub type RxsoterrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXSOTSYNCERROR` reader - RX start of transmission; Set to enable the interrupt for start of transmission synchronization error in the acknowledgement packet reports"]
pub type RxsotsyncerrorR = crate::BitReader;
#[doc = "Field `RXSOTSYNCERROR` writer - RX start of transmission; Set to enable the interrupt for start of transmission synchronization error in the acknowledgement packet reports"]
pub type RxsotsyncerrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXEOTSYNCRR` reader - RxEot Sync Error l set to enable the interrupt for the end of transmission synchronisation Error in the acknowledgment packet reports"]
pub type RxeotsyncrrR = crate::BitReader;
#[doc = "Field `RXEOTSYNCRR` writer - RxEot Sync Error l set to enable the interrupt for the end of transmission synchronisation Error in the acknowledgment packet reports"]
pub type RxeotsyncrrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXESCPMDETRYERR` reader - RxEscape Mode Entry Error; Set to enable the interrupt for Escape Mode Entry command error in the acknowledgment packet reports"]
pub type RxescpmdetryerrR = crate::BitReader;
#[doc = "Field `RXESCPMDETRYERR` writer - RxEscape Mode Entry Error; Set to enable the interrupt for Escape Mode Entry command error in the acknowledgment packet reports"]
pub type RxescpmdetryerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXLPTXSYNCERR` reader - Rx LP tx sync error; Set to enable the interrupt for Low power transmission sync error in the acknowledgment packet reports"]
pub type RxlptxsyncerrR = crate::BitReader;
#[doc = "Field `RXLPTXSYNCERR` writer - Rx LP tx sync error; Set to enable the interrupt for Low power transmission sync error in the acknowledgment packet reports"]
pub type RxlptxsyncerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXPERIPHRCVTOE` reader - Peripheral receive timeout Error; Set to enable the interrupt for the high speed timeout Error or Lp tx timeout error in the acknowledgment packet reports"]
pub type RxperiphrcvtoeR = crate::BitReader;
#[doc = "Field `RXPERIPHRCVTOE` writer - Peripheral receive timeout Error; Set to enable the interrupt for the high speed timeout Error or Lp tx timeout error in the acknowledgment packet reports"]
pub type RxperiphrcvtoeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFALSE` reader - RxFalse Control error; set to enable the interrupt for control error in the acknowledgment packet reports."]
pub type RxfalseR = crate::BitReader;
#[doc = "Field `RXFALSE` writer - RxFalse Control error; set to enable the interrupt for control error in the acknowledgment packet reports."]
pub type RxfalseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXECCS` reader - RxECC single bit error; Set to enable the interrupt for ECC syndrome computation and one bit error correction for the acknowledgment packet"]
pub type RxeccsR = crate::BitReader;
#[doc = "Field `RXECCS` writer - RxECC single bit error; Set to enable the interrupt for ECC syndrome computation and one bit error correction for the acknowledgment packet"]
pub type RxeccsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXECCM` reader - RxECC multibit error; Set to enable the interrupt for no ECC correction for the packet or there are more than 2 bit errors reported in the acknowledgment packet"]
pub type RxeccmR = crate::BitReader;
#[doc = "Field `RXECCM` writer - RxECC multibit error; Set to enable the interrupt for no ECC correction for the packet or there are more than 2 bit errors reported in the acknowledgment packet"]
pub type RxeccmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXCHECKSUM` reader - Rxchecksum error; Set to enable the interrupt for the computed CRC differs from the received CRC value in the acknowledgment packet reports"]
pub type RxchecksumR = crate::BitReader;
#[doc = "Field `RXCHECKSUM` writer - Rxchecksum error; Set to enable the interrupt for the computed CRC differs from the received CRC value in the acknowledgment packet reports"]
pub type RxchecksumW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RxDSIData` reader - RxDSI data type not recognised; Set to enable the interrupt for the un recognised data type in the acknowledgment packet reports"]
pub type RxDsidataR = crate::BitReader;
#[doc = "Field `RxDSIData` writer - RxDSI data type not recognised; Set to enable the interrupt for the un recognised data type in the acknowledgment packet reports"]
pub type RxDsidataW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RxDSIV` reader - RxDSI VC ID invalid virtual channel; Set to enable the interrupt for invalid ID in the acknowledgment packet reports"]
pub type RxDsivR = crate::BitReader;
#[doc = "Field `RxDSIV` writer - RxDSI VC ID invalid virtual channel; Set to enable the interrupt for invalid ID in the acknowledgment packet reports"]
pub type RxDsivW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TxFalseCntrl` reader - TxFalse Control; Set to enable the interrupt for the control error observed on the lanes by the Arasan_DSI_host"]
pub type TxFalseCntrlR = crate::BitReader;
#[doc = "Field `TxFalseCntrl` writer - TxFalse Control; Set to enable the interrupt for the control error observed on the lanes by the Arasan_DSI_host"]
pub type TxFalseCntrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TxECCS` reader - TxECC single bit; Set to enable the interrupt if ECC syndrome was computed and is corrected for one bit error during the reception of packets by the Arasan DSI Host"]
pub type TxEccsR = crate::BitReader;
#[doc = "Field `TxECCS` writer - TxECC single bit; Set to enable the interrupt if ECC syndrome was computed and is corrected for one bit error during the reception of packets by the Arasan DSI Host"]
pub type TxEccsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TxECCM` reader - TxECC multibit; Set to enable the interrupt if there is no ECC correction for the packet or there are more than 2 bit errors in the packet received by Arasan DSI host"]
pub type TxEccmR = crate::BitReader;
#[doc = "Field `TxECCM` writer - TxECC multibit; Set to enable the interrupt if there is no ECC correction for the packet or there are more than 2 bit errors in the packet received by Arasan DSI host"]
pub type TxEccmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXCHCKSUM` reader - Txchecksum error; Set to enable the interrupt if the computed CRC differs from the received CRC value for the received packets"]
pub type TxchcksumR = crate::BitReader;
#[doc = "Field `TXCHCKSUM` writer - Txchecksum error; Set to enable the interrupt if the computed CRC differs from the received CRC value for the received packets"]
pub type TxchcksumW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TxDSID` reader - TxDSI data type not recognised; Set to enable the interrupt if the received packets data type is not recognised"]
pub type TxDsidR = crate::BitReader;
#[doc = "Field `TxDSID` writer - TxDSI data type not recognised; Set to enable the interrupt if the received packets data type is not recognised"]
pub type TxDsidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TxDSIV` reader - TxDSI VC ID invalid; Set to enable the interrupt if the received packets virtual channel ID is invalid"]
pub type TxDsivR = crate::BitReader;
#[doc = "Field `TxDSIV` writer - TxDSI VC ID invalid; Set to enable the interrupt if the received packets virtual channel ID is invalid"]
pub type TxDsivW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HIGHC` reader - High contention; Set to enable a LP high fault interrupt"]
pub type HighcR = crate::BitReader;
#[doc = "Field `HIGHC` writer - High contention; Set to enable a LP high fault interrupt"]
pub type HighcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOWC` reader - Low contention; Set to enable a LP low fault interrupt"]
pub type LowcR = crate::BitReader;
#[doc = "Field `LOWC` writer - Low contention; Set to enable a LP low fault interrupt"]
pub type LowcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFOEMPTY` reader - Set to enable a FIFO empty interrupt"]
pub type FifoemptyR = crate::BitReader;
#[doc = "Field `FIFOEMPTY` writer - Set to enable a FIFO empty interrupt"]
pub type FifoemptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSTXTIMEOUT` reader - Set to enable a high speed transmission timeout"]
pub type HstxtimeoutR = crate::BitReader;
#[doc = "Field `HSTXTIMEOUT` writer - Set to enable a high speed transmission timeout"]
pub type HstxtimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPRXTIMEOUT` reader - Set to enable low power reception count timeouts"]
pub type LprxtimeoutR = crate::BitReader;
#[doc = "Field `LPRXTIMEOUT` writer - Set to enable low power reception count timeouts"]
pub type LprxtimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TURNARNDACK` reader - Set to enable turn around acknowledgement sequence timeout"]
pub type TurnarndackR = crate::BitReader;
#[doc = "Field `TURNARNDACK` writer - Set to enable turn around acknowledgement sequence timeout"]
pub type TurnarndackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACKWITHNOERR` reader - ACK with No_error; Set to enable acknowledge trigger message reception with out any error"]
pub type AckwithnoerrR = crate::BitReader;
#[doc = "Field `ACKWITHNOERR` writer - ACK with No_error; Set to enable acknowledge trigger message reception with out any error"]
pub type AckwithnoerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXINV` reader - Rx Invalid transmission count error; Set to enable acknowledge invalid transmission counterror"]
pub type RxinvR = crate::BitReader;
#[doc = "Field `RXINV` writer - Rx Invalid transmission count error; Set to enable acknowledge invalid transmission counterror"]
pub type RxinvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXDSI` reader - Rx DSI protocol violation; Set to enable DSI protocol violation error"]
pub type RxdsiR = crate::BitReader;
#[doc = "Field `RXDSI` writer - Rx DSI protocol violation; Set to enable DSI protocol violation error"]
pub type RxdsiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPECIALPACK` reader - Special packet command sent; Set to enable the confirmation interrupt for transmitting DPI events set in the dpi data and dpi control registers"]
pub type SpecialpackR = crate::BitReader;
#[doc = "Field `SPECIALPACK` writer - Special packet command sent; Set to enable the confirmation interrupt for transmitting DPI events set in the dpi data and dpi control registers"]
pub type SpecialpackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INITDONE` reader - Set 1 indicates that the DSI initialisation is done DSI Tx is ready to accept the DPI or DBI or Generic transfer"]
pub type InitdoneR = crate::BitReader;
#[doc = "Field `INITDONE` writer - Set 1 indicates that the DSI initialisation is done DSI Tx is ready to accept the DPI or DBI or Generic transfer"]
pub type InitdoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXCONTENT` reader - Detected Rx Contention Detected; Set to enable the interrupt for contention detected error in the acknowledgment packet reports"]
pub type RxcontentR = crate::BitReader;
#[doc = "Field `RXCONTENT` writer - Detected Rx Contention Detected; Set to enable the interrupt for contention detected error in the acknowledgment packet reports"]
pub type RxcontentW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPILINETO` reader - Dpi line timeout; Set to 1 indicates that the line time out during the DPI transfer"]
pub type DpilinetoR = crate::BitReader;
#[doc = "Field `DPILINETO` writer - Dpi line timeout; Set to 1 indicates that the line time out during the DPI transfer"]
pub type DpilinetoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPI` reader - PGRMERR DPI program error; Set to 1 indicates that the error in DPI parameters programming"]
pub type DpiR = crate::BitReader;
#[doc = "Field `DPI` writer - PGRMERR DPI program error; Set to 1 indicates that the error in DPI parameters programming"]
pub type DpiW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RX start of transmission; set to enable the interrupt for start of transmission"]
    #[inline(always)]
    pub fn rxsoterror(&self) -> RxsoterrorR {
        RxsoterrorR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RX start of transmission; Set to enable the interrupt for start of transmission synchronization error in the acknowledgement packet reports"]
    #[inline(always)]
    pub fn rxsotsyncerror(&self) -> RxsotsyncerrorR {
        RxsotsyncerrorR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RxEot Sync Error l set to enable the interrupt for the end of transmission synchronisation Error in the acknowledgment packet reports"]
    #[inline(always)]
    pub fn rxeotsyncrr(&self) -> RxeotsyncrrR {
        RxeotsyncrrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RxEscape Mode Entry Error; Set to enable the interrupt for Escape Mode Entry command error in the acknowledgment packet reports"]
    #[inline(always)]
    pub fn rxescpmdetryerr(&self) -> RxescpmdetryerrR {
        RxescpmdetryerrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Rx LP tx sync error; Set to enable the interrupt for Low power transmission sync error in the acknowledgment packet reports"]
    #[inline(always)]
    pub fn rxlptxsyncerr(&self) -> RxlptxsyncerrR {
        RxlptxsyncerrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Peripheral receive timeout Error; Set to enable the interrupt for the high speed timeout Error or Lp tx timeout error in the acknowledgment packet reports"]
    #[inline(always)]
    pub fn rxperiphrcvtoe(&self) -> RxperiphrcvtoeR {
        RxperiphrcvtoeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RxFalse Control error; set to enable the interrupt for control error in the acknowledgment packet reports."]
    #[inline(always)]
    pub fn rxfalse(&self) -> RxfalseR {
        RxfalseR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RxECC single bit error; Set to enable the interrupt for ECC syndrome computation and one bit error correction for the acknowledgment packet"]
    #[inline(always)]
    pub fn rxeccs(&self) -> RxeccsR {
        RxeccsR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - RxECC multibit error; Set to enable the interrupt for no ECC correction for the packet or there are more than 2 bit errors reported in the acknowledgment packet"]
    #[inline(always)]
    pub fn rxeccm(&self) -> RxeccmR {
        RxeccmR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Rxchecksum error; Set to enable the interrupt for the computed CRC differs from the received CRC value in the acknowledgment packet reports"]
    #[inline(always)]
    pub fn rxchecksum(&self) -> RxchecksumR {
        RxchecksumR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - RxDSI data type not recognised; Set to enable the interrupt for the un recognised data type in the acknowledgment packet reports"]
    #[inline(always)]
    pub fn rx_dsidata(&self) -> RxDsidataR {
        RxDsidataR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - RxDSI VC ID invalid virtual channel; Set to enable the interrupt for invalid ID in the acknowledgment packet reports"]
    #[inline(always)]
    pub fn rx_dsiv(&self) -> RxDsivR {
        RxDsivR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - TxFalse Control; Set to enable the interrupt for the control error observed on the lanes by the Arasan_DSI_host"]
    #[inline(always)]
    pub fn tx_false_cntrl(&self) -> TxFalseCntrlR {
        TxFalseCntrlR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TxECC single bit; Set to enable the interrupt if ECC syndrome was computed and is corrected for one bit error during the reception of packets by the Arasan DSI Host"]
    #[inline(always)]
    pub fn tx_eccs(&self) -> TxEccsR {
        TxEccsR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - TxECC multibit; Set to enable the interrupt if there is no ECC correction for the packet or there are more than 2 bit errors in the packet received by Arasan DSI host"]
    #[inline(always)]
    pub fn tx_eccm(&self) -> TxEccmR {
        TxEccmR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Txchecksum error; Set to enable the interrupt if the computed CRC differs from the received CRC value for the received packets"]
    #[inline(always)]
    pub fn txchcksum(&self) -> TxchcksumR {
        TxchcksumR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - TxDSI data type not recognised; Set to enable the interrupt if the received packets data type is not recognised"]
    #[inline(always)]
    pub fn tx_dsid(&self) -> TxDsidR {
        TxDsidR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TxDSI VC ID invalid; Set to enable the interrupt if the received packets virtual channel ID is invalid"]
    #[inline(always)]
    pub fn tx_dsiv(&self) -> TxDsivR {
        TxDsivR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - High contention; Set to enable a LP high fault interrupt"]
    #[inline(always)]
    pub fn highc(&self) -> HighcR {
        HighcR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Low contention; Set to enable a LP low fault interrupt"]
    #[inline(always)]
    pub fn lowc(&self) -> LowcR {
        LowcR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Set to enable a FIFO empty interrupt"]
    #[inline(always)]
    pub fn fifoempty(&self) -> FifoemptyR {
        FifoemptyR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Set to enable a high speed transmission timeout"]
    #[inline(always)]
    pub fn hstxtimeout(&self) -> HstxtimeoutR {
        HstxtimeoutR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Set to enable low power reception count timeouts"]
    #[inline(always)]
    pub fn lprxtimeout(&self) -> LprxtimeoutR {
        LprxtimeoutR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Set to enable turn around acknowledgement sequence timeout"]
    #[inline(always)]
    pub fn turnarndack(&self) -> TurnarndackR {
        TurnarndackR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - ACK with No_error; Set to enable acknowledge trigger message reception with out any error"]
    #[inline(always)]
    pub fn ackwithnoerr(&self) -> AckwithnoerrR {
        AckwithnoerrR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Rx Invalid transmission count error; Set to enable acknowledge invalid transmission counterror"]
    #[inline(always)]
    pub fn rxinv(&self) -> RxinvR {
        RxinvR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Rx DSI protocol violation; Set to enable DSI protocol violation error"]
    #[inline(always)]
    pub fn rxdsi(&self) -> RxdsiR {
        RxdsiR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Special packet command sent; Set to enable the confirmation interrupt for transmitting DPI events set in the dpi data and dpi control registers"]
    #[inline(always)]
    pub fn specialpack(&self) -> SpecialpackR {
        SpecialpackR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Set 1 indicates that the DSI initialisation is done DSI Tx is ready to accept the DPI or DBI or Generic transfer"]
    #[inline(always)]
    pub fn initdone(&self) -> InitdoneR {
        InitdoneR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Detected Rx Contention Detected; Set to enable the interrupt for contention detected error in the acknowledgment packet reports"]
    #[inline(always)]
    pub fn rxcontent(&self) -> RxcontentR {
        RxcontentR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Dpi line timeout; Set to 1 indicates that the line time out during the DPI transfer"]
    #[inline(always)]
    pub fn dpilineto(&self) -> DpilinetoR {
        DpilinetoR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - PGRMERR DPI program error; Set to 1 indicates that the error in DPI parameters programming"]
    #[inline(always)]
    pub fn dpi(&self) -> DpiR {
        DpiR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RX start of transmission; set to enable the interrupt for start of transmission"]
    #[inline(always)]
    #[must_use]
    pub fn rxsoterror(&mut self) -> RxsoterrorW<IntrenSpec> {
        RxsoterrorW::new(self, 0)
    }
    #[doc = "Bit 1 - RX start of transmission; Set to enable the interrupt for start of transmission synchronization error in the acknowledgement packet reports"]
    #[inline(always)]
    #[must_use]
    pub fn rxsotsyncerror(&mut self) -> RxsotsyncerrorW<IntrenSpec> {
        RxsotsyncerrorW::new(self, 1)
    }
    #[doc = "Bit 2 - RxEot Sync Error l set to enable the interrupt for the end of transmission synchronisation Error in the acknowledgment packet reports"]
    #[inline(always)]
    #[must_use]
    pub fn rxeotsyncrr(&mut self) -> RxeotsyncrrW<IntrenSpec> {
        RxeotsyncrrW::new(self, 2)
    }
    #[doc = "Bit 3 - RxEscape Mode Entry Error; Set to enable the interrupt for Escape Mode Entry command error in the acknowledgment packet reports"]
    #[inline(always)]
    #[must_use]
    pub fn rxescpmdetryerr(&mut self) -> RxescpmdetryerrW<IntrenSpec> {
        RxescpmdetryerrW::new(self, 3)
    }
    #[doc = "Bit 4 - Rx LP tx sync error; Set to enable the interrupt for Low power transmission sync error in the acknowledgment packet reports"]
    #[inline(always)]
    #[must_use]
    pub fn rxlptxsyncerr(&mut self) -> RxlptxsyncerrW<IntrenSpec> {
        RxlptxsyncerrW::new(self, 4)
    }
    #[doc = "Bit 5 - Peripheral receive timeout Error; Set to enable the interrupt for the high speed timeout Error or Lp tx timeout error in the acknowledgment packet reports"]
    #[inline(always)]
    #[must_use]
    pub fn rxperiphrcvtoe(&mut self) -> RxperiphrcvtoeW<IntrenSpec> {
        RxperiphrcvtoeW::new(self, 5)
    }
    #[doc = "Bit 6 - RxFalse Control error; set to enable the interrupt for control error in the acknowledgment packet reports."]
    #[inline(always)]
    #[must_use]
    pub fn rxfalse(&mut self) -> RxfalseW<IntrenSpec> {
        RxfalseW::new(self, 6)
    }
    #[doc = "Bit 7 - RxECC single bit error; Set to enable the interrupt for ECC syndrome computation and one bit error correction for the acknowledgment packet"]
    #[inline(always)]
    #[must_use]
    pub fn rxeccs(&mut self) -> RxeccsW<IntrenSpec> {
        RxeccsW::new(self, 7)
    }
    #[doc = "Bit 8 - RxECC multibit error; Set to enable the interrupt for no ECC correction for the packet or there are more than 2 bit errors reported in the acknowledgment packet"]
    #[inline(always)]
    #[must_use]
    pub fn rxeccm(&mut self) -> RxeccmW<IntrenSpec> {
        RxeccmW::new(self, 8)
    }
    #[doc = "Bit 9 - Rxchecksum error; Set to enable the interrupt for the computed CRC differs from the received CRC value in the acknowledgment packet reports"]
    #[inline(always)]
    #[must_use]
    pub fn rxchecksum(&mut self) -> RxchecksumW<IntrenSpec> {
        RxchecksumW::new(self, 9)
    }
    #[doc = "Bit 10 - RxDSI data type not recognised; Set to enable the interrupt for the un recognised data type in the acknowledgment packet reports"]
    #[inline(always)]
    #[must_use]
    pub fn rx_dsidata(&mut self) -> RxDsidataW<IntrenSpec> {
        RxDsidataW::new(self, 10)
    }
    #[doc = "Bit 11 - RxDSI VC ID invalid virtual channel; Set to enable the interrupt for invalid ID in the acknowledgment packet reports"]
    #[inline(always)]
    #[must_use]
    pub fn rx_dsiv(&mut self) -> RxDsivW<IntrenSpec> {
        RxDsivW::new(self, 11)
    }
    #[doc = "Bit 12 - TxFalse Control; Set to enable the interrupt for the control error observed on the lanes by the Arasan_DSI_host"]
    #[inline(always)]
    #[must_use]
    pub fn tx_false_cntrl(&mut self) -> TxFalseCntrlW<IntrenSpec> {
        TxFalseCntrlW::new(self, 12)
    }
    #[doc = "Bit 13 - TxECC single bit; Set to enable the interrupt if ECC syndrome was computed and is corrected for one bit error during the reception of packets by the Arasan DSI Host"]
    #[inline(always)]
    #[must_use]
    pub fn tx_eccs(&mut self) -> TxEccsW<IntrenSpec> {
        TxEccsW::new(self, 13)
    }
    #[doc = "Bit 14 - TxECC multibit; Set to enable the interrupt if there is no ECC correction for the packet or there are more than 2 bit errors in the packet received by Arasan DSI host"]
    #[inline(always)]
    #[must_use]
    pub fn tx_eccm(&mut self) -> TxEccmW<IntrenSpec> {
        TxEccmW::new(self, 14)
    }
    #[doc = "Bit 15 - Txchecksum error; Set to enable the interrupt if the computed CRC differs from the received CRC value for the received packets"]
    #[inline(always)]
    #[must_use]
    pub fn txchcksum(&mut self) -> TxchcksumW<IntrenSpec> {
        TxchcksumW::new(self, 15)
    }
    #[doc = "Bit 16 - TxDSI data type not recognised; Set to enable the interrupt if the received packets data type is not recognised"]
    #[inline(always)]
    #[must_use]
    pub fn tx_dsid(&mut self) -> TxDsidW<IntrenSpec> {
        TxDsidW::new(self, 16)
    }
    #[doc = "Bit 17 - TxDSI VC ID invalid; Set to enable the interrupt if the received packets virtual channel ID is invalid"]
    #[inline(always)]
    #[must_use]
    pub fn tx_dsiv(&mut self) -> TxDsivW<IntrenSpec> {
        TxDsivW::new(self, 17)
    }
    #[doc = "Bit 18 - High contention; Set to enable a LP high fault interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn highc(&mut self) -> HighcW<IntrenSpec> {
        HighcW::new(self, 18)
    }
    #[doc = "Bit 19 - Low contention; Set to enable a LP low fault interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn lowc(&mut self) -> LowcW<IntrenSpec> {
        LowcW::new(self, 19)
    }
    #[doc = "Bit 20 - Set to enable a FIFO empty interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn fifoempty(&mut self) -> FifoemptyW<IntrenSpec> {
        FifoemptyW::new(self, 20)
    }
    #[doc = "Bit 21 - Set to enable a high speed transmission timeout"]
    #[inline(always)]
    #[must_use]
    pub fn hstxtimeout(&mut self) -> HstxtimeoutW<IntrenSpec> {
        HstxtimeoutW::new(self, 21)
    }
    #[doc = "Bit 22 - Set to enable low power reception count timeouts"]
    #[inline(always)]
    #[must_use]
    pub fn lprxtimeout(&mut self) -> LprxtimeoutW<IntrenSpec> {
        LprxtimeoutW::new(self, 22)
    }
    #[doc = "Bit 23 - Set to enable turn around acknowledgement sequence timeout"]
    #[inline(always)]
    #[must_use]
    pub fn turnarndack(&mut self) -> TurnarndackW<IntrenSpec> {
        TurnarndackW::new(self, 23)
    }
    #[doc = "Bit 24 - ACK with No_error; Set to enable acknowledge trigger message reception with out any error"]
    #[inline(always)]
    #[must_use]
    pub fn ackwithnoerr(&mut self) -> AckwithnoerrW<IntrenSpec> {
        AckwithnoerrW::new(self, 24)
    }
    #[doc = "Bit 25 - Rx Invalid transmission count error; Set to enable acknowledge invalid transmission counterror"]
    #[inline(always)]
    #[must_use]
    pub fn rxinv(&mut self) -> RxinvW<IntrenSpec> {
        RxinvW::new(self, 25)
    }
    #[doc = "Bit 26 - Rx DSI protocol violation; Set to enable DSI protocol violation error"]
    #[inline(always)]
    #[must_use]
    pub fn rxdsi(&mut self) -> RxdsiW<IntrenSpec> {
        RxdsiW::new(self, 26)
    }
    #[doc = "Bit 27 - Special packet command sent; Set to enable the confirmation interrupt for transmitting DPI events set in the dpi data and dpi control registers"]
    #[inline(always)]
    #[must_use]
    pub fn specialpack(&mut self) -> SpecialpackW<IntrenSpec> {
        SpecialpackW::new(self, 27)
    }
    #[doc = "Bit 28 - Set 1 indicates that the DSI initialisation is done DSI Tx is ready to accept the DPI or DBI or Generic transfer"]
    #[inline(always)]
    #[must_use]
    pub fn initdone(&mut self) -> InitdoneW<IntrenSpec> {
        InitdoneW::new(self, 28)
    }
    #[doc = "Bit 29 - Detected Rx Contention Detected; Set to enable the interrupt for contention detected error in the acknowledgment packet reports"]
    #[inline(always)]
    #[must_use]
    pub fn rxcontent(&mut self) -> RxcontentW<IntrenSpec> {
        RxcontentW::new(self, 29)
    }
    #[doc = "Bit 30 - Dpi line timeout; Set to 1 indicates that the line time out during the DPI transfer"]
    #[inline(always)]
    #[must_use]
    pub fn dpilineto(&mut self) -> DpilinetoW<IntrenSpec> {
        DpilinetoW::new(self, 30)
    }
    #[doc = "Bit 31 - PGRMERR DPI program error; Set to 1 indicates that the error in DPI parameters programming"]
    #[inline(always)]
    #[must_use]
    pub fn dpi(&mut self) -> DpiW<IntrenSpec> {
        DpiW::new(self, 31)
    }
}
#[doc = "Interrupt enable register.\n\nYou can [`read`](crate::Reg::read) this register and get [`intren::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intren::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrenSpec;
impl crate::RegisterSpec for IntrenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intren::R`](R) reader structure"]
impl crate::Readable for IntrenSpec {}
#[doc = "`write(|w| ..)` method takes [`intren::W`](W) writer structure"]
impl crate::Writable for IntrenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTREN to value 0"]
impl crate::Resettable for IntrenSpec {
    const RESET_VALUE: u32 = 0;
}
