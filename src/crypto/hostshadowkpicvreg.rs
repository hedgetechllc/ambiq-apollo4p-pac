#[doc = "Register `HOSTSHADOWKPICVREG` reader"]
pub type R = crate::R<HostshadowkpicvregSpec>;
#[doc = "Register `HOSTSHADOWKPICVREG` writer"]
pub type W = crate::W<HostshadowkpicvregSpec>;
#[doc = "Field `HOSTSHADOWKPICVREG` reader - This field is used to update the KPICV registers when the device is in CM or DM mode, The KPICV is updated by shifting"]
pub type HostshadowkpicvregR = crate::BitReader;
#[doc = "Field `HOSTSHADOWKPICVREG` writer - This field is used to update the KPICV registers when the device is in CM or DM mode, The KPICV is updated by shifting"]
pub type HostshadowkpicvregW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This field is used to update the KPICV registers when the device is in CM or DM mode, The KPICV is updated by shifting"]
    #[inline(always)]
    pub fn hostshadowkpicvreg(&self) -> HostshadowkpicvregR {
        HostshadowkpicvregR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This field is used to update the KPICV registers when the device is in CM or DM mode, The KPICV is updated by shifting"]
    #[inline(always)]
    #[must_use]
    pub fn hostshadowkpicvreg(&mut self) -> HostshadowkpicvregW<HostshadowkpicvregSpec> {
        HostshadowkpicvregW::new(self, 0)
    }
}
#[doc = "This register interface is used to update the KPICV registers when the device is in CM or DM mode, The KPICV is updated by shifting\n\nYou can [`read`](crate::Reg::read) this register and get [`hostshadowkpicvreg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hostshadowkpicvreg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HostshadowkpicvregSpec;
impl crate::RegisterSpec for HostshadowkpicvregSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hostshadowkpicvreg::R`](R) reader structure"]
impl crate::Readable for HostshadowkpicvregSpec {}
#[doc = "`write(|w| ..)` method takes [`hostshadowkpicvreg::W`](W) writer structure"]
impl crate::Writable for HostshadowkpicvregSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HOSTSHADOWKPICVREG to value 0"]
impl crate::Resettable for HostshadowkpicvregSpec {
    const RESET_VALUE: u32 = 0;
}
