#[doc = "Register `REG40` reader"]
pub type R = crate::R<Reg40Spec>;
#[doc = "Register `REG40` writer"]
pub type W = crate::W<Reg40Spec>;
#[doc = "Field `BF60` reader - This bitfield is reserved."]
pub type Bf60R = crate::FieldReader;
#[doc = "Field `BF60` writer - This bitfield is reserved."]
pub type Bf60W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `BF77` reader - This bitfield is reserved."]
pub type Bf77R = crate::BitReader;
#[doc = "Field `BF77` writer - This bitfield is reserved."]
pub type Bf77W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - This bitfield is reserved."]
    #[inline(always)]
    pub fn bf60(&self) -> Bf60R {
        Bf60R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - This bitfield is reserved."]
    #[inline(always)]
    pub fn bf77(&self) -> Bf77R {
        Bf77R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - This bitfield is reserved."]
    #[inline(always)]
    #[must_use]
    pub fn bf60(&mut self) -> Bf60W<Reg40Spec> {
        Bf60W::new(self, 0)
    }
    #[doc = "Bit 7 - This bitfield is reserved."]
    #[inline(always)]
    #[must_use]
    pub fn bf77(&mut self) -> Bf77W<Reg40Spec> {
        Bf77W::new(self, 7)
    }
}
#[doc = "Register description here\n\nYou can [`read`](crate::Reg::read) this register and get [`reg40::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg40::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reg40Spec;
impl crate::RegisterSpec for Reg40Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg40::R`](R) reader structure"]
impl crate::Readable for Reg40Spec {}
#[doc = "`write(|w| ..)` method takes [`reg40::W`](W) writer structure"]
impl crate::Writable for Reg40Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REG40 to value 0"]
impl crate::Resettable for Reg40Spec {
    const RESET_VALUE: u32 = 0;
}
