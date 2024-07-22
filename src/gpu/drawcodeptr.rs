#[doc = "Register `DRAWCODEPTR` reader"]
pub type R = crate::R<DrawcodeptrSpec>;
#[doc = "Register `DRAWCODEPTR` writer"]
pub type W = crate::W<DrawcodeptrSpec>;
#[doc = "Field `FRGND` reader - the pointer for the instruction that will be executed for foreground pixel"]
pub type FrgndR = crate::FieldReader<u16>;
#[doc = "Field `FRGND` writer - the pointer for the instruction that will be executed for foreground pixel"]
pub type FrgndW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `BKGND` reader - the pointer for the instruction that will be executed for background pixel"]
pub type BkgndR = crate::FieldReader<u16>;
#[doc = "Field `BKGND` writer - the pointer for the instruction that will be executed for background pixel"]
pub type BkgndW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - the pointer for the instruction that will be executed for foreground pixel"]
    #[inline(always)]
    pub fn frgnd(&self) -> FrgndR {
        FrgndR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - the pointer for the instruction that will be executed for background pixel"]
    #[inline(always)]
    pub fn bkgnd(&self) -> BkgndR {
        BkgndR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - the pointer for the instruction that will be executed for foreground pixel"]
    #[inline(always)]
    #[must_use]
    pub fn frgnd(&mut self) -> FrgndW<DrawcodeptrSpec> {
        FrgndW::new(self, 0)
    }
    #[doc = "Bits 16:31 - the pointer for the instruction that will be executed for background pixel"]
    #[inline(always)]
    #[must_use]
    pub fn bkgnd(&mut self) -> BkgndW<DrawcodeptrSpec> {
        BkgndW::new(self, 16)
    }
}
#[doc = "DRAWCODEPTR register description needed here.\n\nYou can [`read`](crate::Reg::read) this register and get [`drawcodeptr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`drawcodeptr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DrawcodeptrSpec;
impl crate::RegisterSpec for DrawcodeptrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`drawcodeptr::R`](R) reader structure"]
impl crate::Readable for DrawcodeptrSpec {}
#[doc = "`write(|w| ..)` method takes [`drawcodeptr::W`](W) writer structure"]
impl crate::Writable for DrawcodeptrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DRAWCODEPTR to value 0"]
impl crate::Resettable for DrawcodeptrSpec {
    const RESET_VALUE: u32 = 0;
}
