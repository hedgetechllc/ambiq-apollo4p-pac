#[doc = "Register `LAYER2RESXY` reader"]
pub type R = crate::R<Layer2resxySpec>;
#[doc = "Register `LAYER2RESXY` writer"]
pub type W = crate::W<Layer2resxySpec>;
#[doc = "Field `LAYER2PIXRESY` reader - Specify the layer n pixel resolution in the Y dimension"]
pub type Layer2pixresyR = crate::FieldReader<u16>;
#[doc = "Field `LAYER2PIXRESY` writer - Specify the layer n pixel resolution in the Y dimension"]
pub type Layer2pixresyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `LAYER2PIXRESX` reader - Specify the layer n pixel resolution in the X dimension"]
pub type Layer2pixresxR = crate::FieldReader<u16>;
#[doc = "Field `LAYER2PIXRESX` writer - Specify the layer n pixel resolution in the X dimension"]
pub type Layer2pixresxW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Specify the layer n pixel resolution in the Y dimension"]
    #[inline(always)]
    pub fn layer2pixresy(&self) -> Layer2pixresyR {
        Layer2pixresyR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Specify the layer n pixel resolution in the X dimension"]
    #[inline(always)]
    pub fn layer2pixresx(&self) -> Layer2pixresxR {
        Layer2pixresxR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Specify the layer n pixel resolution in the Y dimension"]
    #[inline(always)]
    #[must_use]
    pub fn layer2pixresy(&mut self) -> Layer2pixresyW<Layer2resxySpec> {
        Layer2pixresyW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Specify the layer n pixel resolution in the X dimension"]
    #[inline(always)]
    #[must_use]
    pub fn layer2pixresx(&mut self) -> Layer2pixresxW<Layer2resxySpec> {
        Layer2pixresxW::new(self, 16)
    }
}
#[doc = "X and Y dimensions for the resolution of layer 2.\n\nYou can [`read`](crate::Reg::read) this register and get [`layer2resxy::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`layer2resxy::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Layer2resxySpec;
impl crate::RegisterSpec for Layer2resxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`layer2resxy::R`](R) reader structure"]
impl crate::Readable for Layer2resxySpec {}
#[doc = "`write(|w| ..)` method takes [`layer2resxy::W`](W) writer structure"]
impl crate::Writable for Layer2resxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LAYER2RESXY to value 0"]
impl crate::Resettable for Layer2resxySpec {
    const RESET_VALUE: u32 = 0;
}
