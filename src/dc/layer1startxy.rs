#[doc = "Register `LAYER1STARTXY` reader"]
pub type R = crate::R<Layer1startxySpec>;
#[doc = "Register `LAYER1STARTXY` writer"]
pub type W = crate::W<Layer1startxySpec>;
#[doc = "Field `LAYER1YOFF` reader - Specify the pixel offset of the starting Y dimension of layer 1"]
pub type Layer1yoffR = crate::FieldReader<u16>;
#[doc = "Field `LAYER1YOFF` writer - Specify the pixel offset of the starting Y dimension of layer 1"]
pub type Layer1yoffW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `LAYER1XOFF` reader - Specify the pixel offset of the starting X dimension of layer 1"]
pub type Layer1xoffR = crate::FieldReader<u16>;
#[doc = "Field `LAYER1XOFF` writer - Specify the pixel offset of the starting X dimension of layer 1"]
pub type Layer1xoffW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Specify the pixel offset of the starting Y dimension of layer 1"]
    #[inline(always)]
    pub fn layer1yoff(&self) -> Layer1yoffR {
        Layer1yoffR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Specify the pixel offset of the starting X dimension of layer 1"]
    #[inline(always)]
    pub fn layer1xoff(&self) -> Layer1xoffR {
        Layer1xoffR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Specify the pixel offset of the starting Y dimension of layer 1"]
    #[inline(always)]
    #[must_use]
    pub fn layer1yoff(&mut self) -> Layer1yoffW<Layer1startxySpec> {
        Layer1yoffW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Specify the pixel offset of the starting X dimension of layer 1"]
    #[inline(always)]
    #[must_use]
    pub fn layer1xoff(&mut self) -> Layer1xoffW<Layer1startxySpec> {
        Layer1xoffW::new(self, 16)
    }
}
#[doc = "X and Y start dimensions of layer 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`layer1startxy::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`layer1startxy::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Layer1startxySpec;
impl crate::RegisterSpec for Layer1startxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`layer1startxy::R`](R) reader structure"]
impl crate::Readable for Layer1startxySpec {}
#[doc = "`write(|w| ..)` method takes [`layer1startxy::W`](W) writer structure"]
impl crate::Writable for Layer1startxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LAYER1STARTXY to value 0"]
impl crate::Resettable for Layer1startxySpec {
    const RESET_VALUE: u32 = 0;
}
