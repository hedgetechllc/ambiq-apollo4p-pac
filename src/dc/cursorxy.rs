#[doc = "Register `CURSORXY` reader"]
pub type R = crate::R<CursorxySpec>;
#[doc = "Register `CURSORXY` writer"]
pub type W = crate::W<CursorxySpec>;
#[doc = "Field `CURSORY` reader - Specify cursor's Y dimension"]
pub type CursoryR = crate::FieldReader<u16>;
#[doc = "Field `CURSORY` writer - Specify cursor's Y dimension"]
pub type CursoryW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CURSORX` reader - Specify cursor's X dimension"]
pub type CursorxR = crate::FieldReader<u16>;
#[doc = "Field `CURSORX` writer - Specify cursor's X dimension"]
pub type CursorxW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Specify cursor's Y dimension"]
    #[inline(always)]
    pub fn cursory(&self) -> CursoryR {
        CursoryR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Specify cursor's X dimension"]
    #[inline(always)]
    pub fn cursorx(&self) -> CursorxR {
        CursorxR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Specify cursor's Y dimension"]
    #[inline(always)]
    #[must_use]
    pub fn cursory(&mut self) -> CursoryW<CursorxySpec> {
        CursoryW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Specify cursor's X dimension"]
    #[inline(always)]
    #[must_use]
    pub fn cursorx(&mut self) -> CursorxW<CursorxySpec> {
        CursorxW::new(self, 16)
    }
}
#[doc = "Specifies the cursor's start X and Y coordinates.\n\nYou can [`read`](crate::Reg::read) this register and get [`cursorxy::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cursorxy::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CursorxySpec;
impl crate::RegisterSpec for CursorxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cursorxy::R`](R) reader structure"]
impl crate::Readable for CursorxySpec {}
#[doc = "`write(|w| ..)` method takes [`cursorxy::W`](W) writer structure"]
impl crate::Writable for CursorxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CURSORXY to value 0"]
impl crate::Resettable for CursorxySpec {
    const RESET_VALUE: u32 = 0;
}
