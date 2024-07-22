#[doc = "Register `RNGDMASRCMASK` reader"]
pub type R = crate::R<RngdmasrcmaskSpec>;
#[doc = "Register `RNGDMASRCMASK` writer"]
pub type W = crate::W<RngdmasrcmaskSpec>;
#[doc = "Field `ENSRCSEL0` reader - Writing value 0x1 enables SRC_SEL 0."]
pub type Ensrcsel0R = crate::BitReader;
#[doc = "Field `ENSRCSEL0` writer - Writing value 0x1 enables SRC_SEL 0."]
pub type Ensrcsel0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENSRCSEL1` reader - Writing value 0x1 enables SRC_SEL 1."]
pub type Ensrcsel1R = crate::BitReader;
#[doc = "Field `ENSRCSEL1` writer - Writing value 0x1 enables SRC_SEL 1."]
pub type Ensrcsel1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENSRCSEL2` reader - Writing value 0x1 enables SRC_SEL 2."]
pub type Ensrcsel2R = crate::BitReader;
#[doc = "Field `ENSRCSEL2` writer - Writing value 0x1 enables SRC_SEL 2."]
pub type Ensrcsel2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENSRCSEL3` reader - Writing value 0x1 enables SRC_SEL 3."]
pub type Ensrcsel3R = crate::BitReader;
#[doc = "Field `ENSRCSEL3` writer - Writing value 0x1 enables SRC_SEL 3."]
pub type Ensrcsel3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Writing value 0x1 enables SRC_SEL 0."]
    #[inline(always)]
    pub fn ensrcsel0(&self) -> Ensrcsel0R {
        Ensrcsel0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Writing value 0x1 enables SRC_SEL 1."]
    #[inline(always)]
    pub fn ensrcsel1(&self) -> Ensrcsel1R {
        Ensrcsel1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Writing value 0x1 enables SRC_SEL 2."]
    #[inline(always)]
    pub fn ensrcsel2(&self) -> Ensrcsel2R {
        Ensrcsel2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Writing value 0x1 enables SRC_SEL 3."]
    #[inline(always)]
    pub fn ensrcsel3(&self) -> Ensrcsel3R {
        Ensrcsel3R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Writing value 0x1 enables SRC_SEL 0."]
    #[inline(always)]
    #[must_use]
    pub fn ensrcsel0(&mut self) -> Ensrcsel0W<RngdmasrcmaskSpec> {
        Ensrcsel0W::new(self, 0)
    }
    #[doc = "Bit 1 - Writing value 0x1 enables SRC_SEL 1."]
    #[inline(always)]
    #[must_use]
    pub fn ensrcsel1(&mut self) -> Ensrcsel1W<RngdmasrcmaskSpec> {
        Ensrcsel1W::new(self, 1)
    }
    #[doc = "Bit 2 - Writing value 0x1 enables SRC_SEL 2."]
    #[inline(always)]
    #[must_use]
    pub fn ensrcsel2(&mut self) -> Ensrcsel2W<RngdmasrcmaskSpec> {
        Ensrcsel2W::new(self, 2)
    }
    #[doc = "Bit 3 - Writing value 0x1 enables SRC_SEL 3."]
    #[inline(always)]
    #[must_use]
    pub fn ensrcsel3(&mut self) -> Ensrcsel3W<RngdmasrcmaskSpec> {
        Ensrcsel3W::new(self, 3)
    }
}
#[doc = "This register defines which ring-oscillator length should be used when using the RNG DMA.\n\nYou can [`read`](crate::Reg::read) this register and get [`rngdmasrcmask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rngdmasrcmask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RngdmasrcmaskSpec;
impl crate::RegisterSpec for RngdmasrcmaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rngdmasrcmask::R`](R) reader structure"]
impl crate::Readable for RngdmasrcmaskSpec {}
#[doc = "`write(|w| ..)` method takes [`rngdmasrcmask::W`](W) writer structure"]
impl crate::Writable for RngdmasrcmaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RNGDMASRCMASK to value 0"]
impl crate::Resettable for RngdmasrcmaskSpec {
    const RESET_VALUE: u32 = 0;
}
