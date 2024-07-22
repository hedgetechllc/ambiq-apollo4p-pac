#[doc = "Register `FIFOTHR` reader"]
pub type R = crate::R<FifothrSpec>;
#[doc = "Register `FIFOTHR` writer"]
pub type W = crate::W<FifothrSpec>;
#[doc = "Field `FIFOTHR` reader - FIFO size interrupt threshold."]
pub type FifothrR = crate::FieldReader;
#[doc = "Field `FIFOTHR` writer - FIFO size interrupt threshold."]
pub type FifothrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - FIFO size interrupt threshold."]
    #[inline(always)]
    pub fn fifothr(&self) -> FifothrR {
        FifothrR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - FIFO size interrupt threshold."]
    #[inline(always)]
    #[must_use]
    pub fn fifothr(&mut self) -> FifothrW<FifothrSpec> {
        FifothrW::new(self, 0)
    }
}
#[doc = "FIFO Threshold Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`fifothr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifothr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FifothrSpec;
impl crate::RegisterSpec for FifothrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifothr::R`](R) reader structure"]
impl crate::Readable for FifothrSpec {}
#[doc = "`write(|w| ..)` method takes [`fifothr::W`](W) writer structure"]
impl crate::Writable for FifothrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FIFOTHR to value 0"]
impl crate::Resettable for FifothrSpec {
    const RESET_VALUE: u32 = 0;
}
