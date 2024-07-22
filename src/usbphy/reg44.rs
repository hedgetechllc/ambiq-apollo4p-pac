#[doc = "Register `REG44` reader"]
pub type R = crate::R<Reg44Spec>;
#[doc = "Register `REG44` writer"]
pub type W = crate::W<Reg44Spec>;
#[doc = "Field `BF00` reader - 1: DP/DM will be sampled in HS Tx or Rx state 0: DP/DM will be sampled only in Hs Rx state"]
pub type Bf00R = crate::BitReader;
#[doc = "Field `BF00` writer - 1: DP/DM will be sampled in HS Tx or Rx state 0: DP/DM will be sampled only in Hs Rx state"]
pub type Bf00W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BF11` reader - Disconnect squelch and comparator calibration bypass, active high"]
pub type Bf11R = crate::BitReader;
#[doc = "Field `BF11` writer - Disconnect squelch and comparator calibration bypass, active high"]
pub type Bf11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BF42` reader - This bitfield is reserved."]
pub type Bf42R = crate::FieldReader;
#[doc = "Field `BF42` writer - This bitfield is reserved."]
pub type Bf42W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BF65` reader - This bitfield is reserved."]
pub type Bf65R = crate::FieldReader;
#[doc = "Field `BF65` writer - This bitfield is reserved."]
pub type Bf65W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BF77` reader - This bitfield is reserved."]
pub type Bf77R = crate::BitReader;
#[doc = "Field `BF77` writer - This bitfield is reserved."]
pub type Bf77W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 1: DP/DM will be sampled in HS Tx or Rx state 0: DP/DM will be sampled only in Hs Rx state"]
    #[inline(always)]
    pub fn bf00(&self) -> Bf00R {
        Bf00R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Disconnect squelch and comparator calibration bypass, active high"]
    #[inline(always)]
    pub fn bf11(&self) -> Bf11R {
        Bf11R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - This bitfield is reserved."]
    #[inline(always)]
    pub fn bf42(&self) -> Bf42R {
        Bf42R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 5:6 - This bitfield is reserved."]
    #[inline(always)]
    pub fn bf65(&self) -> Bf65R {
        Bf65R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - This bitfield is reserved."]
    #[inline(always)]
    pub fn bf77(&self) -> Bf77R {
        Bf77R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1: DP/DM will be sampled in HS Tx or Rx state 0: DP/DM will be sampled only in Hs Rx state"]
    #[inline(always)]
    #[must_use]
    pub fn bf00(&mut self) -> Bf00W<Reg44Spec> {
        Bf00W::new(self, 0)
    }
    #[doc = "Bit 1 - Disconnect squelch and comparator calibration bypass, active high"]
    #[inline(always)]
    #[must_use]
    pub fn bf11(&mut self) -> Bf11W<Reg44Spec> {
        Bf11W::new(self, 1)
    }
    #[doc = "Bits 2:4 - This bitfield is reserved."]
    #[inline(always)]
    #[must_use]
    pub fn bf42(&mut self) -> Bf42W<Reg44Spec> {
        Bf42W::new(self, 2)
    }
    #[doc = "Bits 5:6 - This bitfield is reserved."]
    #[inline(always)]
    #[must_use]
    pub fn bf65(&mut self) -> Bf65W<Reg44Spec> {
        Bf65W::new(self, 5)
    }
    #[doc = "Bit 7 - This bitfield is reserved."]
    #[inline(always)]
    #[must_use]
    pub fn bf77(&mut self) -> Bf77W<Reg44Spec> {
        Bf77W::new(self, 7)
    }
}
#[doc = "Register description here\n\nYou can [`read`](crate::Reg::read) this register and get [`reg44::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg44::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reg44Spec;
impl crate::RegisterSpec for Reg44Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg44::R`](R) reader structure"]
impl crate::Readable for Reg44Spec {}
#[doc = "`write(|w| ..)` method takes [`reg44::W`](W) writer structure"]
impl crate::Writable for Reg44Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REG44 to value 0"]
impl crate::Resettable for Reg44Spec {
    const RESET_VALUE: u32 = 0;
}
