#[doc = "Register `LAYER3SIZEXY` reader"]
pub type R = crate::R<Layer3sizexySpec>;
#[doc = "Register `LAYER3SIZEXY` writer"]
pub type W = crate::W<Layer3sizexySpec>;
#[doc = "Field `LAYER3PIXSZEY` reader - Specify the pixel size of the layer 3 in the Y dimension"]
pub type Layer3pixszeyR = crate::FieldReader<u16>;
#[doc = "Field `LAYER3PIXSZEY` writer - Specify the pixel size of the layer 3 in the Y dimension"]
pub type Layer3pixszeyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `LAYER3PIXSZEX` reader - Specify the pixel size of the layer 3 in the X dimension"]
pub type Layer3pixszexR = crate::FieldReader<u16>;
#[doc = "Field `LAYER3PIXSZEX` writer - Specify the pixel size of the layer 3 in the X dimension"]
pub type Layer3pixszexW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Specify the pixel size of the layer 3 in the Y dimension"]
    #[inline(always)]
    pub fn layer3pixszey(&self) -> Layer3pixszeyR {
        Layer3pixszeyR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Specify the pixel size of the layer 3 in the X dimension"]
    #[inline(always)]
    pub fn layer3pixszex(&self) -> Layer3pixszexR {
        Layer3pixszexR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Specify the pixel size of the layer 3 in the Y dimension"]
    #[inline(always)]
    #[must_use]
    pub fn layer3pixszey(&mut self) -> Layer3pixszeyW<Layer3sizexySpec> {
        Layer3pixszeyW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Specify the pixel size of the layer 3 in the X dimension"]
    #[inline(always)]
    #[must_use]
    pub fn layer3pixszex(&mut self) -> Layer3pixszexW<Layer3sizexySpec> {
        Layer3pixszexW::new(self, 16)
    }
}
#[doc = "X and Y size of layer 3.\n\nYou can [`read`](crate::Reg::read) this register and get [`layer3sizexy::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`layer3sizexy::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Layer3sizexySpec;
impl crate::RegisterSpec for Layer3sizexySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`layer3sizexy::R`](R) reader structure"]
impl crate::Readable for Layer3sizexySpec {}
#[doc = "`write(|w| ..)` method takes [`layer3sizexy::W`](W) writer structure"]
impl crate::Writable for Layer3sizexySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LAYER3SIZEXY to value 0"]
impl crate::Resettable for Layer3sizexySpec {
    const RESET_VALUE: u32 = 0;
}
