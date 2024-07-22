#[doc = "Register `LAYER2SIZEXY` reader"]
pub type R = crate::R<Layer2sizexySpec>;
#[doc = "Register `LAYER2SIZEXY` writer"]
pub type W = crate::W<Layer2sizexySpec>;
#[doc = "Field `LAYER2PIXSZEY` reader - Specify the pixel size of the layer 2 in the Y dimension"]
pub type Layer2pixszeyR = crate::FieldReader<u16>;
#[doc = "Field `LAYER2PIXSZEY` writer - Specify the pixel size of the layer 2 in the Y dimension"]
pub type Layer2pixszeyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `LAYER2PIXSZEX` reader - Specify the pixel size of the layer 2 in the X dimension"]
pub type Layer2pixszexR = crate::FieldReader<u16>;
#[doc = "Field `LAYER2PIXSZEX` writer - Specify the pixel size of the layer 2 in the X dimension"]
pub type Layer2pixszexW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Specify the pixel size of the layer 2 in the Y dimension"]
    #[inline(always)]
    pub fn layer2pixszey(&self) -> Layer2pixszeyR {
        Layer2pixszeyR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Specify the pixel size of the layer 2 in the X dimension"]
    #[inline(always)]
    pub fn layer2pixszex(&self) -> Layer2pixszexR {
        Layer2pixszexR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Specify the pixel size of the layer 2 in the Y dimension"]
    #[inline(always)]
    #[must_use]
    pub fn layer2pixszey(&mut self) -> Layer2pixszeyW<Layer2sizexySpec> {
        Layer2pixszeyW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Specify the pixel size of the layer 2 in the X dimension"]
    #[inline(always)]
    #[must_use]
    pub fn layer2pixszex(&mut self) -> Layer2pixszexW<Layer2sizexySpec> {
        Layer2pixszexW::new(self, 16)
    }
}
#[doc = "X and Y size of layer 2.\n\nYou can [`read`](crate::Reg::read) this register and get [`layer2sizexy::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`layer2sizexy::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Layer2sizexySpec;
impl crate::RegisterSpec for Layer2sizexySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`layer2sizexy::R`](R) reader structure"]
impl crate::Readable for Layer2sizexySpec {}
#[doc = "`write(|w| ..)` method takes [`layer2sizexy::W`](W) writer structure"]
impl crate::Writable for Layer2sizexySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LAYER2SIZEXY to value 0"]
impl crate::Resettable for Layer2sizexySpec {
    const RESET_VALUE: u32 = 0;
}
