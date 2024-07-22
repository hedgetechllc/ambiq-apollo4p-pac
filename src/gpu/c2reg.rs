#[doc = "Register `C2REG` reader"]
pub type R = crate::R<C2regSpec>;
#[doc = "Register `C2REG` writer"]
pub type W = crate::W<C2regSpec>;
#[doc = "Field `C2SHADER` reader - Shader constant register 2"]
pub type C2shaderR = crate::FieldReader<u32>;
#[doc = "Field `C2SHADER` writer - Shader constant register 2"]
pub type C2shaderW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Shader constant register 2"]
    #[inline(always)]
    pub fn c2shader(&self) -> C2shaderR {
        C2shaderR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Shader constant register 2"]
    #[inline(always)]
    #[must_use]
    pub fn c2shader(&mut self) -> C2shaderW<C2regSpec> {
        C2shaderW::new(self, 0)
    }
}
#[doc = "Shader constant register 2.\n\nYou can [`read`](crate::Reg::read) this register and get [`c2reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C2regSpec;
impl crate::RegisterSpec for C2regSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c2reg::R`](R) reader structure"]
impl crate::Readable for C2regSpec {}
#[doc = "`write(|w| ..)` method takes [`c2reg::W`](W) writer structure"]
impl crate::Writable for C2regSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets C2REG to value 0"]
impl crate::Resettable for C2regSpec {
    const RESET_VALUE: u32 = 0;
}
