#[doc = "Register `LOCK` reader"]
pub type R = crate::R<LockSpec>;
#[doc = "Register `LOCK` writer"]
pub type W = crate::W<LockSpec>;
#[doc = "Writing 0x3A locks the watchdog timer. Once locked, the WDTCFG reg cannot be written and WDTEN is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lock {
    #[doc = "58: This is the key value to write to WDTLOCK to lock the WDT."]
    Keyvalue = 58,
    #[doc = "0: Default/Reset value."]
    Default = 0,
}
impl From<Lock> for u8 {
    #[inline(always)]
    fn from(variant: Lock) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lock {
    type Ux = u8;
}
impl crate::IsEnum for Lock {}
#[doc = "Field `LOCK` reader - Writing 0x3A locks the watchdog timer. Once locked, the WDTCFG reg cannot be written and WDTEN is set."]
pub type LockR = crate::FieldReader<Lock>;
impl LockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Lock> {
        match self.bits {
            58 => Some(Lock::Keyvalue),
            0 => Some(Lock::Default),
            _ => None,
        }
    }
    #[doc = "This is the key value to write to WDTLOCK to lock the WDT."]
    #[inline(always)]
    pub fn is_keyvalue(&self) -> bool {
        *self == Lock::Keyvalue
    }
    #[doc = "Default/Reset value."]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == Lock::Default
    }
}
#[doc = "Field `LOCK` writer - Writing 0x3A locks the watchdog timer. Once locked, the WDTCFG reg cannot be written and WDTEN is set."]
pub type LockW<'a, REG> = crate::FieldWriter<'a, REG, 8, Lock>;
impl<'a, REG> LockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "This is the key value to write to WDTLOCK to lock the WDT."]
    #[inline(always)]
    pub fn keyvalue(self) -> &'a mut crate::W<REG> {
        self.variant(Lock::Keyvalue)
    }
    #[doc = "Default/Reset value."]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(Lock::Default)
    }
}
impl R {
    #[doc = "Bits 0:7 - Writing 0x3A locks the watchdog timer. Once locked, the WDTCFG reg cannot be written and WDTEN is set."]
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Writing 0x3A locks the watchdog timer. Once locked, the WDTCFG reg cannot be written and WDTEN is set."]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LockW<LockSpec> {
        LockW::new(self, 0)
    }
}
#[doc = "This register locks the watch dog timer. Once it is locked, the configuration register (WDTCFG) for watch dog timer cannot be written to.\n\nYou can [`read`](crate::Reg::read) this register and get [`lock::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lock::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LockSpec;
impl crate::RegisterSpec for LockSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lock::R`](R) reader structure"]
impl crate::Readable for LockSpec {}
#[doc = "`write(|w| ..)` method takes [`lock::W`](W) writer structure"]
impl crate::Writable for LockSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LOCK to value 0"]
impl crate::Resettable for LockSpec {
    const RESET_VALUE: u32 = 0;
}
