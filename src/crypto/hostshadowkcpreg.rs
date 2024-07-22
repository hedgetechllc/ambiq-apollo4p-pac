#[doc = "Register `HOSTSHADOWKCPREG` reader"]
pub type R = crate::R<HostshadowkcpregSpec>;
#[doc = "Register `HOSTSHADOWKCPREG` writer"]
pub type W = crate::W<HostshadowkcpregSpec>;
#[doc = "Field `HOSTSHADOWKCPREG` reader - This field is used to update the KCP registers when the device is in CM or DM mode, The KCP is updated by shifting"]
pub type HostshadowkcpregR = crate::BitReader;
#[doc = "Field `HOSTSHADOWKCPREG` writer - This field is used to update the KCP registers when the device is in CM or DM mode, The KCP is updated by shifting"]
pub type HostshadowkcpregW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This field is used to update the KCP registers when the device is in CM or DM mode, The KCP is updated by shifting"]
    #[inline(always)]
    pub fn hostshadowkcpreg(&self) -> HostshadowkcpregR {
        HostshadowkcpregR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This field is used to update the KCP registers when the device is in CM or DM mode, The KCP is updated by shifting"]
    #[inline(always)]
    #[must_use]
    pub fn hostshadowkcpreg(&mut self) -> HostshadowkcpregW<HostshadowkcpregSpec> {
        HostshadowkcpregW::new(self, 0)
    }
}
#[doc = "This register interface is used to update the KCP registers when the device is in CM or DM mode, The KCP is updated by shifting\n\nYou can [`read`](crate::Reg::read) this register and get [`hostshadowkcpreg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hostshadowkcpreg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HostshadowkcpregSpec;
impl crate::RegisterSpec for HostshadowkcpregSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hostshadowkcpreg::R`](R) reader structure"]
impl crate::Readable for HostshadowkcpregSpec {}
#[doc = "`write(|w| ..)` method takes [`hostshadowkcpreg::W`](W) writer structure"]
impl crate::Writable for HostshadowkcpregSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HOSTSHADOWKCPREG to value 0"]
impl crate::Resettable for HostshadowkcpregSpec {
    const RESET_VALUE: u32 = 0;
}
