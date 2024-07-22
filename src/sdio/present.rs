#[doc = "Register `PRESENT` reader"]
pub type R = crate::R<PresentSpec>;
#[doc = "Register `PRESENT` writer"]
pub type W = crate::W<PresentSpec>;
#[doc = "If this bit is 0, it indicates the CMD line is not in use and the HC can issue a SD command using the CMD line. This bit is set immediately after the Command register (00Fh) is written. This bit is cleared when the command response is received. Even if the Command Inhibit (DAT) is set to 1, Commands using only the CMD line can be issued if this bit is 0. Changing from 1 to 0 generates a Command complete interrupt in the Normal Interrupt Status register. If the HC cannot issue the command because of a command conflict error or because of Command Not Issued By Auto CMD12 Error, this bit shall remain 1 and the Command Complete is not set. Status issuing Auto CMD12 is not read from this bit. Auto CMD12 and Auto CMD23 consist of two responses. In this case, this bit is not cleared by the response of CMD12 or CMD23 but cleared by the response of a read/write command. Status issuing Auto CMD12 is not read from this bit. So if a command is issued during Auto CMD12 operation, Host Controller shall manage to issue two commands: CMD12 and a command set by Command register. Note: DAT line active indicates whether one of the DAT line is on SD bus is in use. (a) In the case of read transactions This status indicates whether a read transfer is executing on the SD Bus. Changing this value from 1 to 0 generates a Block Gap Event interrupt in the Normal Interrupt Status register, as the result of the Stop At Block Gap Request being set. This bit shall be set in either of the following cases: (1) After the end bit of the read command. (2) When writing a 1 to Continue Request in the Block Gap Control register to restart a read transfer. This bit shall be cleared in either of the following cases: (1) When the end bit of the last data block is sent from the SD Bus to the Host Controller. In case of ADMA2, the last block is designated by the last transfer of Descriptor Table. (2) When a read transfer is stopped at the block gap initiated by a Stop At Block Gap Request. The Host Controller shall stop read operation at the start of the interrupt cycle of the next block gap by driving Read Wait or stopping SD clock. If the Read Wait signal is already driven (due to data buffer cannot receive data), the Host Controller can continue to stop read operation by driving the Read Wait signal. It is necessary to support Read Wait in order to use suspend / resume function. (b) In the case of write transactions This status indicates that a write transfer is executing on the SD Bus. Changing this value from 1 to 0 generate a Transfer Complete interrupt in the Normal Interrupt Status register. This bit shall be set in either of the following cases: (1) After the end bit of the write command. (2) When writing to 1 to Continue Request in the Block Gap Control register to continue a write transfer. This bit shall be cleared in either of the following cases: (1) When the SD card releases write busy of the last data block. If SD card does not drive busy signal for 8 SD Clocks, the Host Controller shall consider the card drive Not Busy. In case of ADMA2, the last block is designated by the last transfer of Descriptor Table. (2) When the SD card releases write busy prior to waiting for write transfer as a result of a Stop At Block Gap Request. (c) Command with busy This status indicates whether a command indicates busy (ex. erase command for memory) is executing on the SD Bus. This bit is set after the end bit of the command with busy and cleared when busy is de-asserted. Changing this bit from 1 to 0 generate a Transfer Complete interrupt in the Normal Interrupt Status register. Note: The HD can issue cmd0, cmd12, cmd13 (for memory) and cmd52 (for SDIO) when the DAT lines are busy during data transfer. These commands can be issued when Command Inhibit (CMD) is set to zero. Other commands shall be issued when Command Inhibit (DAT) is set to zero.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmdinhcmd {
    #[doc = "1: CMD line is in use"]
    Dontissue = 1,
    #[doc = "0: Indicates that the CMD line is not in use and the HC can issue a SD command using the CMD line."]
    Issue = 0,
}
impl From<Cmdinhcmd> for bool {
    #[inline(always)]
    fn from(variant: Cmdinhcmd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMDINHCMD` reader - If this bit is 0, it indicates the CMD line is not in use and the HC can issue a SD command using the CMD line. This bit is set immediately after the Command register (00Fh) is written. This bit is cleared when the command response is received. Even if the Command Inhibit (DAT) is set to 1, Commands using only the CMD line can be issued if this bit is 0. Changing from 1 to 0 generates a Command complete interrupt in the Normal Interrupt Status register. If the HC cannot issue the command because of a command conflict error or because of Command Not Issued By Auto CMD12 Error, this bit shall remain 1 and the Command Complete is not set. Status issuing Auto CMD12 is not read from this bit. Auto CMD12 and Auto CMD23 consist of two responses. In this case, this bit is not cleared by the response of CMD12 or CMD23 but cleared by the response of a read/write command. Status issuing Auto CMD12 is not read from this bit. So if a command is issued during Auto CMD12 operation, Host Controller shall manage to issue two commands: CMD12 and a command set by Command register. Note: DAT line active indicates whether one of the DAT line is on SD bus is in use. (a) In the case of read transactions This status indicates whether a read transfer is executing on the SD Bus. Changing this value from 1 to 0 generates a Block Gap Event interrupt in the Normal Interrupt Status register, as the result of the Stop At Block Gap Request being set. This bit shall be set in either of the following cases: (1) After the end bit of the read command. (2) When writing a 1 to Continue Request in the Block Gap Control register to restart a read transfer. This bit shall be cleared in either of the following cases: (1) When the end bit of the last data block is sent from the SD Bus to the Host Controller. In case of ADMA2, the last block is designated by the last transfer of Descriptor Table. (2) When a read transfer is stopped at the block gap initiated by a Stop At Block Gap Request. The Host Controller shall stop read operation at the start of the interrupt cycle of the next block gap by driving Read Wait or stopping SD clock. If the Read Wait signal is already driven (due to data buffer cannot receive data), the Host Controller can continue to stop read operation by driving the Read Wait signal. It is necessary to support Read Wait in order to use suspend / resume function. (b) In the case of write transactions This status indicates that a write transfer is executing on the SD Bus. Changing this value from 1 to 0 generate a Transfer Complete interrupt in the Normal Interrupt Status register. This bit shall be set in either of the following cases: (1) After the end bit of the write command. (2) When writing to 1 to Continue Request in the Block Gap Control register to continue a write transfer. This bit shall be cleared in either of the following cases: (1) When the SD card releases write busy of the last data block. If SD card does not drive busy signal for 8 SD Clocks, the Host Controller shall consider the card drive Not Busy. In case of ADMA2, the last block is designated by the last transfer of Descriptor Table. (2) When the SD card releases write busy prior to waiting for write transfer as a result of a Stop At Block Gap Request. (c) Command with busy This status indicates whether a command indicates busy (ex. erase command for memory) is executing on the SD Bus. This bit is set after the end bit of the command with busy and cleared when busy is de-asserted. Changing this bit from 1 to 0 generate a Transfer Complete interrupt in the Normal Interrupt Status register. Note: The HD can issue cmd0, cmd12, cmd13 (for memory) and cmd52 (for SDIO) when the DAT lines are busy during data transfer. These commands can be issued when Command Inhibit (CMD) is set to zero. Other commands shall be issued when Command Inhibit (DAT) is set to zero."]
pub type CmdinhcmdR = crate::BitReader<Cmdinhcmd>;
impl CmdinhcmdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmdinhcmd {
        match self.bits {
            true => Cmdinhcmd::Dontissue,
            false => Cmdinhcmd::Issue,
        }
    }
    #[doc = "CMD line is in use"]
    #[inline(always)]
    pub fn is_dontissue(&self) -> bool {
        *self == Cmdinhcmd::Dontissue
    }
    #[doc = "Indicates that the CMD line is not in use and the HC can issue a SD command using the CMD line."]
    #[inline(always)]
    pub fn is_issue(&self) -> bool {
        *self == Cmdinhcmd::Issue
    }
}
#[doc = "Field `CMDINHCMD` writer - If this bit is 0, it indicates the CMD line is not in use and the HC can issue a SD command using the CMD line. This bit is set immediately after the Command register (00Fh) is written. This bit is cleared when the command response is received. Even if the Command Inhibit (DAT) is set to 1, Commands using only the CMD line can be issued if this bit is 0. Changing from 1 to 0 generates a Command complete interrupt in the Normal Interrupt Status register. If the HC cannot issue the command because of a command conflict error or because of Command Not Issued By Auto CMD12 Error, this bit shall remain 1 and the Command Complete is not set. Status issuing Auto CMD12 is not read from this bit. Auto CMD12 and Auto CMD23 consist of two responses. In this case, this bit is not cleared by the response of CMD12 or CMD23 but cleared by the response of a read/write command. Status issuing Auto CMD12 is not read from this bit. So if a command is issued during Auto CMD12 operation, Host Controller shall manage to issue two commands: CMD12 and a command set by Command register. Note: DAT line active indicates whether one of the DAT line is on SD bus is in use. (a) In the case of read transactions This status indicates whether a read transfer is executing on the SD Bus. Changing this value from 1 to 0 generates a Block Gap Event interrupt in the Normal Interrupt Status register, as the result of the Stop At Block Gap Request being set. This bit shall be set in either of the following cases: (1) After the end bit of the read command. (2) When writing a 1 to Continue Request in the Block Gap Control register to restart a read transfer. This bit shall be cleared in either of the following cases: (1) When the end bit of the last data block is sent from the SD Bus to the Host Controller. In case of ADMA2, the last block is designated by the last transfer of Descriptor Table. (2) When a read transfer is stopped at the block gap initiated by a Stop At Block Gap Request. The Host Controller shall stop read operation at the start of the interrupt cycle of the next block gap by driving Read Wait or stopping SD clock. If the Read Wait signal is already driven (due to data buffer cannot receive data), the Host Controller can continue to stop read operation by driving the Read Wait signal. It is necessary to support Read Wait in order to use suspend / resume function. (b) In the case of write transactions This status indicates that a write transfer is executing on the SD Bus. Changing this value from 1 to 0 generate a Transfer Complete interrupt in the Normal Interrupt Status register. This bit shall be set in either of the following cases: (1) After the end bit of the write command. (2) When writing to 1 to Continue Request in the Block Gap Control register to continue a write transfer. This bit shall be cleared in either of the following cases: (1) When the SD card releases write busy of the last data block. If SD card does not drive busy signal for 8 SD Clocks, the Host Controller shall consider the card drive Not Busy. In case of ADMA2, the last block is designated by the last transfer of Descriptor Table. (2) When the SD card releases write busy prior to waiting for write transfer as a result of a Stop At Block Gap Request. (c) Command with busy This status indicates whether a command indicates busy (ex. erase command for memory) is executing on the SD Bus. This bit is set after the end bit of the command with busy and cleared when busy is de-asserted. Changing this bit from 1 to 0 generate a Transfer Complete interrupt in the Normal Interrupt Status register. Note: The HD can issue cmd0, cmd12, cmd13 (for memory) and cmd52 (for SDIO) when the DAT lines are busy during data transfer. These commands can be issued when Command Inhibit (CMD) is set to zero. Other commands shall be issued when Command Inhibit (DAT) is set to zero."]
pub type CmdinhcmdW<'a, REG> = crate::BitWriter<'a, REG, Cmdinhcmd>;
impl<'a, REG> CmdinhcmdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CMD line is in use"]
    #[inline(always)]
    pub fn dontissue(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdinhcmd::Dontissue)
    }
    #[doc = "Indicates that the CMD line is not in use and the HC can issue a SD command using the CMD line."]
    #[inline(always)]
    pub fn issue(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdinhcmd::Issue)
    }
}
#[doc = "This status bit is generated if either the DAT Line Active or the Read transfer Active is set to 1. If this bit is 0, it indicates the HC can issue the next SD command. Commands with busy signal belong to Command Inhibit (DAT) (ex. R1b, R5b type). Changing from 1 to 0 generates a Transfer Complete interrupt in the Normal interrupt status register. Note: The SD Host Driver can save registers in the range of 000-00Dh for a suspend transaction after this bit has changed from 1 to 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmdinhdat {
    #[doc = "1: cannot issue command which uses the DAT line"]
    Dontissue = 1,
    #[doc = "0: Can issue command which uses the DAT line"]
    Issue = 0,
}
impl From<Cmdinhdat> for bool {
    #[inline(always)]
    fn from(variant: Cmdinhdat) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMDINHDAT` reader - This status bit is generated if either the DAT Line Active or the Read transfer Active is set to 1. If this bit is 0, it indicates the HC can issue the next SD command. Commands with busy signal belong to Command Inhibit (DAT) (ex. R1b, R5b type). Changing from 1 to 0 generates a Transfer Complete interrupt in the Normal interrupt status register. Note: The SD Host Driver can save registers in the range of 000-00Dh for a suspend transaction after this bit has changed from 1 to 0."]
pub type CmdinhdatR = crate::BitReader<Cmdinhdat>;
impl CmdinhdatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmdinhdat {
        match self.bits {
            true => Cmdinhdat::Dontissue,
            false => Cmdinhdat::Issue,
        }
    }
    #[doc = "cannot issue command which uses the DAT line"]
    #[inline(always)]
    pub fn is_dontissue(&self) -> bool {
        *self == Cmdinhdat::Dontissue
    }
    #[doc = "Can issue command which uses the DAT line"]
    #[inline(always)]
    pub fn is_issue(&self) -> bool {
        *self == Cmdinhdat::Issue
    }
}
#[doc = "Field `CMDINHDAT` writer - This status bit is generated if either the DAT Line Active or the Read transfer Active is set to 1. If this bit is 0, it indicates the HC can issue the next SD command. Commands with busy signal belong to Command Inhibit (DAT) (ex. R1b, R5b type). Changing from 1 to 0 generates a Transfer Complete interrupt in the Normal interrupt status register. Note: The SD Host Driver can save registers in the range of 000-00Dh for a suspend transaction after this bit has changed from 1 to 0."]
pub type CmdinhdatW<'a, REG> = crate::BitWriter<'a, REG, Cmdinhdat>;
impl<'a, REG> CmdinhdatW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "cannot issue command which uses the DAT line"]
    #[inline(always)]
    pub fn dontissue(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdinhdat::Dontissue)
    }
    #[doc = "Can issue command which uses the DAT line"]
    #[inline(always)]
    pub fn issue(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdinhdat::Issue)
    }
}
#[doc = "This bit indicates whether one of the DAT line on SD bus is in use.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dlineact {
    #[doc = "1: DAT line active"]
    Active = 1,
    #[doc = "0: DAT line inactive"]
    Inactive = 0,
}
impl From<Dlineact> for bool {
    #[inline(always)]
    fn from(variant: Dlineact) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DLINEACT` reader - This bit indicates whether one of the DAT line on SD bus is in use."]
pub type DlineactR = crate::BitReader<Dlineact>;
impl DlineactR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dlineact {
        match self.bits {
            true => Dlineact::Active,
            false => Dlineact::Inactive,
        }
    }
    #[doc = "DAT line active"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Dlineact::Active
    }
    #[doc = "DAT line inactive"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == Dlineact::Inactive
    }
}
#[doc = "Field `DLINEACT` writer - This bit indicates whether one of the DAT line on SD bus is in use."]
pub type DlineactW<'a, REG> = crate::BitWriter<'a, REG, Dlineact>;
impl<'a, REG> DlineactW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DAT line active"]
    #[inline(always)]
    pub fn active(self) -> &'a mut crate::W<REG> {
        self.variant(Dlineact::Active)
    }
    #[doc = "DAT line inactive"]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(Dlineact::Inactive)
    }
}
#[doc = "Re-Tuning Request Host Controller may request Host Driver to execute re-tuning sequence by setting this bit when the data window is shifted by temperature drift and a tuned sampling point does not have a good margin to receive correct data. This bit is cleared when a command is issued with setting Execute Tuning in the Host Control 2 register. Changing of this bit from 0 to 1 generates Re-Tuning Event. Refer to Normal Interrupt registers for more detail. This bit isn't set to 1 if Sampling Clock Select in the Host Control 2 register is set to 0 (using fixed sampling clock).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Retuningrequest {
    #[doc = "1: Sampling clock needs re-tuning"]
    Retuneneeded = 1,
    #[doc = "0: Fixed or well tuned sampling clock"]
    Welltuned = 0,
}
impl From<Retuningrequest> for bool {
    #[inline(always)]
    fn from(variant: Retuningrequest) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RETUNINGREQUEST` reader - Re-Tuning Request Host Controller may request Host Driver to execute re-tuning sequence by setting this bit when the data window is shifted by temperature drift and a tuned sampling point does not have a good margin to receive correct data. This bit is cleared when a command is issued with setting Execute Tuning in the Host Control 2 register. Changing of this bit from 0 to 1 generates Re-Tuning Event. Refer to Normal Interrupt registers for more detail. This bit isn't set to 1 if Sampling Clock Select in the Host Control 2 register is set to 0 (using fixed sampling clock)."]
pub type RetuningrequestR = crate::BitReader<Retuningrequest>;
impl RetuningrequestR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Retuningrequest {
        match self.bits {
            true => Retuningrequest::Retuneneeded,
            false => Retuningrequest::Welltuned,
        }
    }
    #[doc = "Sampling clock needs re-tuning"]
    #[inline(always)]
    pub fn is_retuneneeded(&self) -> bool {
        *self == Retuningrequest::Retuneneeded
    }
    #[doc = "Fixed or well tuned sampling clock"]
    #[inline(always)]
    pub fn is_welltuned(&self) -> bool {
        *self == Retuningrequest::Welltuned
    }
}
#[doc = "Field `RETUNINGREQUEST` writer - Re-Tuning Request Host Controller may request Host Driver to execute re-tuning sequence by setting this bit when the data window is shifted by temperature drift and a tuned sampling point does not have a good margin to receive correct data. This bit is cleared when a command is issued with setting Execute Tuning in the Host Control 2 register. Changing of this bit from 0 to 1 generates Re-Tuning Event. Refer to Normal Interrupt registers for more detail. This bit isn't set to 1 if Sampling Clock Select in the Host Control 2 register is set to 0 (using fixed sampling clock)."]
pub type RetuningrequestW<'a, REG> = crate::BitWriter<'a, REG, Retuningrequest>;
impl<'a, REG> RetuningrequestW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Sampling clock needs re-tuning"]
    #[inline(always)]
    pub fn retuneneeded(self) -> &'a mut crate::W<REG> {
        self.variant(Retuningrequest::Retuneneeded)
    }
    #[doc = "Fixed or well tuned sampling clock"]
    #[inline(always)]
    pub fn welltuned(self) -> &'a mut crate::W<REG> {
        self.variant(Retuningrequest::Welltuned)
    }
}
#[doc = "This status indicates a write transfer is active. If this bit is 0, it means no valid write data exists in the HC. This bit is set in either of the following cases: After the end bit of the write command. When writing a 1 to Continue Request in the Block Gap Control register to restart a write transfer. This bit is cleared in either of the following cases: After getting the CRC status of the last data block as specified by the transfer count (Single or Multiple) After getting a CRC status of any block where data transmission is about to be stopped by a Stop At Block Gap Request. During a write transaction, a Block Gap Event interrupt is generated when this bit is changed to 0, as a result of the Stop At Block Gap Request being set. This status is useful for the HD in determining when to issue commands during write busy.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wrxferact {
    #[doc = "1: transferring data"]
    Transferring = 1,
    #[doc = "0: No valid data"]
    Novaliddata = 0,
}
impl From<Wrxferact> for bool {
    #[inline(always)]
    fn from(variant: Wrxferact) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WRXFERACT` reader - This status indicates a write transfer is active. If this bit is 0, it means no valid write data exists in the HC. This bit is set in either of the following cases: After the end bit of the write command. When writing a 1 to Continue Request in the Block Gap Control register to restart a write transfer. This bit is cleared in either of the following cases: After getting the CRC status of the last data block as specified by the transfer count (Single or Multiple) After getting a CRC status of any block where data transmission is about to be stopped by a Stop At Block Gap Request. During a write transaction, a Block Gap Event interrupt is generated when this bit is changed to 0, as a result of the Stop At Block Gap Request being set. This status is useful for the HD in determining when to issue commands during write busy."]
pub type WrxferactR = crate::BitReader<Wrxferact>;
impl WrxferactR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wrxferact {
        match self.bits {
            true => Wrxferact::Transferring,
            false => Wrxferact::Novaliddata,
        }
    }
    #[doc = "transferring data"]
    #[inline(always)]
    pub fn is_transferring(&self) -> bool {
        *self == Wrxferact::Transferring
    }
    #[doc = "No valid data"]
    #[inline(always)]
    pub fn is_novaliddata(&self) -> bool {
        *self == Wrxferact::Novaliddata
    }
}
#[doc = "Field `WRXFERACT` writer - This status indicates a write transfer is active. If this bit is 0, it means no valid write data exists in the HC. This bit is set in either of the following cases: After the end bit of the write command. When writing a 1 to Continue Request in the Block Gap Control register to restart a write transfer. This bit is cleared in either of the following cases: After getting the CRC status of the last data block as specified by the transfer count (Single or Multiple) After getting a CRC status of any block where data transmission is about to be stopped by a Stop At Block Gap Request. During a write transaction, a Block Gap Event interrupt is generated when this bit is changed to 0, as a result of the Stop At Block Gap Request being set. This status is useful for the HD in determining when to issue commands during write busy."]
pub type WrxferactW<'a, REG> = crate::BitWriter<'a, REG, Wrxferact>;
impl<'a, REG> WrxferactW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "transferring data"]
    #[inline(always)]
    pub fn transferring(self) -> &'a mut crate::W<REG> {
        self.variant(Wrxferact::Transferring)
    }
    #[doc = "No valid data"]
    #[inline(always)]
    pub fn novaliddata(self) -> &'a mut crate::W<REG> {
        self.variant(Wrxferact::Novaliddata)
    }
}
#[doc = "This status is used for detecting completion of a read transfer. This bit is set to 1 for either of the following conditions: After the end bit of the read command When writing a 1 to continue Request in the Block Gap Control register to restart a read transfer This bit is cleared to 0 for either of the following conditions: When the last data block as specified by block length is transferred to the system. When all valid data blocks have been transferred to the system and no current block transfers are being sent as a result of the Stop At Block Gap Request set to 1. A transfer complete interrupt is generated when this bit changes to 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rdxferact {
    #[doc = "1: Transferring data"]
    Transferring = 1,
    #[doc = "0: No valid data"]
    Novalidata = 0,
}
impl From<Rdxferact> for bool {
    #[inline(always)]
    fn from(variant: Rdxferact) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDXFERACT` reader - This status is used for detecting completion of a read transfer. This bit is set to 1 for either of the following conditions: After the end bit of the read command When writing a 1 to continue Request in the Block Gap Control register to restart a read transfer This bit is cleared to 0 for either of the following conditions: When the last data block as specified by block length is transferred to the system. When all valid data blocks have been transferred to the system and no current block transfers are being sent as a result of the Stop At Block Gap Request set to 1. A transfer complete interrupt is generated when this bit changes to 0."]
pub type RdxferactR = crate::BitReader<Rdxferact>;
impl RdxferactR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rdxferact {
        match self.bits {
            true => Rdxferact::Transferring,
            false => Rdxferact::Novalidata,
        }
    }
    #[doc = "Transferring data"]
    #[inline(always)]
    pub fn is_transferring(&self) -> bool {
        *self == Rdxferact::Transferring
    }
    #[doc = "No valid data"]
    #[inline(always)]
    pub fn is_novalidata(&self) -> bool {
        *self == Rdxferact::Novalidata
    }
}
#[doc = "Field `RDXFERACT` writer - This status is used for detecting completion of a read transfer. This bit is set to 1 for either of the following conditions: After the end bit of the read command When writing a 1 to continue Request in the Block Gap Control register to restart a read transfer This bit is cleared to 0 for either of the following conditions: When the last data block as specified by block length is transferred to the system. When all valid data blocks have been transferred to the system and no current block transfers are being sent as a result of the Stop At Block Gap Request set to 1. A transfer complete interrupt is generated when this bit changes to 0."]
pub type RdxferactW<'a, REG> = crate::BitWriter<'a, REG, Rdxferact>;
impl<'a, REG> RdxferactW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transferring data"]
    #[inline(always)]
    pub fn transferring(self) -> &'a mut crate::W<REG> {
        self.variant(Rdxferact::Transferring)
    }
    #[doc = "No valid data"]
    #[inline(always)]
    pub fn novalidata(self) -> &'a mut crate::W<REG> {
        self.variant(Rdxferact::Novalidata)
    }
}
#[doc = "This status is used for non-DMA write transfers. This read only flag indicates if space is available for write data. If this bit is 1, data can be written to the buffer. A change of this bit from 1 to 0 occurs when all the block data is written to the buffer. A change of this bit from 0 to 1 occurs when top of block data can be written to the buffer and generates the Buffer Write Ready Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bufwren {
    #[doc = "0: Write Disable"]
    Disable = 0,
    #[doc = "1: Write Enable."]
    Enable = 1,
}
impl From<Bufwren> for bool {
    #[inline(always)]
    fn from(variant: Bufwren) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUFWREN` reader - This status is used for non-DMA write transfers. This read only flag indicates if space is available for write data. If this bit is 1, data can be written to the buffer. A change of this bit from 1 to 0 occurs when all the block data is written to the buffer. A change of this bit from 0 to 1 occurs when top of block data can be written to the buffer and generates the Buffer Write Ready Interrupt."]
pub type BufwrenR = crate::BitReader<Bufwren>;
impl BufwrenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bufwren {
        match self.bits {
            false => Bufwren::Disable,
            true => Bufwren::Enable,
        }
    }
    #[doc = "Write Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Bufwren::Disable
    }
    #[doc = "Write Enable."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Bufwren::Enable
    }
}
#[doc = "Field `BUFWREN` writer - This status is used for non-DMA write transfers. This read only flag indicates if space is available for write data. If this bit is 1, data can be written to the buffer. A change of this bit from 1 to 0 occurs when all the block data is written to the buffer. A change of this bit from 0 to 1 occurs when top of block data can be written to the buffer and generates the Buffer Write Ready Interrupt."]
pub type BufwrenW<'a, REG> = crate::BitWriter<'a, REG, Bufwren>;
impl<'a, REG> BufwrenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Bufwren::Disable)
    }
    #[doc = "Write Enable."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Bufwren::Enable)
    }
}
#[doc = "This status is used for non-DMA read transfers. This read only flag indicates that valid data exists in the host side buffer status. If this bit is 1, readable data exists in the buffer. A change of this bit from 1 to 0 occurs when all the block data is read from the buffer. A change of this bit from 0 to 1 occurs when all the block data is ready in the buffer and generates the Buffer Read Ready Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bufrden {
    #[doc = "0: Read Disable"]
    Disable = 0,
    #[doc = "1: Read Enable."]
    Enable = 1,
}
impl From<Bufrden> for bool {
    #[inline(always)]
    fn from(variant: Bufrden) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUFRDEN` reader - This status is used for non-DMA read transfers. This read only flag indicates that valid data exists in the host side buffer status. If this bit is 1, readable data exists in the buffer. A change of this bit from 1 to 0 occurs when all the block data is read from the buffer. A change of this bit from 0 to 1 occurs when all the block data is ready in the buffer and generates the Buffer Read Ready Interrupt."]
pub type BufrdenR = crate::BitReader<Bufrden>;
impl BufrdenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bufrden {
        match self.bits {
            false => Bufrden::Disable,
            true => Bufrden::Enable,
        }
    }
    #[doc = "Read Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Bufrden::Disable
    }
    #[doc = "Read Enable."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Bufrden::Enable
    }
}
#[doc = "Field `BUFRDEN` writer - This status is used for non-DMA read transfers. This read only flag indicates that valid data exists in the host side buffer status. If this bit is 1, readable data exists in the buffer. A change of this bit from 1 to 0 occurs when all the block data is read from the buffer. A change of this bit from 0 to 1 occurs when all the block data is ready in the buffer and generates the Buffer Read Ready Interrupt."]
pub type BufrdenW<'a, REG> = crate::BitWriter<'a, REG, Bufrden>;
impl<'a, REG> BufrdenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Read Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Bufrden::Disable)
    }
    #[doc = "Read Enable."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Bufrden::Enable)
    }
}
#[doc = "This bit indicates whether a card has been inserted. Changing from 0 to 1 generates a Card Insertion interrupt in the Normal Interrupt Status register and changing from 1 to 0 generates a Card Removal Interrupt in the Normal Interrupt Status register. The Software Reset For All in the Software Reset register shall not affect this bit. If a Card is removed while its power is on and its clock is oscillating, the HC shall clear SD Bus Power in the Power Control register and SD Clock Enable in the Clock control register. In addition the HD should clear the HC by the Software Reset For All in Software register. The card detect is active regardless of the SD Bus Power.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cardinserted {
    #[doc = "0: Reset or Debouncing or No Card"]
    ResetDebouncingNocard = 0,
    #[doc = "1: Card Inserted"]
    Cardinserted = 1,
}
impl From<Cardinserted> for bool {
    #[inline(always)]
    fn from(variant: Cardinserted) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CARDINSERTED` reader - This bit indicates whether a card has been inserted. Changing from 0 to 1 generates a Card Insertion interrupt in the Normal Interrupt Status register and changing from 1 to 0 generates a Card Removal Interrupt in the Normal Interrupt Status register. The Software Reset For All in the Software Reset register shall not affect this bit. If a Card is removed while its power is on and its clock is oscillating, the HC shall clear SD Bus Power in the Power Control register and SD Clock Enable in the Clock control register. In addition the HD should clear the HC by the Software Reset For All in Software register. The card detect is active regardless of the SD Bus Power."]
pub type CardinsertedR = crate::BitReader<Cardinserted>;
impl CardinsertedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cardinserted {
        match self.bits {
            false => Cardinserted::ResetDebouncingNocard,
            true => Cardinserted::Cardinserted,
        }
    }
    #[doc = "Reset or Debouncing or No Card"]
    #[inline(always)]
    pub fn is_reset_debouncing_nocard(&self) -> bool {
        *self == Cardinserted::ResetDebouncingNocard
    }
    #[doc = "Card Inserted"]
    #[inline(always)]
    pub fn is_cardinserted(&self) -> bool {
        *self == Cardinserted::Cardinserted
    }
}
#[doc = "Field `CARDINSERTED` writer - This bit indicates whether a card has been inserted. Changing from 0 to 1 generates a Card Insertion interrupt in the Normal Interrupt Status register and changing from 1 to 0 generates a Card Removal Interrupt in the Normal Interrupt Status register. The Software Reset For All in the Software Reset register shall not affect this bit. If a Card is removed while its power is on and its clock is oscillating, the HC shall clear SD Bus Power in the Power Control register and SD Clock Enable in the Clock control register. In addition the HD should clear the HC by the Software Reset For All in Software register. The card detect is active regardless of the SD Bus Power."]
pub type CardinsertedW<'a, REG> = crate::BitWriter<'a, REG, Cardinserted>;
impl<'a, REG> CardinsertedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset or Debouncing or No Card"]
    #[inline(always)]
    pub fn reset_debouncing_nocard(self) -> &'a mut crate::W<REG> {
        self.variant(Cardinserted::ResetDebouncingNocard)
    }
    #[doc = "Card Inserted"]
    #[inline(always)]
    pub fn cardinserted(self) -> &'a mut crate::W<REG> {
        self.variant(Cardinserted::Cardinserted)
    }
}
#[doc = "This bit is used for testing. If it is 0, the Card Detect Pin Level is not stable. If this bit is set to 1, it means the Card Detect Pin Level is stable. The Software Reset For All in the Software Reset Register shall not affect this bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cardstable {
    #[doc = "0: Reset or Debouncing or No Card"]
    ResetDebouncingNocard = 0,
    #[doc = "1: Card Inserted"]
    Cardinserted = 1,
}
impl From<Cardstable> for bool {
    #[inline(always)]
    fn from(variant: Cardstable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CARDSTABLE` reader - This bit is used for testing. If it is 0, the Card Detect Pin Level is not stable. If this bit is set to 1, it means the Card Detect Pin Level is stable. The Software Reset For All in the Software Reset Register shall not affect this bit."]
pub type CardstableR = crate::BitReader<Cardstable>;
impl CardstableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cardstable {
        match self.bits {
            false => Cardstable::ResetDebouncingNocard,
            true => Cardstable::Cardinserted,
        }
    }
    #[doc = "Reset or Debouncing or No Card"]
    #[inline(always)]
    pub fn is_reset_debouncing_nocard(&self) -> bool {
        *self == Cardstable::ResetDebouncingNocard
    }
    #[doc = "Card Inserted"]
    #[inline(always)]
    pub fn is_cardinserted(&self) -> bool {
        *self == Cardstable::Cardinserted
    }
}
#[doc = "Field `CARDSTABLE` writer - This bit is used for testing. If it is 0, the Card Detect Pin Level is not stable. If this bit is set to 1, it means the Card Detect Pin Level is stable. The Software Reset For All in the Software Reset Register shall not affect this bit."]
pub type CardstableW<'a, REG> = crate::BitWriter<'a, REG, Cardstable>;
impl<'a, REG> CardstableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset or Debouncing or No Card"]
    #[inline(always)]
    pub fn reset_debouncing_nocard(self) -> &'a mut crate::W<REG> {
        self.variant(Cardstable::ResetDebouncingNocard)
    }
    #[doc = "Card Inserted"]
    #[inline(always)]
    pub fn cardinserted(self) -> &'a mut crate::W<REG> {
        self.variant(Cardstable::Cardinserted)
    }
}
#[doc = "This bit reflects the inverse value of the SDCD# pin.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Carddet {
    #[doc = "0: No Card present (SDCD# = 1)"]
    Nocardpresent = 0,
    #[doc = "1: Card present (SDCD# = 0)"]
    Cardpresent = 1,
}
impl From<Carddet> for bool {
    #[inline(always)]
    fn from(variant: Carddet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CARDDET` reader - This bit reflects the inverse value of the SDCD# pin."]
pub type CarddetR = crate::BitReader<Carddet>;
impl CarddetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Carddet {
        match self.bits {
            false => Carddet::Nocardpresent,
            true => Carddet::Cardpresent,
        }
    }
    #[doc = "No Card present (SDCD# = 1)"]
    #[inline(always)]
    pub fn is_nocardpresent(&self) -> bool {
        *self == Carddet::Nocardpresent
    }
    #[doc = "Card present (SDCD# = 0)"]
    #[inline(always)]
    pub fn is_cardpresent(&self) -> bool {
        *self == Carddet::Cardpresent
    }
}
#[doc = "Field `CARDDET` writer - This bit reflects the inverse value of the SDCD# pin."]
pub type CarddetW<'a, REG> = crate::BitWriter<'a, REG, Carddet>;
impl<'a, REG> CarddetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Card present (SDCD# = 1)"]
    #[inline(always)]
    pub fn nocardpresent(self) -> &'a mut crate::W<REG> {
        self.variant(Carddet::Nocardpresent)
    }
    #[doc = "Card present (SDCD# = 0)"]
    #[inline(always)]
    pub fn cardpresent(self) -> &'a mut crate::W<REG> {
        self.variant(Carddet::Cardpresent)
    }
}
#[doc = "The Write Protect Switch is supported for memory and combo cards. This bit reflects the SDWP# pin.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wrprotsw {
    #[doc = "0: Write protected (SDWP# = 0)"]
    Writeprotected = 0,
    #[doc = "1: Write enabled (SDWP# = 1)"]
    Writeenabled = 1,
}
impl From<Wrprotsw> for bool {
    #[inline(always)]
    fn from(variant: Wrprotsw) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WRPROTSW` reader - The Write Protect Switch is supported for memory and combo cards. This bit reflects the SDWP# pin."]
pub type WrprotswR = crate::BitReader<Wrprotsw>;
impl WrprotswR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wrprotsw {
        match self.bits {
            false => Wrprotsw::Writeprotected,
            true => Wrprotsw::Writeenabled,
        }
    }
    #[doc = "Write protected (SDWP# = 0)"]
    #[inline(always)]
    pub fn is_writeprotected(&self) -> bool {
        *self == Wrprotsw::Writeprotected
    }
    #[doc = "Write enabled (SDWP# = 1)"]
    #[inline(always)]
    pub fn is_writeenabled(&self) -> bool {
        *self == Wrprotsw::Writeenabled
    }
}
#[doc = "Field `WRPROTSW` writer - The Write Protect Switch is supported for memory and combo cards. This bit reflects the SDWP# pin."]
pub type WrprotswW<'a, REG> = crate::BitWriter<'a, REG, Wrprotsw>;
impl<'a, REG> WrprotswW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write protected (SDWP# = 0)"]
    #[inline(always)]
    pub fn writeprotected(self) -> &'a mut crate::W<REG> {
        self.variant(Wrprotsw::Writeprotected)
    }
    #[doc = "Write enabled (SDWP# = 1)"]
    #[inline(always)]
    pub fn writeenabled(self) -> &'a mut crate::W<REG> {
        self.variant(Wrprotsw::Writeenabled)
    }
}
#[doc = "This status is used to check DAT line level to recover from errors, and for debugging. This is especially useful in detecting the busy signal level from DAT\\[0\\].\n\nValue on reset: 15"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dat30line {
    #[doc = "8: DAT\\[3\\]"]
    Dat3 = 8,
    #[doc = "4: DAT\\[2\\]"]
    Dat2 = 4,
    #[doc = "2: DAT\\[1\\]"]
    Dat1 = 2,
    #[doc = "1: DAT\\[0\\]"]
    Dat0 = 1,
}
impl From<Dat30line> for u8 {
    #[inline(always)]
    fn from(variant: Dat30line) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dat30line {
    type Ux = u8;
}
impl crate::IsEnum for Dat30line {}
#[doc = "Field `DAT30LINE` reader - This status is used to check DAT line level to recover from errors, and for debugging. This is especially useful in detecting the busy signal level from DAT\\[0\\]."]
pub type Dat30lineR = crate::FieldReader<Dat30line>;
impl Dat30lineR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dat30line> {
        match self.bits {
            8 => Some(Dat30line::Dat3),
            4 => Some(Dat30line::Dat2),
            2 => Some(Dat30line::Dat1),
            1 => Some(Dat30line::Dat0),
            _ => None,
        }
    }
    #[doc = "DAT\\[3\\]"]
    #[inline(always)]
    pub fn is_dat3(&self) -> bool {
        *self == Dat30line::Dat3
    }
    #[doc = "DAT\\[2\\]"]
    #[inline(always)]
    pub fn is_dat2(&self) -> bool {
        *self == Dat30line::Dat2
    }
    #[doc = "DAT\\[1\\]"]
    #[inline(always)]
    pub fn is_dat1(&self) -> bool {
        *self == Dat30line::Dat1
    }
    #[doc = "DAT\\[0\\]"]
    #[inline(always)]
    pub fn is_dat0(&self) -> bool {
        *self == Dat30line::Dat0
    }
}
#[doc = "Field `DAT30LINE` writer - This status is used to check DAT line level to recover from errors, and for debugging. This is especially useful in detecting the busy signal level from DAT\\[0\\]."]
pub type Dat30lineW<'a, REG> = crate::FieldWriter<'a, REG, 4, Dat30line>;
impl<'a, REG> Dat30lineW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DAT\\[3\\]"]
    #[inline(always)]
    pub fn dat3(self) -> &'a mut crate::W<REG> {
        self.variant(Dat30line::Dat3)
    }
    #[doc = "DAT\\[2\\]"]
    #[inline(always)]
    pub fn dat2(self) -> &'a mut crate::W<REG> {
        self.variant(Dat30line::Dat2)
    }
    #[doc = "DAT\\[1\\]"]
    #[inline(always)]
    pub fn dat1(self) -> &'a mut crate::W<REG> {
        self.variant(Dat30line::Dat1)
    }
    #[doc = "DAT\\[0\\]"]
    #[inline(always)]
    pub fn dat0(self) -> &'a mut crate::W<REG> {
        self.variant(Dat30line::Dat0)
    }
}
#[doc = "Field `CMDLINE` reader - This status is used to check CMD line level to recover from errors, and for debugging."]
pub type CmdlineR = crate::BitReader;
#[doc = "Field `CMDLINE` writer - This status is used to check CMD line level to recover from errors, and for debugging."]
pub type CmdlineW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "This status is used to check DAT line level to recover from errors, and for debugging.\n\nValue on reset: 15"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dat74line {
    #[doc = "8: DAT\\[7\\]"]
    Dat7 = 8,
    #[doc = "4: DAT\\[6\\]"]
    Dat6 = 4,
    #[doc = "2: DAT\\[5\\]"]
    Dat5 = 2,
    #[doc = "1: DAT\\[4\\]"]
    Dat4 = 1,
}
impl From<Dat74line> for u8 {
    #[inline(always)]
    fn from(variant: Dat74line) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dat74line {
    type Ux = u8;
}
impl crate::IsEnum for Dat74line {}
#[doc = "Field `DAT74LINE` reader - This status is used to check DAT line level to recover from errors, and for debugging."]
pub type Dat74lineR = crate::FieldReader<Dat74line>;
impl Dat74lineR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dat74line> {
        match self.bits {
            8 => Some(Dat74line::Dat7),
            4 => Some(Dat74line::Dat6),
            2 => Some(Dat74line::Dat5),
            1 => Some(Dat74line::Dat4),
            _ => None,
        }
    }
    #[doc = "DAT\\[7\\]"]
    #[inline(always)]
    pub fn is_dat7(&self) -> bool {
        *self == Dat74line::Dat7
    }
    #[doc = "DAT\\[6\\]"]
    #[inline(always)]
    pub fn is_dat6(&self) -> bool {
        *self == Dat74line::Dat6
    }
    #[doc = "DAT\\[5\\]"]
    #[inline(always)]
    pub fn is_dat5(&self) -> bool {
        *self == Dat74line::Dat5
    }
    #[doc = "DAT\\[4\\]"]
    #[inline(always)]
    pub fn is_dat4(&self) -> bool {
        *self == Dat74line::Dat4
    }
}
#[doc = "Field `DAT74LINE` writer - This status is used to check DAT line level to recover from errors, and for debugging."]
pub type Dat74lineW<'a, REG> = crate::FieldWriter<'a, REG, 4, Dat74line>;
impl<'a, REG> Dat74lineW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DAT\\[7\\]"]
    #[inline(always)]
    pub fn dat7(self) -> &'a mut crate::W<REG> {
        self.variant(Dat74line::Dat7)
    }
    #[doc = "DAT\\[6\\]"]
    #[inline(always)]
    pub fn dat6(self) -> &'a mut crate::W<REG> {
        self.variant(Dat74line::Dat6)
    }
    #[doc = "DAT\\[5\\]"]
    #[inline(always)]
    pub fn dat5(self) -> &'a mut crate::W<REG> {
        self.variant(Dat74line::Dat5)
    }
    #[doc = "DAT\\[4\\]"]
    #[inline(always)]
    pub fn dat4(self) -> &'a mut crate::W<REG> {
        self.variant(Dat74line::Dat4)
    }
}
impl R {
    #[doc = "Bit 0 - If this bit is 0, it indicates the CMD line is not in use and the HC can issue a SD command using the CMD line. This bit is set immediately after the Command register (00Fh) is written. This bit is cleared when the command response is received. Even if the Command Inhibit (DAT) is set to 1, Commands using only the CMD line can be issued if this bit is 0. Changing from 1 to 0 generates a Command complete interrupt in the Normal Interrupt Status register. If the HC cannot issue the command because of a command conflict error or because of Command Not Issued By Auto CMD12 Error, this bit shall remain 1 and the Command Complete is not set. Status issuing Auto CMD12 is not read from this bit. Auto CMD12 and Auto CMD23 consist of two responses. In this case, this bit is not cleared by the response of CMD12 or CMD23 but cleared by the response of a read/write command. Status issuing Auto CMD12 is not read from this bit. So if a command is issued during Auto CMD12 operation, Host Controller shall manage to issue two commands: CMD12 and a command set by Command register. Note: DAT line active indicates whether one of the DAT line is on SD bus is in use. (a) In the case of read transactions This status indicates whether a read transfer is executing on the SD Bus. Changing this value from 1 to 0 generates a Block Gap Event interrupt in the Normal Interrupt Status register, as the result of the Stop At Block Gap Request being set. This bit shall be set in either of the following cases: (1) After the end bit of the read command. (2) When writing a 1 to Continue Request in the Block Gap Control register to restart a read transfer. This bit shall be cleared in either of the following cases: (1) When the end bit of the last data block is sent from the SD Bus to the Host Controller. In case of ADMA2, the last block is designated by the last transfer of Descriptor Table. (2) When a read transfer is stopped at the block gap initiated by a Stop At Block Gap Request. The Host Controller shall stop read operation at the start of the interrupt cycle of the next block gap by driving Read Wait or stopping SD clock. If the Read Wait signal is already driven (due to data buffer cannot receive data), the Host Controller can continue to stop read operation by driving the Read Wait signal. It is necessary to support Read Wait in order to use suspend / resume function. (b) In the case of write transactions This status indicates that a write transfer is executing on the SD Bus. Changing this value from 1 to 0 generate a Transfer Complete interrupt in the Normal Interrupt Status register. This bit shall be set in either of the following cases: (1) After the end bit of the write command. (2) When writing to 1 to Continue Request in the Block Gap Control register to continue a write transfer. This bit shall be cleared in either of the following cases: (1) When the SD card releases write busy of the last data block. If SD card does not drive busy signal for 8 SD Clocks, the Host Controller shall consider the card drive Not Busy. In case of ADMA2, the last block is designated by the last transfer of Descriptor Table. (2) When the SD card releases write busy prior to waiting for write transfer as a result of a Stop At Block Gap Request. (c) Command with busy This status indicates whether a command indicates busy (ex. erase command for memory) is executing on the SD Bus. This bit is set after the end bit of the command with busy and cleared when busy is de-asserted. Changing this bit from 1 to 0 generate a Transfer Complete interrupt in the Normal Interrupt Status register. Note: The HD can issue cmd0, cmd12, cmd13 (for memory) and cmd52 (for SDIO) when the DAT lines are busy during data transfer. These commands can be issued when Command Inhibit (CMD) is set to zero. Other commands shall be issued when Command Inhibit (DAT) is set to zero."]
    #[inline(always)]
    pub fn cmdinhcmd(&self) -> CmdinhcmdR {
        CmdinhcmdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This status bit is generated if either the DAT Line Active or the Read transfer Active is set to 1. If this bit is 0, it indicates the HC can issue the next SD command. Commands with busy signal belong to Command Inhibit (DAT) (ex. R1b, R5b type). Changing from 1 to 0 generates a Transfer Complete interrupt in the Normal interrupt status register. Note: The SD Host Driver can save registers in the range of 000-00Dh for a suspend transaction after this bit has changed from 1 to 0."]
    #[inline(always)]
    pub fn cmdinhdat(&self) -> CmdinhdatR {
        CmdinhdatR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This bit indicates whether one of the DAT line on SD bus is in use."]
    #[inline(always)]
    pub fn dlineact(&self) -> DlineactR {
        DlineactR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Re-Tuning Request Host Controller may request Host Driver to execute re-tuning sequence by setting this bit when the data window is shifted by temperature drift and a tuned sampling point does not have a good margin to receive correct data. This bit is cleared when a command is issued with setting Execute Tuning in the Host Control 2 register. Changing of this bit from 0 to 1 generates Re-Tuning Event. Refer to Normal Interrupt registers for more detail. This bit isn't set to 1 if Sampling Clock Select in the Host Control 2 register is set to 0 (using fixed sampling clock)."]
    #[inline(always)]
    pub fn retuningrequest(&self) -> RetuningrequestR {
        RetuningrequestR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - This status indicates a write transfer is active. If this bit is 0, it means no valid write data exists in the HC. This bit is set in either of the following cases: After the end bit of the write command. When writing a 1 to Continue Request in the Block Gap Control register to restart a write transfer. This bit is cleared in either of the following cases: After getting the CRC status of the last data block as specified by the transfer count (Single or Multiple) After getting a CRC status of any block where data transmission is about to be stopped by a Stop At Block Gap Request. During a write transaction, a Block Gap Event interrupt is generated when this bit is changed to 0, as a result of the Stop At Block Gap Request being set. This status is useful for the HD in determining when to issue commands during write busy."]
    #[inline(always)]
    pub fn wrxferact(&self) -> WrxferactR {
        WrxferactR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - This status is used for detecting completion of a read transfer. This bit is set to 1 for either of the following conditions: After the end bit of the read command When writing a 1 to continue Request in the Block Gap Control register to restart a read transfer This bit is cleared to 0 for either of the following conditions: When the last data block as specified by block length is transferred to the system. When all valid data blocks have been transferred to the system and no current block transfers are being sent as a result of the Stop At Block Gap Request set to 1. A transfer complete interrupt is generated when this bit changes to 0."]
    #[inline(always)]
    pub fn rdxferact(&self) -> RdxferactR {
        RdxferactR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - This status is used for non-DMA write transfers. This read only flag indicates if space is available for write data. If this bit is 1, data can be written to the buffer. A change of this bit from 1 to 0 occurs when all the block data is written to the buffer. A change of this bit from 0 to 1 occurs when top of block data can be written to the buffer and generates the Buffer Write Ready Interrupt."]
    #[inline(always)]
    pub fn bufwren(&self) -> BufwrenR {
        BufwrenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - This status is used for non-DMA read transfers. This read only flag indicates that valid data exists in the host side buffer status. If this bit is 1, readable data exists in the buffer. A change of this bit from 1 to 0 occurs when all the block data is read from the buffer. A change of this bit from 0 to 1 occurs when all the block data is ready in the buffer and generates the Buffer Read Ready Interrupt."]
    #[inline(always)]
    pub fn bufrden(&self) -> BufrdenR {
        BufrdenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - This bit indicates whether a card has been inserted. Changing from 0 to 1 generates a Card Insertion interrupt in the Normal Interrupt Status register and changing from 1 to 0 generates a Card Removal Interrupt in the Normal Interrupt Status register. The Software Reset For All in the Software Reset register shall not affect this bit. If a Card is removed while its power is on and its clock is oscillating, the HC shall clear SD Bus Power in the Power Control register and SD Clock Enable in the Clock control register. In addition the HD should clear the HC by the Software Reset For All in Software register. The card detect is active regardless of the SD Bus Power."]
    #[inline(always)]
    pub fn cardinserted(&self) -> CardinsertedR {
        CardinsertedR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - This bit is used for testing. If it is 0, the Card Detect Pin Level is not stable. If this bit is set to 1, it means the Card Detect Pin Level is stable. The Software Reset For All in the Software Reset Register shall not affect this bit."]
    #[inline(always)]
    pub fn cardstable(&self) -> CardstableR {
        CardstableR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - This bit reflects the inverse value of the SDCD# pin."]
    #[inline(always)]
    pub fn carddet(&self) -> CarddetR {
        CarddetR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - The Write Protect Switch is supported for memory and combo cards. This bit reflects the SDWP# pin."]
    #[inline(always)]
    pub fn wrprotsw(&self) -> WrprotswR {
        WrprotswR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - This status is used to check DAT line level to recover from errors, and for debugging. This is especially useful in detecting the busy signal level from DAT\\[0\\]."]
    #[inline(always)]
    pub fn dat30line(&self) -> Dat30lineR {
        Dat30lineR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - This status is used to check CMD line level to recover from errors, and for debugging."]
    #[inline(always)]
    pub fn cmdline(&self) -> CmdlineR {
        CmdlineR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:28 - This status is used to check DAT line level to recover from errors, and for debugging."]
    #[inline(always)]
    pub fn dat74line(&self) -> Dat74lineR {
        Dat74lineR::new(((self.bits >> 25) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - If this bit is 0, it indicates the CMD line is not in use and the HC can issue a SD command using the CMD line. This bit is set immediately after the Command register (00Fh) is written. This bit is cleared when the command response is received. Even if the Command Inhibit (DAT) is set to 1, Commands using only the CMD line can be issued if this bit is 0. Changing from 1 to 0 generates a Command complete interrupt in the Normal Interrupt Status register. If the HC cannot issue the command because of a command conflict error or because of Command Not Issued By Auto CMD12 Error, this bit shall remain 1 and the Command Complete is not set. Status issuing Auto CMD12 is not read from this bit. Auto CMD12 and Auto CMD23 consist of two responses. In this case, this bit is not cleared by the response of CMD12 or CMD23 but cleared by the response of a read/write command. Status issuing Auto CMD12 is not read from this bit. So if a command is issued during Auto CMD12 operation, Host Controller shall manage to issue two commands: CMD12 and a command set by Command register. Note: DAT line active indicates whether one of the DAT line is on SD bus is in use. (a) In the case of read transactions This status indicates whether a read transfer is executing on the SD Bus. Changing this value from 1 to 0 generates a Block Gap Event interrupt in the Normal Interrupt Status register, as the result of the Stop At Block Gap Request being set. This bit shall be set in either of the following cases: (1) After the end bit of the read command. (2) When writing a 1 to Continue Request in the Block Gap Control register to restart a read transfer. This bit shall be cleared in either of the following cases: (1) When the end bit of the last data block is sent from the SD Bus to the Host Controller. In case of ADMA2, the last block is designated by the last transfer of Descriptor Table. (2) When a read transfer is stopped at the block gap initiated by a Stop At Block Gap Request. The Host Controller shall stop read operation at the start of the interrupt cycle of the next block gap by driving Read Wait or stopping SD clock. If the Read Wait signal is already driven (due to data buffer cannot receive data), the Host Controller can continue to stop read operation by driving the Read Wait signal. It is necessary to support Read Wait in order to use suspend / resume function. (b) In the case of write transactions This status indicates that a write transfer is executing on the SD Bus. Changing this value from 1 to 0 generate a Transfer Complete interrupt in the Normal Interrupt Status register. This bit shall be set in either of the following cases: (1) After the end bit of the write command. (2) When writing to 1 to Continue Request in the Block Gap Control register to continue a write transfer. This bit shall be cleared in either of the following cases: (1) When the SD card releases write busy of the last data block. If SD card does not drive busy signal for 8 SD Clocks, the Host Controller shall consider the card drive Not Busy. In case of ADMA2, the last block is designated by the last transfer of Descriptor Table. (2) When the SD card releases write busy prior to waiting for write transfer as a result of a Stop At Block Gap Request. (c) Command with busy This status indicates whether a command indicates busy (ex. erase command for memory) is executing on the SD Bus. This bit is set after the end bit of the command with busy and cleared when busy is de-asserted. Changing this bit from 1 to 0 generate a Transfer Complete interrupt in the Normal Interrupt Status register. Note: The HD can issue cmd0, cmd12, cmd13 (for memory) and cmd52 (for SDIO) when the DAT lines are busy during data transfer. These commands can be issued when Command Inhibit (CMD) is set to zero. Other commands shall be issued when Command Inhibit (DAT) is set to zero."]
    #[inline(always)]
    #[must_use]
    pub fn cmdinhcmd(&mut self) -> CmdinhcmdW<PresentSpec> {
        CmdinhcmdW::new(self, 0)
    }
    #[doc = "Bit 1 - This status bit is generated if either the DAT Line Active or the Read transfer Active is set to 1. If this bit is 0, it indicates the HC can issue the next SD command. Commands with busy signal belong to Command Inhibit (DAT) (ex. R1b, R5b type). Changing from 1 to 0 generates a Transfer Complete interrupt in the Normal interrupt status register. Note: The SD Host Driver can save registers in the range of 000-00Dh for a suspend transaction after this bit has changed from 1 to 0."]
    #[inline(always)]
    #[must_use]
    pub fn cmdinhdat(&mut self) -> CmdinhdatW<PresentSpec> {
        CmdinhdatW::new(self, 1)
    }
    #[doc = "Bit 2 - This bit indicates whether one of the DAT line on SD bus is in use."]
    #[inline(always)]
    #[must_use]
    pub fn dlineact(&mut self) -> DlineactW<PresentSpec> {
        DlineactW::new(self, 2)
    }
    #[doc = "Bit 3 - Re-Tuning Request Host Controller may request Host Driver to execute re-tuning sequence by setting this bit when the data window is shifted by temperature drift and a tuned sampling point does not have a good margin to receive correct data. This bit is cleared when a command is issued with setting Execute Tuning in the Host Control 2 register. Changing of this bit from 0 to 1 generates Re-Tuning Event. Refer to Normal Interrupt registers for more detail. This bit isn't set to 1 if Sampling Clock Select in the Host Control 2 register is set to 0 (using fixed sampling clock)."]
    #[inline(always)]
    #[must_use]
    pub fn retuningrequest(&mut self) -> RetuningrequestW<PresentSpec> {
        RetuningrequestW::new(self, 3)
    }
    #[doc = "Bit 8 - This status indicates a write transfer is active. If this bit is 0, it means no valid write data exists in the HC. This bit is set in either of the following cases: After the end bit of the write command. When writing a 1 to Continue Request in the Block Gap Control register to restart a write transfer. This bit is cleared in either of the following cases: After getting the CRC status of the last data block as specified by the transfer count (Single or Multiple) After getting a CRC status of any block where data transmission is about to be stopped by a Stop At Block Gap Request. During a write transaction, a Block Gap Event interrupt is generated when this bit is changed to 0, as a result of the Stop At Block Gap Request being set. This status is useful for the HD in determining when to issue commands during write busy."]
    #[inline(always)]
    #[must_use]
    pub fn wrxferact(&mut self) -> WrxferactW<PresentSpec> {
        WrxferactW::new(self, 8)
    }
    #[doc = "Bit 9 - This status is used for detecting completion of a read transfer. This bit is set to 1 for either of the following conditions: After the end bit of the read command When writing a 1 to continue Request in the Block Gap Control register to restart a read transfer This bit is cleared to 0 for either of the following conditions: When the last data block as specified by block length is transferred to the system. When all valid data blocks have been transferred to the system and no current block transfers are being sent as a result of the Stop At Block Gap Request set to 1. A transfer complete interrupt is generated when this bit changes to 0."]
    #[inline(always)]
    #[must_use]
    pub fn rdxferact(&mut self) -> RdxferactW<PresentSpec> {
        RdxferactW::new(self, 9)
    }
    #[doc = "Bit 10 - This status is used for non-DMA write transfers. This read only flag indicates if space is available for write data. If this bit is 1, data can be written to the buffer. A change of this bit from 1 to 0 occurs when all the block data is written to the buffer. A change of this bit from 0 to 1 occurs when top of block data can be written to the buffer and generates the Buffer Write Ready Interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn bufwren(&mut self) -> BufwrenW<PresentSpec> {
        BufwrenW::new(self, 10)
    }
    #[doc = "Bit 11 - This status is used for non-DMA read transfers. This read only flag indicates that valid data exists in the host side buffer status. If this bit is 1, readable data exists in the buffer. A change of this bit from 1 to 0 occurs when all the block data is read from the buffer. A change of this bit from 0 to 1 occurs when all the block data is ready in the buffer and generates the Buffer Read Ready Interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn bufrden(&mut self) -> BufrdenW<PresentSpec> {
        BufrdenW::new(self, 11)
    }
    #[doc = "Bit 16 - This bit indicates whether a card has been inserted. Changing from 0 to 1 generates a Card Insertion interrupt in the Normal Interrupt Status register and changing from 1 to 0 generates a Card Removal Interrupt in the Normal Interrupt Status register. The Software Reset For All in the Software Reset register shall not affect this bit. If a Card is removed while its power is on and its clock is oscillating, the HC shall clear SD Bus Power in the Power Control register and SD Clock Enable in the Clock control register. In addition the HD should clear the HC by the Software Reset For All in Software register. The card detect is active regardless of the SD Bus Power."]
    #[inline(always)]
    #[must_use]
    pub fn cardinserted(&mut self) -> CardinsertedW<PresentSpec> {
        CardinsertedW::new(self, 16)
    }
    #[doc = "Bit 17 - This bit is used for testing. If it is 0, the Card Detect Pin Level is not stable. If this bit is set to 1, it means the Card Detect Pin Level is stable. The Software Reset For All in the Software Reset Register shall not affect this bit."]
    #[inline(always)]
    #[must_use]
    pub fn cardstable(&mut self) -> CardstableW<PresentSpec> {
        CardstableW::new(self, 17)
    }
    #[doc = "Bit 18 - This bit reflects the inverse value of the SDCD# pin."]
    #[inline(always)]
    #[must_use]
    pub fn carddet(&mut self) -> CarddetW<PresentSpec> {
        CarddetW::new(self, 18)
    }
    #[doc = "Bit 19 - The Write Protect Switch is supported for memory and combo cards. This bit reflects the SDWP# pin."]
    #[inline(always)]
    #[must_use]
    pub fn wrprotsw(&mut self) -> WrprotswW<PresentSpec> {
        WrprotswW::new(self, 19)
    }
    #[doc = "Bits 20:23 - This status is used to check DAT line level to recover from errors, and for debugging. This is especially useful in detecting the busy signal level from DAT\\[0\\]."]
    #[inline(always)]
    #[must_use]
    pub fn dat30line(&mut self) -> Dat30lineW<PresentSpec> {
        Dat30lineW::new(self, 20)
    }
    #[doc = "Bit 24 - This status is used to check CMD line level to recover from errors, and for debugging."]
    #[inline(always)]
    #[must_use]
    pub fn cmdline(&mut self) -> CmdlineW<PresentSpec> {
        CmdlineW::new(self, 24)
    }
    #[doc = "Bits 25:28 - This status is used to check DAT line level to recover from errors, and for debugging."]
    #[inline(always)]
    #[must_use]
    pub fn dat74line(&mut self) -> Dat74lineW<PresentSpec> {
        Dat74lineW::new(self, 25)
    }
}
#[doc = "Present state\n\nYou can [`read`](crate::Reg::read) this register and get [`present::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`present::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PresentSpec;
impl crate::RegisterSpec for PresentSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`present::R`](R) reader structure"]
impl crate::Readable for PresentSpec {}
#[doc = "`write(|w| ..)` method takes [`present::W`](W) writer structure"]
impl crate::Writable for PresentSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRESENT to value 0x1ff0_0000"]
impl crate::Resettable for PresentSpec {
    const RESET_VALUE: u32 = 0x1ff0_0000;
}
