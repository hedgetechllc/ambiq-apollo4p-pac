#[doc = "Register `BLUEY` reader"]
pub type R = crate::R<BlueySpec>;
#[doc = "Register `BLUEY` writer"]
pub type W = crate::W<BlueySpec>;
#[doc = "Field `BLUEY` reader - Added blue value for each step at y-axis"]
pub type BlueyR = crate::FieldReader<u32>;
#[doc = "Field `BLUEY` writer - Added blue value for each step at y-axis"]
pub type BlueyW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Added blue value for each step at y-axis"]
    #[inline(always)]
    pub fn bluey(&self) -> BlueyR {
        BlueyR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Added blue value for each step at y-axis"]
    #[inline(always)]
    #[must_use]
    pub fn bluey(&mut self) -> BlueyW<BlueySpec> {
        BlueyW::new(self, 0)
    }
}
#[doc = "Added blue value for each step at y-axis, (16, 16 fixed point)\n\nYou can [`read`](crate::Reg::read) this register and get [`bluey::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bluey::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlueySpec;
impl crate::RegisterSpec for BlueySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bluey::R`](R) reader structure"]
impl crate::Readable for BlueySpec {}
#[doc = "`write(|w| ..)` method takes [`bluey::W`](W) writer structure"]
impl crate::Writable for BlueySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BLUEY to value 0"]
impl crate::Resettable for BlueySpec {
    const RESET_VALUE: u32 = 0;
}
