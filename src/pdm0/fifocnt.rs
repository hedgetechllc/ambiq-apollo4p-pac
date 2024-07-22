#[doc = "Register `FIFOCNT` reader"]
pub type R = crate::R<FifocntSpec>;
#[doc = "Register `FIFOCNT` writer"]
pub type W = crate::W<FifocntSpec>;
#[doc = "Field `FIFOCNT` reader - Valid 32-bit entries currently in the FIFO."]
pub type FifocntR = crate::FieldReader;
#[doc = "Field `FIFOCNT` writer - Valid 32-bit entries currently in the FIFO."]
pub type FifocntW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Valid 32-bit entries currently in the FIFO."]
    #[inline(always)]
    pub fn fifocnt(&self) -> FifocntR {
        FifocntR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Valid 32-bit entries currently in the FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn fifocnt(&mut self) -> FifocntW<FifocntSpec> {
        FifocntW::new(self, 0)
    }
}
#[doc = "FIFO count\n\nYou can [`read`](crate::Reg::read) this register and get [`fifocnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifocnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FifocntSpec;
impl crate::RegisterSpec for FifocntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifocnt::R`](R) reader structure"]
impl crate::Readable for FifocntSpec {}
#[doc = "`write(|w| ..)` method takes [`fifocnt::W`](W) writer structure"]
impl crate::Writable for FifocntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FIFOCNT to value 0"]
impl crate::Resettable for FifocntSpec {
    const RESET_VALUE: u32 = 0;
}
