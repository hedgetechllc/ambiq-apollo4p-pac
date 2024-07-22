#[doc = "Register `HOSTSHADOWKCEICVREG` reader"]
pub type R = crate::R<HostshadowkceicvregSpec>;
#[doc = "Register `HOSTSHADOWKCEICVREG` writer"]
pub type W = crate::W<HostshadowkceicvregSpec>;
#[doc = "Field `HOSTSHADOWKCEICVREG` reader - This field is used to update the KCEICV registers when the device is in CM or DM mode, The KCEICV is updated by shifting"]
pub type HostshadowkceicvregR = crate::BitReader;
#[doc = "Field `HOSTSHADOWKCEICVREG` writer - This field is used to update the KCEICV registers when the device is in CM or DM mode, The KCEICV is updated by shifting"]
pub type HostshadowkceicvregW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This field is used to update the KCEICV registers when the device is in CM or DM mode, The KCEICV is updated by shifting"]
    #[inline(always)]
    pub fn hostshadowkceicvreg(&self) -> HostshadowkceicvregR {
        HostshadowkceicvregR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This field is used to update the KCEICV registers when the device is in CM or DM mode, The KCEICV is updated by shifting"]
    #[inline(always)]
    #[must_use]
    pub fn hostshadowkceicvreg(&mut self) -> HostshadowkceicvregW<HostshadowkceicvregSpec> {
        HostshadowkceicvregW::new(self, 0)
    }
}
#[doc = "This register interface is used to update the KCEICV registers when the device is in CM or DM mode, The KCEICV is updated by shifting\n\nYou can [`read`](crate::Reg::read) this register and get [`hostshadowkceicvreg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hostshadowkceicvreg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HostshadowkceicvregSpec;
impl crate::RegisterSpec for HostshadowkceicvregSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hostshadowkceicvreg::R`](R) reader structure"]
impl crate::Readable for HostshadowkceicvregSpec {}
#[doc = "`write(|w| ..)` method takes [`hostshadowkceicvreg::W`](W) writer structure"]
impl crate::Writable for HostshadowkceicvregSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HOSTSHADOWKCEICVREG to value 0"]
impl crate::Resettable for HostshadowkceicvregSpec {
    const RESET_VALUE: u32 = 0;
}
