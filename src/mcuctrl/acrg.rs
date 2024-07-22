#[doc = "Register `ACRG` reader"]
pub type R = crate::R<AcrgSpec>;
#[doc = "Register `ACRG` writer"]
pub type W = crate::W<AcrgSpec>;
#[doc = "Field `ACRGSWE` reader - Software enablement for ACRG register. A value of 1 will allow writes to the register"]
pub type AcrgsweR = crate::BitReader;
#[doc = "Field `ACRGSWE` writer - Software enablement for ACRG register. A value of 1 will allow writes to the register"]
pub type AcrgsweW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Power down the ACRG.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Acrgpwd {
    #[doc = "1: Powers down the ACRG trim."]
    AcrgPwrDn = 1,
    #[doc = "0: Power up the ACRG trim."]
    AcrgPwrUp = 0,
}
impl From<Acrgpwd> for bool {
    #[inline(always)]
    fn from(variant: Acrgpwd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACRGPWD` reader - Power down the ACRG."]
pub type AcrgpwdR = crate::BitReader<Acrgpwd>;
impl AcrgpwdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Acrgpwd {
        match self.bits {
            true => Acrgpwd::AcrgPwrDn,
            false => Acrgpwd::AcrgPwrUp,
        }
    }
    #[doc = "Powers down the ACRG trim."]
    #[inline(always)]
    pub fn is_acrg_pwr_dn(&self) -> bool {
        *self == Acrgpwd::AcrgPwrDn
    }
    #[doc = "Power up the ACRG trim."]
    #[inline(always)]
    pub fn is_acrg_pwr_up(&self) -> bool {
        *self == Acrgpwd::AcrgPwrUp
    }
}
#[doc = "Field `ACRGPWD` writer - Power down the ACRG."]
pub type AcrgpwdW<'a, REG> = crate::BitWriter<'a, REG, Acrgpwd>;
impl<'a, REG> AcrgpwdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Powers down the ACRG trim."]
    #[inline(always)]
    pub fn acrg_pwr_dn(self) -> &'a mut crate::W<REG> {
        self.variant(Acrgpwd::AcrgPwrDn)
    }
    #[doc = "Power up the ACRG trim."]
    #[inline(always)]
    pub fn acrg_pwr_up(self) -> &'a mut crate::W<REG> {
        self.variant(Acrgpwd::AcrgPwrUp)
    }
}
#[doc = "Set the ACRG ibias. Note: the SWE mux select in PWRSEQ2SWE must be set for this to take effect. The inversion of this register is driven to analog.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Acrgibiassel {
    #[doc = "0: Selects the bandgap"]
    Bgsel = 0,
    #[doc = "1: Selects the CCRG"]
    Ccrgsel = 1,
}
impl From<Acrgibiassel> for bool {
    #[inline(always)]
    fn from(variant: Acrgibiassel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACRGIBIASSEL` reader - Set the ACRG ibias. Note: the SWE mux select in PWRSEQ2SWE must be set for this to take effect. The inversion of this register is driven to analog."]
pub type AcrgibiasselR = crate::BitReader<Acrgibiassel>;
impl AcrgibiasselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Acrgibiassel {
        match self.bits {
            false => Acrgibiassel::Bgsel,
            true => Acrgibiassel::Ccrgsel,
        }
    }
    #[doc = "Selects the bandgap"]
    #[inline(always)]
    pub fn is_bgsel(&self) -> bool {
        *self == Acrgibiassel::Bgsel
    }
    #[doc = "Selects the CCRG"]
    #[inline(always)]
    pub fn is_ccrgsel(&self) -> bool {
        *self == Acrgibiassel::Ccrgsel
    }
}
#[doc = "Field `ACRGIBIASSEL` writer - Set the ACRG ibias. Note: the SWE mux select in PWRSEQ2SWE must be set for this to take effect. The inversion of this register is driven to analog."]
pub type AcrgibiasselW<'a, REG> = crate::BitWriter<'a, REG, Acrgibiassel>;
impl<'a, REG> AcrgibiasselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Selects the bandgap"]
    #[inline(always)]
    pub fn bgsel(self) -> &'a mut crate::W<REG> {
        self.variant(Acrgibiassel::Bgsel)
    }
    #[doc = "Selects the CCRG"]
    #[inline(always)]
    pub fn ccrgsel(self) -> &'a mut crate::W<REG> {
        self.variant(Acrgibiassel::Ccrgsel)
    }
}
#[doc = "Field `ACRGTRIM` reader - ACRG Trim value"]
pub type AcrgtrimR = crate::FieldReader;
#[doc = "Field `ACRGTRIM` writer - ACRG Trim value"]
pub type AcrgtrimW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bit 0 - Software enablement for ACRG register. A value of 1 will allow writes to the register"]
    #[inline(always)]
    pub fn acrgswe(&self) -> AcrgsweR {
        AcrgsweR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Power down the ACRG."]
    #[inline(always)]
    pub fn acrgpwd(&self) -> AcrgpwdR {
        AcrgpwdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set the ACRG ibias. Note: the SWE mux select in PWRSEQ2SWE must be set for this to take effect. The inversion of this register is driven to analog."]
    #[inline(always)]
    pub fn acrgibiassel(&self) -> AcrgibiasselR {
        AcrgibiasselR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:7 - ACRG Trim value"]
    #[inline(always)]
    pub fn acrgtrim(&self) -> AcrgtrimR {
        AcrgtrimR::new(((self.bits >> 3) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Software enablement for ACRG register. A value of 1 will allow writes to the register"]
    #[inline(always)]
    #[must_use]
    pub fn acrgswe(&mut self) -> AcrgsweW<AcrgSpec> {
        AcrgsweW::new(self, 0)
    }
    #[doc = "Bit 1 - Power down the ACRG."]
    #[inline(always)]
    #[must_use]
    pub fn acrgpwd(&mut self) -> AcrgpwdW<AcrgSpec> {
        AcrgpwdW::new(self, 1)
    }
    #[doc = "Bit 2 - Set the ACRG ibias. Note: the SWE mux select in PWRSEQ2SWE must be set for this to take effect. The inversion of this register is driven to analog."]
    #[inline(always)]
    #[must_use]
    pub fn acrgibiassel(&mut self) -> AcrgibiasselW<AcrgSpec> {
        AcrgibiasselW::new(self, 2)
    }
    #[doc = "Bits 3:7 - ACRG Trim value"]
    #[inline(always)]
    #[must_use]
    pub fn acrgtrim(&mut self) -> AcrgtrimW<AcrgSpec> {
        AcrgtrimW::new(self, 3)
    }
}
#[doc = "Active Current Reference Generator Control\n\nYou can [`read`](crate::Reg::read) this register and get [`acrg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acrg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AcrgSpec;
impl crate::RegisterSpec for AcrgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`acrg::R`](R) reader structure"]
impl crate::Readable for AcrgSpec {}
#[doc = "`write(|w| ..)` method takes [`acrg::W`](W) writer structure"]
impl crate::Writable for AcrgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ACRG to value 0x80"]
impl crate::Resettable for AcrgSpec {
    const RESET_VALUE: u32 = 0x80;
}
