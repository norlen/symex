; ModuleID = 'out_of_bounds.036f039f-cgu.0'
source_filename = "out_of_bounds.036f039f-cgu.0"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

%"core::panic::location::Location" = type { { [0 x i8]*, i64 }, i32, i32 }
%"unwind::libunwind::_Unwind_Exception" = type { i64, void (i32, %"unwind::libunwind::_Unwind_Exception"*)*, [6 x i64] }
%"unwind::libunwind::_Unwind_Context" = type { [0 x i8] }

@alloc9 = private unnamed_addr constant <{ [16 x i8] }> <{ [16 x i8] c"out_of_bounds.rs" }>, align 1
@alloc10 = private unnamed_addr constant <{ i8*, [16 x i8] }> <{ i8* getelementptr inbounds (<{ [16 x i8] }>, <{ [16 x i8] }>* @alloc9, i32 0, i32 0, i32 0), [16 x i8] c"\10\00\00\00\00\00\00\00\03\00\00\00\05\00\00\00" }>, align 8
@vtable.0 = private unnamed_addr constant <{ i8*, [16 x i8], i8*, i8*, i8*, [0 x i8] }> <{ i8* bitcast (void (i64**)* @"_ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17h41cc5abd21ac083cE" to i8*), [16 x i8] c"\08\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", i8* bitcast (i32 (i64**)* @"_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h8a1e49d7c9b1cd15E" to i8*), i8* bitcast (i32 (i64**)* @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hfb1fbe0a67cec066E" to i8*), i8* bitcast (i32 (i64**)* @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hfb1fbe0a67cec066E" to i8*), [0 x i8] zeroinitializer }>, align 8

; core::ops::function::FnOnce::call_once{{vtable.shim}}
; Function Attrs: inlinehint nounwind nonlazybind
define internal i32 @"_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h8a1e49d7c9b1cd15E"(i64** %_1) unnamed_addr #0 {
start:
  %_2 = alloca {}, align 1
  %0 = load i64*, i64** %_1, align 8, !nonnull !3
; call core::ops::function::FnOnce::call_once
  %1 = call i32 @_ZN4core3ops8function6FnOnce9call_once17h650a214f4371813aE(i64* nonnull %0) #6
  br label %bb1

bb1:                                              ; preds = %start
  ret i32 %1
}

; core::ops::function::FnOnce::call_once
; Function Attrs: inlinehint nounwind nonlazybind
define internal i32 @_ZN4core3ops8function6FnOnce9call_once17h650a214f4371813aE(i64* nonnull %0) unnamed_addr #0 {
start:
  %_2 = alloca {}, align 1
  %_1 = alloca i64*, align 8
  store i64* %0, i64** %_1, align 8
; call std::rt::lang_start::{{closure}}
  %1 = call i32 @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hfb1fbe0a67cec066E"(i64** align 8 dereferenceable(8) %_1) #6
  br label %bb1

bb1:                                              ; preds = %start
  br label %bb2

bb2:                                              ; preds = %bb1
  ret i32 %1
}

; core::ops::function::FnOnce::call_once
; Function Attrs: inlinehint nounwind nonlazybind
define internal void @_ZN4core3ops8function6FnOnce9call_once17hf239a27de7d4fe9aE(void ()* nonnull %_1) unnamed_addr #0 {
start:
  %_2 = alloca {}, align 1
  call void %_1() #6
  br label %bb1

bb1:                                              ; preds = %start
  ret void
}

; core::ptr::drop_in_place<std::rt::lang_start<()>::{{closure}}>
; Function Attrs: inlinehint nounwind nonlazybind
define internal void @"_ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17h41cc5abd21ac083cE"(i64** %_1) unnamed_addr #0 {
start:
  ret void
}

; out_of_bounds::out_of_bounds
; Function Attrs: nounwind nonlazybind
define internal i32 @_ZN13out_of_bounds13out_of_bounds17h932a58394a26fb50E(i64 %idx) unnamed_addr #1 {
start:
  %arr = alloca [4 x i32], align 4
  %0 = bitcast [4 x i32]* %arr to i32*
  store i32 0, i32* %0, align 4
  %1 = getelementptr inbounds [4 x i32], [4 x i32]* %arr, i32 0, i32 1
  store i32 1, i32* %1, align 4
  %2 = getelementptr inbounds [4 x i32], [4 x i32]* %arr, i32 0, i32 2
  store i32 2, i32* %2, align 4
  %3 = getelementptr inbounds [4 x i32], [4 x i32]* %arr, i32 0, i32 3
  store i32 3, i32* %3, align 4
  %_5 = icmp ult i64 %idx, 4
  %4 = call i1 @llvm.expect.i1(i1 %_5, i1 true)
  br i1 %4, label %bb1, label %panic

bb1:                                              ; preds = %start
  %5 = getelementptr inbounds [4 x i32], [4 x i32]* %arr, i64 0, i64 %idx
  %6 = load i32, i32* %5, align 4
  ret i32 %6

panic:                                            ; preds = %start
; call core::panicking::panic_bounds_check
  call void @_ZN4core9panicking18panic_bounds_check17h449d4ff4d992b84fE(i64 %idx, i64 4, %"core::panic::location::Location"* align 8 dereferenceable(24) bitcast (<{ i8*, [16 x i8] }>* @alloc10 to %"core::panic::location::Location"*)) #7
  unreachable
}

; out_of_bounds::out_of_bounds_unchecked
; Function Attrs: nounwind nonlazybind
define internal i32 @_ZN13out_of_bounds23out_of_bounds_unchecked17hd69d9274d5937f7aE(i64 %idx) unnamed_addr #1 {
start:
  %arr = alloca [4 x i32], align 4
  %0 = bitcast [4 x i32]* %arr to i32*
  store i32 0, i32* %0, align 4
  %1 = getelementptr inbounds [4 x i32], [4 x i32]* %arr, i32 0, i32 1
  store i32 1, i32* %1, align 4
  %2 = getelementptr inbounds [4 x i32], [4 x i32]* %arr, i32 0, i32 2
  store i32 2, i32* %2, align 4
  %3 = getelementptr inbounds [4 x i32], [4 x i32]* %arr, i32 0, i32 3
  store i32 3, i32* %3, align 4
  %_4.0 = bitcast [4 x i32]* %arr to [0 x i32]*
; call core::slice::<impl [T]>::get_unchecked
  %_3 = call align 4 dereferenceable(4) i32* @"_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$13get_unchecked17h78fadeb51608f89dE"([0 x i32]* nonnull align 4 %_4.0, i64 4, i64 %idx) #6
  br label %bb1

bb1:                                              ; preds = %start
  %4 = load i32, i32* %_3, align 4
  ret i32 %4
}

; out_of_bounds::main
; Function Attrs: nounwind nonlazybind
define internal void @_ZN13out_of_bounds4main17he53fd038d6422467E() unnamed_addr #1 {
start:
; call out_of_bounds::out_of_bounds
  %_1 = call i32 @_ZN13out_of_bounds13out_of_bounds17h932a58394a26fb50E(i64 1) #6
  br label %bb1

bb1:                                              ; preds = %start
; call out_of_bounds::out_of_bounds_unchecked
  %_2 = call i32 @_ZN13out_of_bounds23out_of_bounds_unchecked17hd69d9274d5937f7aE(i64 1) #6
  br label %bb2

bb2:                                              ; preds = %bb1
  ret void
}

; core::hint::black_box
; Function Attrs: inlinehint nounwind nonlazybind
define internal void @_ZN4core4hint9black_box17h87b7a15885f62dc0E() unnamed_addr #0 {
start:
  call void asm sideeffect "", "r,~{memory}"({}* undef), !srcloc !4
  br label %bb1

bb1:                                              ; preds = %start
  ret void
}

; core::ptr::const_ptr::<impl *const [T]>::as_ptr
; Function Attrs: inlinehint nounwind nonlazybind
define internal i32* @"_ZN4core3ptr9const_ptr43_$LT$impl$u20$$BP$const$u20$$u5b$T$u5d$$GT$6as_ptr17hc46956f4a2739396E"([0 x i32]* %self.0, i64 %self.1) unnamed_addr #0 {
start:
  %0 = bitcast [0 x i32]* %self.0 to i32*
  ret i32* %0
}

; <() as std::process::Termination>::report
; Function Attrs: inlinehint nounwind nonlazybind
define internal i32 @"_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17hbebfb9460c5dbe72E"() unnamed_addr #0 {
start:
; call <std::process::ExitCode as std::process::Termination>::report
  %0 = call i32 @"_ZN68_$LT$std..process..ExitCode$u20$as$u20$std..process..Termination$GT$6report17hc86188271a909184E"(i8 0) #6
  br label %bb1

bb1:                                              ; preds = %start
  ret i32 %0
}

; <std::process::ExitCode as std::process::Termination>::report
; Function Attrs: inlinehint nounwind nonlazybind
define internal i32 @"_ZN68_$LT$std..process..ExitCode$u20$as$u20$std..process..Termination$GT$6report17hc86188271a909184E"(i8 %0) unnamed_addr #0 {
start:
  %self = alloca i8, align 1
  store i8 %0, i8* %self, align 1
; call std::sys::unix::process::process_common::ExitCode::as_i32
  %1 = call i32 @_ZN3std3sys4unix7process14process_common8ExitCode6as_i3217hd98b2f3717c5b5feE(i8* align 1 dereferenceable(1) %self) #6
  br label %bb1

bb1:                                              ; preds = %start
  ret i32 %1
}

; std::rt::lang_start
; Function Attrs: nounwind nonlazybind
define hidden i64 @_ZN3std2rt10lang_start17h9606622d27fd11ebE(void ()* nonnull %main, i64 %argc, i8** %argv) unnamed_addr #1 {
start:
  %_8 = alloca i64*, align 8
  %_4 = alloca i64, align 8
  %0 = bitcast i64** %_8 to void ()**
  store void ()* %main, void ()** %0, align 8
  %_5.0 = bitcast i64** %_8 to {}*
; call std::rt::lang_start_internal
  %1 = call i64 @_ZN3std2rt19lang_start_internal17h52e73755f77c7dd9E({}* nonnull align 1 %_5.0, [3 x i64]* align 8 dereferenceable(24) bitcast (<{ i8*, [16 x i8], i8*, i8*, i8*, [0 x i8] }>* @vtable.0 to [3 x i64]*), i64 %argc, i8** %argv) #6
  store i64 %1, i64* %_4, align 8
  br label %bb1

bb1:                                              ; preds = %start
  %v = load i64, i64* %_4, align 8
  ret i64 %v
}

; std::rt::lang_start::{{closure}}
; Function Attrs: inlinehint nounwind nonlazybind
define internal i32 @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hfb1fbe0a67cec066E"(i64** align 8 dereferenceable(8) %_1) unnamed_addr #0 {
start:
  %0 = bitcast i64** %_1 to void ()**
  %_3 = load void ()*, void ()** %0, align 8, !nonnull !3
; call std::sys_common::backtrace::__rust_begin_short_backtrace
  call void @_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h670ad1d15e13adb8E(void ()* nonnull %_3) #6
  br label %bb1

bb1:                                              ; preds = %start
; call <() as std::process::Termination>::report
  %1 = call i32 @"_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17hbebfb9460c5dbe72E"() #6
  br label %bb2

bb2:                                              ; preds = %bb1
  ret i32 %1
}

; std::sys_common::backtrace::__rust_begin_short_backtrace
; Function Attrs: noinline nounwind nonlazybind
define internal void @_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h670ad1d15e13adb8E(void ()* nonnull %f) unnamed_addr #2 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
start:
  %0 = alloca { i8*, i32 }, align 8
; call core::ops::function::FnOnce::call_once
  call void @_ZN4core3ops8function6FnOnce9call_once17hf239a27de7d4fe9aE(void ()* nonnull %f) #6
  br label %bb1

bb1:                                              ; preds = %start
; call core::hint::black_box
  call void @_ZN4core4hint9black_box17h87b7a15885f62dc0E() #6
  br label %bb2

bb2:                                              ; preds = %bb1
  ret void

bb3:                                              ; No predecessors!
  br label %bb4

bb4:                                              ; preds = %bb3
  %1 = bitcast { i8*, i32 }* %0 to i8**
  %2 = load i8*, i8** %1, align 8
  %3 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %0, i32 0, i32 1
  %4 = load i32, i32* %3, align 8
  %5 = insertvalue { i8*, i32 } undef, i8* %2, 0
  %6 = insertvalue { i8*, i32 } %5, i32 %4, 1
  resume { i8*, i32 } %6
}

; <usize as core::slice::index::SliceIndex<[T]>>::get_unchecked
; Function Attrs: inlinehint nounwind nonlazybind
define internal i32* @"_ZN75_$LT$usize$u20$as$u20$core..slice..index..SliceIndex$LT$$u5b$T$u5d$$GT$$GT$13get_unchecked17h86a567e8057a3db3E"(i64 %self, [0 x i32]* %slice.0, i64 %slice.1) unnamed_addr #0 {
start:
  %0 = alloca i32*, align 8
; call core::ptr::const_ptr::<impl *const [T]>::as_ptr
  %_3 = call i32* @"_ZN4core3ptr9const_ptr43_$LT$impl$u20$$BP$const$u20$$u5b$T$u5d$$GT$6as_ptr17hc46956f4a2739396E"([0 x i32]* %slice.0, i64 %slice.1) #6
  br label %bb1

bb1:                                              ; preds = %start
  %1 = getelementptr inbounds i32, i32* %_3, i64 %self
  store i32* %1, i32** %0, align 8
  %2 = load i32*, i32** %0, align 8
  br label %bb2

bb2:                                              ; preds = %bb1
  ret i32* %2
}

; core::slice::<impl [T]>::get_unchecked
; Function Attrs: inlinehint nounwind nonlazybind
define internal align 4 dereferenceable(4) i32* @"_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$13get_unchecked17h78fadeb51608f89dE"([0 x i32]* nonnull align 4 %self.0, i64 %self.1, i64 %index) unnamed_addr #0 {
start:
; call <usize as core::slice::index::SliceIndex<[T]>>::get_unchecked
  %_3 = call i32* @"_ZN75_$LT$usize$u20$as$u20$core..slice..index..SliceIndex$LT$$u5b$T$u5d$$GT$$GT$13get_unchecked17h86a567e8057a3db3E"(i64 %index, [0 x i32]* %self.0, i64 %self.1) #6
  br label %bb1

bb1:                                              ; preds = %start
  ret i32* %_3
}

; std::sys::unix::process::process_common::ExitCode::as_i32
; Function Attrs: inlinehint nounwind nonlazybind
define internal i32 @_ZN3std3sys4unix7process14process_common8ExitCode6as_i3217hd98b2f3717c5b5feE(i8* align 1 dereferenceable(1) %self) unnamed_addr #0 {
start:
  %_2 = load i8, i8* %self, align 1
  %0 = zext i8 %_2 to i32
  ret i32 %0
}

; Function Attrs: nofree nosync nounwind readnone willreturn
declare i1 @llvm.expect.i1(i1, i1) #3

; core::panicking::panic_bounds_check
; Function Attrs: cold noinline noreturn nounwind nonlazybind
declare void @_ZN4core9panicking18panic_bounds_check17h449d4ff4d992b84fE(i64, i64, %"core::panic::location::Location"* align 8 dereferenceable(24)) unnamed_addr #4

; std::rt::lang_start_internal
; Function Attrs: nounwind nonlazybind
declare i64 @_ZN3std2rt19lang_start_internal17h52e73755f77c7dd9E({}* nonnull align 1, [3 x i64]* align 8 dereferenceable(24), i64, i8**) unnamed_addr #1

; Function Attrs: nounwind nonlazybind
declare i32 @rust_eh_personality(i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*) unnamed_addr #1

; Function Attrs: nonlazybind
define i32 @main(i32 %0, i8** %1) unnamed_addr #5 {
top:
  %2 = sext i32 %0 to i64
; call std::rt::lang_start
  %3 = call i64 @_ZN3std2rt10lang_start17h9606622d27fd11ebE(void ()* @_ZN13out_of_bounds4main17he53fd038d6422467E, i64 %2, i8** %1)
  %4 = trunc i64 %3 to i32
  ret i32 %4
}

attributes #0 = { inlinehint nounwind nonlazybind "probe-stack"="__rust_probestack" "target-cpu"="x86-64" }
attributes #1 = { nounwind nonlazybind "probe-stack"="__rust_probestack" "target-cpu"="x86-64" }
attributes #2 = { noinline nounwind nonlazybind "probe-stack"="__rust_probestack" "target-cpu"="x86-64" }
attributes #3 = { nofree nosync nounwind readnone willreturn }
attributes #4 = { cold noinline noreturn nounwind nonlazybind "probe-stack"="__rust_probestack" "target-cpu"="x86-64" }
attributes #5 = { nonlazybind "target-cpu"="x86-64" }
attributes #6 = { nounwind }
attributes #7 = { noreturn nounwind }

!llvm.module.flags = !{!0, !1, !2}

!0 = !{i32 7, !"PIC Level", i32 2}
!1 = !{i32 7, !"PIE Level", i32 2}
!2 = !{i32 2, !"RtLibUseGOT", i32 1}
!3 = !{}
!4 = !{i32 3213117}
