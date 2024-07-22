#[doc = "Register `REG7C` reader"]
pub type R = crate::R<Reg7cSpec>;
#[doc = "Register `REG7C` writer"]
pub type W = crate::W<Reg7cSpec>;
#[doc = "Field `BF40` reader - This bitfield is reserved."]
pub type Bf40R = crate::FieldReader;
#[doc = "Field `BF40` writer - This bitfield is reserved."]
pub type Bf40W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `BF55` reader - No leakage current on DP/DM pin when VCCA3P3 power down, active low. Keeping the default value was greatly appreciated"]
pub type Bf55R = crate::BitReader;
#[doc = "Field `BF55` writer - No leakage current on DP/DM pin when VCCA3P3 power down, active low. Keeping the default value was greatly appreciated"]
pub type Bf55W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BF66` reader - Hs chirp mode amplitude increasing register, active high."]
pub type Bf66R = crate::BitReader;
#[doc = "Field `BF66` writer - Hs chirp mode amplitude increasing register, active high."]
pub type Bf66W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BF77` reader - Clk60m source clock select. 1'b1: free clock 60M 1'b0: utmi_clk"]
pub type Bf77R = crate::BitReader;
#[doc = "Field `BF77` writer - Clk60m source clock select. 1'b1: free clock 60M 1'b0: utmi_clk"]
pub type Bf77W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - This bitfield is reserved."]
    #[inline(always)]
    pub fn bf40(&self) -> Bf40R {
        Bf40R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - No leakage current on DP/DM pin when VCCA3P3 power down, active low. Keeping the default value was greatly appreciated"]
    #[inline(always)]
    pub fn bf55(&self) -> Bf55R {
        Bf55R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Hs chirp mode amplitude increasing register, active high."]
    #[inline(always)]
    pub fn bf66(&self) -> Bf66R {
        Bf66R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Clk60m source clock select. 1'b1: free clock 60M 1'b0: utmi_clk"]
    #[inline(always)]
    pub fn bf77(&self) -> Bf77R {
        Bf77R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - This bitfield is reserved."]
    #[inline(always)]
    #[must_use]
    pub fn bf40(&mut self) -> Bf40W<Reg7cSpec> {
        Bf40W::new(self, 0)
    }
    #[doc = "Bit 5 - No leakage current on DP/DM pin when VCCA3P3 power down, active low. Keeping the default value was greatly appreciated"]
    #[inline(always)]
    #[must_use]
    pub fn bf55(&mut self) -> Bf55W<Reg7cSpec> {
        Bf55W::new(self, 5)
    }
    #[doc = "Bit 6 - Hs chirp mode amplitude increasing register, active high."]
    #[inline(always)]
    #[must_use]
    pub fn bf66(&mut self) -> Bf66W<Reg7cSpec> {
        Bf66W::new(self, 6)
    }
    #[doc = "Bit 7 - Clk60m source clock select. 1'b1: free clock 60M 1'b0: utmi_clk"]
    #[inline(always)]
    #[must_use]
    pub fn bf77(&mut self) -> Bf77W<Reg7cSpec> {
        Bf77W::new(self, 7)
    }
}
#[doc = "Register description here\n\nYou can [`read`](crate::Reg::read) this register and get [`reg7c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg7c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reg7cSpec;
impl crate::RegisterSpec for Reg7cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg7c::R`](R) reader structure"]
impl crate::Readable for Reg7cSpec {}
#[doc = "`write(|w| ..)` method takes [`reg7c::W`](W) writer structure"]
impl crate::Writable for Reg7cSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REG7C to value 0"]
impl crate::Resettable for Reg7cSpec {
    const RESET_VALUE: u32 = 0;
}
