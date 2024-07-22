#[doc = "Register `LAYER0STARTXY` reader"]
pub type R = crate::R<Layer0startxySpec>;
#[doc = "Register `LAYER0STARTXY` writer"]
pub type W = crate::W<Layer0startxySpec>;
#[doc = "Field `LAYER0YOFF` reader - Specify the pixel offset of the starting Y dimension of layer 0"]
pub type Layer0yoffR = crate::FieldReader<u16>;
#[doc = "Field `LAYER0YOFF` writer - Specify the pixel offset of the starting Y dimension of layer 0"]
pub type Layer0yoffW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `LAYER0XOFF` reader - Specify the pixel offset of the starting X dimension of layer 0"]
pub type Layer0xoffR = crate::FieldReader<u16>;
#[doc = "Field `LAYER0XOFF` writer - Specify the pixel offset of the starting X dimension of layer 0"]
pub type Layer0xoffW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Specify the pixel offset of the starting Y dimension of layer 0"]
    #[inline(always)]
    pub fn layer0yoff(&self) -> Layer0yoffR {
        Layer0yoffR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Specify the pixel offset of the starting X dimension of layer 0"]
    #[inline(always)]
    pub fn layer0xoff(&self) -> Layer0xoffR {
        Layer0xoffR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Specify the pixel offset of the starting Y dimension of layer 0"]
    #[inline(always)]
    #[must_use]
    pub fn layer0yoff(&mut self) -> Layer0yoffW<Layer0startxySpec> {
        Layer0yoffW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Specify the pixel offset of the starting X dimension of layer 0"]
    #[inline(always)]
    #[must_use]
    pub fn layer0xoff(&mut self) -> Layer0xoffW<Layer0startxySpec> {
        Layer0xoffW::new(self, 16)
    }
}
#[doc = "X and Y start dimensions of layer 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`layer0startxy::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`layer0startxy::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Layer0startxySpec;
impl crate::RegisterSpec for Layer0startxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`layer0startxy::R`](R) reader structure"]
impl crate::Readable for Layer0startxySpec {}
#[doc = "`write(|w| ..)` method takes [`layer0startxy::W`](W) writer structure"]
impl crate::Writable for Layer0startxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LAYER0STARTXY to value 0"]
impl crate::Resettable for Layer0startxySpec {
    const RESET_VALUE: u32 = 0;
}
