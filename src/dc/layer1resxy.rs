#[doc = "Register `LAYER1RESXY` reader"]
pub type R = crate::R<Layer1resxySpec>;
#[doc = "Register `LAYER1RESXY` writer"]
pub type W = crate::W<Layer1resxySpec>;
#[doc = "Field `LAYER1PIXRESY` reader - Specify the layer n pixel resolution in the Y dimension"]
pub type Layer1pixresyR = crate::FieldReader<u16>;
#[doc = "Field `LAYER1PIXRESY` writer - Specify the layer n pixel resolution in the Y dimension"]
pub type Layer1pixresyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `LAYER1PIXRESX` reader - Specify the layer n pixel resolution in the X dimension"]
pub type Layer1pixresxR = crate::FieldReader<u16>;
#[doc = "Field `LAYER1PIXRESX` writer - Specify the layer n pixel resolution in the X dimension"]
pub type Layer1pixresxW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Specify the layer n pixel resolution in the Y dimension"]
    #[inline(always)]
    pub fn layer1pixresy(&self) -> Layer1pixresyR {
        Layer1pixresyR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Specify the layer n pixel resolution in the X dimension"]
    #[inline(always)]
    pub fn layer1pixresx(&self) -> Layer1pixresxR {
        Layer1pixresxR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Specify the layer n pixel resolution in the Y dimension"]
    #[inline(always)]
    #[must_use]
    pub fn layer1pixresy(&mut self) -> Layer1pixresyW<Layer1resxySpec> {
        Layer1pixresyW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Specify the layer n pixel resolution in the X dimension"]
    #[inline(always)]
    #[must_use]
    pub fn layer1pixresx(&mut self) -> Layer1pixresxW<Layer1resxySpec> {
        Layer1pixresxW::new(self, 16)
    }
}
#[doc = "X and Y dimensions for the resolution of layer 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`layer1resxy::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`layer1resxy::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Layer1resxySpec;
impl crate::RegisterSpec for Layer1resxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`layer1resxy::R`](R) reader structure"]
impl crate::Readable for Layer1resxySpec {}
#[doc = "`write(|w| ..)` method takes [`layer1resxy::W`](W) writer structure"]
impl crate::Writable for Layer1resxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LAYER1RESXY to value 0"]
impl crate::Resettable for Layer1resxySpec {
    const RESET_VALUE: u32 = 0;
}
