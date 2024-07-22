#[doc = "Register `ALFX` reader"]
pub type R = crate::R<AlfxSpec>;
#[doc = "Register `ALFX` writer"]
pub type W = crate::W<AlfxSpec>;
#[doc = "Field `ALFX` reader - Added alfa value for each step at x-axis"]
pub type AlfxR = crate::FieldReader<u32>;
#[doc = "Field `ALFX` writer - Added alfa value for each step at x-axis"]
pub type AlfxW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Added alfa value for each step at x-axis"]
    #[inline(always)]
    pub fn alfx(&self) -> AlfxR {
        AlfxR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Added alfa value for each step at x-axis"]
    #[inline(always)]
    #[must_use]
    pub fn alfx(&mut self) -> AlfxW<AlfxSpec> {
        AlfxW::new(self, 0)
    }
}
#[doc = "Added alfa value for each step at x-axis, (16, 16 fixed point)\n\nYou can [`read`](crate::Reg::read) this register and get [`alfx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alfx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AlfxSpec;
impl crate::RegisterSpec for AlfxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`alfx::R`](R) reader structure"]
impl crate::Readable for AlfxSpec {}
#[doc = "`write(|w| ..)` method takes [`alfx::W`](W) writer structure"]
impl crate::Writable for AlfxSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ALFX to value 0"]
impl crate::Resettable for AlfxSpec {
    const RESET_VALUE: u32 = 0;
}
