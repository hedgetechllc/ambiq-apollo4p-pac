#[doc = "Register `DMASRAMRPROT1` reader"]
pub type R = crate::R<Dmasramrprot1Spec>;
#[doc = "Register `DMASRAMRPROT1` writer"]
pub type W = crate::W<Dmasramrprot1Spec>;
#[doc = "Field `DMARPROT1` reader - Read protect SRAM from DMA. Each bit provides write protection for an 8KB region of memory. When set to 1, the region will be protected from DMA reads, when set to 0, DMA may read the region."]
pub type Dmarprot1R = crate::FieldReader<u16>;
#[doc = "Field `DMARPROT1` writer - Read protect SRAM from DMA. Each bit provides write protection for an 8KB region of memory. When set to 1, the region will be protected from DMA reads, when set to 0, DMA may read the region."]
pub type Dmarprot1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Read protect SRAM from DMA. Each bit provides write protection for an 8KB region of memory. When set to 1, the region will be protected from DMA reads, when set to 0, DMA may read the region."]
    #[inline(always)]
    pub fn dmarprot1(&self) -> Dmarprot1R {
        Dmarprot1R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Read protect SRAM from DMA. Each bit provides write protection for an 8KB region of memory. When set to 1, the region will be protected from DMA reads, when set to 0, DMA may read the region."]
    #[inline(always)]
    #[must_use]
    pub fn dmarprot1(&mut self) -> Dmarprot1W<Dmasramrprot1Spec> {
        Dmarprot1W::new(self, 0)
    }
}
#[doc = "These bits read-protect system SRAM from DMA operations in 8KB chunks.\n\nYou can [`read`](crate::Reg::read) this register and get [`dmasramrprot1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmasramrprot1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dmasramrprot1Spec;
impl crate::RegisterSpec for Dmasramrprot1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmasramrprot1::R`](R) reader structure"]
impl crate::Readable for Dmasramrprot1Spec {}
#[doc = "`write(|w| ..)` method takes [`dmasramrprot1::W`](W) writer structure"]
impl crate::Writable for Dmasramrprot1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMASRAMRPROT1 to value 0"]
impl crate::Resettable for Dmasramrprot1Spec {
    const RESET_VALUE: u32 = 0;
}
