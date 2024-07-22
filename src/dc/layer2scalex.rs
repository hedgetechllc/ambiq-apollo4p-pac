#[doc = "Register `LAYER2SCALEX` reader"]
pub type R = crate::R<Layer2scalexSpec>;
#[doc = "Register `LAYER2SCALEX` writer"]
pub type W = crate::W<Layer2scalexSpec>;
#[doc = "Field `LAYER2XFACTOR` reader - Specify the scale X factor of layer n (4.14 fixed point number)"]
pub type Layer2xfactorR = crate::FieldReader<u32>;
#[doc = "Field `LAYER2XFACTOR` writer - Specify the scale X factor of layer n (4.14 fixed point number)"]
pub type Layer2xfactorW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Specify the scale X factor of layer n (4.14 fixed point number)"]
    #[inline(always)]
    pub fn layer2xfactor(&self) -> Layer2xfactorR {
        Layer2xfactorR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Specify the scale X factor of layer n (4.14 fixed point number)"]
    #[inline(always)]
    #[must_use]
    pub fn layer2xfactor(&mut self) -> Layer2xfactorW<Layer2scalexSpec> {
        Layer2xfactorW::new(self, 0)
    }
}
#[doc = "Scale X factor of layer 2.\n\nYou can [`read`](crate::Reg::read) this register and get [`layer2scalex::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`layer2scalex::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Layer2scalexSpec;
impl crate::RegisterSpec for Layer2scalexSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`layer2scalex::R`](R) reader structure"]
impl crate::Readable for Layer2scalexSpec {}
#[doc = "`write(|w| ..)` method takes [`layer2scalex::W`](W) writer structure"]
impl crate::Writable for Layer2scalexSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LAYER2SCALEX to value 0"]
impl crate::Resettable for Layer2scalexSpec {
    const RESET_VALUE: u32 = 0;
}
