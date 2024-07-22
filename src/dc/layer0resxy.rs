#[doc = "Register `LAYER0RESXY` reader"]
pub type R = crate::R<Layer0resxySpec>;
#[doc = "Register `LAYER0RESXY` writer"]
pub type W = crate::W<Layer0resxySpec>;
#[doc = "Field `LAYER0PIXRESY` reader - Specify the layer n pixel resolution in the Y dimension"]
pub type Layer0pixresyR = crate::FieldReader<u16>;
#[doc = "Field `LAYER0PIXRESY` writer - Specify the layer n pixel resolution in the Y dimension"]
pub type Layer0pixresyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `LAYER0PIXRESX` reader - Specify the layer n pixel resolution in the X dimension"]
pub type Layer0pixresxR = crate::FieldReader<u16>;
#[doc = "Field `LAYER0PIXRESX` writer - Specify the layer n pixel resolution in the X dimension"]
pub type Layer0pixresxW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Specify the layer n pixel resolution in the Y dimension"]
    #[inline(always)]
    pub fn layer0pixresy(&self) -> Layer0pixresyR {
        Layer0pixresyR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Specify the layer n pixel resolution in the X dimension"]
    #[inline(always)]
    pub fn layer0pixresx(&self) -> Layer0pixresxR {
        Layer0pixresxR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Specify the layer n pixel resolution in the Y dimension"]
    #[inline(always)]
    #[must_use]
    pub fn layer0pixresy(&mut self) -> Layer0pixresyW<Layer0resxySpec> {
        Layer0pixresyW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Specify the layer n pixel resolution in the X dimension"]
    #[inline(always)]
    #[must_use]
    pub fn layer0pixresx(&mut self) -> Layer0pixresxW<Layer0resxySpec> {
        Layer0pixresxW::new(self, 16)
    }
}
#[doc = "X and Y dimensions for the resolution of layer 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`layer0resxy::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`layer0resxy::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Layer0resxySpec;
impl crate::RegisterSpec for Layer0resxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`layer0resxy::R`](R) reader structure"]
impl crate::Readable for Layer0resxySpec {}
#[doc = "`write(|w| ..)` method takes [`layer0resxy::W`](W) writer structure"]
impl crate::Writable for Layer0resxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LAYER0RESXY to value 0"]
impl crate::Resettable for Layer0resxySpec {
    const RESET_VALUE: u32 = 0;
}
