#[doc = "Register `REG04` reader"]
pub type R = crate::R<Reg04Spec>;
#[doc = "Register `REG04` writer"]
pub type W = crate::W<Reg04Spec>;
#[doc = "Field `BF20` reader - Manually set the Tx Clock phase select. These bits will tune the HS TX path sample timing between digital and analog inside PHY 3'b000 represents the earliest phase 3'b111 represents the latest phase The delay associated with each step is 256ps"]
pub type Bf20R = crate::FieldReader;
#[doc = "Field `BF20` writer - Manually set the Tx Clock phase select. These bits will tune the HS TX path sample timing between digital and analog inside PHY 3'b000 represents the earliest phase 3'b111 represents the latest phase The delay associated with each step is 256ps"]
pub type Bf20W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BF43` reader - Squelch detector bias current tuning, 2'b00 represents the minimum bias current 2'b11 represents the maximum bias current"]
pub type Bf43R = crate::FieldReader;
#[doc = "Field `BF43` writer - Squelch detector bias current tuning, 2'b00 represents the minimum bias current 2'b11 represents the maximum bias current"]
pub type Bf43W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BF55` reader - This bitfield is reserved."]
pub type Bf55R = crate::BitReader;
#[doc = "Field `BF55` writer - This bitfield is reserved."]
pub type Bf55W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BF76` reader - disconnect detector bias current tuning"]
pub type Bf76R = crate::FieldReader;
#[doc = "Field `BF76` writer - disconnect detector bias current tuning"]
pub type Bf76W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:2 - Manually set the Tx Clock phase select. These bits will tune the HS TX path sample timing between digital and analog inside PHY 3'b000 represents the earliest phase 3'b111 represents the latest phase The delay associated with each step is 256ps"]
    #[inline(always)]
    pub fn bf20(&self) -> Bf20R {
        Bf20R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:4 - Squelch detector bias current tuning, 2'b00 represents the minimum bias current 2'b11 represents the maximum bias current"]
    #[inline(always)]
    pub fn bf43(&self) -> Bf43R {
        Bf43R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - This bitfield is reserved."]
    #[inline(always)]
    pub fn bf55(&self) -> Bf55R {
        Bf55R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - disconnect detector bias current tuning"]
    #[inline(always)]
    pub fn bf76(&self) -> Bf76R {
        Bf76R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Manually set the Tx Clock phase select. These bits will tune the HS TX path sample timing between digital and analog inside PHY 3'b000 represents the earliest phase 3'b111 represents the latest phase The delay associated with each step is 256ps"]
    #[inline(always)]
    #[must_use]
    pub fn bf20(&mut self) -> Bf20W<Reg04Spec> {
        Bf20W::new(self, 0)
    }
    #[doc = "Bits 3:4 - Squelch detector bias current tuning, 2'b00 represents the minimum bias current 2'b11 represents the maximum bias current"]
    #[inline(always)]
    #[must_use]
    pub fn bf43(&mut self) -> Bf43W<Reg04Spec> {
        Bf43W::new(self, 3)
    }
    #[doc = "Bit 5 - This bitfield is reserved."]
    #[inline(always)]
    #[must_use]
    pub fn bf55(&mut self) -> Bf55W<Reg04Spec> {
        Bf55W::new(self, 5)
    }
    #[doc = "Bits 6:7 - disconnect detector bias current tuning"]
    #[inline(always)]
    #[must_use]
    pub fn bf76(&mut self) -> Bf76W<Reg04Spec> {
        Bf76W::new(self, 6)
    }
}
#[doc = "Register description here\n\nYou can [`read`](crate::Reg::read) this register and get [`reg04::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg04::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reg04Spec;
impl crate::RegisterSpec for Reg04Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg04::R`](R) reader structure"]
impl crate::Readable for Reg04Spec {}
#[doc = "`write(|w| ..)` method takes [`reg04::W`](W) writer structure"]
impl crate::Writable for Reg04Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REG04 to value 0"]
impl crate::Resettable for Reg04Spec {
    const RESET_VALUE: u32 = 0;
}
