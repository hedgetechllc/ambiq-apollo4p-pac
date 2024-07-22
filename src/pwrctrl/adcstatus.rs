#[doc = "Register `ADCSTATUS` reader"]
pub type R = crate::R<AdcstatusSpec>;
#[doc = "Register `ADCSTATUS` writer"]
pub type W = crate::W<AdcstatusSpec>;
#[doc = "Field `ADCPWD` reader - This bit indicates that the ADC is powered down"]
pub type AdcpwdR = crate::BitReader;
#[doc = "Field `ADCPWD` writer - This bit indicates that the ADC is powered down"]
pub type AdcpwdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BGTPWD` reader - This bit indicates that the ADC Band Gap is powered down"]
pub type BgtpwdR = crate::BitReader;
#[doc = "Field `BGTPWD` writer - This bit indicates that the ADC Band Gap is powered down"]
pub type BgtpwdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VPTATPWD` reader - This bit indicates that the ADC temperature sensor input buffer is powered down"]
pub type VptatpwdR = crate::BitReader;
#[doc = "Field `VPTATPWD` writer - This bit indicates that the ADC temperature sensor input buffer is powered down"]
pub type VptatpwdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBATPWD` reader - This bit indicates that the ADC VBAT resistor divider is powered down"]
pub type VbatpwdR = crate::BitReader;
#[doc = "Field `VBATPWD` writer - This bit indicates that the ADC VBAT resistor divider is powered down"]
pub type VbatpwdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REFKEEPPWD` reader - This bit indicates that the ADC REFKEEP is powered down"]
pub type RefkeeppwdR = crate::BitReader;
#[doc = "Field `REFKEEPPWD` writer - This bit indicates that the ADC REFKEEP is powered down"]
pub type RefkeeppwdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REFBUFPWD` reader - This bit indicates that the ADC REFBUF is powered down"]
pub type RefbufpwdR = crate::BitReader;
#[doc = "Field `REFBUFPWD` writer - This bit indicates that the ADC REFBUF is powered down"]
pub type RefbufpwdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This bit indicates that the ADC is powered down"]
    #[inline(always)]
    pub fn adcpwd(&self) -> AdcpwdR {
        AdcpwdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit indicates that the ADC Band Gap is powered down"]
    #[inline(always)]
    pub fn bgtpwd(&self) -> BgtpwdR {
        BgtpwdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This bit indicates that the ADC temperature sensor input buffer is powered down"]
    #[inline(always)]
    pub fn vptatpwd(&self) -> VptatpwdR {
        VptatpwdR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This bit indicates that the ADC VBAT resistor divider is powered down"]
    #[inline(always)]
    pub fn vbatpwd(&self) -> VbatpwdR {
        VbatpwdR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This bit indicates that the ADC REFKEEP is powered down"]
    #[inline(always)]
    pub fn refkeeppwd(&self) -> RefkeeppwdR {
        RefkeeppwdR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This bit indicates that the ADC REFBUF is powered down"]
    #[inline(always)]
    pub fn refbufpwd(&self) -> RefbufpwdR {
        RefbufpwdR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit indicates that the ADC is powered down"]
    #[inline(always)]
    #[must_use]
    pub fn adcpwd(&mut self) -> AdcpwdW<AdcstatusSpec> {
        AdcpwdW::new(self, 0)
    }
    #[doc = "Bit 1 - This bit indicates that the ADC Band Gap is powered down"]
    #[inline(always)]
    #[must_use]
    pub fn bgtpwd(&mut self) -> BgtpwdW<AdcstatusSpec> {
        BgtpwdW::new(self, 1)
    }
    #[doc = "Bit 2 - This bit indicates that the ADC temperature sensor input buffer is powered down"]
    #[inline(always)]
    #[must_use]
    pub fn vptatpwd(&mut self) -> VptatpwdW<AdcstatusSpec> {
        VptatpwdW::new(self, 2)
    }
    #[doc = "Bit 3 - This bit indicates that the ADC VBAT resistor divider is powered down"]
    #[inline(always)]
    #[must_use]
    pub fn vbatpwd(&mut self) -> VbatpwdW<AdcstatusSpec> {
        VbatpwdW::new(self, 3)
    }
    #[doc = "Bit 4 - This bit indicates that the ADC REFKEEP is powered down"]
    #[inline(always)]
    #[must_use]
    pub fn refkeeppwd(&mut self) -> RefkeeppwdW<AdcstatusSpec> {
        RefkeeppwdW::new(self, 4)
    }
    #[doc = "Bit 5 - This bit indicates that the ADC REFBUF is powered down"]
    #[inline(always)]
    #[must_use]
    pub fn refbufpwd(&mut self) -> RefbufpwdW<AdcstatusSpec> {
        RefbufpwdW::new(self, 5)
    }
}
#[doc = "This provides the power status for various blocks within the ADC. These status comes directly from the ADC module and is captured through this interface.\n\nYou can [`read`](crate::Reg::read) this register and get [`adcstatus::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcstatus::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcstatusSpec;
impl crate::RegisterSpec for AdcstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcstatus::R`](R) reader structure"]
impl crate::Readable for AdcstatusSpec {}
#[doc = "`write(|w| ..)` method takes [`adcstatus::W`](W) writer structure"]
impl crate::Writable for AdcstatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADCSTATUS to value 0x3f"]
impl crate::Resettable for AdcstatusSpec {
    const RESET_VALUE: u32 = 0x3f;
}
