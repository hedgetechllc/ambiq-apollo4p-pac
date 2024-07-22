#[doc = "Register `CONFG` reader"]
pub type R = crate::R<ConfgSpec>;
#[doc = "Register `CONFG` writer"]
pub type W = crate::W<ConfgSpec>;
#[doc = "Field `CFGGLBGAMMAEN` reader - Indicates that Global Gamma/Palette is enabled"]
pub type CfgglbgammaenR = crate::BitReader;
#[doc = "Field `CFGGLBGAMMAEN` writer - Indicates that Global Gamma/Palette is enabled"]
pub type CfgglbgammaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFGFCURSOREN` reader - Indicates that fixed cursor is enabled"]
pub type CfgfcursorenR = crate::BitReader;
#[doc = "Field `CFGFCURSOREN` writer - Indicates that fixed cursor is enabled"]
pub type CfgfcursorenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFGPCURSOREN` reader - Indicates that programmable cursor is enabled"]
pub type CfgpcursorenR = crate::BitReader;
#[doc = "Field `CFGPCURSOREN` writer - Indicates that programmable cursor is enabled"]
pub type CfgpcursorenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFGDITHEREN` reader - Indicates that dithering is enabled"]
pub type CfgditherenR = crate::BitReader;
#[doc = "Field `CFGDITHEREN` writer - Indicates that dithering is enabled"]
pub type CfgditherenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFGFORMATTEN` reader - Indicates that formatting is enabled"]
pub type CfgformattenR = crate::BitReader;
#[doc = "Field `CFGFORMATTEN` writer - Indicates that formatting is enabled"]
pub type CfgformattenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFGYUVCNVTEN` reader - Indicates that high quality YUV converter is enabled"]
pub type CfgyuvcnvtenR = crate::BitReader;
#[doc = "Field `CFGYUVCNVTEN` writer - Indicates that high quality YUV converter is enabled"]
pub type CfgyuvcnvtenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFGDBITYPEBEN` reader - Indicates that DBI Type-B interface is enabled"]
pub type CfgdbitypebenR = crate::BitReader;
#[doc = "Field `CFGDBITYPEBEN` writer - Indicates that DBI Type-B interface is enabled"]
pub type CfgdbitypebenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFGRGB2YUVEN` reader - Indicates that RGB to YUV converter is enabled"]
pub type Cfgrgb2yuvenR = crate::BitReader;
#[doc = "Field `CFGRGB2YUVEN` writer - Indicates that RGB to YUV converter is enabled"]
pub type Cfgrgb2yuvenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFGLAYER0EN` reader - Indicates that layer 0 is enabled"]
pub type Cfglayer0enR = crate::BitReader;
#[doc = "Field `CFGLAYER0EN` writer - Indicates that layer 0 is enabled"]
pub type Cfglayer0enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFGLAYER0BLENDER` reader - Indicates that layer 0 has blender"]
pub type Cfglayer0blenderR = crate::BitReader;
#[doc = "Field `CFGLAYER0BLENDER` writer - Indicates that layer 0 has blender"]
pub type Cfglayer0blenderW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFGLAYER0SCALAR` reader - Indicates that layer 0 has scaler"]
pub type Cfglayer0scalarR = crate::BitReader;
#[doc = "Field `CFGLAYER0SCALAR` writer - Indicates that layer 0 has scaler"]
pub type Cfglayer0scalarW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFGLAYER0GAMMALUT` reader - Indicates that layer 0 has gamma LUT"]
pub type Cfglayer0gammalutR = crate::BitReader;
#[doc = "Field `CFGLAYER0GAMMALUT` writer - Indicates that layer 0 has gamma LUT"]
pub type Cfglayer0gammalutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFGLAYER1EN` reader - Indicates that layer 1 is enabled"]
pub type Cfglayer1enR = crate::BitReader;
#[doc = "Field `CFGLAYER1EN` writer - Indicates that layer 1 is enabled"]
pub type Cfglayer1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFGLAYER1BLENDER` reader - Indicates that layer 1 has blender"]
pub type Cfglayer1blenderR = crate::BitReader;
#[doc = "Field `CFGLAYER1BLENDER` writer - Indicates that layer 1 has blender"]
pub type Cfglayer1blenderW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFGLAYER1SCALAR` reader - Indicates that layer 1 has scaler"]
pub type Cfglayer1scalarR = crate::BitReader;
#[doc = "Field `CFGLAYER1SCALAR` writer - Indicates that layer 1 has scaler"]
pub type Cfglayer1scalarW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFGLAYER1GAMMALUT` reader - Indicates that layer 1 has gamma LUT"]
pub type Cfglayer1gammalutR = crate::BitReader;
#[doc = "Field `CFGLAYER1GAMMALUT` writer - Indicates that layer 1 has gamma LUT"]
pub type Cfglayer1gammalutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFGLAYER2EN` reader - Indicates that layer 2 is enabled"]
pub type Cfglayer2enR = crate::BitReader;
#[doc = "Field `CFGLAYER2EN` writer - Indicates that layer 2 is enabled"]
pub type Cfglayer2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFGLAYER2BLENDER` reader - Indicates that layer 2 has blender"]
pub type Cfglayer2blenderR = crate::BitReader;
#[doc = "Field `CFGLAYER2BLENDER` writer - Indicates that layer 2 has blender"]
pub type Cfglayer2blenderW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFGLAYER2SCALAR` reader - Indicates that layer 2 has scaler"]
pub type Cfglayer2scalarR = crate::BitReader;
#[doc = "Field `CFGLAYER2SCALAR` writer - Indicates that layer 2 has scaler"]
pub type Cfglayer2scalarW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFGLAYER2GAMMALUT` reader - Indicates that layer 2 has gamma LUT"]
pub type Cfglayer2gammalutR = crate::BitReader;
#[doc = "Field `CFGLAYER2GAMMALUT` writer - Indicates that layer 2 has gamma LUT"]
pub type Cfglayer2gammalutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFGLAYER3EN` reader - Indicates that layer 3 is enabled"]
pub type Cfglayer3enR = crate::BitReader;
#[doc = "Field `CFGLAYER3EN` writer - Indicates that layer 3 is enabled"]
pub type Cfglayer3enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFGLAYER3BLENDER` reader - Indicates that layer 3 has blender"]
pub type Cfglayer3blenderR = crate::BitReader;
#[doc = "Field `CFGLAYER3BLENDER` writer - Indicates that layer 3 has blender"]
pub type Cfglayer3blenderW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFGLAYER3SCALAR` reader - Indicates that layer 3 has scaler"]
pub type Cfglayer3scalarR = crate::BitReader;
#[doc = "Field `CFGLAYER3SCALAR` writer - Indicates that layer 3 has scaler"]
pub type Cfglayer3scalarW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFGLAYER3GAMMALUT` reader - Indicates that layer 3 has gamma LUT"]
pub type Cfglayer3gammalutR = crate::BitReader;
#[doc = "Field `CFGLAYER3GAMMALUT` writer - Indicates that layer 3 has gamma LUT"]
pub type Cfglayer3gammalutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD` reader - This field is reserved."]
pub type RsvdR = crate::FieldReader;
#[doc = "Field `RSVD` writer - This field is reserved."]
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Indicates that Global Gamma/Palette is enabled"]
    #[inline(always)]
    pub fn cfgglbgammaen(&self) -> CfgglbgammaenR {
        CfgglbgammaenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Indicates that fixed cursor is enabled"]
    #[inline(always)]
    pub fn cfgfcursoren(&self) -> CfgfcursorenR {
        CfgfcursorenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Indicates that programmable cursor is enabled"]
    #[inline(always)]
    pub fn cfgpcursoren(&self) -> CfgpcursorenR {
        CfgpcursorenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Indicates that dithering is enabled"]
    #[inline(always)]
    pub fn cfgditheren(&self) -> CfgditherenR {
        CfgditherenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Indicates that formatting is enabled"]
    #[inline(always)]
    pub fn cfgformatten(&self) -> CfgformattenR {
        CfgformattenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Indicates that high quality YUV converter is enabled"]
    #[inline(always)]
    pub fn cfgyuvcnvten(&self) -> CfgyuvcnvtenR {
        CfgyuvcnvtenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Indicates that DBI Type-B interface is enabled"]
    #[inline(always)]
    pub fn cfgdbitypeben(&self) -> CfgdbitypebenR {
        CfgdbitypebenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Indicates that RGB to YUV converter is enabled"]
    #[inline(always)]
    pub fn cfgrgb2yuven(&self) -> Cfgrgb2yuvenR {
        Cfgrgb2yuvenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Indicates that layer 0 is enabled"]
    #[inline(always)]
    pub fn cfglayer0en(&self) -> Cfglayer0enR {
        Cfglayer0enR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Indicates that layer 0 has blender"]
    #[inline(always)]
    pub fn cfglayer0blender(&self) -> Cfglayer0blenderR {
        Cfglayer0blenderR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Indicates that layer 0 has scaler"]
    #[inline(always)]
    pub fn cfglayer0scalar(&self) -> Cfglayer0scalarR {
        Cfglayer0scalarR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Indicates that layer 0 has gamma LUT"]
    #[inline(always)]
    pub fn cfglayer0gammalut(&self) -> Cfglayer0gammalutR {
        Cfglayer0gammalutR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Indicates that layer 1 is enabled"]
    #[inline(always)]
    pub fn cfglayer1en(&self) -> Cfglayer1enR {
        Cfglayer1enR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Indicates that layer 1 has blender"]
    #[inline(always)]
    pub fn cfglayer1blender(&self) -> Cfglayer1blenderR {
        Cfglayer1blenderR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Indicates that layer 1 has scaler"]
    #[inline(always)]
    pub fn cfglayer1scalar(&self) -> Cfglayer1scalarR {
        Cfglayer1scalarR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Indicates that layer 1 has gamma LUT"]
    #[inline(always)]
    pub fn cfglayer1gammalut(&self) -> Cfglayer1gammalutR {
        Cfglayer1gammalutR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Indicates that layer 2 is enabled"]
    #[inline(always)]
    pub fn cfglayer2en(&self) -> Cfglayer2enR {
        Cfglayer2enR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Indicates that layer 2 has blender"]
    #[inline(always)]
    pub fn cfglayer2blender(&self) -> Cfglayer2blenderR {
        Cfglayer2blenderR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Indicates that layer 2 has scaler"]
    #[inline(always)]
    pub fn cfglayer2scalar(&self) -> Cfglayer2scalarR {
        Cfglayer2scalarR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Indicates that layer 2 has gamma LUT"]
    #[inline(always)]
    pub fn cfglayer2gammalut(&self) -> Cfglayer2gammalutR {
        Cfglayer2gammalutR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Indicates that layer 3 is enabled"]
    #[inline(always)]
    pub fn cfglayer3en(&self) -> Cfglayer3enR {
        Cfglayer3enR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Indicates that layer 3 has blender"]
    #[inline(always)]
    pub fn cfglayer3blender(&self) -> Cfglayer3blenderR {
        Cfglayer3blenderR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Indicates that layer 3 has scaler"]
    #[inline(always)]
    pub fn cfglayer3scalar(&self) -> Cfglayer3scalarR {
        Cfglayer3scalarR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Indicates that layer 3 has gamma LUT"]
    #[inline(always)]
    pub fn cfglayer3gammalut(&self) -> Cfglayer3gammalutR {
        Cfglayer3gammalutR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:31 - This field is reserved."]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Indicates that Global Gamma/Palette is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn cfgglbgammaen(&mut self) -> CfgglbgammaenW<ConfgSpec> {
        CfgglbgammaenW::new(self, 0)
    }
    #[doc = "Bit 1 - Indicates that fixed cursor is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn cfgfcursoren(&mut self) -> CfgfcursorenW<ConfgSpec> {
        CfgfcursorenW::new(self, 1)
    }
    #[doc = "Bit 2 - Indicates that programmable cursor is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn cfgpcursoren(&mut self) -> CfgpcursorenW<ConfgSpec> {
        CfgpcursorenW::new(self, 2)
    }
    #[doc = "Bit 3 - Indicates that dithering is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn cfgditheren(&mut self) -> CfgditherenW<ConfgSpec> {
        CfgditherenW::new(self, 3)
    }
    #[doc = "Bit 4 - Indicates that formatting is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn cfgformatten(&mut self) -> CfgformattenW<ConfgSpec> {
        CfgformattenW::new(self, 4)
    }
    #[doc = "Bit 5 - Indicates that high quality YUV converter is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn cfgyuvcnvten(&mut self) -> CfgyuvcnvtenW<ConfgSpec> {
        CfgyuvcnvtenW::new(self, 5)
    }
    #[doc = "Bit 6 - Indicates that DBI Type-B interface is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn cfgdbitypeben(&mut self) -> CfgdbitypebenW<ConfgSpec> {
        CfgdbitypebenW::new(self, 6)
    }
    #[doc = "Bit 7 - Indicates that RGB to YUV converter is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn cfgrgb2yuven(&mut self) -> Cfgrgb2yuvenW<ConfgSpec> {
        Cfgrgb2yuvenW::new(self, 7)
    }
    #[doc = "Bit 8 - Indicates that layer 0 is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn cfglayer0en(&mut self) -> Cfglayer0enW<ConfgSpec> {
        Cfglayer0enW::new(self, 8)
    }
    #[doc = "Bit 9 - Indicates that layer 0 has blender"]
    #[inline(always)]
    #[must_use]
    pub fn cfglayer0blender(&mut self) -> Cfglayer0blenderW<ConfgSpec> {
        Cfglayer0blenderW::new(self, 9)
    }
    #[doc = "Bit 10 - Indicates that layer 0 has scaler"]
    #[inline(always)]
    #[must_use]
    pub fn cfglayer0scalar(&mut self) -> Cfglayer0scalarW<ConfgSpec> {
        Cfglayer0scalarW::new(self, 10)
    }
    #[doc = "Bit 11 - Indicates that layer 0 has gamma LUT"]
    #[inline(always)]
    #[must_use]
    pub fn cfglayer0gammalut(&mut self) -> Cfglayer0gammalutW<ConfgSpec> {
        Cfglayer0gammalutW::new(self, 11)
    }
    #[doc = "Bit 12 - Indicates that layer 1 is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn cfglayer1en(&mut self) -> Cfglayer1enW<ConfgSpec> {
        Cfglayer1enW::new(self, 12)
    }
    #[doc = "Bit 13 - Indicates that layer 1 has blender"]
    #[inline(always)]
    #[must_use]
    pub fn cfglayer1blender(&mut self) -> Cfglayer1blenderW<ConfgSpec> {
        Cfglayer1blenderW::new(self, 13)
    }
    #[doc = "Bit 14 - Indicates that layer 1 has scaler"]
    #[inline(always)]
    #[must_use]
    pub fn cfglayer1scalar(&mut self) -> Cfglayer1scalarW<ConfgSpec> {
        Cfglayer1scalarW::new(self, 14)
    }
    #[doc = "Bit 15 - Indicates that layer 1 has gamma LUT"]
    #[inline(always)]
    #[must_use]
    pub fn cfglayer1gammalut(&mut self) -> Cfglayer1gammalutW<ConfgSpec> {
        Cfglayer1gammalutW::new(self, 15)
    }
    #[doc = "Bit 16 - Indicates that layer 2 is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn cfglayer2en(&mut self) -> Cfglayer2enW<ConfgSpec> {
        Cfglayer2enW::new(self, 16)
    }
    #[doc = "Bit 17 - Indicates that layer 2 has blender"]
    #[inline(always)]
    #[must_use]
    pub fn cfglayer2blender(&mut self) -> Cfglayer2blenderW<ConfgSpec> {
        Cfglayer2blenderW::new(self, 17)
    }
    #[doc = "Bit 18 - Indicates that layer 2 has scaler"]
    #[inline(always)]
    #[must_use]
    pub fn cfglayer2scalar(&mut self) -> Cfglayer2scalarW<ConfgSpec> {
        Cfglayer2scalarW::new(self, 18)
    }
    #[doc = "Bit 19 - Indicates that layer 2 has gamma LUT"]
    #[inline(always)]
    #[must_use]
    pub fn cfglayer2gammalut(&mut self) -> Cfglayer2gammalutW<ConfgSpec> {
        Cfglayer2gammalutW::new(self, 19)
    }
    #[doc = "Bit 20 - Indicates that layer 3 is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn cfglayer3en(&mut self) -> Cfglayer3enW<ConfgSpec> {
        Cfglayer3enW::new(self, 20)
    }
    #[doc = "Bit 21 - Indicates that layer 3 has blender"]
    #[inline(always)]
    #[must_use]
    pub fn cfglayer3blender(&mut self) -> Cfglayer3blenderW<ConfgSpec> {
        Cfglayer3blenderW::new(self, 21)
    }
    #[doc = "Bit 22 - Indicates that layer 3 has scaler"]
    #[inline(always)]
    #[must_use]
    pub fn cfglayer3scalar(&mut self) -> Cfglayer3scalarW<ConfgSpec> {
        Cfglayer3scalarW::new(self, 22)
    }
    #[doc = "Bit 23 - Indicates that layer 3 has gamma LUT"]
    #[inline(always)]
    #[must_use]
    pub fn cfglayer3gammalut(&mut self) -> Cfglayer3gammalutW<ConfgSpec> {
        Cfglayer3gammalutW::new(self, 23)
    }
    #[doc = "Bits 24:31 - This field is reserved."]
    #[inline(always)]
    #[must_use]
    pub fn rsvd(&mut self) -> RsvdW<ConfgSpec> {
        RsvdW::new(self, 24)
    }
}
#[doc = "Information of the layers n activation and setup.\n\nYou can [`read`](crate::Reg::read) this register and get [`confg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`confg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConfgSpec;
impl crate::RegisterSpec for ConfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`confg::R`](R) reader structure"]
impl crate::Readable for ConfgSpec {}
#[doc = "`write(|w| ..)` method takes [`confg::W`](W) writer structure"]
impl crate::Writable for ConfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONFG to value 0xf000_0000"]
impl crate::Resettable for ConfgSpec {
    const RESET_VALUE: u32 = 0xf000_0000;
}
