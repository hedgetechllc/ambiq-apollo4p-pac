#[doc = "Register `GREENX` reader"]
pub type R = crate::R<GreenxSpec>;
#[doc = "Register `GREENX` writer"]
pub type W = crate::W<GreenxSpec>;
#[doc = "Field `GREENX` reader - Added green value for each step at x-axis"]
pub type GreenxR = crate::FieldReader<u32>;
#[doc = "Field `GREENX` writer - Added green value for each step at x-axis"]
pub type GreenxW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Added green value for each step at x-axis"]
    #[inline(always)]
    pub fn greenx(&self) -> GreenxR {
        GreenxR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Added green value for each step at x-axis"]
    #[inline(always)]
    #[must_use]
    pub fn greenx(&mut self) -> GreenxW<GreenxSpec> {
        GreenxW::new(self, 0)
    }
}
#[doc = "Added green value for each step at x-axis, (16, 16 fixed point)\n\nYou can [`read`](crate::Reg::read) this register and get [`greenx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`greenx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GreenxSpec;
impl crate::RegisterSpec for GreenxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`greenx::R`](R) reader structure"]
impl crate::Readable for GreenxSpec {}
#[doc = "`write(|w| ..)` method takes [`greenx::W`](W) writer structure"]
impl crate::Writable for GreenxSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GREENX to value 0"]
impl crate::Resettable for GreenxSpec {
    const RESET_VALUE: u32 = 0;
}
