#[doc = "Register `TEX1BASE` reader"]
pub type R = crate::R<Tex1baseSpec>;
#[doc = "Register `TEX1BASE` writer"]
pub type W = crate::W<Tex1baseSpec>;
#[doc = "Field `Base` reader - address 1: base address of the drawing surface 1 (must be word aligned)."]
pub type BaseR = crate::FieldReader<u32>;
#[doc = "Field `Base` writer - address 1: base address of the drawing surface 1 (must be word aligned)."]
pub type BaseW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - address 1: base address of the drawing surface 1 (must be word aligned)."]
    #[inline(always)]
    pub fn base(&self) -> BaseR {
        BaseR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - address 1: base address of the drawing surface 1 (must be word aligned)."]
    #[inline(always)]
    #[must_use]
    pub fn base(&mut self) -> BaseW<Tex1baseSpec> {
        BaseW::new(self, 0)
    }
}
#[doc = "Base address of the drawing surface 1 (must be word aligned).\n\nYou can [`read`](crate::Reg::read) this register and get [`tex1base::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tex1base::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tex1baseSpec;
impl crate::RegisterSpec for Tex1baseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tex1base::R`](R) reader structure"]
impl crate::Readable for Tex1baseSpec {}
#[doc = "`write(|w| ..)` method takes [`tex1base::W`](W) writer structure"]
impl crate::Writable for Tex1baseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TEX1BASE to value 0"]
impl crate::Resettable for Tex1baseSpec {
    const RESET_VALUE: u32 = 0;
}
