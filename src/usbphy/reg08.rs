#[doc = "Register `REG08` reader"]
pub type R = crate::R<Reg08Spec>;
#[doc = "Register `REG08` writer"]
pub type W = crate::W<Reg08Spec>;
#[doc = "Field `BF30` reader - 2'b00 represents the minimum bias current 2'b11 represents the maximum bias current Rx squelch trigger point configures. Allows tuning of the squelch trigger point in order to compensate for package and board level parasitic. 4'b0000:112.5mV 4'b0001:150mV 4'b0010:87.5mV 4'b0011:162.5mV 4'b0100:100mV 4'b0101:137.5mV 4'b0110:75mV 4'b0111:150mV 4'b1000:125mV 4'b1001:162.5mV 4'b1010:100mV 4'b1011:175mV 4'b1100:150mV(default) 4'b1101:187.5mV 4'b1110:125mV 4'b1111:200mV"]
pub type Bf30R = crate::FieldReader;
#[doc = "Field `BF30` writer - 2'b00 represents the minimum bias current 2'b11 represents the maximum bias current Rx squelch trigger point configures. Allows tuning of the squelch trigger point in order to compensate for package and board level parasitic. 4'b0000:112.5mV 4'b0001:150mV 4'b0010:87.5mV 4'b0011:162.5mV 4'b0100:100mV 4'b0101:137.5mV 4'b0110:75mV 4'b0111:150mV 4'b1000:125mV 4'b1001:162.5mV 4'b1010:100mV 4'b1011:175mV 4'b1100:150mV(default) 4'b1101:187.5mV 4'b1110:125mV 4'b1111:200mV"]
pub type Bf30W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BF64` reader - HS eye height tuning 3'b000:400mV(default) 3'b001:475mV 3'b010:350mV 3'b011:500mV 3'b100:412.5mV 3'b101:425mV 3'b110:437.5mV 3'b111:450mV"]
pub type Bf64R = crate::FieldReader;
#[doc = "Field `BF64` writer - HS eye height tuning 3'b000:400mV(default) 3'b001:475mV 3'b010:350mV 3'b011:500mV 3'b100:412.5mV 3'b101:425mV 3'b110:437.5mV 3'b111:450mV"]
pub type Bf64W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BF77` reader - digital squelch filter select, this bit is used to filter the glitch on the HS RX squelch signal. 1: 1 clock cycle filter 0: 2 clock cycle fitter"]
pub type Bf77R = crate::BitReader;
#[doc = "Field `BF77` writer - digital squelch filter select, this bit is used to filter the glitch on the HS RX squelch signal. 1: 1 clock cycle filter 0: 2 clock cycle fitter"]
pub type Bf77W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - 2'b00 represents the minimum bias current 2'b11 represents the maximum bias current Rx squelch trigger point configures. Allows tuning of the squelch trigger point in order to compensate for package and board level parasitic. 4'b0000:112.5mV 4'b0001:150mV 4'b0010:87.5mV 4'b0011:162.5mV 4'b0100:100mV 4'b0101:137.5mV 4'b0110:75mV 4'b0111:150mV 4'b1000:125mV 4'b1001:162.5mV 4'b1010:100mV 4'b1011:175mV 4'b1100:150mV(default) 4'b1101:187.5mV 4'b1110:125mV 4'b1111:200mV"]
    #[inline(always)]
    pub fn bf30(&self) -> Bf30R {
        Bf30R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - HS eye height tuning 3'b000:400mV(default) 3'b001:475mV 3'b010:350mV 3'b011:500mV 3'b100:412.5mV 3'b101:425mV 3'b110:437.5mV 3'b111:450mV"]
    #[inline(always)]
    pub fn bf64(&self) -> Bf64R {
        Bf64R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - digital squelch filter select, this bit is used to filter the glitch on the HS RX squelch signal. 1: 1 clock cycle filter 0: 2 clock cycle fitter"]
    #[inline(always)]
    pub fn bf77(&self) -> Bf77R {
        Bf77R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 2'b00 represents the minimum bias current 2'b11 represents the maximum bias current Rx squelch trigger point configures. Allows tuning of the squelch trigger point in order to compensate for package and board level parasitic. 4'b0000:112.5mV 4'b0001:150mV 4'b0010:87.5mV 4'b0011:162.5mV 4'b0100:100mV 4'b0101:137.5mV 4'b0110:75mV 4'b0111:150mV 4'b1000:125mV 4'b1001:162.5mV 4'b1010:100mV 4'b1011:175mV 4'b1100:150mV(default) 4'b1101:187.5mV 4'b1110:125mV 4'b1111:200mV"]
    #[inline(always)]
    #[must_use]
    pub fn bf30(&mut self) -> Bf30W<Reg08Spec> {
        Bf30W::new(self, 0)
    }
    #[doc = "Bits 4:6 - HS eye height tuning 3'b000:400mV(default) 3'b001:475mV 3'b010:350mV 3'b011:500mV 3'b100:412.5mV 3'b101:425mV 3'b110:437.5mV 3'b111:450mV"]
    #[inline(always)]
    #[must_use]
    pub fn bf64(&mut self) -> Bf64W<Reg08Spec> {
        Bf64W::new(self, 4)
    }
    #[doc = "Bit 7 - digital squelch filter select, this bit is used to filter the glitch on the HS RX squelch signal. 1: 1 clock cycle filter 0: 2 clock cycle fitter"]
    #[inline(always)]
    #[must_use]
    pub fn bf77(&mut self) -> Bf77W<Reg08Spec> {
        Bf77W::new(self, 7)
    }
}
#[doc = "Register description here\n\nYou can [`read`](crate::Reg::read) this register and get [`reg08::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg08::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reg08Spec;
impl crate::RegisterSpec for Reg08Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg08::R`](R) reader structure"]
impl crate::Readable for Reg08Spec {}
#[doc = "`write(|w| ..)` method takes [`reg08::W`](W) writer structure"]
impl crate::Writable for Reg08Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REG08 to value 0"]
impl crate::Resettable for Reg08Spec {
    const RESET_VALUE: u32 = 0;
}
