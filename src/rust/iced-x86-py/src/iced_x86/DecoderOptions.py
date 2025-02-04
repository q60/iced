# SPDX-License-Identifier: MIT
# Copyright (C) 2018-present iced project and contributors

# ⚠️This file was generated by GENERATOR!🦹‍♂️

# pylint: disable=invalid-name
# pylint: disable=line-too-long
# pylint: disable=too-many-lines

"""
Decoder options
"""

import typing
if typing.TYPE_CHECKING:
	from ._iced_x86_py import DecoderOptions
else:
	DecoderOptions = int

NONE: DecoderOptions = 0x0000_0000 # type: ignore
"""
No option is enabled
"""
NO_INVALID_CHECK: DecoderOptions = 0x0000_0001 # type: ignore
"""
Disable some checks for invalid encodings of instructions, eg. most instructions can't use a ``LOCK`` prefix so if one is found, they're decoded as :class:`iced_x86.Code.INVALID` unless this option is enabled.
"""
AMD: DecoderOptions = 0x0000_0002 # type: ignore
"""
: AMD decoder: allow 16-bit branch/ret instructions in 64-bit mode, no ``o64 CALL/JMP FAR [mem], o64 LSS/LFS/LGS``, ``UD0`` has no modr/m byte, decode ``LOCK MOV CR``. The AMD decoder can still decode Intel instructions.
"""
FORCE_RESERVED_NOP: DecoderOptions = 0x0000_0004 # type: ignore
"""
Decode opcodes ``0F0D`` and ``0F18-0F1F`` as reserved-nop instructions (eg. :class:`iced_x86.Code.RESERVEDNOP_RM32_R32_0F1D`)
"""
UMOV: DecoderOptions = 0x0000_0008 # type: ignore
"""
Decode ``UMOV`` instructions
"""
XBTS: DecoderOptions = 0x0000_0010 # type: ignore
"""
Decode ``XBTS``/``IBTS``
"""
CMPXCHG486A: DecoderOptions = 0x0000_0020 # type: ignore
"""
Decode ``0FA6``/``0FA7`` as ``CMPXCHG``
"""
OLD_FPU: DecoderOptions = 0x0000_0040 # type: ignore
"""
Decode some old removed FPU instructions (eg. ``FRSTPM``)
"""
PCOMMIT: DecoderOptions = 0x0000_0080 # type: ignore
"""
Decode ``PCOMMIT``
"""
LOADALL286: DecoderOptions = 0x0000_0100 # type: ignore
"""
Decode 286 ``STOREALL``/``LOADALL`` (``0F04`` and ``0F05``)
"""
LOADALL386: DecoderOptions = 0x0000_0200 # type: ignore
"""
Decode 386 ``LOADALL``
"""
CL1INVMB: DecoderOptions = 0x0000_0400 # type: ignore
"""
Decode ``CL1INVMB``
"""
MOV_TR: DecoderOptions = 0x0000_0800 # type: ignore
"""
Decode ``MOV r32,tr`` and ``MOV tr,r32``
"""
JMPE: DecoderOptions = 0x0000_1000 # type: ignore
"""
Decode ``JMPE`` instructions
"""
NO_PAUSE: DecoderOptions = 0x0000_2000 # type: ignore
"""
Don't decode ``PAUSE``, decode ``NOP`` instead
"""
NO_WBNOINVD: DecoderOptions = 0x0000_4000 # type: ignore
"""
Don't decode ``WBNOINVD``, decode ``WBINVD`` instead
"""
UDBG: DecoderOptions = 0x0000_8000 # type: ignore
"""
Decode undocumented Intel ``RDUDBG`` and ``WRUDBG`` instructions
"""
NO_MPFX_0FBC: DecoderOptions = 0x0001_0000 # type: ignore
"""
Don't decode ``TZCNT``, decode ``BSF`` instead
"""
NO_MPFX_0FBD: DecoderOptions = 0x0002_0000 # type: ignore
"""
Don't decode ``LZCNT``, decode ``BSR`` instead
"""
NO_LAHF_SAHF_64: DecoderOptions = 0x0004_0000 # type: ignore
"""
Don't decode ``LAHF`` and ``SAHF`` in 64-bit mode
"""
MPX: DecoderOptions = 0x0008_0000 # type: ignore
"""
Decode ``MPX`` instructions
"""
CYRIX: DecoderOptions = 0x0010_0000 # type: ignore
"""
: Decode most Cyrix instructions: ``FPU``, ``EMMI``, ``SMM``, ``DDI``
"""
CYRIX_SMINT_0F7E: DecoderOptions = 0x0020_0000 # type: ignore
"""
Decode Cyrix ``SMINT 0F7E`` (Cyrix 6x86 or earlier)
"""
CYRIX_DMI: DecoderOptions = 0x0040_0000 # type: ignore
"""
Decode Cyrix ``DMI`` instructions (AMD Geode GX/LX)
"""
ALTINST: DecoderOptions = 0x0080_0000 # type: ignore
"""
Decode Centaur ``ALTINST``
"""
