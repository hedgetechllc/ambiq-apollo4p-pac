#[doc = "Register `HOSTSHADOWKDRREG` reader"]
pub type R = crate::R<HostshadowkdrregSpec>;
#[doc = "Register `HOSTSHADOWKDRREG` writer"]
pub type W = crate::W<HostshadowkdrregSpec>;
#[doc = "Field `HOSTSHADOWKDRREG` reader - This field is used to update the KDR registers when the device is in CM , DM or RMA mode, The KDR is updated by shifting ."]
pub type HostshadowkdrregR = crate::BitReader;
#[doc = "Field `HOSTSHADOWKDRREG` writer - This field is used to update the KDR registers when the device is in CM , DM or RMA mode, The KDR is updated by shifting ."]
pub type HostshadowkdrregW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This field is used to update the KDR registers when the device is in CM , DM or RMA mode, The KDR is updated by shifting ."]
    #[inline(always)]
    pub fn hostshadowkdrreg(&self) -> HostshadowkdrregR {
        HostshadowkdrregR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This field is used to update the KDR registers when the device is in CM , DM or RMA mode, The KDR is updated by shifting ."]
    #[inline(always)]
    #[must_use]
    pub fn hostshadowkdrreg(&mut self) -> HostshadowkdrregW<HostshadowkdrregSpec> {
        HostshadowkdrregW::new(self, 0)
    }
}
#[doc = "This register interface is used to update the RKEK(KDR) registers when the device is in CM or DM mode , it is Write-once (per warm boot) in RMA LCS, The RKEK is updated by shifting .\n\nYou can [`read`](crate::Reg::read) this register and get [`hostshadowkdrreg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hostshadowkdrreg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HostshadowkdrregSpec;
impl crate::RegisterSpec for HostshadowkdrregSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hostshadowkdrreg::R`](R) reader structure"]
impl crate::Readable for HostshadowkdrregSpec {}
#[doc = "`write(|w| ..)` method takes [`hostshadowkdrreg::W`](W) writer structure"]
impl crate::Writable for HostshadowkdrregSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HOSTSHADOWKDRREG to value 0"]
impl crate::Resettable for HostshadowkdrregSpec {
    const RESET_VALUE: u32 = 0;
}
