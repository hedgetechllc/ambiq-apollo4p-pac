#[doc = "Register `CHACHADEBUGREG` reader"]
pub type R = crate::R<ChachadebugregSpec>;
#[doc = "Register `CHACHADEBUGREG` writer"]
pub type W = crate::W<ChachadebugregSpec>;
#[doc = "CHACHA_DEBUG_FSM_STATE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Chachadebugfsmstate {
    #[doc = "0: The idle state."]
    IdleState = 0,
    #[doc = "1: The init state."]
    InitState = 1,
}
impl From<Chachadebugfsmstate> for u8 {
    #[inline(always)]
    fn from(variant: Chachadebugfsmstate) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Chachadebugfsmstate {
    type Ux = u8;
}
impl crate::IsEnum for Chachadebugfsmstate {}
#[doc = "Field `CHACHADEBUGFSMSTATE` reader - CHACHA_DEBUG_FSM_STATE"]
pub type ChachadebugfsmstateR = crate::FieldReader<Chachadebugfsmstate>;
impl ChachadebugfsmstateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Chachadebugfsmstate> {
        match self.bits {
            0 => Some(Chachadebugfsmstate::IdleState),
            1 => Some(Chachadebugfsmstate::InitState),
            _ => None,
        }
    }
    #[doc = "The idle state."]
    #[inline(always)]
    pub fn is_idle_state(&self) -> bool {
        *self == Chachadebugfsmstate::IdleState
    }
    #[doc = "The init state."]
    #[inline(always)]
    pub fn is_init_state(&self) -> bool {
        *self == Chachadebugfsmstate::InitState
    }
}
#[doc = "Field `CHACHADEBUGFSMSTATE` writer - CHACHA_DEBUG_FSM_STATE"]
pub type ChachadebugfsmstateW<'a, REG> = crate::FieldWriter<'a, REG, 2, Chachadebugfsmstate>;
impl<'a, REG> ChachadebugfsmstateW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The idle state."]
    #[inline(always)]
    pub fn idle_state(self) -> &'a mut crate::W<REG> {
        self.variant(Chachadebugfsmstate::IdleState)
    }
    #[doc = "The init state."]
    #[inline(always)]
    pub fn init_state(self) -> &'a mut crate::W<REG> {
        self.variant(Chachadebugfsmstate::InitState)
    }
}
impl R {
    #[doc = "Bits 0:1 - CHACHA_DEBUG_FSM_STATE"]
    #[inline(always)]
    pub fn chachadebugfsmstate(&self) -> ChachadebugfsmstateR {
        ChachadebugfsmstateR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - CHACHA_DEBUG_FSM_STATE"]
    #[inline(always)]
    #[must_use]
    pub fn chachadebugfsmstate(&mut self) -> ChachadebugfsmstateW<ChachadebugregSpec> {
        ChachadebugfsmstateW::new(self, 0)
    }
}
#[doc = "This register is used to debug the CHACHA engine\n\nYou can [`read`](crate::Reg::read) this register and get [`chachadebugreg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chachadebugreg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChachadebugregSpec;
impl crate::RegisterSpec for ChachadebugregSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chachadebugreg::R`](R) reader structure"]
impl crate::Readable for ChachadebugregSpec {}
#[doc = "`write(|w| ..)` method takes [`chachadebugreg::W`](W) writer structure"]
impl crate::Writable for ChachadebugregSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHACHADEBUGREG to value 0"]
impl crate::Resettable for ChachadebugregSpec {
    const RESET_VALUE: u32 = 0;
}
