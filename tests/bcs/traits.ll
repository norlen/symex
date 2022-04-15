; ModuleID = 'traits.1a858608-cgu.0'
source_filename = "traits.1a858608-cgu.0"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

%"core::panic::location::Location" = type { { [0 x i8]*, i64 }, i32, i32 }
%"unwind::libunwind::_Unwind_Exception" = type { i64, void (i32, %"unwind::libunwind::_Unwind_Exception"*)*, [6 x i64] }
%"unwind::libunwind::_Unwind_Context" = type { [0 x i8] }

@alloc9 = private unnamed_addr constant <{ [9 x i8] }> <{ [9 x i8] c"traits.rs" }>, align 1
@alloc10 = private unnamed_addr constant <{ i8*, [16 x i8] }> <{ i8* getelementptr inbounds (<{ [9 x i8] }>, <{ [9 x i8] }>* @alloc9, i32 0, i32 0, i32 0), [16 x i8] c"\09\00\00\00\00\00\00\00\0B\00\00\00\09\00\00\00" }>, align 8
@str.0 = internal constant [28 x i8] c"attempt to add with overflow"
@vtable.1 = private unnamed_addr constant <{ i8*, [16 x i8], i8*, [0 x i8] }>
  <{
    i8* bitcast (void (i32*)* @"_ZN4core3ptr32drop_in_place$LT$traits..Bar$GT$17h5dbe101316bb053cE" to i8*),
    [16 x i8] c"\04\00\00\00\00\00\00\00\04\00\00\00\00\00\00\00",
    i8* bitcast (i32 (i32*)* @"_ZN43_$LT$traits..Bar$u20$as$u20$traits..Foo$GT$10get_random17h6d5dd3cd4d7c39b7E" to i8*),
    [0 x i8] zeroinitializer 
  }>, align 8


@vtable.2 = private unnamed_addr constant <{ i8*, [16 x i8], i8*, i8*, i8*, [0 x i8] }> <{ i8* bitcast (void (i64**)* @"_ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17ha0fabb4ba73a18cbE" to i8*), [16 x i8] c"\08\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", i8* bitcast (i32 (i64**)* @"_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17hd7815f3c5fd9e217E" to i8*), i8* bitcast (i32 (i64**)* @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h2d1dd85a0d069c44E" to i8*), i8* bitcast (i32 (i64**)* @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h2d1dd85a0d069c44E" to i8*), [0 x i8] zeroinitializer }>, align 8

; core::ops::function::FnOnce::call_once{{vtable.shim}}
; Function Attrs: inlinehint nounwind nonlazybind
define internal i32 @"_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17hd7815f3c5fd9e217E"(i64** %_1) unnamed_addr #0 {
start:
  %_2 = alloca {}, align 1
  %0 = load i64*, i64** %_1, align 8, !nonnull !3
; call core::ops::function::FnOnce::call_once
  %1 = call i32 @_ZN4core3ops8function6FnOnce9call_once17he0e1d855d37fc0d7E(i64* nonnull %0) #7
  br label %bb1

bb1:                                              ; preds = %start
  ret i32 %1
}

; core::ops::function::FnOnce::call_once
; Function Attrs: inlinehint nounwind nonlazybind
define internal void @_ZN4core3ops8function6FnOnce9call_once17h43b4487fc5fd61c2E(void ()* nonnull %_1) unnamed_addr #0 {
start:
  %_2 = alloca {}, align 1
  call void %_1() #7
  br label %bb1

bb1:                                              ; preds = %start
  ret void
}

; core::ops::function::FnOnce::call_once
; Function Attrs: inlinehint nounwind nonlazybind
define internal i32 @_ZN4core3ops8function6FnOnce9call_once17he0e1d855d37fc0d7E(i64* nonnull %0) unnamed_addr #0 {
start:
  %_2 = alloca {}, align 1
  %_1 = alloca i64*, align 8
  store i64* %0, i64** %_1, align 8
; call std::rt::lang_start::{{closure}}
  %1 = call i32 @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h2d1dd85a0d069c44E"(i64** align 8 dereferenceable(8) %_1) #7
  br label %bb1

bb1:                                              ; preds = %start
  br label %bb2

bb2:                                              ; preds = %bb1
  ret i32 %1
}

; core::ptr::drop_in_place<traits::Bar>
; Function Attrs: inlinehint nounwind nonlazybind
define internal void @"_ZN4core3ptr32drop_in_place$LT$traits..Bar$GT$17h5dbe101316bb053cE"(i32* %_1) unnamed_addr #0 {
start:
  ret void
}

; core::ptr::drop_in_place<std::rt::lang_start<()>::{{closure}}>
; Function Attrs: inlinehint nounwind nonlazybind
define internal void @"_ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17ha0fabb4ba73a18cbE"(i64** %_1) unnamed_addr #0 {
start:
  ret void
}

; <traits::Bar as traits::Foo>::get_random
; Function Attrs: nounwind nonlazybind
define internal i32 @"_ZN43_$LT$traits..Bar$u20$as$u20$traits..Foo$GT$10get_random17h6d5dd3cd4d7c39b7E"(i32* align 4 dereferenceable(4) %self) unnamed_addr #1 {
start:
  %0 = load i32, i32* %self, align 4
  %1 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %0, i32 5)
  %_2.0 = extractvalue { i32, i1 } %1, 0
  %_2.1 = extractvalue { i32, i1 } %1, 1
  %2 = call i1 @llvm.expect.i1(i1 %_2.1, i1 false)
  br i1 %2, label %panic, label %bb1

bb1:                                              ; preds = %start
  store i32 %_2.0, i32* %self, align 4
  %3 = load i32, i32* %self, align 4
  ret i32 %3

panic:                                            ; preds = %start
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17h97167cd315d19cd4E([0 x i8]* nonnull align 1 bitcast ([28 x i8]* @str.0 to [0 x i8]*), i64 28, %"core::panic::location::Location"* align 8 dereferenceable(24) bitcast (<{ i8*, [16 x i8] }>* @alloc10 to %"core::panic::location::Location"*)) #8
  unreachable
}

; traits::rand
; Function Attrs: nounwind nonlazybind
define internal i32 @_ZN6traits4rand17h81a48c484cbd35ceE(
  {}* nonnull align 1 %f.0,
  [3 x i64]* align 8 dereferenceable(24) %f.1
) unnamed_addr #1 {
start:
  ; cast @vtable.1
  %0 = bitcast [3 x i64]* %f.1 to i32 ({}*)**
  ; get addr to third element of @vtable1. which should be
  ; i8* bitcast (i32 (i32*)* @"_ZN43_$LT$traits..Bar$u20$as$u20$traits..Foo$GT$10get_random17h6d5dd3cd4d7c39b7E" to i8*),
  %1 = getelementptr inbounds i32 ({}*)*, i32 ({}*)** %0, i64 3
  ; load the address to the function
  %2 = load i32 ({}*)*, i32 ({}*)** %1, align 8, !invariant.load !3, !nonnull !3
  ; call the function address.
  %3 = call i32 %2({}* align 1 %f.0) #7
  br label %bb1

bb1:                                              ; preds = %start
  ret i32 %3
}

; traits::foo
; Function Attrs: nounwind nonlazybind
define internal i32 @_ZN6traits3foo17hca7758adafd4e4e6E() unnamed_addr #1 {
start:
  %b = alloca i32, align 4
  store i32 5, i32* %b, align 4
  %_2.0 = bitcast i32* %b to {}*
; call traits::rand
  %0 = call i32 @_ZN6traits4rand17h81a48c484cbd35ceE(
    {}* nonnull align 1 %_2.0,
    [3 x i64]* align 8 dereferenceable(24) bitcast (
      <{ i8*, [16 x i8],i8*, [0 x i8] }>* @vtable.1 to [3 x i64]*
    )
  ) #7
  br label %bb1

bb1:                                              ; preds = %start
  ret i32 %0
}

; traits::main
; Function Attrs: nounwind nonlazybind
define internal void @_ZN6traits4main17h60c74002998e82a5E() unnamed_addr #1 {
start:
  %b = alloca i32, align 4
  store i32 5, i32* %b, align 4
  %_3.0 = bitcast i32* %b to {}*
; call traits::rand
  %_2 = call i32 @_ZN6traits4rand17h81a48c484cbd35ceE({}* nonnull align 1 %_3.0, [3 x i64]* align 8 dereferenceable(24) bitcast (<{ i8*, [16 x i8], i8*, [0 x i8] }>* @vtable.1 to [3 x i64]*)) #7
  br label %bb1

bb1:                                              ; preds = %start
; call traits::foo
  %z = call i32 @_ZN6traits3foo17hca7758adafd4e4e6E() #7
  br label %bb2

bb2:                                              ; preds = %bb1
  ret void
}

; core::hint::black_box
; Function Attrs: inlinehint nounwind nonlazybind
define internal void @_ZN4core4hint9black_box17h04805f53f41105a9E() unnamed_addr #0 {
start:
  call void asm sideeffect "", "r,~{memory}"({}* undef), !srcloc !4
  br label %bb1

bb1:                                              ; preds = %start
  ret void
}

; <() as std::process::Termination>::report
; Function Attrs: inlinehint nounwind nonlazybind
define internal i32 @"_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17h2a758e14e51677e6E"() unnamed_addr #0 {
start:
; call <std::process::ExitCode as std::process::Termination>::report
  %0 = call i32 @"_ZN68_$LT$std..process..ExitCode$u20$as$u20$std..process..Termination$GT$6report17hde663a5eb7aae5f6E"(i8 0) #7
  br label %bb1

bb1:                                              ; preds = %start
  ret i32 %0
}

; <std::process::ExitCode as std::process::Termination>::report
; Function Attrs: inlinehint nounwind nonlazybind
define internal i32 @"_ZN68_$LT$std..process..ExitCode$u20$as$u20$std..process..Termination$GT$6report17hde663a5eb7aae5f6E"(i8 %0) unnamed_addr #0 {
start:
  %self = alloca i8, align 1
  store i8 %0, i8* %self, align 1
; call std::sys::unix::process::process_common::ExitCode::as_i32
  %1 = call i32 @_ZN3std3sys4unix7process14process_common8ExitCode6as_i3217hfe3ce6c8485deab0E(i8* align 1 dereferenceable(1) %self) #7
  br label %bb1

bb1:                                              ; preds = %start
  ret i32 %1
}

; std::rt::lang_start
; Function Attrs: nounwind nonlazybind
define hidden i64 @_ZN3std2rt10lang_start17h6ec95d6fe2edae12E(void ()* nonnull %main, i64 %argc, i8** %argv) unnamed_addr #1 {
start:
  %_8 = alloca i64*, align 8
  %_4 = alloca i64, align 8
  %0 = bitcast i64** %_8 to void ()**
  store void ()* %main, void ()** %0, align 8
  %_5.0 = bitcast i64** %_8 to {}*
; call std::rt::lang_start_internal
  %1 = call i64 @_ZN3std2rt19lang_start_internal17h52e73755f77c7dd9E({}* nonnull align 1 %_5.0, [3 x i64]* align 8 dereferenceable(24) bitcast (<{ i8*, [16 x i8], i8*, i8*, i8*, [0 x i8] }>* @vtable.2 to [3 x i64]*), i64 %argc, i8** %argv) #7
  store i64 %1, i64* %_4, align 8
  br label %bb1

bb1:                                              ; preds = %start
  %v = load i64, i64* %_4, align 8
  ret i64 %v
}

; std::rt::lang_start::{{closure}}
; Function Attrs: inlinehint nounwind nonlazybind
define internal i32 @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h2d1dd85a0d069c44E"(i64** align 8 dereferenceable(8) %_1) unnamed_addr #0 {
start:
  %0 = bitcast i64** %_1 to void ()**
  %_3 = load void ()*, void ()** %0, align 8, !nonnull !3
; call std::sys_common::backtrace::__rust_begin_short_backtrace
  call void @_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h6deeb788a2f38acaE(void ()* nonnull %_3) #7
  br label %bb1

bb1:                                              ; preds = %start
; call <() as std::process::Termination>::report
  %1 = call i32 @"_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17h2a758e14e51677e6E"() #7
  br label %bb2

bb2:                                              ; preds = %bb1
  ret i32 %1
}

; std::sys_common::backtrace::__rust_begin_short_backtrace
; Function Attrs: noinline nounwind nonlazybind
define internal void @_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h6deeb788a2f38acaE(void ()* nonnull %f) unnamed_addr #2 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
start:
  %0 = alloca { i8*, i32 }, align 8
; call core::ops::function::FnOnce::call_once
  call void @_ZN4core3ops8function6FnOnce9call_once17h43b4487fc5fd61c2E(void ()* nonnull %f) #7
  br label %bb1

bb1:                                              ; preds = %start
; call core::hint::black_box
  call void @_ZN4core4hint9black_box17h04805f53f41105a9E() #7
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
define internal i32 @_ZN3std3sys4unix7process14process_common8ExitCode6as_i3217hfe3ce6c8485deab0E(i8* align 1 dereferenceable(1) %self) unnamed_addr #0 {
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
  %3 = call i64 @_ZN3std2rt10lang_start17h6ec95d6fe2edae12E(void ()* @_ZN6traits4main17h60c74002998e82a5E, i64 %2, i8** %1)
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
!4 = !{i32 3213236}
