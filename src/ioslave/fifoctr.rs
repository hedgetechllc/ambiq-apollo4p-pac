#[doc = "Register `FIFOCTR` reader"]
pub type R = crate::R<FifoctrSpec>;
#[doc = "Register `FIFOCTR` writer"]
pub type W = crate::W<FifoctrSpec>;
#[doc = "Field `FIFOCTR` reader - Virtual FIFO byte count"]
pub type FifoctrR = crate::FieldReader<u16>;
#[doc = "Field `FIFOCTR` writer - Virtual FIFO byte count"]
pub type FifoctrW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Virtual FIFO byte count"]
    #[inline(always)]
    pub fn fifoctr(&self) -> FifoctrR {
        FifoctrR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Virtual FIFO byte count"]
    #[inline(always)]
    #[must_use]
    pub fn fifoctr(&mut self) -> FifoctrW<FifoctrSpec> {
        FifoctrW::new(self, 0)
    }
}
#[doc = "Overall FIFO Counter\n\nYou can [`read`](crate::Reg::read) this register and get [`fifoctr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifoctr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FifoctrSpec;
impl crate::RegisterSpec for FifoctrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifoctr::R`](R) reader structure"]
impl crate::Readable for FifoctrSpec {}
#[doc = "`write(|w| ..)` method takes [`fifoctr::W`](W) writer structure"]
impl crate::Writable for FifoctrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FIFOCTR to value 0"]
impl crate::Resettable for FifoctrSpec {
    const RESET_VALUE: u32 = 0;
}
