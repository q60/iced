// SPDX-License-Identifier: MIT
// Copyright (C) 2018-present iced project and contributors

use crate::decoder::handlers::*;
use crate::decoder::*;
use crate::*;

#[cfg(not(feature = "no_d3now"))]
static CODE_VALUES: [Code; 0x100] = [
	// GENERATOR-BEGIN: D3nowCodeValues
	// ⚠️This was generated by GENERATOR!🦹‍♂️
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::D3NOW_Pi2fw_mm_mmm64,
	Code::D3NOW_Pi2fd_mm_mmm64,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::D3NOW_Pf2iw_mm_mmm64,
	Code::D3NOW_Pf2id_mm_mmm64,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::D3NOW_Pfrcpv_mm_mmm64,
	Code::D3NOW_Pfrsqrtv_mm_mmm64,
	Code::INVALID,
	Code::INVALID,
	Code::D3NOW_Pfnacc_mm_mmm64,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::D3NOW_Pfpnacc_mm_mmm64,
	Code::INVALID,
	Code::D3NOW_Pfcmpge_mm_mmm64,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::D3NOW_Pfmin_mm_mmm64,
	Code::INVALID,
	Code::D3NOW_Pfrcp_mm_mmm64,
	Code::D3NOW_Pfrsqrt_mm_mmm64,
	Code::INVALID,
	Code::INVALID,
	Code::D3NOW_Pfsub_mm_mmm64,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::D3NOW_Pfadd_mm_mmm64,
	Code::INVALID,
	Code::D3NOW_Pfcmpgt_mm_mmm64,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::D3NOW_Pfmax_mm_mmm64,
	Code::INVALID,
	Code::D3NOW_Pfrcpit1_mm_mmm64,
	Code::D3NOW_Pfrsqit1_mm_mmm64,
	Code::INVALID,
	Code::INVALID,
	Code::D3NOW_Pfsubr_mm_mmm64,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::D3NOW_Pfacc_mm_mmm64,
	Code::INVALID,
	Code::D3NOW_Pfcmpeq_mm_mmm64,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::D3NOW_Pfmul_mm_mmm64,
	Code::INVALID,
	Code::D3NOW_Pfrcpit2_mm_mmm64,
	Code::D3NOW_Pmulhrw_mm_mmm64,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::D3NOW_Pswapd_mm_mmm64,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::D3NOW_Pavgusb_mm_mmm64,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	Code::INVALID,
	// GENERATOR-END: D3nowCodeValues
];

#[allow(non_camel_case_types)]
#[repr(C)]
pub(super) struct OpCodeHandler_D3NOW {
	has_modrm: bool,
}

#[cfg(not(feature = "no_d3now"))]
impl OpCodeHandler_D3NOW {
	#[inline]
	pub(super) fn new() -> (OpCodeHandlerDecodeFn, Self) {
		debug_assert_eq!(CODE_VALUES.len(), 0x100);
		(OpCodeHandler_D3NOW::decode, Self { has_modrm: true })
	}

	fn decode(_self_ptr: *const OpCodeHandler, decoder: &mut Decoder<'_>, instruction: &mut Instruction) {
		debug_assert_eq!(decoder.state.encoding(), EncodingKind::Legacy as u32);
		const_assert_eq!(OpKind::Register as u32, 0);
		//instruction.set_op0_kind(OpKind::Register);
		instruction.set_op0_register(unsafe { mem::transmute((decoder.state.reg + Register::MM0 as u32) as RegisterUnderlyingType) });
		if decoder.state.mod_ == 3 {
			const_assert_eq!(OpKind::Register as u32, 0);
			//instruction.set_op1_kind(OpKind::Register);
			instruction.set_op1_register(unsafe { mem::transmute((decoder.state.rm + Register::MM0 as u32) as RegisterUnderlyingType) });
		} else {
			instruction.set_op1_kind(OpKind::Memory);
			decoder.read_op_mem(instruction);
		}
		let ib = decoder.read_u8();
		let mut code = CODE_VALUES[ib];
		match code {
			Code::D3NOW_Pfrcpv_mm_mmm64 | Code::D3NOW_Pfrsqrtv_mm_mmm64 => {
				if (decoder.options & DecoderOptions::CYRIX) == 0 || decoder.bitness() == 64 {
					code = Code::INVALID;
				}
			}
			_ => {}
		}
		instruction.set_code(code);
		if code == Code::INVALID {
			decoder.set_invalid_instruction();
		}
	}
}

#[cfg(feature = "no_d3now")]
impl OpCodeHandler_D3NOW {
	#[inline]
	pub(super) fn new() -> (OpCodeHandlerDecodeFn, Self) {
		(OpCodeHandler_D3NOW::decode, Self { has_modrm: true })
	}

	fn decode(_self_ptr: *const OpCodeHandler, decoder: &mut Decoder<'_>, _instruction: &mut Instruction) {
		debug_assert_eq!(decoder.state.encoding(), EncodingKind::Legacy as u32);
		decoder.set_invalid_instruction();
	}
}
