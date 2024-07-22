#[doc = "Register `HOSTSHADOWKCEREG` reader"]
pub type R = crate::R<HostshadowkceregSpec>;
#[doc = "Register `HOSTSHADOWKCEREG` writer"]
pub type W = crate::W<HostshadowkceregSpec>;
#[doc = "Field `HOSTSHADOWKCEREG` reader - This field is used to update the KCE registers when the device is in CM or DM mode, The KCE is updated by shifting"]
pub type HostshadowkceregR = crate::BitReader;
#[doc = "Field `HOSTSHADOWKCEREG` writer - This field is used to update the KCE registers when the device is in CM or DM mode, The KCE is updated by shifting"]
pub type HostshadowkceregW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This field is used to update the KCE registers when the device is in CM or DM mode, The KCE is updated by shifting"]
    #[inline(always)]
    pub fn hostshadowkcereg(&self) -> HostshadowkceregR {
        HostshadowkceregR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This field is used to update the KCE registers when the device is in CM or DM mode, The KCE is updated by shifting"]
    #[inline(always)]
    #[must_use]
    pub fn hostshadowkcereg(&mut self) -> HostshadowkceregW<HostshadowkceregSpec> {
        HostshadowkceregW::new(self, 0)
    }
}
#[doc = "This register interface is used to update the KCE registers when the device is in CM or DM mode, The KCE is updated by shifting\n\nYou can [`read`](crate::Reg::read) this register and get [`hostshadowkcereg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hostshadowkcereg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HostshadowkceregSpec;
impl crate::RegisterSpec for HostshadowkceregSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hostshadowkcereg::R`](R) reader structure"]
impl crate::Readable for HostshadowkceregSpec {}
#[doc = "`write(|w| ..)` method takes [`hostshadowkcereg::W`](W) writer structure"]
impl crate::Writable for HostshadowkceregSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HOSTSHADOWKCEREG to value 0"]
impl crate::Resettable for HostshadowkceregSpec {
    const RESET_VALUE: u32 = 0;
}
