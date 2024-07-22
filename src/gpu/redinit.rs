#[doc = "Register `REDINIT` reader"]
pub type R = crate::R<RedinitSpec>;
#[doc = "Register `REDINIT` writer"]
pub type W = crate::W<RedinitSpec>;
#[doc = "Field `REDXY` reader - Red value of STARTXY pixel"]
pub type RedxyR = crate::FieldReader<u32>;
#[doc = "Field `REDXY` writer - Red value of STARTXY pixel"]
pub type RedxyW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Red value of STARTXY pixel"]
    #[inline(always)]
    pub fn redxy(&self) -> RedxyR {
        RedxyR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Red value of STARTXY pixel"]
    #[inline(always)]
    #[must_use]
    pub fn redxy(&mut self) -> RedxyW<RedinitSpec> {
        RedxyW::new(self, 0)
    }
}
#[doc = "Red value of STARTXY pixel, (16, 16 fixed point)\n\nYou can [`read`](crate::Reg::read) this register and get [`redinit::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`redinit::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RedinitSpec;
impl crate::RegisterSpec for RedinitSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`redinit::R`](R) reader structure"]
impl crate::Readable for RedinitSpec {}
#[doc = "`write(|w| ..)` method takes [`redinit::W`](W) writer structure"]
impl crate::Writable for RedinitSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REDINIT to value 0"]
impl crate::Resettable for RedinitSpec {
    const RESET_VALUE: u32 = 0;
}
