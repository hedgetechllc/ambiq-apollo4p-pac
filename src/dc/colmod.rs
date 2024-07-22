#[doc = "Register `COLMOD` reader"]
pub type R = crate::R<ColmodSpec>;
#[doc = "Register `COLMOD` writer"]
pub type W = crate::W<ColmodSpec>;
#[doc = "Field `CLMDTSC4TSC6` reader - Indicates that the TSc4/TSc6 propietary color format is enabled"]
pub type Clmdtsc4tsc6R = crate::BitReader;
#[doc = "Field `CLMDTSC4TSC6` writer - Indicates that the TSc4/TSc6 propietary color format is enabled"]
pub type Clmdtsc4tsc6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLMDTLYUV420` reader - Indicates that the TLYUV420 color format is enabled"]
pub type Clmdtlyuv420R = crate::BitReader;
#[doc = "Field `CLMDTLYUV420` writer - Indicates that the TLYUV420 color format is enabled"]
pub type Clmdtlyuv420W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLMDVYUV420` reader - Indicates that the V_YUV420 color format is enabled"]
pub type Clmdvyuv420R = crate::BitReader;
#[doc = "Field `CLMDVYUV420` writer - Indicates that the V_YUV420 color format is enabled"]
pub type Clmdvyuv420W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLMDBGRA8888` reader - Indicates that the BGRA8888 32-bit color format is enabled"]
pub type Clmdbgra8888R = crate::BitReader;
#[doc = "Field `CLMDBGRA8888` writer - Indicates that the BGRA8888 32-bit color format is enabled"]
pub type Clmdbgra8888W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLMDABGR8888` reader - Indicates that the ABGR8888 32-bit color format is enabled"]
pub type Clmdabgr8888R = crate::BitReader;
#[doc = "Field `CLMDABGR8888` writer - Indicates that the ABGR8888 32-bit color format is enabled"]
pub type Clmdabgr8888W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLMDYUY2` reader - Indicates that the YUY2 color format is enabled"]
pub type Clmdyuy2R = crate::BitReader;
#[doc = "Field `CLMDYUY2` writer - Indicates that the YUY2 color format is enabled"]
pub type Clmdyuy2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLMDRGB888` reader - Indicates that the RGB888 24-bit color format is enabled"]
pub type Clmdrgb888R = crate::BitReader;
#[doc = "Field `CLMDRGB888` writer - Indicates that the RGB888 24-bit color format is enabled"]
pub type Clmdrgb888W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLMDYUYV` reader - Indicates that the YUYV color format is enabled"]
pub type ClmdyuyvR = crate::BitReader;
#[doc = "Field `CLMDYUYV` writer - Indicates that the YUYV color format is enabled"]
pub type ClmdyuyvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLMDL4` reader - Indicates that the L4 color format is enabled"]
pub type Clmdl4R = crate::BitReader;
#[doc = "Field `CLMDL4` writer - Indicates that the L4 color format is enabled"]
pub type Clmdl4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLMDL1` reader - Indicates that the L1 color format is enabled"]
pub type Clmdl1R = crate::BitReader;
#[doc = "Field `CLMDL1` writer - Indicates that the L1 color format is enabled"]
pub type Clmdl1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLMDL8` reader - Indicates that the L8 color format is enabled"]
pub type Clmdl8R = crate::BitReader;
#[doc = "Field `CLMDL8` writer - Indicates that the L8 color format is enabled"]
pub type Clmdl8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLMDARGB8888` reader - Indicates that the ARGB8888 32-bit color format is enabled"]
pub type Clmdargb8888R = crate::BitReader;
#[doc = "Field `CLMDARGB8888` writer - Indicates that the ARGB8888 32-bit color format is enabled"]
pub type Clmdargb8888W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLMDRGB565` reader - Indicates that the RGB565 16-bit color format is enabled"]
pub type Clmdrgb565R = crate::BitReader;
#[doc = "Field `CLMDRGB565` writer - Indicates that the RGB565 16-bit color format is enabled"]
pub type Clmdrgb565W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLMDRGB332` reader - Indicates that the RGB332 8-bit color format is enabled"]
pub type Clmdrgb332R = crate::BitReader;
#[doc = "Field `CLMDRGB332` writer - Indicates that the RGB332 8-bit color format is enabled"]
pub type Clmdrgb332W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLMDRGBA8888` reader - Indicates that the RGBA8888 32-bit color format is enabled"]
pub type Clmdrgba8888R = crate::BitReader;
#[doc = "Field `CLMDRGBA8888` writer - Indicates that the RGBA8888 32-bit color format is enabled"]
pub type Clmdrgba8888W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLMDRGBA5551` reader - Indicates that the RGBA5551 16-bit color format is enabled"]
pub type Clmdrgba5551R = crate::BitReader;
#[doc = "Field `CLMDRGBA5551` writer - Indicates that the RGBA5551 16-bit color format is enabled"]
pub type Clmdrgba5551W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLMDLUT8` reader - Indicates that the LUT8 color format is enabled"]
pub type Clmdlut8R = crate::BitReader;
#[doc = "Field `CLMDLUT8` writer - Indicates that the LUT8 color format is enabled"]
pub type Clmdlut8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLMDTSC` reader - Indicates that back pressure support for the DBI Type B interface is enabled"]
pub type ClmdtscR = crate::BitReader;
#[doc = "Field `CLMDTSC` writer - Indicates that back pressure support for the DBI Type B interface is enabled"]
pub type ClmdtscW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLMDTSC6` reader - Indicates that back pressure support for the DBI Type B interface is enabled"]
pub type Clmdtsc6R = crate::BitReader;
#[doc = "Field `CLMDTSC6` writer - Indicates that back pressure support for the DBI Type B interface is enabled"]
pub type Clmdtsc6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLMDDBIBEXTCTRL` reader - Indicates that back pressure support for the DBI Type B interface is enabled"]
pub type ClmddbibextctrlR = crate::BitReader;
#[doc = "Field `CLMDDBIBEXTCTRL` writer - Indicates that back pressure support for the DBI Type B interface is enabled"]
pub type ClmddbibextctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLMDQPI` reader - Indicates that back pressure support for the DBI Type B interface is enabled"]
pub type ClmdqpiR = crate::BitReader;
#[doc = "Field `CLMDQPI` writer - Indicates that back pressure support for the DBI Type B interface is enabled"]
pub type ClmdqpiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLMDRGBA4444` reader - Indicates that back pressure support for the DBI Type B interface is enabled"]
pub type Clmdrgba4444R = crate::BitReader;
#[doc = "Field `CLMDRGBA4444` writer - Indicates that back pressure support for the DBI Type B interface is enabled"]
pub type Clmdrgba4444W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLMDARGB4444` reader - Indicates that back pressure support for the DBI Type B interface is enabled"]
pub type Clmdargb4444R = crate::BitReader;
#[doc = "Field `CLMDARGB4444` writer - Indicates that back pressure support for the DBI Type B interface is enabled"]
pub type Clmdargb4444W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLMDJDI` reader - Indicates that back pressure support for the DBI Type B interface is enabled"]
pub type ClmdjdiR = crate::BitReader;
#[doc = "Field `CLMDJDI` writer - Indicates that back pressure support for the DBI Type B interface is enabled"]
pub type ClmdjdiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLMDLVDS` reader - Indicates that back pressure support for the DBI Type B interface is enabled"]
pub type ClmdlvdsR = crate::BitReader;
#[doc = "Field `CLMDLVDS` writer - Indicates that back pressure support for the DBI Type B interface is enabled"]
pub type ClmdlvdsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLMDBKPRESSURE` reader - Indicates that back pressure support for the DBI Type B interface is enabled"]
pub type ClmdbkpressureR = crate::BitReader;
#[doc = "Field `CLMDBKPRESSURE` writer - Indicates that back pressure support for the DBI Type B interface is enabled"]
pub type ClmdbkpressureW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Indicates that the TSc4/TSc6 propietary color format is enabled"]
    #[inline(always)]
    pub fn clmdtsc4tsc6(&self) -> Clmdtsc4tsc6R {
        Clmdtsc4tsc6R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Indicates that the TLYUV420 color format is enabled"]
    #[inline(always)]
    pub fn clmdtlyuv420(&self) -> Clmdtlyuv420R {
        Clmdtlyuv420R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Indicates that the V_YUV420 color format is enabled"]
    #[inline(always)]
    pub fn clmdvyuv420(&self) -> Clmdvyuv420R {
        Clmdvyuv420R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Indicates that the BGRA8888 32-bit color format is enabled"]
    #[inline(always)]
    pub fn clmdbgra8888(&self) -> Clmdbgra8888R {
        Clmdbgra8888R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Indicates that the ABGR8888 32-bit color format is enabled"]
    #[inline(always)]
    pub fn clmdabgr8888(&self) -> Clmdabgr8888R {
        Clmdabgr8888R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Indicates that the YUY2 color format is enabled"]
    #[inline(always)]
    pub fn clmdyuy2(&self) -> Clmdyuy2R {
        Clmdyuy2R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Indicates that the RGB888 24-bit color format is enabled"]
    #[inline(always)]
    pub fn clmdrgb888(&self) -> Clmdrgb888R {
        Clmdrgb888R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Indicates that the YUYV color format is enabled"]
    #[inline(always)]
    pub fn clmdyuyv(&self) -> ClmdyuyvR {
        ClmdyuyvR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Indicates that the L4 color format is enabled"]
    #[inline(always)]
    pub fn clmdl4(&self) -> Clmdl4R {
        Clmdl4R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Indicates that the L1 color format is enabled"]
    #[inline(always)]
    pub fn clmdl1(&self) -> Clmdl1R {
        Clmdl1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Indicates that the L8 color format is enabled"]
    #[inline(always)]
    pub fn clmdl8(&self) -> Clmdl8R {
        Clmdl8R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Indicates that the ARGB8888 32-bit color format is enabled"]
    #[inline(always)]
    pub fn clmdargb8888(&self) -> Clmdargb8888R {
        Clmdargb8888R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Indicates that the RGB565 16-bit color format is enabled"]
    #[inline(always)]
    pub fn clmdrgb565(&self) -> Clmdrgb565R {
        Clmdrgb565R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Indicates that the RGB332 8-bit color format is enabled"]
    #[inline(always)]
    pub fn clmdrgb332(&self) -> Clmdrgb332R {
        Clmdrgb332R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Indicates that the RGBA8888 32-bit color format is enabled"]
    #[inline(always)]
    pub fn clmdrgba8888(&self) -> Clmdrgba8888R {
        Clmdrgba8888R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Indicates that the RGBA5551 16-bit color format is enabled"]
    #[inline(always)]
    pub fn clmdrgba5551(&self) -> Clmdrgba5551R {
        Clmdrgba5551R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Indicates that the LUT8 color format is enabled"]
    #[inline(always)]
    pub fn clmdlut8(&self) -> Clmdlut8R {
        Clmdlut8R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Indicates that back pressure support for the DBI Type B interface is enabled"]
    #[inline(always)]
    pub fn clmdtsc(&self) -> ClmdtscR {
        ClmdtscR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Indicates that back pressure support for the DBI Type B interface is enabled"]
    #[inline(always)]
    pub fn clmdtsc6(&self) -> Clmdtsc6R {
        Clmdtsc6R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Indicates that back pressure support for the DBI Type B interface is enabled"]
    #[inline(always)]
    pub fn clmddbibextctrl(&self) -> ClmddbibextctrlR {
        ClmddbibextctrlR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Indicates that back pressure support for the DBI Type B interface is enabled"]
    #[inline(always)]
    pub fn clmdqpi(&self) -> ClmdqpiR {
        ClmdqpiR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Indicates that back pressure support for the DBI Type B interface is enabled"]
    #[inline(always)]
    pub fn clmdrgba4444(&self) -> Clmdrgba4444R {
        Clmdrgba4444R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Indicates that back pressure support for the DBI Type B interface is enabled"]
    #[inline(always)]
    pub fn clmdargb4444(&self) -> Clmdargb4444R {
        Clmdargb4444R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 29 - Indicates that back pressure support for the DBI Type B interface is enabled"]
    #[inline(always)]
    pub fn clmdjdi(&self) -> ClmdjdiR {
        ClmdjdiR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Indicates that back pressure support for the DBI Type B interface is enabled"]
    #[inline(always)]
    pub fn clmdlvds(&self) -> ClmdlvdsR {
        ClmdlvdsR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Indicates that back pressure support for the DBI Type B interface is enabled"]
    #[inline(always)]
    pub fn clmdbkpressure(&self) -> ClmdbkpressureR {
        ClmdbkpressureR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Indicates that the TSc4/TSc6 propietary color format is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn clmdtsc4tsc6(&mut self) -> Clmdtsc4tsc6W<ColmodSpec> {
        Clmdtsc4tsc6W::new(self, 0)
    }
    #[doc = "Bit 1 - Indicates that the TLYUV420 color format is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn clmdtlyuv420(&mut self) -> Clmdtlyuv420W<ColmodSpec> {
        Clmdtlyuv420W::new(self, 1)
    }
    #[doc = "Bit 2 - Indicates that the V_YUV420 color format is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn clmdvyuv420(&mut self) -> Clmdvyuv420W<ColmodSpec> {
        Clmdvyuv420W::new(self, 2)
    }
    #[doc = "Bit 3 - Indicates that the BGRA8888 32-bit color format is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn clmdbgra8888(&mut self) -> Clmdbgra8888W<ColmodSpec> {
        Clmdbgra8888W::new(self, 3)
    }
    #[doc = "Bit 4 - Indicates that the ABGR8888 32-bit color format is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn clmdabgr8888(&mut self) -> Clmdabgr8888W<ColmodSpec> {
        Clmdabgr8888W::new(self, 4)
    }
    #[doc = "Bit 5 - Indicates that the YUY2 color format is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn clmdyuy2(&mut self) -> Clmdyuy2W<ColmodSpec> {
        Clmdyuy2W::new(self, 5)
    }
    #[doc = "Bit 6 - Indicates that the RGB888 24-bit color format is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn clmdrgb888(&mut self) -> Clmdrgb888W<ColmodSpec> {
        Clmdrgb888W::new(self, 6)
    }
    #[doc = "Bit 7 - Indicates that the YUYV color format is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn clmdyuyv(&mut self) -> ClmdyuyvW<ColmodSpec> {
        ClmdyuyvW::new(self, 7)
    }
    #[doc = "Bit 8 - Indicates that the L4 color format is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn clmdl4(&mut self) -> Clmdl4W<ColmodSpec> {
        Clmdl4W::new(self, 8)
    }
    #[doc = "Bit 9 - Indicates that the L1 color format is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn clmdl1(&mut self) -> Clmdl1W<ColmodSpec> {
        Clmdl1W::new(self, 9)
    }
    #[doc = "Bit 10 - Indicates that the L8 color format is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn clmdl8(&mut self) -> Clmdl8W<ColmodSpec> {
        Clmdl8W::new(self, 10)
    }
    #[doc = "Bit 11 - Indicates that the ARGB8888 32-bit color format is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn clmdargb8888(&mut self) -> Clmdargb8888W<ColmodSpec> {
        Clmdargb8888W::new(self, 11)
    }
    #[doc = "Bit 12 - Indicates that the RGB565 16-bit color format is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn clmdrgb565(&mut self) -> Clmdrgb565W<ColmodSpec> {
        Clmdrgb565W::new(self, 12)
    }
    #[doc = "Bit 13 - Indicates that the RGB332 8-bit color format is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn clmdrgb332(&mut self) -> Clmdrgb332W<ColmodSpec> {
        Clmdrgb332W::new(self, 13)
    }
    #[doc = "Bit 14 - Indicates that the RGBA8888 32-bit color format is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn clmdrgba8888(&mut self) -> Clmdrgba8888W<ColmodSpec> {
        Clmdrgba8888W::new(self, 14)
    }
    #[doc = "Bit 15 - Indicates that the RGBA5551 16-bit color format is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn clmdrgba5551(&mut self) -> Clmdrgba5551W<ColmodSpec> {
        Clmdrgba5551W::new(self, 15)
    }
    #[doc = "Bit 16 - Indicates that the LUT8 color format is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn clmdlut8(&mut self) -> Clmdlut8W<ColmodSpec> {
        Clmdlut8W::new(self, 16)
    }
    #[doc = "Bit 17 - Indicates that back pressure support for the DBI Type B interface is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn clmdtsc(&mut self) -> ClmdtscW<ColmodSpec> {
        ClmdtscW::new(self, 17)
    }
    #[doc = "Bit 18 - Indicates that back pressure support for the DBI Type B interface is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn clmdtsc6(&mut self) -> Clmdtsc6W<ColmodSpec> {
        Clmdtsc6W::new(self, 18)
    }
    #[doc = "Bit 19 - Indicates that back pressure support for the DBI Type B interface is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn clmddbibextctrl(&mut self) -> ClmddbibextctrlW<ColmodSpec> {
        ClmddbibextctrlW::new(self, 19)
    }
    #[doc = "Bit 20 - Indicates that back pressure support for the DBI Type B interface is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn clmdqpi(&mut self) -> ClmdqpiW<ColmodSpec> {
        ClmdqpiW::new(self, 20)
    }
    #[doc = "Bit 21 - Indicates that back pressure support for the DBI Type B interface is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn clmdrgba4444(&mut self) -> Clmdrgba4444W<ColmodSpec> {
        Clmdrgba4444W::new(self, 21)
    }
    #[doc = "Bit 22 - Indicates that back pressure support for the DBI Type B interface is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn clmdargb4444(&mut self) -> Clmdargb4444W<ColmodSpec> {
        Clmdargb4444W::new(self, 22)
    }
    #[doc = "Bit 29 - Indicates that back pressure support for the DBI Type B interface is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn clmdjdi(&mut self) -> ClmdjdiW<ColmodSpec> {
        ClmdjdiW::new(self, 29)
    }
    #[doc = "Bit 30 - Indicates that back pressure support for the DBI Type B interface is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn clmdlvds(&mut self) -> ClmdlvdsW<ColmodSpec> {
        ClmdlvdsW::new(self, 30)
    }
    #[doc = "Bit 31 - Indicates that back pressure support for the DBI Type B interface is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn clmdbkpressure(&mut self) -> ClmdbkpressureW<ColmodSpec> {
        ClmdbkpressureW::new(self, 31)
    }
}
#[doc = "Color mode status register indicating formats/back pressure are enabled.\n\nYou can [`read`](crate::Reg::read) this register and get [`colmod::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`colmod::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ColmodSpec;
impl crate::RegisterSpec for ColmodSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`colmod::R`](R) reader structure"]
impl crate::Readable for ColmodSpec {}
#[doc = "`write(|w| ..)` method takes [`colmod::W`](W) writer structure"]
impl crate::Writable for ColmodSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets COLMOD to value 0x006e_0000"]
impl crate::Resettable for ColmodSpec {
    const RESET_VALUE: u32 = 0x006e_0000;
}
