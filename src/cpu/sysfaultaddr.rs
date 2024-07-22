#[doc = "Register `SYSFAULTADDR` reader"]
pub type R = crate::R<SysfaultaddrSpec>;
#[doc = "Register `SYSFAULTADDR` writer"]
pub type W = crate::W<SysfaultaddrSpec>;
#[doc = "Field `SYSFAULTADDR` reader - SYS bus address observed when a Bus Fault occurred. Once an address is captured in this field, it is held until the corresponding Fault Observed bit is cleared in the FAULTSTATUS register."]
pub type SysfaultaddrR = crate::FieldReader<u32>;
#[doc = "Field `SYSFAULTADDR` writer - SYS bus address observed when a Bus Fault occurred. Once an address is captured in this field, it is held until the corresponding Fault Observed bit is cleared in the FAULTSTATUS register."]
pub type SysfaultaddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SYS bus address observed when a Bus Fault occurred. Once an address is captured in this field, it is held until the corresponding Fault Observed bit is cleared in the FAULTSTATUS register."]
    #[inline(always)]
    pub fn sysfaultaddr(&self) -> SysfaultaddrR {
        SysfaultaddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SYS bus address observed when a Bus Fault occurred. Once an address is captured in this field, it is held until the corresponding Fault Observed bit is cleared in the FAULTSTATUS register."]
    #[inline(always)]
    #[must_use]
    pub fn sysfaultaddr(&mut self) -> SysfaultaddrW<SysfaultaddrSpec> {
        SysfaultaddrW::new(self, 0)
    }
}
#[doc = "System bus address which was present when a bus fault occurred.\n\nYou can [`read`](crate::Reg::read) this register and get [`sysfaultaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysfaultaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysfaultaddrSpec;
impl crate::RegisterSpec for SysfaultaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sysfaultaddr::R`](R) reader structure"]
impl crate::Readable for SysfaultaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`sysfaultaddr::W`](W) writer structure"]
impl crate::Writable for SysfaultaddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYSFAULTADDR to value 0"]
impl crate::Resettable for SysfaultaddrSpec {
    const RESET_VALUE: u32 = 0;
}
