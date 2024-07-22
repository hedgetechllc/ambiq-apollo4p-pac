#[doc = "Register `REG48` reader"]
pub type R = crate::R<Reg48Spec>;
#[doc = "Register `REG48` writer"]
pub type W = crate::W<Reg48Spec>;
#[doc = "Field `BF00` reader - Enable TX shutdown, active LOW. This bit is only used for debug purpose , nothing to do with the normal operation and signal quality, keeping the default value is strongly recommended."]
pub type Bf00R = crate::BitReader;
#[doc = "Field `BF00` writer - Enable TX shutdown, active LOW. This bit is only used for debug purpose , nothing to do with the normal operation and signal quality, keeping the default value is strongly recommended."]
pub type Bf00W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BF71` reader - This bitfield is reserved."]
pub type Bf71R = crate::FieldReader;
#[doc = "Field `BF71` writer - This bitfield is reserved."]
pub type Bf71W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bit 0 - Enable TX shutdown, active LOW. This bit is only used for debug purpose , nothing to do with the normal operation and signal quality, keeping the default value is strongly recommended."]
    #[inline(always)]
    pub fn bf00(&self) -> Bf00R {
        Bf00R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - This bitfield is reserved."]
    #[inline(always)]
    pub fn bf71(&self) -> Bf71R {
        Bf71R::new(((self.bits >> 1) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable TX shutdown, active LOW. This bit is only used for debug purpose , nothing to do with the normal operation and signal quality, keeping the default value is strongly recommended."]
    #[inline(always)]
    #[must_use]
    pub fn bf00(&mut self) -> Bf00W<Reg48Spec> {
        Bf00W::new(self, 0)
    }
    #[doc = "Bits 1:7 - This bitfield is reserved."]
    #[inline(always)]
    #[must_use]
    pub fn bf71(&mut self) -> Bf71W<Reg48Spec> {
        Bf71W::new(self, 1)
    }
}
#[doc = "Register description here\n\nYou can [`read`](crate::Reg::read) this register and get [`reg48::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg48::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reg48Spec;
impl crate::RegisterSpec for Reg48Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg48::R`](R) reader structure"]
impl crate::Readable for Reg48Spec {}
#[doc = "`write(|w| ..)` method takes [`reg48::W`](W) writer structure"]
impl crate::Writable for Reg48Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REG48 to value 0"]
impl crate::Resettable for Reg48Spec {
    const RESET_VALUE: u32 = 0;
}
