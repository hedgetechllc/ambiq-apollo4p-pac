#[doc = "Register `REDY` reader"]
pub type R = crate::R<RedySpec>;
#[doc = "Register `REDY` writer"]
pub type W = crate::W<RedySpec>;
#[doc = "Field `REDY` reader - red value for each step at y-axis"]
pub type RedyR = crate::FieldReader<u32>;
#[doc = "Field `REDY` writer - red value for each step at y-axis"]
pub type RedyW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - red value for each step at y-axis"]
    #[inline(always)]
    pub fn redy(&self) -> RedyR {
        RedyR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - red value for each step at y-axis"]
    #[inline(always)]
    #[must_use]
    pub fn redy(&mut self) -> RedyW<RedySpec> {
        RedyW::new(self, 0)
    }
}
#[doc = "Added red value for each step at y-axis, (16, 16 fixed point)\n\nYou can [`read`](crate::Reg::read) this register and get [`redy::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`redy::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RedySpec;
impl crate::RegisterSpec for RedySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`redy::R`](R) reader structure"]
impl crate::Readable for RedySpec {}
#[doc = "`write(|w| ..)` method takes [`redy::W`](W) writer structure"]
impl crate::Writable for RedySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REDY to value 0"]
impl crate::Resettable for RedySpec {
    const RESET_VALUE: u32 = 0;
}
