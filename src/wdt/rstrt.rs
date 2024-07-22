#[doc = "Register `RSTRT` reader"]
pub type R = crate::R<RstrtSpec>;
#[doc = "Register `RSTRT` writer"]
pub type W = crate::W<RstrtSpec>;
#[doc = "Writing 0xB2 to WDTRSTRT restarts the watchdog timer. This is a write only register. Reading this register will only provide all 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rstrt {
    #[doc = "178: This is the key value to write to WDTRSTRT to restart the WDT. This is a write only register."]
    Keyvalue = 178,
    #[doc = "0: Default/Reset value. This is a write only register."]
    Default = 0,
}
impl From<Rstrt> for u8 {
    #[inline(always)]
    fn from(variant: Rstrt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rstrt {
    type Ux = u8;
}
impl crate::IsEnum for Rstrt {}
#[doc = "Field `RSTRT` reader - Writing 0xB2 to WDTRSTRT restarts the watchdog timer. This is a write only register. Reading this register will only provide all 0."]
pub type RstrtR = crate::FieldReader<Rstrt>;
impl RstrtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Rstrt> {
        match self.bits {
            178 => Some(Rstrt::Keyvalue),
            0 => Some(Rstrt::Default),
            _ => None,
        }
    }
    #[doc = "This is the key value to write to WDTRSTRT to restart the WDT. This is a write only register."]
    #[inline(always)]
    pub fn is_keyvalue(&self) -> bool {
        *self == Rstrt::Keyvalue
    }
    #[doc = "Default/Reset value. This is a write only register."]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == Rstrt::Default
    }
}
#[doc = "Field `RSTRT` writer - Writing 0xB2 to WDTRSTRT restarts the watchdog timer. This is a write only register. Reading this register will only provide all 0."]
pub type RstrtW<'a, REG> = crate::FieldWriter<'a, REG, 8, Rstrt>;
impl<'a, REG> RstrtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "This is the key value to write to WDTRSTRT to restart the WDT. This is a write only register."]
    #[inline(always)]
    pub fn keyvalue(self) -> &'a mut crate::W<REG> {
        self.variant(Rstrt::Keyvalue)
    }
    #[doc = "Default/Reset value. This is a write only register."]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(Rstrt::Default)
    }
}
impl R {
    #[doc = "Bits 0:7 - Writing 0xB2 to WDTRSTRT restarts the watchdog timer. This is a write only register. Reading this register will only provide all 0."]
    #[inline(always)]
    pub fn rstrt(&self) -> RstrtR {
        RstrtR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Writing 0xB2 to WDTRSTRT restarts the watchdog timer. This is a write only register. Reading this register will only provide all 0."]
    #[inline(always)]
    #[must_use]
    pub fn rstrt(&mut self) -> RstrtW<RstrtSpec> {
        RstrtW::new(self, 0)
    }
}
#[doc = "This register will Restart the watchdog timer. Writing a special key value into this register will result in the watch dog timer being reset, so that the count will start again. It is expected that the software will periodically write to this register to indicate that the system is functional. The watch dog timer can continue running when the system is in deep sleep, and the interrupt will trigger the wake. After the wake, the core can reset the watch dog timer.\n\nYou can [`read`](crate::Reg::read) this register and get [`rstrt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstrt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RstrtSpec;
impl crate::RegisterSpec for RstrtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rstrt::R`](R) reader structure"]
impl crate::Readable for RstrtSpec {}
#[doc = "`write(|w| ..)` method takes [`rstrt::W`](W) writer structure"]
impl crate::Writable for RstrtSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RSTRT to value 0"]
impl crate::Resettable for RstrtSpec {
    const RESET_VALUE: u32 = 0;
}
