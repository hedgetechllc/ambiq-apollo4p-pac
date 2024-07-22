#[doc = "Register `DRAWPT0` reader"]
pub type R = crate::R<Drawpt0Spec>;
#[doc = "Register `DRAWPT0` writer"]
pub type W = crate::W<Drawpt0Spec>;
#[doc = "Field `COORDX` reader - vertex 0 X coordinate (integer value)"]
pub type CoordxR = crate::FieldReader<u16>;
#[doc = "Field `COORDX` writer - vertex 0 X coordinate (integer value)"]
pub type CoordxW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `COORDY` reader - vertex 0 Y coordinate (integer value)"]
pub type CoordyR = crate::FieldReader<u16>;
#[doc = "Field `COORDY` writer - vertex 0 Y coordinate (integer value)"]
pub type CoordyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - vertex 0 X coordinate (integer value)"]
    #[inline(always)]
    pub fn coordx(&self) -> CoordxR {
        CoordxR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - vertex 0 Y coordinate (integer value)"]
    #[inline(always)]
    pub fn coordy(&self) -> CoordyR {
        CoordyR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - vertex 0 X coordinate (integer value)"]
    #[inline(always)]
    #[must_use]
    pub fn coordx(&mut self) -> CoordxW<Drawpt0Spec> {
        CoordxW::new(self, 0)
    }
    #[doc = "Bits 16:31 - vertex 0 Y coordinate (integer value)"]
    #[inline(always)]
    #[must_use]
    pub fn coordy(&mut self) -> CoordyW<Drawpt0Spec> {
        CoordyW::new(self, 16)
    }
}
#[doc = "Stores only integer values. For greater accurancy DRAWPT0X and DRAWPT0Y registers are used which are 16, 16 fixed point.\n\nYou can [`read`](crate::Reg::read) this register and get [`drawpt0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`drawpt0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Drawpt0Spec;
impl crate::RegisterSpec for Drawpt0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`drawpt0::R`](R) reader structure"]
impl crate::Readable for Drawpt0Spec {}
#[doc = "`write(|w| ..)` method takes [`drawpt0::W`](W) writer structure"]
impl crate::Writable for Drawpt0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DRAWPT0 to value 0"]
impl crate::Resettable for Drawpt0Spec {
    const RESET_VALUE: u32 = 0;
}
