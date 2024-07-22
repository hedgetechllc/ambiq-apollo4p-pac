#[doc = "Register `DCODEFAULTADDR` reader"]
pub type R = crate::R<DcodefaultaddrSpec>;
#[doc = "Register `DCODEFAULTADDR` writer"]
pub type W = crate::W<DcodefaultaddrSpec>;
#[doc = "Field `DCODEFAULTADDR` reader - The DCODE bus address observed when a Bus Fault occurred. Once an address is captured in this field, it is held until the corresponding Fault Observed bit is cleared in the FAULTSTATUS register."]
pub type DcodefaultaddrR = crate::FieldReader<u32>;
#[doc = "Field `DCODEFAULTADDR` writer - The DCODE bus address observed when a Bus Fault occurred. Once an address is captured in this field, it is held until the corresponding Fault Observed bit is cleared in the FAULTSTATUS register."]
pub type DcodefaultaddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - The DCODE bus address observed when a Bus Fault occurred. Once an address is captured in this field, it is held until the corresponding Fault Observed bit is cleared in the FAULTSTATUS register."]
    #[inline(always)]
    pub fn dcodefaultaddr(&self) -> DcodefaultaddrR {
        DcodefaultaddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The DCODE bus address observed when a Bus Fault occurred. Once an address is captured in this field, it is held until the corresponding Fault Observed bit is cleared in the FAULTSTATUS register."]
    #[inline(always)]
    #[must_use]
    pub fn dcodefaultaddr(&mut self) -> DcodefaultaddrW<DcodefaultaddrSpec> {
        DcodefaultaddrW::new(self, 0)
    }
}
#[doc = "DCODE bus address which was present when a bus fault occurred.\n\nYou can [`read`](crate::Reg::read) this register and get [`dcodefaultaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcodefaultaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcodefaultaddrSpec;
impl crate::RegisterSpec for DcodefaultaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcodefaultaddr::R`](R) reader structure"]
impl crate::Readable for DcodefaultaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`dcodefaultaddr::W`](W) writer structure"]
impl crate::Writable for DcodefaultaddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCODEFAULTADDR to value 0"]
impl crate::Resettable for DcodefaultaddrSpec {
    const RESET_VALUE: u32 = 0;
}
