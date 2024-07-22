#[doc = "Register `INTENABLE` reader"]
pub type R = crate::R<IntenableSpec>;
#[doc = "Register `INTENABLE` writer"]
pub type W = crate::W<IntenableSpec>;
#[doc = "Field `COMMANDCOMPLETESTATUSENABLE` reader - Description"]
pub type CommandcompletestatusenableR = crate::BitReader;
#[doc = "Field `COMMANDCOMPLETESTATUSENABLE` writer - Description"]
pub type CommandcompletestatusenableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRANSFERCOMPLETESTATUSENABLE` reader - Description"]
pub type TransfercompletestatusenableR = crate::BitReader;
#[doc = "Field `TRANSFERCOMPLETESTATUSENABLE` writer - Description"]
pub type TransfercompletestatusenableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLOCKGAPEVENTSTATUSENABLE` reader - Description"]
pub type BlockgapeventstatusenableR = crate::BitReader;
#[doc = "Field `BLOCKGAPEVENTSTATUSENABLE` writer - Description"]
pub type BlockgapeventstatusenableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAINTERRUPTSTATUSENABLE` reader - Description"]
pub type DmainterruptstatusenableR = crate::BitReader;
#[doc = "Field `DMAINTERRUPTSTATUSENABLE` writer - Description"]
pub type DmainterruptstatusenableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUFFERWRITEREADYSTATUSENABLE` reader - Description"]
pub type BufferwritereadystatusenableR = crate::BitReader;
#[doc = "Field `BUFFERWRITEREADYSTATUSENABLE` writer - Description"]
pub type BufferwritereadystatusenableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUFFERREADREADYSTATUSENABLE` reader - Description"]
pub type BufferreadreadystatusenableR = crate::BitReader;
#[doc = "Field `BUFFERREADREADYSTATUSENABLE` writer - Description"]
pub type BufferreadreadystatusenableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CARDINSERTIONSTATUSENABLE` reader - Description"]
pub type CardinsertionstatusenableR = crate::BitReader;
#[doc = "Field `CARDINSERTIONSTATUSENABLE` writer - Description"]
pub type CardinsertionstatusenableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CARDREMOVALSTATUSENABLE` reader - Description"]
pub type CardremovalstatusenableR = crate::BitReader;
#[doc = "Field `CARDREMOVALSTATUSENABLE` writer - Description"]
pub type CardremovalstatusenableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CARDINTERRUPTSTATUSENABLE` reader - If this bit is set to 0, the HC shall clear Interrupt request to the System. The Card Interrupt detection is stopped when this bit is cleared and restarted when this bit is set to 1. The HD may clear the Card Interrupt Status Enable before servicing the Card Interrupt and may set this bit again after all Interrupt requests from the card are cleared to prevent inadvertent Interrupts."]
pub type CardinterruptstatusenableR = crate::BitReader;
#[doc = "Field `CARDINTERRUPTSTATUSENABLE` writer - If this bit is set to 0, the HC shall clear Interrupt request to the System. The Card Interrupt detection is stopped when this bit is cleared and restarted when this bit is set to 1. The HD may clear the Card Interrupt Status Enable before servicing the Card Interrupt and may set this bit again after all Interrupt requests from the card are cleared to prevent inadvertent Interrupts."]
pub type CardinterruptstatusenableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTASTATUSENABLE` reader - If this bit is set to 0, the Host Controller shall clear the interrupt request to the System. The Host Driver may clear this bit before servicing the INT_A and may set this bit again after all interrupt requests to INT_A pin are cleared to prevent inadvertent interrupts."]
pub type IntastatusenableR = crate::BitReader;
#[doc = "Field `INTASTATUSENABLE` writer - If this bit is set to 0, the Host Controller shall clear the interrupt request to the System. The Host Driver may clear this bit before servicing the INT_A and may set this bit again after all interrupt requests to INT_A pin are cleared to prevent inadvertent interrupts."]
pub type IntastatusenableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTBSTATUSENABLE` reader - If this bit is set to 0, the Host Controller shall clear the interrupt request to the System. The Host Driver may clear this bit before servicing the INT_B and may set this bit again after all interrupt requests to INT_B pin are cleared to prevent inadvertent interrupts."]
pub type IntbstatusenableR = crate::BitReader;
#[doc = "Field `INTBSTATUSENABLE` writer - If this bit is set to 0, the Host Controller shall clear the interrupt request to the System. The Host Driver may clear this bit before servicing the INT_B and may set this bit again after all interrupt requests to INT_B pin are cleared to prevent inadvertent interrupts."]
pub type IntbstatusenableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTCSTATUSENABLE` reader - If this bit is set to 0, the Host Controller shall clear the interrupt request to the System. The Host Driver may clear this bit before servicing the INT_C and may set this bit again after all interrupt requests to INT_C pin are cleared to prevent inadvertent interrupts. Interrupt enable"]
pub type IntcstatusenableR = crate::BitReader;
#[doc = "Field `INTCSTATUSENABLE` writer - If this bit is set to 0, the Host Controller shall clear the interrupt request to the System. The Host Driver may clear this bit before servicing the INT_C and may set this bit again after all interrupt requests to INT_C pin are cleared to prevent inadvertent interrupts. Interrupt enable"]
pub type IntcstatusenableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RETUNINGEVENTSTATUSENABLE` reader - Interrupt"]
pub type RetuningeventstatusenableR = crate::BitReader;
#[doc = "Field `RETUNINGEVENTSTATUSENABLE` writer - Interrupt"]
pub type RetuningeventstatusenableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOOTACKRCVENABLE` reader - Interrupt"]
pub type BootackrcvenableR = crate::BitReader;
#[doc = "Field `BOOTACKRCVENABLE` writer - Interrupt"]
pub type BootackrcvenableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOOTTERMINATE` reader - Boot is terminated?"]
pub type BootterminateR = crate::BitReader;
#[doc = "Field `BOOTTERMINATE` writer - Boot is terminated?"]
pub type BootterminateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIXEDTO0` reader - The HC shall control error Interrupts using the Error Interrupt Status Enable register."]
pub type Fixedto0R = crate::BitReader;
#[doc = "Field `FIXEDTO0` writer - The HC shall control error Interrupts using the Error Interrupt Status Enable register."]
pub type Fixedto0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMMANDTIMEOUTERRORSTATUSENABLE` reader - Desc"]
pub type CommandtimeouterrorstatusenableR = crate::BitReader;
#[doc = "Field `COMMANDTIMEOUTERRORSTATUSENABLE` writer - Desc"]
pub type CommandtimeouterrorstatusenableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMMANDCRCERRORSTATUSENABLE` reader - Desc"]
pub type CommandcrcerrorstatusenableR = crate::BitReader;
#[doc = "Field `COMMANDCRCERRORSTATUSENABLE` writer - Desc"]
pub type CommandcrcerrorstatusenableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMMANDENDBITERRORSTATUSENABLE` reader - Desc"]
pub type CommandendbiterrorstatusenableR = crate::BitReader;
#[doc = "Field `COMMANDENDBITERRORSTATUSENABLE` writer - Desc"]
pub type CommandendbiterrorstatusenableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMMANDINDEXERRORSTATUSENABLE` reader - Desc"]
pub type CommandindexerrorstatusenableR = crate::BitReader;
#[doc = "Field `COMMANDINDEXERRORSTATUSENABLE` writer - Desc"]
pub type CommandindexerrorstatusenableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATATIMEOUTERRORSTATUSENABLE` reader - Desc"]
pub type DatatimeouterrorstatusenableR = crate::BitReader;
#[doc = "Field `DATATIMEOUTERRORSTATUSENABLE` writer - Desc"]
pub type DatatimeouterrorstatusenableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATACRCERRORSTATUSENABLE` reader - Desc"]
pub type DatacrcerrorstatusenableR = crate::BitReader;
#[doc = "Field `DATACRCERRORSTATUSENABLE` writer - Desc"]
pub type DatacrcerrorstatusenableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATAENDBITERRORSTATUSENABLE` reader - Desc"]
pub type DataendbiterrorstatusenableR = crate::BitReader;
#[doc = "Field `DATAENDBITERRORSTATUSENABLE` writer - Desc"]
pub type DataendbiterrorstatusenableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CURRENTLIMITERRORSTATUSENABLE` reader - Desc"]
pub type CurrentlimiterrorstatusenableR = crate::BitReader;
#[doc = "Field `CURRENTLIMITERRORSTATUSENABLE` writer - Desc"]
pub type CurrentlimiterrorstatusenableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTOCMD12ERRORSTATUSENABLE` reader - Desc"]
pub type Autocmd12errorstatusenableR = crate::BitReader;
#[doc = "Field `AUTOCMD12ERRORSTATUSENABLE` writer - Desc"]
pub type Autocmd12errorstatusenableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADMAERRORSTATUSENABLE` reader - Desc"]
pub type AdmaerrorstatusenableR = crate::BitReader;
#[doc = "Field `ADMAERRORSTATUSENABLE` writer - Desc"]
pub type AdmaerrorstatusenableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TUNINGERRORSTATUS` reader - enable"]
pub type TuningerrorstatusR = crate::BitReader;
#[doc = "Field `TUNINGERRORSTATUS` writer - enable"]
pub type TuningerrorstatusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TGTRESPERRHOSTERRSTATEN` reader - Desc"]
pub type TgtresperrhosterrstatenR = crate::BitReader;
#[doc = "Field `TGTRESPERRHOSTERRSTATEN` writer - Desc"]
pub type TgtresperrhosterrstatenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VENDORSPECIFICERRORSTATUSENABLE` reader - Vendor-specific error status enable."]
pub type VendorspecificerrorstatusenableR = crate::FieldReader;
#[doc = "Field `VENDORSPECIFICERRORSTATUSENABLE` writer - Vendor-specific error status enable."]
pub type VendorspecificerrorstatusenableW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - Description"]
    #[inline(always)]
    pub fn commandcompletestatusenable(&self) -> CommandcompletestatusenableR {
        CommandcompletestatusenableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Description"]
    #[inline(always)]
    pub fn transfercompletestatusenable(&self) -> TransfercompletestatusenableR {
        TransfercompletestatusenableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Description"]
    #[inline(always)]
    pub fn blockgapeventstatusenable(&self) -> BlockgapeventstatusenableR {
        BlockgapeventstatusenableR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Description"]
    #[inline(always)]
    pub fn dmainterruptstatusenable(&self) -> DmainterruptstatusenableR {
        DmainterruptstatusenableR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Description"]
    #[inline(always)]
    pub fn bufferwritereadystatusenable(&self) -> BufferwritereadystatusenableR {
        BufferwritereadystatusenableR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Description"]
    #[inline(always)]
    pub fn bufferreadreadystatusenable(&self) -> BufferreadreadystatusenableR {
        BufferreadreadystatusenableR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Description"]
    #[inline(always)]
    pub fn cardinsertionstatusenable(&self) -> CardinsertionstatusenableR {
        CardinsertionstatusenableR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Description"]
    #[inline(always)]
    pub fn cardremovalstatusenable(&self) -> CardremovalstatusenableR {
        CardremovalstatusenableR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - If this bit is set to 0, the HC shall clear Interrupt request to the System. The Card Interrupt detection is stopped when this bit is cleared and restarted when this bit is set to 1. The HD may clear the Card Interrupt Status Enable before servicing the Card Interrupt and may set this bit again after all Interrupt requests from the card are cleared to prevent inadvertent Interrupts."]
    #[inline(always)]
    pub fn cardinterruptstatusenable(&self) -> CardinterruptstatusenableR {
        CardinterruptstatusenableR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - If this bit is set to 0, the Host Controller shall clear the interrupt request to the System. The Host Driver may clear this bit before servicing the INT_A and may set this bit again after all interrupt requests to INT_A pin are cleared to prevent inadvertent interrupts."]
    #[inline(always)]
    pub fn intastatusenable(&self) -> IntastatusenableR {
        IntastatusenableR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - If this bit is set to 0, the Host Controller shall clear the interrupt request to the System. The Host Driver may clear this bit before servicing the INT_B and may set this bit again after all interrupt requests to INT_B pin are cleared to prevent inadvertent interrupts."]
    #[inline(always)]
    pub fn intbstatusenable(&self) -> IntbstatusenableR {
        IntbstatusenableR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - If this bit is set to 0, the Host Controller shall clear the interrupt request to the System. The Host Driver may clear this bit before servicing the INT_C and may set this bit again after all interrupt requests to INT_C pin are cleared to prevent inadvertent interrupts. Interrupt enable"]
    #[inline(always)]
    pub fn intcstatusenable(&self) -> IntcstatusenableR {
        IntcstatusenableR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Interrupt"]
    #[inline(always)]
    pub fn retuningeventstatusenable(&self) -> RetuningeventstatusenableR {
        RetuningeventstatusenableR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt"]
    #[inline(always)]
    pub fn bootackrcvenable(&self) -> BootackrcvenableR {
        BootackrcvenableR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Boot is terminated?"]
    #[inline(always)]
    pub fn bootterminate(&self) -> BootterminateR {
        BootterminateR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - The HC shall control error Interrupts using the Error Interrupt Status Enable register."]
    #[inline(always)]
    pub fn fixedto0(&self) -> Fixedto0R {
        Fixedto0R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Desc"]
    #[inline(always)]
    pub fn commandtimeouterrorstatusenable(&self) -> CommandtimeouterrorstatusenableR {
        CommandtimeouterrorstatusenableR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Desc"]
    #[inline(always)]
    pub fn commandcrcerrorstatusenable(&self) -> CommandcrcerrorstatusenableR {
        CommandcrcerrorstatusenableR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Desc"]
    #[inline(always)]
    pub fn commandendbiterrorstatusenable(&self) -> CommandendbiterrorstatusenableR {
        CommandendbiterrorstatusenableR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Desc"]
    #[inline(always)]
    pub fn commandindexerrorstatusenable(&self) -> CommandindexerrorstatusenableR {
        CommandindexerrorstatusenableR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Desc"]
    #[inline(always)]
    pub fn datatimeouterrorstatusenable(&self) -> DatatimeouterrorstatusenableR {
        DatatimeouterrorstatusenableR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Desc"]
    #[inline(always)]
    pub fn datacrcerrorstatusenable(&self) -> DatacrcerrorstatusenableR {
        DatacrcerrorstatusenableR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Desc"]
    #[inline(always)]
    pub fn dataendbiterrorstatusenable(&self) -> DataendbiterrorstatusenableR {
        DataendbiterrorstatusenableR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Desc"]
    #[inline(always)]
    pub fn currentlimiterrorstatusenable(&self) -> CurrentlimiterrorstatusenableR {
        CurrentlimiterrorstatusenableR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Desc"]
    #[inline(always)]
    pub fn autocmd12errorstatusenable(&self) -> Autocmd12errorstatusenableR {
        Autocmd12errorstatusenableR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Desc"]
    #[inline(always)]
    pub fn admaerrorstatusenable(&self) -> AdmaerrorstatusenableR {
        AdmaerrorstatusenableR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - enable"]
    #[inline(always)]
    pub fn tuningerrorstatus(&self) -> TuningerrorstatusR {
        TuningerrorstatusR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - Desc"]
    #[inline(always)]
    pub fn tgtresperrhosterrstaten(&self) -> TgtresperrhosterrstatenR {
        TgtresperrhosterrstatenR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:31 - Vendor-specific error status enable."]
    #[inline(always)]
    pub fn vendorspecificerrorstatusenable(&self) -> VendorspecificerrorstatusenableR {
        VendorspecificerrorstatusenableR::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Description"]
    #[inline(always)]
    #[must_use]
    pub fn commandcompletestatusenable(&mut self) -> CommandcompletestatusenableW<IntenableSpec> {
        CommandcompletestatusenableW::new(self, 0)
    }
    #[doc = "Bit 1 - Description"]
    #[inline(always)]
    #[must_use]
    pub fn transfercompletestatusenable(&mut self) -> TransfercompletestatusenableW<IntenableSpec> {
        TransfercompletestatusenableW::new(self, 1)
    }
    #[doc = "Bit 2 - Description"]
    #[inline(always)]
    #[must_use]
    pub fn blockgapeventstatusenable(&mut self) -> BlockgapeventstatusenableW<IntenableSpec> {
        BlockgapeventstatusenableW::new(self, 2)
    }
    #[doc = "Bit 3 - Description"]
    #[inline(always)]
    #[must_use]
    pub fn dmainterruptstatusenable(&mut self) -> DmainterruptstatusenableW<IntenableSpec> {
        DmainterruptstatusenableW::new(self, 3)
    }
    #[doc = "Bit 4 - Description"]
    #[inline(always)]
    #[must_use]
    pub fn bufferwritereadystatusenable(&mut self) -> BufferwritereadystatusenableW<IntenableSpec> {
        BufferwritereadystatusenableW::new(self, 4)
    }
    #[doc = "Bit 5 - Description"]
    #[inline(always)]
    #[must_use]
    pub fn bufferreadreadystatusenable(&mut self) -> BufferreadreadystatusenableW<IntenableSpec> {
        BufferreadreadystatusenableW::new(self, 5)
    }
    #[doc = "Bit 6 - Description"]
    #[inline(always)]
    #[must_use]
    pub fn cardinsertionstatusenable(&mut self) -> CardinsertionstatusenableW<IntenableSpec> {
        CardinsertionstatusenableW::new(self, 6)
    }
    #[doc = "Bit 7 - Description"]
    #[inline(always)]
    #[must_use]
    pub fn cardremovalstatusenable(&mut self) -> CardremovalstatusenableW<IntenableSpec> {
        CardremovalstatusenableW::new(self, 7)
    }
    #[doc = "Bit 8 - If this bit is set to 0, the HC shall clear Interrupt request to the System. The Card Interrupt detection is stopped when this bit is cleared and restarted when this bit is set to 1. The HD may clear the Card Interrupt Status Enable before servicing the Card Interrupt and may set this bit again after all Interrupt requests from the card are cleared to prevent inadvertent Interrupts."]
    #[inline(always)]
    #[must_use]
    pub fn cardinterruptstatusenable(&mut self) -> CardinterruptstatusenableW<IntenableSpec> {
        CardinterruptstatusenableW::new(self, 8)
    }
    #[doc = "Bit 9 - If this bit is set to 0, the Host Controller shall clear the interrupt request to the System. The Host Driver may clear this bit before servicing the INT_A and may set this bit again after all interrupt requests to INT_A pin are cleared to prevent inadvertent interrupts."]
    #[inline(always)]
    #[must_use]
    pub fn intastatusenable(&mut self) -> IntastatusenableW<IntenableSpec> {
        IntastatusenableW::new(self, 9)
    }
    #[doc = "Bit 10 - If this bit is set to 0, the Host Controller shall clear the interrupt request to the System. The Host Driver may clear this bit before servicing the INT_B and may set this bit again after all interrupt requests to INT_B pin are cleared to prevent inadvertent interrupts."]
    #[inline(always)]
    #[must_use]
    pub fn intbstatusenable(&mut self) -> IntbstatusenableW<IntenableSpec> {
        IntbstatusenableW::new(self, 10)
    }
    #[doc = "Bit 11 - If this bit is set to 0, the Host Controller shall clear the interrupt request to the System. The Host Driver may clear this bit before servicing the INT_C and may set this bit again after all interrupt requests to INT_C pin are cleared to prevent inadvertent interrupts. Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn intcstatusenable(&mut self) -> IntcstatusenableW<IntenableSpec> {
        IntcstatusenableW::new(self, 11)
    }
    #[doc = "Bit 12 - Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn retuningeventstatusenable(&mut self) -> RetuningeventstatusenableW<IntenableSpec> {
        RetuningeventstatusenableW::new(self, 12)
    }
    #[doc = "Bit 13 - Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn bootackrcvenable(&mut self) -> BootackrcvenableW<IntenableSpec> {
        BootackrcvenableW::new(self, 13)
    }
    #[doc = "Bit 14 - Boot is terminated?"]
    #[inline(always)]
    #[must_use]
    pub fn bootterminate(&mut self) -> BootterminateW<IntenableSpec> {
        BootterminateW::new(self, 14)
    }
    #[doc = "Bit 15 - The HC shall control error Interrupts using the Error Interrupt Status Enable register."]
    #[inline(always)]
    #[must_use]
    pub fn fixedto0(&mut self) -> Fixedto0W<IntenableSpec> {
        Fixedto0W::new(self, 15)
    }
    #[doc = "Bit 16 - Desc"]
    #[inline(always)]
    #[must_use]
    pub fn commandtimeouterrorstatusenable(
        &mut self,
    ) -> CommandtimeouterrorstatusenableW<IntenableSpec> {
        CommandtimeouterrorstatusenableW::new(self, 16)
    }
    #[doc = "Bit 17 - Desc"]
    #[inline(always)]
    #[must_use]
    pub fn commandcrcerrorstatusenable(&mut self) -> CommandcrcerrorstatusenableW<IntenableSpec> {
        CommandcrcerrorstatusenableW::new(self, 17)
    }
    #[doc = "Bit 18 - Desc"]
    #[inline(always)]
    #[must_use]
    pub fn commandendbiterrorstatusenable(
        &mut self,
    ) -> CommandendbiterrorstatusenableW<IntenableSpec> {
        CommandendbiterrorstatusenableW::new(self, 18)
    }
    #[doc = "Bit 19 - Desc"]
    #[inline(always)]
    #[must_use]
    pub fn commandindexerrorstatusenable(
        &mut self,
    ) -> CommandindexerrorstatusenableW<IntenableSpec> {
        CommandindexerrorstatusenableW::new(self, 19)
    }
    #[doc = "Bit 20 - Desc"]
    #[inline(always)]
    #[must_use]
    pub fn datatimeouterrorstatusenable(&mut self) -> DatatimeouterrorstatusenableW<IntenableSpec> {
        DatatimeouterrorstatusenableW::new(self, 20)
    }
    #[doc = "Bit 21 - Desc"]
    #[inline(always)]
    #[must_use]
    pub fn datacrcerrorstatusenable(&mut self) -> DatacrcerrorstatusenableW<IntenableSpec> {
        DatacrcerrorstatusenableW::new(self, 21)
    }
    #[doc = "Bit 22 - Desc"]
    #[inline(always)]
    #[must_use]
    pub fn dataendbiterrorstatusenable(&mut self) -> DataendbiterrorstatusenableW<IntenableSpec> {
        DataendbiterrorstatusenableW::new(self, 22)
    }
    #[doc = "Bit 23 - Desc"]
    #[inline(always)]
    #[must_use]
    pub fn currentlimiterrorstatusenable(
        &mut self,
    ) -> CurrentlimiterrorstatusenableW<IntenableSpec> {
        CurrentlimiterrorstatusenableW::new(self, 23)
    }
    #[doc = "Bit 24 - Desc"]
    #[inline(always)]
    #[must_use]
    pub fn autocmd12errorstatusenable(&mut self) -> Autocmd12errorstatusenableW<IntenableSpec> {
        Autocmd12errorstatusenableW::new(self, 24)
    }
    #[doc = "Bit 25 - Desc"]
    #[inline(always)]
    #[must_use]
    pub fn admaerrorstatusenable(&mut self) -> AdmaerrorstatusenableW<IntenableSpec> {
        AdmaerrorstatusenableW::new(self, 25)
    }
    #[doc = "Bit 26 - enable"]
    #[inline(always)]
    #[must_use]
    pub fn tuningerrorstatus(&mut self) -> TuningerrorstatusW<IntenableSpec> {
        TuningerrorstatusW::new(self, 26)
    }
    #[doc = "Bit 28 - Desc"]
    #[inline(always)]
    #[must_use]
    pub fn tgtresperrhosterrstaten(&mut self) -> TgtresperrhosterrstatenW<IntenableSpec> {
        TgtresperrhosterrstatenW::new(self, 28)
    }
    #[doc = "Bits 29:31 - Vendor-specific error status enable."]
    #[inline(always)]
    #[must_use]
    pub fn vendorspecificerrorstatusenable(
        &mut self,
    ) -> VendorspecificerrorstatusenableW<IntenableSpec> {
        VendorspecificerrorstatusenableW::new(self, 29)
    }
}
#[doc = "Normal interrupt status enable\n\nYou can [`read`](crate::Reg::read) this register and get [`intenable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenableSpec;
impl crate::RegisterSpec for IntenableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intenable::R`](R) reader structure"]
impl crate::Readable for IntenableSpec {}
#[doc = "`write(|w| ..)` method takes [`intenable::W`](W) writer structure"]
impl crate::Writable for IntenableSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTENABLE to value 0"]
impl crate::Resettable for IntenableSpec {
    const RESET_VALUE: u32 = 0;
}
