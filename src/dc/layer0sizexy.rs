#[doc = "Register `LAYER0SIZEXY` reader"]
pub type R = crate::R<Layer0sizexySpec>;
#[doc = "Register `LAYER0SIZEXY` writer"]
pub type W = crate::W<Layer0sizexySpec>;
#[doc = "Field `LAYER0PIXSZEY` reader - Specify the pixel size of the layer 0 in the Y dimension"]
pub type Layer0pixszeyR = crate::FieldReader<u16>;
#[doc = "Field `LAYER0PIXSZEY` writer - Specify the pixel size of the layer 0 in the Y dimension"]
pub type Layer0pixszeyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `LAYER0PIXSZEX` reader - Specify the pixel size of the layer 0 in the X dimension"]
pub type Layer0pixszexR = crate::FieldReader<u16>;
#[doc = "Field `LAYER0PIXSZEX` writer - Specify the pixel size of the layer 0 in the X dimension"]
pub type Layer0pixszexW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Specify the pixel size of the layer 0 in the Y dimension"]
    #[inline(always)]
    pub fn layer0pixszey(&self) -> Layer0pixszeyR {
        Layer0pixszeyR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Specify the pixel size of the layer 0 in the X dimension"]
    #[inline(always)]
    pub fn layer0pixszex(&self) -> Layer0pixszexR {
        Layer0pixszexR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Specify the pixel size of the layer 0 in the Y dimension"]
    #[inline(always)]
    #[must_use]
    pub fn layer0pixszey(&mut self) -> Layer0pixszeyW<Layer0sizexySpec> {
        Layer0pixszeyW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Specify the pixel size of the layer 0 in the X dimension"]
    #[inline(always)]
    #[must_use]
    pub fn layer0pixszex(&mut self) -> Layer0pixszexW<Layer0sizexySpec> {
        Layer0pixszexW::new(self, 16)
    }
}
#[doc = "X and Y size of layer 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`layer0sizexy::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`layer0sizexy::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Layer0sizexySpec;
impl crate::RegisterSpec for Layer0sizexySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`layer0sizexy::R`](R) reader structure"]
impl crate::Readable for Layer0sizexySpec {}
#[doc = "`write(|w| ..)` method takes [`layer0sizexy::W`](W) writer structure"]
impl crate::Writable for Layer0sizexySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LAYER0SIZEXY to value 0"]
impl crate::Resettable for Layer0sizexySpec {
    const RESET_VALUE: u32 = 0;
}
