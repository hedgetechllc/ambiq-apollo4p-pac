#[doc = "Register `LAYER3RESXY` reader"]
pub type R = crate::R<Layer3resxySpec>;
#[doc = "Register `LAYER3RESXY` writer"]
pub type W = crate::W<Layer3resxySpec>;
#[doc = "Field `LAYER3PIXRESY` reader - Specify the layer n pixel resolution in the Y dimension"]
pub type Layer3pixresyR = crate::FieldReader<u16>;
#[doc = "Field `LAYER3PIXRESY` writer - Specify the layer n pixel resolution in the Y dimension"]
pub type Layer3pixresyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `LAYER3PIXRESX` reader - Specify the layer n pixel resolution in the X dimension"]
pub type Layer3pixresxR = crate::FieldReader<u16>;
#[doc = "Field `LAYER3PIXRESX` writer - Specify the layer n pixel resolution in the X dimension"]
pub type Layer3pixresxW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Specify the layer n pixel resolution in the Y dimension"]
    #[inline(always)]
    pub fn layer3pixresy(&self) -> Layer3pixresyR {
        Layer3pixresyR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Specify the layer n pixel resolution in the X dimension"]
    #[inline(always)]
    pub fn layer3pixresx(&self) -> Layer3pixresxR {
        Layer3pixresxR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Specify the layer n pixel resolution in the Y dimension"]
    #[inline(always)]
    #[must_use]
    pub fn layer3pixresy(&mut self) -> Layer3pixresyW<Layer3resxySpec> {
        Layer3pixresyW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Specify the layer n pixel resolution in the X dimension"]
    #[inline(always)]
    #[must_use]
    pub fn layer3pixresx(&mut self) -> Layer3pixresxW<Layer3resxySpec> {
        Layer3pixresxW::new(self, 16)
    }
}
#[doc = "X and Y dimensions for the resolution of layer 3.\n\nYou can [`read`](crate::Reg::read) this register and get [`layer3resxy::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`layer3resxy::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Layer3resxySpec;
impl crate::RegisterSpec for Layer3resxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`layer3resxy::R`](R) reader structure"]
impl crate::Readable for Layer3resxySpec {}
#[doc = "`write(|w| ..)` method takes [`layer3resxy::W`](W) writer structure"]
impl crate::Writable for Layer3resxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LAYER3RESXY to value 0"]
impl crate::Resettable for Layer3resxySpec {
    const RESET_VALUE: u32 = 0;
}
