#[doc = "Register `DCXCTRL` reader"]
pub type R = crate::R<DcxctrlSpec>;
#[doc = "Register `DCXCTRL` writer"]
pub type W = crate::W<DcxctrlSpec>;
#[doc = "Field `DCXSEL` reader - Selects the CE channel used to convey the DCX function. The select is bitwise encoded, with bit 0 = 1 enabling CE0 for DCX transmission, bit 1 = 1 enableing CE1 for DCX transmission, etc. If the CE used for the SPI transaction is set, it will be ignored and used as the transaction CE instead. Multiple CE channels can be selected at once. To enable the DCX signal to be transmitted out of the chip, the corresponding pin mux function must be enabled in the GPIO logic as well."]
pub type DcxselR = crate::FieldReader;
#[doc = "Field `DCXSEL` writer - Selects the CE channel used to convey the DCX function. The select is bitwise encoded, with bit 0 = 1 enabling CE0 for DCX transmission, bit 1 = 1 enableing CE1 for DCX transmission, etc. If the CE used for the SPI transaction is set, it will be ignored and used as the transaction CE instead. Multiple CE channels can be selected at once. To enable the DCX signal to be transmitted out of the chip, the corresponding pin mux function must be enabled in the GPIO logic as well."]
pub type DcxselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DCXEN` reader - Global enable of the DCX function. Setting to 1 will enable the generation of the DCX signal, which will assert when sending the offset bytes of the SPI transaction."]
pub type DcxenR = crate::BitReader;
#[doc = "Field `DCXEN` writer - Global enable of the DCX function. Setting to 1 will enable the generation of the DCX signal, which will assert when sending the offset bytes of the SPI transaction."]
pub type DcxenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Selects the CE channel used to convey the DCX function. The select is bitwise encoded, with bit 0 = 1 enabling CE0 for DCX transmission, bit 1 = 1 enableing CE1 for DCX transmission, etc. If the CE used for the SPI transaction is set, it will be ignored and used as the transaction CE instead. Multiple CE channels can be selected at once. To enable the DCX signal to be transmitted out of the chip, the corresponding pin mux function must be enabled in the GPIO logic as well."]
    #[inline(always)]
    pub fn dcxsel(&self) -> DcxselR {
        DcxselR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Global enable of the DCX function. Setting to 1 will enable the generation of the DCX signal, which will assert when sending the offset bytes of the SPI transaction."]
    #[inline(always)]
    pub fn dcxen(&self) -> DcxenR {
        DcxenR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Selects the CE channel used to convey the DCX function. The select is bitwise encoded, with bit 0 = 1 enabling CE0 for DCX transmission, bit 1 = 1 enableing CE1 for DCX transmission, etc. If the CE used for the SPI transaction is set, it will be ignored and used as the transaction CE instead. Multiple CE channels can be selected at once. To enable the DCX signal to be transmitted out of the chip, the corresponding pin mux function must be enabled in the GPIO logic as well."]
    #[inline(always)]
    #[must_use]
    pub fn dcxsel(&mut self) -> DcxselW<DcxctrlSpec> {
        DcxselW::new(self, 0)
    }
    #[doc = "Bit 4 - Global enable of the DCX function. Setting to 1 will enable the generation of the DCX signal, which will assert when sending the offset bytes of the SPI transaction."]
    #[inline(always)]
    #[must_use]
    pub fn dcxen(&mut self) -> DcxenW<DcxctrlSpec> {
        DcxenW::new(self, 4)
    }
}
#[doc = "Enables transmission of DCX signal with SPI transactions and selects which CE signals will be used to transmit the DCX signal.\n\nYou can [`read`](crate::Reg::read) this register and get [`dcxctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcxctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcxctrlSpec;
impl crate::RegisterSpec for DcxctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcxctrl::R`](R) reader structure"]
impl crate::Readable for DcxctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`dcxctrl::W`](W) writer structure"]
impl crate::Writable for DcxctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCXCTRL to value 0"]
impl crate::Resettable for DcxctrlSpec {
    const RESET_VALUE: u32 = 0;
}
