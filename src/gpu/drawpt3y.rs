#[doc = "Register `DRAWPT3Y` reader"]
pub type R = crate::R<Drawpt3ySpec>;
#[doc = "Register `DRAWPT3Y` writer"]
pub type W = crate::W<Drawpt3ySpec>;
#[doc = "Field `DRAW3Y` reader - Y coordinate."]
pub type Draw3yR = crate::FieldReader<u32>;
#[doc = "Field `DRAW3Y` writer - Y coordinate."]
pub type Draw3yW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Y coordinate."]
    #[inline(always)]
    pub fn draw3y(&self) -> Draw3yR {
        Draw3yR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Y coordinate."]
    #[inline(always)]
    #[must_use]
    pub fn draw3y(&mut self) -> Draw3yW<Drawpt3ySpec> {
        Draw3yW::new(self, 0)
    }
}
#[doc = "Y coordinate of Vertex 3 drawing primitive 16, 16 fixed point.\n\nYou can [`read`](crate::Reg::read) this register and get [`drawpt3y::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`drawpt3y::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Drawpt3ySpec;
impl crate::RegisterSpec for Drawpt3ySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`drawpt3y::R`](R) reader structure"]
impl crate::Readable for Drawpt3ySpec {}
#[doc = "`write(|w| ..)` method takes [`drawpt3y::W`](W) writer structure"]
impl crate::Writable for Drawpt3ySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DRAWPT3Y to value 0"]
impl crate::Resettable for Drawpt3ySpec {
    const RESET_VALUE: u32 = 0;
}
