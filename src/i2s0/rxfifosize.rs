#[doc = "Register `RXFIFOSIZE` reader"]
pub type R = crate::R<RxfifosizeSpec>;
#[doc = "Register `RXFIFOSIZE` writer"]
pub type W = crate::W<RxfifosizeSpec>;
#[doc = "Field `SIZE` reader - Size of the receive FIFO in units of i2S samples. Read only value."]
pub type SizeR = crate::FieldReader<u32>;
#[doc = "Field `SIZE` writer - Size of the receive FIFO in units of i2S samples. Read only value."]
pub type SizeW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Size of the receive FIFO in units of i2S samples. Read only value."]
    #[inline(always)]
    pub fn size(&self) -> SizeR {
        SizeR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Size of the receive FIFO in units of i2S samples. Read only value."]
    #[inline(always)]
    #[must_use]
    pub fn size(&mut self) -> SizeW<RxfifosizeSpec> {
        SizeW::new(self, 0)
    }
}
#[doc = "Holds the size of the receive FIFO in samples\n\nYou can [`read`](crate::Reg::read) this register and get [`rxfifosize::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxfifosize::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxfifosizeSpec;
impl crate::RegisterSpec for RxfifosizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxfifosize::R`](R) reader structure"]
impl crate::Readable for RxfifosizeSpec {}
#[doc = "`write(|w| ..)` method takes [`rxfifosize::W`](W) writer structure"]
impl crate::Writable for RxfifosizeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RXFIFOSIZE to value 0"]
impl crate::Resettable for RxfifosizeSpec {
    const RESET_VALUE: u32 = 0;
}
