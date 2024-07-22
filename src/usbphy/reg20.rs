#[doc = "Register `REG20` reader"]
pub type R = crate::R<Reg20Spec>;
#[doc = "Register `REG20` writer"]
pub type W = crate::W<Reg20Spec>;
#[doc = "Field `BF20` reader - Rx enable delay select. 3'b000: 4 clocks (480Mhz clock) 3'b001: 5 clocks 3'b010: 6 clocks 3'b011: 7 clocks 3'b100: 8 clocks 3'b101: 9 clocks 3'b110: 10 clocks 3'b111: 12 clocks"]
pub type Bf20R = crate::FieldReader;
#[doc = "Field `BF20` writer - Rx enable delay select. 3'b000: 4 clocks (480Mhz clock) 3'b001: 5 clocks 3'b010: 6 clocks 3'b011: 7 clocks 3'b100: 8 clocks 3'b101: 9 clocks 3'b110: 10 clocks 3'b111: 12 clocks"]
pub type Bf20W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BF33` reader - This bitfield is reserved."]
pub type Bf33R = crate::BitReader;
#[doc = "Field `BF33` writer - This bitfield is reserved."]
pub type Bf33W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BF54` reader - Analog observation port select. for detailed information, please refer to section 10.3 , Table 30 : Debug and OBS port"]
pub type Bf54R = crate::FieldReader;
#[doc = "Field `BF54` writer - Analog observation port select. for detailed information, please refer to section 10.3 , Table 30 : Debug and OBS port"]
pub type Bf54W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BF76` reader - This bitfield is reserved."]
pub type Bf76R = crate::FieldReader;
#[doc = "Field `BF76` writer - This bitfield is reserved."]
pub type Bf76W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:2 - Rx enable delay select. 3'b000: 4 clocks (480Mhz clock) 3'b001: 5 clocks 3'b010: 6 clocks 3'b011: 7 clocks 3'b100: 8 clocks 3'b101: 9 clocks 3'b110: 10 clocks 3'b111: 12 clocks"]
    #[inline(always)]
    pub fn bf20(&self) -> Bf20R {
        Bf20R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - This bitfield is reserved."]
    #[inline(always)]
    pub fn bf33(&self) -> Bf33R {
        Bf33R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Analog observation port select. for detailed information, please refer to section 10.3 , Table 30 : Debug and OBS port"]
    #[inline(always)]
    pub fn bf54(&self) -> Bf54R {
        Bf54R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - This bitfield is reserved."]
    #[inline(always)]
    pub fn bf76(&self) -> Bf76R {
        Bf76R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Rx enable delay select. 3'b000: 4 clocks (480Mhz clock) 3'b001: 5 clocks 3'b010: 6 clocks 3'b011: 7 clocks 3'b100: 8 clocks 3'b101: 9 clocks 3'b110: 10 clocks 3'b111: 12 clocks"]
    #[inline(always)]
    #[must_use]
    pub fn bf20(&mut self) -> Bf20W<Reg20Spec> {
        Bf20W::new(self, 0)
    }
    #[doc = "Bit 3 - This bitfield is reserved."]
    #[inline(always)]
    #[must_use]
    pub fn bf33(&mut self) -> Bf33W<Reg20Spec> {
        Bf33W::new(self, 3)
    }
    #[doc = "Bits 4:5 - Analog observation port select. for detailed information, please refer to section 10.3 , Table 30 : Debug and OBS port"]
    #[inline(always)]
    #[must_use]
    pub fn bf54(&mut self) -> Bf54W<Reg20Spec> {
        Bf54W::new(self, 4)
    }
    #[doc = "Bits 6:7 - This bitfield is reserved."]
    #[inline(always)]
    #[must_use]
    pub fn bf76(&mut self) -> Bf76W<Reg20Spec> {
        Bf76W::new(self, 6)
    }
}
#[doc = "Register description here\n\nYou can [`read`](crate::Reg::read) this register and get [`reg20::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg20::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reg20Spec;
impl crate::RegisterSpec for Reg20Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg20::R`](R) reader structure"]
impl crate::Readable for Reg20Spec {}
#[doc = "`write(|w| ..)` method takes [`reg20::W`](W) writer structure"]
impl crate::Writable for Reg20Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REG20 to value 0"]
impl crate::Resettable for Reg20Spec {
    const RESET_VALUE: u32 = 0;
}
