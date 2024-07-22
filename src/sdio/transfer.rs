#[doc = "Register `TRANSFER` reader"]
pub type R = crate::R<TransferSpec>;
#[doc = "Register `TRANSFER` writer"]
pub type W = crate::W<TransferSpec>;
#[doc = "DMA can be enabled only if DMA Support bit in the Capabilities register is set. If this bit is set to 1, a DMA operation shall begin when the HD writes to the upper byte of Command register (00Fh).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmaen {
    #[doc = "0: Disable"]
    Disable = 0,
    #[doc = "1: Enable"]
    Enable = 1,
}
impl From<Dmaen> for bool {
    #[inline(always)]
    fn from(variant: Dmaen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAEN` reader - DMA can be enabled only if DMA Support bit in the Capabilities register is set. If this bit is set to 1, a DMA operation shall begin when the HD writes to the upper byte of Command register (00Fh)."]
pub type DmaenR = crate::BitReader<Dmaen>;
impl DmaenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmaen {
        match self.bits {
            false => Dmaen::Disable,
            true => Dmaen::Enable,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dmaen::Disable
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dmaen::Enable
    }
}
#[doc = "Field `DMAEN` writer - DMA can be enabled only if DMA Support bit in the Capabilities register is set. If this bit is set to 1, a DMA operation shall begin when the HD writes to the upper byte of Command register (00Fh)."]
pub type DmaenW<'a, REG> = crate::BitWriter<'a, REG, Dmaen>;
impl<'a, REG> DmaenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaen::Disable)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaen::Enable)
    }
}
#[doc = "This bit is used to enable the Block count register, which is only relevant for multiple block transfers. When this bit is 0, the Block Count register is disabled, which is useful in executing an infinite transfer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Blkcnten {
    #[doc = "0: Disable"]
    Disable = 0,
    #[doc = "1: Enable"]
    Enable = 1,
}
impl From<Blkcnten> for bool {
    #[inline(always)]
    fn from(variant: Blkcnten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BLKCNTEN` reader - This bit is used to enable the Block count register, which is only relevant for multiple block transfers. When this bit is 0, the Block Count register is disabled, which is useful in executing an infinite transfer."]
pub type BlkcntenR = crate::BitReader<Blkcnten>;
impl BlkcntenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Blkcnten {
        match self.bits {
            false => Blkcnten::Disable,
            true => Blkcnten::Enable,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Blkcnten::Disable
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Blkcnten::Enable
    }
}
#[doc = "Field `BLKCNTEN` writer - This bit is used to enable the Block count register, which is only relevant for multiple block transfers. When this bit is 0, the Block Count register is disabled, which is useful in executing an infinite transfer."]
pub type BlkcntenW<'a, REG> = crate::BitWriter<'a, REG, Blkcnten>;
impl<'a, REG> BlkcntenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Blkcnten::Disable)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Blkcnten::Enable)
    }
}
#[doc = "This field determines use of auto command functions. There are two methods to stop Multiple-block read and write operation. (1) Auto CMD12 Enable Multiple-block read and write commands for memory require CMD12 to stop the operation. When this field is set to 01b, the Host Controller issues CMD12 automatically when last block transfer is completed. Auto CMD12 error is indicated to the Auto CMD Error Status register. The Host Driver shall not set this bit if the command does not require CMD12. (2) Auto CMD23 Enable When this bit field is set to 10b, the Host Controller issues a CMD23 automatically before issuing a command specified in the Command Register The following conditions are required to use the Auto CMD23.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Acmden {
    #[doc = "0: Auto Command Disabled"]
    Disabled = 0,
    #[doc = "1: Auto CMD12 Enable"]
    Cmd12enable = 1,
    #[doc = "2: Auto CMD23 Enable"]
    Cmd23enable = 2,
}
impl From<Acmden> for u8 {
    #[inline(always)]
    fn from(variant: Acmden) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Acmden {
    type Ux = u8;
}
impl crate::IsEnum for Acmden {}
#[doc = "Field `ACMDEN` reader - This field determines use of auto command functions. There are two methods to stop Multiple-block read and write operation. (1) Auto CMD12 Enable Multiple-block read and write commands for memory require CMD12 to stop the operation. When this field is set to 01b, the Host Controller issues CMD12 automatically when last block transfer is completed. Auto CMD12 error is indicated to the Auto CMD Error Status register. The Host Driver shall not set this bit if the command does not require CMD12. (2) Auto CMD23 Enable When this bit field is set to 10b, the Host Controller issues a CMD23 automatically before issuing a command specified in the Command Register The following conditions are required to use the Auto CMD23."]
pub type AcmdenR = crate::FieldReader<Acmden>;
impl AcmdenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Acmden> {
        match self.bits {
            0 => Some(Acmden::Disabled),
            1 => Some(Acmden::Cmd12enable),
            2 => Some(Acmden::Cmd23enable),
            _ => None,
        }
    }
    #[doc = "Auto Command Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Acmden::Disabled
    }
    #[doc = "Auto CMD12 Enable"]
    #[inline(always)]
    pub fn is_cmd12enable(&self) -> bool {
        *self == Acmden::Cmd12enable
    }
    #[doc = "Auto CMD23 Enable"]
    #[inline(always)]
    pub fn is_cmd23enable(&self) -> bool {
        *self == Acmden::Cmd23enable
    }
}
#[doc = "Field `ACMDEN` writer - This field determines use of auto command functions. There are two methods to stop Multiple-block read and write operation. (1) Auto CMD12 Enable Multiple-block read and write commands for memory require CMD12 to stop the operation. When this field is set to 01b, the Host Controller issues CMD12 automatically when last block transfer is completed. Auto CMD12 error is indicated to the Auto CMD Error Status register. The Host Driver shall not set this bit if the command does not require CMD12. (2) Auto CMD23 Enable When this bit field is set to 10b, the Host Controller issues a CMD23 automatically before issuing a command specified in the Command Register The following conditions are required to use the Auto CMD23."]
pub type AcmdenW<'a, REG> = crate::FieldWriter<'a, REG, 2, Acmden>;
impl<'a, REG> AcmdenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Auto Command Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Acmden::Disabled)
    }
    #[doc = "Auto CMD12 Enable"]
    #[inline(always)]
    pub fn cmd12enable(self) -> &'a mut crate::W<REG> {
        self.variant(Acmden::Cmd12enable)
    }
    #[doc = "Auto CMD23 Enable"]
    #[inline(always)]
    pub fn cmd23enable(self) -> &'a mut crate::W<REG> {
        self.variant(Acmden::Cmd23enable)
    }
}
#[doc = "Data Transfer Direction Select. This bit defines the direction of data transfers.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dxferdirsel {
    #[doc = "0: Write (Host to Card)"]
    Write = 0,
    #[doc = "1: Read (Card to Host)"]
    Read = 1,
}
impl From<Dxferdirsel> for bool {
    #[inline(always)]
    fn from(variant: Dxferdirsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DXFERDIRSEL` reader - Data Transfer Direction Select. This bit defines the direction of data transfers."]
pub type DxferdirselR = crate::BitReader<Dxferdirsel>;
impl DxferdirselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dxferdirsel {
        match self.bits {
            false => Dxferdirsel::Write,
            true => Dxferdirsel::Read,
        }
    }
    #[doc = "Write (Host to Card)"]
    #[inline(always)]
    pub fn is_write(&self) -> bool {
        *self == Dxferdirsel::Write
    }
    #[doc = "Read (Card to Host)"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == Dxferdirsel::Read
    }
}
#[doc = "Field `DXFERDIRSEL` writer - Data Transfer Direction Select. This bit defines the direction of data transfers."]
pub type DxferdirselW<'a, REG> = crate::BitWriter<'a, REG, Dxferdirsel>;
impl<'a, REG> DxferdirselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write (Host to Card)"]
    #[inline(always)]
    pub fn write(self) -> &'a mut crate::W<REG> {
        self.variant(Dxferdirsel::Write)
    }
    #[doc = "Read (Card to Host)"]
    #[inline(always)]
    pub fn read(self) -> &'a mut crate::W<REG> {
        self.variant(Dxferdirsel::Read)
    }
}
#[doc = "This bit enables multiple block data transfers.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Blksel {
    #[doc = "0: Single Block"]
    Singleblock = 0,
    #[doc = "1: Multiple Block"]
    Multipleblock = 1,
}
impl From<Blksel> for bool {
    #[inline(always)]
    fn from(variant: Blksel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BLKSEL` reader - This bit enables multiple block data transfers."]
pub type BlkselR = crate::BitReader<Blksel>;
impl BlkselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Blksel {
        match self.bits {
            false => Blksel::Singleblock,
            true => Blksel::Multipleblock,
        }
    }
    #[doc = "Single Block"]
    #[inline(always)]
    pub fn is_singleblock(&self) -> bool {
        *self == Blksel::Singleblock
    }
    #[doc = "Multiple Block"]
    #[inline(always)]
    pub fn is_multipleblock(&self) -> bool {
        *self == Blksel::Multipleblock
    }
}
#[doc = "Field `BLKSEL` writer - This bit enables multiple block data transfers."]
pub type BlkselW<'a, REG> = crate::BitWriter<'a, REG, Blksel>;
impl<'a, REG> BlkselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Single Block"]
    #[inline(always)]
    pub fn singleblock(self) -> &'a mut crate::W<REG> {
        self.variant(Blksel::Singleblock)
    }
    #[doc = "Multiple Block"]
    #[inline(always)]
    pub fn multipleblock(self) -> &'a mut crate::W<REG> {
        self.variant(Blksel::Multipleblock)
    }
}
#[doc = "Response Type Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Resptypesel {
    #[doc = "0: No Response"]
    Noresponse = 0,
    #[doc = "1: Response length 136"]
    Len136 = 1,
    #[doc = "2: Response length 48"]
    Len48 = 2,
    #[doc = "3: Response length 48 check Busy after response"]
    Len48chkbusy = 3,
}
impl From<Resptypesel> for u8 {
    #[inline(always)]
    fn from(variant: Resptypesel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Resptypesel {
    type Ux = u8;
}
impl crate::IsEnum for Resptypesel {}
#[doc = "Field `RESPTYPESEL` reader - Response Type Select"]
pub type ResptypeselR = crate::FieldReader<Resptypesel>;
impl ResptypeselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Resptypesel {
        match self.bits {
            0 => Resptypesel::Noresponse,
            1 => Resptypesel::Len136,
            2 => Resptypesel::Len48,
            3 => Resptypesel::Len48chkbusy,
            _ => unreachable!(),
        }
    }
    #[doc = "No Response"]
    #[inline(always)]
    pub fn is_noresponse(&self) -> bool {
        *self == Resptypesel::Noresponse
    }
    #[doc = "Response length 136"]
    #[inline(always)]
    pub fn is_len136(&self) -> bool {
        *self == Resptypesel::Len136
    }
    #[doc = "Response length 48"]
    #[inline(always)]
    pub fn is_len48(&self) -> bool {
        *self == Resptypesel::Len48
    }
    #[doc = "Response length 48 check Busy after response"]
    #[inline(always)]
    pub fn is_len48chkbusy(&self) -> bool {
        *self == Resptypesel::Len48chkbusy
    }
}
#[doc = "Field `RESPTYPESEL` writer - Response Type Select"]
pub type ResptypeselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Resptypesel, crate::Safe>;
impl<'a, REG> ResptypeselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No Response"]
    #[inline(always)]
    pub fn noresponse(self) -> &'a mut crate::W<REG> {
        self.variant(Resptypesel::Noresponse)
    }
    #[doc = "Response length 136"]
    #[inline(always)]
    pub fn len136(self) -> &'a mut crate::W<REG> {
        self.variant(Resptypesel::Len136)
    }
    #[doc = "Response length 48"]
    #[inline(always)]
    pub fn len48(self) -> &'a mut crate::W<REG> {
        self.variant(Resptypesel::Len48)
    }
    #[doc = "Response length 48 check Busy after response"]
    #[inline(always)]
    pub fn len48chkbusy(self) -> &'a mut crate::W<REG> {
        self.variant(Resptypesel::Len48chkbusy)
    }
}
#[doc = "If this bit is set to 1, the HC shall check the CRC field in the response. If an error is detected, it is reported as a Command CRC Error. If this bit is set to 0, the CRC field is not checked.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmdcrcchken {
    #[doc = "0: Disable"]
    Disable = 0,
    #[doc = "1: Enable"]
    Enable = 1,
}
impl From<Cmdcrcchken> for bool {
    #[inline(always)]
    fn from(variant: Cmdcrcchken) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMDCRCCHKEN` reader - If this bit is set to 1, the HC shall check the CRC field in the response. If an error is detected, it is reported as a Command CRC Error. If this bit is set to 0, the CRC field is not checked."]
pub type CmdcrcchkenR = crate::BitReader<Cmdcrcchken>;
impl CmdcrcchkenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmdcrcchken {
        match self.bits {
            false => Cmdcrcchken::Disable,
            true => Cmdcrcchken::Enable,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cmdcrcchken::Disable
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Cmdcrcchken::Enable
    }
}
#[doc = "Field `CMDCRCCHKEN` writer - If this bit is set to 1, the HC shall check the CRC field in the response. If an error is detected, it is reported as a Command CRC Error. If this bit is set to 0, the CRC field is not checked."]
pub type CmdcrcchkenW<'a, REG> = crate::BitWriter<'a, REG, Cmdcrcchken>;
impl<'a, REG> CmdcrcchkenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdcrcchken::Disable)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdcrcchken::Enable)
    }
}
#[doc = "If this bit is set to 1, the HC shall check the index field in the response to see if it has the same value as the command index. If it is not, it is reported as a Command Index Error. If this bit is set to 0, the Index field is not checked.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmdidxchken {
    #[doc = "0: Disable"]
    Disable = 0,
    #[doc = "1: Enable"]
    Enable = 1,
}
impl From<Cmdidxchken> for bool {
    #[inline(always)]
    fn from(variant: Cmdidxchken) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMDIDXCHKEN` reader - If this bit is set to 1, the HC shall check the index field in the response to see if it has the same value as the command index. If it is not, it is reported as a Command Index Error. If this bit is set to 0, the Index field is not checked."]
pub type CmdidxchkenR = crate::BitReader<Cmdidxchken>;
impl CmdidxchkenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmdidxchken {
        match self.bits {
            false => Cmdidxchken::Disable,
            true => Cmdidxchken::Enable,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cmdidxchken::Disable
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Cmdidxchken::Enable
    }
}
#[doc = "Field `CMDIDXCHKEN` writer - If this bit is set to 1, the HC shall check the index field in the response to see if it has the same value as the command index. If it is not, it is reported as a Command Index Error. If this bit is set to 0, the Index field is not checked."]
pub type CmdidxchkenW<'a, REG> = crate::BitWriter<'a, REG, Cmdidxchken>;
impl<'a, REG> CmdidxchkenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdidxchken::Disable)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdidxchken::Enable)
    }
}
#[doc = "This bit is set to 1 to indicate that data is present and shall be transferred using the DAT line. If is set to 0 for the following: 1. Commands using only CMD line (ex. CMD52) 2. Commands with no data transfer but using busy signal on DAT\\[0\\]
line (R1b or R5b ex. CMD38) 3. Resume Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dataprsntsel {
    #[doc = "0: No Data Present"]
    Nodatapresent = 0,
    #[doc = "1: Data Present"]
    Datapresent = 1,
}
impl From<Dataprsntsel> for bool {
    #[inline(always)]
    fn from(variant: Dataprsntsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATAPRSNTSEL` reader - This bit is set to 1 to indicate that data is present and shall be transferred using the DAT line. If is set to 0 for the following: 1. Commands using only CMD line (ex. CMD52) 2. Commands with no data transfer but using busy signal on DAT\\[0\\]
line (R1b or R5b ex. CMD38) 3. Resume Command"]
pub type DataprsntselR = crate::BitReader<Dataprsntsel>;
impl DataprsntselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dataprsntsel {
        match self.bits {
            false => Dataprsntsel::Nodatapresent,
            true => Dataprsntsel::Datapresent,
        }
    }
    #[doc = "No Data Present"]
    #[inline(always)]
    pub fn is_nodatapresent(&self) -> bool {
        *self == Dataprsntsel::Nodatapresent
    }
    #[doc = "Data Present"]
    #[inline(always)]
    pub fn is_datapresent(&self) -> bool {
        *self == Dataprsntsel::Datapresent
    }
}
#[doc = "Field `DATAPRSNTSEL` writer - This bit is set to 1 to indicate that data is present and shall be transferred using the DAT line. If is set to 0 for the following: 1. Commands using only CMD line (ex. CMD52) 2. Commands with no data transfer but using busy signal on DAT\\[0\\]
line (R1b or R5b ex. CMD38) 3. Resume Command"]
pub type DataprsntselW<'a, REG> = crate::BitWriter<'a, REG, Dataprsntsel>;
impl<'a, REG> DataprsntselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Data Present"]
    #[inline(always)]
    pub fn nodatapresent(self) -> &'a mut crate::W<REG> {
        self.variant(Dataprsntsel::Nodatapresent)
    }
    #[doc = "Data Present"]
    #[inline(always)]
    pub fn datapresent(self) -> &'a mut crate::W<REG> {
        self.variant(Dataprsntsel::Datapresent)
    }
}
#[doc = "There are three types of special commands. Suspend, Resume and Abort. These bits shall bet set to 00b for all other commands. Suspend Command If the Suspend command succeeds, the HC shall assume the SD Bus has been released and that it is possible to issue the next command which uses the DAT line. The HC shall de-assert Read Wait for read transactions and stop checking busy for write transactions. The Interrupt cycle shall start, in 4-bit mode. If the Suspend command fails, the HC shall maintain its current state. and the HD shall restart the transfer by setting Continue Request in the Block Gap Control Register. Resume Command The HD re-starts the data transfer by restoring the registers in the range of 000-00Dh. The HC shall check for busy before starting write transfers. Abort Command If this command is set when executing a read transfer, the HC shall stop reads to the buffer. If this command is set when executing a write transfer, the HC shall stop driving the DAT line. After issuing the Abort command, the HD should issue a software reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmdtype {
    #[doc = "0: Normal"]
    Normal = 0,
    #[doc = "1: Suspend"]
    Suspend = 1,
    #[doc = "2: Resume"]
    Resume = 2,
    #[doc = "3: Abort"]
    Abort = 3,
}
impl From<Cmdtype> for u8 {
    #[inline(always)]
    fn from(variant: Cmdtype) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmdtype {
    type Ux = u8;
}
impl crate::IsEnum for Cmdtype {}
#[doc = "Field `CMDTYPE` reader - There are three types of special commands. Suspend, Resume and Abort. These bits shall bet set to 00b for all other commands. Suspend Command If the Suspend command succeeds, the HC shall assume the SD Bus has been released and that it is possible to issue the next command which uses the DAT line. The HC shall de-assert Read Wait for read transactions and stop checking busy for write transactions. The Interrupt cycle shall start, in 4-bit mode. If the Suspend command fails, the HC shall maintain its current state. and the HD shall restart the transfer by setting Continue Request in the Block Gap Control Register. Resume Command The HD re-starts the data transfer by restoring the registers in the range of 000-00Dh. The HC shall check for busy before starting write transfers. Abort Command If this command is set when executing a read transfer, the HC shall stop reads to the buffer. If this command is set when executing a write transfer, the HC shall stop driving the DAT line. After issuing the Abort command, the HD should issue a software reset"]
pub type CmdtypeR = crate::FieldReader<Cmdtype>;
impl CmdtypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmdtype {
        match self.bits {
            0 => Cmdtype::Normal,
            1 => Cmdtype::Suspend,
            2 => Cmdtype::Resume,
            3 => Cmdtype::Abort,
            _ => unreachable!(),
        }
    }
    #[doc = "Normal"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Cmdtype::Normal
    }
    #[doc = "Suspend"]
    #[inline(always)]
    pub fn is_suspend(&self) -> bool {
        *self == Cmdtype::Suspend
    }
    #[doc = "Resume"]
    #[inline(always)]
    pub fn is_resume(&self) -> bool {
        *self == Cmdtype::Resume
    }
    #[doc = "Abort"]
    #[inline(always)]
    pub fn is_abort(&self) -> bool {
        *self == Cmdtype::Abort
    }
}
#[doc = "Field `CMDTYPE` writer - There are three types of special commands. Suspend, Resume and Abort. These bits shall bet set to 00b for all other commands. Suspend Command If the Suspend command succeeds, the HC shall assume the SD Bus has been released and that it is possible to issue the next command which uses the DAT line. The HC shall de-assert Read Wait for read transactions and stop checking busy for write transactions. The Interrupt cycle shall start, in 4-bit mode. If the Suspend command fails, the HC shall maintain its current state. and the HD shall restart the transfer by setting Continue Request in the Block Gap Control Register. Resume Command The HD re-starts the data transfer by restoring the registers in the range of 000-00Dh. The HC shall check for busy before starting write transfers. Abort Command If this command is set when executing a read transfer, the HC shall stop reads to the buffer. If this command is set when executing a write transfer, the HC shall stop driving the DAT line. After issuing the Abort command, the HD should issue a software reset"]
pub type CmdtypeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cmdtype, crate::Safe>;
impl<'a, REG> CmdtypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Normal"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdtype::Normal)
    }
    #[doc = "Suspend"]
    #[inline(always)]
    pub fn suspend(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdtype::Suspend)
    }
    #[doc = "Resume"]
    #[inline(always)]
    pub fn resume(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdtype::Resume)
    }
    #[doc = "Abort"]
    #[inline(always)]
    pub fn abort(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdtype::Abort)
    }
}
#[doc = "Field `CMDIDX` reader - This bit shall be set to the command number (CMD0-63, ACMD063)."]
pub type CmdidxR = crate::FieldReader;
#[doc = "Field `CMDIDX` writer - This bit shall be set to the command number (CMD0-63, ACMD063)."]
pub type CmdidxW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 0 - DMA can be enabled only if DMA Support bit in the Capabilities register is set. If this bit is set to 1, a DMA operation shall begin when the HD writes to the upper byte of Command register (00Fh)."]
    #[inline(always)]
    pub fn dmaen(&self) -> DmaenR {
        DmaenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit is used to enable the Block count register, which is only relevant for multiple block transfers. When this bit is 0, the Block Count register is disabled, which is useful in executing an infinite transfer."]
    #[inline(always)]
    pub fn blkcnten(&self) -> BlkcntenR {
        BlkcntenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - This field determines use of auto command functions. There are two methods to stop Multiple-block read and write operation. (1) Auto CMD12 Enable Multiple-block read and write commands for memory require CMD12 to stop the operation. When this field is set to 01b, the Host Controller issues CMD12 automatically when last block transfer is completed. Auto CMD12 error is indicated to the Auto CMD Error Status register. The Host Driver shall not set this bit if the command does not require CMD12. (2) Auto CMD23 Enable When this bit field is set to 10b, the Host Controller issues a CMD23 automatically before issuing a command specified in the Command Register The following conditions are required to use the Auto CMD23."]
    #[inline(always)]
    pub fn acmden(&self) -> AcmdenR {
        AcmdenR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Data Transfer Direction Select. This bit defines the direction of data transfers."]
    #[inline(always)]
    pub fn dxferdirsel(&self) -> DxferdirselR {
        DxferdirselR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This bit enables multiple block data transfers."]
    #[inline(always)]
    pub fn blksel(&self) -> BlkselR {
        BlkselR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Response Type Select"]
    #[inline(always)]
    pub fn resptypesel(&self) -> ResptypeselR {
        ResptypeselR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 19 - If this bit is set to 1, the HC shall check the CRC field in the response. If an error is detected, it is reported as a Command CRC Error. If this bit is set to 0, the CRC field is not checked."]
    #[inline(always)]
    pub fn cmdcrcchken(&self) -> CmdcrcchkenR {
        CmdcrcchkenR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - If this bit is set to 1, the HC shall check the index field in the response to see if it has the same value as the command index. If it is not, it is reported as a Command Index Error. If this bit is set to 0, the Index field is not checked."]
    #[inline(always)]
    pub fn cmdidxchken(&self) -> CmdidxchkenR {
        CmdidxchkenR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - This bit is set to 1 to indicate that data is present and shall be transferred using the DAT line. If is set to 0 for the following: 1. Commands using only CMD line (ex. CMD52) 2. Commands with no data transfer but using busy signal on DAT\\[0\\]
line (R1b or R5b ex. CMD38) 3. Resume Command"]
    #[inline(always)]
    pub fn dataprsntsel(&self) -> DataprsntselR {
        DataprsntselR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:23 - There are three types of special commands. Suspend, Resume and Abort. These bits shall bet set to 00b for all other commands. Suspend Command If the Suspend command succeeds, the HC shall assume the SD Bus has been released and that it is possible to issue the next command which uses the DAT line. The HC shall de-assert Read Wait for read transactions and stop checking busy for write transactions. The Interrupt cycle shall start, in 4-bit mode. If the Suspend command fails, the HC shall maintain its current state. and the HD shall restart the transfer by setting Continue Request in the Block Gap Control Register. Resume Command The HD re-starts the data transfer by restoring the registers in the range of 000-00Dh. The HC shall check for busy before starting write transfers. Abort Command If this command is set when executing a read transfer, the HC shall stop reads to the buffer. If this command is set when executing a write transfer, the HC shall stop driving the DAT line. After issuing the Abort command, the HD should issue a software reset"]
    #[inline(always)]
    pub fn cmdtype(&self) -> CmdtypeR {
        CmdtypeR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:29 - This bit shall be set to the command number (CMD0-63, ACMD063)."]
    #[inline(always)]
    pub fn cmdidx(&self) -> CmdidxR {
        CmdidxR::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - DMA can be enabled only if DMA Support bit in the Capabilities register is set. If this bit is set to 1, a DMA operation shall begin when the HD writes to the upper byte of Command register (00Fh)."]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DmaenW<TransferSpec> {
        DmaenW::new(self, 0)
    }
    #[doc = "Bit 1 - This bit is used to enable the Block count register, which is only relevant for multiple block transfers. When this bit is 0, the Block Count register is disabled, which is useful in executing an infinite transfer."]
    #[inline(always)]
    #[must_use]
    pub fn blkcnten(&mut self) -> BlkcntenW<TransferSpec> {
        BlkcntenW::new(self, 1)
    }
    #[doc = "Bits 2:3 - This field determines use of auto command functions. There are two methods to stop Multiple-block read and write operation. (1) Auto CMD12 Enable Multiple-block read and write commands for memory require CMD12 to stop the operation. When this field is set to 01b, the Host Controller issues CMD12 automatically when last block transfer is completed. Auto CMD12 error is indicated to the Auto CMD Error Status register. The Host Driver shall not set this bit if the command does not require CMD12. (2) Auto CMD23 Enable When this bit field is set to 10b, the Host Controller issues a CMD23 automatically before issuing a command specified in the Command Register The following conditions are required to use the Auto CMD23."]
    #[inline(always)]
    #[must_use]
    pub fn acmden(&mut self) -> AcmdenW<TransferSpec> {
        AcmdenW::new(self, 2)
    }
    #[doc = "Bit 4 - Data Transfer Direction Select. This bit defines the direction of data transfers."]
    #[inline(always)]
    #[must_use]
    pub fn dxferdirsel(&mut self) -> DxferdirselW<TransferSpec> {
        DxferdirselW::new(self, 4)
    }
    #[doc = "Bit 5 - This bit enables multiple block data transfers."]
    #[inline(always)]
    #[must_use]
    pub fn blksel(&mut self) -> BlkselW<TransferSpec> {
        BlkselW::new(self, 5)
    }
    #[doc = "Bits 16:17 - Response Type Select"]
    #[inline(always)]
    #[must_use]
    pub fn resptypesel(&mut self) -> ResptypeselW<TransferSpec> {
        ResptypeselW::new(self, 16)
    }
    #[doc = "Bit 19 - If this bit is set to 1, the HC shall check the CRC field in the response. If an error is detected, it is reported as a Command CRC Error. If this bit is set to 0, the CRC field is not checked."]
    #[inline(always)]
    #[must_use]
    pub fn cmdcrcchken(&mut self) -> CmdcrcchkenW<TransferSpec> {
        CmdcrcchkenW::new(self, 19)
    }
    #[doc = "Bit 20 - If this bit is set to 1, the HC shall check the index field in the response to see if it has the same value as the command index. If it is not, it is reported as a Command Index Error. If this bit is set to 0, the Index field is not checked."]
    #[inline(always)]
    #[must_use]
    pub fn cmdidxchken(&mut self) -> CmdidxchkenW<TransferSpec> {
        CmdidxchkenW::new(self, 20)
    }
    #[doc = "Bit 21 - This bit is set to 1 to indicate that data is present and shall be transferred using the DAT line. If is set to 0 for the following: 1. Commands using only CMD line (ex. CMD52) 2. Commands with no data transfer but using busy signal on DAT\\[0\\]
line (R1b or R5b ex. CMD38) 3. Resume Command"]
    #[inline(always)]
    #[must_use]
    pub fn dataprsntsel(&mut self) -> DataprsntselW<TransferSpec> {
        DataprsntselW::new(self, 21)
    }
    #[doc = "Bits 22:23 - There are three types of special commands. Suspend, Resume and Abort. These bits shall bet set to 00b for all other commands. Suspend Command If the Suspend command succeeds, the HC shall assume the SD Bus has been released and that it is possible to issue the next command which uses the DAT line. The HC shall de-assert Read Wait for read transactions and stop checking busy for write transactions. The Interrupt cycle shall start, in 4-bit mode. If the Suspend command fails, the HC shall maintain its current state. and the HD shall restart the transfer by setting Continue Request in the Block Gap Control Register. Resume Command The HD re-starts the data transfer by restoring the registers in the range of 000-00Dh. The HC shall check for busy before starting write transfers. Abort Command If this command is set when executing a read transfer, the HC shall stop reads to the buffer. If this command is set when executing a write transfer, the HC shall stop driving the DAT line. After issuing the Abort command, the HD should issue a software reset"]
    #[inline(always)]
    #[must_use]
    pub fn cmdtype(&mut self) -> CmdtypeW<TransferSpec> {
        CmdtypeW::new(self, 22)
    }
    #[doc = "Bits 24:29 - This bit shall be set to the command number (CMD0-63, ACMD063)."]
    #[inline(always)]
    #[must_use]
    pub fn cmdidx(&mut self) -> CmdidxW<TransferSpec> {
        CmdidxW::new(self, 24)
    }
}
#[doc = "Transfer mode\n\nYou can [`read`](crate::Reg::read) this register and get [`transfer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`transfer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TransferSpec;
impl crate::RegisterSpec for TransferSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`transfer::R`](R) reader structure"]
impl crate::Readable for TransferSpec {}
#[doc = "`write(|w| ..)` method takes [`transfer::W`](W) writer structure"]
impl crate::Writable for TransferSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRANSFER to value 0"]
impl crate::Resettable for TransferSpec {
    const RESET_VALUE: u32 = 0;
}
