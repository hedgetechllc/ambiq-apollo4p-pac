#[doc = "Register `BBSETCLEAR` reader"]
pub type R = crate::R<BbsetclearSpec>;
#[doc = "Register `BBSETCLEAR` writer"]
pub type W = crate::W<BbsetclearSpec>;
#[doc = "Field `SET` reader - Write 1 to Set PIO value (set hier priority than clear if both bit set)"]
pub type SetR = crate::FieldReader;
#[doc = "Field `SET` writer - Write 1 to Set PIO value (set hier priority than clear if both bit set)"]
pub type SetW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CLEAR` reader - Write 1 to Clear PIO value"]
pub type ClearR = crate::FieldReader;
#[doc = "Field `CLEAR` writer - Write 1 to Clear PIO value"]
pub type ClearW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Write 1 to Set PIO value (set hier priority than clear if both bit set)"]
    #[inline(always)]
    pub fn set_(&self) -> SetR {
        SetR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Write 1 to Clear PIO value"]
    #[inline(always)]
    pub fn clear(&self) -> ClearR {
        ClearR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Write 1 to Set PIO value (set hier priority than clear if both bit set)"]
    #[inline(always)]
    #[must_use]
    pub fn set_(&mut self) -> SetW<BbsetclearSpec> {
        SetW::new(self, 0)
    }
    #[doc = "Bits 16:23 - Write 1 to Clear PIO value"]
    #[inline(always)]
    #[must_use]
    pub fn clear(&mut self) -> ClearW<BbsetclearSpec> {
        ClearW::new(self, 16)
    }
}
#[doc = "Set/Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`bbsetclear::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bbsetclear::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BbsetclearSpec;
impl crate::RegisterSpec for BbsetclearSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bbsetclear::R`](R) reader structure"]
impl crate::Readable for BbsetclearSpec {}
#[doc = "`write(|w| ..)` method takes [`bbsetclear::W`](W) writer structure"]
impl crate::Writable for BbsetclearSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BBSETCLEAR to value 0"]
impl crate::Resettable for BbsetclearSpec {
    const RESET_VALUE: u32 = 0;
}
