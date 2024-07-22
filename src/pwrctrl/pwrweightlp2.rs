#[doc = "Register `PWRWEIGHTLP2` reader"]
pub type R = crate::R<Pwrweightlp2Spec>;
#[doc = "Register `PWRWEIGHTLP2` writer"]
pub type W = crate::W<Pwrweightlp2Spec>;
#[doc = "Field `WTLPADC` reader - Weight used for LP mode ADC"]
pub type WtlpadcR = crate::FieldReader;
#[doc = "Field `WTLPADC` writer - Weight used for LP mode ADC"]
pub type WtlpadcW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WTLPMSPI0` reader - Weight used for LP mode MSPI0"]
pub type Wtlpmspi0R = crate::FieldReader;
#[doc = "Field `WTLPMSPI0` writer - Weight used for LP mode MSPI0"]
pub type Wtlpmspi0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WTLPMSPI1` reader - Weight used for LP mode MSPI1"]
pub type Wtlpmspi1R = crate::FieldReader;
#[doc = "Field `WTLPMSPI1` writer - Weight used for LP mode MSPI1"]
pub type Wtlpmspi1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WTLPGFX` reader - Weight used for LP mode GFX"]
pub type WtlpgfxR = crate::FieldReader;
#[doc = "Field `WTLPGFX` writer - Weight used for LP mode GFX"]
pub type WtlpgfxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WTLPDISP` reader - Weight used for LP mode DISP"]
pub type WtlpdispR = crate::FieldReader;
#[doc = "Field `WTLPDISP` writer - Weight used for LP mode DISP"]
pub type WtlpdispW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WTLPCRYPTO` reader - Weight used for LP mode CRYPTO"]
pub type WtlpcryptoR = crate::FieldReader;
#[doc = "Field `WTLPCRYPTO` writer - Weight used for LP mode CRYPTO"]
pub type WtlpcryptoW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WTLPSDIO` reader - Weight used for LP mode SDIO"]
pub type WtlpsdioR = crate::FieldReader;
#[doc = "Field `WTLPSDIO` writer - Weight used for LP mode SDIO"]
pub type WtlpsdioW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WTLPUSB` reader - Weight used for LP mode USB"]
pub type WtlpusbR = crate::FieldReader;
#[doc = "Field `WTLPUSB` writer - Weight used for LP mode USB"]
pub type WtlpusbW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Weight used for LP mode ADC"]
    #[inline(always)]
    pub fn wtlpadc(&self) -> WtlpadcR {
        WtlpadcR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Weight used for LP mode MSPI0"]
    #[inline(always)]
    pub fn wtlpmspi0(&self) -> Wtlpmspi0R {
        Wtlpmspi0R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Weight used for LP mode MSPI1"]
    #[inline(always)]
    pub fn wtlpmspi1(&self) -> Wtlpmspi1R {
        Wtlpmspi1R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Weight used for LP mode GFX"]
    #[inline(always)]
    pub fn wtlpgfx(&self) -> WtlpgfxR {
        WtlpgfxR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Weight used for LP mode DISP"]
    #[inline(always)]
    pub fn wtlpdisp(&self) -> WtlpdispR {
        WtlpdispR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Weight used for LP mode CRYPTO"]
    #[inline(always)]
    pub fn wtlpcrypto(&self) -> WtlpcryptoR {
        WtlpcryptoR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Weight used for LP mode SDIO"]
    #[inline(always)]
    pub fn wtlpsdio(&self) -> WtlpsdioR {
        WtlpsdioR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Weight used for LP mode USB"]
    #[inline(always)]
    pub fn wtlpusb(&self) -> WtlpusbR {
        WtlpusbR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Weight used for LP mode ADC"]
    #[inline(always)]
    #[must_use]
    pub fn wtlpadc(&mut self) -> WtlpadcW<Pwrweightlp2Spec> {
        WtlpadcW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Weight used for LP mode MSPI0"]
    #[inline(always)]
    #[must_use]
    pub fn wtlpmspi0(&mut self) -> Wtlpmspi0W<Pwrweightlp2Spec> {
        Wtlpmspi0W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Weight used for LP mode MSPI1"]
    #[inline(always)]
    #[must_use]
    pub fn wtlpmspi1(&mut self) -> Wtlpmspi1W<Pwrweightlp2Spec> {
        Wtlpmspi1W::new(self, 8)
    }
    #[doc = "Bits 12:15 - Weight used for LP mode GFX"]
    #[inline(always)]
    #[must_use]
    pub fn wtlpgfx(&mut self) -> WtlpgfxW<Pwrweightlp2Spec> {
        WtlpgfxW::new(self, 12)
    }
    #[doc = "Bits 16:19 - Weight used for LP mode DISP"]
    #[inline(always)]
    #[must_use]
    pub fn wtlpdisp(&mut self) -> WtlpdispW<Pwrweightlp2Spec> {
        WtlpdispW::new(self, 16)
    }
    #[doc = "Bits 20:23 - Weight used for LP mode CRYPTO"]
    #[inline(always)]
    #[must_use]
    pub fn wtlpcrypto(&mut self) -> WtlpcryptoW<Pwrweightlp2Spec> {
        WtlpcryptoW::new(self, 20)
    }
    #[doc = "Bits 24:27 - Weight used for LP mode SDIO"]
    #[inline(always)]
    #[must_use]
    pub fn wtlpsdio(&mut self) -> WtlpsdioW<Pwrweightlp2Spec> {
        WtlpsdioW::new(self, 24)
    }
    #[doc = "Bits 28:31 - Weight used for LP mode USB"]
    #[inline(always)]
    #[must_use]
    pub fn wtlpusb(&mut self) -> WtlpusbW<Pwrweightlp2Spec> {
        WtlpusbW::new(self, 28)
    }
}
#[doc = "Weights specified in this register are applied to each of the masters active requests. The aggregate of all the masters is compared against the allowed value to change the buck from active to inactive mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`pwrweightlp2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrweightlp2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pwrweightlp2Spec;
impl crate::RegisterSpec for Pwrweightlp2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwrweightlp2::R`](R) reader structure"]
impl crate::Readable for Pwrweightlp2Spec {}
#[doc = "`write(|w| ..)` method takes [`pwrweightlp2::W`](W) writer structure"]
impl crate::Writable for Pwrweightlp2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWRWEIGHTLP2 to value 0"]
impl crate::Resettable for Pwrweightlp2Spec {
    const RESET_VALUE: u32 = 0;
}
