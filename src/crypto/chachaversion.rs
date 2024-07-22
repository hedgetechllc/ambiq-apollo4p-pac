#[doc = "Register `CHACHAVERSION` reader"]
pub type R = crate::R<ChachaversionSpec>;
#[doc = "Register `CHACHAVERSION` writer"]
pub type W = crate::W<ChachaversionSpec>;
#[doc = "Field `CHACHAVERSION` reader - CHACHA version."]
pub type ChachaversionR = crate::FieldReader<u32>;
#[doc = "Field `CHACHAVERSION` writer - CHACHA version."]
pub type ChachaversionW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CHACHA version."]
    #[inline(always)]
    pub fn chachaversion(&self) -> ChachaversionR {
        ChachaversionR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CHACHA version."]
    #[inline(always)]
    #[must_use]
    pub fn chachaversion(&mut self) -> ChachaversionW<ChachaversionSpec> {
        ChachaversionW::new(self, 0)
    }
}
#[doc = "CHACHA Version\n\nYou can [`read`](crate::Reg::read) this register and get [`chachaversion::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chachaversion::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChachaversionSpec;
impl crate::RegisterSpec for ChachaversionSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chachaversion::R`](R) reader structure"]
impl crate::Readable for ChachaversionSpec {}
#[doc = "`write(|w| ..)` method takes [`chachaversion::W`](W) writer structure"]
impl crate::Writable for ChachaversionSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHACHAVERSION to value 0x01"]
impl crate::Resettable for ChachaversionSpec {
    const RESET_VALUE: u32 = 0x01;
}
