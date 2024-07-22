#[doc = "Register `LAYER1SCALEY` reader"]
pub type R = crate::R<Layer1scaleySpec>;
#[doc = "Register `LAYER1SCALEY` writer"]
pub type W = crate::W<Layer1scaleySpec>;
#[doc = "Field `LAYER1YFACTOR` reader - Specify the scale Y factor of layer n (4.14 fixed point number)"]
pub type Layer1yfactorR = crate::FieldReader<u32>;
#[doc = "Field `LAYER1YFACTOR` writer - Specify the scale Y factor of layer n (4.14 fixed point number)"]
pub type Layer1yfactorW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Specify the scale Y factor of layer n (4.14 fixed point number)"]
    #[inline(always)]
    pub fn layer1yfactor(&self) -> Layer1yfactorR {
        Layer1yfactorR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Specify the scale Y factor of layer n (4.14 fixed point number)"]
    #[inline(always)]
    #[must_use]
    pub fn layer1yfactor(&mut self) -> Layer1yfactorW<Layer1scaleySpec> {
        Layer1yfactorW::new(self, 0)
    }
}
#[doc = "Scale Y factor of layer 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`layer1scaley::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`layer1scaley::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Layer1scaleySpec;
impl crate::RegisterSpec for Layer1scaleySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`layer1scaley::R`](R) reader structure"]
impl crate::Readable for Layer1scaleySpec {}
#[doc = "`write(|w| ..)` method takes [`layer1scaley::W`](W) writer structure"]
impl crate::Writable for Layer1scaleySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LAYER1SCALEY to value 0"]
impl crate::Resettable for Layer1scaleySpec {
    const RESET_VALUE: u32 = 0;
}
