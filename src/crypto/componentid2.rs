#[doc = "Register `COMPONENTID2` reader"]
pub type R = crate::R<Componentid2Spec>;
#[doc = "Register `COMPONENTID2` writer"]
pub type W = crate::W<Componentid2Spec>;
#[doc = "Field `PRMBL2` reader - constant 0x5"]
pub type Prmbl2R = crate::FieldReader;
#[doc = "Field `PRMBL2` writer - constant 0x5"]
pub type Prmbl2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - constant 0x5"]
    #[inline(always)]
    pub fn prmbl2(&self) -> Prmbl2R {
        Prmbl2R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - constant 0x5"]
    #[inline(always)]
    #[must_use]
    pub fn prmbl2(&mut self) -> Prmbl2W<Componentid2Spec> {
        Prmbl2W::new(self, 0)
    }
}
#[doc = "Component ID2.\n\nYou can [`read`](crate::Reg::read) this register and get [`componentid2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`componentid2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Componentid2Spec;
impl crate::RegisterSpec for Componentid2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`componentid2::R`](R) reader structure"]
impl crate::Readable for Componentid2Spec {}
#[doc = "`write(|w| ..)` method takes [`componentid2::W`](W) writer structure"]
impl crate::Writable for Componentid2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets COMPONENTID2 to value 0x05"]
impl crate::Resettable for Componentid2Spec {
    const RESET_VALUE: u32 = 0x05;
}
