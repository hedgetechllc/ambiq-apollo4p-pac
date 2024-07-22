#[doc = "Register `CMD` reader"]
pub type R = crate::R<CmdSpec>;
#[doc = "Register `CMD` writer"]
pub type W = crate::W<CmdSpec>;
#[doc = "Command for submodule.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmd {
    #[doc = "1: Write command using count of offset bytes specified in the OFFSETCNT field"]
    Write = 1,
    #[doc = "2: Read command using count of offset bytes specified in the OFFSETCNT field"]
    Read = 2,
    #[doc = "3: SPI only. Test mode to do constant write operations. Useful for debug and power measurements. Will continually send data in OFFSET field"]
    Tmw = 3,
    #[doc = "4: SPI Only. Test mode to do constant read operations. Useful for debug and power measurements. Will continually read data from external input"]
    Tmr = 4,
}
impl From<Cmd> for u8 {
    #[inline(always)]
    fn from(variant: Cmd) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmd {
    type Ux = u8;
}
impl crate::IsEnum for Cmd {}
#[doc = "Field `CMD` reader - Command for submodule."]
pub type CmdR = crate::FieldReader<Cmd>;
impl CmdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cmd> {
        match self.bits {
            1 => Some(Cmd::Write),
            2 => Some(Cmd::Read),
            3 => Some(Cmd::Tmw),
            4 => Some(Cmd::Tmr),
            _ => None,
        }
    }
    #[doc = "Write command using count of offset bytes specified in the OFFSETCNT field"]
    #[inline(always)]
    pub fn is_write(&self) -> bool {
        *self == Cmd::Write
    }
    #[doc = "Read command using count of offset bytes specified in the OFFSETCNT field"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == Cmd::Read
    }
    #[doc = "SPI only. Test mode to do constant write operations. Useful for debug and power measurements. Will continually send data in OFFSET field"]
    #[inline(always)]
    pub fn is_tmw(&self) -> bool {
        *self == Cmd::Tmw
    }
    #[doc = "SPI Only. Test mode to do constant read operations. Useful for debug and power measurements. Will continually read data from external input"]
    #[inline(always)]
    pub fn is_tmr(&self) -> bool {
        *self == Cmd::Tmr
    }
}
#[doc = "Field `CMD` writer - Command for submodule."]
pub type CmdW<'a, REG> = crate::FieldWriter<'a, REG, 4, Cmd>;
impl<'a, REG> CmdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Write command using count of offset bytes specified in the OFFSETCNT field"]
    #[inline(always)]
    pub fn write(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd::Write)
    }
    #[doc = "Read command using count of offset bytes specified in the OFFSETCNT field"]
    #[inline(always)]
    pub fn read(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd::Read)
    }
    #[doc = "SPI only. Test mode to do constant write operations. Useful for debug and power measurements. Will continually send data in OFFSET field"]
    #[inline(always)]
    pub fn tmw(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd::Tmw)
    }
    #[doc = "SPI Only. Test mode to do constant read operations. Useful for debug and power measurements. Will continually read data from external input"]
    #[inline(always)]
    pub fn tmr(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd::Tmr)
    }
}
#[doc = "Field `OFFSETCNT` reader - Number of offset bytes to use for the command - 0, 1, 2, 3, 4, 5 are valid selections. The second (byte 1),third (byte 2), and forth (byte 3) are read from the OFFSETHI register, and the low order byte is pulled from this register in the OFFSETLO field. Offset bytes are transmitted highest byte first. EG if offsetcnt == 4, OFFSETHI\\[23:16\\]
will be transmitted first, then OFFSETHI\\[15:8\\], then OFFSETHI\\[7:0\\]
then OFFSETLO. If offsetcnt == 5, OFFSETHI\\[31:24\\]
will be transmitted, then OFFSETHI\\[23:0\\], then OFFSETLO. If offsetcnt == 4, OFFSETHI\\[23:16\\]
will be transmitted, then OFFSETHI\\[15:0\\], then OFFSETLO. If offsetcnt == 3, OFFSETHI\\[15:8\\]
will be transmitted, then OFFSETHI\\[7:0\\], then OFFSETLO. If offsetcnt == 2, OFFSETHI\\[7:0\\]
will be transmitted, then OFFSETLO. If offsetcnt == 1, only OFFSETLO will be transmitted. Offset bytes are always transmitted MSB first, regardless of the value of the LSB control bit within the module configuration."]
pub type OffsetcntR = crate::FieldReader;
#[doc = "Field `OFFSETCNT` writer - Number of offset bytes to use for the command - 0, 1, 2, 3, 4, 5 are valid selections. The second (byte 1),third (byte 2), and forth (byte 3) are read from the OFFSETHI register, and the low order byte is pulled from this register in the OFFSETLO field. Offset bytes are transmitted highest byte first. EG if offsetcnt == 4, OFFSETHI\\[23:16\\]
will be transmitted first, then OFFSETHI\\[15:8\\], then OFFSETHI\\[7:0\\]
then OFFSETLO. If offsetcnt == 5, OFFSETHI\\[31:24\\]
will be transmitted, then OFFSETHI\\[23:0\\], then OFFSETLO. If offsetcnt == 4, OFFSETHI\\[23:16\\]
will be transmitted, then OFFSETHI\\[15:0\\], then OFFSETLO. If offsetcnt == 3, OFFSETHI\\[15:8\\]
will be transmitted, then OFFSETHI\\[7:0\\], then OFFSETLO. If offsetcnt == 2, OFFSETHI\\[7:0\\]
will be transmitted, then OFFSETLO. If offsetcnt == 1, only OFFSETLO will be transmitted. Offset bytes are always transmitted MSB first, regardless of the value of the LSB control bit within the module configuration."]
pub type OffsetcntW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CONT` reader - Contine to hold the bus after the current transaction if set to a 1 with a new command issued."]
pub type ContR = crate::BitReader;
#[doc = "Field `CONT` writer - Contine to hold the bus after the current transaction if set to a 1 with a new command issued."]
pub type ContW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSIZE` reader - Defines the transaction size in bytes. The offset transfer is not included in this size."]
pub type TsizeR = crate::FieldReader<u16>;
#[doc = "Field `TSIZE` writer - Defines the transaction size in bytes. The offset transfer is not included in this size."]
pub type TsizeW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `CMDSEL` reader - Command Specific selection information. Not used in Master I2C. Used as CEn select for Master SPI transactions"]
pub type CmdselR = crate::FieldReader;
#[doc = "Field `CMDSEL` writer - Command Specific selection information. Not used in Master I2C. Used as CEn select for Master SPI transactions"]
pub type CmdselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OFFSETLO` reader - This register holds the low order byte of offset to be used in the transaction. The number of offset bytes to use is set with bits 1:0 of the command."]
pub type OffsetloR = crate::FieldReader;
#[doc = "Field `OFFSETLO` writer - This register holds the low order byte of offset to be used in the transaction. The number of offset bytes to use is set with bits 1:0 of the command."]
pub type OffsetloW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:3 - Command for submodule."]
    #[inline(always)]
    pub fn cmd(&self) -> CmdR {
        CmdR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Number of offset bytes to use for the command - 0, 1, 2, 3, 4, 5 are valid selections. The second (byte 1),third (byte 2), and forth (byte 3) are read from the OFFSETHI register, and the low order byte is pulled from this register in the OFFSETLO field. Offset bytes are transmitted highest byte first. EG if offsetcnt == 4, OFFSETHI\\[23:16\\]
will be transmitted first, then OFFSETHI\\[15:8\\], then OFFSETHI\\[7:0\\]
then OFFSETLO. If offsetcnt == 5, OFFSETHI\\[31:24\\]
will be transmitted, then OFFSETHI\\[23:0\\], then OFFSETLO. If offsetcnt == 4, OFFSETHI\\[23:16\\]
will be transmitted, then OFFSETHI\\[15:0\\], then OFFSETLO. If offsetcnt == 3, OFFSETHI\\[15:8\\]
will be transmitted, then OFFSETHI\\[7:0\\], then OFFSETLO. If offsetcnt == 2, OFFSETHI\\[7:0\\]
will be transmitted, then OFFSETLO. If offsetcnt == 1, only OFFSETLO will be transmitted. Offset bytes are always transmitted MSB first, regardless of the value of the LSB control bit within the module configuration."]
    #[inline(always)]
    pub fn offsetcnt(&self) -> OffsetcntR {
        OffsetcntR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Contine to hold the bus after the current transaction if set to a 1 with a new command issued."]
    #[inline(always)]
    pub fn cont(&self) -> ContR {
        ContR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:19 - Defines the transaction size in bytes. The offset transfer is not included in this size."]
    #[inline(always)]
    pub fn tsize(&self) -> TsizeR {
        TsizeR::new(((self.bits >> 8) & 0x0fff) as u16)
    }
    #[doc = "Bits 20:21 - Command Specific selection information. Not used in Master I2C. Used as CEn select for Master SPI transactions"]
    #[inline(always)]
    pub fn cmdsel(&self) -> CmdselR {
        CmdselR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:31 - This register holds the low order byte of offset to be used in the transaction. The number of offset bytes to use is set with bits 1:0 of the command."]
    #[inline(always)]
    pub fn offsetlo(&self) -> OffsetloR {
        OffsetloR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Command for submodule."]
    #[inline(always)]
    #[must_use]
    pub fn cmd(&mut self) -> CmdW<CmdSpec> {
        CmdW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Number of offset bytes to use for the command - 0, 1, 2, 3, 4, 5 are valid selections. The second (byte 1),third (byte 2), and forth (byte 3) are read from the OFFSETHI register, and the low order byte is pulled from this register in the OFFSETLO field. Offset bytes are transmitted highest byte first. EG if offsetcnt == 4, OFFSETHI\\[23:16\\]
will be transmitted first, then OFFSETHI\\[15:8\\], then OFFSETHI\\[7:0\\]
then OFFSETLO. If offsetcnt == 5, OFFSETHI\\[31:24\\]
will be transmitted, then OFFSETHI\\[23:0\\], then OFFSETLO. If offsetcnt == 4, OFFSETHI\\[23:16\\]
will be transmitted, then OFFSETHI\\[15:0\\], then OFFSETLO. If offsetcnt == 3, OFFSETHI\\[15:8\\]
will be transmitted, then OFFSETHI\\[7:0\\], then OFFSETLO. If offsetcnt == 2, OFFSETHI\\[7:0\\]
will be transmitted, then OFFSETLO. If offsetcnt == 1, only OFFSETLO will be transmitted. Offset bytes are always transmitted MSB first, regardless of the value of the LSB control bit within the module configuration."]
    #[inline(always)]
    #[must_use]
    pub fn offsetcnt(&mut self) -> OffsetcntW<CmdSpec> {
        OffsetcntW::new(self, 4)
    }
    #[doc = "Bit 7 - Contine to hold the bus after the current transaction if set to a 1 with a new command issued."]
    #[inline(always)]
    #[must_use]
    pub fn cont(&mut self) -> ContW<CmdSpec> {
        ContW::new(self, 7)
    }
    #[doc = "Bits 8:19 - Defines the transaction size in bytes. The offset transfer is not included in this size."]
    #[inline(always)]
    #[must_use]
    pub fn tsize(&mut self) -> TsizeW<CmdSpec> {
        TsizeW::new(self, 8)
    }
    #[doc = "Bits 20:21 - Command Specific selection information. Not used in Master I2C. Used as CEn select for Master SPI transactions"]
    #[inline(always)]
    #[must_use]
    pub fn cmdsel(&mut self) -> CmdselW<CmdSpec> {
        CmdselW::new(self, 20)
    }
    #[doc = "Bits 24:31 - This register holds the low order byte of offset to be used in the transaction. The number of offset bytes to use is set with bits 1:0 of the command."]
    #[inline(always)]
    #[must_use]
    pub fn offsetlo(&mut self) -> OffsetloW<CmdSpec> {
        OffsetloW::new(self, 24)
    }
}
#[doc = "Writes to this register will start an IO transaction, as well as set various parameters for the command itself. Reads will return the command value written to the CMD register. To read the number of bytes that have yet to be transferred, refer to the CTSIZE field within the CMDSTAT register.\n\nYou can [`read`](crate::Reg::read) this register and get [`cmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmdSpec;
impl crate::RegisterSpec for CmdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd::R`](R) reader structure"]
impl crate::Readable for CmdSpec {}
#[doc = "`write(|w| ..)` method takes [`cmd::W`](W) writer structure"]
impl crate::Writable for CmdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CmdSpec {
    const RESET_VALUE: u32 = 0;
}
