#[doc = "Register `CMDSTAT` reader"]
pub type R = crate::R<CmdstatSpec>;
#[doc = "Register `CMDSTAT` writer"]
pub type W = crate::W<CmdstatSpec>;
#[doc = "Field `CCMD` reader - current command that is being executed"]
pub type CcmdR = crate::FieldReader;
#[doc = "Field `CCMD` writer - current command that is being executed"]
pub type CcmdW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "The current status of the command execution.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmdstat {
    #[doc = "1: Error encountered with command"]
    Err = 1,
    #[doc = "2: Actively processing command"]
    Active = 2,
    #[doc = "4: Idle state, no active command, no error"]
    Idle = 4,
    #[doc = "6: Command in progress, but waiting on data from host"]
    Wait = 6,
}
impl From<Cmdstat> for u8 {
    #[inline(always)]
    fn from(variant: Cmdstat) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmdstat {
    type Ux = u8;
}
impl crate::IsEnum for Cmdstat {}
#[doc = "Field `CMDSTAT` reader - The current status of the command execution."]
pub type CmdstatR = crate::FieldReader<Cmdstat>;
impl CmdstatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cmdstat> {
        match self.bits {
            1 => Some(Cmdstat::Err),
            2 => Some(Cmdstat::Active),
            4 => Some(Cmdstat::Idle),
            6 => Some(Cmdstat::Wait),
            _ => None,
        }
    }
    #[doc = "Error encountered with command"]
    #[inline(always)]
    pub fn is_err(&self) -> bool {
        *self == Cmdstat::Err
    }
    #[doc = "Actively processing command"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Cmdstat::Active
    }
    #[doc = "Idle state, no active command, no error"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == Cmdstat::Idle
    }
    #[doc = "Command in progress, but waiting on data from host"]
    #[inline(always)]
    pub fn is_wait(&self) -> bool {
        *self == Cmdstat::Wait
    }
}
#[doc = "Field `CMDSTAT` writer - The current status of the command execution."]
pub type CmdstatW<'a, REG> = crate::FieldWriter<'a, REG, 3, Cmdstat>;
impl<'a, REG> CmdstatW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Error encountered with command"]
    #[inline(always)]
    pub fn err(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdstat::Err)
    }
    #[doc = "Actively processing command"]
    #[inline(always)]
    pub fn active(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdstat::Active)
    }
    #[doc = "Idle state, no active command, no error"]
    #[inline(always)]
    pub fn idle(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdstat::Idle)
    }
    #[doc = "Command in progress, but waiting on data from host"]
    #[inline(always)]
    pub fn wait(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdstat::Wait)
    }
}
#[doc = "Field `CTSIZE` reader - The current number of bytes still to be transferred with this command. This field will count down to zero."]
pub type CtsizeR = crate::FieldReader<u16>;
#[doc = "Field `CTSIZE` writer - The current number of bytes still to be transferred with this command. This field will count down to zero."]
pub type CtsizeW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:4 - current command that is being executed"]
    #[inline(always)]
    pub fn ccmd(&self) -> CcmdR {
        CcmdR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:7 - The current status of the command execution."]
    #[inline(always)]
    pub fn cmdstat(&self) -> CmdstatR {
        CmdstatR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:19 - The current number of bytes still to be transferred with this command. This field will count down to zero."]
    #[inline(always)]
    pub fn ctsize(&self) -> CtsizeR {
        CtsizeR::new(((self.bits >> 8) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:4 - current command that is being executed"]
    #[inline(always)]
    #[must_use]
    pub fn ccmd(&mut self) -> CcmdW<CmdstatSpec> {
        CcmdW::new(self, 0)
    }
    #[doc = "Bits 5:7 - The current status of the command execution."]
    #[inline(always)]
    #[must_use]
    pub fn cmdstat(&mut self) -> CmdstatW<CmdstatSpec> {
        CmdstatW::new(self, 5)
    }
    #[doc = "Bits 8:19 - The current number of bytes still to be transferred with this command. This field will count down to zero."]
    #[inline(always)]
    #[must_use]
    pub fn ctsize(&mut self) -> CtsizeW<CmdstatSpec> {
        CtsizeW::new(self, 8)
    }
}
#[doc = "Provides staus on the execution of the command currently in progress. The fields in this register will reflect the real time status of the internal state machines and data transfers within the IOM. These are read only fields and writes to the registers are ignored.\n\nYou can [`read`](crate::Reg::read) this register and get [`cmdstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmdstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmdstatSpec;
impl crate::RegisterSpec for CmdstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmdstat::R`](R) reader structure"]
impl crate::Readable for CmdstatSpec {}
#[doc = "`write(|w| ..)` method takes [`cmdstat::W`](W) writer structure"]
impl crate::Writable for CmdstatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMDSTAT to value 0"]
impl crate::Resettable for CmdstatSpec {
    const RESET_VALUE: u32 = 0;
}
