; ModuleID = 'vec.8e098474-cgu.0'
source_filename = "vec.8e098474-cgu.0"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

%"alloc::vec::Vec<i32>" = type { { i32*, i64 }, i64 }
%"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>" = type { {}*, [2 x i64] }
%"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>::Some" = type { { i8*, { i64, i64 } } }
%"core::result::Result<(), alloc::collections::TryReserveError>" = type { i64, [2 x i64] }
%"core::result::Result<core::ptr::non_null::NonNull<[u8]>, alloc::collections::TryReserveError>" = type { i64, [2 x i64] }
%"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveError>, core::ptr::non_null::NonNull<[u8]>>" = type { i64, [2 x i64] }
%"core::result::Result<usize, alloc::collections::TryReserveErrorKind>" = type { i64, [2 x i64] }
%"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveErrorKind>, usize>" = type { i64, [2 x i64] }
%"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveErrorKind>, usize>::Continue" = type { [1 x i64], i64 }
%"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveErrorKind>, usize>::Break" = type { [1 x i64], { i64, i64 } }
%"core::panic::location::Location" = type { { [0 x i8]*, i64 }, i32, i32 }
%"alloc::alloc::Global" = type {}
%"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveError>, core::ptr::non_null::NonNull<[u8]>>::Continue" = type { [1 x i64], { i8*, i64 } }
%"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveError>, core::ptr::non_null::NonNull<[u8]>>::Break" = type { [1 x i64], { i64, i64 } }
%"core::result::Result<(), alloc::collections::TryReserveError>::Err" = type { [1 x i64], { i64, i64 } }
%"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveError>>" = type { i64, [2 x i64] }
%"core::result::Result<core::alloc::layout::Layout, alloc::collections::TryReserveErrorKind>" = type { i64, [2 x i64] }
%"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveErrorKind>, core::alloc::layout::Layout>" = type { i64, [2 x i64] }
%"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveErrorKind>, core::alloc::layout::Layout>::Continue" = type { [1 x i64], { i64, i64 } }
%"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveErrorKind>, core::alloc::layout::Layout>::Break" = type { [1 x i64], { i64, i64 } }
%"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveError>>::Break" = type { [1 x i64], { i64, i64 } }
%"core::result::Result<(), alloc::collections::TryReserveErrorKind>" = type { i64, [2 x i64] }
%"core::result::Result<(), alloc::collections::TryReserveErrorKind>::Err" = type { [1 x i64], { i64, i64 } }
%"core::ptr::metadata::PtrRepr<[u8]>" = type { [2 x i64] }
%"core::ptr::metadata::PtrRepr<[i32]>" = type { [2 x i64] }
%"core::result::Result<usize, alloc::collections::TryReserveErrorKind>::Err" = type { [1 x i64], { i64, i64 } }
%"core::option::Option<usize>::Some" = type { [1 x i64], i64 }
%"core::result::Result<usize, alloc::collections::TryReserveErrorKind>::Ok" = type { [1 x i64], i64 }
%"core::result::Result<core::ptr::non_null::NonNull<u8>, core::alloc::AllocError>::Err" = type { %"core::alloc::AllocError" }
%"core::alloc::AllocError" = type {}
%"core::result::Result<usize, core::alloc::layout::LayoutError>::Err" = type { [8 x i8], %"core::alloc::layout::LayoutError" }
%"core::alloc::layout::LayoutError" = type {}
%"core::result::Result<usize, core::alloc::layout::LayoutError>::Ok" = type { [1 x i64], i64 }
%"core::result::Result<core::ptr::non_null::NonNull<[u8]>, alloc::collections::TryReserveError>::Ok" = type { [1 x i64], { i8*, i64 } }
%"core::result::Result<core::ptr::non_null::NonNull<[u8]>, alloc::collections::TryReserveError>::Err" = type { [1 x i64], { i64, i64 } }
%"core::result::Result<(), alloc::collections::TryReserveErrorKind>::Ok" = type { [8 x i8], {} }
%"core::result::Result<core::alloc::layout::Layout, alloc::collections::TryReserveErrorKind>::Ok" = type { [1 x i64], { i64, i64 } }
%"core::result::Result<core::alloc::layout::Layout, alloc::collections::TryReserveErrorKind>::Err" = type { [1 x i64], { i64, i64 } }
%"core::result::Result<core::convert::Infallible, core::alloc::layout::LayoutError>::Err" = type { %"core::alloc::layout::LayoutError" }
%"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, core::alloc::layout::LayoutError>, usize>::Continue" = type { [1 x i64], i64 }
%"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, core::alloc::layout::LayoutError>, usize>::Break" = type { [8 x i8], %"core::result::Result<core::convert::Infallible, core::alloc::layout::LayoutError>::Err" }
%"core::result::Result<core::convert::Infallible, core::alloc::AllocError>::Err" = type { %"core::alloc::AllocError" }
%"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, core::alloc::AllocError>, core::ptr::non_null::NonNull<[u8]>>::Break" = type { %"core::result::Result<core::convert::Infallible, core::alloc::AllocError>::Err" }
%"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveError>>::Continue" = type { [8 x i8], {} }
%"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, core::alloc::AllocError>, core::ptr::non_null::NonNull<u8>>::Break" = type { %"core::result::Result<core::convert::Infallible, core::alloc::AllocError>::Err" }
%"core::result::Result<core::ptr::non_null::NonNull<[u8]>, core::alloc::AllocError>::Err" = type { %"core::alloc::AllocError" }
%"core::result::Result<core::alloc::layout::Layout, core::alloc::layout::LayoutError>::Err" = type { %"core::alloc::layout::LayoutError" }
%"unwind::libunwind::_Unwind_Exception" = type { i64, void (i32, %"unwind::libunwind::_Unwind_Exception"*)*, [6 x i64] }
%"unwind::libunwind::_Unwind_Context" = type { [0 x i8] }

@alloc24 = private unnamed_addr constant <{ [76 x i8] }> <{ [76 x i8] c"/rustc/9d1b2106e23b1abd32fce1f17267604a5102f57a/library/alloc/src/raw_vec.rs" }>, align 1
@alloc19 = private unnamed_addr constant <{ i8*, [16 x i8] }> <{ i8* getelementptr inbounds (<{ [76 x i8] }>, <{ [76 x i8] }>* @alloc24, i32 0, i32 0, i32 0), [16 x i8] c"L\00\00\00\00\00\00\00\88\01\00\00\1C\00\00\00" }>, align 8
@alloc21 = private unnamed_addr constant <{ i8*, [16 x i8] }> <{ i8* getelementptr inbounds (<{ [76 x i8] }>, <{ [76 x i8] }>* @alloc24, i32 0, i32 0, i32 0), [16 x i8] c"L\00\00\00\00\00\00\00\92\01\00\00\13\00\00\00" }>, align 8
@alloc23 = private unnamed_addr constant <{ i8*, [16 x i8] }> <{ i8* getelementptr inbounds (<{ [76 x i8] }>, <{ [76 x i8] }>* @alloc24, i32 0, i32 0, i32 0), [16 x i8] c"L\00\00\00\00\00\00\00\C9\01\00\00\16\00\00\00" }>, align 8
@alloc25 = private unnamed_addr constant <{ i8*, [16 x i8] }> <{ i8* getelementptr inbounds (<{ [76 x i8] }>, <{ [76 x i8] }>* @alloc24, i32 0, i32 0, i32 0), [16 x i8] c"L\00\00\00\00\00\00\00\CB\01\00\00\05\00\00\00" }>, align 8
@alloc30 = private unnamed_addr constant <{ [74 x i8] }> <{ [74 x i8] c"/rustc/9d1b2106e23b1abd32fce1f17267604a5102f57a/library/alloc/src/alloc.rs" }>, align 1
@alloc27 = private unnamed_addr constant <{ i8*, [16 x i8] }> <{ i8* getelementptr inbounds (<{ [74 x i8] }>, <{ [74 x i8] }>* @alloc30, i32 0, i32 0, i32 0), [16 x i8] c"J\00\00\00\00\00\00\00\AA\00\00\00\1B\00\00\00" }>, align 8
@alloc29 = private unnamed_addr constant <{ i8*, [16 x i8] }> <{ i8* getelementptr inbounds (<{ [74 x i8] }>, <{ [74 x i8] }>* @alloc30, i32 0, i32 0, i32 0), [16 x i8] c"J\00\00\00\00\00\00\00\CA\00\00\00\1B\00\00\00" }>, align 8
@alloc31 = private unnamed_addr constant <{ i8*, [16 x i8] }> <{ i8* getelementptr inbounds (<{ [74 x i8] }>, <{ [74 x i8] }>* @alloc30, i32 0, i32 0, i32 0), [16 x i8] c"J\00\00\00\00\00\00\00\D7\00\00\00\1F\00\00\00" }>, align 8
@0 = private unnamed_addr constant <{ [16 x i8] }> <{ [16 x i8] c"\04\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00" }>, align 8
@vtable.0 = private unnamed_addr constant <{ i8*, [16 x i8], i8*, i8*, i8*, [0 x i8] }> <{ i8* bitcast (void (i64**)* @"_ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17h1d91ff6940f232dfE" to i8*), [16 x i8] c"\08\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", i8* bitcast (i32 (i64**)* @"_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h3a2ef7bcdaa8ac9fE" to i8*), i8* bitcast (i32 (i64**)* @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h370f508993df7f7eE" to i8*), i8* bitcast (i32 (i64**)* @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h370f508993df7f7eE" to i8*), [0 x i8] zeroinitializer }>, align 8
@alloc54 = private unnamed_addr constant <{ [80 x i8] }> <{ [80 x i8] c"/rustc/9d1b2106e23b1abd32fce1f17267604a5102f57a/library/core/src/alloc/layout.rs" }>, align 1
@alloc55 = private unnamed_addr constant <{ i8*, [16 x i8] }> <{ i8* getelementptr inbounds (<{ [80 x i8] }>, <{ [80 x i8] }>* @alloc54, i32 0, i32 0, i32 0), [16 x i8] c"P\00\00\00\00\00\00\00\98\01\00\00\1A\00\00\00" }>, align 8

; core::ops::function::FnOnce::call_once{{vtable.shim}}
; Function Attrs: inlinehint nounwind nonlazybind
define internal i32 @"_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h3a2ef7bcdaa8ac9fE"(i64** %_1) unnamed_addr #0 {
start:
  %_2 = alloca {}, align 1
  %0 = load i64*, i64** %_1, align 8, !nonnull !3
; call core::ops::function::FnOnce::call_once
  %1 = call i32 @_ZN4core3ops8function6FnOnce9call_once17h0091d0978c172a4cE(i64* nonnull %0) #11
  br label %bb1

bb1:                                              ; preds = %start
  ret i32 %1
}

; core::ops::function::FnOnce::call_once
; Function Attrs: inlinehint nounwind nonlazybind
define internal i32 @_ZN4core3ops8function6FnOnce9call_once17h0091d0978c172a4cE(i64* nonnull %0) unnamed_addr #0 {
start:
  %_2 = alloca {}, align 1
  %_1 = alloca i64*, align 8
  store i64* %0, i64** %_1, align 8
; call std::rt::lang_start::{{closure}}
  %1 = call i32 @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h370f508993df7f7eE"(i64** align 8 dereferenceable(8) %_1) #11
  br label %bb1

bb1:                                              ; preds = %start
  br label %bb2

bb2:                                              ; preds = %bb1
  ret i32 %1
}

; core::ops::function::FnOnce::call_once
; Function Attrs: inlinehint nounwind nonlazybind
define internal i8 @_ZN4core3ops8function6FnOnce9call_once17h7a32591392ef8fbbE(i64* align 8 dereferenceable(8) %0, i64* align 8 dereferenceable(8) %1) unnamed_addr #0 {
start:
  %_2 = alloca { i64*, i64* }, align 8
  %2 = bitcast { i64*, i64* }* %_2 to i64**
  store i64* %0, i64** %2, align 8
  %3 = getelementptr inbounds { i64*, i64* }, { i64*, i64* }* %_2, i32 0, i32 1
  store i64* %1, i64** %3, align 8
  %4 = bitcast { i64*, i64* }* %_2 to i64**
  %5 = load i64*, i64** %4, align 8, !nonnull !3
  %6 = getelementptr inbounds { i64*, i64* }, { i64*, i64* }* %_2, i32 0, i32 1
  %7 = load i64*, i64** %6, align 8, !nonnull !3
; call core::cmp::impls::<impl core::cmp::Ord for usize>::cmp
  %8 = call i8 @"_ZN4core3cmp5impls50_$LT$impl$u20$core..cmp..Ord$u20$for$u20$usize$GT$3cmp17hef3ac0443fee4a2bE"(i64* align 8 dereferenceable(8) %5, i64* align 8 dereferenceable(8) %7) #11, !range !4
  br label %bb1

bb1:                                              ; preds = %start
  ret i8 %8
}

; core::ops::function::FnOnce::call_once
; Function Attrs: inlinehint nounwind nonlazybind
define internal void @_ZN4core3ops8function6FnOnce9call_once17hbd0c30072a9b7299E(void ()* nonnull %_1) unnamed_addr #0 {
start:
  %_2 = alloca {}, align 1
  call void %_1() #11
  br label %bb1

bb1:                                              ; preds = %start
  ret void
}

; core::ptr::drop_in_place<alloc::vec::Vec<i32>>
; Function Attrs: nounwind nonlazybind
define internal void @"_ZN4core3ptr47drop_in_place$LT$alloc..vec..Vec$LT$i32$GT$$GT$17hd5b45fa0a764dac2E"(%"alloc::vec::Vec<i32>"* %_1) unnamed_addr #1 {
start:
; call <alloc::vec::Vec<T,A> as core::ops::drop::Drop>::drop
  call void @"_ZN70_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h672635a5205cb45fE"(%"alloc::vec::Vec<i32>"* align 8 dereferenceable(24) %_1) #11
  br label %bb2

bb2:                                              ; preds = %start
  %0 = bitcast %"alloc::vec::Vec<i32>"* %_1 to { i32*, i64 }*
; call core::ptr::drop_in_place<alloc::raw_vec::RawVec<i32>>
  call void @"_ZN4core3ptr54drop_in_place$LT$alloc..raw_vec..RawVec$LT$i32$GT$$GT$17h2e02d878152e6bb9E"({ i32*, i64 }* %0) #11
  br label %bb1

bb1:                                              ; preds = %bb2
  ret void
}

; core::ptr::drop_in_place<alloc::raw_vec::RawVec<i32>>
; Function Attrs: nounwind nonlazybind
define internal void @"_ZN4core3ptr54drop_in_place$LT$alloc..raw_vec..RawVec$LT$i32$GT$$GT$17h2e02d878152e6bb9E"({ i32*, i64 }* %_1) unnamed_addr #1 {
start:
; call <alloc::raw_vec::RawVec<T,A> as core::ops::drop::Drop>::drop
  call void @"_ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17hd56ac955c50c3290E"({ i32*, i64 }* align 8 dereferenceable(16) %_1) #11
  br label %bb1

bb1:                                              ; preds = %start
  ret void
}

; core::ptr::drop_in_place<std::rt::lang_start<()>::{{closure}}>
; Function Attrs: inlinehint nounwind nonlazybind
define internal void @"_ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17h1d91ff6940f232dfE"(i64** %_1) unnamed_addr #0 {
start:
  ret void
}

; core::clone::Clone::clone
; Function Attrs: inlinehint nounwind nonlazybind
define internal void @_ZN4core5clone5Clone5clone17had41c4bc85d28bbcE({}* nonnull align 1 %_1) unnamed_addr #0 {
start:
  ret void
}

; vec::foo
; Function Attrs: nounwind nonlazybind
define internal void @_ZN3vec3foo17h5e160f928db88539E() unnamed_addr #1 {
start:
  %a = alloca %"alloc::vec::Vec<i32>", align 8
; call alloc::vec::Vec<T>::new
  call void @"_ZN5alloc3vec12Vec$LT$T$GT$3new17hcfe09e8a68ed215aE"(%"alloc::vec::Vec<i32>"* noalias nocapture sret(%"alloc::vec::Vec<i32>") dereferenceable(24) %a) #11
  br label %bb1

bb1:                                              ; preds = %start
; call alloc::vec::Vec<T,A>::push
  call void @"_ZN5alloc3vec16Vec$LT$T$C$A$GT$4push17h335fdb8aa55e893dE"(%"alloc::vec::Vec<i32>"* align 8 dereferenceable(24) %a, i32 1) #11
  br label %bb2

bb2:                                              ; preds = %bb1
; call core::ptr::drop_in_place<alloc::vec::Vec<i32>>
  call void @"_ZN4core3ptr47drop_in_place$LT$alloc..vec..Vec$LT$i32$GT$$GT$17hd5b45fa0a764dac2E"(%"alloc::vec::Vec<i32>"* %a) #11
  br label %bb3

bb3:                                              ; preds = %bb2
  ret void
}

; vec::main
; Function Attrs: nounwind nonlazybind
define internal void @_ZN3vec4main17h0fff19dbb1ce78d1E() unnamed_addr #1 {
start:
; call vec::foo
  call void @_ZN3vec3foo17h5e160f928db88539E() #11
  br label %bb1

bb1:                                              ; preds = %start
  ret void
}

; alloc::raw_vec::RawVec<T,A>::ptr
; Function Attrs: inlinehint nounwind nonlazybind
define internal i32* @"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$3ptr17hf51cd8d24eb24d44E"({ i32*, i64 }* align 8 dereferenceable(16) %self) unnamed_addr #0 {
start:
  %0 = bitcast { i32*, i64 }* %self to i32**
  %_2 = load i32*, i32** %0, align 8, !nonnull !3
; call core::ptr::unique::Unique<T>::as_ptr
  %1 = call i32* @"_ZN4core3ptr6unique15Unique$LT$T$GT$6as_ptr17hf27a8920f282d9a0E"(i32* nonnull %_2) #11
  br label %bb1

bb1:                                              ; preds = %start
  ret i32* %1
}

; alloc::raw_vec::RawVec<T,A>::current_memory
; Function Attrs: nounwind nonlazybind
define internal void @"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14current_memory17hf0dd6c0d5cd112ccE"(%"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>"* noalias nocapture sret(%"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>") dereferenceable(24) %0, { i32*, i64 }* align 8 dereferenceable(16) %self) unnamed_addr #1 {
start:
  %_13 = alloca { i8*, { i64, i64 } }, align 8
  %_2 = alloca i8, align 1
  br label %bb4

bb4:                                              ; preds = %start
  %1 = icmp eq i64 4, 0
  br i1 %1, label %bb1, label %bb2

bb1:                                              ; preds = %bb4
  store i8 1, i8* %_2, align 1
  br label %bb3

bb2:                                              ; preds = %bb4
  %2 = getelementptr inbounds { i32*, i64 }, { i32*, i64 }* %self, i32 0, i32 1
  %_5 = load i64, i64* %2, align 8
  %_4 = icmp eq i64 %_5, 0
  %3 = zext i1 %_4 to i8
  store i8 %3, i8* %_2, align 1
  br label %bb3

bb3:                                              ; preds = %bb1, %bb2
  %4 = load i8, i8* %_2, align 1, !range !5
  %5 = trunc i8 %4 to i1
  br i1 %5, label %bb5, label %bb6

bb6:                                              ; preds = %bb3
  br label %bb7

bb5:                                              ; preds = %bb3
  %6 = bitcast %"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>"* %0 to {}**
  store {}* null, {}** %6, align 8
  br label %bb12

bb12:                                             ; preds = %bb11, %bb5
  ret void

bb7:                                              ; preds = %bb6
  br label %bb8

bb8:                                              ; preds = %bb7
  %7 = getelementptr inbounds { i32*, i64 }, { i32*, i64 }* %self, i32 0, i32 1
  %_9 = load i64, i64* %7, align 8
  %size = mul i64 4, %_9
; call core::alloc::layout::Layout::from_size_align_unchecked
  %8 = call { i64, i64 } @_ZN4core5alloc6layout6Layout25from_size_align_unchecked17hed3a5110f94d0efeE(i64 %size, i64 4) #11
  %layout.0 = extractvalue { i64, i64 } %8, 0
  %layout.1 = extractvalue { i64, i64 } %8, 1
  br label %bb9

bb9:                                              ; preds = %bb8
  %9 = bitcast { i32*, i64 }* %self to i32**
  %_16 = load i32*, i32** %9, align 8, !nonnull !3
; call core::ptr::unique::Unique<T>::cast
  %_15 = call nonnull i8* @"_ZN4core3ptr6unique15Unique$LT$T$GT$4cast17h42655d37bf6d0057E"(i32* nonnull %_16) #11
  br label %bb10

bb10:                                             ; preds = %bb9
; call <T as core::convert::Into<U>>::into
  %_14 = call nonnull i8* @"_ZN50_$LT$T$u20$as$u20$core..convert..Into$LT$U$GT$$GT$4into17ha86098bf45577f87E"(i8* nonnull %_15) #11
  br label %bb11

bb11:                                             ; preds = %bb10
  %10 = bitcast { i8*, { i64, i64 } }* %_13 to i8**
  store i8* %_14, i8** %10, align 8
  %11 = getelementptr inbounds { i8*, { i64, i64 } }, { i8*, { i64, i64 } }* %_13, i32 0, i32 1
  %12 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %11, i32 0, i32 0
  store i64 %layout.0, i64* %12, align 8
  %13 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %11, i32 0, i32 1
  store i64 %layout.1, i64* %13, align 8
  %14 = bitcast %"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>"* %0 to %"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>::Some"*
  %15 = bitcast %"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>::Some"* %14 to { i8*, { i64, i64 } }*
  %16 = bitcast { i8*, { i64, i64 } }* %15 to i8*
  %17 = bitcast { i8*, { i64, i64 } }* %_13 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %16, i8* align 8 %17, i64 24, i1 false)
  br label %bb12
}

; alloc::raw_vec::RawVec<T,A>::reserve_for_push
; Function Attrs: noinline nounwind nonlazybind
define internal void @"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$16reserve_for_push17haa0e1e5e2b63f67aE"({ i32*, i64 }* align 8 dereferenceable(16) %self, i64 %len) unnamed_addr #2 {
start:
  %_4 = alloca %"core::result::Result<(), alloc::collections::TryReserveError>", align 8
; call alloc::raw_vec::RawVec<T,A>::grow_amortized
  call void @"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14grow_amortized17h96fba905360e0885E"(%"core::result::Result<(), alloc::collections::TryReserveError>"* noalias nocapture sret(%"core::result::Result<(), alloc::collections::TryReserveError>") dereferenceable(24) %_4, { i32*, i64 }* align 8 dereferenceable(16) %self, i64 %len, i64 1) #11
  br label %bb1

bb1:                                              ; preds = %start
; call alloc::raw_vec::handle_reserve
  call void @_ZN5alloc7raw_vec14handle_reserve17h7337eab9ee492301E(%"core::result::Result<(), alloc::collections::TryReserveError>"* noalias nocapture dereferenceable(24) %_4) #11
  br label %bb2

bb2:                                              ; preds = %bb1
  ret void
}

; alloc::raw_vec::RawVec<T,A>::set_ptr_and_cap
; Function Attrs: nounwind nonlazybind
define internal void @"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$15set_ptr_and_cap17hdf47c3b1f6474164E"({ i32*, i64 }* align 8 dereferenceable(16) %self, i8* nonnull %ptr.0, i64 %ptr.1, i64 %cap) unnamed_addr #1 {
start:
; call core::ptr::non_null::NonNull<T>::cast
  %_6 = call nonnull i32* @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$4cast17hacb27b780c73e81cE"(i8* nonnull %ptr.0, i64 %ptr.1) #11
  br label %bb1

bb1:                                              ; preds = %start
; call core::ptr::non_null::NonNull<T>::as_ptr
  %_5 = call i32* @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$6as_ptr17h3765aa681de6eda6E"(i32* nonnull %_6) #11
  br label %bb2

bb2:                                              ; preds = %bb1
; call core::ptr::unique::Unique<T>::new_unchecked
  %_4 = call nonnull i32* @"_ZN4core3ptr6unique15Unique$LT$T$GT$13new_unchecked17he969dda07b0c4bceE"(i32* %_5) #11
  br label %bb3

bb3:                                              ; preds = %bb2
  %0 = bitcast { i32*, i64 }* %self to i32**
  store i32* %_4, i32** %0, align 8
  %1 = getelementptr inbounds { i32*, i64 }, { i32*, i64 }* %self, i32 0, i32 1
  store i64 %cap, i64* %1, align 8
  ret void
}

; alloc::raw_vec::RawVec<T,A>::grow_amortized
; Function Attrs: nounwind nonlazybind
define internal void @"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14grow_amortized17h96fba905360e0885E"(%"core::result::Result<(), alloc::collections::TryReserveError>"* noalias nocapture sret(%"core::result::Result<(), alloc::collections::TryReserveError>") dereferenceable(24) %0, { i32*, i64 }* align 8 dereferenceable(16) %self, i64 %len, i64 %additional) unnamed_addr #1 {
start:
  %_30 = alloca %"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>", align 8
  %_28 = alloca %"core::result::Result<core::ptr::non_null::NonNull<[u8]>, alloc::collections::TryReserveError>", align 8
  %_27 = alloca %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveError>, core::ptr::non_null::NonNull<[u8]>>", align 8
  %_13 = alloca { i64, i64 }, align 8
  %_9 = alloca %"core::result::Result<usize, alloc::collections::TryReserveErrorKind>", align 8
  %_8 = alloca %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveErrorKind>, usize>", align 8
  %_6 = alloca { i64, i64 }, align 8
  br label %bb1

bb1:                                              ; preds = %start
  %1 = icmp eq i64 4, 0
  br i1 %1, label %bb2, label %bb4

bb2:                                              ; preds = %bb1
  %2 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %_6, i32 0, i32 1
  store i64 0, i64* %2, align 8
  %3 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %_6, i32 0, i32 0
  %4 = load i64, i64* %3, align 8
  %5 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %_6, i32 0, i32 1
  %6 = load i64, i64* %5, align 8
; call <T as core::convert::Into<U>>::into
  %7 = call { i64, i64 } @"_ZN50_$LT$T$u20$as$u20$core..convert..Into$LT$U$GT$$GT$4into17h5cabd2ba38994778E"(i64 %4, i64 %6) #11
  %_5.0 = extractvalue { i64, i64 } %7, 0
  %_5.1 = extractvalue { i64, i64 } %7, 1
  br label %bb3

bb4:                                              ; preds = %bb1
; call core::num::<impl usize>::checked_add
  %8 = call { i64, i64 } @"_ZN4core3num23_$LT$impl$u20$usize$GT$11checked_add17h5629599ae0178fafE"(i64 %len, i64 %additional) #11
  %_10.0 = extractvalue { i64, i64 } %8, 0
  %_10.1 = extractvalue { i64, i64 } %8, 1
  br label %bb5

bb5:                                              ; preds = %bb4
  %9 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %_13, i32 0, i32 1
  store i64 0, i64* %9, align 8
  %10 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %_13, i32 0, i32 0
  %11 = load i64, i64* %10, align 8
  %12 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %_13, i32 0, i32 1
  %13 = load i64, i64* %12, align 8
; call core::option::Option<T>::ok_or
  call void @"_ZN4core6option15Option$LT$T$GT$5ok_or17h353dffde265f05a4E"(%"core::result::Result<usize, alloc::collections::TryReserveErrorKind>"* noalias nocapture sret(%"core::result::Result<usize, alloc::collections::TryReserveErrorKind>") dereferenceable(24) %_9, i64 %_10.0, i64 %_10.1, i64 %11, i64 %13) #11
  br label %bb6

bb6:                                              ; preds = %bb5
; call <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
  call void @"_ZN79_$LT$core..result..Result$LT$T$C$E$GT$$u20$as$u20$core..ops..try_trait..Try$GT$6branch17h91d8513c54bc4f35E"(%"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveErrorKind>, usize>"* noalias nocapture sret(%"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveErrorKind>, usize>") dereferenceable(24) %_8, %"core::result::Result<usize, alloc::collections::TryReserveErrorKind>"* noalias nocapture dereferenceable(24) %_9) #11
  br label %bb7

bb7:                                              ; preds = %bb6
  %14 = bitcast %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveErrorKind>, usize>"* %_8 to i64*
  %_14 = load i64, i64* %14, align 8, !range !6
  switch i64 %_14, label %bb9 [
    i64 0, label %bb8
    i64 1, label %bb10
  ]

bb9:                                              ; preds = %bb7
  unreachable

bb8:                                              ; preds = %bb7
  %15 = bitcast %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveErrorKind>, usize>"* %_8 to %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveErrorKind>, usize>::Continue"*
  %16 = getelementptr inbounds %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveErrorKind>, usize>::Continue", %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveErrorKind>, usize>::Continue"* %15, i32 0, i32 1
  %val = load i64, i64* %16, align 8
  %17 = getelementptr inbounds { i32*, i64 }, { i32*, i64 }* %self, i32 0, i32 1
  %_20 = load i64, i64* %17, align 8
  %_19 = mul i64 %_20, 2
; call core::cmp::max
  %cap = call i64 @_ZN4core3cmp3max17hd755847590d220faE(i64 %_19, i64 %val) #11
  br label %bb12

bb10:                                             ; preds = %bb7
  %18 = bitcast %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveErrorKind>, usize>"* %_8 to %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveErrorKind>, usize>::Break"*
  %19 = getelementptr inbounds %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveErrorKind>, usize>::Break", %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveErrorKind>, usize>::Break"* %18, i32 0, i32 1
  %20 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %19, i32 0, i32 0
  %residual.0 = load i64, i64* %20, align 8
  %21 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %19, i32 0, i32 1
  %residual.1 = load i64, i64* %21, align 8
; call <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
  call void @"_ZN153_$LT$core..result..Result$LT$T$C$F$GT$$u20$as$u20$core..ops..try_trait..FromResidual$LT$core..result..Result$LT$core..convert..Infallible$C$E$GT$$GT$$GT$13from_residual17haa158d46f4a111f3E"(%"core::result::Result<(), alloc::collections::TryReserveError>"* noalias nocapture sret(%"core::result::Result<(), alloc::collections::TryReserveError>") dereferenceable(24) %0, i64 %residual.0, i64 %residual.1, %"core::panic::location::Location"* align 8 dereferenceable(24) bitcast (<{ i8*, [16 x i8] }>* @alloc19 to %"core::panic::location::Location"*)) #11
  br label %bb11

bb11:                                             ; preds = %bb10
  br label %bb23

bb23:                                             ; preds = %bb21, %bb11
  br label %bb24

bb12:                                             ; preds = %bb8
; call core::cmp::max
  %cap1 = call i64 @_ZN4core3cmp3max17hd755847590d220faE(i64 4, i64 %cap) #11
  br label %bb13

bb13:                                             ; preds = %bb12
; call core::alloc::layout::Layout::array
  %22 = call { i64, i64 } @_ZN4core5alloc6layout6Layout5array17h0e5a95af5beae033E(i64 %cap1) #11
  %new_layout.0 = extractvalue { i64, i64 } %22, 0
  %new_layout.1 = extractvalue { i64, i64 } %22, 1
  br label %bb14

bb14:                                             ; preds = %bb13
; call alloc::raw_vec::RawVec<T,A>::current_memory
  call void @"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14current_memory17hf0dd6c0d5cd112ccE"(%"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>"* noalias nocapture sret(%"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>") dereferenceable(24) %_30, { i32*, i64 }* align 8 dereferenceable(16) %self) #11
  br label %bb15

bb15:                                             ; preds = %bb14
  %_33 = bitcast { i32*, i64 }* %self to %"alloc::alloc::Global"*
; call alloc::raw_vec::finish_grow
  call void @_ZN5alloc7raw_vec11finish_grow17hef08c8017d9a88a3E(%"core::result::Result<core::ptr::non_null::NonNull<[u8]>, alloc::collections::TryReserveError>"* noalias nocapture sret(%"core::result::Result<core::ptr::non_null::NonNull<[u8]>, alloc::collections::TryReserveError>") dereferenceable(24) %_28, i64 %new_layout.0, i64 %new_layout.1, %"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>"* noalias nocapture dereferenceable(24) %_30, %"alloc::alloc::Global"* nonnull align 1 %_33) #11
  br label %bb16

bb16:                                             ; preds = %bb15
; call <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
  call void @"_ZN79_$LT$core..result..Result$LT$T$C$E$GT$$u20$as$u20$core..ops..try_trait..Try$GT$6branch17h15bb74d3338a4f44E"(%"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveError>, core::ptr::non_null::NonNull<[u8]>>"* noalias nocapture sret(%"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveError>, core::ptr::non_null::NonNull<[u8]>>") dereferenceable(24) %_27, %"core::result::Result<core::ptr::non_null::NonNull<[u8]>, alloc::collections::TryReserveError>"* noalias nocapture dereferenceable(24) %_28) #11
  br label %bb17

bb17:                                             ; preds = %bb16
  %23 = bitcast %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveError>, core::ptr::non_null::NonNull<[u8]>>"* %_27 to i64*
  %_34 = load i64, i64* %23, align 8, !range !6
  switch i64 %_34, label %bb19 [
    i64 0, label %bb18
    i64 1, label %bb20
  ]

bb19:                                             ; preds = %bb17
  unreachable

bb18:                                             ; preds = %bb17
  %24 = bitcast %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveError>, core::ptr::non_null::NonNull<[u8]>>"* %_27 to %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveError>, core::ptr::non_null::NonNull<[u8]>>::Continue"*
  %25 = getelementptr inbounds %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveError>, core::ptr::non_null::NonNull<[u8]>>::Continue", %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveError>, core::ptr::non_null::NonNull<[u8]>>::Continue"* %24, i32 0, i32 1
  %26 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %25, i32 0, i32 0
  %val.0 = load i8*, i8** %26, align 8, !nonnull !3
  %27 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %25, i32 0, i32 1
  %val.1 = load i64, i64* %27, align 8
; call alloc::raw_vec::RawVec<T,A>::set_ptr_and_cap
  call void @"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$15set_ptr_and_cap17hdf47c3b1f6474164E"({ i32*, i64 }* align 8 dereferenceable(16) %self, i8* nonnull %val.0, i64 %val.1, i64 %cap1) #11
  br label %bb22

bb20:                                             ; preds = %bb17
  %28 = bitcast %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveError>, core::ptr::non_null::NonNull<[u8]>>"* %_27 to %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveError>, core::ptr::non_null::NonNull<[u8]>>::Break"*
  %29 = getelementptr inbounds %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveError>, core::ptr::non_null::NonNull<[u8]>>::Break", %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveError>, core::ptr::non_null::NonNull<[u8]>>::Break"* %28, i32 0, i32 1
  %30 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %29, i32 0, i32 0
  %residual.02 = load i64, i64* %30, align 8
  %31 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %29, i32 0, i32 1
  %residual.13 = load i64, i64* %31, align 8
; call <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
  call void @"_ZN153_$LT$core..result..Result$LT$T$C$F$GT$$u20$as$u20$core..ops..try_trait..FromResidual$LT$core..result..Result$LT$core..convert..Infallible$C$E$GT$$GT$$GT$13from_residual17h3b6176bcfb7719dcE"(%"core::result::Result<(), alloc::collections::TryReserveError>"* noalias nocapture sret(%"core::result::Result<(), alloc::collections::TryReserveError>") dereferenceable(24) %0, i64 %residual.02, i64 %residual.13, %"core::panic::location::Location"* align 8 dereferenceable(24) bitcast (<{ i8*, [16 x i8] }>* @alloc21 to %"core::panic::location::Location"*)) #11
  br label %bb21

bb21:                                             ; preds = %bb20
  br label %bb23

bb24:                                             ; preds = %bb3, %bb22, %bb23
  ret void

bb22:                                             ; preds = %bb18
  %32 = bitcast %"core::result::Result<(), alloc::collections::TryReserveError>"* %0 to i64*
  store i64 0, i64* %32, align 8
  br label %bb24

bb3:                                              ; preds = %bb2
  %33 = bitcast %"core::result::Result<(), alloc::collections::TryReserveError>"* %0 to %"core::result::Result<(), alloc::collections::TryReserveError>::Err"*
  %34 = getelementptr inbounds %"core::result::Result<(), alloc::collections::TryReserveError>::Err", %"core::result::Result<(), alloc::collections::TryReserveError>::Err"* %33, i32 0, i32 1
  %35 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %34, i32 0, i32 0
  store i64 %_5.0, i64* %35, align 8
  %36 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %34, i32 0, i32 1
  store i64 %_5.1, i64* %36, align 8
  %37 = bitcast %"core::result::Result<(), alloc::collections::TryReserveError>"* %0 to i64*
  store i64 1, i64* %37, align 8
  br label %bb24
}

; alloc::raw_vec::finish_grow
; Function Attrs: noinline nounwind nonlazybind
define internal void @_ZN5alloc7raw_vec11finish_grow17hef08c8017d9a88a3E(%"core::result::Result<core::ptr::non_null::NonNull<[u8]>, alloc::collections::TryReserveError>"* noalias nocapture sret(%"core::result::Result<core::ptr::non_null::NonNull<[u8]>, alloc::collections::TryReserveError>") dereferenceable(24) %0, i64 %new_layout.0, i64 %new_layout.1, %"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>"* noalias nocapture dereferenceable(24) %current_memory, %"alloc::alloc::Global"* nonnull align 1 %alloc) unnamed_addr #2 {
start:
  %_41 = alloca i64*, align 8
  %old_layout = alloca { i64, i64 }, align 8
  %memory = alloca { i8*, i64 }, align 8
  %_14 = alloca %"core::result::Result<(), alloc::collections::TryReserveError>", align 8
  %_13 = alloca %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveError>>", align 8
  %_6 = alloca %"core::result::Result<core::alloc::layout::Layout, alloc::collections::TryReserveErrorKind>", align 8
  %_5 = alloca %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveErrorKind>, core::alloc::layout::Layout>", align 8
  %new_layout = alloca { i64, i64 }, align 8
; call core::result::Result<T,E>::map_err
  call void @"_ZN4core6result19Result$LT$T$C$E$GT$7map_err17he544d618a000548cE"(%"core::result::Result<core::alloc::layout::Layout, alloc::collections::TryReserveErrorKind>"* noalias nocapture sret(%"core::result::Result<core::alloc::layout::Layout, alloc::collections::TryReserveErrorKind>") dereferenceable(24) %_6, i64 %new_layout.0, i64 %new_layout.1) #11
  br label %bb1

bb1:                                              ; preds = %start
; call <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
  call void @"_ZN79_$LT$core..result..Result$LT$T$C$E$GT$$u20$as$u20$core..ops..try_trait..Try$GT$6branch17h7f9906f35b565ee6E"(%"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveErrorKind>, core::alloc::layout::Layout>"* noalias nocapture sret(%"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveErrorKind>, core::alloc::layout::Layout>") dereferenceable(24) %_5, %"core::result::Result<core::alloc::layout::Layout, alloc::collections::TryReserveErrorKind>"* noalias nocapture dereferenceable(24) %_6) #11
  br label %bb2

bb2:                                              ; preds = %bb1
  %1 = bitcast %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveErrorKind>, core::alloc::layout::Layout>"* %_5 to i64*
  %_9 = load i64, i64* %1, align 8, !range !6
  switch i64 %_9, label %bb4 [
    i64 0, label %bb3
    i64 1, label %bb5
  ]

bb4:                                              ; preds = %bb2
  unreachable

bb3:                                              ; preds = %bb2
  %2 = bitcast %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveErrorKind>, core::alloc::layout::Layout>"* %_5 to %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveErrorKind>, core::alloc::layout::Layout>::Continue"*
  %3 = getelementptr inbounds %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveErrorKind>, core::alloc::layout::Layout>::Continue", %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveErrorKind>, core::alloc::layout::Layout>::Continue"* %2, i32 0, i32 1
  %4 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %3, i32 0, i32 0
  %val.0 = load i64, i64* %4, align 8
  %5 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %3, i32 0, i32 1
  %val.1 = load i64, i64* %5, align 8, !range !7
  %6 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %new_layout, i32 0, i32 0
  store i64 %val.0, i64* %6, align 8
  %7 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %new_layout, i32 0, i32 1
  store i64 %val.1, i64* %7, align 8
; call core::alloc::layout::Layout::size
  %_15 = call i64 @_ZN4core5alloc6layout6Layout4size17h081c8035e28220cdE({ i64, i64 }* align 8 dereferenceable(16) %new_layout) #11
  br label %bb7

bb5:                                              ; preds = %bb2
  %8 = bitcast %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveErrorKind>, core::alloc::layout::Layout>"* %_5 to %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveErrorKind>, core::alloc::layout::Layout>::Break"*
  %9 = getelementptr inbounds %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveErrorKind>, core::alloc::layout::Layout>::Break", %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveErrorKind>, core::alloc::layout::Layout>::Break"* %8, i32 0, i32 1
  %10 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %9, i32 0, i32 0
  %residual.0 = load i64, i64* %10, align 8
  %11 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %9, i32 0, i32 1
  %residual.1 = load i64, i64* %11, align 8
; call <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
  call void @"_ZN153_$LT$core..result..Result$LT$T$C$F$GT$$u20$as$u20$core..ops..try_trait..FromResidual$LT$core..result..Result$LT$core..convert..Infallible$C$E$GT$$GT$$GT$13from_residual17h0c3a62696a58b0e2E"(%"core::result::Result<core::ptr::non_null::NonNull<[u8]>, alloc::collections::TryReserveError>"* noalias nocapture sret(%"core::result::Result<core::ptr::non_null::NonNull<[u8]>, alloc::collections::TryReserveError>") dereferenceable(24) %0, i64 %residual.0, i64 %residual.1, %"core::panic::location::Location"* align 8 dereferenceable(24) bitcast (<{ i8*, [16 x i8] }>* @alloc23 to %"core::panic::location::Location"*)) #11
  br label %bb6

bb6:                                              ; preds = %bb5
  br label %bb23

bb23:                                             ; preds = %bb13, %bb6
  br label %bb24

bb7:                                              ; preds = %bb3
; call alloc::raw_vec::alloc_guard
  call void @_ZN5alloc7raw_vec11alloc_guard17h6c56d83ecb90f3d6E(%"core::result::Result<(), alloc::collections::TryReserveError>"* noalias nocapture sret(%"core::result::Result<(), alloc::collections::TryReserveError>") dereferenceable(24) %_14, i64 %_15) #11
  br label %bb8

bb8:                                              ; preds = %bb7
; call <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
  call void @"_ZN79_$LT$core..result..Result$LT$T$C$E$GT$$u20$as$u20$core..ops..try_trait..Try$GT$6branch17h65a9c1ea517ae954E"(%"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveError>>"* noalias nocapture sret(%"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveError>>") dereferenceable(24) %_13, %"core::result::Result<(), alloc::collections::TryReserveError>"* noalias nocapture dereferenceable(24) %_14) #11
  br label %bb9

bb9:                                              ; preds = %bb8
  %12 = bitcast %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveError>>"* %_13 to i64*
  %_17 = load i64, i64* %12, align 8, !range !6
  switch i64 %_17, label %bb11 [
    i64 0, label %bb10
    i64 1, label %bb12
  ]

bb11:                                             ; preds = %bb9
  unreachable

bb10:                                             ; preds = %bb9
  %13 = bitcast %"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>"* %current_memory to {}**
  %14 = load {}*, {}** %13, align 8
  %15 = icmp eq {}* %14, null
  %_22 = select i1 %15, i64 0, i64 1
  %16 = icmp eq i64 %_22, 1
  br i1 %16, label %bb15, label %bb14

bb12:                                             ; preds = %bb9
  %17 = bitcast %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveError>>"* %_13 to %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveError>>::Break"*
  %18 = getelementptr inbounds %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveError>>::Break", %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveError>>::Break"* %17, i32 0, i32 1
  %19 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %18, i32 0, i32 0
  %residual.01 = load i64, i64* %19, align 8
  %20 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %18, i32 0, i32 1
  %residual.12 = load i64, i64* %20, align 8
; call <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
  call void @"_ZN153_$LT$core..result..Result$LT$T$C$F$GT$$u20$as$u20$core..ops..try_trait..FromResidual$LT$core..result..Result$LT$core..convert..Infallible$C$E$GT$$GT$$GT$13from_residual17hf7b0ca1e6476cb8cE"(%"core::result::Result<core::ptr::non_null::NonNull<[u8]>, alloc::collections::TryReserveError>"* noalias nocapture sret(%"core::result::Result<core::ptr::non_null::NonNull<[u8]>, alloc::collections::TryReserveError>") dereferenceable(24) %0, i64 %residual.01, i64 %residual.12, %"core::panic::location::Location"* align 8 dereferenceable(24) bitcast (<{ i8*, [16 x i8] }>* @alloc25 to %"core::panic::location::Location"*)) #11
  br label %bb13

bb13:                                             ; preds = %bb12
  br label %bb23

bb24:                                             ; preds = %bb22, %bb23
  ret void

bb15:                                             ; preds = %bb10
  %21 = bitcast %"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>"* %current_memory to %"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>::Some"*
  %22 = bitcast %"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>::Some"* %21 to { i8*, { i64, i64 } }*
  %23 = bitcast { i8*, { i64, i64 } }* %22 to i8**
  %ptr = load i8*, i8** %23, align 8, !nonnull !3
  %24 = bitcast %"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>"* %current_memory to %"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>::Some"*
  %25 = bitcast %"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>::Some"* %24 to { i8*, { i64, i64 } }*
  %26 = getelementptr inbounds { i8*, { i64, i64 } }, { i8*, { i64, i64 } }* %25, i32 0, i32 1
  %27 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %26, i32 0, i32 0
  %28 = load i64, i64* %27, align 8
  %29 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %26, i32 0, i32 1
  %30 = load i64, i64* %29, align 8, !range !7
  %31 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %old_layout, i32 0, i32 0
  store i64 %28, i64* %31, align 8
  %32 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %old_layout, i32 0, i32 1
  store i64 %30, i64* %32, align 8
; call core::alloc::layout::Layout::align
  %_30 = call i64 @_ZN4core5alloc6layout6Layout5align17he5ae42b9af0cb70fE({ i64, i64 }* align 8 dereferenceable(16) %old_layout) #11
  br label %bb16

bb14:                                             ; preds = %bb10
  %33 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %new_layout, i32 0, i32 0
  %_39.0 = load i64, i64* %33, align 8
  %34 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %new_layout, i32 0, i32 1
  %_39.1 = load i64, i64* %34, align 8, !range !7
; call <alloc::alloc::Global as core::alloc::Allocator>::allocate
  %35 = call { i8*, i64 } @"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h5ca58336f2900dc9E"(%"alloc::alloc::Global"* nonnull align 1 %alloc, i64 %_39.0, i64 %_39.1) #11
  store { i8*, i64 } %35, { i8*, i64 }* %memory, align 8
  br label %bb20

bb20:                                             ; preds = %bb14
  br label %bb21

bb21:                                             ; preds = %bb19, %bb20
  %36 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %memory, i32 0, i32 0
  %_40.0 = load i8*, i8** %36, align 8
  %37 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %memory, i32 0, i32 1
  %_40.1 = load i64, i64* %37, align 8
  %38 = bitcast i64** %_41 to { i64, i64 }**
  store { i64, i64 }* %new_layout, { i64, i64 }** %38, align 8
  %39 = load i64*, i64** %_41, align 8, !nonnull !3
; call core::result::Result<T,E>::map_err
  call void @"_ZN4core6result19Result$LT$T$C$E$GT$7map_err17h1aab0f13572c0699E"(%"core::result::Result<core::ptr::non_null::NonNull<[u8]>, alloc::collections::TryReserveError>"* noalias nocapture sret(%"core::result::Result<core::ptr::non_null::NonNull<[u8]>, alloc::collections::TryReserveError>") dereferenceable(24) %0, i8* %_40.0, i64 %_40.1, i64* align 8 dereferenceable(16) %39) #11
  br label %bb22

bb16:                                             ; preds = %bb15
; call core::alloc::layout::Layout::align
  %_32 = call i64 @_ZN4core5alloc6layout6Layout5align17he5ae42b9af0cb70fE({ i64, i64 }* align 8 dereferenceable(16) %new_layout) #11
  br label %bb17

bb17:                                             ; preds = %bb16
  %_29 = icmp eq i64 %_30, %_32
  call void @llvm.assume(i1 %_29)
  br label %bb18

bb18:                                             ; preds = %bb17
  %40 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %old_layout, i32 0, i32 0
  %_36.0 = load i64, i64* %40, align 8
  %41 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %old_layout, i32 0, i32 1
  %_36.1 = load i64, i64* %41, align 8, !range !7
  %42 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %new_layout, i32 0, i32 0
  %_37.0 = load i64, i64* %42, align 8
  %43 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %new_layout, i32 0, i32 1
  %_37.1 = load i64, i64* %43, align 8, !range !7
; call <alloc::alloc::Global as core::alloc::Allocator>::grow
  %44 = call { i8*, i64 } @"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$4grow17h5dbb4d8923be7b7cE"(%"alloc::alloc::Global"* nonnull align 1 %alloc, i8* nonnull %ptr, i64 %_36.0, i64 %_36.1, i64 %_37.0, i64 %_37.1) #11
  store { i8*, i64 } %44, { i8*, i64 }* %memory, align 8
  br label %bb19

bb19:                                             ; preds = %bb18
  br label %bb21

bb22:                                             ; preds = %bb21
  br label %bb24
}

; alloc::raw_vec::finish_grow::{{closure}}
; Function Attrs: inlinehint nounwind nonlazybind
define internal { i64, i64 } @"_ZN5alloc7raw_vec11finish_grow28_$u7b$$u7b$closure$u7d$$u7d$17h521d57bea205a370E"() unnamed_addr #0 {
start:
  %0 = alloca { i64, i64 }, align 8
  %1 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %0, i32 0, i32 1
  store i64 0, i64* %1, align 8
  %2 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %0, i32 0, i32 0
  %3 = load i64, i64* %2, align 8
  %4 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %0, i32 0, i32 1
  %5 = load i64, i64* %4, align 8
  %6 = insertvalue { i64, i64 } undef, i64 %3, 0
  %7 = insertvalue { i64, i64 } %6, i64 %5, 1
  ret { i64, i64 } %7
}

; alloc::raw_vec::finish_grow::{{closure}}
; Function Attrs: inlinehint nounwind nonlazybind
define internal { i64, i64 } @"_ZN5alloc7raw_vec11finish_grow28_$u7b$$u7b$closure$u7d$$u7d$17h46de84ed766927f7E"(i64* align 8 dereferenceable(16) %_1) unnamed_addr #0 {
start:
  %_3 = alloca { i64, i64 }, align 8
  %0 = bitcast i64* %_1 to { i64, i64 }*
  %1 = bitcast i64* %_1 to { i64, i64 }*
  %2 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %1, i32 0, i32 0
  %_4.0 = load i64, i64* %2, align 8
  %3 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %1, i32 0, i32 1
  %_4.1 = load i64, i64* %3, align 8, !range !7
  %4 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %_3, i32 0, i32 0
  store i64 %_4.0, i64* %4, align 8
  %5 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %_3, i32 0, i32 1
  store i64 %_4.1, i64* %5, align 8
  %6 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %_3, i32 0, i32 0
  %7 = load i64, i64* %6, align 8
  %8 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %_3, i32 0, i32 1
  %9 = load i64, i64* %8, align 8
; call <T as core::convert::Into<U>>::into
  %10 = call { i64, i64 } @"_ZN50_$LT$T$u20$as$u20$core..convert..Into$LT$U$GT$$GT$4into17h5cabd2ba38994778E"(i64 %7, i64 %9) #11
  %11 = extractvalue { i64, i64 } %10, 0
  %12 = extractvalue { i64, i64 } %10, 1
  br label %bb1

bb1:                                              ; preds = %start
  %13 = insertvalue { i64, i64 } undef, i64 %11, 0
  %14 = insertvalue { i64, i64 } %13, i64 %12, 1
  ret { i64, i64 } %14
}

; <alloc::raw_vec::RawVec<T,A> as core::ops::drop::Drop>::drop
; Function Attrs: nounwind nonlazybind
define internal void @"_ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17hd56ac955c50c3290E"({ i32*, i64 }* align 8 dereferenceable(16) %self) unnamed_addr #1 {
start:
  %_2 = alloca %"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>", align 8
; call alloc::raw_vec::RawVec<T,A>::current_memory
  call void @"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14current_memory17hf0dd6c0d5cd112ccE"(%"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>"* noalias nocapture sret(%"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>") dereferenceable(24) %_2, { i32*, i64 }* align 8 dereferenceable(16) %self) #11
  br label %bb1

bb1:                                              ; preds = %start
  %0 = bitcast %"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>"* %_2 to {}**
  %1 = load {}*, {}** %0, align 8
  %2 = icmp eq {}* %1, null
  %_4 = select i1 %2, i64 0, i64 1
  %3 = icmp eq i64 %_4, 1
  br i1 %3, label %bb2, label %bb4

bb2:                                              ; preds = %bb1
  %4 = bitcast %"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>"* %_2 to %"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>::Some"*
  %5 = bitcast %"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>::Some"* %4 to { i8*, { i64, i64 } }*
  %6 = bitcast { i8*, { i64, i64 } }* %5 to i8**
  %ptr = load i8*, i8** %6, align 8, !nonnull !3
  %7 = bitcast %"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>"* %_2 to %"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>::Some"*
  %8 = bitcast %"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>::Some"* %7 to { i8*, { i64, i64 } }*
  %9 = getelementptr inbounds { i8*, { i64, i64 } }, { i8*, { i64, i64 } }* %8, i32 0, i32 1
  %10 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %9, i32 0, i32 0
  %layout.0 = load i64, i64* %10, align 8
  %11 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %9, i32 0, i32 1
  %layout.1 = load i64, i64* %11, align 8, !range !7
  %_7 = bitcast { i32*, i64 }* %self to %"alloc::alloc::Global"*
; call <alloc::alloc::Global as core::alloc::Allocator>::deallocate
  call void @"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$10deallocate17hf656adfb5cf9e03dE"(%"alloc::alloc::Global"* nonnull align 1 %_7, i8* nonnull %ptr, i64 %layout.0, i64 %layout.1) #11
  br label %bb3

bb4:                                              ; preds = %bb3, %bb1
  ret void

bb3:                                              ; preds = %bb2
  br label %bb4
}

; alloc::raw_vec::handle_reserve
; Function Attrs: inlinehint nounwind nonlazybind
define internal void @_ZN5alloc7raw_vec14handle_reserve17h7337eab9ee492301E(%"core::result::Result<(), alloc::collections::TryReserveError>"* noalias nocapture dereferenceable(24) %result) unnamed_addr #0 {
start:
  %_3 = alloca %"core::result::Result<(), alloc::collections::TryReserveError>", align 8
  %_2 = alloca %"core::result::Result<(), alloc::collections::TryReserveErrorKind>", align 8
  %0 = bitcast %"core::result::Result<(), alloc::collections::TryReserveError>"* %_3 to i8*
  %1 = bitcast %"core::result::Result<(), alloc::collections::TryReserveError>"* %result to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %0, i8* align 8 %1, i64 24, i1 false)
; call core::result::Result<T,E>::map_err
  call void @"_ZN4core6result19Result$LT$T$C$E$GT$7map_err17h8c0dfe53da6f1e72E"(%"core::result::Result<(), alloc::collections::TryReserveErrorKind>"* noalias nocapture sret(%"core::result::Result<(), alloc::collections::TryReserveErrorKind>") dereferenceable(24) %_2, %"core::result::Result<(), alloc::collections::TryReserveError>"* noalias nocapture dereferenceable(24) %_3) #11
  br label %bb1

bb1:                                              ; preds = %start
  %2 = bitcast %"core::result::Result<(), alloc::collections::TryReserveErrorKind>"* %_2 to i64*
  %_6 = load i64, i64* %2, align 8, !range !6
  switch i64 %_6, label %bb4 [
    i64 0, label %bb2
    i64 1, label %bb3
  ]

bb4:                                              ; preds = %bb3, %bb1
  unreachable

bb2:                                              ; preds = %bb1
  ret void

bb3:                                              ; preds = %bb1
  %3 = bitcast %"core::result::Result<(), alloc::collections::TryReserveErrorKind>"* %_2 to %"core::result::Result<(), alloc::collections::TryReserveErrorKind>::Err"*
  %4 = getelementptr inbounds %"core::result::Result<(), alloc::collections::TryReserveErrorKind>::Err", %"core::result::Result<(), alloc::collections::TryReserveErrorKind>::Err"* %3, i32 0, i32 1
  %5 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %4, i32 0, i32 1
  %6 = load i64, i64* %5, align 8
  %7 = icmp eq i64 %6, 0
  %_5 = select i1 %7, i64 0, i64 1
  switch i64 %_5, label %bb4 [
    i64 0, label %bb5
    i64 1, label %bb6
  ]

bb5:                                              ; preds = %bb3
; call alloc::raw_vec::capacity_overflow
  call void @_ZN5alloc7raw_vec17capacity_overflow17h50ea6dbeac46bafcE() #12
  unreachable

bb6:                                              ; preds = %bb3
  %8 = bitcast %"core::result::Result<(), alloc::collections::TryReserveErrorKind>"* %_2 to %"core::result::Result<(), alloc::collections::TryReserveErrorKind>::Err"*
  %9 = getelementptr inbounds %"core::result::Result<(), alloc::collections::TryReserveErrorKind>::Err", %"core::result::Result<(), alloc::collections::TryReserveErrorKind>::Err"* %8, i32 0, i32 1
  %10 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %9, i32 0, i32 0
  %layout.0 = load i64, i64* %10, align 8
  %11 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %9, i32 0, i32 1
  %layout.1 = load i64, i64* %11, align 8, !range !7
; call alloc::alloc::handle_alloc_error
  call void @_ZN5alloc5alloc18handle_alloc_error17h0ece3c434a8e0eb4E(i64 %layout.0, i64 %layout.1) #12
  unreachable
}

; alloc::raw_vec::handle_reserve::{{closure}}
; Function Attrs: inlinehint nounwind nonlazybind
define internal { i64, i64 } @"_ZN5alloc7raw_vec14handle_reserve28_$u7b$$u7b$closure$u7d$$u7d$17h46cf85d469126324E"(i64 %0, i64 %1) unnamed_addr #0 {
start:
  %e = alloca { i64, i64 }, align 8
  %2 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %e, i32 0, i32 0
  store i64 %0, i64* %2, align 8
  %3 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %e, i32 0, i32 1
  store i64 %1, i64* %3, align 8
; call alloc::collections::TryReserveError::kind
  %4 = call { i64, i64 } @_ZN5alloc11collections15TryReserveError4kind17h96bdb01e2cd62539E({ i64, i64 }* align 8 dereferenceable(16) %e) #11
  %5 = extractvalue { i64, i64 } %4, 0
  %6 = extractvalue { i64, i64 } %4, 1
  br label %bb1

bb1:                                              ; preds = %start
  %7 = insertvalue { i64, i64 } undef, i64 %5, 0
  %8 = insertvalue { i64, i64 } %7, i64 %6, 1
  ret { i64, i64 } %8
}

; alloc::raw_vec::alloc_guard
; Function Attrs: inlinehint nounwind nonlazybind
define internal void @_ZN5alloc7raw_vec11alloc_guard17h6c56d83ecb90f3d6E(%"core::result::Result<(), alloc::collections::TryReserveError>"* noalias nocapture sret(%"core::result::Result<(), alloc::collections::TryReserveError>") dereferenceable(24) %0, i64 %alloc_size) unnamed_addr #0 {
start:
  %_4 = alloca { i64, i64 }, align 8
  br i1 false, label %bb1, label %bb3

bb3:                                              ; preds = %start
  %1 = bitcast %"core::result::Result<(), alloc::collections::TryReserveError>"* %0 to i64*
  store i64 0, i64* %1, align 8
  br label %bb4

bb1:                                              ; preds = %start
  %2 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %_4, i32 0, i32 1
  store i64 0, i64* %2, align 8
  %3 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %_4, i32 0, i32 0
  %4 = load i64, i64* %3, align 8
  %5 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %_4, i32 0, i32 1
  %6 = load i64, i64* %5, align 8
; call <T as core::convert::Into<U>>::into
  %7 = call { i64, i64 } @"_ZN50_$LT$T$u20$as$u20$core..convert..Into$LT$U$GT$$GT$4into17h5cabd2ba38994778E"(i64 %4, i64 %6) #11
  %_3.0 = extractvalue { i64, i64 } %7, 0
  %_3.1 = extractvalue { i64, i64 } %7, 1
  br label %bb2

bb2:                                              ; preds = %bb1
  %8 = bitcast %"core::result::Result<(), alloc::collections::TryReserveError>"* %0 to %"core::result::Result<(), alloc::collections::TryReserveError>::Err"*
  %9 = getelementptr inbounds %"core::result::Result<(), alloc::collections::TryReserveError>::Err", %"core::result::Result<(), alloc::collections::TryReserveError>::Err"* %8, i32 0, i32 1
  %10 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %9, i32 0, i32 0
  store i64 %_3.0, i64* %10, align 8
  %11 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %9, i32 0, i32 1
  store i64 %_3.1, i64* %11, align 8
  %12 = bitcast %"core::result::Result<(), alloc::collections::TryReserveError>"* %0 to i64*
  store i64 1, i64* %12, align 8
  br label %bb4

bb4:                                              ; preds = %bb3, %bb2
  ret void
}

; alloc::alloc::alloc
; Function Attrs: inlinehint nounwind nonlazybind
define internal i8* @_ZN5alloc5alloc5alloc17hbcfcfa8988af1ea6E(i64 %0, i64 %1) unnamed_addr #0 {
start:
  %layout = alloca { i64, i64 }, align 8
  %2 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %layout, i32 0, i32 0
  store i64 %0, i64* %2, align 8
  %3 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %layout, i32 0, i32 1
  store i64 %1, i64* %3, align 8
; call core::alloc::layout::Layout::size
  %_2 = call i64 @_ZN4core5alloc6layout6Layout4size17h081c8035e28220cdE({ i64, i64 }* align 8 dereferenceable(16) %layout) #11
  br label %bb1

bb1:                                              ; preds = %start
; call core::alloc::layout::Layout::align
  %_4 = call i64 @_ZN4core5alloc6layout6Layout5align17he5ae42b9af0cb70fE({ i64, i64 }* align 8 dereferenceable(16) %layout) #11
  br label %bb2

bb2:                                              ; preds = %bb1
  %4 = call i8* @__rust_alloc(i64 %_2, i64 %_4) #11
  br label %bb3

bb3:                                              ; preds = %bb2
  ret i8* %4
}

; alloc::alloc::dealloc
; Function Attrs: inlinehint nounwind nonlazybind
define internal void @_ZN5alloc5alloc7dealloc17hec8886323695bfafE(i8* %ptr, i64 %0, i64 %1) unnamed_addr #0 {
start:
  %layout = alloca { i64, i64 }, align 8
  %2 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %layout, i32 0, i32 0
  store i64 %0, i64* %2, align 8
  %3 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %layout, i32 0, i32 1
  store i64 %1, i64* %3, align 8
; call core::alloc::layout::Layout::size
  %_4 = call i64 @_ZN4core5alloc6layout6Layout4size17h081c8035e28220cdE({ i64, i64 }* align 8 dereferenceable(16) %layout) #11
  br label %bb1

bb1:                                              ; preds = %start
; call core::alloc::layout::Layout::align
  %_6 = call i64 @_ZN4core5alloc6layout6Layout5align17he5ae42b9af0cb70fE({ i64, i64 }* align 8 dereferenceable(16) %layout) #11
  br label %bb2

bb2:                                              ; preds = %bb1
  call void @__rust_dealloc(i8* %ptr, i64 %_4, i64 %_6) #11
  br label %bb3

bb3:                                              ; preds = %bb2
  ret void
}

; alloc::alloc::realloc
; Function Attrs: inlinehint nounwind nonlazybind
define internal i8* @_ZN5alloc5alloc7realloc17h2e6f31f2e696d1baE(i8* %ptr, i64 %0, i64 %1, i64 %new_size) unnamed_addr #0 {
start:
  %layout = alloca { i64, i64 }, align 8
  %2 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %layout, i32 0, i32 0
  store i64 %0, i64* %2, align 8
  %3 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %layout, i32 0, i32 1
  store i64 %1, i64* %3, align 8
; call core::alloc::layout::Layout::size
  %_5 = call i64 @_ZN4core5alloc6layout6Layout4size17h081c8035e28220cdE({ i64, i64 }* align 8 dereferenceable(16) %layout) #11
  br label %bb1

bb1:                                              ; preds = %start
; call core::alloc::layout::Layout::align
  %_7 = call i64 @_ZN4core5alloc6layout6Layout5align17he5ae42b9af0cb70fE({ i64, i64 }* align 8 dereferenceable(16) %layout) #11
  br label %bb2

bb2:                                              ; preds = %bb1
  %4 = call i8* @__rust_realloc(i8* %ptr, i64 %_5, i64 %_7, i64 %new_size) #11
  br label %bb3

bb3:                                              ; preds = %bb2
  ret i8* %4
}

; alloc::alloc::alloc_zeroed
; Function Attrs: inlinehint nounwind nonlazybind
define internal i8* @_ZN5alloc5alloc12alloc_zeroed17h065f53cafb6c489fE(i64 %0, i64 %1) unnamed_addr #0 {
start:
  %layout = alloca { i64, i64 }, align 8
  %2 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %layout, i32 0, i32 0
  store i64 %0, i64* %2, align 8
  %3 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %layout, i32 0, i32 1
  store i64 %1, i64* %3, align 8
; call core::alloc::layout::Layout::size
  %_2 = call i64 @_ZN4core5alloc6layout6Layout4size17h081c8035e28220cdE({ i64, i64 }* align 8 dereferenceable(16) %layout) #11
  br label %bb1

bb1:                                              ; preds = %start
; call core::alloc::layout::Layout::align
  %_4 = call i64 @_ZN4core5alloc6layout6Layout5align17he5ae42b9af0cb70fE({ i64, i64 }* align 8 dereferenceable(16) %layout) #11
  br label %bb2

bb2:                                              ; preds = %bb1
  %4 = call i8* @__rust_alloc_zeroed(i64 %_2, i64 %_4) #11
  br label %bb3

bb3:                                              ; preds = %bb2
  ret i8* %4
}

; alloc::alloc::Global::alloc_impl
; Function Attrs: inlinehint nounwind nonlazybind
define internal { i8*, i64 } @_ZN5alloc5alloc6Global10alloc_impl17h3eff2f8c61e037c0E(%"alloc::alloc::Global"* nonnull align 1 %self, i64 %0, i64 %1, i1 zeroext %zeroed) unnamed_addr #0 {
start:
  %_15 = alloca i8*, align 8
  %raw_ptr = alloca i8*, align 8
  %2 = alloca { i8*, i64 }, align 8
  %layout = alloca { i64, i64 }, align 8
  %3 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %layout, i32 0, i32 0
  store i64 %0, i64* %3, align 8
  %4 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %layout, i32 0, i32 1
  store i64 %1, i64* %4, align 8
; call core::alloc::layout::Layout::size
  %_4 = call i64 @_ZN4core5alloc6layout6Layout4size17h081c8035e28220cdE({ i64, i64 }* align 8 dereferenceable(16) %layout) #11
  br label %bb1

bb1:                                              ; preds = %start
  %5 = icmp eq i64 %_4, 0
  br i1 %5, label %bb3, label %bb2

bb3:                                              ; preds = %bb1
; call core::alloc::layout::Layout::dangling
  %_7 = call nonnull i8* @_ZN4core5alloc6layout6Layout8dangling17h2a26107f9d17f54fE({ i64, i64 }* align 8 dereferenceable(16) %layout) #11
  br label %bb4

bb2:                                              ; preds = %bb1
  br i1 %zeroed, label %bb6, label %bb8

bb8:                                              ; preds = %bb2
  %6 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %layout, i32 0, i32 0
  %_13.0 = load i64, i64* %6, align 8
  %7 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %layout, i32 0, i32 1
  %_13.1 = load i64, i64* %7, align 8, !range !7
; call alloc::alloc::alloc
  %8 = call i8* @_ZN5alloc5alloc5alloc17hbcfcfa8988af1ea6E(i64 %_13.0, i64 %_13.1) #11
  store i8* %8, i8** %raw_ptr, align 8
  br label %bb9

bb6:                                              ; preds = %bb2
  %9 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %layout, i32 0, i32 0
  %_12.0 = load i64, i64* %9, align 8
  %10 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %layout, i32 0, i32 1
  %_12.1 = load i64, i64* %10, align 8, !range !7
; call alloc::alloc::alloc_zeroed
  %11 = call i8* @_ZN5alloc5alloc12alloc_zeroed17h065f53cafb6c489fE(i64 %_12.0, i64 %_12.1) #11
  store i8* %11, i8** %raw_ptr, align 8
  br label %bb7

bb7:                                              ; preds = %bb6
  br label %bb10

bb10:                                             ; preds = %bb9, %bb7
  %_18 = load i8*, i8** %raw_ptr, align 8
; call core::ptr::non_null::NonNull<T>::new
  %_17 = call i8* @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$3new17he37c1fa2ab774037E"(i8* %_18) #11
  br label %bb11

bb9:                                              ; preds = %bb8
  br label %bb10

bb11:                                             ; preds = %bb10
; call core::option::Option<T>::ok_or
  %_16 = call i8* @"_ZN4core6option15Option$LT$T$GT$5ok_or17h4e06d488786df54cE"(i8* %_17) #11
  br label %bb12

bb12:                                             ; preds = %bb11
; call <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
  %12 = call i8* @"_ZN79_$LT$core..result..Result$LT$T$C$E$GT$$u20$as$u20$core..ops..try_trait..Try$GT$6branch17h667d37608974688bE"(i8* %_16) #11
  store i8* %12, i8** %_15, align 8
  br label %bb13

bb13:                                             ; preds = %bb12
  %13 = bitcast i8** %_15 to {}**
  %14 = load {}*, {}** %13, align 8
  %15 = icmp eq {}* %14, null
  %_20 = select i1 %15, i64 1, i64 0
  switch i64 %_20, label %bb15 [
    i64 0, label %bb14
    i64 1, label %bb16
  ]

bb15:                                             ; preds = %bb13
  unreachable

bb14:                                             ; preds = %bb13
  %val = load i8*, i8** %_15, align 8, !nonnull !3
; call core::ptr::non_null::NonNull<[T]>::slice_from_raw_parts
  %16 = call { i8*, i64 } @"_ZN4core3ptr8non_null26NonNull$LT$$u5b$T$u5d$$GT$20slice_from_raw_parts17h8ec9972bf04915b8E"(i8* nonnull %val, i64 %_4) #11
  %_24.0 = extractvalue { i8*, i64 } %16, 0
  %_24.1 = extractvalue { i8*, i64 } %16, 1
  br label %bb18

bb16:                                             ; preds = %bb13
; call <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
  %17 = call { i8*, i64 } @"_ZN153_$LT$core..result..Result$LT$T$C$F$GT$$u20$as$u20$core..ops..try_trait..FromResidual$LT$core..result..Result$LT$core..convert..Infallible$C$E$GT$$GT$$GT$13from_residual17h7338d4240b4d5599E"(%"core::panic::location::Location"* align 8 dereferenceable(24) bitcast (<{ i8*, [16 x i8] }>* @alloc27 to %"core::panic::location::Location"*)) #11
  store { i8*, i64 } %17, { i8*, i64 }* %2, align 8
  br label %bb17

bb17:                                             ; preds = %bb16
  br label %bb20

bb20:                                             ; preds = %bb19, %bb17
  %18 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %2, i32 0, i32 0
  %19 = load i8*, i8** %18, align 8
  %20 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %2, i32 0, i32 1
  %21 = load i64, i64* %20, align 8
  %22 = insertvalue { i8*, i64 } undef, i8* %19, 0
  %23 = insertvalue { i8*, i64 } %22, i64 %21, 1
  ret { i8*, i64 } %23

bb18:                                             ; preds = %bb14
  %24 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %2, i32 0, i32 0
  store i8* %_24.0, i8** %24, align 8
  %25 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %2, i32 0, i32 1
  store i64 %_24.1, i64* %25, align 8
  br label %bb19

bb19:                                             ; preds = %bb5, %bb18
  br label %bb20

bb4:                                              ; preds = %bb3
; call core::ptr::non_null::NonNull<[T]>::slice_from_raw_parts
  %26 = call { i8*, i64 } @"_ZN4core3ptr8non_null26NonNull$LT$$u5b$T$u5d$$GT$20slice_from_raw_parts17h8ec9972bf04915b8E"(i8* nonnull %_7, i64 0) #11
  %_6.0 = extractvalue { i8*, i64 } %26, 0
  %_6.1 = extractvalue { i8*, i64 } %26, 1
  br label %bb5

bb5:                                              ; preds = %bb4
  %27 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %2, i32 0, i32 0
  store i8* %_6.0, i8** %27, align 8
  %28 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %2, i32 0, i32 1
  store i64 %_6.1, i64* %28, align 8
  br label %bb19
}

; alloc::alloc::Global::grow_impl
; Function Attrs: inlinehint nounwind nonlazybind
define internal { i8*, i64 } @_ZN5alloc5alloc6Global9grow_impl17hc6fef0c4789d9a22E(%"alloc::alloc::Global"* nonnull align 1 %self, i8* nonnull %ptr, i64 %0, i64 %1, i64 %2, i64 %3, i1 zeroext %zeroed) unnamed_addr #0 {
start:
  %4 = alloca i8*, align 8
  %_53 = alloca { i8*, i64 }, align 8
  %_31 = alloca i8*, align 8
  %_6 = alloca i64, align 8
  %5 = alloca { i8*, i64 }, align 8
  %new_layout = alloca { i64, i64 }, align 8
  %old_layout = alloca { i64, i64 }, align 8
  %6 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %old_layout, i32 0, i32 0
  store i64 %0, i64* %6, align 8
  %7 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %old_layout, i32 0, i32 1
  store i64 %1, i64* %7, align 8
  %8 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %new_layout, i32 0, i32 0
  store i64 %2, i64* %8, align 8
  %9 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %new_layout, i32 0, i32 1
  store i64 %3, i64* %9, align 8
; call core::alloc::layout::Layout::size
  %10 = call i64 @_ZN4core5alloc6layout6Layout4size17h081c8035e28220cdE({ i64, i64 }* align 8 dereferenceable(16) %old_layout) #11
  store i64 %10, i64* %_6, align 8
  br label %bb1

bb1:                                              ; preds = %start
  %11 = load i64, i64* %_6, align 8
  %12 = icmp eq i64 %11, 0
  br i1 %12, label %bb2, label %bb4

bb2:                                              ; preds = %bb1
  %13 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %new_layout, i32 0, i32 0
  %_9.0 = load i64, i64* %13, align 8
  %14 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %new_layout, i32 0, i32 1
  %_9.1 = load i64, i64* %14, align 8, !range !7
; call alloc::alloc::Global::alloc_impl
  %15 = call { i8*, i64 } @_ZN5alloc5alloc6Global10alloc_impl17h3eff2f8c61e037c0E(%"alloc::alloc::Global"* nonnull align 1 %self, i64 %_9.0, i64 %_9.1, i1 zeroext %zeroed) #11
  store { i8*, i64 } %15, { i8*, i64 }* %5, align 8
  br label %bb3

bb4:                                              ; preds = %bb1
; call core::alloc::layout::Layout::align
  %_14 = call i64 @_ZN4core5alloc6layout6Layout5align17he5ae42b9af0cb70fE({ i64, i64 }* align 8 dereferenceable(16) %old_layout) #11
  br label %bb5

bb5:                                              ; preds = %bb4
; call core::alloc::layout::Layout::align
  %_16 = call i64 @_ZN4core5alloc6layout6Layout5align17he5ae42b9af0cb70fE({ i64, i64 }* align 8 dereferenceable(16) %new_layout) #11
  br label %bb6

bb6:                                              ; preds = %bb5
  %_13 = icmp eq i64 %_14, %_16
  br i1 %_13, label %bb7, label %bb8

bb8:                                              ; preds = %bb6
  %old_size1 = load i64, i64* %_6, align 8
  %16 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %new_layout, i32 0, i32 0
  %_56.0 = load i64, i64* %16, align 8
  %17 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %new_layout, i32 0, i32 1
  %_56.1 = load i64, i64* %17, align 8, !range !7
; call alloc::alloc::Global::alloc_impl
  %18 = call { i8*, i64 } @_ZN5alloc5alloc6Global10alloc_impl17h3eff2f8c61e037c0E(%"alloc::alloc::Global"* nonnull align 1 %self, i64 %_56.0, i64 %_56.1, i1 zeroext %zeroed) #11
  %_54.0 = extractvalue { i8*, i64 } %18, 0
  %_54.1 = extractvalue { i8*, i64 } %18, 1
  br label %bb26

bb7:                                              ; preds = %bb6
  %old_size = load i64, i64* %_6, align 8
; call core::alloc::layout::Layout::size
  %new_size = call i64 @_ZN4core5alloc6layout6Layout4size17h081c8035e28220cdE({ i64, i64 }* align 8 dereferenceable(16) %new_layout) #11
  br label %bb9

bb9:                                              ; preds = %bb7
; call core::alloc::layout::Layout::size
  %_23 = call i64 @_ZN4core5alloc6layout6Layout4size17h081c8035e28220cdE({ i64, i64 }* align 8 dereferenceable(16) %old_layout) #11
  br label %bb10

bb10:                                             ; preds = %bb9
  %_21 = icmp uge i64 %new_size, %_23
  call void @llvm.assume(i1 %_21)
  br label %bb11

bb11:                                             ; preds = %bb10
; call core::ptr::non_null::NonNull<T>::as_ptr
  %_26 = call i8* @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$6as_ptr17h0aa33435f2c3dbf6E"(i8* nonnull %ptr) #11
  br label %bb12

bb12:                                             ; preds = %bb11
  %19 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %old_layout, i32 0, i32 0
  %_28.0 = load i64, i64* %19, align 8
  %20 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %old_layout, i32 0, i32 1
  %_28.1 = load i64, i64* %20, align 8, !range !7
; call alloc::alloc::realloc
  %raw_ptr = call i8* @_ZN5alloc5alloc7realloc17h2e6f31f2e696d1baE(i8* %_26, i64 %_28.0, i64 %_28.1, i64 %new_size) #11
  br label %bb13

bb13:                                             ; preds = %bb12
; call core::ptr::non_null::NonNull<T>::new
  %_33 = call i8* @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$3new17he37c1fa2ab774037E"(i8* %raw_ptr) #11
  br label %bb14

bb14:                                             ; preds = %bb13
; call core::option::Option<T>::ok_or
  %_32 = call i8* @"_ZN4core6option15Option$LT$T$GT$5ok_or17h4e06d488786df54cE"(i8* %_33) #11
  br label %bb15

bb15:                                             ; preds = %bb14
; call <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
  %21 = call i8* @"_ZN79_$LT$core..result..Result$LT$T$C$E$GT$$u20$as$u20$core..ops..try_trait..Try$GT$6branch17h667d37608974688bE"(i8* %_32) #11
  store i8* %21, i8** %_31, align 8
  br label %bb16

bb16:                                             ; preds = %bb15
  %22 = bitcast i8** %_31 to {}**
  %23 = load {}*, {}** %22, align 8
  %24 = icmp eq {}* %23, null
  %_36 = select i1 %24, i64 1, i64 0
  switch i64 %_36, label %bb18 [
    i64 0, label %bb17
    i64 1, label %bb19
  ]

bb18:                                             ; preds = %bb16
  unreachable

bb17:                                             ; preds = %bb16
  %val = load i8*, i8** %_31, align 8, !nonnull !3
  br i1 %zeroed, label %bb21, label %bb24

bb19:                                             ; preds = %bb16
; call <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
  %25 = call { i8*, i64 } @"_ZN153_$LT$core..result..Result$LT$T$C$F$GT$$u20$as$u20$core..ops..try_trait..FromResidual$LT$core..result..Result$LT$core..convert..Infallible$C$E$GT$$GT$$GT$13from_residual17h7338d4240b4d5599E"(%"core::panic::location::Location"* align 8 dereferenceable(24) bitcast (<{ i8*, [16 x i8] }>* @alloc29 to %"core::panic::location::Location"*)) #11
  store { i8*, i64 } %25, { i8*, i64 }* %5, align 8
  br label %bb20

bb20:                                             ; preds = %bb19
  br label %bb37

bb37:                                             ; preds = %bb31, %bb20
  br label %bb38

bb24:                                             ; preds = %bb23, %bb17
; call core::ptr::non_null::NonNull<[T]>::slice_from_raw_parts
  %26 = call { i8*, i64 } @"_ZN4core3ptr8non_null26NonNull$LT$$u5b$T$u5d$$GT$20slice_from_raw_parts17h8ec9972bf04915b8E"(i8* nonnull %val, i64 %new_size) #11
  %_48.0 = extractvalue { i8*, i64 } %26, 0
  %_48.1 = extractvalue { i8*, i64 } %26, 1
  br label %bb25

bb21:                                             ; preds = %bb17
  %27 = getelementptr inbounds i8, i8* %raw_ptr, i64 %old_size
  store i8* %27, i8** %4, align 8
  %_3.i.i = load i8*, i8** %4, align 8
  br label %bb22

bb22:                                             ; preds = %bb21
  %_45 = sub i64 %new_size, %old_size
; call core::intrinsics::write_bytes
  call void @_ZN4core10intrinsics11write_bytes17h04f23dd5d4d322c4E(i8* %_3.i.i, i8 0, i64 %_45) #11
  br label %bb23

bb23:                                             ; preds = %bb22
  br label %bb24

bb25:                                             ; preds = %bb24
  %28 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %5, i32 0, i32 0
  store i8* %_48.0, i8** %28, align 8
  %29 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %5, i32 0, i32 1
  store i64 %_48.1, i64* %29, align 8
  br label %bb36

bb36:                                             ; preds = %bb3, %bb35, %bb25
  br label %bb38

bb26:                                             ; preds = %bb8
; call <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
  %30 = call { i8*, i64 } @"_ZN79_$LT$core..result..Result$LT$T$C$E$GT$$u20$as$u20$core..ops..try_trait..Try$GT$6branch17h1e85c060f3c552f7E"(i8* %_54.0, i64 %_54.1) #11
  store { i8*, i64 } %30, { i8*, i64 }* %_53, align 8
  br label %bb27

bb27:                                             ; preds = %bb26
  %31 = bitcast { i8*, i64 }* %_53 to {}**
  %32 = load {}*, {}** %31, align 8
  %33 = icmp eq {}* %32, null
  %_58 = select i1 %33, i64 1, i64 0
  switch i64 %_58, label %bb29 [
    i64 0, label %bb28
    i64 1, label %bb30
  ]

bb29:                                             ; preds = %bb27
  unreachable

bb28:                                             ; preds = %bb27
  %34 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %_53, i32 0, i32 0
  %val.0 = load i8*, i8** %34, align 8, !nonnull !3
  %35 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %_53, i32 0, i32 1
  %val.1 = load i64, i64* %35, align 8
; call core::ptr::non_null::NonNull<T>::as_ptr
  %_64 = call i8* @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$6as_ptr17h0aa33435f2c3dbf6E"(i8* nonnull %ptr) #11
  br label %bb32

bb30:                                             ; preds = %bb27
; call <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
  %36 = call { i8*, i64 } @"_ZN153_$LT$core..result..Result$LT$T$C$F$GT$$u20$as$u20$core..ops..try_trait..FromResidual$LT$core..result..Result$LT$core..convert..Infallible$C$E$GT$$GT$$GT$13from_residual17h7338d4240b4d5599E"(%"core::panic::location::Location"* align 8 dereferenceable(24) bitcast (<{ i8*, [16 x i8] }>* @alloc31 to %"core::panic::location::Location"*)) #11
  store { i8*, i64 } %36, { i8*, i64 }* %5, align 8
  br label %bb31

bb31:                                             ; preds = %bb30
  br label %bb37

bb38:                                             ; preds = %bb36, %bb37
  %37 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %5, i32 0, i32 0
  %38 = load i8*, i8** %37, align 8
  %39 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %5, i32 0, i32 1
  %40 = load i64, i64* %39, align 8
  %41 = insertvalue { i8*, i64 } undef, i8* %38, 0
  %42 = insertvalue { i8*, i64 } %41, i64 %40, 1
  ret { i8*, i64 } %42

bb32:                                             ; preds = %bb28
; call core::ptr::non_null::NonNull<[T]>::as_mut_ptr
  %_66 = call i8* @"_ZN4core3ptr8non_null26NonNull$LT$$u5b$T$u5d$$GT$10as_mut_ptr17h8c85829ce4dc4d36E"(i8* nonnull %val.0, i64 %val.1) #11
  br label %bb33

bb33:                                             ; preds = %bb32
; call core::intrinsics::copy_nonoverlapping
  call void @_ZN4core10intrinsics19copy_nonoverlapping17h6400f07f7115ca25E(i8* %_64, i8* %_66, i64 %old_size1) #11
  br label %bb34

bb34:                                             ; preds = %bb33
  %43 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %old_layout, i32 0, i32 0
  %_72.0 = load i64, i64* %43, align 8
  %44 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %old_layout, i32 0, i32 1
  %_72.1 = load i64, i64* %44, align 8, !range !7
; call <alloc::alloc::Global as core::alloc::Allocator>::deallocate
  call void @"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$10deallocate17hf656adfb5cf9e03dE"(%"alloc::alloc::Global"* nonnull align 1 %self, i8* nonnull %ptr, i64 %_72.0, i64 %_72.1) #11
  br label %bb35

bb35:                                             ; preds = %bb34
  %45 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %5, i32 0, i32 0
  store i8* %val.0, i8** %45, align 8
  %46 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %5, i32 0, i32 1
  store i64 %val.1, i64* %46, align 8
  br label %bb36

bb3:                                              ; preds = %bb2
  br label %bb36
}

; <alloc::alloc::Global as core::alloc::Allocator>::allocate
; Function Attrs: inlinehint nounwind nonlazybind
define internal { i8*, i64 } @"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h5ca58336f2900dc9E"(%"alloc::alloc::Global"* nonnull align 1 %self, i64 %layout.0, i64 %layout.1) unnamed_addr #0 {
start:
; call alloc::alloc::Global::alloc_impl
  %0 = call { i8*, i64 } @_ZN5alloc5alloc6Global10alloc_impl17h3eff2f8c61e037c0E(%"alloc::alloc::Global"* nonnull align 1 %self, i64 %layout.0, i64 %layout.1, i1 zeroext false) #11
  %1 = extractvalue { i8*, i64 } %0, 0
  %2 = extractvalue { i8*, i64 } %0, 1
  br label %bb1

bb1:                                              ; preds = %start
  %3 = insertvalue { i8*, i64 } undef, i8* %1, 0
  %4 = insertvalue { i8*, i64 } %3, i64 %2, 1
  ret { i8*, i64 } %4
}

; <alloc::alloc::Global as core::alloc::Allocator>::deallocate
; Function Attrs: inlinehint nounwind nonlazybind
define internal void @"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$10deallocate17hf656adfb5cf9e03dE"(%"alloc::alloc::Global"* nonnull align 1 %self, i8* nonnull %ptr, i64 %0, i64 %1) unnamed_addr #0 {
start:
  %layout = alloca { i64, i64 }, align 8
  %2 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %layout, i32 0, i32 0
  store i64 %0, i64* %2, align 8
  %3 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %layout, i32 0, i32 1
  store i64 %1, i64* %3, align 8
; call core::alloc::layout::Layout::size
  %_4 = call i64 @_ZN4core5alloc6layout6Layout4size17h081c8035e28220cdE({ i64, i64 }* align 8 dereferenceable(16) %layout) #11
  br label %bb1

bb1:                                              ; preds = %start
  %4 = icmp eq i64 %_4, 0
  br i1 %4, label %bb5, label %bb2

bb5:                                              ; preds = %bb1
  br label %bb6

bb2:                                              ; preds = %bb1
; call core::ptr::non_null::NonNull<T>::as_ptr
  %_6 = call i8* @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$6as_ptr17h0aa33435f2c3dbf6E"(i8* nonnull %ptr) #11
  br label %bb3

bb3:                                              ; preds = %bb2
  %5 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %layout, i32 0, i32 0
  %_8.0 = load i64, i64* %5, align 8
  %6 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %layout, i32 0, i32 1
  %_8.1 = load i64, i64* %6, align 8, !range !7
; call alloc::alloc::dealloc
  call void @_ZN5alloc5alloc7dealloc17hec8886323695bfafE(i8* %_6, i64 %_8.0, i64 %_8.1) #11
  br label %bb4

bb4:                                              ; preds = %bb3
  br label %bb6

bb6:                                              ; preds = %bb5, %bb4
  ret void
}

; <alloc::alloc::Global as core::alloc::Allocator>::grow
; Function Attrs: inlinehint nounwind nonlazybind
define internal { i8*, i64 } @"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$4grow17h5dbb4d8923be7b7cE"(%"alloc::alloc::Global"* nonnull align 1 %self, i8* nonnull %ptr, i64 %old_layout.0, i64 %old_layout.1, i64 %new_layout.0, i64 %new_layout.1) unnamed_addr #0 {
start:
; call alloc::alloc::Global::grow_impl
  %0 = call { i8*, i64 } @_ZN5alloc5alloc6Global9grow_impl17hc6fef0c4789d9a22E(%"alloc::alloc::Global"* nonnull align 1 %self, i8* nonnull %ptr, i64 %old_layout.0, i64 %old_layout.1, i64 %new_layout.0, i64 %new_layout.1, i1 zeroext false) #11
  %1 = extractvalue { i8*, i64 } %0, 0
  %2 = extractvalue { i8*, i64 } %0, 1
  br label %bb1

bb1:                                              ; preds = %start
  %3 = insertvalue { i8*, i64 } undef, i8* %1, 0
  %4 = insertvalue { i8*, i64 } %3, i64 %2, 1
  ret { i8*, i64 } %4
}

; core::hint::black_box
; Function Attrs: inlinehint nounwind nonlazybind
define internal void @_ZN4core4hint9black_box17hc4074ee890bbb658E() unnamed_addr #0 {
start:
  call void asm sideeffect "", "r,~{memory}"({}* undef), !srcloc !8
  br label %bb1

bb1:                                              ; preds = %start
  ret void
}

; core::intrinsics::copy_nonoverlapping
; Function Attrs: inlinehint nounwind nonlazybind
define internal void @_ZN4core10intrinsics19copy_nonoverlapping17h6400f07f7115ca25E(i8* %src, i8* %dst, i64 %count) unnamed_addr #0 {
start:
  %0 = mul i64 %count, 1
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %dst, i8* align 1 %src, i64 %0, i1 false)
  ret void
}

; core::intrinsics::write_bytes
; Function Attrs: inlinehint nounwind nonlazybind
define internal void @_ZN4core10intrinsics11write_bytes17h04f23dd5d4d322c4E(i8* %dst, i8 %val, i64 %count) unnamed_addr #0 {
start:
  %0 = mul i64 1, %count
  call void @llvm.memset.p0i8.i64(i8* align 1 %dst, i8 %val, i64 %0, i1 false)
  br label %bb1

bb1:                                              ; preds = %start
  ret void
}

; core::ptr::metadata::from_raw_parts_mut
; Function Attrs: inlinehint nounwind nonlazybind
define internal { [0 x i8]*, i64 } @_ZN4core3ptr8metadata18from_raw_parts_mut17h310fd7da80926ceaE({}* %data_address, i64 %metadata) unnamed_addr #0 {
start:
  %_4 = alloca { i8*, i64 }, align 8
  %_3 = alloca %"core::ptr::metadata::PtrRepr<[u8]>", align 8
  %0 = bitcast { i8*, i64 }* %_4 to {}**
  store {}* %data_address, {}** %0, align 8
  %1 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %_4, i32 0, i32 1
  store i64 %metadata, i64* %1, align 8
  %2 = bitcast %"core::ptr::metadata::PtrRepr<[u8]>"* %_3 to { i8*, i64 }*
  %3 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %_4, i32 0, i32 0
  %4 = load i8*, i8** %3, align 8
  %5 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %_4, i32 0, i32 1
  %6 = load i64, i64* %5, align 8
  %7 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %2, i32 0, i32 0
  store i8* %4, i8** %7, align 8
  %8 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %2, i32 0, i32 1
  store i64 %6, i64* %8, align 8
  %9 = bitcast %"core::ptr::metadata::PtrRepr<[u8]>"* %_3 to { [0 x i8]*, i64 }*
  %10 = getelementptr inbounds { [0 x i8]*, i64 }, { [0 x i8]*, i64 }* %9, i32 0, i32 0
  %11 = load [0 x i8]*, [0 x i8]** %10, align 8
  %12 = getelementptr inbounds { [0 x i8]*, i64 }, { [0 x i8]*, i64 }* %9, i32 0, i32 1
  %13 = load i64, i64* %12, align 8
  %14 = insertvalue { [0 x i8]*, i64 } undef, [0 x i8]* %11, 0
  %15 = insertvalue { [0 x i8]*, i64 } %14, i64 %13, 1
  ret { [0 x i8]*, i64 } %15
}

; core::ptr::metadata::from_raw_parts_mut
; Function Attrs: inlinehint nounwind nonlazybind
define internal { [0 x i32]*, i64 } @_ZN4core3ptr8metadata18from_raw_parts_mut17he61391023ac9f995E({}* %data_address, i64 %metadata) unnamed_addr #0 {
start:
  %_4 = alloca { i8*, i64 }, align 8
  %_3 = alloca %"core::ptr::metadata::PtrRepr<[i32]>", align 8
  %0 = bitcast { i8*, i64 }* %_4 to {}**
  store {}* %data_address, {}** %0, align 8
  %1 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %_4, i32 0, i32 1
  store i64 %metadata, i64* %1, align 8
  %2 = bitcast %"core::ptr::metadata::PtrRepr<[i32]>"* %_3 to { i8*, i64 }*
  %3 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %_4, i32 0, i32 0
  %4 = load i8*, i8** %3, align 8
  %5 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %_4, i32 0, i32 1
  %6 = load i64, i64* %5, align 8
  %7 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %2, i32 0, i32 0
  store i8* %4, i8** %7, align 8
  %8 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %2, i32 0, i32 1
  store i64 %6, i64* %8, align 8
  %9 = bitcast %"core::ptr::metadata::PtrRepr<[i32]>"* %_3 to { [0 x i32]*, i64 }*
  %10 = getelementptr inbounds { [0 x i32]*, i64 }, { [0 x i32]*, i64 }* %9, i32 0, i32 0
  %11 = load [0 x i32]*, [0 x i32]** %10, align 8
  %12 = getelementptr inbounds { [0 x i32]*, i64 }, { [0 x i32]*, i64 }* %9, i32 0, i32 1
  %13 = load i64, i64* %12, align 8
  %14 = insertvalue { [0 x i32]*, i64 } undef, [0 x i32]* %11, 0
  %15 = insertvalue { [0 x i32]*, i64 } %14, i64 %13, 1
  ret { [0 x i32]*, i64 } %15
}

; core::ptr::non_null::NonNull<T>::new_unchecked
; Function Attrs: inlinehint nounwind nonlazybind
define internal { i8*, i64 } @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked17h0d978d6a1eb64ab3E"([0 x i8]* %ptr.0, i64 %ptr.1) unnamed_addr #0 {
start:
  %0 = alloca { i8*, i64 }, align 8
  %1 = bitcast { i8*, i64 }* %0 to { [0 x i8]*, i64 }*
  %2 = getelementptr inbounds { [0 x i8]*, i64 }, { [0 x i8]*, i64 }* %1, i32 0, i32 0
  store [0 x i8]* %ptr.0, [0 x i8]** %2, align 8
  %3 = getelementptr inbounds { [0 x i8]*, i64 }, { [0 x i8]*, i64 }* %1, i32 0, i32 1
  store i64 %ptr.1, i64* %3, align 8
  %4 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %0, i32 0, i32 0
  %5 = load i8*, i8** %4, align 8, !nonnull !3
  %6 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %0, i32 0, i32 1
  %7 = load i64, i64* %6, align 8
  %8 = insertvalue { i8*, i64 } undef, i8* %5, 0
  %9 = insertvalue { i8*, i64 } %8, i64 %7, 1
  ret { i8*, i64 } %9
}

; core::ptr::non_null::NonNull<T>::new_unchecked
; Function Attrs: inlinehint nounwind nonlazybind
define internal nonnull i32* @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked17h835b34f988bae1c5E"(i32* %ptr) unnamed_addr #0 {
start:
  %0 = alloca i32*, align 8
  store i32* %ptr, i32** %0, align 8
  %1 = load i32*, i32** %0, align 8, !nonnull !3
  ret i32* %1
}

; core::ptr::non_null::NonNull<T>::new_unchecked
; Function Attrs: inlinehint nounwind nonlazybind
define internal nonnull i8* @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked17hbce36cd865745a6dE"(i8* %ptr) unnamed_addr #0 {
start:
  %0 = alloca i8*, align 8
  store i8* %ptr, i8** %0, align 8
  %1 = load i8*, i8** %0, align 8, !nonnull !3
  ret i8* %1
}

; core::ptr::non_null::NonNull<T>::new
; Function Attrs: inlinehint nounwind nonlazybind
define internal i8* @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$3new17he37c1fa2ab774037E"(i8* %ptr) unnamed_addr #0 {
start:
  %0 = alloca i8*, align 8
; call core::ptr::mut_ptr::<impl *mut T>::is_null
  %_3 = call zeroext i1 @"_ZN4core3ptr7mut_ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$7is_null17h8134c5e722b4f49fE"(i8* %ptr) #11
  br label %bb1

bb1:                                              ; preds = %start
  %_2 = xor i1 %_3, true
  br i1 %_2, label %bb2, label %bb4

bb4:                                              ; preds = %bb1
  %1 = bitcast i8** %0 to {}**
  store {}* null, {}** %1, align 8
  br label %bb5

bb2:                                              ; preds = %bb1
; call core::ptr::non_null::NonNull<T>::new_unchecked
  %_5 = call nonnull i8* @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked17hbce36cd865745a6dE"(i8* %ptr) #11
  br label %bb3

bb3:                                              ; preds = %bb2
  store i8* %_5, i8** %0, align 8
  br label %bb5

bb5:                                              ; preds = %bb4, %bb3
  %2 = load i8*, i8** %0, align 8
  ret i8* %2
}

; core::ptr::non_null::NonNull<T>::as_ptr
; Function Attrs: inlinehint nounwind nonlazybind
define internal i8* @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$6as_ptr17h0aa33435f2c3dbf6E"(i8* nonnull %self) unnamed_addr #0 {
start:
  ret i8* %self
}

; core::ptr::non_null::NonNull<T>::as_ptr
; Function Attrs: inlinehint nounwind nonlazybind
define internal i32* @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$6as_ptr17h3765aa681de6eda6E"(i32* nonnull %self) unnamed_addr #0 {
start:
  ret i32* %self
}

; core::ptr::non_null::NonNull<T>::as_ptr
; Function Attrs: inlinehint nounwind nonlazybind
define internal { [0 x i8]*, i64 } @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$6as_ptr17hff4e157c51219da4E"(i8* nonnull %self.0, i64 %self.1) unnamed_addr #0 {
start:
  %_2.0 = bitcast i8* %self.0 to [0 x i8]*
  %0 = insertvalue { [0 x i8]*, i64 } undef, [0 x i8]* %_2.0, 0
  %1 = insertvalue { [0 x i8]*, i64 } %0, i64 %self.1, 1
  ret { [0 x i8]*, i64 } %1
}

; core::ptr::non_null::NonNull<T>::cast
; Function Attrs: inlinehint nounwind nonlazybind
define internal nonnull i32* @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$4cast17hacb27b780c73e81cE"(i8* nonnull %self.0, i64 %self.1) unnamed_addr #0 {
start:
; call core::ptr::non_null::NonNull<T>::as_ptr
  %0 = call { [0 x i8]*, i64 } @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$6as_ptr17hff4e157c51219da4E"(i8* nonnull %self.0, i64 %self.1) #11
  %_3.0 = extractvalue { [0 x i8]*, i64 } %0, 0
  %_3.1 = extractvalue { [0 x i8]*, i64 } %0, 1
  br label %bb1

bb1:                                              ; preds = %start
  %_2 = bitcast [0 x i8]* %_3.0 to i32*
; call core::ptr::non_null::NonNull<T>::new_unchecked
  %1 = call nonnull i32* @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked17h835b34f988bae1c5E"(i32* %_2) #11
  br label %bb2

bb2:                                              ; preds = %bb1
  ret i32* %1
}

; core::ptr::non_null::NonNull<[T]>::slice_from_raw_parts
; Function Attrs: inlinehint nounwind nonlazybind
define internal { i8*, i64 } @"_ZN4core3ptr8non_null26NonNull$LT$$u5b$T$u5d$$GT$20slice_from_raw_parts17h8ec9972bf04915b8E"(i8* nonnull %data, i64 %len) unnamed_addr #0 {
start:
; call core::ptr::non_null::NonNull<T>::as_ptr
  %_4 = call i8* @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$6as_ptr17h0aa33435f2c3dbf6E"(i8* nonnull %data) #11
  br label %bb1

bb1:                                              ; preds = %start
; call core::ptr::slice_from_raw_parts_mut
  %0 = call { [0 x i8]*, i64 } @_ZN4core3ptr24slice_from_raw_parts_mut17h19aa777c2c070af4E(i8* %_4, i64 %len) #11
  %_3.0 = extractvalue { [0 x i8]*, i64 } %0, 0
  %_3.1 = extractvalue { [0 x i8]*, i64 } %0, 1
  br label %bb2

bb2:                                              ; preds = %bb1
; call core::ptr::non_null::NonNull<T>::new_unchecked
  %1 = call { i8*, i64 } @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked17h0d978d6a1eb64ab3E"([0 x i8]* %_3.0, i64 %_3.1) #11
  %2 = extractvalue { i8*, i64 } %1, 0
  %3 = extractvalue { i8*, i64 } %1, 1
  br label %bb3

bb3:                                              ; preds = %bb2
  %4 = insertvalue { i8*, i64 } undef, i8* %2, 0
  %5 = insertvalue { i8*, i64 } %4, i64 %3, 1
  ret { i8*, i64 } %5
}

; core::ptr::non_null::NonNull<[T]>::as_non_null_ptr
; Function Attrs: inlinehint nounwind nonlazybind
define internal nonnull i8* @"_ZN4core3ptr8non_null26NonNull$LT$$u5b$T$u5d$$GT$15as_non_null_ptr17h3ff897adc57d32adE"(i8* nonnull %self.0, i64 %self.1) unnamed_addr #0 {
start:
; call core::ptr::non_null::NonNull<T>::as_ptr
  %0 = call { [0 x i8]*, i64 } @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$6as_ptr17hff4e157c51219da4E"(i8* nonnull %self.0, i64 %self.1) #11
  %_3.0 = extractvalue { [0 x i8]*, i64 } %0, 0
  %_3.1 = extractvalue { [0 x i8]*, i64 } %0, 1
  br label %bb1

bb1:                                              ; preds = %start
  %1 = bitcast [0 x i8]* %_3.0 to i8*
  br label %bb2

bb2:                                              ; preds = %bb1
; call core::ptr::non_null::NonNull<T>::new_unchecked
  %2 = call nonnull i8* @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked17hbce36cd865745a6dE"(i8* %1) #11
  br label %bb3

bb3:                                              ; preds = %bb2
  ret i8* %2
}

; core::ptr::non_null::NonNull<[T]>::as_mut_ptr
; Function Attrs: inlinehint nounwind nonlazybind
define internal i8* @"_ZN4core3ptr8non_null26NonNull$LT$$u5b$T$u5d$$GT$10as_mut_ptr17h8c85829ce4dc4d36E"(i8* nonnull %self.0, i64 %self.1) unnamed_addr #0 {
start:
; call core::ptr::non_null::NonNull<[T]>::as_non_null_ptr
  %_2 = call nonnull i8* @"_ZN4core3ptr8non_null26NonNull$LT$$u5b$T$u5d$$GT$15as_non_null_ptr17h3ff897adc57d32adE"(i8* nonnull %self.0, i64 %self.1) #11
  br label %bb1

bb1:                                              ; preds = %start
; call core::ptr::non_null::NonNull<T>::as_ptr
  %0 = call i8* @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$6as_ptr17h0aa33435f2c3dbf6E"(i8* nonnull %_2) #11
  br label %bb2

bb2:                                              ; preds = %bb1
  ret i8* %0
}

; <core::ptr::non_null::NonNull<T> as core::convert::From<core::ptr::unique::Unique<T>>>::from
; Function Attrs: inlinehint nounwind nonlazybind
define internal nonnull i8* @"_ZN119_$LT$core..ptr..non_null..NonNull$LT$T$GT$$u20$as$u20$core..convert..From$LT$core..ptr..unique..Unique$LT$T$GT$$GT$$GT$4from17ha3fdac2b0aed3b08E"(i8* nonnull %unique) unnamed_addr #0 {
start:
; call core::ptr::unique::Unique<T>::as_ptr
  %_2 = call i8* @"_ZN4core3ptr6unique15Unique$LT$T$GT$6as_ptr17ha27999923c5937c3E"(i8* nonnull %unique) #11
  br label %bb1

bb1:                                              ; preds = %start
; call core::ptr::non_null::NonNull<T>::new_unchecked
  %0 = call nonnull i8* @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked17hbce36cd865745a6dE"(i8* %_2) #11
  br label %bb2

bb2:                                              ; preds = %bb1
  ret i8* %0
}

; core::ptr::unique::Unique<T>::new_unchecked
; Function Attrs: inlinehint nounwind nonlazybind
define internal nonnull i8* @"_ZN4core3ptr6unique15Unique$LT$T$GT$13new_unchecked17hd8ac69cc40a7f242E"(i8* %ptr) unnamed_addr #0 {
start:
  %0 = alloca i8*, align 8
  store i8* %ptr, i8** %0, align 8
  %1 = load i8*, i8** %0, align 8, !nonnull !3
  ret i8* %1
}

; core::ptr::unique::Unique<T>::new_unchecked
; Function Attrs: inlinehint nounwind nonlazybind
define internal nonnull i32* @"_ZN4core3ptr6unique15Unique$LT$T$GT$13new_unchecked17he969dda07b0c4bceE"(i32* %ptr) unnamed_addr #0 {
start:
  %0 = alloca i32*, align 8
  store i32* %ptr, i32** %0, align 8
  %1 = load i32*, i32** %0, align 8, !nonnull !3
  ret i32* %1
}

; core::ptr::unique::Unique<T>::as_ptr
; Function Attrs: inlinehint nounwind nonlazybind
define internal i8* @"_ZN4core3ptr6unique15Unique$LT$T$GT$6as_ptr17ha27999923c5937c3E"(i8* nonnull %self) unnamed_addr #0 {
start:
  ret i8* %self
}

; core::ptr::unique::Unique<T>::as_ptr
; Function Attrs: inlinehint nounwind nonlazybind
define internal i32* @"_ZN4core3ptr6unique15Unique$LT$T$GT$6as_ptr17hf27a8920f282d9a0E"(i32* nonnull %self) unnamed_addr #0 {
start:
  ret i32* %self
}

; core::ptr::unique::Unique<T>::cast
; Function Attrs: inlinehint nounwind nonlazybind
define internal nonnull i8* @"_ZN4core3ptr6unique15Unique$LT$T$GT$4cast17h42655d37bf6d0057E"(i32* nonnull %self) unnamed_addr #0 {
start:
; call core::ptr::unique::Unique<T>::as_ptr
  %_3 = call i32* @"_ZN4core3ptr6unique15Unique$LT$T$GT$6as_ptr17hf27a8920f282d9a0E"(i32* nonnull %self) #11
  br label %bb1

bb1:                                              ; preds = %start
  %_2 = bitcast i32* %_3 to i8*
; call core::ptr::unique::Unique<T>::new_unchecked
  %0 = call nonnull i8* @"_ZN4core3ptr6unique15Unique$LT$T$GT$13new_unchecked17hd8ac69cc40a7f242E"(i8* %_2) #11
  br label %bb2

bb2:                                              ; preds = %bb1
  ret i8* %0
}

; core::ptr::mut_ptr::<impl *mut T>::is_null
; Function Attrs: inlinehint nounwind nonlazybind
define internal zeroext i1 @"_ZN4core3ptr7mut_ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$7is_null17h8134c5e722b4f49fE"(i8* %self) unnamed_addr #0 {
start:
  br label %bb1

bb1:                                              ; preds = %start
; call core::ptr::mut_ptr::<impl *mut T>::guaranteed_eq
  %0 = call zeroext i1 @"_ZN4core3ptr7mut_ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$13guaranteed_eq17ha27b31272fa19a3fE"(i8* %self, i8* null) #11
  br label %bb2

bb2:                                              ; preds = %bb1
  ret i1 %0
}

; core::ptr::mut_ptr::<impl *mut T>::is_null
; Function Attrs: inlinehint nounwind nonlazybind
define internal zeroext i1 @"_ZN4core3ptr7mut_ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$7is_null17h826eb03ba8c10c72E"(i32* %self) unnamed_addr #0 {
start:
  %_2 = bitcast i32* %self to i8*
  br label %bb1

bb1:                                              ; preds = %start
; call core::ptr::mut_ptr::<impl *mut T>::guaranteed_eq
  %0 = call zeroext i1 @"_ZN4core3ptr7mut_ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$13guaranteed_eq17ha27b31272fa19a3fE"(i8* %_2, i8* null) #11
  br label %bb2

bb2:                                              ; preds = %bb1
  ret i1 %0
}

; core::ptr::mut_ptr::<impl *mut T>::guaranteed_eq
; Function Attrs: inlinehint nounwind nonlazybind
define internal zeroext i1 @"_ZN4core3ptr7mut_ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$13guaranteed_eq17ha27b31272fa19a3fE"(i8* %self, i8* %other) unnamed_addr #0 {
start:
  %0 = alloca i8, align 1
  %1 = icmp eq i8* %self, %other
  %2 = zext i1 %1 to i8
  store i8 %2, i8* %0, align 1
  %3 = load i8, i8* %0, align 1, !range !5
  %4 = trunc i8 %3 to i1
  br label %bb1

bb1:                                              ; preds = %start
  ret i1 %4
}

; core::ptr::slice_from_raw_parts_mut
; Function Attrs: inlinehint nounwind nonlazybind
define internal { [0 x i8]*, i64 } @_ZN4core3ptr24slice_from_raw_parts_mut17h19aa777c2c070af4E(i8* %data, i64 %len) unnamed_addr #0 {
start:
  %0 = bitcast i8* %data to {}*
  br label %bb1

bb1:                                              ; preds = %start
; call core::ptr::metadata::from_raw_parts_mut
  %1 = call { [0 x i8]*, i64 } @_ZN4core3ptr8metadata18from_raw_parts_mut17h310fd7da80926ceaE({}* %0, i64 %len) #11
  %2 = extractvalue { [0 x i8]*, i64 } %1, 0
  %3 = extractvalue { [0 x i8]*, i64 } %1, 1
  br label %bb2

bb2:                                              ; preds = %bb1
  %4 = insertvalue { [0 x i8]*, i64 } undef, [0 x i8]* %2, 0
  %5 = insertvalue { [0 x i8]*, i64 } %4, i64 %3, 1
  ret { [0 x i8]*, i64 } %5
}

; core::ptr::slice_from_raw_parts_mut
; Function Attrs: inlinehint nounwind nonlazybind
define internal { [0 x i32]*, i64 } @_ZN4core3ptr24slice_from_raw_parts_mut17hb8598b066d922583E(i32* %data, i64 %len) unnamed_addr #0 {
start:
  %0 = bitcast i32* %data to {}*
  br label %bb1

bb1:                                              ; preds = %start
; call core::ptr::metadata::from_raw_parts_mut
  %1 = call { [0 x i32]*, i64 } @_ZN4core3ptr8metadata18from_raw_parts_mut17he61391023ac9f995E({}* %0, i64 %len) #11
  %2 = extractvalue { [0 x i32]*, i64 } %1, 0
  %3 = extractvalue { [0 x i32]*, i64 } %1, 1
  br label %bb2

bb2:                                              ; preds = %bb1
  %4 = insertvalue { [0 x i32]*, i64 } undef, [0 x i32]* %2, 0
  %5 = insertvalue { [0 x i32]*, i64 } %4, i64 %3, 1
  ret { [0 x i32]*, i64 } %5
}

; core::ptr::write
; Function Attrs: inlinehint nounwind nonlazybind
define internal void @_ZN4core3ptr5write17h6a579f8c36292fe2E(i32* %dst, i32 %0) unnamed_addr #0 {
start:
  %src = alloca i32, align 4
  store i32 %0, i32* %src, align 4
  %1 = bitcast i32* %dst to i8*
  %2 = bitcast i32* %src to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 4 %1, i8* align 4 %2, i64 4, i1 false)
  ret void
}

; core::cmp::Ord::max
; Function Attrs: inlinehint nounwind nonlazybind
define internal i64 @_ZN4core3cmp3Ord3max17h53f59996875dd13eE(i64 %self, i64 %other) unnamed_addr #0 {
start:
; call core::cmp::max_by
  %0 = call i64 @_ZN4core3cmp6max_by17h3c504843bb2842d5E(i64 %self, i64 %other) #11
  br label %bb1

bb1:                                              ; preds = %start
  ret i64 %0
}

; core::cmp::max
; Function Attrs: inlinehint nounwind nonlazybind
define internal i64 @_ZN4core3cmp3max17hd755847590d220faE(i64 %v1, i64 %v2) unnamed_addr #0 {
start:
; call core::cmp::Ord::max
  %0 = call i64 @_ZN4core3cmp3Ord3max17h53f59996875dd13eE(i64 %v1, i64 %v2) #11
  br label %bb1

bb1:                                              ; preds = %start
  ret i64 %0
}

; core::cmp::max_by
; Function Attrs: inlinehint nounwind nonlazybind
define internal i64 @_ZN4core3cmp6max_by17h3c504843bb2842d5E(i64 %0, i64 %1) unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
start:
  %2 = alloca { i8*, i32 }, align 8
  %_13 = alloca i8, align 1
  %_12 = alloca i8, align 1
  %_6 = alloca { i64*, i64* }, align 8
  %_4 = alloca i8, align 1
  %3 = alloca i64, align 8
  %v2 = alloca i64, align 8
  %v1 = alloca i64, align 8
  store i64 %0, i64* %v1, align 8
  store i64 %1, i64* %v2, align 8
  store i8 0, i8* %_13, align 1
  store i8 0, i8* %_12, align 1
  store i8 1, i8* %_13, align 1
  store i8 1, i8* %_12, align 1
  %4 = bitcast { i64*, i64* }* %_6 to i64**
  store i64* %v1, i64** %4, align 8
  %5 = getelementptr inbounds { i64*, i64* }, { i64*, i64* }* %_6, i32 0, i32 1
  store i64* %v2, i64** %5, align 8
  %6 = getelementptr inbounds { i64*, i64* }, { i64*, i64* }* %_6, i32 0, i32 0
  %7 = load i64*, i64** %6, align 8, !nonnull !3
  %8 = getelementptr inbounds { i64*, i64* }, { i64*, i64* }* %_6, i32 0, i32 1
  %9 = load i64*, i64** %8, align 8, !nonnull !3
; call core::ops::function::FnOnce::call_once
  %10 = call i8 @_ZN4core3ops8function6FnOnce9call_once17h7a32591392ef8fbbE(i64* align 8 dereferenceable(8) %7, i64* align 8 dereferenceable(8) %9) #11, !range !4
  store i8 %10, i8* %_4, align 1
  br label %bb1

bb1:                                              ; preds = %start
  %_11 = load i8, i8* %_4, align 1, !range !4
  switch i8 %_11, label %bb3 [
    i8 -1, label %bb4
    i8 0, label %bb4
    i8 1, label %bb2
  ]

bb8:                                              ; No predecessors!
  br label %bb13

bb13:                                             ; preds = %bb8
  %11 = load i8, i8* %_13, align 1, !range !5
  %12 = trunc i8 %11 to i1
  br i1 %12, label %bb12, label %bb9

bb3:                                              ; preds = %bb1
  unreachable

bb4:                                              ; preds = %bb1, %bb1
  store i8 0, i8* %_12, align 1
  %13 = load i64, i64* %v2, align 8
  store i64 %13, i64* %3, align 8
  br label %bb5

bb2:                                              ; preds = %bb1
  store i8 0, i8* %_13, align 1
  %14 = load i64, i64* %v1, align 8
  store i64 %14, i64* %3, align 8
  br label %bb5

bb5:                                              ; preds = %bb4, %bb2
  %15 = load i8, i8* %_12, align 1, !range !5
  %16 = trunc i8 %15 to i1
  br i1 %16, label %bb10, label %bb6

bb6:                                              ; preds = %bb10, %bb5
  %17 = load i8, i8* %_13, align 1, !range !5
  %18 = trunc i8 %17 to i1
  br i1 %18, label %bb11, label %bb7

bb10:                                             ; preds = %bb5
  br label %bb6

bb9:                                              ; preds = %bb12, %bb13
  %19 = bitcast { i8*, i32 }* %2 to i8**
  %20 = load i8*, i8** %19, align 8
  %21 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %2, i32 0, i32 1
  %22 = load i32, i32* %21, align 8
  %23 = insertvalue { i8*, i32 } undef, i8* %20, 0
  %24 = insertvalue { i8*, i32 } %23, i32 %22, 1
  resume { i8*, i32 } %24

bb12:                                             ; preds = %bb13
  br label %bb9

bb7:                                              ; preds = %bb11, %bb6
  %25 = load i64, i64* %3, align 8
  ret i64 %25

bb11:                                             ; preds = %bb6
  br label %bb7
}

; <T as core::convert::Into<U>>::into
; Function Attrs: nounwind nonlazybind
define internal { i64, i64 } @"_ZN50_$LT$T$u20$as$u20$core..convert..Into$LT$U$GT$$GT$4into17h5cabd2ba38994778E"(i64 %self.0, i64 %self.1) unnamed_addr #1 {
start:
; call <alloc::collections::TryReserveError as core::convert::From<alloc::collections::TryReserveErrorKind>>::from
  %0 = call { i64, i64 } @"_ZN122_$LT$alloc..collections..TryReserveError$u20$as$u20$core..convert..From$LT$alloc..collections..TryReserveErrorKind$GT$$GT$4from17hcee37011040de3f3E"(i64 %self.0, i64 %self.1) #11
  %1 = extractvalue { i64, i64 } %0, 0
  %2 = extractvalue { i64, i64 } %0, 1
  br label %bb1

bb1:                                              ; preds = %start
  %3 = insertvalue { i64, i64 } undef, i64 %1, 0
  %4 = insertvalue { i64, i64 } %3, i64 %2, 1
  ret { i64, i64 } %4
}

; <T as core::convert::Into<U>>::into
; Function Attrs: nounwind nonlazybind
define internal nonnull i8* @"_ZN50_$LT$T$u20$as$u20$core..convert..Into$LT$U$GT$$GT$4into17ha86098bf45577f87E"(i8* nonnull %self) unnamed_addr #1 {
start:
; call <core::ptr::non_null::NonNull<T> as core::convert::From<core::ptr::unique::Unique<T>>>::from
  %0 = call nonnull i8* @"_ZN119_$LT$core..ptr..non_null..NonNull$LT$T$GT$$u20$as$u20$core..convert..From$LT$core..ptr..unique..Unique$LT$T$GT$$GT$$GT$4from17ha3fdac2b0aed3b08E"(i8* nonnull %self) #11
  br label %bb1

bb1:                                              ; preds = %start
  ret i8* %0
}

; <T as core::convert::From<T>>::from
; Function Attrs: nounwind nonlazybind
define internal { i64, i64 } @"_ZN50_$LT$T$u20$as$u20$core..convert..From$LT$T$GT$$GT$4from17h0c870bb39bb6f75eE"(i64 %t.0, i64 %t.1) unnamed_addr #1 {
start:
  %0 = insertvalue { i64, i64 } undef, i64 %t.0, 0
  %1 = insertvalue { i64, i64 } %0, i64 %t.1, 1
  ret { i64, i64 } %1
}

; <T as core::convert::From<T>>::from
; Function Attrs: nounwind nonlazybind
define internal void @"_ZN50_$LT$T$u20$as$u20$core..convert..From$LT$T$GT$$GT$4from17h1abfc401a4fea46eE"() unnamed_addr #1 {
start:
  ret void
}

; <T as core::convert::From<T>>::from
; Function Attrs: nounwind nonlazybind
define internal void @"_ZN50_$LT$T$u20$as$u20$core..convert..From$LT$T$GT$$GT$4from17he66e1966cc279e3eE"() unnamed_addr #1 {
start:
  ret void
}

; alloc::collections::TryReserveError::kind
; Function Attrs: inlinehint nounwind nonlazybind
define internal { i64, i64 } @_ZN5alloc11collections15TryReserveError4kind17h96bdb01e2cd62539E({ i64, i64 }* align 8 dereferenceable(16) %self) unnamed_addr #0 {
start:
; call <alloc::collections::TryReserveErrorKind as core::clone::Clone>::clone
  %0 = call { i64, i64 } @"_ZN78_$LT$alloc..collections..TryReserveErrorKind$u20$as$u20$core..clone..Clone$GT$5clone17hb59cf41eca2dc70cE"({ i64, i64 }* align 8 dereferenceable(16) %self) #11
  %1 = extractvalue { i64, i64 } %0, 0
  %2 = extractvalue { i64, i64 } %0, 1
  br label %bb1

bb1:                                              ; preds = %start
  %3 = insertvalue { i64, i64 } undef, i64 %1, 0
  %4 = insertvalue { i64, i64 } %3, i64 %2, 1
  ret { i64, i64 } %4
}

; <alloc::collections::TryReserveError as core::convert::From<alloc::collections::TryReserveErrorKind>>::from
; Function Attrs: inlinehint nounwind nonlazybind
define internal { i64, i64 } @"_ZN122_$LT$alloc..collections..TryReserveError$u20$as$u20$core..convert..From$LT$alloc..collections..TryReserveErrorKind$GT$$GT$4from17hcee37011040de3f3E"(i64 %kind.0, i64 %kind.1) unnamed_addr #0 {
start:
  %0 = alloca { i64, i64 }, align 8
  %1 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %0, i32 0, i32 0
  store i64 %kind.0, i64* %1, align 8
  %2 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %0, i32 0, i32 1
  store i64 %kind.1, i64* %2, align 8
  %3 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %0, i32 0, i32 0
  %4 = load i64, i64* %3, align 8
  %5 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %0, i32 0, i32 1
  %6 = load i64, i64* %5, align 8
  %7 = insertvalue { i64, i64 } undef, i64 %4, 0
  %8 = insertvalue { i64, i64 } %7, i64 %6, 1
  ret { i64, i64 } %8
}

; alloc::vec::Vec<T>::new
; Function Attrs: inlinehint nounwind nonlazybind
define internal void @"_ZN5alloc3vec12Vec$LT$T$GT$3new17hcfe09e8a68ed215aE"(%"alloc::vec::Vec<i32>"* noalias nocapture sret(%"alloc::vec::Vec<i32>") dereferenceable(24) %0) unnamed_addr #0 {
start:
  %1 = bitcast %"alloc::vec::Vec<i32>"* %0 to { i32*, i64 }*
  %2 = load i32*, i32** getelementptr inbounds ({ i32*, i64 }, { i32*, i64 }* bitcast (<{ [16 x i8] }>* @0 to { i32*, i64 }*), i32 0, i32 0), align 8, !nonnull !3
  %3 = load i64, i64* getelementptr inbounds ({ i32*, i64 }, { i32*, i64 }* bitcast (<{ [16 x i8] }>* @0 to { i32*, i64 }*), i32 0, i32 1), align 8
  %4 = getelementptr inbounds { i32*, i64 }, { i32*, i64 }* %1, i32 0, i32 0
  store i32* %2, i32** %4, align 8
  %5 = getelementptr inbounds { i32*, i64 }, { i32*, i64 }* %1, i32 0, i32 1
  store i64 %3, i64* %5, align 8
  %6 = getelementptr inbounds %"alloc::vec::Vec<i32>", %"alloc::vec::Vec<i32>"* %0, i32 0, i32 1
  store i64 0, i64* %6, align 8
  ret void
}

; alloc::vec::Vec<T,A>::as_mut_ptr
; Function Attrs: inlinehint nounwind nonlazybind
define internal i32* @"_ZN5alloc3vec16Vec$LT$T$C$A$GT$10as_mut_ptr17he0bea1162e872c51E"(%"alloc::vec::Vec<i32>"* align 8 dereferenceable(24) %self) unnamed_addr #0 {
start:
  %_2 = bitcast %"alloc::vec::Vec<i32>"* %self to { i32*, i64 }*
; call alloc::raw_vec::RawVec<T,A>::ptr
  %ptr = call i32* @"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$3ptr17hf51cd8d24eb24d44E"({ i32*, i64 }* align 8 dereferenceable(16) %_2) #11
  br label %bb1

bb1:                                              ; preds = %start
; call core::ptr::mut_ptr::<impl *mut T>::is_null
  %_5 = call zeroext i1 @"_ZN4core3ptr7mut_ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$7is_null17h826eb03ba8c10c72E"(i32* %ptr) #11
  br label %bb2

bb2:                                              ; preds = %bb1
  %_4 = xor i1 %_5, true
  call void @llvm.assume(i1 %_4)
  br label %bb3

bb3:                                              ; preds = %bb2
  ret i32* %ptr
}

; alloc::vec::Vec<T,A>::push
; Function Attrs: inlinehint nounwind nonlazybind
define internal void @"_ZN5alloc3vec16Vec$LT$T$C$A$GT$4push17h335fdb8aa55e893dE"(%"alloc::vec::Vec<i32>"* align 8 dereferenceable(24) %self, i32 %value) unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
start:
  %0 = alloca i32*, align 8
  %1 = alloca i64, align 8
  %2 = alloca { i8*, i32 }, align 8
  %_17 = alloca i8, align 1
  store i8 0, i8* %_17, align 1
  store i8 1, i8* %_17, align 1
  %3 = getelementptr inbounds %"alloc::vec::Vec<i32>", %"alloc::vec::Vec<i32>"* %self, i32 0, i32 1
  %_4 = load i64, i64* %3, align 8
  %_6 = bitcast %"alloc::vec::Vec<i32>"* %self to { i32*, i64 }*
  %4 = icmp eq i64 4, 0
  br i1 %4, label %bb2.i, label %bb3.i

bb2.i:                                            ; preds = %start
  store i64 -1, i64* %1, align 8
  br label %"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$8capacity17h222326ee1cc163adE.exit"

bb3.i:                                            ; preds = %start
  %5 = getelementptr inbounds { i32*, i64 }, { i32*, i64 }* %_6, i32 0, i32 1
  %6 = load i64, i64* %5, align 8
  store i64 %6, i64* %1, align 8
  br label %"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$8capacity17h222326ee1cc163adE.exit"

"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$8capacity17h222326ee1cc163adE.exit": ; preds = %bb2.i, %bb3.i
  %7 = load i64, i64* %1, align 8
  br label %bb1

bb1:                                              ; preds = %"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$8capacity17h222326ee1cc163adE.exit"
  %_3 = icmp eq i64 %_4, %7
  br i1 %_3, label %bb2, label %bb4

bb4:                                              ; preds = %bb3, %bb1
; call alloc::vec::Vec<T,A>::as_mut_ptr
  %_11 = call i32* @"_ZN5alloc3vec16Vec$LT$T$C$A$GT$10as_mut_ptr17he0bea1162e872c51E"(%"alloc::vec::Vec<i32>"* align 8 dereferenceable(24) %self) #11
  br label %bb5

bb2:                                              ; preds = %bb1
  %_8 = bitcast %"alloc::vec::Vec<i32>"* %self to { i32*, i64 }*
  %8 = getelementptr inbounds %"alloc::vec::Vec<i32>", %"alloc::vec::Vec<i32>"* %self, i32 0, i32 1
  %_9 = load i64, i64* %8, align 8
; call alloc::raw_vec::RawVec<T,A>::reserve_for_push
  call void @"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$16reserve_for_push17haa0e1e5e2b63f67aE"({ i32*, i64 }* align 8 dereferenceable(16) %_8, i64 %_9) #11
  br label %bb3

bb3:                                              ; preds = %bb2
  br label %bb4

bb5:                                              ; preds = %bb4
  %9 = getelementptr inbounds %"alloc::vec::Vec<i32>", %"alloc::vec::Vec<i32>"* %self, i32 0, i32 1
  %_13 = load i64, i64* %9, align 8
  %10 = getelementptr inbounds i32, i32* %_11, i64 %_13
  store i32* %10, i32** %0, align 8
  %_3.i.i = load i32*, i32** %0, align 8
  br label %bb6

bb6:                                              ; preds = %bb5
  store i8 0, i8* %_17, align 1
; call core::ptr::write
  call void @_ZN4core3ptr5write17h6a579f8c36292fe2E(i32* %_3.i.i, i32 %value) #11
  br label %bb7

bb7:                                              ; preds = %bb6
  %11 = getelementptr inbounds %"alloc::vec::Vec<i32>", %"alloc::vec::Vec<i32>"* %self, i32 0, i32 1
  %12 = getelementptr inbounds %"alloc::vec::Vec<i32>", %"alloc::vec::Vec<i32>"* %self, i32 0, i32 1
  %13 = load i64, i64* %12, align 8
  %14 = add i64 %13, 1
  store i64 %14, i64* %11, align 8
  ret void

bb10:                                             ; No predecessors!
  %15 = load i8, i8* %_17, align 1, !range !5
  %16 = trunc i8 %15 to i1
  br i1 %16, label %bb9, label %bb8

bb8:                                              ; preds = %bb9, %bb10
  %17 = bitcast { i8*, i32 }* %2 to i8**
  %18 = load i8*, i8** %17, align 8
  %19 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %2, i32 0, i32 1
  %20 = load i32, i32* %19, align 8
  %21 = insertvalue { i8*, i32 } undef, i8* %18, 0
  %22 = insertvalue { i8*, i32 } %21, i32 %20, 1
  resume { i8*, i32 } %22

bb9:                                              ; preds = %bb10
  br label %bb8
}

; <alloc::vec::Vec<T,A> as core::ops::drop::Drop>::drop
; Function Attrs: nounwind nonlazybind
define internal void @"_ZN70_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h672635a5205cb45fE"(%"alloc::vec::Vec<i32>"* align 8 dereferenceable(24) %self) unnamed_addr #1 {
start:
; call alloc::vec::Vec<T,A>::as_mut_ptr
  %_3 = call i32* @"_ZN5alloc3vec16Vec$LT$T$C$A$GT$10as_mut_ptr17he0bea1162e872c51E"(%"alloc::vec::Vec<i32>"* align 8 dereferenceable(24) %self) #11
  br label %bb1

bb1:                                              ; preds = %start
  %0 = getelementptr inbounds %"alloc::vec::Vec<i32>", %"alloc::vec::Vec<i32>"* %self, i32 0, i32 1
  %_5 = load i64, i64* %0, align 8
; call core::ptr::slice_from_raw_parts_mut
  %1 = call { [0 x i32]*, i64 } @_ZN4core3ptr24slice_from_raw_parts_mut17hb8598b066d922583E(i32* %_3, i64 %_5) #11
  %_2.0 = extractvalue { [0 x i32]*, i64 } %1, 0
  %_2.1 = extractvalue { [0 x i32]*, i64 } %1, 1
  br label %bb2

bb2:                                              ; preds = %bb1
  br label %bb3

bb3:                                              ; preds = %bb2
  ret void
}

; <() as std::process::Termination>::report
; Function Attrs: inlinehint nounwind nonlazybind
define internal i32 @"_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17he6bda9206bcab9a0E"() unnamed_addr #0 {
start:
; call <std::process::ExitCode as std::process::Termination>::report
  %0 = call i32 @"_ZN68_$LT$std..process..ExitCode$u20$as$u20$std..process..Termination$GT$6report17h7da18fcbe2f6af30E"(i8 0) #11
  br label %bb1

bb1:                                              ; preds = %start
  ret i32 %0
}

; <std::process::ExitCode as std::process::Termination>::report
; Function Attrs: inlinehint nounwind nonlazybind
define internal i32 @"_ZN68_$LT$std..process..ExitCode$u20$as$u20$std..process..Termination$GT$6report17h7da18fcbe2f6af30E"(i8 %0) unnamed_addr #0 {
start:
  %self = alloca i8, align 1
  store i8 %0, i8* %self, align 1
; call std::sys::unix::process::process_common::ExitCode::as_i32
  %1 = call i32 @_ZN3std3sys4unix7process14process_common8ExitCode6as_i3217h1eadc22b4efb4f09E(i8* align 1 dereferenceable(1) %self) #11
  br label %bb1

bb1:                                              ; preds = %start
  ret i32 %1
}

; <alloc::collections::TryReserveErrorKind as core::clone::Clone>::clone
; Function Attrs: inlinehint nounwind nonlazybind
define internal { i64, i64 } @"_ZN78_$LT$alloc..collections..TryReserveErrorKind$u20$as$u20$core..clone..Clone$GT$5clone17hb59cf41eca2dc70cE"({ i64, i64 }* align 8 dereferenceable(16) %self) unnamed_addr #0 {
start:
  %_2 = alloca i64*, align 8
  %0 = alloca { i64, i64 }, align 8
  %1 = bitcast i64** %_2 to { i64, i64 }**
  store { i64, i64 }* %self, { i64, i64 }** %1, align 8
  %2 = bitcast i64** %_2 to { i64, i64 }**
  %3 = load { i64, i64 }*, { i64, i64 }** %2, align 8, !nonnull !3
  %4 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %3, i32 0, i32 1
  %5 = load i64, i64* %4, align 8
  %6 = icmp eq i64 %5, 0
  %_4 = select i1 %6, i64 0, i64 1
  switch i64 %_4, label %bb2 [
    i64 0, label %bb3
    i64 1, label %bb1
  ]

bb2:                                              ; preds = %start
  unreachable

bb3:                                              ; preds = %start
  %7 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %0, i32 0, i32 1
  store i64 0, i64* %7, align 8
  br label %bb6

bb1:                                              ; preds = %start
  %8 = bitcast i64** %_2 to { i64, i64 }**
  %__self_0 = load { i64, i64 }*, { i64, i64 }** %8, align 8, !nonnull !3
  %9 = bitcast i64** %_2 to { i64, i64 }**
  %10 = load { i64, i64 }*, { i64, i64 }** %9, align 8, !nonnull !3
  %__self_1 = bitcast { i64, i64 }* %10 to {}*
; call <core::alloc::layout::Layout as core::clone::Clone>::clone
  %11 = call { i64, i64 } @"_ZN66_$LT$core..alloc..layout..Layout$u20$as$u20$core..clone..Clone$GT$5clone17h034b00774e006c4dE"({ i64, i64 }* align 8 dereferenceable(16) %__self_0) #11
  %_7.0 = extractvalue { i64, i64 } %11, 0
  %_7.1 = extractvalue { i64, i64 } %11, 1
  br label %bb4

bb4:                                              ; preds = %bb1
; call core::clone::Clone::clone
  call void @_ZN4core5clone5Clone5clone17had41c4bc85d28bbcE({}* nonnull align 1 %__self_1) #11
  br label %bb5

bb5:                                              ; preds = %bb4
  %12 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %0, i32 0, i32 0
  store i64 %_7.0, i64* %12, align 8
  %13 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %0, i32 0, i32 1
  store i64 %_7.1, i64* %13, align 8
  br label %bb6

bb6:                                              ; preds = %bb3, %bb5
  %14 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %0, i32 0, i32 0
  %15 = load i64, i64* %14, align 8
  %16 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %0, i32 0, i32 1
  %17 = load i64, i64* %16, align 8
  %18 = insertvalue { i64, i64 } undef, i64 %15, 0
  %19 = insertvalue { i64, i64 } %18, i64 %17, 1
  ret { i64, i64 } %19
}

; core::option::Option<T>::ok_or
; Function Attrs: inlinehint nounwind nonlazybind
define internal void @"_ZN4core6option15Option$LT$T$GT$5ok_or17h353dffde265f05a4E"(%"core::result::Result<usize, alloc::collections::TryReserveErrorKind>"* noalias nocapture sret(%"core::result::Result<usize, alloc::collections::TryReserveErrorKind>") dereferenceable(24) %0, i64 %1, i64 %2, i64 %err.0, i64 %err.1) unnamed_addr #0 {
start:
  %_7 = alloca i8, align 1
  %self = alloca { i64, i64 }, align 8
  %3 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %self, i32 0, i32 0
  store i64 %1, i64* %3, align 8
  %4 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %self, i32 0, i32 1
  store i64 %2, i64* %4, align 8
  store i8 0, i8* %_7, align 1
  store i8 1, i8* %_7, align 1
  %5 = bitcast { i64, i64 }* %self to i64*
  %_3 = load i64, i64* %5, align 8, !range !6
  switch i64 %_3, label %bb2 [
    i64 0, label %bb1
    i64 1, label %bb3
  ]

bb2:                                              ; preds = %start
  unreachable

bb1:                                              ; preds = %start
  store i8 0, i8* %_7, align 1
  %6 = bitcast %"core::result::Result<usize, alloc::collections::TryReserveErrorKind>"* %0 to %"core::result::Result<usize, alloc::collections::TryReserveErrorKind>::Err"*
  %7 = getelementptr inbounds %"core::result::Result<usize, alloc::collections::TryReserveErrorKind>::Err", %"core::result::Result<usize, alloc::collections::TryReserveErrorKind>::Err"* %6, i32 0, i32 1
  %8 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %7, i32 0, i32 0
  store i64 %err.0, i64* %8, align 8
  %9 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %7, i32 0, i32 1
  store i64 %err.1, i64* %9, align 8
  %10 = bitcast %"core::result::Result<usize, alloc::collections::TryReserveErrorKind>"* %0 to i64*
  store i64 1, i64* %10, align 8
  br label %bb6

bb3:                                              ; preds = %start
  %11 = bitcast { i64, i64 }* %self to %"core::option::Option<usize>::Some"*
  %12 = getelementptr inbounds %"core::option::Option<usize>::Some", %"core::option::Option<usize>::Some"* %11, i32 0, i32 1
  %v = load i64, i64* %12, align 8
  %13 = bitcast %"core::result::Result<usize, alloc::collections::TryReserveErrorKind>"* %0 to %"core::result::Result<usize, alloc::collections::TryReserveErrorKind>::Ok"*
  %14 = getelementptr inbounds %"core::result::Result<usize, alloc::collections::TryReserveErrorKind>::Ok", %"core::result::Result<usize, alloc::collections::TryReserveErrorKind>::Ok"* %13, i32 0, i32 1
  store i64 %v, i64* %14, align 8
  %15 = bitcast %"core::result::Result<usize, alloc::collections::TryReserveErrorKind>"* %0 to i64*
  store i64 0, i64* %15, align 8
  br label %bb6

bb6:                                              ; preds = %bb1, %bb3
  %16 = load i8, i8* %_7, align 1, !range !5
  %17 = trunc i8 %16 to i1
  br i1 %17, label %bb5, label %bb4

bb4:                                              ; preds = %bb5, %bb6
  ret void

bb5:                                              ; preds = %bb6
  br label %bb4
}

; core::option::Option<T>::ok_or
; Function Attrs: inlinehint nounwind nonlazybind
define internal i8* @"_ZN4core6option15Option$LT$T$GT$5ok_or17h4e06d488786df54cE"(i8* %0) unnamed_addr #0 {
start:
  %_7 = alloca i8, align 1
  %1 = alloca i8*, align 8
  %self = alloca i8*, align 8
  store i8* %0, i8** %self, align 8
  store i8 0, i8* %_7, align 1
  store i8 1, i8* %_7, align 1
  %2 = bitcast i8** %self to {}**
  %3 = load {}*, {}** %2, align 8
  %4 = icmp eq {}* %3, null
  %_3 = select i1 %4, i64 0, i64 1
  switch i64 %_3, label %bb2 [
    i64 0, label %bb1
    i64 1, label %bb3
  ]

bb2:                                              ; preds = %start
  unreachable

bb1:                                              ; preds = %start
  store i8 0, i8* %_7, align 1
  %5 = bitcast i8** %1 to %"core::result::Result<core::ptr::non_null::NonNull<u8>, core::alloc::AllocError>::Err"*
  %6 = bitcast %"core::result::Result<core::ptr::non_null::NonNull<u8>, core::alloc::AllocError>::Err"* %5 to %"core::alloc::AllocError"*
  %7 = bitcast i8** %1 to {}**
  store {}* null, {}** %7, align 8
  br label %bb6

bb3:                                              ; preds = %start
  %v = load i8*, i8** %self, align 8, !nonnull !3
  store i8* %v, i8** %1, align 8
  br label %bb6

bb6:                                              ; preds = %bb1, %bb3
  %8 = load i8, i8* %_7, align 1, !range !5
  %9 = trunc i8 %8 to i1
  br i1 %9, label %bb5, label %bb4

bb4:                                              ; preds = %bb5, %bb6
  %10 = load i8*, i8** %1, align 8
  ret i8* %10

bb5:                                              ; preds = %bb6
  br label %bb4
}

; core::option::Option<T>::ok_or
; Function Attrs: inlinehint nounwind nonlazybind
define internal { i64, i64 } @"_ZN4core6option15Option$LT$T$GT$5ok_or17h8372c131d912044bE"(i64 %0, i64 %1) unnamed_addr #0 {
start:
  %_7 = alloca i8, align 1
  %2 = alloca { i64, i64 }, align 8
  %self = alloca { i64, i64 }, align 8
  %3 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %self, i32 0, i32 0
  store i64 %0, i64* %3, align 8
  %4 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %self, i32 0, i32 1
  store i64 %1, i64* %4, align 8
  store i8 0, i8* %_7, align 1
  store i8 1, i8* %_7, align 1
  %5 = bitcast { i64, i64 }* %self to i64*
  %_3 = load i64, i64* %5, align 8, !range !6
  switch i64 %_3, label %bb2 [
    i64 0, label %bb1
    i64 1, label %bb3
  ]

bb2:                                              ; preds = %start
  unreachable

bb1:                                              ; preds = %start
  store i8 0, i8* %_7, align 1
  %6 = bitcast { i64, i64 }* %2 to %"core::result::Result<usize, core::alloc::layout::LayoutError>::Err"*
  %7 = getelementptr inbounds %"core::result::Result<usize, core::alloc::layout::LayoutError>::Err", %"core::result::Result<usize, core::alloc::layout::LayoutError>::Err"* %6, i32 0, i32 1
  %8 = bitcast { i64, i64 }* %2 to i64*
  store i64 1, i64* %8, align 8
  br label %bb6

bb3:                                              ; preds = %start
  %9 = bitcast { i64, i64 }* %self to %"core::option::Option<usize>::Some"*
  %10 = getelementptr inbounds %"core::option::Option<usize>::Some", %"core::option::Option<usize>::Some"* %9, i32 0, i32 1
  %v = load i64, i64* %10, align 8
  %11 = bitcast { i64, i64 }* %2 to %"core::result::Result<usize, core::alloc::layout::LayoutError>::Ok"*
  %12 = getelementptr inbounds %"core::result::Result<usize, core::alloc::layout::LayoutError>::Ok", %"core::result::Result<usize, core::alloc::layout::LayoutError>::Ok"* %11, i32 0, i32 1
  store i64 %v, i64* %12, align 8
  %13 = bitcast { i64, i64 }* %2 to i64*
  store i64 0, i64* %13, align 8
  br label %bb6

bb6:                                              ; preds = %bb1, %bb3
  %14 = load i8, i8* %_7, align 1, !range !5
  %15 = trunc i8 %14 to i1
  br i1 %15, label %bb5, label %bb4

bb4:                                              ; preds = %bb5, %bb6
  %16 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %2, i32 0, i32 0
  %17 = load i64, i64* %16, align 8, !range !6
  %18 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %2, i32 0, i32 1
  %19 = load i64, i64* %18, align 8
  %20 = insertvalue { i64, i64 } undef, i64 %17, 0
  %21 = insertvalue { i64, i64 } %20, i64 %19, 1
  ret { i64, i64 } %21

bb5:                                              ; preds = %bb6
  br label %bb4
}

; core::result::Result<T,E>::map_err
; Function Attrs: inlinehint nounwind nonlazybind
define internal void @"_ZN4core6result19Result$LT$T$C$E$GT$7map_err17h1aab0f13572c0699E"(%"core::result::Result<core::ptr::non_null::NonNull<[u8]>, alloc::collections::TryReserveError>"* noalias nocapture sret(%"core::result::Result<core::ptr::non_null::NonNull<[u8]>, alloc::collections::TryReserveError>") dereferenceable(24) %0, i8* %1, i64 %2, i64* align 8 dereferenceable(16) %op) unnamed_addr #0 {
start:
  %_11 = alloca i8, align 1
  %_9 = alloca { %"core::alloc::AllocError" }, align 1
  %self = alloca { i8*, i64 }, align 8
  %3 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %self, i32 0, i32 0
  store i8* %1, i8** %3, align 8
  %4 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %self, i32 0, i32 1
  store i64 %2, i64* %4, align 8
  store i8 0, i8* %_11, align 1
  store i8 1, i8* %_11, align 1
  %5 = bitcast { i8*, i64 }* %self to {}**
  %6 = load {}*, {}** %5, align 8
  %7 = icmp eq {}* %6, null
  %_3 = select i1 %7, i64 1, i64 0
  switch i64 %_3, label %bb2 [
    i64 0, label %bb3
    i64 1, label %bb1
  ]

bb2:                                              ; preds = %start
  unreachable

bb3:                                              ; preds = %start
  %8 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %self, i32 0, i32 0
  %t.0 = load i8*, i8** %8, align 8, !nonnull !3
  %9 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %self, i32 0, i32 1
  %t.1 = load i64, i64* %9, align 8
  %10 = bitcast %"core::result::Result<core::ptr::non_null::NonNull<[u8]>, alloc::collections::TryReserveError>"* %0 to %"core::result::Result<core::ptr::non_null::NonNull<[u8]>, alloc::collections::TryReserveError>::Ok"*
  %11 = getelementptr inbounds %"core::result::Result<core::ptr::non_null::NonNull<[u8]>, alloc::collections::TryReserveError>::Ok", %"core::result::Result<core::ptr::non_null::NonNull<[u8]>, alloc::collections::TryReserveError>::Ok"* %10, i32 0, i32 1
  %12 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %11, i32 0, i32 0
  store i8* %t.0, i8** %12, align 8
  %13 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %11, i32 0, i32 1
  store i64 %t.1, i64* %13, align 8
  %14 = bitcast %"core::result::Result<core::ptr::non_null::NonNull<[u8]>, alloc::collections::TryReserveError>"* %0 to i64*
  store i64 0, i64* %14, align 8
  br label %bb7

bb1:                                              ; preds = %start
  store i8 0, i8* %_11, align 1
  %15 = bitcast { %"core::alloc::AllocError" }* %_9 to %"core::alloc::AllocError"*
; call alloc::raw_vec::finish_grow::{{closure}}
  %16 = call { i64, i64 } @"_ZN5alloc7raw_vec11finish_grow28_$u7b$$u7b$closure$u7d$$u7d$17h46de84ed766927f7E"(i64* align 8 dereferenceable(16) %op) #11
  %_7.0 = extractvalue { i64, i64 } %16, 0
  %_7.1 = extractvalue { i64, i64 } %16, 1
  br label %bb4

bb4:                                              ; preds = %bb1
  %17 = bitcast %"core::result::Result<core::ptr::non_null::NonNull<[u8]>, alloc::collections::TryReserveError>"* %0 to %"core::result::Result<core::ptr::non_null::NonNull<[u8]>, alloc::collections::TryReserveError>::Err"*
  %18 = getelementptr inbounds %"core::result::Result<core::ptr::non_null::NonNull<[u8]>, alloc::collections::TryReserveError>::Err", %"core::result::Result<core::ptr::non_null::NonNull<[u8]>, alloc::collections::TryReserveError>::Err"* %17, i32 0, i32 1
  %19 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %18, i32 0, i32 0
  store i64 %_7.0, i64* %19, align 8
  %20 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %18, i32 0, i32 1
  store i64 %_7.1, i64* %20, align 8
  %21 = bitcast %"core::result::Result<core::ptr::non_null::NonNull<[u8]>, alloc::collections::TryReserveError>"* %0 to i64*
  store i64 1, i64* %21, align 8
  br label %bb7

bb7:                                              ; preds = %bb3, %bb4
  %22 = load i8, i8* %_11, align 1, !range !5
  %23 = trunc i8 %22 to i1
  br i1 %23, label %bb6, label %bb5

bb5:                                              ; preds = %bb6, %bb7
  ret void

bb6:                                              ; preds = %bb7
  br label %bb5
}

; core::result::Result<T,E>::map_err
; Function Attrs: inlinehint nounwind nonlazybind
define internal void @"_ZN4core6result19Result$LT$T$C$E$GT$7map_err17h8c0dfe53da6f1e72E"(%"core::result::Result<(), alloc::collections::TryReserveErrorKind>"* noalias nocapture sret(%"core::result::Result<(), alloc::collections::TryReserveErrorKind>") dereferenceable(24) %0, %"core::result::Result<(), alloc::collections::TryReserveError>"* noalias nocapture dereferenceable(24) %self) unnamed_addr #0 {
start:
  %_11 = alloca i8, align 1
  %_9 = alloca { i64, i64 }, align 8
  store i8 0, i8* %_11, align 1
  store i8 1, i8* %_11, align 1
  %1 = bitcast %"core::result::Result<(), alloc::collections::TryReserveError>"* %self to i64*
  %_3 = load i64, i64* %1, align 8, !range !6
  switch i64 %_3, label %bb2 [
    i64 0, label %bb3
    i64 1, label %bb1
  ]

bb2:                                              ; preds = %start
  unreachable

bb3:                                              ; preds = %start
  %2 = bitcast %"core::result::Result<(), alloc::collections::TryReserveErrorKind>"* %0 to %"core::result::Result<(), alloc::collections::TryReserveErrorKind>::Ok"*
  %3 = getelementptr inbounds %"core::result::Result<(), alloc::collections::TryReserveErrorKind>::Ok", %"core::result::Result<(), alloc::collections::TryReserveErrorKind>::Ok"* %2, i32 0, i32 1
  %4 = bitcast %"core::result::Result<(), alloc::collections::TryReserveErrorKind>"* %0 to i64*
  store i64 0, i64* %4, align 8
  br label %bb7

bb1:                                              ; preds = %start
  %5 = bitcast %"core::result::Result<(), alloc::collections::TryReserveError>"* %self to %"core::result::Result<(), alloc::collections::TryReserveError>::Err"*
  %6 = getelementptr inbounds %"core::result::Result<(), alloc::collections::TryReserveError>::Err", %"core::result::Result<(), alloc::collections::TryReserveError>::Err"* %5, i32 0, i32 1
  %7 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %6, i32 0, i32 0
  %e.0 = load i64, i64* %7, align 8
  %8 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %6, i32 0, i32 1
  %e.1 = load i64, i64* %8, align 8
  store i8 0, i8* %_11, align 1
  %9 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %_9, i32 0, i32 0
  store i64 %e.0, i64* %9, align 8
  %10 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %_9, i32 0, i32 1
  store i64 %e.1, i64* %10, align 8
  %11 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %_9, i32 0, i32 0
  %12 = load i64, i64* %11, align 8
  %13 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %_9, i32 0, i32 1
  %14 = load i64, i64* %13, align 8
; call alloc::raw_vec::handle_reserve::{{closure}}
  %15 = call { i64, i64 } @"_ZN5alloc7raw_vec14handle_reserve28_$u7b$$u7b$closure$u7d$$u7d$17h46cf85d469126324E"(i64 %12, i64 %14) #11
  %_7.0 = extractvalue { i64, i64 } %15, 0
  %_7.1 = extractvalue { i64, i64 } %15, 1
  br label %bb4

bb4:                                              ; preds = %bb1
  %16 = bitcast %"core::result::Result<(), alloc::collections::TryReserveErrorKind>"* %0 to %"core::result::Result<(), alloc::collections::TryReserveErrorKind>::Err"*
  %17 = getelementptr inbounds %"core::result::Result<(), alloc::collections::TryReserveErrorKind>::Err", %"core::result::Result<(), alloc::collections::TryReserveErrorKind>::Err"* %16, i32 0, i32 1
  %18 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %17, i32 0, i32 0
  store i64 %_7.0, i64* %18, align 8
  %19 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %17, i32 0, i32 1
  store i64 %_7.1, i64* %19, align 8
  %20 = bitcast %"core::result::Result<(), alloc::collections::TryReserveErrorKind>"* %0 to i64*
  store i64 1, i64* %20, align 8
  br label %bb7

bb7:                                              ; preds = %bb3, %bb4
  %21 = load i8, i8* %_11, align 1, !range !5
  %22 = trunc i8 %21 to i1
  br i1 %22, label %bb6, label %bb5

bb5:                                              ; preds = %bb6, %bb7
  ret void

bb6:                                              ; preds = %bb7
  br label %bb5
}

; core::result::Result<T,E>::map_err
; Function Attrs: inlinehint nounwind nonlazybind
define internal void @"_ZN4core6result19Result$LT$T$C$E$GT$7map_err17he544d618a000548cE"(%"core::result::Result<core::alloc::layout::Layout, alloc::collections::TryReserveErrorKind>"* noalias nocapture sret(%"core::result::Result<core::alloc::layout::Layout, alloc::collections::TryReserveErrorKind>") dereferenceable(24) %0, i64 %1, i64 %2) unnamed_addr #0 {
start:
  %_11 = alloca i8, align 1
  %_9 = alloca { %"core::alloc::layout::LayoutError" }, align 1
  %self = alloca { i64, i64 }, align 8
  %3 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %self, i32 0, i32 0
  store i64 %1, i64* %3, align 8
  %4 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %self, i32 0, i32 1
  store i64 %2, i64* %4, align 8
  store i8 0, i8* %_11, align 1
  store i8 1, i8* %_11, align 1
  %5 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %self, i32 0, i32 1
  %6 = load i64, i64* %5, align 8
  %7 = icmp eq i64 %6, 0
  %_3 = select i1 %7, i64 1, i64 0
  switch i64 %_3, label %bb2 [
    i64 0, label %bb3
    i64 1, label %bb1
  ]

bb2:                                              ; preds = %start
  unreachable

bb3:                                              ; preds = %start
  %8 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %self, i32 0, i32 0
  %t.0 = load i64, i64* %8, align 8
  %9 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %self, i32 0, i32 1
  %t.1 = load i64, i64* %9, align 8, !range !7
  %10 = bitcast %"core::result::Result<core::alloc::layout::Layout, alloc::collections::TryReserveErrorKind>"* %0 to %"core::result::Result<core::alloc::layout::Layout, alloc::collections::TryReserveErrorKind>::Ok"*
  %11 = getelementptr inbounds %"core::result::Result<core::alloc::layout::Layout, alloc::collections::TryReserveErrorKind>::Ok", %"core::result::Result<core::alloc::layout::Layout, alloc::collections::TryReserveErrorKind>::Ok"* %10, i32 0, i32 1
  %12 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %11, i32 0, i32 0
  store i64 %t.0, i64* %12, align 8
  %13 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %11, i32 0, i32 1
  store i64 %t.1, i64* %13, align 8
  %14 = bitcast %"core::result::Result<core::alloc::layout::Layout, alloc::collections::TryReserveErrorKind>"* %0 to i64*
  store i64 0, i64* %14, align 8
  br label %bb7

bb1:                                              ; preds = %start
  store i8 0, i8* %_11, align 1
  %15 = bitcast { %"core::alloc::layout::LayoutError" }* %_9 to %"core::alloc::layout::LayoutError"*
; call alloc::raw_vec::finish_grow::{{closure}}
  %16 = call { i64, i64 } @"_ZN5alloc7raw_vec11finish_grow28_$u7b$$u7b$closure$u7d$$u7d$17h521d57bea205a370E"() #11
  %_7.0 = extractvalue { i64, i64 } %16, 0
  %_7.1 = extractvalue { i64, i64 } %16, 1
  br label %bb4

bb4:                                              ; preds = %bb1
  %17 = bitcast %"core::result::Result<core::alloc::layout::Layout, alloc::collections::TryReserveErrorKind>"* %0 to %"core::result::Result<core::alloc::layout::Layout, alloc::collections::TryReserveErrorKind>::Err"*
  %18 = getelementptr inbounds %"core::result::Result<core::alloc::layout::Layout, alloc::collections::TryReserveErrorKind>::Err", %"core::result::Result<core::alloc::layout::Layout, alloc::collections::TryReserveErrorKind>::Err"* %17, i32 0, i32 1
  %19 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %18, i32 0, i32 0
  store i64 %_7.0, i64* %19, align 8
  %20 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %18, i32 0, i32 1
  store i64 %_7.1, i64* %20, align 8
  %21 = bitcast %"core::result::Result<core::alloc::layout::Layout, alloc::collections::TryReserveErrorKind>"* %0 to i64*
  store i64 1, i64* %21, align 8
  br label %bb7

bb7:                                              ; preds = %bb3, %bb4
  %22 = load i8, i8* %_11, align 1, !range !5
  %23 = trunc i8 %22 to i1
  br i1 %23, label %bb6, label %bb5

bb5:                                              ; preds = %bb6, %bb7
  ret void

bb6:                                              ; preds = %bb7
  br label %bb5
}

; <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
; Function Attrs: inlinehint nounwind nonlazybind
define internal void @"_ZN79_$LT$core..result..Result$LT$T$C$E$GT$$u20$as$u20$core..ops..try_trait..Try$GT$6branch17h15bb74d3338a4f44E"(%"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveError>, core::ptr::non_null::NonNull<[u8]>>"* noalias nocapture sret(%"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveError>, core::ptr::non_null::NonNull<[u8]>>") dereferenceable(24) %0, %"core::result::Result<core::ptr::non_null::NonNull<[u8]>, alloc::collections::TryReserveError>"* noalias nocapture dereferenceable(24) %self) unnamed_addr #0 {
start:
  %_6 = alloca { i64, i64 }, align 8
  %1 = bitcast %"core::result::Result<core::ptr::non_null::NonNull<[u8]>, alloc::collections::TryReserveError>"* %self to i64*
  %_2 = load i64, i64* %1, align 8, !range !6
  switch i64 %_2, label %bb2 [
    i64 0, label %bb3
    i64 1, label %bb1
  ]

bb2:                                              ; preds = %start
  unreachable

bb3:                                              ; preds = %start
  %2 = bitcast %"core::result::Result<core::ptr::non_null::NonNull<[u8]>, alloc::collections::TryReserveError>"* %self to %"core::result::Result<core::ptr::non_null::NonNull<[u8]>, alloc::collections::TryReserveError>::Ok"*
  %3 = getelementptr inbounds %"core::result::Result<core::ptr::non_null::NonNull<[u8]>, alloc::collections::TryReserveError>::Ok", %"core::result::Result<core::ptr::non_null::NonNull<[u8]>, alloc::collections::TryReserveError>::Ok"* %2, i32 0, i32 1
  %4 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %3, i32 0, i32 0
  %v.0 = load i8*, i8** %4, align 8, !nonnull !3
  %5 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %3, i32 0, i32 1
  %v.1 = load i64, i64* %5, align 8
  %6 = bitcast %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveError>, core::ptr::non_null::NonNull<[u8]>>"* %0 to %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveError>, core::ptr::non_null::NonNull<[u8]>>::Continue"*
  %7 = getelementptr inbounds %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveError>, core::ptr::non_null::NonNull<[u8]>>::Continue", %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveError>, core::ptr::non_null::NonNull<[u8]>>::Continue"* %6, i32 0, i32 1
  %8 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %7, i32 0, i32 0
  store i8* %v.0, i8** %8, align 8
  %9 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %7, i32 0, i32 1
  store i64 %v.1, i64* %9, align 8
  %10 = bitcast %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveError>, core::ptr::non_null::NonNull<[u8]>>"* %0 to i64*
  store i64 0, i64* %10, align 8
  br label %bb4

bb1:                                              ; preds = %start
  %11 = bitcast %"core::result::Result<core::ptr::non_null::NonNull<[u8]>, alloc::collections::TryReserveError>"* %self to %"core::result::Result<core::ptr::non_null::NonNull<[u8]>, alloc::collections::TryReserveError>::Err"*
  %12 = getelementptr inbounds %"core::result::Result<core::ptr::non_null::NonNull<[u8]>, alloc::collections::TryReserveError>::Err", %"core::result::Result<core::ptr::non_null::NonNull<[u8]>, alloc::collections::TryReserveError>::Err"* %11, i32 0, i32 1
  %13 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %12, i32 0, i32 0
  %e.0 = load i64, i64* %13, align 8
  %14 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %12, i32 0, i32 1
  %e.1 = load i64, i64* %14, align 8
  %15 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %_6, i32 0, i32 0
  store i64 %e.0, i64* %15, align 8
  %16 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %_6, i32 0, i32 1
  store i64 %e.1, i64* %16, align 8
  %17 = bitcast %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveError>, core::ptr::non_null::NonNull<[u8]>>"* %0 to %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveError>, core::ptr::non_null::NonNull<[u8]>>::Break"*
  %18 = getelementptr inbounds %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveError>, core::ptr::non_null::NonNull<[u8]>>::Break", %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveError>, core::ptr::non_null::NonNull<[u8]>>::Break"* %17, i32 0, i32 1
  %19 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %_6, i32 0, i32 0
  %20 = load i64, i64* %19, align 8
  %21 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %_6, i32 0, i32 1
  %22 = load i64, i64* %21, align 8
  %23 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %18, i32 0, i32 0
  store i64 %20, i64* %23, align 8
  %24 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %18, i32 0, i32 1
  store i64 %22, i64* %24, align 8
  %25 = bitcast %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveError>, core::ptr::non_null::NonNull<[u8]>>"* %0 to i64*
  store i64 1, i64* %25, align 8
  br label %bb4

bb4:                                              ; preds = %bb3, %bb1
  ret void
}

; <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
; Function Attrs: inlinehint nounwind nonlazybind
define internal { i64, i64 } @"_ZN79_$LT$core..result..Result$LT$T$C$E$GT$$u20$as$u20$core..ops..try_trait..Try$GT$6branch17h17ea615427507abeE"(i64 %0, i64 %1) unnamed_addr #0 {
start:
  %_6 = alloca %"core::result::Result<core::convert::Infallible, core::alloc::layout::LayoutError>::Err", align 1
  %2 = alloca { i64, i64 }, align 8
  %self = alloca { i64, i64 }, align 8
  %3 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %self, i32 0, i32 0
  store i64 %0, i64* %3, align 8
  %4 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %self, i32 0, i32 1
  store i64 %1, i64* %4, align 8
  %5 = bitcast { i64, i64 }* %self to i64*
  %_2 = load i64, i64* %5, align 8, !range !6
  switch i64 %_2, label %bb2 [
    i64 0, label %bb3
    i64 1, label %bb1
  ]

bb2:                                              ; preds = %start
  unreachable

bb3:                                              ; preds = %start
  %6 = bitcast { i64, i64 }* %self to %"core::result::Result<usize, core::alloc::layout::LayoutError>::Ok"*
  %7 = getelementptr inbounds %"core::result::Result<usize, core::alloc::layout::LayoutError>::Ok", %"core::result::Result<usize, core::alloc::layout::LayoutError>::Ok"* %6, i32 0, i32 1
  %v = load i64, i64* %7, align 8
  %8 = bitcast { i64, i64 }* %2 to %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, core::alloc::layout::LayoutError>, usize>::Continue"*
  %9 = getelementptr inbounds %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, core::alloc::layout::LayoutError>, usize>::Continue", %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, core::alloc::layout::LayoutError>, usize>::Continue"* %8, i32 0, i32 1
  store i64 %v, i64* %9, align 8
  %10 = bitcast { i64, i64 }* %2 to i64*
  store i64 0, i64* %10, align 8
  br label %bb4

bb1:                                              ; preds = %start
  %11 = bitcast %"core::result::Result<core::convert::Infallible, core::alloc::layout::LayoutError>::Err"* %_6 to %"core::alloc::layout::LayoutError"*
  %12 = bitcast { i64, i64 }* %2 to %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, core::alloc::layout::LayoutError>, usize>::Break"*
  %13 = getelementptr inbounds %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, core::alloc::layout::LayoutError>, usize>::Break", %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, core::alloc::layout::LayoutError>, usize>::Break"* %12, i32 0, i32 1
  %14 = bitcast { i64, i64 }* %2 to i64*
  store i64 1, i64* %14, align 8
  br label %bb4

bb4:                                              ; preds = %bb3, %bb1
  %15 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %2, i32 0, i32 0
  %16 = load i64, i64* %15, align 8, !range !6
  %17 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %2, i32 0, i32 1
  %18 = load i64, i64* %17, align 8
  %19 = insertvalue { i64, i64 } undef, i64 %16, 0
  %20 = insertvalue { i64, i64 } %19, i64 %18, 1
  ret { i64, i64 } %20
}

; <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
; Function Attrs: inlinehint nounwind nonlazybind
define internal { i8*, i64 } @"_ZN79_$LT$core..result..Result$LT$T$C$E$GT$$u20$as$u20$core..ops..try_trait..Try$GT$6branch17h1e85c060f3c552f7E"(i8* %0, i64 %1) unnamed_addr #0 {
start:
  %_6 = alloca %"core::result::Result<core::convert::Infallible, core::alloc::AllocError>::Err", align 1
  %2 = alloca { i8*, i64 }, align 8
  %self = alloca { i8*, i64 }, align 8
  %3 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %self, i32 0, i32 0
  store i8* %0, i8** %3, align 8
  %4 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %self, i32 0, i32 1
  store i64 %1, i64* %4, align 8
  %5 = bitcast { i8*, i64 }* %self to {}**
  %6 = load {}*, {}** %5, align 8
  %7 = icmp eq {}* %6, null
  %_2 = select i1 %7, i64 1, i64 0
  switch i64 %_2, label %bb2 [
    i64 0, label %bb3
    i64 1, label %bb1
  ]

bb2:                                              ; preds = %start
  unreachable

bb3:                                              ; preds = %start
  %8 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %self, i32 0, i32 0
  %v.0 = load i8*, i8** %8, align 8, !nonnull !3
  %9 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %self, i32 0, i32 1
  %v.1 = load i64, i64* %9, align 8
  %10 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %2, i32 0, i32 0
  store i8* %v.0, i8** %10, align 8
  %11 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %2, i32 0, i32 1
  store i64 %v.1, i64* %11, align 8
  br label %bb4

bb1:                                              ; preds = %start
  %12 = bitcast %"core::result::Result<core::convert::Infallible, core::alloc::AllocError>::Err"* %_6 to %"core::alloc::AllocError"*
  %13 = bitcast { i8*, i64 }* %2 to %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, core::alloc::AllocError>, core::ptr::non_null::NonNull<[u8]>>::Break"*
  %14 = bitcast %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, core::alloc::AllocError>, core::ptr::non_null::NonNull<[u8]>>::Break"* %13 to %"core::result::Result<core::convert::Infallible, core::alloc::AllocError>::Err"*
  %15 = bitcast { i8*, i64 }* %2 to {}**
  store {}* null, {}** %15, align 8
  br label %bb4

bb4:                                              ; preds = %bb3, %bb1
  %16 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %2, i32 0, i32 0
  %17 = load i8*, i8** %16, align 8
  %18 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %2, i32 0, i32 1
  %19 = load i64, i64* %18, align 8
  %20 = insertvalue { i8*, i64 } undef, i8* %17, 0
  %21 = insertvalue { i8*, i64 } %20, i64 %19, 1
  ret { i8*, i64 } %21
}

; <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
; Function Attrs: inlinehint nounwind nonlazybind
define internal void @"_ZN79_$LT$core..result..Result$LT$T$C$E$GT$$u20$as$u20$core..ops..try_trait..Try$GT$6branch17h65a9c1ea517ae954E"(%"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveError>>"* noalias nocapture sret(%"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveError>>") dereferenceable(24) %0, %"core::result::Result<(), alloc::collections::TryReserveError>"* noalias nocapture dereferenceable(24) %self) unnamed_addr #0 {
start:
  %_6 = alloca { i64, i64 }, align 8
  %1 = bitcast %"core::result::Result<(), alloc::collections::TryReserveError>"* %self to i64*
  %_2 = load i64, i64* %1, align 8, !range !6
  switch i64 %_2, label %bb2 [
    i64 0, label %bb3
    i64 1, label %bb1
  ]

bb2:                                              ; preds = %start
  unreachable

bb3:                                              ; preds = %start
  %2 = bitcast %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveError>>"* %0 to %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveError>>::Continue"*
  %3 = getelementptr inbounds %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveError>>::Continue", %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveError>>::Continue"* %2, i32 0, i32 1
  %4 = bitcast %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveError>>"* %0 to i64*
  store i64 0, i64* %4, align 8
  br label %bb4

bb1:                                              ; preds = %start
  %5 = bitcast %"core::result::Result<(), alloc::collections::TryReserveError>"* %self to %"core::result::Result<(), alloc::collections::TryReserveError>::Err"*
  %6 = getelementptr inbounds %"core::result::Result<(), alloc::collections::TryReserveError>::Err", %"core::result::Result<(), alloc::collections::TryReserveError>::Err"* %5, i32 0, i32 1
  %7 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %6, i32 0, i32 0
  %e.0 = load i64, i64* %7, align 8
  %8 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %6, i32 0, i32 1
  %e.1 = load i64, i64* %8, align 8
  %9 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %_6, i32 0, i32 0
  store i64 %e.0, i64* %9, align 8
  %10 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %_6, i32 0, i32 1
  store i64 %e.1, i64* %10, align 8
  %11 = bitcast %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveError>>"* %0 to %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveError>>::Break"*
  %12 = getelementptr inbounds %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveError>>::Break", %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveError>>::Break"* %11, i32 0, i32 1
  %13 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %_6, i32 0, i32 0
  %14 = load i64, i64* %13, align 8
  %15 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %_6, i32 0, i32 1
  %16 = load i64, i64* %15, align 8
  %17 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %12, i32 0, i32 0
  store i64 %14, i64* %17, align 8
  %18 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %12, i32 0, i32 1
  store i64 %16, i64* %18, align 8
  %19 = bitcast %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveError>>"* %0 to i64*
  store i64 1, i64* %19, align 8
  br label %bb4

bb4:                                              ; preds = %bb3, %bb1
  ret void
}

; <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
; Function Attrs: inlinehint nounwind nonlazybind
define internal i8* @"_ZN79_$LT$core..result..Result$LT$T$C$E$GT$$u20$as$u20$core..ops..try_trait..Try$GT$6branch17h667d37608974688bE"(i8* %0) unnamed_addr #0 {
start:
  %_6 = alloca %"core::result::Result<core::convert::Infallible, core::alloc::AllocError>::Err", align 1
  %1 = alloca i8*, align 8
  %self = alloca i8*, align 8
  store i8* %0, i8** %self, align 8
  %2 = bitcast i8** %self to {}**
  %3 = load {}*, {}** %2, align 8
  %4 = icmp eq {}* %3, null
  %_2 = select i1 %4, i64 1, i64 0
  switch i64 %_2, label %bb2 [
    i64 0, label %bb3
    i64 1, label %bb1
  ]

bb2:                                              ; preds = %start
  unreachable

bb3:                                              ; preds = %start
  %v = load i8*, i8** %self, align 8, !nonnull !3
  store i8* %v, i8** %1, align 8
  br label %bb4

bb1:                                              ; preds = %start
  %5 = bitcast %"core::result::Result<core::convert::Infallible, core::alloc::AllocError>::Err"* %_6 to %"core::alloc::AllocError"*
  %6 = bitcast i8** %1 to %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, core::alloc::AllocError>, core::ptr::non_null::NonNull<u8>>::Break"*
  %7 = bitcast %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, core::alloc::AllocError>, core::ptr::non_null::NonNull<u8>>::Break"* %6 to %"core::result::Result<core::convert::Infallible, core::alloc::AllocError>::Err"*
  %8 = bitcast i8** %1 to {}**
  store {}* null, {}** %8, align 8
  br label %bb4

bb4:                                              ; preds = %bb3, %bb1
  %9 = load i8*, i8** %1, align 8
  ret i8* %9
}

; <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
; Function Attrs: inlinehint nounwind nonlazybind
define internal void @"_ZN79_$LT$core..result..Result$LT$T$C$E$GT$$u20$as$u20$core..ops..try_trait..Try$GT$6branch17h7f9906f35b565ee6E"(%"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveErrorKind>, core::alloc::layout::Layout>"* noalias nocapture sret(%"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveErrorKind>, core::alloc::layout::Layout>") dereferenceable(24) %0, %"core::result::Result<core::alloc::layout::Layout, alloc::collections::TryReserveErrorKind>"* noalias nocapture dereferenceable(24) %self) unnamed_addr #0 {
start:
  %_6 = alloca { i64, i64 }, align 8
  %1 = bitcast %"core::result::Result<core::alloc::layout::Layout, alloc::collections::TryReserveErrorKind>"* %self to i64*
  %_2 = load i64, i64* %1, align 8, !range !6
  switch i64 %_2, label %bb2 [
    i64 0, label %bb3
    i64 1, label %bb1
  ]

bb2:                                              ; preds = %start
  unreachable

bb3:                                              ; preds = %start
  %2 = bitcast %"core::result::Result<core::alloc::layout::Layout, alloc::collections::TryReserveErrorKind>"* %self to %"core::result::Result<core::alloc::layout::Layout, alloc::collections::TryReserveErrorKind>::Ok"*
  %3 = getelementptr inbounds %"core::result::Result<core::alloc::layout::Layout, alloc::collections::TryReserveErrorKind>::Ok", %"core::result::Result<core::alloc::layout::Layout, alloc::collections::TryReserveErrorKind>::Ok"* %2, i32 0, i32 1
  %4 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %3, i32 0, i32 0
  %v.0 = load i64, i64* %4, align 8
  %5 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %3, i32 0, i32 1
  %v.1 = load i64, i64* %5, align 8, !range !7
  %6 = bitcast %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveErrorKind>, core::alloc::layout::Layout>"* %0 to %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveErrorKind>, core::alloc::layout::Layout>::Continue"*
  %7 = getelementptr inbounds %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveErrorKind>, core::alloc::layout::Layout>::Continue", %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveErrorKind>, core::alloc::layout::Layout>::Continue"* %6, i32 0, i32 1
  %8 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %7, i32 0, i32 0
  store i64 %v.0, i64* %8, align 8
  %9 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %7, i32 0, i32 1
  store i64 %v.1, i64* %9, align 8
  %10 = bitcast %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveErrorKind>, core::alloc::layout::Layout>"* %0 to i64*
  store i64 0, i64* %10, align 8
  br label %bb4

bb1:                                              ; preds = %start
  %11 = bitcast %"core::result::Result<core::alloc::layout::Layout, alloc::collections::TryReserveErrorKind>"* %self to %"core::result::Result<core::alloc::layout::Layout, alloc::collections::TryReserveErrorKind>::Err"*
  %12 = getelementptr inbounds %"core::result::Result<core::alloc::layout::Layout, alloc::collections::TryReserveErrorKind>::Err", %"core::result::Result<core::alloc::layout::Layout, alloc::collections::TryReserveErrorKind>::Err"* %11, i32 0, i32 1
  %13 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %12, i32 0, i32 0
  %e.0 = load i64, i64* %13, align 8
  %14 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %12, i32 0, i32 1
  %e.1 = load i64, i64* %14, align 8
  %15 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %_6, i32 0, i32 0
  store i64 %e.0, i64* %15, align 8
  %16 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %_6, i32 0, i32 1
  store i64 %e.1, i64* %16, align 8
  %17 = bitcast %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveErrorKind>, core::alloc::layout::Layout>"* %0 to %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveErrorKind>, core::alloc::layout::Layout>::Break"*
  %18 = getelementptr inbounds %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveErrorKind>, core::alloc::layout::Layout>::Break", %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveErrorKind>, core::alloc::layout::Layout>::Break"* %17, i32 0, i32 1
  %19 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %_6, i32 0, i32 0
  %20 = load i64, i64* %19, align 8
  %21 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %_6, i32 0, i32 1
  %22 = load i64, i64* %21, align 8
  %23 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %18, i32 0, i32 0
  store i64 %20, i64* %23, align 8
  %24 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %18, i32 0, i32 1
  store i64 %22, i64* %24, align 8
  %25 = bitcast %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveErrorKind>, core::alloc::layout::Layout>"* %0 to i64*
  store i64 1, i64* %25, align 8
  br label %bb4

bb4:                                              ; preds = %bb3, %bb1
  ret void
}

; <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
; Function Attrs: inlinehint nounwind nonlazybind
define internal void @"_ZN79_$LT$core..result..Result$LT$T$C$E$GT$$u20$as$u20$core..ops..try_trait..Try$GT$6branch17h91d8513c54bc4f35E"(%"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveErrorKind>, usize>"* noalias nocapture sret(%"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveErrorKind>, usize>") dereferenceable(24) %0, %"core::result::Result<usize, alloc::collections::TryReserveErrorKind>"* noalias nocapture dereferenceable(24) %self) unnamed_addr #0 {
start:
  %_6 = alloca { i64, i64 }, align 8
  %1 = bitcast %"core::result::Result<usize, alloc::collections::TryReserveErrorKind>"* %self to i64*
  %_2 = load i64, i64* %1, align 8, !range !6
  switch i64 %_2, label %bb2 [
    i64 0, label %bb3
    i64 1, label %bb1
  ]

bb2:                                              ; preds = %start
  unreachable

bb3:                                              ; preds = %start
  %2 = bitcast %"core::result::Result<usize, alloc::collections::TryReserveErrorKind>"* %self to %"core::result::Result<usize, alloc::collections::TryReserveErrorKind>::Ok"*
  %3 = getelementptr inbounds %"core::result::Result<usize, alloc::collections::TryReserveErrorKind>::Ok", %"core::result::Result<usize, alloc::collections::TryReserveErrorKind>::Ok"* %2, i32 0, i32 1
  %v = load i64, i64* %3, align 8
  %4 = bitcast %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveErrorKind>, usize>"* %0 to %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveErrorKind>, usize>::Continue"*
  %5 = getelementptr inbounds %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveErrorKind>, usize>::Continue", %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveErrorKind>, usize>::Continue"* %4, i32 0, i32 1
  store i64 %v, i64* %5, align 8
  %6 = bitcast %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveErrorKind>, usize>"* %0 to i64*
  store i64 0, i64* %6, align 8
  br label %bb4

bb1:                                              ; preds = %start
  %7 = bitcast %"core::result::Result<usize, alloc::collections::TryReserveErrorKind>"* %self to %"core::result::Result<usize, alloc::collections::TryReserveErrorKind>::Err"*
  %8 = getelementptr inbounds %"core::result::Result<usize, alloc::collections::TryReserveErrorKind>::Err", %"core::result::Result<usize, alloc::collections::TryReserveErrorKind>::Err"* %7, i32 0, i32 1
  %9 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %8, i32 0, i32 0
  %e.0 = load i64, i64* %9, align 8
  %10 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %8, i32 0, i32 1
  %e.1 = load i64, i64* %10, align 8
  %11 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %_6, i32 0, i32 0
  store i64 %e.0, i64* %11, align 8
  %12 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %_6, i32 0, i32 1
  store i64 %e.1, i64* %12, align 8
  %13 = bitcast %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveErrorKind>, usize>"* %0 to %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveErrorKind>, usize>::Break"*
  %14 = getelementptr inbounds %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveErrorKind>, usize>::Break", %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveErrorKind>, usize>::Break"* %13, i32 0, i32 1
  %15 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %_6, i32 0, i32 0
  %16 = load i64, i64* %15, align 8
  %17 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %_6, i32 0, i32 1
  %18 = load i64, i64* %17, align 8
  %19 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %14, i32 0, i32 0
  store i64 %16, i64* %19, align 8
  %20 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %14, i32 0, i32 1
  store i64 %18, i64* %20, align 8
  %21 = bitcast %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, alloc::collections::TryReserveErrorKind>, usize>"* %0 to i64*
  store i64 1, i64* %21, align 8
  br label %bb4

bb4:                                              ; preds = %bb3, %bb1
  ret void
}

; <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
; Function Attrs: inlinehint nounwind nonlazybind
define internal void @"_ZN153_$LT$core..result..Result$LT$T$C$F$GT$$u20$as$u20$core..ops..try_trait..FromResidual$LT$core..result..Result$LT$core..convert..Infallible$C$E$GT$$GT$$GT$13from_residual17h0c3a62696a58b0e2E"(%"core::result::Result<core::ptr::non_null::NonNull<[u8]>, alloc::collections::TryReserveError>"* noalias nocapture sret(%"core::result::Result<core::ptr::non_null::NonNull<[u8]>, alloc::collections::TryReserveError>") dereferenceable(24) %0, i64 %1, i64 %2, %"core::panic::location::Location"* align 8 dereferenceable(24) %3) unnamed_addr #0 {
start:
  %residual = alloca { i64, i64 }, align 8
  %4 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %residual, i32 0, i32 0
  store i64 %1, i64* %4, align 8
  %5 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %residual, i32 0, i32 1
  store i64 %2, i64* %5, align 8
  %6 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %residual, i32 0, i32 0
  %e.0 = load i64, i64* %6, align 8
  %7 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %residual, i32 0, i32 1
  %e.1 = load i64, i64* %7, align 8
; call <alloc::collections::TryReserveError as core::convert::From<alloc::collections::TryReserveErrorKind>>::from
  %8 = call { i64, i64 } @"_ZN122_$LT$alloc..collections..TryReserveError$u20$as$u20$core..convert..From$LT$alloc..collections..TryReserveErrorKind$GT$$GT$4from17hcee37011040de3f3E"(i64 %e.0, i64 %e.1) #11
  %_3.0 = extractvalue { i64, i64 } %8, 0
  %_3.1 = extractvalue { i64, i64 } %8, 1
  br label %bb1

bb1:                                              ; preds = %start
  %9 = bitcast %"core::result::Result<core::ptr::non_null::NonNull<[u8]>, alloc::collections::TryReserveError>"* %0 to %"core::result::Result<core::ptr::non_null::NonNull<[u8]>, alloc::collections::TryReserveError>::Err"*
  %10 = getelementptr inbounds %"core::result::Result<core::ptr::non_null::NonNull<[u8]>, alloc::collections::TryReserveError>::Err", %"core::result::Result<core::ptr::non_null::NonNull<[u8]>, alloc::collections::TryReserveError>::Err"* %9, i32 0, i32 1
  %11 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %10, i32 0, i32 0
  store i64 %_3.0, i64* %11, align 8
  %12 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %10, i32 0, i32 1
  store i64 %_3.1, i64* %12, align 8
  %13 = bitcast %"core::result::Result<core::ptr::non_null::NonNull<[u8]>, alloc::collections::TryReserveError>"* %0 to i64*
  store i64 1, i64* %13, align 8
  ret void
}

; <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
; Function Attrs: inlinehint nounwind nonlazybind
define internal void @"_ZN153_$LT$core..result..Result$LT$T$C$F$GT$$u20$as$u20$core..ops..try_trait..FromResidual$LT$core..result..Result$LT$core..convert..Infallible$C$E$GT$$GT$$GT$13from_residual17h3b6176bcfb7719dcE"(%"core::result::Result<(), alloc::collections::TryReserveError>"* noalias nocapture sret(%"core::result::Result<(), alloc::collections::TryReserveError>") dereferenceable(24) %0, i64 %1, i64 %2, %"core::panic::location::Location"* align 8 dereferenceable(24) %3) unnamed_addr #0 {
start:
  %residual = alloca { i64, i64 }, align 8
  %4 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %residual, i32 0, i32 0
  store i64 %1, i64* %4, align 8
  %5 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %residual, i32 0, i32 1
  store i64 %2, i64* %5, align 8
  %6 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %residual, i32 0, i32 0
  %e.0 = load i64, i64* %6, align 8
  %7 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %residual, i32 0, i32 1
  %e.1 = load i64, i64* %7, align 8
; call <T as core::convert::From<T>>::from
  %8 = call { i64, i64 } @"_ZN50_$LT$T$u20$as$u20$core..convert..From$LT$T$GT$$GT$4from17h0c870bb39bb6f75eE"(i64 %e.0, i64 %e.1) #11
  %_3.0 = extractvalue { i64, i64 } %8, 0
  %_3.1 = extractvalue { i64, i64 } %8, 1
  br label %bb1

bb1:                                              ; preds = %start
  %9 = bitcast %"core::result::Result<(), alloc::collections::TryReserveError>"* %0 to %"core::result::Result<(), alloc::collections::TryReserveError>::Err"*
  %10 = getelementptr inbounds %"core::result::Result<(), alloc::collections::TryReserveError>::Err", %"core::result::Result<(), alloc::collections::TryReserveError>::Err"* %9, i32 0, i32 1
  %11 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %10, i32 0, i32 0
  store i64 %_3.0, i64* %11, align 8
  %12 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %10, i32 0, i32 1
  store i64 %_3.1, i64* %12, align 8
  %13 = bitcast %"core::result::Result<(), alloc::collections::TryReserveError>"* %0 to i64*
  store i64 1, i64* %13, align 8
  ret void
}

; <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
; Function Attrs: inlinehint nounwind nonlazybind
define internal { i8*, i64 } @"_ZN153_$LT$core..result..Result$LT$T$C$F$GT$$u20$as$u20$core..ops..try_trait..FromResidual$LT$core..result..Result$LT$core..convert..Infallible$C$E$GT$$GT$$GT$13from_residual17h7338d4240b4d5599E"(%"core::panic::location::Location"* align 8 dereferenceable(24) %0) unnamed_addr #0 {
start:
  %1 = alloca { i8*, i64 }, align 8
; call <T as core::convert::From<T>>::from
  call void @"_ZN50_$LT$T$u20$as$u20$core..convert..From$LT$T$GT$$GT$4from17h1abfc401a4fea46eE"() #11
  br label %bb1

bb1:                                              ; preds = %start
  %2 = bitcast { i8*, i64 }* %1 to %"core::result::Result<core::ptr::non_null::NonNull<[u8]>, core::alloc::AllocError>::Err"*
  %3 = bitcast %"core::result::Result<core::ptr::non_null::NonNull<[u8]>, core::alloc::AllocError>::Err"* %2 to %"core::alloc::AllocError"*
  %4 = bitcast { i8*, i64 }* %1 to {}**
  store {}* null, {}** %4, align 8
  %5 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %1, i32 0, i32 0
  %6 = load i8*, i8** %5, align 8
  %7 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %1, i32 0, i32 1
  %8 = load i64, i64* %7, align 8
  %9 = insertvalue { i8*, i64 } undef, i8* %6, 0
  %10 = insertvalue { i8*, i64 } %9, i64 %8, 1
  ret { i8*, i64 } %10
}

; <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
; Function Attrs: inlinehint nounwind nonlazybind
define internal void @"_ZN153_$LT$core..result..Result$LT$T$C$F$GT$$u20$as$u20$core..ops..try_trait..FromResidual$LT$core..result..Result$LT$core..convert..Infallible$C$E$GT$$GT$$GT$13from_residual17haa158d46f4a111f3E"(%"core::result::Result<(), alloc::collections::TryReserveError>"* noalias nocapture sret(%"core::result::Result<(), alloc::collections::TryReserveError>") dereferenceable(24) %0, i64 %1, i64 %2, %"core::panic::location::Location"* align 8 dereferenceable(24) %3) unnamed_addr #0 {
start:
  %residual = alloca { i64, i64 }, align 8
  %4 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %residual, i32 0, i32 0
  store i64 %1, i64* %4, align 8
  %5 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %residual, i32 0, i32 1
  store i64 %2, i64* %5, align 8
  %6 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %residual, i32 0, i32 0
  %e.0 = load i64, i64* %6, align 8
  %7 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %residual, i32 0, i32 1
  %e.1 = load i64, i64* %7, align 8
; call <alloc::collections::TryReserveError as core::convert::From<alloc::collections::TryReserveErrorKind>>::from
  %8 = call { i64, i64 } @"_ZN122_$LT$alloc..collections..TryReserveError$u20$as$u20$core..convert..From$LT$alloc..collections..TryReserveErrorKind$GT$$GT$4from17hcee37011040de3f3E"(i64 %e.0, i64 %e.1) #11
  %_3.0 = extractvalue { i64, i64 } %8, 0
  %_3.1 = extractvalue { i64, i64 } %8, 1
  br label %bb1

bb1:                                              ; preds = %start
  %9 = bitcast %"core::result::Result<(), alloc::collections::TryReserveError>"* %0 to %"core::result::Result<(), alloc::collections::TryReserveError>::Err"*
  %10 = getelementptr inbounds %"core::result::Result<(), alloc::collections::TryReserveError>::Err", %"core::result::Result<(), alloc::collections::TryReserveError>::Err"* %9, i32 0, i32 1
  %11 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %10, i32 0, i32 0
  store i64 %_3.0, i64* %11, align 8
  %12 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %10, i32 0, i32 1
  store i64 %_3.1, i64* %12, align 8
  %13 = bitcast %"core::result::Result<(), alloc::collections::TryReserveError>"* %0 to i64*
  store i64 1, i64* %13, align 8
  ret void
}

; <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
; Function Attrs: inlinehint nounwind nonlazybind
define internal { i64, i64 } @"_ZN153_$LT$core..result..Result$LT$T$C$F$GT$$u20$as$u20$core..ops..try_trait..FromResidual$LT$core..result..Result$LT$core..convert..Infallible$C$E$GT$$GT$$GT$13from_residual17hb9752d2d6b65f17bE"(%"core::panic::location::Location"* align 8 dereferenceable(24) %0) unnamed_addr #0 {
start:
  %1 = alloca { i64, i64 }, align 8
; call <T as core::convert::From<T>>::from
  call void @"_ZN50_$LT$T$u20$as$u20$core..convert..From$LT$T$GT$$GT$4from17he66e1966cc279e3eE"() #11
  br label %bb1

bb1:                                              ; preds = %start
  %2 = bitcast { i64, i64 }* %1 to %"core::result::Result<core::alloc::layout::Layout, core::alloc::layout::LayoutError>::Err"*
  %3 = bitcast %"core::result::Result<core::alloc::layout::Layout, core::alloc::layout::LayoutError>::Err"* %2 to %"core::alloc::layout::LayoutError"*
  %4 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %1, i32 0, i32 1
  store i64 0, i64* %4, align 8
  %5 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %1, i32 0, i32 0
  %6 = load i64, i64* %5, align 8
  %7 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %1, i32 0, i32 1
  %8 = load i64, i64* %7, align 8
  %9 = insertvalue { i64, i64 } undef, i64 %6, 0
  %10 = insertvalue { i64, i64 } %9, i64 %8, 1
  ret { i64, i64 } %10
}

; <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
; Function Attrs: inlinehint nounwind nonlazybind
define internal void @"_ZN153_$LT$core..result..Result$LT$T$C$F$GT$$u20$as$u20$core..ops..try_trait..FromResidual$LT$core..result..Result$LT$core..convert..Infallible$C$E$GT$$GT$$GT$13from_residual17hf7b0ca1e6476cb8cE"(%"core::result::Result<core::ptr::non_null::NonNull<[u8]>, alloc::collections::TryReserveError>"* noalias nocapture sret(%"core::result::Result<core::ptr::non_null::NonNull<[u8]>, alloc::collections::TryReserveError>") dereferenceable(24) %0, i64 %1, i64 %2, %"core::panic::location::Location"* align 8 dereferenceable(24) %3) unnamed_addr #0 {
start:
  %residual = alloca { i64, i64 }, align 8
  %4 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %residual, i32 0, i32 0
  store i64 %1, i64* %4, align 8
  %5 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %residual, i32 0, i32 1
  store i64 %2, i64* %5, align 8
  %6 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %residual, i32 0, i32 0
  %e.0 = load i64, i64* %6, align 8
  %7 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %residual, i32 0, i32 1
  %e.1 = load i64, i64* %7, align 8
; call <T as core::convert::From<T>>::from
  %8 = call { i64, i64 } @"_ZN50_$LT$T$u20$as$u20$core..convert..From$LT$T$GT$$GT$4from17h0c870bb39bb6f75eE"(i64 %e.0, i64 %e.1) #11
  %_3.0 = extractvalue { i64, i64 } %8, 0
  %_3.1 = extractvalue { i64, i64 } %8, 1
  br label %bb1

bb1:                                              ; preds = %start
  %9 = bitcast %"core::result::Result<core::ptr::non_null::NonNull<[u8]>, alloc::collections::TryReserveError>"* %0 to %"core::result::Result<core::ptr::non_null::NonNull<[u8]>, alloc::collections::TryReserveError>::Err"*
  %10 = getelementptr inbounds %"core::result::Result<core::ptr::non_null::NonNull<[u8]>, alloc::collections::TryReserveError>::Err", %"core::result::Result<core::ptr::non_null::NonNull<[u8]>, alloc::collections::TryReserveError>::Err"* %9, i32 0, i32 1
  %11 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %10, i32 0, i32 0
  store i64 %_3.0, i64* %11, align 8
  %12 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %10, i32 0, i32 1
  store i64 %_3.1, i64* %12, align 8
  %13 = bitcast %"core::result::Result<core::ptr::non_null::NonNull<[u8]>, alloc::collections::TryReserveError>"* %0 to i64*
  store i64 1, i64* %13, align 8
  ret void
}

; std::rt::lang_start
; Function Attrs: nounwind nonlazybind
define hidden i64 @_ZN3std2rt10lang_start17h22d2c127fcd1cc68E(void ()* nonnull %main, i64 %argc, i8** %argv) unnamed_addr #1 {
start:
  %_8 = alloca i64*, align 8
  %_4 = alloca i64, align 8
  %0 = bitcast i64** %_8 to void ()**
  store void ()* %main, void ()** %0, align 8
  %_5.0 = bitcast i64** %_8 to {}*
; call std::rt::lang_start_internal
  %1 = call i64 @_ZN3std2rt19lang_start_internal17h52e73755f77c7dd9E({}* nonnull align 1 %_5.0, [3 x i64]* align 8 dereferenceable(24) bitcast (<{ i8*, [16 x i8], i8*, i8*, i8*, [0 x i8] }>* @vtable.0 to [3 x i64]*), i64 %argc, i8** %argv) #11
  store i64 %1, i64* %_4, align 8
  br label %bb1

bb1:                                              ; preds = %start
  %v = load i64, i64* %_4, align 8
  ret i64 %v
}

; std::rt::lang_start::{{closure}}
; Function Attrs: inlinehint nounwind nonlazybind
define internal i32 @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h370f508993df7f7eE"(i64** align 8 dereferenceable(8) %_1) unnamed_addr #0 {
start:
  %0 = bitcast i64** %_1 to void ()**
  %_3 = load void ()*, void ()** %0, align 8, !nonnull !3
; call std::sys_common::backtrace::__rust_begin_short_backtrace
  call void @_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h23d543a8170eb016E(void ()* nonnull %_3) #11
  br label %bb1

bb1:                                              ; preds = %start
; call <() as std::process::Termination>::report
  %1 = call i32 @"_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17he6bda9206bcab9a0E"() #11
  br label %bb2

bb2:                                              ; preds = %bb1
  ret i32 %1
}

; std::sys_common::backtrace::__rust_begin_short_backtrace
; Function Attrs: noinline nounwind nonlazybind
define internal void @_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h23d543a8170eb016E(void ()* nonnull %f) unnamed_addr #2 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
start:
  %0 = alloca { i8*, i32 }, align 8
; call core::ops::function::FnOnce::call_once
  call void @_ZN4core3ops8function6FnOnce9call_once17hbd0c30072a9b7299E(void ()* nonnull %f) #11
  br label %bb1

bb1:                                              ; preds = %start
; call core::hint::black_box
  call void @_ZN4core4hint9black_box17hc4074ee890bbb658E() #11
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

; core::alloc::layout::Layout::from_size_align_unchecked
; Function Attrs: inlinehint nounwind nonlazybind
define internal { i64, i64 } @_ZN4core5alloc6layout6Layout25from_size_align_unchecked17hed3a5110f94d0efeE(i64 %size, i64 %align) unnamed_addr #0 {
start:
  %0 = alloca { i64, i64 }, align 8
; call core::num::nonzero::NonZeroUsize::new_unchecked
  %_4 = call i64 @_ZN4core3num7nonzero12NonZeroUsize13new_unchecked17h79e7726c57306d85E(i64 %align) #11, !range !7
  br label %bb1

bb1:                                              ; preds = %start
  %1 = bitcast { i64, i64 }* %0 to i64*
  store i64 %size, i64* %1, align 8
  %2 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %0, i32 0, i32 1
  store i64 %_4, i64* %2, align 8
  %3 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %0, i32 0, i32 0
  %4 = load i64, i64* %3, align 8
  %5 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %0, i32 0, i32 1
  %6 = load i64, i64* %5, align 8, !range !7
  %7 = insertvalue { i64, i64 } undef, i64 %4, 0
  %8 = insertvalue { i64, i64 } %7, i64 %6, 1
  ret { i64, i64 } %8
}

; core::alloc::layout::Layout::size
; Function Attrs: inlinehint nounwind nonlazybind
define internal i64 @_ZN4core5alloc6layout6Layout4size17h081c8035e28220cdE({ i64, i64 }* align 8 dereferenceable(16) %self) unnamed_addr #0 {
start:
  %0 = bitcast { i64, i64 }* %self to i64*
  %1 = load i64, i64* %0, align 8
  ret i64 %1
}

; core::alloc::layout::Layout::align
; Function Attrs: inlinehint nounwind nonlazybind
define internal i64 @_ZN4core5alloc6layout6Layout5align17he5ae42b9af0cb70fE({ i64, i64 }* align 8 dereferenceable(16) %self) unnamed_addr #0 {
start:
  %0 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %self, i32 0, i32 1
  %_2 = load i64, i64* %0, align 8, !range !7
; call core::num::nonzero::NonZeroUsize::get
  %1 = call i64 @_ZN4core3num7nonzero12NonZeroUsize3get17h03520103c9110ca8E(i64 %_2) #11
  br label %bb1

bb1:                                              ; preds = %start
  ret i64 %1
}

; core::alloc::layout::Layout::dangling
; Function Attrs: inlinehint nounwind nonlazybind
define internal nonnull i8* @_ZN4core5alloc6layout6Layout8dangling17h2a26107f9d17f54fE({ i64, i64 }* align 8 dereferenceable(16) %self) unnamed_addr #0 {
start:
; call core::alloc::layout::Layout::align
  %_3 = call i64 @_ZN4core5alloc6layout6Layout5align17he5ae42b9af0cb70fE({ i64, i64 }* align 8 dereferenceable(16) %self) #11
  br label %bb1

bb1:                                              ; preds = %start
  %_2 = inttoptr i64 %_3 to i8*
; call core::ptr::non_null::NonNull<T>::new_unchecked
  %0 = call nonnull i8* @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked17hbce36cd865745a6dE"(i8* %_2) #11
  br label %bb2

bb2:                                              ; preds = %bb1
  ret i8* %0
}

; core::alloc::layout::Layout::array
; Function Attrs: inlinehint nounwind nonlazybind
define internal { i64, i64 } @_ZN4core5alloc6layout6Layout5array17h0e5a95af5beae033E(i64 %n) unnamed_addr #0 {
start:
  %_3 = alloca { i64, i64 }, align 8
  %0 = alloca { i64, i64 }, align 8
  br label %bb1

bb1:                                              ; preds = %start
; call core::num::<impl usize>::checked_mul
  %1 = call { i64, i64 } @"_ZN4core3num23_$LT$impl$u20$usize$GT$11checked_mul17h1b3d801de79639f4E"(i64 4, i64 %n) #11
  %_5.0 = extractvalue { i64, i64 } %1, 0
  %_5.1 = extractvalue { i64, i64 } %1, 1
  br label %bb2

bb2:                                              ; preds = %bb1
; call core::option::Option<T>::ok_or
  %2 = call { i64, i64 } @"_ZN4core6option15Option$LT$T$GT$5ok_or17h8372c131d912044bE"(i64 %_5.0, i64 %_5.1) #11
  %_4.0 = extractvalue { i64, i64 } %2, 0
  %_4.1 = extractvalue { i64, i64 } %2, 1
  br label %bb3

bb3:                                              ; preds = %bb2
; call <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
  %3 = call { i64, i64 } @"_ZN79_$LT$core..result..Result$LT$T$C$E$GT$$u20$as$u20$core..ops..try_trait..Try$GT$6branch17h17ea615427507abeE"(i64 %_4.0, i64 %_4.1) #11
  store { i64, i64 } %3, { i64, i64 }* %_3, align 8
  br label %bb4

bb4:                                              ; preds = %bb3
  %4 = bitcast { i64, i64 }* %_3 to i64*
  %_9 = load i64, i64* %4, align 8, !range !6
  switch i64 %_9, label %bb6 [
    i64 0, label %bb5
    i64 1, label %bb7
  ]

bb6:                                              ; preds = %bb4
  unreachable

bb5:                                              ; preds = %bb4
  %5 = bitcast { i64, i64 }* %_3 to %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, core::alloc::layout::LayoutError>, usize>::Continue"*
  %6 = getelementptr inbounds %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, core::alloc::layout::LayoutError>, usize>::Continue", %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, core::alloc::layout::LayoutError>, usize>::Continue"* %5, i32 0, i32 1
  %val = load i64, i64* %6, align 8
  br label %bb9

bb7:                                              ; preds = %bb4
; call <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
  %7 = call { i64, i64 } @"_ZN153_$LT$core..result..Result$LT$T$C$F$GT$$u20$as$u20$core..ops..try_trait..FromResidual$LT$core..result..Result$LT$core..convert..Infallible$C$E$GT$$GT$$GT$13from_residual17hb9752d2d6b65f17bE"(%"core::panic::location::Location"* align 8 dereferenceable(24) bitcast (<{ i8*, [16 x i8] }>* @alloc55 to %"core::panic::location::Location"*)) #11
  store { i64, i64 } %7, { i64, i64 }* %0, align 8
  br label %bb8

bb8:                                              ; preds = %bb7
  br label %bb11

bb11:                                             ; preds = %bb10, %bb8
  %8 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %0, i32 0, i32 0
  %9 = load i64, i64* %8, align 8
  %10 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %0, i32 0, i32 1
  %11 = load i64, i64* %10, align 8
  %12 = insertvalue { i64, i64 } undef, i64 %9, 0
  %13 = insertvalue { i64, i64 } %12, i64 %11, 1
  ret { i64, i64 } %13

bb9:                                              ; preds = %bb5
; call core::alloc::layout::Layout::from_size_align_unchecked
  %14 = call { i64, i64 } @_ZN4core5alloc6layout6Layout25from_size_align_unchecked17hed3a5110f94d0efeE(i64 %val, i64 4) #11
  %_13.0 = extractvalue { i64, i64 } %14, 0
  %_13.1 = extractvalue { i64, i64 } %14, 1
  br label %bb10

bb10:                                             ; preds = %bb9
  %15 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %0, i32 0, i32 0
  store i64 %_13.0, i64* %15, align 8
  %16 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %0, i32 0, i32 1
  store i64 %_13.1, i64* %16, align 8
  br label %bb11
}

; std::sys::unix::process::process_common::ExitCode::as_i32
; Function Attrs: inlinehint nounwind nonlazybind
define internal i32 @_ZN3std3sys4unix7process14process_common8ExitCode6as_i3217h1eadc22b4efb4f09E(i8* align 1 dereferenceable(1) %self) unnamed_addr #0 {
start:
  %_2 = load i8, i8* %self, align 1
  %0 = zext i8 %_2 to i32
  ret i32 %0
}

; core::num::nonzero::NonZeroUsize::new_unchecked
; Function Attrs: inlinehint nounwind nonlazybind
define internal i64 @_ZN4core3num7nonzero12NonZeroUsize13new_unchecked17h79e7726c57306d85E(i64 %n) unnamed_addr #0 {
start:
  %0 = alloca i64, align 8
  store i64 %n, i64* %0, align 8
  %1 = load i64, i64* %0, align 8, !range !7
  ret i64 %1
}

; core::num::nonzero::NonZeroUsize::get
; Function Attrs: inlinehint nounwind nonlazybind
define internal i64 @_ZN4core3num7nonzero12NonZeroUsize3get17h03520103c9110ca8E(i64 %self) unnamed_addr #0 {
start:
  ret i64 %self
}

; core::num::<impl usize>::checked_add
; Function Attrs: inlinehint nounwind nonlazybind
define internal { i64, i64 } @"_ZN4core3num23_$LT$impl$u20$usize$GT$11checked_add17h5629599ae0178fafE"(i64 %self, i64 %rhs) unnamed_addr #0 {
start:
  %0 = alloca { i64, i8 }, align 8
  %1 = alloca { i64, i8 }, align 8
  %2 = alloca i8, align 1
  %3 = alloca { i64, i64 }, align 8
  %4 = call { i64, i1 } @llvm.uadd.with.overflow.i64(i64 %self, i64 %rhs) #11
  %5 = extractvalue { i64, i1 } %4, 0
  %6 = extractvalue { i64, i1 } %4, 1
  %7 = zext i1 %6 to i8
  %8 = bitcast { i64, i8 }* %0 to i64*
  store i64 %5, i64* %8, align 8
  %9 = getelementptr inbounds { i64, i8 }, { i64, i8 }* %0, i32 0, i32 1
  store i8 %7, i8* %9, align 8
  %10 = getelementptr inbounds { i64, i8 }, { i64, i8 }* %0, i32 0, i32 0
  %_5.0.i = load i64, i64* %10, align 8
  %11 = getelementptr inbounds { i64, i8 }, { i64, i8 }* %0, i32 0, i32 1
  %12 = load i8, i8* %11, align 8, !range !5
  %_5.1.i = trunc i8 %12 to i1
  %13 = bitcast { i64, i8 }* %1 to i64*
  store i64 %_5.0.i, i64* %13, align 8
  %14 = getelementptr inbounds { i64, i8 }, { i64, i8 }* %1, i32 0, i32 1
  %15 = zext i1 %_5.1.i to i8
  store i8 %15, i8* %14, align 8
  %16 = getelementptr inbounds { i64, i8 }, { i64, i8 }* %1, i32 0, i32 0
  %17 = load i64, i64* %16, align 8
  %18 = getelementptr inbounds { i64, i8 }, { i64, i8 }* %1, i32 0, i32 1
  %19 = load i8, i8* %18, align 8, !range !5
  %20 = trunc i8 %19 to i1
  %21 = zext i1 %20 to i8
  %22 = insertvalue { i64, i8 } undef, i64 %17, 0
  %23 = insertvalue { i64, i8 } %22, i8 %21, 1
  %_5.0 = extractvalue { i64, i8 } %23, 0
  %24 = extractvalue { i64, i8 } %23, 1
  %_5.1 = trunc i8 %24 to i1
  br label %bb1

bb1:                                              ; preds = %start
  %25 = call i1 @llvm.expect.i1(i1 %_5.1, i1 false)
  %26 = zext i1 %25 to i8
  store i8 %26, i8* %2, align 1
  %27 = load i8, i8* %2, align 1, !range !5
  %_8 = trunc i8 %27 to i1
  br label %bb2

bb2:                                              ; preds = %bb1
  br i1 %_8, label %bb3, label %bb4

bb4:                                              ; preds = %bb2
  %28 = bitcast { i64, i64 }* %3 to %"core::option::Option<usize>::Some"*
  %29 = getelementptr inbounds %"core::option::Option<usize>::Some", %"core::option::Option<usize>::Some"* %28, i32 0, i32 1
  store i64 %_5.0, i64* %29, align 8
  %30 = bitcast { i64, i64 }* %3 to i64*
  store i64 1, i64* %30, align 8
  br label %bb5

bb3:                                              ; preds = %bb2
  %31 = bitcast { i64, i64 }* %3 to i64*
  store i64 0, i64* %31, align 8
  br label %bb5

bb5:                                              ; preds = %bb4, %bb3
  %32 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %3, i32 0, i32 0
  %33 = load i64, i64* %32, align 8, !range !6
  %34 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %3, i32 0, i32 1
  %35 = load i64, i64* %34, align 8
  %36 = insertvalue { i64, i64 } undef, i64 %33, 0
  %37 = insertvalue { i64, i64 } %36, i64 %35, 1
  ret { i64, i64 } %37
}

; core::num::<impl usize>::checked_mul
; Function Attrs: inlinehint nounwind nonlazybind
define internal { i64, i64 } @"_ZN4core3num23_$LT$impl$u20$usize$GT$11checked_mul17h1b3d801de79639f4E"(i64 %self, i64 %rhs) unnamed_addr #0 {
start:
  %0 = alloca { i64, i8 }, align 8
  %1 = alloca { i64, i8 }, align 8
  %2 = alloca i8, align 1
  %3 = alloca { i64, i64 }, align 8
  %4 = call { i64, i1 } @llvm.umul.with.overflow.i64(i64 %self, i64 %rhs) #11
  %5 = extractvalue { i64, i1 } %4, 0
  %6 = extractvalue { i64, i1 } %4, 1
  %7 = zext i1 %6 to i8
  %8 = bitcast { i64, i8 }* %0 to i64*
  store i64 %5, i64* %8, align 8
  %9 = getelementptr inbounds { i64, i8 }, { i64, i8 }* %0, i32 0, i32 1
  store i8 %7, i8* %9, align 8
  %10 = getelementptr inbounds { i64, i8 }, { i64, i8 }* %0, i32 0, i32 0
  %_5.0.i = load i64, i64* %10, align 8
  %11 = getelementptr inbounds { i64, i8 }, { i64, i8 }* %0, i32 0, i32 1
  %12 = load i8, i8* %11, align 8, !range !5
  %_5.1.i = trunc i8 %12 to i1
  %13 = bitcast { i64, i8 }* %1 to i64*
  store i64 %_5.0.i, i64* %13, align 8
  %14 = getelementptr inbounds { i64, i8 }, { i64, i8 }* %1, i32 0, i32 1
  %15 = zext i1 %_5.1.i to i8
  store i8 %15, i8* %14, align 8
  %16 = getelementptr inbounds { i64, i8 }, { i64, i8 }* %1, i32 0, i32 0
  %17 = load i64, i64* %16, align 8
  %18 = getelementptr inbounds { i64, i8 }, { i64, i8 }* %1, i32 0, i32 1
  %19 = load i8, i8* %18, align 8, !range !5
  %20 = trunc i8 %19 to i1
  %21 = zext i1 %20 to i8
  %22 = insertvalue { i64, i8 } undef, i64 %17, 0
  %23 = insertvalue { i64, i8 } %22, i8 %21, 1
  %_5.0 = extractvalue { i64, i8 } %23, 0
  %24 = extractvalue { i64, i8 } %23, 1
  %_5.1 = trunc i8 %24 to i1
  br label %bb1

bb1:                                              ; preds = %start
  %25 = call i1 @llvm.expect.i1(i1 %_5.1, i1 false)
  %26 = zext i1 %25 to i8
  store i8 %26, i8* %2, align 1
  %27 = load i8, i8* %2, align 1, !range !5
  %_8 = trunc i8 %27 to i1
  br label %bb2

bb2:                                              ; preds = %bb1
  br i1 %_8, label %bb3, label %bb4

bb4:                                              ; preds = %bb2
  %28 = bitcast { i64, i64 }* %3 to %"core::option::Option<usize>::Some"*
  %29 = getelementptr inbounds %"core::option::Option<usize>::Some", %"core::option::Option<usize>::Some"* %28, i32 0, i32 1
  store i64 %_5.0, i64* %29, align 8
  %30 = bitcast { i64, i64 }* %3 to i64*
  store i64 1, i64* %30, align 8
  br label %bb5

bb3:                                              ; preds = %bb2
  %31 = bitcast { i64, i64 }* %3 to i64*
  store i64 0, i64* %31, align 8
  br label %bb5

bb5:                                              ; preds = %bb4, %bb3
  %32 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %3, i32 0, i32 0
  %33 = load i64, i64* %32, align 8, !range !6
  %34 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %3, i32 0, i32 1
  %35 = load i64, i64* %34, align 8
  %36 = insertvalue { i64, i64 } undef, i64 %33, 0
  %37 = insertvalue { i64, i64 } %36, i64 %35, 1
  ret { i64, i64 } %37
}

; core::cmp::impls::<impl core::cmp::Ord for usize>::cmp
; Function Attrs: inlinehint nounwind nonlazybind
define internal i8 @"_ZN4core3cmp5impls50_$LT$impl$u20$core..cmp..Ord$u20$for$u20$usize$GT$3cmp17hef3ac0443fee4a2bE"(i64* align 8 dereferenceable(8) %self, i64* align 8 dereferenceable(8) %other) unnamed_addr #0 {
start:
  %0 = alloca i8, align 1
  %_4 = load i64, i64* %self, align 8
  %_5 = load i64, i64* %other, align 8
  %_3 = icmp ult i64 %_4, %_5
  br i1 %_3, label %bb1, label %bb2

bb2:                                              ; preds = %start
  %_7 = load i64, i64* %self, align 8
  %_8 = load i64, i64* %other, align 8
  %_6 = icmp eq i64 %_7, %_8
  br i1 %_6, label %bb3, label %bb4

bb1:                                              ; preds = %start
  store i8 -1, i8* %0, align 1
  br label %bb6

bb6:                                              ; preds = %bb5, %bb1
  %1 = load i8, i8* %0, align 1, !range !4
  ret i8 %1

bb4:                                              ; preds = %bb2
  store i8 1, i8* %0, align 1
  br label %bb5

bb3:                                              ; preds = %bb2
  store i8 0, i8* %0, align 1
  br label %bb5

bb5:                                              ; preds = %bb4, %bb3
  br label %bb6
}

; <core::alloc::layout::Layout as core::clone::Clone>::clone
; Function Attrs: inlinehint nounwind nonlazybind
define internal { i64, i64 } @"_ZN66_$LT$core..alloc..layout..Layout$u20$as$u20$core..clone..Clone$GT$5clone17h034b00774e006c4dE"({ i64, i64 }* align 8 dereferenceable(16) %self) unnamed_addr #0 {
start:
  %0 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %self, i32 0, i32 0
  %1 = load i64, i64* %0, align 8
  %2 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %self, i32 0, i32 1
  %3 = load i64, i64* %2, align 8, !range !7
  %4 = insertvalue { i64, i64 } undef, i64 %1, 0
  %5 = insertvalue { i64, i64 } %4, i64 %3, 1
  ret { i64, i64 } %5
}

; Function Attrs: argmemonly nofree nounwind willreturn
declare void @llvm.memcpy.p0i8.p0i8.i64(i8* noalias nocapture writeonly, i8* noalias nocapture readonly, i64, i1 immarg) #3

; Function Attrs: inaccessiblememonly nofree nosync nounwind willreturn
declare void @llvm.assume(i1 noundef) #4

; alloc::alloc::handle_alloc_error
; Function Attrs: cold noreturn nounwind nonlazybind
declare void @_ZN5alloc5alloc18handle_alloc_error17h0ece3c434a8e0eb4E(i64, i64) unnamed_addr #5

; alloc::raw_vec::capacity_overflow
; Function Attrs: noreturn nounwind nonlazybind
declare void @_ZN5alloc7raw_vec17capacity_overflow17h50ea6dbeac46bafcE() unnamed_addr #6

; Function Attrs: nounwind nonlazybind
declare noalias i8* @__rust_alloc(i64, i64) unnamed_addr #1

; Function Attrs: nounwind nonlazybind
declare void @__rust_dealloc(i8*, i64, i64) unnamed_addr #1

; Function Attrs: nounwind nonlazybind
declare i8* @__rust_realloc(i8*, i64, i64, i64) unnamed_addr #1

; Function Attrs: nounwind nonlazybind
declare i8* @__rust_alloc_zeroed(i64, i64) unnamed_addr #1

; Function Attrs: argmemonly nofree nounwind willreturn writeonly
declare void @llvm.memset.p0i8.i64(i8* nocapture writeonly, i8, i64, i1 immarg) #7

; Function Attrs: nounwind nonlazybind
declare i32 @rust_eh_personality(i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*) unnamed_addr #1

; std::rt::lang_start_internal
; Function Attrs: nounwind nonlazybind
declare i64 @_ZN3std2rt19lang_start_internal17h52e73755f77c7dd9E({}* nonnull align 1, [3 x i64]* align 8 dereferenceable(24), i64, i8**) unnamed_addr #1

; Function Attrs: nofree nosync nounwind readnone willreturn
declare i1 @llvm.expect.i1(i1, i1) #8

; Function Attrs: nofree nosync nounwind readnone speculatable willreturn
declare { i64, i1 } @llvm.uadd.with.overflow.i64(i64, i64) #9

; Function Attrs: nofree nosync nounwind readnone speculatable willreturn
declare { i64, i1 } @llvm.umul.with.overflow.i64(i64, i64) #9

; Function Attrs: nonlazybind
define i32 @main(i32 %0, i8** %1) unnamed_addr #10 {
top:
  %2 = sext i32 %0 to i64
; call std::rt::lang_start
  %3 = call i64 @_ZN3std2rt10lang_start17h22d2c127fcd1cc68E(void ()* @_ZN3vec4main17h0fff19dbb1ce78d1E, i64 %2, i8** %1)
  %4 = trunc i64 %3 to i32
  ret i32 %4
}

attributes #0 = { inlinehint nounwind nonlazybind "probe-stack"="__rust_probestack" "target-cpu"="x86-64" }
attributes #1 = { nounwind nonlazybind "probe-stack"="__rust_probestack" "target-cpu"="x86-64" }
attributes #2 = { noinline nounwind nonlazybind "probe-stack"="__rust_probestack" "target-cpu"="x86-64" }
attributes #3 = { argmemonly nofree nounwind willreturn }
attributes #4 = { inaccessiblememonly nofree nosync nounwind willreturn }
attributes #5 = { cold noreturn nounwind nonlazybind "probe-stack"="__rust_probestack" "target-cpu"="x86-64" }
attributes #6 = { noreturn nounwind nonlazybind "probe-stack"="__rust_probestack" "target-cpu"="x86-64" }
attributes #7 = { argmemonly nofree nounwind willreturn writeonly }
attributes #8 = { nofree nosync nounwind readnone willreturn }
attributes #9 = { nofree nosync nounwind readnone speculatable willreturn }
attributes #10 = { nonlazybind "target-cpu"="x86-64" }
attributes #11 = { nounwind }
attributes #12 = { noreturn nounwind }

!llvm.module.flags = !{!0, !1, !2}

!0 = !{i32 7, !"PIC Level", i32 2}
!1 = !{i32 7, !"PIE Level", i32 2}
!2 = !{i32 2, !"RtLibUseGOT", i32 1}
!3 = !{}
!4 = !{i8 -1, i8 2}
!5 = !{i8 0, i8 2}
!6 = !{i64 0, i64 2}
!7 = !{i64 1, i64 0}
!8 = !{i32 3212931}
