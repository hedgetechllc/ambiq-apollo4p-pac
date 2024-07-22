#[doc = "Register `HASHPADCFG` reader"]
pub type R = crate::R<HashpadcfgSpec>;
#[doc = "Register `HASHPADCFG` writer"]
pub type W = crate::W<HashpadcfgSpec>;
#[doc = "Field `DOPAD` reader - Enable Padding generation. must be reset upon completion of padding."]
pub type DopadR = crate::BitReader;
#[doc = "Field `DOPAD` writer - Enable Padding generation. must be reset upon completion of padding."]
pub type DopadW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - Enable Padding generation. must be reset upon completion of padding."]
    #[inline(always)]
    pub fn dopad(&self) -> DopadR {
        DopadR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Enable Padding generation. must be reset upon completion of padding."]
    #[inline(always)]
    #[must_use]
    pub fn dopad(&mut self) -> DopadW<HashpadcfgSpec> {
        DopadW::new(self, 2)
    }
}
#[doc = "This is a special register, affected by internal logic. Test result of this register is NA.\n\nYou can [`read`](crate::Reg::read) this register and get [`hashpadcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hashpadcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HashpadcfgSpec;
impl crate::RegisterSpec for HashpadcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hashpadcfg::R`](R) reader structure"]
impl crate::Readable for HashpadcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`hashpadcfg::W`](W) writer structure"]
impl crate::Writable for HashpadcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HASHPADCFG to value 0"]
impl crate::Resettable for HashpadcfgSpec {
    const RESET_VALUE: u32 = 0;
}
