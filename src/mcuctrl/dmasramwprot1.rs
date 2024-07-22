#[doc = "Register `DMASRAMWPROT1` reader"]
pub type R = crate::R<Dmasramwprot1Spec>;
#[doc = "Register `DMASRAMWPROT1` writer"]
pub type W = crate::W<Dmasramwprot1Spec>;
#[doc = "Field `DMAWPROT1` reader - Write protect SRAM from DMA. Each bit provides write protection for an 8KB region of memory. When set to 1, the region will be protected from DMA writes, when set to 0, DMA may write the region."]
pub type Dmawprot1R = crate::FieldReader<u16>;
#[doc = "Field `DMAWPROT1` writer - Write protect SRAM from DMA. Each bit provides write protection for an 8KB region of memory. When set to 1, the region will be protected from DMA writes, when set to 0, DMA may write the region."]
pub type Dmawprot1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Write protect SRAM from DMA. Each bit provides write protection for an 8KB region of memory. When set to 1, the region will be protected from DMA writes, when set to 0, DMA may write the region."]
    #[inline(always)]
    pub fn dmawprot1(&self) -> Dmawprot1R {
        Dmawprot1R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Write protect SRAM from DMA. Each bit provides write protection for an 8KB region of memory. When set to 1, the region will be protected from DMA writes, when set to 0, DMA may write the region."]
    #[inline(always)]
    #[must_use]
    pub fn dmawprot1(&mut self) -> Dmawprot1W<Dmasramwprot1Spec> {
        Dmawprot1W::new(self, 0)
    }
}
#[doc = "These bits write-protect system SRAM from DMA operations in 8KB chunks.\n\nYou can [`read`](crate::Reg::read) this register and get [`dmasramwprot1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmasramwprot1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dmasramwprot1Spec;
impl crate::RegisterSpec for Dmasramwprot1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmasramwprot1::R`](R) reader structure"]
impl crate::Readable for Dmasramwprot1Spec {}
#[doc = "`write(|w| ..)` method takes [`dmasramwprot1::W`](W) writer structure"]
impl crate::Writable for Dmasramwprot1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMASRAMWPROT1 to value 0"]
impl crate::Resettable for Dmasramwprot1Spec {
    const RESET_VALUE: u32 = 0;
}
