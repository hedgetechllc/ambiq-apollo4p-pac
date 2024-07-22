#[doc = "Register `CQFLAGS` reader"]
pub type R = crate::R<CqflagsSpec>;
#[doc = "Register `CQFLAGS` writer"]
pub type W = crate::W<CqflagsSpec>;
#[doc = "Current flag status (read-only). Bits \\[7:0\\]
are software controllable and bits \\[15:8\\]
are hardware status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Cqflags {
    #[doc = "32768: CQ Stop Flag. When set, CQ processing will complete."]
    Stop = 32768,
    #[doc = "16384: CQ Index Pointers (CURIDX/ENDIDX) match."]
    Cqidx = 16384,
    #[doc = "8192: Buffer 1 Ready Status (from selected IOM/MSPI). This status is the result of XOR'ing the IOM1START with the incoming status from the IOM. When high, MSPI can transfer the buffer."]
    Buf1xoren = 8192,
    #[doc = "4096: Buffer 0 Ready Status (from selected IOM/MSPI). This status is the result of XOR'ing the IOM0START with the incoming status from the IOM. When high, MSPI can transfer the buffer."]
    Buf0xoren = 4096,
    #[doc = "2048: DMA Complete Status (hardwired DMACPL bit in DMASTAT)"]
    Dmacpl = 2048,
    #[doc = "1024: PIO Operation completed (STATUS bit in CTRL register)"]
    Cmdcpl = 1024,
    #[doc = "512: (BUF1XNOREN) IOM Buffer 1 Ready Status (from selected IOM). This status is the result of XNOR'ing the IOM0START with the incoming status from the IOM. When high, MSPI can send to the buffer."]
    Iom1ready = 512,
    #[doc = "256: (BUF0XNOREN) IOM Buffer 0 Ready Status (from selected IOM). This status is the result of XNOR'ing the IOM0START with the incoming status from the IOM. When high, MSPI can send to the buffer."]
    Iom0ready = 256,
    #[doc = "128: Software flag 7. Can be used by software to start/pause operations."]
    Swflag7 = 128,
    #[doc = "64: Software flag 6. Can be used by software to start/pause operations."]
    Swflag6 = 64,
    #[doc = "32: Software flag 5. Can be used by software to start/pause operations."]
    Swflag5 = 32,
    #[doc = "16: Software flag 4. Can be used by software to start/pause operations."]
    Swflag4 = 16,
    #[doc = "8: Software flag 3. Can be used by software to start/pause operations."]
    Swflag3 = 8,
    #[doc = "4: Software flag 2. Can be used by software to start/pause operations."]
    Swflag2 = 4,
    #[doc = "2: Software flag 1. Can be used by software to start/pause operations. Also, IOM Buffer 1 status. When linked to IOM, indicates to IOM that buffer 1 is ready."]
    Swflag1 = 2,
    #[doc = "1: Software flag 0. Can be used by software to start/pause operations. Also, IOM Buffer 0 status. When linked to IOM, indicates to IOM that buffer 0 is ready."]
    Swflag0 = 1,
}
impl From<Cqflags> for u16 {
    #[inline(always)]
    fn from(variant: Cqflags) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cqflags {
    type Ux = u16;
}
impl crate::IsEnum for Cqflags {}
#[doc = "Field `CQFLAGS` reader - Current flag status (read-only). Bits \\[7:0\\]
are software controllable and bits \\[15:8\\]
are hardware status."]
pub type CqflagsR = crate::FieldReader<Cqflags>;
impl CqflagsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cqflags> {
        match self.bits {
            32768 => Some(Cqflags::Stop),
            16384 => Some(Cqflags::Cqidx),
            8192 => Some(Cqflags::Buf1xoren),
            4096 => Some(Cqflags::Buf0xoren),
            2048 => Some(Cqflags::Dmacpl),
            1024 => Some(Cqflags::Cmdcpl),
            512 => Some(Cqflags::Iom1ready),
            256 => Some(Cqflags::Iom0ready),
            128 => Some(Cqflags::Swflag7),
            64 => Some(Cqflags::Swflag6),
            32 => Some(Cqflags::Swflag5),
            16 => Some(Cqflags::Swflag4),
            8 => Some(Cqflags::Swflag3),
            4 => Some(Cqflags::Swflag2),
            2 => Some(Cqflags::Swflag1),
            1 => Some(Cqflags::Swflag0),
            _ => None,
        }
    }
    #[doc = "CQ Stop Flag. When set, CQ processing will complete."]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == Cqflags::Stop
    }
    #[doc = "CQ Index Pointers (CURIDX/ENDIDX) match."]
    #[inline(always)]
    pub fn is_cqidx(&self) -> bool {
        *self == Cqflags::Cqidx
    }
    #[doc = "Buffer 1 Ready Status (from selected IOM/MSPI). This status is the result of XOR'ing the IOM1START with the incoming status from the IOM. When high, MSPI can transfer the buffer."]
    #[inline(always)]
    pub fn is_buf1xoren(&self) -> bool {
        *self == Cqflags::Buf1xoren
    }
    #[doc = "Buffer 0 Ready Status (from selected IOM/MSPI). This status is the result of XOR'ing the IOM0START with the incoming status from the IOM. When high, MSPI can transfer the buffer."]
    #[inline(always)]
    pub fn is_buf0xoren(&self) -> bool {
        *self == Cqflags::Buf0xoren
    }
    #[doc = "DMA Complete Status (hardwired DMACPL bit in DMASTAT)"]
    #[inline(always)]
    pub fn is_dmacpl(&self) -> bool {
        *self == Cqflags::Dmacpl
    }
    #[doc = "PIO Operation completed (STATUS bit in CTRL register)"]
    #[inline(always)]
    pub fn is_cmdcpl(&self) -> bool {
        *self == Cqflags::Cmdcpl
    }
    #[doc = "(BUF1XNOREN) IOM Buffer 1 Ready Status (from selected IOM). This status is the result of XNOR'ing the IOM0START with the incoming status from the IOM. When high, MSPI can send to the buffer."]
    #[inline(always)]
    pub fn is_iom1ready(&self) -> bool {
        *self == Cqflags::Iom1ready
    }
    #[doc = "(BUF0XNOREN) IOM Buffer 0 Ready Status (from selected IOM). This status is the result of XNOR'ing the IOM0START with the incoming status from the IOM. When high, MSPI can send to the buffer."]
    #[inline(always)]
    pub fn is_iom0ready(&self) -> bool {
        *self == Cqflags::Iom0ready
    }
    #[doc = "Software flag 7. Can be used by software to start/pause operations."]
    #[inline(always)]
    pub fn is_swflag7(&self) -> bool {
        *self == Cqflags::Swflag7
    }
    #[doc = "Software flag 6. Can be used by software to start/pause operations."]
    #[inline(always)]
    pub fn is_swflag6(&self) -> bool {
        *self == Cqflags::Swflag6
    }
    #[doc = "Software flag 5. Can be used by software to start/pause operations."]
    #[inline(always)]
    pub fn is_swflag5(&self) -> bool {
        *self == Cqflags::Swflag5
    }
    #[doc = "Software flag 4. Can be used by software to start/pause operations."]
    #[inline(always)]
    pub fn is_swflag4(&self) -> bool {
        *self == Cqflags::Swflag4
    }
    #[doc = "Software flag 3. Can be used by software to start/pause operations."]
    #[inline(always)]
    pub fn is_swflag3(&self) -> bool {
        *self == Cqflags::Swflag3
    }
    #[doc = "Software flag 2. Can be used by software to start/pause operations."]
    #[inline(always)]
    pub fn is_swflag2(&self) -> bool {
        *self == Cqflags::Swflag2
    }
    #[doc = "Software flag 1. Can be used by software to start/pause operations. Also, IOM Buffer 1 status. When linked to IOM, indicates to IOM that buffer 1 is ready."]
    #[inline(always)]
    pub fn is_swflag1(&self) -> bool {
        *self == Cqflags::Swflag1
    }
    #[doc = "Software flag 0. Can be used by software to start/pause operations. Also, IOM Buffer 0 status. When linked to IOM, indicates to IOM that buffer 0 is ready."]
    #[inline(always)]
    pub fn is_swflag0(&self) -> bool {
        *self == Cqflags::Swflag0
    }
}
#[doc = "Field `CQFLAGS` writer - Current flag status (read-only). Bits \\[7:0\\]
are software controllable and bits \\[15:8\\]
are hardware status."]
pub type CqflagsW<'a, REG> = crate::FieldWriter<'a, REG, 16, Cqflags>;
impl<'a, REG> CqflagsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "CQ Stop Flag. When set, CQ processing will complete."]
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(Cqflags::Stop)
    }
    #[doc = "CQ Index Pointers (CURIDX/ENDIDX) match."]
    #[inline(always)]
    pub fn cqidx(self) -> &'a mut crate::W<REG> {
        self.variant(Cqflags::Cqidx)
    }
    #[doc = "Buffer 1 Ready Status (from selected IOM/MSPI). This status is the result of XOR'ing the IOM1START with the incoming status from the IOM. When high, MSPI can transfer the buffer."]
    #[inline(always)]
    pub fn buf1xoren(self) -> &'a mut crate::W<REG> {
        self.variant(Cqflags::Buf1xoren)
    }
    #[doc = "Buffer 0 Ready Status (from selected IOM/MSPI). This status is the result of XOR'ing the IOM0START with the incoming status from the IOM. When high, MSPI can transfer the buffer."]
    #[inline(always)]
    pub fn buf0xoren(self) -> &'a mut crate::W<REG> {
        self.variant(Cqflags::Buf0xoren)
    }
    #[doc = "DMA Complete Status (hardwired DMACPL bit in DMASTAT)"]
    #[inline(always)]
    pub fn dmacpl(self) -> &'a mut crate::W<REG> {
        self.variant(Cqflags::Dmacpl)
    }
    #[doc = "PIO Operation completed (STATUS bit in CTRL register)"]
    #[inline(always)]
    pub fn cmdcpl(self) -> &'a mut crate::W<REG> {
        self.variant(Cqflags::Cmdcpl)
    }
    #[doc = "(BUF1XNOREN) IOM Buffer 1 Ready Status (from selected IOM). This status is the result of XNOR'ing the IOM0START with the incoming status from the IOM. When high, MSPI can send to the buffer."]
    #[inline(always)]
    pub fn iom1ready(self) -> &'a mut crate::W<REG> {
        self.variant(Cqflags::Iom1ready)
    }
    #[doc = "(BUF0XNOREN) IOM Buffer 0 Ready Status (from selected IOM). This status is the result of XNOR'ing the IOM0START with the incoming status from the IOM. When high, MSPI can send to the buffer."]
    #[inline(always)]
    pub fn iom0ready(self) -> &'a mut crate::W<REG> {
        self.variant(Cqflags::Iom0ready)
    }
    #[doc = "Software flag 7. Can be used by software to start/pause operations."]
    #[inline(always)]
    pub fn swflag7(self) -> &'a mut crate::W<REG> {
        self.variant(Cqflags::Swflag7)
    }
    #[doc = "Software flag 6. Can be used by software to start/pause operations."]
    #[inline(always)]
    pub fn swflag6(self) -> &'a mut crate::W<REG> {
        self.variant(Cqflags::Swflag6)
    }
    #[doc = "Software flag 5. Can be used by software to start/pause operations."]
    #[inline(always)]
    pub fn swflag5(self) -> &'a mut crate::W<REG> {
        self.variant(Cqflags::Swflag5)
    }
    #[doc = "Software flag 4. Can be used by software to start/pause operations."]
    #[inline(always)]
    pub fn swflag4(self) -> &'a mut crate::W<REG> {
        self.variant(Cqflags::Swflag4)
    }
    #[doc = "Software flag 3. Can be used by software to start/pause operations."]
    #[inline(always)]
    pub fn swflag3(self) -> &'a mut crate::W<REG> {
        self.variant(Cqflags::Swflag3)
    }
    #[doc = "Software flag 2. Can be used by software to start/pause operations."]
    #[inline(always)]
    pub fn swflag2(self) -> &'a mut crate::W<REG> {
        self.variant(Cqflags::Swflag2)
    }
    #[doc = "Software flag 1. Can be used by software to start/pause operations. Also, IOM Buffer 1 status. When linked to IOM, indicates to IOM that buffer 1 is ready."]
    #[inline(always)]
    pub fn swflag1(self) -> &'a mut crate::W<REG> {
        self.variant(Cqflags::Swflag1)
    }
    #[doc = "Software flag 0. Can be used by software to start/pause operations. Also, IOM Buffer 0 status. When linked to IOM, indicates to IOM that buffer 0 is ready."]
    #[inline(always)]
    pub fn swflag0(self) -> &'a mut crate::W<REG> {
        self.variant(Cqflags::Swflag0)
    }
}
impl R {
    #[doc = "Bits 0:15 - Current flag status (read-only). Bits \\[7:0\\]
are software controllable and bits \\[15:8\\]
are hardware status."]
    #[inline(always)]
    pub fn cqflags(&self) -> CqflagsR {
        CqflagsR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Current flag status (read-only). Bits \\[7:0\\]
are software controllable and bits \\[15:8\\]
are hardware status."]
    #[inline(always)]
    #[must_use]
    pub fn cqflags(&mut self) -> CqflagsW<CqflagsSpec> {
        CqflagsW::new(self, 0)
    }
}
#[doc = "Command Queue Flags\n\nYou can [`read`](crate::Reg::read) this register and get [`cqflags::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cqflags::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CqflagsSpec;
impl crate::RegisterSpec for CqflagsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cqflags::R`](R) reader structure"]
impl crate::Readable for CqflagsSpec {}
#[doc = "`write(|w| ..)` method takes [`cqflags::W`](W) writer structure"]
impl crate::Writable for CqflagsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CQFLAGS to value 0"]
impl crate::Resettable for CqflagsSpec {
    const RESET_VALUE: u32 = 0;
}
