#[doc = "Register `COMPONENTID1` reader"]
pub type R = crate::R<Componentid1Spec>;
#[doc = "Register `COMPONENTID1` writer"]
pub type W = crate::W<Componentid1Spec>;
#[doc = "Field `PRMBL1` reader - constant 0x0"]
pub type Prmbl1R = crate::FieldReader;
#[doc = "Field `PRMBL1` writer - constant 0x0"]
pub type Prmbl1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CLASS` reader - component type 0 0xF for Cryptocell"]
pub type ClassR = crate::FieldReader;
#[doc = "Field `CLASS` writer - component type 0 0xF for Cryptocell"]
pub type ClassW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - constant 0x0"]
    #[inline(always)]
    pub fn prmbl1(&self) -> Prmbl1R {
        Prmbl1R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - component type 0 0xF for Cryptocell"]
    #[inline(always)]
    pub fn class(&self) -> ClassR {
        ClassR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - constant 0x0"]
    #[inline(always)]
    #[must_use]
    pub fn prmbl1(&mut self) -> Prmbl1W<Componentid1Spec> {
        Prmbl1W::new(self, 0)
    }
    #[doc = "Bits 4:7 - component type 0 0xF for Cryptocell"]
    #[inline(always)]
    #[must_use]
    pub fn class(&mut self) -> ClassW<Componentid1Spec> {
        ClassW::new(self, 4)
    }
}
#[doc = "Component ID1.\n\nYou can [`read`](crate::Reg::read) this register and get [`componentid1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`componentid1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Componentid1Spec;
impl crate::RegisterSpec for Componentid1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`componentid1::R`](R) reader structure"]
impl crate::Readable for Componentid1Spec {}
#[doc = "`write(|w| ..)` method takes [`componentid1::W`](W) writer structure"]
impl crate::Writable for Componentid1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets COMPONENTID1 to value 0xf0"]
impl crate::Resettable for Componentid1Spec {
    const RESET_VALUE: u32 = 0xf0;
}
