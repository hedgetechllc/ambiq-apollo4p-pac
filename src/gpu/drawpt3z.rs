#[doc = "Register `DRAWPT3Z` reader"]
pub type R = crate::R<Drawpt3zSpec>;
#[doc = "Register `DRAWPT3Z` writer"]
pub type W = crate::W<Drawpt3zSpec>;
#[doc = "Field `DRAW3Z` reader - Fixed value (not accessible)"]
pub type Draw3zR = crate::FieldReader<u32>;
#[doc = "Field `DRAW3Z` writer - Fixed value (not accessible)"]
pub type Draw3zW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Fixed value (not accessible)"]
    #[inline(always)]
    pub fn draw3z(&self) -> Draw3zR {
        Draw3zR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Fixed value (not accessible)"]
    #[inline(always)]
    #[must_use]
    pub fn draw3z(&mut self) -> Draw3zW<Drawpt3zSpec> {
        Draw3zW::new(self, 0)
    }
}
#[doc = "Fixed value (not accessible). Registers 0x160-0x180 are the elements of the 3x3 transformation matrix used for homogeneous conversion from screen coordinates to texture coordinates; the elements are floating points\n\nYou can [`read`](crate::Reg::read) this register and get [`drawpt3z::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`drawpt3z::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Drawpt3zSpec;
impl crate::RegisterSpec for Drawpt3zSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`drawpt3z::R`](R) reader structure"]
impl crate::Readable for Drawpt3zSpec {}
#[doc = "`write(|w| ..)` method takes [`drawpt3z::W`](W) writer structure"]
impl crate::Writable for Drawpt3zSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DRAWPT3Z to value 0"]
impl crate::Resettable for Drawpt3zSpec {
    const RESET_VALUE: u32 = 0;
}
