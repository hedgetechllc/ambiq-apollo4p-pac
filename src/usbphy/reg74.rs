#[doc = "Register `REG74` reader"]
pub type R = crate::R<Reg74Spec>;
#[doc = "Register `REG74` writer"]
pub type W = crate::W<Reg74Spec>;
#[doc = "Field `BF00` reader - Disconnect detection block input res load sel. 1'b0: disconnect detection block input res load bypass 1'b1: disconnect detection block input res load enable"]
pub type Bf00R = crate::BitReader;
#[doc = "Field `BF00` writer - Disconnect detection block input res load sel. 1'b0: disconnect detection block input res load bypass 1'b1: disconnect detection block input res load enable"]
pub type Bf00W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BF31` reader - HS driver slew rate tuning 001:SR is weakest 111:SR is strongest.000 is forbidden."]
pub type Bf31R = crate::FieldReader;
#[doc = "Field `BF31` writer - HS driver slew rate tuning 001:SR is weakest 111:SR is strongest.000 is forbidden."]
pub type Bf31W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BF74` reader - This bitfield is reserved."]
pub type Bf74R = crate::FieldReader;
#[doc = "Field `BF74` writer - This bitfield is reserved."]
pub type Bf74W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Disconnect detection block input res load sel. 1'b0: disconnect detection block input res load bypass 1'b1: disconnect detection block input res load enable"]
    #[inline(always)]
    pub fn bf00(&self) -> Bf00R {
        Bf00R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - HS driver slew rate tuning 001:SR is weakest 111:SR is strongest.000 is forbidden."]
    #[inline(always)]
    pub fn bf31(&self) -> Bf31R {
        Bf31R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 4:7 - This bitfield is reserved."]
    #[inline(always)]
    pub fn bf74(&self) -> Bf74R {
        Bf74R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Disconnect detection block input res load sel. 1'b0: disconnect detection block input res load bypass 1'b1: disconnect detection block input res load enable"]
    #[inline(always)]
    #[must_use]
    pub fn bf00(&mut self) -> Bf00W<Reg74Spec> {
        Bf00W::new(self, 0)
    }
    #[doc = "Bits 1:3 - HS driver slew rate tuning 001:SR is weakest 111:SR is strongest.000 is forbidden."]
    #[inline(always)]
    #[must_use]
    pub fn bf31(&mut self) -> Bf31W<Reg74Spec> {
        Bf31W::new(self, 1)
    }
    #[doc = "Bits 4:7 - This bitfield is reserved."]
    #[inline(always)]
    #[must_use]
    pub fn bf74(&mut self) -> Bf74W<Reg74Spec> {
        Bf74W::new(self, 4)
    }
}
#[doc = "Register description here\n\nYou can [`read`](crate::Reg::read) this register and get [`reg74::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg74::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reg74Spec;
impl crate::RegisterSpec for Reg74Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg74::R`](R) reader structure"]
impl crate::Readable for Reg74Spec {}
#[doc = "`write(|w| ..)` method takes [`reg74::W`](W) writer structure"]
impl crate::Writable for Reg74Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REG74 to value 0"]
impl crate::Resettable for Reg74Spec {
    const RESET_VALUE: u32 = 0;
}
