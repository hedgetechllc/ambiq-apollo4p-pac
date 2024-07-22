#[doc = "Register `LAYER0SCALEX` reader"]
pub type R = crate::R<Layer0scalexSpec>;
#[doc = "Register `LAYER0SCALEX` writer"]
pub type W = crate::W<Layer0scalexSpec>;
#[doc = "Field `LAYER0XFACTOR` reader - Specify the scale X factor of layer n (4.14 fixed point number)"]
pub type Layer0xfactorR = crate::FieldReader<u32>;
#[doc = "Field `LAYER0XFACTOR` writer - Specify the scale X factor of layer n (4.14 fixed point number)"]
pub type Layer0xfactorW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Specify the scale X factor of layer n (4.14 fixed point number)"]
    #[inline(always)]
    pub fn layer0xfactor(&self) -> Layer0xfactorR {
        Layer0xfactorR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Specify the scale X factor of layer n (4.14 fixed point number)"]
    #[inline(always)]
    #[must_use]
    pub fn layer0xfactor(&mut self) -> Layer0xfactorW<Layer0scalexSpec> {
        Layer0xfactorW::new(self, 0)
    }
}
#[doc = "Scale X factor of layer 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`layer0scalex::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`layer0scalex::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Layer0scalexSpec;
impl crate::RegisterSpec for Layer0scalexSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`layer0scalex::R`](R) reader structure"]
impl crate::Readable for Layer0scalexSpec {}
#[doc = "`write(|w| ..)` method takes [`layer0scalex::W`](W) writer structure"]
impl crate::Writable for Layer0scalexSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LAYER0SCALEX to value 0"]
impl crate::Resettable for Layer0scalexSpec {
    const RESET_VALUE: u32 = 0;
}
