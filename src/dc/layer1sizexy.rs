#[doc = "Register `LAYER1SIZEXY` reader"]
pub type R = crate::R<Layer1sizexySpec>;
#[doc = "Register `LAYER1SIZEXY` writer"]
pub type W = crate::W<Layer1sizexySpec>;
#[doc = "Field `LAYER1PIXSZEY` reader - Specify the pixel size of the layer 1 in the Y dimension"]
pub type Layer1pixszeyR = crate::FieldReader<u16>;
#[doc = "Field `LAYER1PIXSZEY` writer - Specify the pixel size of the layer 1 in the Y dimension"]
pub type Layer1pixszeyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `LAYER1PIXSZEX` reader - Specify the pixel size of the layer 1 in the X dimension"]
pub type Layer1pixszexR = crate::FieldReader<u16>;
#[doc = "Field `LAYER1PIXSZEX` writer - Specify the pixel size of the layer 1 in the X dimension"]
pub type Layer1pixszexW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Specify the pixel size of the layer 1 in the Y dimension"]
    #[inline(always)]
    pub fn layer1pixszey(&self) -> Layer1pixszeyR {
        Layer1pixszeyR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Specify the pixel size of the layer 1 in the X dimension"]
    #[inline(always)]
    pub fn layer1pixszex(&self) -> Layer1pixszexR {
        Layer1pixszexR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Specify the pixel size of the layer 1 in the Y dimension"]
    #[inline(always)]
    #[must_use]
    pub fn layer1pixszey(&mut self) -> Layer1pixszeyW<Layer1sizexySpec> {
        Layer1pixszeyW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Specify the pixel size of the layer 1 in the X dimension"]
    #[inline(always)]
    #[must_use]
    pub fn layer1pixszex(&mut self) -> Layer1pixszexW<Layer1sizexySpec> {
        Layer1pixszexW::new(self, 16)
    }
}
#[doc = "X and Y size of layer 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`layer1sizexy::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`layer1sizexy::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Layer1sizexySpec;
impl crate::RegisterSpec for Layer1sizexySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`layer1sizexy::R`](R) reader structure"]
impl crate::Readable for Layer1sizexySpec {}
#[doc = "`write(|w| ..)` method takes [`layer1sizexy::W`](W) writer structure"]
impl crate::Writable for Layer1sizexySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LAYER1SIZEXY to value 0"]
impl crate::Resettable for Layer1sizexySpec {
    const RESET_VALUE: u32 = 0;
}
