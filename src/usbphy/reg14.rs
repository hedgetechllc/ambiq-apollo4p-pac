#[doc = "Register `REG14` reader"]
pub type R = crate::R<Reg14Spec>;
#[doc = "Register `REG14` writer"]
pub type W = crate::W<Reg14Spec>;
#[doc = "Field `BF00` reader - Dflop output select signal delay compared with digital clock enable signal 1'b0:3 clocks 1'b1:2 clocks"]
pub type Bf00R = crate::BitReader;
#[doc = "Field `BF00` writer - Dflop output select signal delay compared with digital clock enable signal 1'b0:3 clocks 1'b1:2 clocks"]
pub type Bf00W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BF11` reader - PLL bandwidth option 1'b0 default 1'b1 increases the PLL bandwidth"]
pub type Bf11R = crate::BitReader;
#[doc = "Field `BF11` writer - PLL bandwidth option 1'b0 default 1'b1 increases the PLL bandwidth"]
pub type Bf11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BF42` reader - Tx HS pre-emphasis strength 3'b111 represents the strongest , 3'b000 the weakest"]
pub type Bf42R = crate::FieldReader;
#[doc = "Field `BF42` writer - Tx HS pre-emphasis strength 3'b111 represents the strongest , 3'b000 the weakest"]
pub type Bf42W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BF55` reader - PLL feedback divider ratio option."]
pub type Bf55R = crate::BitReader;
#[doc = "Field `BF55` writer - PLL feedback divider ratio option."]
pub type Bf55W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BF66` reader - BF66 field description needed."]
pub type Bf66R = crate::BitReader;
#[doc = "Field `BF66` writer - BF66 field description needed."]
pub type Bf66W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BF77` reader - This bitfield is reserved."]
pub type Bf77R = crate::BitReader;
#[doc = "Field `BF77` writer - This bitfield is reserved."]
pub type Bf77W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Dflop output select signal delay compared with digital clock enable signal 1'b0:3 clocks 1'b1:2 clocks"]
    #[inline(always)]
    pub fn bf00(&self) -> Bf00R {
        Bf00R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PLL bandwidth option 1'b0 default 1'b1 increases the PLL bandwidth"]
    #[inline(always)]
    pub fn bf11(&self) -> Bf11R {
        Bf11R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - Tx HS pre-emphasis strength 3'b111 represents the strongest , 3'b000 the weakest"]
    #[inline(always)]
    pub fn bf42(&self) -> Bf42R {
        Bf42R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 5 - PLL feedback divider ratio option."]
    #[inline(always)]
    pub fn bf55(&self) -> Bf55R {
        Bf55R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - BF66 field description needed."]
    #[inline(always)]
    pub fn bf66(&self) -> Bf66R {
        Bf66R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This bitfield is reserved."]
    #[inline(always)]
    pub fn bf77(&self) -> Bf77R {
        Bf77R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Dflop output select signal delay compared with digital clock enable signal 1'b0:3 clocks 1'b1:2 clocks"]
    #[inline(always)]
    #[must_use]
    pub fn bf00(&mut self) -> Bf00W<Reg14Spec> {
        Bf00W::new(self, 0)
    }
    #[doc = "Bit 1 - PLL bandwidth option 1'b0 default 1'b1 increases the PLL bandwidth"]
    #[inline(always)]
    #[must_use]
    pub fn bf11(&mut self) -> Bf11W<Reg14Spec> {
        Bf11W::new(self, 1)
    }
    #[doc = "Bits 2:4 - Tx HS pre-emphasis strength 3'b111 represents the strongest , 3'b000 the weakest"]
    #[inline(always)]
    #[must_use]
    pub fn bf42(&mut self) -> Bf42W<Reg14Spec> {
        Bf42W::new(self, 2)
    }
    #[doc = "Bit 5 - PLL feedback divider ratio option."]
    #[inline(always)]
    #[must_use]
    pub fn bf55(&mut self) -> Bf55W<Reg14Spec> {
        Bf55W::new(self, 5)
    }
    #[doc = "Bit 6 - BF66 field description needed."]
    #[inline(always)]
    #[must_use]
    pub fn bf66(&mut self) -> Bf66W<Reg14Spec> {
        Bf66W::new(self, 6)
    }
    #[doc = "Bit 7 - This bitfield is reserved."]
    #[inline(always)]
    #[must_use]
    pub fn bf77(&mut self) -> Bf77W<Reg14Spec> {
        Bf77W::new(self, 7)
    }
}
#[doc = "Register description here\n\nYou can [`read`](crate::Reg::read) this register and get [`reg14::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg14::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reg14Spec;
impl crate::RegisterSpec for Reg14Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg14::R`](R) reader structure"]
impl crate::Readable for Reg14Spec {}
#[doc = "`write(|w| ..)` method takes [`reg14::W`](W) writer structure"]
impl crate::Writable for Reg14Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REG14 to value 0"]
impl crate::Resettable for Reg14Spec {
    const RESET_VALUE: u32 = 0;
}
