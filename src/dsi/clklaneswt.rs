#[doc = "Register `CLKLANESWT` reader"]
pub type R = crate::R<ClklaneswtSpec>;
#[doc = "Register `CLKLANESWT` writer"]
pub type W = crate::W<ClklaneswtSpec>;
#[doc = "Field `HISPLPSW` reader - High speed to low power switching time in terms byte clock (txbyteclkhs). This value is based on the byte clock (txbyteclkhs) and low power clock frequency; HS to LP switch count = Tclk_trail + THS_Exit + 3 byteclk Tclk_trail = programmed value of cln_cnt_hs_trail in AHB Reg 70h bit (23:16) THS_Exit = programmed value of cln_cnt_hs_exit in AHB Reg 70h bit (31:24) Typical value - Number of byte clocks request to switch from high speed mode to low power mode after txrequesths_clk is de-asserted."]
pub type HisplpswR = crate::FieldReader<u16>;
#[doc = "Field `HISPLPSW` writer - High speed to low power switching time in terms byte clock (txbyteclkhs). This value is based on the byte clock (txbyteclkhs) and low power clock frequency; HS to LP switch count = Tclk_trail + THS_Exit + 3 byteclk Tclk_trail = programmed value of cln_cnt_hs_trail in AHB Reg 70h bit (23:16) THS_Exit = programmed value of cln_cnt_hs_exit in AHB Reg 70h bit (31:24) Typical value - Number of byte clocks request to switch from high speed mode to low power mode after txrequesths_clk is de-asserted."]
pub type HisplpswW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `LOWPWR2HI` reader - This value is based on the byte clock (txbyteclkhs) and low power clock frequency (txclkesc)LP to HS switch count = 4 * Tlpx + (programmed Tclk_prep + extracount (1 byteclk) ) + (programmed Tclk_zero + extracount (1 byteclk) ) + Tclk_pre + 2 byteclk Tlpx = Low power clock equivalence in terms of byte clock programmed in AHB reg 68h Tclk_prep = programmed value of cln_cnt_prep in AHB Reg 70h bit (7:0) Tclk_zero = programmed value of cln_cnt_zero in AHB Reg 70h bit (15:8) Tclk_pre = 8 UI Typical value x96 Number of byte clocks required to switch from low power mode to high speed mode after txrequesths_clk is asserted."]
pub type Lowpwr2hiR = crate::FieldReader<u16>;
#[doc = "Field `LOWPWR2HI` writer - This value is based on the byte clock (txbyteclkhs) and low power clock frequency (txclkesc)LP to HS switch count = 4 * Tlpx + (programmed Tclk_prep + extracount (1 byteclk) ) + (programmed Tclk_zero + extracount (1 byteclk) ) + Tclk_pre + 2 byteclk Tlpx = Low power clock equivalence in terms of byte clock programmed in AHB reg 68h Tclk_prep = programmed value of cln_cnt_prep in AHB Reg 70h bit (7:0) Tclk_zero = programmed value of cln_cnt_zero in AHB Reg 70h bit (15:8) Tclk_pre = 8 UI Typical value x96 Number of byte clocks required to switch from low power mode to high speed mode after txrequesths_clk is asserted."]
pub type Lowpwr2hiW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - High speed to low power switching time in terms byte clock (txbyteclkhs). This value is based on the byte clock (txbyteclkhs) and low power clock frequency; HS to LP switch count = Tclk_trail + THS_Exit + 3 byteclk Tclk_trail = programmed value of cln_cnt_hs_trail in AHB Reg 70h bit (23:16) THS_Exit = programmed value of cln_cnt_hs_exit in AHB Reg 70h bit (31:24) Typical value - Number of byte clocks request to switch from high speed mode to low power mode after txrequesths_clk is de-asserted."]
    #[inline(always)]
    pub fn hisplpsw(&self) -> HisplpswR {
        HisplpswR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - This value is based on the byte clock (txbyteclkhs) and low power clock frequency (txclkesc)LP to HS switch count = 4 * Tlpx + (programmed Tclk_prep + extracount (1 byteclk) ) + (programmed Tclk_zero + extracount (1 byteclk) ) + Tclk_pre + 2 byteclk Tlpx = Low power clock equivalence in terms of byte clock programmed in AHB reg 68h Tclk_prep = programmed value of cln_cnt_prep in AHB Reg 70h bit (7:0) Tclk_zero = programmed value of cln_cnt_zero in AHB Reg 70h bit (15:8) Tclk_pre = 8 UI Typical value x96 Number of byte clocks required to switch from low power mode to high speed mode after txrequesths_clk is asserted."]
    #[inline(always)]
    pub fn lowpwr2hi(&self) -> Lowpwr2hiR {
        Lowpwr2hiR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - High speed to low power switching time in terms byte clock (txbyteclkhs). This value is based on the byte clock (txbyteclkhs) and low power clock frequency; HS to LP switch count = Tclk_trail + THS_Exit + 3 byteclk Tclk_trail = programmed value of cln_cnt_hs_trail in AHB Reg 70h bit (23:16) THS_Exit = programmed value of cln_cnt_hs_exit in AHB Reg 70h bit (31:24) Typical value - Number of byte clocks request to switch from high speed mode to low power mode after txrequesths_clk is de-asserted."]
    #[inline(always)]
    #[must_use]
    pub fn hisplpsw(&mut self) -> HisplpswW<ClklaneswtSpec> {
        HisplpswW::new(self, 0)
    }
    #[doc = "Bits 16:31 - This value is based on the byte clock (txbyteclkhs) and low power clock frequency (txclkesc)LP to HS switch count = 4 * Tlpx + (programmed Tclk_prep + extracount (1 byteclk) ) + (programmed Tclk_zero + extracount (1 byteclk) ) + Tclk_pre + 2 byteclk Tlpx = Low power clock equivalence in terms of byte clock programmed in AHB reg 68h Tclk_prep = programmed value of cln_cnt_prep in AHB Reg 70h bit (7:0) Tclk_zero = programmed value of cln_cnt_zero in AHB Reg 70h bit (15:8) Tclk_pre = 8 UI Typical value x96 Number of byte clocks required to switch from low power mode to high speed mode after txrequesths_clk is asserted."]
    #[inline(always)]
    #[must_use]
    pub fn lowpwr2hi(&mut self) -> Lowpwr2hiW<ClklaneswtSpec> {
        Lowpwr2hiW::new(self, 16)
    }
}
#[doc = "High speed to low power switching time in terms ofbyte clock (txbyteclkhs)\n\nYou can [`read`](crate::Reg::read) this register and get [`clklaneswt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clklaneswt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClklaneswtSpec;
impl crate::RegisterSpec for ClklaneswtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clklaneswt::R`](R) reader structure"]
impl crate::Readable for ClklaneswtSpec {}
#[doc = "`write(|w| ..)` method takes [`clklaneswt::W`](W) writer structure"]
impl crate::Writable for ClklaneswtSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKLANESWT to value 0"]
impl crate::Resettable for ClklaneswtSpec {
    const RESET_VALUE: u32 = 0;
}
