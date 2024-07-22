#[doc = "Register `ICODEFAULTADDR` reader"]
pub type R = crate::R<IcodefaultaddrSpec>;
#[doc = "Register `ICODEFAULTADDR` writer"]
pub type W = crate::W<IcodefaultaddrSpec>;
#[doc = "Field `ICODEFAULTADDR` reader - The ICODE bus address observed when a Bus Fault occurred. Once an address is captured in this field, it is held until the corresponding Fault Observed bit is cleared in the FAULTSTATUS register."]
pub type IcodefaultaddrR = crate::FieldReader<u32>;
#[doc = "Field `ICODEFAULTADDR` writer - The ICODE bus address observed when a Bus Fault occurred. Once an address is captured in this field, it is held until the corresponding Fault Observed bit is cleared in the FAULTSTATUS register."]
pub type IcodefaultaddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - The ICODE bus address observed when a Bus Fault occurred. Once an address is captured in this field, it is held until the corresponding Fault Observed bit is cleared in the FAULTSTATUS register."]
    #[inline(always)]
    pub fn icodefaultaddr(&self) -> IcodefaultaddrR {
        IcodefaultaddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The ICODE bus address observed when a Bus Fault occurred. Once an address is captured in this field, it is held until the corresponding Fault Observed bit is cleared in the FAULTSTATUS register."]
    #[inline(always)]
    #[must_use]
    pub fn icodefaultaddr(&mut self) -> IcodefaultaddrW<IcodefaultaddrSpec> {
        IcodefaultaddrW::new(self, 0)
    }
}
#[doc = "ICODE bus address which was present when a bus fault occurred.\n\nYou can [`read`](crate::Reg::read) this register and get [`icodefaultaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icodefaultaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcodefaultaddrSpec;
impl crate::RegisterSpec for IcodefaultaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icodefaultaddr::R`](R) reader structure"]
impl crate::Readable for IcodefaultaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`icodefaultaddr::W`](W) writer structure"]
impl crate::Writable for IcodefaultaddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICODEFAULTADDR to value 0"]
impl crate::Resettable for IcodefaultaddrSpec {
    const RESET_VALUE: u32 = 0;
}
