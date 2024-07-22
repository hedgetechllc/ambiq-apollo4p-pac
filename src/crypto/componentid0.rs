#[doc = "Register `COMPONENTID0` reader"]
pub type R = crate::R<Componentid0Spec>;
#[doc = "Register `COMPONENTID0` writer"]
pub type W = crate::W<Componentid0Spec>;
#[doc = "Field `PRMBL0` reader - constant 0xD"]
pub type Prmbl0R = crate::FieldReader;
#[doc = "Field `PRMBL0` writer - constant 0xD"]
pub type Prmbl0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - constant 0xD"]
    #[inline(always)]
    pub fn prmbl0(&self) -> Prmbl0R {
        Prmbl0R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - constant 0xD"]
    #[inline(always)]
    #[must_use]
    pub fn prmbl0(&mut self) -> Prmbl0W<Componentid0Spec> {
        Prmbl0W::new(self, 0)
    }
}
#[doc = "Component ID0.\n\nYou can [`read`](crate::Reg::read) this register and get [`componentid0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`componentid0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Componentid0Spec;
impl crate::RegisterSpec for Componentid0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`componentid0::R`](R) reader structure"]
impl crate::Readable for Componentid0Spec {}
#[doc = "`write(|w| ..)` method takes [`componentid0::W`](W) writer structure"]
impl crate::Writable for Componentid0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets COMPONENTID0 to value 0x0d"]
impl crate::Resettable for Componentid0Spec {
    const RESET_VALUE: u32 = 0x0d;
}
