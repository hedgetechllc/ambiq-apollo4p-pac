#[doc = "Register `LAYER2SCALEY` reader"]
pub type R = crate::R<Layer2scaleySpec>;
#[doc = "Register `LAYER2SCALEY` writer"]
pub type W = crate::W<Layer2scaleySpec>;
#[doc = "Field `LAYER2YFACTOR` reader - Specify the scale Y factor of layer n (4.14 fixed point number)"]
pub type Layer2yfactorR = crate::FieldReader<u32>;
#[doc = "Field `LAYER2YFACTOR` writer - Specify the scale Y factor of layer n (4.14 fixed point number)"]
pub type Layer2yfactorW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Specify the scale Y factor of layer n (4.14 fixed point number)"]
    #[inline(always)]
    pub fn layer2yfactor(&self) -> Layer2yfactorR {
        Layer2yfactorR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Specify the scale Y factor of layer n (4.14 fixed point number)"]
    #[inline(always)]
    #[must_use]
    pub fn layer2yfactor(&mut self) -> Layer2yfactorW<Layer2scaleySpec> {
        Layer2yfactorW::new(self, 0)
    }
}
#[doc = "Scale Y factor of layer 2.\n\nYou can [`read`](crate::Reg::read) this register and get [`layer2scaley::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`layer2scaley::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Layer2scaleySpec;
impl crate::RegisterSpec for Layer2scaleySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`layer2scaley::R`](R) reader structure"]
impl crate::Readable for Layer2scaleySpec {}
#[doc = "`write(|w| ..)` method takes [`layer2scaley::W`](W) writer structure"]
impl crate::Writable for Layer2scaleySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LAYER2SCALEY to value 0"]
impl crate::Resettable for Layer2scaleySpec {
    const RESET_VALUE: u32 = 0;
}
