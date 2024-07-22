#[doc = "Register `LCSISVALID` reader"]
pub type R = crate::R<LcsisvalidSpec>;
#[doc = "Register `LCSISVALID` writer"]
pub type W = crate::W<LcsisvalidSpec>;
#[doc = "Field `LCSISVALIDREG` reader - Indicates whether LCS is valid."]
pub type LcsisvalidregR = crate::BitReader;
#[doc = "Field `LCSISVALIDREG` writer - Indicates whether LCS is valid."]
pub type LcsisvalidregW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Indicates whether LCS is valid."]
    #[inline(always)]
    pub fn lcsisvalidreg(&self) -> LcsisvalidregR {
        LcsisvalidregR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Indicates whether LCS is valid."]
    #[inline(always)]
    #[must_use]
    pub fn lcsisvalidreg(&mut self) -> LcsisvalidregW<LcsisvalidSpec> {
        LcsisvalidregW::new(self, 0)
    }
}
#[doc = "Indicates that the LCS register holds a valid value.Note: This is a special register, affected by internal logic. Test result of this register is NA.\n\nYou can [`read`](crate::Reg::read) this register and get [`lcsisvalid::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcsisvalid::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcsisvalidSpec;
impl crate::RegisterSpec for LcsisvalidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcsisvalid::R`](R) reader structure"]
impl crate::Readable for LcsisvalidSpec {}
#[doc = "`write(|w| ..)` method takes [`lcsisvalid::W`](W) writer structure"]
impl crate::Writable for LcsisvalidSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LCSISVALID to value 0x01"]
impl crate::Resettable for LcsisvalidSpec {
    const RESET_VALUE: u32 = 0x01;
}
