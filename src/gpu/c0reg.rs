#[doc = "Register `C0REG` reader"]
pub type R = crate::R<C0regSpec>;
#[doc = "Register `C0REG` writer"]
pub type W = crate::W<C0regSpec>;
#[doc = "Field `C0SHADER` reader - Shader constant register 0."]
pub type C0shaderR = crate::FieldReader<u32>;
#[doc = "Field `C0SHADER` writer - Shader constant register 0."]
pub type C0shaderW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Shader constant register 0."]
    #[inline(always)]
    pub fn c0shader(&self) -> C0shaderR {
        C0shaderR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Shader constant register 0."]
    #[inline(always)]
    #[must_use]
    pub fn c0shader(&mut self) -> C0shaderW<C0regSpec> {
        C0shaderW::new(self, 0)
    }
}
#[doc = "Shader constant register 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`c0reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c0reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C0regSpec;
impl crate::RegisterSpec for C0regSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c0reg::R`](R) reader structure"]
impl crate::Readable for C0regSpec {}
#[doc = "`write(|w| ..)` method takes [`c0reg::W`](W) writer structure"]
impl crate::Writable for C0regSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets C0REG to value 0"]
impl crate::Resettable for C0regSpec {
    const RESET_VALUE: u32 = 0;
}
