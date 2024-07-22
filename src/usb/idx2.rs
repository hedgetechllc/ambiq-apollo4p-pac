#[doc = "Register `IDX2` reader"]
pub type R = crate::R<Idx2Spec>;
#[doc = "Register `IDX2` writer"]
pub type W = crate::W<Idx2Spec>;
#[doc = "Field `ENDPTOUTCOUNT` reader - Endpoint OUT Count. When CFG3_ENDPOINT = 1 to 5, this read-only field holds the number of received data bytes in the packet in the Endpoint's OUT FIFO. When CFG3_ENDPOINT = 0, this read-only field holds 7-bit data for number of received data bytes in Endpoint 0 FIFO (OUT count). In either case, the value returned changes as the contents of the FIFO change and is only valid while OutPktRdy is set. (IMPORTANT: The address for the OUTCOUNT register is actually the same as COUNT0. However to avoid CMSIS conflicts, the address here includes an additional offset of 0x1000. Access to this register must take this into account.)If CFG3_ENDPOINT = 0x1-0x5, this read-only field holds the number of received data bytes in the packet in the OUT FIFO. If the packet was transmitted as multiple bulk packets, the number given will be for the combined packet.If CFG3_ENDPOINT = 0x0, this read-only field holds 7-bit data for number of received data bytes in Endpoint 0 FIFO (OUT count)."]
pub type EndptoutcountR = crate::FieldReader<u16>;
#[doc = "Field `ENDPTOUTCOUNT` writer - Endpoint OUT Count. When CFG3_ENDPOINT = 1 to 5, this read-only field holds the number of received data bytes in the packet in the Endpoint's OUT FIFO. When CFG3_ENDPOINT = 0, this read-only field holds 7-bit data for number of received data bytes in Endpoint 0 FIFO (OUT count). In either case, the value returned changes as the contents of the FIFO change and is only valid while OutPktRdy is set. (IMPORTANT: The address for the OUTCOUNT register is actually the same as COUNT0. However to avoid CMSIS conflicts, the address here includes an additional offset of 0x1000. Access to this register must take this into account.)If CFG3_ENDPOINT = 0x1-0x5, this read-only field holds the number of received data bytes in the packet in the OUT FIFO. If the packet was transmitted as multiple bulk packets, the number given will be for the combined packet.If CFG3_ENDPOINT = 0x0, this read-only field holds 7-bit data for number of received data bytes in Endpoint 0 FIFO (OUT count)."]
pub type EndptoutcountW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `INFIFOSZ` reader - IN FIFO Size. Sets the size of the selected IN endpoint FIFO. Bit 4 of this field defines whether double-packet buffering supported. When set, double-packet buffering is supported. When cleared, only single-packet buffering is supported. Bits \\[3:0\\]
of this field determine maximum packet size, where 2^^(b3:b0 + 3) is the maximum packet size to be allowed (before any splitting within the FIFO of Bulk/High-Bandwidth packets prior to transmission)."]
pub type InfifoszR = crate::FieldReader;
#[doc = "Field `INFIFOSZ` writer - IN FIFO Size. Sets the size of the selected IN endpoint FIFO. Bit 4 of this field defines whether double-packet buffering supported. When set, double-packet buffering is supported. When cleared, only single-packet buffering is supported. Bits \\[3:0\\]
of this field determine maximum packet size, where 2^^(b3:b0 + 3) is the maximum packet size to be allowed (before any splitting within the FIFO of Bulk/High-Bandwidth packets prior to transmission)."]
pub type InfifoszW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `OUTFIFOSZ` reader - OUT FIFO Size. Sets the size of the selected OUT endpoint FIFO. Bit 4 of this field defines whether double-packet buffering is supported. When set, double-packet buffering is supported. When cleared, only single-packet buffering is supported. Bits \\[3:0\\]
of this field determine maximum packet size, where 2^^(b3:b0 + 3) is the maximum packet size to be allowed (before any splitting within the FIFO of Bulk/High-Bandwidth packets prior to transmission)."]
pub type OutfifoszR = crate::FieldReader;
#[doc = "Field `OUTFIFOSZ` writer - OUT FIFO Size. Sets the size of the selected OUT endpoint FIFO. Bit 4 of this field defines whether double-packet buffering is supported. When set, double-packet buffering is supported. When cleared, only single-packet buffering is supported. Bits \\[3:0\\]
of this field determine maximum packet size, where 2^^(b3:b0 + 3) is the maximum packet size to be allowed (before any splitting within the FIFO of Bulk/High-Bandwidth packets prior to transmission)."]
pub type OutfifoszW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:12 - Endpoint OUT Count. When CFG3_ENDPOINT = 1 to 5, this read-only field holds the number of received data bytes in the packet in the Endpoint's OUT FIFO. When CFG3_ENDPOINT = 0, this read-only field holds 7-bit data for number of received data bytes in Endpoint 0 FIFO (OUT count). In either case, the value returned changes as the contents of the FIFO change and is only valid while OutPktRdy is set. (IMPORTANT: The address for the OUTCOUNT register is actually the same as COUNT0. However to avoid CMSIS conflicts, the address here includes an additional offset of 0x1000. Access to this register must take this into account.)If CFG3_ENDPOINT = 0x1-0x5, this read-only field holds the number of received data bytes in the packet in the OUT FIFO. If the packet was transmitted as multiple bulk packets, the number given will be for the combined packet.If CFG3_ENDPOINT = 0x0, this read-only field holds 7-bit data for number of received data bytes in Endpoint 0 FIFO (OUT count)."]
    #[inline(always)]
    pub fn endptoutcount(&self) -> EndptoutcountR {
        EndptoutcountR::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:20 - IN FIFO Size. Sets the size of the selected IN endpoint FIFO. Bit 4 of this field defines whether double-packet buffering supported. When set, double-packet buffering is supported. When cleared, only single-packet buffering is supported. Bits \\[3:0\\]
of this field determine maximum packet size, where 2^^(b3:b0 + 3) is the maximum packet size to be allowed (before any splitting within the FIFO of Bulk/High-Bandwidth packets prior to transmission)."]
    #[inline(always)]
    pub fn infifosz(&self) -> InfifoszR {
        InfifoszR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - OUT FIFO Size. Sets the size of the selected OUT endpoint FIFO. Bit 4 of this field defines whether double-packet buffering is supported. When set, double-packet buffering is supported. When cleared, only single-packet buffering is supported. Bits \\[3:0\\]
of this field determine maximum packet size, where 2^^(b3:b0 + 3) is the maximum packet size to be allowed (before any splitting within the FIFO of Bulk/High-Bandwidth packets prior to transmission)."]
    #[inline(always)]
    pub fn outfifosz(&self) -> OutfifoszR {
        OutfifoszR::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:12 - Endpoint OUT Count. When CFG3_ENDPOINT = 1 to 5, this read-only field holds the number of received data bytes in the packet in the Endpoint's OUT FIFO. When CFG3_ENDPOINT = 0, this read-only field holds 7-bit data for number of received data bytes in Endpoint 0 FIFO (OUT count). In either case, the value returned changes as the contents of the FIFO change and is only valid while OutPktRdy is set. (IMPORTANT: The address for the OUTCOUNT register is actually the same as COUNT0. However to avoid CMSIS conflicts, the address here includes an additional offset of 0x1000. Access to this register must take this into account.)If CFG3_ENDPOINT = 0x1-0x5, this read-only field holds the number of received data bytes in the packet in the OUT FIFO. If the packet was transmitted as multiple bulk packets, the number given will be for the combined packet.If CFG3_ENDPOINT = 0x0, this read-only field holds 7-bit data for number of received data bytes in Endpoint 0 FIFO (OUT count)."]
    #[inline(always)]
    #[must_use]
    pub fn endptoutcount(&mut self) -> EndptoutcountW<Idx2Spec> {
        EndptoutcountW::new(self, 0)
    }
    #[doc = "Bits 16:20 - IN FIFO Size. Sets the size of the selected IN endpoint FIFO. Bit 4 of this field defines whether double-packet buffering supported. When set, double-packet buffering is supported. When cleared, only single-packet buffering is supported. Bits \\[3:0\\]
of this field determine maximum packet size, where 2^^(b3:b0 + 3) is the maximum packet size to be allowed (before any splitting within the FIFO of Bulk/High-Bandwidth packets prior to transmission)."]
    #[inline(always)]
    #[must_use]
    pub fn infifosz(&mut self) -> InfifoszW<Idx2Spec> {
        InfifoszW::new(self, 16)
    }
    #[doc = "Bits 24:28 - OUT FIFO Size. Sets the size of the selected OUT endpoint FIFO. Bit 4 of this field defines whether double-packet buffering is supported. When set, double-packet buffering is supported. When cleared, only single-packet buffering is supported. Bits \\[3:0\\]
of this field determine maximum packet size, where 2^^(b3:b0 + 3) is the maximum packet size to be allowed (before any splitting within the FIFO of Bulk/High-Bandwidth packets prior to transmission)."]
    #[inline(always)]
    #[must_use]
    pub fn outfifosz(&mut self) -> OutfifoszW<Idx2Spec> {
        OutfifoszW::new(self, 24)
    }
}
#[doc = "Contains the outcount value for number of received bytes in the packet in the OUT FIFO, and the configurable IN and OUT Endpoint FIFO size.\n\nYou can [`read`](crate::Reg::read) this register and get [`idx2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idx2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Idx2Spec;
impl crate::RegisterSpec for Idx2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idx2::R`](R) reader structure"]
impl crate::Readable for Idx2Spec {}
#[doc = "`write(|w| ..)` method takes [`idx2::W`](W) writer structure"]
impl crate::Writable for Idx2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IDX2 to value 0"]
impl crate::Resettable for Idx2Spec {
    const RESET_VALUE: u32 = 0;
}
