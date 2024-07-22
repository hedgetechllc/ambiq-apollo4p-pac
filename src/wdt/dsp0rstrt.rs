#[doc = "Register `DSP0RSTRT` reader"]
pub type R = crate::R<Dsp0rstrtSpec>;
#[doc = "Register `DSP0RSTRT` writer"]
pub type W = crate::W<Dsp0rstrtSpec>;
#[doc = "Writing 0x69 to DSP0RSTRT restarts the watchdog timer. This is a write only register. Reading this register will return 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dsp0rstart {
    #[doc = "105: This is the key value to write to WDTRSTRT to restart the WDT. This is a write only register."]
    Keyvalue = 105,
    #[doc = "0: Reset value"]
    Default = 0,
}
impl From<Dsp0rstart> for u8 {
    #[inline(always)]
    fn from(variant: Dsp0rstart) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dsp0rstart {
    type Ux = u8;
}
impl crate::IsEnum for Dsp0rstart {}
#[doc = "Field `DSP0RSTART` reader - Writing 0x69 to DSP0RSTRT restarts the watchdog timer. This is a write only register. Reading this register will return 0."]
pub type Dsp0rstartR = crate::FieldReader<Dsp0rstart>;
impl Dsp0rstartR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dsp0rstart> {
        match self.bits {
            105 => Some(Dsp0rstart::Keyvalue),
            0 => Some(Dsp0rstart::Default),
            _ => None,
        }
    }
    #[doc = "This is the key value to write to WDTRSTRT to restart the WDT. This is a write only register."]
    #[inline(always)]
    pub fn is_keyvalue(&self) -> bool {
        *self == Dsp0rstart::Keyvalue
    }
    #[doc = "Reset value"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == Dsp0rstart::Default
    }
}
#[doc = "Field `DSP0RSTART` writer - Writing 0x69 to DSP0RSTRT restarts the watchdog timer. This is a write only register. Reading this register will return 0."]
pub type Dsp0rstartW<'a, REG> = crate::FieldWriter<'a, REG, 8, Dsp0rstart>;
impl<'a, REG> Dsp0rstartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "This is the key value to write to WDTRSTRT to restart the WDT. This is a write only register."]
    #[inline(always)]
    pub fn keyvalue(self) -> &'a mut crate::W<REG> {
        self.variant(Dsp0rstart::Keyvalue)
    }
    #[doc = "Reset value"]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(Dsp0rstart::Default)
    }
}
impl R {
    #[doc = "Bits 0:7 - Writing 0x69 to DSP0RSTRT restarts the watchdog timer. This is a write only register. Reading this register will return 0."]
    #[inline(always)]
    pub fn dsp0rstart(&self) -> Dsp0rstartR {
        Dsp0rstartR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Writing 0x69 to DSP0RSTRT restarts the watchdog timer. This is a write only register. Reading this register will return 0."]
    #[inline(always)]
    #[must_use]
    pub fn dsp0rstart(&mut self) -> Dsp0rstartW<Dsp0rstrtSpec> {
        Dsp0rstartW::new(self, 0)
    }
}
#[doc = "This register will restart the watchdog timer. Writing a special key value into this register will result in the watch dog timer being reset, so that the count will start again. It is expected that the software will periodically write to this register to indicate that the system is functional. The watch dog timer can continue running when the system is in deep sleep, and the interrupt will trigger the wake. After the wake, the core can reset the watch dog timer.\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0rstrt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0rstrt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dsp0rstrtSpec;
impl crate::RegisterSpec for Dsp0rstrtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsp0rstrt::R`](R) reader structure"]
impl crate::Readable for Dsp0rstrtSpec {}
#[doc = "`write(|w| ..)` method takes [`dsp0rstrt::W`](W) writer structure"]
impl crate::Writable for Dsp0rstrtSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSP0RSTRT to value 0"]
impl crate::Resettable for Dsp0rstrtSpec {
    const RESET_VALUE: u32 = 0;
}
