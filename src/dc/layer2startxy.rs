#[doc = "Register `LAYER2STARTXY` reader"]
pub type R = crate::R<Layer2startxySpec>;
#[doc = "Register `LAYER2STARTXY` writer"]
pub type W = crate::W<Layer2startxySpec>;
#[doc = "Field `LAYER2YOFF` reader - Specify the pixel offset of the starting Y dimension of layer 2"]
pub type Layer2yoffR = crate::FieldReader<u16>;
#[doc = "Field `LAYER2YOFF` writer - Specify the pixel offset of the starting Y dimension of layer 2"]
pub type Layer2yoffW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `LAYER2XOFF` reader - Specify the pixel offset of the starting X dimension of layer 2"]
pub type Layer2xoffR = crate::FieldReader<u16>;
#[doc = "Field `LAYER2XOFF` writer - Specify the pixel offset of the starting X dimension of layer 2"]
pub type Layer2xoffW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Specify the pixel offset of the starting Y dimension of layer 2"]
    #[inline(always)]
    pub fn layer2yoff(&self) -> Layer2yoffR {
        Layer2yoffR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Specify the pixel offset of the starting X dimension of layer 2"]
    #[inline(always)]
    pub fn layer2xoff(&self) -> Layer2xoffR {
        Layer2xoffR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Specify the pixel offset of the starting Y dimension of layer 2"]
    #[inline(always)]
    #[must_use]
    pub fn layer2yoff(&mut self) -> Layer2yoffW<Layer2startxySpec> {
        Layer2yoffW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Specify the pixel offset of the starting X dimension of layer 2"]
    #[inline(always)]
    #[must_use]
    pub fn layer2xoff(&mut self) -> Layer2xoffW<Layer2startxySpec> {
        Layer2xoffW::new(self, 16)
    }
}
#[doc = "X and Y start dimensions of layer 2.\n\nYou can [`read`](crate::Reg::read) this register and get [`layer2startxy::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`layer2startxy::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Layer2startxySpec;
impl crate::RegisterSpec for Layer2startxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`layer2startxy::R`](R) reader structure"]
impl crate::Readable for Layer2startxySpec {}
#[doc = "`write(|w| ..)` method takes [`layer2startxy::W`](W) writer structure"]
impl crate::Writable for Layer2startxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LAYER2STARTXY to value 0"]
impl crate::Resettable for Layer2startxySpec {
    const RESET_VALUE: u32 = 0;
}
