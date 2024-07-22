#[doc = "Register `C1REG` reader"]
pub type R = crate::R<C1regSpec>;
#[doc = "Register `C1REG` writer"]
pub type W = crate::W<C1regSpec>;
#[doc = "Field `C1SHADER` reader - Shader constant register 1."]
pub type C1shaderR = crate::FieldReader<u32>;
#[doc = "Field `C1SHADER` writer - Shader constant register 1."]
pub type C1shaderW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Shader constant register 1."]
    #[inline(always)]
    pub fn c1shader(&self) -> C1shaderR {
        C1shaderR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Shader constant register 1."]
    #[inline(always)]
    #[must_use]
    pub fn c1shader(&mut self) -> C1shaderW<C1regSpec> {
        C1shaderW::new(self, 0)
    }
}
#[doc = "Shader constant register 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`c1reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C1regSpec;
impl crate::RegisterSpec for C1regSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c1reg::R`](R) reader structure"]
impl crate::Readable for C1regSpec {}
#[doc = "`write(|w| ..)` method takes [`c1reg::W`](W) writer structure"]
impl crate::Writable for C1regSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets C1REG to value 0"]
impl crate::Resettable for C1regSpec {
    const RESET_VALUE: u32 = 0;
}
