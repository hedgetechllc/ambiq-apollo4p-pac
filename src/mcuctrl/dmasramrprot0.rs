#[doc = "Register `DMASRAMRPROT0` reader"]
pub type R = crate::R<Dmasramrprot0Spec>;
#[doc = "Register `DMASRAMRPROT0` writer"]
pub type W = crate::W<Dmasramrprot0Spec>;
#[doc = "Field `DMARPROT0` reader - Read protect SRAM from DMA. Each bit provides write protection for an 8KB region of memory. When set to 1, the region will be protected from DMA reads, when set to 0, DMA may read the region."]
pub type Dmarprot0R = crate::FieldReader<u32>;
#[doc = "Field `DMARPROT0` writer - Read protect SRAM from DMA. Each bit provides write protection for an 8KB region of memory. When set to 1, the region will be protected from DMA reads, when set to 0, DMA may read the region."]
pub type Dmarprot0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Read protect SRAM from DMA. Each bit provides write protection for an 8KB region of memory. When set to 1, the region will be protected from DMA reads, when set to 0, DMA may read the region."]
    #[inline(always)]
    pub fn dmarprot0(&self) -> Dmarprot0R {
        Dmarprot0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Read protect SRAM from DMA. Each bit provides write protection for an 8KB region of memory. When set to 1, the region will be protected from DMA reads, when set to 0, DMA may read the region."]
    #[inline(always)]
    #[must_use]
    pub fn dmarprot0(&mut self) -> Dmarprot0W<Dmasramrprot0Spec> {
        Dmarprot0W::new(self, 0)
    }
}
#[doc = "These bits read-protect system SRAM from DMA operations in 8KB chunks.\n\nYou can [`read`](crate::Reg::read) this register and get [`dmasramrprot0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmasramrprot0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dmasramrprot0Spec;
impl crate::RegisterSpec for Dmasramrprot0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmasramrprot0::R`](R) reader structure"]
impl crate::Readable for Dmasramrprot0Spec {}
#[doc = "`write(|w| ..)` method takes [`dmasramrprot0::W`](W) writer structure"]
impl crate::Writable for Dmasramrprot0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMASRAMRPROT0 to value 0"]
impl crate::Resettable for Dmasramrprot0Spec {
    const RESET_VALUE: u32 = 0;
}
