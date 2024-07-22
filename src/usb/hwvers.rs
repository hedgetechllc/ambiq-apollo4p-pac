#[doc = "Register `HWVERS` reader"]
pub type R = crate::R<HwversSpec>;
#[doc = "Register `HWVERS` writer"]
pub type W = crate::W<HwversSpec>;
#[doc = "Field `yyy` reader - Minor Version Number (Range 0 - 999)."]
pub type YyyR = crate::FieldReader<u16>;
#[doc = "Field `yyy` writer - Minor Version Number (Range 0 - 999)."]
pub type YyyW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `xx` reader - Major Version Number (Range 0 - 31)."]
pub type XxR = crate::FieldReader;
#[doc = "Field `xx` writer - Major Version Number (Range 0 - 31)."]
pub type XxW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `RC` reader - Unused"]
pub type RcR = crate::BitReader;
#[doc = "Field `RC` writer - Unused"]
pub type RcW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - Minor Version Number (Range 0 - 999)."]
    #[inline(always)]
    pub fn yyy(&self) -> YyyR {
        YyyR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:14 - Major Version Number (Range 0 - 31)."]
    #[inline(always)]
    pub fn xx(&self) -> XxR {
        XxR::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bit 15 - Unused"]
    #[inline(always)]
    pub fn rc(&self) -> RcR {
        RcR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - Minor Version Number (Range 0 - 999)."]
    #[inline(always)]
    #[must_use]
    pub fn yyy(&mut self) -> YyyW<HwversSpec> {
        YyyW::new(self, 0)
    }
    #[doc = "Bits 10:14 - Major Version Number (Range 0 - 31)."]
    #[inline(always)]
    #[must_use]
    pub fn xx(&mut self) -> XxW<HwversSpec> {
        XxW::new(self, 10)
    }
    #[doc = "Bit 15 - Unused"]
    #[inline(always)]
    #[must_use]
    pub fn rc(&mut self) -> RcW<HwversSpec> {
        RcW::new(self, 15)
    }
}
#[doc = "Read-only register that returns version number (xx.yyy) of the core hardware.\n\nYou can [`read`](crate::Reg::read) this register and get [`hwvers::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwvers::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HwversSpec;
impl crate::RegisterSpec for HwversSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hwvers::R`](R) reader structure"]
impl crate::Readable for HwversSpec {}
#[doc = "`write(|w| ..)` method takes [`hwvers::W`](W) writer structure"]
impl crate::Writable for HwversSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HWVERS to value 0"]
impl crate::Resettable for HwversSpec {
    const RESET_VALUE: u32 = 0;
}
