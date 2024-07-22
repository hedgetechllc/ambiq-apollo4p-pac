#[doc = "Register `LFRCCTRL` reader"]
pub type R = crate::R<LfrcctrlSpec>;
#[doc = "Register `LFRCCTRL` writer"]
pub type W = crate::W<LfrcctrlSpec>;
#[doc = "Field `LFRCOUT` reader - Disable LFRC output"]
pub type LfrcoutR = crate::BitReader;
#[doc = "Field `LFRCOUT` writer - Disable LFRC output"]
pub type LfrcoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LFRCPWD` reader - Power down LFRC"]
pub type LfrcpwdR = crate::BitReader;
#[doc = "Field `LFRCPWD` writer - Power down LFRC"]
pub type LfrcpwdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Disable LFRC output"]
    #[inline(always)]
    pub fn lfrcout(&self) -> LfrcoutR {
        LfrcoutR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Power down LFRC"]
    #[inline(always)]
    pub fn lfrcpwd(&self) -> LfrcpwdR {
        LfrcpwdR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Disable LFRC output"]
    #[inline(always)]
    #[must_use]
    pub fn lfrcout(&mut self) -> LfrcoutW<LfrcctrlSpec> {
        LfrcoutW::new(self, 0)
    }
    #[doc = "Bit 1 - Power down LFRC"]
    #[inline(always)]
    #[must_use]
    pub fn lfrcpwd(&mut self) -> LfrcpwdW<LfrcctrlSpec> {
        LfrcpwdW::new(self, 1)
    }
}
#[doc = "LFRC control\n\nYou can [`read`](crate::Reg::read) this register and get [`lfrcctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfrcctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfrcctrlSpec;
impl crate::RegisterSpec for LfrcctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfrcctrl::R`](R) reader structure"]
impl crate::Readable for LfrcctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`lfrcctrl::W`](W) writer structure"]
impl crate::Writable for LfrcctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LFRCCTRL to value 0"]
impl crate::Resettable for LfrcctrlSpec {
    const RESET_VALUE: u32 = 0;
}
