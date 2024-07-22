#[doc = "Register `RESXY` reader"]
pub type R = crate::R<ResxySpec>;
#[doc = "Register `RESXY` writer"]
pub type W = crate::W<ResxySpec>;
#[doc = "Field `YRES` reader - Value of Y resolution in pixels"]
pub type YresR = crate::FieldReader<u16>;
#[doc = "Field `YRES` writer - Value of Y resolution in pixels"]
pub type YresW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `XRES` reader - Value of X resolution in pixels"]
pub type XresR = crate::FieldReader<u16>;
#[doc = "Field `XRES` writer - Value of X resolution in pixels"]
pub type XresW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Value of Y resolution in pixels"]
    #[inline(always)]
    pub fn yres(&self) -> YresR {
        YresR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Value of X resolution in pixels"]
    #[inline(always)]
    pub fn xres(&self) -> XresR {
        XresR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Value of Y resolution in pixels"]
    #[inline(always)]
    #[must_use]
    pub fn yres(&mut self) -> YresW<ResxySpec> {
        YresW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Value of X resolution in pixels"]
    #[inline(always)]
    #[must_use]
    pub fn xres(&mut self) -> XresW<ResxySpec> {
        XresW::new(self, 16)
    }
}
#[doc = "Specifies the main X and Y resolutions.\n\nYou can [`read`](crate::Reg::read) this register and get [`resxy::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`resxy::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ResxySpec;
impl crate::RegisterSpec for ResxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`resxy::R`](R) reader structure"]
impl crate::Readable for ResxySpec {}
#[doc = "`write(|w| ..)` method takes [`resxy::W`](W) writer structure"]
impl crate::Writable for ResxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RESXY to value 0"]
impl crate::Resettable for ResxySpec {
    const RESET_VALUE: u32 = 0;
}
