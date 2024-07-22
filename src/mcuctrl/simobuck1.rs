#[doc = "Register `SIMOBUCK1` reader"]
pub type R = crate::R<Simobuck1Spec>;
#[doc = "Register `SIMOBUCK1` writer"]
pub type W = crate::W<Simobuck1Spec>;
#[doc = "Field `RXCLKACTTRIM` reader - This divides the 5 MHz refresh clock. Even divides are supported only. This value represents the division amount minus 1."]
pub type RxclkacttrimR = crate::FieldReader;
#[doc = "Field `RXCLKACTTRIM` writer - This divides the 5 MHz refresh clock. Even divides are supported only. This value represents the division amount minus 1."]
pub type RxclkacttrimW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TONCLKTRIM` reader - This divides the 100 MHz ton clock. Even divides are supported only. This value represents the division amount minus 1."]
pub type TonclktrimR = crate::FieldReader;
#[doc = "Field `TONCLKTRIM` writer - This divides the 100 MHz ton clock. Even divides are supported only. This value represents the division amount minus 1."]
pub type TonclktrimW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 6:10 - This divides the 5 MHz refresh clock. Even divides are supported only. This value represents the division amount minus 1."]
    #[inline(always)]
    pub fn rxclkacttrim(&self) -> RxclkacttrimR {
        RxclkacttrimR::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 22:25 - This divides the 100 MHz ton clock. Even divides are supported only. This value represents the division amount minus 1."]
    #[inline(always)]
    pub fn tonclktrim(&self) -> TonclktrimR {
        TonclktrimR::new(((self.bits >> 22) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 6:10 - This divides the 5 MHz refresh clock. Even divides are supported only. This value represents the division amount minus 1."]
    #[inline(always)]
    #[must_use]
    pub fn rxclkacttrim(&mut self) -> RxclkacttrimW<Simobuck1Spec> {
        RxclkacttrimW::new(self, 6)
    }
    #[doc = "Bits 22:25 - This divides the 100 MHz ton clock. Even divides are supported only. This value represents the division amount minus 1."]
    #[inline(always)]
    #[must_use]
    pub fn tonclktrim(&mut self) -> TonclktrimW<Simobuck1Spec> {
        TonclktrimW::new(self, 22)
    }
}
#[doc = "1. Control the even division of 3 clocks: refresh, low power and TONCLK. 2. Control gap bewteen secondary switches. 3. Debug features: control the amount of time TONCLK is on, and the time before snubber asserts for each buck sequence. 4. Enable or disable the observation bus. 5. Select the buck sequence operation mode. 6. Control delay between primary Pmos and Nmos transitions.\n\nYou can [`read`](crate::Reg::read) this register and get [`simobuck1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`simobuck1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Simobuck1Spec;
impl crate::RegisterSpec for Simobuck1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`simobuck1::R`](R) reader structure"]
impl crate::Readable for Simobuck1Spec {}
#[doc = "`write(|w| ..)` method takes [`simobuck1::W`](W) writer structure"]
impl crate::Writable for Simobuck1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SIMOBUCK1 to value 0x0040_0000"]
impl crate::Resettable for Simobuck1Spec {
    const RESET_VALUE: u32 = 0x0040_0000;
}
