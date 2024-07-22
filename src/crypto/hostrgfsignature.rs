#[doc = "Register `HOSTRGFSIGNATURE` reader"]
pub type R = crate::R<HostrgfsignatureSpec>;
#[doc = "Register `HOSTRGFSIGNATURE` writer"]
pub type W = crate::W<HostrgfsignatureSpec>;
#[doc = "Field `HOSTSIGNATURE` reader - Identification 'signature': always returns a fixed value, used by Host driver to verify CryptoCell presence at this address."]
pub type HostsignatureR = crate::FieldReader<u32>;
#[doc = "Field `HOSTSIGNATURE` writer - Identification 'signature': always returns a fixed value, used by Host driver to verify CryptoCell presence at this address."]
pub type HostsignatureW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Identification 'signature': always returns a fixed value, used by Host driver to verify CryptoCell presence at this address."]
    #[inline(always)]
    pub fn hostsignature(&self) -> HostsignatureR {
        HostsignatureR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Identification 'signature': always returns a fixed value, used by Host driver to verify CryptoCell presence at this address."]
    #[inline(always)]
    #[must_use]
    pub fn hostsignature(&mut self) -> HostsignatureW<HostrgfsignatureSpec> {
        HostsignatureW::new(self, 0)
    }
}
#[doc = "This register holds the CryptoCell product signature.\n\nYou can [`read`](crate::Reg::read) this register and get [`hostrgfsignature::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hostrgfsignature::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HostrgfsignatureSpec;
impl crate::RegisterSpec for HostrgfsignatureSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hostrgfsignature::R`](R) reader structure"]
impl crate::Readable for HostrgfsignatureSpec {}
#[doc = "`write(|w| ..)` method takes [`hostrgfsignature::W`](W) writer structure"]
impl crate::Writable for HostrgfsignatureSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HOSTRGFSIGNATURE to value 0x10e0_0000"]
impl crate::Resettable for HostrgfsignatureSpec {
    const RESET_VALUE: u32 = 0x10e0_0000;
}
