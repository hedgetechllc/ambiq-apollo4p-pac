#[doc = "Register `PKASRAMADDR` reader"]
pub type R = crate::R<PkasramaddrSpec>;
#[doc = "Register `PKASRAMADDR` writer"]
pub type W = crate::W<PkasramaddrSpec>;
#[doc = "Field `PKASRAMADDR` reader - PKA SRAM write starting address"]
pub type PkasramaddrR = crate::FieldReader<u32>;
#[doc = "Field `PKASRAMADDR` writer - PKA SRAM write starting address"]
pub type PkasramaddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - PKA SRAM write starting address"]
    #[inline(always)]
    pub fn pkasramaddr(&self) -> PkasramaddrR {
        PkasramaddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - PKA SRAM write starting address"]
    #[inline(always)]
    #[must_use]
    pub fn pkasramaddr(&mut self) -> PkasramaddrW<PkasramaddrSpec> {
        PkasramaddrW::new(self, 0)
    }
}
#[doc = "first address given to PKA SRAM for write transactions.\n\nYou can [`read`](crate::Reg::read) this register and get [`pkasramaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pkasramaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PkasramaddrSpec;
impl crate::RegisterSpec for PkasramaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pkasramaddr::R`](R) reader structure"]
impl crate::Readable for PkasramaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`pkasramaddr::W`](W) writer structure"]
impl crate::Writable for PkasramaddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PKASRAMADDR to value 0"]
impl crate::Resettable for PkasramaddrSpec {
    const RESET_VALUE: u32 = 0;
}
