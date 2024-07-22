#[doc = "Register `PKABUFFADDR` reader"]
pub type R = crate::R<PkabuffaddrSpec>;
#[doc = "Register `PKABUFFADDR` writer"]
pub type W = crate::W<PkabuffaddrSpec>;
#[doc = "Field `PKABUFADDR` reader - Contains the physical address in memory to map the buffer registers."]
pub type PkabufaddrR = crate::FieldReader<u16>;
#[doc = "Field `PKABUFADDR` writer - Contains the physical address in memory to map the buffer registers."]
pub type PkabufaddrW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Contains the physical address in memory to map the buffer registers."]
    #[inline(always)]
    pub fn pkabufaddr(&self) -> PkabufaddrR {
        PkabufaddrR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Contains the physical address in memory to map the buffer registers."]
    #[inline(always)]
    #[must_use]
    pub fn pkabufaddr(&mut self) -> PkabufaddrW<PkabuffaddrSpec> {
        PkabufaddrW::new(self, 0)
    }
}
#[doc = "This register maps the virtual buffer registers to a physical address in memory.\n\nYou can [`read`](crate::Reg::read) this register and get [`pkabuffaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pkabuffaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PkabuffaddrSpec;
impl crate::RegisterSpec for PkabuffaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pkabuffaddr::R`](R) reader structure"]
impl crate::Readable for PkabuffaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`pkabuffaddr::W`](W) writer structure"]
impl crate::Writable for PkabuffaddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PKABUFFADDR to value 0"]
impl crate::Resettable for PkabuffaddrSpec {
    const RESET_VALUE: u32 = 0;
}
