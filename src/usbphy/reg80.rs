#[doc = "Register `REG80` reader"]
pub type R = crate::R<Reg80Spec>;
#[doc = "Register `REG80` writer"]
pub type W = crate::W<Reg80Spec>;
#[doc = "Field `BF00` reader - Digital clock enable bypass 1'b1: digital clock bypass enable 1'b0: digital clock bypass disable"]
pub type Bf00R = crate::BitReader;
#[doc = "Field `BF00` writer - Digital clock enable bypass 1'b1: digital clock bypass enable 1'b0: digital clock bypass disable"]
pub type Bf00W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BF11` reader - Digital clock enable bypass value 1'b1: digital clock enable 1'b0: digital clock disable"]
pub type Bf11R = crate::BitReader;
#[doc = "Field `BF11` writer - Digital clock enable bypass value 1'b1: digital clock enable 1'b0: digital clock disable"]
pub type Bf11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BF22` reader - utmi clock always on 1'b1: utmi clock always on 1'b0: utmi clock relative to suspendm"]
pub type Bf22R = crate::BitReader;
#[doc = "Field `BF22` writer - utmi clock always on 1'b1: utmi clock always on 1'b0: utmi clock relative to suspendm"]
pub type Bf22W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BF73` reader - This bitfield is reserved."]
pub type Bf73R = crate::FieldReader;
#[doc = "Field `BF73` writer - This bitfield is reserved."]
pub type Bf73W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bit 0 - Digital clock enable bypass 1'b1: digital clock bypass enable 1'b0: digital clock bypass disable"]
    #[inline(always)]
    pub fn bf00(&self) -> Bf00R {
        Bf00R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Digital clock enable bypass value 1'b1: digital clock enable 1'b0: digital clock disable"]
    #[inline(always)]
    pub fn bf11(&self) -> Bf11R {
        Bf11R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - utmi clock always on 1'b1: utmi clock always on 1'b0: utmi clock relative to suspendm"]
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
    #[doc = "Bit 0 - Digital clock enable bypass 1'b1: digital clock bypass enable 1'b0: digital clock bypass disable"]
    #[inline(always)]
    #[must_use]
    pub fn bf00(&mut self) -> Bf00W<Reg80Spec> {
        Bf00W::new(self, 0)
    }
    #[doc = "Bit 1 - Digital clock enable bypass value 1'b1: digital clock enable 1'b0: digital clock disable"]
    #[inline(always)]
    #[must_use]
    pub fn bf11(&mut self) -> Bf11W<Reg80Spec> {
        Bf11W::new(self, 1)
    }
    #[doc = "Bit 2 - utmi clock always on 1'b1: utmi clock always on 1'b0: utmi clock relative to suspendm"]
    #[inline(always)]
    #[must_use]
    pub fn bf22(&mut self) -> Bf22W<Reg80Spec> {
        Bf22W::new(self, 2)
    }
    #[doc = "Bits 3:7 - This bitfield is reserved."]
    #[inline(always)]
    #[must_use]
    pub fn bf73(&mut self) -> Bf73W<Reg80Spec> {
        Bf73W::new(self, 3)
    }
}
#[doc = "Register description here\n\nYou can [`read`](crate::Reg::read) this register and get [`reg80::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg80::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reg80Spec;
impl crate::RegisterSpec for Reg80Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg80::R`](R) reader structure"]
impl crate::Readable for Reg80Spec {}
#[doc = "`write(|w| ..)` method takes [`reg80::W`](W) writer structure"]
impl crate::Writable for Reg80Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REG80 to value 0"]
impl crate::Resettable for Reg80Spec {
    const RESET_VALUE: u32 = 0;
}
