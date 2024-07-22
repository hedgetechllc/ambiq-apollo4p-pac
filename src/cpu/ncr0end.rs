#[doc = "Register `NCR0END` reader"]
pub type R = crate::R<Ncr0endSpec>;
#[doc = "Register `NCR0END` writer"]
pub type W = crate::W<Ncr0endSpec>;
#[doc = "Field `ADDR` reader - End address for non-cacheable region 0"]
pub type AddrR = crate::FieldReader<u32>;
#[doc = "Field `ADDR` writer - End address for non-cacheable region 0"]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
impl R {
    #[doc = "Bits 4:28 - End address for non-cacheable region 0"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new((self.bits >> 4) & 0x01ff_ffff)
    }
}
impl W {
    #[doc = "Bits 4:28 - End address for non-cacheable region 0"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> AddrW<Ncr0endSpec> {
        AddrW::new(self, 4)
    }
}
#[doc = "CM4 Cache Noncachable Region 0 End\n\nYou can [`read`](crate::Reg::read) this register and get [`ncr0end::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ncr0end::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ncr0endSpec;
impl crate::RegisterSpec for Ncr0endSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ncr0end::R`](R) reader structure"]
impl crate::Readable for Ncr0endSpec {}
#[doc = "`write(|w| ..)` method takes [`ncr0end::W`](W) writer structure"]
impl crate::Writable for Ncr0endSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NCR0END to value 0"]
impl crate::Resettable for Ncr0endSpec {
    const RESET_VALUE: u32 = 0;
}
