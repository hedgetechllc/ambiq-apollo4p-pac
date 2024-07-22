#[doc = "Register `DMATHRESH` reader"]
pub type R = crate::R<DmathreshSpec>;
#[doc = "Register `DMATHRESH` writer"]
pub type W = crate::W<DmathreshSpec>;
#[doc = "Field `DMATXTHRESH` reader - DMA transfer FIFO level trigger. For read operations, DMA is triggered when the FIFO level is greater than this value. For write operations, DMA is triggered when the FIFO level is less than this level. Each DMA operation will consist of BCOUNT bytes."]
pub type DmatxthreshR = crate::FieldReader;
#[doc = "Field `DMATXTHRESH` writer - DMA transfer FIFO level trigger. For read operations, DMA is triggered when the FIFO level is greater than this value. For write operations, DMA is triggered when the FIFO level is less than this level. Each DMA operation will consist of BCOUNT bytes."]
pub type DmatxthreshW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DMARXTHRESH` reader - DMA transfer FIFO level trigger. For read operations, DMA is triggered when the FIFO level is greater than this value. For write operations, DMA is triggered when the FIFO level is less than this level. Each DMA operation will consist of BCOUNT bytes. This value should not be set more than ALTRXFIFOFULL."]
pub type DmarxthreshR = crate::FieldReader;
#[doc = "Field `DMARXTHRESH` writer - DMA transfer FIFO level trigger. For read operations, DMA is triggered when the FIFO level is greater than this value. For write operations, DMA is triggered when the FIFO level is less than this level. Each DMA operation will consist of BCOUNT bytes. This value should not be set more than ALTRXFIFOFULL."]
pub type DmarxthreshW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - DMA transfer FIFO level trigger. For read operations, DMA is triggered when the FIFO level is greater than this value. For write operations, DMA is triggered when the FIFO level is less than this level. Each DMA operation will consist of BCOUNT bytes."]
    #[inline(always)]
    pub fn dmatxthresh(&self) -> DmatxthreshR {
        DmatxthreshR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - DMA transfer FIFO level trigger. For read operations, DMA is triggered when the FIFO level is greater than this value. For write operations, DMA is triggered when the FIFO level is less than this level. Each DMA operation will consist of BCOUNT bytes. This value should not be set more than ALTRXFIFOFULL."]
    #[inline(always)]
    pub fn dmarxthresh(&self) -> DmarxthreshR {
        DmarxthreshR::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - DMA transfer FIFO level trigger. For read operations, DMA is triggered when the FIFO level is greater than this value. For write operations, DMA is triggered when the FIFO level is less than this level. Each DMA operation will consist of BCOUNT bytes."]
    #[inline(always)]
    #[must_use]
    pub fn dmatxthresh(&mut self) -> DmatxthreshW<DmathreshSpec> {
        DmatxthreshW::new(self, 0)
    }
    #[doc = "Bits 8:12 - DMA transfer FIFO level trigger. For read operations, DMA is triggered when the FIFO level is greater than this value. For write operations, DMA is triggered when the FIFO level is less than this level. Each DMA operation will consist of BCOUNT bytes. This value should not be set more than ALTRXFIFOFULL."]
    #[inline(always)]
    #[must_use]
    pub fn dmarxthresh(&mut self) -> DmarxthreshW<DmathreshSpec> {
        DmarxthreshW::new(self, 8)
    }
}
#[doc = "Indicates FIFO level at which a DMA should be triggered. For most configurations, a setting of 8 is recommended for both read and write operations.\n\nYou can [`read`](crate::Reg::read) this register and get [`dmathresh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmathresh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmathreshSpec;
impl crate::RegisterSpec for DmathreshSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmathresh::R`](R) reader structure"]
impl crate::Readable for DmathreshSpec {}
#[doc = "`write(|w| ..)` method takes [`dmathresh::W`](W) writer structure"]
impl crate::Writable for DmathreshSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMATHRESH to value 0x0808"]
impl crate::Resettable for DmathreshSpec {
    const RESET_VALUE: u32 = 0x0808;
}
