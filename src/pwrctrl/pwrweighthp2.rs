#[doc = "Register `PWRWEIGHTHP2` reader"]
pub type R = crate::R<Pwrweighthp2Spec>;
#[doc = "Register `PWRWEIGHTHP2` writer"]
pub type W = crate::W<Pwrweighthp2Spec>;
#[doc = "Field `WTHPADC` reader - Weight used for HP mode ADC"]
pub type WthpadcR = crate::FieldReader;
#[doc = "Field `WTHPADC` writer - Weight used for HP mode ADC"]
pub type WthpadcW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WTHPMSPI0` reader - Weight used for HP mode MSPI0"]
pub type Wthpmspi0R = crate::FieldReader;
#[doc = "Field `WTHPMSPI0` writer - Weight used for HP mode MSPI0"]
pub type Wthpmspi0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WTHPMSPI1` reader - Weight used for HP mode MSPI1"]
pub type Wthpmspi1R = crate::FieldReader;
#[doc = "Field `WTHPMSPI1` writer - Weight used for HP mode MSPI1"]
pub type Wthpmspi1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WTHPGFX` reader - Weight used for HP mode GFX"]
pub type WthpgfxR = crate::FieldReader;
#[doc = "Field `WTHPGFX` writer - Weight used for HP mode GFX"]
pub type WthpgfxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WTHPDISP` reader - Weight used for HP mode DISP"]
pub type WthpdispR = crate::FieldReader;
#[doc = "Field `WTHPDISP` writer - Weight used for HP mode DISP"]
pub type WthpdispW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WTHPCRYPTO` reader - Weight used for HP mode CRYPTO"]
pub type WthpcryptoR = crate::FieldReader;
#[doc = "Field `WTHPCRYPTO` writer - Weight used for HP mode CRYPTO"]
pub type WthpcryptoW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WTHPSDIO` reader - Weight used for HP mode SDIO"]
pub type WthpsdioR = crate::FieldReader;
#[doc = "Field `WTHPSDIO` writer - Weight used for HP mode SDIO"]
pub type WthpsdioW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WTHPUSB` reader - Weight used for HP mode USB"]
pub type WthpusbR = crate::FieldReader;
#[doc = "Field `WTHPUSB` writer - Weight used for HP mode USB"]
pub type WthpusbW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Weight used for HP mode ADC"]
    #[inline(always)]
    pub fn wthpadc(&self) -> WthpadcR {
        WthpadcR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Weight used for HP mode MSPI0"]
    #[inline(always)]
    pub fn wthpmspi0(&self) -> Wthpmspi0R {
        Wthpmspi0R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Weight used for HP mode MSPI1"]
    #[inline(always)]
    pub fn wthpmspi1(&self) -> Wthpmspi1R {
        Wthpmspi1R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Weight used for HP mode GFX"]
    #[inline(always)]
    pub fn wthpgfx(&self) -> WthpgfxR {
        WthpgfxR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Weight used for HP mode DISP"]
    #[inline(always)]
    pub fn wthpdisp(&self) -> WthpdispR {
        WthpdispR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Weight used for HP mode CRYPTO"]
    #[inline(always)]
    pub fn wthpcrypto(&self) -> WthpcryptoR {
        WthpcryptoR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Weight used for HP mode SDIO"]
    #[inline(always)]
    pub fn wthpsdio(&self) -> WthpsdioR {
        WthpsdioR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Weight used for HP mode USB"]
    #[inline(always)]
    pub fn wthpusb(&self) -> WthpusbR {
        WthpusbR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Weight used for HP mode ADC"]
    #[inline(always)]
    #[must_use]
    pub fn wthpadc(&mut self) -> WthpadcW<Pwrweighthp2Spec> {
        WthpadcW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Weight used for HP mode MSPI0"]
    #[inline(always)]
    #[must_use]
    pub fn wthpmspi0(&mut self) -> Wthpmspi0W<Pwrweighthp2Spec> {
        Wthpmspi0W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Weight used for HP mode MSPI1"]
    #[inline(always)]
    #[must_use]
    pub fn wthpmspi1(&mut self) -> Wthpmspi1W<Pwrweighthp2Spec> {
        Wthpmspi1W::new(self, 8)
    }
    #[doc = "Bits 12:15 - Weight used for HP mode GFX"]
    #[inline(always)]
    #[must_use]
    pub fn wthpgfx(&mut self) -> WthpgfxW<Pwrweighthp2Spec> {
        WthpgfxW::new(self, 12)
    }
    #[doc = "Bits 16:19 - Weight used for HP mode DISP"]
    #[inline(always)]
    #[must_use]
    pub fn wthpdisp(&mut self) -> WthpdispW<Pwrweighthp2Spec> {
        WthpdispW::new(self, 16)
    }
    #[doc = "Bits 20:23 - Weight used for HP mode CRYPTO"]
    #[inline(always)]
    #[must_use]
    pub fn wthpcrypto(&mut self) -> WthpcryptoW<Pwrweighthp2Spec> {
        WthpcryptoW::new(self, 20)
    }
    #[doc = "Bits 24:27 - Weight used for HP mode SDIO"]
    #[inline(always)]
    #[must_use]
    pub fn wthpsdio(&mut self) -> WthpsdioW<Pwrweighthp2Spec> {
        WthpsdioW::new(self, 24)
    }
    #[doc = "Bits 28:31 - Weight used for HP mode USB"]
    #[inline(always)]
    #[must_use]
    pub fn wthpusb(&mut self) -> WthpusbW<Pwrweighthp2Spec> {
        WthpusbW::new(self, 28)
    }
}
#[doc = "Weights specified in this register are applied to each of the masters active requests. The aggregate of all the masters is compared against the allowed value to change the buck from active to inactive mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`pwrweighthp2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrweighthp2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pwrweighthp2Spec;
impl crate::RegisterSpec for Pwrweighthp2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwrweighthp2::R`](R) reader structure"]
impl crate::Readable for Pwrweighthp2Spec {}
#[doc = "`write(|w| ..)` method takes [`pwrweighthp2::W`](W) writer structure"]
impl crate::Writable for Pwrweighthp2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWRWEIGHTHP2 to value 0"]
impl crate::Resettable for Pwrweighthp2Spec {
    const RESET_VALUE: u32 = 0;
}
