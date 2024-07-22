#[doc = "Register `DSP1RSTRT` reader"]
pub type R = crate::R<Dsp1rstrtSpec>;
#[doc = "Register `DSP1RSTRT` writer"]
pub type W = crate::W<Dsp1rstrtSpec>;
#[doc = "Writing 0xd2 to DSP1RSTRT restarts the watchdog timer. This is a write only register. Reading this register will return 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dsp1rstart {
    #[doc = "210: This is the key value to write to WDTRSTRT to restart the WDT. This is a write only register."]
    Keyvalue = 210,
    #[doc = "0: Reset value"]
    Default = 0,
}
impl From<Dsp1rstart> for u8 {
    #[inline(always)]
    fn from(variant: Dsp1rstart) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dsp1rstart {
    type Ux = u8;
}
impl crate::IsEnum for Dsp1rstart {}
#[doc = "Field `DSP1RSTART` reader - Writing 0xd2 to DSP1RSTRT restarts the watchdog timer. This is a write only register. Reading this register will return 0."]
pub type Dsp1rstartR = crate::FieldReader<Dsp1rstart>;
impl Dsp1rstartR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dsp1rstart> {
        match self.bits {
            210 => Some(Dsp1rstart::Keyvalue),
            0 => Some(Dsp1rstart::Default),
            _ => None,
        }
    }
    #[doc = "This is the key value to write to WDTRSTRT to restart the WDT. This is a write only register."]
    #[inline(always)]
    pub fn is_keyvalue(&self) -> bool {
        *self == Dsp1rstart::Keyvalue
    }
    #[doc = "Reset value"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == Dsp1rstart::Default
    }
}
#[doc = "Field `DSP1RSTART` writer - Writing 0xd2 to DSP1RSTRT restarts the watchdog timer. This is a write only register. Reading this register will return 0."]
pub type Dsp1rstartW<'a, REG> = crate::FieldWriter<'a, REG, 8, Dsp1rstart>;
impl<'a, REG> Dsp1rstartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "This is the key value to write to WDTRSTRT to restart the WDT. This is a write only register."]
    #[inline(always)]
    pub fn keyvalue(self) -> &'a mut crate::W<REG> {
        self.variant(Dsp1rstart::Keyvalue)
    }
    #[doc = "Reset value"]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(Dsp1rstart::Default)
    }
}
impl R {
    #[doc = "Bits 0:7 - Writing 0xd2 to DSP1RSTRT restarts the watchdog timer. This is a write only register. Reading this register will return 0."]
    #[inline(always)]
    pub fn dsp1rstart(&self) -> Dsp1rstartR {
        Dsp1rstartR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Writing 0xd2 to DSP1RSTRT restarts the watchdog timer. This is a write only register. Reading this register will return 0."]
    #[inline(always)]
    #[must_use]
    pub fn dsp1rstart(&mut self) -> Dsp1rstartW<Dsp1rstrtSpec> {
        Dsp1rstartW::new(self, 0)
    }
}
#[doc = "This register will restart the watchdog timer. Writing a special key value into this register will result in the watch dog timer being reset, so that the count will start again. It is expected that the software will periodically write to this register to indicate that the system is functional. The watch dog timer can continue running when the system is in deep sleep, and the interrupt will trigger the wake. After the wake, the core can reset the watch dog timer.\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1rstrt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1rstrt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dsp1rstrtSpec;
impl crate::RegisterSpec for Dsp1rstrtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsp1rstrt::R`](R) reader structure"]
impl crate::Readable for Dsp1rstrtSpec {}
#[doc = "`write(|w| ..)` method takes [`dsp1rstrt::W`](W) writer structure"]
impl crate::Writable for Dsp1rstrtSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSP1RSTRT to value 0"]
impl crate::Resettable for Dsp1rstrtSpec {
    const RESET_VALUE: u32 = 0;
}
