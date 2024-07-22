#[doc = "Register `BLUEX` reader"]
pub type R = crate::R<BluexSpec>;
#[doc = "Register `BLUEX` writer"]
pub type W = crate::W<BluexSpec>;
#[doc = "Field `BLUEX` reader - Added blue value for each step at x-axis"]
pub type BluexR = crate::FieldReader<u32>;
#[doc = "Field `BLUEX` writer - Added blue value for each step at x-axis"]
pub type BluexW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Added blue value for each step at x-axis"]
    #[inline(always)]
    pub fn bluex(&self) -> BluexR {
        BluexR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Added blue value for each step at x-axis"]
    #[inline(always)]
    #[must_use]
    pub fn bluex(&mut self) -> BluexW<BluexSpec> {
        BluexW::new(self, 0)
    }
}
#[doc = "Added blue value for each step at x-axis, (16, 16 fixed point)\n\nYou can [`read`](crate::Reg::read) this register and get [`bluex::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bluex::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BluexSpec;
impl crate::RegisterSpec for BluexSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bluex::R`](R) reader structure"]
impl crate::Readable for BluexSpec {}
#[doc = "`write(|w| ..)` method takes [`bluex::W`](W) writer structure"]
impl crate::Writable for BluexSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BLUEX to value 0"]
impl crate::Resettable for BluexSpec {
    const RESET_VALUE: u32 = 0;
}
