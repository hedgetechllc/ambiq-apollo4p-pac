#[doc = "Register `DATALANEHILOSWCNT` reader"]
pub type R = crate::R<DatalanehiloswcntSpec>;
#[doc = "Register `DATALANEHILOSWCNT` writer"]
pub type W = crate::W<DatalanehiloswcntSpec>;
#[doc = "Field `DATALHLSWCNT` reader - High speed to low power or Low power to high speed power or Low switching time in terms byte clock (txbyteclkhs). This power to high speed switch count value is based on the byte clock (txbyteclkhs) and low power clock frequency (txclkesc); Data lane Switch count = 4 * Tlpx + programmed THS_prep + programmed THS_zero + 4 byteclk Tlpx = Low power clock equivalence in of terms byte clock programmed in AHB reg 68h; THS_prep = programmed value of dln_cnt_hs_prep in AHB Reg 6ch bit (7:0) THS_zero = programmed value of dln_cnt_hs_zero in AHB Reg 6ch bit (15:8) Programmable count should be equal or greater than this value. Typical value x96 Number of byte clocks required to switch from low power mode to high speed mode after txrequesths is asserted."]
pub type DatalhlswcntR = crate::FieldReader<u16>;
#[doc = "Field `DATALHLSWCNT` writer - High speed to low power or Low power to high speed power or Low switching time in terms byte clock (txbyteclkhs). This power to high speed switch count value is based on the byte clock (txbyteclkhs) and low power clock frequency (txclkesc); Data lane Switch count = 4 * Tlpx + programmed THS_prep + programmed THS_zero + 4 byteclk Tlpx = Low power clock equivalence in of terms byte clock programmed in AHB reg 68h; THS_prep = programmed value of dln_cnt_hs_prep in AHB Reg 6ch bit (7:0) THS_zero = programmed value of dln_cnt_hs_zero in AHB Reg 6ch bit (15:8) Programmable count should be equal or greater than this value. Typical value x96 Number of byte clocks required to switch from low power mode to high speed mode after txrequesths is asserted."]
pub type DatalhlswcntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - High speed to low power or Low power to high speed power or Low switching time in terms byte clock (txbyteclkhs). This power to high speed switch count value is based on the byte clock (txbyteclkhs) and low power clock frequency (txclkesc); Data lane Switch count = 4 * Tlpx + programmed THS_prep + programmed THS_zero + 4 byteclk Tlpx = Low power clock equivalence in of terms byte clock programmed in AHB reg 68h; THS_prep = programmed value of dln_cnt_hs_prep in AHB Reg 6ch bit (7:0) THS_zero = programmed value of dln_cnt_hs_zero in AHB Reg 6ch bit (15:8) Programmable count should be equal or greater than this value. Typical value x96 Number of byte clocks required to switch from low power mode to high speed mode after txrequesths is asserted."]
    #[inline(always)]
    pub fn datalhlswcnt(&self) -> DatalhlswcntR {
        DatalhlswcntR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - High speed to low power or Low power to high speed power or Low switching time in terms byte clock (txbyteclkhs). This power to high speed switch count value is based on the byte clock (txbyteclkhs) and low power clock frequency (txclkesc); Data lane Switch count = 4 * Tlpx + programmed THS_prep + programmed THS_zero + 4 byteclk Tlpx = Low power clock equivalence in of terms byte clock programmed in AHB reg 68h; THS_prep = programmed value of dln_cnt_hs_prep in AHB Reg 6ch bit (7:0) THS_zero = programmed value of dln_cnt_hs_zero in AHB Reg 6ch bit (15:8) Programmable count should be equal or greater than this value. Typical value x96 Number of byte clocks required to switch from low power mode to high speed mode after txrequesths is asserted."]
    #[inline(always)]
    #[must_use]
    pub fn datalhlswcnt(&mut self) -> DatalhlswcntW<DatalanehiloswcntSpec> {
        DatalhlswcntW::new(self, 0)
    }
}
#[doc = "High speed to low power or Low power to high speed switching time\n\nYou can [`read`](crate::Reg::read) this register and get [`datalanehiloswcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`datalanehiloswcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DatalanehiloswcntSpec;
impl crate::RegisterSpec for DatalanehiloswcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`datalanehiloswcnt::R`](R) reader structure"]
impl crate::Readable for DatalanehiloswcntSpec {}
#[doc = "`write(|w| ..)` method takes [`datalanehiloswcnt::W`](W) writer structure"]
impl crate::Writable for DatalanehiloswcntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DATALANEHILOSWCNT to value 0"]
impl crate::Resettable for DatalanehiloswcntSpec {
    const RESET_VALUE: u32 = 0;
}
