#[doc = "Register `PKAPIPERDY` reader"]
pub type R = crate::R<PkapiperdySpec>;
#[doc = "Register `PKAPIPERDY` writer"]
pub type W = crate::W<PkapiperdySpec>;
#[doc = "Field `PKAPIPERDY` reader - Indication whether PKA pipe is ready for new OPCODE."]
pub type PkapiperdyR = crate::BitReader;
#[doc = "Field `PKAPIPERDY` writer - Indication whether PKA pipe is ready for new OPCODE."]
pub type PkapiperdyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Indication whether PKA pipe is ready for new OPCODE."]
    #[inline(always)]
    pub fn pkapiperdy(&self) -> PkapiperdyR {
        PkapiperdyR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Indication whether PKA pipe is ready for new OPCODE."]
    #[inline(always)]
    #[must_use]
    pub fn pkapiperdy(&mut self) -> PkapiperdyW<PkapiperdySpec> {
        PkapiperdyW::new(self, 0)
    }
}
#[doc = "This register indicates whether the PKA pipe is ready to receive a new OPCODE.\n\nYou can [`read`](crate::Reg::read) this register and get [`pkapiperdy::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pkapiperdy::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PkapiperdySpec;
impl crate::RegisterSpec for PkapiperdySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pkapiperdy::R`](R) reader structure"]
impl crate::Readable for PkapiperdySpec {}
#[doc = "`write(|w| ..)` method takes [`pkapiperdy::W`](W) writer structure"]
impl crate::Writable for PkapiperdySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PKAPIPERDY to value 0x01"]
impl crate::Resettable for PkapiperdySpec {
    const RESET_VALUE: u32 = 0x01;
}
