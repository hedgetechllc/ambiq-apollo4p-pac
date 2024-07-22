#[doc = "Register `REG1C` reader"]
pub type R = crate::R<Reg1cSpec>;
#[doc = "Register `REG1C` writer"]
pub type W = crate::W<Reg1cSpec>;
#[doc = "Field `BF00` reader - Set IO high-Z state. Active high"]
pub type Bf00R = crate::BitReader;
#[doc = "Field `BF00` writer - Set IO high-Z state. Active high"]
pub type Bf00W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BF11` reader - Tx power down in suspend state. Active low."]
pub type Bf11R = crate::BitReader;
#[doc = "Field `BF11` writer - Tx power down in suspend state. Active low."]
pub type Bf11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BF22` reader - PLL enable bypass from suspend module. 1'b1: bypass enable 1'b0:bypass disable"]
pub type Bf22R = crate::BitReader;
#[doc = "Field `BF22` writer - PLL enable bypass from suspend module. 1'b1: bypass enable 1'b0:bypass disable"]
pub type Bf22W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BF33` reader - PLL enable value from suspend module. 1'b1:pll enable"]
pub type Bf33R = crate::BitReader;
#[doc = "Field `BF33` writer - PLL enable value from suspend module. 1'b1:pll enable"]
pub type Bf33W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BF44` reader - 480M clock out enable. 1'b0:pll disable 1'b1: 480M clock out enable 1'b0: 480M clock out disable"]
pub type Bf44R = crate::BitReader;
#[doc = "Field `BF44` writer - 480M clock out enable. 1'b0:pll disable 1'b1: 480M clock out enable 1'b0: 480M clock out disable"]
pub type Bf44W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BF55` reader - BG power down control bit, active high 1: power down band-gap 0: normal operation mode"]
pub type Bf55R = crate::BitReader;
#[doc = "Field `BF55` writer - BG power down control bit, active high 1: power down band-gap 0: normal operation mode"]
pub type Bf55W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BF66` reader - This bitfield is reserved."]
pub type Bf66R = crate::BitReader;
#[doc = "Field `BF66` writer - This bitfield is reserved."]
pub type Bf66W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BF77` reader - This bitfield is reserved."]
pub type Bf77R = crate::BitReader;
#[doc = "Field `BF77` writer - This bitfield is reserved."]
pub type Bf77W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set IO high-Z state. Active high"]
    #[inline(always)]
    pub fn bf00(&self) -> Bf00R {
        Bf00R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Tx power down in suspend state. Active low."]
    #[inline(always)]
    pub fn bf11(&self) -> Bf11R {
        Bf11R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PLL enable bypass from suspend module. 1'b1: bypass enable 1'b0:bypass disable"]
    #[inline(always)]
    pub fn bf22(&self) -> Bf22R {
        Bf22R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PLL enable value from suspend module. 1'b1:pll enable"]
    #[inline(always)]
    pub fn bf33(&self) -> Bf33R {
        Bf33R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 480M clock out enable. 1'b0:pll disable 1'b1: 480M clock out enable 1'b0: 480M clock out disable"]
    #[inline(always)]
    pub fn bf44(&self) -> Bf44R {
        Bf44R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - BG power down control bit, active high 1: power down band-gap 0: normal operation mode"]
    #[inline(always)]
    pub fn bf55(&self) -> Bf55R {
        Bf55R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This bitfield is reserved."]
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
    #[doc = "Bit 0 - Set IO high-Z state. Active high"]
    #[inline(always)]
    #[must_use]
    pub fn bf00(&mut self) -> Bf00W<Reg1cSpec> {
        Bf00W::new(self, 0)
    }
    #[doc = "Bit 1 - Tx power down in suspend state. Active low."]
    #[inline(always)]
    #[must_use]
    pub fn bf11(&mut self) -> Bf11W<Reg1cSpec> {
        Bf11W::new(self, 1)
    }
    #[doc = "Bit 2 - PLL enable bypass from suspend module. 1'b1: bypass enable 1'b0:bypass disable"]
    #[inline(always)]
    #[must_use]
    pub fn bf22(&mut self) -> Bf22W<Reg1cSpec> {
        Bf22W::new(self, 2)
    }
    #[doc = "Bit 3 - PLL enable value from suspend module. 1'b1:pll enable"]
    #[inline(always)]
    #[must_use]
    pub fn bf33(&mut self) -> Bf33W<Reg1cSpec> {
        Bf33W::new(self, 3)
    }
    #[doc = "Bit 4 - 480M clock out enable. 1'b0:pll disable 1'b1: 480M clock out enable 1'b0: 480M clock out disable"]
    #[inline(always)]
    #[must_use]
    pub fn bf44(&mut self) -> Bf44W<Reg1cSpec> {
        Bf44W::new(self, 4)
    }
    #[doc = "Bit 5 - BG power down control bit, active high 1: power down band-gap 0: normal operation mode"]
    #[inline(always)]
    #[must_use]
    pub fn bf55(&mut self) -> Bf55W<Reg1cSpec> {
        Bf55W::new(self, 5)
    }
    #[doc = "Bit 6 - This bitfield is reserved."]
    #[inline(always)]
    #[must_use]
    pub fn bf66(&mut self) -> Bf66W<Reg1cSpec> {
        Bf66W::new(self, 6)
    }
    #[doc = "Bit 7 - This bitfield is reserved."]
    #[inline(always)]
    #[must_use]
    pub fn bf77(&mut self) -> Bf77W<Reg1cSpec> {
        Bf77W::new(self, 7)
    }
}
#[doc = "Register description here\n\nYou can [`read`](crate::Reg::read) this register and get [`reg1c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg1c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reg1cSpec;
impl crate::RegisterSpec for Reg1cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg1c::R`](R) reader structure"]
impl crate::Readable for Reg1cSpec {}
#[doc = "`write(|w| ..)` method takes [`reg1c::W`](W) writer structure"]
impl crate::Writable for Reg1cSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REG1C to value 0"]
impl crate::Resettable for Reg1cSpec {
    const RESET_VALUE: u32 = 0;
}
