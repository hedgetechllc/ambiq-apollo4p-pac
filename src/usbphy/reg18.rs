#[doc = "Register `REG18` reader"]
pub type R = crate::R<Reg18Spec>;
#[doc = "Register `REG18` writer"]
pub type W = crate::W<Reg18Spec>;
#[doc = "Field `BF10` reader - HS receiver bias current tuning. 2'b00 represents the minimum bias current 2'b11 represents the maximum bias current"]
pub type Bf10R = crate::FieldReader;
#[doc = "Field `BF10` writer - HS receiver bias current tuning. 2'b00 represents the minimum bias current 2'b11 represents the maximum bias current"]
pub type Bf10W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BF22` reader - Clk60m, clk12m and clk48m enable. 1'b0:disables the clocks 1'b1:enables the clocks"]
pub type Bf22R = crate::BitReader;
#[doc = "Field `BF22` writer - Clk60m, clk12m and clk48m enable. 1'b0:disables the clocks 1'b1:enables the clocks"]
pub type Bf22W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BF73` reader - This bitfield is reserved."]
pub type Bf73R = crate::FieldReader;
#[doc = "Field `BF73` writer - This bitfield is reserved."]
pub type Bf73W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:1 - HS receiver bias current tuning. 2'b00 represents the minimum bias current 2'b11 represents the maximum bias current"]
    #[inline(always)]
    pub fn bf10(&self) -> Bf10R {
        Bf10R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Clk60m, clk12m and clk48m enable. 1'b0:disables the clocks 1'b1:enables the clocks"]
    #[inline(always)]
    pub fn bf22(&self) -> Bf22R {
        Bf22R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:7 - This bitfield is reserved."]
    #[inline(always)]
    pub fn bf73(&self) -> Bf73R {
        Bf73R::new(((self.bits >> 3) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - HS receiver bias current tuning. 2'b00 represents the minimum bias current 2'b11 represents the maximum bias current"]
    #[inline(always)]
    #[must_use]
    pub fn bf10(&mut self) -> Bf10W<Reg18Spec> {
        Bf10W::new(self, 0)
    }
    #[doc = "Bit 2 - Clk60m, clk12m and clk48m enable. 1'b0:disables the clocks 1'b1:enables the clocks"]
    #[inline(always)]
    #[must_use]
    pub fn bf22(&mut self) -> Bf22W<Reg18Spec> {
        Bf22W::new(self, 2)
    }
    #[doc = "Bits 3:7 - This bitfield is reserved."]
    #[inline(always)]
    #[must_use]
    pub fn bf73(&mut self) -> Bf73W<Reg18Spec> {
        Bf73W::new(self, 3)
    }
}
#[doc = "Register description here\n\nYou can [`read`](crate::Reg::read) this register and get [`reg18::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg18::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reg18Spec;
impl crate::RegisterSpec for Reg18Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg18::R`](R) reader structure"]
impl crate::Readable for Reg18Spec {}
#[doc = "`write(|w| ..)` method takes [`reg18::W`](W) writer structure"]
impl crate::Writable for Reg18Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REG18 to value 0"]
impl crate::Resettable for Reg18Spec {
    const RESET_VALUE: u32 = 0;
}
