#[doc = "Register `ERRORAUTORCOV` reader"]
pub type R = crate::R<ErrorautorcovSpec>;
#[doc = "Register `ERRORAUTORCOV` writer"]
pub type W = crate::W<ErrorautorcovSpec>;
#[doc = "Field `ECCMULERRCLR` reader - if this bit is set to 1, Ecc_mul_err_clr error recovery action is taken immediately by DSI TX"]
pub type EccmulerrclrR = crate::BitReader;
#[doc = "Field `ECCMULERRCLR` writer - if this bit is set to 1, Ecc_mul_err_clr error recovery action is taken immediately by DSI TX"]
pub type EccmulerrclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INVLDDTCLR` reader - If this bit is set to 1, Invld_dt_clr error recovery action is taken immediately by DSI TX"]
pub type InvlddtclrR = crate::BitReader;
#[doc = "Field `INVLDDTCLR` writer - If this bit is set to 1, Invld_dt_clr error recovery action is taken immediately by DSI TX"]
pub type InvlddtclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HICONTCLR` reader - If this bit is set to 1, Hi_cont_clr error recover action is taken immediately by DSI TX"]
pub type HicontclrR = crate::BitReader;
#[doc = "Field `HICONTCLR` writer - If this bit is set to 1, Hi_cont_clr error recover action is taken immediately by DSI TX"]
pub type HicontclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCONTCLR` reader - If this bit is set to 1, lo_cont_clr error recovery action is taken immediately by DSI TX"]
pub type LocontclrR = crate::BitReader;
#[doc = "Field `LOCONTCLR` writer - If this bit is set to 1, lo_cont_clr error recovery action is taken immediately by DSI TX"]
pub type LocontclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSRXTIMEOUTCLR` reader - If this bit is set to 1, Hs_rx_timeout_clr error recovery action is taken immediately by DSI TX"]
pub type HsrxtimeoutclrR = crate::BitReader;
#[doc = "Field `HSRXTIMEOUTCLR` writer - If this bit is set to 1, Hs_rx_timeout_clr error recovery action is taken immediately by DSI TX"]
pub type HsrxtimeoutclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPRXTIMEOUTCLR` reader - If this bit is set to 1, lp_rx_timeout_clr error recovery action is taken immediately by DSI TX"]
pub type LprxtimeoutclrR = crate::BitReader;
#[doc = "Field `LPRXTIMEOUTCLR` writer - If this bit is set to 1, lp_rx_timeout_clr error recovery action is taken immediately by DSI TX"]
pub type LprxtimeoutclrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - if this bit is set to 1, Ecc_mul_err_clr error recovery action is taken immediately by DSI TX"]
    #[inline(always)]
    pub fn eccmulerrclr(&self) -> EccmulerrclrR {
        EccmulerrclrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - If this bit is set to 1, Invld_dt_clr error recovery action is taken immediately by DSI TX"]
    #[inline(always)]
    pub fn invlddtclr(&self) -> InvlddtclrR {
        InvlddtclrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - If this bit is set to 1, Hi_cont_clr error recover action is taken immediately by DSI TX"]
    #[inline(always)]
    pub fn hicontclr(&self) -> HicontclrR {
        HicontclrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - If this bit is set to 1, lo_cont_clr error recovery action is taken immediately by DSI TX"]
    #[inline(always)]
    pub fn locontclr(&self) -> LocontclrR {
        LocontclrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - If this bit is set to 1, Hs_rx_timeout_clr error recovery action is taken immediately by DSI TX"]
    #[inline(always)]
    pub fn hsrxtimeoutclr(&self) -> HsrxtimeoutclrR {
        HsrxtimeoutclrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - If this bit is set to 1, lp_rx_timeout_clr error recovery action is taken immediately by DSI TX"]
    #[inline(always)]
    pub fn lprxtimeoutclr(&self) -> LprxtimeoutclrR {
        LprxtimeoutclrR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - if this bit is set to 1, Ecc_mul_err_clr error recovery action is taken immediately by DSI TX"]
    #[inline(always)]
    #[must_use]
    pub fn eccmulerrclr(&mut self) -> EccmulerrclrW<ErrorautorcovSpec> {
        EccmulerrclrW::new(self, 0)
    }
    #[doc = "Bit 1 - If this bit is set to 1, Invld_dt_clr error recovery action is taken immediately by DSI TX"]
    #[inline(always)]
    #[must_use]
    pub fn invlddtclr(&mut self) -> InvlddtclrW<ErrorautorcovSpec> {
        InvlddtclrW::new(self, 1)
    }
    #[doc = "Bit 2 - If this bit is set to 1, Hi_cont_clr error recover action is taken immediately by DSI TX"]
    #[inline(always)]
    #[must_use]
    pub fn hicontclr(&mut self) -> HicontclrW<ErrorautorcovSpec> {
        HicontclrW::new(self, 2)
    }
    #[doc = "Bit 3 - If this bit is set to 1, lo_cont_clr error recovery action is taken immediately by DSI TX"]
    #[inline(always)]
    #[must_use]
    pub fn locontclr(&mut self) -> LocontclrW<ErrorautorcovSpec> {
        LocontclrW::new(self, 3)
    }
    #[doc = "Bit 4 - If this bit is set to 1, Hs_rx_timeout_clr error recovery action is taken immediately by DSI TX"]
    #[inline(always)]
    #[must_use]
    pub fn hsrxtimeoutclr(&mut self) -> HsrxtimeoutclrW<ErrorautorcovSpec> {
        HsrxtimeoutclrW::new(self, 4)
    }
    #[doc = "Bit 5 - If this bit is set to 1, lp_rx_timeout_clr error recovery action is taken immediately by DSI TX"]
    #[inline(always)]
    #[must_use]
    pub fn lprxtimeoutclr(&mut self) -> LprxtimeoutclrW<ErrorautorcovSpec> {
        LprxtimeoutclrW::new(self, 5)
    }
}
#[doc = "Errir ayti recivert register\n\nYou can [`read`](crate::Reg::read) this register and get [`errorautorcov::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`errorautorcov::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErrorautorcovSpec;
impl crate::RegisterSpec for ErrorautorcovSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`errorautorcov::R`](R) reader structure"]
impl crate::Readable for ErrorautorcovSpec {}
#[doc = "`write(|w| ..)` method takes [`errorautorcov::W`](W) writer structure"]
impl crate::Writable for ErrorautorcovSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ERRORAUTORCOV to value 0x3f"]
impl crate::Resettable for ErrorautorcovSpec {
    const RESET_VALUE: u32 = 0x3f;
}
