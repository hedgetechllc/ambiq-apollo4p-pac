#[doc = "Register `C3REG` reader"]
pub type R = crate::R<C3regSpec>;
#[doc = "Register `C3REG` writer"]
pub type W = crate::W<C3regSpec>;
#[doc = "Field `C3SHADER` reader - Shader constant register 3"]
pub type C3shaderR = crate::FieldReader<u32>;
#[doc = "Field `C3SHADER` writer - Shader constant register 3"]
pub type C3shaderW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Shader constant register 3"]
    #[inline(always)]
    pub fn c3shader(&self) -> C3shaderR {
        C3shaderR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Shader constant register 3"]
    #[inline(always)]
    #[must_use]
    pub fn c3shader(&mut self) -> C3shaderW<C3regSpec> {
        C3shaderW::new(self, 0)
    }
}
#[doc = "Shader constant register 3, the dirty Region Register\n\nYou can [`read`](crate::Reg::read) this register and get [`c3reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c3reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C3regSpec;
impl crate::RegisterSpec for C3regSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c3reg::R`](R) reader structure"]
impl crate::Readable for C3regSpec {}
#[doc = "`write(|w| ..)` method takes [`c3reg::W`](W) writer structure"]
impl crate::Writable for C3regSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets C3REG to value 0"]
impl crate::Resettable for C3regSpec {
    const RESET_VALUE: u32 = 0;
}
