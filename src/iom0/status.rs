#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Register `STATUS` writer"]
pub type W = crate::W<StatusSpec>;
#[doc = "Bit has been deprecated. Please refer to the other error indicators. This will always return 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Err {
    #[doc = "1: Bit has been deprecated and will always return 0."]
    Error = 1,
    #[doc = "0: Default value."]
    Default = 0,
}
impl From<Err> for bool {
    #[inline(always)]
    fn from(variant: Err) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERR` reader - Bit has been deprecated. Please refer to the other error indicators. This will always return 0."]
pub type ErrR = crate::BitReader<Err>;
impl ErrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Err {
        match self.bits {
            true => Err::Error,
            false => Err::Default,
        }
    }
    #[doc = "Bit has been deprecated and will always return 0."]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == Err::Error
    }
    #[doc = "Default value."]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == Err::Default
    }
}
#[doc = "Field `ERR` writer - Bit has been deprecated. Please refer to the other error indicators. This will always return 0."]
pub type ErrW<'a, REG> = crate::BitWriter<'a, REG, Err>;
impl<'a, REG> ErrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bit has been deprecated and will always return 0."]
    #[inline(always)]
    pub fn error(self) -> &'a mut crate::W<REG> {
        self.variant(Err::Error)
    }
    #[doc = "Default value."]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(Err::Default)
    }
}
#[doc = "Indicates if the active I/O Command is currently processing a transaction, or command is complete, but the FIFO pointers are still syncronizing internally. This bit will go high at the start of the transaction, and will go low when the command is complete, and the data and pointers within the FIFO have been syncronized.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmdact {
    #[doc = "1: An I/O command is active. Indicates the active module has an active command and is processing this. De-asserted when the command is completed."]
    Active = 1,
    #[doc = "0: An I/O command is inactive. Indicating that the command is complete."]
    Inactive = 0,
}
impl From<Cmdact> for bool {
    #[inline(always)]
    fn from(variant: Cmdact) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMDACT` reader - Indicates if the active I/O Command is currently processing a transaction, or command is complete, but the FIFO pointers are still syncronizing internally. This bit will go high at the start of the transaction, and will go low when the command is complete, and the data and pointers within the FIFO have been syncronized."]
pub type CmdactR = crate::BitReader<Cmdact>;
impl CmdactR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmdact {
        match self.bits {
            true => Cmdact::Active,
            false => Cmdact::Inactive,
        }
    }
    #[doc = "An I/O command is active. Indicates the active module has an active command and is processing this. De-asserted when the command is completed."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Cmdact::Active
    }
    #[doc = "An I/O command is inactive. Indicating that the command is complete."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == Cmdact::Inactive
    }
}
#[doc = "Field `CMDACT` writer - Indicates if the active I/O Command is currently processing a transaction, or command is complete, but the FIFO pointers are still syncronizing internally. This bit will go high at the start of the transaction, and will go low when the command is complete, and the data and pointers within the FIFO have been syncronized."]
pub type CmdactW<'a, REG> = crate::BitWriter<'a, REG, Cmdact>;
impl<'a, REG> CmdactW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "An I/O command is active. Indicates the active module has an active command and is processing this. De-asserted when the command is completed."]
    #[inline(always)]
    pub fn active(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdact::Active)
    }
    #[doc = "An I/O command is inactive. Indicating that the command is complete."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdact::Inactive)
    }
}
#[doc = "indicates if the active I/O state machine is IDLE. Note - The state machine could be in idle state due to holdoffs from data availability, or as the command gets propagated into the logic from the registers.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Idlest {
    #[doc = "1: The I/O state machine is in the idle state."]
    Idle = 1,
    #[doc = "0: The I/O state machine is in the non-idle/run state."]
    Run = 0,
}
impl From<Idlest> for bool {
    #[inline(always)]
    fn from(variant: Idlest) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLEST` reader - indicates if the active I/O state machine is IDLE. Note - The state machine could be in idle state due to holdoffs from data availability, or as the command gets propagated into the logic from the registers."]
pub type IdlestR = crate::BitReader<Idlest>;
impl IdlestR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Idlest {
        match self.bits {
            true => Idlest::Idle,
            false => Idlest::Run,
        }
    }
    #[doc = "The I/O state machine is in the idle state."]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == Idlest::Idle
    }
    #[doc = "The I/O state machine is in the non-idle/run state."]
    #[inline(always)]
    pub fn is_run(&self) -> bool {
        *self == Idlest::Run
    }
}
#[doc = "Field `IDLEST` writer - indicates if the active I/O state machine is IDLE. Note - The state machine could be in idle state due to holdoffs from data availability, or as the command gets propagated into the logic from the registers."]
pub type IdlestW<'a, REG> = crate::BitWriter<'a, REG, Idlest>;
impl<'a, REG> IdlestW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The I/O state machine is in the idle state."]
    #[inline(always)]
    pub fn idle(self) -> &'a mut crate::W<REG> {
        self.variant(Idlest::Idle)
    }
    #[doc = "The I/O state machine is in the non-idle/run state."]
    #[inline(always)]
    pub fn run(self) -> &'a mut crate::W<REG> {
        self.variant(Idlest::Run)
    }
}
impl R {
    #[doc = "Bit 0 - Bit has been deprecated. Please refer to the other error indicators. This will always return 0."]
    #[inline(always)]
    pub fn err(&self) -> ErrR {
        ErrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Indicates if the active I/O Command is currently processing a transaction, or command is complete, but the FIFO pointers are still syncronizing internally. This bit will go high at the start of the transaction, and will go low when the command is complete, and the data and pointers within the FIFO have been syncronized."]
    #[inline(always)]
    pub fn cmdact(&self) -> CmdactR {
        CmdactR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - indicates if the active I/O state machine is IDLE. Note - The state machine could be in idle state due to holdoffs from data availability, or as the command gets propagated into the logic from the registers."]
    #[inline(always)]
    pub fn idlest(&self) -> IdlestR {
        IdlestR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bit has been deprecated. Please refer to the other error indicators. This will always return 0."]
    #[inline(always)]
    #[must_use]
    pub fn err(&mut self) -> ErrW<StatusSpec> {
        ErrW::new(self, 0)
    }
    #[doc = "Bit 1 - Indicates if the active I/O Command is currently processing a transaction, or command is complete, but the FIFO pointers are still syncronizing internally. This bit will go high at the start of the transaction, and will go low when the command is complete, and the data and pointers within the FIFO have been syncronized."]
    #[inline(always)]
    #[must_use]
    pub fn cmdact(&mut self) -> CmdactW<StatusSpec> {
        CmdactW::new(self, 1)
    }
    #[doc = "Bit 2 - indicates if the active I/O state machine is IDLE. Note - The state machine could be in idle state due to holdoffs from data availability, or as the command gets propagated into the logic from the registers."]
    #[inline(always)]
    #[must_use]
    pub fn idlest(&mut self) -> IdlestW<StatusSpec> {
        IdlestW::new(self, 2)
    }
}
#[doc = "IOM Module Status\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for StatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {
    const RESET_VALUE: u32 = 0;
}
