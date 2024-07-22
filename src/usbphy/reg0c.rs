#[doc = "Register `REG0C` reader"]
pub type R = crate::R<Reg0cSpec>;
#[doc = "Register `REG0C` writer"]
pub type W = crate::W<Reg0cSpec>;
#[doc = "Field `BF10` reader - BG output voltage reference adjust, normally these bits are recommended to be kept as the default values. 00: standard center level around 1.25v output, recommended 01: relative higher output 1x: relative higher output"]
pub type Bf10R = crate::FieldReader;
#[doc = "Field `BF10` writer - BG output voltage reference adjust, normally these bits are recommended to be kept as the default values. 00: standard center level around 1.25v output, recommended 01: relative higher output 1x: relative higher output"]
pub type Bf10W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BF62` reader - 45ohm HS ODT value tuning and FS/LS driver strength tuning 5'b11111: smallest HS ODT value and largest FS/LS driver strength and fastest FS/LS slew rate 5'b10000: biggest HS ODT value and smallest FS/LS driver strength and slowest FS/LS slew rate"]
pub type Bf62R = crate::FieldReader;
#[doc = "Field `BF62` writer - 45ohm HS ODT value tuning and FS/LS driver strength tuning 5'b11111: smallest HS ODT value and largest FS/LS driver strength and fastest FS/LS slew rate 5'b10000: biggest HS ODT value and smallest FS/LS driver strength and slowest FS/LS slew rate"]
pub type Bf62W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `BF77` reader - This bitfield is reserved."]
pub type Bf77R = crate::BitReader;
#[doc = "Field `BF77` writer - This bitfield is reserved."]
pub type Bf77W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - BG output voltage reference adjust, normally these bits are recommended to be kept as the default values. 00: standard center level around 1.25v output, recommended 01: relative higher output 1x: relative higher output"]
    #[inline(always)]
    pub fn bf10(&self) -> Bf10R {
        Bf10R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:6 - 45ohm HS ODT value tuning and FS/LS driver strength tuning 5'b11111: smallest HS ODT value and largest FS/LS driver strength and fastest FS/LS slew rate 5'b10000: biggest HS ODT value and smallest FS/LS driver strength and slowest FS/LS slew rate"]
    #[inline(always)]
    pub fn bf62(&self) -> Bf62R {
        Bf62R::new(((self.bits >> 2) & 0x1f) as u8)
    }
    #[doc = "Bit 7 - This bitfield is reserved."]
    #[inline(always)]
    pub fn bf77(&self) -> Bf77R {
        Bf77R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - BG output voltage reference adjust, normally these bits are recommended to be kept as the default values. 00: standard center level around 1.25v output, recommended 01: relative higher output 1x: relative higher output"]
    #[inline(always)]
    #[must_use]
    pub fn bf10(&mut self) -> Bf10W<Reg0cSpec> {
        Bf10W::new(self, 0)
    }
    #[doc = "Bits 2:6 - 45ohm HS ODT value tuning and FS/LS driver strength tuning 5'b11111: smallest HS ODT value and largest FS/LS driver strength and fastest FS/LS slew rate 5'b10000: biggest HS ODT value and smallest FS/LS driver strength and slowest FS/LS slew rate"]
    #[inline(always)]
    #[must_use]
    pub fn bf62(&mut self) -> Bf62W<Reg0cSpec> {
        Bf62W::new(self, 2)
    }
    #[doc = "Bit 7 - This bitfield is reserved."]
    #[inline(always)]
    #[must_use]
    pub fn bf77(&mut self) -> Bf77W<Reg0cSpec> {
        Bf77W::new(self, 7)
    }
}
#[doc = "Register description here\n\nYou can [`read`](crate::Reg::read) this register and get [`reg0c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg0c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reg0cSpec;
impl crate::RegisterSpec for Reg0cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg0c::R`](R) reader structure"]
impl crate::Readable for Reg0cSpec {}
#[doc = "`write(|w| ..)` method takes [`reg0c::W`](W) writer structure"]
impl crate::Writable for Reg0cSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REG0C to value 0"]
impl crate::Resettable for Reg0cSpec {
    const RESET_VALUE: u32 = 0;
}
