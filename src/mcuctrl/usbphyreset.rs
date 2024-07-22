#[doc = "Register `USBPHYRESET` reader"]
pub type R = crate::R<UsbphyresetSpec>;
#[doc = "Register `USBPHYRESET` writer"]
pub type W = crate::W<UsbphyresetSpec>;
#[doc = "Field `USBPHYPORRSTDIS` reader - De-assert USB PHY POR reset override"]
pub type UsbphyporrstdisR = crate::BitReader;
#[doc = "Field `USBPHYPORRSTDIS` writer - De-assert USB PHY POR reset override"]
pub type UsbphyporrstdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBPHYUTMIRSTDIS` reader - De-assert USB PHY UTMI reset override"]
pub type UsbphyutmirstdisR = crate::BitReader;
#[doc = "Field `USBPHYUTMIRSTDIS` writer - De-assert USB PHY UTMI reset override"]
pub type UsbphyutmirstdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED01` reader - DSP0 ICACHE TAG EMA"]
pub type Reserved01R = crate::FieldReader;
#[doc = "Field `RESERVED01` writer - DSP0 ICACHE TAG EMA"]
pub type Reserved01W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RESERVED02` reader - DSP0 ICACHE TAG EMAS"]
pub type Reserved02R = crate::BitReader;
#[doc = "Field `RESERVED02` writer - DSP0 ICACHE TAG EMAS"]
pub type Reserved02W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED03` reader - DSP0 ICACHE TAG EMAW"]
pub type Reserved03R = crate::FieldReader;
#[doc = "Field `RESERVED03` writer - DSP0 ICACHE TAG EMAW"]
pub type Reserved03W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RESERVED04` reader - DSP0 ICACHE TAG RAWL"]
pub type Reserved04R = crate::BitReader;
#[doc = "Field `RESERVED04` writer - DSP0 ICACHE TAG RAWL"]
pub type Reserved04W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED05` reader - DSP0 ICACHE TAG RAWLM"]
pub type Reserved05R = crate::FieldReader;
#[doc = "Field `RESERVED05` writer - DSP0 ICACHE TAG RAWLM"]
pub type Reserved05W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RESERVED06` reader - DSP0 ICACHE TAG RAM WABL - Write Assist Enable (active high)"]
pub type Reserved06R = crate::BitReader;
#[doc = "Field `RESERVED06` writer - DSP0 ICACHE TAG RAM WABL - Write Assist Enable (active high)"]
pub type Reserved06W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED07` reader - DSP0 ICACHE TAG WABLM - 00=No adjust 11=increased delay, enabled by WABL"]
pub type Reserved07R = crate::FieldReader;
#[doc = "Field `RESERVED07` writer - DSP0 ICACHE TAG WABLM - 00=No adjust 11=increased delay, enabled by WABL"]
pub type Reserved07W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RESERVED09` reader - DSP0 ICACHE DATA RET1N value"]
pub type Reserved09R = crate::BitReader;
#[doc = "Field `RESERVED09` writer - DSP0 ICACHE DATA RET1N value"]
pub type Reserved09W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED10` reader - Override for DSP0 ICACHE DATA RET1N override enable"]
pub type Reserved10R = crate::BitReader;
#[doc = "Field `RESERVED10` writer - Override for DSP0 ICACHE DATA RET1N override enable"]
pub type Reserved10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED11` reader - DSP0 ICACHE DATA EMA"]
pub type Reserved11R = crate::FieldReader;
#[doc = "Field `RESERVED11` writer - DSP0 ICACHE DATA EMA"]
pub type Reserved11W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RESERVED12` reader - DSP0 ICACHE DATA EMAS"]
pub type Reserved12R = crate::BitReader;
#[doc = "Field `RESERVED12` writer - DSP0 ICACHE DATA EMAS"]
pub type Reserved12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED13` reader - DSP0 ICACHE DATA EMAW"]
pub type Reserved13R = crate::FieldReader;
#[doc = "Field `RESERVED13` writer - DSP0 ICACHE DATA EMAW"]
pub type Reserved13W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RESERVED14` reader - DSP0 ICACHE DATA RAWL"]
pub type Reserved14R = crate::BitReader;
#[doc = "Field `RESERVED14` writer - DSP0 ICACHE DATA RAWL"]
pub type Reserved14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED15` reader - DSP0 ICACHE DATA RAWLM"]
pub type Reserved15R = crate::FieldReader;
#[doc = "Field `RESERVED15` writer - DSP0 ICACHE DATA RAWLM"]
pub type Reserved15W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RESERVED16` reader - DSP0 ICACHE DATA RAM WABL - Write Assist Enable (active high)"]
pub type Reserved16R = crate::BitReader;
#[doc = "Field `RESERVED16` writer - DSP0 ICACHE DATA RAM WABL - Write Assist Enable (active high)"]
pub type Reserved16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED17` reader - DSP0 ICACHE DATA WABLM - 00=No adjust 11=increased delay, enabled by WABL"]
pub type Reserved17R = crate::FieldReader;
#[doc = "Field `RESERVED17` writer - DSP0 ICACHE DATA WABLM - 00=No adjust 11=increased delay, enabled by WABL"]
pub type Reserved17W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RESERVED18` reader - Self-timed override (test mode only)"]
pub type Reserved18R = crate::BitReader;
#[doc = "Field `RESERVED18` writer - Self-timed override (test mode only)"]
pub type Reserved18W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - De-assert USB PHY POR reset override"]
    #[inline(always)]
    pub fn usbphyporrstdis(&self) -> UsbphyporrstdisR {
        UsbphyporrstdisR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - De-assert USB PHY UTMI reset override"]
    #[inline(always)]
    pub fn usbphyutmirstdis(&self) -> UsbphyutmirstdisR {
        UsbphyutmirstdisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - DSP0 ICACHE TAG EMA"]
    #[inline(always)]
    pub fn reserved01(&self) -> Reserved01R {
        Reserved01R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 5 - DSP0 ICACHE TAG EMAS"]
    #[inline(always)]
    pub fn reserved02(&self) -> Reserved02R {
        Reserved02R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - DSP0 ICACHE TAG EMAW"]
    #[inline(always)]
    pub fn reserved03(&self) -> Reserved03R {
        Reserved03R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - DSP0 ICACHE TAG RAWL"]
    #[inline(always)]
    pub fn reserved04(&self) -> Reserved04R {
        Reserved04R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - DSP0 ICACHE TAG RAWLM"]
    #[inline(always)]
    pub fn reserved05(&self) -> Reserved05R {
        Reserved05R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - DSP0 ICACHE TAG RAM WABL - Write Assist Enable (active high)"]
    #[inline(always)]
    pub fn reserved06(&self) -> Reserved06R {
        Reserved06R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - DSP0 ICACHE TAG WABLM - 00=No adjust 11=increased delay, enabled by WABL"]
    #[inline(always)]
    pub fn reserved07(&self) -> Reserved07R {
        Reserved07R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 16 - DSP0 ICACHE DATA RET1N value"]
    #[inline(always)]
    pub fn reserved09(&self) -> Reserved09R {
        Reserved09R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Override for DSP0 ICACHE DATA RET1N override enable"]
    #[inline(always)]
    pub fn reserved10(&self) -> Reserved10R {
        Reserved10R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:20 - DSP0 ICACHE DATA EMA"]
    #[inline(always)]
    pub fn reserved11(&self) -> Reserved11R {
        Reserved11R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bit 21 - DSP0 ICACHE DATA EMAS"]
    #[inline(always)]
    pub fn reserved12(&self) -> Reserved12R {
        Reserved12R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:23 - DSP0 ICACHE DATA EMAW"]
    #[inline(always)]
    pub fn reserved13(&self) -> Reserved13R {
        Reserved13R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 24 - DSP0 ICACHE DATA RAWL"]
    #[inline(always)]
    pub fn reserved14(&self) -> Reserved14R {
        Reserved14R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:26 - DSP0 ICACHE DATA RAWLM"]
    #[inline(always)]
    pub fn reserved15(&self) -> Reserved15R {
        Reserved15R::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bit 27 - DSP0 ICACHE DATA RAM WABL - Write Assist Enable (active high)"]
    #[inline(always)]
    pub fn reserved16(&self) -> Reserved16R {
        Reserved16R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30 - DSP0 ICACHE DATA WABLM - 00=No adjust 11=increased delay, enabled by WABL"]
    #[inline(always)]
    pub fn reserved17(&self) -> Reserved17R {
        Reserved17R::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - Self-timed override (test mode only)"]
    #[inline(always)]
    pub fn reserved18(&self) -> Reserved18R {
        Reserved18R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - De-assert USB PHY POR reset override"]
    #[inline(always)]
    #[must_use]
    pub fn usbphyporrstdis(&mut self) -> UsbphyporrstdisW<UsbphyresetSpec> {
        UsbphyporrstdisW::new(self, 0)
    }
    #[doc = "Bit 1 - De-assert USB PHY UTMI reset override"]
    #[inline(always)]
    #[must_use]
    pub fn usbphyutmirstdis(&mut self) -> UsbphyutmirstdisW<UsbphyresetSpec> {
        UsbphyutmirstdisW::new(self, 1)
    }
    #[doc = "Bits 2:4 - DSP0 ICACHE TAG EMA"]
    #[inline(always)]
    #[must_use]
    pub fn reserved01(&mut self) -> Reserved01W<UsbphyresetSpec> {
        Reserved01W::new(self, 2)
    }
    #[doc = "Bit 5 - DSP0 ICACHE TAG EMAS"]
    #[inline(always)]
    #[must_use]
    pub fn reserved02(&mut self) -> Reserved02W<UsbphyresetSpec> {
        Reserved02W::new(self, 5)
    }
    #[doc = "Bits 6:7 - DSP0 ICACHE TAG EMAW"]
    #[inline(always)]
    #[must_use]
    pub fn reserved03(&mut self) -> Reserved03W<UsbphyresetSpec> {
        Reserved03W::new(self, 6)
    }
    #[doc = "Bit 8 - DSP0 ICACHE TAG RAWL"]
    #[inline(always)]
    #[must_use]
    pub fn reserved04(&mut self) -> Reserved04W<UsbphyresetSpec> {
        Reserved04W::new(self, 8)
    }
    #[doc = "Bits 9:10 - DSP0 ICACHE TAG RAWLM"]
    #[inline(always)]
    #[must_use]
    pub fn reserved05(&mut self) -> Reserved05W<UsbphyresetSpec> {
        Reserved05W::new(self, 9)
    }
    #[doc = "Bit 11 - DSP0 ICACHE TAG RAM WABL - Write Assist Enable (active high)"]
    #[inline(always)]
    #[must_use]
    pub fn reserved06(&mut self) -> Reserved06W<UsbphyresetSpec> {
        Reserved06W::new(self, 11)
    }
    #[doc = "Bits 12:14 - DSP0 ICACHE TAG WABLM - 00=No adjust 11=increased delay, enabled by WABL"]
    #[inline(always)]
    #[must_use]
    pub fn reserved07(&mut self) -> Reserved07W<UsbphyresetSpec> {
        Reserved07W::new(self, 12)
    }
    #[doc = "Bit 16 - DSP0 ICACHE DATA RET1N value"]
    #[inline(always)]
    #[must_use]
    pub fn reserved09(&mut self) -> Reserved09W<UsbphyresetSpec> {
        Reserved09W::new(self, 16)
    }
    #[doc = "Bit 17 - Override for DSP0 ICACHE DATA RET1N override enable"]
    #[inline(always)]
    #[must_use]
    pub fn reserved10(&mut self) -> Reserved10W<UsbphyresetSpec> {
        Reserved10W::new(self, 17)
    }
    #[doc = "Bits 18:20 - DSP0 ICACHE DATA EMA"]
    #[inline(always)]
    #[must_use]
    pub fn reserved11(&mut self) -> Reserved11W<UsbphyresetSpec> {
        Reserved11W::new(self, 18)
    }
    #[doc = "Bit 21 - DSP0 ICACHE DATA EMAS"]
    #[inline(always)]
    #[must_use]
    pub fn reserved12(&mut self) -> Reserved12W<UsbphyresetSpec> {
        Reserved12W::new(self, 21)
    }
    #[doc = "Bits 22:23 - DSP0 ICACHE DATA EMAW"]
    #[inline(always)]
    #[must_use]
    pub fn reserved13(&mut self) -> Reserved13W<UsbphyresetSpec> {
        Reserved13W::new(self, 22)
    }
    #[doc = "Bit 24 - DSP0 ICACHE DATA RAWL"]
    #[inline(always)]
    #[must_use]
    pub fn reserved14(&mut self) -> Reserved14W<UsbphyresetSpec> {
        Reserved14W::new(self, 24)
    }
    #[doc = "Bits 25:26 - DSP0 ICACHE DATA RAWLM"]
    #[inline(always)]
    #[must_use]
    pub fn reserved15(&mut self) -> Reserved15W<UsbphyresetSpec> {
        Reserved15W::new(self, 25)
    }
    #[doc = "Bit 27 - DSP0 ICACHE DATA RAM WABL - Write Assist Enable (active high)"]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> Reserved16W<UsbphyresetSpec> {
        Reserved16W::new(self, 27)
    }
    #[doc = "Bits 28:30 - DSP0 ICACHE DATA WABLM - 00=No adjust 11=increased delay, enabled by WABL"]
    #[inline(always)]
    #[must_use]
    pub fn reserved17(&mut self) -> Reserved17W<UsbphyresetSpec> {
        Reserved17W::new(self, 28)
    }
    #[doc = "Bit 31 - Self-timed override (test mode only)"]
    #[inline(always)]
    #[must_use]
    pub fn reserved18(&mut self) -> Reserved18W<UsbphyresetSpec> {
        Reserved18W::new(self, 31)
    }
}
#[doc = "DSP0 CACHE RAM TRIM\n\nYou can [`read`](crate::Reg::read) this register and get [`usbphyreset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbphyreset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsbphyresetSpec;
impl crate::RegisterSpec for UsbphyresetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usbphyreset::R`](R) reader structure"]
impl crate::Readable for UsbphyresetSpec {}
#[doc = "`write(|w| ..)` method takes [`usbphyreset::W`](W) writer structure"]
impl crate::Writable for UsbphyresetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USBPHYRESET to value 0x1bfc_19fc"]
impl crate::Resettable for UsbphyresetSpec {
    const RESET_VALUE: u32 = 0x1bfc_19fc;
}
