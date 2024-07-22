#[doc = "Register `DMASRAMWPROT0` reader"]
pub type R = crate::R<Dmasramwprot0Spec>;
#[doc = "Register `DMASRAMWPROT0` writer"]
pub type W = crate::W<Dmasramwprot0Spec>;
#[doc = "Field `DMAWPROT0` reader - Write protect SRAM from DMA. Each bit provides write protection for an 8KB region of memory. When set to 1, the region will be protected from DMA writes, when set to 0, DMA may write the region."]
pub type Dmawprot0R = crate::FieldReader<u32>;
#[doc = "Field `DMAWPROT0` writer - Write protect SRAM from DMA. Each bit provides write protection for an 8KB region of memory. When set to 1, the region will be protected from DMA writes, when set to 0, DMA may write the region."]
pub type Dmawprot0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Write protect SRAM from DMA. Each bit provides write protection for an 8KB region of memory. When set to 1, the region will be protected from DMA writes, when set to 0, DMA may write the region."]
    #[inline(always)]
    pub fn dmawprot0(&self) -> Dmawprot0R {
        Dmawprot0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Write protect SRAM from DMA. Each bit provides write protection for an 8KB region of memory. When set to 1, the region will be protected from DMA writes, when set to 0, DMA may write the region."]
    #[inline(always)]
    #[must_use]
    pub fn dmawprot0(&mut self) -> Dmawprot0W<Dmasramwprot0Spec> {
        Dmawprot0W::new(self, 0)
    }
}
#[doc = "These bits write-protect system SRAM from DMA operations in 8KB chunks.\n\nYou can [`read`](crate::Reg::read) this register and get [`dmasramwprot0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmasramwprot0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dmasramwprot0Spec;
impl crate::RegisterSpec for Dmasramwprot0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmasramwprot0::R`](R) reader structure"]
impl crate::Readable for Dmasramwprot0Spec {}
#[doc = "`write(|w| ..)` method takes [`dmasramwprot0::W`](W) writer structure"]
impl crate::Writable for Dmasramwprot0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMASRAMWPROT0 to value 0"]
impl crate::Resettable for Dmasramwprot0Spec {
    const RESET_VALUE: u32 = 0;
}
