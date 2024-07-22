#[doc = "Register `GAINCFG` reader"]
pub type R = crate::R<GaincfgSpec>;
#[doc = "Register `GAINCFG` writer"]
pub type W = crate::W<GaincfgSpec>;
#[doc = "Field `PGACTRLEN` reader - Enable PGA gain updates."]
pub type PgactrlenR = crate::BitReader;
#[doc = "Field `PGACTRLEN` writer - Enable PGA gain updates."]
pub type PgactrlenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "PGA update mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Updatemode {
    #[doc = "0: Immediate update mode. Once gain is written, it is immediately encoded and provided to the PGA."]
    Immed = 0,
    #[doc = "1: Update gain only at detected zero crossing as configured by ZX registers."]
    Zx = 1,
}
impl From<Updatemode> for bool {
    #[inline(always)]
    fn from(variant: Updatemode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UPDATEMODE` reader - PGA update mode"]
pub type UpdatemodeR = crate::BitReader<Updatemode>;
impl UpdatemodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Updatemode {
        match self.bits {
            false => Updatemode::Immed,
            true => Updatemode::Zx,
        }
    }
    #[doc = "Immediate update mode. Once gain is written, it is immediately encoded and provided to the PGA."]
    #[inline(always)]
    pub fn is_immed(&self) -> bool {
        *self == Updatemode::Immed
    }
    #[doc = "Update gain only at detected zero crossing as configured by ZX registers."]
    #[inline(always)]
    pub fn is_zx(&self) -> bool {
        *self == Updatemode::Zx
    }
}
#[doc = "Field `UPDATEMODE` writer - PGA update mode"]
pub type UpdatemodeW<'a, REG> = crate::BitWriter<'a, REG, Updatemode>;
impl<'a, REG> UpdatemodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Immediate update mode. Once gain is written, it is immediately encoded and provided to the PGA."]
    #[inline(always)]
    pub fn immed(self) -> &'a mut crate::W<REG> {
        self.variant(Updatemode::Immed)
    }
    #[doc = "Update gain only at detected zero crossing as configured by ZX registers."]
    #[inline(always)]
    pub fn zx(self) -> &'a mut crate::W<REG> {
        self.variant(Updatemode::Zx)
    }
}
impl R {
    #[doc = "Bit 0 - Enable PGA gain updates."]
    #[inline(always)]
    pub fn pgactrlen(&self) -> PgactrlenR {
        PgactrlenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - PGA update mode"]
    #[inline(always)]
    pub fn updatemode(&self) -> UpdatemodeR {
        UpdatemodeR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable PGA gain updates."]
    #[inline(always)]
    #[must_use]
    pub fn pgactrlen(&mut self) -> PgactrlenW<GaincfgSpec> {
        PgactrlenW::new(self, 0)
    }
    #[doc = "Bit 4 - PGA update mode"]
    #[inline(always)]
    #[must_use]
    pub fn updatemode(&mut self) -> UpdatemodeW<GaincfgSpec> {
        UpdatemodeW::new(self, 4)
    }
}
#[doc = "PGA Gain Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`gaincfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gaincfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GaincfgSpec;
impl crate::RegisterSpec for GaincfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gaincfg::R`](R) reader structure"]
impl crate::Readable for GaincfgSpec {}
#[doc = "`write(|w| ..)` method takes [`gaincfg::W`](W) writer structure"]
impl crate::Writable for GaincfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GAINCFG to value 0"]
impl crate::Resettable for GaincfgSpec {
    const RESET_VALUE: u32 = 0;
}
