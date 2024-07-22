#[doc = "Register `TRNGDEBUGCONTROL` reader"]
pub type R = crate::R<TrngdebugcontrolSpec>;
#[doc = "Register `TRNGDEBUGCONTROL` writer"]
pub type W = crate::W<TrngdebugcontrolSpec>;
#[doc = "Field `VNCBYPASS` reader - When this bit is set, the Von-Neumann balancer is bypassed (including the 32 consecutive bits test). Note: Can only be set while in debug mode. If TRNG_TESTS_BYPASS_EN HW flag is defined, this bit can be set while not in debug mode."]
pub type VncbypassR = crate::BitReader;
#[doc = "Field `VNCBYPASS` writer - When this bit is set, the Von-Neumann balancer is bypassed (including the 32 consecutive bits test). Note: Can only be set while in debug mode. If TRNG_TESTS_BYPASS_EN HW flag is defined, this bit can be set while not in debug mode."]
pub type VncbypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRNGCRNGTBYPASS` reader - When this bit is set, the CRNGT test in the TRNG is bypassed. Note: Can only be set while in debug mode. If TRNG_TESTS_BYPASS_EN HW flag is defined, this bit can be set while not in debug mode."]
pub type TrngcrngtbypassR = crate::BitReader;
#[doc = "Field `TRNGCRNGTBYPASS` writer - When this bit is set, the CRNGT test in the TRNG is bypassed. Note: Can only be set while in debug mode. If TRNG_TESTS_BYPASS_EN HW flag is defined, this bit can be set while not in debug mode."]
pub type TrngcrngtbypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTOCORRELATEBYPASS` reader - When this bit is set, the autocorrelation test in the TRNG module is bypassed. Note: Can only be set while in debug mode. If TRNG_TESTS_BYPASS_EN HW flag is defined, this bit can be set while not in debug mode."]
pub type AutocorrelatebypassR = crate::BitReader;
#[doc = "Field `AUTOCORRELATEBYPASS` writer - When this bit is set, the autocorrelation test in the TRNG module is bypassed. Note: Can only be set while in debug mode. If TRNG_TESTS_BYPASS_EN HW flag is defined, this bit can be set while not in debug mode."]
pub type AutocorrelatebypassW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - When this bit is set, the Von-Neumann balancer is bypassed (including the 32 consecutive bits test). Note: Can only be set while in debug mode. If TRNG_TESTS_BYPASS_EN HW flag is defined, this bit can be set while not in debug mode."]
    #[inline(always)]
    pub fn vncbypass(&self) -> VncbypassR {
        VncbypassR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - When this bit is set, the CRNGT test in the TRNG is bypassed. Note: Can only be set while in debug mode. If TRNG_TESTS_BYPASS_EN HW flag is defined, this bit can be set while not in debug mode."]
    #[inline(always)]
    pub fn trngcrngtbypass(&self) -> TrngcrngtbypassR {
        TrngcrngtbypassR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - When this bit is set, the autocorrelation test in the TRNG module is bypassed. Note: Can only be set while in debug mode. If TRNG_TESTS_BYPASS_EN HW flag is defined, this bit can be set while not in debug mode."]
    #[inline(always)]
    pub fn autocorrelatebypass(&self) -> AutocorrelatebypassR {
        AutocorrelatebypassR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - When this bit is set, the Von-Neumann balancer is bypassed (including the 32 consecutive bits test). Note: Can only be set while in debug mode. If TRNG_TESTS_BYPASS_EN HW flag is defined, this bit can be set while not in debug mode."]
    #[inline(always)]
    #[must_use]
    pub fn vncbypass(&mut self) -> VncbypassW<TrngdebugcontrolSpec> {
        VncbypassW::new(self, 1)
    }
    #[doc = "Bit 2 - When this bit is set, the CRNGT test in the TRNG is bypassed. Note: Can only be set while in debug mode. If TRNG_TESTS_BYPASS_EN HW flag is defined, this bit can be set while not in debug mode."]
    #[inline(always)]
    #[must_use]
    pub fn trngcrngtbypass(&mut self) -> TrngcrngtbypassW<TrngdebugcontrolSpec> {
        TrngcrngtbypassW::new(self, 2)
    }
    #[doc = "Bit 3 - When this bit is set, the autocorrelation test in the TRNG module is bypassed. Note: Can only be set while in debug mode. If TRNG_TESTS_BYPASS_EN HW flag is defined, this bit can be set while not in debug mode."]
    #[inline(always)]
    #[must_use]
    pub fn autocorrelatebypass(&mut self) -> AutocorrelatebypassW<TrngdebugcontrolSpec> {
        AutocorrelatebypassW::new(self, 3)
    }
}
#[doc = "This register is used to debug the TRNG\n\nYou can [`read`](crate::Reg::read) this register and get [`trngdebugcontrol::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trngdebugcontrol::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrngdebugcontrolSpec;
impl crate::RegisterSpec for TrngdebugcontrolSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trngdebugcontrol::R`](R) reader structure"]
impl crate::Readable for TrngdebugcontrolSpec {}
#[doc = "`write(|w| ..)` method takes [`trngdebugcontrol::W`](W) writer structure"]
impl crate::Writable for TrngdebugcontrolSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRNGDEBUGCONTROL to value 0"]
impl crate::Resettable for TrngdebugcontrolSpec {
    const RESET_VALUE: u32 = 0;
}
