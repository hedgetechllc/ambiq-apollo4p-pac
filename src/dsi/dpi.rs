#[doc = "Register `DPI` reader"]
pub type R = crate::R<DpiSpec>;
#[doc = "Register `DPI` writer"]
pub type W = crate::W<DpiSpec>;
#[doc = "Field `SHUTDOWN` reader - Set to 1 to indicate a shut down short packet has to be packetised for the DPIs virtual channel"]
pub type ShutdownR = crate::BitReader;
#[doc = "Field `SHUTDOWN` writer - Set to 1 to indicate a shut down short packet has to be packetised for the DPIs virtual channel"]
pub type ShutdownW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TURNON1` reader - Set to 1 to indicate a Turn ON short packet has to be packetised for the DPIs virtual channel"]
pub type Turnon1R = crate::BitReader;
#[doc = "Field `TURNON1` writer - Set to 1 to indicate a Turn ON short packet has to be packetised for the DPIs virtual channel"]
pub type Turnon1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COLOR` reader - MODEON Set to 1 to indicate a color Mode ON short packet has to be packetised for the DPIs virtual channel."]
pub type ColorR = crate::BitReader;
#[doc = "Field `COLOR` writer - MODEON Set to 1 to indicate a color Mode ON short packet has to be packetised for the DPIs virtual channel."]
pub type ColorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COLORMODEOFF` reader - Set to 1 to indicate a Color Mode OFF short packet has to be packetised for the DPIs virtual channel"]
pub type ColormodeoffR = crate::BitReader;
#[doc = "Field `COLORMODEOFF` writer - Set to 1 to indicate a Color Mode OFF short packet has to be packetised for the DPIs virtual channel"]
pub type ColormodeoffW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set to 1 to indicate a shut down short packet has to be packetised for the DPIs virtual channel"]
    #[inline(always)]
    pub fn shutdown(&self) -> ShutdownR {
        ShutdownR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set to 1 to indicate a Turn ON short packet has to be packetised for the DPIs virtual channel"]
    #[inline(always)]
    pub fn turnon1(&self) -> Turnon1R {
        Turnon1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MODEON Set to 1 to indicate a color Mode ON short packet has to be packetised for the DPIs virtual channel."]
    #[inline(always)]
    pub fn color(&self) -> ColorR {
        ColorR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set to 1 to indicate a Color Mode OFF short packet has to be packetised for the DPIs virtual channel"]
    #[inline(always)]
    pub fn colormodeoff(&self) -> ColormodeoffR {
        ColormodeoffR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set to 1 to indicate a shut down short packet has to be packetised for the DPIs virtual channel"]
    #[inline(always)]
    #[must_use]
    pub fn shutdown(&mut self) -> ShutdownW<DpiSpec> {
        ShutdownW::new(self, 0)
    }
    #[doc = "Bit 1 - Set to 1 to indicate a Turn ON short packet has to be packetised for the DPIs virtual channel"]
    #[inline(always)]
    #[must_use]
    pub fn turnon1(&mut self) -> Turnon1W<DpiSpec> {
        Turnon1W::new(self, 1)
    }
    #[doc = "Bit 2 - MODEON Set to 1 to indicate a color Mode ON short packet has to be packetised for the DPIs virtual channel."]
    #[inline(always)]
    #[must_use]
    pub fn color(&mut self) -> ColorW<DpiSpec> {
        ColorW::new(self, 2)
    }
    #[doc = "Bit 3 - Set to 1 to indicate a Color Mode OFF short packet has to be packetised for the DPIs virtual channel"]
    #[inline(always)]
    #[must_use]
    pub fn colormodeoff(&mut self) -> ColormodeoffW<DpiSpec> {
        ColormodeoffW::new(self, 3)
    }
}
#[doc = "DPI control register.\n\nYou can [`read`](crate::Reg::read) this register and get [`dpi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpiSpec;
impl crate::RegisterSpec for DpiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dpi::R`](R) reader structure"]
impl crate::Readable for DpiSpec {}
#[doc = "`write(|w| ..)` method takes [`dpi::W`](W) writer structure"]
impl crate::Writable for DpiSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DPI to value 0"]
impl crate::Resettable for DpiSpec {
    const RESET_VALUE: u32 = 0;
}
