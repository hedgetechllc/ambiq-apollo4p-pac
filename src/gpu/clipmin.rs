#[doc = "Register `CLIPMIN` reader"]
pub type R = crate::R<ClipminSpec>;
#[doc = "Register `CLIPMIN` writer"]
pub type W = crate::W<ClipminSpec>;
#[doc = "Field `COORDX` reader - upper left X coordinate"]
pub type CoordxR = crate::FieldReader<u16>;
#[doc = "Field `COORDX` writer - upper left X coordinate"]
pub type CoordxW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `COORDY` reader - upper left Y coordinate"]
pub type CoordyR = crate::FieldReader<u16>;
#[doc = "Field `COORDY` writer - upper left Y coordinate"]
pub type CoordyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - upper left X coordinate"]
    #[inline(always)]
    pub fn coordx(&self) -> CoordxR {
        CoordxR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - upper left Y coordinate"]
    #[inline(always)]
    pub fn coordy(&self) -> CoordyR {
        CoordyR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - upper left X coordinate"]
    #[inline(always)]
    #[must_use]
    pub fn coordx(&mut self) -> CoordxW<ClipminSpec> {
        CoordxW::new(self, 0)
    }
    #[doc = "Bits 16:31 - upper left Y coordinate"]
    #[inline(always)]
    #[must_use]
    pub fn coordy(&mut self) -> CoordyW<ClipminSpec> {
        CoordyW::new(self, 16)
    }
}
#[doc = "Clipping rectangle upper left vertex.\n\nYou can [`read`](crate::Reg::read) this register and get [`clipmin::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clipmin::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClipminSpec;
impl crate::RegisterSpec for ClipminSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clipmin::R`](R) reader structure"]
impl crate::Readable for ClipminSpec {}
#[doc = "`write(|w| ..)` method takes [`clipmin::W`](W) writer structure"]
impl crate::Writable for ClipminSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLIPMIN to value 0"]
impl crate::Resettable for ClipminSpec {
    const RESET_VALUE: u32 = 0;
}
