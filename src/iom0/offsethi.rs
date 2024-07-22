#[doc = "Register `OFFSETHI` reader"]
pub type R = crate::R<OffsethiSpec>;
#[doc = "Register `OFFSETHI` writer"]
pub type W = crate::W<OffsethiSpec>;
#[doc = "Field `OFFSETHI` reader - Holds the high order bytes of the byte addressing/offset field to use with IO commands. The number of offset bytes to use is specified in the command register"]
pub type OffsethiR = crate::FieldReader<u32>;
#[doc = "Field `OFFSETHI` writer - Holds the high order bytes of the byte addressing/offset field to use with IO commands. The number of offset bytes to use is specified in the command register"]
pub type OffsethiW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Holds the high order bytes of the byte addressing/offset field to use with IO commands. The number of offset bytes to use is specified in the command register"]
    #[inline(always)]
    pub fn offsethi(&self) -> OffsethiR {
        OffsethiR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Holds the high order bytes of the byte addressing/offset field to use with IO commands. The number of offset bytes to use is specified in the command register"]
    #[inline(always)]
    #[must_use]
    pub fn offsethi(&mut self) -> OffsethiW<OffsethiSpec> {
        OffsethiW::new(self, 0)
    }
}
#[doc = "High order bytes of offset for IO transaction\n\nYou can [`read`](crate::Reg::read) this register and get [`offsethi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`offsethi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OffsethiSpec;
impl crate::RegisterSpec for OffsethiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`offsethi::R`](R) reader structure"]
impl crate::Readable for OffsethiSpec {}
#[doc = "`write(|w| ..)` method takes [`offsethi::W`](W) writer structure"]
impl crate::Writable for OffsethiSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OFFSETHI to value 0"]
impl crate::Resettable for OffsethiSpec {
    const RESET_VALUE: u32 = 0;
}
