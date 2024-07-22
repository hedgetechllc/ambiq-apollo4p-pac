#[doc = "Register `AHBMHNONSEC` reader"]
pub type R = crate::R<AhbmhnonsecSpec>;
#[doc = "Register `AHBMHNONSEC` writer"]
pub type W = crate::W<AhbmhnonsecSpec>;
#[doc = "Field `AHBWRITEHNONSEC` reader - The hnonsec value for write transaction."]
pub type AhbwritehnonsecR = crate::BitReader;
#[doc = "Field `AHBWRITEHNONSEC` writer - The hnonsec value for write transaction."]
pub type AhbwritehnonsecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHBREADHNONSEC` reader - The hnonsec value for read transaction."]
pub type AhbreadhnonsecR = crate::BitReader;
#[doc = "Field `AHBREADHNONSEC` writer - The hnonsec value for read transaction."]
pub type AhbreadhnonsecW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The hnonsec value for write transaction."]
    #[inline(always)]
    pub fn ahbwritehnonsec(&self) -> AhbwritehnonsecR {
        AhbwritehnonsecR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The hnonsec value for read transaction."]
    #[inline(always)]
    pub fn ahbreadhnonsec(&self) -> AhbreadhnonsecR {
        AhbreadhnonsecR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The hnonsec value for write transaction."]
    #[inline(always)]
    #[must_use]
    pub fn ahbwritehnonsec(&mut self) -> AhbwritehnonsecW<AhbmhnonsecSpec> {
        AhbwritehnonsecW::new(self, 0)
    }
    #[doc = "Bit 1 - The hnonsec value for read transaction."]
    #[inline(always)]
    #[must_use]
    pub fn ahbreadhnonsec(&mut self) -> AhbreadhnonsecW<AhbmhnonsecSpec> {
        AhbreadhnonsecW::new(self, 1)
    }
}
#[doc = "This register holds ahb hnonsec value\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbmhnonsec::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbmhnonsec::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AhbmhnonsecSpec;
impl crate::RegisterSpec for AhbmhnonsecSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahbmhnonsec::R`](R) reader structure"]
impl crate::Readable for AhbmhnonsecSpec {}
#[doc = "`write(|w| ..)` method takes [`ahbmhnonsec::W`](W) writer structure"]
impl crate::Writable for AhbmhnonsecSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHBMHNONSEC to value 0"]
impl crate::Resettable for AhbmhnonsecSpec {
    const RESET_VALUE: u32 = 0;
}
