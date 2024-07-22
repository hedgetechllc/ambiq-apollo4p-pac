#[doc = "Register `LAYER0SCALEY` reader"]
pub type R = crate::R<Layer0scaleySpec>;
#[doc = "Register `LAYER0SCALEY` writer"]
pub type W = crate::W<Layer0scaleySpec>;
#[doc = "Field `LAYER0YFACTOR` reader - Specify the scale Y factor of layer n (4.14 fixed point number)"]
pub type Layer0yfactorR = crate::FieldReader<u32>;
#[doc = "Field `LAYER0YFACTOR` writer - Specify the scale Y factor of layer n (4.14 fixed point number)"]
pub type Layer0yfactorW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Specify the scale Y factor of layer n (4.14 fixed point number)"]
    #[inline(always)]
    pub fn layer0yfactor(&self) -> Layer0yfactorR {
        Layer0yfactorR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Specify the scale Y factor of layer n (4.14 fixed point number)"]
    #[inline(always)]
    #[must_use]
    pub fn layer0yfactor(&mut self) -> Layer0yfactorW<Layer0scaleySpec> {
        Layer0yfactorW::new(self, 0)
    }
}
#[doc = "Scale Y factor of layer 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`layer0scaley::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`layer0scaley::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Layer0scaleySpec;
impl crate::RegisterSpec for Layer0scaleySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`layer0scaley::R`](R) reader structure"]
impl crate::Readable for Layer0scaleySpec {}
#[doc = "`write(|w| ..)` method takes [`layer0scaley::W`](W) writer structure"]
impl crate::Writable for Layer0scaleySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LAYER0SCALEY to value 0"]
impl crate::Resettable for Layer0scaleySpec {
    const RESET_VALUE: u32 = 0;
}
