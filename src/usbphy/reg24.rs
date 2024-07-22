#[doc = "Register `REG24` reader"]
pub type R = crate::R<Reg24Spec>;
#[doc = "Register `REG24` writer"]
pub type W = crate::W<Reg24Spec>;
#[doc = "Field `BF00` reader - it0"]
pub type Bf00R = crate::BitReader;
#[doc = "Field `BF00` writer - it0"]
pub type Bf00W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BF71` reader - This bitfield is reserved."]
pub type Bf71R = crate::FieldReader;
#[doc = "Field `BF71` writer - This bitfield is reserved."]
pub type Bf71W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bit 0 - it0"]
    #[inline(always)]
    pub fn bf00(&self) -> Bf00R {
        Bf00R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - This bitfield is reserved."]
    #[inline(always)]
    pub fn bf71(&self) -> Bf71R {
        Bf71R::new(((self.bits >> 1) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - it0"]
    #[inline(always)]
    #[must_use]
    pub fn bf00(&mut self) -> Bf00W<Reg24Spec> {
        Bf00W::new(self, 0)
    }
    #[doc = "Bits 1:7 - This bitfield is reserved."]
    #[inline(always)]
    #[must_use]
    pub fn bf71(&mut self) -> Bf71W<Reg24Spec> {
        Bf71W::new(self, 1)
    }
}
#[doc = "Register description here\n\nYou can [`read`](crate::Reg::read) this register and get [`reg24::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg24::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reg24Spec;
impl crate::RegisterSpec for Reg24Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg24::R`](R) reader structure"]
impl crate::Readable for Reg24Spec {}
#[doc = "`write(|w| ..)` method takes [`reg24::W`](W) writer structure"]
impl crate::Writable for Reg24Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REG24 to value 0"]
impl crate::Resettable for Reg24Spec {
    const RESET_VALUE: u32 = 0;
}
