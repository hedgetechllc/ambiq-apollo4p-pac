#[doc = "Register `LAYER3STARTXY` reader"]
pub type R = crate::R<Layer3startxySpec>;
#[doc = "Register `LAYER3STARTXY` writer"]
pub type W = crate::W<Layer3startxySpec>;
#[doc = "Field `LAYER3YOFF` reader - Specify the pixel offset of the starting Y dimension of layer 3"]
pub type Layer3yoffR = crate::FieldReader<u16>;
#[doc = "Field `LAYER3YOFF` writer - Specify the pixel offset of the starting Y dimension of layer 3"]
pub type Layer3yoffW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `LAYER3XOFF` reader - Specify the pixel offset of the starting X dimension of layer 3"]
pub type Layer3xoffR = crate::FieldReader<u16>;
#[doc = "Field `LAYER3XOFF` writer - Specify the pixel offset of the starting X dimension of layer 3"]
pub type Layer3xoffW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Specify the pixel offset of the starting Y dimension of layer 3"]
    #[inline(always)]
    pub fn layer3yoff(&self) -> Layer3yoffR {
        Layer3yoffR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Specify the pixel offset of the starting X dimension of layer 3"]
    #[inline(always)]
    pub fn layer3xoff(&self) -> Layer3xoffR {
        Layer3xoffR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Specify the pixel offset of the starting Y dimension of layer 3"]
    #[inline(always)]
    #[must_use]
    pub fn layer3yoff(&mut self) -> Layer3yoffW<Layer3startxySpec> {
        Layer3yoffW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Specify the pixel offset of the starting X dimension of layer 3"]
    #[inline(always)]
    #[must_use]
    pub fn layer3xoff(&mut self) -> Layer3xoffW<Layer3startxySpec> {
        Layer3xoffW::new(self, 16)
    }
}
#[doc = "X and Y start dimensions of layer 3.\n\nYou can [`read`](crate::Reg::read) this register and get [`layer3startxy::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`layer3startxy::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Layer3startxySpec;
impl crate::RegisterSpec for Layer3startxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`layer3startxy::R`](R) reader structure"]
impl crate::Readable for Layer3startxySpec {}
#[doc = "`write(|w| ..)` method takes [`layer3startxy::W`](W) writer structure"]
impl crate::Writable for Layer3startxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LAYER3STARTXY to value 0"]
impl crate::Resettable for Layer3startxySpec {
    const RESET_VALUE: u32 = 0;
}
