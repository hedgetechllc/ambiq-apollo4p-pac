#[doc = "Register `REG2C` reader"]
pub type R = crate::R<Reg2cSpec>;
#[doc = "Register `REG2C` writer"]
pub type W = crate::W<Reg2cSpec>;
#[doc = "Field `BF00` reader - All port z bypass value. 1'b1: bypass enable 1'b0:bypass disable"]
pub type Bf00R = crate::BitReader;
#[doc = "Field `BF00` writer - All port z bypass value. 1'b1: bypass enable 1'b0:bypass disable"]
pub type Bf00W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BF11` reader - This bitfield is reserved."]
pub type Bf11R = crate::BitReader;
#[doc = "Field `BF11` writer - This bitfield is reserved."]
pub type Bf11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BF22` reader - HS keep alive enable. 1'b1: HS keep alive enable 1'b0: HS keep alive disable"]
pub type Bf22R = crate::BitReader;
#[doc = "Field `BF22` writer - HS keep alive enable. 1'b1: HS keep alive enable 1'b0: HS keep alive disable"]
pub type Bf22W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BF33` reader - This bitfield is reserved."]
pub type Bf33R = crate::BitReader;
#[doc = "Field `BF33` writer - This bitfield is reserved."]
pub type Bf33W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BF44` reader - This bitfield is reserved."]
pub type Bf44R = crate::BitReader;
#[doc = "Field `BF44` writer - This bitfield is reserved."]
pub type Bf44W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BF75` reader - This bitfield is reserved."]
pub type Bf75R = crate::FieldReader;
#[doc = "Field `BF75` writer - This bitfield is reserved."]
pub type Bf75W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - All port z bypass value. 1'b1: bypass enable 1'b0:bypass disable"]
    #[inline(always)]
    pub fn bf00(&self) -> Bf00R {
        Bf00R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bitfield is reserved."]
    #[inline(always)]
    pub fn bf11(&self) -> Bf11R {
        Bf11R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HS keep alive enable. 1'b1: HS keep alive enable 1'b0: HS keep alive disable"]
    #[inline(always)]
    pub fn bf22(&self) -> Bf22R {
        Bf22R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This bitfield is reserved."]
    #[inline(always)]
    pub fn bf33(&self) -> Bf33R {
        Bf33R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This bitfield is reserved."]
    #[inline(always)]
    pub fn bf44(&self) -> Bf44R {
        Bf44R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - This bitfield is reserved."]
    #[inline(always)]
    pub fn bf75(&self) -> Bf75R {
        Bf75R::new(((self.bits >> 5) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - All port z bypass value. 1'b1: bypass enable 1'b0:bypass disable"]
    #[inline(always)]
    #[must_use]
    pub fn bf00(&mut self) -> Bf00W<Reg2cSpec> {
        Bf00W::new(self, 0)
    }
    #[doc = "Bit 1 - This bitfield is reserved."]
    #[inline(always)]
    #[must_use]
    pub fn bf11(&mut self) -> Bf11W<Reg2cSpec> {
        Bf11W::new(self, 1)
    }
    #[doc = "Bit 2 - HS keep alive enable. 1'b1: HS keep alive enable 1'b0: HS keep alive disable"]
    #[inline(always)]
    #[must_use]
    pub fn bf22(&mut self) -> Bf22W<Reg2cSpec> {
        Bf22W::new(self, 2)
    }
    #[doc = "Bit 3 - This bitfield is reserved."]
    #[inline(always)]
    #[must_use]
    pub fn bf33(&mut self) -> Bf33W<Reg2cSpec> {
        Bf33W::new(self, 3)
    }
    #[doc = "Bit 4 - This bitfield is reserved."]
    #[inline(always)]
    #[must_use]
    pub fn bf44(&mut self) -> Bf44W<Reg2cSpec> {
        Bf44W::new(self, 4)
    }
    #[doc = "Bits 5:7 - This bitfield is reserved."]
    #[inline(always)]
    #[must_use]
    pub fn bf75(&mut self) -> Bf75W<Reg2cSpec> {
        Bf75W::new(self, 5)
    }
}
#[doc = "Register description here\n\nYou can [`read`](crate::Reg::read) this register and get [`reg2c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg2c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reg2cSpec;
impl crate::RegisterSpec for Reg2cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg2c::R`](R) reader structure"]
impl crate::Readable for Reg2cSpec {}
#[doc = "`write(|w| ..)` method takes [`reg2c::W`](W) writer structure"]
impl crate::Writable for Reg2cSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REG2C to value 0"]
impl crate::Resettable for Reg2cSpec {
    const RESET_VALUE: u32 = 0;
}
