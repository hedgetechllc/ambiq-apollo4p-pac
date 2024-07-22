#[doc = "Register `LAYER3SCALEX` reader"]
pub type R = crate::R<Layer3scalexSpec>;
#[doc = "Register `LAYER3SCALEX` writer"]
pub type W = crate::W<Layer3scalexSpec>;
#[doc = "Field `LAYER3XFACTOR` reader - Specify the scale X factor of layer n (4.14 fixed point number)"]
pub type Layer3xfactorR = crate::FieldReader<u32>;
#[doc = "Field `LAYER3XFACTOR` writer - Specify the scale X factor of layer n (4.14 fixed point number)"]
pub type Layer3xfactorW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Specify the scale X factor of layer n (4.14 fixed point number)"]
    #[inline(always)]
    pub fn layer3xfactor(&self) -> Layer3xfactorR {
        Layer3xfactorR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Specify the scale X factor of layer n (4.14 fixed point number)"]
    #[inline(always)]
    #[must_use]
    pub fn layer3xfactor(&mut self) -> Layer3xfactorW<Layer3scalexSpec> {
        Layer3xfactorW::new(self, 0)
    }
}
#[doc = "Scale X factor of layer 3.\n\nYou can [`read`](crate::Reg::read) this register and get [`layer3scalex::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`layer3scalex::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Layer3scalexSpec;
impl crate::RegisterSpec for Layer3scalexSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`layer3scalex::R`](R) reader structure"]
impl crate::Readable for Layer3scalexSpec {}
#[doc = "`write(|w| ..)` method takes [`layer3scalex::W`](W) writer structure"]
impl crate::Writable for Layer3scalexSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LAYER3SCALEX to value 0"]
impl crate::Resettable for Layer3scalexSpec {
    const RESET_VALUE: u32 = 0;
}
