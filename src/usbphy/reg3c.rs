#[doc = "Register `REG3C` reader"]
pub type R = crate::R<Reg3cSpec>;
#[doc = "Register `REG3C` writer"]
pub type W = crate::W<Reg3cSpec>;
#[doc = "Field `BF10` reader - BF10 field description needed."]
pub type Bf10R = crate::FieldReader;
#[doc = "Field `BF10` writer - BF10 field description needed."]
pub type Bf10W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BF42` reader - This bitfield is reserved."]
pub type Bf42R = crate::FieldReader;
#[doc = "Field `BF42` writer - This bitfield is reserved."]
pub type Bf42W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BF75` reader - Host disconnect filter select. 3'b100:6 clocks(480M clock) 3'b101:8 clocks 3'b111:disconnect disable Other: invalid"]
pub type Bf75R = crate::FieldReader;
#[doc = "Field `BF75` writer - Host disconnect filter select. 3'b100:6 clocks(480M clock) 3'b101:8 clocks 3'b111:disconnect disable Other: invalid"]
pub type Bf75W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:1 - BF10 field description needed."]
    #[inline(always)]
    pub fn bf10(&self) -> Bf10R {
        Bf10R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:4 - This bitfield is reserved."]
    #[inline(always)]
    pub fn bf42(&self) -> Bf42R {
        Bf42R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 5:7 - Host disconnect filter select. 3'b100:6 clocks(480M clock) 3'b101:8 clocks 3'b111:disconnect disable Other: invalid"]
    #[inline(always)]
    pub fn bf75(&self) -> Bf75R {
        Bf75R::new(((self.bits >> 5) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - BF10 field description needed."]
    #[inline(always)]
    #[must_use]
    pub fn bf10(&mut self) -> Bf10W<Reg3cSpec> {
        Bf10W::new(self, 0)
    }
    #[doc = "Bits 2:4 - This bitfield is reserved."]
    #[inline(always)]
    #[must_use]
    pub fn bf42(&mut self) -> Bf42W<Reg3cSpec> {
        Bf42W::new(self, 2)
    }
    #[doc = "Bits 5:7 - Host disconnect filter select. 3'b100:6 clocks(480M clock) 3'b101:8 clocks 3'b111:disconnect disable Other: invalid"]
    #[inline(always)]
    #[must_use]
    pub fn bf75(&mut self) -> Bf75W<Reg3cSpec> {
        Bf75W::new(self, 5)
    }
}
#[doc = "Register description here\n\nYou can [`read`](crate::Reg::read) this register and get [`reg3c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg3c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reg3cSpec;
impl crate::RegisterSpec for Reg3cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg3c::R`](R) reader structure"]
impl crate::Readable for Reg3cSpec {}
#[doc = "`write(|w| ..)` method takes [`reg3c::W`](W) writer structure"]
impl crate::Writable for Reg3cSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REG3C to value 0"]
impl crate::Resettable for Reg3cSpec {
    const RESET_VALUE: u32 = 0;
}
