target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-linux-gnu"

; Copy over symex_lib::assume declaration from a generated ll.
declare void @_ZN9symex_lib6assume17hfd5bf6c9c604b625E(i1 zeroext) unnamed_addr #1

; symex_lib::symbolic for i32.
declare void @_ZN9symex_lib8symbolic17h692d82273b6bba04E(i32* align 4) unnamed_addr #1

; Create a symbolic value containing a value in the given range.
define dso_local i32 @symbolic_range(i32 %min, i32 %max) #0 {
    ; make symbolic
    %local = alloca i32, align 4
    call void @_ZN9symex_lib8symbolic17h692d82273b6bba04E(i32* align 4 %local)

    ; setup condition: min <= len <= max
    %var = load i32, i32* %local
    %c0 = icmp uge i32 %var, 3
    %c1 = icmp ule i32 %var, 4
    %cond = and i1 %c0, %c1

    ; assume condition
    call void @_ZN9symex_lib6assume17hfd5bf6c9c604b625E(i1 zeroext %cond)

    ret i32 %var
}

; --------------------------------------------------------------------------------------------------
; Standard C/C++ intrinsics
; --------------------------------------------------------------------------------------------------

; memcpy
declare void @llvm.memcpy.p0i8.p0i8.i32(i8* %dst, i8* %src, i32 %len, i1 %isvolatile)

define dso_local [4 x i16] @test_memcpy() #0 {
    %1 = alloca [4 x i16], align 4
    %2 = alloca [4 x i16], align 4
    store [4 x i16] [i16 u0xabcd, i16 u0x1234, i16 u0x5667, i16 u0xbebe], [4 x i16]* %1
    store [4 x i16] [i16 6, i16 7, i16 u0xfecb, i16 u0x6543], [4 x i16]* %2
    
    %src = bitcast [4 x i16]* %1 to i8*
    %dst = bitcast [4 x i16]* %2 to i8*
    ; copy 2,5 elements so 0xabcd, 0x1234, 0x67
    call void @llvm.memcpy.p0i8.p0i8.i32(i8* %dst, i8* %src, i32 5, i1 0)

    %ret = load [4 x i16], [4 x i16]* %2
    ret [4 x i16] %ret
    ; expect [0xcd, 0xab, 0x34, 0x12, 0x67, 0xfe, 0x43, 0x65]
    ;   -> 0x6543fe671234abcd
}

; memmove
declare void @llvm.memmove.p0i8.p0i8.i32(i8* %dst, i8* %src, i32 %len, i1 %isvolatile)

define dso_local [4 x i16] @test_memmove() #0 {
    %1 = alloca [4 x i16], align 4
    %2 = alloca [4 x i16], align 4

    ; [0xcd, 0xab, 0x34, 0x12, 0x67, 0x56, 0xbe, 0xbe]
    store [4 x i16] [i16 u0xabcd, i16 u0x1234, i16 u0x5667, i16 u0xbebe], [4 x i16]* %1

    ; [0x06, 0x00, 0x07, 0x00, 0xcb, 0xfe, 0x43, 0x65]
    store [4 x i16] [i16 6, i16 7, i16 u0xfecb, i16 u0x6543], [4 x i16]* %2
    
    %src = bitcast [4 x i16]* %1 to i8*
    %dst = bitcast [4 x i16]* %2 to i8*
    ; copy 2,5 elements so 0xabcd, 0x1234, 0x67
    call void @llvm.memmove.p0i8.p0i8.i32(i8* %dst, i8* %src, i32 5, i1 0)

    %ret = load [4 x i16], [4 x i16]* %2
    ret [4 x i16] %ret
    ; expect [0xcd, 0xab, 0x34, 0x12, 0x67, 0xfe, 0x43, 0x65]
    ;   -> 0x6543fe671234abcd
}

define dso_local [4 x i16] @test_memmove_overlapping() #0 {
    %1 = alloca [4 x i16], align 4
    store [4 x i16] [i16 u0xabcd, i16 u0x1234, i16 u0x5667, i16 u0xbebe], [4 x i16]* %1
    
    %src = bitcast [4 x i16]* %1 to i8*

    %dsti64 = ptrtoint [4 x i16]* %1 to i64
    %dstadd = add i64 %dsti64, 3
    %dst = inttoptr i64 %dstadd to i8*

    ; move over 5 i8s where the array is
    ; [0xcd, 0xab, 0x34, 0x12, 0x67, 0x56, 0xbe, 0xbe]
    ;  ^                 ^
    ;  |                 |
    ; src               dst
    ;
    ; yielding
    ; [0xcd, 0xab, 0x34, 0xcd, 0xab, 0x34, 0xcd, 0xab]
    call void @llvm.memmove.p0i8.p0i8.i32(i8* %dst, i8* %src, i32 5, i1 0)
    
    %ret = load [4 x i16], [4 x i16]* %1
    ret [4 x i16] %ret
    ; expect [0xcd, 0xab, 0x34, 0xcd, 0xab, 0x34, 0xcd, 0xab]
    ;   -> 0xabcd34abcd34abcd
}

define dso_local [4 x i16] @test_memmove_symbolic_len() #0 {
    %1 = alloca [4 x i16], align 4
    %2 = alloca [4 x i16], align 4

    ; [0xcd, 0xab, 0x34, 0x12, 0x67, 0x56, 0xbe, 0xbe]
    store [4 x i16] [i16 u0xabcd, i16 u0x1234, i16 u0x5667, i16 u0xbebe], [4 x i16]* %1

    ; [0x06, 0x00, 0x07, 0x00, 0xcb, 0xfe, 0x43, 0x65]
    store [4 x i16] [i16 6, i16 7, i16 u0xfecb, i16 u0x6543], [4 x i16]* %2

    ; setup symbolic len, but constrain to [3, 4].
    %len = call i32 @symbolic_range(i32 3, i32 4)
    
    %src = bitcast [4 x i16]* %1 to i8*
    %dst = bitcast [4 x i16]* %2 to i8*
    ; copy over 3 or 4 elements.
    call void @llvm.memmove.p0i8.p0i8.i32(i8* %dst, i8* %src, i32 %len, i1 0)

    ; overwrite last two 1i6 in dest with len.
    %el_addr = getelementptr inbounds [4 x i16], [4 x i16]* %2, i64 0, i32 2
    %el_addr_i32 = bitcast i16* %el_addr to i32*
    store i32 %len, i32* %el_addr_i32

    %ret = load [4 x i16], [4 x i16]* %2
    ret [4 x i16] %ret
    ; for len := 3
    ; expect [0xcd, 0xab, 0x34, 0x00, 0x03, 0x00, 0x00, 0x00]
    ;   -> 0x000000030034abcd
    ; for len := 4
    ; expect [0xcd, 0xab, 0x34, 0x12, 0x04, 0x00, 0x00, 0x00]
    ;   -> 0x000000041234abcd
}

; memset
declare void @llvm.memset.p0i8.i32(i8* %dest, i8 %val, i32 %len, i1 %isvolatile)

define dso_local [8 x i8] @test_memset() #0 {
    %1 = alloca [8 x i8], align 4
    %2 = bitcast [8 x i8]* %1 to i8*
    call void @llvm.memset.p0i8.i32(i8* %2, i8 u0xab, i32 8, i1 0)
    call void @llvm.memset.p0i8.i32(i8* %2, i8 u0xcb, i32 4, i1 0)
    %3 = load [8 x i8], [8 x i8]* %1
    ret [8 x i8] %3 ; expect 0xababababcbcbcbcb
}

; umax
declare i32 @llvm.umax.i32(i32, i32)
declare <2 x i32> @llvm.umax.v2i32(<2 x i32>, <2 x i32>)

define dso_local i32 @test_umax() #0 {
    %1 = add i32 0, u0xabcd
    %2 = add i32 0, u0xbcef
    %3 = call i32 @llvm.umax.i32(i32 %1, i32 %2)
    ret i32 %3 ; expect 0xbcef
}

define dso_local <2 x i32> @test_umax_vec() #0 {
    %1 = add <2 x i32> <i32 0, i32 0>, <i32 u0xabcd, i32 u0x4321>
    %2 = add <2 x i32> <i32 0, i32 0>, <i32 u0xbcef, i32 u0x1234>
    %3 = call <2 x i32> @llvm.umax.v2i32(<2 x i32> %1, <2 x i32> %2)
    ret <2 x i32> %3
    ; expect <0x0000bcef, 0x00004321>
    ;   -> 0x000043210000bcef
}


; --------------------------------------------------------------------------------------------------
; Arithmetic with overflow intrinsics
; --------------------------------------------------------------------------------------------------

declare {i8, i1} @llvm.sadd.with.overflow.i8(i8, i8)
declare {<4 x i8>, <4 x i1>} @llvm.sadd.with.overflow.v4i8(<4 x i8>, <4 x i8>)

declare {i8, i1} @llvm.uadd.with.overflow.i8(i8, i8)
declare {i8, i1} @llvm.ssub.with.overflow.i8(i8, i8)
declare {i8, i1} @llvm.usub.with.overflow.i8(i8, i8)
declare {i8, i1} @llvm.smul.with.overflow.i8(i8, i8)
declare {i8, i1} @llvm.umul.with.overflow.i8(i8, i8)


; sadd
define dso_local {i8, i1} @test_sadd_with_overflow0() #0 {
    %1 = call {i8, i1} @llvm.sadd.with.overflow.i8(i8 120, i8 10) ; 130 = 0x82
    ret {i8, i1} %1 ; expect {0x82, 0x01} -> 0x0182
}

define dso_local {i8, i1} @test_sadd_with_overflow1() #0 {
    %1 = call {i8, i1} @llvm.sadd.with.overflow.i8(i8 100, i8 10) ; 110 = 0x6e
    ret {i8, i1} %1 ; expect {0x6e, 0x00} -> 0x006e
}

define dso_local {<4 x i8>, <4 x i1>} @test_sadd_with_overflow_vec() #0 {
    ; <120, 100, -100 (0x9c), -50 (0xce)>, <10, 10, -30 (0xe2), -30 (0xe2)>
    %1 = call {<4 x i8>, <4 x i1>} @llvm.sadd.with.overflow.v4i8(
        <4 x i8> <i8 120, i8 100, i8 u0x9c, i8 u0xce>,
        <4 x i8> <i8 10, i8 10, i8 u0xe2, i8 u0xe2>
    )
    ; res <130, 110, -130, -80> = <0x82, 0x6e, 0x7e, 0xb0>
    ; overflow <1, 0, 1, 0>
    ret {<4 x i8>, <4 x i1>} %1
    ; expect {<0x82, 0x6e, 0x7e, 0xb0>, <1, 0, 1, 0>}
    ;   <0xb07e6e82>, <0b0101 = 0x5>
    ; expect: 0x5b07e6e82
}

; uadd
define dso_local {i8, i1} @test_uadd_with_overflow0() #0 {
    %1 = call {i8, i1} @llvm.uadd.with.overflow.i8(i8 200, i8 60) ; 260 = 0x104
    ret {i8, i1} %1 ; expect {0x04, 0x01} -> 0x0104
}

define dso_local {i8, i1} @test_uadd_with_overflow1() #0 {
    %1 = call {i8, i1} @llvm.uadd.with.overflow.i8(i8 200, i8 50) ; 250 = 0xfa
    ret {i8, i1} %1 ; expect {0xfa, 0x00} -> 0x00fa
}

; ssub

; usub

; smul

; umum

; --------------------------------------------------------------------------------------------------
; Saturation arithmetic intrinsics
; --------------------------------------------------------------------------------------------------

declare i4 @llvm.sadd.sat.i4(i4 %a, i4 %b)

declare i4 @llvm.uadd.sat.i4(i4 %a, i4 %b)
declare <3 x i4> @llvm.uadd.sat.v3i4(<3 x i4> %a, <3 x i4> %b)

; sadd.sat: From LLVM IR reference
define dso_local i64 @test_sadd_sat0() #0 {
    %1 = call i4 @llvm.sadd.sat.i4(i4 1, i4 2) ; %res = 3
    %2 = sext i4 %1 to i64
    ret i64 %2
}

define dso_local i64 @test_sadd_sat1() #0 {
    %1 = call i4 @llvm.sadd.sat.i4(i4 5, i4 6) ; %res = 7
    %2 = sext i4 %1 to i64
    ret i64 %2
}

define dso_local i64 @test_sadd_sat2() #0 {
    %1 = call i4 @llvm.sadd.sat.i4(i4 -4, i4 2) ; %res = -2
    %2 = sext i4 %1 to i64
    ret i64 %2
}

define dso_local i64 @test_sadd_sat3() #0 {
    %1 = call i4 @llvm.sadd.sat.i4(i4 -4, i4 -5) ; %res = -8
    %2 = sext i4 %1 to i64
    ret i64 %2
}

; uadd.sat: From LLVM IR reference
define dso_local i4 @test_uadd_sat0() #0 {
    %1 = call i4 @llvm.uadd.sat.i4(i4 1, i4 2) ; %res = 3
    ret i4 %1
}

define dso_local i4 @test_uadd_sat1() #0 {
    %1 = call i4 @llvm.uadd.sat.i4(i4 5, i4 6) ; %res = 11
    ret i4 %1
}

define dso_local i4 @test_uadd_sat2() #0 {
    %1 = call i4 @llvm.uadd.sat.i4(i4 8, i4 8) ; %res = 15
    ret i4 %1
}

define dso_local <3 x i4> @test_uadd_sat_vec() #0 {
    %1 = call <3 x i4> @llvm.uadd.sat.v3i4(<3 x i4> <i4 1, i4 5, i4 8>, <3 x i4> <i4 2, i4 6, i4 8>)
    ret <3 x i4> %1
    ; expect <3, 11, 15>
    ;   <0x3, 0xb, 0xf> -> 0xfb3
}

; --------------------------------------------------------------------------------------------------
; General intrinsics
; --------------------------------------------------------------------------------------------------

declare i32 @llvm.expect.i32(i32 %val, i32 %expected_val)
declare void @llvm.assume(i1 %cond)

define dso_local i32 @test_expect() #0 {
    %1 = add i32 0, 100
    %2 = add i32 0, 5
    %3 = call i32 @llvm.expect.i32(i32 %1, i32 %2)
    ret i32 %3 ; expect 100
}

define dso_local i32 @test_assume(i32 %0) #0 {
    %2 = icmp eq i32 %0, 5
    call void @llvm.assume(i1 %2)
    ret i32 %0 ; expect 5
}


declare void @assume(i32) #1

attributes #0 = { noinline nounwind optnone sspstrong uwtable "frame-pointer"="all" "min-legal-vector-width"="0" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #1 = { "frame-pointer"="all" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
