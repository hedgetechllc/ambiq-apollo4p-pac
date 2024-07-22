#[doc = "Register `DMATRIGSTAT` reader"]
pub type R = crate::R<DmatrigstatSpec>;
#[doc = "Register `DMATRIGSTAT` writer"]
pub type W = crate::W<DmatrigstatSpec>;
#[doc = "Field `DCMDCMP` reader - Triggered DMA from Command complete event. Bit is read only and can be cleared by disabling the DCMDCMP trigger enable or by disabling DMA."]
pub type DcmdcmpR = crate::BitReader;
#[doc = "Field `DCMDCMP` writer - Triggered DMA from Command complete event. Bit is read only and can be cleared by disabling the DCMDCMP trigger enable or by disabling DMA."]
pub type DcmdcmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTHR` reader - Triggered DMA from THR event. Bit is read only and can be cleared by disabling the DTHR trigger enable or by disabling DMA."]
pub type DthrR = crate::BitReader;
#[doc = "Field `DTHR` writer - Triggered DMA from THR event. Bit is read only and can be cleared by disabling the DTHR trigger enable or by disabling DMA."]
pub type DthrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTOTCMP` reader - DMA triggered when DCMDCMP = 0, and the amount of data in the FIFO was enough to complete the DMA operation (greater than or equal to current TOTCOUNT) when the command completed. This trigger is default active when the DCMDCMP trigger is disabled and there is enough data in the FIFO to complete the DMA operation."]
pub type DtotcmpR = crate::BitReader;
#[doc = "Field `DTOTCMP` writer - DMA triggered when DCMDCMP = 0, and the amount of data in the FIFO was enough to complete the DMA operation (greater than or equal to current TOTCOUNT) when the command completed. This trigger is default active when the DCMDCMP trigger is disabled and there is enough data in the FIFO to complete the DMA operation."]
pub type DtotcmpW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Triggered DMA from Command complete event. Bit is read only and can be cleared by disabling the DCMDCMP trigger enable or by disabling DMA."]
    #[inline(always)]
    pub fn dcmdcmp(&self) -> DcmdcmpR {
        DcmdcmpR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Triggered DMA from THR event. Bit is read only and can be cleared by disabling the DTHR trigger enable or by disabling DMA."]
    #[inline(always)]
    pub fn dthr(&self) -> DthrR {
        DthrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMA triggered when DCMDCMP = 0, and the amount of data in the FIFO was enough to complete the DMA operation (greater than or equal to current TOTCOUNT) when the command completed. This trigger is default active when the DCMDCMP trigger is disabled and there is enough data in the FIFO to complete the DMA operation."]
    #[inline(always)]
    pub fn dtotcmp(&self) -> DtotcmpR {
        DtotcmpR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Triggered DMA from Command complete event. Bit is read only and can be cleared by disabling the DCMDCMP trigger enable or by disabling DMA."]
    #[inline(always)]
    #[must_use]
    pub fn dcmdcmp(&mut self) -> DcmdcmpW<DmatrigstatSpec> {
        DcmdcmpW::new(self, 0)
    }
    #[doc = "Bit 1 - Triggered DMA from THR event. Bit is read only and can be cleared by disabling the DTHR trigger enable or by disabling DMA."]
    #[inline(always)]
    #[must_use]
    pub fn dthr(&mut self) -> DthrW<DmatrigstatSpec> {
        DthrW::new(self, 1)
    }
    #[doc = "Bit 2 - DMA triggered when DCMDCMP = 0, and the amount of data in the FIFO was enough to complete the DMA operation (greater than or equal to current TOTCOUNT) when the command completed. This trigger is default active when the DCMDCMP trigger is disabled and there is enough data in the FIFO to complete the DMA operation."]
    #[inline(always)]
    #[must_use]
    pub fn dtotcmp(&mut self) -> DtotcmpW<DmatrigstatSpec> {
        DtotcmpW::new(self, 2)
    }
}
#[doc = "Provides the status of trigger events that have occurred for the transaction. Some of the bits are read only and some can be reset via a write of 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`dmatrigstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmatrigstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmatrigstatSpec;
impl crate::RegisterSpec for DmatrigstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmatrigstat::R`](R) reader structure"]
impl crate::Readable for DmatrigstatSpec {}
#[doc = "`write(|w| ..)` method takes [`dmatrigstat::W`](W) writer structure"]
impl crate::Writable for DmatrigstatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMATRIGSTAT to value 0"]
impl crate::Resettable for DmatrigstatSpec {
    const RESET_VALUE: u32 = 0;
}
