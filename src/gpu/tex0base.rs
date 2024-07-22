#[doc = "Register `TEX0BASE` reader"]
pub type R = crate::R<Tex0baseSpec>;
#[doc = "Register `TEX0BASE` writer"]
pub type W = crate::W<Tex0baseSpec>;
#[doc = "Field `Base` reader - Address 0: base address of the drawing surface 0 (must be word aligned)."]
pub type BaseR = crate::FieldReader<u32>;
#[doc = "Field `Base` writer - Address 0: base address of the drawing surface 0 (must be word aligned)."]
pub type BaseW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Address 0: base address of the drawing surface 0 (must be word aligned)."]
    #[inline(always)]
    pub fn base(&self) -> BaseR {
        BaseR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Address 0: base address of the drawing surface 0 (must be word aligned)."]
    #[inline(always)]
    #[must_use]
    pub fn base(&mut self) -> BaseW<Tex0baseSpec> {
        BaseW::new(self, 0)
    }
}
#[doc = "Base address of the drawing surface 0 (must be word aligned).\n\nYou can [`read`](crate::Reg::read) this register and get [`tex0base::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tex0base::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tex0baseSpec;
impl crate::RegisterSpec for Tex0baseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tex0base::R`](R) reader structure"]
impl crate::Readable for Tex0baseSpec {}
#[doc = "`write(|w| ..)` method takes [`tex0base::W`](W) writer structure"]
impl crate::Writable for Tex0baseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TEX0BASE to value 0"]
impl crate::Resettable for Tex0baseSpec {
    const RESET_VALUE: u32 = 0;
}
