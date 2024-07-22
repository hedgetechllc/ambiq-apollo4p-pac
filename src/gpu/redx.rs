#[doc = "Register `REDX` reader"]
pub type R = crate::R<RedxSpec>;
#[doc = "Register `REDX` writer"]
pub type W = crate::W<RedxSpec>;
#[doc = "Field `REDX` reader - Added red value for each step at x-axis"]
pub type RedxR = crate::FieldReader<u32>;
#[doc = "Field `REDX` writer - Added red value for each step at x-axis"]
pub type RedxW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Added red value for each step at x-axis"]
    #[inline(always)]
    pub fn redx(&self) -> RedxR {
        RedxR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Added red value for each step at x-axis"]
    #[inline(always)]
    #[must_use]
    pub fn redx(&mut self) -> RedxW<RedxSpec> {
        RedxW::new(self, 0)
    }
}
#[doc = "Added red value for each step at x-axis, (16, 16 fixed point)\n\nYou can [`read`](crate::Reg::read) this register and get [`redx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`redx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RedxSpec;
impl crate::RegisterSpec for RedxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`redx::R`](R) reader structure"]
impl crate::Readable for RedxSpec {}
#[doc = "`write(|w| ..)` method takes [`redx::W`](W) writer structure"]
impl crate::Writable for RedxSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REDX to value 0"]
impl crate::Resettable for RedxSpec {
    const RESET_VALUE: u32 = 0;
}
