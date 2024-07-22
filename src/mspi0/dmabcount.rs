#[doc = "Register `DMABCOUNT` reader"]
pub type R = crate::R<DmabcountSpec>;
#[doc = "Register `DMABCOUNT` writer"]
pub type W = crate::W<DmabcountSpec>;
#[doc = "Field `BCOUNT` reader - Burst transfer size in bytes. This is the number of bytes transferred when a FIFO trigger event occurs. Recommended value is 32."]
pub type BcountR = crate::FieldReader;
#[doc = "Field `BCOUNT` writer - Burst transfer size in bytes. This is the number of bytes transferred when a FIFO trigger event occurs. Recommended value is 32."]
pub type BcountW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Burst transfer size in bytes. This is the number of bytes transferred when a FIFO trigger event occurs. Recommended value is 32."]
    #[inline(always)]
    pub fn bcount(&self) -> BcountR {
        BcountR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Burst transfer size in bytes. This is the number of bytes transferred when a FIFO trigger event occurs. Recommended value is 32."]
    #[inline(always)]
    #[must_use]
    pub fn bcount(&mut self) -> BcountW<DmabcountSpec> {
        BcountW::new(self, 0)
    }
}
#[doc = "DMA BYTE Transfer Count\n\nYou can [`read`](crate::Reg::read) this register and get [`dmabcount::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmabcount::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmabcountSpec;
impl crate::RegisterSpec for DmabcountSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmabcount::R`](R) reader structure"]
impl crate::Readable for DmabcountSpec {}
#[doc = "`write(|w| ..)` method takes [`dmabcount::W`](W) writer structure"]
impl crate::Writable for DmabcountSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMABCOUNT to value 0x20"]
impl crate::Resettable for DmabcountSpec {
    const RESET_VALUE: u32 = 0x20;
}
