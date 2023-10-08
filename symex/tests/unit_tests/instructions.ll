; source_filename = ""
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-linux-gnu"

; --------------------------------------------------------------------------------------------------
; Unary Operations
;
; Unsupported: fneg
; --------------------------------------------------------------------------------------------------


; --------------------------------------------------------------------------------------------------
; Binary Operations
;
; add, sub, mul, udiv, sdiv, urem, srem
; Unsupported: fadd, fsub, fmul, fdiv, frem
; --------------------------------------------------------------------------------------------------

; Test add
define dso_local i64 @test_add() #0 {
    %1 = add i64 5, 10
    ret i64 %1 ; expect 15
}

define dso_local i8 @test_add_overflow() #0 {
    %1 = add i8 200, 100
    ret i8 %1
}

; Test add vectors
define dso_local <2 x i32> @test_add_vector() #0 {
    %1 = add <2 x i32> <i32 5, i32 10>, <i32 20, i32 100>
    ret <2 x i32> %1 ; expect <25, 110>
    ; In memory this will be laid out as
    ;   [0x0000_0019, 0x0000_006E]
    ; and when checking the result it will be casted to an i64, which in little endian would yield
    ;   0x0000_006E_0000_0019
}


; Test sub
define dso_local i64 @test_sub() #0 {
    %1 = sub i64 20, 10
    ret i64 %1 ; expect 10
}

; Test mul
define dso_local i64 @test_mul() #0 {
    %1 = mul i64 5, 7
    ret i64 %1 ; expect 35
}

; Test udiv
define dso_local i64 @test_udiv() #0 {
    %1 = udiv i64 200, 10
    ret i64 %1 ; expect 20
}

; Test sdiv
define dso_local i64 @test_sdiv() #0 {
    %1 = sdiv i64 200, -10
    ret i64 %1 ; expect -20
}

; Test urem
define dso_local i64 @test_urem() #0 {
    %1 = urem i64 15, 4
    ret i64 %1 ; expect 3
}

; Test srem
define dso_local i64 @test_srem() #0 {
    %1 = srem i64 15, -4
    ret i64 %1 ; expect 3
}

; Test srem
define dso_local i64 @test_srem2() #0 {
    %1 = srem i64 -15, 4
    ret i64 %1 ; expect -3
}

; --------------------------------------------------------------------------------------------------
; Bitwise Binary Operations
;
; and, or, xor, shl, lshr, ashr
; --------------------------------------------------------------------------------------------------

define dso_local i64 @test_and() #0 {
    %1 = and i64 58, 49 ; 0b0011_1010 & 0b0011_0001
    ret i64 %1 ; expect 48 (0b0011_0000)
}

define dso_local i64 @test_or() #0 {
    %1 = or i64 58, 49 ; 0b0011_1010 & 0b0011_0001
    ret i64 %1 ; expect 59 (0b0011_1011)
}

define dso_local i64 @test_xor() #0 {
    %1 = xor i64 58, 49 ; 0b0011_1010 & 0b0011_0001
    ret i64 %1 ; expect 11 (0b0000_1011)
}

define dso_local i64 @test_shl() #0 {
    %1 = shl i64 58, 1 ; 0b0011_1010 << 1
    ret i64 %1 ; expect 116 (0b0111_0100)
}

define dso_local i64 @test_lshr() #0 {
    %1 = lshr i64 58, 1 ; 0b0011_1010 >> 1
    ret i64 %1 ; expect 29 (0b0001_1101)
}

define dso_local i8 @test_ashr() #0 {
    %1 = ashr i8 59, 1 ; 0b0011_1011 >> 1
    ret i8 %1 ; expect 29 (0b0001_1101)
}

define dso_local i8 @test_ashr2() #0 {
    %1 = ashr i8 187, 1 ; 0b1011_1011 >> 1
    ret i8 %1 ; expect 221 (-35) (0b1101_1101)
}

; --------------------------------------------------------------------------------------------------
; Vector Operations
;
; Unsupported: extractelement, insertelement, shufflevector
; --------------------------------------------------------------------------------------------------

; --------------------------------------------------------------------------------------------------
; Aggregate Operations
;
; extractvalue, insertvalue
; --------------------------------------------------------------------------------------------------

; extract value

define dso_local i32 @test_extract_value_arr1() #0 {
    %1 = extractvalue [4 x i32] [i32 1, i32 2, i32 3, i32 4], 0
    ret i32 %1 ; expect 1
}

define dso_local i32 @test_extract_value_arr2() #0 {
    %1 = extractvalue [4 x i32] [i32 1, i32 2, i32 3, i32 4], 1
    ret i32 %1 ; expect 2
}

define dso_local i32 @test_extract_value_arr3() #0 {
    %1 = extractvalue [4 x i32] [i32 1, i32 2, i32 3, i32 4], 2
    ret i32 %1 ; expect 3
}

define dso_local i32 @test_extract_value_arr4() #0 {
    %1 = extractvalue [4 x i32] [i32 1, i32 2, i32 3, i32 4], 3
    ret i32 %1 ; expect 4
}

define dso_local i32 @test_extract_value_struct1() #0 {
    %1 = extractvalue { i32, i64, [2 x i32] } { i32 4, i64 10, [2 x i32] [i32 1, i32 2]}, 0
    ret i32 %1 ; expect 4
}

define dso_local i64 @test_extract_value_struct2() #0 {
    %1 = extractvalue { i32, i64, [2 x i32] } { i32 4, i64 10, [2 x i32] [i32 1, i32 2]}, 1
    ret i64 %1 ; expect 10
}

define dso_local i32 @test_extract_value_struct3() #0 {
    %1 = extractvalue { i32, i64, [2 x i32] } { i32 4, i64 10, [2 x i32] [i32 1, i32 2]}, 2, 0
    ret i32 %1 ; expect 1
}

define dso_local i32 @test_extract_value_struct4() #0 {
    %1 = extractvalue { i32, i64, [2 x i32] } { i32 4, i64 10, [2 x i32] [i32 1, i32 2]}, 2, 1
    ret i32 %1 ; expect 2
}

; insert value

define dso_local [4 x i8] @test_insert_value_arr1() #0 {
    %1 = insertvalue [4 x i8] [i8 1, i8 2, i8 3, i8 4], i8 10, 0
    ret [4 x i8] %1 ; [0x0a, 0x02, 0x03, 0x04] as i64 = 0x0403020a
}

define dso_local [4 x i8] @test_insert_value_arr2() #0 {
    %1 = insertvalue [4 x i8] [i8 1, i8 2, i8 3, i8 4], i8 10, 1
    ret [4 x i8] %1 ; [0x01, 0x0a, 0x03, 0x04] as i64 = 0x04030a01
}

define dso_local [4 x i8] @test_insert_value_arr3() #0 {
    %1 = insertvalue [4 x i8] [i8 1, i8 2, i8 3, i8 4], i8 10, 2
    ret [4 x i8] %1 ; [0x01, 0x02, 0x0a, 0x04] as i64 = 0x040a0201
}

define dso_local [4 x i8] @test_insert_value_arr4() #0 {
    %1 = insertvalue [4 x i8] [i8 1, i8 2, i8 3, i8 4], i8 10, 3
    ret [4 x i8] %1 ; [0x01, 0x02, 0x03, 0x0a] as i64 = 0x0a030201
}

define dso_local [4 x i16] @test_insert_value_arr5() #0 {
    ; [0x0102, 0x0304, 0x0405, 0x0506], insert 0x0708
    %1 = insertvalue [4 x i16] [i16 258, i16 772, i16 1029, i16 1286], i16 1800, 2
    ret [4 x i16] %1
    ; [0x0102, 0x0304, 0x0708, 0x0506] as i64
    ;   0x0102 | 0x0304 << 16 | 0x0708 << 32 | 0x0506 << 48 =
    ;   0x0506070803040102 (expected)
}

define dso_local { i16, i16, [2 x i16] } @test_insert_value_struct1() #0 {
    %1 = insertvalue { i16, i16, [2 x i16] } { i16 4, i16 10, [2 x i16] [i16 1, i16 2]}, i16 15, 0
    ret { i16, i16, [2 x i16] } %1
    ; { 0x0f, 0x0a, [0x01, 0x02] } as i64 gives
    ;   { 0x000f, 0x000a, [0x0001, 0x0002] } =
    ;   0x000f | 0x000a << 16 | 0x0001 << 32 | 0x0002 << 48 =
    ;   0x00020001000a000f (expected)
}

define dso_local { i16, i16, [2 x i16] } @test_insert_value_struct2() #0 {
    %1 = insertvalue { i16, i16, [2 x i16] } { i16 4, i16 10, [2 x i16] [i16 1, i16 2]}, i16 15, 1
    ret { i16, i16, [2 x i16] } %1
    ; { 0x04, 0x0f, [0x01, 0x02] } as i64 gives
    ;   { 0x0004, 0x000f, [0x0001, 0x0002] } =
    ;   0x0004 | 0x000f << 16 | 0x0001 << 32 | 0x0002 << 48 =
    ;   0x00020001000f0004 (expected)
}

define dso_local { i16, i16, [2 x i16] } @test_insert_value_struct3() #0 {
    %1 = insertvalue { i16, i16, [2 x i16] } { i16 4, i16 10, [2 x i16] [i16 1, i16 2]}, i16 15, 2, 0
    ret { i16, i16, [2 x i16] } %1
    ; { 0x04, 0x0a, [0x0f, 0x02] } as i64 gives
    ;   { 0x0004, 0x000a, [0x000f, 0x0002] } =
    ;   0x0004 | 0x000a << 16 | 0x000f << 32 | 0x0002 << 48 =
    ;   0x0002000f000a0004 (expected)
}

define dso_local { i16, i16, [2 x i16] } @test_insert_value_struct4() #0 {
    %1 = insertvalue { i16, i16, [2 x i16] } { i16 4, i16 10, [2 x i16] [i16 1, i16 2]}, i16 15, 2, 1
    ret { i16, i16, [2 x i16] } %1
    ; { 0x04, 0x0a, [0x01, 0x0f] } as i64 gives
    ;   { 0x0004, 0x000a, [0x0001, 0x000f] } =
    ;   0x0004 | 0x000a << 16 | 0x0001 << 32 | 0x000f << 48 =
    ;   0x000f0001000a0004 (expected)
}


; --------------------------------------------------------------------------------------------------
; Memory access and Addressing Operations
;
; alloca, load, store, getelementptr
; Unsupported: fence, cmpxchg, atomicrmw
; --------------------------------------------------------------------------------------------------

; tests with alloca, load & store.

define dso_local i32 @test_load_store1() #0 {
    %ptr1 = alloca i32, align 4
    %ptr2 = alloca i32, align 4
    store i32 u0x1234, i32* %ptr1
    store i32 u0x3456, i32* %ptr2
    %val = load i32, i32* %ptr2
    ret i32 %val ; expect 0x3456
}

define dso_local i8 @test_load_store2() #0 {
    %ptr1 = alloca i32, align 4
    %ptr2 = alloca i32, align 4
    store i32 u0x1234, i32* %ptr1
    store i32 u0x3456, i32* %ptr2
    %ptr = bitcast i32* %ptr2 to i8*
    %val = load i8, i8* %ptr
    ret i8 %val ; expect 0x56
}

; Simpler version of the struct in the LLVM IR reference.
%struct.RT = type { i8, [2 x i32], i8 }
%struct.ST = type { i32, i64, %struct.RT }

define dso_local i32 @test_gep1() #0 {
    %ptr = alloca %struct.ST, align 8
    store %struct.ST { i32 1, i64 2, %struct.RT { i8 3, [2 x i32] [i32 4, i32 5], i8 6 } }, ptr %ptr
    %1 = getelementptr inbounds %struct.ST, ptr %ptr, i64 0, i32 0
    %2 = load i32, i32* %1
    ret i32 %2 ; expect 1 ([0])
}q

define dso_local i64 @test_gep2() #0 {
    %ptr = alloca %struct.ST, align 8
    store %struct.ST { i32 1, i64 2, %struct.RT { i8 3, [2 x i32] [i32 4, i32 5], i8 6 } }, ptr %ptr
    %1 = getelementptr inbounds %struct.ST, ptr %ptr, i64 0, i32 1
    %2 = load i64, i64* %1
    ret i64 %2 ; expect 2 ([1])
}

define dso_local i8 @test_gep3() #0 {
    %ptr = alloca %struct.ST, align 8
    store %struct.ST { i32 1, i64 2, %struct.RT { i8 3, [2 x i32] [i32 4, i32 5], i8 6 } }, ptr %ptr
    %1 = getelementptr inbounds %struct.ST, ptr %ptr, i64 0, i32 2, i32 0
    %2 = load i8, i8* %1
    ret i8 %2 ; expect 3 ([2][0])
}

define dso_local i32 @test_gep4() #0 {
    %ptr = alloca %struct.ST, align 8
    store %struct.ST { i32 1, i64 2, %struct.RT { i8 3, [2 x i32] [i32 4, i32 5], i8 6 } }, %struct.ST* %ptr
    %1 = getelementptr inbounds %struct.ST, %struct.ST* %ptr, i64 0, i32 2, i32 1, i32 0
    %2 = load i32, i32* %1
    ret i32 %2 ; expect 4 ([2][1][0])
}

define dso_local i32 @test_gep5() #0 {
    %ptr = alloca %struct.ST, align 8
    store %struct.ST { i32 1, i64 2, %struct.RT { i8 3, [2 x i32] [i32 4, i32 5], i8 6 } }, %struct.ST* %ptr
    %1 = getelementptr inbounds %struct.ST, %struct.ST* %ptr, i64 0, i32 2, i32 1, i32 1
    %2 = load i32, i32* %1
    ret i32 %2 ; expect 5 ([2][1][1])
}

define dso_local i8 @test_gep6() #0 {
    %ptr = alloca %struct.ST, align 8
    store %struct.ST { i32 1, i64 2, %struct.RT { i8 3, [2 x i32] [i32 4, i32 5], i8 6 } }, %struct.ST* %ptr
    %1 = getelementptr inbounds %struct.ST, %struct.ST* %ptr, i64 0, i32 2, i32 2
    %2 = load i8, i8* %1
    ret i8 %2 ; expect 6 ([2][2])
}

define dso_local i32 @test_gep_arr() #0 {
    %ptr = alloca [4 x i32], align 4
    store [4 x i32] [i32 0, i32 1, i32 2, i32 3], [4 x i32]* %ptr
    %1 = getelementptr inbounds [4 x i32], [4 x i32]* %ptr, i64 0, i32 2
    %2 = load i32, i32* %1
    ret i32 %2 ; expect 2 ([2])
}

define dso_local i32 @test_gep_arr2() #0 {
    %ptr = alloca [4 x i32], align 4
    store [4 x i32] [i32 0, i32 1, i32 2, i32 3], [4 x i32]* %ptr
    ; so for this assume we have the c code:
    ;   int arr[4] = {0, 1, 2, 3};
    ; a `getelementptr ty, ty* %arr, i64 1` can be seen as
    ;   int* ptr = &arr[1] // (char*)arr + 4
    %ptr2 = bitcast [4 x i32]* %ptr to i32*
    %1 = getelementptr inbounds i32, i32* %ptr2, i64 2
    ; should be the same as the previous example
    %2 = load i32, i32* %1
    ret i32 %2 ; expect 2 ([2])
}

; --------------------------------------------------------------------------------------------------
; Conversion Operations
;
; trunc, zext, sext, ptrtoint, inttoptr, bitcast, addrspacecast
; Unsupported: fptrunc, fpext, fptoui, fptosi, uitpfp, sitofp
; --------------------------------------------------------------------------------------------------

define dso_local i8 @test_trunc() #0 {
    %1 = trunc i32 u0xABCD to i8
    ret i8 %1 ; expect: 0xCD
}

define dso_local i16 @test_zext() #0 {
    %1 = zext i8 u0xFF to i16
    ret i16 %1 ; expect: 0x00FF
}

define dso_local <2 x i16> @test_zext_vec() #0 {
    %1 = zext <2 x i8> <i8 u0xFF, i8 u0x0F> to <2 x i16>
    ret <2 x i16> %1 ; expect: 0x000F00FF
}

define dso_local i16 @test_sext() #0 {
    %1 = sext i8 u0xFF to i16
    ret i16 %1 ; expect: 0xFFFF
}

; on the current architecture the pointer size is 64 bits.

; inttoptr

define dso_local i32* @test_inttoptr_trunc() #0 {
    %1 = inttoptr i128 u0x11112222333344445555666677778888 to i32*
    ret i32* %1 ; expect 0x5555666677778888
}

define dso_local i32* @test_inttoptr_noop() #0 {
    %1 = inttoptr i64 u0x1111222233334444 to i32*
    ret i32* %1 ; expect 0x1111222233334444
}

define dso_local i32* @test_inttoptr_extend() #0 {
    %1 = inttoptr i32 u0x11112222 to i32*
    ret i32* %1 ; expect 0x0000000011112222
}

; ptrtoint

define dso_local i8 @test_ptrtoint_trunc() #0 {
    %1 = inttoptr i64 u0x1111222233334444 to i32*
    %2 = ptrtoint i32* %1 to i8
    ret i8 %2 ; expect: 0x44
}

define dso_local i64 @test_ptrtoint_noop() #0 {
    %1 = inttoptr i64 u0x1111222233334444 to i32*
    %2 = ptrtoint i32* %1 to i64
    ret i64 %2 ; expect: 0x1111222233334444
}

; todo
; define dso_local i64 @test_ptrtoint_extend() #0 {}

define dso_local <2 x i32> @test_ptrtoint_vec_trunc() #0 {
    %1 = inttoptr <2 x i64> <i64 u0x1111222233334444, i64 u0x5555666677778888> to <2 x i32*>
    %2 = ptrtoint <2 x i32*> %1 to <2 x i32>
    ret <2 x i32> %2 ; expect: 0x7777888833334444
}

define dso_local i32 @test_bitcast1() #0 {
    ; <0x1234, 0x5678> as i32
    %1 = bitcast <2 x i16> <i16 4660, i16 22136> to i32
    ret i32 %1 ; expect: 0x56781234
}

define dso_local i32 @test_bitcast2() #0 {
    ; <0x12, 0x34, 0x56, 0x78> as i32
    %1 = bitcast <4 x i8> <i8 u0x12, i8 u0x34, i8 u0x56, i8 u0x78> to i32
    ret i32 %1 ; expect: 0x78563412
}

; these are currently the same as bitcasts, since addressspaces aren't supported.
define dso_local i32 addrspace(1)* @test_addrspacecast() #0 {
    %1 = inttoptr i64 u0x1111222233334444 to i32*
    %2 = addrspacecast i32* %1 to i32 addrspace(1)*
    ret i32 addrspace(1)* %2 ; expect: 0x1111222233334444
}

; --------------------------------------------------------------------------------------------------
; Other Operations
;
; icmp, phi, select, call
; Unsupported: fcmp, freeze, va_arg, landingpad, catchpad, cleanuppad
; --------------------------------------------------------------------------------------------------

; eq: lhs == rhs
define dso_local i1 @test_icmp_eq_false() #0 {
    %1 = icmp eq i32 4, 5
    ret i1 %1 ; expect: 0x0
}

define dso_local i1 @test_icmp_eq_true() #0 {
    %1 = icmp eq i32 4, 4
    ret i1 %1 ; expect: 0x1
}

; ne: lhs != rhs
define dso_local i1 @test_icmp_ne_false() #0 {
    %1 = icmp ne i32 4, 4
    ret i1 %1 ; expect: 0x0
}

define dso_local i1 @test_icmp_ne_true() #0 {
    %1 = icmp ne i32 4, 5
    ret i1 %1 ; expect: 0x1
}

; ugt: lhs > rhs
define dso_local i1 @test_icmp_ugt_false() #0 {
    %1 = icmp ugt i32 4, 4
    ret i1 %1 ; expect: 0x0
}

define dso_local i1 @test_icmp_ugt_true() #0 {
    %1 = icmp ugt i32 5, 4
    ret i1 %1 ; expect: 0x1
}

; uge: lhs >= rhs
define dso_local i1 @test_icmp_uge_false() #0 {
    %1 = icmp uge i32 4, 5
    ret i1 %1 ; expect: 0x0
}

define dso_local i1 @test_icmp_uge_true() #0 {
    %1 = icmp uge i32 4, 4
    ret i1 %1 ; expect: 0x1
}

; ult: lhs < rhs
define dso_local i1 @test_icmp_ult_false() #0 {
    %1 = icmp ult i32 4, 4
    ret i1 %1 ; expect: 0x0
}

define dso_local i1 @test_icmp_ult_true() #0 {
    %1 = icmp ult i32 4, 5
    ret i1 %1 ; expect: 0x1
}

; ule: lhs <= rhs
define dso_local i1 @test_icmp_ule_false() #0 {
    %1 = icmp ule i32 5, 4
    ret i1 %1 ; expect: 0x0
}

define dso_local i1 @test_icmp_ule_true() #0 {
    %1 = icmp ule i32 4, 4
    ret i1 %1 ; expect: 0x1
}

; sgt: lhs > rhs
define dso_local i1 @test_icmp_sgt_false() #0 {
    %1 = icmp sgt i32 -4, -4
    ret i1 %1 ; expect: 0x0
}

define dso_local i1 @test_icmp_sgt_true() #0 {
    %1 = icmp sgt i32 -4, -5
    ret i1 %1 ; expect: 0x1
}

; sge: lhs >= rhs
define dso_local i1 @test_icmp_sge_false() #0 {
    %1 = icmp sge i32 -5, -4
    ret i1 %1 ; expect: 0x0
}

define dso_local i1 @test_icmp_sge_true() #0 {
    %1 = icmp sge i32 -4, -4
    ret i1 %1 ; expect: 0x1
}

; slt: lhs < rhs
define dso_local i1 @test_icmp_slt_false() #0 {
    %1 = icmp slt i32 -4, -4
    ret i1 %1 ; expect: 0x0
}

define dso_local i1 @test_icmp_slt_true() #0 {
    %1 = icmp slt i32 -4, -3
    ret i1 %1 ; expect: 0x1
}

; sle: lhs <= rhs
define dso_local i1 @test_icmp_sle_false() #0 {
    %1 = icmp sle i32 -4, -5
    ret i1 %1 ; expect: 0x0
}

define dso_local i1 @test_icmp_sle_true() #0 {
    %1 = icmp sle i32 -4, -4
    ret i1 %1 ; expect: 0x1
}

; eq vec
define dso_local <3 x i1> @test_icmp_eq_vec() #0 {
    %1 = icmp eq <3 x i32> <i32 1, i32 2, i32 3>, <i32 1, i32 3, i32 3>
    ret <3 x i1> %1 ; expect: <0x1, 0x0, 0x1> -> 0b0101 -> 0x5
}

; phi

define dso_local i32 @test_phi1() #0 {
    %1 = add i32 0, 100
    %2 = icmp eq i32 %1, 100
    br i1 %2, label %bb1, label %bb2
bb1:
    %3 = add i32 0, u0xab ; 171
    br label %bb3
bb2:
    %4 = add i32 0, u0xcd ; 205
    br label %bb3
bb3:
    %5 = phi i32 [%3, %bb1], [%4, %bb2]
    ret i32 %5 ; expect: 0xab
}

define dso_local i32 @test_phi2() #0 {
    %1 = add i32 0, 100
    %2 = icmp eq i32 %1, 101
    br i1 %2, label %bb1, label %bb2
bb1:
    %3 = add i32 0, u0xab
    br label %bb3
bb2:
    %4 = add i32 0, u0xcd
    br label %bb3
bb3:
    %5 = phi i32 [%3, %bb1], [%4, %bb2]
    ret i32 %5 ; expect: 0xcd
}

; select

define dso_local i32 @test_select1() #0 {
    %1 = add i32 0, 100
    %2 = icmp eq i32 %1, 100
    %3 = select i1 %2, i32 u0xaa, i32 u0xbb
    ret i32 %3 ; expect 0xaa
}

define dso_local i32 @test_select2() #0 {
    %1 = add i32 0, 100
    %2 = icmp eq i32 %1, 101
    %3 = select i1 %2, i32 u0xaa, i32 u0xbb
    ret i32 %3 ; expect 0xbb
}

define dso_local <2 x i32> @test_select_vec_values() #0 {
    %1 = add i32 0, 100
    %2 = icmp eq i32 %1, 101
    %3 = select i1 %2, <2 x i32> <i32 u0xaa, i32 u0xbb>, <2 x i32> <i32 u0xcc, i32 u0xdd>
    ret <2 x i32> %3 ; expect <0xcc, 0xdd> -> 0x000000dd000000cc
}

define dso_local <2 x i32> @test_select_vec_cond() #0 {
    %1 = add <2 x i32> <i32 5, i32 15>, <i32 10, i32 20> ; <i32 15, i32 35>
    %2 = icmp eq <2 x i32> %1, <i32 15, i32 30>          ; <i1 1, i1 0>
    %3 = select <2 x i1> %2, <2 x i32> <i32 u0xaa, i32 u0xbb>, <2 x i32> <i32 u0xcc, i32 u0xdd>
    ret <2 x i32> %3 ; expect <0xaa, 0xdd> -> 0x000000dd000000aa
}

; call

define dso_local i32 @test_call_called_function() #0 {
    ret i32 u0xabcd
}

define dso_local i32 @test_call() #0 {
    %1 = call i32 @test_call_called_function()
    ret i32 %1 ; expect 0xabcd
}

; --------------------------------------------------------------------------------------------------
; Constants
; --------------------------------------------------------------------------------------------------

; Example from https://llvm.org/docs/LangRef.html#vector-type
;
; The engine should treat stuff as little-endian.
define dso_local i16 @test_vector_constant() #0 {
    %val = bitcast <4 x i4> <i4 1, i4 2, i4 3, i4 5> to i16
    ; Bitcasting from a vector to an integral type can be seen as
    ; concatenating the values:
    ;   %val now has the hexadecimal value 0x5321.
    ret i16 %val
}

define dso_local i32 @test_vector_constant2() #0 {
    %val = bitcast <4 x i8> <i8 1, i8 2, i8 3, i8 4> to i32
    ; These are laid out as array, so in the format
    ;   [0x01, 0x02, 0x03, 0x04]
    ; So a cast to an i32 would yield
    ;   0x04030201 which is the same as above for an i32 in little-endian.
    ret i32 %val
}

declare void @assume(i32) #1

attributes #0 = { noinline nounwind optnone sspstrong uwtable "frame-pointer"="all" "min-legal-vector-width"="0" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #1 = { "frame-pointer"="all" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
