#[doc = "Register `TEX3BASE` reader"]
pub type R = crate::R<Tex3baseSpec>;
#[doc = "Register `TEX3BASE` writer"]
pub type W = crate::W<Tex3baseSpec>;
#[doc = "Field `Image` reader - 3 Base address of the drawing surface"]
pub type ImageR = crate::FieldReader<u32>;
#[doc = "Field `Image` writer - 3 Base address of the drawing surface"]
pub type ImageW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 3 Base address of the drawing surface"]
    #[inline(always)]
    pub fn image(&self) -> ImageR {
        ImageR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 3 Base address of the drawing surface"]
    #[inline(always)]
    #[must_use]
    pub fn image(&mut self) -> ImageW<Tex3baseSpec> {
        ImageW::new(self, 0)
    }
}
#[doc = "Base address of the drawing surface 3 (must be word aligned).\n\nYou can [`read`](crate::Reg::read) this register and get [`tex3base::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tex3base::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tex3baseSpec;
impl crate::RegisterSpec for Tex3baseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tex3base::R`](R) reader structure"]
impl crate::Readable for Tex3baseSpec {}
#[doc = "`write(|w| ..)` method takes [`tex3base::W`](W) writer structure"]
impl crate::Writable for Tex3baseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TEX3BASE to value 0"]
impl crate::Resettable for Tex3baseSpec {
    const RESET_VALUE: u32 = 0;
}
