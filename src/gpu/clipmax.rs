#[doc = "Register `CLIPMAX` reader"]
pub type R = crate::R<ClipmaxSpec>;
#[doc = "Register `CLIPMAX` writer"]
pub type W = crate::W<ClipmaxSpec>;
#[doc = "Field `COORDX` reader - bottom right X coordinate"]
pub type CoordxR = crate::FieldReader<u16>;
#[doc = "Field `COORDX` writer - bottom right X coordinate"]
pub type CoordxW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `COORDY` reader - bottom right Y coordinate"]
pub type CoordyR = crate::FieldReader<u16>;
#[doc = "Field `COORDY` writer - bottom right Y coordinate"]
pub type CoordyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - bottom right X coordinate"]
    #[inline(always)]
    pub fn coordx(&self) -> CoordxR {
        CoordxR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - bottom right Y coordinate"]
    #[inline(always)]
    pub fn coordy(&self) -> CoordyR {
        CoordyR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - bottom right X coordinate"]
    #[inline(always)]
    #[must_use]
    pub fn coordx(&mut self) -> CoordxW<ClipmaxSpec> {
        CoordxW::new(self, 0)
    }
    #[doc = "Bits 16:31 - bottom right Y coordinate"]
    #[inline(always)]
    #[must_use]
    pub fn coordy(&mut self) -> CoordyW<ClipmaxSpec> {
        CoordyW::new(self, 16)
    }
}
#[doc = "Clipping rectangle bottom right vertex.\n\nYou can [`read`](crate::Reg::read) this register and get [`clipmax::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clipmax::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClipmaxSpec;
impl crate::RegisterSpec for ClipmaxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clipmax::R`](R) reader structure"]
impl crate::Readable for ClipmaxSpec {}
#[doc = "`write(|w| ..)` method takes [`clipmax::W`](W) writer structure"]
impl crate::Writable for ClipmaxSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLIPMAX to value 0x7fff_7fff"]
impl crate::Resettable for ClipmaxSpec {
    const RESET_VALUE: u32 = 0x7fff_7fff;
}
