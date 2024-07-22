#[doc = "Register `LAYER1SCALEX` reader"]
pub type R = crate::R<Layer1scalexSpec>;
#[doc = "Register `LAYER1SCALEX` writer"]
pub type W = crate::W<Layer1scalexSpec>;
#[doc = "Field `LAYER1XFACTOR` reader - Specify the scale X factor of layer n (4.14 fixed point number)"]
pub type Layer1xfactorR = crate::FieldReader<u32>;
#[doc = "Field `LAYER1XFACTOR` writer - Specify the scale X factor of layer n (4.14 fixed point number)"]
pub type Layer1xfactorW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Specify the scale X factor of layer n (4.14 fixed point number)"]
    #[inline(always)]
    pub fn layer1xfactor(&self) -> Layer1xfactorR {
        Layer1xfactorR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Specify the scale X factor of layer n (4.14 fixed point number)"]
    #[inline(always)]
    #[must_use]
    pub fn layer1xfactor(&mut self) -> Layer1xfactorW<Layer1scalexSpec> {
        Layer1xfactorW::new(self, 0)
    }
}
#[doc = "Scale X factor of layer 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`layer1scalex::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`layer1scalex::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Layer1scalexSpec;
impl crate::RegisterSpec for Layer1scalexSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`layer1scalex::R`](R) reader structure"]
impl crate::Readable for Layer1scalexSpec {}
#[doc = "`write(|w| ..)` method takes [`layer1scalex::W`](W) writer structure"]
impl crate::Writable for Layer1scalexSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LAYER1SCALEX to value 0"]
impl crate::Resettable for Layer1scalexSpec {
    const RESET_VALUE: u32 = 0;
}
