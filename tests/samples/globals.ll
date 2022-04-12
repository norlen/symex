; ModuleID = 'globals.c866c38a-cgu.0'
source_filename = "globals.c866c38a-cgu.0"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

%"core::panic::location::Location" = type { { [0 x i8]*, i64 }, i32, i32 }
%"unwind::libunwind::_Unwind_Exception" = type { i64, void (i32, %"unwind::libunwind::_Unwind_Exception"*)*, [6 x i64] }
%"unwind::libunwind::_Unwind_Context" = type { [0 x i8] }

@alloc13 = private unnamed_addr constant <{ [10 x i8] }> <{ [10 x i8] c"globals.rs" }>, align 1
@alloc14 = private unnamed_addr constant <{ i8*, [16 x i8] }> <{ i8* getelementptr inbounds (<{ [10 x i8] }>, <{ [10 x i8] }>* @alloc13, i32 0, i32 0, i32 0), [16 x i8] c"\0A\00\00\00\00\00\00\00\06\00\00\00\05\00\00\00" }>, align 8
@str.0 = internal constant [28 x i8] c"attempt to add with overflow"
@vtable.1 = private unnamed_addr constant <{ i8*, [16 x i8], i8*, i8*, i8*, [0 x i8] }> <{ i8* bitcast (void (i64**)* @"_ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17h54ec66b714267d49E" to i8*), [16 x i8] c"\08\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", i8* bitcast (i32 (i64**)* @"_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17hb71c85682ec75eb8E" to i8*), i8* bitcast (i32 (i64**)* @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h2176a968728b7e8bE" to i8*), i8* bitcast (i32 (i64**)* @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h2176a968728b7e8bE" to i8*), [0 x i8] zeroinitializer }>, align 8

; core::ops::function::FnOnce::call_once{{vtable.shim}}
; Function Attrs: inlinehint nounwind nonlazybind
define internal i32 @"_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17hb71c85682ec75eb8E"(i64** %_1) unnamed_addr #0 {
start:
  %_2 = alloca {}, align 1
  %0 = load i64*, i64** %_1, align 8, !nonnull !3
; call core::ops::function::FnOnce::call_once
  %1 = call i32 @_ZN4core3ops8function6FnOnce9call_once17hf539fe0a8387221aE(i64* nonnull %0) #7
  br label %bb1

bb1:                                              ; preds = %start
  ret i32 %1
}

; core::ops::function::FnOnce::call_once
; Function Attrs: inlinehint nounwind nonlazybind
define internal void @_ZN4core3ops8function6FnOnce9call_once17hbc00740d8f46fb1fE(void ()* nonnull %_1) unnamed_addr #0 {
start:
  %_2 = alloca {}, align 1
  call void %_1() #7
  br label %bb1

bb1:                                              ; preds = %start
  ret void
}

; core::ops::function::FnOnce::call_once
; Function Attrs: inlinehint nounwind nonlazybind
define internal i32 @_ZN4core3ops8function6FnOnce9call_once17hf539fe0a8387221aE(i64* nonnull %0) unnamed_addr #0 {
start:
  %_2 = alloca {}, align 1
  %_1 = alloca i64*, align 8
  store i64* %0, i64** %_1, align 8
; call std::rt::lang_start::{{closure}}
  %1 = call i32 @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h2176a968728b7e8bE"(i64** align 8 dereferenceable(8) %_1) #7
  br label %bb1

bb1:                                              ; preds = %start
  br label %bb2

bb2:                                              ; preds = %bb1
  ret i32 %1
}

; core::ptr::drop_in_place<std::rt::lang_start<()>::{{closure}}>
; Function Attrs: inlinehint nounwind nonlazybind
define internal void @"_ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17h54ec66b714267d49E"(i64** %_1) unnamed_addr #0 {
start:
  ret void
}

; globals::myfunc
; Function Attrs: nounwind nonlazybind
define internal i32 @_ZN7globals6myfunc17h02b93dd4697d27f2E(i32 %a) unnamed_addr #1 {
start:
  ret i32 %a
}

; globals::myfunc2
; Function Attrs: nounwind nonlazybind
define internal i32 @_ZN7globals7myfunc217h0030e0903c47361dE(i32 %a) unnamed_addr #1 {
start:
  %0 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %a, i32 1)
  %_3.0 = extractvalue { i32, i1 } %0, 0
  %_3.1 = extractvalue { i32, i1 } %0, 1
  %1 = call i1 @llvm.expect.i1(i1 %_3.1, i1 false)
  br i1 %1, label %panic, label %bb1

bb1:                                              ; preds = %start
  ret i32 %_3.0

panic:                                            ; preds = %start
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17h97167cd315d19cd4E([0 x i8]* nonnull align 1 bitcast ([28 x i8]* @str.0 to [0 x i8]*), i64 28, %"core::panic::location::Location"* align 8 dereferenceable(24) bitcast (<{ i8*, [16 x i8] }>* @alloc14 to %"core::panic::location::Location"*)) #8
  unreachable
}

; globals::bar
; Function Attrs: nounwind nonlazybind
define internal nonnull i32 (i32)* @_ZN7globals3bar17h9f9314d7ecf006f4E() unnamed_addr #1 {
start:
  ret i32 (i32)* @_ZN7globals6myfunc17h02b93dd4697d27f2E
}

; globals::bar2
; Function Attrs: nounwind nonlazybind
define internal nonnull i32 (i32)* @_ZN7globals4bar217h3456b6d88e065311E() unnamed_addr #1 {
start:
  ret i32 (i32)* @_ZN7globals7myfunc217h0030e0903c47361dE
}

; globals::foo
; Function Attrs: nounwind nonlazybind
define internal i32 @_ZN7globals3foo17hbaa00d4f81c2ed9dE(i1 zeroext %b) unnamed_addr #1 {
start:
  %f = alloca i32 (i32)*, align 8
  br i1 %b, label %bb1, label %bb2

bb2:                                              ; preds = %start
; call globals::bar2
  %0 = call nonnull i32 (i32)* @_ZN7globals4bar217h3456b6d88e065311E() #7
  store i32 (i32)* %0, i32 (i32)** %f, align 8
  br label %bb3

bb1:                                              ; preds = %start
; call globals::bar
  %1 = call nonnull i32 (i32)* @_ZN7globals3bar17h9f9314d7ecf006f4E() #7
  store i32 (i32)* %1, i32 (i32)** %f, align 8
  br label %bb3

bb3:                                              ; preds = %bb2, %bb1
  %_5 = load i32 (i32)*, i32 (i32)** %f, align 8, !nonnull !3
  %b1 = call i32 %_5(i32 10) #7
  br label %bb4

bb4:                                              ; preds = %bb3
  ret i32 %b1
}

; globals::main
; Function Attrs: nounwind nonlazybind
define internal void @_ZN7globals4main17h916173cf78991abeE() unnamed_addr #1 {
start:
; call globals::foo
  %_1 = call i32 @_ZN7globals3foo17hbaa00d4f81c2ed9dE(i1 zeroext true) #7
  br label %bb1

bb1:                                              ; preds = %start
  ret void
}

; core::hint::black_box
; Function Attrs: inlinehint nounwind nonlazybind
define internal void @_ZN4core4hint9black_box17hb21f20c0f3b307f1E() unnamed_addr #0 {
start:
  call void asm sideeffect "", "r,~{memory}"({}* undef), !srcloc !4
  br label %bb1

bb1:                                              ; preds = %start
  ret void
}

; <() as std::process::Termination>::report
; Function Attrs: inlinehint nounwind nonlazybind
define internal i32 @"_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17h6e6c6b955923631bE"() unnamed_addr #0 {
start:
; call <std::process::ExitCode as std::process::Termination>::report
  %0 = call i32 @"_ZN68_$LT$std..process..ExitCode$u20$as$u20$std..process..Termination$GT$6report17h6d1ffd0de92e3abbE"(i8 0) #7
  br label %bb1

bb1:                                              ; preds = %start
  ret i32 %0
}

; <std::process::ExitCode as std::process::Termination>::report
; Function Attrs: inlinehint nounwind nonlazybind
define internal i32 @"_ZN68_$LT$std..process..ExitCode$u20$as$u20$std..process..Termination$GT$6report17h6d1ffd0de92e3abbE"(i8 %0) unnamed_addr #0 {
start:
  %self = alloca i8, align 1
  store i8 %0, i8* %self, align 1
; call std::sys::unix::process::process_common::ExitCode::as_i32
  %1 = call i32 @_ZN3std3sys4unix7process14process_common8ExitCode6as_i3217h186e97d4780d4443E(i8* align 1 dereferenceable(1) %self) #7
  br label %bb1

bb1:                                              ; preds = %start
  ret i32 %1
}

; std::rt::lang_start
; Function Attrs: nounwind nonlazybind
define hidden i64 @_ZN3std2rt10lang_start17he102999bdddcd942E(void ()* nonnull %main, i64 %argc, i8** %argv) unnamed_addr #1 {
start:
  %_8 = alloca i64*, align 8
  %_4 = alloca i64, align 8
  %0 = bitcast i64** %_8 to void ()**
  store void ()* %main, void ()** %0, align 8
  %_5.0 = bitcast i64** %_8 to {}*
; call std::rt::lang_start_internal
  %1 = call i64 @_ZN3std2rt19lang_start_internal17h52e73755f77c7dd9E({}* nonnull align 1 %_5.0, [3 x i64]* align 8 dereferenceable(24) bitcast (<{ i8*, [16 x i8], i8*, i8*, i8*, [0 x i8] }>* @vtable.1 to [3 x i64]*), i64 %argc, i8** %argv) #7
  store i64 %1, i64* %_4, align 8
  br label %bb1

bb1:                                              ; preds = %start
  %v = load i64, i64* %_4, align 8
  ret i64 %v
}

; std::rt::lang_start::{{closure}}
; Function Attrs: inlinehint nounwind nonlazybind
define internal i32 @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h2176a968728b7e8bE"(i64** align 8 dereferenceable(8) %_1) unnamed_addr #0 {
start:
  %0 = bitcast i64** %_1 to void ()**
  %_3 = load void ()*, void ()** %0, align 8, !nonnull !3
; call std::sys_common::backtrace::__rust_begin_short_backtrace
  call void @_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h7d3503ec0de75605E(void ()* nonnull %_3) #7
  br label %bb1

bb1:                                              ; preds = %start
; call <() as std::process::Termination>::report
  %1 = call i32 @"_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17h6e6c6b955923631bE"() #7
  br label %bb2

bb2:                                              ; preds = %bb1
  ret i32 %1
}

; std::sys_common::backtrace::__rust_begin_short_backtrace
; Function Attrs: noinline nounwind nonlazybind
define internal void @_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h7d3503ec0de75605E(void ()* nonnull %f) unnamed_addr #2 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
start:
  %0 = alloca { i8*, i32 }, align 8
; call core::ops::function::FnOnce::call_once
  call void @_ZN4core3ops8function6FnOnce9call_once17hbc00740d8f46fb1fE(void ()* nonnull %f) #7
  br label %bb1

bb1:                                              ; preds = %start
; call core::hint::black_box
  call void @_ZN4core4hint9black_box17hb21f20c0f3b307f1E() #7
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

; std::sys::unix::process::process_common::ExitCode::as_i32
; Function Attrs: inlinehint nounwind nonlazybind
define internal i32 @_ZN3std3sys4unix7process14process_common8ExitCode6as_i3217h186e97d4780d4443E(i8* align 1 dereferenceable(1) %self) unnamed_addr #0 {
start:
  %_2 = load i8, i8* %self, align 1
  %0 = zext i8 %_2 to i32
  ret i32 %0
}

; Function Attrs: nofree nosync nounwind readnone speculatable willreturn
declare { i32, i1 } @llvm.sadd.with.overflow.i32(i32, i32) #3

; Function Attrs: nofree nosync nounwind readnone willreturn
declare i1 @llvm.expect.i1(i1, i1) #4

; core::panicking::panic
; Function Attrs: cold noinline noreturn nounwind nonlazybind
declare void @_ZN4core9panicking5panic17h97167cd315d19cd4E([0 x i8]* nonnull align 1, i64, %"core::panic::location::Location"* align 8 dereferenceable(24)) unnamed_addr #5

; std::rt::lang_start_internal
; Function Attrs: nounwind nonlazybind
declare i64 @_ZN3std2rt19lang_start_internal17h52e73755f77c7dd9E({}* nonnull align 1, [3 x i64]* align 8 dereferenceable(24), i64, i8**) unnamed_addr #1

; Function Attrs: nounwind nonlazybind
declare i32 @rust_eh_personality(i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*) unnamed_addr #1

; Function Attrs: nonlazybind
define i32 @main(i32 %0, i8** %1) unnamed_addr #6 {
top:
  %2 = sext i32 %0 to i64
; call std::rt::lang_start
  %3 = call i64 @_ZN3std2rt10lang_start17he102999bdddcd942E(void ()* @_ZN7globals4main17h916173cf78991abeE, i64 %2, i8** %1)
  %4 = trunc i64 %3 to i32
  ret i32 %4
}

attributes #0 = { inlinehint nounwind nonlazybind "probe-stack"="__rust_probestack" "target-cpu"="x86-64" }
attributes #1 = { nounwind nonlazybind "probe-stack"="__rust_probestack" "target-cpu"="x86-64" }
attributes #2 = { noinline nounwind nonlazybind "probe-stack"="__rust_probestack" "target-cpu"="x86-64" }
attributes #3 = { nofree nosync nounwind readnone speculatable willreturn }
attributes #4 = { nofree nosync nounwind readnone willreturn }
attributes #5 = { cold noinline noreturn nounwind nonlazybind "probe-stack"="__rust_probestack" "target-cpu"="x86-64" }
attributes #6 = { nonlazybind "target-cpu"="x86-64" }
attributes #7 = { nounwind }
attributes #8 = { noreturn nounwind }

!llvm.module.flags = !{!0, !1, !2}

!0 = !{i32 7, !"PIC Level", i32 2}
!1 = !{i32 7, !"PIE Level", i32 2}
!2 = !{i32 2, !"RtLibUseGOT", i32 1}
!3 = !{}
!4 = !{i32 3213155}
