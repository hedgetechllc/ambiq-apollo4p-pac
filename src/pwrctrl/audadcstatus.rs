#[doc = "Register `AUDADCSTATUS` reader"]
pub type R = crate::R<AudadcstatusSpec>;
#[doc = "Register `AUDADCSTATUS` writer"]
pub type W = crate::W<AudadcstatusSpec>;
#[doc = "Field `AUDADCPWD` reader - This bit indicates that the ADC is powered down"]
pub type AudadcpwdR = crate::BitReader;
#[doc = "Field `AUDADCPWD` writer - This bit indicates that the ADC is powered down"]
pub type AudadcpwdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUDBGTPWD` reader - This bit indicates that the ADC Band Gap is powered down"]
pub type AudbgtpwdR = crate::BitReader;
#[doc = "Field `AUDBGTPWD` writer - This bit indicates that the ADC Band Gap is powered down"]
pub type AudbgtpwdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUDVPTATPWD` reader - This bit indicates that the ADC temperature sensor input buffer is powered down"]
pub type AudvptatpwdR = crate::BitReader;
#[doc = "Field `AUDVPTATPWD` writer - This bit indicates that the ADC temperature sensor input buffer is powered down"]
pub type AudvptatpwdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUDVBATPWD` reader - This bit indicates that the ADC VBAT resistor divider is powered down"]
pub type AudvbatpwdR = crate::BitReader;
#[doc = "Field `AUDVBATPWD` writer - This bit indicates that the ADC VBAT resistor divider is powered down"]
pub type AudvbatpwdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUDREFKEEPPWD` reader - This bit indicates that the ADC REFKEEP is powered down"]
pub type AudrefkeeppwdR = crate::BitReader;
#[doc = "Field `AUDREFKEEPPWD` writer - This bit indicates that the ADC REFKEEP is powered down"]
pub type AudrefkeeppwdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUDREFBUFPWD` reader - This bit indicates that the ADC REFBUF is powered down"]
pub type AudrefbufpwdR = crate::BitReader;
#[doc = "Field `AUDREFBUFPWD` writer - This bit indicates that the ADC REFBUF is powered down"]
pub type AudrefbufpwdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This bit indicates that the ADC is powered down"]
    #[inline(always)]
    pub fn audadcpwd(&self) -> AudadcpwdR {
        AudadcpwdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit indicates that the ADC Band Gap is powered down"]
    #[inline(always)]
    pub fn audbgtpwd(&self) -> AudbgtpwdR {
        AudbgtpwdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This bit indicates that the ADC temperature sensor input buffer is powered down"]
    #[inline(always)]
    pub fn audvptatpwd(&self) -> AudvptatpwdR {
        AudvptatpwdR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This bit indicates that the ADC VBAT resistor divider is powered down"]
    #[inline(always)]
    pub fn audvbatpwd(&self) -> AudvbatpwdR {
        AudvbatpwdR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This bit indicates that the ADC REFKEEP is powered down"]
    #[inline(always)]
    pub fn audrefkeeppwd(&self) -> AudrefkeeppwdR {
        AudrefkeeppwdR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This bit indicates that the ADC REFBUF is powered down"]
    #[inline(always)]
    pub fn audrefbufpwd(&self) -> AudrefbufpwdR {
        AudrefbufpwdR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit indicates that the ADC is powered down"]
    #[inline(always)]
    #[must_use]
    pub fn audadcpwd(&mut self) -> AudadcpwdW<AudadcstatusSpec> {
        AudadcpwdW::new(self, 0)
    }
    #[doc = "Bit 1 - This bit indicates that the ADC Band Gap is powered down"]
    #[inline(always)]
    #[must_use]
    pub fn audbgtpwd(&mut self) -> AudbgtpwdW<AudadcstatusSpec> {
        AudbgtpwdW::new(self, 1)
    }
    #[doc = "Bit 2 - This bit indicates that the ADC temperature sensor input buffer is powered down"]
    #[inline(always)]
    #[must_use]
    pub fn audvptatpwd(&mut self) -> AudvptatpwdW<AudadcstatusSpec> {
        AudvptatpwdW::new(self, 2)
    }
    #[doc = "Bit 3 - This bit indicates that the ADC VBAT resistor divider is powered down"]
    #[inline(always)]
    #[must_use]
    pub fn audvbatpwd(&mut self) -> AudvbatpwdW<AudadcstatusSpec> {
        AudvbatpwdW::new(self, 3)
    }
    #[doc = "Bit 4 - This bit indicates that the ADC REFKEEP is powered down"]
    #[inline(always)]
    #[must_use]
    pub fn audrefkeeppwd(&mut self) -> AudrefkeeppwdW<AudadcstatusSpec> {
        AudrefkeeppwdW::new(self, 4)
    }
    #[doc = "Bit 5 - This bit indicates that the ADC REFBUF is powered down"]
    #[inline(always)]
    #[must_use]
    pub fn audrefbufpwd(&mut self) -> AudrefbufpwdW<AudadcstatusSpec> {
        AudrefbufpwdW::new(self, 5)
    }
}
#[doc = "This provides the power status for various blocks within the audio ADC. These status comes directly from the audio ADC module and is captured through this interface.\n\nYou can [`read`](crate::Reg::read) this register and get [`audadcstatus::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`audadcstatus::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AudadcstatusSpec;
impl crate::RegisterSpec for AudadcstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`audadcstatus::R`](R) reader structure"]
impl crate::Readable for AudadcstatusSpec {}
#[doc = "`write(|w| ..)` method takes [`audadcstatus::W`](W) writer structure"]
impl crate::Writable for AudadcstatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AUDADCSTATUS to value 0x3f"]
impl crate::Resettable for AudadcstatusSpec {
    const RESET_VALUE: u32 = 0x3f;
}
