#[doc = "Register `REG60` reader"]
pub type R = crate::R<Reg60Spec>;
#[doc = "Register `REG60` writer"]
pub type W = crate::W<Reg60Spec>;
#[doc = "Field `BF70` reader - This bitfield is reserved."]
pub type Bf70R = crate::FieldReader;
#[doc = "Field `BF70` writer - This bitfield is reserved."]
pub type Bf70W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - This bitfield is reserved."]
    #[inline(always)]
    pub fn bf70(&self) -> Bf70R {
        Bf70R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - This bitfield is reserved."]
    #[inline(always)]
    #[must_use]
    pub fn bf70(&mut self) -> Bf70W<Reg60Spec> {
        Bf70W::new(self, 0)
    }
}
#[doc = "Register description here\n\nYou can [`read`](crate::Reg::read) this register and get [`reg60::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg60::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reg60Spec;
impl crate::RegisterSpec for Reg60Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg60::R`](R) reader structure"]
impl crate::Readable for Reg60Spec {}
#[doc = "`write(|w| ..)` method takes [`reg60::W`](W) writer structure"]
impl crate::Writable for Reg60Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REG60 to value 0"]
impl crate::Resettable for Reg60Spec {
    const RESET_VALUE: u32 = 0;
}
