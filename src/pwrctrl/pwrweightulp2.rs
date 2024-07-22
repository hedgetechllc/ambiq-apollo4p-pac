#[doc = "Register `PWRWEIGHTULP2` reader"]
pub type R = crate::R<Pwrweightulp2Spec>;
#[doc = "Register `PWRWEIGHTULP2` writer"]
pub type W = crate::W<Pwrweightulp2Spec>;
#[doc = "Field `WTULPADC` reader - Weight used for ULP mode ADC"]
pub type WtulpadcR = crate::FieldReader;
#[doc = "Field `WTULPADC` writer - Weight used for ULP mode ADC"]
pub type WtulpadcW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WTULPMSPI0` reader - Weight used for ULP mode MSPI0"]
pub type Wtulpmspi0R = crate::FieldReader;
#[doc = "Field `WTULPMSPI0` writer - Weight used for ULP mode MSPI0"]
pub type Wtulpmspi0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WTULPMSPI1` reader - Weight used for ULP mode MSPI1"]
pub type Wtulpmspi1R = crate::FieldReader;
#[doc = "Field `WTULPMSPI1` writer - Weight used for ULP mode MSPI1"]
pub type Wtulpmspi1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WTULPGFX` reader - Weight used for ULP mode GFX"]
pub type WtulpgfxR = crate::FieldReader;
#[doc = "Field `WTULPGFX` writer - Weight used for ULP mode GFX"]
pub type WtulpgfxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WTULPDISP` reader - Weight used for ULP mode DISP"]
pub type WtulpdispR = crate::FieldReader;
#[doc = "Field `WTULPDISP` writer - Weight used for ULP mode DISP"]
pub type WtulpdispW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WTULPCRYPTO` reader - Weight used for ULP mode CRYPTO"]
pub type WtulpcryptoR = crate::FieldReader;
#[doc = "Field `WTULPCRYPTO` writer - Weight used for ULP mode CRYPTO"]
pub type WtulpcryptoW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WTULPSDIO` reader - Weight used for ULP mode SDIO"]
pub type WtulpsdioR = crate::FieldReader;
#[doc = "Field `WTULPSDIO` writer - Weight used for ULP mode SDIO"]
pub type WtulpsdioW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WTULPUSB` reader - Weight used for ULP mode USB"]
pub type WtulpusbR = crate::FieldReader;
#[doc = "Field `WTULPUSB` writer - Weight used for ULP mode USB"]
pub type WtulpusbW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Weight used for ULP mode ADC"]
    #[inline(always)]
    pub fn wtulpadc(&self) -> WtulpadcR {
        WtulpadcR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Weight used for ULP mode MSPI0"]
    #[inline(always)]
    pub fn wtulpmspi0(&self) -> Wtulpmspi0R {
        Wtulpmspi0R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Weight used for ULP mode MSPI1"]
    #[inline(always)]
    pub fn wtulpmspi1(&self) -> Wtulpmspi1R {
        Wtulpmspi1R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Weight used for ULP mode GFX"]
    #[inline(always)]
    pub fn wtulpgfx(&self) -> WtulpgfxR {
        WtulpgfxR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Weight used for ULP mode DISP"]
    #[inline(always)]
    pub fn wtulpdisp(&self) -> WtulpdispR {
        WtulpdispR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Weight used for ULP mode CRYPTO"]
    #[inline(always)]
    pub fn wtulpcrypto(&self) -> WtulpcryptoR {
        WtulpcryptoR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Weight used for ULP mode SDIO"]
    #[inline(always)]
    pub fn wtulpsdio(&self) -> WtulpsdioR {
        WtulpsdioR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Weight used for ULP mode USB"]
    #[inline(always)]
    pub fn wtulpusb(&self) -> WtulpusbR {
        WtulpusbR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Weight used for ULP mode ADC"]
    #[inline(always)]
    #[must_use]
    pub fn wtulpadc(&mut self) -> WtulpadcW<Pwrweightulp2Spec> {
        WtulpadcW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Weight used for ULP mode MSPI0"]
    #[inline(always)]
    #[must_use]
    pub fn wtulpmspi0(&mut self) -> Wtulpmspi0W<Pwrweightulp2Spec> {
        Wtulpmspi0W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Weight used for ULP mode MSPI1"]
    #[inline(always)]
    #[must_use]
    pub fn wtulpmspi1(&mut self) -> Wtulpmspi1W<Pwrweightulp2Spec> {
        Wtulpmspi1W::new(self, 8)
    }
    #[doc = "Bits 12:15 - Weight used for ULP mode GFX"]
    #[inline(always)]
    #[must_use]
    pub fn wtulpgfx(&mut self) -> WtulpgfxW<Pwrweightulp2Spec> {
        WtulpgfxW::new(self, 12)
    }
    #[doc = "Bits 16:19 - Weight used for ULP mode DISP"]
    #[inline(always)]
    #[must_use]
    pub fn wtulpdisp(&mut self) -> WtulpdispW<Pwrweightulp2Spec> {
        WtulpdispW::new(self, 16)
    }
    #[doc = "Bits 20:23 - Weight used for ULP mode CRYPTO"]
    #[inline(always)]
    #[must_use]
    pub fn wtulpcrypto(&mut self) -> WtulpcryptoW<Pwrweightulp2Spec> {
        WtulpcryptoW::new(self, 20)
    }
    #[doc = "Bits 24:27 - Weight used for ULP mode SDIO"]
    #[inline(always)]
    #[must_use]
    pub fn wtulpsdio(&mut self) -> WtulpsdioW<Pwrweightulp2Spec> {
        WtulpsdioW::new(self, 24)
    }
    #[doc = "Bits 28:31 - Weight used for ULP mode USB"]
    #[inline(always)]
    #[must_use]
    pub fn wtulpusb(&mut self) -> WtulpusbW<Pwrweightulp2Spec> {
        WtulpusbW::new(self, 28)
    }
}
#[doc = "Weights specified in this register are applied to each of the masters active requests. The aggregate of all the masters is compared against the allowed value to change the buck from active to inactive mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`pwrweightulp2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrweightulp2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pwrweightulp2Spec;
impl crate::RegisterSpec for Pwrweightulp2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwrweightulp2::R`](R) reader structure"]
impl crate::Readable for Pwrweightulp2Spec {}
#[doc = "`write(|w| ..)` method takes [`pwrweightulp2::W`](W) writer structure"]
impl crate::Writable for Pwrweightulp2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWRWEIGHTULP2 to value 0"]
impl crate::Resettable for Pwrweightulp2Spec {
    const RESET_VALUE: u32 = 0;
}
