#[doc = "Register `REG00` reader"]
pub type R = crate::R<Reg00Spec>;
#[doc = "Register `REG00` writer"]
pub type W = crate::W<Reg00Spec>;
#[doc = "Field `BF20` reader - This bitfield is reserved."]
pub type Bf20R = crate::FieldReader;
#[doc = "Field `BF20` writer - This bitfield is reserved."]
pub type Bf20W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BF43` reader - BG observation enable signal. Active low. When enabled, vref 400mV can be observed through USB0PP/USB0PN"]
pub type Bf43R = crate::FieldReader;
#[doc = "Field `BF43` writer - BG observation enable signal. Active low. When enabled, vref 400mV can be observed through USB0PP/USB0PN"]
pub type Bf43W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BF75` reader - Manually set the Rx Clock phase select. These bits will tune the HS RX path sample timing between digital and analog inside PHY: 3'b000 represents the earliest phase 3'b111 represents the latest phase The delay associated with each step is 256ps"]
pub type Bf75R = crate::FieldReader;
#[doc = "Field `BF75` writer - Manually set the Rx Clock phase select. These bits will tune the HS RX path sample timing between digital and analog inside PHY: 3'b000 represents the earliest phase 3'b111 represents the latest phase The delay associated with each step is 256ps"]
pub type Bf75W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - This bitfield is reserved."]
    #[inline(always)]
    pub fn bf20(&self) -> Bf20R {
        Bf20R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:4 - BG observation enable signal. Active low. When enabled, vref 400mV can be observed through USB0PP/USB0PN"]
    #[inline(always)]
    pub fn bf43(&self) -> Bf43R {
        Bf43R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:7 - Manually set the Rx Clock phase select. These bits will tune the HS RX path sample timing between digital and analog inside PHY: 3'b000 represents the earliest phase 3'b111 represents the latest phase The delay associated with each step is 256ps"]
    #[inline(always)]
    pub fn bf75(&self) -> Bf75R {
        Bf75R::new(((self.bits >> 5) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - This bitfield is reserved."]
    #[inline(always)]
    #[must_use]
    pub fn bf20(&mut self) -> Bf20W<Reg00Spec> {
        Bf20W::new(self, 0)
    }
    #[doc = "Bits 3:4 - BG observation enable signal. Active low. When enabled, vref 400mV can be observed through USB0PP/USB0PN"]
    #[inline(always)]
    #[must_use]
    pub fn bf43(&mut self) -> Bf43W<Reg00Spec> {
        Bf43W::new(self, 3)
    }
    #[doc = "Bits 5:7 - Manually set the Rx Clock phase select. These bits will tune the HS RX path sample timing between digital and analog inside PHY: 3'b000 represents the earliest phase 3'b111 represents the latest phase The delay associated with each step is 256ps"]
    #[inline(always)]
    #[must_use]
    pub fn bf75(&mut self) -> Bf75W<Reg00Spec> {
        Bf75W::new(self, 5)
    }
}
#[doc = "Register description here\n\nYou can [`read`](crate::Reg::read) this register and get [`reg00::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg00::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reg00Spec;
impl crate::RegisterSpec for Reg00Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg00::R`](R) reader structure"]
impl crate::Readable for Reg00Spec {}
#[doc = "`write(|w| ..)` method takes [`reg00::W`](W) writer structure"]
impl crate::Writable for Reg00Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REG00 to value 0"]
impl crate::Resettable for Reg00Spec {
    const RESET_VALUE: u32 = 0;
}
