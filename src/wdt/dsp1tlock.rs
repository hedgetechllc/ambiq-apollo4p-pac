#[doc = "Register `DSP1TLOCK` reader"]
pub type R = crate::R<Dsp1tlockSpec>;
#[doc = "Register `DSP1TLOCK` writer"]
pub type W = crate::W<Dsp1tlockSpec>;
#[doc = "Writing 0x4e locks the watchdog timer. Once locked, the WDTCFG reg cannot be written and WDTEN is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dsp1lock {
    #[doc = "78: This is the key value to write to WDTLOCK to lock the WDT."]
    Keyvalue = 78,
    #[doc = "0: Reset Value."]
    Default = 0,
}
impl From<Dsp1lock> for u8 {
    #[inline(always)]
    fn from(variant: Dsp1lock) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dsp1lock {
    type Ux = u8;
}
impl crate::IsEnum for Dsp1lock {}
#[doc = "Field `DSP1LOCK` reader - Writing 0x4e locks the watchdog timer. Once locked, the WDTCFG reg cannot be written and WDTEN is set."]
pub type Dsp1lockR = crate::FieldReader<Dsp1lock>;
impl Dsp1lockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dsp1lock> {
        match self.bits {
            78 => Some(Dsp1lock::Keyvalue),
            0 => Some(Dsp1lock::Default),
            _ => None,
        }
    }
    #[doc = "This is the key value to write to WDTLOCK to lock the WDT."]
    #[inline(always)]
    pub fn is_keyvalue(&self) -> bool {
        *self == Dsp1lock::Keyvalue
    }
    #[doc = "Reset Value."]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == Dsp1lock::Default
    }
}
#[doc = "Field `DSP1LOCK` writer - Writing 0x4e locks the watchdog timer. Once locked, the WDTCFG reg cannot be written and WDTEN is set."]
pub type Dsp1lockW<'a, REG> = crate::FieldWriter<'a, REG, 8, Dsp1lock>;
impl<'a, REG> Dsp1lockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "This is the key value to write to WDTLOCK to lock the WDT."]
    #[inline(always)]
    pub fn keyvalue(self) -> &'a mut crate::W<REG> {
        self.variant(Dsp1lock::Keyvalue)
    }
    #[doc = "Reset Value."]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(Dsp1lock::Default)
    }
}
impl R {
    #[doc = "Bits 0:7 - Writing 0x4e locks the watchdog timer. Once locked, the WDTCFG reg cannot be written and WDTEN is set."]
    #[inline(always)]
    pub fn dsp1lock(&self) -> Dsp1lockR {
        Dsp1lockR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Writing 0x4e locks the watchdog timer. Once locked, the WDTCFG reg cannot be written and WDTEN is set."]
    #[inline(always)]
    #[must_use]
    pub fn dsp1lock(&mut self) -> Dsp1lockW<Dsp1tlockSpec> {
        Dsp1lockW::new(self, 0)
    }
}
#[doc = "This register locks the watch dog timer. Once it is locked, the configuration register (DSP1CFG) for watch dog timer cannot be written to and the timer is automatically enabled (WDTEN is set).\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1tlock::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1tlock::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dsp1tlockSpec;
impl crate::RegisterSpec for Dsp1tlockSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsp1tlock::R`](R) reader structure"]
impl crate::Readable for Dsp1tlockSpec {}
#[doc = "`write(|w| ..)` method takes [`dsp1tlock::W`](W) writer structure"]
impl crate::Writable for Dsp1tlockSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSP1TLOCK to value 0"]
impl crate::Resettable for Dsp1tlockSpec {
    const RESET_VALUE: u32 = 0;
}
