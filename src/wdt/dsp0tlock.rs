#[doc = "Register `DSP0TLOCK` reader"]
pub type R = crate::R<Dsp0tlockSpec>;
#[doc = "Register `DSP0TLOCK` writer"]
pub type W = crate::W<Dsp0tlockSpec>;
#[doc = "Writing 0xa7 locks the watchdog timer. Once locked, the WDTCFG reg cannot be written and WDTEN is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dsp0lock {
    #[doc = "167: This is the key value to write to WDTLOCK to lock the WDT."]
    Keyvalue = 167,
    #[doc = "0: Reset Value."]
    Default = 0,
}
impl From<Dsp0lock> for u8 {
    #[inline(always)]
    fn from(variant: Dsp0lock) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dsp0lock {
    type Ux = u8;
}
impl crate::IsEnum for Dsp0lock {}
#[doc = "Field `DSP0LOCK` reader - Writing 0xa7 locks the watchdog timer. Once locked, the WDTCFG reg cannot be written and WDTEN is set."]
pub type Dsp0lockR = crate::FieldReader<Dsp0lock>;
impl Dsp0lockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dsp0lock> {
        match self.bits {
            167 => Some(Dsp0lock::Keyvalue),
            0 => Some(Dsp0lock::Default),
            _ => None,
        }
    }
    #[doc = "This is the key value to write to WDTLOCK to lock the WDT."]
    #[inline(always)]
    pub fn is_keyvalue(&self) -> bool {
        *self == Dsp0lock::Keyvalue
    }
    #[doc = "Reset Value."]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == Dsp0lock::Default
    }
}
#[doc = "Field `DSP0LOCK` writer - Writing 0xa7 locks the watchdog timer. Once locked, the WDTCFG reg cannot be written and WDTEN is set."]
pub type Dsp0lockW<'a, REG> = crate::FieldWriter<'a, REG, 8, Dsp0lock>;
impl<'a, REG> Dsp0lockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "This is the key value to write to WDTLOCK to lock the WDT."]
    #[inline(always)]
    pub fn keyvalue(self) -> &'a mut crate::W<REG> {
        self.variant(Dsp0lock::Keyvalue)
    }
    #[doc = "Reset Value."]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(Dsp0lock::Default)
    }
}
impl R {
    #[doc = "Bits 0:7 - Writing 0xa7 locks the watchdog timer. Once locked, the WDTCFG reg cannot be written and WDTEN is set."]
    #[inline(always)]
    pub fn dsp0lock(&self) -> Dsp0lockR {
        Dsp0lockR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Writing 0xa7 locks the watchdog timer. Once locked, the WDTCFG reg cannot be written and WDTEN is set."]
    #[inline(always)]
    #[must_use]
    pub fn dsp0lock(&mut self) -> Dsp0lockW<Dsp0tlockSpec> {
        Dsp0lockW::new(self, 0)
    }
}
#[doc = "This register locks the watch dog timer. Once it is locked, the configuration register (DSP0CFG) for watch dog timer cannot be written to and the timer is automatically enabled (WDTEN is set).\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0tlock::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0tlock::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dsp0tlockSpec;
impl crate::RegisterSpec for Dsp0tlockSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsp0tlock::R`](R) reader structure"]
impl crate::Readable for Dsp0tlockSpec {}
#[doc = "`write(|w| ..)` method takes [`dsp0tlock::W`](W) writer structure"]
impl crate::Writable for Dsp0tlockSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSP0TLOCK to value 0"]
impl crate::Resettable for Dsp0tlockSpec {
    const RESET_VALUE: u32 = 0;
}
