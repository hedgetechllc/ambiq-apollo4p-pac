#[doc = "Register `LAYER3SCALEY` reader"]
pub type R = crate::R<Layer3scaleySpec>;
#[doc = "Register `LAYER3SCALEY` writer"]
pub type W = crate::W<Layer3scaleySpec>;
#[doc = "Field `LAYER3YFACTOR` reader - Specify the scale Y factor of layer n (4.14 fixed point number)"]
pub type Layer3yfactorR = crate::FieldReader<u32>;
#[doc = "Field `LAYER3YFACTOR` writer - Specify the scale Y factor of layer n (4.14 fixed point number)"]
pub type Layer3yfactorW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Specify the scale Y factor of layer n (4.14 fixed point number)"]
    #[inline(always)]
    pub fn layer3yfactor(&self) -> Layer3yfactorR {
        Layer3yfactorR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Specify the scale Y factor of layer n (4.14 fixed point number)"]
    #[inline(always)]
    #[must_use]
    pub fn layer3yfactor(&mut self) -> Layer3yfactorW<Layer3scaleySpec> {
        Layer3yfactorW::new(self, 0)
    }
}
#[doc = "Scale Y factor of layer 3.\n\nYou can [`read`](crate::Reg::read) this register and get [`layer3scaley::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`layer3scaley::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Layer3scaleySpec;
impl crate::RegisterSpec for Layer3scaleySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`layer3scaley::R`](R) reader structure"]
impl crate::Readable for Layer3scaleySpec {}
#[doc = "`write(|w| ..)` method takes [`layer3scaley::W`](W) writer structure"]
impl crate::Writable for Layer3scaleySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LAYER3SCALEY to value 0"]
impl crate::Resettable for Layer3scaleySpec {
    const RESET_VALUE: u32 = 0;
}
