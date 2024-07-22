#[doc = "Register `GREENY` reader"]
pub type R = crate::R<GreenySpec>;
#[doc = "Register `GREENY` writer"]
pub type W = crate::W<GreenySpec>;
#[doc = "Field `GREENY` reader - Added green value for each step at y-axis"]
pub type GreenyR = crate::FieldReader<u32>;
#[doc = "Field `GREENY` writer - Added green value for each step at y-axis"]
pub type GreenyW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Added green value for each step at y-axis"]
    #[inline(always)]
    pub fn greeny(&self) -> GreenyR {
        GreenyR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Added green value for each step at y-axis"]
    #[inline(always)]
    #[must_use]
    pub fn greeny(&mut self) -> GreenyW<GreenySpec> {
        GreenyW::new(self, 0)
    }
}
#[doc = "Added green value for each step at y-axis, (16, 16 fixed point)\n\nYou can [`read`](crate::Reg::read) this register and get [`greeny::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`greeny::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GreenySpec;
impl crate::RegisterSpec for GreenySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`greeny::R`](R) reader structure"]
impl crate::Readable for GreenySpec {}
#[doc = "`write(|w| ..)` method takes [`greeny::W`](W) writer structure"]
impl crate::Writable for GreenySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GREENY to value 0"]
impl crate::Resettable for GreenySpec {
    const RESET_VALUE: u32 = 0;
}
