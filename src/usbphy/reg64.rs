#[doc = "Register `REG64` reader"]
pub type R = crate::R<Reg64Spec>;
#[doc = "Register `REG64` writer"]
pub type W = crate::W<Reg64Spec>;
#[doc = "Field `BF00` reader - This bitfield is reserved."]
pub type Bf00R = crate::BitReader;
#[doc = "Field `BF00` writer - This bitfield is reserved."]
pub type Bf00W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This bitfield is reserved."]
    #[inline(always)]
    pub fn bf00(&self) -> Bf00R {
        Bf00R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bitfield is reserved."]
    #[inline(always)]
    #[must_use]
    pub fn bf00(&mut self) -> Bf00W<Reg64Spec> {
        Bf00W::new(self, 0)
    }
}
#[doc = "Register description here\n\nYou can [`read`](crate::Reg::read) this register and get [`reg64::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg64::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reg64Spec;
impl crate::RegisterSpec for Reg64Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg64::R`](R) reader structure"]
impl crate::Readable for Reg64Spec {}
#[doc = "`write(|w| ..)` method takes [`reg64::W`](W) writer structure"]
impl crate::Writable for Reg64Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REG64 to value 0"]
impl crate::Resettable for Reg64Spec {
    const RESET_VALUE: u32 = 0;
}
