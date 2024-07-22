#[doc = "Register `DEBUGGER` reader"]
pub type R = crate::R<DebuggerSpec>;
#[doc = "Register `DEBUGGER` writer"]
pub type W = crate::W<DebuggerSpec>;
#[doc = "Field `LOCKOUT` reader - Lockout of debugger (SWD)."]
pub type LockoutR = crate::FieldReader<u32>;
#[doc = "Field `LOCKOUT` writer - Lockout of debugger (SWD)."]
pub type LockoutW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Lockout of debugger (SWD)."]
    #[inline(always)]
    pub fn lockout(&self) -> LockoutR {
        LockoutR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Lockout of debugger (SWD)."]
    #[inline(always)]
    #[must_use]
    pub fn lockout(&mut self) -> LockoutW<DebuggerSpec> {
        LockoutW::new(self, 0)
    }
}
#[doc = "Debugger Control\n\nYou can [`read`](crate::Reg::read) this register and get [`debugger::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debugger::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DebuggerSpec;
impl crate::RegisterSpec for DebuggerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`debugger::R`](R) reader structure"]
impl crate::Readable for DebuggerSpec {}
#[doc = "`write(|w| ..)` method takes [`debugger::W`](W) writer structure"]
impl crate::Writable for DebuggerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEBUGGER to value 0"]
impl crate::Resettable for DebuggerSpec {
    const RESET_VALUE: u32 = 0;
}
