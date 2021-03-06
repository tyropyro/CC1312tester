#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PKA Vector A Address uring execution of basic PKCP operations, this register is double buffered and can be written with a new value for the next operation; when not written, the value remains intact. During the execution of sequencer-controlled complex operations, this register may not be written and its value is undefined at the conclusion of the operation. The driver software cannot rely on the written value to remain intact."]
    pub aptr: APTR,
    #[doc = "0x04 - PKA Vector B Address uring execution of basic PKCP operations, this register is double buffered and can be written with a new value for the next operation; when not written, the value remains intact. During the execution of sequencer-controlled complex operations, this register may not be written and its value is undefined at the conclusion of the operation. The driver software cannot rely on the written value to remain intact."]
    pub bptr: BPTR,
    #[doc = "0x08 - PKA Vector C Address uring execution of basic PKCP operations, this register is double buffered and can be written with a new value for the next operation; when not written, the value remains intact. During the execution of sequencer-controlled complex operations, this register may not be written and its value is undefined at the conclusion of the operation. The driver software cannot rely on the written value to remain intact."]
    pub cptr: CPTR,
    #[doc = "0x0c - PKA Vector D Address uring execution of basic PKCP operations, this register is double buffered and can be written with a new value for the next operation; when not written, the value remains intact. During the execution of sequencer-controlled complex operations, this register may not be written and its value is undefined at the conclusion of the operation. The driver software cannot rely on the written value to remain intact."]
    pub dptr: DPTR,
    #[doc = "0x10 - PKA Vector A Length uring execution of basic PKCP operations, this register is double buffered and can be written with a new value for the next operation; when not written, the value remains intact. During the execution of sequencer-controlled complex operations, this register may not be written and its value is undefined at the conclusion of the operation. The driver software cannot rely on the written value to remain intact."]
    pub alength: ALENGTH,
    #[doc = "0x14 - PKA Vector B Length uring execution of basic PKCP operations, this register is double buffered and can be written with a new value for the next operation; when not written, the value remains intact. During the execution of sequencer-controlled complex operations, this register may not be written and its value is undefined at the conclusion of the operation. The driver software cannot rely on the written value to remain intact."]
    pub blength: BLENGTH,
    #[doc = "0x18 - PKA Bit Shift Value or basic PKCP operations, modifying the contents of this register is made impossible while the operation is being performed. For the ExpMod-variable and ExpMod-CRT operations, this register is used to indicate the number of odd powers to use (directly as a value in the range 1-16). For the ModInv and ECC operations, this register is used to hold a completion code."]
    pub shift: SHIFT,
    #[doc = "0x1c - PKA Function his register contains the control bits to start basic PKCP as well as complex sequencer operations. The run bit can be used to poll for the completion of the operation. Modifying bits \\[11:0\\]
is made impossible during the execution of a basic PKCP operation. uring the execution of sequencer-controlled complex operations, this register is modified, the run and stall result bits are set to zero at the conclusion, but other bits are undefined. OTE: Continuously reading this register to poll the run bit is not allowed when executing complex sequencer operations (the sequencer cannot access the PKCP when this is done). Leave at least one sysclk cycle between poll operations."]
    pub function: FUNCTION,
    #[doc = "0x20 - PKA compare result his register provides the result of a basic PKCP compare operation. It is updated when the FUNCTION.RUN bit is reset at the end of that operation. tatus after a complex sequencer operation is unknown"]
    pub compare: COMPARE,
    #[doc = "0x24 - PKA most-significant-word of result vector his register indicates the (word) address in the PKA RAM where the most significant nonzero 32-bit word of the result is stored. Should be ignored for modulo operations. For basic PKCP operations, this register is updated FUNCTION.RUN bit is reset at the end of the operation. For the complex-sequencer controlled operations, updating of the final value matching the actual result is done near the end of the operation; note that the result is only meaningful if no errors were detected and that for ECC operations, this register will provide information for the x-coordinate of the result point only."]
    pub msw: MSW,
    #[doc = "0x28 - PKA most-significant-word of divide remainder his register indicates the (32-bit word) address in the PKA RAM where the most significant nonzero 32-bit word of the remainder result for the basic divide and modulo operations is stored. Bits \\[4:0\\]
are loaded with the bit number of the most-significant nonzero bit in the most-significant nonzero word when MS one control bit is set. For divide, modulo, and MS one reporting, this register is updated when FUNCTION.RUN bit is reset at the end of the operation. For the complex sequencer controlled operations, updating of bits \\[4:0\\]
of this register with the most-significant bit location of the actual result is done near the end of the operation. The result is meaningful only if no errors were detected and that for ECC operations; this register provides information for the x-coordinate of the result point only."]
    pub divmsw: DIVMSW,
    _reserved11: [u8; 156usize],
    #[doc = "0xc8 - PKA sequencer control and status register he sequencer is interfaced with the outside world through a single control and status register. With the exception of bit \\[31\\], the actual use of bits in the separate sub-fields of this register is determined by the sequencer firmware. This register need only be accessed when the sequencer program is stored in RAM. The reset value of the RESET bit depends upon the option chosen for sequencer program storage. NOTE: For Agama the sequencer firmware is executed from ROM."]
    pub seqctrl: SEQCTRL,
    _reserved12: [u8; 40usize],
    #[doc = "0xf4 - PKA hardware options register his register provides the host with a means to determine the hardware configuration implemented in this PKA engine, focused on options that have an effect on software interacting with the module."]
    pub options: OPTIONS,
    #[doc = "0xf8 - PKA firmware revision and capabilities register his register allows the host access to the internal firmware revision number of the PKA Engine for software driver matching and diagnostic purposes. This register also contains a field that encodes the capabilities of the embedded firmware. his register is written by the firmware within a few clock cycles after starting up that firmware. The hardware reset value is zero, indicating that the information has not been written yet."]
    pub fwrev: FWREV,
    #[doc = "0xfc - PKA hardware revision register his register allows the host access to the hardware revision number of the PKA engine for software driver matching and diagnostic purposes. It is always located at the highest address in the access space of the module and contains an encoding of the EIP number (with its complement as signature) for recognition of the hardware module."]
    pub hwrev: HWREV,
}
#[doc = "PKA Vector A Address uring execution of basic PKCP operations, this register is double buffered and can be written with a new value for the next operation; when not written, the value remains intact. During the execution of sequencer-controlled complex operations, this register may not be written and its value is undefined at the conclusion of the operation. The driver software cannot rely on the written value to remain intact.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aptr](aptr) module"]
pub type APTR = crate::Reg<u32, _APTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APTR;
#[doc = "`read()` method returns [aptr::R](aptr::R) reader structure"]
impl crate::Readable for APTR {}
#[doc = "`write(|w| ..)` method takes [aptr::W](aptr::W) writer structure"]
impl crate::Writable for APTR {}
#[doc = "PKA Vector A Address uring execution of basic PKCP operations, this register is double buffered and can be written with a new value for the next operation; when not written, the value remains intact. During the execution of sequencer-controlled complex operations, this register may not be written and its value is undefined at the conclusion of the operation. The driver software cannot rely on the written value to remain intact."]
pub mod aptr;
#[doc = "PKA Vector B Address uring execution of basic PKCP operations, this register is double buffered and can be written with a new value for the next operation; when not written, the value remains intact. During the execution of sequencer-controlled complex operations, this register may not be written and its value is undefined at the conclusion of the operation. The driver software cannot rely on the written value to remain intact.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bptr](bptr) module"]
pub type BPTR = crate::Reg<u32, _BPTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BPTR;
#[doc = "`read()` method returns [bptr::R](bptr::R) reader structure"]
impl crate::Readable for BPTR {}
#[doc = "`write(|w| ..)` method takes [bptr::W](bptr::W) writer structure"]
impl crate::Writable for BPTR {}
#[doc = "PKA Vector B Address uring execution of basic PKCP operations, this register is double buffered and can be written with a new value for the next operation; when not written, the value remains intact. During the execution of sequencer-controlled complex operations, this register may not be written and its value is undefined at the conclusion of the operation. The driver software cannot rely on the written value to remain intact."]
pub mod bptr;
#[doc = "PKA Vector C Address uring execution of basic PKCP operations, this register is double buffered and can be written with a new value for the next operation; when not written, the value remains intact. During the execution of sequencer-controlled complex operations, this register may not be written and its value is undefined at the conclusion of the operation. The driver software cannot rely on the written value to remain intact.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cptr](cptr) module"]
pub type CPTR = crate::Reg<u32, _CPTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPTR;
#[doc = "`read()` method returns [cptr::R](cptr::R) reader structure"]
impl crate::Readable for CPTR {}
#[doc = "`write(|w| ..)` method takes [cptr::W](cptr::W) writer structure"]
impl crate::Writable for CPTR {}
#[doc = "PKA Vector C Address uring execution of basic PKCP operations, this register is double buffered and can be written with a new value for the next operation; when not written, the value remains intact. During the execution of sequencer-controlled complex operations, this register may not be written and its value is undefined at the conclusion of the operation. The driver software cannot rely on the written value to remain intact."]
pub mod cptr;
#[doc = "PKA Vector D Address uring execution of basic PKCP operations, this register is double buffered and can be written with a new value for the next operation; when not written, the value remains intact. During the execution of sequencer-controlled complex operations, this register may not be written and its value is undefined at the conclusion of the operation. The driver software cannot rely on the written value to remain intact.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dptr](dptr) module"]
pub type DPTR = crate::Reg<u32, _DPTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPTR;
#[doc = "`read()` method returns [dptr::R](dptr::R) reader structure"]
impl crate::Readable for DPTR {}
#[doc = "`write(|w| ..)` method takes [dptr::W](dptr::W) writer structure"]
impl crate::Writable for DPTR {}
#[doc = "PKA Vector D Address uring execution of basic PKCP operations, this register is double buffered and can be written with a new value for the next operation; when not written, the value remains intact. During the execution of sequencer-controlled complex operations, this register may not be written and its value is undefined at the conclusion of the operation. The driver software cannot rely on the written value to remain intact."]
pub mod dptr;
#[doc = "PKA Vector A Length uring execution of basic PKCP operations, this register is double buffered and can be written with a new value for the next operation; when not written, the value remains intact. During the execution of sequencer-controlled complex operations, this register may not be written and its value is undefined at the conclusion of the operation. The driver software cannot rely on the written value to remain intact.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alength](alength) module"]
pub type ALENGTH = crate::Reg<u32, _ALENGTH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ALENGTH;
#[doc = "`read()` method returns [alength::R](alength::R) reader structure"]
impl crate::Readable for ALENGTH {}
#[doc = "`write(|w| ..)` method takes [alength::W](alength::W) writer structure"]
impl crate::Writable for ALENGTH {}
#[doc = "PKA Vector A Length uring execution of basic PKCP operations, this register is double buffered and can be written with a new value for the next operation; when not written, the value remains intact. During the execution of sequencer-controlled complex operations, this register may not be written and its value is undefined at the conclusion of the operation. The driver software cannot rely on the written value to remain intact."]
pub mod alength;
#[doc = "PKA Vector B Length uring execution of basic PKCP operations, this register is double buffered and can be written with a new value for the next operation; when not written, the value remains intact. During the execution of sequencer-controlled complex operations, this register may not be written and its value is undefined at the conclusion of the operation. The driver software cannot rely on the written value to remain intact.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blength](blength) module"]
pub type BLENGTH = crate::Reg<u32, _BLENGTH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BLENGTH;
#[doc = "`read()` method returns [blength::R](blength::R) reader structure"]
impl crate::Readable for BLENGTH {}
#[doc = "`write(|w| ..)` method takes [blength::W](blength::W) writer structure"]
impl crate::Writable for BLENGTH {}
#[doc = "PKA Vector B Length uring execution of basic PKCP operations, this register is double buffered and can be written with a new value for the next operation; when not written, the value remains intact. During the execution of sequencer-controlled complex operations, this register may not be written and its value is undefined at the conclusion of the operation. The driver software cannot rely on the written value to remain intact."]
pub mod blength;
#[doc = "PKA Bit Shift Value or basic PKCP operations, modifying the contents of this register is made impossible while the operation is being performed. For the ExpMod-variable and ExpMod-CRT operations, this register is used to indicate the number of odd powers to use (directly as a value in the range 1-16). For the ModInv and ECC operations, this register is used to hold a completion code.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shift](shift) module"]
pub type SHIFT = crate::Reg<u32, _SHIFT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHIFT;
#[doc = "`read()` method returns [shift::R](shift::R) reader structure"]
impl crate::Readable for SHIFT {}
#[doc = "`write(|w| ..)` method takes [shift::W](shift::W) writer structure"]
impl crate::Writable for SHIFT {}
#[doc = "PKA Bit Shift Value or basic PKCP operations, modifying the contents of this register is made impossible while the operation is being performed. For the ExpMod-variable and ExpMod-CRT operations, this register is used to indicate the number of odd powers to use (directly as a value in the range 1-16). For the ModInv and ECC operations, this register is used to hold a completion code."]
pub mod shift;
#[doc = "PKA Function his register contains the control bits to start basic PKCP as well as complex sequencer operations. The run bit can be used to poll for the completion of the operation. Modifying bits \\[11:0\\]
is made impossible during the execution of a basic PKCP operation. uring the execution of sequencer-controlled complex operations, this register is modified, the run and stall result bits are set to zero at the conclusion, but other bits are undefined. OTE: Continuously reading this register to poll the run bit is not allowed when executing complex sequencer operations (the sequencer cannot access the PKCP when this is done). Leave at least one sysclk cycle between poll operations.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [function](function) module"]
pub type FUNCTION = crate::Reg<u32, _FUNCTION>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNCTION;
#[doc = "`read()` method returns [function::R](function::R) reader structure"]
impl crate::Readable for FUNCTION {}
#[doc = "`write(|w| ..)` method takes [function::W](function::W) writer structure"]
impl crate::Writable for FUNCTION {}
#[doc = "PKA Function his register contains the control bits to start basic PKCP as well as complex sequencer operations. The run bit can be used to poll for the completion of the operation. Modifying bits \\[11:0\\]
is made impossible during the execution of a basic PKCP operation. uring the execution of sequencer-controlled complex operations, this register is modified, the run and stall result bits are set to zero at the conclusion, but other bits are undefined. OTE: Continuously reading this register to poll the run bit is not allowed when executing complex sequencer operations (the sequencer cannot access the PKCP when this is done). Leave at least one sysclk cycle between poll operations."]
pub mod function;
#[doc = "PKA compare result his register provides the result of a basic PKCP compare operation. It is updated when the FUNCTION.RUN bit is reset at the end of that operation. tatus after a complex sequencer operation is unknown\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [compare](compare) module"]
pub type COMPARE = crate::Reg<u32, _COMPARE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMPARE;
#[doc = "`read()` method returns [compare::R](compare::R) reader structure"]
impl crate::Readable for COMPARE {}
#[doc = "PKA compare result his register provides the result of a basic PKCP compare operation. It is updated when the FUNCTION.RUN bit is reset at the end of that operation. tatus after a complex sequencer operation is unknown"]
pub mod compare;
#[doc = "PKA most-significant-word of result vector his register indicates the (word) address in the PKA RAM where the most significant nonzero 32-bit word of the result is stored. Should be ignored for modulo operations. For basic PKCP operations, this register is updated FUNCTION.RUN bit is reset at the end of the operation. For the complex-sequencer controlled operations, updating of the final value matching the actual result is done near the end of the operation; note that the result is only meaningful if no errors were detected and that for ECC operations, this register will provide information for the x-coordinate of the result point only.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [msw](msw) module"]
pub type MSW = crate::Reg<u32, _MSW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MSW;
#[doc = "`read()` method returns [msw::R](msw::R) reader structure"]
impl crate::Readable for MSW {}
#[doc = "PKA most-significant-word of result vector his register indicates the (word) address in the PKA RAM where the most significant nonzero 32-bit word of the result is stored. Should be ignored for modulo operations. For basic PKCP operations, this register is updated FUNCTION.RUN bit is reset at the end of the operation. For the complex-sequencer controlled operations, updating of the final value matching the actual result is done near the end of the operation; note that the result is only meaningful if no errors were detected and that for ECC operations, this register will provide information for the x-coordinate of the result point only."]
pub mod msw;
#[doc = "PKA most-significant-word of divide remainder his register indicates the (32-bit word) address in the PKA RAM where the most significant nonzero 32-bit word of the remainder result for the basic divide and modulo operations is stored. Bits \\[4:0\\]
are loaded with the bit number of the most-significant nonzero bit in the most-significant nonzero word when MS one control bit is set. For divide, modulo, and MS one reporting, this register is updated when FUNCTION.RUN bit is reset at the end of the operation. For the complex sequencer controlled operations, updating of bits \\[4:0\\]
of this register with the most-significant bit location of the actual result is done near the end of the operation. The result is meaningful only if no errors were detected and that for ECC operations; this register provides information for the x-coordinate of the result point only.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [divmsw](divmsw) module"]
pub type DIVMSW = crate::Reg<u32, _DIVMSW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIVMSW;
#[doc = "`read()` method returns [divmsw::R](divmsw::R) reader structure"]
impl crate::Readable for DIVMSW {}
#[doc = "PKA most-significant-word of divide remainder his register indicates the (32-bit word) address in the PKA RAM where the most significant nonzero 32-bit word of the remainder result for the basic divide and modulo operations is stored. Bits \\[4:0\\]
are loaded with the bit number of the most-significant nonzero bit in the most-significant nonzero word when MS one control bit is set. For divide, modulo, and MS one reporting, this register is updated when FUNCTION.RUN bit is reset at the end of the operation. For the complex sequencer controlled operations, updating of bits \\[4:0\\]
of this register with the most-significant bit location of the actual result is done near the end of the operation. The result is meaningful only if no errors were detected and that for ECC operations; this register provides information for the x-coordinate of the result point only."]
pub mod divmsw;
#[doc = "PKA sequencer control and status register he sequencer is interfaced with the outside world through a single control and status register. With the exception of bit \\[31\\], the actual use of bits in the separate sub-fields of this register is determined by the sequencer firmware. This register need only be accessed when the sequencer program is stored in RAM. The reset value of the RESET bit depends upon the option chosen for sequencer program storage. NOTE: For Agama the sequencer firmware is executed from ROM.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seqctrl](seqctrl) module"]
pub type SEQCTRL = crate::Reg<u32, _SEQCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEQCTRL;
#[doc = "`read()` method returns [seqctrl::R](seqctrl::R) reader structure"]
impl crate::Readable for SEQCTRL {}
#[doc = "`write(|w| ..)` method takes [seqctrl::W](seqctrl::W) writer structure"]
impl crate::Writable for SEQCTRL {}
#[doc = "PKA sequencer control and status register he sequencer is interfaced with the outside world through a single control and status register. With the exception of bit \\[31\\], the actual use of bits in the separate sub-fields of this register is determined by the sequencer firmware. This register need only be accessed when the sequencer program is stored in RAM. The reset value of the RESET bit depends upon the option chosen for sequencer program storage. NOTE: For Agama the sequencer firmware is executed from ROM."]
pub mod seqctrl;
#[doc = "PKA hardware options register his register provides the host with a means to determine the hardware configuration implemented in this PKA engine, focused on options that have an effect on software interacting with the module.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [options](options) module"]
pub type OPTIONS = crate::Reg<u32, _OPTIONS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPTIONS;
#[doc = "`read()` method returns [options::R](options::R) reader structure"]
impl crate::Readable for OPTIONS {}
#[doc = "PKA hardware options register his register provides the host with a means to determine the hardware configuration implemented in this PKA engine, focused on options that have an effect on software interacting with the module."]
pub mod options;
#[doc = "PKA firmware revision and capabilities register his register allows the host access to the internal firmware revision number of the PKA Engine for software driver matching and diagnostic purposes. This register also contains a field that encodes the capabilities of the embedded firmware. his register is written by the firmware within a few clock cycles after starting up that firmware. The hardware reset value is zero, indicating that the information has not been written yet.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fwrev](fwrev) module"]
pub type FWREV = crate::Reg<u32, _FWREV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FWREV;
#[doc = "`read()` method returns [fwrev::R](fwrev::R) reader structure"]
impl crate::Readable for FWREV {}
#[doc = "PKA firmware revision and capabilities register his register allows the host access to the internal firmware revision number of the PKA Engine for software driver matching and diagnostic purposes. This register also contains a field that encodes the capabilities of the embedded firmware. his register is written by the firmware within a few clock cycles after starting up that firmware. The hardware reset value is zero, indicating that the information has not been written yet."]
pub mod fwrev;
#[doc = "PKA hardware revision register his register allows the host access to the hardware revision number of the PKA engine for software driver matching and diagnostic purposes. It is always located at the highest address in the access space of the module and contains an encoding of the EIP number (with its complement as signature) for recognition of the hardware module.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwrev](hwrev) module"]
pub type HWREV = crate::Reg<u32, _HWREV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HWREV;
#[doc = "`read()` method returns [hwrev::R](hwrev::R) reader structure"]
impl crate::Readable for HWREV {}
#[doc = "PKA hardware revision register his register allows the host access to the hardware revision number of the PKA engine for software driver matching and diagnostic purposes. It is always located at the highest address in the access space of the module and contains an encoding of the EIP number (with its complement as signature) for recognition of the hardware module."]
pub mod hwrev;
