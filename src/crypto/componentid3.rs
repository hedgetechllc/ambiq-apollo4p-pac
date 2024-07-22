#[doc = "Register `COMPONENTID3` reader"]
pub type R = crate::R<Componentid3Spec>;
#[doc = "Register `COMPONENTID3` writer"]
pub type W = crate::W<Componentid3Spec>;
#[doc = "Field `PRMBL3` reader - constant 0xB1"]
pub type Prmbl3R = crate::FieldReader;
#[doc = "Field `PRMBL3` writer - constant 0xB1"]
pub type Prmbl3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - constant 0xB1"]
    #[inline(always)]
    pub fn prmbl3(&self) -> Prmbl3R {
        Prmbl3R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - constant 0xB1"]
    #[inline(always)]
    #[must_use]
    pub fn prmbl3(&mut self) -> Prmbl3W<Componentid3Spec> {
        Prmbl3W::new(self, 0)
    }
}
#[doc = "Component ID3.\n\nYou can [`read`](crate::Reg::read) this register and get [`componentid3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`componentid3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Componentid3Spec;
impl crate::RegisterSpec for Componentid3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`componentid3::R`](R) reader structure"]
impl crate::Readable for Componentid3Spec {}
#[doc = "`write(|w| ..)` method takes [`componentid3::W`](W) writer structure"]
impl crate::Writable for Componentid3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets COMPONENTID3 to value 0xb1"]
impl crate::Resettable for Componentid3Spec {
    const RESET_VALUE: u32 = 0xb1;
}
