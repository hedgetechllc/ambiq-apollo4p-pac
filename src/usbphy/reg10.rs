#[doc = "Register `REG10` reader"]
pub type R = crate::R<Reg10Spec>;
#[doc = "Register `REG10` writer"]
pub type W = crate::W<Reg10Spec>;
#[doc = "Field `BF00` reader - Bypass squelch trigger point configure in chirp modes , active high, keep the default value is strongly recommended . 1: Bypass squelch trigger point configure in chirp modes , 0: squelch trigger point set to 250mV in chirp modes."]
pub type Bf00R = crate::BitReader;
#[doc = "Field `BF00` writer - Bypass squelch trigger point configure in chirp modes , active high, keep the default value is strongly recommended . 1: Bypass squelch trigger point configure in chirp modes , 0: squelch trigger point set to 250mV in chirp modes."]
pub type Bf00W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BF11` reader - Turn off LS/FS differential receiver in suspend mode, active low 1: keep the LS/FS differential receiver , pin fss_rxrcv will toggling according to the DP/DM state 0: turn off the LS/FS differential receiver, pin fss_rxrcv will not toggling according to the DP/DM state"]
pub type Bf11R = crate::BitReader;
#[doc = "Field `BF11` writer - Turn off LS/FS differential receiver in suspend mode, active low 1: keep the LS/FS differential receiver , pin fss_rxrcv will toggling according to the DP/DM state 0: turn off the LS/FS differential receiver, pin fss_rxrcv will not toggling according to the DP/DM state"]
pub type Bf11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BF22` reader - Half bit pre-emphasis enable. Active high 1: half bit pre-emphasize mode, recommended 0: full bit pre-emphasize mode"]
pub type Bf22R = crate::BitReader;
#[doc = "Field `BF22` writer - Half bit pre-emphasis enable. Active high 1: half bit pre-emphasize mode, recommended 0: full bit pre-emphasize mode"]
pub type Bf22W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BF33` reader - Single ended disconnect detection enable, active high. 1: enable Single ended disconnect detection 0: disenable Single ended disconnect detection"]
pub type Bf33R = crate::BitReader;
#[doc = "Field `BF33` writer - Single ended disconnect detection enable, active high. 1: enable Single ended disconnect detection 0: disenable Single ended disconnect detection"]
pub type Bf33W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BF74` reader - HOST disconnect detection trigger point. Only valid in host mode. Allows compensation for package and board level parasitics which tend to drop in the input voltage. 4'b0000:625mV 4'b0001:675mV 4'b0010:612.5mV 4'b0011:575mV 4'b0100:550mV 4'b0101:600mV (default) 4'b0110:537.5mV 4'b0111:500mV 4'b1000:600mV 4'b1001:650mV 4'b1010:587.5mV 4'b1011:550mV 4'b1100:575mV 4'b1101:625mV 4'b1110:562.5mV 4'b1111:525mV"]
pub type Bf74R = crate::FieldReader;
#[doc = "Field `BF74` writer - HOST disconnect detection trigger point. Only valid in host mode. Allows compensation for package and board level parasitics which tend to drop in the input voltage. 4'b0000:625mV 4'b0001:675mV 4'b0010:612.5mV 4'b0011:575mV 4'b0100:550mV 4'b0101:600mV (default) 4'b0110:537.5mV 4'b0111:500mV 4'b1000:600mV 4'b1001:650mV 4'b1010:587.5mV 4'b1011:550mV 4'b1100:575mV 4'b1101:625mV 4'b1110:562.5mV 4'b1111:525mV"]
pub type Bf74W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Bypass squelch trigger point configure in chirp modes , active high, keep the default value is strongly recommended . 1: Bypass squelch trigger point configure in chirp modes , 0: squelch trigger point set to 250mV in chirp modes."]
    #[inline(always)]
    pub fn bf00(&self) -> Bf00R {
        Bf00R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Turn off LS/FS differential receiver in suspend mode, active low 1: keep the LS/FS differential receiver , pin fss_rxrcv will toggling according to the DP/DM state 0: turn off the LS/FS differential receiver, pin fss_rxrcv will not toggling according to the DP/DM state"]
    #[inline(always)]
    pub fn bf11(&self) -> Bf11R {
        Bf11R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Half bit pre-emphasis enable. Active high 1: half bit pre-emphasize mode, recommended 0: full bit pre-emphasize mode"]
    #[inline(always)]
    pub fn bf22(&self) -> Bf22R {
        Bf22R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Single ended disconnect detection enable, active high. 1: enable Single ended disconnect detection 0: disenable Single ended disconnect detection"]
    #[inline(always)]
    pub fn bf33(&self) -> Bf33R {
        Bf33R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - HOST disconnect detection trigger point. Only valid in host mode. Allows compensation for package and board level parasitics which tend to drop in the input voltage. 4'b0000:625mV 4'b0001:675mV 4'b0010:612.5mV 4'b0011:575mV 4'b0100:550mV 4'b0101:600mV (default) 4'b0110:537.5mV 4'b0111:500mV 4'b1000:600mV 4'b1001:650mV 4'b1010:587.5mV 4'b1011:550mV 4'b1100:575mV 4'b1101:625mV 4'b1110:562.5mV 4'b1111:525mV"]
    #[inline(always)]
    pub fn bf74(&self) -> Bf74R {
        Bf74R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Bypass squelch trigger point configure in chirp modes , active high, keep the default value is strongly recommended . 1: Bypass squelch trigger point configure in chirp modes , 0: squelch trigger point set to 250mV in chirp modes."]
    #[inline(always)]
    #[must_use]
    pub fn bf00(&mut self) -> Bf00W<Reg10Spec> {
        Bf00W::new(self, 0)
    }
    #[doc = "Bit 1 - Turn off LS/FS differential receiver in suspend mode, active low 1: keep the LS/FS differential receiver , pin fss_rxrcv will toggling according to the DP/DM state 0: turn off the LS/FS differential receiver, pin fss_rxrcv will not toggling according to the DP/DM state"]
    #[inline(always)]
    #[must_use]
    pub fn bf11(&mut self) -> Bf11W<Reg10Spec> {
        Bf11W::new(self, 1)
    }
    #[doc = "Bit 2 - Half bit pre-emphasis enable. Active high 1: half bit pre-emphasize mode, recommended 0: full bit pre-emphasize mode"]
    #[inline(always)]
    #[must_use]
    pub fn bf22(&mut self) -> Bf22W<Reg10Spec> {
        Bf22W::new(self, 2)
    }
    #[doc = "Bit 3 - Single ended disconnect detection enable, active high. 1: enable Single ended disconnect detection 0: disenable Single ended disconnect detection"]
    #[inline(always)]
    #[must_use]
    pub fn bf33(&mut self) -> Bf33W<Reg10Spec> {
        Bf33W::new(self, 3)
    }
    #[doc = "Bits 4:7 - HOST disconnect detection trigger point. Only valid in host mode. Allows compensation for package and board level parasitics which tend to drop in the input voltage. 4'b0000:625mV 4'b0001:675mV 4'b0010:612.5mV 4'b0011:575mV 4'b0100:550mV 4'b0101:600mV (default) 4'b0110:537.5mV 4'b0111:500mV 4'b1000:600mV 4'b1001:650mV 4'b1010:587.5mV 4'b1011:550mV 4'b1100:575mV 4'b1101:625mV 4'b1110:562.5mV 4'b1111:525mV"]
    #[inline(always)]
    #[must_use]
    pub fn bf74(&mut self) -> Bf74W<Reg10Spec> {
        Bf74W::new(self, 4)
    }
}
#[doc = "Register description here\n\nYou can [`read`](crate::Reg::read) this register and get [`reg10::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg10::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reg10Spec;
impl crate::RegisterSpec for Reg10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg10::R`](R) reader structure"]
impl crate::Readable for Reg10Spec {}
#[doc = "`write(|w| ..)` method takes [`reg10::W`](W) writer structure"]
impl crate::Writable for Reg10Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REG10 to value 0"]
impl crate::Resettable for Reg10Spec {
    const RESET_VALUE: u32 = 0;
}
