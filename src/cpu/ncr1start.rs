#[doc = "Register `NCR1START` reader"]
pub type R = crate::R<Ncr1startSpec>;
#[doc = "Register `NCR1START` writer"]
pub type W = crate::W<Ncr1startSpec>;
#[doc = "Field `ADDR` reader - Start address for non-cacheable region 1"]
pub type AddrR = crate::FieldReader<u32>;
#[doc = "Field `ADDR` writer - Start address for non-cacheable region 1"]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
impl R {
    #[doc = "Bits 4:28 - Start address for non-cacheable region 1"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new((self.bits >> 4) & 0x01ff_ffff)
    }
}
impl W {
    #[doc = "Bits 4:28 - Start address for non-cacheable region 1"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> AddrW<Ncr1startSpec> {
        AddrW::new(self, 4)
    }
}
#[doc = "CM4 Cache Noncachable Region 1 Start\n\nYou can [`read`](crate::Reg::read) this register and get [`ncr1start::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ncr1start::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ncr1startSpec;
impl crate::RegisterSpec for Ncr1startSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ncr1start::R`](R) reader structure"]
impl crate::Readable for Ncr1startSpec {}
#[doc = "`write(|w| ..)` method takes [`ncr1start::W`](W) writer structure"]
impl crate::Writable for Ncr1startSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NCR1START to value 0"]
impl crate::Resettable for Ncr1startSpec {
    const RESET_VALUE: u32 = 0;
}
