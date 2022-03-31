; ModuleID = 'locals.ebb8ff6e-cgu.0'
source_filename = "locals.ebb8ff6e-cgu.0"
target datalayout = "e-m:e-p:32:32-Fi8-i64:64-v128:64:128-a:0:32-n32-S64"
target triple = "thumbv7m-none-unknown-eabi"

%"rtic::RacyCell<heapless::spsc::Queue<(app::P1_T, u8), 3_usize>>" = type { %"core::cell::UnsafeCell<heapless::spsc::Queue<(app::P1_T, u8), 3_usize>>" }
%"core::cell::UnsafeCell<heapless::spsc::Queue<(app::P1_T, u8), 3_usize>>" = type { %"heapless::spsc::Queue<(app::P1_T, u8), 3_usize>" }
%"heapless::spsc::Queue<(app::P1_T, u8), 3_usize>" = type { %"core::sync::atomic::AtomicUsize", %"core::sync::atomic::AtomicUsize", [3 x { i8, i8 }], [2 x i8] }
%"core::sync::atomic::AtomicUsize" = type { i32 }
%"heapless::spsc::Queue<u8, 2_usize>" = type { %"core::sync::atomic::AtomicUsize", %"core::sync::atomic::AtomicUsize", [2 x i8], [2 x i8] }
%"rtic::RacyCell<heapless::spsc::Queue<u8, 2_usize>>" = type { %"core::cell::UnsafeCell<heapless::spsc::Queue<u8, 2_usize>>" }
%"core::cell::UnsafeCell<heapless::spsc::Queue<u8, 2_usize>>" = type { %"heapless::spsc::Queue<u8, 2_usize>" }
%"core::fmt::Arguments" = type { { [0 x { [0 x i8]*, i32 }]*, i32 }, { i32*, i32 }, { [0 x { i8*, i32* }]*, i32 } }
%"core::panic::location::Location" = type { { [0 x i8]*, i32 }, i32, i32 }
%"core::cell::UnsafeCell<[core::mem::maybe_uninit::MaybeUninit<()>; 1]>" = type { [1 x %"core::mem::maybe_uninit::MaybeUninit<()>"] }
%"core::mem::maybe_uninit::MaybeUninit<()>" = type { [0 x i8] }
%"rtic::RacyCell<[core::mem::maybe_uninit::MaybeUninit<()>; 1]>" = type { %"core::cell::UnsafeCell<[core::mem::maybe_uninit::MaybeUninit<()>; 1]>" }
%"core::option::Option<u8>::Some" = type { [1 x i8], i8 }
%"bare_metal::CriticalSection" = type { {} }
%"[closure@examples/locals.rs:10:1: 10:62].3" = type {}
%"[closure@examples/locals.rs:10:1: 10:62].4" = type {}
%"[closure@examples/locals.rs:10:1: 10:62].5" = type {}
%"cortex_m::peripheral::Peripherals" = type { %"cortex_m::peripheral::CBP", %"cortex_m::peripheral::CPUID", %"cortex_m::peripheral::DCB", %"cortex_m::peripheral::DWT", %"cortex_m::peripheral::FPB", %"cortex_m::peripheral::FPU", %"cortex_m::peripheral::ICB", %"cortex_m::peripheral::ITM", %"cortex_m::peripheral::MPU", %"cortex_m::peripheral::NVIC", %"cortex_m::peripheral::SAU", %"cortex_m::peripheral::SCB", %"cortex_m::peripheral::SYST", %"cortex_m::peripheral::TPIU", {} }
%"cortex_m::peripheral::CBP" = type { %"core::marker::PhantomData<*const ()>" }
%"core::marker::PhantomData<*const ()>" = type {}
%"cortex_m::peripheral::CPUID" = type { %"core::marker::PhantomData<*const ()>" }
%"cortex_m::peripheral::DCB" = type { %"core::marker::PhantomData<*const ()>" }
%"cortex_m::peripheral::DWT" = type { %"core::marker::PhantomData<*const ()>" }
%"cortex_m::peripheral::FPB" = type { %"core::marker::PhantomData<*const ()>" }
%"cortex_m::peripheral::FPU" = type { %"core::marker::PhantomData<*const ()>" }
%"cortex_m::peripheral::ICB" = type { %"core::marker::PhantomData<*const ()>" }
%"cortex_m::peripheral::ITM" = type { %"core::marker::PhantomData<*const ()>" }
%"cortex_m::peripheral::MPU" = type { %"core::marker::PhantomData<*const ()>" }
%"cortex_m::peripheral::NVIC" = type { %"core::marker::PhantomData<*const ()>" }
%"cortex_m::peripheral::SAU" = type { %"core::marker::PhantomData<*const ()>" }
%"cortex_m::peripheral::SCB" = type { %"core::marker::PhantomData<*const ()>" }
%"cortex_m::peripheral::SYST" = type { %"core::marker::PhantomData<*const ()>" }
%"cortex_m::peripheral::TPIU" = type { %"core::marker::PhantomData<*const ()>" }
%"[closure@examples/locals.rs:10:1: 10:62]" = type {}
%"[closure@examples/locals.rs:10:1: 10:62].0" = type {}
%"[closure@examples/locals.rs:10:1: 10:62].6" = type { %"cortex_m::peripheral::Peripherals" }
%"app::Local" = type { i64, i64, i64 }
%"app::__rtic_internal_Monotonics" = type {}
%"app::Shared" = type {}
%"app::__rtic_internal_init_Context" = type { %"cortex_m::peripheral::Peripherals", %"lm3s6965::Peripherals", %"bare_metal::CriticalSection.2" }
%"lm3s6965::Peripherals" = type { {} }
%"bare_metal::CriticalSection.2" = type { %"core::marker::PhantomData<&()>" }
%"core::marker::PhantomData<&()>" = type {}
%"cortex_m::peripheral::nvic::RegisterBlock" = type { [16 x i32], [16 x i32], [16 x i32], [16 x i32], [16 x i32], [16 x i32], [16 x i32], [16 x i32], [16 x i32], [48 x i32], [496 x i8], [580 x i32], i32 }
%"heapless::spsc::IterMut<u8, 2_usize>" = type { %"heapless::spsc::Queue<u8, 2_usize>"*, i32, i32 }
%"heapless::spsc::IterMut<(app::P1_T, u8), 3_usize>" = type { %"heapless::spsc::Queue<(app::P1_T, u8), 3_usize>"*, i32, i32 }
%"core::mem::manually_drop::ManuallyDrop<()>" = type { {} }
%"[closure@core::iter::traits::iterator::Iterator::for_each::call<u8, [closure@examples/locals.rs:10:1: 10:62]>::{closure#0}].1" = type { %"[closure@examples/locals.rs:10:1: 10:62]" }
%"[closure@core::iter::traits::iterator::Iterator::for_each::call<u8, [closure@examples/locals.rs:10:1: 10:62]>::{closure#0}]" = type { %"[closure@examples/locals.rs:10:1: 10:62].0" }
%"core::fmt::Formatter" = type { i32, i32, { i32, i32 }, { i32, i32 }, { {}*, [3 x i32]* }, i8, [3 x i8] }
%"core::fmt::Opaque" = type {}

@alloc180 = private unnamed_addr constant <{ [18 x i8] }> <{ [18 x i8] c"examples/locals.rs" }>, align 1
@alloc167 = private unnamed_addr constant <{ i8*, [12 x i8] }> <{ i8* getelementptr inbounds (<{ [18 x i8] }>, <{ [18 x i8] }>* @alloc180, i32 0, i32 0, i32 0), [12 x i8] c"\12\00\00\00\1B\00\00\00\16\00\00\00" }>, align 4
@alloc169 = private unnamed_addr constant <{ i8*, [12 x i8] }> <{ i8* getelementptr inbounds (<{ [18 x i8] }>, <{ [18 x i8] }>* @alloc180, i32 0, i32 0, i32 0), [12 x i8] c"\12\00\00\00\1C\00\00\00\16\00\00\00" }>, align 4
@alloc171 = private unnamed_addr constant <{ i8*, [12 x i8] }> <{ i8* getelementptr inbounds (<{ [18 x i8] }>, <{ [18 x i8] }>* @alloc180, i32 0, i32 0, i32 0), [12 x i8] c"\12\00\00\00.\00\00\00\09\00\00\00" }>, align 4
@str.0 = internal constant [28 x i8] c"attempt to add with overflow"
@alloc159 = private unnamed_addr constant <{ [22 x i8] }> <{ [22 x i8] c"idle: local_to_idle = " }>, align 1
@alloc112 = private unnamed_addr constant <{ [1 x i8] }> <{ [1 x i8] c"\0A" }>, align 1
@alloc160 = private unnamed_addr constant <{ i8*, [4 x i8], i8*, [4 x i8] }> <{ i8* getelementptr inbounds (<{ [22 x i8] }>, <{ [22 x i8] }>* @alloc159, i32 0, i32 0, i32 0), [4 x i8] c"\16\00\00\00", i8* getelementptr inbounds (<{ [1 x i8] }>, <{ [1 x i8] }>* @alloc112, i32 0, i32 0, i32 0), [4 x i8] c"\01\00\00\00" }>, align 4
@alloc173 = private unnamed_addr constant <{ i8*, [12 x i8] }> <{ i8* getelementptr inbounds (<{ [18 x i8] }>, <{ [18 x i8] }>* @alloc180, i32 0, i32 0, i32 0), [12 x i8] c"\12\00\00\000\00\00\00>\00\00\00" }>, align 4
@alloc175 = private unnamed_addr constant <{ i8*, [12 x i8] }> <{ i8* getelementptr inbounds (<{ [18 x i8] }>, <{ [18 x i8] }>* @alloc180, i32 0, i32 0, i32 0), [12 x i8] c"\12\00\00\00C\00\00\00\09\00\00\00" }>, align 4
@alloc120 = private unnamed_addr constant <{ [20 x i8] }> <{ [20 x i8] c"foo: local_to_foo = " }>, align 1
@alloc121 = private unnamed_addr constant <{ i8*, [4 x i8], i8*, [4 x i8] }> <{ i8* getelementptr inbounds (<{ [20 x i8] }>, <{ [20 x i8] }>* @alloc120, i32 0, i32 0, i32 0), [4 x i8] c"\14\00\00\00", i8* getelementptr inbounds (<{ [1 x i8] }>, <{ [1 x i8] }>* @alloc112, i32 0, i32 0, i32 0), [4 x i8] c"\01\00\00\00" }>, align 4
@alloc177 = private unnamed_addr constant <{ i8*, [12 x i8] }> <{ i8* getelementptr inbounds (<{ [18 x i8] }>, <{ [18 x i8] }>* @alloc180, i32 0, i32 0, i32 0), [12 x i8] c"\12\00\00\00H\00\00\00;\00\00\00" }>, align 4
@alloc179 = private unnamed_addr constant <{ i8*, [12 x i8] }> <{ i8* getelementptr inbounds (<{ [18 x i8] }>, <{ [18 x i8] }>* @alloc180, i32 0, i32 0, i32 0), [12 x i8] c"\12\00\00\00O\00\00\00\09\00\00\00" }>, align 4
@alloc110 = private unnamed_addr constant <{ [20 x i8] }> <{ [20 x i8] c"bar: local_to_bar = " }>, align 1
@alloc111 = private unnamed_addr constant <{ i8*, [4 x i8], i8*, [4 x i8] }> <{ i8* getelementptr inbounds (<{ [20 x i8] }>, <{ [20 x i8] }>* @alloc110, i32 0, i32 0, i32 0), [4 x i8] c"\14\00\00\00", i8* getelementptr inbounds (<{ [1 x i8] }>, <{ [1 x i8] }>* @alloc112, i32 0, i32 0, i32 0), [4 x i8] c"\01\00\00\00" }>, align 4
@alloc181 = private unnamed_addr constant <{ i8*, [12 x i8] }> <{ i8* getelementptr inbounds (<{ [18 x i8] }>, <{ [18 x i8] }>* @alloc180, i32 0, i32 0, i32 0), [12 x i8] c"\12\00\00\00T\00\00\00;\00\00\00" }>, align 4
@_ZN6locals3app43__rtic_internal_local_resource_local_to_foo17ha825634817032b38E = internal global <{ [8 x i8] }> undef, section ".uninit.rtic0", align 8, !dbg !0
@_ZN6locals3app43__rtic_internal_local_resource_local_to_bar17hedba4fee37a14ecaE = internal global <{ [8 x i8] }> undef, section ".uninit.rtic1", align 8, !dbg !31
@_ZN6locals3app44__rtic_internal_local_resource_local_to_idle17h9592b26e73b21be8E = internal global <{ [8 x i8] }> undef, section ".uninit.rtic2", align 8, !dbg !33
@_ZN6locals3app22__rtic_internal_foo_FQ17heb020d3b7041f04fE = internal global <{ [12 x i8] }> zeroinitializer, align 4, !dbg !35
@_ZN6locals3app26__rtic_internal_foo_INPUTS17hdf9dc66c5136e5d7E = internal global <{ [0 x i8] }> zeroinitializer, section ".uninit.rtic3", align 1, !dbg !82
@_ZN6locals3app22__rtic_internal_bar_FQ17h2ab7573cfd764f60E = internal global <{ [12 x i8] }> zeroinitializer, align 4, !dbg !104
@_ZN6locals3app26__rtic_internal_bar_INPUTS17h557822a7bd01f7e2E = internal global <{ [0 x i8] }> zeroinitializer, section ".uninit.rtic4", align 1, !dbg !106
@_ZN6locals3app21__rtic_internal_P1_RQ17h70e79590858a2cecE = internal global <{ [16 x i8] }> zeroinitializer, align 4, !dbg !108
@alloc184 = private unnamed_addr constant <{ [52 x i8] }> <{ [52 x i8] c"/home/norlen/uni/sources/cortex-m-rtic/src/export.rs" }>, align 1
@alloc183 = private unnamed_addr constant <{ i8*, [12 x i8] }> <{ i8* getelementptr inbounds (<{ [52 x i8] }>, <{ [52 x i8] }>* @alloc184, i32 0, i32 0, i32 0), [12 x i8] c"4\00\00\00<\01\00\00\06\00\00\00" }>, align 4
@str.1 = internal constant [35 x i8] c"attempt to shift left with overflow"
@alloc185 = private unnamed_addr constant <{ i8*, [12 x i8] }> <{ i8* getelementptr inbounds (<{ [52 x i8] }>, <{ [52 x i8] }>* @alloc184, i32 0, i32 0, i32 0), [12 x i8] c"4\00\00\00<\01\00\00\05\00\00\00" }>, align 4
@str.2 = internal constant [33 x i8] c"attempt to subtract with overflow"
@alloc187 = private unnamed_addr constant <{ i8*, [12 x i8] }> <{ i8* getelementptr inbounds (<{ [52 x i8] }>, <{ [52 x i8] }>* @alloc184, i32 0, i32 0, i32 0), [12 x i8] c"4\00\00\00<\01\00\00*\00\00\00" }>, align 4
@alloc196 = private unnamed_addr constant <{ [98 x i8] }> <{ [98 x i8] c"/home/norlen/.cargo/registry/src/github.com-1ecc6299db9ec823/cortex-m-0.7.4/src/peripheral/nvic.rs" }>, align 1
@alloc189 = private unnamed_addr constant <{ i8*, [12 x i8] }> <{ i8* getelementptr inbounds (<{ [98 x i8] }>, <{ [98 x i8] }>* @alloc196, i32 0, i32 0, i32 0), [12 x i8] c"b\00\00\00y\00\00\00\09\00\00\00" }>, align 4
@alloc191 = private unnamed_addr constant <{ i8*, [12 x i8] }> <{ i8* getelementptr inbounds (<{ [98 x i8] }>, <{ [98 x i8] }>* @alloc196, i32 0, i32 0, i32 0), [12 x i8] c"b\00\00\00y\00\00\009\00\00\00" }>, align 4
@alloc193 = private unnamed_addr constant <{ i8*, [12 x i8] }> <{ i8* getelementptr inbounds (<{ [98 x i8] }>, <{ [98 x i8] }>* @alloc196, i32 0, i32 0, i32 0), [12 x i8] c"b\00\00\00\C7\00\00\00\12\00\00\00" }>, align 4
@alloc195 = private unnamed_addr constant <{ i8*, [12 x i8] }> <{ i8* getelementptr inbounds (<{ [98 x i8] }>, <{ [98 x i8] }>* @alloc196, i32 0, i32 0, i32 0), [12 x i8] c"b\00\00\00\C7\00\00\00B\00\00\00" }>, align 4
@alloc197 = private unnamed_addr constant <{ i8*, [12 x i8] }> <{ i8* getelementptr inbounds (<{ [98 x i8] }>, <{ [98 x i8] }>* @alloc196, i32 0, i32 0, i32 0), [12 x i8] c"b\00\00\00\DE\00\00\00\0D\00\00\00" }>, align 4
@_ZN8cortex_m10peripheral5TAKEN17h164fa9be397b07d5E = external dso_local global i8
@alloc143 = private unnamed_addr constant <{ [1 x i8] }> zeroinitializer, align 1
@alloc202 = private unnamed_addr constant <{ [88 x i8] }> <{ [88 x i8] c"/home/norlen/.cargo/registry/src/github.com-1ecc6299db9ec823/heapless-0.7.10/src/spsc.rs" }>, align 1
@alloc199 = private unnamed_addr constant <{ i8*, [12 x i8] }> <{ i8* getelementptr inbounds (<{ [88 x i8] }>, <{ [88 x i8] }>* @alloc202, i32 0, i32 0, i32 0), [12 x i8] c"X\00\00\00q\00\00\00\09\00\00\00" }>, align 4
@alloc201 = private unnamed_addr constant <{ i8*, [12 x i8] }> <{ i8* getelementptr inbounds (<{ [88 x i8] }>, <{ [88 x i8] }>* @alloc202, i32 0, i32 0, i32 0), [12 x i8] c"X\00\00\00\7F\01\00\00\15\00\00\00" }>, align 4
@alloc203 = private unnamed_addr constant <{ i8*, [12 x i8] }> <{ i8* getelementptr inbounds (<{ [88 x i8] }>, <{ [88 x i8] }>* @alloc202, i32 0, i32 0, i32 0), [12 x i8] c"X\00\00\00\80\01\00\00\0D\00\00\00" }>, align 4
@alloc204 = private unnamed_addr constant <{ [75 x i8] }> <{ [75 x i8] c"/rustc/9d1b2106e23b1abd32fce1f17267604a5102f57a/library/core/src/ptr/mod.rs" }>, align 1
@alloc205 = private unnamed_addr constant <{ i8*, [12 x i8] }> <{ i8* getelementptr inbounds (<{ [75 x i8] }>, <{ [75 x i8] }>* @alloc204, i32 0, i32 0, i32 0), [12 x i8] c"K\00\00\00\BE\02\00\00\0D\00\00\00" }>, align 4
@alloc206 = private unnamed_addr constant <{ [43 x i8] }> <{ [43 x i8] c"called `Result::unwrap()` on an `Err` value" }>, align 1
@vtable.3 = private unnamed_addr constant <{ i8*, [8 x i8], i8*, [0 x i8] }> <{ i8* bitcast (void ({}*)* @"_ZN4core3ptr29drop_in_place$LT$$LP$$RP$$GT$17h69cf282f965e7cc1E" to i8*), [8 x i8] c"\00\00\00\00\01\00\00\00", i8* bitcast (i1 ({}*, %"core::fmt::Formatter"*)* @"_ZN45_$LT$$LP$$RP$$u20$as$u20$core..fmt..Debug$GT$3fmt17h4cbe5cdd5d7bb3fbE" to i8*), [0 x i8] zeroinitializer }>, align 4, !dbg !148
@alloc210 = private unnamed_addr constant <{ [50 x i8] }> <{ [50 x i8] c"there is no such thing as an acquire/release store" }>, align 1
@alloc220 = private unnamed_addr constant <{ [79 x i8] }> <{ [79 x i8] c"/rustc/9d1b2106e23b1abd32fce1f17267604a5102f57a/library/core/src/sync/atomic.rs" }>, align 1
@alloc212 = private unnamed_addr constant <{ i8*, [12 x i8] }> <{ i8* getelementptr inbounds (<{ [79 x i8] }>, <{ [79 x i8] }>* @alloc220, i32 0, i32 0, i32 0), [12 x i8] c"O\00\00\001\09\00\00\17\00\00\00" }>, align 4
@alloc213 = private unnamed_addr constant <{ [42 x i8] }> <{ [42 x i8] c"there is no such thing as an acquire store" }>, align 1
@alloc215 = private unnamed_addr constant <{ i8*, [12 x i8] }> <{ i8* getelementptr inbounds (<{ [79 x i8] }>, <{ [79 x i8] }>* @alloc220, i32 0, i32 0, i32 0), [12 x i8] c"O\00\00\000\09\00\00\18\00\00\00" }>, align 4
@alloc216 = private unnamed_addr constant <{ [49 x i8] }> <{ [49 x i8] c"there is no such thing as an acquire/release load" }>, align 1
@alloc218 = private unnamed_addr constant <{ i8*, [12 x i8] }> <{ i8* getelementptr inbounds (<{ [79 x i8] }>, <{ [79 x i8] }>* @alloc220, i32 0, i32 0, i32 0), [12 x i8] c"O\00\00\00?\09\00\00\17\00\00\00" }>, align 4
@alloc219 = private unnamed_addr constant <{ [40 x i8] }> <{ [40 x i8] c"there is no such thing as a release load" }>, align 1
@alloc221 = private unnamed_addr constant <{ i8*, [12 x i8] }> <{ i8* getelementptr inbounds (<{ [79 x i8] }>, <{ [79 x i8] }>* @alloc220, i32 0, i32 0, i32 0), [12 x i8] c"O\00\00\00>\09\00\00\18\00\00\00" }>, align 4
@alloc222 = private unnamed_addr constant <{ [12 x i8] }> <{ [12 x i8] c"invalid args" }>, align 1
@alloc223 = private unnamed_addr constant <{ [75 x i8] }> <{ [75 x i8] c"/rustc/9d1b2106e23b1abd32fce1f17267604a5102f57a/library/core/src/fmt/mod.rs" }>, align 1
@alloc224 = private unnamed_addr constant <{ i8*, [12 x i8] }> <{ i8* getelementptr inbounds (<{ [75 x i8] }>, <{ [75 x i8] }>* @alloc223, i32 0, i32 0, i32 0), [12 x i8] c"K\00\00\00k\01\00\00\0D\00\00\00" }>, align 4
@alloc225 = private unnamed_addr constant <{ [2 x i8] }> <{ [2 x i8] c"()" }>, align 1

; core::ptr::drop_in_place<rtic::RacyCell<heapless::spsc::Queue<(locals::app::P1_T,u8),3_usize>>>
; Function Attrs: nounwind
define internal void @"_ZN4core3ptr112drop_in_place$LT$rtic..RacyCell$LT$heapless..spsc..Queue$LT$$LP$locals..app..P1_T$C$u8$RP$$C$3_usize$GT$$GT$$GT$17hb274f222f9172188E"(%"rtic::RacyCell<heapless::spsc::Queue<(app::P1_T, u8), 3_usize>>"* %_1) unnamed_addr #0 !dbg !228 {
start:
  %_1.dbg.spill = alloca %"rtic::RacyCell<heapless::spsc::Queue<(app::P1_T, u8), 3_usize>>"*, align 4
  store %"rtic::RacyCell<heapless::spsc::Queue<(app::P1_T, u8), 3_usize>>"* %_1, %"rtic::RacyCell<heapless::spsc::Queue<(app::P1_T, u8), 3_usize>>"** %_1.dbg.spill, align 4
  call void @llvm.dbg.declare(metadata %"rtic::RacyCell<heapless::spsc::Queue<(app::P1_T, u8), 3_usize>>"** %_1.dbg.spill, metadata !235, metadata !DIExpression()), !dbg !238
  %0 = bitcast %"rtic::RacyCell<heapless::spsc::Queue<(app::P1_T, u8), 3_usize>>"* %_1 to %"core::cell::UnsafeCell<heapless::spsc::Queue<(app::P1_T, u8), 3_usize>>"*, !dbg !238
; call core::ptr::drop_in_place<core::cell::UnsafeCell<heapless::spsc::Queue<(locals::app::P1_T,u8),3_usize>>>
  call void @"_ZN4core3ptr120drop_in_place$LT$core..cell..UnsafeCell$LT$heapless..spsc..Queue$LT$$LP$locals..app..P1_T$C$u8$RP$$C$3_usize$GT$$GT$$GT$17hf78b367adf48cdd4E"(%"core::cell::UnsafeCell<heapless::spsc::Queue<(app::P1_T, u8), 3_usize>>"* %0) #10, !dbg !238
  br label %bb1, !dbg !238

bb1:                                              ; preds = %start
  ret void, !dbg !238
}

; core::ptr::drop_in_place<core::cell::UnsafeCell<heapless::spsc::Queue<(locals::app::P1_T,u8),3_usize>>>
; Function Attrs: nounwind
define internal void @"_ZN4core3ptr120drop_in_place$LT$core..cell..UnsafeCell$LT$heapless..spsc..Queue$LT$$LP$locals..app..P1_T$C$u8$RP$$C$3_usize$GT$$GT$$GT$17hf78b367adf48cdd4E"(%"core::cell::UnsafeCell<heapless::spsc::Queue<(app::P1_T, u8), 3_usize>>"* %_1) unnamed_addr #0 !dbg !239 {
start:
  %_1.dbg.spill = alloca %"core::cell::UnsafeCell<heapless::spsc::Queue<(app::P1_T, u8), 3_usize>>"*, align 4
  store %"core::cell::UnsafeCell<heapless::spsc::Queue<(app::P1_T, u8), 3_usize>>"* %_1, %"core::cell::UnsafeCell<heapless::spsc::Queue<(app::P1_T, u8), 3_usize>>"** %_1.dbg.spill, align 4
  call void @llvm.dbg.declare(metadata %"core::cell::UnsafeCell<heapless::spsc::Queue<(app::P1_T, u8), 3_usize>>"** %_1.dbg.spill, metadata !244, metadata !DIExpression()), !dbg !247
  %0 = bitcast %"core::cell::UnsafeCell<heapless::spsc::Queue<(app::P1_T, u8), 3_usize>>"* %_1 to %"heapless::spsc::Queue<(app::P1_T, u8), 3_usize>"*, !dbg !247
; call core::ptr::drop_in_place<heapless::spsc::Queue<(locals::app::P1_T,u8),3_usize>>
  call void @"_ZN4core3ptr90drop_in_place$LT$heapless..spsc..Queue$LT$$LP$locals..app..P1_T$C$u8$RP$$C$3_usize$GT$$GT$17he5437f9cfde940d1E"(%"heapless::spsc::Queue<(app::P1_T, u8), 3_usize>"* %0) #10, !dbg !247
  br label %bb1, !dbg !247

bb1:                                              ; preds = %start
  ret void, !dbg !247
}

; core::ptr::drop_in_place<()>
; Function Attrs: inlinehint nounwind
define internal void @"_ZN4core3ptr29drop_in_place$LT$$LP$$RP$$GT$17h69cf282f965e7cc1E"({}* %_1) unnamed_addr #1 !dbg !248 {
start:
  %_1.dbg.spill = alloca {}*, align 4
  store {}* %_1, {}** %_1.dbg.spill, align 4
  call void @llvm.dbg.declare(metadata {}** %_1.dbg.spill, metadata !253, metadata !DIExpression()), !dbg !254
  ret void, !dbg !254
}

; core::ptr::drop_in_place<heapless::spsc::Queue<u8,2_usize>>
; Function Attrs: nounwind
define internal void @"_ZN4core3ptr62drop_in_place$LT$heapless..spsc..Queue$LT$u8$C$2_usize$GT$$GT$17h579993977b3e9a2eE"(%"heapless::spsc::Queue<u8, 2_usize>"* %_1) unnamed_addr #0 !dbg !255 {
start:
  %_1.dbg.spill = alloca %"heapless::spsc::Queue<u8, 2_usize>"*, align 4
  store %"heapless::spsc::Queue<u8, 2_usize>"* %_1, %"heapless::spsc::Queue<u8, 2_usize>"** %_1.dbg.spill, align 4
  call void @llvm.dbg.declare(metadata %"heapless::spsc::Queue<u8, 2_usize>"** %_1.dbg.spill, metadata !260, metadata !DIExpression()), !dbg !261
; call <heapless::spsc::Queue<T,_> as core::ops::drop::Drop>::drop
  call void @"_ZN76_$LT$heapless..spsc..Queue$LT$T$C$_$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h9ad36f3161ed8116E"(%"heapless::spsc::Queue<u8, 2_usize>"* align 4 dereferenceable(12) %_1) #10, !dbg !261
  br label %bb1, !dbg !261

bb1:                                              ; preds = %start
  ret void, !dbg !261
}

; core::ptr::drop_in_place<rtic::RacyCell<heapless::spsc::Queue<u8,2_usize>>>
; Function Attrs: nounwind
define internal void @"_ZN4core3ptr84drop_in_place$LT$rtic..RacyCell$LT$heapless..spsc..Queue$LT$u8$C$2_usize$GT$$GT$$GT$17h72e41ff8bc62b96bE"(%"rtic::RacyCell<heapless::spsc::Queue<u8, 2_usize>>"* %_1) unnamed_addr #0 !dbg !262 {
start:
  %_1.dbg.spill = alloca %"rtic::RacyCell<heapless::spsc::Queue<u8, 2_usize>>"*, align 4
  store %"rtic::RacyCell<heapless::spsc::Queue<u8, 2_usize>>"* %_1, %"rtic::RacyCell<heapless::spsc::Queue<u8, 2_usize>>"** %_1.dbg.spill, align 4
  call void @llvm.dbg.declare(metadata %"rtic::RacyCell<heapless::spsc::Queue<u8, 2_usize>>"** %_1.dbg.spill, metadata !267, metadata !DIExpression()), !dbg !270
  %0 = bitcast %"rtic::RacyCell<heapless::spsc::Queue<u8, 2_usize>>"* %_1 to %"core::cell::UnsafeCell<heapless::spsc::Queue<u8, 2_usize>>"*, !dbg !270
; call core::ptr::drop_in_place<core::cell::UnsafeCell<heapless::spsc::Queue<u8,2_usize>>>
  call void @"_ZN4core3ptr92drop_in_place$LT$core..cell..UnsafeCell$LT$heapless..spsc..Queue$LT$u8$C$2_usize$GT$$GT$$GT$17h049b29bad89ea28fE"(%"core::cell::UnsafeCell<heapless::spsc::Queue<u8, 2_usize>>"* %0) #10, !dbg !270
  br label %bb1, !dbg !270

bb1:                                              ; preds = %start
  ret void, !dbg !270
}

; core::ptr::drop_in_place<heapless::spsc::Queue<(locals::app::P1_T,u8),3_usize>>
; Function Attrs: nounwind
define internal void @"_ZN4core3ptr90drop_in_place$LT$heapless..spsc..Queue$LT$$LP$locals..app..P1_T$C$u8$RP$$C$3_usize$GT$$GT$17he5437f9cfde940d1E"(%"heapless::spsc::Queue<(app::P1_T, u8), 3_usize>"* %_1) unnamed_addr #0 !dbg !271 {
start:
  %_1.dbg.spill = alloca %"heapless::spsc::Queue<(app::P1_T, u8), 3_usize>"*, align 4
  store %"heapless::spsc::Queue<(app::P1_T, u8), 3_usize>"* %_1, %"heapless::spsc::Queue<(app::P1_T, u8), 3_usize>"** %_1.dbg.spill, align 4
  call void @llvm.dbg.declare(metadata %"heapless::spsc::Queue<(app::P1_T, u8), 3_usize>"** %_1.dbg.spill, metadata !276, metadata !DIExpression()), !dbg !277
; call <heapless::spsc::Queue<T,_> as core::ops::drop::Drop>::drop
  call void @"_ZN76_$LT$heapless..spsc..Queue$LT$T$C$_$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17hde889c84868322b6E"(%"heapless::spsc::Queue<(app::P1_T, u8), 3_usize>"* align 4 dereferenceable(16) %_1) #10, !dbg !277
  br label %bb1, !dbg !277

bb1:                                              ; preds = %start
  ret void, !dbg !277
}

; core::ptr::drop_in_place<core::cell::UnsafeCell<heapless::spsc::Queue<u8,2_usize>>>
; Function Attrs: nounwind
define internal void @"_ZN4core3ptr92drop_in_place$LT$core..cell..UnsafeCell$LT$heapless..spsc..Queue$LT$u8$C$2_usize$GT$$GT$$GT$17h049b29bad89ea28fE"(%"core::cell::UnsafeCell<heapless::spsc::Queue<u8, 2_usize>>"* %_1) unnamed_addr #0 !dbg !278 {
start:
  %_1.dbg.spill = alloca %"core::cell::UnsafeCell<heapless::spsc::Queue<u8, 2_usize>>"*, align 4
  store %"core::cell::UnsafeCell<heapless::spsc::Queue<u8, 2_usize>>"* %_1, %"core::cell::UnsafeCell<heapless::spsc::Queue<u8, 2_usize>>"** %_1.dbg.spill, align 4
  call void @llvm.dbg.declare(metadata %"core::cell::UnsafeCell<heapless::spsc::Queue<u8, 2_usize>>"** %_1.dbg.spill, metadata !283, metadata !DIExpression()), !dbg !286
  %0 = bitcast %"core::cell::UnsafeCell<heapless::spsc::Queue<u8, 2_usize>>"* %_1 to %"heapless::spsc::Queue<u8, 2_usize>"*, !dbg !286
; call core::ptr::drop_in_place<heapless::spsc::Queue<u8,2_usize>>
  call void @"_ZN4core3ptr62drop_in_place$LT$heapless..spsc..Queue$LT$u8$C$2_usize$GT$$GT$17h579993977b3e9a2eE"(%"heapless::spsc::Queue<u8, 2_usize>"* %0) #10, !dbg !286
  br label %bb1, !dbg !286

bb1:                                              ; preds = %start
  ret void, !dbg !286
}

; <lm3s6965::Interrupt as bare_metal::Nr>::nr
; Function Attrs: inlinehint nounwind
define internal i8 @"_ZN54_$LT$lm3s6965..Interrupt$u20$as$u20$bare_metal..Nr$GT$2nr17h9796bf85b9167b29E"(i8* align 1 dereferenceable(1) %self) unnamed_addr #1 !dbg !287 {
start:
  %self.dbg.spill = alloca i8*, align 4
  %0 = alloca i8, align 1
  store i8* %self, i8** %self.dbg.spill, align 4
  call void @llvm.dbg.declare(metadata i8** %self.dbg.spill, metadata !294, metadata !DIExpression()), !dbg !295
  %1 = load i8, i8* %self, align 1, !dbg !296, !range !297
  %_2 = zext i8 %1 to i32, !dbg !296
  switch i32 %_2, label %bb2 [
    i32 0, label %bb3
    i32 1, label %bb4
    i32 2, label %bb5
    i32 3, label %bb6
    i32 4, label %bb7
    i32 5, label %bb8
    i32 6, label %bb9
    i32 7, label %bb10
    i32 8, label %bb11
    i32 9, label %bb12
    i32 10, label %bb13
    i32 11, label %bb14
    i32 12, label %bb15
    i32 13, label %bb16
    i32 14, label %bb17
    i32 15, label %bb18
    i32 16, label %bb19
    i32 17, label %bb20
    i32 18, label %bb21
    i32 19, label %bb22
    i32 20, label %bb23
    i32 21, label %bb24
    i32 22, label %bb25
    i32 23, label %bb26
    i32 24, label %bb27
    i32 25, label %bb28
    i32 26, label %bb29
    i32 27, label %bb30
    i32 28, label %bb31
    i32 29, label %bb32
    i32 30, label %bb33
    i32 31, label %bb34
    i32 32, label %bb35
    i32 33, label %bb36
    i32 34, label %bb37
    i32 35, label %bb38
    i32 36, label %bb39
    i32 37, label %bb1
  ], !dbg !298

bb2:                                              ; preds = %start
  unreachable, !dbg !296

bb3:                                              ; preds = %start
  store i8 0, i8* %0, align 1, !dbg !299
  br label %bb40, !dbg !299

bb4:                                              ; preds = %start
  store i8 1, i8* %0, align 1, !dbg !300
  br label %bb40, !dbg !300

bb5:                                              ; preds = %start
  store i8 2, i8* %0, align 1, !dbg !301
  br label %bb40, !dbg !301

bb6:                                              ; preds = %start
  store i8 3, i8* %0, align 1, !dbg !302
  br label %bb40, !dbg !302

bb7:                                              ; preds = %start
  store i8 4, i8* %0, align 1, !dbg !303
  br label %bb40, !dbg !303

bb8:                                              ; preds = %start
  store i8 5, i8* %0, align 1, !dbg !304
  br label %bb40, !dbg !304

bb9:                                              ; preds = %start
  store i8 6, i8* %0, align 1, !dbg !305
  br label %bb40, !dbg !305

bb10:                                             ; preds = %start
  store i8 7, i8* %0, align 1, !dbg !306
  br label %bb40, !dbg !306

bb11:                                             ; preds = %start
  store i8 8, i8* %0, align 1, !dbg !307
  br label %bb40, !dbg !307

bb12:                                             ; preds = %start
  store i8 9, i8* %0, align 1, !dbg !308
  br label %bb40, !dbg !308

bb13:                                             ; preds = %start
  store i8 10, i8* %0, align 1, !dbg !309
  br label %bb40, !dbg !309

bb14:                                             ; preds = %start
  store i8 11, i8* %0, align 1, !dbg !310
  br label %bb40, !dbg !310

bb15:                                             ; preds = %start
  store i8 12, i8* %0, align 1, !dbg !311
  br label %bb40, !dbg !311

bb16:                                             ; preds = %start
  store i8 13, i8* %0, align 1, !dbg !312
  br label %bb40, !dbg !312

bb17:                                             ; preds = %start
  store i8 14, i8* %0, align 1, !dbg !313
  br label %bb40, !dbg !313

bb18:                                             ; preds = %start
  store i8 15, i8* %0, align 1, !dbg !314
  br label %bb40, !dbg !314

bb19:                                             ; preds = %start
  store i8 16, i8* %0, align 1, !dbg !315
  br label %bb40, !dbg !315

bb20:                                             ; preds = %start
  store i8 17, i8* %0, align 1, !dbg !316
  br label %bb40, !dbg !316

bb21:                                             ; preds = %start
  store i8 18, i8* %0, align 1, !dbg !317
  br label %bb40, !dbg !317

bb22:                                             ; preds = %start
  store i8 19, i8* %0, align 1, !dbg !318
  br label %bb40, !dbg !318

bb23:                                             ; preds = %start
  store i8 20, i8* %0, align 1, !dbg !319
  br label %bb40, !dbg !319

bb24:                                             ; preds = %start
  store i8 21, i8* %0, align 1, !dbg !320
  br label %bb40, !dbg !320

bb25:                                             ; preds = %start
  store i8 22, i8* %0, align 1, !dbg !321
  br label %bb40, !dbg !321

bb26:                                             ; preds = %start
  store i8 23, i8* %0, align 1, !dbg !322
  br label %bb40, !dbg !322

bb27:                                             ; preds = %start
  store i8 24, i8* %0, align 1, !dbg !323
  br label %bb40, !dbg !323

bb28:                                             ; preds = %start
  store i8 25, i8* %0, align 1, !dbg !324
  br label %bb40, !dbg !324

bb29:                                             ; preds = %start
  store i8 26, i8* %0, align 1, !dbg !325
  br label %bb40, !dbg !325

bb30:                                             ; preds = %start
  store i8 28, i8* %0, align 1, !dbg !326
  br label %bb40, !dbg !326

bb31:                                             ; preds = %start
  store i8 29, i8* %0, align 1, !dbg !327
  br label %bb40, !dbg !327

bb32:                                             ; preds = %start
  store i8 30, i8* %0, align 1, !dbg !328
  br label %bb40, !dbg !328

bb33:                                             ; preds = %start
  store i8 31, i8* %0, align 1, !dbg !329
  br label %bb40, !dbg !329

bb34:                                             ; preds = %start
  store i8 33, i8* %0, align 1, !dbg !330
  br label %bb40, !dbg !330

bb35:                                             ; preds = %start
  store i8 35, i8* %0, align 1, !dbg !331
  br label %bb40, !dbg !331

bb36:                                             ; preds = %start
  store i8 36, i8* %0, align 1, !dbg !332
  br label %bb40, !dbg !332

bb37:                                             ; preds = %start
  store i8 37, i8* %0, align 1, !dbg !333
  br label %bb40, !dbg !333

bb38:                                             ; preds = %start
  store i8 38, i8* %0, align 1, !dbg !334
  br label %bb40, !dbg !334

bb39:                                             ; preds = %start
  store i8 42, i8* %0, align 1, !dbg !335
  br label %bb40, !dbg !335

bb1:                                              ; preds = %start
  store i8 43, i8* %0, align 1, !dbg !336
  br label %bb40, !dbg !336

bb40:                                             ; preds = %bb3, %bb4, %bb5, %bb6, %bb7, %bb8, %bb9, %bb10, %bb11, %bb12, %bb13, %bb14, %bb15, %bb16, %bb17, %bb18, %bb19, %bb20, %bb21, %bb22, %bb23, %bb24, %bb25, %bb26, %bb27, %bb28, %bb29, %bb30, %bb31, %bb32, %bb33, %bb34, %bb35, %bb36, %bb37, %bb38, %bb39, %bb1
  %2 = load i8, i8* %0, align 1, !dbg !337
  ret i8 %2, !dbg !337
}

; cortex_m::asm::nop
; Function Attrs: inlinehint nounwind
define internal void @_ZN8cortex_m3asm3nop17he6e4dcc664e4aa97E() unnamed_addr #1 !dbg !338 {
start:
  call void @__nop() #10, !dbg !343
  br label %bb1, !dbg !343

bb1:                                              ; preds = %start
  ret void, !dbg !344
}

; locals::app::idle
; Function Attrs: noreturn nounwind
define internal void @_ZN6locals3app4idle17h10dbad5f387d0984E(i64* align 8 dereferenceable(8) %cx) unnamed_addr #2 !dbg !345 {
start:
  %_args.dbg.spill = alloca i32*, align 4
  %cx.dbg.spill = alloca i64*, align 4
  %_14 = alloca i32*, align 4
  %_13 = alloca [1 x { i8*, i32* }], align 4
  %_6 = alloca %"core::fmt::Arguments", align 4
  %local_to_idle = alloca i64*, align 4
  store i64* %cx, i64** %cx.dbg.spill, align 4
  call void @llvm.dbg.declare(metadata i64** %cx.dbg.spill, metadata !356, metadata !DIExpression()), !dbg !367
  call void @llvm.dbg.declare(metadata i64** %local_to_idle, metadata !357, metadata !DIExpression()), !dbg !368
  store i64* %cx, i64** %local_to_idle, align 4, !dbg !369
  %0 = load i64*, i64** %local_to_idle, align 4, !dbg !370, !nonnull !59
  %1 = load i64, i64* %0, align 8, !dbg !370
  %2 = call { i64, i1 } @llvm.sadd.with.overflow.i64(i64 %1, i64 1), !dbg !370
  %_3.0 = extractvalue { i64, i1 } %2, 0, !dbg !370
  %_3.1 = extractvalue { i64, i1 } %2, 1, !dbg !370
  %3 = call i1 @llvm.expect.i1(i1 %_3.1, i1 false), !dbg !370
  br i1 %3, label %panic, label %bb1, !dbg !370

bb1:                                              ; preds = %start
  %4 = load i64*, i64** %local_to_idle, align 4, !dbg !370, !nonnull !59
  store i64 %_3.0, i64* %4, align 8, !dbg !370
  %5 = bitcast i32** %_14 to i64***, !dbg !371
  store i64** %local_to_idle, i64*** %5, align 4, !dbg !371
  %_args = load i32*, i32** %_14, align 4, !dbg !371, !nonnull !59
  store i32* %_args, i32** %_args.dbg.spill, align 4, !dbg !371
  call void @llvm.dbg.declare(metadata i32** %_args.dbg.spill, metadata !359, metadata !DIExpression()), !dbg !372
  %_18 = bitcast i32* %_args to i64**, !dbg !372
; call core::fmt::ArgumentV1::new
  %6 = call { i8*, i32* } @_ZN4core3fmt10ArgumentV13new17h85113322f5b5e658E(i64** align 4 dereferenceable(4) %_18, i1 (i64**, %"core::fmt::Formatter"*)* nonnull @"_ZN52_$LT$$RF$mut$u20$T$u20$as$u20$core..fmt..Display$GT$3fmt17h50bf9aa690e1dd41E") #10, !dbg !372
  %_17.0 = extractvalue { i8*, i32* } %6, 0, !dbg !372
  %_17.1 = extractvalue { i8*, i32* } %6, 1, !dbg !372
  br label %bb2, !dbg !372

panic:                                            ; preds = %start
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17h9e72682d8e5e8cf1E([0 x i8]* nonnull align 1 bitcast ([28 x i8]* @str.0 to [0 x i8]*), i32 28, %"core::panic::location::Location"* align 4 dereferenceable(16) bitcast (<{ i8*, [12 x i8] }>* @alloc171 to %"core::panic::location::Location"*)) #11, !dbg !370
  unreachable, !dbg !370

bb2:                                              ; preds = %bb1
  %7 = bitcast [1 x { i8*, i32* }]* %_13 to { i8*, i32* }*, !dbg !372
  %8 = getelementptr inbounds { i8*, i32* }, { i8*, i32* }* %7, i32 0, i32 0, !dbg !372
  store i8* %_17.0, i8** %8, align 4, !dbg !372
  %9 = getelementptr inbounds { i8*, i32* }, { i8*, i32* }* %7, i32 0, i32 1, !dbg !372
  store i32* %_17.1, i32** %9, align 4, !dbg !372
  %_10.0 = bitcast [1 x { i8*, i32* }]* %_13 to [0 x { i8*, i32* }]*, !dbg !371
; call core::fmt::Arguments::new_v1
  call void @_ZN4core3fmt9Arguments6new_v117hc132f62239bd6a9fE(%"core::fmt::Arguments"* noalias nocapture sret(%"core::fmt::Arguments") dereferenceable(24) %_6, [0 x { [0 x i8]*, i32 }]* nonnull align 4 bitcast (<{ i8*, [4 x i8], i8*, [4 x i8] }>* @alloc160 to [0 x { [0 x i8]*, i32 }]*), i32 2, [0 x { i8*, i32* }]* nonnull align 4 %_10.0, i32 1) #10, !dbg !371
  br label %bb3, !dbg !371

bb3:                                              ; preds = %bb2
; call cortex_m_semihosting::export::hstdout_fmt
  %_5 = call zeroext i1 @_ZN20cortex_m_semihosting6export11hstdout_fmt17h247612a662c736f1E(%"core::fmt::Arguments"* noalias nocapture dereferenceable(24) %_6) #10, !dbg !371
  br label %bb4, !dbg !371

bb4:                                              ; preds = %bb3
; call core::result::Result<T,E>::unwrap
  call void @"_ZN4core6result19Result$LT$T$C$E$GT$6unwrap17had281be05a095966E"(i1 zeroext %_5, %"core::panic::location::Location"* align 4 dereferenceable(16) bitcast (<{ i8*, [12 x i8] }>* @alloc173 to %"core::panic::location::Location"*)) #10, !dbg !371
  br label %bb5, !dbg !371

bb5:                                              ; preds = %bb4
; call cortex_m_semihosting::debug::exit
  call void @_ZN20cortex_m_semihosting5debug4exit17h63eb89992f0c26a5E(i1 zeroext false) #10, !dbg !373
  br label %bb6, !dbg !373

bb6:                                              ; preds = %bb6, %bb5
; call cortex_m::asm::nop
  call void @_ZN8cortex_m3asm3nop17he6e4dcc664e4aa97E() #10, !dbg !374
  br label %bb6, !dbg !374
}

; locals::app::foo
; Function Attrs: nounwind
define internal void @_ZN6locals3app3foo17ha55c0f8edfab0f06E(i64* align 8 dereferenceable(8) %cx) unnamed_addr #0 !dbg !375 {
start:
  %_args.dbg.spill = alloca i32*, align 4
  %cx.dbg.spill = alloca i64*, align 4
  %_14 = alloca i32*, align 4
  %_13 = alloca [1 x { i8*, i32* }], align 4
  %_6 = alloca %"core::fmt::Arguments", align 4
  %local_to_foo = alloca i64*, align 4
  store i64* %cx, i64** %cx.dbg.spill, align 4
  call void @llvm.dbg.declare(metadata i64** %cx.dbg.spill, metadata !385, metadata !DIExpression()), !dbg !391
  call void @llvm.dbg.declare(metadata i64** %local_to_foo, metadata !386, metadata !DIExpression()), !dbg !392
  store i64* %cx, i64** %local_to_foo, align 4, !dbg !393
  %0 = load i64*, i64** %local_to_foo, align 4, !dbg !394, !nonnull !59
  %1 = load i64, i64* %0, align 8, !dbg !394
  %2 = call { i64, i1 } @llvm.sadd.with.overflow.i64(i64 %1, i64 1), !dbg !394
  %_3.0 = extractvalue { i64, i1 } %2, 0, !dbg !394
  %_3.1 = extractvalue { i64, i1 } %2, 1, !dbg !394
  %3 = call i1 @llvm.expect.i1(i1 %_3.1, i1 false), !dbg !394
  br i1 %3, label %panic, label %bb1, !dbg !394

bb1:                                              ; preds = %start
  %4 = load i64*, i64** %local_to_foo, align 4, !dbg !394, !nonnull !59
  store i64 %_3.0, i64* %4, align 8, !dbg !394
  %5 = bitcast i32** %_14 to i64***, !dbg !395
  store i64** %local_to_foo, i64*** %5, align 4, !dbg !395
  %_args = load i32*, i32** %_14, align 4, !dbg !395, !nonnull !59
  store i32* %_args, i32** %_args.dbg.spill, align 4, !dbg !395
  call void @llvm.dbg.declare(metadata i32** %_args.dbg.spill, metadata !388, metadata !DIExpression()), !dbg !396
  %_18 = bitcast i32* %_args to i64**, !dbg !396
; call core::fmt::ArgumentV1::new
  %6 = call { i8*, i32* } @_ZN4core3fmt10ArgumentV13new17h85113322f5b5e658E(i64** align 4 dereferenceable(4) %_18, i1 (i64**, %"core::fmt::Formatter"*)* nonnull @"_ZN52_$LT$$RF$mut$u20$T$u20$as$u20$core..fmt..Display$GT$3fmt17h50bf9aa690e1dd41E") #10, !dbg !396
  %_17.0 = extractvalue { i8*, i32* } %6, 0, !dbg !396
  %_17.1 = extractvalue { i8*, i32* } %6, 1, !dbg !396
  br label %bb2, !dbg !396

panic:                                            ; preds = %start
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17h9e72682d8e5e8cf1E([0 x i8]* nonnull align 1 bitcast ([28 x i8]* @str.0 to [0 x i8]*), i32 28, %"core::panic::location::Location"* align 4 dereferenceable(16) bitcast (<{ i8*, [12 x i8] }>* @alloc175 to %"core::panic::location::Location"*)) #11, !dbg !394
  unreachable, !dbg !394

bb2:                                              ; preds = %bb1
  %7 = bitcast [1 x { i8*, i32* }]* %_13 to { i8*, i32* }*, !dbg !396
  %8 = getelementptr inbounds { i8*, i32* }, { i8*, i32* }* %7, i32 0, i32 0, !dbg !396
  store i8* %_17.0, i8** %8, align 4, !dbg !396
  %9 = getelementptr inbounds { i8*, i32* }, { i8*, i32* }* %7, i32 0, i32 1, !dbg !396
  store i32* %_17.1, i32** %9, align 4, !dbg !396
  %_10.0 = bitcast [1 x { i8*, i32* }]* %_13 to [0 x { i8*, i32* }]*, !dbg !395
; call core::fmt::Arguments::new_v1
  call void @_ZN4core3fmt9Arguments6new_v117hc132f62239bd6a9fE(%"core::fmt::Arguments"* noalias nocapture sret(%"core::fmt::Arguments") dereferenceable(24) %_6, [0 x { [0 x i8]*, i32 }]* nonnull align 4 bitcast (<{ i8*, [4 x i8], i8*, [4 x i8] }>* @alloc121 to [0 x { [0 x i8]*, i32 }]*), i32 2, [0 x { i8*, i32* }]* nonnull align 4 %_10.0, i32 1) #10, !dbg !395
  br label %bb3, !dbg !395

bb3:                                              ; preds = %bb2
; call cortex_m_semihosting::export::hstdout_fmt
  %_5 = call zeroext i1 @_ZN20cortex_m_semihosting6export11hstdout_fmt17h247612a662c736f1E(%"core::fmt::Arguments"* noalias nocapture dereferenceable(24) %_6) #10, !dbg !395
  br label %bb4, !dbg !395

bb4:                                              ; preds = %bb3
; call core::result::Result<T,E>::unwrap
  call void @"_ZN4core6result19Result$LT$T$C$E$GT$6unwrap17had281be05a095966E"(i1 zeroext %_5, %"core::panic::location::Location"* align 4 dereferenceable(16) bitcast (<{ i8*, [12 x i8] }>* @alloc177 to %"core::panic::location::Location"*)) #10, !dbg !395
  br label %bb5, !dbg !395

bb5:                                              ; preds = %bb4
  ret void, !dbg !397
}

; locals::app::bar
; Function Attrs: nounwind
define internal void @_ZN6locals3app3bar17h3dfc1d35d287dc45E(i64* align 8 dereferenceable(8) %cx) unnamed_addr #0 !dbg !398 {
start:
  %_args.dbg.spill = alloca i32*, align 4
  %cx.dbg.spill = alloca i64*, align 4
  %_14 = alloca i32*, align 4
  %_13 = alloca [1 x { i8*, i32* }], align 4
  %_6 = alloca %"core::fmt::Arguments", align 4
  %local_to_bar = alloca i64*, align 4
  store i64* %cx, i64** %cx.dbg.spill, align 4
  call void @llvm.dbg.declare(metadata i64** %cx.dbg.spill, metadata !408, metadata !DIExpression()), !dbg !414
  call void @llvm.dbg.declare(metadata i64** %local_to_bar, metadata !409, metadata !DIExpression()), !dbg !415
  store i64* %cx, i64** %local_to_bar, align 4, !dbg !416
  %0 = load i64*, i64** %local_to_bar, align 4, !dbg !417, !nonnull !59
  %1 = load i64, i64* %0, align 8, !dbg !417
  %2 = call { i64, i1 } @llvm.sadd.with.overflow.i64(i64 %1, i64 1), !dbg !417
  %_3.0 = extractvalue { i64, i1 } %2, 0, !dbg !417
  %_3.1 = extractvalue { i64, i1 } %2, 1, !dbg !417
  %3 = call i1 @llvm.expect.i1(i1 %_3.1, i1 false), !dbg !417
  br i1 %3, label %panic, label %bb1, !dbg !417

bb1:                                              ; preds = %start
  %4 = load i64*, i64** %local_to_bar, align 4, !dbg !417, !nonnull !59
  store i64 %_3.0, i64* %4, align 8, !dbg !417
  %5 = bitcast i32** %_14 to i64***, !dbg !418
  store i64** %local_to_bar, i64*** %5, align 4, !dbg !418
  %_args = load i32*, i32** %_14, align 4, !dbg !418, !nonnull !59
  store i32* %_args, i32** %_args.dbg.spill, align 4, !dbg !418
  call void @llvm.dbg.declare(metadata i32** %_args.dbg.spill, metadata !411, metadata !DIExpression()), !dbg !419
  %_18 = bitcast i32* %_args to i64**, !dbg !419
; call core::fmt::ArgumentV1::new
  %6 = call { i8*, i32* } @_ZN4core3fmt10ArgumentV13new17h85113322f5b5e658E(i64** align 4 dereferenceable(4) %_18, i1 (i64**, %"core::fmt::Formatter"*)* nonnull @"_ZN52_$LT$$RF$mut$u20$T$u20$as$u20$core..fmt..Display$GT$3fmt17h50bf9aa690e1dd41E") #10, !dbg !419
  %_17.0 = extractvalue { i8*, i32* } %6, 0, !dbg !419
  %_17.1 = extractvalue { i8*, i32* } %6, 1, !dbg !419
  br label %bb2, !dbg !419

panic:                                            ; preds = %start
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17h9e72682d8e5e8cf1E([0 x i8]* nonnull align 1 bitcast ([28 x i8]* @str.0 to [0 x i8]*), i32 28, %"core::panic::location::Location"* align 4 dereferenceable(16) bitcast (<{ i8*, [12 x i8] }>* @alloc179 to %"core::panic::location::Location"*)) #11, !dbg !417
  unreachable, !dbg !417

bb2:                                              ; preds = %bb1
  %7 = bitcast [1 x { i8*, i32* }]* %_13 to { i8*, i32* }*, !dbg !419
  %8 = getelementptr inbounds { i8*, i32* }, { i8*, i32* }* %7, i32 0, i32 0, !dbg !419
  store i8* %_17.0, i8** %8, align 4, !dbg !419
  %9 = getelementptr inbounds { i8*, i32* }, { i8*, i32* }* %7, i32 0, i32 1, !dbg !419
  store i32* %_17.1, i32** %9, align 4, !dbg !419
  %_10.0 = bitcast [1 x { i8*, i32* }]* %_13 to [0 x { i8*, i32* }]*, !dbg !418
; call core::fmt::Arguments::new_v1
  call void @_ZN4core3fmt9Arguments6new_v117hc132f62239bd6a9fE(%"core::fmt::Arguments"* noalias nocapture sret(%"core::fmt::Arguments") dereferenceable(24) %_6, [0 x { [0 x i8]*, i32 }]* nonnull align 4 bitcast (<{ i8*, [4 x i8], i8*, [4 x i8] }>* @alloc111 to [0 x { [0 x i8]*, i32 }]*), i32 2, [0 x { i8*, i32* }]* nonnull align 4 %_10.0, i32 1) #10, !dbg !418
  br label %bb3, !dbg !418

bb3:                                              ; preds = %bb2
; call cortex_m_semihosting::export::hstdout_fmt
  %_5 = call zeroext i1 @_ZN20cortex_m_semihosting6export11hstdout_fmt17h247612a662c736f1E(%"core::fmt::Arguments"* noalias nocapture dereferenceable(24) %_6) #10, !dbg !418
  br label %bb4, !dbg !418

bb4:                                              ; preds = %bb3
; call core::result::Result<T,E>::unwrap
  call void @"_ZN4core6result19Result$LT$T$C$E$GT$6unwrap17had281be05a095966E"(i1 zeroext %_5, %"core::panic::location::Location"* align 4 dereferenceable(16) bitcast (<{ i8*, [12 x i8] }>* @alloc181 to %"core::panic::location::Location"*)) #10, !dbg !418
  br label %bb5, !dbg !418

bb5:                                              ; preds = %bb4
  ret void, !dbg !420
}

; lm3s6965::Peripherals::steal
; Function Attrs: inlinehint nounwind
define internal void @_ZN8lm3s696511Peripherals5steal17h618f0739b52a798eE() unnamed_addr #1 !dbg !421 {
start:
  ret void, !dbg !425
}

; locals::app::__rtic_internal_foo_spawn
; Function Attrs: nounwind
define internal zeroext i1 @_ZN6locals3app25__rtic_internal_foo_spawn17hf95648dd5d6bc8a0E() unnamed_addr #0 !dbg !426 {
start:
  %self.dbg.spill.i3 = alloca %"core::cell::UnsafeCell<[core::mem::maybe_uninit::MaybeUninit<()>; 1]>"*, align 4
  %self.dbg.spill.i2 = alloca {}*, align 4
  %self.dbg.spill.i1 = alloca %"core::mem::maybe_uninit::MaybeUninit<()>"*, align 4
  %self.dbg.spill.i = alloca %"rtic::RacyCell<[core::mem::maybe_uninit::MaybeUninit<()>; 1]>"*, align 4
  %input.dbg.spill = alloca {}, align 1
  %_23 = alloca i8, align 1
  %_20 = alloca i8*, align 4
  %index = alloca i8, align 1
  %_2 = alloca { i8, i8 }, align 1
  %0 = alloca i8, align 1
  call void @llvm.dbg.declare(metadata {}* %input.dbg.spill, metadata !430, metadata !DIExpression()), !dbg !434
  call void @llvm.dbg.declare(metadata i8* %index, metadata !432, metadata !DIExpression()), !dbg !435
; call cortex_m::interrupt::free
  %1 = call { i8, i8 } @_ZN8cortex_m9interrupt4free17h6900db02a4e65978E() #10, !dbg !434
  store { i8, i8 } %1, { i8, i8 }* %_2, align 1, !dbg !434
  br label %bb1, !dbg !434

bb1:                                              ; preds = %start
  %2 = bitcast { i8, i8 }* %_2 to i8*, !dbg !434
  %3 = load i8, i8* %2, align 1, !dbg !434, !range !436
  %4 = trunc i8 %3 to i1, !dbg !434
  %_4 = zext i1 %4 to i32, !dbg !434
  %5 = icmp eq i32 %_4, 1, !dbg !434
  br i1 %5, label %bb2, label %bb10, !dbg !434

bb2:                                              ; preds = %bb1
  %6 = bitcast { i8, i8 }* %_2 to %"core::option::Option<u8>::Some"*, !dbg !434
  %7 = getelementptr inbounds %"core::option::Option<u8>::Some", %"core::option::Option<u8>::Some"* %6, i32 0, i32 1, !dbg !434
  %8 = load i8, i8* %7, align 1, !dbg !434
  store i8 %8, i8* %index, align 1, !dbg !434
  store %"rtic::RacyCell<[core::mem::maybe_uninit::MaybeUninit<()>; 1]>"* bitcast (<{ [0 x i8] }>* @_ZN6locals3app26__rtic_internal_foo_INPUTS17hdf9dc66c5136e5d7E to %"rtic::RacyCell<[core::mem::maybe_uninit::MaybeUninit<()>; 1]>"*), %"rtic::RacyCell<[core::mem::maybe_uninit::MaybeUninit<()>; 1]>"** %self.dbg.spill.i, align 4
  call void @llvm.dbg.declare(metadata %"rtic::RacyCell<[core::mem::maybe_uninit::MaybeUninit<()>; 1]>"** %self.dbg.spill.i, metadata !437, metadata !DIExpression()) #10, !dbg !445
  store %"core::cell::UnsafeCell<[core::mem::maybe_uninit::MaybeUninit<()>; 1]>"* bitcast (<{ [0 x i8] }>* @_ZN6locals3app26__rtic_internal_foo_INPUTS17hdf9dc66c5136e5d7E to %"core::cell::UnsafeCell<[core::mem::maybe_uninit::MaybeUninit<()>; 1]>"*), %"core::cell::UnsafeCell<[core::mem::maybe_uninit::MaybeUninit<()>; 1]>"** %self.dbg.spill.i3, align 4
  call void @llvm.dbg.declare(metadata %"core::cell::UnsafeCell<[core::mem::maybe_uninit::MaybeUninit<()>; 1]>"** %self.dbg.spill.i3, metadata !447, metadata !DIExpression()), !dbg !454
  br label %bb3, !dbg !434

bb10:                                             ; preds = %bb1
  store i8 1, i8* %0, align 1, !dbg !434
  br label %bb11, !dbg !434

bb11:                                             ; preds = %bb9, %bb10
  %9 = load i8, i8* %0, align 1, !dbg !456, !range !436
  %10 = trunc i8 %9 to i1, !dbg !456
  ret i1 %10, !dbg !456

bb3:                                              ; preds = %bb2
  %_10.0 = bitcast [1 x %"core::mem::maybe_uninit::MaybeUninit<()>"]* bitcast (<{ [0 x i8] }>* @_ZN6locals3app26__rtic_internal_foo_INPUTS17hdf9dc66c5136e5d7E to [1 x %"core::mem::maybe_uninit::MaybeUninit<()>"]*) to [0 x %"core::mem::maybe_uninit::MaybeUninit<()>"]*, !dbg !434
  %_17 = load i8, i8* %index, align 1, !dbg !434
; call core::convert::num::<impl core::convert::From<u8> for usize>::from
  %_16 = call i32 @"_ZN4core7convert3num65_$LT$impl$u20$core..convert..From$LT$u8$GT$$u20$for$u20$usize$GT$4from17h5a3358a42c13f974E"(i8 %_17) #10, !dbg !434
  br label %bb4, !dbg !434

bb4:                                              ; preds = %bb3
; call core::slice::<impl [T]>::get_unchecked_mut
  %_9 = call nonnull align 1 %"core::mem::maybe_uninit::MaybeUninit<()>"* @"_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$17get_unchecked_mut17hcb783b7fcc0d5eb1E"([0 x %"core::mem::maybe_uninit::MaybeUninit<()>"]* nonnull align 1 %_10.0, i32 1, i32 %_16) #10, !dbg !434
  br label %bb5, !dbg !434

bb5:                                              ; preds = %bb4
  store %"core::mem::maybe_uninit::MaybeUninit<()>"* %_9, %"core::mem::maybe_uninit::MaybeUninit<()>"** %self.dbg.spill.i1, align 4
  call void @llvm.dbg.declare(metadata %"core::mem::maybe_uninit::MaybeUninit<()>"** %self.dbg.spill.i1, metadata !457, metadata !DIExpression()), !dbg !464
  %11 = bitcast %"core::mem::maybe_uninit::MaybeUninit<()>"* %_9 to {}*, !dbg !466
  br label %bb6, !dbg !434

bb6:                                              ; preds = %bb5
  store {}* %11, {}** %self.dbg.spill.i2, align 4
  call void @llvm.dbg.declare(metadata {}** %self.dbg.spill.i2, metadata !467, metadata !DIExpression()) #10, !dbg !476
  call void @llvm.dbg.declare(metadata {}* undef, metadata !475, metadata !DIExpression()) #10, !dbg !478
; call core::ptr::write
  call void @_ZN4core3ptr5write17h3411c4701a77cae5E({}* %11) #10, !dbg !479
  br label %bb7, !dbg !434

bb7:                                              ; preds = %bb6
  store i8* %index, i8** %_20, align 4, !dbg !434
  %12 = load i8*, i8** %_20, align 4, !dbg !434, !nonnull !59
; call cortex_m::interrupt::free
  call void @_ZN8cortex_m9interrupt4free17he76fbd21a9f7aec8E(i8* align 1 dereferenceable(1) %12) #10, !dbg !434
  br label %bb8, !dbg !434

bb8:                                              ; preds = %bb7
  store i8 5, i8* %_23, align 1, !dbg !480
  %13 = load i8, i8* %_23, align 1, !dbg !434, !range !297
; call rtic::pend
  call void @_ZN4rtic4pend17h9dbd054a9ea733f9E(i8 %13) #10, !dbg !434
  br label %bb9, !dbg !434

bb9:                                              ; preds = %bb8
  store i8 0, i8* %0, align 1, !dbg !434
  br label %bb11, !dbg !434
}

; locals::app::__rtic_internal_foo_spawn::{{closure}}
; Function Attrs: inlinehint nounwind
define internal { i8, i8 } @"_ZN6locals3app25__rtic_internal_foo_spawn28_$u7b$$u7b$closure$u7d$$u7d$17h0fbdbcbf10f46067E"(%"bare_metal::CriticalSection"* nonnull align 1 %_2) unnamed_addr #1 !dbg !481 {
start:
  %self.dbg.spill.i1 = alloca %"core::cell::UnsafeCell<heapless::spsc::Queue<u8, 2_usize>>"*, align 4
  %self.dbg.spill.i = alloca %"rtic::RacyCell<heapless::spsc::Queue<u8, 2_usize>>"*, align 4
  %_2.dbg.spill = alloca %"bare_metal::CriticalSection"*, align 4
  %_1.dbg.spill = alloca %"[closure@examples/locals.rs:10:1: 10:62].3", align 1
  call void @llvm.dbg.declare(metadata %"[closure@examples/locals.rs:10:1: 10:62].3"* %_1.dbg.spill, metadata !504, metadata !DIExpression()), !dbg !506
  store %"bare_metal::CriticalSection"* %_2, %"bare_metal::CriticalSection"** %_2.dbg.spill, align 4
  call void @llvm.dbg.declare(metadata %"bare_metal::CriticalSection"** %_2.dbg.spill, metadata !505, metadata !DIExpression()), !dbg !506
  store %"rtic::RacyCell<heapless::spsc::Queue<u8, 2_usize>>"* bitcast (<{ [12 x i8] }>* @_ZN6locals3app22__rtic_internal_foo_FQ17heb020d3b7041f04fE to %"rtic::RacyCell<heapless::spsc::Queue<u8, 2_usize>>"*), %"rtic::RacyCell<heapless::spsc::Queue<u8, 2_usize>>"** %self.dbg.spill.i, align 4
  call void @llvm.dbg.declare(metadata %"rtic::RacyCell<heapless::spsc::Queue<u8, 2_usize>>"** %self.dbg.spill.i, metadata !507, metadata !DIExpression()) #10, !dbg !513
  store %"core::cell::UnsafeCell<heapless::spsc::Queue<u8, 2_usize>>"* bitcast (<{ [12 x i8] }>* @_ZN6locals3app22__rtic_internal_foo_FQ17heb020d3b7041f04fE to %"core::cell::UnsafeCell<heapless::spsc::Queue<u8, 2_usize>>"*), %"core::cell::UnsafeCell<heapless::spsc::Queue<u8, 2_usize>>"** %self.dbg.spill.i1, align 4
  call void @llvm.dbg.declare(metadata %"core::cell::UnsafeCell<heapless::spsc::Queue<u8, 2_usize>>"** %self.dbg.spill.i1, metadata !515, metadata !DIExpression()), !dbg !521
  br label %bb1, !dbg !506

bb1:                                              ; preds = %start
; call heapless::spsc::Queue<T,_>::dequeue
  %0 = call { i8, i8 } @"_ZN8heapless4spsc18Queue$LT$T$C$_$GT$7dequeue17hbddbe69e01b0948fE"(%"heapless::spsc::Queue<u8, 2_usize>"* align 4 dereferenceable(12) bitcast (<{ [12 x i8] }>* @_ZN6locals3app22__rtic_internal_foo_FQ17heb020d3b7041f04fE to %"heapless::spsc::Queue<u8, 2_usize>"*)) #10, !dbg !506
  %1 = extractvalue { i8, i8 } %0, 0, !dbg !506
  %2 = trunc i8 %1 to i1, !dbg !506
  %3 = extractvalue { i8, i8 } %0, 1, !dbg !506
  br label %bb2, !dbg !506

bb2:                                              ; preds = %bb1
  %4 = zext i1 %2 to i8, !dbg !523
  %5 = insertvalue { i8, i8 } undef, i8 %4, 0, !dbg !523
  %6 = insertvalue { i8, i8 } %5, i8 %3, 1, !dbg !523
  ret { i8, i8 } %6, !dbg !523
}

; locals::app::__rtic_internal_foo_spawn::{{closure}}
; Function Attrs: inlinehint nounwind
define internal void @"_ZN6locals3app25__rtic_internal_foo_spawn28_$u7b$$u7b$closure$u7d$$u7d$17hce2f397cafa79609E"(i8* align 1 dereferenceable(1) %_1, %"bare_metal::CriticalSection"* nonnull align 1 %_2) unnamed_addr #1 !dbg !524 {
start:
  %self.dbg.spill.i1 = alloca %"core::cell::UnsafeCell<heapless::spsc::Queue<(app::P1_T, u8), 3_usize>>"*, align 4
  %self.dbg.spill.i = alloca %"rtic::RacyCell<heapless::spsc::Queue<(app::P1_T, u8), 3_usize>>"*, align 4
  %_2.dbg.spill = alloca %"bare_metal::CriticalSection"*, align 4
  %_1.dbg.spill = alloca i8*, align 4
  %_9 = alloca { i8, i8 }, align 1
  store i8* %_1, i8** %_1.dbg.spill, align 4
  %0 = load i8*, i8** %_1.dbg.spill, align 4, !nonnull !59
  call void @llvm.dbg.declare(metadata i8** %_1.dbg.spill, metadata !532, metadata !DIExpression(DW_OP_deref)), !dbg !534
  store %"bare_metal::CriticalSection"* %_2, %"bare_metal::CriticalSection"** %_2.dbg.spill, align 4
  call void @llvm.dbg.declare(metadata %"bare_metal::CriticalSection"** %_2.dbg.spill, metadata !533, metadata !DIExpression()), !dbg !534
  store %"rtic::RacyCell<heapless::spsc::Queue<(app::P1_T, u8), 3_usize>>"* bitcast (<{ [16 x i8] }>* @_ZN6locals3app21__rtic_internal_P1_RQ17h70e79590858a2cecE to %"rtic::RacyCell<heapless::spsc::Queue<(app::P1_T, u8), 3_usize>>"*), %"rtic::RacyCell<heapless::spsc::Queue<(app::P1_T, u8), 3_usize>>"** %self.dbg.spill.i, align 4
  call void @llvm.dbg.declare(metadata %"rtic::RacyCell<heapless::spsc::Queue<(app::P1_T, u8), 3_usize>>"** %self.dbg.spill.i, metadata !535, metadata !DIExpression()) #10, !dbg !541
  store %"core::cell::UnsafeCell<heapless::spsc::Queue<(app::P1_T, u8), 3_usize>>"* bitcast (<{ [16 x i8] }>* @_ZN6locals3app21__rtic_internal_P1_RQ17h70e79590858a2cecE to %"core::cell::UnsafeCell<heapless::spsc::Queue<(app::P1_T, u8), 3_usize>>"*), %"core::cell::UnsafeCell<heapless::spsc::Queue<(app::P1_T, u8), 3_usize>>"** %self.dbg.spill.i1, align 4
  call void @llvm.dbg.declare(metadata %"core::cell::UnsafeCell<heapless::spsc::Queue<(app::P1_T, u8), 3_usize>>"** %self.dbg.spill.i1, metadata !543, metadata !DIExpression()), !dbg !549
  br label %bb1, !dbg !534

bb1:                                              ; preds = %start
  %_10 = load i8, i8* %_1, align 1, !dbg !534
  %1 = bitcast { i8, i8 }* %_9 to i8*, !dbg !534
  store i8 1, i8* %1, align 1, !dbg !534
  %2 = getelementptr inbounds { i8, i8 }, { i8, i8 }* %_9, i32 0, i32 1, !dbg !534
  store i8 %_10, i8* %2, align 1, !dbg !534
  %3 = getelementptr inbounds { i8, i8 }, { i8, i8 }* %_9, i32 0, i32 0, !dbg !534
  %4 = load i8, i8* %3, align 1, !dbg !534, !range !436
  %5 = trunc i8 %4 to i1, !dbg !534
  %6 = getelementptr inbounds { i8, i8 }, { i8, i8 }* %_9, i32 0, i32 1, !dbg !534
  %7 = load i8, i8* %6, align 1, !dbg !534
; call heapless::spsc::Queue<T,_>::enqueue_unchecked
  call void @"_ZN8heapless4spsc18Queue$LT$T$C$_$GT$17enqueue_unchecked17ha419a43839953ab6E"(%"heapless::spsc::Queue<(app::P1_T, u8), 3_usize>"* align 4 dereferenceable(16) bitcast (<{ [16 x i8] }>* @_ZN6locals3app21__rtic_internal_P1_RQ17h70e79590858a2cecE to %"heapless::spsc::Queue<(app::P1_T, u8), 3_usize>"*), i1 zeroext %5, i8 %7) #10, !dbg !534
  br label %bb2, !dbg !534

bb2:                                              ; preds = %bb1
  ret void, !dbg !551
}

; <T as cortex_m::interrupt::InterruptNumber>::number
; Function Attrs: inlinehint nounwind
define internal i16 @"_ZN58_$LT$T$u20$as$u20$cortex_m..interrupt..InterruptNumber$GT$6number17h7a5d7a0fd28511f6E"(i8 %0) unnamed_addr #1 !dbg !552 {
start:
  %self = alloca i8, align 1
  store i8 %0, i8* %self, align 1
  call void @llvm.dbg.declare(metadata i8* %self, metadata !560, metadata !DIExpression()), !dbg !563
; call <lm3s6965::Interrupt as bare_metal::Nr>::nr
  %_2 = call i8 @"_ZN54_$LT$lm3s6965..Interrupt$u20$as$u20$bare_metal..Nr$GT$2nr17h9796bf85b9167b29E"(i8* align 1 dereferenceable(1) %self) #10, !dbg !564
  br label %bb1, !dbg !564

bb1:                                              ; preds = %start
  %1 = zext i8 %_2 to i16, !dbg !564
  ret i16 %1, !dbg !565
}

; cortex_m::interrupt::disable
; Function Attrs: inlinehint nounwind
define internal void @_ZN8cortex_m9interrupt7disable17hebd3846882b05121E() unnamed_addr #1 !dbg !566 {
start:
  call void @__cpsid() #10, !dbg !567
  br label %bb1, !dbg !567

bb1:                                              ; preds = %start
  ret void, !dbg !568
}

; cortex_m::interrupt::enable
; Function Attrs: inlinehint nounwind
define internal void @_ZN8cortex_m9interrupt6enable17h5a8c08841dc312b2E() unnamed_addr #1 !dbg !569 {
start:
  call void @__cpsie() #10, !dbg !570
  br label %bb1, !dbg !570

bb1:                                              ; preds = %start
  ret void, !dbg !571
}

; cortex_m::interrupt::free
; Function Attrs: inlinehint nounwind
define internal { i8, i8 } @_ZN8cortex_m9interrupt4free17h6900db02a4e65978E() unnamed_addr #1 !dbg !572 {
start:
  %r.dbg.spill = alloca { i8, i8 }, align 1
  %primask.dbg.spill = alloca i8, align 1
  %f.dbg.spill = alloca %"[closure@examples/locals.rs:10:1: 10:62].3", align 1
  %_8 = alloca %"bare_metal::CriticalSection", align 1
  %_5 = alloca i8*, align 4
  call void @llvm.dbg.declare(metadata %"[closure@examples/locals.rs:10:1: 10:62].3"* %f.dbg.spill, metadata !576, metadata !DIExpression()), !dbg !584
; call cortex_m::register::primask::read
  %primask = call zeroext i1 @_ZN8cortex_m8register7primask4read17he7f40456c7aa8a00E() #10, !dbg !585
  %0 = zext i1 %primask to i8, !dbg !585
  store i8 %0, i8* %primask.dbg.spill, align 1, !dbg !585
  call void @llvm.dbg.declare(metadata i8* %primask.dbg.spill, metadata !577, metadata !DIExpression()), !dbg !586
  br label %bb1, !dbg !585

bb1:                                              ; preds = %start
; call cortex_m::interrupt::disable
  call void @_ZN8cortex_m9interrupt7disable17hebd3846882b05121E() #10, !dbg !587
  br label %bb2, !dbg !587

bb2:                                              ; preds = %bb1
; call bare_metal::CriticalSection::new
  call void @_ZN10bare_metal15CriticalSection3new17h77ebf15fa45305c2E() #10, !dbg !588
  br label %bb3, !dbg !588

bb3:                                              ; preds = %bb2
  %1 = bitcast i8** %_5 to %"bare_metal::CriticalSection"**, !dbg !589
  store %"bare_metal::CriticalSection"* %_8, %"bare_metal::CriticalSection"** %1, align 4, !dbg !589
  %2 = load i8*, i8** %_5, align 4, !dbg !589, !nonnull !59
  %3 = bitcast i8* %2 to %"bare_metal::CriticalSection"*, !dbg !589
; call locals::app::__rtic_internal_foo_spawn::{{closure}}
  %4 = call { i8, i8 } @"_ZN6locals3app25__rtic_internal_foo_spawn28_$u7b$$u7b$closure$u7d$$u7d$17h0fbdbcbf10f46067E"(%"bare_metal::CriticalSection"* nonnull align 1 %3) #10, !dbg !589
  %5 = extractvalue { i8, i8 } %4, 0, !dbg !589
  %r.0 = trunc i8 %5 to i1, !dbg !589
  %r.1 = extractvalue { i8, i8 } %4, 1, !dbg !589
  %6 = getelementptr inbounds { i8, i8 }, { i8, i8 }* %r.dbg.spill, i32 0, i32 0, !dbg !589
  %7 = zext i1 %r.0 to i8, !dbg !589
  store i8 %7, i8* %6, align 1, !dbg !589
  %8 = getelementptr inbounds { i8, i8 }, { i8, i8 }* %r.dbg.spill, i32 0, i32 1, !dbg !589
  store i8 %r.1, i8* %8, align 1, !dbg !589
  call void @llvm.dbg.declare(metadata { i8, i8 }* %r.dbg.spill, metadata !579, metadata !DIExpression()), !dbg !590
  br label %bb4, !dbg !589

bb4:                                              ; preds = %bb3
; call cortex_m::register::primask::Primask::is_active
  %_10 = call zeroext i1 @_ZN8cortex_m8register7primask7Primask9is_active17h26a0e7131c940062E(i1 zeroext %primask) #10, !dbg !591
  br label %bb5, !dbg !591

bb5:                                              ; preds = %bb4
  br i1 %_10, label %bb6, label %bb7, !dbg !591

bb7:                                              ; preds = %bb6, %bb5
  %9 = zext i1 %r.0 to i8, !dbg !592
  %10 = insertvalue { i8, i8 } undef, i8 %9, 0, !dbg !592
  %11 = insertvalue { i8, i8 } %10, i8 %r.1, 1, !dbg !592
  ret { i8, i8 } %11, !dbg !592

bb6:                                              ; preds = %bb5
; call cortex_m::interrupt::enable
  call void @_ZN8cortex_m9interrupt6enable17h5a8c08841dc312b2E() #10, !dbg !593
  br label %bb7, !dbg !593
}

; cortex_m::interrupt::free
; Function Attrs: inlinehint nounwind
define internal { i8, i8 } @_ZN8cortex_m9interrupt4free17hcb8de0e5e31ca0b1E() unnamed_addr #1 !dbg !594 {
start:
  %r.dbg.spill = alloca { i8, i8 }, align 1
  %primask.dbg.spill = alloca i8, align 1
  %f.dbg.spill = alloca %"[closure@examples/locals.rs:10:1: 10:62].4", align 1
  %_8 = alloca %"bare_metal::CriticalSection", align 1
  %_5 = alloca i8*, align 4
  call void @llvm.dbg.declare(metadata %"[closure@examples/locals.rs:10:1: 10:62].4"* %f.dbg.spill, metadata !600, metadata !DIExpression()), !dbg !607
; call cortex_m::register::primask::read
  %primask = call zeroext i1 @_ZN8cortex_m8register7primask4read17he7f40456c7aa8a00E() #10, !dbg !608
  %0 = zext i1 %primask to i8, !dbg !608
  store i8 %0, i8* %primask.dbg.spill, align 1, !dbg !608
  call void @llvm.dbg.declare(metadata i8* %primask.dbg.spill, metadata !601, metadata !DIExpression()), !dbg !609
  br label %bb1, !dbg !608

bb1:                                              ; preds = %start
; call cortex_m::interrupt::disable
  call void @_ZN8cortex_m9interrupt7disable17hebd3846882b05121E() #10, !dbg !610
  br label %bb2, !dbg !610

bb2:                                              ; preds = %bb1
; call bare_metal::CriticalSection::new
  call void @_ZN10bare_metal15CriticalSection3new17h77ebf15fa45305c2E() #10, !dbg !611
  br label %bb3, !dbg !611

bb3:                                              ; preds = %bb2
  %1 = bitcast i8** %_5 to %"bare_metal::CriticalSection"**, !dbg !612
  store %"bare_metal::CriticalSection"* %_8, %"bare_metal::CriticalSection"** %1, align 4, !dbg !612
  %2 = load i8*, i8** %_5, align 4, !dbg !612, !nonnull !59
  %3 = bitcast i8* %2 to %"bare_metal::CriticalSection"*, !dbg !612
; call locals::app::__rtic_internal_bar_spawn::{{closure}}
  %4 = call { i8, i8 } @"_ZN6locals3app25__rtic_internal_bar_spawn28_$u7b$$u7b$closure$u7d$$u7d$17hf91bfa12477a85f6E"(%"bare_metal::CriticalSection"* nonnull align 1 %3) #10, !dbg !612
  %5 = extractvalue { i8, i8 } %4, 0, !dbg !612
  %r.0 = trunc i8 %5 to i1, !dbg !612
  %r.1 = extractvalue { i8, i8 } %4, 1, !dbg !612
  %6 = getelementptr inbounds { i8, i8 }, { i8, i8 }* %r.dbg.spill, i32 0, i32 0, !dbg !612
  %7 = zext i1 %r.0 to i8, !dbg !612
  store i8 %7, i8* %6, align 1, !dbg !612
  %8 = getelementptr inbounds { i8, i8 }, { i8, i8 }* %r.dbg.spill, i32 0, i32 1, !dbg !612
  store i8 %r.1, i8* %8, align 1, !dbg !612
  call void @llvm.dbg.declare(metadata { i8, i8 }* %r.dbg.spill, metadata !603, metadata !DIExpression()), !dbg !613
  br label %bb4, !dbg !612

bb4:                                              ; preds = %bb3
; call cortex_m::register::primask::Primask::is_active
  %_10 = call zeroext i1 @_ZN8cortex_m8register7primask7Primask9is_active17h26a0e7131c940062E(i1 zeroext %primask) #10, !dbg !614
  br label %bb5, !dbg !614

bb5:                                              ; preds = %bb4
  br i1 %_10, label %bb6, label %bb7, !dbg !614

bb7:                                              ; preds = %bb6, %bb5
  %9 = zext i1 %r.0 to i8, !dbg !615
  %10 = insertvalue { i8, i8 } undef, i8 %9, 0, !dbg !615
  %11 = insertvalue { i8, i8 } %10, i8 %r.1, 1, !dbg !615
  ret { i8, i8 } %11, !dbg !615

bb6:                                              ; preds = %bb5
; call cortex_m::interrupt::enable
  call void @_ZN8cortex_m9interrupt6enable17h5a8c08841dc312b2E() #10, !dbg !616
  br label %bb7, !dbg !616
}

; cortex_m::interrupt::free
; Function Attrs: inlinehint nounwind
define internal void @_ZN8cortex_m9interrupt4free17hceee848d0f9dcae4E(i8* align 1 dereferenceable(1) %f) unnamed_addr #1 !dbg !617 {
start:
  %primask.dbg.spill = alloca i8, align 1
  %f.dbg.spill = alloca i8*, align 4
  %r.dbg.spill = alloca {}, align 1
  %_8 = alloca %"bare_metal::CriticalSection", align 1
  %_5 = alloca i8*, align 4
  call void @llvm.dbg.declare(metadata {}* %r.dbg.spill, metadata !627, metadata !DIExpression()), !dbg !632
  store i8* %f, i8** %f.dbg.spill, align 4
  call void @llvm.dbg.declare(metadata i8** %f.dbg.spill, metadata !624, metadata !DIExpression()), !dbg !633
; call cortex_m::register::primask::read
  %primask = call zeroext i1 @_ZN8cortex_m8register7primask4read17he7f40456c7aa8a00E() #10, !dbg !634
  %0 = zext i1 %primask to i8, !dbg !634
  store i8 %0, i8* %primask.dbg.spill, align 1, !dbg !634
  call void @llvm.dbg.declare(metadata i8* %primask.dbg.spill, metadata !625, metadata !DIExpression()), !dbg !635
  br label %bb1, !dbg !634

bb1:                                              ; preds = %start
; call cortex_m::interrupt::disable
  call void @_ZN8cortex_m9interrupt7disable17hebd3846882b05121E() #10, !dbg !636
  br label %bb2, !dbg !636

bb2:                                              ; preds = %bb1
; call bare_metal::CriticalSection::new
  call void @_ZN10bare_metal15CriticalSection3new17h77ebf15fa45305c2E() #10, !dbg !637
  br label %bb3, !dbg !637

bb3:                                              ; preds = %bb2
  %1 = bitcast i8** %_5 to %"bare_metal::CriticalSection"**, !dbg !638
  store %"bare_metal::CriticalSection"* %_8, %"bare_metal::CriticalSection"** %1, align 4, !dbg !638
  %2 = load i8*, i8** %_5, align 4, !dbg !638, !nonnull !59
  %3 = bitcast i8* %2 to %"bare_metal::CriticalSection"*, !dbg !638
; call locals::app::__rtic_internal_bar_spawn::{{closure}}
  call void @"_ZN6locals3app25__rtic_internal_bar_spawn28_$u7b$$u7b$closure$u7d$$u7d$17h48b2b744e8040946E"(i8* align 1 dereferenceable(1) %f, %"bare_metal::CriticalSection"* nonnull align 1 %3) #10, !dbg !638
  br label %bb4, !dbg !638

bb4:                                              ; preds = %bb3
; call cortex_m::register::primask::Primask::is_active
  %_10 = call zeroext i1 @_ZN8cortex_m8register7primask7Primask9is_active17h26a0e7131c940062E(i1 zeroext %primask) #10, !dbg !639
  br label %bb5, !dbg !639

bb5:                                              ; preds = %bb4
  br i1 %_10, label %bb6, label %bb7, !dbg !639

bb7:                                              ; preds = %bb6, %bb5
  ret void, !dbg !640

bb6:                                              ; preds = %bb5
; call cortex_m::interrupt::enable
  call void @_ZN8cortex_m9interrupt6enable17h5a8c08841dc312b2E() #10, !dbg !641
  br label %bb7, !dbg !641
}

; cortex_m::interrupt::free
; Function Attrs: inlinehint nounwind
define internal void @_ZN8cortex_m9interrupt4free17he76fbd21a9f7aec8E(i8* align 1 dereferenceable(1) %f) unnamed_addr #1 !dbg !642 {
start:
  %primask.dbg.spill = alloca i8, align 1
  %f.dbg.spill = alloca i8*, align 4
  %r.dbg.spill = alloca {}, align 1
  %_8 = alloca %"bare_metal::CriticalSection", align 1
  %_5 = alloca i8*, align 4
  call void @llvm.dbg.declare(metadata {}* %r.dbg.spill, metadata !649, metadata !DIExpression()), !dbg !653
  store i8* %f, i8** %f.dbg.spill, align 4
  call void @llvm.dbg.declare(metadata i8** %f.dbg.spill, metadata !646, metadata !DIExpression()), !dbg !654
; call cortex_m::register::primask::read
  %primask = call zeroext i1 @_ZN8cortex_m8register7primask4read17he7f40456c7aa8a00E() #10, !dbg !655
  %0 = zext i1 %primask to i8, !dbg !655
  store i8 %0, i8* %primask.dbg.spill, align 1, !dbg !655
  call void @llvm.dbg.declare(metadata i8* %primask.dbg.spill, metadata !647, metadata !DIExpression()), !dbg !656
  br label %bb1, !dbg !655

bb1:                                              ; preds = %start
; call cortex_m::interrupt::disable
  call void @_ZN8cortex_m9interrupt7disable17hebd3846882b05121E() #10, !dbg !657
  br label %bb2, !dbg !657

bb2:                                              ; preds = %bb1
; call bare_metal::CriticalSection::new
  call void @_ZN10bare_metal15CriticalSection3new17h77ebf15fa45305c2E() #10, !dbg !658
  br label %bb3, !dbg !658

bb3:                                              ; preds = %bb2
  %1 = bitcast i8** %_5 to %"bare_metal::CriticalSection"**, !dbg !659
  store %"bare_metal::CriticalSection"* %_8, %"bare_metal::CriticalSection"** %1, align 4, !dbg !659
  %2 = load i8*, i8** %_5, align 4, !dbg !659, !nonnull !59
  %3 = bitcast i8* %2 to %"bare_metal::CriticalSection"*, !dbg !659
; call locals::app::__rtic_internal_foo_spawn::{{closure}}
  call void @"_ZN6locals3app25__rtic_internal_foo_spawn28_$u7b$$u7b$closure$u7d$$u7d$17hce2f397cafa79609E"(i8* align 1 dereferenceable(1) %f, %"bare_metal::CriticalSection"* nonnull align 1 %3) #10, !dbg !659
  br label %bb4, !dbg !659

bb4:                                              ; preds = %bb3
; call cortex_m::register::primask::Primask::is_active
  %_10 = call zeroext i1 @_ZN8cortex_m8register7primask7Primask9is_active17h26a0e7131c940062E(i1 zeroext %primask) #10, !dbg !660
  br label %bb5, !dbg !660

bb5:                                              ; preds = %bb4
  br i1 %_10, label %bb6, label %bb7, !dbg !660

bb7:                                              ; preds = %bb6, %bb5
  ret void, !dbg !661

bb6:                                              ; preds = %bb5
; call cortex_m::interrupt::enable
  call void @_ZN8cortex_m9interrupt6enable17h5a8c08841dc312b2E() #10, !dbg !662
  br label %bb7, !dbg !662
}

; locals::app::__rtic_internal_bar_spawn
; Function Attrs: nounwind
define internal zeroext i1 @_ZN6locals3app25__rtic_internal_bar_spawn17hbcc8391a9543253dE() unnamed_addr #0 !dbg !663 {
start:
  %self.dbg.spill.i3 = alloca %"core::cell::UnsafeCell<[core::mem::maybe_uninit::MaybeUninit<()>; 1]>"*, align 4
  %self.dbg.spill.i2 = alloca {}*, align 4
  %self.dbg.spill.i1 = alloca %"core::mem::maybe_uninit::MaybeUninit<()>"*, align 4
  %self.dbg.spill.i = alloca %"rtic::RacyCell<[core::mem::maybe_uninit::MaybeUninit<()>; 1]>"*, align 4
  %input.dbg.spill = alloca {}, align 1
  %_23 = alloca i8, align 1
  %_20 = alloca i8*, align 4
  %index = alloca i8, align 1
  %_2 = alloca { i8, i8 }, align 1
  %0 = alloca i8, align 1
  call void @llvm.dbg.declare(metadata {}* %input.dbg.spill, metadata !665, metadata !DIExpression()), !dbg !669
  call void @llvm.dbg.declare(metadata i8* %index, metadata !667, metadata !DIExpression()), !dbg !670
; call cortex_m::interrupt::free
  %1 = call { i8, i8 } @_ZN8cortex_m9interrupt4free17hcb8de0e5e31ca0b1E() #10, !dbg !669
  store { i8, i8 } %1, { i8, i8 }* %_2, align 1, !dbg !669
  br label %bb1, !dbg !669

bb1:                                              ; preds = %start
  %2 = bitcast { i8, i8 }* %_2 to i8*, !dbg !669
  %3 = load i8, i8* %2, align 1, !dbg !669, !range !436
  %4 = trunc i8 %3 to i1, !dbg !669
  %_4 = zext i1 %4 to i32, !dbg !669
  %5 = icmp eq i32 %_4, 1, !dbg !669
  br i1 %5, label %bb2, label %bb10, !dbg !669

bb2:                                              ; preds = %bb1
  %6 = bitcast { i8, i8 }* %_2 to %"core::option::Option<u8>::Some"*, !dbg !669
  %7 = getelementptr inbounds %"core::option::Option<u8>::Some", %"core::option::Option<u8>::Some"* %6, i32 0, i32 1, !dbg !669
  %8 = load i8, i8* %7, align 1, !dbg !669
  store i8 %8, i8* %index, align 1, !dbg !669
  store %"rtic::RacyCell<[core::mem::maybe_uninit::MaybeUninit<()>; 1]>"* bitcast (<{ [0 x i8] }>* @_ZN6locals3app26__rtic_internal_bar_INPUTS17h557822a7bd01f7e2E to %"rtic::RacyCell<[core::mem::maybe_uninit::MaybeUninit<()>; 1]>"*), %"rtic::RacyCell<[core::mem::maybe_uninit::MaybeUninit<()>; 1]>"** %self.dbg.spill.i, align 4
  call void @llvm.dbg.declare(metadata %"rtic::RacyCell<[core::mem::maybe_uninit::MaybeUninit<()>; 1]>"** %self.dbg.spill.i, metadata !437, metadata !DIExpression()) #10, !dbg !671
  store %"core::cell::UnsafeCell<[core::mem::maybe_uninit::MaybeUninit<()>; 1]>"* bitcast (<{ [0 x i8] }>* @_ZN6locals3app26__rtic_internal_bar_INPUTS17h557822a7bd01f7e2E to %"core::cell::UnsafeCell<[core::mem::maybe_uninit::MaybeUninit<()>; 1]>"*), %"core::cell::UnsafeCell<[core::mem::maybe_uninit::MaybeUninit<()>; 1]>"** %self.dbg.spill.i3, align 4
  call void @llvm.dbg.declare(metadata %"core::cell::UnsafeCell<[core::mem::maybe_uninit::MaybeUninit<()>; 1]>"** %self.dbg.spill.i3, metadata !447, metadata !DIExpression()), !dbg !673
  br label %bb3, !dbg !669

bb10:                                             ; preds = %bb1
  store i8 1, i8* %0, align 1, !dbg !669
  br label %bb11, !dbg !669

bb11:                                             ; preds = %bb9, %bb10
  %9 = load i8, i8* %0, align 1, !dbg !675, !range !436
  %10 = trunc i8 %9 to i1, !dbg !675
  ret i1 %10, !dbg !675

bb3:                                              ; preds = %bb2
  %_10.0 = bitcast [1 x %"core::mem::maybe_uninit::MaybeUninit<()>"]* bitcast (<{ [0 x i8] }>* @_ZN6locals3app26__rtic_internal_bar_INPUTS17h557822a7bd01f7e2E to [1 x %"core::mem::maybe_uninit::MaybeUninit<()>"]*) to [0 x %"core::mem::maybe_uninit::MaybeUninit<()>"]*, !dbg !669
  %_17 = load i8, i8* %index, align 1, !dbg !669
; call core::convert::num::<impl core::convert::From<u8> for usize>::from
  %_16 = call i32 @"_ZN4core7convert3num65_$LT$impl$u20$core..convert..From$LT$u8$GT$$u20$for$u20$usize$GT$4from17h5a3358a42c13f974E"(i8 %_17) #10, !dbg !669
  br label %bb4, !dbg !669

bb4:                                              ; preds = %bb3
; call core::slice::<impl [T]>::get_unchecked_mut
  %_9 = call nonnull align 1 %"core::mem::maybe_uninit::MaybeUninit<()>"* @"_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$17get_unchecked_mut17hcb783b7fcc0d5eb1E"([0 x %"core::mem::maybe_uninit::MaybeUninit<()>"]* nonnull align 1 %_10.0, i32 1, i32 %_16) #10, !dbg !669
  br label %bb5, !dbg !669

bb5:                                              ; preds = %bb4
  store %"core::mem::maybe_uninit::MaybeUninit<()>"* %_9, %"core::mem::maybe_uninit::MaybeUninit<()>"** %self.dbg.spill.i1, align 4
  call void @llvm.dbg.declare(metadata %"core::mem::maybe_uninit::MaybeUninit<()>"** %self.dbg.spill.i1, metadata !457, metadata !DIExpression()), !dbg !676
  %11 = bitcast %"core::mem::maybe_uninit::MaybeUninit<()>"* %_9 to {}*, !dbg !678
  br label %bb6, !dbg !669

bb6:                                              ; preds = %bb5
  store {}* %11, {}** %self.dbg.spill.i2, align 4
  call void @llvm.dbg.declare(metadata {}** %self.dbg.spill.i2, metadata !467, metadata !DIExpression()) #10, !dbg !679
  call void @llvm.dbg.declare(metadata {}* undef, metadata !475, metadata !DIExpression()) #10, !dbg !681
; call core::ptr::write
  call void @_ZN4core3ptr5write17h3411c4701a77cae5E({}* %11) #10, !dbg !682
  br label %bb7, !dbg !669

bb7:                                              ; preds = %bb6
  store i8* %index, i8** %_20, align 4, !dbg !669
  %12 = load i8*, i8** %_20, align 4, !dbg !669, !nonnull !59
; call cortex_m::interrupt::free
  call void @_ZN8cortex_m9interrupt4free17hceee848d0f9dcae4E(i8* align 1 dereferenceable(1) %12) #10, !dbg !669
  br label %bb8, !dbg !669

bb8:                                              ; preds = %bb7
  store i8 5, i8* %_23, align 1, !dbg !683
  %13 = load i8, i8* %_23, align 1, !dbg !669, !range !297
; call rtic::pend
  call void @_ZN4rtic4pend17h9dbd054a9ea733f9E(i8 %13) #10, !dbg !669
  br label %bb9, !dbg !669

bb9:                                              ; preds = %bb8
  store i8 0, i8* %0, align 1, !dbg !669
  br label %bb11, !dbg !669
}

; locals::app::__rtic_internal_bar_spawn::{{closure}}
; Function Attrs: inlinehint nounwind
define internal { i8, i8 } @"_ZN6locals3app25__rtic_internal_bar_spawn28_$u7b$$u7b$closure$u7d$$u7d$17hf91bfa12477a85f6E"(%"bare_metal::CriticalSection"* nonnull align 1 %_2) unnamed_addr #1 !dbg !684 {
start:
  %self.dbg.spill.i1 = alloca %"core::cell::UnsafeCell<heapless::spsc::Queue<u8, 2_usize>>"*, align 4
  %self.dbg.spill.i = alloca %"rtic::RacyCell<heapless::spsc::Queue<u8, 2_usize>>"*, align 4
  %_2.dbg.spill = alloca %"bare_metal::CriticalSection"*, align 4
  %_1.dbg.spill = alloca %"[closure@examples/locals.rs:10:1: 10:62].4", align 1
  call void @llvm.dbg.declare(metadata %"[closure@examples/locals.rs:10:1: 10:62].4"* %_1.dbg.spill, metadata !688, metadata !DIExpression()), !dbg !690
  store %"bare_metal::CriticalSection"* %_2, %"bare_metal::CriticalSection"** %_2.dbg.spill, align 4
  call void @llvm.dbg.declare(metadata %"bare_metal::CriticalSection"** %_2.dbg.spill, metadata !689, metadata !DIExpression()), !dbg !690
  store %"rtic::RacyCell<heapless::spsc::Queue<u8, 2_usize>>"* bitcast (<{ [12 x i8] }>* @_ZN6locals3app22__rtic_internal_bar_FQ17h2ab7573cfd764f60E to %"rtic::RacyCell<heapless::spsc::Queue<u8, 2_usize>>"*), %"rtic::RacyCell<heapless::spsc::Queue<u8, 2_usize>>"** %self.dbg.spill.i, align 4
  call void @llvm.dbg.declare(metadata %"rtic::RacyCell<heapless::spsc::Queue<u8, 2_usize>>"** %self.dbg.spill.i, metadata !507, metadata !DIExpression()) #10, !dbg !691
  store %"core::cell::UnsafeCell<heapless::spsc::Queue<u8, 2_usize>>"* bitcast (<{ [12 x i8] }>* @_ZN6locals3app22__rtic_internal_bar_FQ17h2ab7573cfd764f60E to %"core::cell::UnsafeCell<heapless::spsc::Queue<u8, 2_usize>>"*), %"core::cell::UnsafeCell<heapless::spsc::Queue<u8, 2_usize>>"** %self.dbg.spill.i1, align 4
  call void @llvm.dbg.declare(metadata %"core::cell::UnsafeCell<heapless::spsc::Queue<u8, 2_usize>>"** %self.dbg.spill.i1, metadata !515, metadata !DIExpression()), !dbg !693
  br label %bb1, !dbg !690

bb1:                                              ; preds = %start
; call heapless::spsc::Queue<T,_>::dequeue
  %0 = call { i8, i8 } @"_ZN8heapless4spsc18Queue$LT$T$C$_$GT$7dequeue17hbddbe69e01b0948fE"(%"heapless::spsc::Queue<u8, 2_usize>"* align 4 dereferenceable(12) bitcast (<{ [12 x i8] }>* @_ZN6locals3app22__rtic_internal_bar_FQ17h2ab7573cfd764f60E to %"heapless::spsc::Queue<u8, 2_usize>"*)) #10, !dbg !690
  %1 = extractvalue { i8, i8 } %0, 0, !dbg !690
  %2 = trunc i8 %1 to i1, !dbg !690
  %3 = extractvalue { i8, i8 } %0, 1, !dbg !690
  br label %bb2, !dbg !690

bb2:                                              ; preds = %bb1
  %4 = zext i1 %2 to i8, !dbg !695
  %5 = insertvalue { i8, i8 } undef, i8 %4, 0, !dbg !695
  %6 = insertvalue { i8, i8 } %5, i8 %3, 1, !dbg !695
  ret { i8, i8 } %6, !dbg !695
}

; locals::app::__rtic_internal_bar_spawn::{{closure}}
; Function Attrs: inlinehint nounwind
define internal void @"_ZN6locals3app25__rtic_internal_bar_spawn28_$u7b$$u7b$closure$u7d$$u7d$17h48b2b744e8040946E"(i8* align 1 dereferenceable(1) %_1, %"bare_metal::CriticalSection"* nonnull align 1 %_2) unnamed_addr #1 !dbg !696 {
start:
  %self.dbg.spill.i1 = alloca %"core::cell::UnsafeCell<heapless::spsc::Queue<(app::P1_T, u8), 3_usize>>"*, align 4
  %self.dbg.spill.i = alloca %"rtic::RacyCell<heapless::spsc::Queue<(app::P1_T, u8), 3_usize>>"*, align 4
  %_2.dbg.spill = alloca %"bare_metal::CriticalSection"*, align 4
  %_1.dbg.spill = alloca i8*, align 4
  %_9 = alloca { i8, i8 }, align 1
  store i8* %_1, i8** %_1.dbg.spill, align 4
  %0 = load i8*, i8** %_1.dbg.spill, align 4, !nonnull !59
  call void @llvm.dbg.declare(metadata i8** %_1.dbg.spill, metadata !700, metadata !DIExpression(DW_OP_deref)), !dbg !702
  store %"bare_metal::CriticalSection"* %_2, %"bare_metal::CriticalSection"** %_2.dbg.spill, align 4
  call void @llvm.dbg.declare(metadata %"bare_metal::CriticalSection"** %_2.dbg.spill, metadata !701, metadata !DIExpression()), !dbg !702
  store %"rtic::RacyCell<heapless::spsc::Queue<(app::P1_T, u8), 3_usize>>"* bitcast (<{ [16 x i8] }>* @_ZN6locals3app21__rtic_internal_P1_RQ17h70e79590858a2cecE to %"rtic::RacyCell<heapless::spsc::Queue<(app::P1_T, u8), 3_usize>>"*), %"rtic::RacyCell<heapless::spsc::Queue<(app::P1_T, u8), 3_usize>>"** %self.dbg.spill.i, align 4
  call void @llvm.dbg.declare(metadata %"rtic::RacyCell<heapless::spsc::Queue<(app::P1_T, u8), 3_usize>>"** %self.dbg.spill.i, metadata !535, metadata !DIExpression()) #10, !dbg !703
  store %"core::cell::UnsafeCell<heapless::spsc::Queue<(app::P1_T, u8), 3_usize>>"* bitcast (<{ [16 x i8] }>* @_ZN6locals3app21__rtic_internal_P1_RQ17h70e79590858a2cecE to %"core::cell::UnsafeCell<heapless::spsc::Queue<(app::P1_T, u8), 3_usize>>"*), %"core::cell::UnsafeCell<heapless::spsc::Queue<(app::P1_T, u8), 3_usize>>"** %self.dbg.spill.i1, align 4
  call void @llvm.dbg.declare(metadata %"core::cell::UnsafeCell<heapless::spsc::Queue<(app::P1_T, u8), 3_usize>>"** %self.dbg.spill.i1, metadata !543, metadata !DIExpression()), !dbg !705
  br label %bb1, !dbg !702

bb1:                                              ; preds = %start
  %_10 = load i8, i8* %_1, align 1, !dbg !702
  %1 = bitcast { i8, i8 }* %_9 to i8*, !dbg !702
  store i8 0, i8* %1, align 1, !dbg !702
  %2 = getelementptr inbounds { i8, i8 }, { i8, i8 }* %_9, i32 0, i32 1, !dbg !702
  store i8 %_10, i8* %2, align 1, !dbg !702
  %3 = getelementptr inbounds { i8, i8 }, { i8, i8 }* %_9, i32 0, i32 0, !dbg !702
  %4 = load i8, i8* %3, align 1, !dbg !702, !range !436
  %5 = trunc i8 %4 to i1, !dbg !702
  %6 = getelementptr inbounds { i8, i8 }, { i8, i8 }* %_9, i32 0, i32 1, !dbg !702
  %7 = load i8, i8* %6, align 1, !dbg !702
; call heapless::spsc::Queue<T,_>::enqueue_unchecked
  call void @"_ZN8heapless4spsc18Queue$LT$T$C$_$GT$17enqueue_unchecked17ha419a43839953ab6E"(%"heapless::spsc::Queue<(app::P1_T, u8), 3_usize>"* align 4 dereferenceable(16) bitcast (<{ [16 x i8] }>* @_ZN6locals3app21__rtic_internal_P1_RQ17h70e79590858a2cecE to %"heapless::spsc::Queue<(app::P1_T, u8), 3_usize>"*), i1 zeroext %5, i8 %7) #10, !dbg !702
  br label %bb2, !dbg !702

bb2:                                              ; preds = %bb1
  ret void, !dbg !707
}

; Function Attrs: nounwind
define dso_local void @UART0() unnamed_addr #0 !dbg !708 {
start:
  %priority.dbg.spill.i = alloca i8, align 1
  store i8 1, i8* %priority.dbg.spill.i, align 1
  call void @llvm.dbg.declare(metadata i8* %priority.dbg.spill.i, metadata !709, metadata !DIExpression()) #10, !dbg !723
  call void @llvm.dbg.declare(metadata %"[closure@examples/locals.rs:10:1: 10:62].5"* undef, metadata !718, metadata !DIExpression()) #10, !dbg !725
; call locals::app::UART0::{{closure}}
  call void @"_ZN6locals3app5UART028_$u7b$$u7b$closure$u7d$$u7d$17h03af6aeda782247bE"() #10, !dbg !726
; call cortex_m::register::basepri::write
  call void @_ZN8cortex_m8register7basepri5write17h8c83f3cc5950a711E(i8 0) #10, !dbg !727
  br label %bb1, !dbg !728

bb1:                                              ; preds = %start
  ret void, !dbg !729
}

; rtic::export::logical2hw
; Function Attrs: inlinehint nounwind
define internal i8 @_ZN4rtic6export10logical2hw17h210e62a82e639146E(i8 %logical, i8 %nvic_prio_bits) unnamed_addr #1 !dbg !730 {
start:
  %nvic_prio_bits.dbg.spill = alloca i8, align 1
  %logical.dbg.spill = alloca i8, align 1
  store i8 %logical, i8* %logical.dbg.spill, align 1
  call void @llvm.dbg.declare(metadata i8* %logical.dbg.spill, metadata !734, metadata !DIExpression()), !dbg !736
  store i8 %nvic_prio_bits, i8* %nvic_prio_bits.dbg.spill, align 1
  call void @llvm.dbg.declare(metadata i8* %nvic_prio_bits.dbg.spill, metadata !735, metadata !DIExpression()), !dbg !737
  %0 = and i8 %nvic_prio_bits, -8, !dbg !738
  %_6.1 = icmp ne i8 %0, 0, !dbg !738
  %1 = and i8 %nvic_prio_bits, 7, !dbg !738
  %_6.0 = shl i8 1, %1, !dbg !738
  %2 = call i1 @llvm.expect.i1(i1 %_6.1, i1 false), !dbg !738
  br i1 %2, label %panic, label %bb1, !dbg !738

bb1:                                              ; preds = %start
  %3 = call { i8, i1 } @llvm.usub.with.overflow.i8(i8 %_6.0, i8 %logical), !dbg !739
  %_8.0 = extractvalue { i8, i1 } %3, 0, !dbg !739
  %_8.1 = extractvalue { i8, i1 } %3, 1, !dbg !739
  %4 = call i1 @llvm.expect.i1(i1 %_8.1, i1 false), !dbg !739
  br i1 %4, label %panic1, label %bb2, !dbg !739

panic:                                            ; preds = %start
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17h9e72682d8e5e8cf1E([0 x i8]* nonnull align 1 bitcast ([35 x i8]* @str.1 to [0 x i8]*), i32 35, %"core::panic::location::Location"* align 4 dereferenceable(16) bitcast (<{ i8*, [12 x i8] }>* @alloc183 to %"core::panic::location::Location"*)) #11, !dbg !738
  unreachable, !dbg !738

bb2:                                              ; preds = %bb1
  %5 = call { i8, i1 } @llvm.usub.with.overflow.i8(i8 8, i8 %nvic_prio_bits), !dbg !740
  %_11.0 = extractvalue { i8, i1 } %5, 0, !dbg !740
  %_11.1 = extractvalue { i8, i1 } %5, 1, !dbg !740
  %6 = call i1 @llvm.expect.i1(i1 %_11.1, i1 false), !dbg !740
  br i1 %6, label %panic2, label %bb3, !dbg !740

panic1:                                           ; preds = %bb1
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17h9e72682d8e5e8cf1E([0 x i8]* nonnull align 1 bitcast ([33 x i8]* @str.2 to [0 x i8]*), i32 33, %"core::panic::location::Location"* align 4 dereferenceable(16) bitcast (<{ i8*, [12 x i8] }>* @alloc185 to %"core::panic::location::Location"*)) #11, !dbg !739
  unreachable, !dbg !739

bb3:                                              ; preds = %bb2
  %7 = and i8 %_11.0, -8, !dbg !739
  %_12.1 = icmp ne i8 %7, 0, !dbg !739
  %8 = and i8 %_11.0, 7, !dbg !739
  %_12.0 = shl i8 %_8.0, %8, !dbg !739
  %9 = call i1 @llvm.expect.i1(i1 %_12.1, i1 false), !dbg !739
  br i1 %9, label %panic3, label %bb4, !dbg !739

panic2:                                           ; preds = %bb2
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17h9e72682d8e5e8cf1E([0 x i8]* nonnull align 1 bitcast ([33 x i8]* @str.2 to [0 x i8]*), i32 33, %"core::panic::location::Location"* align 4 dereferenceable(16) bitcast (<{ i8*, [12 x i8] }>* @alloc187 to %"core::panic::location::Location"*)) #11, !dbg !740
  unreachable, !dbg !740

bb4:                                              ; preds = %bb3
  ret i8 %_12.0, !dbg !741

panic3:                                           ; preds = %bb3
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17h9e72682d8e5e8cf1E([0 x i8]* nonnull align 1 bitcast ([35 x i8]* @str.1 to [0 x i8]*), i32 35, %"core::panic::location::Location"* align 4 dereferenceable(16) bitcast (<{ i8*, [12 x i8] }>* @alloc185 to %"core::panic::location::Location"*)) #11, !dbg !739
  unreachable, !dbg !739
}

; locals::app::UART0::{{closure}}
; Function Attrs: inlinehint nounwind
define internal void @"_ZN6locals3app5UART028_$u7b$$u7b$closure$u7d$$u7d$17h03af6aeda782247bE"() unnamed_addr #1 !dbg !742 {
start:
  %self.dbg.spill.i26 = alloca i64*, align 4
  %self.dbg.spill.i25 = alloca i64*, align 4
  %self.dbg.spill.i24 = alloca %"core::cell::UnsafeCell<heapless::spsc::Queue<u8, 2_usize>>"*, align 4
  %self.dbg.spill.i23 = alloca %"core::cell::UnsafeCell<heapless::spsc::Queue<u8, 2_usize>>"*, align 4
  %self.dbg.spill.i22 = alloca %"core::cell::UnsafeCell<heapless::spsc::Queue<(app::P1_T, u8), 3_usize>>"*, align 4
  %self.dbg.spill.i21 = alloca %"core::cell::UnsafeCell<[core::mem::maybe_uninit::MaybeUninit<()>; 1]>"*, align 4
  %self.dbg.spill.i20 = alloca %"core::cell::UnsafeCell<[core::mem::maybe_uninit::MaybeUninit<()>; 1]>"*, align 4
  %self.dbg.spill.i19 = alloca i64*, align 4
  %self.dbg.spill.i18 = alloca i64*, align 4
  %self.dbg.spill.i17 = alloca %"core::mem::maybe_uninit::MaybeUninit<()>"*, align 4
  %self.dbg.spill.i16 = alloca %"core::mem::maybe_uninit::MaybeUninit<()>"*, align 4
  %self.dbg.spill.i14 = alloca %"rtic::RacyCell<[core::mem::maybe_uninit::MaybeUninit<()>; 1]>"*, align 4
  %self.dbg.spill.i13 = alloca %"rtic::RacyCell<[core::mem::maybe_uninit::MaybeUninit<()>; 1]>"*, align 4
  %self.dbg.spill.i12 = alloca i64*, align 4
  %self.dbg.spill.i11 = alloca i64*, align 4
  %self.dbg.spill.i10 = alloca %"rtic::RacyCell<heapless::spsc::Queue<u8, 2_usize>>"*, align 4
  %self.dbg.spill.i9 = alloca %"rtic::RacyCell<heapless::spsc::Queue<u8, 2_usize>>"*, align 4
  %self.dbg.spill.i = alloca %"rtic::RacyCell<heapless::spsc::Queue<(app::P1_T, u8), 3_usize>>"*, align 4
  %0 = alloca i64*, align 4
  %1 = alloca i64*, align 4
  %value.dbg.spill.i5 = alloca i8, align 1
  %2 = alloca i8, align 1
  %value.dbg.spill.i = alloca i8, align 1
  %3 = alloca i8, align 1
  %priority.dbg.spill.i2 = alloca i8*, align 4
  %4 = alloca i64*, align 4
  %priority.dbg.spill.i = alloca i8*, align 4
  %5 = alloca i64*, align 4
  %priority.dbg.spill1 = alloca i8*, align 4
  %priority.dbg.spill = alloca i8*, align 4
  %index.dbg.spill = alloca i8, align 1
  %_1.dbg.spill = alloca %"[closure@examples/locals.rs:10:1: 10:62].5", align 1
  %_62 = alloca i8, align 1
  %_54 = alloca { i32*, i32* }, align 4
  %_37 = alloca i8, align 1
  %_29 = alloca { i32*, i32* }, align 4
  %task = alloca i8, align 1
  %_5 = alloca { i32*, i32* }, align 4
  %_3 = alloca { i8, i8 }, align 1
  call void @llvm.dbg.declare(metadata %"[closure@examples/locals.rs:10:1: 10:62].5"* %_1.dbg.spill, metadata !763, metadata !DIExpression()), !dbg !764
  call void @llvm.dbg.declare(metadata i8* %task, metadata !746, metadata !DIExpression()), !dbg !765
  br label %bb1, !dbg !764

bb1:                                              ; preds = %bb18, %bb28, %start
  store %"rtic::RacyCell<heapless::spsc::Queue<(app::P1_T, u8), 3_usize>>"* bitcast (<{ [16 x i8] }>* @_ZN6locals3app21__rtic_internal_P1_RQ17h70e79590858a2cecE to %"rtic::RacyCell<heapless::spsc::Queue<(app::P1_T, u8), 3_usize>>"*), %"rtic::RacyCell<heapless::spsc::Queue<(app::P1_T, u8), 3_usize>>"** %self.dbg.spill.i, align 4
  call void @llvm.dbg.declare(metadata %"rtic::RacyCell<heapless::spsc::Queue<(app::P1_T, u8), 3_usize>>"** %self.dbg.spill.i, metadata !535, metadata !DIExpression()) #10, !dbg !766
  store %"core::cell::UnsafeCell<heapless::spsc::Queue<(app::P1_T, u8), 3_usize>>"* bitcast (<{ [16 x i8] }>* @_ZN6locals3app21__rtic_internal_P1_RQ17h70e79590858a2cecE to %"core::cell::UnsafeCell<heapless::spsc::Queue<(app::P1_T, u8), 3_usize>>"*), %"core::cell::UnsafeCell<heapless::spsc::Queue<(app::P1_T, u8), 3_usize>>"** %self.dbg.spill.i22, align 4
  call void @llvm.dbg.declare(metadata %"core::cell::UnsafeCell<heapless::spsc::Queue<(app::P1_T, u8), 3_usize>>"** %self.dbg.spill.i22, metadata !543, metadata !DIExpression()), !dbg !768
  br label %bb2, !dbg !764

bb2:                                              ; preds = %bb1
; call heapless::spsc::Queue<T,_>::split
  %6 = call { i32*, i32* } @"_ZN8heapless4spsc18Queue$LT$T$C$_$GT$5split17h4fd76e368a9e3d31E"(%"heapless::spsc::Queue<(app::P1_T, u8), 3_usize>"* align 4 dereferenceable(16) bitcast (<{ [16 x i8] }>* @_ZN6locals3app21__rtic_internal_P1_RQ17h70e79590858a2cecE to %"heapless::spsc::Queue<(app::P1_T, u8), 3_usize>"*)) #10, !dbg !764
  store { i32*, i32* } %6, { i32*, i32* }* %_5, align 4, !dbg !764
  br label %bb3, !dbg !764

bb3:                                              ; preds = %bb2
  %_4 = getelementptr inbounds { i32*, i32* }, { i32*, i32* }* %_5, i32 0, i32 1, !dbg !764
; call heapless::spsc::Consumer<T,_>::dequeue
  %7 = call { i8, i8 } @"_ZN8heapless4spsc21Consumer$LT$T$C$_$GT$7dequeue17h4076781cd95e852cE"(i32** align 4 dereferenceable(4) %_4) #10, !dbg !764
  store { i8, i8 } %7, { i8, i8 }* %_3, align 1, !dbg !764
  br label %bb4, !dbg !764

bb4:                                              ; preds = %bb3
  %8 = bitcast { i8, i8 }* %_3 to i8*, !dbg !764
  %9 = load i8, i8* %8, align 1, !dbg !764, !range !770
  %10 = sub i8 %9, 2, !dbg !764
  %11 = icmp eq i8 %10, 0, !dbg !764
  %_11 = select i1 %11, i32 0, i32 1, !dbg !764
  %12 = icmp eq i32 %_11, 1, !dbg !764
  br i1 %12, label %bb5, label %bb29, !dbg !764

bb5:                                              ; preds = %bb4
  %13 = bitcast { i8, i8 }* %_3 to i8*, !dbg !764
  %14 = load i8, i8* %13, align 1, !dbg !764, !range !436
  %15 = trunc i8 %14 to i1, !dbg !764
  %16 = zext i1 %15 to i8, !dbg !764
  store i8 %16, i8* %task, align 1, !dbg !764
  %17 = getelementptr inbounds { i8, i8 }, { i8, i8 }* %_3, i32 0, i32 1, !dbg !764
  %index = load i8, i8* %17, align 1, !dbg !764
  store i8 %index, i8* %index.dbg.spill, align 1, !dbg !764
  call void @llvm.dbg.declare(metadata i8* %index.dbg.spill, metadata !748, metadata !DIExpression()), !dbg !765
  %18 = load i8, i8* %task, align 1, !dbg !764, !range !436
  %19 = trunc i8 %18 to i1, !dbg !764
  %_14 = zext i1 %19 to i32, !dbg !764
  switch i32 %_14, label %bb7 [
    i32 0, label %bb8
    i32 1, label %bb6
  ], !dbg !764

bb29:                                             ; preds = %bb4
  ret void, !dbg !771

bb7:                                              ; preds = %bb5
  unreachable, !dbg !764

bb8:                                              ; preds = %bb5
  store %"rtic::RacyCell<[core::mem::maybe_uninit::MaybeUninit<()>; 1]>"* bitcast (<{ [0 x i8] }>* @_ZN6locals3app26__rtic_internal_bar_INPUTS17h557822a7bd01f7e2E to %"rtic::RacyCell<[core::mem::maybe_uninit::MaybeUninit<()>; 1]>"*), %"rtic::RacyCell<[core::mem::maybe_uninit::MaybeUninit<()>; 1]>"** %self.dbg.spill.i13, align 4
  call void @llvm.dbg.declare(metadata %"rtic::RacyCell<[core::mem::maybe_uninit::MaybeUninit<()>; 1]>"** %self.dbg.spill.i13, metadata !772, metadata !DIExpression()) #10, !dbg !778
  store %"core::cell::UnsafeCell<[core::mem::maybe_uninit::MaybeUninit<()>; 1]>"* bitcast (<{ [0 x i8] }>* @_ZN6locals3app26__rtic_internal_bar_INPUTS17h557822a7bd01f7e2E to %"core::cell::UnsafeCell<[core::mem::maybe_uninit::MaybeUninit<()>; 1]>"*), %"core::cell::UnsafeCell<[core::mem::maybe_uninit::MaybeUninit<()>; 1]>"** %self.dbg.spill.i21, align 4
  call void @llvm.dbg.declare(metadata %"core::cell::UnsafeCell<[core::mem::maybe_uninit::MaybeUninit<()>; 1]>"** %self.dbg.spill.i21, metadata !447, metadata !DIExpression()), !dbg !780
  br label %bb9, !dbg !764

bb6:                                              ; preds = %bb5
  store %"rtic::RacyCell<[core::mem::maybe_uninit::MaybeUninit<()>; 1]>"* bitcast (<{ [0 x i8] }>* @_ZN6locals3app26__rtic_internal_foo_INPUTS17hdf9dc66c5136e5d7E to %"rtic::RacyCell<[core::mem::maybe_uninit::MaybeUninit<()>; 1]>"*), %"rtic::RacyCell<[core::mem::maybe_uninit::MaybeUninit<()>; 1]>"** %self.dbg.spill.i14, align 4
  call void @llvm.dbg.declare(metadata %"rtic::RacyCell<[core::mem::maybe_uninit::MaybeUninit<()>; 1]>"** %self.dbg.spill.i14, metadata !772, metadata !DIExpression()) #10, !dbg !782
  store %"core::cell::UnsafeCell<[core::mem::maybe_uninit::MaybeUninit<()>; 1]>"* bitcast (<{ [0 x i8] }>* @_ZN6locals3app26__rtic_internal_foo_INPUTS17hdf9dc66c5136e5d7E to %"core::cell::UnsafeCell<[core::mem::maybe_uninit::MaybeUninit<()>; 1]>"*), %"core::cell::UnsafeCell<[core::mem::maybe_uninit::MaybeUninit<()>; 1]>"** %self.dbg.spill.i20, align 4
  call void @llvm.dbg.declare(metadata %"core::cell::UnsafeCell<[core::mem::maybe_uninit::MaybeUninit<()>; 1]>"** %self.dbg.spill.i20, metadata !447, metadata !DIExpression()), !dbg !784
  br label %bb19, !dbg !764

bb19:                                             ; preds = %bb6
  %_44.0 = bitcast [1 x %"core::mem::maybe_uninit::MaybeUninit<()>"]* bitcast (<{ [0 x i8] }>* @_ZN6locals3app26__rtic_internal_foo_INPUTS17hdf9dc66c5136e5d7E to [1 x %"core::mem::maybe_uninit::MaybeUninit<()>"]*) to [0 x %"core::mem::maybe_uninit::MaybeUninit<()>"]*, !dbg !764
; call core::convert::num::<impl core::convert::From<u8> for usize>::from
  %_50 = call i32 @"_ZN4core7convert3num65_$LT$impl$u20$core..convert..From$LT$u8$GT$$u20$for$u20$usize$GT$4from17h5a3358a42c13f974E"(i8 %index) #10, !dbg !764
  br label %bb20, !dbg !764

bb20:                                             ; preds = %bb19
; call core::slice::<impl [T]>::get_unchecked
  %_43 = call nonnull align 1 %"core::mem::maybe_uninit::MaybeUninit<()>"* @"_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$13get_unchecked17hd2da507ba33e3510E"([0 x %"core::mem::maybe_uninit::MaybeUninit<()>"]* nonnull align 1 %_44.0, i32 1, i32 %_50) #10, !dbg !764
  br label %bb21, !dbg !764

bb21:                                             ; preds = %bb20
  store %"core::mem::maybe_uninit::MaybeUninit<()>"* %_43, %"core::mem::maybe_uninit::MaybeUninit<()>"** %self.dbg.spill.i17, align 4
  call void @llvm.dbg.declare(metadata %"core::mem::maybe_uninit::MaybeUninit<()>"** %self.dbg.spill.i17, metadata !786, metadata !DIExpression()), !dbg !792
  %20 = bitcast %"core::mem::maybe_uninit::MaybeUninit<()>"* %_43 to {}*, !dbg !794
  br label %bb22, !dbg !764

bb22:                                             ; preds = %bb21
; call core::ptr::const_ptr::<impl *const T>::read
  call void @"_ZN4core3ptr9const_ptr33_$LT$impl$u20$$BP$const$u20$T$GT$4read17h60f26a767622c7c8E"({}* %20) #10, !dbg !764
  br label %bb23, !dbg !764

bb23:                                             ; preds = %bb22
  store %"rtic::RacyCell<heapless::spsc::Queue<u8, 2_usize>>"* bitcast (<{ [12 x i8] }>* @_ZN6locals3app22__rtic_internal_foo_FQ17heb020d3b7041f04fE to %"rtic::RacyCell<heapless::spsc::Queue<u8, 2_usize>>"*), %"rtic::RacyCell<heapless::spsc::Queue<u8, 2_usize>>"** %self.dbg.spill.i10, align 4
  call void @llvm.dbg.declare(metadata %"rtic::RacyCell<heapless::spsc::Queue<u8, 2_usize>>"** %self.dbg.spill.i10, metadata !507, metadata !DIExpression()) #10, !dbg !795
  store %"core::cell::UnsafeCell<heapless::spsc::Queue<u8, 2_usize>>"* bitcast (<{ [12 x i8] }>* @_ZN6locals3app22__rtic_internal_foo_FQ17heb020d3b7041f04fE to %"core::cell::UnsafeCell<heapless::spsc::Queue<u8, 2_usize>>"*), %"core::cell::UnsafeCell<heapless::spsc::Queue<u8, 2_usize>>"** %self.dbg.spill.i23, align 4
  call void @llvm.dbg.declare(metadata %"core::cell::UnsafeCell<heapless::spsc::Queue<u8, 2_usize>>"** %self.dbg.spill.i23, metadata !515, metadata !DIExpression()), !dbg !797
  br label %bb24, !dbg !764

bb24:                                             ; preds = %bb23
; call heapless::spsc::Queue<T,_>::split
  %21 = call { i32*, i32* } @"_ZN8heapless4spsc18Queue$LT$T$C$_$GT$5split17h5bcea21f35589d84E"(%"heapless::spsc::Queue<u8, 2_usize>"* align 4 dereferenceable(12) bitcast (<{ [12 x i8] }>* @_ZN6locals3app22__rtic_internal_foo_FQ17heb020d3b7041f04fE to %"heapless::spsc::Queue<u8, 2_usize>"*)) #10, !dbg !764
  store { i32*, i32* } %21, { i32*, i32* }* %_54, align 4, !dbg !764
  br label %bb25, !dbg !764

bb25:                                             ; preds = %bb24
  %_53 = bitcast { i32*, i32* }* %_54 to i32**, !dbg !764
; call heapless::spsc::Producer<T,_>::enqueue_unchecked
  call void @"_ZN8heapless4spsc21Producer$LT$T$C$_$GT$17enqueue_unchecked17h2ab9bd46dff93908E"(i32** align 4 dereferenceable(4) %_53, i8 %index) #10, !dbg !764
  br label %bb26, !dbg !764

bb26:                                             ; preds = %bb25
  store i8 1, i8* %value.dbg.spill.i5, align 1
  call void @llvm.dbg.declare(metadata i8* %value.dbg.spill.i5, metadata !799, metadata !DIExpression()) #10, !dbg !804
; call core::cell::Cell<T>::new
  %_2.i6 = call i8 @"_ZN4core4cell13Cell$LT$T$GT$3new17h0ec9730ff8e64e09E"(i8 1) #10, !dbg !806
  store i8 %_2.i6, i8* %2, align 1, !dbg !807
  %22 = load i8, i8* %2, align 1, !dbg !808
  store i8 %22, i8* %_62, align 1, !dbg !764
  br label %bb27, !dbg !764

bb27:                                             ; preds = %bb26
  store i8* %_62, i8** %priority.dbg.spill, align 4, !dbg !764
  call void @llvm.dbg.declare(metadata i8** %priority.dbg.spill, metadata !761, metadata !DIExpression()), !dbg !809
  store i8* %_62, i8** %priority.dbg.spill.i, align 4
  call void @llvm.dbg.declare(metadata i8** %priority.dbg.spill.i, metadata !810, metadata !DIExpression()) #10, !dbg !815
  store i64* bitcast (<{ [8 x i8] }>* @_ZN6locals3app43__rtic_internal_local_resource_local_to_foo17ha825634817032b38E to i64*), i64** %self.dbg.spill.i12, align 4
  call void @llvm.dbg.declare(metadata i64** %self.dbg.spill.i12, metadata !817, metadata !DIExpression()) #10, !dbg !824
  store i64* bitcast (<{ [8 x i8] }>* @_ZN6locals3app43__rtic_internal_local_resource_local_to_foo17ha825634817032b38E to i64*), i64** %self.dbg.spill.i25, align 4
  call void @llvm.dbg.declare(metadata i64** %self.dbg.spill.i25, metadata !830, metadata !DIExpression()), !dbg !836
  store i64* bitcast (<{ [8 x i8] }>* @_ZN6locals3app43__rtic_internal_local_resource_local_to_foo17ha825634817032b38E to i64*), i64** %self.dbg.spill.i19, align 4
  call void @llvm.dbg.declare(metadata i64** %self.dbg.spill.i19, metadata !838, metadata !DIExpression()), !dbg !845
  store i64* bitcast (<{ [8 x i8] }>* @_ZN6locals3app43__rtic_internal_local_resource_local_to_foo17ha825634817032b38E to i64*), i64** %1, align 4, !dbg !847
  %23 = load i64*, i64** %1, align 4, !dbg !848, !nonnull !59
  store i64* %23, i64** %5, align 4, !dbg !815
  %24 = load i64*, i64** %5, align 4, !dbg !849, !nonnull !59
  br label %bb28, !dbg !809

bb28:                                             ; preds = %bb27
; call locals::app::foo
  call void @_ZN6locals3app3foo17ha55c0f8edfab0f06E(i64* align 8 dereferenceable(8) %24) #10, !dbg !809
  br label %bb1, !dbg !809

bb9:                                              ; preds = %bb8
  %_19.0 = bitcast [1 x %"core::mem::maybe_uninit::MaybeUninit<()>"]* bitcast (<{ [0 x i8] }>* @_ZN6locals3app26__rtic_internal_bar_INPUTS17h557822a7bd01f7e2E to [1 x %"core::mem::maybe_uninit::MaybeUninit<()>"]*) to [0 x %"core::mem::maybe_uninit::MaybeUninit<()>"]*, !dbg !764
; call core::convert::num::<impl core::convert::From<u8> for usize>::from
  %_25 = call i32 @"_ZN4core7convert3num65_$LT$impl$u20$core..convert..From$LT$u8$GT$$u20$for$u20$usize$GT$4from17h5a3358a42c13f974E"(i8 %index) #10, !dbg !764
  br label %bb10, !dbg !764

bb10:                                             ; preds = %bb9
; call core::slice::<impl [T]>::get_unchecked
  %_18 = call nonnull align 1 %"core::mem::maybe_uninit::MaybeUninit<()>"* @"_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$13get_unchecked17hd2da507ba33e3510E"([0 x %"core::mem::maybe_uninit::MaybeUninit<()>"]* nonnull align 1 %_19.0, i32 1, i32 %_25) #10, !dbg !764
  br label %bb11, !dbg !764

bb11:                                             ; preds = %bb10
  store %"core::mem::maybe_uninit::MaybeUninit<()>"* %_18, %"core::mem::maybe_uninit::MaybeUninit<()>"** %self.dbg.spill.i16, align 4
  call void @llvm.dbg.declare(metadata %"core::mem::maybe_uninit::MaybeUninit<()>"** %self.dbg.spill.i16, metadata !786, metadata !DIExpression()), !dbg !850
  %25 = bitcast %"core::mem::maybe_uninit::MaybeUninit<()>"* %_18 to {}*, !dbg !852
  br label %bb12, !dbg !764

bb12:                                             ; preds = %bb11
; call core::ptr::const_ptr::<impl *const T>::read
  call void @"_ZN4core3ptr9const_ptr33_$LT$impl$u20$$BP$const$u20$T$GT$4read17h60f26a767622c7c8E"({}* %25) #10, !dbg !764
  br label %bb13, !dbg !764

bb13:                                             ; preds = %bb12
  store %"rtic::RacyCell<heapless::spsc::Queue<u8, 2_usize>>"* bitcast (<{ [12 x i8] }>* @_ZN6locals3app22__rtic_internal_bar_FQ17h2ab7573cfd764f60E to %"rtic::RacyCell<heapless::spsc::Queue<u8, 2_usize>>"*), %"rtic::RacyCell<heapless::spsc::Queue<u8, 2_usize>>"** %self.dbg.spill.i9, align 4
  call void @llvm.dbg.declare(metadata %"rtic::RacyCell<heapless::spsc::Queue<u8, 2_usize>>"** %self.dbg.spill.i9, metadata !507, metadata !DIExpression()) #10, !dbg !853
  store %"core::cell::UnsafeCell<heapless::spsc::Queue<u8, 2_usize>>"* bitcast (<{ [12 x i8] }>* @_ZN6locals3app22__rtic_internal_bar_FQ17h2ab7573cfd764f60E to %"core::cell::UnsafeCell<heapless::spsc::Queue<u8, 2_usize>>"*), %"core::cell::UnsafeCell<heapless::spsc::Queue<u8, 2_usize>>"** %self.dbg.spill.i24, align 4
  call void @llvm.dbg.declare(metadata %"core::cell::UnsafeCell<heapless::spsc::Queue<u8, 2_usize>>"** %self.dbg.spill.i24, metadata !515, metadata !DIExpression()), !dbg !855
  br label %bb14, !dbg !764

bb14:                                             ; preds = %bb13
; call heapless::spsc::Queue<T,_>::split
  %26 = call { i32*, i32* } @"_ZN8heapless4spsc18Queue$LT$T$C$_$GT$5split17h5bcea21f35589d84E"(%"heapless::spsc::Queue<u8, 2_usize>"* align 4 dereferenceable(12) bitcast (<{ [12 x i8] }>* @_ZN6locals3app22__rtic_internal_bar_FQ17h2ab7573cfd764f60E to %"heapless::spsc::Queue<u8, 2_usize>"*)) #10, !dbg !764
  store { i32*, i32* } %26, { i32*, i32* }* %_29, align 4, !dbg !764
  br label %bb15, !dbg !764

bb15:                                             ; preds = %bb14
  %_28 = bitcast { i32*, i32* }* %_29 to i32**, !dbg !764
; call heapless::spsc::Producer<T,_>::enqueue_unchecked
  call void @"_ZN8heapless4spsc21Producer$LT$T$C$_$GT$17enqueue_unchecked17h2ab9bd46dff93908E"(i32** align 4 dereferenceable(4) %_28, i8 %index) #10, !dbg !764
  br label %bb16, !dbg !764

bb16:                                             ; preds = %bb15
  store i8 1, i8* %value.dbg.spill.i, align 1
  call void @llvm.dbg.declare(metadata i8* %value.dbg.spill.i, metadata !799, metadata !DIExpression()) #10, !dbg !857
; call core::cell::Cell<T>::new
  %_2.i4 = call i8 @"_ZN4core4cell13Cell$LT$T$GT$3new17h0ec9730ff8e64e09E"(i8 1) #10, !dbg !859
  store i8 %_2.i4, i8* %3, align 1, !dbg !860
  %27 = load i8, i8* %3, align 1, !dbg !861
  store i8 %27, i8* %_37, align 1, !dbg !764
  br label %bb17, !dbg !764

bb17:                                             ; preds = %bb16
  store i8* %_37, i8** %priority.dbg.spill1, align 4, !dbg !764
  call void @llvm.dbg.declare(metadata i8** %priority.dbg.spill1, metadata !749, metadata !DIExpression()), !dbg !862
  store i8* %_37, i8** %priority.dbg.spill.i2, align 4
  call void @llvm.dbg.declare(metadata i8** %priority.dbg.spill.i2, metadata !863, metadata !DIExpression()) #10, !dbg !868
  store i64* bitcast (<{ [8 x i8] }>* @_ZN6locals3app43__rtic_internal_local_resource_local_to_bar17hedba4fee37a14ecaE to i64*), i64** %self.dbg.spill.i11, align 4
  call void @llvm.dbg.declare(metadata i64** %self.dbg.spill.i11, metadata !817, metadata !DIExpression()) #10, !dbg !870
  store i64* bitcast (<{ [8 x i8] }>* @_ZN6locals3app43__rtic_internal_local_resource_local_to_bar17hedba4fee37a14ecaE to i64*), i64** %self.dbg.spill.i26, align 4
  call void @llvm.dbg.declare(metadata i64** %self.dbg.spill.i26, metadata !830, metadata !DIExpression()), !dbg !876
  store i64* bitcast (<{ [8 x i8] }>* @_ZN6locals3app43__rtic_internal_local_resource_local_to_bar17hedba4fee37a14ecaE to i64*), i64** %self.dbg.spill.i18, align 4
  call void @llvm.dbg.declare(metadata i64** %self.dbg.spill.i18, metadata !838, metadata !DIExpression()), !dbg !878
  store i64* bitcast (<{ [8 x i8] }>* @_ZN6locals3app43__rtic_internal_local_resource_local_to_bar17hedba4fee37a14ecaE to i64*), i64** %0, align 4, !dbg !880
  %28 = load i64*, i64** %0, align 4, !dbg !881, !nonnull !59
  store i64* %28, i64** %4, align 4, !dbg !868
  %29 = load i64*, i64** %4, align 4, !dbg !882, !nonnull !59
  br label %bb18, !dbg !862

bb18:                                             ; preds = %bb17
; call locals::app::bar
  call void @_ZN6locals3app3bar17h3dfc1d35d287dc45E(i64* align 8 dereferenceable(8) %29) #10, !dbg !862
  br label %bb1, !dbg !862
}

; Function Attrs: noreturn nounwind
define dso_local void @main() unnamed_addr #2 !dbg !883 {
start:
  %self.dbg.spill.i2 = alloca i64*, align 4
  %self.dbg.spill.i1 = alloca i64*, align 4
  %self.dbg.spill.i = alloca i64*, align 4
  %value.dbg.spill.i = alloca i8, align 1
  %0 = alloca i8, align 1
  %1 = alloca i64*, align 4
  %priority.dbg.spill.i = alloca i8*, align 4
  %2 = alloca i64*, align 4
  %_22 = alloca i8, align 1
  %_16 = alloca i8, align 1
  %_13 = alloca i8, align 1
  %core = alloca %"cortex_m::peripheral::Peripherals", align 1
  %_7 = alloca { i8, i8 }, align 1
  %_4 = alloca { i8, i8 }, align 1
  call void @llvm.dbg.declare(metadata %"cortex_m::peripheral::Peripherals"* %core, metadata !886, metadata !DIExpression()), !dbg !952
  br label %bb1, !dbg !953

bb1:                                              ; preds = %start
; call cortex_m::interrupt::disable
  call void @_ZN8cortex_m9interrupt7disable17hebd3846882b05121E() #10, !dbg !953
  br label %bb2, !dbg !953

bb2:                                              ; preds = %bb1
  %3 = bitcast { i8, i8 }* %_4 to i8*, !dbg !953
  store i8 0, i8* %3, align 1, !dbg !953
  %4 = getelementptr inbounds { i8, i8 }, { i8, i8 }* %_4, i32 0, i32 1, !dbg !953
  store i8 1, i8* %4, align 1, !dbg !953
  %5 = getelementptr inbounds { i8, i8 }, { i8, i8 }* %_4, i32 0, i32 0, !dbg !953
  %6 = load i8, i8* %5, align 1, !dbg !953
  %7 = getelementptr inbounds { i8, i8 }, { i8, i8 }* %_4, i32 0, i32 1, !dbg !953
  %8 = load i8, i8* %7, align 1, !dbg !953
; call core::iter::traits::iterator::Iterator::for_each
  call void @_ZN4core4iter6traits8iterator8Iterator8for_each17h248455fa942f1094E(i8 %6, i8 %8) #10, !dbg !953
  br label %bb3, !dbg !953

bb3:                                              ; preds = %bb2
  %9 = bitcast { i8, i8 }* %_7 to i8*, !dbg !953
  store i8 0, i8* %9, align 1, !dbg !953
  %10 = getelementptr inbounds { i8, i8 }, { i8, i8 }* %_7, i32 0, i32 1, !dbg !953
  store i8 1, i8* %10, align 1, !dbg !953
  %11 = getelementptr inbounds { i8, i8 }, { i8, i8 }* %_7, i32 0, i32 0, !dbg !953
  %12 = load i8, i8* %11, align 1, !dbg !953
  %13 = getelementptr inbounds { i8, i8 }, { i8, i8 }* %_7, i32 0, i32 1, !dbg !953
  %14 = load i8, i8* %13, align 1, !dbg !953
; call core::iter::traits::iterator::Iterator::for_each
  call void @_ZN4core4iter6traits8iterator8Iterator8for_each17h003edabc4355739dE(i8 %12, i8 %14) #10, !dbg !953
  br label %bb4, !dbg !953

bb4:                                              ; preds = %bb3
; call cortex_m::peripheral::Peripherals::steal
  call void @_ZN8cortex_m10peripheral11Peripherals5steal17hf4660d70e6ed8445E() #10, !dbg !953
  br label %bb5, !dbg !953

bb5:                                              ; preds = %bb4
; call <T as core::convert::Into<U>>::into
  call void @"_ZN50_$LT$T$u20$as$u20$core..convert..Into$LT$U$GT$$GT$4into17hb837751d062ab74fE"() #10, !dbg !953
  br label %bb6, !dbg !953

bb6:                                              ; preds = %bb5
  %_12 = bitcast %"cortex_m::peripheral::Peripherals"* %core to %"cortex_m::peripheral::NVIC"*, !dbg !952
  store i8 5, i8* %_13, align 1, !dbg !952
; call rtic::export::logical2hw
  %_14 = call i8 @_ZN4rtic6export10logical2hw17h210e62a82e639146E(i8 1, i8 3) #10, !dbg !952
  br label %bb7, !dbg !952

bb7:                                              ; preds = %bb6
  %15 = load i8, i8* %_13, align 1, !dbg !952, !range !297
; call cortex_m::peripheral::nvic::<impl cortex_m::peripheral::NVIC>::set_priority
  call void @"_ZN8cortex_m10peripheral4nvic44_$LT$impl$u20$cortex_m..peripheral..NVIC$GT$12set_priority17h755c18aead94bf5aE"(%"cortex_m::peripheral::NVIC"* nonnull align 1 %_12, i8 %15, i8 %_14) #10, !dbg !952
  br label %bb8, !dbg !952

bb8:                                              ; preds = %bb7
  store i8 5, i8* %_16, align 1, !dbg !952
  %16 = load i8, i8* %_16, align 1, !dbg !952, !range !297
; call cortex_m::peripheral::nvic::<impl cortex_m::peripheral::NVIC>::unmask
  call void @"_ZN8cortex_m10peripheral4nvic44_$LT$impl$u20$cortex_m..peripheral..NVIC$GT$6unmask17h825ea550b82b57f2E"(i8 %16) #10, !dbg !952
  br label %bb9, !dbg !952

bb9:                                              ; preds = %bb8
; call locals::app::rtic_ext::main::__rtic_init_resources
  call void @_ZN6locals3app8rtic_ext4main21__rtic_init_resources17h2f1eea4e8bb2e3a9E() #10, !dbg !952
  br label %bb10, !dbg !952

bb10:                                             ; preds = %bb9
  store i8 0, i8* %value.dbg.spill.i, align 1
  call void @llvm.dbg.declare(metadata i8* %value.dbg.spill.i, metadata !799, metadata !DIExpression()) #10, !dbg !954
; call core::cell::Cell<T>::new
  %_2.i = call i8 @"_ZN4core4cell13Cell$LT$T$GT$3new17h0ec9730ff8e64e09E"(i8 0) #10, !dbg !956
  store i8 %_2.i, i8* %0, align 1, !dbg !957
  %17 = load i8, i8* %0, align 1, !dbg !958
  store i8 %17, i8* %_22, align 1, !dbg !952
  br label %bb11, !dbg !952

bb11:                                             ; preds = %bb10
  store i8* %_22, i8** %priority.dbg.spill.i, align 4
  call void @llvm.dbg.declare(metadata i8** %priority.dbg.spill.i, metadata !959, metadata !DIExpression()) #10, !dbg !964
  store i64* bitcast (<{ [8 x i8] }>* @_ZN6locals3app44__rtic_internal_local_resource_local_to_idle17h9592b26e73b21be8E to i64*), i64** %self.dbg.spill.i, align 4
  call void @llvm.dbg.declare(metadata i64** %self.dbg.spill.i, metadata !817, metadata !DIExpression()) #10, !dbg !966
  store i64* bitcast (<{ [8 x i8] }>* @_ZN6locals3app44__rtic_internal_local_resource_local_to_idle17h9592b26e73b21be8E to i64*), i64** %self.dbg.spill.i2, align 4
  call void @llvm.dbg.declare(metadata i64** %self.dbg.spill.i2, metadata !830, metadata !DIExpression()), !dbg !972
  store i64* bitcast (<{ [8 x i8] }>* @_ZN6locals3app44__rtic_internal_local_resource_local_to_idle17h9592b26e73b21be8E to i64*), i64** %self.dbg.spill.i1, align 4
  call void @llvm.dbg.declare(metadata i64** %self.dbg.spill.i1, metadata !838, metadata !DIExpression()), !dbg !974
  store i64* bitcast (<{ [8 x i8] }>* @_ZN6locals3app44__rtic_internal_local_resource_local_to_idle17h9592b26e73b21be8E to i64*), i64** %1, align 4, !dbg !976
  %18 = load i64*, i64** %1, align 4, !dbg !977, !nonnull !59
  store i64* %18, i64** %2, align 4, !dbg !964
  %19 = load i64*, i64** %2, align 4, !dbg !978, !nonnull !59
  br label %bb12, !dbg !952

bb12:                                             ; preds = %bb11
; call locals::app::idle
  call void @_ZN6locals3app4idle17h10dbad5f387d0984E(i64* align 8 dereferenceable(8) %19) #11, !dbg !952
  unreachable, !dbg !952
}

; locals::app::rtic_ext::main::{{closure}}
; Function Attrs: inlinehint nounwind
define internal void @"_ZN6locals3app8rtic_ext4main28_$u7b$$u7b$closure$u7d$$u7d$17h1bdb1d17f7ba3d47E"(%"[closure@examples/locals.rs:10:1: 10:62]"* nonnull align 1 %_1, i8 %i) unnamed_addr #1 !dbg !979 {
start:
  %self.dbg.spill.i1 = alloca %"core::cell::UnsafeCell<heapless::spsc::Queue<u8, 2_usize>>"*, align 4
  %self.dbg.spill.i = alloca %"rtic::RacyCell<heapless::spsc::Queue<u8, 2_usize>>"*, align 4
  %i.dbg.spill = alloca i8, align 1
  %_1.dbg.spill = alloca %"[closure@examples/locals.rs:10:1: 10:62]"*, align 4
  store %"[closure@examples/locals.rs:10:1: 10:62]"* %_1, %"[closure@examples/locals.rs:10:1: 10:62]"** %_1.dbg.spill, align 4
  call void @llvm.dbg.declare(metadata %"[closure@examples/locals.rs:10:1: 10:62]"** %_1.dbg.spill, metadata !987, metadata !DIExpression()), !dbg !988
  store i8 %i, i8* %i.dbg.spill, align 1
  call void @llvm.dbg.declare(metadata i8* %i.dbg.spill, metadata !986, metadata !DIExpression()), !dbg !988
  store %"rtic::RacyCell<heapless::spsc::Queue<u8, 2_usize>>"* bitcast (<{ [12 x i8] }>* @_ZN6locals3app22__rtic_internal_foo_FQ17heb020d3b7041f04fE to %"rtic::RacyCell<heapless::spsc::Queue<u8, 2_usize>>"*), %"rtic::RacyCell<heapless::spsc::Queue<u8, 2_usize>>"** %self.dbg.spill.i, align 4
  call void @llvm.dbg.declare(metadata %"rtic::RacyCell<heapless::spsc::Queue<u8, 2_usize>>"** %self.dbg.spill.i, metadata !507, metadata !DIExpression()) #10, !dbg !989
  store %"core::cell::UnsafeCell<heapless::spsc::Queue<u8, 2_usize>>"* bitcast (<{ [12 x i8] }>* @_ZN6locals3app22__rtic_internal_foo_FQ17heb020d3b7041f04fE to %"core::cell::UnsafeCell<heapless::spsc::Queue<u8, 2_usize>>"*), %"core::cell::UnsafeCell<heapless::spsc::Queue<u8, 2_usize>>"** %self.dbg.spill.i1, align 4
  call void @llvm.dbg.declare(metadata %"core::cell::UnsafeCell<heapless::spsc::Queue<u8, 2_usize>>"** %self.dbg.spill.i1, metadata !515, metadata !DIExpression()), !dbg !991
  br label %bb1, !dbg !988

bb1:                                              ; preds = %start
; call heapless::spsc::Queue<T,_>::enqueue_unchecked
  call void @"_ZN8heapless4spsc18Queue$LT$T$C$_$GT$17enqueue_unchecked17h37791079c683ec91E"(%"heapless::spsc::Queue<u8, 2_usize>"* align 4 dereferenceable(12) bitcast (<{ [12 x i8] }>* @_ZN6locals3app22__rtic_internal_foo_FQ17heb020d3b7041f04fE to %"heapless::spsc::Queue<u8, 2_usize>"*), i8 %i) #10, !dbg !988
  br label %bb2, !dbg !988

bb2:                                              ; preds = %bb1
  ret void, !dbg !993
}

; locals::app::rtic_ext::main::{{closure}}
; Function Attrs: inlinehint nounwind
define internal void @"_ZN6locals3app8rtic_ext4main28_$u7b$$u7b$closure$u7d$$u7d$17h55422711175e8f91E"(%"[closure@examples/locals.rs:10:1: 10:62].0"* nonnull align 1 %_1, i8 %i) unnamed_addr #1 !dbg !994 {
start:
  %self.dbg.spill.i1 = alloca %"core::cell::UnsafeCell<heapless::spsc::Queue<u8, 2_usize>>"*, align 4
  %self.dbg.spill.i = alloca %"rtic::RacyCell<heapless::spsc::Queue<u8, 2_usize>>"*, align 4
  %i.dbg.spill = alloca i8, align 1
  %_1.dbg.spill = alloca %"[closure@examples/locals.rs:10:1: 10:62].0"*, align 4
  store %"[closure@examples/locals.rs:10:1: 10:62].0"* %_1, %"[closure@examples/locals.rs:10:1: 10:62].0"** %_1.dbg.spill, align 4
  call void @llvm.dbg.declare(metadata %"[closure@examples/locals.rs:10:1: 10:62].0"** %_1.dbg.spill, metadata !1001, metadata !DIExpression()), !dbg !1002
  store i8 %i, i8* %i.dbg.spill, align 1
  call void @llvm.dbg.declare(metadata i8* %i.dbg.spill, metadata !1000, metadata !DIExpression()), !dbg !1002
  store %"rtic::RacyCell<heapless::spsc::Queue<u8, 2_usize>>"* bitcast (<{ [12 x i8] }>* @_ZN6locals3app22__rtic_internal_bar_FQ17h2ab7573cfd764f60E to %"rtic::RacyCell<heapless::spsc::Queue<u8, 2_usize>>"*), %"rtic::RacyCell<heapless::spsc::Queue<u8, 2_usize>>"** %self.dbg.spill.i, align 4
  call void @llvm.dbg.declare(metadata %"rtic::RacyCell<heapless::spsc::Queue<u8, 2_usize>>"** %self.dbg.spill.i, metadata !507, metadata !DIExpression()) #10, !dbg !1003
  store %"core::cell::UnsafeCell<heapless::spsc::Queue<u8, 2_usize>>"* bitcast (<{ [12 x i8] }>* @_ZN6locals3app22__rtic_internal_bar_FQ17h2ab7573cfd764f60E to %"core::cell::UnsafeCell<heapless::spsc::Queue<u8, 2_usize>>"*), %"core::cell::UnsafeCell<heapless::spsc::Queue<u8, 2_usize>>"** %self.dbg.spill.i1, align 4
  call void @llvm.dbg.declare(metadata %"core::cell::UnsafeCell<heapless::spsc::Queue<u8, 2_usize>>"** %self.dbg.spill.i1, metadata !515, metadata !DIExpression()), !dbg !1005
  br label %bb1, !dbg !1002

bb1:                                              ; preds = %start
; call heapless::spsc::Queue<T,_>::enqueue_unchecked
  call void @"_ZN8heapless4spsc18Queue$LT$T$C$_$GT$17enqueue_unchecked17h37791079c683ec91E"(%"heapless::spsc::Queue<u8, 2_usize>"* align 4 dereferenceable(12) bitcast (<{ [12 x i8] }>* @_ZN6locals3app22__rtic_internal_bar_FQ17h2ab7573cfd764f60E to %"heapless::spsc::Queue<u8, 2_usize>"*), i8 %i) #10, !dbg !1002
  br label %bb2, !dbg !1002

bb2:                                              ; preds = %bb1
  ret void, !dbg !1007
}

; locals::app::rtic_ext::main::__rtic_init_resources
; Function Attrs: noinline nounwind
define internal void @_ZN6locals3app8rtic_ext4main21__rtic_init_resources17h2f1eea4e8bb2e3a9E() unnamed_addr #3 !dbg !1008 {
start:
  %f.dbg.spill = alloca %"[closure@examples/locals.rs:10:1: 10:62].6", align 1
  call void @llvm.dbg.declare(metadata %"[closure@examples/locals.rs:10:1: 10:62].6"* %f.dbg.spill, metadata !1015, metadata !DIExpression()), !dbg !1018
; call locals::app::rtic_ext::main::{{closure}}
  call void @"_ZN6locals3app8rtic_ext4main28_$u7b$$u7b$closure$u7d$$u7d$17hfd06d08bbe916045E"() #10, !dbg !1018
  br label %bb1, !dbg !1018

bb1:                                              ; preds = %start
  ret void, !dbg !1019
}

; locals::app::rtic_ext::main::{{closure}}
; Function Attrs: inlinehint nounwind
define internal void @"_ZN6locals3app8rtic_ext4main28_$u7b$$u7b$closure$u7d$$u7d$17hfd06d08bbe916045E"() unnamed_addr #1 !dbg !1020 {
start:
  %self.dbg.spill.i15 = alloca i64*, align 4
  %self.dbg.spill.i14 = alloca i64*, align 4
  %self.dbg.spill.i13 = alloca i64*, align 4
  %val.dbg.spill.i11 = alloca i64, align 8
  %self.dbg.spill.i12 = alloca i64*, align 4
  %val.dbg.spill.i9 = alloca i64, align 8
  %self.dbg.spill.i10 = alloca i64*, align 4
  %val.dbg.spill.i7 = alloca i64, align 8
  %self.dbg.spill.i8 = alloca i64*, align 4
  %value.dbg.spill.i.i5 = alloca i64, align 8
  %0 = alloca i64, align 8
  %val.dbg.spill.i6 = alloca i64, align 8
  %1 = alloca i64, align 8
  %value.dbg.spill.i.i3 = alloca i64, align 8
  %2 = alloca i64, align 8
  %val.dbg.spill.i4 = alloca i64, align 8
  %3 = alloca i64, align 8
  %value.dbg.spill.i.i = alloca i64, align 8
  %4 = alloca i64, align 8
  %val.dbg.spill.i = alloca i64, align 8
  %5 = alloca i64, align 8
  %self.dbg.spill.i2 = alloca i64*, align 4
  %self.dbg.spill.i1 = alloca i64*, align 4
  %self.dbg.spill.i = alloca i64*, align 4
  %_6.i = alloca %"app::Local", align 8
  %monotonics.dbg.spill = alloca %"app::__rtic_internal_Monotonics", align 1
  %shared_resources.dbg.spill = alloca %"app::Shared", align 1
  %_1.dbg.spill = alloca %"[closure@examples/locals.rs:10:1: 10:62].6", align 1
  %_5 = alloca { %"app::Shared", %"app::Local", %"app::__rtic_internal_Monotonics" }, align 8
  %local_resources = alloca %"app::Local", align 8
  %6 = bitcast %"[closure@examples/locals.rs:10:1: 10:62].6"* %_1.dbg.spill to %"cortex_m::peripheral::Peripherals"*
  call void @llvm.dbg.declare(metadata %"[closure@examples/locals.rs:10:1: 10:62].6"* %_1.dbg.spill, metadata !1022, metadata !DIExpression()), !dbg !1034
  call void @llvm.dbg.declare(metadata %"app::Shared"* %shared_resources.dbg.spill, metadata !1023, metadata !DIExpression()), !dbg !1035
  call void @llvm.dbg.declare(metadata %"app::Local"* %local_resources, metadata !1026, metadata !DIExpression()), !dbg !1035
  call void @llvm.dbg.declare(metadata %"app::__rtic_internal_Monotonics"* %monotonics.dbg.spill, metadata !1032, metadata !DIExpression()), !dbg !1035
; call <T as core::convert::Into<U>>::into
  call void @"_ZN50_$LT$T$u20$as$u20$core..convert..Into$LT$U$GT$$GT$4into17hb837751d062ab74fE"() #10, !dbg !1034
  br label %bb1, !dbg !1034

bb1:                                              ; preds = %start
  call void @llvm.dbg.declare(metadata %"cortex_m::peripheral::Peripherals"* undef, metadata !1036, metadata !DIExpression()) #10, !dbg !1053
; call lm3s6965::Peripherals::steal
  call void @_ZN8lm3s696511Peripherals5steal17h618f0739b52a798eE() #10, !dbg !1053
  br label %bb2, !dbg !1034

bb2:                                              ; preds = %bb1
  call void @llvm.experimental.noalias.scope.decl(metadata !1055), !dbg !1034
  call void @llvm.dbg.declare(metadata %"app::__rtic_internal_init_Context"* undef, metadata !1058, metadata !DIExpression()) #10, !dbg !1068
; call locals::app::__rtic_internal_foo_spawn
  %_3.i = call zeroext i1 @_ZN6locals3app25__rtic_internal_foo_spawn17hf95648dd5d6bc8a0E() #10, !dbg !1070, !noalias !1055
; call core::result::Result<T,E>::unwrap
  call void @"_ZN4core6result19Result$LT$T$C$E$GT$6unwrap17had281be05a095966E"(i1 zeroext %_3.i, %"core::panic::location::Location"* align 4 dereferenceable(16) bitcast (<{ i8*, [12 x i8] }>* @alloc167 to %"core::panic::location::Location"*)) #10, !dbg !1070, !noalias !1055
; call locals::app::__rtic_internal_bar_spawn
  %_5.i = call zeroext i1 @_ZN6locals3app25__rtic_internal_bar_spawn17hbcc8391a9543253dE() #10, !dbg !1071, !noalias !1055
; call core::result::Result<T,E>::unwrap
  call void @"_ZN4core6result19Result$LT$T$C$E$GT$6unwrap17had281be05a095966E"(i1 zeroext %_5.i, %"core::panic::location::Location"* align 4 dereferenceable(16) bitcast (<{ i8*, [12 x i8] }>* @alloc169 to %"core::panic::location::Location"*)) #10, !dbg !1071, !noalias !1055
  %7 = bitcast %"app::Local"* %_6.i to i64*, !dbg !1072
  store i64 0, i64* %7, align 8, !dbg !1072, !noalias !1055
  %8 = getelementptr inbounds %"app::Local", %"app::Local"* %_6.i, i32 0, i32 1, !dbg !1072
  store i64 0, i64* %8, align 8, !dbg !1072, !noalias !1055
  %9 = getelementptr inbounds %"app::Local", %"app::Local"* %_6.i, i32 0, i32 2, !dbg !1072
  store i64 0, i64* %9, align 8, !dbg !1072, !noalias !1055
  %10 = bitcast { %"app::Shared", %"app::Local", %"app::__rtic_internal_Monotonics" }* %_5 to %"app::Local"*, !dbg !1073
  %11 = bitcast %"app::Local"* %10 to i8*, !dbg !1073
  %12 = bitcast %"app::Local"* %_6.i to i8*, !dbg !1073
  call void @llvm.memcpy.p0i8.p0i8.i32(i8* align 8 %11, i8* align 8 %12, i32 24, i1 false) #10, !dbg !1073
  br label %bb3, !dbg !1034

bb3:                                              ; preds = %bb2
  %13 = bitcast { %"app::Shared", %"app::Local", %"app::__rtic_internal_Monotonics" }* %_5 to %"app::Local"*, !dbg !1034
  %14 = bitcast %"app::Local"* %local_resources to i8*, !dbg !1034
  %15 = bitcast %"app::Local"* %13 to i8*, !dbg !1034
  call void @llvm.memcpy.p0i8.p0i8.i32(i8* align 8 %14, i8* align 8 %15, i32 24, i1 false), !dbg !1034
  store i64* bitcast (<{ [8 x i8] }>* @_ZN6locals3app43__rtic_internal_local_resource_local_to_foo17ha825634817032b38E to i64*), i64** %self.dbg.spill.i2, align 4
  call void @llvm.dbg.declare(metadata i64** %self.dbg.spill.i2, metadata !817, metadata !DIExpression()) #10, !dbg !1074
  store i64* bitcast (<{ [8 x i8] }>* @_ZN6locals3app43__rtic_internal_local_resource_local_to_foo17ha825634817032b38E to i64*), i64** %self.dbg.spill.i13, align 4
  call void @llvm.dbg.declare(metadata i64** %self.dbg.spill.i13, metadata !830, metadata !DIExpression()), !dbg !1076
  br label %bb4, !dbg !1035

bb4:                                              ; preds = %bb3
  %16 = bitcast %"app::Local"* %local_resources to i64*, !dbg !1035
  %_14 = load i64, i64* %16, align 8, !dbg !1035
  store i64 %_14, i64* %val.dbg.spill.i6, align 8
  call void @llvm.dbg.declare(metadata i64* %val.dbg.spill.i6, metadata !1078, metadata !DIExpression()), !dbg !1083
  store i64 %_14, i64* %value.dbg.spill.i.i5, align 8
  call void @llvm.dbg.declare(metadata i64* %value.dbg.spill.i.i5, metadata !1085, metadata !DIExpression()), !dbg !1091
  store i64 %_14, i64* %0, align 8, !dbg !1093
  %17 = load i64, i64* %0, align 8, !dbg !1094
  store i64 %17, i64* %1, align 8, !dbg !1095
  %18 = load i64, i64* %1, align 8, !dbg !1096
  br label %bb5, !dbg !1035

bb5:                                              ; preds = %bb4
  store i64* bitcast (<{ [8 x i8] }>* @_ZN6locals3app43__rtic_internal_local_resource_local_to_foo17ha825634817032b38E to i64*), i64** %self.dbg.spill.i12, align 4
  call void @llvm.dbg.declare(metadata i64** %self.dbg.spill.i12, metadata !1097, metadata !DIExpression()) #10, !dbg !1103
  store i64 %18, i64* %val.dbg.spill.i11, align 8
  call void @llvm.dbg.declare(metadata i64* %val.dbg.spill.i11, metadata !1102, metadata !DIExpression()) #10, !dbg !1105
; call core::ptr::write
  call void @_ZN4core3ptr5write17h82b9f7fb1924c0bdE(i64* bitcast (<{ [8 x i8] }>* @_ZN6locals3app43__rtic_internal_local_resource_local_to_foo17ha825634817032b38E to i64*), i64 %18) #10, !dbg !1106
  br label %bb6, !dbg !1035

bb6:                                              ; preds = %bb5
  store i64* bitcast (<{ [8 x i8] }>* @_ZN6locals3app43__rtic_internal_local_resource_local_to_bar17hedba4fee37a14ecaE to i64*), i64** %self.dbg.spill.i1, align 4
  call void @llvm.dbg.declare(metadata i64** %self.dbg.spill.i1, metadata !817, metadata !DIExpression()) #10, !dbg !1107
  store i64* bitcast (<{ [8 x i8] }>* @_ZN6locals3app43__rtic_internal_local_resource_local_to_bar17hedba4fee37a14ecaE to i64*), i64** %self.dbg.spill.i14, align 4
  call void @llvm.dbg.declare(metadata i64** %self.dbg.spill.i14, metadata !830, metadata !DIExpression()), !dbg !1109
  br label %bb7, !dbg !1035

bb7:                                              ; preds = %bb6
  %19 = getelementptr inbounds %"app::Local", %"app::Local"* %local_resources, i32 0, i32 1, !dbg !1035
  %_20 = load i64, i64* %19, align 8, !dbg !1035
  store i64 %_20, i64* %val.dbg.spill.i4, align 8
  call void @llvm.dbg.declare(metadata i64* %val.dbg.spill.i4, metadata !1078, metadata !DIExpression()), !dbg !1111
  store i64 %_20, i64* %value.dbg.spill.i.i3, align 8
  call void @llvm.dbg.declare(metadata i64* %value.dbg.spill.i.i3, metadata !1085, metadata !DIExpression()), !dbg !1113
  store i64 %_20, i64* %2, align 8, !dbg !1115
  %20 = load i64, i64* %2, align 8, !dbg !1116
  store i64 %20, i64* %3, align 8, !dbg !1117
  %21 = load i64, i64* %3, align 8, !dbg !1118
  br label %bb8, !dbg !1035

bb8:                                              ; preds = %bb7
  store i64* bitcast (<{ [8 x i8] }>* @_ZN6locals3app43__rtic_internal_local_resource_local_to_bar17hedba4fee37a14ecaE to i64*), i64** %self.dbg.spill.i10, align 4
  call void @llvm.dbg.declare(metadata i64** %self.dbg.spill.i10, metadata !1097, metadata !DIExpression()) #10, !dbg !1119
  store i64 %21, i64* %val.dbg.spill.i9, align 8
  call void @llvm.dbg.declare(metadata i64* %val.dbg.spill.i9, metadata !1102, metadata !DIExpression()) #10, !dbg !1121
; call core::ptr::write
  call void @_ZN4core3ptr5write17h82b9f7fb1924c0bdE(i64* bitcast (<{ [8 x i8] }>* @_ZN6locals3app43__rtic_internal_local_resource_local_to_bar17hedba4fee37a14ecaE to i64*), i64 %21) #10, !dbg !1122
  br label %bb9, !dbg !1035

bb9:                                              ; preds = %bb8
  store i64* bitcast (<{ [8 x i8] }>* @_ZN6locals3app44__rtic_internal_local_resource_local_to_idle17h9592b26e73b21be8E to i64*), i64** %self.dbg.spill.i, align 4
  call void @llvm.dbg.declare(metadata i64** %self.dbg.spill.i, metadata !817, metadata !DIExpression()) #10, !dbg !1123
  store i64* bitcast (<{ [8 x i8] }>* @_ZN6locals3app44__rtic_internal_local_resource_local_to_idle17h9592b26e73b21be8E to i64*), i64** %self.dbg.spill.i15, align 4
  call void @llvm.dbg.declare(metadata i64** %self.dbg.spill.i15, metadata !830, metadata !DIExpression()), !dbg !1125
  br label %bb10, !dbg !1035

bb10:                                             ; preds = %bb9
  %22 = getelementptr inbounds %"app::Local", %"app::Local"* %local_resources, i32 0, i32 2, !dbg !1035
  %_26 = load i64, i64* %22, align 8, !dbg !1035
  store i64 %_26, i64* %val.dbg.spill.i, align 8
  call void @llvm.dbg.declare(metadata i64* %val.dbg.spill.i, metadata !1078, metadata !DIExpression()), !dbg !1127
  store i64 %_26, i64* %value.dbg.spill.i.i, align 8
  call void @llvm.dbg.declare(metadata i64* %value.dbg.spill.i.i, metadata !1085, metadata !DIExpression()), !dbg !1129
  store i64 %_26, i64* %4, align 8, !dbg !1131
  %23 = load i64, i64* %4, align 8, !dbg !1132
  store i64 %23, i64* %5, align 8, !dbg !1133
  %24 = load i64, i64* %5, align 8, !dbg !1134
  br label %bb11, !dbg !1035

bb11:                                             ; preds = %bb10
  store i64* bitcast (<{ [8 x i8] }>* @_ZN6locals3app44__rtic_internal_local_resource_local_to_idle17h9592b26e73b21be8E to i64*), i64** %self.dbg.spill.i8, align 4
  call void @llvm.dbg.declare(metadata i64** %self.dbg.spill.i8, metadata !1097, metadata !DIExpression()) #10, !dbg !1135
  store i64 %24, i64* %val.dbg.spill.i7, align 8
  call void @llvm.dbg.declare(metadata i64* %val.dbg.spill.i7, metadata !1102, metadata !DIExpression()) #10, !dbg !1137
; call core::ptr::write
  call void @_ZN4core3ptr5write17h82b9f7fb1924c0bdE(i64* bitcast (<{ [8 x i8] }>* @_ZN6locals3app44__rtic_internal_local_resource_local_to_idle17h9592b26e73b21be8E to i64*), i64 %24) #10, !dbg !1138
  br label %bb12, !dbg !1035

bb12:                                             ; preds = %bb11
; call cortex_m::interrupt::enable
  call void @_ZN8cortex_m9interrupt6enable17h5a8c08841dc312b2E() #10, !dbg !1035
  br label %bb13, !dbg !1035

bb13:                                             ; preds = %bb12
  ret void, !dbg !1139
}

; rtic::pend
; Function Attrs: nounwind
define internal void @_ZN4rtic4pend17h9dbd054a9ea733f9E(i8 %interrupt) unnamed_addr #0 !dbg !1140 {
start:
  %interrupt.dbg.spill = alloca i8, align 1
  store i8 %interrupt, i8* %interrupt.dbg.spill, align 1
  call void @llvm.dbg.declare(metadata i8* %interrupt.dbg.spill, metadata !1144, metadata !DIExpression()), !dbg !1147
; call cortex_m::peripheral::nvic::<impl cortex_m::peripheral::NVIC>::pend
  call void @"_ZN8cortex_m10peripheral4nvic44_$LT$impl$u20$cortex_m..peripheral..NVIC$GT$4pend17h0c9c6720008e79b3E"(i8 %interrupt) #10, !dbg !1148
  br label %bb1, !dbg !1148

bb1:                                              ; preds = %start
  ret void, !dbg !1149
}

; cortex_m::peripheral::nvic::<impl cortex_m::peripheral::NVIC>::unmask
; Function Attrs: inlinehint nounwind
define internal void @"_ZN8cortex_m10peripheral4nvic44_$LT$impl$u20$cortex_m..peripheral..NVIC$GT$6unmask17h825ea550b82b57f2E"(i8 %interrupt) unnamed_addr #1 !dbg !1150 {
start:
  %self.dbg.spill.i2 = alloca i32*, align 4
  %value.dbg.spill.i.i = alloca i32, align 4
  %self.dbg.spill.i.i = alloca i32*, align 4
  %value.dbg.spill.i = alloca i32, align 4
  %self.dbg.spill.i = alloca i32*, align 4
  %nr.dbg.spill = alloca i16, align 2
  %interrupt.dbg.spill = alloca i8, align 1
  store i8 %interrupt, i8* %interrupt.dbg.spill, align 1
  call void @llvm.dbg.declare(metadata i8* %interrupt.dbg.spill, metadata !1153, metadata !DIExpression()), !dbg !1156
; call <T as cortex_m::interrupt::InterruptNumber>::number
  %nr = call i16 @"_ZN58_$LT$T$u20$as$u20$cortex_m..interrupt..InterruptNumber$GT$6number17h7a5d7a0fd28511f6E"(i8 %interrupt) #10, !dbg !1157
  store i16 %nr, i16* %nr.dbg.spill, align 2, !dbg !1157
  call void @llvm.dbg.declare(metadata i16* %nr.dbg.spill, metadata !1154, metadata !DIExpression()), !dbg !1158
  br label %bb1, !dbg !1157

bb1:                                              ; preds = %start
  br label %bb2, !dbg !1159

bb2:                                              ; preds = %bb1
  %_7 = udiv i16 %nr, 32, !dbg !1160
; call core::convert::num::<impl core::convert::From<u16> for usize>::from
  %_6 = call i32 @"_ZN4core7convert3num66_$LT$impl$u20$core..convert..From$LT$u16$GT$$u20$for$u20$usize$GT$4from17hb2e480a8ed69ba2dE"(i16 %_7) #10, !dbg !1161
  br label %bb3, !dbg !1161

bb3:                                              ; preds = %bb2
  %_10 = icmp ult i32 %_6, 16, !dbg !1162
  %0 = call i1 @llvm.expect.i1(i1 %_10, i1 true), !dbg !1162
  br i1 %0, label %bb4, label %panic, !dbg !1162

bb4:                                              ; preds = %bb3
  %1 = bitcast %"cortex_m::peripheral::nvic::RegisterBlock"* inttoptr (i32 -536813312 to %"cortex_m::peripheral::nvic::RegisterBlock"*) to [16 x i32]*, !dbg !1162
  %_4 = getelementptr inbounds [16 x i32], [16 x i32]* %1, i32 0, i32 %_6, !dbg !1162
  %_12 = urem i16 %nr, 32, !dbg !1163
  %2 = and i16 %_12, -32, !dbg !1164
  %_14.1 = icmp ne i16 %2, 0, !dbg !1164
  %3 = zext i16 %_12 to i32, !dbg !1164
  %4 = and i32 %3, 31, !dbg !1164
  %_14.0 = shl i32 1, %4, !dbg !1164
  %5 = call i1 @llvm.expect.i1(i1 %_14.1, i1 false), !dbg !1164
  br i1 %5, label %panic1, label %bb5, !dbg !1164

panic:                                            ; preds = %bb3
; call core::panicking::panic_bounds_check
  call void @_ZN4core9panicking18panic_bounds_check17h5f129c306aeb31cbE(i32 %_6, i32 16, %"core::panic::location::Location"* align 4 dereferenceable(16) bitcast (<{ i8*, [12 x i8] }>* @alloc189 to %"core::panic::location::Location"*)) #11, !dbg !1162
  unreachable, !dbg !1162

bb5:                                              ; preds = %bb4
  store i32* %_4, i32** %self.dbg.spill.i, align 4
  call void @llvm.dbg.declare(metadata i32** %self.dbg.spill.i, metadata !1165, metadata !DIExpression()) #10, !dbg !1187
  store i32 %_14.0, i32* %value.dbg.spill.i, align 4
  call void @llvm.dbg.declare(metadata i32* %value.dbg.spill.i, metadata !1186, metadata !DIExpression()) #10, !dbg !1189
  store i32* %_4, i32** %self.dbg.spill.i.i, align 4
  call void @llvm.dbg.declare(metadata i32** %self.dbg.spill.i.i, metadata !1190, metadata !DIExpression()) #10, !dbg !1198
  store i32 %_14.0, i32* %value.dbg.spill.i.i, align 4
  call void @llvm.dbg.declare(metadata i32* %value.dbg.spill.i.i, metadata !1197, metadata !DIExpression()) #10, !dbg !1200
  store i32* %_4, i32** %self.dbg.spill.i2, align 4
  call void @llvm.dbg.declare(metadata i32** %self.dbg.spill.i2, metadata !1201, metadata !DIExpression()), !dbg !1208
; call core::ptr::write_volatile
  call void @_ZN4core3ptr14write_volatile17h50533fac3ae79608E(i32* %_4, i32 %_14.0) #10, !dbg !1210
  br label %bb6, !dbg !1162

panic1:                                           ; preds = %bb4
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17h9e72682d8e5e8cf1E([0 x i8]* nonnull align 1 bitcast ([35 x i8]* @str.1 to [0 x i8]*), i32 35, %"core::panic::location::Location"* align 4 dereferenceable(16) bitcast (<{ i8*, [12 x i8] }>* @alloc191 to %"core::panic::location::Location"*)) #11, !dbg !1164
  unreachable, !dbg !1164

bb6:                                              ; preds = %bb5
  ret void, !dbg !1211
}

; cortex_m::peripheral::nvic::<impl cortex_m::peripheral::NVIC>::pend
; Function Attrs: inlinehint nounwind
define internal void @"_ZN8cortex_m10peripheral4nvic44_$LT$impl$u20$cortex_m..peripheral..NVIC$GT$4pend17h0c9c6720008e79b3E"(i8 %interrupt) unnamed_addr #1 !dbg !1212 {
start:
  %self.dbg.spill.i2 = alloca i32*, align 4
  %value.dbg.spill.i.i = alloca i32, align 4
  %self.dbg.spill.i.i = alloca i32*, align 4
  %value.dbg.spill.i = alloca i32, align 4
  %self.dbg.spill.i = alloca i32*, align 4
  %nr.dbg.spill = alloca i16, align 2
  %interrupt.dbg.spill = alloca i8, align 1
  store i8 %interrupt, i8* %interrupt.dbg.spill, align 1
  call void @llvm.dbg.declare(metadata i8* %interrupt.dbg.spill, metadata !1214, metadata !DIExpression()), !dbg !1217
; call <T as cortex_m::interrupt::InterruptNumber>::number
  %nr = call i16 @"_ZN58_$LT$T$u20$as$u20$cortex_m..interrupt..InterruptNumber$GT$6number17h7a5d7a0fd28511f6E"(i8 %interrupt) #10, !dbg !1218
  store i16 %nr, i16* %nr.dbg.spill, align 2, !dbg !1218
  call void @llvm.dbg.declare(metadata i16* %nr.dbg.spill, metadata !1215, metadata !DIExpression()), !dbg !1219
  br label %bb1, !dbg !1218

bb1:                                              ; preds = %start
  br label %bb2, !dbg !1220

bb2:                                              ; preds = %bb1
  %_7 = udiv i16 %nr, 32, !dbg !1221
; call core::convert::num::<impl core::convert::From<u16> for usize>::from
  %_6 = call i32 @"_ZN4core7convert3num66_$LT$impl$u20$core..convert..From$LT$u16$GT$$u20$for$u20$usize$GT$4from17hb2e480a8ed69ba2dE"(i16 %_7) #10, !dbg !1222
  br label %bb3, !dbg !1222

bb3:                                              ; preds = %bb2
  %_10 = icmp ult i32 %_6, 16, !dbg !1223
  %0 = call i1 @llvm.expect.i1(i1 %_10, i1 true), !dbg !1223
  br i1 %0, label %bb4, label %panic, !dbg !1223

bb4:                                              ; preds = %bb3
  %1 = getelementptr inbounds %"cortex_m::peripheral::nvic::RegisterBlock", %"cortex_m::peripheral::nvic::RegisterBlock"* inttoptr (i32 -536813312 to %"cortex_m::peripheral::nvic::RegisterBlock"*), i32 0, i32 4, !dbg !1223
  %_4 = getelementptr inbounds [16 x i32], [16 x i32]* %1, i32 0, i32 %_6, !dbg !1223
  %_12 = urem i16 %nr, 32, !dbg !1224
  %2 = and i16 %_12, -32, !dbg !1225
  %_14.1 = icmp ne i16 %2, 0, !dbg !1225
  %3 = zext i16 %_12 to i32, !dbg !1225
  %4 = and i32 %3, 31, !dbg !1225
  %_14.0 = shl i32 1, %4, !dbg !1225
  %5 = call i1 @llvm.expect.i1(i1 %_14.1, i1 false), !dbg !1225
  br i1 %5, label %panic1, label %bb5, !dbg !1225

panic:                                            ; preds = %bb3
; call core::panicking::panic_bounds_check
  call void @_ZN4core9panicking18panic_bounds_check17h5f129c306aeb31cbE(i32 %_6, i32 16, %"core::panic::location::Location"* align 4 dereferenceable(16) bitcast (<{ i8*, [12 x i8] }>* @alloc193 to %"core::panic::location::Location"*)) #11, !dbg !1223
  unreachable, !dbg !1223

bb5:                                              ; preds = %bb4
  store i32* %_4, i32** %self.dbg.spill.i, align 4
  call void @llvm.dbg.declare(metadata i32** %self.dbg.spill.i, metadata !1165, metadata !DIExpression()) #10, !dbg !1226
  store i32 %_14.0, i32* %value.dbg.spill.i, align 4
  call void @llvm.dbg.declare(metadata i32* %value.dbg.spill.i, metadata !1186, metadata !DIExpression()) #10, !dbg !1228
  store i32* %_4, i32** %self.dbg.spill.i.i, align 4
  call void @llvm.dbg.declare(metadata i32** %self.dbg.spill.i.i, metadata !1190, metadata !DIExpression()) #10, !dbg !1229
  store i32 %_14.0, i32* %value.dbg.spill.i.i, align 4
  call void @llvm.dbg.declare(metadata i32* %value.dbg.spill.i.i, metadata !1197, metadata !DIExpression()) #10, !dbg !1231
  store i32* %_4, i32** %self.dbg.spill.i2, align 4
  call void @llvm.dbg.declare(metadata i32** %self.dbg.spill.i2, metadata !1201, metadata !DIExpression()), !dbg !1232
; call core::ptr::write_volatile
  call void @_ZN4core3ptr14write_volatile17h50533fac3ae79608E(i32* %_4, i32 %_14.0) #10, !dbg !1234
  br label %bb6, !dbg !1223

panic1:                                           ; preds = %bb4
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17h9e72682d8e5e8cf1E([0 x i8]* nonnull align 1 bitcast ([35 x i8]* @str.1 to [0 x i8]*), i32 35, %"core::panic::location::Location"* align 4 dereferenceable(16) bitcast (<{ i8*, [12 x i8] }>* @alloc195 to %"core::panic::location::Location"*)) #11, !dbg !1225
  unreachable, !dbg !1225

bb6:                                              ; preds = %bb5
  ret void, !dbg !1235
}

; cortex_m::peripheral::nvic::<impl cortex_m::peripheral::NVIC>::set_priority
; Function Attrs: inlinehint nounwind
define internal void @"_ZN8cortex_m10peripheral4nvic44_$LT$impl$u20$cortex_m..peripheral..NVIC$GT$12set_priority17h755c18aead94bf5aE"(%"cortex_m::peripheral::NVIC"* nonnull align 1 %self, i8 %interrupt, i8 %prio) unnamed_addr #1 !dbg !1236 {
start:
  %self.dbg.spill.i2 = alloca i8*, align 4
  %self.dbg.spill.i1 = alloca %"cortex_m::peripheral::NVIC"*, align 4
  %value.dbg.spill.i.i = alloca i8, align 1
  %self.dbg.spill.i.i = alloca i8*, align 4
  %value.dbg.spill.i = alloca i8, align 1
  %self.dbg.spill.i = alloca i8*, align 4
  %nr.dbg.spill = alloca i16, align 2
  %prio.dbg.spill = alloca i8, align 1
  %interrupt.dbg.spill = alloca i8, align 1
  %self.dbg.spill = alloca %"cortex_m::peripheral::NVIC"*, align 4
  store %"cortex_m::peripheral::NVIC"* %self, %"cortex_m::peripheral::NVIC"** %self.dbg.spill, align 4
  call void @llvm.dbg.declare(metadata %"cortex_m::peripheral::NVIC"** %self.dbg.spill, metadata !1241, metadata !DIExpression()), !dbg !1246
  store i8 %interrupt, i8* %interrupt.dbg.spill, align 1
  call void @llvm.dbg.declare(metadata i8* %interrupt.dbg.spill, metadata !1242, metadata !DIExpression()), !dbg !1247
  store i8 %prio, i8* %prio.dbg.spill, align 1
  call void @llvm.dbg.declare(metadata i8* %prio.dbg.spill, metadata !1243, metadata !DIExpression()), !dbg !1248
; call <T as cortex_m::interrupt::InterruptNumber>::number
  %nr = call i16 @"_ZN58_$LT$T$u20$as$u20$cortex_m..interrupt..InterruptNumber$GT$6number17h7a5d7a0fd28511f6E"(i8 %interrupt) #10, !dbg !1249
  store i16 %nr, i16* %nr.dbg.spill, align 2, !dbg !1249
  call void @llvm.dbg.declare(metadata i16* %nr.dbg.spill, metadata !1244, metadata !DIExpression()), !dbg !1250
  br label %bb1, !dbg !1249

bb1:                                              ; preds = %start
  store %"cortex_m::peripheral::NVIC"* %self, %"cortex_m::peripheral::NVIC"** %self.dbg.spill.i1, align 4
  call void @llvm.dbg.declare(metadata %"cortex_m::peripheral::NVIC"** %self.dbg.spill.i1, metadata !1251, metadata !DIExpression()), !dbg !1302
  br label %bb2, !dbg !1304

bb2:                                              ; preds = %bb1
; call core::convert::num::<impl core::convert::From<u16> for usize>::from
  %_9 = call i32 @"_ZN4core7convert3num66_$LT$impl$u20$core..convert..From$LT$u16$GT$$u20$for$u20$usize$GT$4from17hb2e480a8ed69ba2dE"(i16 %nr) #10, !dbg !1305
  br label %bb3, !dbg !1305

bb3:                                              ; preds = %bb2
  %_12 = icmp ult i32 %_9, 496, !dbg !1304
  %0 = call i1 @llvm.expect.i1(i1 %_12, i1 true), !dbg !1304
  br i1 %0, label %bb4, label %panic, !dbg !1304

bb4:                                              ; preds = %bb3
  %1 = getelementptr inbounds %"cortex_m::peripheral::nvic::RegisterBlock", %"cortex_m::peripheral::nvic::RegisterBlock"* inttoptr (i32 -536813312 to %"cortex_m::peripheral::nvic::RegisterBlock"*), i32 0, i32 10, !dbg !1304
  %_6 = getelementptr inbounds [496 x i8], [496 x i8]* %1, i32 0, i32 %_9, !dbg !1304
  store i8* %_6, i8** %self.dbg.spill.i, align 4
  call void @llvm.dbg.declare(metadata i8** %self.dbg.spill.i, metadata !1306, metadata !DIExpression()) #10, !dbg !1313
  store i8 %prio, i8* %value.dbg.spill.i, align 1
  call void @llvm.dbg.declare(metadata i8* %value.dbg.spill.i, metadata !1312, metadata !DIExpression()) #10, !dbg !1315
  store i8* %_6, i8** %self.dbg.spill.i.i, align 4
  call void @llvm.dbg.declare(metadata i8** %self.dbg.spill.i.i, metadata !1316, metadata !DIExpression()) #10, !dbg !1323
  store i8 %prio, i8* %value.dbg.spill.i.i, align 1
  call void @llvm.dbg.declare(metadata i8* %value.dbg.spill.i.i, metadata !1322, metadata !DIExpression()) #10, !dbg !1325
  store i8* %_6, i8** %self.dbg.spill.i2, align 4
  call void @llvm.dbg.declare(metadata i8** %self.dbg.spill.i2, metadata !1326, metadata !DIExpression()), !dbg !1333
; call core::ptr::write_volatile
  call void @_ZN4core3ptr14write_volatile17h3f979f4073c91a57E(i8* %_6, i8 %prio) #10, !dbg !1335
  br label %bb5, !dbg !1304

panic:                                            ; preds = %bb3
; call core::panicking::panic_bounds_check
  call void @_ZN4core9panicking18panic_bounds_check17h5f129c306aeb31cbE(i32 %_9, i32 496, %"core::panic::location::Location"* align 4 dereferenceable(16) bitcast (<{ i8*, [12 x i8] }>* @alloc197 to %"core::panic::location::Location"*)) #11, !dbg !1304
  unreachable, !dbg !1304

bb5:                                              ; preds = %bb4
  ret void, !dbg !1336
}

; cortex_m::peripheral::Peripherals::steal
; Function Attrs: inlinehint nounwind
define internal void @_ZN8cortex_m10peripheral11Peripherals5steal17hf4660d70e6ed8445E() unnamed_addr #1 !dbg !1337 {
start:
  store i8 1, i8* @_ZN8cortex_m10peripheral5TAKEN17h164fa9be397b07d5E, align 1, !dbg !1338
  ret void, !dbg !1339
}

; cortex_m::register::basepri::read
; Function Attrs: inlinehint nounwind
define internal i8 @_ZN8cortex_m8register7basepri4read17hab22038c38f4f26dE() unnamed_addr #1 !dbg !1340 {
start:
  %0 = call zeroext i8 @__basepri_r() #10, !dbg !1345
  br label %bb1, !dbg !1345

bb1:                                              ; preds = %start
  ret i8 %0, !dbg !1346
}

; cortex_m::register::basepri::write
; Function Attrs: inlinehint nounwind
define internal void @_ZN8cortex_m8register7basepri5write17h8c83f3cc5950a711E(i8 %basepri) unnamed_addr #1 !dbg !1347 {
start:
  %basepri.dbg.spill = alloca i8, align 1
  store i8 %basepri, i8* %basepri.dbg.spill, align 1
  call void @llvm.dbg.declare(metadata i8* %basepri.dbg.spill, metadata !1351, metadata !DIExpression()), !dbg !1352
  call void @__basepri_w(i8 zeroext %basepri) #10, !dbg !1353
  br label %bb1, !dbg !1353

bb1:                                              ; preds = %start
  ret void, !dbg !1354
}

; cortex_m::register::primask::Primask::is_active
; Function Attrs: inlinehint nounwind
define internal zeroext i1 @_ZN8cortex_m8register7primask7Primask9is_active17h26a0e7131c940062E(i1 zeroext %0) unnamed_addr #1 !dbg !1355 {
start:
  %self = alloca i8, align 1
  %1 = zext i1 %0 to i8
  store i8 %1, i8* %self, align 1
  call void @llvm.dbg.declare(metadata i8* %self, metadata !1361, metadata !DIExpression()), !dbg !1362
; call <cortex_m::register::primask::Primask as core::cmp::PartialEq>::eq
  %2 = call zeroext i1 @"_ZN77_$LT$cortex_m..register..primask..Primask$u20$as$u20$core..cmp..PartialEq$GT$2eq17h65a53084a9ea4fb7E"(i8* align 1 dereferenceable(1) %self, i8* align 1 dereferenceable(1) getelementptr inbounds (<{ [1 x i8] }>, <{ [1 x i8] }>* @alloc143, i32 0, i32 0, i32 0)) #10, !dbg !1363
  br label %bb1, !dbg !1363

bb1:                                              ; preds = %start
  ret i1 %2, !dbg !1364
}

; cortex_m::register::primask::read
; Function Attrs: inlinehint nounwind
define internal zeroext i1 @_ZN8cortex_m8register7primask4read17he7f40456c7aa8a00E() unnamed_addr #1 !dbg !1365 {
start:
  %r.dbg.spill = alloca i32, align 4
  %0 = alloca i8, align 1
  %r = call i32 @__primask_r() #10, !dbg !1371
  store i32 %r, i32* %r.dbg.spill, align 4, !dbg !1371
  call void @llvm.dbg.declare(metadata i32* %r.dbg.spill, metadata !1369, metadata !DIExpression()), !dbg !1372
  br label %bb1, !dbg !1371

bb1:                                              ; preds = %start
  %_3 = and i32 %r, 1, !dbg !1373
  %_2 = icmp eq i32 %_3, 1, !dbg !1373
  br i1 %_2, label %bb2, label %bb3, !dbg !1373

bb3:                                              ; preds = %bb1
  store i8 0, i8* %0, align 1, !dbg !1374
  br label %bb4, !dbg !1375

bb2:                                              ; preds = %bb1
  store i8 1, i8* %0, align 1, !dbg !1376
  br label %bb4, !dbg !1375

bb4:                                              ; preds = %bb3, %bb2
  %1 = load i8, i8* %0, align 1, !dbg !1377, !range !436
  %2 = trunc i8 %1 to i1, !dbg !1377
  ret i1 %2, !dbg !1377
}

; <cortex_m::register::primask::Primask as core::cmp::PartialEq>::eq
; Function Attrs: inlinehint nounwind
define internal zeroext i1 @"_ZN77_$LT$cortex_m..register..primask..Primask$u20$as$u20$core..cmp..PartialEq$GT$2eq17h65a53084a9ea4fb7E"(i8* align 1 dereferenceable(1) %self, i8* align 1 dereferenceable(1) %other) unnamed_addr #1 !dbg !1378 {
start:
  %__arg_1_vi.dbg.spill = alloca i32, align 4
  %__self_vi.dbg.spill = alloca i32, align 4
  %other.dbg.spill = alloca i8*, align 4
  %self.dbg.spill = alloca i8*, align 4
  %0 = alloca i8, align 1
  store i8* %self, i8** %self.dbg.spill, align 4
  call void @llvm.dbg.declare(metadata i8** %self.dbg.spill, metadata !1384, metadata !DIExpression()), !dbg !1391
  store i8* %other, i8** %other.dbg.spill, align 4
  call void @llvm.dbg.declare(metadata i8** %other.dbg.spill, metadata !1385, metadata !DIExpression()), !dbg !1391
  %1 = load i8, i8* %self, align 1, !dbg !1391, !range !436
  %2 = trunc i8 %1 to i1, !dbg !1391
  %__self_vi = zext i1 %2 to i32, !dbg !1391
  store i32 %__self_vi, i32* %__self_vi.dbg.spill, align 4, !dbg !1391
  call void @llvm.dbg.declare(metadata i32* %__self_vi.dbg.spill, metadata !1386, metadata !DIExpression()), !dbg !1392
  %3 = load i8, i8* %other, align 1, !dbg !1392, !range !436
  %4 = trunc i8 %3 to i1, !dbg !1392
  %__arg_1_vi = zext i1 %4 to i32, !dbg !1392
  store i32 %__arg_1_vi, i32* %__arg_1_vi.dbg.spill, align 4, !dbg !1392
  call void @llvm.dbg.declare(metadata i32* %__arg_1_vi.dbg.spill, metadata !1389, metadata !DIExpression()), !dbg !1393
  %_10 = icmp eq i32 %__self_vi, %__arg_1_vi, !dbg !1393
  br i1 %_10, label %bb1, label %bb2, !dbg !1393

bb2:                                              ; preds = %start
  store i8 0, i8* %0, align 1, !dbg !1393
  br label %bb3, !dbg !1393

bb1:                                              ; preds = %start
  store i8 1, i8* %0, align 1, !dbg !1393
  br label %bb3, !dbg !1393

bb3:                                              ; preds = %bb2, %bb1
  %5 = load i8, i8* %0, align 1, !dbg !1394, !range !436
  %6 = trunc i8 %5 to i1, !dbg !1394
  ret i1 %6, !dbg !1394
}

; core::mem::replace
; Function Attrs: inlinehint nounwind
define internal i8 @_ZN4core3mem7replace17h6f6f0ffe151c8c1aE(i8* align 1 dereferenceable(1) %dest, i8 %src) unnamed_addr #1 !dbg !1395 {
start:
  %result.dbg.spill = alloca i8, align 1
  %src.dbg.spill = alloca i8, align 1
  %dest.dbg.spill = alloca i8*, align 4
  store i8* %dest, i8** %dest.dbg.spill, align 4
  call void @llvm.dbg.declare(metadata i8** %dest.dbg.spill, metadata !1401, metadata !DIExpression()), !dbg !1405
  store i8 %src, i8* %src.dbg.spill, align 1
  call void @llvm.dbg.declare(metadata i8* %src.dbg.spill, metadata !1402, metadata !DIExpression()), !dbg !1406
; call core::ptr::read
  %result = call i8 @_ZN4core3ptr4read17h7efa0474a61340eeE(i8* %dest) #10, !dbg !1407
  store i8 %result, i8* %result.dbg.spill, align 1, !dbg !1407
  call void @llvm.dbg.declare(metadata i8* %result.dbg.spill, metadata !1403, metadata !DIExpression()), !dbg !1408
  br label %bb1, !dbg !1407

bb1:                                              ; preds = %start
; call core::ptr::write
  call void @_ZN4core3ptr5write17h44c3a12bdde493dfE(i8* %dest, i8 %src) #10, !dbg !1409
  br label %bb2, !dbg !1409

bb2:                                              ; preds = %bb1
  ret i8 %result, !dbg !1410
}

; heapless::spsc::Queue<T,_>::increment
; Function Attrs: inlinehint nounwind
define internal i32 @"_ZN8heapless4spsc18Queue$LT$T$C$_$GT$9increment17h508ec96413021199E"(i32 %val) unnamed_addr #1 !dbg !1411 {
start:
  %val.dbg.spill = alloca i32, align 4
  store i32 %val, i32* %val.dbg.spill, align 4
  call void @llvm.dbg.declare(metadata i32* %val.dbg.spill, metadata !1416, metadata !DIExpression()), !dbg !1417
  %0 = call { i32, i1 } @llvm.uadd.with.overflow.i32(i32 %val, i32 1), !dbg !1418
  %_4.0 = extractvalue { i32, i1 } %0, 0, !dbg !1418
  %_4.1 = extractvalue { i32, i1 } %0, 1, !dbg !1418
  %1 = call i1 @llvm.expect.i1(i1 %_4.1, i1 false), !dbg !1418
  br i1 %1, label %panic, label %bb1, !dbg !1418

bb1:                                              ; preds = %start
  br label %bb2, !dbg !1418

panic:                                            ; preds = %start
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17h9e72682d8e5e8cf1E([0 x i8]* nonnull align 1 bitcast ([28 x i8]* @str.0 to [0 x i8]*), i32 28, %"core::panic::location::Location"* align 4 dereferenceable(16) bitcast (<{ i8*, [12 x i8] }>* @alloc199 to %"core::panic::location::Location"*)) #11, !dbg !1418
  unreachable, !dbg !1418

bb2:                                              ; preds = %bb1
  %2 = urem i32 %_4.0, 3, !dbg !1418
  ret i32 %2, !dbg !1419
}

; heapless::spsc::Queue<T,_>::increment
; Function Attrs: inlinehint nounwind
define internal i32 @"_ZN8heapless4spsc18Queue$LT$T$C$_$GT$9increment17h62b7694f13bd5c4aE"(i32 %val) unnamed_addr #1 !dbg !1420 {
start:
  %val.dbg.spill = alloca i32, align 4
  store i32 %val, i32* %val.dbg.spill, align 4
  call void @llvm.dbg.declare(metadata i32* %val.dbg.spill, metadata !1422, metadata !DIExpression()), !dbg !1423
  %0 = call { i32, i1 } @llvm.uadd.with.overflow.i32(i32 %val, i32 1), !dbg !1424
  %_4.0 = extractvalue { i32, i1 } %0, 0, !dbg !1424
  %_4.1 = extractvalue { i32, i1 } %0, 1, !dbg !1424
  %1 = call i1 @llvm.expect.i1(i1 %_4.1, i1 false), !dbg !1424
  br i1 %1, label %panic, label %bb1, !dbg !1424

bb1:                                              ; preds = %start
  br label %bb2, !dbg !1424

panic:                                            ; preds = %start
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17h9e72682d8e5e8cf1E([0 x i8]* nonnull align 1 bitcast ([28 x i8]* @str.0 to [0 x i8]*), i32 28, %"core::panic::location::Location"* align 4 dereferenceable(16) bitcast (<{ i8*, [12 x i8] }>* @alloc199 to %"core::panic::location::Location"*)) #11, !dbg !1424
  unreachable, !dbg !1424

bb2:                                              ; preds = %bb1
  %2 = urem i32 %_4.0, 2, !dbg !1424
  ret i32 %2, !dbg !1425
}

; heapless::spsc::Queue<T,_>::len
; Function Attrs: inlinehint nounwind
define internal i32 @"_ZN8heapless4spsc18Queue$LT$T$C$_$GT$3len17h0313f1eb84a18b2bE"(%"heapless::spsc::Queue<u8, 2_usize>"* align 4 dereferenceable(12) %self) unnamed_addr #1 !dbg !1426 {
start:
  %rhs.dbg.spill.i1 = alloca i32, align 4
  %self.dbg.spill.i2 = alloca i32, align 4
  %rhs.dbg.spill.i = alloca i32, align 4
  %self.dbg.spill.i = alloca i32, align 4
  %current_tail.dbg.spill = alloca i32, align 4
  %current_head.dbg.spill = alloca i32, align 4
  %self.dbg.spill = alloca %"heapless::spsc::Queue<u8, 2_usize>"*, align 4
  %_7 = alloca i8, align 1
  %_4 = alloca i8, align 1
  store %"heapless::spsc::Queue<u8, 2_usize>"* %self, %"heapless::spsc::Queue<u8, 2_usize>"** %self.dbg.spill, align 4
  call void @llvm.dbg.declare(metadata %"heapless::spsc::Queue<u8, 2_usize>"** %self.dbg.spill, metadata !1431, metadata !DIExpression()), !dbg !1436
  %_3 = bitcast %"heapless::spsc::Queue<u8, 2_usize>"* %self to %"core::sync::atomic::AtomicUsize"*, !dbg !1437
  store i8 0, i8* %_4, align 1, !dbg !1438
  %0 = load i8, i8* %_4, align 1, !dbg !1437, !range !1439
; call core::sync::atomic::AtomicUsize::load
  %current_head = call i32 @_ZN4core4sync6atomic11AtomicUsize4load17hbff932a919f9f519E(%"core::sync::atomic::AtomicUsize"* align 4 dereferenceable(4) %_3, i8 %0) #10, !dbg !1437
  store i32 %current_head, i32* %current_head.dbg.spill, align 4, !dbg !1437
  call void @llvm.dbg.declare(metadata i32* %current_head.dbg.spill, metadata !1432, metadata !DIExpression()), !dbg !1440
  br label %bb1, !dbg !1437

bb1:                                              ; preds = %start
  %_6 = getelementptr inbounds %"heapless::spsc::Queue<u8, 2_usize>", %"heapless::spsc::Queue<u8, 2_usize>"* %self, i32 0, i32 1, !dbg !1441
  store i8 0, i8* %_7, align 1, !dbg !1442
  %1 = load i8, i8* %_7, align 1, !dbg !1441, !range !1439
; call core::sync::atomic::AtomicUsize::load
  %current_tail = call i32 @_ZN4core4sync6atomic11AtomicUsize4load17hbff932a919f9f519E(%"core::sync::atomic::AtomicUsize"* align 4 dereferenceable(4) %_6, i8 %1) #10, !dbg !1441
  store i32 %current_tail, i32* %current_tail.dbg.spill, align 4, !dbg !1441
  call void @llvm.dbg.declare(metadata i32* %current_tail.dbg.spill, metadata !1434, metadata !DIExpression()), !dbg !1443
  br label %bb2, !dbg !1441

bb2:                                              ; preds = %bb1
  store i32 %current_tail, i32* %self.dbg.spill.i2, align 4
  call void @llvm.dbg.declare(metadata i32* %self.dbg.spill.i2, metadata !1444, metadata !DIExpression()), !dbg !1453
  store i32 %current_head, i32* %rhs.dbg.spill.i1, align 4
  call void @llvm.dbg.declare(metadata i32* %rhs.dbg.spill.i1, metadata !1452, metadata !DIExpression()), !dbg !1455
  %2 = sub i32 %current_tail, %current_head, !dbg !1456
  br label %bb3, !dbg !1457

bb3:                                              ; preds = %bb2
  store i32 %2, i32* %self.dbg.spill.i, align 4
  call void @llvm.dbg.declare(metadata i32* %self.dbg.spill.i, metadata !1458, metadata !DIExpression()), !dbg !1462
  store i32 2, i32* %rhs.dbg.spill.i, align 4
  call void @llvm.dbg.declare(metadata i32* %rhs.dbg.spill.i, metadata !1461, metadata !DIExpression()), !dbg !1464
  %3 = add i32 %2, 2, !dbg !1465
  br label %bb4, !dbg !1457

bb4:                                              ; preds = %bb3
  br label %bb5, !dbg !1457

bb5:                                              ; preds = %bb4
  %4 = urem i32 %3, 2, !dbg !1457
  ret i32 %4, !dbg !1466
}

; heapless::spsc::Queue<T,_>::len
; Function Attrs: inlinehint nounwind
define internal i32 @"_ZN8heapless4spsc18Queue$LT$T$C$_$GT$3len17hae3eb13f496ab5d6E"(%"heapless::spsc::Queue<(app::P1_T, u8), 3_usize>"* align 4 dereferenceable(16) %self) unnamed_addr #1 !dbg !1467 {
start:
  %rhs.dbg.spill.i1 = alloca i32, align 4
  %self.dbg.spill.i2 = alloca i32, align 4
  %rhs.dbg.spill.i = alloca i32, align 4
  %self.dbg.spill.i = alloca i32, align 4
  %current_tail.dbg.spill = alloca i32, align 4
  %current_head.dbg.spill = alloca i32, align 4
  %self.dbg.spill = alloca %"heapless::spsc::Queue<(app::P1_T, u8), 3_usize>"*, align 4
  %_7 = alloca i8, align 1
  %_4 = alloca i8, align 1
  store %"heapless::spsc::Queue<(app::P1_T, u8), 3_usize>"* %self, %"heapless::spsc::Queue<(app::P1_T, u8), 3_usize>"** %self.dbg.spill, align 4
  call void @llvm.dbg.declare(metadata %"heapless::spsc::Queue<(app::P1_T, u8), 3_usize>"** %self.dbg.spill, metadata !1472, metadata !DIExpression()), !dbg !1477
  %_3 = bitcast %"heapless::spsc::Queue<(app::P1_T, u8), 3_usize>"* %self to %"core::sync::atomic::AtomicUsize"*, !dbg !1478
  store i8 0, i8* %_4, align 1, !dbg !1479
  %0 = load i8, i8* %_4, align 1, !dbg !1478, !range !1439
; call core::sync::atomic::AtomicUsize::load
  %current_head = call i32 @_ZN4core4sync6atomic11AtomicUsize4load17hbff932a919f9f519E(%"core::sync::atomic::AtomicUsize"* align 4 dereferenceable(4) %_3, i8 %0) #10, !dbg !1478
  store i32 %current_head, i32* %current_head.dbg.spill, align 4, !dbg !1478
  call void @llvm.dbg.declare(metadata i32* %current_head.dbg.spill, metadata !1473, metadata !DIExpression()), !dbg !1480
  br label %bb1, !dbg !1478

bb1:                                              ; preds = %start
  %_6 = getelementptr inbounds %"heapless::spsc::Queue<(app::P1_T, u8), 3_usize>", %"heapless::spsc::Queue<(app::P1_T, u8), 3_usize>"* %self, i32 0, i32 1, !dbg !1481
  store i8 0, i8* %_7, align 1, !dbg !1482
  %1 = load i8, i8* %_7, align 1, !dbg !1481, !range !1439
; call core::sync::atomic::AtomicUsize::load
  %current_tail = call i32 @_ZN4core4sync6atomic11AtomicUsize4load17hbff932a919f9f519E(%"core::sync::atomic::AtomicUsize"* align 4 dereferenceable(4) %_6, i8 %1) #10, !dbg !1481
  store i32 %current_tail, i32* %current_tail.dbg.spill, align 4, !dbg !1481
  call void @llvm.dbg.declare(metadata i32* %current_tail.dbg.spill, metadata !1475, metadata !DIExpression()), !dbg !1483
  br label %bb2, !dbg !1481

bb2:                                              ; preds = %bb1
  store i32 %current_tail, i32* %self.dbg.spill.i2, align 4
  call void @llvm.dbg.declare(metadata i32* %self.dbg.spill.i2, metadata !1444, metadata !DIExpression()), !dbg !1484
  store i32 %current_head, i32* %rhs.dbg.spill.i1, align 4
  call void @llvm.dbg.declare(metadata i32* %rhs.dbg.spill.i1, metadata !1452, metadata !DIExpression()), !dbg !1486
  %2 = sub i32 %current_tail, %current_head, !dbg !1487
  br label %bb3, !dbg !1488

bb3:                                              ; preds = %bb2
  store i32 %2, i32* %self.dbg.spill.i, align 4
  call void @llvm.dbg.declare(metadata i32* %self.dbg.spill.i, metadata !1458, metadata !DIExpression()), !dbg !1489
  store i32 3, i32* %rhs.dbg.spill.i, align 4
  call void @llvm.dbg.declare(metadata i32* %rhs.dbg.spill.i, metadata !1461, metadata !DIExpression()), !dbg !1491
  %3 = add i32 %2, 3, !dbg !1492
  br label %bb4, !dbg !1488

bb4:                                              ; preds = %bb3
  br label %bb5, !dbg !1488

bb5:                                              ; preds = %bb4
  %4 = urem i32 %3, 3, !dbg !1488
  ret i32 %4, !dbg !1493
}

; heapless::spsc::Queue<T,_>::iter_mut
; Function Attrs: nounwind
define internal void @"_ZN8heapless4spsc18Queue$LT$T$C$_$GT$8iter_mut17h39603bd78c1eaa58E"(%"heapless::spsc::IterMut<u8, 2_usize>"* noalias nocapture sret(%"heapless::spsc::IterMut<u8, 2_usize>") dereferenceable(12) %0, %"heapless::spsc::Queue<u8, 2_usize>"* align 4 dereferenceable(12) %self) unnamed_addr #0 !dbg !1494 {
start:
  %len.dbg.spill = alloca i32, align 4
  %self.dbg.spill = alloca %"heapless::spsc::Queue<u8, 2_usize>"*, align 4
  store %"heapless::spsc::Queue<u8, 2_usize>"* %self, %"heapless::spsc::Queue<u8, 2_usize>"** %self.dbg.spill, align 4
  call void @llvm.dbg.declare(metadata %"heapless::spsc::Queue<u8, 2_usize>"** %self.dbg.spill, metadata !1504, metadata !DIExpression()), !dbg !1507
; call heapless::spsc::Queue<T,_>::len
  %len = call i32 @"_ZN8heapless4spsc18Queue$LT$T$C$_$GT$3len17h0313f1eb84a18b2bE"(%"heapless::spsc::Queue<u8, 2_usize>"* align 4 dereferenceable(12) %self) #10, !dbg !1508
  store i32 %len, i32* %len.dbg.spill, align 4, !dbg !1508
  call void @llvm.dbg.declare(metadata i32* %len.dbg.spill, metadata !1505, metadata !DIExpression()), !dbg !1509
  br label %bb1, !dbg !1508

bb1:                                              ; preds = %start
  %1 = bitcast %"heapless::spsc::IterMut<u8, 2_usize>"* %0 to %"heapless::spsc::Queue<u8, 2_usize>"**, !dbg !1510
  store %"heapless::spsc::Queue<u8, 2_usize>"* %self, %"heapless::spsc::Queue<u8, 2_usize>"** %1, align 4, !dbg !1510
  %2 = getelementptr inbounds %"heapless::spsc::IterMut<u8, 2_usize>", %"heapless::spsc::IterMut<u8, 2_usize>"* %0, i32 0, i32 1, !dbg !1510
  store i32 0, i32* %2, align 4, !dbg !1510
  %3 = getelementptr inbounds %"heapless::spsc::IterMut<u8, 2_usize>", %"heapless::spsc::IterMut<u8, 2_usize>"* %0, i32 0, i32 2, !dbg !1510
  store i32 %len, i32* %3, align 4, !dbg !1510
  ret void, !dbg !1511
}

; heapless::spsc::Queue<T,_>::iter_mut
; Function Attrs: nounwind
define internal void @"_ZN8heapless4spsc18Queue$LT$T$C$_$GT$8iter_mut17hf08aa4f1cb5ae041E"(%"heapless::spsc::IterMut<(app::P1_T, u8), 3_usize>"* noalias nocapture sret(%"heapless::spsc::IterMut<(app::P1_T, u8), 3_usize>") dereferenceable(12) %0, %"heapless::spsc::Queue<(app::P1_T, u8), 3_usize>"* align 4 dereferenceable(16) %self) unnamed_addr #0 !dbg !1512 {
start:
  %len.dbg.spill = alloca i32, align 4
  %self.dbg.spill = alloca %"heapless::spsc::Queue<(app::P1_T, u8), 3_usize>"*, align 4
  store %"heapless::spsc::Queue<(app::P1_T, u8), 3_usize>"* %self, %"heapless::spsc::Queue<(app::P1_T, u8), 3_usize>"** %self.dbg.spill, align 4
  call void @llvm.dbg.declare(metadata %"heapless::spsc::Queue<(app::P1_T, u8), 3_usize>"** %self.dbg.spill, metadata !1522, metadata !DIExpression()), !dbg !1525
; call heapless::spsc::Queue<T,_>::len
  %len = call i32 @"_ZN8heapless4spsc18Queue$LT$T$C$_$GT$3len17hae3eb13f496ab5d6E"(%"heapless::spsc::Queue<(app::P1_T, u8), 3_usize>"* align 4 dereferenceable(16) %self) #10, !dbg !1526
  store i32 %len, i32* %len.dbg.spill, align 4, !dbg !1526
  call void @llvm.dbg.declare(metadata i32* %len.dbg.spill, metadata !1523, metadata !DIExpression()), !dbg !1527
  br label %bb1, !dbg !1526

bb1:                                              ; preds = %start
  %1 = bitcast %"heapless::spsc::IterMut<(app::P1_T, u8), 3_usize>"* %0 to %"heapless::spsc::Queue<(app::P1_T, u8), 3_usize>"**, !dbg !1528
  store %"heapless::spsc::Queue<(app::P1_T, u8), 3_usize>"* %self, %"heapless::spsc::Queue<(app::P1_T, u8), 3_usize>"** %1, align 4, !dbg !1528
  %2 = getelementptr inbounds %"heapless::spsc::IterMut<(app::P1_T, u8), 3_usize>", %"heapless::spsc::IterMut<(app::P1_T, u8), 3_usize>"* %0, i32 0, i32 1, !dbg !1528
  store i32 0, i32* %2, align 4, !dbg !1528
  %3 = getelementptr inbounds %"heapless::spsc::IterMut<(app::P1_T, u8), 3_usize>", %"heapless::spsc::IterMut<(app::P1_T, u8), 3_usize>"* %0, i32 0, i32 2, !dbg !1528
  store i32 %len, i32* %3, align 4, !dbg !1528
  ret void, !dbg !1529
}

; heapless::spsc::Queue<T,_>::dequeue
; Function Attrs: inlinehint nounwind
define internal { i8, i8 } @"_ZN8heapless4spsc18Queue$LT$T$C$_$GT$7dequeue17hbddbe69e01b0948fE"(%"heapless::spsc::Queue<u8, 2_usize>"* align 4 dereferenceable(12) %self) unnamed_addr #1 !dbg !1530 {
start:
  %self.dbg.spill = alloca %"heapless::spsc::Queue<u8, 2_usize>"*, align 4
  store %"heapless::spsc::Queue<u8, 2_usize>"* %self, %"heapless::spsc::Queue<u8, 2_usize>"** %self.dbg.spill, align 4
  call void @llvm.dbg.declare(metadata %"heapless::spsc::Queue<u8, 2_usize>"** %self.dbg.spill, metadata !1534, metadata !DIExpression()), !dbg !1535
; call heapless::spsc::Queue<T,_>::inner_dequeue
  %0 = call { i8, i8 } @"_ZN8heapless4spsc18Queue$LT$T$C$_$GT$13inner_dequeue17h69553b06f8d214e3E"(%"heapless::spsc::Queue<u8, 2_usize>"* align 4 dereferenceable(12) %self) #10, !dbg !1536
  %1 = extractvalue { i8, i8 } %0, 0, !dbg !1536
  %2 = trunc i8 %1 to i1, !dbg !1536
  %3 = extractvalue { i8, i8 } %0, 1, !dbg !1536
  br label %bb1, !dbg !1536

bb1:                                              ; preds = %start
  %4 = zext i1 %2 to i8, !dbg !1537
  %5 = insertvalue { i8, i8 } undef, i8 %4, 0, !dbg !1537
  %6 = insertvalue { i8, i8 } %5, i8 %3, 1, !dbg !1537
  ret { i8, i8 } %6, !dbg !1537
}

; heapless::spsc::Queue<T,_>::inner_enqueue_unchecked
; Function Attrs: nounwind
define internal void @"_ZN8heapless4spsc18Queue$LT$T$C$_$GT$23inner_enqueue_unchecked17h89c8e0b55bfe44b7E"(%"heapless::spsc::Queue<u8, 2_usize>"* align 4 dereferenceable(12) %self, i8 %val) unnamed_addr #0 !dbg !1538 {
start:
  %self.dbg.spill.i2 = alloca i8*, align 4
  %val.dbg.spill.i1 = alloca i8, align 1
  %self.dbg.spill.i = alloca i8*, align 4
  %value.dbg.spill.i.i = alloca i8, align 1
  %0 = alloca i8, align 1
  %val.dbg.spill.i = alloca i8, align 1
  %1 = alloca i8, align 1
  %current_tail.dbg.spill = alloca i32, align 4
  %val.dbg.spill = alloca i8, align 1
  %self.dbg.spill = alloca %"heapless::spsc::Queue<u8, 2_usize>"*, align 4
  %_19 = alloca i8, align 1
  %_5 = alloca i8, align 1
  store %"heapless::spsc::Queue<u8, 2_usize>"* %self, %"heapless::spsc::Queue<u8, 2_usize>"** %self.dbg.spill, align 4
  call void @llvm.dbg.declare(metadata %"heapless::spsc::Queue<u8, 2_usize>"** %self.dbg.spill, metadata !1542, metadata !DIExpression()), !dbg !1546
  store i8 %val, i8* %val.dbg.spill, align 1
  call void @llvm.dbg.declare(metadata i8* %val.dbg.spill, metadata !1543, metadata !DIExpression()), !dbg !1547
  %_4 = getelementptr inbounds %"heapless::spsc::Queue<u8, 2_usize>", %"heapless::spsc::Queue<u8, 2_usize>"* %self, i32 0, i32 1, !dbg !1548
  store i8 0, i8* %_5, align 1, !dbg !1549
  %2 = load i8, i8* %_5, align 1, !dbg !1548, !range !1439
; call core::sync::atomic::AtomicUsize::load
  %current_tail = call i32 @_ZN4core4sync6atomic11AtomicUsize4load17hbff932a919f9f519E(%"core::sync::atomic::AtomicUsize"* align 4 dereferenceable(4) %_4, i8 %2) #10, !dbg !1548
  store i32 %current_tail, i32* %current_tail.dbg.spill, align 4, !dbg !1548
  call void @llvm.dbg.declare(metadata i32* %current_tail.dbg.spill, metadata !1544, metadata !DIExpression()), !dbg !1550
  br label %bb1, !dbg !1548

bb1:                                              ; preds = %start
  %_11 = getelementptr inbounds %"heapless::spsc::Queue<u8, 2_usize>", %"heapless::spsc::Queue<u8, 2_usize>"* %self, i32 0, i32 2, !dbg !1551
  %_10.0 = bitcast [2 x i8]* %_11 to [0 x i8]*, !dbg !1551
; call core::slice::<impl [T]>::get_unchecked
  %_9 = call align 1 dereferenceable(1) i8* @"_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$13get_unchecked17h92887aea7def6c01E"([0 x i8]* nonnull align 1 %_10.0, i32 2, i32 %current_tail) #10, !dbg !1551
  br label %bb2, !dbg !1551

bb2:                                              ; preds = %bb1
  store i8* %_9, i8** %self.dbg.spill.i2, align 4
  call void @llvm.dbg.declare(metadata i8** %self.dbg.spill.i2, metadata !1552, metadata !DIExpression()), !dbg !1559
  br label %bb3, !dbg !1561

bb3:                                              ; preds = %bb2
  store i8 %val, i8* %val.dbg.spill.i, align 1
  call void @llvm.dbg.declare(metadata i8* %val.dbg.spill.i, metadata !1562, metadata !DIExpression()), !dbg !1567
  store i8 %val, i8* %value.dbg.spill.i.i, align 1
  call void @llvm.dbg.declare(metadata i8* %value.dbg.spill.i.i, metadata !1569, metadata !DIExpression()), !dbg !1574
  store i8 %val, i8* %0, align 1, !dbg !1576
  %3 = load i8, i8* %0, align 1, !dbg !1577
  store i8 %3, i8* %1, align 1, !dbg !1578
  %4 = load i8, i8* %1, align 1, !dbg !1579
  br label %bb4, !dbg !1580

bb4:                                              ; preds = %bb3
  store i8* %_9, i8** %self.dbg.spill.i, align 4
  call void @llvm.dbg.declare(metadata i8** %self.dbg.spill.i, metadata !1581, metadata !DIExpression()) #10, !dbg !1587
  store i8 %4, i8* %val.dbg.spill.i1, align 1
  call void @llvm.dbg.declare(metadata i8* %val.dbg.spill.i1, metadata !1586, metadata !DIExpression()) #10, !dbg !1589
; call core::ptr::write
  call void @_ZN4core3ptr5write17h61a4480ba65cfdcfE(i8* %_9, i8 %4) #10, !dbg !1590
  br label %bb5, !dbg !1561

bb5:                                              ; preds = %bb4
  %_16 = getelementptr inbounds %"heapless::spsc::Queue<u8, 2_usize>", %"heapless::spsc::Queue<u8, 2_usize>"* %self, i32 0, i32 1, !dbg !1591
; call heapless::spsc::Queue<T,_>::increment
  %_17 = call i32 @"_ZN8heapless4spsc18Queue$LT$T$C$_$GT$9increment17h62b7694f13bd5c4aE"(i32 %current_tail) #10, !dbg !1592
  br label %bb6, !dbg !1592

bb6:                                              ; preds = %bb5
  store i8 1, i8* %_19, align 1, !dbg !1593
  %5 = load i8, i8* %_19, align 1, !dbg !1591, !range !1439
; call core::sync::atomic::AtomicUsize::store
  call void @_ZN4core4sync6atomic11AtomicUsize5store17h7e08c04f0f3a30f3E(%"core::sync::atomic::AtomicUsize"* align 4 dereferenceable(4) %_16, i32 %_17, i8 %5) #10, !dbg !1591
  br label %bb7, !dbg !1591

bb7:                                              ; preds = %bb6
  ret void, !dbg !1594
}

; heapless::spsc::Queue<T,_>::inner_enqueue_unchecked
; Function Attrs: nounwind
define internal void @"_ZN8heapless4spsc18Queue$LT$T$C$_$GT$23inner_enqueue_unchecked17hc697da99f6577d56E"(%"heapless::spsc::Queue<(app::P1_T, u8), 3_usize>"* align 4 dereferenceable(16) %self, i1 zeroext %val.0, i8 %val.1) unnamed_addr #0 !dbg !1595 {
start:
  %self.dbg.spill.i2 = alloca { i8, i8 }*, align 4
  %val.dbg.spill.i1 = alloca { i8, i8 }, align 1
  %self.dbg.spill.i = alloca { i8, i8 }*, align 4
  %value.dbg.spill.i.i = alloca { i8, i8 }, align 1
  %0 = alloca { i8, i8 }, align 1
  %val.dbg.spill.i = alloca { i8, i8 }, align 1
  %1 = alloca { i8, i8 }, align 1
  %current_tail.dbg.spill = alloca i32, align 4
  %val.dbg.spill = alloca { i8, i8 }, align 1
  %self.dbg.spill = alloca %"heapless::spsc::Queue<(app::P1_T, u8), 3_usize>"*, align 4
  %_19 = alloca i8, align 1
  %_5 = alloca i8, align 1
  store %"heapless::spsc::Queue<(app::P1_T, u8), 3_usize>"* %self, %"heapless::spsc::Queue<(app::P1_T, u8), 3_usize>"** %self.dbg.spill, align 4
  call void @llvm.dbg.declare(metadata %"heapless::spsc::Queue<(app::P1_T, u8), 3_usize>"** %self.dbg.spill, metadata !1599, metadata !DIExpression()), !dbg !1603
  %2 = getelementptr inbounds { i8, i8 }, { i8, i8 }* %val.dbg.spill, i32 0, i32 0
  %3 = zext i1 %val.0 to i8
  store i8 %3, i8* %2, align 1
  %4 = getelementptr inbounds { i8, i8 }, { i8, i8 }* %val.dbg.spill, i32 0, i32 1
  store i8 %val.1, i8* %4, align 1
  call void @llvm.dbg.declare(metadata { i8, i8 }* %val.dbg.spill, metadata !1600, metadata !DIExpression()), !dbg !1604
  %_4 = getelementptr inbounds %"heapless::spsc::Queue<(app::P1_T, u8), 3_usize>", %"heapless::spsc::Queue<(app::P1_T, u8), 3_usize>"* %self, i32 0, i32 1, !dbg !1605
  store i8 0, i8* %_5, align 1, !dbg !1606
  %5 = load i8, i8* %_5, align 1, !dbg !1605, !range !1439
; call core::sync::atomic::AtomicUsize::load
  %current_tail = call i32 @_ZN4core4sync6atomic11AtomicUsize4load17hbff932a919f9f519E(%"core::sync::atomic::AtomicUsize"* align 4 dereferenceable(4) %_4, i8 %5) #10, !dbg !1605
  store i32 %current_tail, i32* %current_tail.dbg.spill, align 4, !dbg !1605
  call void @llvm.dbg.declare(metadata i32* %current_tail.dbg.spill, metadata !1601, metadata !DIExpression()), !dbg !1607
  br label %bb1, !dbg !1605

bb1:                                              ; preds = %start
  %_11 = getelementptr inbounds %"heapless::spsc::Queue<(app::P1_T, u8), 3_usize>", %"heapless::spsc::Queue<(app::P1_T, u8), 3_usize>"* %self, i32 0, i32 2, !dbg !1608
  %_10.0 = bitcast [3 x { i8, i8 }]* %_11 to [0 x { i8, i8 }]*, !dbg !1608
; call core::slice::<impl [T]>::get_unchecked
  %_9 = call align 1 dereferenceable(2) { i8, i8 }* @"_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$13get_unchecked17h990719cc23fef4feE"([0 x { i8, i8 }]* nonnull align 1 %_10.0, i32 3, i32 %current_tail) #10, !dbg !1608
  br label %bb2, !dbg !1608

bb2:                                              ; preds = %bb1
  store { i8, i8 }* %_9, { i8, i8 }** %self.dbg.spill.i2, align 4
  call void @llvm.dbg.declare(metadata { i8, i8 }** %self.dbg.spill.i2, metadata !1609, metadata !DIExpression()), !dbg !1616
  br label %bb3, !dbg !1618

bb3:                                              ; preds = %bb2
  %6 = getelementptr inbounds { i8, i8 }, { i8, i8 }* %val.dbg.spill.i, i32 0, i32 0
  %7 = zext i1 %val.0 to i8
  store i8 %7, i8* %6, align 1
  %8 = getelementptr inbounds { i8, i8 }, { i8, i8 }* %val.dbg.spill.i, i32 0, i32 1
  store i8 %val.1, i8* %8, align 1
  call void @llvm.dbg.declare(metadata { i8, i8 }* %val.dbg.spill.i, metadata !1619, metadata !DIExpression()), !dbg !1624
  %9 = getelementptr inbounds { i8, i8 }, { i8, i8 }* %value.dbg.spill.i.i, i32 0, i32 0
  %10 = zext i1 %val.0 to i8
  store i8 %10, i8* %9, align 1
  %11 = getelementptr inbounds { i8, i8 }, { i8, i8 }* %value.dbg.spill.i.i, i32 0, i32 1
  store i8 %val.1, i8* %11, align 1
  call void @llvm.dbg.declare(metadata { i8, i8 }* %value.dbg.spill.i.i, metadata !1626, metadata !DIExpression()), !dbg !1631
  %12 = getelementptr inbounds { i8, i8 }, { i8, i8 }* %0, i32 0, i32 0, !dbg !1633
  %13 = zext i1 %val.0 to i8, !dbg !1633
  store i8 %13, i8* %12, align 1, !dbg !1633
  %14 = getelementptr inbounds { i8, i8 }, { i8, i8 }* %0, i32 0, i32 1, !dbg !1633
  store i8 %val.1, i8* %14, align 1, !dbg !1633
  %15 = getelementptr inbounds { i8, i8 }, { i8, i8 }* %0, i32 0, i32 0, !dbg !1634
  %16 = load i8, i8* %15, align 1, !dbg !1634, !range !436
  %17 = trunc i8 %16 to i1, !dbg !1634
  %18 = getelementptr inbounds { i8, i8 }, { i8, i8 }* %0, i32 0, i32 1, !dbg !1634
  %19 = load i8, i8* %18, align 1, !dbg !1634
  %20 = zext i1 %17 to i8, !dbg !1634
  %21 = insertvalue { i8, i8 } undef, i8 %20, 0, !dbg !1634
  %22 = insertvalue { i8, i8 } %21, i8 %19, 1, !dbg !1634
  %23 = getelementptr inbounds { i8, i8 }, { i8, i8 }* %1, i32 0, i32 0, !dbg !1635
  %24 = zext i1 %17 to i8, !dbg !1635
  store i8 %24, i8* %23, align 1, !dbg !1635
  %25 = getelementptr inbounds { i8, i8 }, { i8, i8 }* %1, i32 0, i32 1, !dbg !1635
  store i8 %19, i8* %25, align 1, !dbg !1635
  %26 = getelementptr inbounds { i8, i8 }, { i8, i8 }* %1, i32 0, i32 0, !dbg !1636
  %27 = load i8, i8* %26, align 1, !dbg !1636
  %28 = getelementptr inbounds { i8, i8 }, { i8, i8 }* %1, i32 0, i32 1, !dbg !1636
  %29 = load i8, i8* %28, align 1, !dbg !1636
  %30 = insertvalue { i8, i8 } undef, i8 %27, 0, !dbg !1636
  %31 = insertvalue { i8, i8 } %30, i8 %29, 1, !dbg !1636
  %_13.0 = extractvalue { i8, i8 } %31, 0, !dbg !1637
  %_13.1 = extractvalue { i8, i8 } %31, 1, !dbg !1637
  br label %bb4, !dbg !1637

bb4:                                              ; preds = %bb3
  store { i8, i8 }* %_9, { i8, i8 }** %self.dbg.spill.i, align 4
  call void @llvm.dbg.declare(metadata { i8, i8 }** %self.dbg.spill.i, metadata !1638, metadata !DIExpression()) #10, !dbg !1644
  %32 = getelementptr inbounds { i8, i8 }, { i8, i8 }* %val.dbg.spill.i1, i32 0, i32 0
  store i8 %_13.0, i8* %32, align 1
  %33 = getelementptr inbounds { i8, i8 }, { i8, i8 }* %val.dbg.spill.i1, i32 0, i32 1
  store i8 %_13.1, i8* %33, align 1
  call void @llvm.dbg.declare(metadata { i8, i8 }* %val.dbg.spill.i1, metadata !1643, metadata !DIExpression()) #10, !dbg !1646
; call core::ptr::write
  call void @_ZN4core3ptr5write17hcf998c1cb7ac1d77E({ i8, i8 }* %_9, i8 %_13.0, i8 %_13.1) #10, !dbg !1647
  br label %bb5, !dbg !1618

bb5:                                              ; preds = %bb4
  %_16 = getelementptr inbounds %"heapless::spsc::Queue<(app::P1_T, u8), 3_usize>", %"heapless::spsc::Queue<(app::P1_T, u8), 3_usize>"* %self, i32 0, i32 1, !dbg !1648
; call heapless::spsc::Queue<T,_>::increment
  %_17 = call i32 @"_ZN8heapless4spsc18Queue$LT$T$C$_$GT$9increment17h508ec96413021199E"(i32 %current_tail) #10, !dbg !1649
  br label %bb6, !dbg !1649

bb6:                                              ; preds = %bb5
  store i8 1, i8* %_19, align 1, !dbg !1650
  %34 = load i8, i8* %_19, align 1, !dbg !1648, !range !1439
; call core::sync::atomic::AtomicUsize::store
  call void @_ZN4core4sync6atomic11AtomicUsize5store17h7e08c04f0f3a30f3E(%"core::sync::atomic::AtomicUsize"* align 4 dereferenceable(4) %_16, i32 %_17, i8 %34) #10, !dbg !1648
  br label %bb7, !dbg !1648

bb7:                                              ; preds = %bb6
  ret void, !dbg !1651
}

; heapless::spsc::Queue<T,_>::enqueue_unchecked
; Function Attrs: nounwind
define internal void @"_ZN8heapless4spsc18Queue$LT$T$C$_$GT$17enqueue_unchecked17h37791079c683ec91E"(%"heapless::spsc::Queue<u8, 2_usize>"* align 4 dereferenceable(12) %self, i8 %val) unnamed_addr #0 !dbg !1652 {
start:
  %val.dbg.spill = alloca i8, align 1
  %self.dbg.spill = alloca %"heapless::spsc::Queue<u8, 2_usize>"*, align 4
  store %"heapless::spsc::Queue<u8, 2_usize>"* %self, %"heapless::spsc::Queue<u8, 2_usize>"** %self.dbg.spill, align 4
  call void @llvm.dbg.declare(metadata %"heapless::spsc::Queue<u8, 2_usize>"** %self.dbg.spill, metadata !1656, metadata !DIExpression()), !dbg !1658
  store i8 %val, i8* %val.dbg.spill, align 1
  call void @llvm.dbg.declare(metadata i8* %val.dbg.spill, metadata !1657, metadata !DIExpression()), !dbg !1659
; call heapless::spsc::Queue<T,_>::inner_enqueue_unchecked
  call void @"_ZN8heapless4spsc18Queue$LT$T$C$_$GT$23inner_enqueue_unchecked17h89c8e0b55bfe44b7E"(%"heapless::spsc::Queue<u8, 2_usize>"* align 4 dereferenceable(12) %self, i8 %val) #10, !dbg !1660
  br label %bb1, !dbg !1660

bb1:                                              ; preds = %start
  ret void, !dbg !1661
}

; heapless::spsc::Queue<T,_>::enqueue_unchecked
; Function Attrs: nounwind
define internal void @"_ZN8heapless4spsc18Queue$LT$T$C$_$GT$17enqueue_unchecked17ha419a43839953ab6E"(%"heapless::spsc::Queue<(app::P1_T, u8), 3_usize>"* align 4 dereferenceable(16) %self, i1 zeroext %val.0, i8 %val.1) unnamed_addr #0 !dbg !1662 {
start:
  %val.dbg.spill = alloca { i8, i8 }, align 1
  %self.dbg.spill = alloca %"heapless::spsc::Queue<(app::P1_T, u8), 3_usize>"*, align 4
  store %"heapless::spsc::Queue<(app::P1_T, u8), 3_usize>"* %self, %"heapless::spsc::Queue<(app::P1_T, u8), 3_usize>"** %self.dbg.spill, align 4
  call void @llvm.dbg.declare(metadata %"heapless::spsc::Queue<(app::P1_T, u8), 3_usize>"** %self.dbg.spill, metadata !1666, metadata !DIExpression()), !dbg !1668
  %0 = getelementptr inbounds { i8, i8 }, { i8, i8 }* %val.dbg.spill, i32 0, i32 0
  %1 = zext i1 %val.0 to i8
  store i8 %1, i8* %0, align 1
  %2 = getelementptr inbounds { i8, i8 }, { i8, i8 }* %val.dbg.spill, i32 0, i32 1
  store i8 %val.1, i8* %2, align 1
  call void @llvm.dbg.declare(metadata { i8, i8 }* %val.dbg.spill, metadata !1667, metadata !DIExpression()), !dbg !1669
; call heapless::spsc::Queue<T,_>::inner_enqueue_unchecked
  call void @"_ZN8heapless4spsc18Queue$LT$T$C$_$GT$23inner_enqueue_unchecked17hc697da99f6577d56E"(%"heapless::spsc::Queue<(app::P1_T, u8), 3_usize>"* align 4 dereferenceable(16) %self, i1 zeroext %val.0, i8 %val.1) #10, !dbg !1670
  br label %bb1, !dbg !1670

bb1:                                              ; preds = %start
  ret void, !dbg !1671
}

; heapless::spsc::Queue<T,_>::inner_dequeue
; Function Attrs: nounwind
define internal { i8, i8 } @"_ZN8heapless4spsc18Queue$LT$T$C$_$GT$13inner_dequeue17h69553b06f8d214e3E"(%"heapless::spsc::Queue<u8, 2_usize>"* align 4 dereferenceable(12) %self) unnamed_addr #0 !dbg !1672 {
start:
  %self.dbg.spill.i = alloca i8*, align 4
  %v.dbg.spill = alloca i8, align 1
  %current_head.dbg.spill = alloca i32, align 4
  %self.dbg.spill = alloca %"heapless::spsc::Queue<u8, 2_usize>"*, align 4
  %_22 = alloca i8, align 1
  %_9 = alloca i8, align 1
  %_4 = alloca i8, align 1
  %0 = alloca { i8, i8 }, align 1
  store %"heapless::spsc::Queue<u8, 2_usize>"* %self, %"heapless::spsc::Queue<u8, 2_usize>"** %self.dbg.spill, align 4
  call void @llvm.dbg.declare(metadata %"heapless::spsc::Queue<u8, 2_usize>"** %self.dbg.spill, metadata !1676, metadata !DIExpression()), !dbg !1681
  %_3 = bitcast %"heapless::spsc::Queue<u8, 2_usize>"* %self to %"core::sync::atomic::AtomicUsize"*, !dbg !1682
  store i8 0, i8* %_4, align 1, !dbg !1683
  %1 = load i8, i8* %_4, align 1, !dbg !1682, !range !1439
; call core::sync::atomic::AtomicUsize::load
  %current_head = call i32 @_ZN4core4sync6atomic11AtomicUsize4load17hbff932a919f9f519E(%"core::sync::atomic::AtomicUsize"* align 4 dereferenceable(4) %_3, i8 %1) #10, !dbg !1682
  store i32 %current_head, i32* %current_head.dbg.spill, align 4, !dbg !1682
  call void @llvm.dbg.declare(metadata i32* %current_head.dbg.spill, metadata !1677, metadata !DIExpression()), !dbg !1684
  br label %bb1, !dbg !1682

bb1:                                              ; preds = %start
  %_8 = getelementptr inbounds %"heapless::spsc::Queue<u8, 2_usize>", %"heapless::spsc::Queue<u8, 2_usize>"* %self, i32 0, i32 1, !dbg !1685
  store i8 2, i8* %_9, align 1, !dbg !1686
  %2 = load i8, i8* %_9, align 1, !dbg !1685, !range !1439
; call core::sync::atomic::AtomicUsize::load
  %_7 = call i32 @_ZN4core4sync6atomic11AtomicUsize4load17hbff932a919f9f519E(%"core::sync::atomic::AtomicUsize"* align 4 dereferenceable(4) %_8, i8 %2) #10, !dbg !1685
  br label %bb2, !dbg !1685

bb2:                                              ; preds = %bb1
  %_5 = icmp eq i32 %current_head, %_7, !dbg !1687
  br i1 %_5, label %bb3, label %bb4, !dbg !1687

bb4:                                              ; preds = %bb2
  %_16 = getelementptr inbounds %"heapless::spsc::Queue<u8, 2_usize>", %"heapless::spsc::Queue<u8, 2_usize>"* %self, i32 0, i32 2, !dbg !1688
  %_15.0 = bitcast [2 x i8]* %_16 to [0 x i8]*, !dbg !1688
; call core::slice::<impl [T]>::get_unchecked
  %_14 = call align 1 dereferenceable(1) i8* @"_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$13get_unchecked17h92887aea7def6c01E"([0 x i8]* nonnull align 1 %_15.0, i32 2, i32 %current_head) #10, !dbg !1688
  br label %bb5, !dbg !1688

bb3:                                              ; preds = %bb2
  %3 = bitcast { i8, i8 }* %0 to i8*, !dbg !1689
  store i8 0, i8* %3, align 1, !dbg !1689
  br label %bb10, !dbg !1690

bb10:                                             ; preds = %bb9, %bb3
  %4 = getelementptr inbounds { i8, i8 }, { i8, i8 }* %0, i32 0, i32 0, !dbg !1691
  %5 = load i8, i8* %4, align 1, !dbg !1691, !range !436
  %6 = trunc i8 %5 to i1, !dbg !1691
  %7 = getelementptr inbounds { i8, i8 }, { i8, i8 }* %0, i32 0, i32 1, !dbg !1691
  %8 = load i8, i8* %7, align 1, !dbg !1691
  %9 = zext i1 %6 to i8, !dbg !1691
  %10 = insertvalue { i8, i8 } undef, i8 %9, 0, !dbg !1691
  %11 = insertvalue { i8, i8 } %10, i8 %8, 1, !dbg !1691
  ret { i8, i8 } %11, !dbg !1691

bb5:                                              ; preds = %bb4
  store i8* %_14, i8** %self.dbg.spill.i, align 4
  call void @llvm.dbg.declare(metadata i8** %self.dbg.spill.i, metadata !1552, metadata !DIExpression()), !dbg !1692
  br label %bb6, !dbg !1688

bb6:                                              ; preds = %bb5
; call core::ptr::const_ptr::<impl *const T>::read
  %v = call i8 @"_ZN4core3ptr9const_ptr33_$LT$impl$u20$$BP$const$u20$T$GT$4read17heb62b597f26bdea0E"(i8* %_14) #10, !dbg !1694
  store i8 %v, i8* %v.dbg.spill, align 1, !dbg !1694
  call void @llvm.dbg.declare(metadata i8* %v.dbg.spill, metadata !1679, metadata !DIExpression()), !dbg !1695
  br label %bb7, !dbg !1694

bb7:                                              ; preds = %bb6
  %_19 = bitcast %"heapless::spsc::Queue<u8, 2_usize>"* %self to %"core::sync::atomic::AtomicUsize"*, !dbg !1696
; call heapless::spsc::Queue<T,_>::increment
  %_20 = call i32 @"_ZN8heapless4spsc18Queue$LT$T$C$_$GT$9increment17h62b7694f13bd5c4aE"(i32 %current_head) #10, !dbg !1697
  br label %bb8, !dbg !1697

bb8:                                              ; preds = %bb7
  store i8 1, i8* %_22, align 1, !dbg !1698
  %12 = load i8, i8* %_22, align 1, !dbg !1696, !range !1439
; call core::sync::atomic::AtomicUsize::store
  call void @_ZN4core4sync6atomic11AtomicUsize5store17h7e08c04f0f3a30f3E(%"core::sync::atomic::AtomicUsize"* align 4 dereferenceable(4) %_19, i32 %_20, i8 %12) #10, !dbg !1696
  br label %bb9, !dbg !1696

bb9:                                              ; preds = %bb8
  %13 = bitcast { i8, i8 }* %0 to %"core::option::Option<u8>::Some"*, !dbg !1699
  %14 = getelementptr inbounds %"core::option::Option<u8>::Some", %"core::option::Option<u8>::Some"* %13, i32 0, i32 1, !dbg !1699
  store i8 %v, i8* %14, align 1, !dbg !1699
  %15 = bitcast { i8, i8 }* %0 to i8*, !dbg !1699
  store i8 1, i8* %15, align 1, !dbg !1699
  br label %bb10, !dbg !1690
}

; heapless::spsc::Queue<T,_>::inner_dequeue
; Function Attrs: nounwind
define internal { i8, i8 } @"_ZN8heapless4spsc18Queue$LT$T$C$_$GT$13inner_dequeue17hac125db0968c02faE"(%"heapless::spsc::Queue<(app::P1_T, u8), 3_usize>"* align 4 dereferenceable(16) %self) unnamed_addr #0 !dbg !1700 {
start:
  %self.dbg.spill.i = alloca { i8, i8 }*, align 4
  %v.dbg.spill = alloca { i8, i8 }, align 1
  %current_head.dbg.spill = alloca i32, align 4
  %self.dbg.spill = alloca %"heapless::spsc::Queue<(app::P1_T, u8), 3_usize>"*, align 4
  %_22 = alloca i8, align 1
  %_9 = alloca i8, align 1
  %_4 = alloca i8, align 1
  %0 = alloca { i8, i8 }, align 1
  store %"heapless::spsc::Queue<(app::P1_T, u8), 3_usize>"* %self, %"heapless::spsc::Queue<(app::P1_T, u8), 3_usize>"** %self.dbg.spill, align 4
  call void @llvm.dbg.declare(metadata %"heapless::spsc::Queue<(app::P1_T, u8), 3_usize>"** %self.dbg.spill, metadata !1714, metadata !DIExpression()), !dbg !1719
  %_3 = bitcast %"heapless::spsc::Queue<(app::P1_T, u8), 3_usize>"* %self to %"core::sync::atomic::AtomicUsize"*, !dbg !1720
  store i8 0, i8* %_4, align 1, !dbg !1721
  %1 = load i8, i8* %_4, align 1, !dbg !1720, !range !1439
; call core::sync::atomic::AtomicUsize::load
  %current_head = call i32 @_ZN4core4sync6atomic11AtomicUsize4load17hbff932a919f9f519E(%"core::sync::atomic::AtomicUsize"* align 4 dereferenceable(4) %_3, i8 %1) #10, !dbg !1720
  store i32 %current_head, i32* %current_head.dbg.spill, align 4, !dbg !1720
  call void @llvm.dbg.declare(metadata i32* %current_head.dbg.spill, metadata !1715, metadata !DIExpression()), !dbg !1722
  br label %bb1, !dbg !1720

bb1:                                              ; preds = %start
  %_8 = getelementptr inbounds %"heapless::spsc::Queue<(app::P1_T, u8), 3_usize>", %"heapless::spsc::Queue<(app::P1_T, u8), 3_usize>"* %self, i32 0, i32 1, !dbg !1723
  store i8 2, i8* %_9, align 1, !dbg !1724
  %2 = load i8, i8* %_9, align 1, !dbg !1723, !range !1439
; call core::sync::atomic::AtomicUsize::load
  %_7 = call i32 @_ZN4core4sync6atomic11AtomicUsize4load17hbff932a919f9f519E(%"core::sync::atomic::AtomicUsize"* align 4 dereferenceable(4) %_8, i8 %2) #10, !dbg !1723
  br label %bb2, !dbg !1723

bb2:                                              ; preds = %bb1
  %_5 = icmp eq i32 %current_head, %_7, !dbg !1725
  br i1 %_5, label %bb3, label %bb4, !dbg !1725

bb4:                                              ; preds = %bb2
  %_16 = getelementptr inbounds %"heapless::spsc::Queue<(app::P1_T, u8), 3_usize>", %"heapless::spsc::Queue<(app::P1_T, u8), 3_usize>"* %self, i32 0, i32 2, !dbg !1726
  %_15.0 = bitcast [3 x { i8, i8 }]* %_16 to [0 x { i8, i8 }]*, !dbg !1726
; call core::slice::<impl [T]>::get_unchecked
  %_14 = call align 1 dereferenceable(2) { i8, i8 }* @"_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$13get_unchecked17h990719cc23fef4feE"([0 x { i8, i8 }]* nonnull align 1 %_15.0, i32 3, i32 %current_head) #10, !dbg !1726
  br label %bb5, !dbg !1726

bb3:                                              ; preds = %bb2
  %3 = bitcast { i8, i8 }* %0 to i8*, !dbg !1727
  call void @llvm.memset.p0i8.i32(i8* align 1 %3, i8 0, i32 2, i1 false), !dbg !1727
  %4 = bitcast { i8, i8 }* %0 to i8*, !dbg !1727
  store i8 2, i8* %4, align 1, !dbg !1727
  br label %bb10, !dbg !1728

bb10:                                             ; preds = %bb9, %bb3
  %5 = getelementptr inbounds { i8, i8 }, { i8, i8 }* %0, i32 0, i32 0, !dbg !1729
  %6 = load i8, i8* %5, align 1, !dbg !1729, !range !770
  %7 = getelementptr inbounds { i8, i8 }, { i8, i8 }* %0, i32 0, i32 1, !dbg !1729
  %8 = load i8, i8* %7, align 1, !dbg !1729
  %9 = insertvalue { i8, i8 } undef, i8 %6, 0, !dbg !1729
  %10 = insertvalue { i8, i8 } %9, i8 %8, 1, !dbg !1729
  ret { i8, i8 } %10, !dbg !1729

bb5:                                              ; preds = %bb4
  store { i8, i8 }* %_14, { i8, i8 }** %self.dbg.spill.i, align 4
  call void @llvm.dbg.declare(metadata { i8, i8 }** %self.dbg.spill.i, metadata !1609, metadata !DIExpression()), !dbg !1730
  br label %bb6, !dbg !1726

bb6:                                              ; preds = %bb5
; call core::ptr::const_ptr::<impl *const T>::read
  %11 = call { i8, i8 } @"_ZN4core3ptr9const_ptr33_$LT$impl$u20$$BP$const$u20$T$GT$4read17h5b4f761fdb64603eE"({ i8, i8 }* %_14) #10, !dbg !1732
  %12 = extractvalue { i8, i8 } %11, 0, !dbg !1732
  %v.0 = trunc i8 %12 to i1, !dbg !1732
  %v.1 = extractvalue { i8, i8 } %11, 1, !dbg !1732
  %13 = getelementptr inbounds { i8, i8 }, { i8, i8 }* %v.dbg.spill, i32 0, i32 0, !dbg !1732
  %14 = zext i1 %v.0 to i8, !dbg !1732
  store i8 %14, i8* %13, align 1, !dbg !1732
  %15 = getelementptr inbounds { i8, i8 }, { i8, i8 }* %v.dbg.spill, i32 0, i32 1, !dbg !1732
  store i8 %v.1, i8* %15, align 1, !dbg !1732
  call void @llvm.dbg.declare(metadata { i8, i8 }* %v.dbg.spill, metadata !1717, metadata !DIExpression()), !dbg !1733
  br label %bb7, !dbg !1732

bb7:                                              ; preds = %bb6
  %_19 = bitcast %"heapless::spsc::Queue<(app::P1_T, u8), 3_usize>"* %self to %"core::sync::atomic::AtomicUsize"*, !dbg !1734
; call heapless::spsc::Queue<T,_>::increment
  %_20 = call i32 @"_ZN8heapless4spsc18Queue$LT$T$C$_$GT$9increment17h508ec96413021199E"(i32 %current_head) #10, !dbg !1735
  br label %bb8, !dbg !1735

bb8:                                              ; preds = %bb7
  store i8 1, i8* %_22, align 1, !dbg !1736
  %16 = load i8, i8* %_22, align 1, !dbg !1734, !range !1439
; call core::sync::atomic::AtomicUsize::store
  call void @_ZN4core4sync6atomic11AtomicUsize5store17h7e08c04f0f3a30f3E(%"core::sync::atomic::AtomicUsize"* align 4 dereferenceable(4) %_19, i32 %_20, i8 %16) #10, !dbg !1734
  br label %bb9, !dbg !1734

bb9:                                              ; preds = %bb8
  %17 = getelementptr inbounds { i8, i8 }, { i8, i8 }* %0, i32 0, i32 0, !dbg !1737
  %18 = zext i1 %v.0 to i8, !dbg !1737
  store i8 %18, i8* %17, align 1, !dbg !1737
  %19 = getelementptr inbounds { i8, i8 }, { i8, i8 }* %0, i32 0, i32 1, !dbg !1737
  store i8 %v.1, i8* %19, align 1, !dbg !1737
  br label %bb10, !dbg !1728
}

; heapless::spsc::Queue<T,_>::split
; Function Attrs: nounwind
define internal { i32*, i32* } @"_ZN8heapless4spsc18Queue$LT$T$C$_$GT$5split17h4fd76e368a9e3d31E"(%"heapless::spsc::Queue<(app::P1_T, u8), 3_usize>"* align 4 dereferenceable(16) %self) unnamed_addr #0 !dbg !1738 {
start:
  %self.dbg.spill = alloca %"heapless::spsc::Queue<(app::P1_T, u8), 3_usize>"*, align 4
  %_4 = alloca i32*, align 4
  %_2 = alloca i32*, align 4
  %0 = alloca { i32*, i32* }, align 4
  store %"heapless::spsc::Queue<(app::P1_T, u8), 3_usize>"* %self, %"heapless::spsc::Queue<(app::P1_T, u8), 3_usize>"** %self.dbg.spill, align 4
  call void @llvm.dbg.declare(metadata %"heapless::spsc::Queue<(app::P1_T, u8), 3_usize>"** %self.dbg.spill, metadata !1752, metadata !DIExpression()), !dbg !1753
  %1 = bitcast i32** %_2 to %"heapless::spsc::Queue<(app::P1_T, u8), 3_usize>"**, !dbg !1754
  store %"heapless::spsc::Queue<(app::P1_T, u8), 3_usize>"* %self, %"heapless::spsc::Queue<(app::P1_T, u8), 3_usize>"** %1, align 4, !dbg !1754
  %2 = bitcast i32** %_4 to %"heapless::spsc::Queue<(app::P1_T, u8), 3_usize>"**, !dbg !1755
  store %"heapless::spsc::Queue<(app::P1_T, u8), 3_usize>"* %self, %"heapless::spsc::Queue<(app::P1_T, u8), 3_usize>"** %2, align 4, !dbg !1755
  %3 = bitcast { i32*, i32* }* %0 to i32**, !dbg !1756
  %4 = load i32*, i32** %_2, align 4, !dbg !1756, !nonnull !59
  store i32* %4, i32** %3, align 4, !dbg !1756
  %5 = getelementptr inbounds { i32*, i32* }, { i32*, i32* }* %0, i32 0, i32 1, !dbg !1756
  %6 = load i32*, i32** %_4, align 4, !dbg !1756, !nonnull !59
  store i32* %6, i32** %5, align 4, !dbg !1756
  %7 = getelementptr inbounds { i32*, i32* }, { i32*, i32* }* %0, i32 0, i32 0, !dbg !1757
  %8 = load i32*, i32** %7, align 4, !dbg !1757, !nonnull !59
  %9 = getelementptr inbounds { i32*, i32* }, { i32*, i32* }* %0, i32 0, i32 1, !dbg !1757
  %10 = load i32*, i32** %9, align 4, !dbg !1757, !nonnull !59
  %11 = insertvalue { i32*, i32* } undef, i32* %8, 0, !dbg !1757
  %12 = insertvalue { i32*, i32* } %11, i32* %10, 1, !dbg !1757
  ret { i32*, i32* } %12, !dbg !1757
}

; heapless::spsc::Queue<T,_>::split
; Function Attrs: nounwind
define internal { i32*, i32* } @"_ZN8heapless4spsc18Queue$LT$T$C$_$GT$5split17h5bcea21f35589d84E"(%"heapless::spsc::Queue<u8, 2_usize>"* align 4 dereferenceable(12) %self) unnamed_addr #0 !dbg !1758 {
start:
  %self.dbg.spill = alloca %"heapless::spsc::Queue<u8, 2_usize>"*, align 4
  %_4 = alloca i32*, align 4
  %_2 = alloca i32*, align 4
  %0 = alloca { i32*, i32* }, align 4
  store %"heapless::spsc::Queue<u8, 2_usize>"* %self, %"heapless::spsc::Queue<u8, 2_usize>"** %self.dbg.spill, align 4
  call void @llvm.dbg.declare(metadata %"heapless::spsc::Queue<u8, 2_usize>"** %self.dbg.spill, metadata !1772, metadata !DIExpression()), !dbg !1773
  %1 = bitcast i32** %_2 to %"heapless::spsc::Queue<u8, 2_usize>"**, !dbg !1774
  store %"heapless::spsc::Queue<u8, 2_usize>"* %self, %"heapless::spsc::Queue<u8, 2_usize>"** %1, align 4, !dbg !1774
  %2 = bitcast i32** %_4 to %"heapless::spsc::Queue<u8, 2_usize>"**, !dbg !1775
  store %"heapless::spsc::Queue<u8, 2_usize>"* %self, %"heapless::spsc::Queue<u8, 2_usize>"** %2, align 4, !dbg !1775
  %3 = bitcast { i32*, i32* }* %0 to i32**, !dbg !1776
  %4 = load i32*, i32** %_2, align 4, !dbg !1776, !nonnull !59
  store i32* %4, i32** %3, align 4, !dbg !1776
  %5 = getelementptr inbounds { i32*, i32* }, { i32*, i32* }* %0, i32 0, i32 1, !dbg !1776
  %6 = load i32*, i32** %_4, align 4, !dbg !1776, !nonnull !59
  store i32* %6, i32** %5, align 4, !dbg !1776
  %7 = getelementptr inbounds { i32*, i32* }, { i32*, i32* }* %0, i32 0, i32 0, !dbg !1777
  %8 = load i32*, i32** %7, align 4, !dbg !1777, !nonnull !59
  %9 = getelementptr inbounds { i32*, i32* }, { i32*, i32* }* %0, i32 0, i32 1, !dbg !1777
  %10 = load i32*, i32** %9, align 4, !dbg !1777, !nonnull !59
  %11 = insertvalue { i32*, i32* } undef, i32* %8, 0, !dbg !1777
  %12 = insertvalue { i32*, i32* } %11, i32* %10, 1, !dbg !1777
  ret { i32*, i32* } %12, !dbg !1777
}

; <heapless::spsc::IterMut<T,_> as core::iter::traits::iterator::Iterator>::next
; Function Attrs: nounwind
define internal align 1 dereferenceable_or_null(2) i8* @"_ZN95_$LT$heapless..spsc..IterMut$LT$T$C$_$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h4d3320e7f4c35bbcE"(%"heapless::spsc::IterMut<(app::P1_T, u8), 3_usize>"* align 4 dereferenceable(12) %self) unnamed_addr #0 !dbg !1778 {
start:
  %self.dbg.spill.i = alloca { i8, i8 }*, align 4
  %i.dbg.spill = alloca i32, align 4
  %head.dbg.spill = alloca i32, align 4
  %self.dbg.spill = alloca %"heapless::spsc::IterMut<(app::P1_T, u8), 3_usize>"*, align 4
  %_7 = alloca i8, align 1
  %0 = alloca i8*, align 4
  store %"heapless::spsc::IterMut<(app::P1_T, u8), 3_usize>"* %self, %"heapless::spsc::IterMut<(app::P1_T, u8), 3_usize>"** %self.dbg.spill, align 4
  call void @llvm.dbg.declare(metadata %"heapless::spsc::IterMut<(app::P1_T, u8), 3_usize>"** %self.dbg.spill, metadata !1798, metadata !DIExpression()), !dbg !1803
  %1 = getelementptr inbounds %"heapless::spsc::IterMut<(app::P1_T, u8), 3_usize>", %"heapless::spsc::IterMut<(app::P1_T, u8), 3_usize>"* %self, i32 0, i32 1, !dbg !1804
  %_3 = load i32, i32* %1, align 4, !dbg !1804
  %2 = getelementptr inbounds %"heapless::spsc::IterMut<(app::P1_T, u8), 3_usize>", %"heapless::spsc::IterMut<(app::P1_T, u8), 3_usize>"* %self, i32 0, i32 2, !dbg !1805
  %_4 = load i32, i32* %2, align 4, !dbg !1805
  %_2 = icmp ult i32 %_3, %_4, !dbg !1804
  br i1 %_2, label %bb1, label %bb8, !dbg !1804

bb8:                                              ; preds = %start
  %3 = bitcast i8** %0 to i8*, !dbg !1806
  call void @llvm.memset.p0i8.i32(i8* align 4 %3, i8 0, i32 4, i1 false), !dbg !1806
  %4 = bitcast i8** %0 to {}**, !dbg !1806
  store {}* null, {}** %4, align 4, !dbg !1806
  br label %bb9, !dbg !1807

bb1:                                              ; preds = %start
  %5 = bitcast %"heapless::spsc::IterMut<(app::P1_T, u8), 3_usize>"* %self to %"heapless::spsc::Queue<(app::P1_T, u8), 3_usize>"**, !dbg !1808
  %6 = load %"heapless::spsc::Queue<(app::P1_T, u8), 3_usize>"*, %"heapless::spsc::Queue<(app::P1_T, u8), 3_usize>"** %5, align 4, !dbg !1808, !nonnull !59
  %_6 = bitcast %"heapless::spsc::Queue<(app::P1_T, u8), 3_usize>"* %6 to %"core::sync::atomic::AtomicUsize"*, !dbg !1808
  store i8 0, i8* %_7, align 1, !dbg !1809
  %7 = load i8, i8* %_7, align 1, !dbg !1808, !range !1439
; call core::sync::atomic::AtomicUsize::load
  %head = call i32 @_ZN4core4sync6atomic11AtomicUsize4load17hbff932a919f9f519E(%"core::sync::atomic::AtomicUsize"* align 4 dereferenceable(4) %_6, i8 %7) #10, !dbg !1808
  store i32 %head, i32* %head.dbg.spill, align 4, !dbg !1808
  call void @llvm.dbg.declare(metadata i32* %head.dbg.spill, metadata !1799, metadata !DIExpression()), !dbg !1810
  br label %bb2, !dbg !1808

bb2:                                              ; preds = %bb1
  %8 = getelementptr inbounds %"heapless::spsc::IterMut<(app::P1_T, u8), 3_usize>", %"heapless::spsc::IterMut<(app::P1_T, u8), 3_usize>"* %self, i32 0, i32 1, !dbg !1811
  %_11 = load i32, i32* %8, align 4, !dbg !1811
  %9 = call { i32, i1 } @llvm.uadd.with.overflow.i32(i32 %head, i32 %_11), !dbg !1812
  %_12.0 = extractvalue { i32, i1 } %9, 0, !dbg !1812
  %_12.1 = extractvalue { i32, i1 } %9, 1, !dbg !1812
  %10 = call i1 @llvm.expect.i1(i1 %_12.1, i1 false), !dbg !1812
  br i1 %10, label %panic, label %bb3, !dbg !1812

bb3:                                              ; preds = %bb2
  br label %bb4, !dbg !1812

panic:                                            ; preds = %bb2
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17h9e72682d8e5e8cf1E([0 x i8]* nonnull align 1 bitcast ([28 x i8]* @str.0 to [0 x i8]*), i32 28, %"core::panic::location::Location"* align 4 dereferenceable(16) bitcast (<{ i8*, [12 x i8] }>* @alloc201 to %"core::panic::location::Location"*)) #11, !dbg !1812
  unreachable, !dbg !1812

bb4:                                              ; preds = %bb3
  %i = urem i32 %_12.0, 3, !dbg !1812
  store i32 %i, i32* %i.dbg.spill, align 4, !dbg !1812
  call void @llvm.dbg.declare(metadata i32* %i.dbg.spill, metadata !1801, metadata !DIExpression()), !dbg !1813
  %11 = getelementptr inbounds %"heapless::spsc::IterMut<(app::P1_T, u8), 3_usize>", %"heapless::spsc::IterMut<(app::P1_T, u8), 3_usize>"* %self, i32 0, i32 1, !dbg !1814
  %12 = load i32, i32* %11, align 4, !dbg !1814
  %13 = call { i32, i1 } @llvm.uadd.with.overflow.i32(i32 %12, i32 1), !dbg !1814
  %_14.0 = extractvalue { i32, i1 } %13, 0, !dbg !1814
  %_14.1 = extractvalue { i32, i1 } %13, 1, !dbg !1814
  %14 = call i1 @llvm.expect.i1(i1 %_14.1, i1 false), !dbg !1814
  br i1 %14, label %panic1, label %bb5, !dbg !1814

bb5:                                              ; preds = %bb4
  %15 = getelementptr inbounds %"heapless::spsc::IterMut<(app::P1_T, u8), 3_usize>", %"heapless::spsc::IterMut<(app::P1_T, u8), 3_usize>"* %self, i32 0, i32 1, !dbg !1814
  store i32 %_14.0, i32* %15, align 4, !dbg !1814
  %16 = bitcast %"heapless::spsc::IterMut<(app::P1_T, u8), 3_usize>"* %self to %"heapless::spsc::Queue<(app::P1_T, u8), 3_usize>"**, !dbg !1815
  %17 = load %"heapless::spsc::Queue<(app::P1_T, u8), 3_usize>"*, %"heapless::spsc::Queue<(app::P1_T, u8), 3_usize>"** %16, align 4, !dbg !1815, !nonnull !59
  %_23 = getelementptr inbounds %"heapless::spsc::Queue<(app::P1_T, u8), 3_usize>", %"heapless::spsc::Queue<(app::P1_T, u8), 3_usize>"* %17, i32 0, i32 2, !dbg !1815
  %_22.0 = bitcast [3 x { i8, i8 }]* %_23 to [0 x { i8, i8 }]*, !dbg !1815
; call core::slice::<impl [T]>::get_unchecked
  %_21 = call align 1 dereferenceable(2) { i8, i8 }* @"_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$13get_unchecked17h990719cc23fef4feE"([0 x { i8, i8 }]* nonnull align 1 %_22.0, i32 3, i32 %i) #10, !dbg !1815
  br label %bb6, !dbg !1815

panic1:                                           ; preds = %bb4
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17h9e72682d8e5e8cf1E([0 x i8]* nonnull align 1 bitcast ([28 x i8]* @str.0 to [0 x i8]*), i32 28, %"core::panic::location::Location"* align 4 dereferenceable(16) bitcast (<{ i8*, [12 x i8] }>* @alloc203 to %"core::panic::location::Location"*)) #11, !dbg !1814
  unreachable, !dbg !1814

bb6:                                              ; preds = %bb5
  store { i8, i8 }* %_21, { i8, i8 }** %self.dbg.spill.i, align 4
  call void @llvm.dbg.declare(metadata { i8, i8 }** %self.dbg.spill.i, metadata !1609, metadata !DIExpression()), !dbg !1816
  br label %bb7, !dbg !1815

bb7:                                              ; preds = %bb6
  %18 = bitcast i8** %0 to { i8, i8 }**, !dbg !1818
  store { i8, i8 }* %_21, { i8, i8 }** %18, align 4, !dbg !1818
  br label %bb9, !dbg !1807

bb9:                                              ; preds = %bb8, %bb7
  %19 = load i8*, i8** %0, align 4, !dbg !1819
  ret i8* %19, !dbg !1819
}

; <heapless::spsc::IterMut<T,_> as core::iter::traits::iterator::Iterator>::next
; Function Attrs: nounwind
define internal align 1 dereferenceable_or_null(1) i8* @"_ZN95_$LT$heapless..spsc..IterMut$LT$T$C$_$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h72004cbeb71d9325E"(%"heapless::spsc::IterMut<u8, 2_usize>"* align 4 dereferenceable(12) %self) unnamed_addr #0 !dbg !1820 {
start:
  %self.dbg.spill.i = alloca i8*, align 4
  %i.dbg.spill = alloca i32, align 4
  %head.dbg.spill = alloca i32, align 4
  %self.dbg.spill = alloca %"heapless::spsc::IterMut<u8, 2_usize>"*, align 4
  %_7 = alloca i8, align 1
  %0 = alloca i8*, align 4
  store %"heapless::spsc::IterMut<u8, 2_usize>"* %self, %"heapless::spsc::IterMut<u8, 2_usize>"** %self.dbg.spill, align 4
  call void @llvm.dbg.declare(metadata %"heapless::spsc::IterMut<u8, 2_usize>"** %self.dbg.spill, metadata !1837, metadata !DIExpression()), !dbg !1842
  %1 = getelementptr inbounds %"heapless::spsc::IterMut<u8, 2_usize>", %"heapless::spsc::IterMut<u8, 2_usize>"* %self, i32 0, i32 1, !dbg !1843
  %_3 = load i32, i32* %1, align 4, !dbg !1843
  %2 = getelementptr inbounds %"heapless::spsc::IterMut<u8, 2_usize>", %"heapless::spsc::IterMut<u8, 2_usize>"* %self, i32 0, i32 2, !dbg !1844
  %_4 = load i32, i32* %2, align 4, !dbg !1844
  %_2 = icmp ult i32 %_3, %_4, !dbg !1843
  br i1 %_2, label %bb1, label %bb8, !dbg !1843

bb8:                                              ; preds = %start
  %3 = bitcast i8** %0 to i8*, !dbg !1845
  call void @llvm.memset.p0i8.i32(i8* align 4 %3, i8 0, i32 4, i1 false), !dbg !1845
  %4 = bitcast i8** %0 to {}**, !dbg !1845
  store {}* null, {}** %4, align 4, !dbg !1845
  br label %bb9, !dbg !1846

bb1:                                              ; preds = %start
  %5 = bitcast %"heapless::spsc::IterMut<u8, 2_usize>"* %self to %"heapless::spsc::Queue<u8, 2_usize>"**, !dbg !1847
  %6 = load %"heapless::spsc::Queue<u8, 2_usize>"*, %"heapless::spsc::Queue<u8, 2_usize>"** %5, align 4, !dbg !1847, !nonnull !59
  %_6 = bitcast %"heapless::spsc::Queue<u8, 2_usize>"* %6 to %"core::sync::atomic::AtomicUsize"*, !dbg !1847
  store i8 0, i8* %_7, align 1, !dbg !1848
  %7 = load i8, i8* %_7, align 1, !dbg !1847, !range !1439
; call core::sync::atomic::AtomicUsize::load
  %head = call i32 @_ZN4core4sync6atomic11AtomicUsize4load17hbff932a919f9f519E(%"core::sync::atomic::AtomicUsize"* align 4 dereferenceable(4) %_6, i8 %7) #10, !dbg !1847
  store i32 %head, i32* %head.dbg.spill, align 4, !dbg !1847
  call void @llvm.dbg.declare(metadata i32* %head.dbg.spill, metadata !1838, metadata !DIExpression()), !dbg !1849
  br label %bb2, !dbg !1847

bb2:                                              ; preds = %bb1
  %8 = getelementptr inbounds %"heapless::spsc::IterMut<u8, 2_usize>", %"heapless::spsc::IterMut<u8, 2_usize>"* %self, i32 0, i32 1, !dbg !1850
  %_11 = load i32, i32* %8, align 4, !dbg !1850
  %9 = call { i32, i1 } @llvm.uadd.with.overflow.i32(i32 %head, i32 %_11), !dbg !1851
  %_12.0 = extractvalue { i32, i1 } %9, 0, !dbg !1851
  %_12.1 = extractvalue { i32, i1 } %9, 1, !dbg !1851
  %10 = call i1 @llvm.expect.i1(i1 %_12.1, i1 false), !dbg !1851
  br i1 %10, label %panic, label %bb3, !dbg !1851

bb3:                                              ; preds = %bb2
  br label %bb4, !dbg !1851

panic:                                            ; preds = %bb2
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17h9e72682d8e5e8cf1E([0 x i8]* nonnull align 1 bitcast ([28 x i8]* @str.0 to [0 x i8]*), i32 28, %"core::panic::location::Location"* align 4 dereferenceable(16) bitcast (<{ i8*, [12 x i8] }>* @alloc201 to %"core::panic::location::Location"*)) #11, !dbg !1851
  unreachable, !dbg !1851

bb4:                                              ; preds = %bb3
  %i = urem i32 %_12.0, 2, !dbg !1851
  store i32 %i, i32* %i.dbg.spill, align 4, !dbg !1851
  call void @llvm.dbg.declare(metadata i32* %i.dbg.spill, metadata !1840, metadata !DIExpression()), !dbg !1852
  %11 = getelementptr inbounds %"heapless::spsc::IterMut<u8, 2_usize>", %"heapless::spsc::IterMut<u8, 2_usize>"* %self, i32 0, i32 1, !dbg !1853
  %12 = load i32, i32* %11, align 4, !dbg !1853
  %13 = call { i32, i1 } @llvm.uadd.with.overflow.i32(i32 %12, i32 1), !dbg !1853
  %_14.0 = extractvalue { i32, i1 } %13, 0, !dbg !1853
  %_14.1 = extractvalue { i32, i1 } %13, 1, !dbg !1853
  %14 = call i1 @llvm.expect.i1(i1 %_14.1, i1 false), !dbg !1853
  br i1 %14, label %panic1, label %bb5, !dbg !1853

bb5:                                              ; preds = %bb4
  %15 = getelementptr inbounds %"heapless::spsc::IterMut<u8, 2_usize>", %"heapless::spsc::IterMut<u8, 2_usize>"* %self, i32 0, i32 1, !dbg !1853
  store i32 %_14.0, i32* %15, align 4, !dbg !1853
  %16 = bitcast %"heapless::spsc::IterMut<u8, 2_usize>"* %self to %"heapless::spsc::Queue<u8, 2_usize>"**, !dbg !1854
  %17 = load %"heapless::spsc::Queue<u8, 2_usize>"*, %"heapless::spsc::Queue<u8, 2_usize>"** %16, align 4, !dbg !1854, !nonnull !59
  %_23 = getelementptr inbounds %"heapless::spsc::Queue<u8, 2_usize>", %"heapless::spsc::Queue<u8, 2_usize>"* %17, i32 0, i32 2, !dbg !1854
  %_22.0 = bitcast [2 x i8]* %_23 to [0 x i8]*, !dbg !1854
; call core::slice::<impl [T]>::get_unchecked
  %_21 = call align 1 dereferenceable(1) i8* @"_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$13get_unchecked17h92887aea7def6c01E"([0 x i8]* nonnull align 1 %_22.0, i32 2, i32 %i) #10, !dbg !1854
  br label %bb6, !dbg !1854

panic1:                                           ; preds = %bb4
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17h9e72682d8e5e8cf1E([0 x i8]* nonnull align 1 bitcast ([28 x i8]* @str.0 to [0 x i8]*), i32 28, %"core::panic::location::Location"* align 4 dereferenceable(16) bitcast (<{ i8*, [12 x i8] }>* @alloc203 to %"core::panic::location::Location"*)) #11, !dbg !1853
  unreachable, !dbg !1853

bb6:                                              ; preds = %bb5
  store i8* %_21, i8** %self.dbg.spill.i, align 4
  call void @llvm.dbg.declare(metadata i8** %self.dbg.spill.i, metadata !1552, metadata !DIExpression()), !dbg !1855
  br label %bb7, !dbg !1854

bb7:                                              ; preds = %bb6
  store i8* %_21, i8** %0, align 4, !dbg !1857
  br label %bb9, !dbg !1846

bb9:                                              ; preds = %bb8, %bb7
  %18 = load i8*, i8** %0, align 4, !dbg !1858
  ret i8* %18, !dbg !1858
}

; <heapless::spsc::Queue<T,_> as core::ops::drop::Drop>::drop
; Function Attrs: nounwind
define internal void @"_ZN76_$LT$heapless..spsc..Queue$LT$T$C$_$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h9ad36f3161ed8116E"(%"heapless::spsc::Queue<u8, 2_usize>"* align 4 dereferenceable(12) %self) unnamed_addr #0 !dbg !1859 {
start:
  %item.dbg.spill = alloca i8*, align 4
  %self.dbg.spill = alloca %"heapless::spsc::Queue<u8, 2_usize>"*, align 4
  %_5 = alloca i8*, align 4
  %iter = alloca %"heapless::spsc::IterMut<u8, 2_usize>", align 4
  %_2 = alloca %"heapless::spsc::IterMut<u8, 2_usize>", align 4
  store %"heapless::spsc::Queue<u8, 2_usize>"* %self, %"heapless::spsc::Queue<u8, 2_usize>"** %self.dbg.spill, align 4
  call void @llvm.dbg.declare(metadata %"heapless::spsc::Queue<u8, 2_usize>"** %self.dbg.spill, metadata !1864, metadata !DIExpression()), !dbg !1869
  call void @llvm.dbg.declare(metadata %"heapless::spsc::IterMut<u8, 2_usize>"* %iter, metadata !1865, metadata !DIExpression()), !dbg !1870
; call <&mut heapless::spsc::Queue<T,_> as core::iter::traits::collect::IntoIterator>::into_iter
  call void @"_ZN108_$LT$$RF$mut$u20$heapless..spsc..Queue$LT$T$C$_$GT$$u20$as$u20$core..iter..traits..collect..IntoIterator$GT$9into_iter17h73e2329ae13ae5ccE"(%"heapless::spsc::IterMut<u8, 2_usize>"* noalias nocapture sret(%"heapless::spsc::IterMut<u8, 2_usize>") dereferenceable(12) %_2, %"heapless::spsc::Queue<u8, 2_usize>"* align 4 dereferenceable(12) %self) #10, !dbg !1871
  br label %bb1, !dbg !1871

bb1:                                              ; preds = %start
  %0 = bitcast %"heapless::spsc::IterMut<u8, 2_usize>"* %iter to i8*, !dbg !1871
  %1 = bitcast %"heapless::spsc::IterMut<u8, 2_usize>"* %_2 to i8*, !dbg !1871
  call void @llvm.memcpy.p0i8.p0i8.i32(i8* align 4 %0, i8* align 4 %1, i32 12, i1 false), !dbg !1871
  br label %bb2, !dbg !1872

bb2:                                              ; preds = %bb4, %bb1
; call <heapless::spsc::IterMut<T,_> as core::iter::traits::iterator::Iterator>::next
  %2 = call align 1 dereferenceable_or_null(1) i8* @"_ZN95_$LT$heapless..spsc..IterMut$LT$T$C$_$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h72004cbeb71d9325E"(%"heapless::spsc::IterMut<u8, 2_usize>"* align 4 dereferenceable(12) %iter) #10, !dbg !1870
  store i8* %2, i8** %_5, align 4, !dbg !1870
  br label %bb3, !dbg !1870

bb3:                                              ; preds = %bb2
  %3 = bitcast i8** %_5 to {}**, !dbg !1870
  %4 = load {}*, {}** %3, align 4, !dbg !1870
  %5 = icmp eq {}* %4, null, !dbg !1870
  %_8 = select i1 %5, i32 0, i32 1, !dbg !1870
  switch i32 %_8, label %bb5 [
    i32 0, label %bb6
    i32 1, label %bb4
  ], !dbg !1870

bb5:                                              ; preds = %bb3
  unreachable, !dbg !1870

bb6:                                              ; preds = %bb3
  ret void, !dbg !1873

bb4:                                              ; preds = %bb3
  %item = load i8*, i8** %_5, align 4, !dbg !1874, !nonnull !59
  store i8* %item, i8** %item.dbg.spill, align 4, !dbg !1874
  call void @llvm.dbg.declare(metadata i8** %item.dbg.spill, metadata !1867, metadata !DIExpression()), !dbg !1875
  br label %bb2, !dbg !1876
}

; <heapless::spsc::Queue<T,_> as core::ops::drop::Drop>::drop
; Function Attrs: nounwind
define internal void @"_ZN76_$LT$heapless..spsc..Queue$LT$T$C$_$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17hde889c84868322b6E"(%"heapless::spsc::Queue<(app::P1_T, u8), 3_usize>"* align 4 dereferenceable(16) %self) unnamed_addr #0 !dbg !1877 {
start:
  %item.dbg.spill = alloca { i8, i8 }*, align 4
  %self.dbg.spill = alloca %"heapless::spsc::Queue<(app::P1_T, u8), 3_usize>"*, align 4
  %_5 = alloca i8*, align 4
  %iter = alloca %"heapless::spsc::IterMut<(app::P1_T, u8), 3_usize>", align 4
  %_2 = alloca %"heapless::spsc::IterMut<(app::P1_T, u8), 3_usize>", align 4
  store %"heapless::spsc::Queue<(app::P1_T, u8), 3_usize>"* %self, %"heapless::spsc::Queue<(app::P1_T, u8), 3_usize>"** %self.dbg.spill, align 4
  call void @llvm.dbg.declare(metadata %"heapless::spsc::Queue<(app::P1_T, u8), 3_usize>"** %self.dbg.spill, metadata !1881, metadata !DIExpression()), !dbg !1886
  call void @llvm.dbg.declare(metadata %"heapless::spsc::IterMut<(app::P1_T, u8), 3_usize>"* %iter, metadata !1882, metadata !DIExpression()), !dbg !1887
; call <&mut heapless::spsc::Queue<T,_> as core::iter::traits::collect::IntoIterator>::into_iter
  call void @"_ZN108_$LT$$RF$mut$u20$heapless..spsc..Queue$LT$T$C$_$GT$$u20$as$u20$core..iter..traits..collect..IntoIterator$GT$9into_iter17h3c0ea4b7b381ec52E"(%"heapless::spsc::IterMut<(app::P1_T, u8), 3_usize>"* noalias nocapture sret(%"heapless::spsc::IterMut<(app::P1_T, u8), 3_usize>") dereferenceable(12) %_2, %"heapless::spsc::Queue<(app::P1_T, u8), 3_usize>"* align 4 dereferenceable(16) %self) #10, !dbg !1888
  br label %bb1, !dbg !1888

bb1:                                              ; preds = %start
  %0 = bitcast %"heapless::spsc::IterMut<(app::P1_T, u8), 3_usize>"* %iter to i8*, !dbg !1888
  %1 = bitcast %"heapless::spsc::IterMut<(app::P1_T, u8), 3_usize>"* %_2 to i8*, !dbg !1888
  call void @llvm.memcpy.p0i8.p0i8.i32(i8* align 4 %0, i8* align 4 %1, i32 12, i1 false), !dbg !1888
  br label %bb2, !dbg !1889

bb2:                                              ; preds = %bb4, %bb1
; call <heapless::spsc::IterMut<T,_> as core::iter::traits::iterator::Iterator>::next
  %2 = call align 1 dereferenceable_or_null(2) i8* @"_ZN95_$LT$heapless..spsc..IterMut$LT$T$C$_$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h4d3320e7f4c35bbcE"(%"heapless::spsc::IterMut<(app::P1_T, u8), 3_usize>"* align 4 dereferenceable(12) %iter) #10, !dbg !1887
  store i8* %2, i8** %_5, align 4, !dbg !1887
  br label %bb3, !dbg !1887

bb3:                                              ; preds = %bb2
  %3 = bitcast i8** %_5 to {}**, !dbg !1887
  %4 = load {}*, {}** %3, align 4, !dbg !1887
  %5 = icmp eq {}* %4, null, !dbg !1887
  %_8 = select i1 %5, i32 0, i32 1, !dbg !1887
  switch i32 %_8, label %bb5 [
    i32 0, label %bb6
    i32 1, label %bb4
  ], !dbg !1887

bb5:                                              ; preds = %bb3
  unreachable, !dbg !1887

bb6:                                              ; preds = %bb3
  ret void, !dbg !1890

bb4:                                              ; preds = %bb3
  %6 = bitcast i8** %_5 to { i8, i8 }**, !dbg !1891
  %item = load { i8, i8 }*, { i8, i8 }** %6, align 4, !dbg !1891, !nonnull !59
  store { i8, i8 }* %item, { i8, i8 }** %item.dbg.spill, align 4, !dbg !1891
  call void @llvm.dbg.declare(metadata { i8, i8 }** %item.dbg.spill, metadata !1884, metadata !DIExpression()), !dbg !1892
  br label %bb2, !dbg !1893
}

; <&mut heapless::spsc::Queue<T,_> as core::iter::traits::collect::IntoIterator>::into_iter
; Function Attrs: nounwind
define internal void @"_ZN108_$LT$$RF$mut$u20$heapless..spsc..Queue$LT$T$C$_$GT$$u20$as$u20$core..iter..traits..collect..IntoIterator$GT$9into_iter17h3c0ea4b7b381ec52E"(%"heapless::spsc::IterMut<(app::P1_T, u8), 3_usize>"* noalias nocapture sret(%"heapless::spsc::IterMut<(app::P1_T, u8), 3_usize>") dereferenceable(12) %0, %"heapless::spsc::Queue<(app::P1_T, u8), 3_usize>"* align 4 dereferenceable(16) %self) unnamed_addr #0 !dbg !1894 {
start:
  %self.dbg.spill = alloca %"heapless::spsc::Queue<(app::P1_T, u8), 3_usize>"*, align 4
  store %"heapless::spsc::Queue<(app::P1_T, u8), 3_usize>"* %self, %"heapless::spsc::Queue<(app::P1_T, u8), 3_usize>"** %self.dbg.spill, align 4
  call void @llvm.dbg.declare(metadata %"heapless::spsc::Queue<(app::P1_T, u8), 3_usize>"** %self.dbg.spill, metadata !1897, metadata !DIExpression()), !dbg !1898
; call heapless::spsc::Queue<T,_>::iter_mut
  call void @"_ZN8heapless4spsc18Queue$LT$T$C$_$GT$8iter_mut17hf08aa4f1cb5ae041E"(%"heapless::spsc::IterMut<(app::P1_T, u8), 3_usize>"* noalias nocapture sret(%"heapless::spsc::IterMut<(app::P1_T, u8), 3_usize>") dereferenceable(12) %0, %"heapless::spsc::Queue<(app::P1_T, u8), 3_usize>"* align 4 dereferenceable(16) %self) #10, !dbg !1899
  br label %bb1, !dbg !1899

bb1:                                              ; preds = %start
  ret void, !dbg !1900
}

; <&mut heapless::spsc::Queue<T,_> as core::iter::traits::collect::IntoIterator>::into_iter
; Function Attrs: nounwind
define internal void @"_ZN108_$LT$$RF$mut$u20$heapless..spsc..Queue$LT$T$C$_$GT$$u20$as$u20$core..iter..traits..collect..IntoIterator$GT$9into_iter17h73e2329ae13ae5ccE"(%"heapless::spsc::IterMut<u8, 2_usize>"* noalias nocapture sret(%"heapless::spsc::IterMut<u8, 2_usize>") dereferenceable(12) %0, %"heapless::spsc::Queue<u8, 2_usize>"* align 4 dereferenceable(12) %self) unnamed_addr #0 !dbg !1901 {
start:
  %self.dbg.spill = alloca %"heapless::spsc::Queue<u8, 2_usize>"*, align 4
  store %"heapless::spsc::Queue<u8, 2_usize>"* %self, %"heapless::spsc::Queue<u8, 2_usize>"** %self.dbg.spill, align 4
  call void @llvm.dbg.declare(metadata %"heapless::spsc::Queue<u8, 2_usize>"** %self.dbg.spill, metadata !1903, metadata !DIExpression()), !dbg !1904
; call heapless::spsc::Queue<T,_>::iter_mut
  call void @"_ZN8heapless4spsc18Queue$LT$T$C$_$GT$8iter_mut17h39603bd78c1eaa58E"(%"heapless::spsc::IterMut<u8, 2_usize>"* noalias nocapture sret(%"heapless::spsc::IterMut<u8, 2_usize>") dereferenceable(12) %0, %"heapless::spsc::Queue<u8, 2_usize>"* align 4 dereferenceable(12) %self) #10, !dbg !1905
  br label %bb1, !dbg !1905

bb1:                                              ; preds = %start
  ret void, !dbg !1906
}

; heapless::spsc::Consumer<T,_>::dequeue
; Function Attrs: inlinehint nounwind
define internal { i8, i8 } @"_ZN8heapless4spsc21Consumer$LT$T$C$_$GT$7dequeue17h4076781cd95e852cE"(i32** align 4 dereferenceable(4) %self) unnamed_addr #1 !dbg !1907 {
start:
  %self.dbg.spill = alloca i32**, align 4
  store i32** %self, i32*** %self.dbg.spill, align 4
  call void @llvm.dbg.declare(metadata i32*** %self.dbg.spill, metadata !1912, metadata !DIExpression()), !dbg !1913
  %0 = bitcast i32** %self to %"heapless::spsc::Queue<(app::P1_T, u8), 3_usize>"**, !dbg !1914
  %_2 = load %"heapless::spsc::Queue<(app::P1_T, u8), 3_usize>"*, %"heapless::spsc::Queue<(app::P1_T, u8), 3_usize>"** %0, align 4, !dbg !1914, !nonnull !59
; call heapless::spsc::Queue<T,_>::inner_dequeue
  %1 = call { i8, i8 } @"_ZN8heapless4spsc18Queue$LT$T$C$_$GT$13inner_dequeue17hac125db0968c02faE"(%"heapless::spsc::Queue<(app::P1_T, u8), 3_usize>"* align 4 dereferenceable(16) %_2) #10, !dbg !1914
  %2 = extractvalue { i8, i8 } %1, 0, !dbg !1914
  %3 = extractvalue { i8, i8 } %1, 1, !dbg !1914
  br label %bb1, !dbg !1914

bb1:                                              ; preds = %start
  %4 = insertvalue { i8, i8 } undef, i8 %2, 0, !dbg !1915
  %5 = insertvalue { i8, i8 } %4, i8 %3, 1, !dbg !1915
  ret { i8, i8 } %5, !dbg !1915
}

; heapless::spsc::Producer<T,_>::enqueue_unchecked
; Function Attrs: inlinehint nounwind
define internal void @"_ZN8heapless4spsc21Producer$LT$T$C$_$GT$17enqueue_unchecked17h2ab9bd46dff93908E"(i32** align 4 dereferenceable(4) %self, i8 %val) unnamed_addr #1 !dbg !1916 {
start:
  %val.dbg.spill = alloca i8, align 1
  %self.dbg.spill = alloca i32**, align 4
  store i32** %self, i32*** %self.dbg.spill, align 4
  call void @llvm.dbg.declare(metadata i32*** %self.dbg.spill, metadata !1921, metadata !DIExpression()), !dbg !1923
  store i8 %val, i8* %val.dbg.spill, align 1
  call void @llvm.dbg.declare(metadata i8* %val.dbg.spill, metadata !1922, metadata !DIExpression()), !dbg !1924
  %0 = bitcast i32** %self to %"heapless::spsc::Queue<u8, 2_usize>"**, !dbg !1925
  %_3 = load %"heapless::spsc::Queue<u8, 2_usize>"*, %"heapless::spsc::Queue<u8, 2_usize>"** %0, align 4, !dbg !1925, !nonnull !59
; call heapless::spsc::Queue<T,_>::inner_enqueue_unchecked
  call void @"_ZN8heapless4spsc18Queue$LT$T$C$_$GT$23inner_enqueue_unchecked17h89c8e0b55bfe44b7E"(%"heapless::spsc::Queue<u8, 2_usize>"* align 4 dereferenceable(12) %_3, i8 %val) #10, !dbg !1925
  br label %bb1, !dbg !1925

bb1:                                              ; preds = %start
  ret void, !dbg !1926
}

; core::ptr::const_ptr::<impl *const T>::read
; Function Attrs: inlinehint nounwind
define internal { i8, i8 } @"_ZN4core3ptr9const_ptr33_$LT$impl$u20$$BP$const$u20$T$GT$4read17h5b4f761fdb64603eE"({ i8, i8 }* %self) unnamed_addr #1 !dbg !1927 {
start:
  %self.dbg.spill = alloca { i8, i8 }*, align 4
  store { i8, i8 }* %self, { i8, i8 }** %self.dbg.spill, align 4
  call void @llvm.dbg.declare(metadata { i8, i8 }** %self.dbg.spill, metadata !1935, metadata !DIExpression()), !dbg !1936
; call core::ptr::read
  %0 = call { i8, i8 } @_ZN4core3ptr4read17haa4dd29abe51efc1E({ i8, i8 }* %self) #10, !dbg !1937
  %1 = extractvalue { i8, i8 } %0, 0, !dbg !1937
  %2 = trunc i8 %1 to i1, !dbg !1937
  %3 = extractvalue { i8, i8 } %0, 1, !dbg !1937
  br label %bb1, !dbg !1937

bb1:                                              ; preds = %start
  %4 = zext i1 %2 to i8, !dbg !1938
  %5 = insertvalue { i8, i8 } undef, i8 %4, 0, !dbg !1938
  %6 = insertvalue { i8, i8 } %5, i8 %3, 1, !dbg !1938
  ret { i8, i8 } %6, !dbg !1938
}

; core::ptr::const_ptr::<impl *const T>::read
; Function Attrs: inlinehint nounwind
define internal void @"_ZN4core3ptr9const_ptr33_$LT$impl$u20$$BP$const$u20$T$GT$4read17h60f26a767622c7c8E"({}* %self) unnamed_addr #1 !dbg !1939 {
start:
  %self.dbg.spill = alloca {}*, align 4
  store {}* %self, {}** %self.dbg.spill, align 4
  call void @llvm.dbg.declare(metadata {}** %self.dbg.spill, metadata !1943, metadata !DIExpression()), !dbg !1944
; call core::ptr::read
  call void @_ZN4core3ptr4read17h2454c075427a7a1dE({}* %self) #10, !dbg !1945
  br label %bb1, !dbg !1945

bb1:                                              ; preds = %start
  ret void, !dbg !1946
}

; core::ptr::const_ptr::<impl *const T>::read
; Function Attrs: inlinehint nounwind
define internal i8 @"_ZN4core3ptr9const_ptr33_$LT$impl$u20$$BP$const$u20$T$GT$4read17heb62b597f26bdea0E"(i8* %self) unnamed_addr #1 !dbg !1947 {
start:
  %self.dbg.spill = alloca i8*, align 4
  store i8* %self, i8** %self.dbg.spill, align 4
  call void @llvm.dbg.declare(metadata i8** %self.dbg.spill, metadata !1952, metadata !DIExpression()), !dbg !1953
; call core::ptr::read
  %0 = call i8 @_ZN4core3ptr4read17h7efa0474a61340eeE(i8* %self) #10, !dbg !1954
  br label %bb1, !dbg !1954

bb1:                                              ; preds = %start
  ret i8 %0, !dbg !1955
}

; core::ptr::const_ptr::<impl *const [T]>::as_ptr
; Function Attrs: inlinehint nounwind
define internal i8* @"_ZN4core3ptr9const_ptr43_$LT$impl$u20$$BP$const$u20$$u5b$T$u5d$$GT$6as_ptr17h382d54ca28161cafE"([0 x i8]* %self.0, i32 %self.1) unnamed_addr #1 !dbg !1956 {
start:
  %self.dbg.spill = alloca { [0 x i8]*, i32 }, align 4
  %0 = getelementptr inbounds { [0 x i8]*, i32 }, { [0 x i8]*, i32 }* %self.dbg.spill, i32 0, i32 0
  store [0 x i8]* %self.0, [0 x i8]** %0, align 4
  %1 = getelementptr inbounds { [0 x i8]*, i32 }, { [0 x i8]*, i32 }* %self.dbg.spill, i32 0, i32 1
  store i32 %self.1, i32* %1, align 4
  call void @llvm.dbg.declare(metadata { [0 x i8]*, i32 }* %self.dbg.spill, metadata !1966, metadata !DIExpression()), !dbg !1969
  %2 = bitcast [0 x i8]* %self.0 to i8*, !dbg !1970
  ret i8* %2, !dbg !1971
}

; core::ptr::const_ptr::<impl *const [T]>::as_ptr
; Function Attrs: inlinehint nounwind
define internal { i8, i8 }* @"_ZN4core3ptr9const_ptr43_$LT$impl$u20$$BP$const$u20$$u5b$T$u5d$$GT$6as_ptr17h8e58d8ff10279c42E"([0 x { i8, i8 }]* %self.0, i32 %self.1) unnamed_addr #1 !dbg !1972 {
start:
  %self.dbg.spill = alloca { [0 x { i8, i8 }]*, i32 }, align 4
  %0 = getelementptr inbounds { [0 x { i8, i8 }]*, i32 }, { [0 x { i8, i8 }]*, i32 }* %self.dbg.spill, i32 0, i32 0
  store [0 x { i8, i8 }]* %self.0, [0 x { i8, i8 }]** %0, align 4
  %1 = getelementptr inbounds { [0 x { i8, i8 }]*, i32 }, { [0 x { i8, i8 }]*, i32 }* %self.dbg.spill, i32 0, i32 1
  store i32 %self.1, i32* %1, align 4
  call void @llvm.dbg.declare(metadata { [0 x { i8, i8 }]*, i32 }* %self.dbg.spill, metadata !1981, metadata !DIExpression()), !dbg !1984
  %2 = bitcast [0 x { i8, i8 }]* %self.0 to { i8, i8 }*, !dbg !1985
  ret { i8, i8 }* %2, !dbg !1986
}

; core::ptr::const_ptr::<impl *const [T]>::as_ptr
; Function Attrs: inlinehint nounwind
define internal %"core::mem::maybe_uninit::MaybeUninit<()>"* @"_ZN4core3ptr9const_ptr43_$LT$impl$u20$$BP$const$u20$$u5b$T$u5d$$GT$6as_ptr17hbe58f58226b6e7d4E"([0 x %"core::mem::maybe_uninit::MaybeUninit<()>"]* %self.0, i32 %self.1) unnamed_addr #1 !dbg !1987 {
start:
  %self.dbg.spill = alloca { [0 x %"core::mem::maybe_uninit::MaybeUninit<()>"]*, i32 }, align 4
  %0 = getelementptr inbounds { [0 x %"core::mem::maybe_uninit::MaybeUninit<()>"]*, i32 }, { [0 x %"core::mem::maybe_uninit::MaybeUninit<()>"]*, i32 }* %self.dbg.spill, i32 0, i32 0
  store [0 x %"core::mem::maybe_uninit::MaybeUninit<()>"]* %self.0, [0 x %"core::mem::maybe_uninit::MaybeUninit<()>"]** %0, align 4
  %1 = getelementptr inbounds { [0 x %"core::mem::maybe_uninit::MaybeUninit<()>"]*, i32 }, { [0 x %"core::mem::maybe_uninit::MaybeUninit<()>"]*, i32 }* %self.dbg.spill, i32 0, i32 1
  store i32 %self.1, i32* %1, align 4
  call void @llvm.dbg.declare(metadata { [0 x %"core::mem::maybe_uninit::MaybeUninit<()>"]*, i32 }* %self.dbg.spill, metadata !1996, metadata !DIExpression()), !dbg !1999
  %2 = bitcast [0 x %"core::mem::maybe_uninit::MaybeUninit<()>"]* %self.0 to %"core::mem::maybe_uninit::MaybeUninit<()>"*, !dbg !2000
  ret %"core::mem::maybe_uninit::MaybeUninit<()>"* %2, !dbg !2001
}

; core::ptr::read
; Function Attrs: inlinehint nounwind
define internal void @_ZN4core3ptr4read17h2454c075427a7a1dE({}* %src) unnamed_addr #1 !dbg !2002 {
start:
  %self.dbg.spill.i = alloca %"core::mem::maybe_uninit::MaybeUninit<()>"*, align 4
  %0 = alloca %"core::mem::maybe_uninit::MaybeUninit<()>", align 1
  %src.dbg.spill = alloca {}*, align 4
  %tmp = alloca %"core::mem::maybe_uninit::MaybeUninit<()>", align 1
  store {}* %src, {}** %src.dbg.spill, align 4
  call void @llvm.dbg.declare(metadata {}** %src.dbg.spill, metadata !2004, metadata !DIExpression()), !dbg !2007
  call void @llvm.dbg.declare(metadata %"core::mem::maybe_uninit::MaybeUninit<()>"* %tmp, metadata !2005, metadata !DIExpression()), !dbg !2008
  %1 = bitcast %"core::mem::maybe_uninit::MaybeUninit<()>"* %0 to {}*, !dbg !2009
  br label %bb1, !dbg !2012

bb1:                                              ; preds = %start
  store %"core::mem::maybe_uninit::MaybeUninit<()>"* %tmp, %"core::mem::maybe_uninit::MaybeUninit<()>"** %self.dbg.spill.i, align 4
  call void @llvm.dbg.declare(metadata %"core::mem::maybe_uninit::MaybeUninit<()>"** %self.dbg.spill.i, metadata !457, metadata !DIExpression()), !dbg !2013
  %2 = bitcast %"core::mem::maybe_uninit::MaybeUninit<()>"* %tmp to {}*, !dbg !2015
  br label %bb2, !dbg !2016

bb2:                                              ; preds = %bb1
  %3 = bitcast {}* %2 to i8*, !dbg !2017
  %4 = bitcast {}* %src to i8*, !dbg !2017
  call void @llvm.memcpy.p0i8.p0i8.i32(i8* align 1 %3, i8* align 1 %4, i32 0, i1 false), !dbg !2017
  call void @llvm.dbg.declare(metadata %"core::mem::maybe_uninit::MaybeUninit<()>"* undef, metadata !2018, metadata !DIExpression()), !dbg !2035
  call void @llvm.dbg.declare(metadata %"core::mem::manually_drop::ManuallyDrop<()>"* undef, metadata !2037, metadata !DIExpression()), !dbg !2042
  br label %bb3, !dbg !2044

bb3:                                              ; preds = %bb2
  ret void, !dbg !2045
}

; core::ptr::read
; Function Attrs: inlinehint nounwind
define internal i8 @_ZN4core3ptr4read17h7efa0474a61340eeE(i8* %src) unnamed_addr #1 !dbg !2046 {
start:
  %slot.dbg.spill.i.i = alloca i8, align 1
  %self.dbg.spill.i1 = alloca i8, align 1
  %self.dbg.spill.i = alloca i8*, align 4
  %0 = alloca i8, align 1
  %src.dbg.spill = alloca i8*, align 4
  %tmp = alloca i8, align 1
  store i8* %src, i8** %src.dbg.spill, align 4
  call void @llvm.dbg.declare(metadata i8** %src.dbg.spill, metadata !2048, metadata !DIExpression()), !dbg !2051
  call void @llvm.dbg.declare(metadata i8* %tmp, metadata !2049, metadata !DIExpression()), !dbg !2052
  %1 = bitcast i8* %0 to {}*, !dbg !2053
  %2 = load i8, i8* %0, align 1, !dbg !2058
  store i8 %2, i8* %tmp, align 1, !dbg !2059
  br label %bb1, !dbg !2059

bb1:                                              ; preds = %start
  store i8* %tmp, i8** %self.dbg.spill.i, align 4
  call void @llvm.dbg.declare(metadata i8** %self.dbg.spill.i, metadata !2060, metadata !DIExpression()), !dbg !2066
  br label %bb2, !dbg !2068

bb2:                                              ; preds = %bb1
  call void @llvm.memcpy.p0i8.p0i8.i32(i8* align 1 %tmp, i8* align 1 %src, i32 1, i1 false), !dbg !2069
  %_6 = load i8, i8* %tmp, align 1, !dbg !2070
  store i8 %_6, i8* %self.dbg.spill.i1, align 1
  call void @llvm.dbg.declare(metadata i8* %self.dbg.spill.i1, metadata !2071, metadata !DIExpression()), !dbg !2076
  store i8 %_6, i8* %slot.dbg.spill.i.i, align 1
  call void @llvm.dbg.declare(metadata i8* %slot.dbg.spill.i.i, metadata !2078, metadata !DIExpression()), !dbg !2083
  br label %bb3, !dbg !2070

bb3:                                              ; preds = %bb2
  ret i8 %_6, !dbg !2085
}

; core::ptr::read
; Function Attrs: inlinehint nounwind
define internal { i8, i8 } @_ZN4core3ptr4read17haa4dd29abe51efc1E({ i8, i8 }* %src) unnamed_addr #1 !dbg !2086 {
start:
  %slot.dbg.spill.i.i = alloca { i8, i8 }, align 1
  %self.dbg.spill.i1 = alloca { i8, i8 }, align 1
  %self.dbg.spill.i = alloca { i8, i8 }*, align 4
  %0 = alloca { i8, i8 }, align 1
  %src.dbg.spill = alloca { i8, i8 }*, align 4
  %tmp = alloca { i8, i8 }, align 1
  store { i8, i8 }* %src, { i8, i8 }** %src.dbg.spill, align 4
  call void @llvm.dbg.declare(metadata { i8, i8 }** %src.dbg.spill, metadata !2088, metadata !DIExpression()), !dbg !2091
  call void @llvm.dbg.declare(metadata { i8, i8 }* %tmp, metadata !2089, metadata !DIExpression()), !dbg !2092
  %1 = bitcast { i8, i8 }* %0 to {}*, !dbg !2093
  %2 = getelementptr inbounds { i8, i8 }, { i8, i8 }* %0, i32 0, i32 0, !dbg !2098
  %3 = load i8, i8* %2, align 1, !dbg !2098
  %4 = getelementptr inbounds { i8, i8 }, { i8, i8 }* %0, i32 0, i32 1, !dbg !2098
  %5 = load i8, i8* %4, align 1, !dbg !2098
  %6 = insertvalue { i8, i8 } undef, i8 %3, 0, !dbg !2098
  %7 = insertvalue { i8, i8 } %6, i8 %5, 1, !dbg !2098
  store { i8, i8 } %7, { i8, i8 }* %tmp, align 1, !dbg !2099
  br label %bb1, !dbg !2099

bb1:                                              ; preds = %start
  store { i8, i8 }* %tmp, { i8, i8 }** %self.dbg.spill.i, align 4
  call void @llvm.dbg.declare(metadata { i8, i8 }** %self.dbg.spill.i, metadata !2100, metadata !DIExpression()), !dbg !2107
  br label %bb2, !dbg !2109

bb2:                                              ; preds = %bb1
  %8 = bitcast { i8, i8 }* %tmp to i8*, !dbg !2110
  %9 = bitcast { i8, i8 }* %src to i8*, !dbg !2110
  call void @llvm.memcpy.p0i8.p0i8.i32(i8* align 1 %8, i8* align 1 %9, i32 2, i1 false), !dbg !2110
  %10 = getelementptr inbounds { i8, i8 }, { i8, i8 }* %tmp, i32 0, i32 0, !dbg !2111
  %_6.0 = load i8, i8* %10, align 1, !dbg !2111
  %11 = getelementptr inbounds { i8, i8 }, { i8, i8 }* %tmp, i32 0, i32 1, !dbg !2111
  %_6.1 = load i8, i8* %11, align 1, !dbg !2111
  %12 = getelementptr inbounds { i8, i8 }, { i8, i8 }* %self.dbg.spill.i1, i32 0, i32 0
  store i8 %_6.0, i8* %12, align 1
  %13 = getelementptr inbounds { i8, i8 }, { i8, i8 }* %self.dbg.spill.i1, i32 0, i32 1
  store i8 %_6.1, i8* %13, align 1
  call void @llvm.dbg.declare(metadata { i8, i8 }* %self.dbg.spill.i1, metadata !2112, metadata !DIExpression()), !dbg !2117
  %_3.0.i = trunc i8 %_6.0 to i1, !dbg !2119
  %14 = getelementptr inbounds { i8, i8 }, { i8, i8 }* %slot.dbg.spill.i.i, i32 0, i32 0
  %15 = zext i1 %_3.0.i to i8
  store i8 %15, i8* %14, align 1
  %16 = getelementptr inbounds { i8, i8 }, { i8, i8 }* %slot.dbg.spill.i.i, i32 0, i32 1
  store i8 %_6.1, i8* %16, align 1
  call void @llvm.dbg.declare(metadata { i8, i8 }* %slot.dbg.spill.i.i, metadata !2120, metadata !DIExpression()), !dbg !2125
  %17 = zext i1 %_3.0.i to i8, !dbg !2127
  %18 = insertvalue { i8, i8 } undef, i8 %17, 0, !dbg !2127
  %19 = insertvalue { i8, i8 } %18, i8 %_6.1, 1, !dbg !2127
  %20 = zext i1 %_3.0.i to i8, !dbg !2128
  %21 = insertvalue { i8, i8 } undef, i8 %20, 0, !dbg !2128
  %22 = insertvalue { i8, i8 } %21, i8 %_6.1, 1, !dbg !2128
  %23 = extractvalue { i8, i8 } %22, 0, !dbg !2111
  %24 = trunc i8 %23 to i1, !dbg !2111
  %25 = extractvalue { i8, i8 } %22, 1, !dbg !2111
  br label %bb3, !dbg !2111

bb3:                                              ; preds = %bb2
  %26 = zext i1 %24 to i8, !dbg !2129
  %27 = insertvalue { i8, i8 } undef, i8 %26, 0, !dbg !2129
  %28 = insertvalue { i8, i8 } %27, i8 %25, 1, !dbg !2129
  ret { i8, i8 } %28, !dbg !2129
}

; core::ptr::write
; Function Attrs: inlinehint nounwind
define internal void @_ZN4core3ptr5write17h3411c4701a77cae5E({}* %dst) unnamed_addr #1 !dbg !2130 {
start:
  %dst.dbg.spill = alloca {}*, align 4
  %src = alloca {}, align 1
  store {}* %dst, {}** %dst.dbg.spill, align 4
  call void @llvm.dbg.declare(metadata {}** %dst.dbg.spill, metadata !2132, metadata !DIExpression()), !dbg !2134
  call void @llvm.dbg.declare(metadata {}* %src, metadata !2133, metadata !DIExpression()), !dbg !2135
  %0 = bitcast {}* %dst to i8*, !dbg !2136
  %1 = bitcast {}* %src to i8*, !dbg !2136
  call void @llvm.memcpy.p0i8.p0i8.i32(i8* align 1 %0, i8* align 1 %1, i32 0, i1 false), !dbg !2136
  ret void, !dbg !2137
}

; core::ptr::write
; Function Attrs: inlinehint nounwind
define internal void @_ZN4core3ptr5write17h44c3a12bdde493dfE(i8* %dst, i8 %0) unnamed_addr #1 !dbg !2138 {
start:
  %dst.dbg.spill = alloca i8*, align 4
  %src = alloca i8, align 1
  store i8 %0, i8* %src, align 1
  store i8* %dst, i8** %dst.dbg.spill, align 4
  call void @llvm.dbg.declare(metadata i8** %dst.dbg.spill, metadata !2142, metadata !DIExpression()), !dbg !2144
  call void @llvm.dbg.declare(metadata i8* %src, metadata !2143, metadata !DIExpression()), !dbg !2145
  call void @llvm.memcpy.p0i8.p0i8.i32(i8* align 1 %dst, i8* align 1 %src, i32 1, i1 false), !dbg !2146
  ret void, !dbg !2147
}

; core::ptr::write
; Function Attrs: inlinehint nounwind
define internal void @_ZN4core3ptr5write17h61a4480ba65cfdcfE(i8* %dst, i8 %0) unnamed_addr #1 !dbg !2148 {
start:
  %dst.dbg.spill = alloca i8*, align 4
  %src = alloca i8, align 1
  store i8 %0, i8* %src, align 1
  store i8* %dst, i8** %dst.dbg.spill, align 4
  call void @llvm.dbg.declare(metadata i8** %dst.dbg.spill, metadata !2150, metadata !DIExpression()), !dbg !2152
  call void @llvm.dbg.declare(metadata i8* %src, metadata !2151, metadata !DIExpression()), !dbg !2153
  call void @llvm.memcpy.p0i8.p0i8.i32(i8* align 1 %dst, i8* align 1 %src, i32 1, i1 false), !dbg !2154
  ret void, !dbg !2155
}

; core::ptr::write
; Function Attrs: inlinehint nounwind
define internal void @_ZN4core3ptr5write17h82b9f7fb1924c0bdE(i64* %dst, i64 %0) unnamed_addr #1 !dbg !2156 {
start:
  %dst.dbg.spill = alloca i64*, align 4
  %src = alloca i64, align 8
  store i64 %0, i64* %src, align 8
  store i64* %dst, i64** %dst.dbg.spill, align 4
  call void @llvm.dbg.declare(metadata i64** %dst.dbg.spill, metadata !2158, metadata !DIExpression()), !dbg !2160
  call void @llvm.dbg.declare(metadata i64* %src, metadata !2159, metadata !DIExpression()), !dbg !2161
  %1 = bitcast i64* %dst to i8*, !dbg !2162
  %2 = bitcast i64* %src to i8*, !dbg !2162
  call void @llvm.memcpy.p0i8.p0i8.i32(i8* align 8 %1, i8* align 8 %2, i32 8, i1 false), !dbg !2162
  ret void, !dbg !2163
}

; core::ptr::write
; Function Attrs: inlinehint nounwind
define internal void @_ZN4core3ptr5write17hcf998c1cb7ac1d77E({ i8, i8 }* %dst, i8 %0, i8 %1) unnamed_addr #1 !dbg !2164 {
start:
  %dst.dbg.spill = alloca { i8, i8 }*, align 4
  %src = alloca { i8, i8 }, align 1
  %2 = getelementptr inbounds { i8, i8 }, { i8, i8 }* %src, i32 0, i32 0
  store i8 %0, i8* %2, align 1
  %3 = getelementptr inbounds { i8, i8 }, { i8, i8 }* %src, i32 0, i32 1
  store i8 %1, i8* %3, align 1
  store { i8, i8 }* %dst, { i8, i8 }** %dst.dbg.spill, align 4
  call void @llvm.dbg.declare(metadata { i8, i8 }** %dst.dbg.spill, metadata !2166, metadata !DIExpression()), !dbg !2168
  call void @llvm.dbg.declare(metadata { i8, i8 }* %src, metadata !2167, metadata !DIExpression()), !dbg !2169
  %4 = bitcast { i8, i8 }* %dst to i8*, !dbg !2170
  %5 = bitcast { i8, i8 }* %src to i8*, !dbg !2170
  call void @llvm.memcpy.p0i8.p0i8.i32(i8* align 1 %4, i8* align 1 %5, i32 2, i1 false), !dbg !2170
  ret void, !dbg !2171
}

; <T as core::convert::Into<U>>::into
; Function Attrs: nounwind
define internal void @"_ZN50_$LT$T$u20$as$u20$core..convert..Into$LT$U$GT$$GT$4into17hb837751d062ab74fE"() unnamed_addr #0 !dbg !2172 {
start:
  %self.dbg.spill = alloca %"cortex_m::peripheral::Peripherals", align 1
  call void @llvm.dbg.declare(metadata %"cortex_m::peripheral::Peripherals"* %self.dbg.spill, metadata !2177, metadata !DIExpression()), !dbg !2181
; call <T as core::convert::From<T>>::from
  call void @"_ZN50_$LT$T$u20$as$u20$core..convert..From$LT$T$GT$$GT$4from17hae6616355c332270E"() #10, !dbg !2182
  br label %bb1, !dbg !2182

bb1:                                              ; preds = %start
  ret void, !dbg !2183
}

; <T as core::convert::From<T>>::from
; Function Attrs: nounwind
define internal void @"_ZN50_$LT$T$u20$as$u20$core..convert..From$LT$T$GT$$GT$4from17hae6616355c332270E"() unnamed_addr #0 !dbg !2184 {
start:
  %t.dbg.spill = alloca %"cortex_m::peripheral::Peripherals", align 1
  call void @llvm.dbg.declare(metadata %"cortex_m::peripheral::Peripherals"* %t.dbg.spill, metadata !2187, metadata !DIExpression()), !dbg !2189
  ret void, !dbg !2190
}

; core::cell::Cell<T>::new
; Function Attrs: inlinehint nounwind
define internal i8 @"_ZN4core4cell13Cell$LT$T$GT$3new17h0ec9730ff8e64e09E"(i8 %value) unnamed_addr #1 !dbg !2191 {
start:
  %value.dbg.spill.i = alloca i8, align 1
  %0 = alloca i8, align 1
  %value.dbg.spill = alloca i8, align 1
  %1 = alloca i8, align 1
  store i8 %value, i8* %value.dbg.spill, align 1
  call void @llvm.dbg.declare(metadata i8* %value.dbg.spill, metadata !2195, metadata !DIExpression()), !dbg !2196
  store i8 %value, i8* %value.dbg.spill.i, align 1
  call void @llvm.dbg.declare(metadata i8* %value.dbg.spill.i, metadata !2197, metadata !DIExpression()), !dbg !2202
  store i8 %value, i8* %0, align 1, !dbg !2204
  %2 = load i8, i8* %0, align 1, !dbg !2205
  br label %bb1, !dbg !2206

bb1:                                              ; preds = %start
  store i8 %2, i8* %1, align 1, !dbg !2207
  %3 = load i8, i8* %1, align 1, !dbg !2208
  ret i8 %3, !dbg !2208
}

; <core::ops::range::Range<T> as core::iter::range::RangeIteratorImpl>::spec_next
; Function Attrs: inlinehint nounwind
define internal { i8, i8 } @"_ZN89_$LT$core..ops..range..Range$LT$T$GT$$u20$as$u20$core..iter..range..RangeIteratorImpl$GT$9spec_next17he4c612b2254b2254E"({ i8, i8 }* align 1 dereferenceable(2) %self) unnamed_addr #1 !dbg !2209 {
start:
  %n.dbg.spill = alloca i8, align 1
  %self.dbg.spill = alloca { i8, i8 }*, align 4
  %0 = alloca { i8, i8 }, align 1
  store { i8, i8 }* %self, { i8, i8 }** %self.dbg.spill, align 4
  call void @llvm.dbg.declare(metadata { i8, i8 }** %self.dbg.spill, metadata !2226, metadata !DIExpression()), !dbg !2229
  %_3 = bitcast { i8, i8 }* %self to i8*, !dbg !2230
  %_4 = getelementptr inbounds { i8, i8 }, { i8, i8 }* %self, i32 0, i32 1, !dbg !2231
; call core::cmp::impls::<impl core::cmp::PartialOrd for u8>::lt
  %_2 = call zeroext i1 @"_ZN4core3cmp5impls54_$LT$impl$u20$core..cmp..PartialOrd$u20$for$u20$u8$GT$2lt17h931a40755a640d7eE"(i8* align 1 dereferenceable(1) %_3, i8* align 1 dereferenceable(1) %_4) #10, !dbg !2230
  br label %bb1, !dbg !2230

bb1:                                              ; preds = %start
  br i1 %_2, label %bb2, label %bb6, !dbg !2230

bb6:                                              ; preds = %bb1
  %1 = bitcast { i8, i8 }* %0 to i8*, !dbg !2232
  store i8 0, i8* %1, align 1, !dbg !2232
  br label %bb7, !dbg !2233

bb2:                                              ; preds = %bb1
  %_7 = bitcast { i8, i8 }* %self to i8*, !dbg !2234
; call core::clone::impls::<impl core::clone::Clone for u8>::clone
  %_6 = call i8 @"_ZN4core5clone5impls51_$LT$impl$u20$core..clone..Clone$u20$for$u20$u8$GT$5clone17h0bd6b78f87b73bf1E"(i8* align 1 dereferenceable(1) %_7) #10, !dbg !2234
  br label %bb3, !dbg !2234

bb3:                                              ; preds = %bb2
; call <u8 as core::iter::range::Step>::forward_unchecked
  %n = call i8 @"_ZN46_$LT$u8$u20$as$u20$core..iter..range..Step$GT$17forward_unchecked17ha8d94eade2e46a9aE"(i8 %_6, i32 1) #10, !dbg !2235
  store i8 %n, i8* %n.dbg.spill, align 1, !dbg !2235
  call void @llvm.dbg.declare(metadata i8* %n.dbg.spill, metadata !2227, metadata !DIExpression()), !dbg !2236
  br label %bb4, !dbg !2235

bb4:                                              ; preds = %bb3
  %_10 = bitcast { i8, i8 }* %self to i8*, !dbg !2237
; call core::mem::replace
  %_8 = call i8 @_ZN4core3mem7replace17h6f6f0ffe151c8c1aE(i8* align 1 dereferenceable(1) %_10, i8 %n) #10, !dbg !2238
  br label %bb5, !dbg !2238

bb5:                                              ; preds = %bb4
  %2 = bitcast { i8, i8 }* %0 to %"core::option::Option<u8>::Some"*, !dbg !2239
  %3 = getelementptr inbounds %"core::option::Option<u8>::Some", %"core::option::Option<u8>::Some"* %2, i32 0, i32 1, !dbg !2239
  store i8 %_8, i8* %3, align 1, !dbg !2239
  %4 = bitcast { i8, i8 }* %0 to i8*, !dbg !2239
  store i8 1, i8* %4, align 1, !dbg !2239
  br label %bb7, !dbg !2233

bb7:                                              ; preds = %bb6, %bb5
  %5 = getelementptr inbounds { i8, i8 }, { i8, i8 }* %0, i32 0, i32 0, !dbg !2240
  %6 = load i8, i8* %5, align 1, !dbg !2240, !range !436
  %7 = trunc i8 %6 to i1, !dbg !2240
  %8 = getelementptr inbounds { i8, i8 }, { i8, i8 }* %0, i32 0, i32 1, !dbg !2240
  %9 = load i8, i8* %8, align 1, !dbg !2240
  %10 = zext i1 %7 to i8, !dbg !2240
  %11 = insertvalue { i8, i8 } undef, i8 %10, 0, !dbg !2240
  %12 = insertvalue { i8, i8 } %11, i8 %9, 1, !dbg !2240
  ret { i8, i8 } %12, !dbg !2240
}

; core::iter::range::<impl core::iter::traits::iterator::Iterator for core::ops::range::Range<A>>::next
; Function Attrs: inlinehint nounwind
define internal { i8, i8 } @"_ZN4core4iter5range101_$LT$impl$u20$core..iter..traits..iterator..Iterator$u20$for$u20$core..ops..range..Range$LT$A$GT$$GT$4next17hcdb043e911fa495fE"({ i8, i8 }* align 1 dereferenceable(2) %self) unnamed_addr #1 !dbg !2241 {
start:
  %self.dbg.spill = alloca { i8, i8 }*, align 4
  store { i8, i8 }* %self, { i8, i8 }** %self.dbg.spill, align 4
  call void @llvm.dbg.declare(metadata { i8, i8 }** %self.dbg.spill, metadata !2244, metadata !DIExpression()), !dbg !2247
; call <core::ops::range::Range<T> as core::iter::range::RangeIteratorImpl>::spec_next
  %0 = call { i8, i8 } @"_ZN89_$LT$core..ops..range..Range$LT$T$GT$$u20$as$u20$core..iter..range..RangeIteratorImpl$GT$9spec_next17he4c612b2254b2254E"({ i8, i8 }* align 1 dereferenceable(2) %self) #10, !dbg !2248
  %1 = extractvalue { i8, i8 } %0, 0, !dbg !2248
  %2 = trunc i8 %1 to i1, !dbg !2248
  %3 = extractvalue { i8, i8 } %0, 1, !dbg !2248
  br label %bb1, !dbg !2248

bb1:                                              ; preds = %start
  %4 = zext i1 %2 to i8, !dbg !2249
  %5 = insertvalue { i8, i8 } undef, i8 %4, 0, !dbg !2249
  %6 = insertvalue { i8, i8 } %5, i8 %3, 1, !dbg !2249
  ret { i8, i8 } %6, !dbg !2249
}

; core::iter::traits::iterator::Iterator::for_each
; Function Attrs: inlinehint nounwind
define internal void @_ZN4core4iter6traits8iterator8Iterator8for_each17h003edabc4355739dE(i8 %self.0, i8 %self.1) unnamed_addr #1 !dbg !2250 {
start:
  %f.dbg.spill = alloca %"[closure@examples/locals.rs:10:1: 10:62].0", align 1
  %self.dbg.spill = alloca { i8, i8 }, align 1
  %0 = getelementptr inbounds { i8, i8 }, { i8, i8 }* %self.dbg.spill, i32 0, i32 0
  store i8 %self.0, i8* %0, align 1
  %1 = getelementptr inbounds { i8, i8 }, { i8, i8 }* %self.dbg.spill, i32 0, i32 1
  store i8 %self.1, i8* %1, align 1
  call void @llvm.dbg.declare(metadata { i8, i8 }* %self.dbg.spill, metadata !2258, metadata !DIExpression()), !dbg !2263
  call void @llvm.dbg.declare(metadata %"[closure@examples/locals.rs:10:1: 10:62].0"* %f.dbg.spill, metadata !2259, metadata !DIExpression()), !dbg !2264
; call core::iter::traits::iterator::Iterator::for_each::call
  call void @_ZN4core4iter6traits8iterator8Iterator8for_each4call17ha52f4198052b8887E() #10, !dbg !2265
  br label %bb1, !dbg !2265

bb1:                                              ; preds = %start
; call core::iter::traits::iterator::Iterator::fold
  call void @_ZN4core4iter6traits8iterator8Iterator4fold17h66e2ab47353ef40dE(i8 %self.0, i8 %self.1) #10, !dbg !2266
  br label %bb2, !dbg !2266

bb2:                                              ; preds = %bb1
  ret void, !dbg !2267
}

; core::iter::traits::iterator::Iterator::for_each
; Function Attrs: inlinehint nounwind
define internal void @_ZN4core4iter6traits8iterator8Iterator8for_each17h248455fa942f1094E(i8 %self.0, i8 %self.1) unnamed_addr #1 !dbg !2268 {
start:
  %f.dbg.spill = alloca %"[closure@examples/locals.rs:10:1: 10:62]", align 1
  %self.dbg.spill = alloca { i8, i8 }, align 1
  %0 = getelementptr inbounds { i8, i8 }, { i8, i8 }* %self.dbg.spill, i32 0, i32 0
  store i8 %self.0, i8* %0, align 1
  %1 = getelementptr inbounds { i8, i8 }, { i8, i8 }* %self.dbg.spill, i32 0, i32 1
  store i8 %self.1, i8* %1, align 1
  call void @llvm.dbg.declare(metadata { i8, i8 }* %self.dbg.spill, metadata !2272, metadata !DIExpression()), !dbg !2276
  call void @llvm.dbg.declare(metadata %"[closure@examples/locals.rs:10:1: 10:62]"* %f.dbg.spill, metadata !2273, metadata !DIExpression()), !dbg !2277
; call core::iter::traits::iterator::Iterator::for_each::call
  call void @_ZN4core4iter6traits8iterator8Iterator8for_each4call17h6690a92d87f028f3E() #10, !dbg !2278
  br label %bb1, !dbg !2278

bb1:                                              ; preds = %start
; call core::iter::traits::iterator::Iterator::fold
  call void @_ZN4core4iter6traits8iterator8Iterator4fold17haea35f2c93648cf3E(i8 %self.0, i8 %self.1) #10, !dbg !2279
  br label %bb2, !dbg !2279

bb2:                                              ; preds = %bb1
  ret void, !dbg !2280
}

; core::iter::traits::iterator::Iterator::for_each::call
; Function Attrs: inlinehint nounwind
define internal void @_ZN4core4iter6traits8iterator8Iterator8for_each4call17h6690a92d87f028f3E() unnamed_addr #1 !dbg !2281 {
start:
  %f.dbg.spill = alloca %"[closure@examples/locals.rs:10:1: 10:62]", align 1
  %0 = alloca %"[closure@core::iter::traits::iterator::Iterator::for_each::call<u8, [closure@examples/locals.rs:10:1: 10:62]>::{closure#0}].1", align 1
  call void @llvm.dbg.declare(metadata %"[closure@examples/locals.rs:10:1: 10:62]"* %f.dbg.spill, metadata !2286, metadata !DIExpression()), !dbg !2289
  %1 = bitcast %"[closure@core::iter::traits::iterator::Iterator::for_each::call<u8, [closure@examples/locals.rs:10:1: 10:62]>::{closure#0}].1"* %0 to %"[closure@examples/locals.rs:10:1: 10:62]"*, !dbg !2290
  ret void, !dbg !2291
}

; core::iter::traits::iterator::Iterator::for_each::call
; Function Attrs: inlinehint nounwind
define internal void @_ZN4core4iter6traits8iterator8Iterator8for_each4call17ha52f4198052b8887E() unnamed_addr #1 !dbg !2292 {
start:
  %f.dbg.spill = alloca %"[closure@examples/locals.rs:10:1: 10:62].0", align 1
  %0 = alloca %"[closure@core::iter::traits::iterator::Iterator::for_each::call<u8, [closure@examples/locals.rs:10:1: 10:62]>::{closure#0}]", align 1
  call void @llvm.dbg.declare(metadata %"[closure@examples/locals.rs:10:1: 10:62].0"* %f.dbg.spill, metadata !2296, metadata !DIExpression()), !dbg !2299
  %1 = bitcast %"[closure@core::iter::traits::iterator::Iterator::for_each::call<u8, [closure@examples/locals.rs:10:1: 10:62]>::{closure#0}]"* %0 to %"[closure@examples/locals.rs:10:1: 10:62].0"*, !dbg !2300
  ret void, !dbg !2301
}

; core::iter::traits::iterator::Iterator::for_each::call::{{closure}}
; Function Attrs: inlinehint nounwind
define internal void @"_ZN4core4iter6traits8iterator8Iterator8for_each4call28_$u7b$$u7b$closure$u7d$$u7d$17h27b028bfd3eed342E"(%"[closure@core::iter::traits::iterator::Iterator::for_each::call<u8, [closure@examples/locals.rs:10:1: 10:62]>::{closure#0}]"* nonnull align 1 %_1, i8 %item) unnamed_addr #1 !dbg !2302 {
start:
  %item.dbg.spill = alloca i8, align 1
  %_2.dbg.spill = alloca {}, align 1
  %_1.dbg.spill = alloca %"[closure@core::iter::traits::iterator::Iterator::for_each::call<u8, [closure@examples/locals.rs:10:1: 10:62]>::{closure#0}]"*, align 4
  %_5 = alloca i8, align 1
  store %"[closure@core::iter::traits::iterator::Iterator::for_each::call<u8, [closure@examples/locals.rs:10:1: 10:62]>::{closure#0}]"* %_1, %"[closure@core::iter::traits::iterator::Iterator::for_each::call<u8, [closure@examples/locals.rs:10:1: 10:62]>::{closure#0}]"** %_1.dbg.spill, align 4
  %0 = load %"[closure@core::iter::traits::iterator::Iterator::for_each::call<u8, [closure@examples/locals.rs:10:1: 10:62]>::{closure#0}]"*, %"[closure@core::iter::traits::iterator::Iterator::for_each::call<u8, [closure@examples/locals.rs:10:1: 10:62]>::{closure#0}]"** %_1.dbg.spill, align 4, !nonnull !59
  %1 = bitcast %"[closure@core::iter::traits::iterator::Iterator::for_each::call<u8, [closure@examples/locals.rs:10:1: 10:62]>::{closure#0}]"* %0 to %"[closure@examples/locals.rs:10:1: 10:62].0"*
  call void @llvm.dbg.declare(metadata %"[closure@core::iter::traits::iterator::Iterator::for_each::call<u8, [closure@examples/locals.rs:10:1: 10:62]>::{closure#0}]"** %_1.dbg.spill, metadata !2312, metadata !DIExpression(DW_OP_deref)), !dbg !2314
  call void @llvm.dbg.declare(metadata {}* %_2.dbg.spill, metadata !2313, metadata !DIExpression()), !dbg !2315
  store i8 %item, i8* %item.dbg.spill, align 1
  call void @llvm.dbg.declare(metadata i8* %item.dbg.spill, metadata !2311, metadata !DIExpression()), !dbg !2316
  %_4 = bitcast %"[closure@core::iter::traits::iterator::Iterator::for_each::call<u8, [closure@examples/locals.rs:10:1: 10:62]>::{closure#0}]"* %_1 to %"[closure@examples/locals.rs:10:1: 10:62].0"*, !dbg !2317
  store i8 %item, i8* %_5, align 1, !dbg !2317
  %2 = load i8, i8* %_5, align 1, !dbg !2317
; call locals::app::rtic_ext::main::{{closure}}
  call void @"_ZN6locals3app8rtic_ext4main28_$u7b$$u7b$closure$u7d$$u7d$17h55422711175e8f91E"(%"[closure@examples/locals.rs:10:1: 10:62].0"* nonnull align 1 %_4, i8 %2) #10, !dbg !2317
  br label %bb1, !dbg !2317

bb1:                                              ; preds = %start
  ret void, !dbg !2318
}

; core::iter::traits::iterator::Iterator::for_each::call::{{closure}}
; Function Attrs: inlinehint nounwind
define internal void @"_ZN4core4iter6traits8iterator8Iterator8for_each4call28_$u7b$$u7b$closure$u7d$$u7d$17h8120bf3118aae8a7E"(%"[closure@core::iter::traits::iterator::Iterator::for_each::call<u8, [closure@examples/locals.rs:10:1: 10:62]>::{closure#0}].1"* nonnull align 1 %_1, i8 %item) unnamed_addr #1 !dbg !2319 {
start:
  %item.dbg.spill = alloca i8, align 1
  %_2.dbg.spill = alloca {}, align 1
  %_1.dbg.spill = alloca %"[closure@core::iter::traits::iterator::Iterator::for_each::call<u8, [closure@examples/locals.rs:10:1: 10:62]>::{closure#0}].1"*, align 4
  %_5 = alloca i8, align 1
  store %"[closure@core::iter::traits::iterator::Iterator::for_each::call<u8, [closure@examples/locals.rs:10:1: 10:62]>::{closure#0}].1"* %_1, %"[closure@core::iter::traits::iterator::Iterator::for_each::call<u8, [closure@examples/locals.rs:10:1: 10:62]>::{closure#0}].1"** %_1.dbg.spill, align 4
  %0 = load %"[closure@core::iter::traits::iterator::Iterator::for_each::call<u8, [closure@examples/locals.rs:10:1: 10:62]>::{closure#0}].1"*, %"[closure@core::iter::traits::iterator::Iterator::for_each::call<u8, [closure@examples/locals.rs:10:1: 10:62]>::{closure#0}].1"** %_1.dbg.spill, align 4, !nonnull !59
  %1 = bitcast %"[closure@core::iter::traits::iterator::Iterator::for_each::call<u8, [closure@examples/locals.rs:10:1: 10:62]>::{closure#0}].1"* %0 to %"[closure@examples/locals.rs:10:1: 10:62]"*
  call void @llvm.dbg.declare(metadata %"[closure@core::iter::traits::iterator::Iterator::for_each::call<u8, [closure@examples/locals.rs:10:1: 10:62]>::{closure#0}].1"** %_1.dbg.spill, metadata !2328, metadata !DIExpression(DW_OP_deref)), !dbg !2330
  call void @llvm.dbg.declare(metadata {}* %_2.dbg.spill, metadata !2329, metadata !DIExpression()), !dbg !2331
  store i8 %item, i8* %item.dbg.spill, align 1
  call void @llvm.dbg.declare(metadata i8* %item.dbg.spill, metadata !2327, metadata !DIExpression()), !dbg !2332
  %_4 = bitcast %"[closure@core::iter::traits::iterator::Iterator::for_each::call<u8, [closure@examples/locals.rs:10:1: 10:62]>::{closure#0}].1"* %_1 to %"[closure@examples/locals.rs:10:1: 10:62]"*, !dbg !2333
  store i8 %item, i8* %_5, align 1, !dbg !2333
  %2 = load i8, i8* %_5, align 1, !dbg !2333
; call locals::app::rtic_ext::main::{{closure}}
  call void @"_ZN6locals3app8rtic_ext4main28_$u7b$$u7b$closure$u7d$$u7d$17h1bdb1d17f7ba3d47E"(%"[closure@examples/locals.rs:10:1: 10:62]"* nonnull align 1 %_4, i8 %2) #10, !dbg !2333
  br label %bb1, !dbg !2333

bb1:                                              ; preds = %start
  ret void, !dbg !2334
}

; core::iter::traits::iterator::Iterator::fold
; Function Attrs: inlinehint nounwind
define internal void @_ZN4core4iter6traits8iterator8Iterator4fold17h66e2ab47353ef40dE(i8 %0, i8 %1) unnamed_addr #1 !dbg !2335 {
start:
  %x.dbg.spill = alloca i8, align 1
  %init.dbg.spill = alloca {}, align 1
  %accum.dbg.spill = alloca {}, align 1
  %_10 = alloca i8, align 1
  %_4 = alloca { i8, i8 }, align 1
  %f = alloca %"[closure@core::iter::traits::iterator::Iterator::for_each::call<u8, [closure@examples/locals.rs:10:1: 10:62]>::{closure#0}]", align 1
  %self = alloca { i8, i8 }, align 1
  %2 = getelementptr inbounds { i8, i8 }, { i8, i8 }* %self, i32 0, i32 0
  store i8 %0, i8* %2, align 1
  %3 = getelementptr inbounds { i8, i8 }, { i8, i8 }* %self, i32 0, i32 1
  store i8 %1, i8* %3, align 1
  call void @llvm.dbg.declare(metadata {}* %accum.dbg.spill, metadata !2342, metadata !DIExpression()), !dbg !2349
  call void @llvm.dbg.declare(metadata { i8, i8 }* %self, metadata !2339, metadata !DIExpression()), !dbg !2350
  call void @llvm.dbg.declare(metadata {}* %init.dbg.spill, metadata !2340, metadata !DIExpression()), !dbg !2351
  call void @llvm.dbg.declare(metadata %"[closure@core::iter::traits::iterator::Iterator::for_each::call<u8, [closure@examples/locals.rs:10:1: 10:62]>::{closure#0}]"* %f, metadata !2341, metadata !DIExpression()), !dbg !2352
  br label %bb1, !dbg !2353

bb1:                                              ; preds = %bb4, %start
; call core::iter::range::<impl core::iter::traits::iterator::Iterator for core::ops::range::Range<A>>::next
  %4 = call { i8, i8 } @"_ZN4core4iter5range101_$LT$impl$u20$core..iter..traits..iterator..Iterator$u20$for$u20$core..ops..range..Range$LT$A$GT$$GT$4next17hcdb043e911fa495fE"({ i8, i8 }* align 1 dereferenceable(2) %self) #10, !dbg !2354
  store { i8, i8 } %4, { i8, i8 }* %_4, align 1, !dbg !2354
  br label %bb2, !dbg !2354

bb2:                                              ; preds = %bb1
  %5 = bitcast { i8, i8 }* %_4 to i8*, !dbg !2355
  %6 = load i8, i8* %5, align 1, !dbg !2355, !range !436
  %7 = trunc i8 %6 to i1, !dbg !2355
  %_6 = zext i1 %7 to i32, !dbg !2355
  %8 = icmp eq i32 %_6, 1, !dbg !2355
  br i1 %8, label %bb3, label %bb5, !dbg !2355

bb3:                                              ; preds = %bb2
  %9 = bitcast { i8, i8 }* %_4 to %"core::option::Option<u8>::Some"*, !dbg !2356
  %10 = getelementptr inbounds %"core::option::Option<u8>::Some", %"core::option::Option<u8>::Some"* %9, i32 0, i32 1, !dbg !2356
  %x = load i8, i8* %10, align 1, !dbg !2356
  store i8 %x, i8* %x.dbg.spill, align 1, !dbg !2356
  call void @llvm.dbg.declare(metadata i8* %x.dbg.spill, metadata !2344, metadata !DIExpression()), !dbg !2357
  %11 = bitcast i8* %_10 to {}*, !dbg !2358
  store i8 %x, i8* %_10, align 1, !dbg !2358
  %12 = load i8, i8* %_10, align 1, !dbg !2358
; call core::iter::traits::iterator::Iterator::for_each::call::{{closure}}
  call void @"_ZN4core4iter6traits8iterator8Iterator8for_each4call28_$u7b$$u7b$closure$u7d$$u7d$17h27b028bfd3eed342E"(%"[closure@core::iter::traits::iterator::Iterator::for_each::call<u8, [closure@examples/locals.rs:10:1: 10:62]>::{closure#0}]"* nonnull align 1 %f, i8 %12) #10, !dbg !2358
  br label %bb4, !dbg !2358

bb5:                                              ; preds = %bb2
  br label %bb6, !dbg !2353

bb6:                                              ; preds = %bb5
  br label %bb7, !dbg !2359

bb7:                                              ; preds = %bb6
  br label %bb8, !dbg !2359

bb8:                                              ; preds = %bb7
  ret void, !dbg !2360

bb4:                                              ; preds = %bb3
  br label %bb1, !dbg !2353
}

; core::iter::traits::iterator::Iterator::fold
; Function Attrs: inlinehint nounwind
define internal void @_ZN4core4iter6traits8iterator8Iterator4fold17haea35f2c93648cf3E(i8 %0, i8 %1) unnamed_addr #1 !dbg !2361 {
start:
  %x.dbg.spill = alloca i8, align 1
  %init.dbg.spill = alloca {}, align 1
  %accum.dbg.spill = alloca {}, align 1
  %_10 = alloca i8, align 1
  %_4 = alloca { i8, i8 }, align 1
  %f = alloca %"[closure@core::iter::traits::iterator::Iterator::for_each::call<u8, [closure@examples/locals.rs:10:1: 10:62]>::{closure#0}].1", align 1
  %self = alloca { i8, i8 }, align 1
  %2 = getelementptr inbounds { i8, i8 }, { i8, i8 }* %self, i32 0, i32 0
  store i8 %0, i8* %2, align 1
  %3 = getelementptr inbounds { i8, i8 }, { i8, i8 }* %self, i32 0, i32 1
  store i8 %1, i8* %3, align 1
  call void @llvm.dbg.declare(metadata {}* %accum.dbg.spill, metadata !2368, metadata !DIExpression()), !dbg !2374
  call void @llvm.dbg.declare(metadata { i8, i8 }* %self, metadata !2365, metadata !DIExpression()), !dbg !2375
  call void @llvm.dbg.declare(metadata {}* %init.dbg.spill, metadata !2366, metadata !DIExpression()), !dbg !2376
  call void @llvm.dbg.declare(metadata %"[closure@core::iter::traits::iterator::Iterator::for_each::call<u8, [closure@examples/locals.rs:10:1: 10:62]>::{closure#0}].1"* %f, metadata !2367, metadata !DIExpression()), !dbg !2377
  br label %bb1, !dbg !2378

bb1:                                              ; preds = %bb4, %start
; call core::iter::range::<impl core::iter::traits::iterator::Iterator for core::ops::range::Range<A>>::next
  %4 = call { i8, i8 } @"_ZN4core4iter5range101_$LT$impl$u20$core..iter..traits..iterator..Iterator$u20$for$u20$core..ops..range..Range$LT$A$GT$$GT$4next17hcdb043e911fa495fE"({ i8, i8 }* align 1 dereferenceable(2) %self) #10, !dbg !2379
  store { i8, i8 } %4, { i8, i8 }* %_4, align 1, !dbg !2379
  br label %bb2, !dbg !2379

bb2:                                              ; preds = %bb1
  %5 = bitcast { i8, i8 }* %_4 to i8*, !dbg !2380
  %6 = load i8, i8* %5, align 1, !dbg !2380, !range !436
  %7 = trunc i8 %6 to i1, !dbg !2380
  %_6 = zext i1 %7 to i32, !dbg !2380
  %8 = icmp eq i32 %_6, 1, !dbg !2380
  br i1 %8, label %bb3, label %bb5, !dbg !2380

bb3:                                              ; preds = %bb2
  %9 = bitcast { i8, i8 }* %_4 to %"core::option::Option<u8>::Some"*, !dbg !2381
  %10 = getelementptr inbounds %"core::option::Option<u8>::Some", %"core::option::Option<u8>::Some"* %9, i32 0, i32 1, !dbg !2381
  %x = load i8, i8* %10, align 1, !dbg !2381
  store i8 %x, i8* %x.dbg.spill, align 1, !dbg !2381
  call void @llvm.dbg.declare(metadata i8* %x.dbg.spill, metadata !2370, metadata !DIExpression()), !dbg !2382
  %11 = bitcast i8* %_10 to {}*, !dbg !2383
  store i8 %x, i8* %_10, align 1, !dbg !2383
  %12 = load i8, i8* %_10, align 1, !dbg !2383
; call core::iter::traits::iterator::Iterator::for_each::call::{{closure}}
  call void @"_ZN4core4iter6traits8iterator8Iterator8for_each4call28_$u7b$$u7b$closure$u7d$$u7d$17h8120bf3118aae8a7E"(%"[closure@core::iter::traits::iterator::Iterator::for_each::call<u8, [closure@examples/locals.rs:10:1: 10:62]>::{closure#0}].1"* nonnull align 1 %f, i8 %12) #10, !dbg !2383
  br label %bb4, !dbg !2383

bb5:                                              ; preds = %bb2
  br label %bb6, !dbg !2378

bb6:                                              ; preds = %bb5
  br label %bb7, !dbg !2384

bb7:                                              ; preds = %bb6
  br label %bb8, !dbg !2384

bb8:                                              ; preds = %bb7
  ret void, !dbg !2385

bb4:                                              ; preds = %bb3
  br label %bb1, !dbg !2378
}

; core::result::Result<T,E>::unwrap
; Function Attrs: inlinehint nounwind
define internal void @"_ZN4core6result19Result$LT$T$C$E$GT$6unwrap17had281be05a095966E"(i1 zeroext %0, %"core::panic::location::Location"* align 4 dereferenceable(16) %1) unnamed_addr #1 !dbg !2386 {
start:
  %t.dbg.spill = alloca {}, align 1
  %e = alloca {}, align 1
  %self = alloca i8, align 1
  %2 = zext i1 %0 to i8
  store i8 %2, i8* %self, align 1
  call void @llvm.dbg.declare(metadata {}* %t.dbg.spill, metadata !2392, metadata !DIExpression()), !dbg !2398
  call void @llvm.dbg.declare(metadata i8* %self, metadata !2391, metadata !DIExpression()), !dbg !2399
  call void @llvm.dbg.declare(metadata {}* %e, metadata !2394, metadata !DIExpression()), !dbg !2400
  %3 = load i8, i8* %self, align 1, !dbg !2401, !range !436
  %4 = trunc i8 %3 to i1, !dbg !2401
  %_2 = zext i1 %4 to i32, !dbg !2401
  switch i32 %_2, label %bb2 [
    i32 0, label %bb3
    i32 1, label %bb1
  ], !dbg !2402

bb2:                                              ; preds = %start
  unreachable, !dbg !2401

bb3:                                              ; preds = %start
  ret void, !dbg !2403

bb1:                                              ; preds = %start
; call core::result::unwrap_failed
  call void @_ZN4core6result13unwrap_failed17h08c1d5e1a9b97c48E([0 x i8]* nonnull align 1 bitcast (<{ [43 x i8] }>* @alloc206 to [0 x i8]*), i32 43, {}* nonnull align 1 %e, [3 x i32]* align 4 dereferenceable(12) bitcast (<{ i8*, [8 x i8], i8*, [0 x i8] }>* @vtable.3 to [3 x i32]*), %"core::panic::location::Location"* align 4 dereferenceable(16) %1) #11, !dbg !2404
  unreachable, !dbg !2404
}

; core::sync::atomic::atomic_store
; Function Attrs: inlinehint nounwind
define internal void @_ZN4core4sync6atomic12atomic_store17hf215097f88ea7971E(i32* %dst, i32 %val, i8 %0) unnamed_addr #1 !dbg !2405 {
start:
  %val.dbg.spill = alloca i32, align 4
  %dst.dbg.spill = alloca i32*, align 4
  %order = alloca i8, align 1
  store i8 %0, i8* %order, align 1
  store i32* %dst, i32** %dst.dbg.spill, align 4
  call void @llvm.dbg.declare(metadata i32** %dst.dbg.spill, metadata !2411, metadata !DIExpression()), !dbg !2414
  store i32 %val, i32* %val.dbg.spill, align 4
  call void @llvm.dbg.declare(metadata i32* %val.dbg.spill, metadata !2412, metadata !DIExpression()), !dbg !2415
  call void @llvm.dbg.declare(metadata i8* %order, metadata !2413, metadata !DIExpression()), !dbg !2416
  %1 = load i8, i8* %order, align 1, !dbg !2417, !range !1439
  %_4 = zext i8 %1 to i32, !dbg !2417
  switch i32 %_4, label %bb2 [
    i32 0, label %bb5
    i32 1, label %bb3
    i32 2, label %bb9
    i32 3, label %bb1
    i32 4, label %bb7
  ], !dbg !2418

bb2:                                              ; preds = %start
  unreachable, !dbg !2417

bb5:                                              ; preds = %start
  store atomic i32 %val, i32* %dst monotonic, align 4, !dbg !2419
  br label %bb6, !dbg !2419

bb3:                                              ; preds = %start
  store atomic i32 %val, i32* %dst release, align 4, !dbg !2420
  br label %bb4, !dbg !2420

bb9:                                              ; preds = %start
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17h9e72682d8e5e8cf1E([0 x i8]* nonnull align 1 bitcast (<{ [42 x i8] }>* @alloc213 to [0 x i8]*), i32 42, %"core::panic::location::Location"* align 4 dereferenceable(16) bitcast (<{ i8*, [12 x i8] }>* @alloc215 to %"core::panic::location::Location"*)) #11, !dbg !2421
  unreachable, !dbg !2421

bb1:                                              ; preds = %start
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17h9e72682d8e5e8cf1E([0 x i8]* nonnull align 1 bitcast (<{ [50 x i8] }>* @alloc210 to [0 x i8]*), i32 50, %"core::panic::location::Location"* align 4 dereferenceable(16) bitcast (<{ i8*, [12 x i8] }>* @alloc212 to %"core::panic::location::Location"*)) #11, !dbg !2422
  unreachable, !dbg !2422

bb7:                                              ; preds = %start
  store atomic i32 %val, i32* %dst seq_cst, align 4, !dbg !2423
  br label %bb8, !dbg !2423

bb8:                                              ; preds = %bb7
  br label %bb10, !dbg !2424

bb10:                                             ; preds = %bb6, %bb4, %bb8
  ret void, !dbg !2425

bb4:                                              ; preds = %bb3
  br label %bb10, !dbg !2426

bb6:                                              ; preds = %bb5
  br label %bb10, !dbg !2427
}

; core::sync::atomic::atomic_load
; Function Attrs: inlinehint nounwind
define internal i32 @_ZN4core4sync6atomic11atomic_load17h774af4d42bd95514E(i32* %dst, i8 %0) unnamed_addr #1 !dbg !2428 {
start:
  %dst.dbg.spill = alloca i32*, align 4
  %1 = alloca i32, align 4
  %order = alloca i8, align 1
  store i8 %0, i8* %order, align 1
  store i32* %dst, i32** %dst.dbg.spill, align 4
  call void @llvm.dbg.declare(metadata i32** %dst.dbg.spill, metadata !2433, metadata !DIExpression()), !dbg !2435
  call void @llvm.dbg.declare(metadata i8* %order, metadata !2434, metadata !DIExpression()), !dbg !2436
  %2 = load i8, i8* %order, align 1, !dbg !2437, !range !1439
  %_3 = zext i8 %2 to i32, !dbg !2437
  switch i32 %_3, label %bb2 [
    i32 0, label %bb5
    i32 1, label %bb9
    i32 2, label %bb3
    i32 3, label %bb1
    i32 4, label %bb7
  ], !dbg !2438

bb2:                                              ; preds = %start
  unreachable, !dbg !2437

bb5:                                              ; preds = %start
  %3 = load atomic i32, i32* %dst monotonic, align 4, !dbg !2439
  store i32 %3, i32* %1, align 4, !dbg !2439
  br label %bb6, !dbg !2439

bb9:                                              ; preds = %start
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17h9e72682d8e5e8cf1E([0 x i8]* nonnull align 1 bitcast (<{ [40 x i8] }>* @alloc219 to [0 x i8]*), i32 40, %"core::panic::location::Location"* align 4 dereferenceable(16) bitcast (<{ i8*, [12 x i8] }>* @alloc221 to %"core::panic::location::Location"*)) #11, !dbg !2440
  unreachable, !dbg !2440

bb3:                                              ; preds = %start
  %4 = load atomic i32, i32* %dst acquire, align 4, !dbg !2441
  store i32 %4, i32* %1, align 4, !dbg !2441
  br label %bb4, !dbg !2441

bb1:                                              ; preds = %start
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17h9e72682d8e5e8cf1E([0 x i8]* nonnull align 1 bitcast (<{ [49 x i8] }>* @alloc216 to [0 x i8]*), i32 49, %"core::panic::location::Location"* align 4 dereferenceable(16) bitcast (<{ i8*, [12 x i8] }>* @alloc218 to %"core::panic::location::Location"*)) #11, !dbg !2442
  unreachable, !dbg !2442

bb7:                                              ; preds = %start
  %5 = load atomic i32, i32* %dst seq_cst, align 4, !dbg !2443
  store i32 %5, i32* %1, align 4, !dbg !2443
  br label %bb8, !dbg !2443

bb8:                                              ; preds = %bb7
  br label %bb10, !dbg !2444

bb10:                                             ; preds = %bb6, %bb4, %bb8
  %6 = load i32, i32* %1, align 4, !dbg !2445
  ret i32 %6, !dbg !2445

bb4:                                              ; preds = %bb3
  br label %bb10, !dbg !2446

bb6:                                              ; preds = %bb5
  br label %bb10, !dbg !2447
}

; core::fmt::ArgumentV1::new
; Function Attrs: nounwind
define internal { i8*, i32* } @_ZN4core3fmt10ArgumentV13new17h85113322f5b5e658E(i64** align 4 dereferenceable(4) %x, i1 (i64**, %"core::fmt::Formatter"*)* nonnull %f) unnamed_addr #0 !dbg !2448 {
start:
  %0 = alloca %"core::fmt::Opaque"*, align 4
  %1 = alloca i1 (%"core::fmt::Opaque"*, %"core::fmt::Formatter"*)*, align 4
  %f.dbg.spill = alloca i1 (i64**, %"core::fmt::Formatter"*)*, align 4
  %x.dbg.spill = alloca i64**, align 4
  %2 = alloca { i8*, i32* }, align 4
  store i64** %x, i64*** %x.dbg.spill, align 4
  call void @llvm.dbg.declare(metadata i64*** %x.dbg.spill, metadata !2491, metadata !DIExpression()), !dbg !2495
  store i1 (i64**, %"core::fmt::Formatter"*)* %f, i1 (i64**, %"core::fmt::Formatter"*)** %f.dbg.spill, align 4
  call void @llvm.dbg.declare(metadata i1 (i64**, %"core::fmt::Formatter"*)** %f.dbg.spill, metadata !2492, metadata !DIExpression()), !dbg !2496
  %3 = bitcast i1 (i64**, %"core::fmt::Formatter"*)* %f to i1 (%"core::fmt::Opaque"*, %"core::fmt::Formatter"*)*, !dbg !2497
  store i1 (%"core::fmt::Opaque"*, %"core::fmt::Formatter"*)* %3, i1 (%"core::fmt::Opaque"*, %"core::fmt::Formatter"*)** %1, align 4, !dbg !2497
  %_3 = load i1 (%"core::fmt::Opaque"*, %"core::fmt::Formatter"*)*, i1 (%"core::fmt::Opaque"*, %"core::fmt::Formatter"*)** %1, align 4, !dbg !2497, !nonnull !59
  br label %bb1, !dbg !2497

bb1:                                              ; preds = %start
  %4 = bitcast i64** %x to %"core::fmt::Opaque"*, !dbg !2498
  store %"core::fmt::Opaque"* %4, %"core::fmt::Opaque"** %0, align 4, !dbg !2498
  %_5 = load %"core::fmt::Opaque"*, %"core::fmt::Opaque"** %0, align 4, !dbg !2498, !nonnull !59
  br label %bb2, !dbg !2498

bb2:                                              ; preds = %bb1
  %5 = bitcast { i8*, i32* }* %2 to %"core::fmt::Opaque"**, !dbg !2499
  store %"core::fmt::Opaque"* %_5, %"core::fmt::Opaque"** %5, align 4, !dbg !2499
  %6 = getelementptr inbounds { i8*, i32* }, { i8*, i32* }* %2, i32 0, i32 1, !dbg !2499
  %7 = bitcast i32** %6 to i1 (%"core::fmt::Opaque"*, %"core::fmt::Formatter"*)**, !dbg !2499
  store i1 (%"core::fmt::Opaque"*, %"core::fmt::Formatter"*)* %_3, i1 (%"core::fmt::Opaque"*, %"core::fmt::Formatter"*)** %7, align 4, !dbg !2499
  %8 = getelementptr inbounds { i8*, i32* }, { i8*, i32* }* %2, i32 0, i32 0, !dbg !2500
  %9 = load i8*, i8** %8, align 4, !dbg !2500, !nonnull !59
  %10 = getelementptr inbounds { i8*, i32* }, { i8*, i32* }* %2, i32 0, i32 1, !dbg !2500
  %11 = load i32*, i32** %10, align 4, !dbg !2500, !nonnull !59
  %12 = insertvalue { i8*, i32* } undef, i8* %9, 0, !dbg !2500
  %13 = insertvalue { i8*, i32* } %12, i32* %11, 1, !dbg !2500
  ret { i8*, i32* } %13, !dbg !2500
}

; core::fmt::Arguments::new_v1
; Function Attrs: inlinehint nounwind
define internal void @_ZN4core3fmt9Arguments6new_v117hc132f62239bd6a9fE(%"core::fmt::Arguments"* noalias nocapture sret(%"core::fmt::Arguments") dereferenceable(24) %0, [0 x { [0 x i8]*, i32 }]* nonnull align 4 %pieces.0, i32 %pieces.1, [0 x { i8*, i32* }]* nonnull align 4 %args.0, i32 %args.1) unnamed_addr #1 !dbg !2501 {
start:
  %args.dbg.spill = alloca { [0 x { i8*, i32* }]*, i32 }, align 4
  %pieces.dbg.spill = alloca { [0 x { [0 x i8]*, i32 }]*, i32 }, align 4
  %_16 = alloca { i32*, i32 }, align 4
  %_3 = alloca i8, align 1
  %1 = getelementptr inbounds { [0 x { [0 x i8]*, i32 }]*, i32 }, { [0 x { [0 x i8]*, i32 }]*, i32 }* %pieces.dbg.spill, i32 0, i32 0
  store [0 x { [0 x i8]*, i32 }]* %pieces.0, [0 x { [0 x i8]*, i32 }]** %1, align 4
  %2 = getelementptr inbounds { [0 x { [0 x i8]*, i32 }]*, i32 }, { [0 x { [0 x i8]*, i32 }]*, i32 }* %pieces.dbg.spill, i32 0, i32 1
  store i32 %pieces.1, i32* %2, align 4
  call void @llvm.dbg.declare(metadata { [0 x { [0 x i8]*, i32 }]*, i32 }* %pieces.dbg.spill, metadata !2563, metadata !DIExpression()), !dbg !2565
  %3 = getelementptr inbounds { [0 x { i8*, i32* }]*, i32 }, { [0 x { i8*, i32* }]*, i32 }* %args.dbg.spill, i32 0, i32 0
  store [0 x { i8*, i32* }]* %args.0, [0 x { i8*, i32* }]** %3, align 4
  %4 = getelementptr inbounds { [0 x { i8*, i32* }]*, i32 }, { [0 x { i8*, i32* }]*, i32 }* %args.dbg.spill, i32 0, i32 1
  store i32 %args.1, i32* %4, align 4
  call void @llvm.dbg.declare(metadata { [0 x { i8*, i32* }]*, i32 }* %args.dbg.spill, metadata !2564, metadata !DIExpression()), !dbg !2566
  %_4 = icmp ult i32 %pieces.1, %args.1, !dbg !2567
  br i1 %_4, label %bb1, label %bb2, !dbg !2567

bb2:                                              ; preds = %start
  %_12 = add i32 %args.1, 1, !dbg !2568
  %_9 = icmp ugt i32 %pieces.1, %_12, !dbg !2569
  %5 = zext i1 %_9 to i8, !dbg !2567
  store i8 %5, i8* %_3, align 1, !dbg !2567
  br label %bb3, !dbg !2567

bb1:                                              ; preds = %start
  store i8 1, i8* %_3, align 1, !dbg !2567
  br label %bb3, !dbg !2567

bb3:                                              ; preds = %bb2, %bb1
  %6 = load i8, i8* %_3, align 1, !dbg !2567, !range !436
  %7 = trunc i8 %6 to i1, !dbg !2567
  br i1 %7, label %bb4, label %bb5, !dbg !2567

bb5:                                              ; preds = %bb3
  %8 = bitcast { i32*, i32 }* %_16 to i8*, !dbg !2570
  call void @llvm.memset.p0i8.i32(i8* align 4 %8, i8 0, i32 8, i1 false), !dbg !2570
  %9 = bitcast { i32*, i32 }* %_16 to {}**, !dbg !2570
  store {}* null, {}** %9, align 4, !dbg !2570
  %10 = bitcast %"core::fmt::Arguments"* %0 to { [0 x { [0 x i8]*, i32 }]*, i32 }*, !dbg !2571
  %11 = getelementptr inbounds { [0 x { [0 x i8]*, i32 }]*, i32 }, { [0 x { [0 x i8]*, i32 }]*, i32 }* %10, i32 0, i32 0, !dbg !2571
  store [0 x { [0 x i8]*, i32 }]* %pieces.0, [0 x { [0 x i8]*, i32 }]** %11, align 4, !dbg !2571
  %12 = getelementptr inbounds { [0 x { [0 x i8]*, i32 }]*, i32 }, { [0 x { [0 x i8]*, i32 }]*, i32 }* %10, i32 0, i32 1, !dbg !2571
  store i32 %pieces.1, i32* %12, align 4, !dbg !2571
  %13 = getelementptr inbounds %"core::fmt::Arguments", %"core::fmt::Arguments"* %0, i32 0, i32 1, !dbg !2571
  %14 = getelementptr inbounds { i32*, i32 }, { i32*, i32 }* %_16, i32 0, i32 0, !dbg !2571
  %15 = load i32*, i32** %14, align 4, !dbg !2571
  %16 = getelementptr inbounds { i32*, i32 }, { i32*, i32 }* %_16, i32 0, i32 1, !dbg !2571
  %17 = load i32, i32* %16, align 4, !dbg !2571
  %18 = getelementptr inbounds { i32*, i32 }, { i32*, i32 }* %13, i32 0, i32 0, !dbg !2571
  store i32* %15, i32** %18, align 4, !dbg !2571
  %19 = getelementptr inbounds { i32*, i32 }, { i32*, i32 }* %13, i32 0, i32 1, !dbg !2571
  store i32 %17, i32* %19, align 4, !dbg !2571
  %20 = getelementptr inbounds %"core::fmt::Arguments", %"core::fmt::Arguments"* %0, i32 0, i32 2, !dbg !2571
  %21 = getelementptr inbounds { [0 x { i8*, i32* }]*, i32 }, { [0 x { i8*, i32* }]*, i32 }* %20, i32 0, i32 0, !dbg !2571
  store [0 x { i8*, i32* }]* %args.0, [0 x { i8*, i32* }]** %21, align 4, !dbg !2571
  %22 = getelementptr inbounds { [0 x { i8*, i32* }]*, i32 }, { [0 x { i8*, i32* }]*, i32 }* %20, i32 0, i32 1, !dbg !2571
  store i32 %args.1, i32* %22, align 4, !dbg !2571
  ret void, !dbg !2572

bb4:                                              ; preds = %bb3
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17h9e72682d8e5e8cf1E([0 x i8]* nonnull align 1 bitcast (<{ [12 x i8] }>* @alloc222 to [0 x i8]*), i32 12, %"core::panic::location::Location"* align 4 dereferenceable(16) bitcast (<{ i8*, [12 x i8] }>* @alloc224 to %"core::panic::location::Location"*)) #11, !dbg !2573
  unreachable, !dbg !2573
}

; <() as core::fmt::Debug>::fmt
; Function Attrs: inlinehint nounwind
define internal zeroext i1 @"_ZN45_$LT$$LP$$RP$$u20$as$u20$core..fmt..Debug$GT$3fmt17h4cbe5cdd5d7bb3fbE"({}* nonnull align 1 %self, %"core::fmt::Formatter"* align 4 dereferenceable(36) %f) unnamed_addr #1 !dbg !2574 {
start:
  %f.dbg.spill = alloca %"core::fmt::Formatter"*, align 4
  %self.dbg.spill = alloca {}*, align 4
  store {}* %self, {}** %self.dbg.spill, align 4
  call void @llvm.dbg.declare(metadata {}** %self.dbg.spill, metadata !2579, metadata !DIExpression()), !dbg !2581
  store %"core::fmt::Formatter"* %f, %"core::fmt::Formatter"** %f.dbg.spill, align 4
  call void @llvm.dbg.declare(metadata %"core::fmt::Formatter"** %f.dbg.spill, metadata !2580, metadata !DIExpression()), !dbg !2582
; call core::fmt::Formatter::pad
  %0 = call zeroext i1 @_ZN4core3fmt9Formatter3pad17h28febf78b2c8a0a3E(%"core::fmt::Formatter"* align 4 dereferenceable(36) %f, [0 x i8]* nonnull align 1 bitcast (<{ [2 x i8] }>* @alloc225 to [0 x i8]*), i32 2) #10, !dbg !2583
  br label %bb1, !dbg !2583

bb1:                                              ; preds = %start
  ret i1 %0, !dbg !2584
}

; <usize as core::slice::index::SliceIndex<[T]>>::get_unchecked
; Function Attrs: inlinehint nounwind
define internal %"core::mem::maybe_uninit::MaybeUninit<()>"* @"_ZN75_$LT$usize$u20$as$u20$core..slice..index..SliceIndex$LT$$u5b$T$u5d$$GT$$GT$13get_unchecked17haf27b5046d34226dE"(i32 %self, [0 x %"core::mem::maybe_uninit::MaybeUninit<()>"]* %slice.0, i32 %slice.1) unnamed_addr #1 !dbg !2585 {
start:
  %0 = alloca %"core::mem::maybe_uninit::MaybeUninit<()>"*, align 4
  %count.dbg.spill.i.i = alloca i32, align 4
  %self.dbg.spill.i.i = alloca %"core::mem::maybe_uninit::MaybeUninit<()>"*, align 4
  %count.dbg.spill.i = alloca i32, align 4
  %self.dbg.spill.i = alloca %"core::mem::maybe_uninit::MaybeUninit<()>"*, align 4
  %slice.dbg.spill = alloca { [0 x %"core::mem::maybe_uninit::MaybeUninit<()>"]*, i32 }, align 4
  %self.dbg.spill = alloca i32, align 4
  store i32 %self, i32* %self.dbg.spill, align 4
  call void @llvm.dbg.declare(metadata i32* %self.dbg.spill, metadata !2593, metadata !DIExpression()), !dbg !2595
  %1 = getelementptr inbounds { [0 x %"core::mem::maybe_uninit::MaybeUninit<()>"]*, i32 }, { [0 x %"core::mem::maybe_uninit::MaybeUninit<()>"]*, i32 }* %slice.dbg.spill, i32 0, i32 0
  store [0 x %"core::mem::maybe_uninit::MaybeUninit<()>"]* %slice.0, [0 x %"core::mem::maybe_uninit::MaybeUninit<()>"]** %1, align 4
  %2 = getelementptr inbounds { [0 x %"core::mem::maybe_uninit::MaybeUninit<()>"]*, i32 }, { [0 x %"core::mem::maybe_uninit::MaybeUninit<()>"]*, i32 }* %slice.dbg.spill, i32 0, i32 1
  store i32 %slice.1, i32* %2, align 4
  call void @llvm.dbg.declare(metadata { [0 x %"core::mem::maybe_uninit::MaybeUninit<()>"]*, i32 }* %slice.dbg.spill, metadata !2594, metadata !DIExpression()), !dbg !2596
; call core::ptr::const_ptr::<impl *const [T]>::as_ptr
  %_3 = call %"core::mem::maybe_uninit::MaybeUninit<()>"* @"_ZN4core3ptr9const_ptr43_$LT$impl$u20$$BP$const$u20$$u5b$T$u5d$$GT$6as_ptr17hbe58f58226b6e7d4E"([0 x %"core::mem::maybe_uninit::MaybeUninit<()>"]* %slice.0, i32 %slice.1) #10, !dbg !2597
  br label %bb1, !dbg !2597

bb1:                                              ; preds = %start
  store %"core::mem::maybe_uninit::MaybeUninit<()>"* %_3, %"core::mem::maybe_uninit::MaybeUninit<()>"** %self.dbg.spill.i, align 4
  call void @llvm.dbg.declare(metadata %"core::mem::maybe_uninit::MaybeUninit<()>"** %self.dbg.spill.i, metadata !2598, metadata !DIExpression()), !dbg !2604
  store i32 %self, i32* %count.dbg.spill.i, align 4
  call void @llvm.dbg.declare(metadata i32* %count.dbg.spill.i, metadata !2603, metadata !DIExpression()), !dbg !2606
  store %"core::mem::maybe_uninit::MaybeUninit<()>"* %_3, %"core::mem::maybe_uninit::MaybeUninit<()>"** %self.dbg.spill.i.i, align 4
  call void @llvm.dbg.declare(metadata %"core::mem::maybe_uninit::MaybeUninit<()>"** %self.dbg.spill.i.i, metadata !2607, metadata !DIExpression()), !dbg !2613
  store i32 %self, i32* %count.dbg.spill.i.i, align 4
  call void @llvm.dbg.declare(metadata i32* %count.dbg.spill.i.i, metadata !2612, metadata !DIExpression()), !dbg !2615
  store %"core::mem::maybe_uninit::MaybeUninit<()>"* %_3, %"core::mem::maybe_uninit::MaybeUninit<()>"** %0, align 4, !dbg !2616
  %3 = load %"core::mem::maybe_uninit::MaybeUninit<()>"*, %"core::mem::maybe_uninit::MaybeUninit<()>"** %0, align 4, !dbg !2616
  br label %bb2, !dbg !2597

bb2:                                              ; preds = %bb1
  ret %"core::mem::maybe_uninit::MaybeUninit<()>"* %3, !dbg !2617
}

; <usize as core::slice::index::SliceIndex<[T]>>::get_unchecked
; Function Attrs: inlinehint nounwind
define internal { i8, i8 }* @"_ZN75_$LT$usize$u20$as$u20$core..slice..index..SliceIndex$LT$$u5b$T$u5d$$GT$$GT$13get_unchecked17hc7512e2bbf26379dE"(i32 %self, [0 x { i8, i8 }]* %slice.0, i32 %slice.1) unnamed_addr #1 !dbg !2618 {
start:
  %0 = alloca { i8, i8 }*, align 4
  %count.dbg.spill.i.i = alloca i32, align 4
  %self.dbg.spill.i.i = alloca { i8, i8 }*, align 4
  %count.dbg.spill.i = alloca i32, align 4
  %self.dbg.spill.i = alloca { i8, i8 }*, align 4
  %slice.dbg.spill = alloca { [0 x { i8, i8 }]*, i32 }, align 4
  %self.dbg.spill = alloca i32, align 4
  store i32 %self, i32* %self.dbg.spill, align 4
  call void @llvm.dbg.declare(metadata i32* %self.dbg.spill, metadata !2622, metadata !DIExpression()), !dbg !2624
  %1 = getelementptr inbounds { [0 x { i8, i8 }]*, i32 }, { [0 x { i8, i8 }]*, i32 }* %slice.dbg.spill, i32 0, i32 0
  store [0 x { i8, i8 }]* %slice.0, [0 x { i8, i8 }]** %1, align 4
  %2 = getelementptr inbounds { [0 x { i8, i8 }]*, i32 }, { [0 x { i8, i8 }]*, i32 }* %slice.dbg.spill, i32 0, i32 1
  store i32 %slice.1, i32* %2, align 4
  call void @llvm.dbg.declare(metadata { [0 x { i8, i8 }]*, i32 }* %slice.dbg.spill, metadata !2623, metadata !DIExpression()), !dbg !2625
; call core::ptr::const_ptr::<impl *const [T]>::as_ptr
  %_3 = call { i8, i8 }* @"_ZN4core3ptr9const_ptr43_$LT$impl$u20$$BP$const$u20$$u5b$T$u5d$$GT$6as_ptr17h8e58d8ff10279c42E"([0 x { i8, i8 }]* %slice.0, i32 %slice.1) #10, !dbg !2626
  br label %bb1, !dbg !2626

bb1:                                              ; preds = %start
  store { i8, i8 }* %_3, { i8, i8 }** %self.dbg.spill.i, align 4
  call void @llvm.dbg.declare(metadata { i8, i8 }** %self.dbg.spill.i, metadata !2627, metadata !DIExpression()), !dbg !2633
  store i32 %self, i32* %count.dbg.spill.i, align 4
  call void @llvm.dbg.declare(metadata i32* %count.dbg.spill.i, metadata !2632, metadata !DIExpression()), !dbg !2635
  store { i8, i8 }* %_3, { i8, i8 }** %self.dbg.spill.i.i, align 4
  call void @llvm.dbg.declare(metadata { i8, i8 }** %self.dbg.spill.i.i, metadata !2636, metadata !DIExpression()), !dbg !2642
  store i32 %self, i32* %count.dbg.spill.i.i, align 4
  call void @llvm.dbg.declare(metadata i32* %count.dbg.spill.i.i, metadata !2641, metadata !DIExpression()), !dbg !2644
  %3 = getelementptr inbounds { i8, i8 }, { i8, i8 }* %_3, i32 %self, !dbg !2645
  store { i8, i8 }* %3, { i8, i8 }** %0, align 4, !dbg !2645
  %4 = load { i8, i8 }*, { i8, i8 }** %0, align 4, !dbg !2645
  br label %bb2, !dbg !2626

bb2:                                              ; preds = %bb1
  ret { i8, i8 }* %4, !dbg !2646
}

; <usize as core::slice::index::SliceIndex<[T]>>::get_unchecked
; Function Attrs: inlinehint nounwind
define internal i8* @"_ZN75_$LT$usize$u20$as$u20$core..slice..index..SliceIndex$LT$$u5b$T$u5d$$GT$$GT$13get_unchecked17hfaefdf0e800697d2E"(i32 %self, [0 x i8]* %slice.0, i32 %slice.1) unnamed_addr #1 !dbg !2647 {
start:
  %0 = alloca i8*, align 4
  %count.dbg.spill.i.i = alloca i32, align 4
  %self.dbg.spill.i.i = alloca i8*, align 4
  %count.dbg.spill.i = alloca i32, align 4
  %self.dbg.spill.i = alloca i8*, align 4
  %slice.dbg.spill = alloca { [0 x i8]*, i32 }, align 4
  %self.dbg.spill = alloca i32, align 4
  store i32 %self, i32* %self.dbg.spill, align 4
  call void @llvm.dbg.declare(metadata i32* %self.dbg.spill, metadata !2651, metadata !DIExpression()), !dbg !2653
  %1 = getelementptr inbounds { [0 x i8]*, i32 }, { [0 x i8]*, i32 }* %slice.dbg.spill, i32 0, i32 0
  store [0 x i8]* %slice.0, [0 x i8]** %1, align 4
  %2 = getelementptr inbounds { [0 x i8]*, i32 }, { [0 x i8]*, i32 }* %slice.dbg.spill, i32 0, i32 1
  store i32 %slice.1, i32* %2, align 4
  call void @llvm.dbg.declare(metadata { [0 x i8]*, i32 }* %slice.dbg.spill, metadata !2652, metadata !DIExpression()), !dbg !2654
; call core::ptr::const_ptr::<impl *const [T]>::as_ptr
  %_3 = call i8* @"_ZN4core3ptr9const_ptr43_$LT$impl$u20$$BP$const$u20$$u5b$T$u5d$$GT$6as_ptr17h382d54ca28161cafE"([0 x i8]* %slice.0, i32 %slice.1) #10, !dbg !2655
  br label %bb1, !dbg !2655

bb1:                                              ; preds = %start
  store i8* %_3, i8** %self.dbg.spill.i, align 4
  call void @llvm.dbg.declare(metadata i8** %self.dbg.spill.i, metadata !2656, metadata !DIExpression()), !dbg !2662
  store i32 %self, i32* %count.dbg.spill.i, align 4
  call void @llvm.dbg.declare(metadata i32* %count.dbg.spill.i, metadata !2661, metadata !DIExpression()), !dbg !2664
  store i8* %_3, i8** %self.dbg.spill.i.i, align 4
  call void @llvm.dbg.declare(metadata i8** %self.dbg.spill.i.i, metadata !2665, metadata !DIExpression()), !dbg !2671
  store i32 %self, i32* %count.dbg.spill.i.i, align 4
  call void @llvm.dbg.declare(metadata i32* %count.dbg.spill.i.i, metadata !2670, metadata !DIExpression()), !dbg !2673
  %3 = getelementptr inbounds i8, i8* %_3, i32 %self, !dbg !2674
  store i8* %3, i8** %0, align 4, !dbg !2674
  %4 = load i8*, i8** %0, align 4, !dbg !2674
  br label %bb2, !dbg !2655

bb2:                                              ; preds = %bb1
  ret i8* %4, !dbg !2675
}

; <usize as core::slice::index::SliceIndex<[T]>>::get_unchecked_mut
; Function Attrs: inlinehint nounwind
define internal %"core::mem::maybe_uninit::MaybeUninit<()>"* @"_ZN75_$LT$usize$u20$as$u20$core..slice..index..SliceIndex$LT$$u5b$T$u5d$$GT$$GT$17get_unchecked_mut17he3cea9bf2e959781E"(i32 %self, [0 x %"core::mem::maybe_uninit::MaybeUninit<()>"]* %slice.0, i32 %slice.1) unnamed_addr #1 !dbg !2676 {
start:
  %self.dbg.spill.i1 = alloca { [0 x %"core::mem::maybe_uninit::MaybeUninit<()>"]*, i32 }, align 4
  %0 = alloca %"core::mem::maybe_uninit::MaybeUninit<()>"*, align 4
  %count.dbg.spill.i.i = alloca i32, align 4
  %self.dbg.spill.i.i = alloca %"core::mem::maybe_uninit::MaybeUninit<()>"*, align 4
  %count.dbg.spill.i = alloca i32, align 4
  %self.dbg.spill.i = alloca %"core::mem::maybe_uninit::MaybeUninit<()>"*, align 4
  %slice.dbg.spill = alloca { [0 x %"core::mem::maybe_uninit::MaybeUninit<()>"]*, i32 }, align 4
  %self.dbg.spill = alloca i32, align 4
  store i32 %self, i32* %self.dbg.spill, align 4
  call void @llvm.dbg.declare(metadata i32* %self.dbg.spill, metadata !2685, metadata !DIExpression()), !dbg !2687
  %1 = getelementptr inbounds { [0 x %"core::mem::maybe_uninit::MaybeUninit<()>"]*, i32 }, { [0 x %"core::mem::maybe_uninit::MaybeUninit<()>"]*, i32 }* %slice.dbg.spill, i32 0, i32 0
  store [0 x %"core::mem::maybe_uninit::MaybeUninit<()>"]* %slice.0, [0 x %"core::mem::maybe_uninit::MaybeUninit<()>"]** %1, align 4
  %2 = getelementptr inbounds { [0 x %"core::mem::maybe_uninit::MaybeUninit<()>"]*, i32 }, { [0 x %"core::mem::maybe_uninit::MaybeUninit<()>"]*, i32 }* %slice.dbg.spill, i32 0, i32 1
  store i32 %slice.1, i32* %2, align 4
  call void @llvm.dbg.declare(metadata { [0 x %"core::mem::maybe_uninit::MaybeUninit<()>"]*, i32 }* %slice.dbg.spill, metadata !2686, metadata !DIExpression()), !dbg !2688
  %3 = getelementptr inbounds { [0 x %"core::mem::maybe_uninit::MaybeUninit<()>"]*, i32 }, { [0 x %"core::mem::maybe_uninit::MaybeUninit<()>"]*, i32 }* %self.dbg.spill.i1, i32 0, i32 0
  store [0 x %"core::mem::maybe_uninit::MaybeUninit<()>"]* %slice.0, [0 x %"core::mem::maybe_uninit::MaybeUninit<()>"]** %3, align 4
  %4 = getelementptr inbounds { [0 x %"core::mem::maybe_uninit::MaybeUninit<()>"]*, i32 }, { [0 x %"core::mem::maybe_uninit::MaybeUninit<()>"]*, i32 }* %self.dbg.spill.i1, i32 0, i32 1
  store i32 %slice.1, i32* %4, align 4
  call void @llvm.dbg.declare(metadata { [0 x %"core::mem::maybe_uninit::MaybeUninit<()>"]*, i32 }* %self.dbg.spill.i1, metadata !2689, metadata !DIExpression()), !dbg !2695
  %5 = bitcast [0 x %"core::mem::maybe_uninit::MaybeUninit<()>"]* %slice.0 to %"core::mem::maybe_uninit::MaybeUninit<()>"*, !dbg !2697
  br label %bb1, !dbg !2698

bb1:                                              ; preds = %start
  store %"core::mem::maybe_uninit::MaybeUninit<()>"* %5, %"core::mem::maybe_uninit::MaybeUninit<()>"** %self.dbg.spill.i, align 4
  call void @llvm.dbg.declare(metadata %"core::mem::maybe_uninit::MaybeUninit<()>"** %self.dbg.spill.i, metadata !2699, metadata !DIExpression()), !dbg !2705
  store i32 %self, i32* %count.dbg.spill.i, align 4
  call void @llvm.dbg.declare(metadata i32* %count.dbg.spill.i, metadata !2704, metadata !DIExpression()), !dbg !2707
  store %"core::mem::maybe_uninit::MaybeUninit<()>"* %5, %"core::mem::maybe_uninit::MaybeUninit<()>"** %self.dbg.spill.i.i, align 4
  call void @llvm.dbg.declare(metadata %"core::mem::maybe_uninit::MaybeUninit<()>"** %self.dbg.spill.i.i, metadata !2708, metadata !DIExpression()), !dbg !2714
  store i32 %self, i32* %count.dbg.spill.i.i, align 4
  call void @llvm.dbg.declare(metadata i32* %count.dbg.spill.i.i, metadata !2713, metadata !DIExpression()), !dbg !2716
  store %"core::mem::maybe_uninit::MaybeUninit<()>"* %5, %"core::mem::maybe_uninit::MaybeUninit<()>"** %0, align 4, !dbg !2717
  %_3.i.i = load %"core::mem::maybe_uninit::MaybeUninit<()>"*, %"core::mem::maybe_uninit::MaybeUninit<()>"** %0, align 4, !dbg !2717
  br label %bb2, !dbg !2698

bb2:                                              ; preds = %bb1
  ret %"core::mem::maybe_uninit::MaybeUninit<()>"* %_3.i.i, !dbg !2718
}

; core::slice::<impl [T]>::get_unchecked
; Function Attrs: inlinehint nounwind
define internal align 1 dereferenceable(1) i8* @"_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$13get_unchecked17h92887aea7def6c01E"([0 x i8]* nonnull align 1 %self.0, i32 %self.1, i32 %index) unnamed_addr #1 !dbg !2719 {
start:
  %index.dbg.spill = alloca i32, align 4
  %self.dbg.spill = alloca { [0 x i8]*, i32 }, align 4
  %0 = getelementptr inbounds { [0 x i8]*, i32 }, { [0 x i8]*, i32 }* %self.dbg.spill, i32 0, i32 0
  store [0 x i8]* %self.0, [0 x i8]** %0, align 4
  %1 = getelementptr inbounds { [0 x i8]*, i32 }, { [0 x i8]*, i32 }* %self.dbg.spill, i32 0, i32 1
  store i32 %self.1, i32* %1, align 4
  call void @llvm.dbg.declare(metadata { [0 x i8]*, i32 }* %self.dbg.spill, metadata !2729, metadata !DIExpression()), !dbg !2733
  store i32 %index, i32* %index.dbg.spill, align 4
  call void @llvm.dbg.declare(metadata i32* %index.dbg.spill, metadata !2730, metadata !DIExpression()), !dbg !2734
; call <usize as core::slice::index::SliceIndex<[T]>>::get_unchecked
  %_3 = call i8* @"_ZN75_$LT$usize$u20$as$u20$core..slice..index..SliceIndex$LT$$u5b$T$u5d$$GT$$GT$13get_unchecked17hfaefdf0e800697d2E"(i32 %index, [0 x i8]* %self.0, i32 %self.1) #10, !dbg !2735
  br label %bb1, !dbg !2735

bb1:                                              ; preds = %start
  ret i8* %_3, !dbg !2736
}

; core::slice::<impl [T]>::get_unchecked
; Function Attrs: inlinehint nounwind
define internal align 1 dereferenceable(2) { i8, i8 }* @"_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$13get_unchecked17h990719cc23fef4feE"([0 x { i8, i8 }]* nonnull align 1 %self.0, i32 %self.1, i32 %index) unnamed_addr #1 !dbg !2737 {
start:
  %index.dbg.spill = alloca i32, align 4
  %self.dbg.spill = alloca { [0 x { i8, i8 }]*, i32 }, align 4
  %0 = getelementptr inbounds { [0 x { i8, i8 }]*, i32 }, { [0 x { i8, i8 }]*, i32 }* %self.dbg.spill, i32 0, i32 0
  store [0 x { i8, i8 }]* %self.0, [0 x { i8, i8 }]** %0, align 4
  %1 = getelementptr inbounds { [0 x { i8, i8 }]*, i32 }, { [0 x { i8, i8 }]*, i32 }* %self.dbg.spill, i32 0, i32 1
  store i32 %self.1, i32* %1, align 4
  call void @llvm.dbg.declare(metadata { [0 x { i8, i8 }]*, i32 }* %self.dbg.spill, metadata !2745, metadata !DIExpression()), !dbg !2748
  store i32 %index, i32* %index.dbg.spill, align 4
  call void @llvm.dbg.declare(metadata i32* %index.dbg.spill, metadata !2746, metadata !DIExpression()), !dbg !2749
; call <usize as core::slice::index::SliceIndex<[T]>>::get_unchecked
  %_3 = call { i8, i8 }* @"_ZN75_$LT$usize$u20$as$u20$core..slice..index..SliceIndex$LT$$u5b$T$u5d$$GT$$GT$13get_unchecked17hc7512e2bbf26379dE"(i32 %index, [0 x { i8, i8 }]* %self.0, i32 %self.1) #10, !dbg !2750
  br label %bb1, !dbg !2750

bb1:                                              ; preds = %start
  ret { i8, i8 }* %_3, !dbg !2751
}

; core::slice::<impl [T]>::get_unchecked
; Function Attrs: inlinehint nounwind
define internal nonnull align 1 %"core::mem::maybe_uninit::MaybeUninit<()>"* @"_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$13get_unchecked17hd2da507ba33e3510E"([0 x %"core::mem::maybe_uninit::MaybeUninit<()>"]* nonnull align 1 %self.0, i32 %self.1, i32 %index) unnamed_addr #1 !dbg !2752 {
start:
  %index.dbg.spill = alloca i32, align 4
  %self.dbg.spill = alloca { [0 x %"core::mem::maybe_uninit::MaybeUninit<()>"]*, i32 }, align 4
  %0 = getelementptr inbounds { [0 x %"core::mem::maybe_uninit::MaybeUninit<()>"]*, i32 }, { [0 x %"core::mem::maybe_uninit::MaybeUninit<()>"]*, i32 }* %self.dbg.spill, i32 0, i32 0
  store [0 x %"core::mem::maybe_uninit::MaybeUninit<()>"]* %self.0, [0 x %"core::mem::maybe_uninit::MaybeUninit<()>"]** %0, align 4
  %1 = getelementptr inbounds { [0 x %"core::mem::maybe_uninit::MaybeUninit<()>"]*, i32 }, { [0 x %"core::mem::maybe_uninit::MaybeUninit<()>"]*, i32 }* %self.dbg.spill, i32 0, i32 1
  store i32 %self.1, i32* %1, align 4
  call void @llvm.dbg.declare(metadata { [0 x %"core::mem::maybe_uninit::MaybeUninit<()>"]*, i32 }* %self.dbg.spill, metadata !2760, metadata !DIExpression()), !dbg !2763
  store i32 %index, i32* %index.dbg.spill, align 4
  call void @llvm.dbg.declare(metadata i32* %index.dbg.spill, metadata !2761, metadata !DIExpression()), !dbg !2764
; call <usize as core::slice::index::SliceIndex<[T]>>::get_unchecked
  %_3 = call %"core::mem::maybe_uninit::MaybeUninit<()>"* @"_ZN75_$LT$usize$u20$as$u20$core..slice..index..SliceIndex$LT$$u5b$T$u5d$$GT$$GT$13get_unchecked17haf27b5046d34226dE"(i32 %index, [0 x %"core::mem::maybe_uninit::MaybeUninit<()>"]* %self.0, i32 %self.1) #10, !dbg !2765
  br label %bb1, !dbg !2765

bb1:                                              ; preds = %start
  ret %"core::mem::maybe_uninit::MaybeUninit<()>"* %_3, !dbg !2766
}

; core::slice::<impl [T]>::get_unchecked_mut
; Function Attrs: inlinehint nounwind
define internal nonnull align 1 %"core::mem::maybe_uninit::MaybeUninit<()>"* @"_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$17get_unchecked_mut17hcb783b7fcc0d5eb1E"([0 x %"core::mem::maybe_uninit::MaybeUninit<()>"]* nonnull align 1 %self.0, i32 %self.1, i32 %index) unnamed_addr #1 !dbg !2767 {
start:
  %index.dbg.spill = alloca i32, align 4
  %self.dbg.spill = alloca { [0 x %"core::mem::maybe_uninit::MaybeUninit<()>"]*, i32 }, align 4
  %0 = getelementptr inbounds { [0 x %"core::mem::maybe_uninit::MaybeUninit<()>"]*, i32 }, { [0 x %"core::mem::maybe_uninit::MaybeUninit<()>"]*, i32 }* %self.dbg.spill, i32 0, i32 0
  store [0 x %"core::mem::maybe_uninit::MaybeUninit<()>"]* %self.0, [0 x %"core::mem::maybe_uninit::MaybeUninit<()>"]** %0, align 4
  %1 = getelementptr inbounds { [0 x %"core::mem::maybe_uninit::MaybeUninit<()>"]*, i32 }, { [0 x %"core::mem::maybe_uninit::MaybeUninit<()>"]*, i32 }* %self.dbg.spill, i32 0, i32 1
  store i32 %self.1, i32* %1, align 4
  call void @llvm.dbg.declare(metadata { [0 x %"core::mem::maybe_uninit::MaybeUninit<()>"]*, i32 }* %self.dbg.spill, metadata !2775, metadata !DIExpression()), !dbg !2777
  store i32 %index, i32* %index.dbg.spill, align 4
  call void @llvm.dbg.declare(metadata i32* %index.dbg.spill, metadata !2776, metadata !DIExpression()), !dbg !2778
; call <usize as core::slice::index::SliceIndex<[T]>>::get_unchecked_mut
  %_6 = call %"core::mem::maybe_uninit::MaybeUninit<()>"* @"_ZN75_$LT$usize$u20$as$u20$core..slice..index..SliceIndex$LT$$u5b$T$u5d$$GT$$GT$17get_unchecked_mut17he3cea9bf2e959781E"(i32 %index, [0 x %"core::mem::maybe_uninit::MaybeUninit<()>"]* %self.0, i32 %self.1) #10, !dbg !2779
  br label %bb1, !dbg !2779

bb1:                                              ; preds = %start
  ret %"core::mem::maybe_uninit::MaybeUninit<()>"* %_6, !dbg !2780
}

; core::clone::impls::<impl core::clone::Clone for u8>::clone
; Function Attrs: inlinehint nounwind
define internal i8 @"_ZN4core5clone5impls51_$LT$impl$u20$core..clone..Clone$u20$for$u20$u8$GT$5clone17h0bd6b78f87b73bf1E"(i8* align 1 dereferenceable(1) %self) unnamed_addr #1 !dbg !2781 {
start:
  %self.dbg.spill = alloca i8*, align 4
  store i8* %self, i8** %self.dbg.spill, align 4
  call void @llvm.dbg.declare(metadata i8** %self.dbg.spill, metadata !2789, metadata !DIExpression()), !dbg !2790
  %0 = load i8, i8* %self, align 1, !dbg !2791
  ret i8 %0, !dbg !2792
}

; core::cmp::impls::<impl core::cmp::PartialOrd for u8>::lt
; Function Attrs: inlinehint nounwind
define internal zeroext i1 @"_ZN4core3cmp5impls54_$LT$impl$u20$core..cmp..PartialOrd$u20$for$u20$u8$GT$2lt17h931a40755a640d7eE"(i8* align 1 dereferenceable(1) %self, i8* align 1 dereferenceable(1) %other) unnamed_addr #1 !dbg !2793 {
start:
  %other.dbg.spill = alloca i8*, align 4
  %self.dbg.spill = alloca i8*, align 4
  store i8* %self, i8** %self.dbg.spill, align 4
  call void @llvm.dbg.declare(metadata i8** %self.dbg.spill, metadata !2801, metadata !DIExpression()), !dbg !2803
  store i8* %other, i8** %other.dbg.spill, align 4
  call void @llvm.dbg.declare(metadata i8** %other.dbg.spill, metadata !2802, metadata !DIExpression()), !dbg !2804
  %_3 = load i8, i8* %self, align 1, !dbg !2805
  %_4 = load i8, i8* %other, align 1, !dbg !2806
  %0 = icmp ult i8 %_3, %_4, !dbg !2805
  ret i1 %0, !dbg !2807
}

; core::convert::num::<impl core::convert::From<u8> for usize>::from
; Function Attrs: inlinehint nounwind
define internal i32 @"_ZN4core7convert3num65_$LT$impl$u20$core..convert..From$LT$u8$GT$$u20$for$u20$usize$GT$4from17h5a3358a42c13f974E"(i8 %small) unnamed_addr #1 !dbg !2808 {
start:
  %small.dbg.spill = alloca i8, align 1
  store i8 %small, i8* %small.dbg.spill, align 1
  call void @llvm.dbg.declare(metadata i8* %small.dbg.spill, metadata !2815, metadata !DIExpression()), !dbg !2816
  %0 = zext i8 %small to i32, !dbg !2817
  ret i32 %0, !dbg !2818
}

; core::convert::num::<impl core::convert::From<u16> for usize>::from
; Function Attrs: inlinehint nounwind
define internal i32 @"_ZN4core7convert3num66_$LT$impl$u20$core..convert..From$LT$u16$GT$$u20$for$u20$usize$GT$4from17hb2e480a8ed69ba2dE"(i16 %small) unnamed_addr #1 !dbg !2819 {
start:
  %small.dbg.spill = alloca i16, align 2
  store i16 %small, i16* %small.dbg.spill, align 2
  call void @llvm.dbg.declare(metadata i16* %small.dbg.spill, metadata !2824, metadata !DIExpression()), !dbg !2825
  %0 = zext i16 %small to i32, !dbg !2826
  ret i32 %0, !dbg !2827
}

; <u8 as core::iter::range::Step>::forward_unchecked
; Function Attrs: inlinehint nounwind
define internal i8 @"_ZN46_$LT$u8$u20$as$u20$core..iter..range..Step$GT$17forward_unchecked17ha8d94eade2e46a9aE"(i8 %start1, i32 %n) unnamed_addr #1 !dbg !2828 {
start:
  %0 = alloca i8, align 1
  %rhs.dbg.spill.i = alloca i8, align 1
  %self.dbg.spill.i = alloca i8, align 1
  %n.dbg.spill = alloca i32, align 4
  %start.dbg.spill = alloca i8, align 1
  store i8 %start1, i8* %start.dbg.spill, align 1
  call void @llvm.dbg.declare(metadata i8* %start.dbg.spill, metadata !2833, metadata !DIExpression()), !dbg !2835
  store i32 %n, i32* %n.dbg.spill, align 4
  call void @llvm.dbg.declare(metadata i32* %n.dbg.spill, metadata !2834, metadata !DIExpression()), !dbg !2836
  %_4 = trunc i32 %n to i8, !dbg !2837
  store i8 %start1, i8* %self.dbg.spill.i, align 1
  call void @llvm.dbg.declare(metadata i8* %self.dbg.spill.i, metadata !2838, metadata !DIExpression()), !dbg !2843
  store i8 %_4, i8* %rhs.dbg.spill.i, align 1
  call void @llvm.dbg.declare(metadata i8* %rhs.dbg.spill.i, metadata !2842, metadata !DIExpression()), !dbg !2845
  %1 = add nuw i8 %start1, %_4, !dbg !2846
  store i8 %1, i8* %0, align 1, !dbg !2846
  %2 = load i8, i8* %0, align 1, !dbg !2846
  br label %bb1, !dbg !2847

bb1:                                              ; preds = %start
  ret i8 %2, !dbg !2848
}

; core::sync::atomic::AtomicUsize::load
; Function Attrs: inlinehint nounwind
define internal i32 @_ZN4core4sync6atomic11AtomicUsize4load17hbff932a919f9f519E(%"core::sync::atomic::AtomicUsize"* align 4 dereferenceable(4) %self, i8 %order) unnamed_addr #1 !dbg !2849 {
start:
  %self.dbg.spill.i = alloca i32*, align 4
  %order.dbg.spill = alloca i8, align 1
  %self.dbg.spill = alloca %"core::sync::atomic::AtomicUsize"*, align 4
  store %"core::sync::atomic::AtomicUsize"* %self, %"core::sync::atomic::AtomicUsize"** %self.dbg.spill, align 4
  call void @llvm.dbg.declare(metadata %"core::sync::atomic::AtomicUsize"** %self.dbg.spill, metadata !2854, metadata !DIExpression()), !dbg !2856
  store i8 %order, i8* %order.dbg.spill, align 1
  call void @llvm.dbg.declare(metadata i8* %order.dbg.spill, metadata !2855, metadata !DIExpression()), !dbg !2857
  %_5 = bitcast %"core::sync::atomic::AtomicUsize"* %self to i32*, !dbg !2858
  store i32* %_5, i32** %self.dbg.spill.i, align 4
  call void @llvm.dbg.declare(metadata i32** %self.dbg.spill.i, metadata !2859, metadata !DIExpression()), !dbg !2865
  br label %bb1, !dbg !2858

bb1:                                              ; preds = %start
; call core::sync::atomic::atomic_load
  %0 = call i32 @_ZN4core4sync6atomic11atomic_load17h774af4d42bd95514E(i32* %_5, i8 %order) #10, !dbg !2867
  br label %bb2, !dbg !2867

bb2:                                              ; preds = %bb1
  ret i32 %0, !dbg !2868
}

; core::sync::atomic::AtomicUsize::store
; Function Attrs: inlinehint nounwind
define internal void @_ZN4core4sync6atomic11AtomicUsize5store17h7e08c04f0f3a30f3E(%"core::sync::atomic::AtomicUsize"* align 4 dereferenceable(4) %self, i32 %val, i8 %order) unnamed_addr #1 !dbg !2869 {
start:
  %self.dbg.spill.i = alloca i32*, align 4
  %order.dbg.spill = alloca i8, align 1
  %val.dbg.spill = alloca i32, align 4
  %self.dbg.spill = alloca %"core::sync::atomic::AtomicUsize"*, align 4
  store %"core::sync::atomic::AtomicUsize"* %self, %"core::sync::atomic::AtomicUsize"** %self.dbg.spill, align 4
  call void @llvm.dbg.declare(metadata %"core::sync::atomic::AtomicUsize"** %self.dbg.spill, metadata !2873, metadata !DIExpression()), !dbg !2876
  store i32 %val, i32* %val.dbg.spill, align 4
  call void @llvm.dbg.declare(metadata i32* %val.dbg.spill, metadata !2874, metadata !DIExpression()), !dbg !2877
  store i8 %order, i8* %order.dbg.spill, align 1
  call void @llvm.dbg.declare(metadata i8* %order.dbg.spill, metadata !2875, metadata !DIExpression()), !dbg !2878
  %_6 = bitcast %"core::sync::atomic::AtomicUsize"* %self to i32*, !dbg !2879
  store i32* %_6, i32** %self.dbg.spill.i, align 4
  call void @llvm.dbg.declare(metadata i32** %self.dbg.spill.i, metadata !2859, metadata !DIExpression()), !dbg !2880
  br label %bb1, !dbg !2879

bb1:                                              ; preds = %start
; call core::sync::atomic::atomic_store
  call void @_ZN4core4sync6atomic12atomic_store17hf215097f88ea7971E(i32* %_6, i32 %val, i8 %order) #10, !dbg !2882
  br label %bb2, !dbg !2882

bb2:                                              ; preds = %bb1
  ret void, !dbg !2883
}

; <&mut T as core::fmt::Display>::fmt
; Function Attrs: nounwind
define internal zeroext i1 @"_ZN52_$LT$$RF$mut$u20$T$u20$as$u20$core..fmt..Display$GT$3fmt17h50bf9aa690e1dd41E"(i64** align 4 dereferenceable(4) %self, %"core::fmt::Formatter"* align 4 dereferenceable(36) %f) unnamed_addr #0 !dbg !2884 {
start:
  %f.dbg.spill = alloca %"core::fmt::Formatter"*, align 4
  %self.dbg.spill = alloca i64**, align 4
  store i64** %self, i64*** %self.dbg.spill, align 4
  call void @llvm.dbg.declare(metadata i64*** %self.dbg.spill, metadata !2887, metadata !DIExpression()), !dbg !2889
  store %"core::fmt::Formatter"* %f, %"core::fmt::Formatter"** %f.dbg.spill, align 4
  call void @llvm.dbg.declare(metadata %"core::fmt::Formatter"** %f.dbg.spill, metadata !2888, metadata !DIExpression()), !dbg !2890
  %_4 = load i64*, i64** %self, align 4, !dbg !2891, !nonnull !59
; call core::fmt::num::imp::<impl core::fmt::Display for i64>::fmt
  %0 = call zeroext i1 @"_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i64$GT$3fmt17he5b6e2cb3c6dc9c5E"(i64* align 8 dereferenceable(8) %_4, %"core::fmt::Formatter"* align 4 dereferenceable(36) %f) #10, !dbg !2892
  br label %bb1, !dbg !2892

bb1:                                              ; preds = %start
  ret i1 %0, !dbg !2893
}

; Function Attrs: nofree nosync nounwind readnone speculatable willreturn
declare void @llvm.dbg.declare(metadata, metadata, metadata) #4

; core::ptr::write_volatile
; Function Attrs: inlinehint nounwind
declare dso_local void @_ZN4core3ptr14write_volatile17h50533fac3ae79608E(i32*, i32) unnamed_addr #1

; core::ptr::write_volatile
; Function Attrs: inlinehint nounwind
declare dso_local void @_ZN4core3ptr14write_volatile17h3f979f4073c91a57E(i8*, i8) unnamed_addr #1

; Function Attrs: nounwind
declare dso_local void @__nop() unnamed_addr #0

; Function Attrs: argmemonly nofree nounwind willreturn
declare void @llvm.memcpy.p0i8.p0i8.i32(i8* noalias nocapture writeonly, i8* noalias nocapture readonly, i32, i1 immarg) #5

; Function Attrs: nofree nosync nounwind readnone speculatable willreturn
declare { i64, i1 } @llvm.sadd.with.overflow.i64(i64, i64) #4

; Function Attrs: nofree nosync nounwind readnone willreturn
declare i1 @llvm.expect.i1(i1, i1) #6

; core::panicking::panic
; Function Attrs: cold noinline noreturn nounwind
declare dso_local void @_ZN4core9panicking5panic17h9e72682d8e5e8cf1E([0 x i8]* nonnull align 1, i32, %"core::panic::location::Location"* align 4 dereferenceable(16)) unnamed_addr #7

; cortex_m_semihosting::export::hstdout_fmt
; Function Attrs: nounwind
declare dso_local zeroext i1 @_ZN20cortex_m_semihosting6export11hstdout_fmt17h247612a662c736f1E(%"core::fmt::Arguments"* noalias nocapture dereferenceable(24)) unnamed_addr #0

; cortex_m_semihosting::debug::exit
; Function Attrs: nounwind
declare dso_local void @_ZN20cortex_m_semihosting5debug4exit17h63eb89992f0c26a5E(i1 zeroext) unnamed_addr #0

; Function Attrs: nounwind
declare dso_local void @__cpsid() unnamed_addr #0

; Function Attrs: nounwind
declare dso_local void @__cpsie() unnamed_addr #0

; bare_metal::CriticalSection::new
; Function Attrs: nounwind
declare dso_local void @_ZN10bare_metal15CriticalSection3new17h77ebf15fa45305c2E() unnamed_addr #0

; Function Attrs: nofree nosync nounwind readnone speculatable willreturn
declare { i8, i1 } @llvm.usub.with.overflow.i8(i8, i8) #4

; core::panicking::panic_bounds_check
; Function Attrs: cold noinline noreturn nounwind
declare dso_local void @_ZN4core9panicking18panic_bounds_check17h5f129c306aeb31cbE(i32, i32, %"core::panic::location::Location"* align 4 dereferenceable(16)) unnamed_addr #7

; Function Attrs: nounwind
declare dso_local zeroext i8 @__basepri_r() unnamed_addr #0

; Function Attrs: nounwind
declare dso_local void @__basepri_w(i8 zeroext) unnamed_addr #0

; Function Attrs: nounwind
declare dso_local i32 @__primask_r() unnamed_addr #0

; Function Attrs: nofree nosync nounwind readnone speculatable willreturn
declare { i32, i1 } @llvm.uadd.with.overflow.i32(i32, i32) #4

; Function Attrs: argmemonly nofree nounwind willreturn writeonly
declare void @llvm.memset.p0i8.i32(i8* nocapture writeonly, i8, i32, i1 immarg) #8

; core::result::unwrap_failed
; Function Attrs: cold noinline noreturn nounwind
declare dso_local void @_ZN4core6result13unwrap_failed17h08c1d5e1a9b97c48E([0 x i8]* nonnull align 1, i32, {}* nonnull align 1, [3 x i32]* align 4 dereferenceable(12), %"core::panic::location::Location"* align 4 dereferenceable(16)) unnamed_addr #7

; core::fmt::Formatter::pad
; Function Attrs: nounwind
declare dso_local zeroext i1 @_ZN4core3fmt9Formatter3pad17h28febf78b2c8a0a3E(%"core::fmt::Formatter"* align 4 dereferenceable(36), [0 x i8]* nonnull align 1, i32) unnamed_addr #0

; core::fmt::num::imp::<impl core::fmt::Display for i64>::fmt
; Function Attrs: nounwind
declare dso_local zeroext i1 @"_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i64$GT$3fmt17he5b6e2cb3c6dc9c5E"(i64* align 8 dereferenceable(8), %"core::fmt::Formatter"* align 4 dereferenceable(36)) unnamed_addr #0

; Function Attrs: inaccessiblememonly nofree nosync nounwind willreturn
declare void @llvm.experimental.noalias.scope.decl(metadata) #9

attributes #0 = { nounwind "frame-pointer"="all" "target-cpu"="generic" }
attributes #1 = { inlinehint nounwind "frame-pointer"="all" "target-cpu"="generic" }
attributes #2 = { noreturn nounwind "frame-pointer"="all" "target-cpu"="generic" }
attributes #3 = { noinline nounwind "frame-pointer"="all" "target-cpu"="generic" }
attributes #4 = { nofree nosync nounwind readnone speculatable willreturn }
attributes #5 = { argmemonly nofree nounwind willreturn }
attributes #6 = { nofree nosync nounwind readnone willreturn }
attributes #7 = { cold noinline noreturn nounwind "frame-pointer"="all" "target-cpu"="generic" }
attributes #8 = { argmemonly nofree nounwind willreturn writeonly }
attributes #9 = { inaccessiblememonly nofree nosync nounwind willreturn }
attributes #10 = { nounwind }
attributes #11 = { noreturn nounwind }

!llvm.dbg.cu = !{!154}
!llvm.module.flags = !{!227}

!0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
!1 = distinct !DIGlobalVariable(name: "__rtic_internal_local_resource_local_to_foo", linkageName: "_ZN6locals3app43__rtic_internal_local_resource_local_to_foo17ha825634817032b38E", scope: !2, file: !4, line: 10, type: !5, isLocal: true, isDefinition: true, align: 8)
!2 = !DINamespace(name: "app", scope: !3)
!3 = !DINamespace(name: "locals", scope: null)
!4 = !DIFile(filename: "examples/locals.rs", directory: "/home/norlen/uni/sources/cortex-m-rtic", checksumkind: CSK_MD5, checksum: "a9eff1f810c7b7630bf20d2f1c3d083c")
!5 = !DICompositeType(tag: DW_TAG_structure_type, name: "RacyCell<core::mem::maybe_uninit::MaybeUninit<i64>>", scope: !7, file: !6, size: 64, align: 64, elements: !8, templateParams: !29, identifier: "f3a460e7ddc8709aff0bd984df1dc51a")
!6 = !DIFile(filename: "<unknown>", directory: "")
!7 = !DINamespace(name: "rtic", scope: null)
!8 = !{!9}
!9 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !5, file: !6, baseType: !10, size: 64, align: 64)
!10 = !DICompositeType(tag: DW_TAG_structure_type, name: "UnsafeCell<core::mem::maybe_uninit::MaybeUninit<i64>>", scope: !11, file: !6, size: 64, align: 64, elements: !13, templateParams: !29, identifier: "338c35fd2c7a9dfacfa195bad419bdf7")
!11 = !DINamespace(name: "cell", scope: !12)
!12 = !DINamespace(name: "core", scope: null)
!13 = !{!14}
!14 = !DIDerivedType(tag: DW_TAG_member, name: "value", scope: !10, file: !6, baseType: !15, size: 64, align: 64)
!15 = !DICompositeType(tag: DW_TAG_union_type, name: "MaybeUninit<i64>", scope: !16, file: !6, size: 64, align: 64, elements: !18, templateParams: !27, identifier: "dcba6ecf079e82ae9478d8848d55b985")
!16 = !DINamespace(name: "maybe_uninit", scope: !17)
!17 = !DINamespace(name: "mem", scope: !12)
!18 = !{!19, !21}
!19 = !DIDerivedType(tag: DW_TAG_member, name: "uninit", scope: !15, file: !6, baseType: !20, align: 8)
!20 = !DIBasicType(name: "()", encoding: DW_ATE_unsigned)
!21 = !DIDerivedType(tag: DW_TAG_member, name: "value", scope: !15, file: !6, baseType: !22, size: 64, align: 64)
!22 = !DICompositeType(tag: DW_TAG_structure_type, name: "ManuallyDrop<i64>", scope: !23, file: !6, size: 64, align: 64, elements: !24, templateParams: !27, identifier: "922eb3c54baf359d19552c6716722362")
!23 = !DINamespace(name: "manually_drop", scope: !17)
!24 = !{!25}
!25 = !DIDerivedType(tag: DW_TAG_member, name: "value", scope: !22, file: !6, baseType: !26, size: 64, align: 64)
!26 = !DIBasicType(name: "i64", size: 64, encoding: DW_ATE_signed)
!27 = !{!28}
!28 = !DITemplateTypeParameter(name: "T", type: !26)
!29 = !{!30}
!30 = !DITemplateTypeParameter(name: "T", type: !15)
!31 = !DIGlobalVariableExpression(var: !32, expr: !DIExpression())
!32 = distinct !DIGlobalVariable(name: "__rtic_internal_local_resource_local_to_bar", linkageName: "_ZN6locals3app43__rtic_internal_local_resource_local_to_bar17hedba4fee37a14ecaE", scope: !2, file: !4, line: 10, type: !5, isLocal: true, isDefinition: true, align: 8)
!33 = !DIGlobalVariableExpression(var: !34, expr: !DIExpression())
!34 = distinct !DIGlobalVariable(name: "__rtic_internal_local_resource_local_to_idle", linkageName: "_ZN6locals3app44__rtic_internal_local_resource_local_to_idle17h9592b26e73b21be8E", scope: !2, file: !4, line: 10, type: !5, isLocal: true, isDefinition: true, align: 8)
!35 = !DIGlobalVariableExpression(var: !36, expr: !DIExpression())
!36 = distinct !DIGlobalVariable(name: "__rtic_internal_foo_FQ", linkageName: "_ZN6locals3app22__rtic_internal_foo_FQ17heb020d3b7041f04fE", scope: !2, file: !4, line: 10, type: !37, isLocal: true, isDefinition: true, align: 4)
!37 = !DICompositeType(tag: DW_TAG_structure_type, name: "RacyCell<heapless::spsc::Queue<u8, 2>>", scope: !7, file: !6, size: 96, align: 32, elements: !38, templateParams: !80, identifier: "ef98794ce4b2c0469a9d4290bfe2b319")
!38 = !{!39}
!39 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !37, file: !6, baseType: !40, size: 96, align: 32)
!40 = !DICompositeType(tag: DW_TAG_structure_type, name: "UnsafeCell<heapless::spsc::Queue<u8, 2>>", scope: !11, file: !6, size: 96, align: 32, elements: !41, templateParams: !80, identifier: "dc190396b4915c0588a3a724ccb44209")
!41 = !{!42}
!42 = !DIDerivedType(tag: DW_TAG_member, name: "value", scope: !40, file: !6, baseType: !43, size: 96, align: 32)
!43 = !DICompositeType(tag: DW_TAG_structure_type, name: "Queue<u8, 2>", scope: !44, file: !6, size: 96, align: 32, elements: !46, templateParams: !74, identifier: "48174e9a96332f91ee19e82debce95df")
!44 = !DINamespace(name: "spsc", scope: !45)
!45 = !DINamespace(name: "heapless", scope: null)
!46 = !{!47, !60, !61}
!47 = !DIDerivedType(tag: DW_TAG_member, name: "head", scope: !43, file: !6, baseType: !48, size: 32, align: 32)
!48 = !DICompositeType(tag: DW_TAG_structure_type, name: "AtomicUsize", scope: !49, file: !6, size: 32, align: 32, elements: !51, templateParams: !59, identifier: "ae6f0b351364628532a83971e117c52")
!49 = !DINamespace(name: "atomic", scope: !50)
!50 = !DINamespace(name: "sync", scope: !12)
!51 = !{!52}
!52 = !DIDerivedType(tag: DW_TAG_member, name: "v", scope: !48, file: !6, baseType: !53, size: 32, align: 32)
!53 = !DICompositeType(tag: DW_TAG_structure_type, name: "UnsafeCell<usize>", scope: !11, file: !6, size: 32, align: 32, elements: !54, templateParams: !57, identifier: "d22c334f585332ca79a871ab0de4240")
!54 = !{!55}
!55 = !DIDerivedType(tag: DW_TAG_member, name: "value", scope: !53, file: !6, baseType: !56, size: 32, align: 32)
!56 = !DIBasicType(name: "usize", size: 32, encoding: DW_ATE_unsigned)
!57 = !{!58}
!58 = !DITemplateTypeParameter(name: "T", type: !56)
!59 = !{}
!60 = !DIDerivedType(tag: DW_TAG_member, name: "tail", scope: !43, file: !6, baseType: !48, size: 32, align: 32, offset: 32)
!61 = !DIDerivedType(tag: DW_TAG_member, name: "buffer", scope: !43, file: !6, baseType: !62, size: 16, align: 8, offset: 64)
!62 = !DICompositeType(tag: DW_TAG_array_type, baseType: !63, size: 16, align: 8, elements: !78)
!63 = !DICompositeType(tag: DW_TAG_structure_type, name: "UnsafeCell<core::mem::maybe_uninit::MaybeUninit<u8>>", scope: !11, file: !6, size: 8, align: 8, elements: !64, templateParams: !76, identifier: "26707ec6374903f6ec88be83c555a494")
!64 = !{!65}
!65 = !DIDerivedType(tag: DW_TAG_member, name: "value", scope: !63, file: !6, baseType: !66, size: 8, align: 8)
!66 = !DICompositeType(tag: DW_TAG_union_type, name: "MaybeUninit<u8>", scope: !16, file: !6, size: 8, align: 8, elements: !67, templateParams: !74, identifier: "8fdb67fca3e2d0f614fee4c03d91e5a")
!67 = !{!68, !69}
!68 = !DIDerivedType(tag: DW_TAG_member, name: "uninit", scope: !66, file: !6, baseType: !20, align: 8)
!69 = !DIDerivedType(tag: DW_TAG_member, name: "value", scope: !66, file: !6, baseType: !70, size: 8, align: 8)
!70 = !DICompositeType(tag: DW_TAG_structure_type, name: "ManuallyDrop<u8>", scope: !23, file: !6, size: 8, align: 8, elements: !71, templateParams: !74, identifier: "704c5e31da933e14ef0c0131430debe6")
!71 = !{!72}
!72 = !DIDerivedType(tag: DW_TAG_member, name: "value", scope: !70, file: !6, baseType: !73, size: 8, align: 8)
!73 = !DIBasicType(name: "u8", size: 8, encoding: DW_ATE_unsigned)
!74 = !{!75}
!75 = !DITemplateTypeParameter(name: "T", type: !73)
!76 = !{!77}
!77 = !DITemplateTypeParameter(name: "T", type: !66)
!78 = !{!79}
!79 = !DISubrange(count: 2, lowerBound: 0)
!80 = !{!81}
!81 = !DITemplateTypeParameter(name: "T", type: !43)
!82 = !DIGlobalVariableExpression(var: !83, expr: !DIExpression())
!83 = distinct !DIGlobalVariable(name: "__rtic_internal_foo_INPUTS", linkageName: "_ZN6locals3app26__rtic_internal_foo_INPUTS17hdf9dc66c5136e5d7E", scope: !2, file: !4, line: 10, type: !84, isLocal: true, isDefinition: true, align: 1)
!84 = !DICompositeType(tag: DW_TAG_structure_type, name: "RacyCell<[core::mem::maybe_uninit::MaybeUninit<()>; 1]>", scope: !7, file: !6, align: 8, elements: !85, templateParams: !102, identifier: "475fc2e223ccc83b1625b07d74aaa45")
!85 = !{!86}
!86 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !84, file: !6, baseType: !87, align: 8)
!87 = !DICompositeType(tag: DW_TAG_structure_type, name: "UnsafeCell<[core::mem::maybe_uninit::MaybeUninit<()>; 1]>", scope: !11, file: !6, align: 8, elements: !88, templateParams: !102, identifier: "965825b71bd406ed3b423cbdc246e41a")
!88 = !{!89}
!89 = !DIDerivedType(tag: DW_TAG_member, name: "value", scope: !87, file: !6, baseType: !90, align: 8)
!90 = !DICompositeType(tag: DW_TAG_array_type, baseType: !91, align: 8, elements: !100)
!91 = !DICompositeType(tag: DW_TAG_union_type, name: "MaybeUninit<()>", scope: !16, file: !6, align: 8, elements: !92, templateParams: !98, identifier: "39ebe3bea03df0573f7b5512069bafa3")
!92 = !{!93, !94}
!93 = !DIDerivedType(tag: DW_TAG_member, name: "uninit", scope: !91, file: !6, baseType: !20, align: 8)
!94 = !DIDerivedType(tag: DW_TAG_member, name: "value", scope: !91, file: !6, baseType: !95, align: 8)
!95 = !DICompositeType(tag: DW_TAG_structure_type, name: "ManuallyDrop<()>", scope: !23, file: !6, align: 8, elements: !96, templateParams: !98, identifier: "db2c8d946f2f7c0d25e8e52574c7343f")
!96 = !{!97}
!97 = !DIDerivedType(tag: DW_TAG_member, name: "value", scope: !95, file: !6, baseType: !20, align: 8)
!98 = !{!99}
!99 = !DITemplateTypeParameter(name: "T", type: !20)
!100 = !{!101}
!101 = !DISubrange(count: 1, lowerBound: 0)
!102 = !{!103}
!103 = !DITemplateTypeParameter(name: "T", type: !90)
!104 = !DIGlobalVariableExpression(var: !105, expr: !DIExpression())
!105 = distinct !DIGlobalVariable(name: "__rtic_internal_bar_FQ", linkageName: "_ZN6locals3app22__rtic_internal_bar_FQ17h2ab7573cfd764f60E", scope: !2, file: !4, line: 10, type: !37, isLocal: true, isDefinition: true, align: 4)
!106 = !DIGlobalVariableExpression(var: !107, expr: !DIExpression())
!107 = distinct !DIGlobalVariable(name: "__rtic_internal_bar_INPUTS", linkageName: "_ZN6locals3app26__rtic_internal_bar_INPUTS17h557822a7bd01f7e2E", scope: !2, file: !4, line: 10, type: !84, isLocal: true, isDefinition: true, align: 1)
!108 = !DIGlobalVariableExpression(var: !109, expr: !DIExpression())
!109 = distinct !DIGlobalVariable(name: "__rtic_internal_P1_RQ", linkageName: "_ZN6locals3app21__rtic_internal_P1_RQ17h70e79590858a2cecE", scope: !2, file: !4, line: 10, type: !110, isLocal: true, isDefinition: true, align: 4)
!110 = !DICompositeType(tag: DW_TAG_structure_type, name: "RacyCell<heapless::spsc::Queue<(locals::app::P1_T, u8), 3>>", scope: !7, file: !6, size: 128, align: 32, elements: !111, templateParams: !146, identifier: "6c51aab16c0fd683a1423b0ceb7e37f0")
!111 = !{!112}
!112 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !110, file: !6, baseType: !113, size: 128, align: 32)
!113 = !DICompositeType(tag: DW_TAG_structure_type, name: "UnsafeCell<heapless::spsc::Queue<(locals::app::P1_T, u8), 3>>", scope: !11, file: !6, size: 128, align: 32, elements: !114, templateParams: !146, identifier: "6077b6feeff063843bc04d6ea9d0dcd0")
!114 = !{!115}
!115 = !DIDerivedType(tag: DW_TAG_member, name: "value", scope: !113, file: !6, baseType: !116, size: 128, align: 32)
!116 = !DICompositeType(tag: DW_TAG_structure_type, name: "Queue<(locals::app::P1_T, u8), 3>", scope: !44, file: !6, size: 128, align: 32, elements: !117, templateParams: !140, identifier: "25644404983068acb49008045a3ed70e")
!117 = !{!118, !119, !120}
!118 = !DIDerivedType(tag: DW_TAG_member, name: "head", scope: !116, file: !6, baseType: !48, size: 32, align: 32)
!119 = !DIDerivedType(tag: DW_TAG_member, name: "tail", scope: !116, file: !6, baseType: !48, size: 32, align: 32, offset: 32)
!120 = !DIDerivedType(tag: DW_TAG_member, name: "buffer", scope: !116, file: !6, baseType: !121, size: 48, align: 8, offset: 64)
!121 = !DICompositeType(tag: DW_TAG_array_type, baseType: !122, size: 48, align: 8, elements: !144)
!122 = !DICompositeType(tag: DW_TAG_structure_type, name: "UnsafeCell<core::mem::maybe_uninit::MaybeUninit<(locals::app::P1_T, u8)>>", scope: !11, file: !6, size: 16, align: 8, elements: !123, templateParams: !142, identifier: "d35953b2d510a0b6980998c115b93523")
!123 = !{!124}
!124 = !DIDerivedType(tag: DW_TAG_member, name: "value", scope: !122, file: !6, baseType: !125, size: 16, align: 8)
!125 = !DICompositeType(tag: DW_TAG_union_type, name: "MaybeUninit<(locals::app::P1_T, u8)>", scope: !16, file: !6, size: 16, align: 8, elements: !126, templateParams: !140, identifier: "900875f234976fbcda3ae870ccac5cac")
!126 = !{!127, !128}
!127 = !DIDerivedType(tag: DW_TAG_member, name: "uninit", scope: !125, file: !6, baseType: !20, align: 8)
!128 = !DIDerivedType(tag: DW_TAG_member, name: "value", scope: !125, file: !6, baseType: !129, size: 16, align: 8)
!129 = !DICompositeType(tag: DW_TAG_structure_type, name: "ManuallyDrop<(locals::app::P1_T, u8)>", scope: !23, file: !6, size: 16, align: 8, elements: !130, templateParams: !140, identifier: "662a4efb7b8516c8d8467a4ada02c622")
!130 = !{!131}
!131 = !DIDerivedType(tag: DW_TAG_member, name: "value", scope: !129, file: !6, baseType: !132, size: 16, align: 8)
!132 = !DICompositeType(tag: DW_TAG_structure_type, name: "(locals::app::P1_T, u8)", file: !6, size: 16, align: 8, elements: !133, templateParams: !59, identifier: "9aae767cb68167baf3bcb45e552a8dd9")
!133 = !{!134, !139}
!134 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !132, file: !6, baseType: !135, size: 8, align: 8)
!135 = !DICompositeType(tag: DW_TAG_enumeration_type, name: "P1_T", scope: !2, file: !6, baseType: !73, size: 8, align: 8, flags: DIFlagEnumClass, elements: !136)
!136 = !{!137, !138}
!137 = !DIEnumerator(name: "bar", value: 0)
!138 = !DIEnumerator(name: "foo", value: 1)
!139 = !DIDerivedType(tag: DW_TAG_member, name: "__1", scope: !132, file: !6, baseType: !73, size: 8, align: 8, offset: 8)
!140 = !{!141}
!141 = !DITemplateTypeParameter(name: "T", type: !132)
!142 = !{!143}
!143 = !DITemplateTypeParameter(name: "T", type: !125)
!144 = !{!145}
!145 = !DISubrange(count: 3, lowerBound: 0)
!146 = !{!147}
!147 = !DITemplateTypeParameter(name: "T", type: !116)
!148 = !DIGlobalVariableExpression(var: !149, expr: !DIExpression())
!149 = distinct !DIGlobalVariable(name: "<() as core::fmt::Debug>::{vtable}", scope: null, file: !6, type: !150, isLocal: true, isDefinition: true)
!150 = !DICompositeType(tag: DW_TAG_array_type, baseType: !151, size: 128, align: 32, elements: !152)
!151 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*const ()", baseType: !20, size: 32, align: 32, dwarfAddressSpace: 0)
!152 = !{!153}
!153 = !DISubrange(count: 4, lowerBound: 0)
!154 = distinct !DICompileUnit(language: DW_LANG_Rust, file: !155, producer: "clang LLVM (rustc version 1.59.0 (9d1b2106e 2022-02-23))", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, enums: !156, globals: !226)
!155 = !DIFile(filename: "examples/locals.rs/@/locals.ebb8ff6e-cgu.0", directory: "/home/norlen/uni/sources/cortex-m-rtic")
!156 = !{!135, !157, !198, !203, !210, !217}
!157 = !DICompositeType(tag: DW_TAG_enumeration_type, name: "Interrupt", scope: !158, file: !6, baseType: !73, size: 8, align: 8, flags: DIFlagEnumClass, elements: !159)
!158 = !DINamespace(name: "lm3s6965", scope: null)
!159 = !{!160, !161, !162, !163, !164, !165, !166, !167, !168, !169, !170, !171, !172, !173, !174, !175, !176, !177, !178, !179, !180, !181, !182, !183, !184, !185, !186, !187, !188, !189, !190, !191, !192, !193, !194, !195, !196, !197}
!160 = !DIEnumerator(name: "GPIOA", value: 0)
!161 = !DIEnumerator(name: "GPIOB", value: 1)
!162 = !DIEnumerator(name: "GPIOC", value: 2)
!163 = !DIEnumerator(name: "GPIOD", value: 3)
!164 = !DIEnumerator(name: "GPIOE", value: 4)
!165 = !DIEnumerator(name: "UART0", value: 5)
!166 = !DIEnumerator(name: "UART1", value: 6)
!167 = !DIEnumerator(name: "SSI0", value: 7)
!168 = !DIEnumerator(name: "I2C0", value: 8)
!169 = !DIEnumerator(name: "PWM_FAULT", value: 9)
!170 = !DIEnumerator(name: "PWM_GENERATOR_0", value: 10)
!171 = !DIEnumerator(name: "PWM_GENERATOR_1", value: 11)
!172 = !DIEnumerator(name: "PWM_GENERATOR_2", value: 12)
!173 = !DIEnumerator(name: "QEI0", value: 13)
!174 = !DIEnumerator(name: "ADC0_SEQUENCE_0", value: 14)
!175 = !DIEnumerator(name: "ADC0_SEQUENCE_1", value: 15)
!176 = !DIEnumerator(name: "ADC0_SEQUENCE_2", value: 16)
!177 = !DIEnumerator(name: "ADC0_SEQUENCE_3", value: 17)
!178 = !DIEnumerator(name: "WATCHDOG_TIMER_0", value: 18)
!179 = !DIEnumerator(name: "TIMER_0A", value: 19)
!180 = !DIEnumerator(name: "TIMER_0B", value: 20)
!181 = !DIEnumerator(name: "TIMER_1A", value: 21)
!182 = !DIEnumerator(name: "TIMER_1B", value: 22)
!183 = !DIEnumerator(name: "TIMER_2A", value: 23)
!184 = !DIEnumerator(name: "TIMER_2B", value: 24)
!185 = !DIEnumerator(name: "ANALOG_COMPARATOR_0", value: 25)
!186 = !DIEnumerator(name: "ANALOG_COMPARATOR_1", value: 26)
!187 = !DIEnumerator(name: "SYSTEM_CONTROL", value: 27)
!188 = !DIEnumerator(name: "FLASH_MEMORY_CONTROL", value: 28)
!189 = !DIEnumerator(name: "GPIOF", value: 29)
!190 = !DIEnumerator(name: "GPIOG", value: 30)
!191 = !DIEnumerator(name: "UART2", value: 31)
!192 = !DIEnumerator(name: "TIMER_3A", value: 32)
!193 = !DIEnumerator(name: "TIMER_3B", value: 33)
!194 = !DIEnumerator(name: "I2C1", value: 34)
!195 = !DIEnumerator(name: "QEI1", value: 35)
!196 = !DIEnumerator(name: "ETHERNET", value: 36)
!197 = !DIEnumerator(name: "HIBERNATION", value: 37)
!198 = !DICompositeType(tag: DW_TAG_enumeration_type, name: "Result", scope: !199, file: !6, baseType: !73, size: 8, align: 8, flags: DIFlagEnumClass, elements: !200)
!199 = !DINamespace(name: "result", scope: !12)
!200 = !{!201, !202}
!201 = !DIEnumerator(name: "Ok", value: 0)
!202 = !DIEnumerator(name: "Err", value: 1)
!203 = !DICompositeType(tag: DW_TAG_enumeration_type, name: "Primask", scope: !204, file: !6, baseType: !73, size: 8, align: 8, flags: DIFlagEnumClass, elements: !207)
!204 = !DINamespace(name: "primask", scope: !205)
!205 = !DINamespace(name: "register", scope: !206)
!206 = !DINamespace(name: "cortex_m", scope: null)
!207 = !{!208, !209}
!208 = !DIEnumerator(name: "Active", value: 0)
!209 = !DIEnumerator(name: "Inactive", value: 1)
!210 = !DICompositeType(tag: DW_TAG_enumeration_type, name: "Ordering", scope: !49, file: !6, baseType: !73, size: 8, align: 8, flags: DIFlagEnumClass, elements: !211)
!211 = !{!212, !213, !214, !215, !216}
!212 = !DIEnumerator(name: "Relaxed", value: 0)
!213 = !DIEnumerator(name: "Release", value: 1)
!214 = !DIEnumerator(name: "Acquire", value: 2)
!215 = !DIEnumerator(name: "AcqRel", value: 3)
!216 = !DIEnumerator(name: "SeqCst", value: 4)
!217 = !DICompositeType(tag: DW_TAG_enumeration_type, name: "Alignment", scope: !218, file: !6, baseType: !73, size: 8, align: 8, flags: DIFlagEnumClass, elements: !221)
!218 = !DINamespace(name: "v1", scope: !219)
!219 = !DINamespace(name: "rt", scope: !220)
!220 = !DINamespace(name: "fmt", scope: !12)
!221 = !{!222, !223, !224, !225}
!222 = !DIEnumerator(name: "Left", value: 0)
!223 = !DIEnumerator(name: "Right", value: 1)
!224 = !DIEnumerator(name: "Center", value: 2)
!225 = !DIEnumerator(name: "Unknown", value: 3)
!226 = !{!0, !31, !33, !35, !82, !104, !106, !108, !148}
!227 = !{i32 2, !"Debug Info Version", i32 3}
!228 = distinct !DISubprogram(name: "drop_in_place<rtic::RacyCell<heapless::spsc::Queue<(locals::app::P1_T, u8), 3>>>", linkageName: "_ZN4core3ptr112drop_in_place$LT$rtic..RacyCell$LT$heapless..spsc..Queue$LT$$LP$locals..app..P1_T$C$u8$RP$$C$3_usize$GT$$GT$$GT$17hb274f222f9172188E", scope: !230, file: !229, line: 188, type: !231, scopeLine: 188, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !236, retainedNodes: !234)
!229 = !DIFile(filename: "/rustc/9d1b2106e23b1abd32fce1f17267604a5102f57a/library/core/src/ptr/mod.rs", directory: "", checksumkind: CSK_MD5, checksum: "0592540d568e762cd458f12fa2315832")
!230 = !DINamespace(name: "ptr", scope: !12)
!231 = !DISubroutineType(types: !232)
!232 = !{null, !233}
!233 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*mut RacyCell<heapless::spsc::Queue<(locals::app::P1_T, u8), 3>>", baseType: !110, size: 32, align: 32, dwarfAddressSpace: 0)
!234 = !{!235}
!235 = !DILocalVariable(arg: 1, scope: !228, file: !229, line: 188, type: !233)
!236 = !{!237}
!237 = !DITemplateTypeParameter(name: "T", type: !110)
!238 = !DILocation(line: 188, column: 1, scope: !228)
!239 = distinct !DISubprogram(name: "drop_in_place<core::cell::UnsafeCell<heapless::spsc::Queue<(locals::app::P1_T, u8), 3>>>", linkageName: "_ZN4core3ptr120drop_in_place$LT$core..cell..UnsafeCell$LT$heapless..spsc..Queue$LT$$LP$locals..app..P1_T$C$u8$RP$$C$3_usize$GT$$GT$$GT$17hf78b367adf48cdd4E", scope: !230, file: !229, line: 188, type: !240, scopeLine: 188, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !245, retainedNodes: !243)
!240 = !DISubroutineType(types: !241)
!241 = !{null, !242}
!242 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*mut UnsafeCell<heapless::spsc::Queue<(locals::app::P1_T, u8), 3>>", baseType: !113, size: 32, align: 32, dwarfAddressSpace: 0)
!243 = !{!244}
!244 = !DILocalVariable(arg: 1, scope: !239, file: !229, line: 188, type: !242)
!245 = !{!246}
!246 = !DITemplateTypeParameter(name: "T", type: !113)
!247 = !DILocation(line: 188, column: 1, scope: !239)
!248 = distinct !DISubprogram(name: "drop_in_place<()>", linkageName: "_ZN4core3ptr29drop_in_place$LT$$LP$$RP$$GT$17h69cf282f965e7cc1E", scope: !230, file: !229, line: 188, type: !249, scopeLine: 188, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !98, retainedNodes: !252)
!249 = !DISubroutineType(types: !250)
!250 = !{null, !251}
!251 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*mut ()", baseType: !20, size: 32, align: 32, dwarfAddressSpace: 0)
!252 = !{!253}
!253 = !DILocalVariable(arg: 1, scope: !248, file: !229, line: 188, type: !251)
!254 = !DILocation(line: 188, column: 1, scope: !248)
!255 = distinct !DISubprogram(name: "drop_in_place<heapless::spsc::Queue<u8, 2>>", linkageName: "_ZN4core3ptr62drop_in_place$LT$heapless..spsc..Queue$LT$u8$C$2_usize$GT$$GT$17h579993977b3e9a2eE", scope: !230, file: !229, line: 188, type: !256, scopeLine: 188, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !80, retainedNodes: !259)
!256 = !DISubroutineType(types: !257)
!257 = !{null, !258}
!258 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*mut Queue<u8, 2>", baseType: !43, size: 32, align: 32, dwarfAddressSpace: 0)
!259 = !{!260}
!260 = !DILocalVariable(arg: 1, scope: !255, file: !229, line: 188, type: !258)
!261 = !DILocation(line: 188, column: 1, scope: !255)
!262 = distinct !DISubprogram(name: "drop_in_place<rtic::RacyCell<heapless::spsc::Queue<u8, 2>>>", linkageName: "_ZN4core3ptr84drop_in_place$LT$rtic..RacyCell$LT$heapless..spsc..Queue$LT$u8$C$2_usize$GT$$GT$$GT$17h72e41ff8bc62b96bE", scope: !230, file: !229, line: 188, type: !263, scopeLine: 188, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !268, retainedNodes: !266)
!263 = !DISubroutineType(types: !264)
!264 = !{null, !265}
!265 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*mut RacyCell<heapless::spsc::Queue<u8, 2>>", baseType: !37, size: 32, align: 32, dwarfAddressSpace: 0)
!266 = !{!267}
!267 = !DILocalVariable(arg: 1, scope: !262, file: !229, line: 188, type: !265)
!268 = !{!269}
!269 = !DITemplateTypeParameter(name: "T", type: !37)
!270 = !DILocation(line: 188, column: 1, scope: !262)
!271 = distinct !DISubprogram(name: "drop_in_place<heapless::spsc::Queue<(locals::app::P1_T, u8), 3>>", linkageName: "_ZN4core3ptr90drop_in_place$LT$heapless..spsc..Queue$LT$$LP$locals..app..P1_T$C$u8$RP$$C$3_usize$GT$$GT$17he5437f9cfde940d1E", scope: !230, file: !229, line: 188, type: !272, scopeLine: 188, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !146, retainedNodes: !275)
!272 = !DISubroutineType(types: !273)
!273 = !{null, !274}
!274 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*mut Queue<(locals::app::P1_T, u8), 3>", baseType: !116, size: 32, align: 32, dwarfAddressSpace: 0)
!275 = !{!276}
!276 = !DILocalVariable(arg: 1, scope: !271, file: !229, line: 188, type: !274)
!277 = !DILocation(line: 188, column: 1, scope: !271)
!278 = distinct !DISubprogram(name: "drop_in_place<core::cell::UnsafeCell<heapless::spsc::Queue<u8, 2>>>", linkageName: "_ZN4core3ptr92drop_in_place$LT$core..cell..UnsafeCell$LT$heapless..spsc..Queue$LT$u8$C$2_usize$GT$$GT$$GT$17h049b29bad89ea28fE", scope: !230, file: !229, line: 188, type: !279, scopeLine: 188, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !284, retainedNodes: !282)
!279 = !DISubroutineType(types: !280)
!280 = !{null, !281}
!281 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*mut UnsafeCell<heapless::spsc::Queue<u8, 2>>", baseType: !40, size: 32, align: 32, dwarfAddressSpace: 0)
!282 = !{!283}
!283 = !DILocalVariable(arg: 1, scope: !278, file: !229, line: 188, type: !281)
!284 = !{!285}
!285 = !DITemplateTypeParameter(name: "T", type: !40)
!286 = !DILocation(line: 188, column: 1, scope: !278)
!287 = distinct !DISubprogram(name: "nr", linkageName: "_ZN54_$LT$lm3s6965..Interrupt$u20$as$u20$bare_metal..Nr$GT$2nr17h9796bf85b9167b29E", scope: !289, file: !288, line: 121, type: !290, scopeLine: 121, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !59, retainedNodes: !293)
!288 = !DIFile(filename: "/home/norlen/.cargo/git/checkouts/lm3s6965-70a7c9508b2d523e/facf63a/src/lib.rs", directory: "", checksumkind: CSK_MD5, checksum: "26c595941cfa78f913d0a19159eb5908")
!289 = !DINamespace(name: "{impl#0}", scope: !158)
!290 = !DISubroutineType(types: !291)
!291 = !{!73, !292}
!292 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&Interrupt", baseType: !157, size: 32, align: 32, dwarfAddressSpace: 0)
!293 = !{!294}
!294 = !DILocalVariable(name: "self", arg: 1, scope: !287, file: !288, line: 121, type: !292)
!295 = !DILocation(line: 121, column: 11, scope: !287)
!296 = !DILocation(line: 122, column: 15, scope: !287)
!297 = !{i8 0, i8 38}
!298 = !DILocation(line: 122, column: 9, scope: !287)
!299 = !DILocation(line: 123, column: 33, scope: !287)
!300 = !DILocation(line: 124, column: 33, scope: !287)
!301 = !DILocation(line: 125, column: 33, scope: !287)
!302 = !DILocation(line: 126, column: 33, scope: !287)
!303 = !DILocation(line: 127, column: 33, scope: !287)
!304 = !DILocation(line: 128, column: 33, scope: !287)
!305 = !DILocation(line: 129, column: 33, scope: !287)
!306 = !DILocation(line: 130, column: 32, scope: !287)
!307 = !DILocation(line: 131, column: 32, scope: !287)
!308 = !DILocation(line: 132, column: 37, scope: !287)
!309 = !DILocation(line: 133, column: 43, scope: !287)
!310 = !DILocation(line: 134, column: 43, scope: !287)
!311 = !DILocation(line: 135, column: 43, scope: !287)
!312 = !DILocation(line: 136, column: 32, scope: !287)
!313 = !DILocation(line: 137, column: 43, scope: !287)
!314 = !DILocation(line: 138, column: 43, scope: !287)
!315 = !DILocation(line: 139, column: 43, scope: !287)
!316 = !DILocation(line: 140, column: 43, scope: !287)
!317 = !DILocation(line: 141, column: 44, scope: !287)
!318 = !DILocation(line: 142, column: 36, scope: !287)
!319 = !DILocation(line: 143, column: 36, scope: !287)
!320 = !DILocation(line: 144, column: 36, scope: !287)
!321 = !DILocation(line: 145, column: 36, scope: !287)
!322 = !DILocation(line: 146, column: 36, scope: !287)
!323 = !DILocation(line: 147, column: 36, scope: !287)
!324 = !DILocation(line: 148, column: 47, scope: !287)
!325 = !DILocation(line: 149, column: 47, scope: !287)
!326 = !DILocation(line: 151, column: 42, scope: !287)
!327 = !DILocation(line: 152, column: 48, scope: !287)
!328 = !DILocation(line: 153, column: 33, scope: !287)
!329 = !DILocation(line: 154, column: 33, scope: !287)
!330 = !DILocation(line: 157, column: 33, scope: !287)
!331 = !DILocation(line: 160, column: 36, scope: !287)
!332 = !DILocation(line: 162, column: 36, scope: !287)
!333 = !DILocation(line: 164, column: 32, scope: !287)
!334 = !DILocation(line: 166, column: 32, scope: !287)
!335 = !DILocation(line: 169, column: 36, scope: !287)
!336 = !DILocation(line: 171, column: 39, scope: !287)
!337 = !DILocation(line: 173, column: 6, scope: !287)
!338 = distinct !DISubprogram(name: "nop", linkageName: "_ZN8cortex_m3asm3nop17he6e4dcc664e4aa97E", scope: !340, file: !339, line: 34, type: !341, scopeLine: 34, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !59, retainedNodes: !59)
!339 = !DIFile(filename: "/home/norlen/.cargo/registry/src/github.com-1ecc6299db9ec823/cortex-m-0.7.4/src/asm.rs", directory: "", checksumkind: CSK_MD5, checksum: "fe9d6d5d92db517d6ea0170311854875")
!340 = !DINamespace(name: "asm", scope: !206)
!341 = !DISubroutineType(types: !342)
!342 = !{null}
!343 = !DILocation(line: 35, column: 5, scope: !338)
!344 = !DILocation(line: 36, column: 2, scope: !338)
!345 = distinct !DISubprogram(name: "idle", linkageName: "_ZN6locals3app4idle17h10dbad5f387d0984E", scope: !2, file: !4, line: 10, type: !346, scopeLine: 10, flags: DIFlagPrototyped | DIFlagNoReturn, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !59, retainedNodes: !355)
!346 = !DISubroutineType(types: !347)
!347 = !{null, !348}
!348 = !DICompositeType(tag: DW_TAG_structure_type, name: "__rtic_internal_idle_Context", scope: !2, file: !6, size: 32, align: 32, elements: !349, templateParams: !59, identifier: "ed2a6ff63bf16e7bd4d4b8e96a7b70a4")
!349 = !{!350}
!350 = !DIDerivedType(tag: DW_TAG_member, name: "local", scope: !348, file: !6, baseType: !351, size: 32, align: 32)
!351 = !DICompositeType(tag: DW_TAG_structure_type, name: "__rtic_internal_idleLocalResources", scope: !2, file: !6, size: 32, align: 32, elements: !352, templateParams: !59, identifier: "eb85dca178d6dc3aee781e25102c6b3c")
!352 = !{!353}
!353 = !DIDerivedType(tag: DW_TAG_member, name: "local_to_idle", scope: !351, file: !6, baseType: !354, size: 32, align: 32)
!354 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&mut i64", baseType: !26, size: 32, align: 32, dwarfAddressSpace: 0)
!355 = !{!356, !357, !359}
!356 = !DILocalVariable(name: "cx", arg: 1, scope: !345, file: !4, line: 44, type: !348)
!357 = !DILocalVariable(name: "local_to_idle", scope: !358, file: !4, line: 45, type: !354, align: 4)
!358 = distinct !DILexicalBlock(scope: !345, file: !4, line: 10, column: 1)
!359 = !DILocalVariable(name: "_args", scope: !360, file: !4, line: 48, type: !363, align: 4)
!360 = !DILexicalBlockFile(scope: !361, file: !4, discriminator: 0)
!361 = distinct !DILexicalBlock(scope: !358, file: !362, line: 56, column: 37)
!362 = !DIFile(filename: "/home/norlen/.cargo/registry/src/github.com-1ecc6299db9ec823/cortex-m-semihosting-0.3.7/src/macros.rs", directory: "", checksumkind: CSK_MD5, checksum: "cc22f0eda8963bf086d6da07906ff946")
!363 = !DICompositeType(tag: DW_TAG_structure_type, name: "(&&mut i64)", file: !6, size: 32, align: 32, elements: !364, templateParams: !59, identifier: "afaab1be9b2691ec15d425ca194a81ee")
!364 = !{!365}
!365 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !363, file: !6, baseType: !366, size: 32, align: 32)
!366 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&&mut i64", baseType: !354, size: 32, align: 32, dwarfAddressSpace: 0)
!367 = !DILocation(line: 44, column: 13, scope: !345)
!368 = !DILocation(line: 45, column: 13, scope: !358)
!369 = !DILocation(line: 45, column: 29, scope: !345)
!370 = !DILocation(line: 46, column: 9, scope: !358)
!371 = !DILocation(line: 48, column: 9, scope: !358)
!372 = !DILocation(line: 48, column: 9, scope: !360)
!373 = !DILocation(line: 50, column: 9, scope: !358)
!374 = !DILocation(line: 59, column: 13, scope: !358)
!375 = distinct !DISubprogram(name: "foo", linkageName: "_ZN6locals3app3foo17ha55c0f8edfab0f06E", scope: !2, file: !4, line: 10, type: !376, scopeLine: 10, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !59, retainedNodes: !384)
!376 = !DISubroutineType(types: !377)
!377 = !{null, !378}
!378 = !DICompositeType(tag: DW_TAG_structure_type, name: "__rtic_internal_foo_Context", scope: !2, file: !6, size: 32, align: 32, elements: !379, templateParams: !59, identifier: "1800876ae343ac72fecd301469dc01c3")
!379 = !{!380}
!380 = !DIDerivedType(tag: DW_TAG_member, name: "local", scope: !378, file: !6, baseType: !381, size: 32, align: 32)
!381 = !DICompositeType(tag: DW_TAG_structure_type, name: "__rtic_internal_fooLocalResources", scope: !2, file: !6, size: 32, align: 32, elements: !382, templateParams: !59, identifier: "f553899c8f0f7979848cfb3b1abf023d")
!382 = !{!383}
!383 = !DIDerivedType(tag: DW_TAG_member, name: "local_to_foo", scope: !381, file: !6, baseType: !354, size: 32, align: 32)
!384 = !{!385, !386, !388}
!385 = !DILocalVariable(name: "cx", arg: 1, scope: !375, file: !4, line: 65, type: !378)
!386 = !DILocalVariable(name: "local_to_foo", scope: !387, file: !4, line: 66, type: !354, align: 4)
!387 = distinct !DILexicalBlock(scope: !375, file: !4, line: 10, column: 1)
!388 = !DILocalVariable(name: "_args", scope: !389, file: !4, line: 72, type: !363, align: 4)
!389 = !DILexicalBlockFile(scope: !390, file: !4, discriminator: 0)
!390 = distinct !DILexicalBlock(scope: !387, file: !362, line: 56, column: 37)
!391 = !DILocation(line: 65, column: 12, scope: !375)
!392 = !DILocation(line: 66, column: 13, scope: !387)
!393 = !DILocation(line: 66, column: 28, scope: !375)
!394 = !DILocation(line: 67, column: 9, scope: !387)
!395 = !DILocation(line: 72, column: 9, scope: !387)
!396 = !DILocation(line: 72, column: 9, scope: !389)
!397 = !DILocation(line: 10, column: 62, scope: !375)
!398 = distinct !DISubprogram(name: "bar", linkageName: "_ZN6locals3app3bar17h3dfc1d35d287dc45E", scope: !2, file: !4, line: 10, type: !399, scopeLine: 10, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !59, retainedNodes: !407)
!399 = !DISubroutineType(types: !400)
!400 = !{null, !401}
!401 = !DICompositeType(tag: DW_TAG_structure_type, name: "__rtic_internal_bar_Context", scope: !2, file: !6, size: 32, align: 32, elements: !402, templateParams: !59, identifier: "b11a2b8b938359b063497446511bc527")
!402 = !{!403}
!403 = !DIDerivedType(tag: DW_TAG_member, name: "local", scope: !401, file: !6, baseType: !404, size: 32, align: 32)
!404 = !DICompositeType(tag: DW_TAG_structure_type, name: "__rtic_internal_barLocalResources", scope: !2, file: !6, size: 32, align: 32, elements: !405, templateParams: !59, identifier: "1e2e047f0527c45a4439a802be524be2")
!405 = !{!406}
!406 = !DIDerivedType(tag: DW_TAG_member, name: "local_to_bar", scope: !404, file: !6, baseType: !354, size: 32, align: 32)
!407 = !{!408, !409, !411}
!408 = !DILocalVariable(name: "cx", arg: 1, scope: !398, file: !4, line: 77, type: !401)
!409 = !DILocalVariable(name: "local_to_bar", scope: !410, file: !4, line: 78, type: !354, align: 4)
!410 = distinct !DILexicalBlock(scope: !398, file: !4, line: 10, column: 1)
!411 = !DILocalVariable(name: "_args", scope: !412, file: !4, line: 84, type: !363, align: 4)
!412 = !DILexicalBlockFile(scope: !413, file: !4, discriminator: 0)
!413 = distinct !DILexicalBlock(scope: !410, file: !362, line: 56, column: 37)
!414 = !DILocation(line: 77, column: 12, scope: !398)
!415 = !DILocation(line: 78, column: 13, scope: !410)
!416 = !DILocation(line: 78, column: 28, scope: !398)
!417 = !DILocation(line: 79, column: 9, scope: !410)
!418 = !DILocation(line: 84, column: 9, scope: !410)
!419 = !DILocation(line: 84, column: 9, scope: !412)
!420 = !DILocation(line: 10, column: 62, scope: !398)
!421 = distinct !DISubprogram(name: "steal", linkageName: "_ZN8lm3s696511Peripherals5steal17h618f0739b52a798eE", scope: !422, file: !288, line: 377, type: !341, scopeLine: 377, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !59, retainedNodes: !59)
!422 = !DICompositeType(tag: DW_TAG_structure_type, name: "Peripherals", scope: !158, file: !6, align: 8, elements: !423, templateParams: !59, identifier: "620621dd6b4e0434e1f5ccd29f1dd285")
!423 = !{!424}
!424 = !DIDerivedType(tag: DW_TAG_member, name: "_0", scope: !422, file: !6, baseType: !20, align: 8)
!425 = !DILocation(line: 379, column: 6, scope: !421)
!426 = distinct !DISubprogram(name: "__rtic_internal_foo_spawn", linkageName: "_ZN6locals3app25__rtic_internal_foo_spawn17hf95648dd5d6bc8a0E", scope: !2, file: !4, line: 10, type: !427, scopeLine: 10, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !59, retainedNodes: !429)
!427 = !DISubroutineType(types: !428)
!428 = !{!198}
!429 = !{!430, !432}
!430 = !DILocalVariable(name: "input", scope: !431, file: !4, line: 10, type: !20, align: 1)
!431 = distinct !DILexicalBlock(scope: !426, file: !4, line: 10, column: 1)
!432 = !DILocalVariable(name: "index", scope: !433, file: !4, line: 10, type: !73, align: 1)
!433 = distinct !DILexicalBlock(scope: !431, file: !4, line: 10, column: 1)
!434 = !DILocation(line: 10, column: 1, scope: !431)
!435 = !DILocation(line: 10, column: 1, scope: !433)
!436 = !{i8 0, i8 2}
!437 = !DILocalVariable(name: "self", arg: 1, scope: !438, file: !439, line: 108, type: !443)
!438 = distinct !DISubprogram(name: "get_mut<[core::mem::maybe_uninit::MaybeUninit<()>; 1]>", linkageName: "_ZN4rtic17RacyCell$LT$T$GT$7get_mut17h6b5bae9378d9fb98E", scope: !84, file: !439, line: 108, type: !440, scopeLine: 108, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !102, retainedNodes: !444)
!439 = !DIFile(filename: "/home/norlen/uni/sources/cortex-m-rtic/src/lib.rs", directory: "", checksumkind: CSK_MD5, checksum: "223430ccdc16f35641de97f3b71b231c")
!440 = !DISubroutineType(types: !441)
!441 = !{!442, !443}
!442 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*mut [core::mem::maybe_uninit::MaybeUninit<()>; 1]", baseType: !90, size: 32, align: 32, dwarfAddressSpace: 0)
!443 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&RacyCell<[core::mem::maybe_uninit::MaybeUninit<()>; 1]>", baseType: !84, size: 32, align: 32, dwarfAddressSpace: 0)
!444 = !{!437}
!445 = !DILocation(line: 108, column: 27, scope: !438, inlinedAt: !446)
!446 = distinct !DILocation(line: 10, column: 1, scope: !431)
!447 = !DILocalVariable(name: "self", arg: 1, scope: !448, file: !449, line: 1913, type: !452)
!448 = distinct !DISubprogram(name: "get<[core::mem::maybe_uninit::MaybeUninit<()>; 1]>", linkageName: "_ZN4core4cell19UnsafeCell$LT$T$GT$3get17h12e9c022cf1411cdE", scope: !87, file: !449, line: 1913, type: !450, scopeLine: 1913, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !102, retainedNodes: !453)
!449 = !DIFile(filename: "/rustc/9d1b2106e23b1abd32fce1f17267604a5102f57a/library/core/src/cell.rs", directory: "", checksumkind: CSK_MD5, checksum: "3bf914e53548fc7a1bd700fd8ccf4e77")
!450 = !DISubroutineType(types: !451)
!451 = !{!442, !452}
!452 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&UnsafeCell<[core::mem::maybe_uninit::MaybeUninit<()>; 1]>", baseType: !87, size: 32, align: 32, dwarfAddressSpace: 0)
!453 = !{!447}
!454 = !DILocation(line: 1913, column: 22, scope: !448, inlinedAt: !455)
!455 = distinct !DILocation(line: 109, column: 9, scope: !438, inlinedAt: !446)
!456 = !DILocation(line: 10, column: 62, scope: !426)
!457 = !DILocalVariable(name: "self", arg: 1, scope: !458, file: !459, line: 573, type: !462)
!458 = distinct !DISubprogram(name: "as_mut_ptr<()>", linkageName: "_ZN4core3mem12maybe_uninit20MaybeUninit$LT$T$GT$10as_mut_ptr17hf50335c56921fdceE", scope: !91, file: !459, line: 573, type: !460, scopeLine: 573, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !98, retainedNodes: !463)
!459 = !DIFile(filename: "/rustc/9d1b2106e23b1abd32fce1f17267604a5102f57a/library/core/src/mem/maybe_uninit.rs", directory: "", checksumkind: CSK_MD5, checksum: "1d8b3691e6aa27448e476227834bd7b8")
!460 = !DISubroutineType(types: !461)
!461 = !{!251, !462}
!462 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&mut MaybeUninit<()>", baseType: !91, size: 32, align: 32, dwarfAddressSpace: 0)
!463 = !{!457}
!464 = !DILocation(line: 573, column: 29, scope: !458, inlinedAt: !465)
!465 = distinct !DILocation(line: 10, column: 1, scope: !431)
!466 = !DILocation(line: 575, column: 9, scope: !458, inlinedAt: !465)
!467 = !DILocalVariable(name: "self", arg: 1, scope: !468, file: !469, line: 1061, type: !251)
!468 = distinct !DISubprogram(name: "write<()>", linkageName: "_ZN4core3ptr7mut_ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$5write17h870af99605e44278E", scope: !470, file: !469, line: 1061, type: !472, scopeLine: 1061, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !98, retainedNodes: !474)
!469 = !DIFile(filename: "/rustc/9d1b2106e23b1abd32fce1f17267604a5102f57a/library/core/src/ptr/mut_ptr.rs", directory: "", checksumkind: CSK_MD5, checksum: "e1156b29e19c0e2e60f5fb415734b7ec")
!470 = !DINamespace(name: "{impl#0}", scope: !471)
!471 = !DINamespace(name: "mut_ptr", scope: !230)
!472 = !DISubroutineType(types: !473)
!473 = !{null, !251, !20}
!474 = !{!467, !475}
!475 = !DILocalVariable(name: "val", arg: 2, scope: !468, file: !469, line: 1061, type: !20)
!476 = !DILocation(line: 1061, column: 31, scope: !468, inlinedAt: !477)
!477 = distinct !DILocation(line: 10, column: 1, scope: !431)
!478 = !DILocation(line: 1061, column: 37, scope: !468, inlinedAt: !477)
!479 = !DILocation(line: 1066, column: 18, scope: !468, inlinedAt: !477)
!480 = !DILocation(line: 10, column: 22, scope: !431)
!481 = distinct !DISubprogram(name: "{closure#0}", linkageName: "_ZN6locals3app25__rtic_internal_foo_spawn28_$u7b$$u7b$closure$u7d$$u7d$17h0fbdbcbf10f46067E", scope: !482, file: !4, line: 10, type: !483, scopeLine: 10, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !59, retainedNodes: !503)
!482 = !DINamespace(name: "__rtic_internal_foo_spawn", scope: !2)
!483 = !DISubroutineType(types: !484)
!484 = !{!485, !497, !498}
!485 = !DICompositeType(tag: DW_TAG_structure_type, name: "Option<u8>", scope: !486, file: !6, size: 16, align: 8, elements: !487, identifier: "e94b37c7d24fcdb5116532e24c981191")
!486 = !DINamespace(name: "option", scope: !12)
!487 = !{!488}
!488 = !DICompositeType(tag: DW_TAG_variant_part, scope: !486, file: !6, size: 16, align: 8, elements: !489, templateParams: !74, identifier: "e94b37c7d24fcdb5116532e24c981191_variant_part", discriminator: !496)
!489 = !{!490, !492}
!490 = !DIDerivedType(tag: DW_TAG_member, name: "None", scope: !488, file: !6, baseType: !491, size: 16, align: 8, extraData: i64 0)
!491 = !DICompositeType(tag: DW_TAG_structure_type, name: "None", scope: !485, file: !6, size: 16, align: 8, elements: !59, templateParams: !74, identifier: "e94b37c7d24fcdb5116532e24c981191::None")
!492 = !DIDerivedType(tag: DW_TAG_member, name: "Some", scope: !488, file: !6, baseType: !493, size: 16, align: 8, extraData: i64 1)
!493 = !DICompositeType(tag: DW_TAG_structure_type, name: "Some", scope: !485, file: !6, size: 16, align: 8, elements: !494, templateParams: !74, identifier: "e94b37c7d24fcdb5116532e24c981191::Some")
!494 = !{!495}
!495 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !493, file: !6, baseType: !73, size: 8, align: 8, offset: 8)
!496 = !DIDerivedType(tag: DW_TAG_member, scope: !486, file: !6, baseType: !73, size: 8, align: 8, flags: DIFlagArtificial)
!497 = !DICompositeType(tag: DW_TAG_structure_type, name: "{closure#0}", scope: !482, file: !6, align: 8, elements: !59, templateParams: !59, identifier: "607d3599e6ff892f2a39a5cb83325925")
!498 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&CriticalSection", baseType: !499, size: 32, align: 32, dwarfAddressSpace: 0)
!499 = !DICompositeType(tag: DW_TAG_structure_type, name: "CriticalSection", scope: !500, file: !6, align: 8, elements: !501, templateParams: !59, identifier: "2ac3f4201d45301a712f847901bae482")
!500 = !DINamespace(name: "bare_metal", scope: null)
!501 = !{!502}
!502 = !DIDerivedType(tag: DW_TAG_member, name: "_0", scope: !499, file: !6, baseType: !20, align: 8)
!503 = !{!504, !505}
!504 = !DILocalVariable(arg: 1, scope: !481, file: !4, line: 10, type: !497)
!505 = !DILocalVariable(arg: 2, scope: !481, file: !4, line: 10, type: !498)
!506 = !DILocation(line: 10, column: 1, scope: !481)
!507 = !DILocalVariable(name: "self", arg: 1, scope: !508, file: !439, line: 108, type: !511)
!508 = distinct !DISubprogram(name: "get_mut<heapless::spsc::Queue<u8, 2>>", linkageName: "_ZN4rtic17RacyCell$LT$T$GT$7get_mut17hcdadf74577444ed8E", scope: !37, file: !439, line: 108, type: !509, scopeLine: 108, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !80, retainedNodes: !512)
!509 = !DISubroutineType(types: !510)
!510 = !{!258, !511}
!511 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&RacyCell<heapless::spsc::Queue<u8, 2>>", baseType: !37, size: 32, align: 32, dwarfAddressSpace: 0)
!512 = !{!507}
!513 = !DILocation(line: 108, column: 27, scope: !508, inlinedAt: !514)
!514 = distinct !DILocation(line: 10, column: 1, scope: !481)
!515 = !DILocalVariable(name: "self", arg: 1, scope: !516, file: !449, line: 1913, type: !519)
!516 = distinct !DISubprogram(name: "get<heapless::spsc::Queue<u8, 2>>", linkageName: "_ZN4core4cell19UnsafeCell$LT$T$GT$3get17hc723543fb8185f92E", scope: !40, file: !449, line: 1913, type: !517, scopeLine: 1913, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !80, retainedNodes: !520)
!517 = !DISubroutineType(types: !518)
!518 = !{!258, !519}
!519 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&UnsafeCell<heapless::spsc::Queue<u8, 2>>", baseType: !40, size: 32, align: 32, dwarfAddressSpace: 0)
!520 = !{!515}
!521 = !DILocation(line: 1913, column: 22, scope: !516, inlinedAt: !522)
!522 = distinct !DILocation(line: 109, column: 9, scope: !508, inlinedAt: !514)
!523 = !DILocation(line: 10, column: 62, scope: !481)
!524 = distinct !DISubprogram(name: "{closure#1}", linkageName: "_ZN6locals3app25__rtic_internal_foo_spawn28_$u7b$$u7b$closure$u7d$$u7d$17hce2f397cafa79609E", scope: !482, file: !4, line: 10, type: !525, scopeLine: 10, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !59, retainedNodes: !531)
!525 = !DISubroutineType(types: !526)
!526 = !{null, !527, !498}
!527 = !DICompositeType(tag: DW_TAG_structure_type, name: "{closure#1}", scope: !482, file: !6, size: 32, align: 32, elements: !528, templateParams: !59, identifier: "48ab7ba039aefa26e8ef5cb29d8617c1")
!528 = !{!529}
!529 = !DIDerivedType(tag: DW_TAG_member, name: "_ref__index", scope: !527, file: !6, baseType: !530, size: 32, align: 32)
!530 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&u8", baseType: !73, size: 32, align: 32, dwarfAddressSpace: 0)
!531 = !{!532, !533}
!532 = !DILocalVariable(name: "index", scope: !524, file: !4, line: 10, type: !73, align: 1)
!533 = !DILocalVariable(arg: 2, scope: !524, file: !4, line: 10, type: !498)
!534 = !DILocation(line: 10, column: 1, scope: !524)
!535 = !DILocalVariable(name: "self", arg: 1, scope: !536, file: !439, line: 108, type: !539)
!536 = distinct !DISubprogram(name: "get_mut<heapless::spsc::Queue<(locals::app::P1_T, u8), 3>>", linkageName: "_ZN4rtic17RacyCell$LT$T$GT$7get_mut17h2d5fab911a53b527E", scope: !110, file: !439, line: 108, type: !537, scopeLine: 108, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !146, retainedNodes: !540)
!537 = !DISubroutineType(types: !538)
!538 = !{!274, !539}
!539 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&RacyCell<heapless::spsc::Queue<(locals::app::P1_T, u8), 3>>", baseType: !110, size: 32, align: 32, dwarfAddressSpace: 0)
!540 = !{!535}
!541 = !DILocation(line: 108, column: 27, scope: !536, inlinedAt: !542)
!542 = distinct !DILocation(line: 10, column: 1, scope: !524)
!543 = !DILocalVariable(name: "self", arg: 1, scope: !544, file: !449, line: 1913, type: !547)
!544 = distinct !DISubprogram(name: "get<heapless::spsc::Queue<(locals::app::P1_T, u8), 3>>", linkageName: "_ZN4core4cell19UnsafeCell$LT$T$GT$3get17h8e00477f09d247d5E", scope: !113, file: !449, line: 1913, type: !545, scopeLine: 1913, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !146, retainedNodes: !548)
!545 = !DISubroutineType(types: !546)
!546 = !{!274, !547}
!547 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&UnsafeCell<heapless::spsc::Queue<(locals::app::P1_T, u8), 3>>", baseType: !113, size: 32, align: 32, dwarfAddressSpace: 0)
!548 = !{!543}
!549 = !DILocation(line: 1913, column: 22, scope: !544, inlinedAt: !550)
!550 = distinct !DILocation(line: 109, column: 9, scope: !536, inlinedAt: !542)
!551 = !DILocation(line: 10, column: 62, scope: !524)
!552 = distinct !DISubprogram(name: "number<lm3s6965::Interrupt>", linkageName: "_ZN58_$LT$T$u20$as$u20$cortex_m..interrupt..InterruptNumber$GT$6number17h7a5d7a0fd28511f6E", scope: !554, file: !553, line: 30, type: !556, scopeLine: 30, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !561, retainedNodes: !559)
!553 = !DIFile(filename: "/home/norlen/.cargo/registry/src/github.com-1ecc6299db9ec823/cortex-m-0.7.4/src/interrupt.rs", directory: "", checksumkind: CSK_MD5, checksum: "45a132beae954d8ed6ccc4536310cdd3")
!554 = !DINamespace(name: "{impl#0}", scope: !555)
!555 = !DINamespace(name: "interrupt", scope: !206)
!556 = !DISubroutineType(types: !557)
!557 = !{!558, !157}
!558 = !DIBasicType(name: "u16", size: 16, encoding: DW_ATE_unsigned)
!559 = !{!560}
!560 = !DILocalVariable(name: "self", arg: 1, scope: !552, file: !553, line: 30, type: !157)
!561 = !{!562}
!562 = !DITemplateTypeParameter(name: "T", type: !157)
!563 = !DILocation(line: 30, column: 15, scope: !552)
!564 = !DILocation(line: 31, column: 9, scope: !552)
!565 = !DILocation(line: 32, column: 6, scope: !552)
!566 = distinct !DISubprogram(name: "disable", linkageName: "_ZN8cortex_m9interrupt7disable17hebd3846882b05121E", scope: !555, file: !553, line: 37, type: !341, scopeLine: 37, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !59, retainedNodes: !59)
!567 = !DILocation(line: 38, column: 5, scope: !566)
!568 = !DILocation(line: 39, column: 2, scope: !566)
!569 = distinct !DISubprogram(name: "enable", linkageName: "_ZN8cortex_m9interrupt6enable17h5a8c08841dc312b2E", scope: !555, file: !553, line: 47, type: !341, scopeLine: 47, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !59, retainedNodes: !59)
!570 = !DILocation(line: 48, column: 5, scope: !569)
!571 = !DILocation(line: 49, column: 2, scope: !569)
!572 = distinct !DISubprogram(name: "free<locals::app::__rtic_internal_foo_spawn::{closure#0}, core::option::Option<u8>>", linkageName: "_ZN8cortex_m9interrupt4free17h6900db02a4e65978E", scope: !555, file: !553, line: 55, type: !573, scopeLine: 55, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !581, retainedNodes: !575)
!573 = !DISubroutineType(types: !574)
!574 = !{!485, !497}
!575 = !{!576, !577, !579}
!576 = !DILocalVariable(name: "f", arg: 1, scope: !572, file: !553, line: 55, type: !497)
!577 = !DILocalVariable(name: "primask", scope: !578, file: !553, line: 59, type: !203, align: 1)
!578 = distinct !DILexicalBlock(scope: !572, file: !553, line: 59, column: 5)
!579 = !DILocalVariable(name: "r", scope: !580, file: !553, line: 64, type: !485, align: 1)
!580 = distinct !DILexicalBlock(scope: !578, file: !553, line: 64, column: 5)
!581 = !{!582, !583}
!582 = !DITemplateTypeParameter(name: "F", type: !497)
!583 = !DITemplateTypeParameter(name: "R", type: !485)
!584 = !DILocation(line: 55, column: 19, scope: !572)
!585 = !DILocation(line: 59, column: 19, scope: !572)
!586 = !DILocation(line: 59, column: 9, scope: !578)
!587 = !DILocation(line: 62, column: 5, scope: !578)
!588 = !DILocation(line: 64, column: 25, scope: !578)
!589 = !DILocation(line: 64, column: 13, scope: !578)
!590 = !DILocation(line: 64, column: 9, scope: !580)
!591 = !DILocation(line: 68, column: 8, scope: !580)
!592 = !DILocation(line: 73, column: 2, scope: !572)
!593 = !DILocation(line: 69, column: 18, scope: !580)
!594 = distinct !DISubprogram(name: "free<locals::app::__rtic_internal_bar_spawn::{closure#0}, core::option::Option<u8>>", linkageName: "_ZN8cortex_m9interrupt4free17hcb8de0e5e31ca0b1E", scope: !555, file: !553, line: 55, type: !595, scopeLine: 55, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !605, retainedNodes: !599)
!595 = !DISubroutineType(types: !596)
!596 = !{!485, !597}
!597 = !DICompositeType(tag: DW_TAG_structure_type, name: "{closure#0}", scope: !598, file: !6, align: 8, elements: !59, templateParams: !59, identifier: "349ec02e1dd3c4624dc0c4454b2c57b7")
!598 = !DINamespace(name: "__rtic_internal_bar_spawn", scope: !2)
!599 = !{!600, !601, !603}
!600 = !DILocalVariable(name: "f", arg: 1, scope: !594, file: !553, line: 55, type: !597)
!601 = !DILocalVariable(name: "primask", scope: !602, file: !553, line: 59, type: !203, align: 1)
!602 = distinct !DILexicalBlock(scope: !594, file: !553, line: 59, column: 5)
!603 = !DILocalVariable(name: "r", scope: !604, file: !553, line: 64, type: !485, align: 1)
!604 = distinct !DILexicalBlock(scope: !602, file: !553, line: 64, column: 5)
!605 = !{!606, !583}
!606 = !DITemplateTypeParameter(name: "F", type: !597)
!607 = !DILocation(line: 55, column: 19, scope: !594)
!608 = !DILocation(line: 59, column: 19, scope: !594)
!609 = !DILocation(line: 59, column: 9, scope: !602)
!610 = !DILocation(line: 62, column: 5, scope: !602)
!611 = !DILocation(line: 64, column: 25, scope: !602)
!612 = !DILocation(line: 64, column: 13, scope: !602)
!613 = !DILocation(line: 64, column: 9, scope: !604)
!614 = !DILocation(line: 68, column: 8, scope: !604)
!615 = !DILocation(line: 73, column: 2, scope: !594)
!616 = !DILocation(line: 69, column: 18, scope: !604)
!617 = distinct !DISubprogram(name: "free<locals::app::__rtic_internal_bar_spawn::{closure#1}, ()>", linkageName: "_ZN8cortex_m9interrupt4free17hceee848d0f9dcae4E", scope: !555, file: !553, line: 55, type: !618, scopeLine: 55, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !629, retainedNodes: !623)
!618 = !DISubroutineType(types: !619)
!619 = !{null, !620}
!620 = !DICompositeType(tag: DW_TAG_structure_type, name: "{closure#1}", scope: !598, file: !6, size: 32, align: 32, elements: !621, templateParams: !59, identifier: "10d0378a3612c6c9ce8cb71f04f1f16b")
!621 = !{!622}
!622 = !DIDerivedType(tag: DW_TAG_member, name: "_ref__index", scope: !620, file: !6, baseType: !530, size: 32, align: 32)
!623 = !{!624, !625, !627}
!624 = !DILocalVariable(name: "f", arg: 1, scope: !617, file: !553, line: 55, type: !620)
!625 = !DILocalVariable(name: "primask", scope: !626, file: !553, line: 59, type: !203, align: 1)
!626 = distinct !DILexicalBlock(scope: !617, file: !553, line: 59, column: 5)
!627 = !DILocalVariable(name: "r", scope: !628, file: !553, line: 64, type: !20, align: 1)
!628 = distinct !DILexicalBlock(scope: !626, file: !553, line: 64, column: 5)
!629 = !{!630, !631}
!630 = !DITemplateTypeParameter(name: "F", type: !620)
!631 = !DITemplateTypeParameter(name: "R", type: !20)
!632 = !DILocation(line: 64, column: 9, scope: !628)
!633 = !DILocation(line: 55, column: 19, scope: !617)
!634 = !DILocation(line: 59, column: 19, scope: !617)
!635 = !DILocation(line: 59, column: 9, scope: !626)
!636 = !DILocation(line: 62, column: 5, scope: !626)
!637 = !DILocation(line: 64, column: 25, scope: !626)
!638 = !DILocation(line: 64, column: 13, scope: !626)
!639 = !DILocation(line: 68, column: 8, scope: !628)
!640 = !DILocation(line: 73, column: 2, scope: !617)
!641 = !DILocation(line: 69, column: 18, scope: !628)
!642 = distinct !DISubprogram(name: "free<locals::app::__rtic_internal_foo_spawn::{closure#1}, ()>", linkageName: "_ZN8cortex_m9interrupt4free17he76fbd21a9f7aec8E", scope: !555, file: !553, line: 55, type: !643, scopeLine: 55, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !651, retainedNodes: !645)
!643 = !DISubroutineType(types: !644)
!644 = !{null, !527}
!645 = !{!646, !647, !649}
!646 = !DILocalVariable(name: "f", arg: 1, scope: !642, file: !553, line: 55, type: !527)
!647 = !DILocalVariable(name: "primask", scope: !648, file: !553, line: 59, type: !203, align: 1)
!648 = distinct !DILexicalBlock(scope: !642, file: !553, line: 59, column: 5)
!649 = !DILocalVariable(name: "r", scope: !650, file: !553, line: 64, type: !20, align: 1)
!650 = distinct !DILexicalBlock(scope: !648, file: !553, line: 64, column: 5)
!651 = !{!652, !631}
!652 = !DITemplateTypeParameter(name: "F", type: !527)
!653 = !DILocation(line: 64, column: 9, scope: !650)
!654 = !DILocation(line: 55, column: 19, scope: !642)
!655 = !DILocation(line: 59, column: 19, scope: !642)
!656 = !DILocation(line: 59, column: 9, scope: !648)
!657 = !DILocation(line: 62, column: 5, scope: !648)
!658 = !DILocation(line: 64, column: 25, scope: !648)
!659 = !DILocation(line: 64, column: 13, scope: !648)
!660 = !DILocation(line: 68, column: 8, scope: !650)
!661 = !DILocation(line: 73, column: 2, scope: !642)
!662 = !DILocation(line: 69, column: 18, scope: !650)
!663 = distinct !DISubprogram(name: "__rtic_internal_bar_spawn", linkageName: "_ZN6locals3app25__rtic_internal_bar_spawn17hbcc8391a9543253dE", scope: !2, file: !4, line: 10, type: !427, scopeLine: 10, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !59, retainedNodes: !664)
!664 = !{!665, !667}
!665 = !DILocalVariable(name: "input", scope: !666, file: !4, line: 10, type: !20, align: 1)
!666 = distinct !DILexicalBlock(scope: !663, file: !4, line: 10, column: 1)
!667 = !DILocalVariable(name: "index", scope: !668, file: !4, line: 10, type: !73, align: 1)
!668 = distinct !DILexicalBlock(scope: !666, file: !4, line: 10, column: 1)
!669 = !DILocation(line: 10, column: 1, scope: !666)
!670 = !DILocation(line: 10, column: 1, scope: !668)
!671 = !DILocation(line: 108, column: 27, scope: !438, inlinedAt: !672)
!672 = distinct !DILocation(line: 10, column: 1, scope: !666)
!673 = !DILocation(line: 1913, column: 22, scope: !448, inlinedAt: !674)
!674 = distinct !DILocation(line: 109, column: 9, scope: !438, inlinedAt: !672)
!675 = !DILocation(line: 10, column: 62, scope: !663)
!676 = !DILocation(line: 573, column: 29, scope: !458, inlinedAt: !677)
!677 = distinct !DILocation(line: 10, column: 1, scope: !666)
!678 = !DILocation(line: 575, column: 9, scope: !458, inlinedAt: !677)
!679 = !DILocation(line: 1061, column: 31, scope: !468, inlinedAt: !680)
!680 = distinct !DILocation(line: 10, column: 1, scope: !666)
!681 = !DILocation(line: 1061, column: 37, scope: !468, inlinedAt: !680)
!682 = !DILocation(line: 1066, column: 18, scope: !468, inlinedAt: !680)
!683 = !DILocation(line: 10, column: 22, scope: !666)
!684 = distinct !DISubprogram(name: "{closure#0}", linkageName: "_ZN6locals3app25__rtic_internal_bar_spawn28_$u7b$$u7b$closure$u7d$$u7d$17hf91bfa12477a85f6E", scope: !598, file: !4, line: 10, type: !685, scopeLine: 10, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !59, retainedNodes: !687)
!685 = !DISubroutineType(types: !686)
!686 = !{!485, !597, !498}
!687 = !{!688, !689}
!688 = !DILocalVariable(arg: 1, scope: !684, file: !4, line: 10, type: !597)
!689 = !DILocalVariable(arg: 2, scope: !684, file: !4, line: 10, type: !498)
!690 = !DILocation(line: 10, column: 1, scope: !684)
!691 = !DILocation(line: 108, column: 27, scope: !508, inlinedAt: !692)
!692 = distinct !DILocation(line: 10, column: 1, scope: !684)
!693 = !DILocation(line: 1913, column: 22, scope: !516, inlinedAt: !694)
!694 = distinct !DILocation(line: 109, column: 9, scope: !508, inlinedAt: !692)
!695 = !DILocation(line: 10, column: 62, scope: !684)
!696 = distinct !DISubprogram(name: "{closure#1}", linkageName: "_ZN6locals3app25__rtic_internal_bar_spawn28_$u7b$$u7b$closure$u7d$$u7d$17h48b2b744e8040946E", scope: !598, file: !4, line: 10, type: !697, scopeLine: 10, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !59, retainedNodes: !699)
!697 = !DISubroutineType(types: !698)
!698 = !{null, !620, !498}
!699 = !{!700, !701}
!700 = !DILocalVariable(name: "index", scope: !696, file: !4, line: 10, type: !73, align: 1)
!701 = !DILocalVariable(arg: 2, scope: !696, file: !4, line: 10, type: !498)
!702 = !DILocation(line: 10, column: 1, scope: !696)
!703 = !DILocation(line: 108, column: 27, scope: !536, inlinedAt: !704)
!704 = distinct !DILocation(line: 10, column: 1, scope: !696)
!705 = !DILocation(line: 1913, column: 22, scope: !544, inlinedAt: !706)
!706 = distinct !DILocation(line: 109, column: 9, scope: !536, inlinedAt: !704)
!707 = !DILocation(line: 10, column: 62, scope: !696)
!708 = distinct !DISubprogram(name: "UART0", scope: !2, file: !4, line: 10, type: !341, scopeLine: 10, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !154, templateParams: !59, retainedNodes: !59)
!709 = !DILocalVariable(name: "priority", arg: 1, scope: !710, file: !711, line: 29, type: !73)
!710 = distinct !DISubprogram(name: "run<locals::app::UART0::{closure#0}>", linkageName: "_ZN4rtic6export3run17h06c6adc339a13d00E", scope: !712, file: !711, line: 29, type: !713, scopeLine: 29, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !721, retainedNodes: !717)
!711 = !DIFile(filename: "/home/norlen/uni/sources/cortex-m-rtic/src/export.rs", directory: "", checksumkind: CSK_MD5, checksum: "bbb33e88f34fd5f0b12613f69300bec8")
!712 = !DINamespace(name: "export", scope: !7)
!713 = !DISubroutineType(types: !714)
!714 = !{null, !73, !715}
!715 = !DICompositeType(tag: DW_TAG_structure_type, name: "{closure#0}", scope: !716, file: !6, align: 8, elements: !59, templateParams: !59, identifier: "e451ce35367982bf7da65b5560a1aab1")
!716 = !DINamespace(name: "UART0", scope: !2)
!717 = !{!709, !718, !719}
!718 = !DILocalVariable(name: "f", arg: 2, scope: !710, file: !711, line: 29, type: !715)
!719 = !DILocalVariable(name: "initial", scope: !720, file: !711, line: 38, type: !73, align: 1)
!720 = distinct !DILexicalBlock(scope: !710, file: !711, line: 38, column: 9)
!721 = !{!722}
!722 = !DITemplateTypeParameter(name: "F", type: !715)
!723 = !DILocation(line: 29, column: 15, scope: !710, inlinedAt: !724)
!724 = distinct !DILocation(line: 10, column: 1, scope: !708)
!725 = !DILocation(line: 29, column: 29, scope: !710, inlinedAt: !724)
!726 = !DILocation(line: 35, column: 9, scope: !710, inlinedAt: !724)
!727 = !DILocation(line: 36, column: 18, scope: !710, inlinedAt: !724)
!728 = !DILocation(line: 10, column: 1, scope: !708)
!729 = !DILocation(line: 10, column: 62, scope: !708)
!730 = distinct !DISubprogram(name: "logical2hw", linkageName: "_ZN4rtic6export10logical2hw17h210e62a82e639146E", scope: !712, file: !711, line: 315, type: !731, scopeLine: 315, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !59, retainedNodes: !733)
!731 = !DISubroutineType(types: !732)
!732 = !{!73, !73, !73}
!733 = !{!734, !735}
!734 = !DILocalVariable(name: "logical", arg: 1, scope: !730, file: !711, line: 315, type: !73)
!735 = !DILocalVariable(name: "nvic_prio_bits", arg: 2, scope: !730, file: !711, line: 315, type: !73)
!736 = !DILocation(line: 315, column: 19, scope: !730)
!737 = !DILocation(line: 315, column: 32, scope: !730)
!738 = !DILocation(line: 316, column: 6, scope: !730)
!739 = !DILocation(line: 316, column: 5, scope: !730)
!740 = !DILocation(line: 316, column: 42, scope: !730)
!741 = !DILocation(line: 317, column: 2, scope: !730)
!742 = distinct !DISubprogram(name: "{closure#0}", linkageName: "_ZN6locals3app5UART028_$u7b$$u7b$closure$u7d$$u7d$17h03af6aeda782247bE", scope: !716, file: !4, line: 10, type: !743, scopeLine: 10, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !59, retainedNodes: !745)
!743 = !DISubroutineType(types: !744)
!744 = !{null, !715}
!745 = !{!746, !748, !749, !761, !763}
!746 = !DILocalVariable(name: "task", scope: !747, file: !4, line: 10, type: !135, align: 1)
!747 = distinct !DILexicalBlock(scope: !742, file: !4, line: 10, column: 1)
!748 = !DILocalVariable(name: "index", scope: !747, file: !4, line: 10, type: !73, align: 1)
!749 = !DILocalVariable(name: "priority", scope: !750, file: !4, line: 10, type: !751, align: 4)
!750 = distinct !DILexicalBlock(scope: !742, file: !4, line: 10, column: 1)
!751 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&Priority", baseType: !752, size: 32, align: 32, dwarfAddressSpace: 0)
!752 = !DICompositeType(tag: DW_TAG_structure_type, name: "Priority", scope: !712, file: !6, size: 8, align: 8, elements: !753, templateParams: !59, identifier: "6846892f6347c3f648f468cc5a2287a7")
!753 = !{!754}
!754 = !DIDerivedType(tag: DW_TAG_member, name: "inner", scope: !752, file: !6, baseType: !755, size: 8, align: 8)
!755 = !DICompositeType(tag: DW_TAG_structure_type, name: "Cell<u8>", scope: !11, file: !6, size: 8, align: 8, elements: !756, templateParams: !74, identifier: "92dbacf0be0bd2cfbeabd5e39071b5b6")
!756 = !{!757}
!757 = !DIDerivedType(tag: DW_TAG_member, name: "value", scope: !755, file: !6, baseType: !758, size: 8, align: 8)
!758 = !DICompositeType(tag: DW_TAG_structure_type, name: "UnsafeCell<u8>", scope: !11, file: !6, size: 8, align: 8, elements: !759, templateParams: !74, identifier: "d6250dce5061df1b55195a084aed5772")
!759 = !{!760}
!760 = !DIDerivedType(tag: DW_TAG_member, name: "value", scope: !758, file: !6, baseType: !73, size: 8, align: 8)
!761 = !DILocalVariable(name: "priority", scope: !762, file: !4, line: 10, type: !751, align: 4)
!762 = distinct !DILexicalBlock(scope: !742, file: !4, line: 10, column: 1)
!763 = !DILocalVariable(arg: 1, scope: !742, file: !4, line: 10, type: !715)
!764 = !DILocation(line: 10, column: 1, scope: !742)
!765 = !DILocation(line: 10, column: 1, scope: !747)
!766 = !DILocation(line: 108, column: 27, scope: !536, inlinedAt: !767)
!767 = distinct !DILocation(line: 10, column: 1, scope: !742)
!768 = !DILocation(line: 1913, column: 22, scope: !544, inlinedAt: !769)
!769 = distinct !DILocation(line: 109, column: 9, scope: !536, inlinedAt: !767)
!770 = !{i8 0, i8 3}
!771 = !DILocation(line: 10, column: 62, scope: !742)
!772 = !DILocalVariable(name: "self", arg: 1, scope: !773, file: !439, line: 118, type: !443)
!773 = distinct !DISubprogram(name: "get<[core::mem::maybe_uninit::MaybeUninit<()>; 1]>", linkageName: "_ZN4rtic17RacyCell$LT$T$GT$3get17h1075ccf742763f2aE", scope: !84, file: !439, line: 118, type: !774, scopeLine: 118, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !102, retainedNodes: !777)
!774 = !DISubroutineType(types: !775)
!775 = !{!776, !443}
!776 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*const [core::mem::maybe_uninit::MaybeUninit<()>; 1]", baseType: !90, size: 32, align: 32, dwarfAddressSpace: 0)
!777 = !{!772}
!778 = !DILocation(line: 118, column: 23, scope: !773, inlinedAt: !779)
!779 = distinct !DILocation(line: 10, column: 1, scope: !742)
!780 = !DILocation(line: 1913, column: 22, scope: !448, inlinedAt: !781)
!781 = distinct !DILocation(line: 119, column: 9, scope: !773, inlinedAt: !779)
!782 = !DILocation(line: 118, column: 23, scope: !773, inlinedAt: !783)
!783 = distinct !DILocation(line: 10, column: 1, scope: !742)
!784 = !DILocation(line: 1913, column: 22, scope: !448, inlinedAt: !785)
!785 = distinct !DILocation(line: 119, column: 9, scope: !773, inlinedAt: !783)
!786 = !DILocalVariable(name: "self", arg: 1, scope: !787, file: !459, line: 534, type: !790)
!787 = distinct !DISubprogram(name: "as_ptr<()>", linkageName: "_ZN4core3mem12maybe_uninit20MaybeUninit$LT$T$GT$6as_ptr17h7d6ebdbd0e4decf0E", scope: !91, file: !459, line: 534, type: !788, scopeLine: 534, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !98, retainedNodes: !791)
!788 = !DISubroutineType(types: !789)
!789 = !{!151, !790}
!790 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&MaybeUninit<()>", baseType: !91, size: 32, align: 32, dwarfAddressSpace: 0)
!791 = !{!786}
!792 = !DILocation(line: 534, column: 25, scope: !787, inlinedAt: !793)
!793 = distinct !DILocation(line: 10, column: 1, scope: !742)
!794 = !DILocation(line: 536, column: 9, scope: !787, inlinedAt: !793)
!795 = !DILocation(line: 108, column: 27, scope: !508, inlinedAt: !796)
!796 = distinct !DILocation(line: 10, column: 1, scope: !742)
!797 = !DILocation(line: 1913, column: 22, scope: !516, inlinedAt: !798)
!798 = distinct !DILocation(line: 109, column: 9, scope: !508, inlinedAt: !796)
!799 = !DILocalVariable(name: "value", arg: 1, scope: !800, file: !711, line: 85, type: !73)
!800 = distinct !DISubprogram(name: "new", linkageName: "_ZN4rtic6export8Priority3new17hd72d81694c26e8dcE", scope: !752, file: !711, line: 85, type: !801, scopeLine: 85, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !59, retainedNodes: !803)
!801 = !DISubroutineType(types: !802)
!802 = !{!752, !73}
!803 = !{!799}
!804 = !DILocation(line: 85, column: 23, scope: !800, inlinedAt: !805)
!805 = distinct !DILocation(line: 10, column: 1, scope: !742)
!806 = !DILocation(line: 87, column: 20, scope: !800, inlinedAt: !805)
!807 = !DILocation(line: 86, column: 9, scope: !800, inlinedAt: !805)
!808 = !DILocation(line: 89, column: 6, scope: !800, inlinedAt: !805)
!809 = !DILocation(line: 10, column: 1, scope: !762)
!810 = !DILocalVariable(name: "priority", arg: 1, scope: !811, file: !4, line: 10, type: !751)
!811 = distinct !DISubprogram(name: "new", linkageName: "_ZN6locals3app27__rtic_internal_foo_Context3new17h4e9c97e5de5a3b21E", scope: !378, file: !4, line: 10, type: !812, scopeLine: 10, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !59, retainedNodes: !814)
!812 = !DISubroutineType(types: !813)
!813 = !{!378, !751}
!814 = !{!810}
!815 = !DILocation(line: 10, column: 1, scope: !811, inlinedAt: !816)
!816 = distinct !DILocation(line: 10, column: 1, scope: !762)
!817 = !DILocalVariable(name: "self", arg: 1, scope: !818, file: !439, line: 108, type: !822)
!818 = distinct !DISubprogram(name: "get_mut<core::mem::maybe_uninit::MaybeUninit<i64>>", linkageName: "_ZN4rtic17RacyCell$LT$T$GT$7get_mut17hd4812c4d9204041bE", scope: !5, file: !439, line: 108, type: !819, scopeLine: 108, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !29, retainedNodes: !823)
!819 = !DISubroutineType(types: !820)
!820 = !{!821, !822}
!821 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*mut MaybeUninit<i64>", baseType: !15, size: 32, align: 32, dwarfAddressSpace: 0)
!822 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&RacyCell<core::mem::maybe_uninit::MaybeUninit<i64>>", baseType: !5, size: 32, align: 32, dwarfAddressSpace: 0)
!823 = !{!817}
!824 = !DILocation(line: 108, column: 27, scope: !818, inlinedAt: !825)
!825 = distinct !DILocation(line: 10, column: 1, scope: !826, inlinedAt: !829)
!826 = distinct !DISubprogram(name: "new", linkageName: "_ZN6locals3app33__rtic_internal_fooLocalResources3new17h84cfce673a39d0f8E", scope: !381, file: !4, line: 10, type: !827, scopeLine: 10, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !59, retainedNodes: !59)
!827 = !DISubroutineType(types: !828)
!828 = !{!381}
!829 = distinct !DILocation(line: 10, column: 1, scope: !811, inlinedAt: !816)
!830 = !DILocalVariable(name: "self", arg: 1, scope: !831, file: !449, line: 1913, type: !834)
!831 = distinct !DISubprogram(name: "get<core::mem::maybe_uninit::MaybeUninit<i64>>", linkageName: "_ZN4core4cell19UnsafeCell$LT$T$GT$3get17hd9b711984272f6d3E", scope: !10, file: !449, line: 1913, type: !832, scopeLine: 1913, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !29, retainedNodes: !835)
!832 = !DISubroutineType(types: !833)
!833 = !{!821, !834}
!834 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&UnsafeCell<core::mem::maybe_uninit::MaybeUninit<i64>>", baseType: !10, size: 32, align: 32, dwarfAddressSpace: 0)
!835 = !{!830}
!836 = !DILocation(line: 1913, column: 22, scope: !831, inlinedAt: !837)
!837 = distinct !DILocation(line: 109, column: 9, scope: !818, inlinedAt: !825)
!838 = !DILocalVariable(name: "self", arg: 1, scope: !839, file: !459, line: 573, type: !843)
!839 = distinct !DISubprogram(name: "as_mut_ptr<i64>", linkageName: "_ZN4core3mem12maybe_uninit20MaybeUninit$LT$T$GT$10as_mut_ptr17h5ebe17bb2d589c52E", scope: !15, file: !459, line: 573, type: !840, scopeLine: 573, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !27, retainedNodes: !844)
!840 = !DISubroutineType(types: !841)
!841 = !{!842, !843}
!842 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*mut i64", baseType: !26, size: 32, align: 32, dwarfAddressSpace: 0)
!843 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&mut MaybeUninit<i64>", baseType: !15, size: 32, align: 32, dwarfAddressSpace: 0)
!844 = !{!838}
!845 = !DILocation(line: 573, column: 29, scope: !839, inlinedAt: !846)
!846 = distinct !DILocation(line: 10, column: 1, scope: !826, inlinedAt: !829)
!847 = !DILocation(line: 10, column: 1, scope: !826, inlinedAt: !829)
!848 = !DILocation(line: 10, column: 62, scope: !826, inlinedAt: !829)
!849 = !DILocation(line: 10, column: 62, scope: !811, inlinedAt: !816)
!850 = !DILocation(line: 534, column: 25, scope: !787, inlinedAt: !851)
!851 = distinct !DILocation(line: 10, column: 1, scope: !742)
!852 = !DILocation(line: 536, column: 9, scope: !787, inlinedAt: !851)
!853 = !DILocation(line: 108, column: 27, scope: !508, inlinedAt: !854)
!854 = distinct !DILocation(line: 10, column: 1, scope: !742)
!855 = !DILocation(line: 1913, column: 22, scope: !516, inlinedAt: !856)
!856 = distinct !DILocation(line: 109, column: 9, scope: !508, inlinedAt: !854)
!857 = !DILocation(line: 85, column: 23, scope: !800, inlinedAt: !858)
!858 = distinct !DILocation(line: 10, column: 1, scope: !742)
!859 = !DILocation(line: 87, column: 20, scope: !800, inlinedAt: !858)
!860 = !DILocation(line: 86, column: 9, scope: !800, inlinedAt: !858)
!861 = !DILocation(line: 89, column: 6, scope: !800, inlinedAt: !858)
!862 = !DILocation(line: 10, column: 1, scope: !750)
!863 = !DILocalVariable(name: "priority", arg: 1, scope: !864, file: !4, line: 10, type: !751)
!864 = distinct !DISubprogram(name: "new", linkageName: "_ZN6locals3app27__rtic_internal_bar_Context3new17h785e4a455e0aac53E", scope: !401, file: !4, line: 10, type: !865, scopeLine: 10, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !59, retainedNodes: !867)
!865 = !DISubroutineType(types: !866)
!866 = !{!401, !751}
!867 = !{!863}
!868 = !DILocation(line: 10, column: 1, scope: !864, inlinedAt: !869)
!869 = distinct !DILocation(line: 10, column: 1, scope: !750)
!870 = !DILocation(line: 108, column: 27, scope: !818, inlinedAt: !871)
!871 = distinct !DILocation(line: 10, column: 1, scope: !872, inlinedAt: !875)
!872 = distinct !DISubprogram(name: "new", linkageName: "_ZN6locals3app33__rtic_internal_barLocalResources3new17h71e09ef7a98ee115E", scope: !404, file: !4, line: 10, type: !873, scopeLine: 10, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !59, retainedNodes: !59)
!873 = !DISubroutineType(types: !874)
!874 = !{!404}
!875 = distinct !DILocation(line: 10, column: 1, scope: !864, inlinedAt: !869)
!876 = !DILocation(line: 1913, column: 22, scope: !831, inlinedAt: !877)
!877 = distinct !DILocation(line: 109, column: 9, scope: !818, inlinedAt: !871)
!878 = !DILocation(line: 573, column: 29, scope: !839, inlinedAt: !879)
!879 = distinct !DILocation(line: 10, column: 1, scope: !872, inlinedAt: !875)
!880 = !DILocation(line: 10, column: 1, scope: !872, inlinedAt: !875)
!881 = !DILocation(line: 10, column: 62, scope: !872, inlinedAt: !875)
!882 = !DILocation(line: 10, column: 62, scope: !864, inlinedAt: !869)
!883 = distinct !DISubprogram(name: "main", scope: !884, file: !4, line: 10, type: !341, scopeLine: 10, flags: DIFlagPrototyped | DIFlagNoReturn, spFlags: DISPFlagDefinition, unit: !154, templateParams: !59, retainedNodes: !885)
!884 = !DINamespace(name: "rtic_ext", scope: !2)
!885 = !{!886}
!886 = !DILocalVariable(name: "core", scope: !887, file: !4, line: 10, type: !888, align: 1)
!887 = distinct !DILexicalBlock(scope: !883, file: !4, line: 10, column: 1)
!888 = !DICompositeType(tag: DW_TAG_structure_type, name: "Peripherals", scope: !889, file: !6, align: 8, elements: !890, templateParams: !59, identifier: "1753cc99e079450d4de3421dcc51085b")
!889 = !DINamespace(name: "peripheral", scope: !206)
!890 = !{!891, !899, !903, !907, !911, !915, !919, !923, !927, !931, !935, !939, !943, !947, !951}
!891 = !DIDerivedType(tag: DW_TAG_member, name: "CBP", scope: !888, file: !6, baseType: !892, align: 8)
!892 = !DICompositeType(tag: DW_TAG_structure_type, name: "CBP", scope: !889, file: !6, align: 8, elements: !893, templateParams: !59, identifier: "8ecca395b60616ddd82350aae9e707db")
!893 = !{!894}
!894 = !DIDerivedType(tag: DW_TAG_member, name: "_marker", scope: !892, file: !6, baseType: !895, align: 8)
!895 = !DICompositeType(tag: DW_TAG_structure_type, name: "PhantomData<*const ()>", scope: !896, file: !6, align: 8, elements: !59, templateParams: !897, identifier: "48ee89568e2f4192617e228e704e7341")
!896 = !DINamespace(name: "marker", scope: !12)
!897 = !{!898}
!898 = !DITemplateTypeParameter(name: "T", type: !151)
!899 = !DIDerivedType(tag: DW_TAG_member, name: "CPUID", scope: !888, file: !6, baseType: !900, align: 8)
!900 = !DICompositeType(tag: DW_TAG_structure_type, name: "CPUID", scope: !889, file: !6, align: 8, elements: !901, templateParams: !59, identifier: "4e58596d2f2c4e48f154cc14d5e04624")
!901 = !{!902}
!902 = !DIDerivedType(tag: DW_TAG_member, name: "_marker", scope: !900, file: !6, baseType: !895, align: 8)
!903 = !DIDerivedType(tag: DW_TAG_member, name: "DCB", scope: !888, file: !6, baseType: !904, align: 8)
!904 = !DICompositeType(tag: DW_TAG_structure_type, name: "DCB", scope: !889, file: !6, align: 8, elements: !905, templateParams: !59, identifier: "f0faf614a588df66e4e413ff9ed8b917")
!905 = !{!906}
!906 = !DIDerivedType(tag: DW_TAG_member, name: "_marker", scope: !904, file: !6, baseType: !895, align: 8)
!907 = !DIDerivedType(tag: DW_TAG_member, name: "DWT", scope: !888, file: !6, baseType: !908, align: 8)
!908 = !DICompositeType(tag: DW_TAG_structure_type, name: "DWT", scope: !889, file: !6, align: 8, elements: !909, templateParams: !59, identifier: "7b98bde0053fc7c92eedc42bcf949941")
!909 = !{!910}
!910 = !DIDerivedType(tag: DW_TAG_member, name: "_marker", scope: !908, file: !6, baseType: !895, align: 8)
!911 = !DIDerivedType(tag: DW_TAG_member, name: "FPB", scope: !888, file: !6, baseType: !912, align: 8)
!912 = !DICompositeType(tag: DW_TAG_structure_type, name: "FPB", scope: !889, file: !6, align: 8, elements: !913, templateParams: !59, identifier: "5b845644456060d5bef6224bd6a2de25")
!913 = !{!914}
!914 = !DIDerivedType(tag: DW_TAG_member, name: "_marker", scope: !912, file: !6, baseType: !895, align: 8)
!915 = !DIDerivedType(tag: DW_TAG_member, name: "FPU", scope: !888, file: !6, baseType: !916, align: 8)
!916 = !DICompositeType(tag: DW_TAG_structure_type, name: "FPU", scope: !889, file: !6, align: 8, elements: !917, templateParams: !59, identifier: "532806cc833250aaca320455f722ad8c")
!917 = !{!918}
!918 = !DIDerivedType(tag: DW_TAG_member, name: "_marker", scope: !916, file: !6, baseType: !895, align: 8)
!919 = !DIDerivedType(tag: DW_TAG_member, name: "ICB", scope: !888, file: !6, baseType: !920, align: 8)
!920 = !DICompositeType(tag: DW_TAG_structure_type, name: "ICB", scope: !889, file: !6, align: 8, elements: !921, templateParams: !59, identifier: "fede5882e4709b97c3a43a52b2814c4")
!921 = !{!922}
!922 = !DIDerivedType(tag: DW_TAG_member, name: "_marker", scope: !920, file: !6, baseType: !895, align: 8)
!923 = !DIDerivedType(tag: DW_TAG_member, name: "ITM", scope: !888, file: !6, baseType: !924, align: 8)
!924 = !DICompositeType(tag: DW_TAG_structure_type, name: "ITM", scope: !889, file: !6, align: 8, elements: !925, templateParams: !59, identifier: "bac7c8c9c1ebdc637985a1d6f6f5d0f7")
!925 = !{!926}
!926 = !DIDerivedType(tag: DW_TAG_member, name: "_marker", scope: !924, file: !6, baseType: !895, align: 8)
!927 = !DIDerivedType(tag: DW_TAG_member, name: "MPU", scope: !888, file: !6, baseType: !928, align: 8)
!928 = !DICompositeType(tag: DW_TAG_structure_type, name: "MPU", scope: !889, file: !6, align: 8, elements: !929, templateParams: !59, identifier: "84bd258118cb62549dd4de7b2807ba43")
!929 = !{!930}
!930 = !DIDerivedType(tag: DW_TAG_member, name: "_marker", scope: !928, file: !6, baseType: !895, align: 8)
!931 = !DIDerivedType(tag: DW_TAG_member, name: "NVIC", scope: !888, file: !6, baseType: !932, align: 8)
!932 = !DICompositeType(tag: DW_TAG_structure_type, name: "NVIC", scope: !889, file: !6, align: 8, elements: !933, templateParams: !59, identifier: "c98a535c5e8878074305a2a8a2f74722")
!933 = !{!934}
!934 = !DIDerivedType(tag: DW_TAG_member, name: "_marker", scope: !932, file: !6, baseType: !895, align: 8)
!935 = !DIDerivedType(tag: DW_TAG_member, name: "SAU", scope: !888, file: !6, baseType: !936, align: 8)
!936 = !DICompositeType(tag: DW_TAG_structure_type, name: "SAU", scope: !889, file: !6, align: 8, elements: !937, templateParams: !59, identifier: "bb960db78b815a16830e5ce4db6ef08b")
!937 = !{!938}
!938 = !DIDerivedType(tag: DW_TAG_member, name: "_marker", scope: !936, file: !6, baseType: !895, align: 8)
!939 = !DIDerivedType(tag: DW_TAG_member, name: "SCB", scope: !888, file: !6, baseType: !940, align: 8)
!940 = !DICompositeType(tag: DW_TAG_structure_type, name: "SCB", scope: !889, file: !6, align: 8, elements: !941, templateParams: !59, identifier: "2de07eac93ae794ac583f143d3830b42")
!941 = !{!942}
!942 = !DIDerivedType(tag: DW_TAG_member, name: "_marker", scope: !940, file: !6, baseType: !895, align: 8)
!943 = !DIDerivedType(tag: DW_TAG_member, name: "SYST", scope: !888, file: !6, baseType: !944, align: 8)
!944 = !DICompositeType(tag: DW_TAG_structure_type, name: "SYST", scope: !889, file: !6, align: 8, elements: !945, templateParams: !59, identifier: "17ff24037a4cd0299ab9f906751c83be")
!945 = !{!946}
!946 = !DIDerivedType(tag: DW_TAG_member, name: "_marker", scope: !944, file: !6, baseType: !895, align: 8)
!947 = !DIDerivedType(tag: DW_TAG_member, name: "TPIU", scope: !888, file: !6, baseType: !948, align: 8)
!948 = !DICompositeType(tag: DW_TAG_structure_type, name: "TPIU", scope: !889, file: !6, align: 8, elements: !949, templateParams: !59, identifier: "8ea6ed4c3b284fe322cdca12e40d5e42")
!949 = !{!950}
!950 = !DIDerivedType(tag: DW_TAG_member, name: "_marker", scope: !948, file: !6, baseType: !895, align: 8)
!951 = !DIDerivedType(tag: DW_TAG_member, name: "_priv", scope: !888, file: !6, baseType: !20, align: 8)
!952 = !DILocation(line: 10, column: 1, scope: !887)
!953 = !DILocation(line: 10, column: 1, scope: !883)
!954 = !DILocation(line: 85, column: 23, scope: !800, inlinedAt: !955)
!955 = distinct !DILocation(line: 10, column: 1, scope: !887)
!956 = !DILocation(line: 87, column: 20, scope: !800, inlinedAt: !955)
!957 = !DILocation(line: 86, column: 9, scope: !800, inlinedAt: !955)
!958 = !DILocation(line: 89, column: 6, scope: !800, inlinedAt: !955)
!959 = !DILocalVariable(name: "priority", arg: 1, scope: !960, file: !4, line: 10, type: !751)
!960 = distinct !DISubprogram(name: "new", linkageName: "_ZN6locals3app28__rtic_internal_idle_Context3new17h96482c82ac4ab76bE", scope: !348, file: !4, line: 10, type: !961, scopeLine: 10, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !59, retainedNodes: !963)
!961 = !DISubroutineType(types: !962)
!962 = !{!348, !751}
!963 = !{!959}
!964 = !DILocation(line: 10, column: 1, scope: !960, inlinedAt: !965)
!965 = distinct !DILocation(line: 10, column: 1, scope: !887)
!966 = !DILocation(line: 108, column: 27, scope: !818, inlinedAt: !967)
!967 = distinct !DILocation(line: 10, column: 1, scope: !968, inlinedAt: !971)
!968 = distinct !DISubprogram(name: "new", linkageName: "_ZN6locals3app34__rtic_internal_idleLocalResources3new17hd3f7af301665367dE", scope: !351, file: !4, line: 10, type: !969, scopeLine: 10, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !59, retainedNodes: !59)
!969 = !DISubroutineType(types: !970)
!970 = !{!351}
!971 = distinct !DILocation(line: 10, column: 1, scope: !960, inlinedAt: !965)
!972 = !DILocation(line: 1913, column: 22, scope: !831, inlinedAt: !973)
!973 = distinct !DILocation(line: 109, column: 9, scope: !818, inlinedAt: !967)
!974 = !DILocation(line: 573, column: 29, scope: !839, inlinedAt: !975)
!975 = distinct !DILocation(line: 10, column: 1, scope: !968, inlinedAt: !971)
!976 = !DILocation(line: 10, column: 1, scope: !968, inlinedAt: !971)
!977 = !DILocation(line: 10, column: 62, scope: !968, inlinedAt: !971)
!978 = !DILocation(line: 10, column: 62, scope: !960, inlinedAt: !965)
!979 = distinct !DISubprogram(name: "{closure#0}", linkageName: "_ZN6locals3app8rtic_ext4main28_$u7b$$u7b$closure$u7d$$u7d$17h1bdb1d17f7ba3d47E", scope: !980, file: !4, line: 10, type: !981, scopeLine: 10, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !59, retainedNodes: !985)
!980 = !DINamespace(name: "main", scope: !884)
!981 = !DISubroutineType(types: !982)
!982 = !{null, !983, !73}
!983 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&mut {closure#0}", baseType: !984, size: 32, align: 32, dwarfAddressSpace: 0)
!984 = !DICompositeType(tag: DW_TAG_structure_type, name: "{closure#0}", scope: !980, file: !6, align: 8, elements: !59, templateParams: !59, identifier: "89e2bf31538dd3ddbaa293d20154b443")
!985 = !{!986, !987}
!986 = !DILocalVariable(name: "i", arg: 2, scope: !979, file: !4, line: 10, type: !73)
!987 = !DILocalVariable(arg: 1, scope: !979, file: !4, line: 10, type: !983)
!988 = !DILocation(line: 10, column: 1, scope: !979)
!989 = !DILocation(line: 108, column: 27, scope: !508, inlinedAt: !990)
!990 = distinct !DILocation(line: 10, column: 1, scope: !979)
!991 = !DILocation(line: 1913, column: 22, scope: !516, inlinedAt: !992)
!992 = distinct !DILocation(line: 109, column: 9, scope: !508, inlinedAt: !990)
!993 = !DILocation(line: 10, column: 62, scope: !979)
!994 = distinct !DISubprogram(name: "{closure#1}", linkageName: "_ZN6locals3app8rtic_ext4main28_$u7b$$u7b$closure$u7d$$u7d$17h55422711175e8f91E", scope: !980, file: !4, line: 10, type: !995, scopeLine: 10, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !59, retainedNodes: !999)
!995 = !DISubroutineType(types: !996)
!996 = !{null, !997, !73}
!997 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&mut {closure#1}", baseType: !998, size: 32, align: 32, dwarfAddressSpace: 0)
!998 = !DICompositeType(tag: DW_TAG_structure_type, name: "{closure#1}", scope: !980, file: !6, align: 8, elements: !59, templateParams: !59, identifier: "903ae31ef50e18c0a4dde2d496655ec3")
!999 = !{!1000, !1001}
!1000 = !DILocalVariable(name: "i", arg: 2, scope: !994, file: !4, line: 10, type: !73)
!1001 = !DILocalVariable(arg: 1, scope: !994, file: !4, line: 10, type: !997)
!1002 = !DILocation(line: 10, column: 1, scope: !994)
!1003 = !DILocation(line: 108, column: 27, scope: !508, inlinedAt: !1004)
!1004 = distinct !DILocation(line: 10, column: 1, scope: !994)
!1005 = !DILocation(line: 1913, column: 22, scope: !516, inlinedAt: !1006)
!1006 = distinct !DILocation(line: 109, column: 9, scope: !508, inlinedAt: !1004)
!1007 = !DILocation(line: 10, column: 62, scope: !994)
!1008 = distinct !DISubprogram(name: "__rtic_init_resources<locals::app::rtic_ext::main::{closure#2}>", linkageName: "_ZN6locals3app8rtic_ext4main21__rtic_init_resources17h2f1eea4e8bb2e3a9E", scope: !980, file: !4, line: 10, type: !1009, scopeLine: 10, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !1016, retainedNodes: !1014)
!1009 = !DISubroutineType(types: !1010)
!1010 = !{null, !1011}
!1011 = !DICompositeType(tag: DW_TAG_structure_type, name: "{closure#2}", scope: !980, file: !6, align: 8, elements: !1012, templateParams: !59, identifier: "2ccfe605a8d5f7f081e962d3b6f9bfd3")
!1012 = !{!1013}
!1013 = !DIDerivedType(tag: DW_TAG_member, name: "core", scope: !1011, file: !6, baseType: !888, align: 8)
!1014 = !{!1015}
!1015 = !DILocalVariable(name: "f", arg: 1, scope: !1008, file: !4, line: 10, type: !1011)
!1016 = !{!1017}
!1017 = !DITemplateTypeParameter(name: "F", type: !1011)
!1018 = !DILocation(line: 10, column: 1, scope: !1008)
!1019 = !DILocation(line: 10, column: 62, scope: !1008)
!1020 = distinct !DISubprogram(name: "{closure#2}", linkageName: "_ZN6locals3app8rtic_ext4main28_$u7b$$u7b$closure$u7d$$u7d$17hfd06d08bbe916045E", scope: !980, file: !4, line: 10, type: !1009, scopeLine: 10, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !59, retainedNodes: !1021)
!1021 = !{!1022, !1023, !1026, !1032}
!1022 = !DILocalVariable(name: "core", scope: !1020, file: !4, line: 10, type: !888, align: 1)
!1023 = !DILocalVariable(name: "shared_resources", scope: !1024, file: !4, line: 10, type: !1025, align: 1)
!1024 = distinct !DILexicalBlock(scope: !1020, file: !4, line: 10, column: 1)
!1025 = !DICompositeType(tag: DW_TAG_structure_type, name: "Shared", scope: !2, file: !6, align: 8, elements: !59, templateParams: !59, identifier: "883baa3ccdee4ab1133f2756a85c65ba")
!1026 = !DILocalVariable(name: "local_resources", scope: !1024, file: !4, line: 10, type: !1027, align: 8)
!1027 = !DICompositeType(tag: DW_TAG_structure_type, name: "Local", scope: !2, file: !6, size: 192, align: 64, elements: !1028, templateParams: !59, identifier: "d2b66ff17ec362eff19f0ca40848cc83")
!1028 = !{!1029, !1030, !1031}
!1029 = !DIDerivedType(tag: DW_TAG_member, name: "local_to_foo", scope: !1027, file: !6, baseType: !26, size: 64, align: 64)
!1030 = !DIDerivedType(tag: DW_TAG_member, name: "local_to_bar", scope: !1027, file: !6, baseType: !26, size: 64, align: 64, offset: 64)
!1031 = !DIDerivedType(tag: DW_TAG_member, name: "local_to_idle", scope: !1027, file: !6, baseType: !26, size: 64, align: 64, offset: 128)
!1032 = !DILocalVariable(name: "monotonics", scope: !1024, file: !4, line: 10, type: !1033, align: 1)
!1033 = !DICompositeType(tag: DW_TAG_structure_type, name: "__rtic_internal_Monotonics", scope: !2, file: !6, align: 8, elements: !59, templateParams: !59, identifier: "c6dd3e5f3c0922874d182d9f27dc6a1")
!1034 = !DILocation(line: 10, column: 1, scope: !1020)
!1035 = !DILocation(line: 10, column: 1, scope: !1024)
!1036 = !DILocalVariable(name: "core", arg: 1, scope: !1037, file: !4, line: 10, type: !888)
!1037 = distinct !DISubprogram(name: "new", linkageName: "_ZN6locals3app28__rtic_internal_init_Context3new17hc706a1ea4a341d7bE", scope: !1038, file: !4, line: 10, type: !1050, scopeLine: 10, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !59, retainedNodes: !1052)
!1038 = !DICompositeType(tag: DW_TAG_structure_type, name: "__rtic_internal_init_Context", scope: !2, file: !6, align: 8, elements: !1039, templateParams: !59, identifier: "d1dc169b65f93786a6cbcedbe7b29dd0")
!1039 = !{!1040, !1041, !1042}
!1040 = !DIDerivedType(tag: DW_TAG_member, name: "core", scope: !1038, file: !6, baseType: !888, align: 8)
!1041 = !DIDerivedType(tag: DW_TAG_member, name: "device", scope: !1038, file: !6, baseType: !422, align: 8)
!1042 = !DIDerivedType(tag: DW_TAG_member, name: "cs", scope: !1038, file: !6, baseType: !1043, align: 8)
!1043 = !DICompositeType(tag: DW_TAG_structure_type, name: "CriticalSection", scope: !500, file: !6, align: 8, elements: !1044, templateParams: !59, identifier: "e7c9c7d4bc1c538ef0c322a85314a7e3")
!1044 = !{!1045}
!1045 = !DIDerivedType(tag: DW_TAG_member, name: "_0", scope: !1043, file: !6, baseType: !1046, align: 8)
!1046 = !DICompositeType(tag: DW_TAG_structure_type, name: "PhantomData<&()>", scope: !896, file: !6, align: 8, elements: !59, templateParams: !1047, identifier: "268c6ad0f82a20a7b86029beb1bf02a5")
!1047 = !{!1048}
!1048 = !DITemplateTypeParameter(name: "T", type: !1049)
!1049 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&()", baseType: !20, size: 32, align: 32, dwarfAddressSpace: 0)
!1050 = !DISubroutineType(types: !1051)
!1051 = !{null, !888}
!1052 = !{!1036}
!1053 = !DILocation(line: 10, column: 1, scope: !1037, inlinedAt: !1054)
!1054 = distinct !DILocation(line: 10, column: 1, scope: !1020)
!1055 = !{!1056}
!1056 = distinct !{!1056, !1057, !"_ZN6locals3app4init17hc7786e86197456adE: argument 0"}
!1057 = distinct !{!1057, !"_ZN6locals3app4init17hc7786e86197456adE"}
!1058 = !DILocalVariable(arg: 1, scope: !1059, file: !4, line: 26, type: !1038)
!1059 = distinct !DISubprogram(name: "init", linkageName: "_ZN6locals3app4init17hc7786e86197456adE", scope: !2, file: !4, line: 10, type: !1060, scopeLine: 10, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !59, retainedNodes: !1067)
!1060 = !DISubroutineType(types: !1061)
!1061 = !{!1062, !1038}
!1062 = !DICompositeType(tag: DW_TAG_structure_type, name: "(locals::app::Shared, locals::app::Local, locals::app::__rtic_internal_Monotonics)", file: !6, size: 192, align: 64, elements: !1063, templateParams: !59, identifier: "9f928c23f0d5e54fe2b585ae766f74a4")
!1063 = !{!1064, !1065, !1066}
!1064 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !1062, file: !6, baseType: !1025, align: 8)
!1065 = !DIDerivedType(tag: DW_TAG_member, name: "__1", scope: !1062, file: !6, baseType: !1027, size: 192, align: 64)
!1066 = !DIDerivedType(tag: DW_TAG_member, name: "__2", scope: !1062, file: !6, baseType: !1033, align: 8, offset: 192)
!1067 = !{!1058}
!1068 = !DILocation(line: 26, column: 13, scope: !1059, inlinedAt: !1069)
!1069 = distinct !DILocation(line: 10, column: 1, scope: !1020)
!1070 = !DILocation(line: 27, column: 9, scope: !1059, inlinedAt: !1069)
!1071 = !DILocation(line: 28, column: 9, scope: !1059, inlinedAt: !1069)
!1072 = !DILocation(line: 33, column: 13, scope: !1059, inlinedAt: !1069)
!1073 = !DILocation(line: 30, column: 9, scope: !1059, inlinedAt: !1069)
!1074 = !DILocation(line: 108, column: 27, scope: !818, inlinedAt: !1075)
!1075 = distinct !DILocation(line: 10, column: 1, scope: !1024)
!1076 = !DILocation(line: 1913, column: 22, scope: !831, inlinedAt: !1077)
!1077 = distinct !DILocation(line: 109, column: 9, scope: !818, inlinedAt: !1075)
!1078 = !DILocalVariable(name: "val", arg: 1, scope: !1079, file: !459, line: 296, type: !26)
!1079 = distinct !DISubprogram(name: "new<i64>", linkageName: "_ZN4core3mem12maybe_uninit20MaybeUninit$LT$T$GT$3new17h8c3169a020f4772fE", scope: !15, file: !459, line: 296, type: !1080, scopeLine: 296, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !27, retainedNodes: !1082)
!1080 = !DISubroutineType(types: !1081)
!1081 = !{!15, !26}
!1082 = !{!1078}
!1083 = !DILocation(line: 296, column: 22, scope: !1079, inlinedAt: !1084)
!1084 = distinct !DILocation(line: 10, column: 1, scope: !1024)
!1085 = !DILocalVariable(name: "value", arg: 1, scope: !1086, file: !1087, line: 69, type: !26)
!1086 = distinct !DISubprogram(name: "new<i64>", linkageName: "_ZN4core3mem13manually_drop21ManuallyDrop$LT$T$GT$3new17h363c15a9092a56b4E", scope: !22, file: !1087, line: 69, type: !1088, scopeLine: 69, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !27, retainedNodes: !1090)
!1087 = !DIFile(filename: "/rustc/9d1b2106e23b1abd32fce1f17267604a5102f57a/library/core/src/mem/manually_drop.rs", directory: "", checksumkind: CSK_MD5, checksum: "c3a40f6695c3596e8a0c4d09d7cf455a")
!1088 = !DISubroutineType(types: !1089)
!1089 = !{!22, !26}
!1090 = !{!1085}
!1091 = !DILocation(line: 69, column: 22, scope: !1086, inlinedAt: !1092)
!1092 = distinct !DILocation(line: 297, column: 30, scope: !1079, inlinedAt: !1084)
!1093 = !DILocation(line: 70, column: 9, scope: !1086, inlinedAt: !1092)
!1094 = !DILocation(line: 71, column: 6, scope: !1086, inlinedAt: !1092)
!1095 = !DILocation(line: 297, column: 9, scope: !1079, inlinedAt: !1084)
!1096 = !DILocation(line: 298, column: 6, scope: !1079, inlinedAt: !1084)
!1097 = !DILocalVariable(name: "self", arg: 1, scope: !1098, file: !469, line: 1061, type: !821)
!1098 = distinct !DISubprogram(name: "write<core::mem::maybe_uninit::MaybeUninit<i64>>", linkageName: "_ZN4core3ptr7mut_ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$5write17h29a16c2641b9f66aE", scope: !470, file: !469, line: 1061, type: !1099, scopeLine: 1061, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !29, retainedNodes: !1101)
!1099 = !DISubroutineType(types: !1100)
!1100 = !{null, !821, !15}
!1101 = !{!1097, !1102}
!1102 = !DILocalVariable(name: "val", arg: 2, scope: !1098, file: !469, line: 1061, type: !15)
!1103 = !DILocation(line: 1061, column: 31, scope: !1098, inlinedAt: !1104)
!1104 = distinct !DILocation(line: 10, column: 1, scope: !1024)
!1105 = !DILocation(line: 1061, column: 37, scope: !1098, inlinedAt: !1104)
!1106 = !DILocation(line: 1066, column: 18, scope: !1098, inlinedAt: !1104)
!1107 = !DILocation(line: 108, column: 27, scope: !818, inlinedAt: !1108)
!1108 = distinct !DILocation(line: 10, column: 1, scope: !1024)
!1109 = !DILocation(line: 1913, column: 22, scope: !831, inlinedAt: !1110)
!1110 = distinct !DILocation(line: 109, column: 9, scope: !818, inlinedAt: !1108)
!1111 = !DILocation(line: 296, column: 22, scope: !1079, inlinedAt: !1112)
!1112 = distinct !DILocation(line: 10, column: 1, scope: !1024)
!1113 = !DILocation(line: 69, column: 22, scope: !1086, inlinedAt: !1114)
!1114 = distinct !DILocation(line: 297, column: 30, scope: !1079, inlinedAt: !1112)
!1115 = !DILocation(line: 70, column: 9, scope: !1086, inlinedAt: !1114)
!1116 = !DILocation(line: 71, column: 6, scope: !1086, inlinedAt: !1114)
!1117 = !DILocation(line: 297, column: 9, scope: !1079, inlinedAt: !1112)
!1118 = !DILocation(line: 298, column: 6, scope: !1079, inlinedAt: !1112)
!1119 = !DILocation(line: 1061, column: 31, scope: !1098, inlinedAt: !1120)
!1120 = distinct !DILocation(line: 10, column: 1, scope: !1024)
!1121 = !DILocation(line: 1061, column: 37, scope: !1098, inlinedAt: !1120)
!1122 = !DILocation(line: 1066, column: 18, scope: !1098, inlinedAt: !1120)
!1123 = !DILocation(line: 108, column: 27, scope: !818, inlinedAt: !1124)
!1124 = distinct !DILocation(line: 10, column: 1, scope: !1024)
!1125 = !DILocation(line: 1913, column: 22, scope: !831, inlinedAt: !1126)
!1126 = distinct !DILocation(line: 109, column: 9, scope: !818, inlinedAt: !1124)
!1127 = !DILocation(line: 296, column: 22, scope: !1079, inlinedAt: !1128)
!1128 = distinct !DILocation(line: 10, column: 1, scope: !1024)
!1129 = !DILocation(line: 69, column: 22, scope: !1086, inlinedAt: !1130)
!1130 = distinct !DILocation(line: 297, column: 30, scope: !1079, inlinedAt: !1128)
!1131 = !DILocation(line: 70, column: 9, scope: !1086, inlinedAt: !1130)
!1132 = !DILocation(line: 71, column: 6, scope: !1086, inlinedAt: !1130)
!1133 = !DILocation(line: 297, column: 9, scope: !1079, inlinedAt: !1128)
!1134 = !DILocation(line: 298, column: 6, scope: !1079, inlinedAt: !1128)
!1135 = !DILocation(line: 1061, column: 31, scope: !1098, inlinedAt: !1136)
!1136 = distinct !DILocation(line: 10, column: 1, scope: !1024)
!1137 = !DILocation(line: 1061, column: 37, scope: !1098, inlinedAt: !1136)
!1138 = !DILocation(line: 1066, column: 18, scope: !1098, inlinedAt: !1136)
!1139 = !DILocation(line: 10, column: 62, scope: !1020)
!1140 = distinct !DISubprogram(name: "pend<lm3s6965::Interrupt>", linkageName: "_ZN4rtic4pend17h9dbd054a9ea733f9E", scope: !7, file: !439, line: 61, type: !1141, scopeLine: 61, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !1145, retainedNodes: !1143)
!1141 = !DISubroutineType(types: !1142)
!1142 = !{null, !157}
!1143 = !{!1144}
!1144 = !DILocalVariable(name: "interrupt", arg: 1, scope: !1140, file: !439, line: 61, type: !157)
!1145 = !{!1146}
!1146 = !DITemplateTypeParameter(name: "I", type: !157)
!1147 = !DILocation(line: 61, column: 16, scope: !1140)
!1148 = !DILocation(line: 65, column: 5, scope: !1140)
!1149 = !DILocation(line: 66, column: 2, scope: !1140)
!1150 = distinct !DISubprogram(name: "unmask<lm3s6965::Interrupt>", linkageName: "_ZN8cortex_m10peripheral4nvic44_$LT$impl$u20$cortex_m..peripheral..NVIC$GT$6unmask17h825ea550b82b57f2E", scope: !932, file: !1151, line: 115, type: !1141, scopeLine: 115, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !1145, retainedNodes: !1152)
!1151 = !DIFile(filename: "/home/norlen/.cargo/registry/src/github.com-1ecc6299db9ec823/cortex-m-0.7.4/src/peripheral/nvic.rs", directory: "", checksumkind: CSK_MD5, checksum: "be3e65b164f6698dc15061bb869c627f")
!1152 = !{!1153, !1154}
!1153 = !DILocalVariable(name: "interrupt", arg: 1, scope: !1150, file: !1151, line: 115, type: !157)
!1154 = !DILocalVariable(name: "nr", scope: !1155, file: !1151, line: 119, type: !558, align: 2)
!1155 = distinct !DILexicalBlock(scope: !1150, file: !1151, line: 119, column: 9)
!1156 = !DILocation(line: 115, column: 29, scope: !1150)
!1157 = !DILocation(line: 119, column: 18, scope: !1150)
!1158 = !DILocation(line: 119, column: 13, scope: !1155)
!1159 = !DILocation(line: 121, column: 11, scope: !1155)
!1160 = !DILocation(line: 121, column: 41, scope: !1155)
!1161 = !DILocation(line: 121, column: 29, scope: !1155)
!1162 = !DILocation(line: 121, column: 9, scope: !1155)
!1163 = !DILocation(line: 121, column: 62, scope: !1155)
!1164 = !DILocation(line: 121, column: 57, scope: !1155)
!1165 = !DILocalVariable(name: "self", arg: 1, scope: !1166, file: !1167, line: 82, type: !1184)
!1166 = distinct !DISubprogram(name: "write<u32>", linkageName: "_ZN17volatile_register11RW$LT$T$GT$5write17hebe3d20704f0ec4eE", scope: !1168, file: !1167, line: 82, type: !1182, scopeLine: 82, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !1180, retainedNodes: !1185)
!1167 = !DIFile(filename: "/home/norlen/.cargo/registry/src/github.com-1ecc6299db9ec823/volatile-register-0.2.1/src/lib.rs", directory: "", checksumkind: CSK_MD5, checksum: "ef14c2dfa677d5b0581851d85d202b1d")
!1168 = !DICompositeType(tag: DW_TAG_structure_type, name: "RW<u32>", scope: !1169, file: !6, size: 32, align: 32, elements: !1170, templateParams: !1180, identifier: "b364699c95df0a8377819319f15c3ba6")
!1169 = !DINamespace(name: "volatile_register", scope: null)
!1170 = !{!1171}
!1171 = !DIDerivedType(tag: DW_TAG_member, name: "register", scope: !1168, file: !6, baseType: !1172, size: 32, align: 32)
!1172 = !DICompositeType(tag: DW_TAG_structure_type, name: "VolatileCell<u32>", scope: !1173, file: !6, size: 32, align: 32, elements: !1174, templateParams: !1180, identifier: "f0201719bf8fd70356c5a79703470d2f")
!1173 = !DINamespace(name: "vcell", scope: null)
!1174 = !{!1175}
!1175 = !DIDerivedType(tag: DW_TAG_member, name: "value", scope: !1172, file: !6, baseType: !1176, size: 32, align: 32)
!1176 = !DICompositeType(tag: DW_TAG_structure_type, name: "UnsafeCell<u32>", scope: !11, file: !6, size: 32, align: 32, elements: !1177, templateParams: !1180, identifier: "1b5523fdb540edf96c91bafcb99db841")
!1177 = !{!1178}
!1178 = !DIDerivedType(tag: DW_TAG_member, name: "value", scope: !1176, file: !6, baseType: !1179, size: 32, align: 32)
!1179 = !DIBasicType(name: "u32", size: 32, encoding: DW_ATE_unsigned)
!1180 = !{!1181}
!1181 = !DITemplateTypeParameter(name: "T", type: !1179)
!1182 = !DISubroutineType(types: !1183)
!1183 = !{null, !1184, !1179}
!1184 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&RW<u32>", baseType: !1168, size: 32, align: 32, dwarfAddressSpace: 0)
!1185 = !{!1165, !1186}
!1186 = !DILocalVariable(name: "value", arg: 2, scope: !1166, file: !1167, line: 82, type: !1179)
!1187 = !DILocation(line: 82, column: 25, scope: !1166, inlinedAt: !1188)
!1188 = distinct !DILocation(line: 121, column: 9, scope: !1155)
!1189 = !DILocation(line: 82, column: 32, scope: !1166, inlinedAt: !1188)
!1190 = !DILocalVariable(name: "self", arg: 1, scope: !1191, file: !1192, line: 38, type: !1195)
!1191 = distinct !DISubprogram(name: "set<u32>", linkageName: "_ZN5vcell21VolatileCell$LT$T$GT$3set17h2afed0183150c680E", scope: !1172, file: !1192, line: 38, type: !1193, scopeLine: 38, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !1180, retainedNodes: !1196)
!1192 = !DIFile(filename: "/home/norlen/.cargo/registry/src/github.com-1ecc6299db9ec823/vcell-0.1.3/src/lib.rs", directory: "", checksumkind: CSK_MD5, checksum: "2729a5031fa775f1a3ff44a4e42d4e87")
!1193 = !DISubroutineType(types: !1194)
!1194 = !{null, !1195, !1179}
!1195 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&VolatileCell<u32>", baseType: !1172, size: 32, align: 32, dwarfAddressSpace: 0)
!1196 = !{!1190, !1197}
!1197 = !DILocalVariable(name: "value", arg: 2, scope: !1191, file: !1192, line: 38, type: !1179)
!1198 = !DILocation(line: 38, column: 16, scope: !1191, inlinedAt: !1199)
!1199 = distinct !DILocation(line: 83, column: 9, scope: !1166, inlinedAt: !1188)
!1200 = !DILocation(line: 38, column: 23, scope: !1191, inlinedAt: !1199)
!1201 = !DILocalVariable(name: "self", arg: 1, scope: !1202, file: !449, line: 1913, type: !1206)
!1202 = distinct !DISubprogram(name: "get<u32>", linkageName: "_ZN4core4cell19UnsafeCell$LT$T$GT$3get17hba911f97743e157fE", scope: !1176, file: !449, line: 1913, type: !1203, scopeLine: 1913, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !1180, retainedNodes: !1207)
!1203 = !DISubroutineType(types: !1204)
!1204 = !{!1205, !1206}
!1205 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*mut u32", baseType: !1179, size: 32, align: 32, dwarfAddressSpace: 0)
!1206 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&UnsafeCell<u32>", baseType: !1176, size: 32, align: 32, dwarfAddressSpace: 0)
!1207 = !{!1201}
!1208 = !DILocation(line: 1913, column: 22, scope: !1202, inlinedAt: !1209)
!1209 = distinct !DILocation(line: 41, column: 38, scope: !1191, inlinedAt: !1199)
!1210 = !DILocation(line: 41, column: 18, scope: !1191, inlinedAt: !1199)
!1211 = !DILocation(line: 122, column: 6, scope: !1150)
!1212 = distinct !DISubprogram(name: "pend<lm3s6965::Interrupt>", linkageName: "_ZN8cortex_m10peripheral4nvic44_$LT$impl$u20$cortex_m..peripheral..NVIC$GT$4pend17h0c9c6720008e79b3E", scope: !932, file: !1151, line: 192, type: !1141, scopeLine: 192, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !1145, retainedNodes: !1213)
!1213 = !{!1214, !1215}
!1214 = !DILocalVariable(name: "interrupt", arg: 1, scope: !1212, file: !1151, line: 192, type: !157)
!1215 = !DILocalVariable(name: "nr", scope: !1216, file: !1151, line: 196, type: !558, align: 2)
!1216 = distinct !DILexicalBlock(scope: !1212, file: !1151, line: 196, column: 9)
!1217 = !DILocation(line: 192, column: 20, scope: !1212)
!1218 = !DILocation(line: 196, column: 18, scope: !1212)
!1219 = !DILocation(line: 196, column: 13, scope: !1216)
!1220 = !DILocation(line: 199, column: 20, scope: !1216)
!1221 = !DILocation(line: 199, column: 50, scope: !1216)
!1222 = !DILocation(line: 199, column: 38, scope: !1216)
!1223 = !DILocation(line: 199, column: 18, scope: !1216)
!1224 = !DILocation(line: 199, column: 71, scope: !1216)
!1225 = !DILocation(line: 199, column: 66, scope: !1216)
!1226 = !DILocation(line: 82, column: 25, scope: !1166, inlinedAt: !1227)
!1227 = distinct !DILocation(line: 199, column: 18, scope: !1216)
!1228 = !DILocation(line: 82, column: 32, scope: !1166, inlinedAt: !1227)
!1229 = !DILocation(line: 38, column: 16, scope: !1191, inlinedAt: !1230)
!1230 = distinct !DILocation(line: 83, column: 9, scope: !1166, inlinedAt: !1227)
!1231 = !DILocation(line: 38, column: 23, scope: !1191, inlinedAt: !1230)
!1232 = !DILocation(line: 1913, column: 22, scope: !1202, inlinedAt: !1233)
!1233 = distinct !DILocation(line: 41, column: 38, scope: !1191, inlinedAt: !1230)
!1234 = !DILocation(line: 41, column: 18, scope: !1191, inlinedAt: !1230)
!1235 = !DILocation(line: 200, column: 6, scope: !1212)
!1236 = distinct !DISubprogram(name: "set_priority<lm3s6965::Interrupt>", linkageName: "_ZN8cortex_m10peripheral4nvic44_$LT$impl$u20$cortex_m..peripheral..NVIC$GT$12set_priority17h755c18aead94bf5aE", scope: !932, file: !1151, line: 215, type: !1237, scopeLine: 215, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !1145, retainedNodes: !1240)
!1237 = !DISubroutineType(types: !1238)
!1238 = !{null, !1239, !157, !73}
!1239 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&mut NVIC", baseType: !932, size: 32, align: 32, dwarfAddressSpace: 0)
!1240 = !{!1241, !1242, !1243, !1244}
!1241 = !DILocalVariable(name: "self", arg: 1, scope: !1236, file: !1151, line: 215, type: !1239)
!1242 = !DILocalVariable(name: "interrupt", arg: 2, scope: !1236, file: !1151, line: 215, type: !157)
!1243 = !DILocalVariable(name: "prio", arg: 3, scope: !1236, file: !1151, line: 215, type: !73)
!1244 = !DILocalVariable(name: "nr", scope: !1245, file: !1151, line: 221, type: !558, align: 2)
!1245 = distinct !DILexicalBlock(scope: !1236, file: !1151, line: 221, column: 13)
!1246 = !DILocation(line: 215, column: 35, scope: !1236)
!1247 = !DILocation(line: 215, column: 46, scope: !1236)
!1248 = !DILocation(line: 215, column: 60, scope: !1236)
!1249 = !DILocation(line: 221, column: 22, scope: !1236)
!1250 = !DILocation(line: 221, column: 17, scope: !1245)
!1251 = !DILocalVariable(name: "self", arg: 1, scope: !1252, file: !1253, line: 555, type: !1300)
!1252 = distinct !DISubprogram(name: "deref", linkageName: "_ZN70_$LT$cortex_m..peripheral..NVIC$u20$as$u20$core..ops..deref..Deref$GT$5deref17h5762d2d11731ddbbE", scope: !1254, file: !1253, line: 555, type: !1255, scopeLine: 555, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !59, retainedNodes: !1301)
!1253 = !DIFile(filename: "/home/norlen/.cargo/registry/src/github.com-1ecc6299db9ec823/cortex-m-0.7.4/src/peripheral/mod.rs", directory: "", checksumkind: CSK_MD5, checksum: "fd1c8d900f3d429490fbd0df3441b93a")
!1254 = !DINamespace(name: "{impl#30}", scope: !889)
!1255 = !DISubroutineType(types: !1256)
!1256 = !{!1257, !1300}
!1257 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&RegisterBlock", baseType: !1258, size: 32, align: 32, dwarfAddressSpace: 0)
!1258 = !DICompositeType(tag: DW_TAG_structure_type, name: "RegisterBlock", scope: !1259, file: !6, size: 28704, align: 32, elements: !1260, templateParams: !59, identifier: "3104445570ca40c296566657fb83c5ba")
!1259 = !DINamespace(name: "nvic", scope: !889)
!1260 = !{!1261, !1265, !1267, !1268, !1269, !1270, !1271, !1272, !1273, !1278, !1282, !1292, !1296}
!1261 = !DIDerivedType(tag: DW_TAG_member, name: "iser", scope: !1258, file: !6, baseType: !1262, size: 512, align: 32)
!1262 = !DICompositeType(tag: DW_TAG_array_type, baseType: !1168, size: 512, align: 32, elements: !1263)
!1263 = !{!1264}
!1264 = !DISubrange(count: 16, lowerBound: 0)
!1265 = !DIDerivedType(tag: DW_TAG_member, name: "_reserved0", scope: !1258, file: !6, baseType: !1266, size: 512, align: 32, offset: 512)
!1266 = !DICompositeType(tag: DW_TAG_array_type, baseType: !1179, size: 512, align: 32, elements: !1263)
!1267 = !DIDerivedType(tag: DW_TAG_member, name: "icer", scope: !1258, file: !6, baseType: !1262, size: 512, align: 32, offset: 1024)
!1268 = !DIDerivedType(tag: DW_TAG_member, name: "_reserved1", scope: !1258, file: !6, baseType: !1266, size: 512, align: 32, offset: 1536)
!1269 = !DIDerivedType(tag: DW_TAG_member, name: "ispr", scope: !1258, file: !6, baseType: !1262, size: 512, align: 32, offset: 2048)
!1270 = !DIDerivedType(tag: DW_TAG_member, name: "_reserved2", scope: !1258, file: !6, baseType: !1266, size: 512, align: 32, offset: 2560)
!1271 = !DIDerivedType(tag: DW_TAG_member, name: "icpr", scope: !1258, file: !6, baseType: !1262, size: 512, align: 32, offset: 3072)
!1272 = !DIDerivedType(tag: DW_TAG_member, name: "_reserved3", scope: !1258, file: !6, baseType: !1266, size: 512, align: 32, offset: 3584)
!1273 = !DIDerivedType(tag: DW_TAG_member, name: "iabr", scope: !1258, file: !6, baseType: !1274, size: 512, align: 32, offset: 4096)
!1274 = !DICompositeType(tag: DW_TAG_array_type, baseType: !1275, size: 512, align: 32, elements: !1263)
!1275 = !DICompositeType(tag: DW_TAG_structure_type, name: "RO<u32>", scope: !1169, file: !6, size: 32, align: 32, elements: !1276, templateParams: !1180, identifier: "75d434ca530659a11265a1ec42d1b791")
!1276 = !{!1277}
!1277 = !DIDerivedType(tag: DW_TAG_member, name: "register", scope: !1275, file: !6, baseType: !1172, size: 32, align: 32)
!1278 = !DIDerivedType(tag: DW_TAG_member, name: "_reserved5", scope: !1258, file: !6, baseType: !1279, size: 1536, align: 32, offset: 4608)
!1279 = !DICompositeType(tag: DW_TAG_array_type, baseType: !1179, size: 1536, align: 32, elements: !1280)
!1280 = !{!1281}
!1281 = !DISubrange(count: 48, lowerBound: 0)
!1282 = !DIDerivedType(tag: DW_TAG_member, name: "ipr", scope: !1258, file: !6, baseType: !1283, size: 3968, align: 8, offset: 6144)
!1283 = !DICompositeType(tag: DW_TAG_array_type, baseType: !1284, size: 3968, align: 8, elements: !1290)
!1284 = !DICompositeType(tag: DW_TAG_structure_type, name: "RW<u8>", scope: !1169, file: !6, size: 8, align: 8, elements: !1285, templateParams: !74, identifier: "664141ded0e2c6754701e113d8a19a8")
!1285 = !{!1286}
!1286 = !DIDerivedType(tag: DW_TAG_member, name: "register", scope: !1284, file: !6, baseType: !1287, size: 8, align: 8)
!1287 = !DICompositeType(tag: DW_TAG_structure_type, name: "VolatileCell<u8>", scope: !1173, file: !6, size: 8, align: 8, elements: !1288, templateParams: !74, identifier: "7f21d7fa7e78cdfc8599cdc163a41f1f")
!1288 = !{!1289}
!1289 = !DIDerivedType(tag: DW_TAG_member, name: "value", scope: !1287, file: !6, baseType: !758, size: 8, align: 8)
!1290 = !{!1291}
!1291 = !DISubrange(count: 496, lowerBound: 0)
!1292 = !DIDerivedType(tag: DW_TAG_member, name: "_reserved6", scope: !1258, file: !6, baseType: !1293, size: 18560, align: 32, offset: 10112)
!1293 = !DICompositeType(tag: DW_TAG_array_type, baseType: !1179, size: 18560, align: 32, elements: !1294)
!1294 = !{!1295}
!1295 = !DISubrange(count: 580, lowerBound: 0)
!1296 = !DIDerivedType(tag: DW_TAG_member, name: "stir", scope: !1258, file: !6, baseType: !1297, size: 32, align: 32, offset: 28672)
!1297 = !DICompositeType(tag: DW_TAG_structure_type, name: "WO<u32>", scope: !1169, file: !6, size: 32, align: 32, elements: !1298, templateParams: !1180, identifier: "ee49fcec07119600c0dfe3c4d4e799f7")
!1298 = !{!1299}
!1299 = !DIDerivedType(tag: DW_TAG_member, name: "register", scope: !1297, file: !6, baseType: !1172, size: 32, align: 32)
!1300 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&NVIC", baseType: !932, size: 32, align: 32, dwarfAddressSpace: 0)
!1301 = !{!1251}
!1302 = !DILocation(line: 555, column: 14, scope: !1252, inlinedAt: !1303)
!1303 = distinct !DILocation(line: 222, column: 13, scope: !1245)
!1304 = !DILocation(line: 222, column: 13, scope: !1245)
!1305 = !DILocation(line: 222, column: 22, scope: !1245)
!1306 = !DILocalVariable(name: "self", arg: 1, scope: !1307, file: !1167, line: 82, type: !1310)
!1307 = distinct !DISubprogram(name: "write<u8>", linkageName: "_ZN17volatile_register11RW$LT$T$GT$5write17hf520e87259d69e49E", scope: !1284, file: !1167, line: 82, type: !1308, scopeLine: 82, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !74, retainedNodes: !1311)
!1308 = !DISubroutineType(types: !1309)
!1309 = !{null, !1310, !73}
!1310 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&RW<u8>", baseType: !1284, size: 32, align: 32, dwarfAddressSpace: 0)
!1311 = !{!1306, !1312}
!1312 = !DILocalVariable(name: "value", arg: 2, scope: !1307, file: !1167, line: 82, type: !73)
!1313 = !DILocation(line: 82, column: 25, scope: !1307, inlinedAt: !1314)
!1314 = distinct !DILocation(line: 222, column: 13, scope: !1245)
!1315 = !DILocation(line: 82, column: 32, scope: !1307, inlinedAt: !1314)
!1316 = !DILocalVariable(name: "self", arg: 1, scope: !1317, file: !1192, line: 38, type: !1320)
!1317 = distinct !DISubprogram(name: "set<u8>", linkageName: "_ZN5vcell21VolatileCell$LT$T$GT$3set17h2dd8bc2ca01e4abfE", scope: !1287, file: !1192, line: 38, type: !1318, scopeLine: 38, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !74, retainedNodes: !1321)
!1318 = !DISubroutineType(types: !1319)
!1319 = !{null, !1320, !73}
!1320 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&VolatileCell<u8>", baseType: !1287, size: 32, align: 32, dwarfAddressSpace: 0)
!1321 = !{!1316, !1322}
!1322 = !DILocalVariable(name: "value", arg: 2, scope: !1317, file: !1192, line: 38, type: !73)
!1323 = !DILocation(line: 38, column: 16, scope: !1317, inlinedAt: !1324)
!1324 = distinct !DILocation(line: 83, column: 9, scope: !1307, inlinedAt: !1314)
!1325 = !DILocation(line: 38, column: 23, scope: !1317, inlinedAt: !1324)
!1326 = !DILocalVariable(name: "self", arg: 1, scope: !1327, file: !449, line: 1913, type: !1331)
!1327 = distinct !DISubprogram(name: "get<u8>", linkageName: "_ZN4core4cell19UnsafeCell$LT$T$GT$3get17h0a729abf95a4da0fE", scope: !758, file: !449, line: 1913, type: !1328, scopeLine: 1913, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !74, retainedNodes: !1332)
!1328 = !DISubroutineType(types: !1329)
!1329 = !{!1330, !1331}
!1330 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*mut u8", baseType: !73, size: 32, align: 32, dwarfAddressSpace: 0)
!1331 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&UnsafeCell<u8>", baseType: !758, size: 32, align: 32, dwarfAddressSpace: 0)
!1332 = !{!1326}
!1333 = !DILocation(line: 1913, column: 22, scope: !1327, inlinedAt: !1334)
!1334 = distinct !DILocation(line: 41, column: 38, scope: !1317, inlinedAt: !1324)
!1335 = !DILocation(line: 41, column: 18, scope: !1317, inlinedAt: !1324)
!1336 = !DILocation(line: 234, column: 6, scope: !1236)
!1337 = distinct !DISubprogram(name: "steal", linkageName: "_ZN8cortex_m10peripheral11Peripherals5steal17hf4660d70e6ed8445E", scope: !888, file: !1253, line: 179, type: !341, scopeLine: 179, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !59, retainedNodes: !59)
!1338 = !DILocation(line: 180, column: 9, scope: !1337)
!1339 = !DILocation(line: 231, column: 6, scope: !1337)
!1340 = distinct !DISubprogram(name: "read", linkageName: "_ZN8cortex_m8register7basepri4read17hab22038c38f4f26dE", scope: !1342, file: !1341, line: 5, type: !1343, scopeLine: 5, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !59, retainedNodes: !59)
!1341 = !DIFile(filename: "/home/norlen/.cargo/registry/src/github.com-1ecc6299db9ec823/cortex-m-0.7.4/src/register/basepri.rs", directory: "", checksumkind: CSK_MD5, checksum: "38bbbbf95426b8cf174ba8b84646374a")
!1342 = !DINamespace(name: "basepri", scope: !205)
!1343 = !DISubroutineType(types: !1344)
!1344 = !{!73}
!1345 = !DILocation(line: 6, column: 5, scope: !1340)
!1346 = !DILocation(line: 7, column: 2, scope: !1340)
!1347 = distinct !DISubprogram(name: "write", linkageName: "_ZN8cortex_m8register7basepri5write17h8c83f3cc5950a711E", scope: !1342, file: !1341, line: 14, type: !1348, scopeLine: 14, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !59, retainedNodes: !1350)
!1348 = !DISubroutineType(types: !1349)
!1349 = !{null, !73}
!1350 = !{!1351}
!1351 = !DILocalVariable(name: "basepri", arg: 1, scope: !1347, file: !1341, line: 14, type: !73)
!1352 = !DILocation(line: 14, column: 21, scope: !1347)
!1353 = !DILocation(line: 22, column: 9, scope: !1347)
!1354 = !DILocation(line: 24, column: 2, scope: !1347)
!1355 = distinct !DISubprogram(name: "is_active", linkageName: "_ZN8cortex_m8register7primask7Primask9is_active17h26a0e7131c940062E", scope: !203, file: !1356, line: 15, type: !1357, scopeLine: 15, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !59, retainedNodes: !1360)
!1356 = !DIFile(filename: "/home/norlen/.cargo/registry/src/github.com-1ecc6299db9ec823/cortex-m-0.7.4/src/register/primask.rs", directory: "", checksumkind: CSK_MD5, checksum: "5296ba0e064b39ddddfd20b600f4a93e")
!1357 = !DISubroutineType(types: !1358)
!1358 = !{!1359, !203}
!1359 = !DIBasicType(name: "bool", size: 8, encoding: DW_ATE_boolean)
!1360 = !{!1361}
!1361 = !DILocalVariable(name: "self", arg: 1, scope: !1355, file: !1356, line: 15, type: !203)
!1362 = !DILocation(line: 15, column: 22, scope: !1355)
!1363 = !DILocation(line: 16, column: 9, scope: !1355)
!1364 = !DILocation(line: 17, column: 6, scope: !1355)
!1365 = distinct !DISubprogram(name: "read", linkageName: "_ZN8cortex_m8register7primask4read17he7f40456c7aa8a00E", scope: !204, file: !1356, line: 28, type: !1366, scopeLine: 28, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !59, retainedNodes: !1368)
!1366 = !DISubroutineType(types: !1367)
!1367 = !{!203}
!1368 = !{!1369}
!1369 = !DILocalVariable(name: "r", scope: !1370, file: !1356, line: 29, type: !1179, align: 4)
!1370 = distinct !DILexicalBlock(scope: !1365, file: !1356, line: 29, column: 5)
!1371 = !DILocation(line: 29, column: 18, scope: !1365)
!1372 = !DILocation(line: 29, column: 9, scope: !1370)
!1373 = !DILocation(line: 30, column: 8, scope: !1370)
!1374 = !DILocation(line: 33, column: 9, scope: !1370)
!1375 = !DILocation(line: 30, column: 5, scope: !1370)
!1376 = !DILocation(line: 31, column: 9, scope: !1370)
!1377 = !DILocation(line: 35, column: 2, scope: !1365)
!1378 = distinct !DISubprogram(name: "eq", linkageName: "_ZN77_$LT$cortex_m..register..primask..Primask$u20$as$u20$core..cmp..PartialEq$GT$2eq17h65a53084a9ea4fb7E", scope: !1379, file: !1356, line: 4, type: !1380, scopeLine: 4, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !59, retainedNodes: !1383)
!1379 = !DINamespace(name: "{impl#7}", scope: !204)
!1380 = !DISubroutineType(types: !1381)
!1381 = !{!1359, !1382, !1382}
!1382 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&Primask", baseType: !203, size: 32, align: 32, dwarfAddressSpace: 0)
!1383 = !{!1384, !1385, !1386, !1389}
!1384 = !DILocalVariable(name: "self", arg: 1, scope: !1378, file: !1356, line: 4, type: !1382)
!1385 = !DILocalVariable(name: "other", arg: 2, scope: !1378, file: !1356, line: 4, type: !1382)
!1386 = !DILocalVariable(name: "__self_vi", scope: !1387, file: !1356, line: 4, type: !1388, align: 4)
!1387 = distinct !DILexicalBlock(scope: !1378, file: !1356, line: 4, column: 34)
!1388 = !DIBasicType(name: "isize", size: 32, encoding: DW_ATE_signed)
!1389 = !DILocalVariable(name: "__arg_1_vi", scope: !1390, file: !1356, line: 4, type: !1388, align: 4)
!1390 = distinct !DILexicalBlock(scope: !1387, file: !1356, line: 4, column: 34)
!1391 = !DILocation(line: 4, column: 34, scope: !1378)
!1392 = !DILocation(line: 4, column: 34, scope: !1387)
!1393 = !DILocation(line: 4, column: 34, scope: !1390)
!1394 = !DILocation(line: 4, column: 43, scope: !1378)
!1395 = distinct !DISubprogram(name: "replace<u8>", linkageName: "_ZN4core3mem7replace17h6f6f0ffe151c8c1aE", scope: !17, file: !1396, line: 834, type: !1397, scopeLine: 834, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !74, retainedNodes: !1400)
!1396 = !DIFile(filename: "/rustc/9d1b2106e23b1abd32fce1f17267604a5102f57a/library/core/src/mem/mod.rs", directory: "", checksumkind: CSK_MD5, checksum: "f20df2c5b1689721d77b5766c4cf702d")
!1397 = !DISubroutineType(types: !1398)
!1398 = !{!73, !1399, !73}
!1399 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&mut u8", baseType: !73, size: 32, align: 32, dwarfAddressSpace: 0)
!1400 = !{!1401, !1402, !1403}
!1401 = !DILocalVariable(name: "dest", arg: 1, scope: !1395, file: !1396, line: 834, type: !1399)
!1402 = !DILocalVariable(name: "src", arg: 2, scope: !1395, file: !1396, line: 834, type: !73)
!1403 = !DILocalVariable(name: "result", scope: !1404, file: !1396, line: 839, type: !73, align: 1)
!1404 = distinct !DILexicalBlock(scope: !1395, file: !1396, line: 839, column: 9)
!1405 = !DILocation(line: 834, column: 25, scope: !1395)
!1406 = !DILocation(line: 834, column: 39, scope: !1395)
!1407 = !DILocation(line: 839, column: 22, scope: !1395)
!1408 = !DILocation(line: 839, column: 13, scope: !1404)
!1409 = !DILocation(line: 840, column: 9, scope: !1404)
!1410 = !DILocation(line: 843, column: 2, scope: !1395)
!1411 = distinct !DISubprogram(name: "increment<(locals::app::P1_T, u8), 3>", linkageName: "_ZN8heapless4spsc18Queue$LT$T$C$_$GT$9increment17h508ec96413021199E", scope: !116, file: !1412, line: 112, type: !1413, scopeLine: 112, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !140, retainedNodes: !1415)
!1412 = !DIFile(filename: "/home/norlen/.cargo/registry/src/github.com-1ecc6299db9ec823/heapless-0.7.10/src/spsc.rs", directory: "", checksumkind: CSK_MD5, checksum: "d4494f36e24cf37b1ac08f35ca2cd6c4")
!1413 = !DISubroutineType(types: !1414)
!1414 = !{!56, !56}
!1415 = !{!1416}
!1416 = !DILocalVariable(name: "val", arg: 1, scope: !1411, file: !1412, line: 112, type: !56)
!1417 = !DILocation(line: 112, column: 18, scope: !1411)
!1418 = !DILocation(line: 113, column: 9, scope: !1411)
!1419 = !DILocation(line: 114, column: 6, scope: !1411)
!1420 = distinct !DISubprogram(name: "increment<u8, 2>", linkageName: "_ZN8heapless4spsc18Queue$LT$T$C$_$GT$9increment17h62b7694f13bd5c4aE", scope: !43, file: !1412, line: 112, type: !1413, scopeLine: 112, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !74, retainedNodes: !1421)
!1421 = !{!1422}
!1422 = !DILocalVariable(name: "val", arg: 1, scope: !1420, file: !1412, line: 112, type: !56)
!1423 = !DILocation(line: 112, column: 18, scope: !1420)
!1424 = !DILocation(line: 113, column: 9, scope: !1420)
!1425 = !DILocation(line: 114, column: 6, scope: !1420)
!1426 = distinct !DISubprogram(name: "len<u8, 2>", linkageName: "_ZN8heapless4spsc18Queue$LT$T$C$_$GT$3len17h0313f1eb84a18b2bE", scope: !43, file: !1412, line: 136, type: !1427, scopeLine: 136, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !74, retainedNodes: !1430)
!1427 = !DISubroutineType(types: !1428)
!1428 = !{!56, !1429}
!1429 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&Queue<u8, 2>", baseType: !43, size: 32, align: 32, dwarfAddressSpace: 0)
!1430 = !{!1431, !1432, !1434}
!1431 = !DILocalVariable(name: "self", arg: 1, scope: !1426, file: !1412, line: 136, type: !1429)
!1432 = !DILocalVariable(name: "current_head", scope: !1433, file: !1412, line: 137, type: !56, align: 4)
!1433 = distinct !DILexicalBlock(scope: !1426, file: !1412, line: 137, column: 9)
!1434 = !DILocalVariable(name: "current_tail", scope: !1435, file: !1412, line: 138, type: !56, align: 4)
!1435 = distinct !DILexicalBlock(scope: !1433, file: !1412, line: 138, column: 9)
!1436 = !DILocation(line: 136, column: 16, scope: !1426)
!1437 = !DILocation(line: 137, column: 28, scope: !1426)
!1438 = !DILocation(line: 137, column: 43, scope: !1426)
!1439 = !{i8 0, i8 5}
!1440 = !DILocation(line: 137, column: 13, scope: !1433)
!1441 = !DILocation(line: 138, column: 28, scope: !1433)
!1442 = !DILocation(line: 138, column: 43, scope: !1433)
!1443 = !DILocation(line: 138, column: 13, scope: !1435)
!1444 = !DILocalVariable(name: "self", arg: 1, scope: !1445, file: !1446, line: 1221, type: !56)
!1445 = distinct !DISubprogram(name: "wrapping_sub", linkageName: "_ZN4core3num23_$LT$impl$u20$usize$GT$12wrapping_sub17h5357e920b93fac51E", scope: !1447, file: !1446, line: 1221, type: !1449, scopeLine: 1221, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !59, retainedNodes: !1451)
!1446 = !DIFile(filename: "/rustc/9d1b2106e23b1abd32fce1f17267604a5102f57a/library/core/src/num/uint_macros.rs", directory: "", checksumkind: CSK_MD5, checksum: "678e3b2eaaf3fce4798f22852c83a022")
!1447 = !DINamespace(name: "{impl#11}", scope: !1448)
!1448 = !DINamespace(name: "num", scope: !12)
!1449 = !DISubroutineType(types: !1450)
!1450 = !{!56, !56, !56}
!1451 = !{!1444, !1452}
!1452 = !DILocalVariable(name: "rhs", arg: 2, scope: !1445, file: !1446, line: 1221, type: !56)
!1453 = !DILocation(line: 1221, column: 35, scope: !1445, inlinedAt: !1454)
!1454 = distinct !DILocation(line: 140, column: 9, scope: !1435)
!1455 = !DILocation(line: 1221, column: 41, scope: !1445, inlinedAt: !1454)
!1456 = !DILocation(line: 1222, column: 13, scope: !1445, inlinedAt: !1454)
!1457 = !DILocation(line: 140, column: 9, scope: !1435)
!1458 = !DILocalVariable(name: "self", arg: 1, scope: !1459, file: !1446, line: 1179, type: !56)
!1459 = distinct !DISubprogram(name: "wrapping_add", linkageName: "_ZN4core3num23_$LT$impl$u20$usize$GT$12wrapping_add17h210a876f49c5555eE", scope: !1447, file: !1446, line: 1179, type: !1449, scopeLine: 1179, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !59, retainedNodes: !1460)
!1460 = !{!1458, !1461}
!1461 = !DILocalVariable(name: "rhs", arg: 2, scope: !1459, file: !1446, line: 1179, type: !56)
!1462 = !DILocation(line: 1179, column: 35, scope: !1459, inlinedAt: !1463)
!1463 = distinct !DILocation(line: 140, column: 9, scope: !1435)
!1464 = !DILocation(line: 1179, column: 41, scope: !1459, inlinedAt: !1463)
!1465 = !DILocation(line: 1180, column: 13, scope: !1459, inlinedAt: !1463)
!1466 = !DILocation(line: 141, column: 6, scope: !1426)
!1467 = distinct !DISubprogram(name: "len<(locals::app::P1_T, u8), 3>", linkageName: "_ZN8heapless4spsc18Queue$LT$T$C$_$GT$3len17hae3eb13f496ab5d6E", scope: !116, file: !1412, line: 136, type: !1468, scopeLine: 136, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !140, retainedNodes: !1471)
!1468 = !DISubroutineType(types: !1469)
!1469 = !{!56, !1470}
!1470 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&Queue<(locals::app::P1_T, u8), 3>", baseType: !116, size: 32, align: 32, dwarfAddressSpace: 0)
!1471 = !{!1472, !1473, !1475}
!1472 = !DILocalVariable(name: "self", arg: 1, scope: !1467, file: !1412, line: 136, type: !1470)
!1473 = !DILocalVariable(name: "current_head", scope: !1474, file: !1412, line: 137, type: !56, align: 4)
!1474 = distinct !DILexicalBlock(scope: !1467, file: !1412, line: 137, column: 9)
!1475 = !DILocalVariable(name: "current_tail", scope: !1476, file: !1412, line: 138, type: !56, align: 4)
!1476 = distinct !DILexicalBlock(scope: !1474, file: !1412, line: 138, column: 9)
!1477 = !DILocation(line: 136, column: 16, scope: !1467)
!1478 = !DILocation(line: 137, column: 28, scope: !1467)
!1479 = !DILocation(line: 137, column: 43, scope: !1467)
!1480 = !DILocation(line: 137, column: 13, scope: !1474)
!1481 = !DILocation(line: 138, column: 28, scope: !1474)
!1482 = !DILocation(line: 138, column: 43, scope: !1474)
!1483 = !DILocation(line: 138, column: 13, scope: !1476)
!1484 = !DILocation(line: 1221, column: 35, scope: !1445, inlinedAt: !1485)
!1485 = distinct !DILocation(line: 140, column: 9, scope: !1476)
!1486 = !DILocation(line: 1221, column: 41, scope: !1445, inlinedAt: !1485)
!1487 = !DILocation(line: 1222, column: 13, scope: !1445, inlinedAt: !1485)
!1488 = !DILocation(line: 140, column: 9, scope: !1476)
!1489 = !DILocation(line: 1179, column: 35, scope: !1459, inlinedAt: !1490)
!1490 = distinct !DILocation(line: 140, column: 9, scope: !1476)
!1491 = !DILocation(line: 1179, column: 41, scope: !1459, inlinedAt: !1490)
!1492 = !DILocation(line: 1180, column: 13, scope: !1459, inlinedAt: !1490)
!1493 = !DILocation(line: 141, column: 6, scope: !1467)
!1494 = distinct !DISubprogram(name: "iter_mut<u8, 2>", linkageName: "_ZN8heapless4spsc18Queue$LT$T$C$_$GT$8iter_mut17h39603bd78c1eaa58E", scope: !43, file: !1412, line: 165, type: !1495, scopeLine: 165, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !74, retainedNodes: !1503)
!1495 = !DISubroutineType(types: !1496)
!1496 = !{!1497, !1500}
!1497 = !DICompositeType(tag: DW_TAG_structure_type, name: "IterMut<u8, 2>", scope: !44, file: !6, size: 96, align: 32, elements: !1498, templateParams: !74, identifier: "940fb1942e6aa8c746086e1447e8c2db")
!1498 = !{!1499, !1501, !1502}
!1499 = !DIDerivedType(tag: DW_TAG_member, name: "rb", scope: !1497, file: !6, baseType: !1500, size: 32, align: 32)
!1500 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&mut Queue<u8, 2>", baseType: !43, size: 32, align: 32, dwarfAddressSpace: 0)
!1501 = !DIDerivedType(tag: DW_TAG_member, name: "index", scope: !1497, file: !6, baseType: !56, size: 32, align: 32, offset: 32)
!1502 = !DIDerivedType(tag: DW_TAG_member, name: "len", scope: !1497, file: !6, baseType: !56, size: 32, align: 32, offset: 64)
!1503 = !{!1504, !1505}
!1504 = !DILocalVariable(name: "self", arg: 1, scope: !1494, file: !1412, line: 165, type: !1500)
!1505 = !DILocalVariable(name: "len", scope: !1506, file: !1412, line: 166, type: !56, align: 4)
!1506 = distinct !DILexicalBlock(scope: !1494, file: !1412, line: 166, column: 9)
!1507 = !DILocation(line: 165, column: 21, scope: !1494)
!1508 = !DILocation(line: 166, column: 19, scope: !1494)
!1509 = !DILocation(line: 166, column: 13, scope: !1506)
!1510 = !DILocation(line: 167, column: 9, scope: !1506)
!1511 = !DILocation(line: 172, column: 6, scope: !1494)
!1512 = distinct !DISubprogram(name: "iter_mut<(locals::app::P1_T, u8), 3>", linkageName: "_ZN8heapless4spsc18Queue$LT$T$C$_$GT$8iter_mut17hf08aa4f1cb5ae041E", scope: !116, file: !1412, line: 165, type: !1513, scopeLine: 165, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !140, retainedNodes: !1521)
!1513 = !DISubroutineType(types: !1514)
!1514 = !{!1515, !1518}
!1515 = !DICompositeType(tag: DW_TAG_structure_type, name: "IterMut<(locals::app::P1_T, u8), 3>", scope: !44, file: !6, size: 96, align: 32, elements: !1516, templateParams: !140, identifier: "181cbbc2677d421e27a1ebde5aac3a44")
!1516 = !{!1517, !1519, !1520}
!1517 = !DIDerivedType(tag: DW_TAG_member, name: "rb", scope: !1515, file: !6, baseType: !1518, size: 32, align: 32)
!1518 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&mut Queue<(locals::app::P1_T, u8), 3>", baseType: !116, size: 32, align: 32, dwarfAddressSpace: 0)
!1519 = !DIDerivedType(tag: DW_TAG_member, name: "index", scope: !1515, file: !6, baseType: !56, size: 32, align: 32, offset: 32)
!1520 = !DIDerivedType(tag: DW_TAG_member, name: "len", scope: !1515, file: !6, baseType: !56, size: 32, align: 32, offset: 64)
!1521 = !{!1522, !1523}
!1522 = !DILocalVariable(name: "self", arg: 1, scope: !1512, file: !1412, line: 165, type: !1518)
!1523 = !DILocalVariable(name: "len", scope: !1524, file: !1412, line: 166, type: !56, align: 4)
!1524 = distinct !DILexicalBlock(scope: !1512, file: !1412, line: 166, column: 9)
!1525 = !DILocation(line: 165, column: 21, scope: !1512)
!1526 = !DILocation(line: 166, column: 19, scope: !1512)
!1527 = !DILocation(line: 166, column: 13, scope: !1524)
!1528 = !DILocation(line: 167, column: 9, scope: !1524)
!1529 = !DILocation(line: 172, column: 6, scope: !1512)
!1530 = distinct !DISubprogram(name: "dequeue<u8, 2>", linkageName: "_ZN8heapless4spsc18Queue$LT$T$C$_$GT$7dequeue17hbddbe69e01b0948fE", scope: !43, file: !1412, line: 184, type: !1531, scopeLine: 184, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !74, retainedNodes: !1533)
!1531 = !DISubroutineType(types: !1532)
!1532 = !{!485, !1500}
!1533 = !{!1534}
!1534 = !DILocalVariable(name: "self", arg: 1, scope: !1530, file: !1412, line: 184, type: !1500)
!1535 = !DILocation(line: 184, column: 20, scope: !1530)
!1536 = !DILocation(line: 185, column: 18, scope: !1530)
!1537 = !DILocation(line: 186, column: 6, scope: !1530)
!1538 = distinct !DISubprogram(name: "inner_enqueue_unchecked<u8, 2>", linkageName: "_ZN8heapless4spsc18Queue$LT$T$C$_$GT$23inner_enqueue_unchecked17h89c8e0b55bfe44b7E", scope: !43, file: !1412, line: 232, type: !1539, scopeLine: 232, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !74, retainedNodes: !1541)
!1539 = !DISubroutineType(types: !1540)
!1540 = !{null, !1429, !73}
!1541 = !{!1542, !1543, !1544}
!1542 = !DILocalVariable(name: "self", arg: 1, scope: !1538, file: !1412, line: 232, type: !1429)
!1543 = !DILocalVariable(name: "val", arg: 2, scope: !1538, file: !1412, line: 232, type: !73)
!1544 = !DILocalVariable(name: "current_tail", scope: !1545, file: !1412, line: 233, type: !56, align: 4)
!1545 = distinct !DILexicalBlock(scope: !1538, file: !1412, line: 233, column: 9)
!1546 = !DILocation(line: 232, column: 39, scope: !1538)
!1547 = !DILocation(line: 232, column: 46, scope: !1538)
!1548 = !DILocation(line: 233, column: 28, scope: !1538)
!1549 = !DILocation(line: 233, column: 43, scope: !1538)
!1550 = !DILocation(line: 233, column: 13, scope: !1545)
!1551 = !DILocation(line: 235, column: 10, scope: !1545)
!1552 = !DILocalVariable(name: "self", arg: 1, scope: !1553, file: !449, line: 1913, type: !1557)
!1553 = distinct !DISubprogram(name: "get<core::mem::maybe_uninit::MaybeUninit<u8>>", linkageName: "_ZN4core4cell19UnsafeCell$LT$T$GT$3get17h9c1ab351e2331ea0E", scope: !63, file: !449, line: 1913, type: !1554, scopeLine: 1913, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !76, retainedNodes: !1558)
!1554 = !DISubroutineType(types: !1555)
!1555 = !{!1556, !1557}
!1556 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*mut MaybeUninit<u8>", baseType: !66, size: 32, align: 32, dwarfAddressSpace: 0)
!1557 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&UnsafeCell<core::mem::maybe_uninit::MaybeUninit<u8>>", baseType: !63, size: 32, align: 32, dwarfAddressSpace: 0)
!1558 = !{!1552}
!1559 = !DILocation(line: 1913, column: 22, scope: !1553, inlinedAt: !1560)
!1560 = distinct !DILocation(line: 235, column: 9, scope: !1545)
!1561 = !DILocation(line: 235, column: 9, scope: !1545)
!1562 = !DILocalVariable(name: "val", arg: 1, scope: !1563, file: !459, line: 296, type: !73)
!1563 = distinct !DISubprogram(name: "new<u8>", linkageName: "_ZN4core3mem12maybe_uninit20MaybeUninit$LT$T$GT$3new17h7d40a8fb71f44e8eE", scope: !66, file: !459, line: 296, type: !1564, scopeLine: 296, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !74, retainedNodes: !1566)
!1564 = !DISubroutineType(types: !1565)
!1565 = !{!66, !73}
!1566 = !{!1562}
!1567 = !DILocation(line: 296, column: 22, scope: !1563, inlinedAt: !1568)
!1568 = distinct !DILocation(line: 235, column: 63, scope: !1545)
!1569 = !DILocalVariable(name: "value", arg: 1, scope: !1570, file: !1087, line: 69, type: !73)
!1570 = distinct !DISubprogram(name: "new<u8>", linkageName: "_ZN4core3mem13manually_drop21ManuallyDrop$LT$T$GT$3new17h6835951543b0d6daE", scope: !70, file: !1087, line: 69, type: !1571, scopeLine: 69, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !74, retainedNodes: !1573)
!1571 = !DISubroutineType(types: !1572)
!1572 = !{!70, !73}
!1573 = !{!1569}
!1574 = !DILocation(line: 69, column: 22, scope: !1570, inlinedAt: !1575)
!1575 = distinct !DILocation(line: 297, column: 30, scope: !1563, inlinedAt: !1568)
!1576 = !DILocation(line: 70, column: 9, scope: !1570, inlinedAt: !1575)
!1577 = !DILocation(line: 71, column: 6, scope: !1570, inlinedAt: !1575)
!1578 = !DILocation(line: 297, column: 9, scope: !1563, inlinedAt: !1568)
!1579 = !DILocation(line: 298, column: 6, scope: !1563, inlinedAt: !1568)
!1580 = !DILocation(line: 235, column: 63, scope: !1545)
!1581 = !DILocalVariable(name: "self", arg: 1, scope: !1582, file: !469, line: 1061, type: !1556)
!1582 = distinct !DISubprogram(name: "write<core::mem::maybe_uninit::MaybeUninit<u8>>", linkageName: "_ZN4core3ptr7mut_ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$5write17h68d6ef86240d48d4E", scope: !470, file: !469, line: 1061, type: !1583, scopeLine: 1061, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !76, retainedNodes: !1585)
!1583 = !DISubroutineType(types: !1584)
!1584 = !{null, !1556, !66}
!1585 = !{!1581, !1586}
!1586 = !DILocalVariable(name: "val", arg: 2, scope: !1582, file: !469, line: 1061, type: !66)
!1587 = !DILocation(line: 1061, column: 31, scope: !1582, inlinedAt: !1588)
!1588 = distinct !DILocation(line: 235, column: 9, scope: !1545)
!1589 = !DILocation(line: 1061, column: 37, scope: !1582, inlinedAt: !1588)
!1590 = !DILocation(line: 1066, column: 18, scope: !1582, inlinedAt: !1588)
!1591 = !DILocation(line: 236, column: 9, scope: !1545)
!1592 = !DILocation(line: 237, column: 20, scope: !1545)
!1593 = !DILocation(line: 237, column: 51, scope: !1545)
!1594 = !DILocation(line: 238, column: 6, scope: !1538)
!1595 = distinct !DISubprogram(name: "inner_enqueue_unchecked<(locals::app::P1_T, u8), 3>", linkageName: "_ZN8heapless4spsc18Queue$LT$T$C$_$GT$23inner_enqueue_unchecked17hc697da99f6577d56E", scope: !116, file: !1412, line: 232, type: !1596, scopeLine: 232, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !140, retainedNodes: !1598)
!1596 = !DISubroutineType(types: !1597)
!1597 = !{null, !1470, !132}
!1598 = !{!1599, !1600, !1601}
!1599 = !DILocalVariable(name: "self", arg: 1, scope: !1595, file: !1412, line: 232, type: !1470)
!1600 = !DILocalVariable(name: "val", arg: 2, scope: !1595, file: !1412, line: 232, type: !132)
!1601 = !DILocalVariable(name: "current_tail", scope: !1602, file: !1412, line: 233, type: !56, align: 4)
!1602 = distinct !DILexicalBlock(scope: !1595, file: !1412, line: 233, column: 9)
!1603 = !DILocation(line: 232, column: 39, scope: !1595)
!1604 = !DILocation(line: 232, column: 46, scope: !1595)
!1605 = !DILocation(line: 233, column: 28, scope: !1595)
!1606 = !DILocation(line: 233, column: 43, scope: !1595)
!1607 = !DILocation(line: 233, column: 13, scope: !1602)
!1608 = !DILocation(line: 235, column: 10, scope: !1602)
!1609 = !DILocalVariable(name: "self", arg: 1, scope: !1610, file: !449, line: 1913, type: !1614)
!1610 = distinct !DISubprogram(name: "get<core::mem::maybe_uninit::MaybeUninit<(locals::app::P1_T, u8)>>", linkageName: "_ZN4core4cell19UnsafeCell$LT$T$GT$3get17hbc0ca8df7361d5e9E", scope: !122, file: !449, line: 1913, type: !1611, scopeLine: 1913, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !142, retainedNodes: !1615)
!1611 = !DISubroutineType(types: !1612)
!1612 = !{!1613, !1614}
!1613 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*mut MaybeUninit<(locals::app::P1_T, u8)>", baseType: !125, size: 32, align: 32, dwarfAddressSpace: 0)
!1614 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&UnsafeCell<core::mem::maybe_uninit::MaybeUninit<(locals::app::P1_T, u8)>>", baseType: !122, size: 32, align: 32, dwarfAddressSpace: 0)
!1615 = !{!1609}
!1616 = !DILocation(line: 1913, column: 22, scope: !1610, inlinedAt: !1617)
!1617 = distinct !DILocation(line: 235, column: 9, scope: !1602)
!1618 = !DILocation(line: 235, column: 9, scope: !1602)
!1619 = !DILocalVariable(name: "val", arg: 1, scope: !1620, file: !459, line: 296, type: !132)
!1620 = distinct !DISubprogram(name: "new<(locals::app::P1_T, u8)>", linkageName: "_ZN4core3mem12maybe_uninit20MaybeUninit$LT$T$GT$3new17h4d1574678be1ab85E", scope: !125, file: !459, line: 296, type: !1621, scopeLine: 296, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !140, retainedNodes: !1623)
!1621 = !DISubroutineType(types: !1622)
!1622 = !{!125, !132}
!1623 = !{!1619}
!1624 = !DILocation(line: 296, column: 22, scope: !1620, inlinedAt: !1625)
!1625 = distinct !DILocation(line: 235, column: 63, scope: !1602)
!1626 = !DILocalVariable(name: "value", arg: 1, scope: !1627, file: !1087, line: 69, type: !132)
!1627 = distinct !DISubprogram(name: "new<(locals::app::P1_T, u8)>", linkageName: "_ZN4core3mem13manually_drop21ManuallyDrop$LT$T$GT$3new17h0dbc5240ba9b611fE", scope: !129, file: !1087, line: 69, type: !1628, scopeLine: 69, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !140, retainedNodes: !1630)
!1628 = !DISubroutineType(types: !1629)
!1629 = !{!129, !132}
!1630 = !{!1626}
!1631 = !DILocation(line: 69, column: 22, scope: !1627, inlinedAt: !1632)
!1632 = distinct !DILocation(line: 297, column: 30, scope: !1620, inlinedAt: !1625)
!1633 = !DILocation(line: 70, column: 9, scope: !1627, inlinedAt: !1632)
!1634 = !DILocation(line: 71, column: 6, scope: !1627, inlinedAt: !1632)
!1635 = !DILocation(line: 297, column: 9, scope: !1620, inlinedAt: !1625)
!1636 = !DILocation(line: 298, column: 6, scope: !1620, inlinedAt: !1625)
!1637 = !DILocation(line: 235, column: 63, scope: !1602)
!1638 = !DILocalVariable(name: "self", arg: 1, scope: !1639, file: !469, line: 1061, type: !1613)
!1639 = distinct !DISubprogram(name: "write<core::mem::maybe_uninit::MaybeUninit<(locals::app::P1_T, u8)>>", linkageName: "_ZN4core3ptr7mut_ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$5write17h0dec1372fcf68b25E", scope: !470, file: !469, line: 1061, type: !1640, scopeLine: 1061, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !142, retainedNodes: !1642)
!1640 = !DISubroutineType(types: !1641)
!1641 = !{null, !1613, !125}
!1642 = !{!1638, !1643}
!1643 = !DILocalVariable(name: "val", arg: 2, scope: !1639, file: !469, line: 1061, type: !125)
!1644 = !DILocation(line: 1061, column: 31, scope: !1639, inlinedAt: !1645)
!1645 = distinct !DILocation(line: 235, column: 9, scope: !1602)
!1646 = !DILocation(line: 1061, column: 37, scope: !1639, inlinedAt: !1645)
!1647 = !DILocation(line: 1066, column: 18, scope: !1639, inlinedAt: !1645)
!1648 = !DILocation(line: 236, column: 9, scope: !1602)
!1649 = !DILocation(line: 237, column: 20, scope: !1602)
!1650 = !DILocation(line: 237, column: 51, scope: !1602)
!1651 = !DILocation(line: 238, column: 6, scope: !1595)
!1652 = distinct !DISubprogram(name: "enqueue_unchecked<u8, 2>", linkageName: "_ZN8heapless4spsc18Queue$LT$T$C$_$GT$17enqueue_unchecked17h37791079c683ec91E", scope: !43, file: !1412, line: 248, type: !1653, scopeLine: 248, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !74, retainedNodes: !1655)
!1653 = !DISubroutineType(types: !1654)
!1654 = !{null, !1500, !73}
!1655 = !{!1656, !1657}
!1656 = !DILocalVariable(name: "self", arg: 1, scope: !1652, file: !1412, line: 248, type: !1500)
!1657 = !DILocalVariable(name: "val", arg: 2, scope: !1652, file: !1412, line: 248, type: !73)
!1658 = !DILocation(line: 248, column: 37, scope: !1652)
!1659 = !DILocation(line: 248, column: 48, scope: !1652)
!1660 = !DILocation(line: 249, column: 9, scope: !1652)
!1661 = !DILocation(line: 250, column: 6, scope: !1652)
!1662 = distinct !DISubprogram(name: "enqueue_unchecked<(locals::app::P1_T, u8), 3>", linkageName: "_ZN8heapless4spsc18Queue$LT$T$C$_$GT$17enqueue_unchecked17ha419a43839953ab6E", scope: !116, file: !1412, line: 248, type: !1663, scopeLine: 248, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !140, retainedNodes: !1665)
!1663 = !DISubroutineType(types: !1664)
!1664 = !{null, !1518, !132}
!1665 = !{!1666, !1667}
!1666 = !DILocalVariable(name: "self", arg: 1, scope: !1662, file: !1412, line: 248, type: !1518)
!1667 = !DILocalVariable(name: "val", arg: 2, scope: !1662, file: !1412, line: 248, type: !132)
!1668 = !DILocation(line: 248, column: 37, scope: !1662)
!1669 = !DILocation(line: 248, column: 48, scope: !1662)
!1670 = !DILocation(line: 249, column: 9, scope: !1662)
!1671 = !DILocation(line: 250, column: 6, scope: !1662)
!1672 = distinct !DISubprogram(name: "inner_dequeue<u8, 2>", linkageName: "_ZN8heapless4spsc18Queue$LT$T$C$_$GT$13inner_dequeue17h69553b06f8d214e3E", scope: !43, file: !1412, line: 255, type: !1673, scopeLine: 255, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !74, retainedNodes: !1675)
!1673 = !DISubroutineType(types: !1674)
!1674 = !{!485, !1429}
!1675 = !{!1676, !1677, !1679}
!1676 = !DILocalVariable(name: "self", arg: 1, scope: !1672, file: !1412, line: 255, type: !1429)
!1677 = !DILocalVariable(name: "current_head", scope: !1678, file: !1412, line: 256, type: !56, align: 4)
!1678 = distinct !DILexicalBlock(scope: !1672, file: !1412, line: 256, column: 9)
!1679 = !DILocalVariable(name: "v", scope: !1680, file: !1412, line: 261, type: !73, align: 1)
!1680 = distinct !DILexicalBlock(scope: !1678, file: !1412, line: 261, column: 13)
!1681 = !DILocation(line: 255, column: 29, scope: !1672)
!1682 = !DILocation(line: 256, column: 28, scope: !1672)
!1683 = !DILocation(line: 256, column: 43, scope: !1672)
!1684 = !DILocation(line: 256, column: 13, scope: !1678)
!1685 = !DILocation(line: 258, column: 28, scope: !1678)
!1686 = !DILocation(line: 258, column: 43, scope: !1678)
!1687 = !DILocation(line: 258, column: 12, scope: !1678)
!1688 = !DILocation(line: 261, column: 22, scope: !1678)
!1689 = !DILocation(line: 259, column: 13, scope: !1678)
!1690 = !DILocation(line: 258, column: 9, scope: !1678)
!1691 = !DILocation(line: 268, column: 6, scope: !1672)
!1692 = !DILocation(line: 1913, column: 22, scope: !1553, inlinedAt: !1693)
!1693 = distinct !DILocation(line: 261, column: 22, scope: !1678)
!1694 = !DILocation(line: 261, column: 21, scope: !1678)
!1695 = !DILocation(line: 261, column: 17, scope: !1680)
!1696 = !DILocation(line: 263, column: 13, scope: !1680)
!1697 = !DILocation(line: 264, column: 24, scope: !1680)
!1698 = !DILocation(line: 264, column: 55, scope: !1680)
!1699 = !DILocation(line: 266, column: 13, scope: !1680)
!1700 = distinct !DISubprogram(name: "inner_dequeue<(locals::app::P1_T, u8), 3>", linkageName: "_ZN8heapless4spsc18Queue$LT$T$C$_$GT$13inner_dequeue17hac125db0968c02faE", scope: !116, file: !1412, line: 255, type: !1701, scopeLine: 255, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !140, retainedNodes: !1713)
!1701 = !DISubroutineType(types: !1702)
!1702 = !{!1703, !1470}
!1703 = !DICompositeType(tag: DW_TAG_structure_type, name: "Option<(locals::app::P1_T, u8)>", scope: !486, file: !6, size: 16, align: 8, elements: !1704, identifier: "14ba119827523658860ae2c6e10a2631")
!1704 = !{!1705}
!1705 = !DICompositeType(tag: DW_TAG_variant_part, scope: !486, file: !6, size: 16, align: 8, elements: !1706, templateParams: !140, identifier: "14ba119827523658860ae2c6e10a2631_variant_part", discriminator: !496)
!1706 = !{!1707, !1709}
!1707 = !DIDerivedType(tag: DW_TAG_member, name: "None", scope: !1705, file: !6, baseType: !1708, size: 16, align: 8, extraData: i64 2)
!1708 = !DICompositeType(tag: DW_TAG_structure_type, name: "None", scope: !1703, file: !6, size: 16, align: 8, elements: !59, templateParams: !140, identifier: "14ba119827523658860ae2c6e10a2631::None")
!1709 = !DIDerivedType(tag: DW_TAG_member, name: "Some", scope: !1705, file: !6, baseType: !1710, size: 16, align: 8)
!1710 = !DICompositeType(tag: DW_TAG_structure_type, name: "Some", scope: !1703, file: !6, size: 16, align: 8, elements: !1711, templateParams: !140, identifier: "14ba119827523658860ae2c6e10a2631::Some")
!1711 = !{!1712}
!1712 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !1710, file: !6, baseType: !132, size: 16, align: 8)
!1713 = !{!1714, !1715, !1717}
!1714 = !DILocalVariable(name: "self", arg: 1, scope: !1700, file: !1412, line: 255, type: !1470)
!1715 = !DILocalVariable(name: "current_head", scope: !1716, file: !1412, line: 256, type: !56, align: 4)
!1716 = distinct !DILexicalBlock(scope: !1700, file: !1412, line: 256, column: 9)
!1717 = !DILocalVariable(name: "v", scope: !1718, file: !1412, line: 261, type: !132, align: 1)
!1718 = distinct !DILexicalBlock(scope: !1716, file: !1412, line: 261, column: 13)
!1719 = !DILocation(line: 255, column: 29, scope: !1700)
!1720 = !DILocation(line: 256, column: 28, scope: !1700)
!1721 = !DILocation(line: 256, column: 43, scope: !1700)
!1722 = !DILocation(line: 256, column: 13, scope: !1716)
!1723 = !DILocation(line: 258, column: 28, scope: !1716)
!1724 = !DILocation(line: 258, column: 43, scope: !1716)
!1725 = !DILocation(line: 258, column: 12, scope: !1716)
!1726 = !DILocation(line: 261, column: 22, scope: !1716)
!1727 = !DILocation(line: 259, column: 13, scope: !1716)
!1728 = !DILocation(line: 258, column: 9, scope: !1716)
!1729 = !DILocation(line: 268, column: 6, scope: !1700)
!1730 = !DILocation(line: 1913, column: 22, scope: !1610, inlinedAt: !1731)
!1731 = distinct !DILocation(line: 261, column: 22, scope: !1716)
!1732 = !DILocation(line: 261, column: 21, scope: !1716)
!1733 = !DILocation(line: 261, column: 17, scope: !1718)
!1734 = !DILocation(line: 263, column: 13, scope: !1718)
!1735 = !DILocation(line: 264, column: 24, scope: !1718)
!1736 = !DILocation(line: 264, column: 55, scope: !1718)
!1737 = !DILocation(line: 266, column: 13, scope: !1718)
!1738 = distinct !DISubprogram(name: "split<(locals::app::P1_T, u8), 3>", linkageName: "_ZN8heapless4spsc18Queue$LT$T$C$_$GT$5split17h4fd76e368a9e3d31E", scope: !116, file: !1412, line: 294, type: !1739, scopeLine: 294, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !140, retainedNodes: !1751)
!1739 = !DISubroutineType(types: !1740)
!1740 = !{!1741, !1518}
!1741 = !DICompositeType(tag: DW_TAG_structure_type, name: "(heapless::spsc::Producer<(locals::app::P1_T, u8), 3>, heapless::spsc::Consumer<(locals::app::P1_T, u8), 3>)", file: !6, size: 64, align: 32, elements: !1742, templateParams: !59, identifier: "f8145396468adc9fb56728226214ca89")
!1742 = !{!1743, !1747}
!1743 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !1741, file: !6, baseType: !1744, size: 32, align: 32)
!1744 = !DICompositeType(tag: DW_TAG_structure_type, name: "Producer<(locals::app::P1_T, u8), 3>", scope: !44, file: !6, size: 32, align: 32, elements: !1745, templateParams: !140, identifier: "83b16a506bbc92be8a8a2e327cbe8af3")
!1745 = !{!1746}
!1746 = !DIDerivedType(tag: DW_TAG_member, name: "rb", scope: !1744, file: !6, baseType: !1470, size: 32, align: 32)
!1747 = !DIDerivedType(tag: DW_TAG_member, name: "__1", scope: !1741, file: !6, baseType: !1748, size: 32, align: 32, offset: 32)
!1748 = !DICompositeType(tag: DW_TAG_structure_type, name: "Consumer<(locals::app::P1_T, u8), 3>", scope: !44, file: !6, size: 32, align: 32, elements: !1749, templateParams: !140, identifier: "4899d611660adab951fc85223ec55b3b")
!1749 = !{!1750}
!1750 = !DIDerivedType(tag: DW_TAG_member, name: "rb", scope: !1748, file: !6, baseType: !1470, size: 32, align: 32)
!1751 = !{!1752}
!1752 = !DILocalVariable(name: "self", arg: 1, scope: !1738, file: !1412, line: 294, type: !1518)
!1753 = !DILocation(line: 294, column: 18, scope: !1738)
!1754 = !DILocation(line: 295, column: 10, scope: !1738)
!1755 = !DILocation(line: 295, column: 33, scope: !1738)
!1756 = !DILocation(line: 295, column: 9, scope: !1738)
!1757 = !DILocation(line: 296, column: 6, scope: !1738)
!1758 = distinct !DISubprogram(name: "split<u8, 2>", linkageName: "_ZN8heapless4spsc18Queue$LT$T$C$_$GT$5split17h5bcea21f35589d84E", scope: !43, file: !1412, line: 294, type: !1759, scopeLine: 294, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !74, retainedNodes: !1771)
!1759 = !DISubroutineType(types: !1760)
!1760 = !{!1761, !1500}
!1761 = !DICompositeType(tag: DW_TAG_structure_type, name: "(heapless::spsc::Producer<u8, 2>, heapless::spsc::Consumer<u8, 2>)", file: !6, size: 64, align: 32, elements: !1762, templateParams: !59, identifier: "7ed25b19eb6e61b4c2bd9b3dd628b5d3")
!1762 = !{!1763, !1767}
!1763 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !1761, file: !6, baseType: !1764, size: 32, align: 32)
!1764 = !DICompositeType(tag: DW_TAG_structure_type, name: "Producer<u8, 2>", scope: !44, file: !6, size: 32, align: 32, elements: !1765, templateParams: !74, identifier: "179cb93fc488b0d7c276eb52ae18ad25")
!1765 = !{!1766}
!1766 = !DIDerivedType(tag: DW_TAG_member, name: "rb", scope: !1764, file: !6, baseType: !1429, size: 32, align: 32)
!1767 = !DIDerivedType(tag: DW_TAG_member, name: "__1", scope: !1761, file: !6, baseType: !1768, size: 32, align: 32, offset: 32)
!1768 = !DICompositeType(tag: DW_TAG_structure_type, name: "Consumer<u8, 2>", scope: !44, file: !6, size: 32, align: 32, elements: !1769, templateParams: !74, identifier: "69c300921d4ed9b37b7add908bf389e")
!1769 = !{!1770}
!1770 = !DIDerivedType(tag: DW_TAG_member, name: "rb", scope: !1768, file: !6, baseType: !1429, size: 32, align: 32)
!1771 = !{!1772}
!1772 = !DILocalVariable(name: "self", arg: 1, scope: !1758, file: !1412, line: 294, type: !1500)
!1773 = !DILocation(line: 294, column: 18, scope: !1758)
!1774 = !DILocation(line: 295, column: 10, scope: !1758)
!1775 = !DILocation(line: 295, column: 33, scope: !1758)
!1776 = !DILocation(line: 295, column: 9, scope: !1758)
!1777 = !DILocation(line: 296, column: 6, scope: !1758)
!1778 = distinct !DISubprogram(name: "next<(locals::app::P1_T, u8), 3>", linkageName: "_ZN95_$LT$heapless..spsc..IterMut$LT$T$C$_$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h4d3320e7f4c35bbcE", scope: !1779, file: !1412, line: 379, type: !1780, scopeLine: 379, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !140, retainedNodes: !1797)
!1779 = !DINamespace(name: "{impl#7}", scope: !44)
!1780 = !DISubroutineType(types: !1781)
!1781 = !{!1782, !1796}
!1782 = !DICompositeType(tag: DW_TAG_structure_type, name: "Option<&mut (locals::app::P1_T, u8)>", scope: !486, file: !6, size: 32, align: 32, elements: !1783, identifier: "aafe52fe0b204a243c1847ed7bad65d0")
!1783 = !{!1784}
!1784 = !DICompositeType(tag: DW_TAG_variant_part, scope: !486, file: !6, size: 32, align: 32, elements: !1785, templateParams: !1788, identifier: "aafe52fe0b204a243c1847ed7bad65d0_variant_part", discriminator: !1795)
!1785 = !{!1786, !1791}
!1786 = !DIDerivedType(tag: DW_TAG_member, name: "None", scope: !1784, file: !6, baseType: !1787, size: 32, align: 32, extraData: i64 0)
!1787 = !DICompositeType(tag: DW_TAG_structure_type, name: "None", scope: !1782, file: !6, size: 32, align: 32, elements: !59, templateParams: !1788, identifier: "aafe52fe0b204a243c1847ed7bad65d0::None")
!1788 = !{!1789}
!1789 = !DITemplateTypeParameter(name: "T", type: !1790)
!1790 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&mut (locals::app::P1_T, u8)", baseType: !132, size: 32, align: 32, dwarfAddressSpace: 0)
!1791 = !DIDerivedType(tag: DW_TAG_member, name: "Some", scope: !1784, file: !6, baseType: !1792, size: 32, align: 32)
!1792 = !DICompositeType(tag: DW_TAG_structure_type, name: "Some", scope: !1782, file: !6, size: 32, align: 32, elements: !1793, templateParams: !1788, identifier: "aafe52fe0b204a243c1847ed7bad65d0::Some")
!1793 = !{!1794}
!1794 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !1792, file: !6, baseType: !1790, size: 32, align: 32)
!1795 = !DIDerivedType(tag: DW_TAG_member, scope: !486, file: !6, baseType: !1179, size: 32, align: 32, flags: DIFlagArtificial)
!1796 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&mut IterMut<(locals::app::P1_T, u8), 3>", baseType: !1515, size: 32, align: 32, dwarfAddressSpace: 0)
!1797 = !{!1798, !1799, !1801}
!1798 = !DILocalVariable(name: "self", arg: 1, scope: !1778, file: !1412, line: 379, type: !1796)
!1799 = !DILocalVariable(name: "head", scope: !1800, file: !1412, line: 381, type: !56, align: 4)
!1800 = distinct !DILexicalBlock(scope: !1778, file: !1412, line: 381, column: 13)
!1801 = !DILocalVariable(name: "i", scope: !1802, file: !1412, line: 383, type: !56, align: 4)
!1802 = distinct !DILexicalBlock(scope: !1800, file: !1412, line: 383, column: 13)
!1803 = !DILocation(line: 379, column: 13, scope: !1778)
!1804 = !DILocation(line: 380, column: 12, scope: !1778)
!1805 = !DILocation(line: 380, column: 25, scope: !1778)
!1806 = !DILocation(line: 388, column: 13, scope: !1778)
!1807 = !DILocation(line: 380, column: 9, scope: !1778)
!1808 = !DILocation(line: 381, column: 24, scope: !1778)
!1809 = !DILocation(line: 381, column: 42, scope: !1778)
!1810 = !DILocation(line: 381, column: 17, scope: !1800)
!1811 = !DILocation(line: 383, column: 29, scope: !1800)
!1812 = !DILocation(line: 383, column: 21, scope: !1800)
!1813 = !DILocation(line: 383, column: 17, scope: !1802)
!1814 = !DILocation(line: 384, column: 13, scope: !1802)
!1815 = !DILocation(line: 386, column: 34, scope: !1802)
!1816 = !DILocation(line: 1913, column: 22, scope: !1610, inlinedAt: !1817)
!1817 = distinct !DILocation(line: 386, column: 34, scope: !1802)
!1818 = !DILocation(line: 386, column: 13, scope: !1802)
!1819 = !DILocation(line: 390, column: 6, scope: !1778)
!1820 = distinct !DISubprogram(name: "next<u8, 2>", linkageName: "_ZN95_$LT$heapless..spsc..IterMut$LT$T$C$_$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h72004cbeb71d9325E", scope: !1779, file: !1412, line: 379, type: !1821, scopeLine: 379, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !74, retainedNodes: !1836)
!1821 = !DISubroutineType(types: !1822)
!1822 = !{!1823, !1835}
!1823 = !DICompositeType(tag: DW_TAG_structure_type, name: "Option<&mut u8>", scope: !486, file: !6, size: 32, align: 32, elements: !1824, identifier: "17daa66e41a57ed6516d5bfd75c3194c")
!1824 = !{!1825}
!1825 = !DICompositeType(tag: DW_TAG_variant_part, scope: !486, file: !6, size: 32, align: 32, elements: !1826, templateParams: !1829, identifier: "17daa66e41a57ed6516d5bfd75c3194c_variant_part", discriminator: !1795)
!1826 = !{!1827, !1831}
!1827 = !DIDerivedType(tag: DW_TAG_member, name: "None", scope: !1825, file: !6, baseType: !1828, size: 32, align: 32, extraData: i64 0)
!1828 = !DICompositeType(tag: DW_TAG_structure_type, name: "None", scope: !1823, file: !6, size: 32, align: 32, elements: !59, templateParams: !1829, identifier: "17daa66e41a57ed6516d5bfd75c3194c::None")
!1829 = !{!1830}
!1830 = !DITemplateTypeParameter(name: "T", type: !1399)
!1831 = !DIDerivedType(tag: DW_TAG_member, name: "Some", scope: !1825, file: !6, baseType: !1832, size: 32, align: 32)
!1832 = !DICompositeType(tag: DW_TAG_structure_type, name: "Some", scope: !1823, file: !6, size: 32, align: 32, elements: !1833, templateParams: !1829, identifier: "17daa66e41a57ed6516d5bfd75c3194c::Some")
!1833 = !{!1834}
!1834 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !1832, file: !6, baseType: !1399, size: 32, align: 32)
!1835 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&mut IterMut<u8, 2>", baseType: !1497, size: 32, align: 32, dwarfAddressSpace: 0)
!1836 = !{!1837, !1838, !1840}
!1837 = !DILocalVariable(name: "self", arg: 1, scope: !1820, file: !1412, line: 379, type: !1835)
!1838 = !DILocalVariable(name: "head", scope: !1839, file: !1412, line: 381, type: !56, align: 4)
!1839 = distinct !DILexicalBlock(scope: !1820, file: !1412, line: 381, column: 13)
!1840 = !DILocalVariable(name: "i", scope: !1841, file: !1412, line: 383, type: !56, align: 4)
!1841 = distinct !DILexicalBlock(scope: !1839, file: !1412, line: 383, column: 13)
!1842 = !DILocation(line: 379, column: 13, scope: !1820)
!1843 = !DILocation(line: 380, column: 12, scope: !1820)
!1844 = !DILocation(line: 380, column: 25, scope: !1820)
!1845 = !DILocation(line: 388, column: 13, scope: !1820)
!1846 = !DILocation(line: 380, column: 9, scope: !1820)
!1847 = !DILocation(line: 381, column: 24, scope: !1820)
!1848 = !DILocation(line: 381, column: 42, scope: !1820)
!1849 = !DILocation(line: 381, column: 17, scope: !1839)
!1850 = !DILocation(line: 383, column: 29, scope: !1839)
!1851 = !DILocation(line: 383, column: 21, scope: !1839)
!1852 = !DILocation(line: 383, column: 17, scope: !1841)
!1853 = !DILocation(line: 384, column: 13, scope: !1841)
!1854 = !DILocation(line: 386, column: 34, scope: !1841)
!1855 = !DILocation(line: 1913, column: 22, scope: !1553, inlinedAt: !1856)
!1856 = distinct !DILocation(line: 386, column: 34, scope: !1841)
!1857 = !DILocation(line: 386, column: 13, scope: !1841)
!1858 = !DILocation(line: 390, column: 6, scope: !1820)
!1859 = distinct !DISubprogram(name: "drop<u8, 2>", linkageName: "_ZN76_$LT$heapless..spsc..Queue$LT$T$C$_$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h9ad36f3161ed8116E", scope: !1860, file: !1412, line: 424, type: !1861, scopeLine: 424, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !74, retainedNodes: !1863)
!1860 = !DINamespace(name: "{impl#10}", scope: !44)
!1861 = !DISubroutineType(types: !1862)
!1862 = !{null, !1500}
!1863 = !{!1864, !1865, !1867}
!1864 = !DILocalVariable(name: "self", arg: 1, scope: !1859, file: !1412, line: 424, type: !1500)
!1865 = !DILocalVariable(name: "iter", scope: !1866, file: !1412, line: 425, type: !1497, align: 4)
!1866 = distinct !DILexicalBlock(scope: !1859, file: !1412, line: 425, column: 9)
!1867 = !DILocalVariable(name: "item", scope: !1868, file: !1412, line: 425, type: !1399, align: 4)
!1868 = distinct !DILexicalBlock(scope: !1866, file: !1412, line: 425, column: 26)
!1869 = !DILocation(line: 424, column: 13, scope: !1859)
!1870 = !DILocation(line: 425, column: 21, scope: !1866)
!1871 = !DILocation(line: 425, column: 21, scope: !1859)
!1872 = !DILocation(line: 425, column: 9, scope: !1866)
!1873 = !DILocation(line: 430, column: 6, scope: !1859)
!1874 = !DILocation(line: 425, column: 13, scope: !1866)
!1875 = !DILocation(line: 425, column: 13, scope: !1868)
!1876 = !DILocation(line: 427, column: 17, scope: !1868)
!1877 = distinct !DISubprogram(name: "drop<(locals::app::P1_T, u8), 3>", linkageName: "_ZN76_$LT$heapless..spsc..Queue$LT$T$C$_$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17hde889c84868322b6E", scope: !1860, file: !1412, line: 424, type: !1878, scopeLine: 424, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !140, retainedNodes: !1880)
!1878 = !DISubroutineType(types: !1879)
!1879 = !{null, !1518}
!1880 = !{!1881, !1882, !1884}
!1881 = !DILocalVariable(name: "self", arg: 1, scope: !1877, file: !1412, line: 424, type: !1518)
!1882 = !DILocalVariable(name: "iter", scope: !1883, file: !1412, line: 425, type: !1515, align: 4)
!1883 = distinct !DILexicalBlock(scope: !1877, file: !1412, line: 425, column: 9)
!1884 = !DILocalVariable(name: "item", scope: !1885, file: !1412, line: 425, type: !1790, align: 4)
!1885 = distinct !DILexicalBlock(scope: !1883, file: !1412, line: 425, column: 26)
!1886 = !DILocation(line: 424, column: 13, scope: !1877)
!1887 = !DILocation(line: 425, column: 21, scope: !1883)
!1888 = !DILocation(line: 425, column: 21, scope: !1877)
!1889 = !DILocation(line: 425, column: 9, scope: !1883)
!1890 = !DILocation(line: 430, column: 6, scope: !1877)
!1891 = !DILocation(line: 425, column: 13, scope: !1883)
!1892 = !DILocation(line: 425, column: 13, scope: !1885)
!1893 = !DILocation(line: 427, column: 17, scope: !1885)
!1894 = distinct !DISubprogram(name: "into_iter<(locals::app::P1_T, u8), 3>", linkageName: "_ZN108_$LT$$RF$mut$u20$heapless..spsc..Queue$LT$T$C$_$GT$$u20$as$u20$core..iter..traits..collect..IntoIterator$GT$9into_iter17h3c0ea4b7b381ec52E", scope: !1895, file: !1412, line: 479, type: !1513, scopeLine: 479, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !140, retainedNodes: !1896)
!1895 = !DINamespace(name: "{impl#15}", scope: !44)
!1896 = !{!1897}
!1897 = !DILocalVariable(name: "self", arg: 1, scope: !1894, file: !1412, line: 479, type: !1518)
!1898 = !DILocation(line: 479, column: 18, scope: !1894)
!1899 = !DILocation(line: 480, column: 9, scope: !1894)
!1900 = !DILocation(line: 481, column: 6, scope: !1894)
!1901 = distinct !DISubprogram(name: "into_iter<u8, 2>", linkageName: "_ZN108_$LT$$RF$mut$u20$heapless..spsc..Queue$LT$T$C$_$GT$$u20$as$u20$core..iter..traits..collect..IntoIterator$GT$9into_iter17h73e2329ae13ae5ccE", scope: !1895, file: !1412, line: 479, type: !1495, scopeLine: 479, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !74, retainedNodes: !1902)
!1902 = !{!1903}
!1903 = !DILocalVariable(name: "self", arg: 1, scope: !1901, file: !1412, line: 479, type: !1500)
!1904 = !DILocation(line: 479, column: 18, scope: !1901)
!1905 = !DILocation(line: 480, column: 9, scope: !1901)
!1906 = !DILocation(line: 481, column: 6, scope: !1901)
!1907 = distinct !DISubprogram(name: "dequeue<(locals::app::P1_T, u8), 3>", linkageName: "_ZN8heapless4spsc21Consumer$LT$T$C$_$GT$7dequeue17h4076781cd95e852cE", scope: !1748, file: !1412, line: 503, type: !1908, scopeLine: 503, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !140, retainedNodes: !1911)
!1908 = !DISubroutineType(types: !1909)
!1909 = !{!1703, !1910}
!1910 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&mut Consumer<(locals::app::P1_T, u8), 3>", baseType: !1748, size: 32, align: 32, dwarfAddressSpace: 0)
!1911 = !{!1912}
!1912 = !DILocalVariable(name: "self", arg: 1, scope: !1907, file: !1412, line: 503, type: !1910)
!1913 = !DILocation(line: 503, column: 20, scope: !1907)
!1914 = !DILocation(line: 504, column: 18, scope: !1907)
!1915 = !DILocation(line: 505, column: 6, scope: !1907)
!1916 = distinct !DISubprogram(name: "enqueue_unchecked<u8, 2>", linkageName: "_ZN8heapless4spsc21Producer$LT$T$C$_$GT$17enqueue_unchecked17h2ab9bd46dff93908E", scope: !1764, file: !1412, line: 567, type: !1917, scopeLine: 567, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !74, retainedNodes: !1920)
!1917 = !DISubroutineType(types: !1918)
!1918 = !{null, !1919, !73}
!1919 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&mut Producer<u8, 2>", baseType: !1764, size: 32, align: 32, dwarfAddressSpace: 0)
!1920 = !{!1921, !1922}
!1921 = !DILocalVariable(name: "self", arg: 1, scope: !1916, file: !1412, line: 567, type: !1919)
!1922 = !DILocalVariable(name: "val", arg: 2, scope: !1916, file: !1412, line: 567, type: !73)
!1923 = !DILocation(line: 567, column: 37, scope: !1916)
!1924 = !DILocation(line: 567, column: 48, scope: !1916)
!1925 = !DILocation(line: 568, column: 9, scope: !1916)
!1926 = !DILocation(line: 569, column: 6, scope: !1916)
!1927 = distinct !DISubprogram(name: "read<(locals::app::P1_T, u8)>", linkageName: "_ZN4core3ptr9const_ptr33_$LT$impl$u20$$BP$const$u20$T$GT$4read17h5b4f761fdb64603eE", scope: !1929, file: !1928, line: 807, type: !1931, scopeLine: 807, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !140, retainedNodes: !1934)
!1928 = !DIFile(filename: "/rustc/9d1b2106e23b1abd32fce1f17267604a5102f57a/library/core/src/ptr/const_ptr.rs", directory: "", checksumkind: CSK_MD5, checksum: "c7994d012e815816025e6334f599fdb2")
!1929 = !DINamespace(name: "{impl#0}", scope: !1930)
!1930 = !DINamespace(name: "const_ptr", scope: !230)
!1931 = !DISubroutineType(types: !1932)
!1932 = !{!132, !1933}
!1933 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*const (locals::app::P1_T, u8)", baseType: !132, size: 32, align: 32, dwarfAddressSpace: 0)
!1934 = !{!1935}
!1935 = !DILocalVariable(name: "self", arg: 1, scope: !1927, file: !1928, line: 807, type: !1933)
!1936 = !DILocation(line: 807, column: 30, scope: !1927)
!1937 = !DILocation(line: 812, column: 18, scope: !1927)
!1938 = !DILocation(line: 813, column: 6, scope: !1927)
!1939 = distinct !DISubprogram(name: "read<()>", linkageName: "_ZN4core3ptr9const_ptr33_$LT$impl$u20$$BP$const$u20$T$GT$4read17h60f26a767622c7c8E", scope: !1929, file: !1928, line: 807, type: !1940, scopeLine: 807, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !98, retainedNodes: !1942)
!1940 = !DISubroutineType(types: !1941)
!1941 = !{null, !151}
!1942 = !{!1943}
!1943 = !DILocalVariable(name: "self", arg: 1, scope: !1939, file: !1928, line: 807, type: !151)
!1944 = !DILocation(line: 807, column: 30, scope: !1939)
!1945 = !DILocation(line: 812, column: 18, scope: !1939)
!1946 = !DILocation(line: 813, column: 6, scope: !1939)
!1947 = distinct !DISubprogram(name: "read<u8>", linkageName: "_ZN4core3ptr9const_ptr33_$LT$impl$u20$$BP$const$u20$T$GT$4read17heb62b597f26bdea0E", scope: !1929, file: !1928, line: 807, type: !1948, scopeLine: 807, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !74, retainedNodes: !1951)
!1948 = !DISubroutineType(types: !1949)
!1949 = !{!73, !1950}
!1950 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*const u8", baseType: !73, size: 32, align: 32, dwarfAddressSpace: 0)
!1951 = !{!1952}
!1952 = !DILocalVariable(name: "self", arg: 1, scope: !1947, file: !1928, line: 807, type: !1950)
!1953 = !DILocation(line: 807, column: 30, scope: !1947)
!1954 = !DILocation(line: 812, column: 18, scope: !1947)
!1955 = !DILocation(line: 813, column: 6, scope: !1947)
!1956 = distinct !DISubprogram(name: "as_ptr<core::cell::UnsafeCell<core::mem::maybe_uninit::MaybeUninit<u8>>>", linkageName: "_ZN4core3ptr9const_ptr43_$LT$impl$u20$$BP$const$u20$$u5b$T$u5d$$GT$6as_ptr17h382d54ca28161cafE", scope: !1957, file: !1928, line: 1001, type: !1958, scopeLine: 1001, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !1967, retainedNodes: !1965)
!1957 = !DINamespace(name: "{impl#1}", scope: !1930)
!1958 = !DISubroutineType(types: !1959)
!1959 = !{!1960, !1961}
!1960 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*const UnsafeCell<core::mem::maybe_uninit::MaybeUninit<u8>>", baseType: !63, size: 32, align: 32, dwarfAddressSpace: 0)
!1961 = !DICompositeType(tag: DW_TAG_structure_type, name: "*const [core::cell::UnsafeCell<core::mem::maybe_uninit::MaybeUninit<u8>>]", file: !6, size: 64, align: 32, elements: !1962, templateParams: !59, identifier: "70d9dd8148c54264fe05c0e834d98084")
!1962 = !{!1963, !1964}
!1963 = !DIDerivedType(tag: DW_TAG_member, name: "data_ptr", scope: !1961, file: !6, baseType: !1960, size: 32, align: 32)
!1964 = !DIDerivedType(tag: DW_TAG_member, name: "length", scope: !1961, file: !6, baseType: !56, size: 32, align: 32, offset: 32)
!1965 = !{!1966}
!1966 = !DILocalVariable(name: "self", arg: 1, scope: !1956, file: !1928, line: 1001, type: !1961)
!1967 = !{!1968}
!1968 = !DITemplateTypeParameter(name: "T", type: !63)
!1969 = !DILocation(line: 1001, column: 25, scope: !1956)
!1970 = !DILocation(line: 1002, column: 9, scope: !1956)
!1971 = !DILocation(line: 1003, column: 6, scope: !1956)
!1972 = distinct !DISubprogram(name: "as_ptr<core::cell::UnsafeCell<core::mem::maybe_uninit::MaybeUninit<(locals::app::P1_T, u8)>>>", linkageName: "_ZN4core3ptr9const_ptr43_$LT$impl$u20$$BP$const$u20$$u5b$T$u5d$$GT$6as_ptr17h8e58d8ff10279c42E", scope: !1957, file: !1928, line: 1001, type: !1973, scopeLine: 1001, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !1982, retainedNodes: !1980)
!1973 = !DISubroutineType(types: !1974)
!1974 = !{!1975, !1976}
!1975 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*const UnsafeCell<core::mem::maybe_uninit::MaybeUninit<(locals::app::P1_T, u8)>>", baseType: !122, size: 32, align: 32, dwarfAddressSpace: 0)
!1976 = !DICompositeType(tag: DW_TAG_structure_type, name: "*const [core::cell::UnsafeCell<core::mem::maybe_uninit::MaybeUninit<(locals::app::P1_T, u8)>>]", file: !6, size: 64, align: 32, elements: !1977, templateParams: !59, identifier: "88e01c05ba15c20464521a13b7bca60e")
!1977 = !{!1978, !1979}
!1978 = !DIDerivedType(tag: DW_TAG_member, name: "data_ptr", scope: !1976, file: !6, baseType: !1975, size: 32, align: 32)
!1979 = !DIDerivedType(tag: DW_TAG_member, name: "length", scope: !1976, file: !6, baseType: !56, size: 32, align: 32, offset: 32)
!1980 = !{!1981}
!1981 = !DILocalVariable(name: "self", arg: 1, scope: !1972, file: !1928, line: 1001, type: !1976)
!1982 = !{!1983}
!1983 = !DITemplateTypeParameter(name: "T", type: !122)
!1984 = !DILocation(line: 1001, column: 25, scope: !1972)
!1985 = !DILocation(line: 1002, column: 9, scope: !1972)
!1986 = !DILocation(line: 1003, column: 6, scope: !1972)
!1987 = distinct !DISubprogram(name: "as_ptr<core::mem::maybe_uninit::MaybeUninit<()>>", linkageName: "_ZN4core3ptr9const_ptr43_$LT$impl$u20$$BP$const$u20$$u5b$T$u5d$$GT$6as_ptr17hbe58f58226b6e7d4E", scope: !1957, file: !1928, line: 1001, type: !1988, scopeLine: 1001, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !1997, retainedNodes: !1995)
!1988 = !DISubroutineType(types: !1989)
!1989 = !{!1990, !1991}
!1990 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*const MaybeUninit<()>", baseType: !91, size: 32, align: 32, dwarfAddressSpace: 0)
!1991 = !DICompositeType(tag: DW_TAG_structure_type, name: "*const [core::mem::maybe_uninit::MaybeUninit<()>]", file: !6, size: 64, align: 32, elements: !1992, templateParams: !59, identifier: "486435563ba3622d2829ebf639fc1e6b")
!1992 = !{!1993, !1994}
!1993 = !DIDerivedType(tag: DW_TAG_member, name: "data_ptr", scope: !1991, file: !6, baseType: !1990, size: 32, align: 32)
!1994 = !DIDerivedType(tag: DW_TAG_member, name: "length", scope: !1991, file: !6, baseType: !56, size: 32, align: 32, offset: 32)
!1995 = !{!1996}
!1996 = !DILocalVariable(name: "self", arg: 1, scope: !1987, file: !1928, line: 1001, type: !1991)
!1997 = !{!1998}
!1998 = !DITemplateTypeParameter(name: "T", type: !91)
!1999 = !DILocation(line: 1001, column: 25, scope: !1987)
!2000 = !DILocation(line: 1002, column: 9, scope: !1987)
!2001 = !DILocation(line: 1003, column: 6, scope: !1987)
!2002 = distinct !DISubprogram(name: "read<()>", linkageName: "_ZN4core3ptr4read17h2454c075427a7a1dE", scope: !230, file: !229, line: 685, type: !1940, scopeLine: 685, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !98, retainedNodes: !2003)
!2003 = !{!2004, !2005}
!2004 = !DILocalVariable(name: "src", arg: 1, scope: !2002, file: !229, line: 685, type: !151)
!2005 = !DILocalVariable(name: "tmp", scope: !2006, file: !229, line: 693, type: !91, align: 1)
!2006 = distinct !DILexicalBlock(scope: !2002, file: !229, line: 693, column: 5)
!2007 = !DILocation(line: 685, column: 29, scope: !2002)
!2008 = !DILocation(line: 693, column: 9, scope: !2006)
!2009 = !DILocation(line: 320, column: 9, scope: !2010, inlinedAt: !2011)
!2010 = distinct !DISubprogram(name: "uninit<()>", linkageName: "_ZN4core3mem12maybe_uninit20MaybeUninit$LT$T$GT$6uninit17h68ef21e463453e0cE", scope: !91, file: !459, line: 319, type: !341, scopeLine: 319, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !98, retainedNodes: !59)
!2011 = distinct !DILocation(line: 693, column: 19, scope: !2002)
!2012 = !DILocation(line: 693, column: 19, scope: !2002)
!2013 = !DILocation(line: 573, column: 29, scope: !458, inlinedAt: !2014)
!2014 = distinct !DILocation(line: 701, column: 34, scope: !2006)
!2015 = !DILocation(line: 575, column: 9, scope: !458, inlinedAt: !2014)
!2016 = !DILocation(line: 701, column: 34, scope: !2006)
!2017 = !DILocation(line: 701, column: 9, scope: !2006)
!2018 = !DILocalVariable(name: "self", arg: 1, scope: !2019, file: !459, line: 628, type: !91)
!2019 = distinct !DISubprogram(name: "assume_init<()>", linkageName: "_ZN4core3mem12maybe_uninit20MaybeUninit$LT$T$GT$11assume_init17h430837ac58c4ab48E", scope: !91, file: !459, line: 628, type: !2020, scopeLine: 628, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !98, retainedNodes: !2034)
!2020 = !DISubroutineType(types: !2021)
!2021 = !{null, !91, !2022}
!2022 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&Location", baseType: !2023, size: 32, align: 32, dwarfAddressSpace: 0)
!2023 = !DICompositeType(tag: DW_TAG_structure_type, name: "Location", scope: !2024, file: !6, size: 128, align: 32, elements: !2026, templateParams: !59, identifier: "375a94f916566009c415a6faa578d18c")
!2024 = !DINamespace(name: "location", scope: !2025)
!2025 = !DINamespace(name: "panic", scope: !12)
!2026 = !{!2027, !2032, !2033}
!2027 = !DIDerivedType(tag: DW_TAG_member, name: "file", scope: !2023, file: !6, baseType: !2028, size: 64, align: 32)
!2028 = !DICompositeType(tag: DW_TAG_structure_type, name: "&str", file: !6, size: 64, align: 32, elements: !2029, templateParams: !59, identifier: "7ef2a91eecc7bcf4b4aaea2dbce79437")
!2029 = !{!2030, !2031}
!2030 = !DIDerivedType(tag: DW_TAG_member, name: "data_ptr", scope: !2028, file: !6, baseType: !1950, size: 32, align: 32)
!2031 = !DIDerivedType(tag: DW_TAG_member, name: "length", scope: !2028, file: !6, baseType: !56, size: 32, align: 32, offset: 32)
!2032 = !DIDerivedType(tag: DW_TAG_member, name: "line", scope: !2023, file: !6, baseType: !1179, size: 32, align: 32, offset: 64)
!2033 = !DIDerivedType(tag: DW_TAG_member, name: "col", scope: !2023, file: !6, baseType: !1179, size: 32, align: 32, offset: 96)
!2034 = !{!2018}
!2035 = !DILocation(line: 628, column: 37, scope: !2019, inlinedAt: !2036)
!2036 = distinct !DILocation(line: 702, column: 9, scope: !2006)
!2037 = !DILocalVariable(name: "slot", arg: 1, scope: !2038, file: !1087, line: 87, type: !95)
!2038 = distinct !DISubprogram(name: "into_inner<()>", linkageName: "_ZN4core3mem13manually_drop21ManuallyDrop$LT$T$GT$10into_inner17hf7b926d0523427c7E", scope: !95, file: !1087, line: 87, type: !2039, scopeLine: 87, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !98, retainedNodes: !2041)
!2039 = !DISubroutineType(types: !2040)
!2040 = !{null, !95}
!2041 = !{!2037}
!2042 = !DILocation(line: 87, column: 29, scope: !2038, inlinedAt: !2043)
!2043 = distinct !DILocation(line: 633, column: 13, scope: !2019, inlinedAt: !2036)
!2044 = !DILocation(line: 702, column: 9, scope: !2006)
!2045 = !DILocation(line: 704, column: 2, scope: !2002)
!2046 = distinct !DISubprogram(name: "read<u8>", linkageName: "_ZN4core3ptr4read17h7efa0474a61340eeE", scope: !230, file: !229, line: 685, type: !1948, scopeLine: 685, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !74, retainedNodes: !2047)
!2047 = !{!2048, !2049}
!2048 = !DILocalVariable(name: "src", arg: 1, scope: !2046, file: !229, line: 685, type: !1950)
!2049 = !DILocalVariable(name: "tmp", scope: !2050, file: !229, line: 693, type: !66, align: 1)
!2050 = distinct !DILexicalBlock(scope: !2046, file: !229, line: 693, column: 5)
!2051 = !DILocation(line: 685, column: 29, scope: !2046)
!2052 = !DILocation(line: 693, column: 9, scope: !2050)
!2053 = !DILocation(line: 320, column: 9, scope: !2054, inlinedAt: !2057)
!2054 = distinct !DISubprogram(name: "uninit<u8>", linkageName: "_ZN4core3mem12maybe_uninit20MaybeUninit$LT$T$GT$6uninit17h6e651faebe85ff51E", scope: !66, file: !459, line: 319, type: !2055, scopeLine: 319, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !74, retainedNodes: !59)
!2055 = !DISubroutineType(types: !2056)
!2056 = !{!66}
!2057 = distinct !DILocation(line: 693, column: 19, scope: !2046)
!2058 = !DILocation(line: 321, column: 6, scope: !2054, inlinedAt: !2057)
!2059 = !DILocation(line: 693, column: 19, scope: !2046)
!2060 = !DILocalVariable(name: "self", arg: 1, scope: !2061, file: !459, line: 573, type: !2064)
!2061 = distinct !DISubprogram(name: "as_mut_ptr<u8>", linkageName: "_ZN4core3mem12maybe_uninit20MaybeUninit$LT$T$GT$10as_mut_ptr17hb4e10630e69656e1E", scope: !66, file: !459, line: 573, type: !2062, scopeLine: 573, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !74, retainedNodes: !2065)
!2062 = !DISubroutineType(types: !2063)
!2063 = !{!1330, !2064}
!2064 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&mut MaybeUninit<u8>", baseType: !66, size: 32, align: 32, dwarfAddressSpace: 0)
!2065 = !{!2060}
!2066 = !DILocation(line: 573, column: 29, scope: !2061, inlinedAt: !2067)
!2067 = distinct !DILocation(line: 701, column: 34, scope: !2050)
!2068 = !DILocation(line: 701, column: 34, scope: !2050)
!2069 = !DILocation(line: 701, column: 9, scope: !2050)
!2070 = !DILocation(line: 702, column: 9, scope: !2050)
!2071 = !DILocalVariable(name: "self", arg: 1, scope: !2072, file: !459, line: 628, type: !66)
!2072 = distinct !DISubprogram(name: "assume_init<u8>", linkageName: "_ZN4core3mem12maybe_uninit20MaybeUninit$LT$T$GT$11assume_init17h1235d1de8d620a2bE", scope: !66, file: !459, line: 628, type: !2073, scopeLine: 628, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !74, retainedNodes: !2075)
!2073 = !DISubroutineType(types: !2074)
!2074 = !{!73, !66, !2022}
!2075 = !{!2071}
!2076 = !DILocation(line: 628, column: 37, scope: !2072, inlinedAt: !2077)
!2077 = distinct !DILocation(line: 702, column: 9, scope: !2050)
!2078 = !DILocalVariable(name: "slot", arg: 1, scope: !2079, file: !1087, line: 87, type: !70)
!2079 = distinct !DISubprogram(name: "into_inner<u8>", linkageName: "_ZN4core3mem13manually_drop21ManuallyDrop$LT$T$GT$10into_inner17h67a2b3417f643c4eE", scope: !70, file: !1087, line: 87, type: !2080, scopeLine: 87, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !74, retainedNodes: !2082)
!2080 = !DISubroutineType(types: !2081)
!2081 = !{!73, !70}
!2082 = !{!2078}
!2083 = !DILocation(line: 87, column: 29, scope: !2079, inlinedAt: !2084)
!2084 = distinct !DILocation(line: 633, column: 13, scope: !2072, inlinedAt: !2077)
!2085 = !DILocation(line: 704, column: 2, scope: !2046)
!2086 = distinct !DISubprogram(name: "read<(locals::app::P1_T, u8)>", linkageName: "_ZN4core3ptr4read17haa4dd29abe51efc1E", scope: !230, file: !229, line: 685, type: !1931, scopeLine: 685, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !140, retainedNodes: !2087)
!2087 = !{!2088, !2089}
!2088 = !DILocalVariable(name: "src", arg: 1, scope: !2086, file: !229, line: 685, type: !1933)
!2089 = !DILocalVariable(name: "tmp", scope: !2090, file: !229, line: 693, type: !125, align: 1)
!2090 = distinct !DILexicalBlock(scope: !2086, file: !229, line: 693, column: 5)
!2091 = !DILocation(line: 685, column: 29, scope: !2086)
!2092 = !DILocation(line: 693, column: 9, scope: !2090)
!2093 = !DILocation(line: 320, column: 9, scope: !2094, inlinedAt: !2097)
!2094 = distinct !DISubprogram(name: "uninit<(locals::app::P1_T, u8)>", linkageName: "_ZN4core3mem12maybe_uninit20MaybeUninit$LT$T$GT$6uninit17h399c8efe78fd3eb6E", scope: !125, file: !459, line: 319, type: !2095, scopeLine: 319, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !140, retainedNodes: !59)
!2095 = !DISubroutineType(types: !2096)
!2096 = !{!125}
!2097 = distinct !DILocation(line: 693, column: 19, scope: !2086)
!2098 = !DILocation(line: 321, column: 6, scope: !2094, inlinedAt: !2097)
!2099 = !DILocation(line: 693, column: 19, scope: !2086)
!2100 = !DILocalVariable(name: "self", arg: 1, scope: !2101, file: !459, line: 573, type: !2105)
!2101 = distinct !DISubprogram(name: "as_mut_ptr<(locals::app::P1_T, u8)>", linkageName: "_ZN4core3mem12maybe_uninit20MaybeUninit$LT$T$GT$10as_mut_ptr17h946034ae0aa81bdfE", scope: !125, file: !459, line: 573, type: !2102, scopeLine: 573, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !140, retainedNodes: !2106)
!2102 = !DISubroutineType(types: !2103)
!2103 = !{!2104, !2105}
!2104 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*mut (locals::app::P1_T, u8)", baseType: !132, size: 32, align: 32, dwarfAddressSpace: 0)
!2105 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&mut MaybeUninit<(locals::app::P1_T, u8)>", baseType: !125, size: 32, align: 32, dwarfAddressSpace: 0)
!2106 = !{!2100}
!2107 = !DILocation(line: 573, column: 29, scope: !2101, inlinedAt: !2108)
!2108 = distinct !DILocation(line: 701, column: 34, scope: !2090)
!2109 = !DILocation(line: 701, column: 34, scope: !2090)
!2110 = !DILocation(line: 701, column: 9, scope: !2090)
!2111 = !DILocation(line: 702, column: 9, scope: !2090)
!2112 = !DILocalVariable(name: "self", arg: 1, scope: !2113, file: !459, line: 628, type: !125)
!2113 = distinct !DISubprogram(name: "assume_init<(locals::app::P1_T, u8)>", linkageName: "_ZN4core3mem12maybe_uninit20MaybeUninit$LT$T$GT$11assume_init17hfb49cccbe84dfa62E", scope: !125, file: !459, line: 628, type: !2114, scopeLine: 628, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !140, retainedNodes: !2116)
!2114 = !DISubroutineType(types: !2115)
!2115 = !{!132, !125, !2022}
!2116 = !{!2112}
!2117 = !DILocation(line: 628, column: 37, scope: !2113, inlinedAt: !2118)
!2118 = distinct !DILocation(line: 702, column: 9, scope: !2090)
!2119 = !DILocation(line: 633, column: 38, scope: !2113, inlinedAt: !2118)
!2120 = !DILocalVariable(name: "slot", arg: 1, scope: !2121, file: !1087, line: 87, type: !129)
!2121 = distinct !DISubprogram(name: "into_inner<(locals::app::P1_T, u8)>", linkageName: "_ZN4core3mem13manually_drop21ManuallyDrop$LT$T$GT$10into_inner17h6de50b426a0d7b91E", scope: !129, file: !1087, line: 87, type: !2122, scopeLine: 87, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !140, retainedNodes: !2124)
!2122 = !DISubroutineType(types: !2123)
!2123 = !{!132, !129}
!2124 = !{!2120}
!2125 = !DILocation(line: 87, column: 29, scope: !2121, inlinedAt: !2126)
!2126 = distinct !DILocation(line: 633, column: 13, scope: !2113, inlinedAt: !2118)
!2127 = !DILocation(line: 89, column: 6, scope: !2121, inlinedAt: !2126)
!2128 = !DILocation(line: 635, column: 6, scope: !2113, inlinedAt: !2118)
!2129 = !DILocation(line: 704, column: 2, scope: !2086)
!2130 = distinct !DISubprogram(name: "write<()>", linkageName: "_ZN4core3ptr5write17h3411c4701a77cae5E", scope: !230, file: !229, line: 880, type: !472, scopeLine: 880, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !98, retainedNodes: !2131)
!2131 = !{!2132, !2133}
!2132 = !DILocalVariable(name: "dst", arg: 1, scope: !2130, file: !229, line: 880, type: !251)
!2133 = !DILocalVariable(name: "src", arg: 2, scope: !2130, file: !229, line: 880, type: !20)
!2134 = !DILocation(line: 880, column: 30, scope: !2130)
!2135 = !DILocation(line: 880, column: 43, scope: !2130)
!2136 = !DILocation(line: 892, column: 9, scope: !2130)
!2137 = !DILocation(line: 895, column: 2, scope: !2130)
!2138 = distinct !DISubprogram(name: "write<u8>", linkageName: "_ZN4core3ptr5write17h44c3a12bdde493dfE", scope: !230, file: !229, line: 880, type: !2139, scopeLine: 880, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !74, retainedNodes: !2141)
!2139 = !DISubroutineType(types: !2140)
!2140 = !{null, !1330, !73}
!2141 = !{!2142, !2143}
!2142 = !DILocalVariable(name: "dst", arg: 1, scope: !2138, file: !229, line: 880, type: !1330)
!2143 = !DILocalVariable(name: "src", arg: 2, scope: !2138, file: !229, line: 880, type: !73)
!2144 = !DILocation(line: 880, column: 30, scope: !2138)
!2145 = !DILocation(line: 880, column: 43, scope: !2138)
!2146 = !DILocation(line: 892, column: 9, scope: !2138)
!2147 = !DILocation(line: 895, column: 2, scope: !2138)
!2148 = distinct !DISubprogram(name: "write<core::mem::maybe_uninit::MaybeUninit<u8>>", linkageName: "_ZN4core3ptr5write17h61a4480ba65cfdcfE", scope: !230, file: !229, line: 880, type: !1583, scopeLine: 880, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !76, retainedNodes: !2149)
!2149 = !{!2150, !2151}
!2150 = !DILocalVariable(name: "dst", arg: 1, scope: !2148, file: !229, line: 880, type: !1556)
!2151 = !DILocalVariable(name: "src", arg: 2, scope: !2148, file: !229, line: 880, type: !66)
!2152 = !DILocation(line: 880, column: 30, scope: !2148)
!2153 = !DILocation(line: 880, column: 43, scope: !2148)
!2154 = !DILocation(line: 892, column: 9, scope: !2148)
!2155 = !DILocation(line: 895, column: 2, scope: !2148)
!2156 = distinct !DISubprogram(name: "write<core::mem::maybe_uninit::MaybeUninit<i64>>", linkageName: "_ZN4core3ptr5write17h82b9f7fb1924c0bdE", scope: !230, file: !229, line: 880, type: !1099, scopeLine: 880, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !29, retainedNodes: !2157)
!2157 = !{!2158, !2159}
!2158 = !DILocalVariable(name: "dst", arg: 1, scope: !2156, file: !229, line: 880, type: !821)
!2159 = !DILocalVariable(name: "src", arg: 2, scope: !2156, file: !229, line: 880, type: !15)
!2160 = !DILocation(line: 880, column: 30, scope: !2156)
!2161 = !DILocation(line: 880, column: 43, scope: !2156)
!2162 = !DILocation(line: 892, column: 9, scope: !2156)
!2163 = !DILocation(line: 895, column: 2, scope: !2156)
!2164 = distinct !DISubprogram(name: "write<core::mem::maybe_uninit::MaybeUninit<(locals::app::P1_T, u8)>>", linkageName: "_ZN4core3ptr5write17hcf998c1cb7ac1d77E", scope: !230, file: !229, line: 880, type: !1640, scopeLine: 880, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !142, retainedNodes: !2165)
!2165 = !{!2166, !2167}
!2166 = !DILocalVariable(name: "dst", arg: 1, scope: !2164, file: !229, line: 880, type: !1613)
!2167 = !DILocalVariable(name: "src", arg: 2, scope: !2164, file: !229, line: 880, type: !125)
!2168 = !DILocation(line: 880, column: 30, scope: !2164)
!2169 = !DILocation(line: 880, column: 43, scope: !2164)
!2170 = !DILocation(line: 892, column: 9, scope: !2164)
!2171 = !DILocation(line: 895, column: 2, scope: !2164)
!2172 = distinct !DISubprogram(name: "into<cortex_m::peripheral::Peripherals, cortex_m::peripheral::Peripherals>", linkageName: "_ZN50_$LT$T$u20$as$u20$core..convert..Into$LT$U$GT$$GT$4into17hb837751d062ab74fE", scope: !2174, file: !2173, line: 542, type: !1050, scopeLine: 542, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !2178, retainedNodes: !2176)
!2173 = !DIFile(filename: "/rustc/9d1b2106e23b1abd32fce1f17267604a5102f57a/library/core/src/convert/mod.rs", directory: "", checksumkind: CSK_MD5, checksum: "b0c222d3c5929ece18f8a89a151f3a67")
!2174 = !DINamespace(name: "{impl#3}", scope: !2175)
!2175 = !DINamespace(name: "convert", scope: !12)
!2176 = !{!2177}
!2177 = !DILocalVariable(name: "self", arg: 1, scope: !2172, file: !2173, line: 542, type: !888)
!2178 = !{!2179, !2180}
!2179 = !DITemplateTypeParameter(name: "T", type: !888)
!2180 = !DITemplateTypeParameter(name: "U", type: !888)
!2181 = !DILocation(line: 542, column: 13, scope: !2172)
!2182 = !DILocation(line: 543, column: 9, scope: !2172)
!2183 = !DILocation(line: 544, column: 6, scope: !2172)
!2184 = distinct !DISubprogram(name: "from<cortex_m::peripheral::Peripherals>", linkageName: "_ZN50_$LT$T$u20$as$u20$core..convert..From$LT$T$GT$$GT$4from17hae6616355c332270E", scope: !2185, file: !2173, line: 551, type: !1050, scopeLine: 551, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !2188, retainedNodes: !2186)
!2185 = !DINamespace(name: "{impl#4}", scope: !2175)
!2186 = !{!2187}
!2187 = !DILocalVariable(name: "t", arg: 1, scope: !2184, file: !2173, line: 551, type: !888)
!2188 = !{!2179}
!2189 = !DILocation(line: 551, column: 13, scope: !2184)
!2190 = !DILocation(line: 553, column: 6, scope: !2184)
!2191 = distinct !DISubprogram(name: "new<u8>", linkageName: "_ZN4core4cell13Cell$LT$T$GT$3new17h0ec9730ff8e64e09E", scope: !755, file: !449, line: 336, type: !2192, scopeLine: 336, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !74, retainedNodes: !2194)
!2192 = !DISubroutineType(types: !2193)
!2193 = !{!755, !73}
!2194 = !{!2195}
!2195 = !DILocalVariable(name: "value", arg: 1, scope: !2191, file: !449, line: 336, type: !73)
!2196 = !DILocation(line: 336, column: 22, scope: !2191)
!2197 = !DILocalVariable(name: "value", arg: 1, scope: !2198, file: !449, line: 1870, type: !73)
!2198 = distinct !DISubprogram(name: "new<u8>", linkageName: "_ZN4core4cell19UnsafeCell$LT$T$GT$3new17h9340055361c5cd64E", scope: !758, file: !449, line: 1870, type: !2199, scopeLine: 1870, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !74, retainedNodes: !2201)
!2199 = !DISubroutineType(types: !2200)
!2200 = !{!758, !73}
!2201 = !{!2197}
!2202 = !DILocation(line: 1870, column: 22, scope: !2198, inlinedAt: !2203)
!2203 = distinct !DILocation(line: 337, column: 23, scope: !2191)
!2204 = !DILocation(line: 1871, column: 9, scope: !2198, inlinedAt: !2203)
!2205 = !DILocation(line: 1872, column: 6, scope: !2198, inlinedAt: !2203)
!2206 = !DILocation(line: 337, column: 23, scope: !2191)
!2207 = !DILocation(line: 337, column: 9, scope: !2191)
!2208 = !DILocation(line: 338, column: 6, scope: !2191)
!2209 = distinct !DISubprogram(name: "spec_next<u8>", linkageName: "_ZN89_$LT$core..ops..range..Range$LT$T$GT$$u20$as$u20$core..iter..range..RangeIteratorImpl$GT$9spec_next17he4c612b2254b2254E", scope: !2211, file: !2210, line: 620, type: !2214, scopeLine: 620, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !74, retainedNodes: !2225)
!2210 = !DIFile(filename: "/rustc/9d1b2106e23b1abd32fce1f17267604a5102f57a/library/core/src/iter/range.rs", directory: "", checksumkind: CSK_MD5, checksum: "e860d54a407eb4bee70316e8f1009b86")
!2211 = !DINamespace(name: "{impl#2}", scope: !2212)
!2212 = !DINamespace(name: "range", scope: !2213)
!2213 = !DINamespace(name: "iter", scope: !12)
!2214 = !DISubroutineType(types: !2215)
!2215 = !{!485, !2216}
!2216 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&mut Range<u8>", baseType: !2217, size: 32, align: 32, dwarfAddressSpace: 0)
!2217 = !DICompositeType(tag: DW_TAG_structure_type, name: "Range<u8>", scope: !2218, file: !6, size: 16, align: 8, elements: !2220, templateParams: !2223, identifier: "30ce1212ce865043ad3f38490ef114c0")
!2218 = !DINamespace(name: "range", scope: !2219)
!2219 = !DINamespace(name: "ops", scope: !12)
!2220 = !{!2221, !2222}
!2221 = !DIDerivedType(tag: DW_TAG_member, name: "start", scope: !2217, file: !6, baseType: !73, size: 8, align: 8)
!2222 = !DIDerivedType(tag: DW_TAG_member, name: "end", scope: !2217, file: !6, baseType: !73, size: 8, align: 8, offset: 8)
!2223 = !{!2224}
!2224 = !DITemplateTypeParameter(name: "Idx", type: !73)
!2225 = !{!2226, !2227}
!2226 = !DILocalVariable(name: "self", arg: 1, scope: !2209, file: !2210, line: 620, type: !2216)
!2227 = !DILocalVariable(name: "n", scope: !2228, file: !2210, line: 623, type: !73, align: 1)
!2228 = distinct !DILexicalBlock(scope: !2209, file: !2210, line: 623, column: 13)
!2229 = !DILocation(line: 620, column: 18, scope: !2209)
!2230 = !DILocation(line: 621, column: 12, scope: !2209)
!2231 = !DILocation(line: 621, column: 25, scope: !2209)
!2232 = !DILocation(line: 626, column: 13, scope: !2209)
!2233 = !DILocation(line: 621, column: 9, scope: !2209)
!2234 = !DILocation(line: 623, column: 54, scope: !2209)
!2235 = !DILocation(line: 623, column: 30, scope: !2209)
!2236 = !DILocation(line: 623, column: 17, scope: !2228)
!2237 = !DILocation(line: 624, column: 31, scope: !2228)
!2238 = !DILocation(line: 624, column: 18, scope: !2228)
!2239 = !DILocation(line: 624, column: 13, scope: !2228)
!2240 = !DILocation(line: 628, column: 6, scope: !2209)
!2241 = distinct !DISubprogram(name: "next<u8>", linkageName: "_ZN4core4iter5range101_$LT$impl$u20$core..iter..traits..iterator..Iterator$u20$for$u20$core..ops..range..Range$LT$A$GT$$GT$4next17hcdb043e911fa495fE", scope: !2242, file: !2210, line: 710, type: !2214, scopeLine: 710, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !2245, retainedNodes: !2243)
!2242 = !DINamespace(name: "{impl#3}", scope: !2212)
!2243 = !{!2244}
!2244 = !DILocalVariable(name: "self", arg: 1, scope: !2241, file: !2210, line: 710, type: !2216)
!2245 = !{!2246}
!2246 = !DITemplateTypeParameter(name: "A", type: !73)
!2247 = !DILocation(line: 710, column: 13, scope: !2241)
!2248 = !DILocation(line: 711, column: 9, scope: !2241)
!2249 = !DILocation(line: 712, column: 6, scope: !2241)
!2250 = distinct !DISubprogram(name: "for_each<core::ops::range::Range<u8>, locals::app::rtic_ext::main::{closure#1}>", linkageName: "_ZN4core4iter6traits8iterator8Iterator8for_each17h003edabc4355739dE", scope: !2252, file: !2251, line: 726, type: !2255, scopeLine: 726, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !2260, retainedNodes: !2257)
!2251 = !DIFile(filename: "/rustc/9d1b2106e23b1abd32fce1f17267604a5102f57a/library/core/src/iter/traits/iterator.rs", directory: "", checksumkind: CSK_MD5, checksum: "e98d086c50152624a7369b78543346a5")
!2252 = !DINamespace(name: "Iterator", scope: !2253)
!2253 = !DINamespace(name: "iterator", scope: !2254)
!2254 = !DINamespace(name: "traits", scope: !2213)
!2255 = !DISubroutineType(types: !2256)
!2256 = !{null, !2217, !998}
!2257 = !{!2258, !2259}
!2258 = !DILocalVariable(name: "self", arg: 1, scope: !2250, file: !2251, line: 726, type: !2217)
!2259 = !DILocalVariable(name: "f", arg: 2, scope: !2250, file: !2251, line: 726, type: !998)
!2260 = !{!2261, !2262}
!2261 = !DITemplateTypeParameter(name: "Self", type: !2217)
!2262 = !DITemplateTypeParameter(name: "F", type: !998)
!2263 = !DILocation(line: 726, column: 20, scope: !2250)
!2264 = !DILocation(line: 726, column: 26, scope: !2250)
!2265 = !DILocation(line: 736, column: 23, scope: !2250)
!2266 = !DILocation(line: 736, column: 9, scope: !2250)
!2267 = !DILocation(line: 737, column: 6, scope: !2250)
!2268 = distinct !DISubprogram(name: "for_each<core::ops::range::Range<u8>, locals::app::rtic_ext::main::{closure#0}>", linkageName: "_ZN4core4iter6traits8iterator8Iterator8for_each17h248455fa942f1094E", scope: !2252, file: !2251, line: 726, type: !2269, scopeLine: 726, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !2274, retainedNodes: !2271)
!2269 = !DISubroutineType(types: !2270)
!2270 = !{null, !2217, !984}
!2271 = !{!2272, !2273}
!2272 = !DILocalVariable(name: "self", arg: 1, scope: !2268, file: !2251, line: 726, type: !2217)
!2273 = !DILocalVariable(name: "f", arg: 2, scope: !2268, file: !2251, line: 726, type: !984)
!2274 = !{!2261, !2275}
!2275 = !DITemplateTypeParameter(name: "F", type: !984)
!2276 = !DILocation(line: 726, column: 20, scope: !2268)
!2277 = !DILocation(line: 726, column: 26, scope: !2268)
!2278 = !DILocation(line: 736, column: 23, scope: !2268)
!2279 = !DILocation(line: 736, column: 9, scope: !2268)
!2280 = !DILocation(line: 737, column: 6, scope: !2268)
!2281 = distinct !DISubprogram(name: "call<u8, locals::app::rtic_ext::main::{closure#0}>", linkageName: "_ZN4core4iter6traits8iterator8Iterator8for_each4call17h6690a92d87f028f3E", scope: !2282, file: !2251, line: 732, type: !2283, scopeLine: 732, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !2287, retainedNodes: !2285)
!2282 = !DINamespace(name: "for_each", scope: !2252)
!2283 = !DISubroutineType(types: !2284)
!2284 = !{null, !984}
!2285 = !{!2286}
!2286 = !DILocalVariable(name: "f", arg: 1, scope: !2281, file: !2251, line: 732, type: !984)
!2287 = !{!75, !2288}
!2288 = !DITemplateTypeParameter(name: "impl FnMut(T)", type: !984)
!2289 = !DILocation(line: 732, column: 20, scope: !2281)
!2290 = !DILocation(line: 733, column: 13, scope: !2281)
!2291 = !DILocation(line: 734, column: 10, scope: !2281)
!2292 = distinct !DISubprogram(name: "call<u8, locals::app::rtic_ext::main::{closure#1}>", linkageName: "_ZN4core4iter6traits8iterator8Iterator8for_each4call17ha52f4198052b8887E", scope: !2282, file: !2251, line: 732, type: !2293, scopeLine: 732, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !2297, retainedNodes: !2295)
!2293 = !DISubroutineType(types: !2294)
!2294 = !{null, !998}
!2295 = !{!2296}
!2296 = !DILocalVariable(name: "f", arg: 1, scope: !2292, file: !2251, line: 732, type: !998)
!2297 = !{!75, !2298}
!2298 = !DITemplateTypeParameter(name: "impl FnMut(T)", type: !998)
!2299 = !DILocation(line: 732, column: 20, scope: !2292)
!2300 = !DILocation(line: 733, column: 13, scope: !2292)
!2301 = !DILocation(line: 734, column: 10, scope: !2292)
!2302 = distinct !DISubprogram(name: "{closure#0}<u8, locals::app::rtic_ext::main::{closure#1}>", linkageName: "_ZN4core4iter6traits8iterator8Iterator8for_each4call28_$u7b$$u7b$closure$u7d$$u7d$17h27b028bfd3eed342E", scope: !2303, file: !2251, line: 733, type: !2304, scopeLine: 733, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !2297, retainedNodes: !2310)
!2303 = !DINamespace(name: "call", scope: !2282)
!2304 = !DISubroutineType(types: !2305)
!2305 = !{null, !2306, !20, !73}
!2306 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&mut {closure#0}", baseType: !2307, size: 32, align: 32, dwarfAddressSpace: 0)
!2307 = !DICompositeType(tag: DW_TAG_structure_type, name: "{closure#0}", scope: !2303, file: !6, align: 8, elements: !2308, templateParams: !59, identifier: "2e40ed888e058d41ab8b72d3519729fd")
!2308 = !{!2309}
!2309 = !DIDerivedType(tag: DW_TAG_member, name: "f", scope: !2307, file: !6, baseType: !998, align: 8)
!2310 = !{!2311, !2312, !2313}
!2311 = !DILocalVariable(name: "item", arg: 3, scope: !2302, file: !2251, line: 733, type: !73)
!2312 = !DILocalVariable(name: "f", scope: !2302, file: !2251, line: 732, type: !998, align: 1)
!2313 = !DILocalVariable(arg: 2, scope: !2302, file: !2251, line: 733, type: !20)
!2314 = !DILocation(line: 732, column: 20, scope: !2302)
!2315 = !DILocation(line: 733, column: 19, scope: !2302)
!2316 = !DILocation(line: 733, column: 23, scope: !2302)
!2317 = !DILocation(line: 733, column: 29, scope: !2302)
!2318 = !DILocation(line: 733, column: 36, scope: !2302)
!2319 = distinct !DISubprogram(name: "{closure#0}<u8, locals::app::rtic_ext::main::{closure#0}>", linkageName: "_ZN4core4iter6traits8iterator8Iterator8for_each4call28_$u7b$$u7b$closure$u7d$$u7d$17h8120bf3118aae8a7E", scope: !2303, file: !2251, line: 733, type: !2320, scopeLine: 733, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !2287, retainedNodes: !2326)
!2320 = !DISubroutineType(types: !2321)
!2321 = !{null, !2322, !20, !73}
!2322 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&mut {closure#0}", baseType: !2323, size: 32, align: 32, dwarfAddressSpace: 0)
!2323 = !DICompositeType(tag: DW_TAG_structure_type, name: "{closure#0}", scope: !2303, file: !6, align: 8, elements: !2324, templateParams: !59, identifier: "bbaecdcae70bccd9c66cd2a79cee2ef6")
!2324 = !{!2325}
!2325 = !DIDerivedType(tag: DW_TAG_member, name: "f", scope: !2323, file: !6, baseType: !984, align: 8)
!2326 = !{!2327, !2328, !2329}
!2327 = !DILocalVariable(name: "item", arg: 3, scope: !2319, file: !2251, line: 733, type: !73)
!2328 = !DILocalVariable(name: "f", scope: !2319, file: !2251, line: 732, type: !984, align: 1)
!2329 = !DILocalVariable(arg: 2, scope: !2319, file: !2251, line: 733, type: !20)
!2330 = !DILocation(line: 732, column: 20, scope: !2319)
!2331 = !DILocation(line: 733, column: 19, scope: !2319)
!2332 = !DILocation(line: 733, column: 23, scope: !2319)
!2333 = !DILocation(line: 733, column: 29, scope: !2319)
!2334 = !DILocation(line: 733, column: 36, scope: !2319)
!2335 = distinct !DISubprogram(name: "fold<core::ops::range::Range<u8>, (), core::iter::traits::iterator::Iterator::for_each::call::{closure#0}>", linkageName: "_ZN4core4iter6traits8iterator8Iterator4fold17h66e2ab47353ef40dE", scope: !2252, file: !2251, line: 2164, type: !2336, scopeLine: 2164, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !2346, retainedNodes: !2338)
!2336 = !DISubroutineType(types: !2337)
!2337 = !{null, !2217, !20, !2307}
!2338 = !{!2339, !2340, !2341, !2342, !2344}
!2339 = !DILocalVariable(name: "self", arg: 1, scope: !2335, file: !2251, line: 2164, type: !2217)
!2340 = !DILocalVariable(name: "init", arg: 2, scope: !2335, file: !2251, line: 2164, type: !20)
!2341 = !DILocalVariable(name: "f", arg: 3, scope: !2335, file: !2251, line: 2164, type: !2307)
!2342 = !DILocalVariable(name: "accum", scope: !2343, file: !2251, line: 2169, type: !20, align: 1)
!2343 = distinct !DILexicalBlock(scope: !2335, file: !2251, line: 2169, column: 9)
!2344 = !DILocalVariable(name: "x", scope: !2345, file: !2251, line: 2170, type: !73, align: 1)
!2345 = distinct !DILexicalBlock(scope: !2343, file: !2251, line: 2170, column: 19)
!2346 = !{!2261, !2347, !2348}
!2347 = !DITemplateTypeParameter(name: "B", type: !20)
!2348 = !DITemplateTypeParameter(name: "F", type: !2307)
!2349 = !DILocation(line: 2169, column: 13, scope: !2343)
!2350 = !DILocation(line: 2164, column: 19, scope: !2335)
!2351 = !DILocation(line: 2164, column: 29, scope: !2335)
!2352 = !DILocation(line: 2164, column: 38, scope: !2335)
!2353 = !DILocation(line: 2170, column: 9, scope: !2343)
!2354 = !DILocation(line: 2170, column: 29, scope: !2343)
!2355 = !DILocation(line: 2170, column: 19, scope: !2343)
!2356 = !DILocation(line: 2170, column: 24, scope: !2343)
!2357 = !DILocation(line: 2170, column: 24, scope: !2345)
!2358 = !DILocation(line: 2171, column: 21, scope: !2343)
!2359 = !DILocation(line: 2174, column: 5, scope: !2335)
!2360 = !DILocation(line: 2174, column: 6, scope: !2335)
!2361 = distinct !DISubprogram(name: "fold<core::ops::range::Range<u8>, (), core::iter::traits::iterator::Iterator::for_each::call::{closure#0}>", linkageName: "_ZN4core4iter6traits8iterator8Iterator4fold17haea35f2c93648cf3E", scope: !2252, file: !2251, line: 2164, type: !2362, scopeLine: 2164, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !2372, retainedNodes: !2364)
!2362 = !DISubroutineType(types: !2363)
!2363 = !{null, !2217, !20, !2323}
!2364 = !{!2365, !2366, !2367, !2368, !2370}
!2365 = !DILocalVariable(name: "self", arg: 1, scope: !2361, file: !2251, line: 2164, type: !2217)
!2366 = !DILocalVariable(name: "init", arg: 2, scope: !2361, file: !2251, line: 2164, type: !20)
!2367 = !DILocalVariable(name: "f", arg: 3, scope: !2361, file: !2251, line: 2164, type: !2323)
!2368 = !DILocalVariable(name: "accum", scope: !2369, file: !2251, line: 2169, type: !20, align: 1)
!2369 = distinct !DILexicalBlock(scope: !2361, file: !2251, line: 2169, column: 9)
!2370 = !DILocalVariable(name: "x", scope: !2371, file: !2251, line: 2170, type: !73, align: 1)
!2371 = distinct !DILexicalBlock(scope: !2369, file: !2251, line: 2170, column: 19)
!2372 = !{!2261, !2347, !2373}
!2373 = !DITemplateTypeParameter(name: "F", type: !2323)
!2374 = !DILocation(line: 2169, column: 13, scope: !2369)
!2375 = !DILocation(line: 2164, column: 19, scope: !2361)
!2376 = !DILocation(line: 2164, column: 29, scope: !2361)
!2377 = !DILocation(line: 2164, column: 38, scope: !2361)
!2378 = !DILocation(line: 2170, column: 9, scope: !2369)
!2379 = !DILocation(line: 2170, column: 29, scope: !2369)
!2380 = !DILocation(line: 2170, column: 19, scope: !2369)
!2381 = !DILocation(line: 2170, column: 24, scope: !2369)
!2382 = !DILocation(line: 2170, column: 24, scope: !2371)
!2383 = !DILocation(line: 2171, column: 21, scope: !2369)
!2384 = !DILocation(line: 2174, column: 5, scope: !2361)
!2385 = !DILocation(line: 2174, column: 6, scope: !2361)
!2386 = distinct !DISubprogram(name: "unwrap<(), ()>", linkageName: "_ZN4core6result19Result$LT$T$C$E$GT$6unwrap17had281be05a095966E", scope: !198, file: !2387, line: 1012, type: !2388, scopeLine: 1012, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !2396, retainedNodes: !2390)
!2387 = !DIFile(filename: "/rustc/9d1b2106e23b1abd32fce1f17267604a5102f57a/library/core/src/result.rs", directory: "", checksumkind: CSK_MD5, checksum: "d707081f395d898bde7a53d998c9bcd3")
!2388 = !DISubroutineType(types: !2389)
!2389 = !{null, !198, !2022}
!2390 = !{!2391, !2392, !2394}
!2391 = !DILocalVariable(name: "self", arg: 1, scope: !2386, file: !2387, line: 1012, type: !198)
!2392 = !DILocalVariable(name: "t", scope: !2393, file: !2387, line: 1017, type: !20, align: 1)
!2393 = distinct !DILexicalBlock(scope: !2386, file: !2387, line: 1017, column: 13)
!2394 = !DILocalVariable(name: "e", scope: !2395, file: !2387, line: 1018, type: !20, align: 1)
!2395 = distinct !DILexicalBlock(scope: !2386, file: !2387, line: 1018, column: 13)
!2396 = !{!99, !2397}
!2397 = !DITemplateTypeParameter(name: "E", type: !20)
!2398 = !DILocation(line: 1017, column: 16, scope: !2393)
!2399 = !DILocation(line: 1012, column: 19, scope: !2386)
!2400 = !DILocation(line: 1018, column: 17, scope: !2395)
!2401 = !DILocation(line: 1016, column: 15, scope: !2386)
!2402 = !DILocation(line: 1016, column: 9, scope: !2386)
!2403 = !DILocation(line: 1020, column: 6, scope: !2386)
!2404 = !DILocation(line: 1018, column: 23, scope: !2395)
!2405 = distinct !DISubprogram(name: "atomic_store<usize>", linkageName: "_ZN4core4sync6atomic12atomic_store17hf215097f88ea7971E", scope: !49, file: !2406, line: 2345, type: !2407, scopeLine: 2345, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !57, retainedNodes: !2410)
!2406 = !DIFile(filename: "/rustc/9d1b2106e23b1abd32fce1f17267604a5102f57a/library/core/src/sync/atomic.rs", directory: "", checksumkind: CSK_MD5, checksum: "eb8b32b486c739bd751affdd3774b113")
!2407 = !DISubroutineType(types: !2408)
!2408 = !{null, !2409, !56, !210}
!2409 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*mut usize", baseType: !56, size: 32, align: 32, dwarfAddressSpace: 0)
!2410 = !{!2411, !2412, !2413}
!2411 = !DILocalVariable(name: "dst", arg: 1, scope: !2405, file: !2406, line: 2345, type: !2409)
!2412 = !DILocalVariable(name: "val", arg: 2, scope: !2405, file: !2406, line: 2345, type: !56)
!2413 = !DILocalVariable(name: "order", arg: 3, scope: !2405, file: !2406, line: 2345, type: !210)
!2414 = !DILocation(line: 2345, column: 33, scope: !2405)
!2415 = !DILocation(line: 2345, column: 46, scope: !2405)
!2416 = !DILocation(line: 2345, column: 54, scope: !2405)
!2417 = !DILocation(line: 2348, column: 15, scope: !2405)
!2418 = !DILocation(line: 2348, column: 9, scope: !2405)
!2419 = !DILocation(line: 2350, column: 24, scope: !2405)
!2420 = !DILocation(line: 2349, column: 24, scope: !2405)
!2421 = !DILocation(line: 2352, column: 24, scope: !2405)
!2422 = !DILocation(line: 2353, column: 23, scope: !2405)
!2423 = !DILocation(line: 2351, column: 23, scope: !2405)
!2424 = !DILocation(line: 2351, column: 56, scope: !2405)
!2425 = !DILocation(line: 2356, column: 2, scope: !2405)
!2426 = !DILocation(line: 2349, column: 61, scope: !2405)
!2427 = !DILocation(line: 2350, column: 65, scope: !2405)
!2428 = distinct !DISubprogram(name: "atomic_load<usize>", linkageName: "_ZN4core4sync6atomic11atomic_load17h774af4d42bd95514E", scope: !49, file: !2406, line: 2359, type: !2429, scopeLine: 2359, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !57, retainedNodes: !2432)
!2429 = !DISubroutineType(types: !2430)
!2430 = !{!56, !2431, !210}
!2431 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*const usize", baseType: !56, size: 32, align: 32, dwarfAddressSpace: 0)
!2432 = !{!2433, !2434}
!2433 = !DILocalVariable(name: "dst", arg: 1, scope: !2428, file: !2406, line: 2359, type: !2431)
!2434 = !DILocalVariable(name: "order", arg: 2, scope: !2428, file: !2406, line: 2359, type: !210)
!2435 = !DILocation(line: 2359, column: 32, scope: !2428)
!2436 = !DILocation(line: 2359, column: 47, scope: !2428)
!2437 = !DILocation(line: 2362, column: 15, scope: !2428)
!2438 = !DILocation(line: 2362, column: 9, scope: !2428)
!2439 = !DILocation(line: 2364, column: 24, scope: !2428)
!2440 = !DILocation(line: 2366, column: 24, scope: !2428)
!2441 = !DILocation(line: 2363, column: 24, scope: !2428)
!2442 = !DILocation(line: 2367, column: 23, scope: !2428)
!2443 = !DILocation(line: 2365, column: 23, scope: !2428)
!2444 = !DILocation(line: 2365, column: 50, scope: !2428)
!2445 = !DILocation(line: 2370, column: 2, scope: !2428)
!2446 = !DILocation(line: 2363, column: 55, scope: !2428)
!2447 = !DILocation(line: 2364, column: 59, scope: !2428)
!2448 = distinct !DISubprogram(name: "new<&mut i64>", linkageName: "_ZN4core3fmt10ArgumentV13new17h85113322f5b5e658E", scope: !2450, file: !2449, line: 314, type: !2485, scopeLine: 314, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !2493, retainedNodes: !2490)
!2449 = !DIFile(filename: "/rustc/9d1b2106e23b1abd32fce1f17267604a5102f57a/library/core/src/fmt/mod.rs", directory: "", checksumkind: CSK_MD5, checksum: "3f502a0de6625195d1b1e100f1964677")
!2450 = !DICompositeType(tag: DW_TAG_structure_type, name: "ArgumentV1", scope: !220, file: !6, size: 64, align: 32, elements: !2451, templateParams: !59, identifier: "7ac15a37f207a870543bd2b5c4618142")
!2451 = !{!2452, !2455}
!2452 = !DIDerivedType(tag: DW_TAG_member, name: "value", scope: !2450, file: !6, baseType: !2453, size: 32, align: 32)
!2453 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&Opaque", baseType: !2454, size: 32, align: 32, dwarfAddressSpace: 0)
!2454 = !DICompositeType(tag: DW_TAG_structure_type, name: "Opaque", file: !6, align: 8, elements: !59, identifier: "e6e2ce4ccb1b66b315cb9697d9c1957")
!2455 = !DIDerivedType(tag: DW_TAG_member, name: "formatter", scope: !2450, file: !6, baseType: !2456, size: 32, align: 32, offset: 32)
!2456 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "fn(&core::fmt::{extern#0}::Opaque, &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error>", baseType: !2457, size: 32, align: 32, dwarfAddressSpace: 0)
!2457 = !DISubroutineType(types: !2458)
!2458 = !{!198, !2453, !2459}
!2459 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&mut Formatter", baseType: !2460, size: 32, align: 32, dwarfAddressSpace: 0)
!2460 = !DICompositeType(tag: DW_TAG_structure_type, name: "Formatter", scope: !220, file: !6, size: 288, align: 32, elements: !2461, templateParams: !59, identifier: "4e0a1079e7d122981e6dd08d4af92dfd")
!2461 = !{!2462, !2463, !2465, !2466, !2477, !2478}
!2462 = !DIDerivedType(tag: DW_TAG_member, name: "flags", scope: !2460, file: !6, baseType: !1179, size: 32, align: 32)
!2463 = !DIDerivedType(tag: DW_TAG_member, name: "fill", scope: !2460, file: !6, baseType: !2464, size: 32, align: 32, offset: 32)
!2464 = !DIBasicType(name: "char", size: 32, encoding: DW_ATE_unsigned_char)
!2465 = !DIDerivedType(tag: DW_TAG_member, name: "align", scope: !2460, file: !6, baseType: !217, size: 8, align: 8, offset: 256)
!2466 = !DIDerivedType(tag: DW_TAG_member, name: "width", scope: !2460, file: !6, baseType: !2467, size: 64, align: 32, offset: 64)
!2467 = !DICompositeType(tag: DW_TAG_structure_type, name: "Option<usize>", scope: !486, file: !6, size: 64, align: 32, elements: !2468, identifier: "b9a2084aef7fc3fd73fb09827d24298")
!2468 = !{!2469}
!2469 = !DICompositeType(tag: DW_TAG_variant_part, scope: !486, file: !6, size: 64, align: 32, elements: !2470, templateParams: !57, identifier: "b9a2084aef7fc3fd73fb09827d24298_variant_part", discriminator: !1795)
!2470 = !{!2471, !2473}
!2471 = !DIDerivedType(tag: DW_TAG_member, name: "None", scope: !2469, file: !6, baseType: !2472, size: 64, align: 32, extraData: i64 0)
!2472 = !DICompositeType(tag: DW_TAG_structure_type, name: "None", scope: !2467, file: !6, size: 64, align: 32, elements: !59, templateParams: !57, identifier: "b9a2084aef7fc3fd73fb09827d24298::None")
!2473 = !DIDerivedType(tag: DW_TAG_member, name: "Some", scope: !2469, file: !6, baseType: !2474, size: 64, align: 32, extraData: i64 1)
!2474 = !DICompositeType(tag: DW_TAG_structure_type, name: "Some", scope: !2467, file: !6, size: 64, align: 32, elements: !2475, templateParams: !57, identifier: "b9a2084aef7fc3fd73fb09827d24298::Some")
!2475 = !{!2476}
!2476 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !2474, file: !6, baseType: !56, size: 32, align: 32, offset: 32)
!2477 = !DIDerivedType(tag: DW_TAG_member, name: "precision", scope: !2460, file: !6, baseType: !2467, size: 64, align: 32, offset: 128)
!2478 = !DIDerivedType(tag: DW_TAG_member, name: "buf", scope: !2460, file: !6, baseType: !2479, size: 64, align: 32, offset: 192)
!2479 = !DICompositeType(tag: DW_TAG_structure_type, name: "&mut dyn core::fmt::Write", file: !6, size: 64, align: 32, elements: !2480, templateParams: !59, identifier: "c083f0a92d27b2dda11083a9dedf39f7")
!2480 = !{!2481, !2482}
!2481 = !DIDerivedType(tag: DW_TAG_member, name: "pointer", scope: !2479, file: !6, baseType: !1330, size: 32, align: 32, flags: DIFlagArtificial)
!2482 = !DIDerivedType(tag: DW_TAG_member, name: "vtable", scope: !2479, file: !6, baseType: !2483, size: 32, align: 32, offset: 32, flags: DIFlagArtificial)
!2483 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&[usize; 3]", baseType: !2484, size: 32, align: 32, dwarfAddressSpace: 0)
!2484 = !DICompositeType(tag: DW_TAG_array_type, baseType: !56, size: 96, align: 32, elements: !144)
!2485 = !DISubroutineType(types: !2486)
!2486 = !{!2450, !366, !2487}
!2487 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "fn(&&mut i64, &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error>", baseType: !2488, size: 32, align: 32, dwarfAddressSpace: 0)
!2488 = !DISubroutineType(types: !2489)
!2489 = !{!198, !366, !2459}
!2490 = !{!2491, !2492}
!2491 = !DILocalVariable(name: "x", arg: 1, scope: !2448, file: !2449, line: 314, type: !366)
!2492 = !DILocalVariable(name: "f", arg: 2, scope: !2448, file: !2449, line: 314, type: !2487)
!2493 = !{!2494}
!2494 = !DITemplateTypeParameter(name: "T", type: !354)
!2495 = !DILocation(line: 314, column: 23, scope: !2448)
!2496 = !DILocation(line: 314, column: 33, scope: !2448)
!2497 = !DILocation(line: 323, column: 42, scope: !2448)
!2498 = !DILocation(line: 323, column: 68, scope: !2448)
!2499 = !DILocation(line: 323, column: 18, scope: !2448)
!2500 = !DILocation(line: 324, column: 6, scope: !2448)
!2501 = distinct !DISubprogram(name: "new_v1", linkageName: "_ZN4core3fmt9Arguments6new_v117hc132f62239bd6a9fE", scope: !2502, file: !2449, line: 361, type: !2560, scopeLine: 361, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !59, retainedNodes: !2562)
!2502 = !DICompositeType(tag: DW_TAG_structure_type, name: "Arguments", scope: !220, file: !6, size: 192, align: 32, elements: !2503, templateParams: !59, identifier: "9d7eac934f82b4555db0d4b64eed0240")
!2503 = !{!2504, !2510, !2554}
!2504 = !DIDerivedType(tag: DW_TAG_member, name: "pieces", scope: !2502, file: !6, baseType: !2505, size: 64, align: 32)
!2505 = !DICompositeType(tag: DW_TAG_structure_type, name: "&[&str]", file: !6, size: 64, align: 32, elements: !2506, templateParams: !59, identifier: "e5181a2ba73cefd2b9372dc5646453a9")
!2506 = !{!2507, !2509}
!2507 = !DIDerivedType(tag: DW_TAG_member, name: "data_ptr", scope: !2505, file: !6, baseType: !2508, size: 32, align: 32)
!2508 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*const &str", baseType: !2028, size: 32, align: 32, dwarfAddressSpace: 0)
!2509 = !DIDerivedType(tag: DW_TAG_member, name: "length", scope: !2505, file: !6, baseType: !56, size: 32, align: 32, offset: 32)
!2510 = !DIDerivedType(tag: DW_TAG_member, name: "fmt", scope: !2502, file: !6, baseType: !2511, size: 64, align: 32, offset: 64)
!2511 = !DICompositeType(tag: DW_TAG_structure_type, name: "Option<&[core::fmt::rt::v1::Argument]>", scope: !486, file: !6, size: 64, align: 32, elements: !2512, identifier: "18e55d9250ed7514c952542db51e9798")
!2512 = !{!2513}
!2513 = !DICompositeType(tag: DW_TAG_variant_part, scope: !486, file: !6, size: 64, align: 32, elements: !2514, templateParams: !2517, identifier: "18e55d9250ed7514c952542db51e9798_variant_part", discriminator: !1795)
!2514 = !{!2515, !2550}
!2515 = !DIDerivedType(tag: DW_TAG_member, name: "None", scope: !2513, file: !6, baseType: !2516, size: 64, align: 32, extraData: i64 0)
!2516 = !DICompositeType(tag: DW_TAG_structure_type, name: "None", scope: !2511, file: !6, size: 64, align: 32, elements: !59, templateParams: !2517, identifier: "18e55d9250ed7514c952542db51e9798::None")
!2517 = !{!2518}
!2518 = !DITemplateTypeParameter(name: "T", type: !2519)
!2519 = !DICompositeType(tag: DW_TAG_structure_type, name: "&[core::fmt::rt::v1::Argument]", file: !6, size: 64, align: 32, elements: !2520, templateParams: !59, identifier: "dd04bde91bfc769093d2698a160638d5")
!2520 = !{!2521, !2549}
!2521 = !DIDerivedType(tag: DW_TAG_member, name: "data_ptr", scope: !2519, file: !6, baseType: !2522, size: 32, align: 32)
!2522 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*const Argument", baseType: !2523, size: 32, align: 32, dwarfAddressSpace: 0)
!2523 = !DICompositeType(tag: DW_TAG_structure_type, name: "Argument", scope: !218, file: !6, size: 256, align: 32, elements: !2524, templateParams: !59, identifier: "35ff7b92c901eabaa4c28224a62e943f")
!2524 = !{!2525, !2526}
!2525 = !DIDerivedType(tag: DW_TAG_member, name: "position", scope: !2523, file: !6, baseType: !56, size: 32, align: 32)
!2526 = !DIDerivedType(tag: DW_TAG_member, name: "format", scope: !2523, file: !6, baseType: !2527, size: 224, align: 32, offset: 32)
!2527 = !DICompositeType(tag: DW_TAG_structure_type, name: "FormatSpec", scope: !218, file: !6, size: 224, align: 32, elements: !2528, templateParams: !59, identifier: "83c352b27274d89d11e9801de4698347")
!2528 = !{!2529, !2530, !2531, !2532, !2548}
!2529 = !DIDerivedType(tag: DW_TAG_member, name: "fill", scope: !2527, file: !6, baseType: !2464, size: 32, align: 32)
!2530 = !DIDerivedType(tag: DW_TAG_member, name: "align", scope: !2527, file: !6, baseType: !217, size: 8, align: 8, offset: 192)
!2531 = !DIDerivedType(tag: DW_TAG_member, name: "flags", scope: !2527, file: !6, baseType: !1179, size: 32, align: 32, offset: 32)
!2532 = !DIDerivedType(tag: DW_TAG_member, name: "precision", scope: !2527, file: !6, baseType: !2533, size: 64, align: 32, offset: 64)
!2533 = !DICompositeType(tag: DW_TAG_structure_type, name: "Count", scope: !218, file: !6, size: 64, align: 32, elements: !2534, identifier: "20ef1b97db03129d1370a0be7282ed70")
!2534 = !{!2535}
!2535 = !DICompositeType(tag: DW_TAG_variant_part, scope: !218, file: !6, size: 64, align: 32, elements: !2536, templateParams: !59, identifier: "20ef1b97db03129d1370a0be7282ed70_variant_part", discriminator: !2547)
!2536 = !{!2537, !2541, !2545}
!2537 = !DIDerivedType(tag: DW_TAG_member, name: "Is", scope: !2535, file: !6, baseType: !2538, size: 64, align: 32, extraData: i64 0)
!2538 = !DICompositeType(tag: DW_TAG_structure_type, name: "Is", scope: !2533, file: !6, size: 64, align: 32, elements: !2539, templateParams: !59, identifier: "20ef1b97db03129d1370a0be7282ed70::Is")
!2539 = !{!2540}
!2540 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !2538, file: !6, baseType: !56, size: 32, align: 32, offset: 32)
!2541 = !DIDerivedType(tag: DW_TAG_member, name: "Param", scope: !2535, file: !6, baseType: !2542, size: 64, align: 32, extraData: i64 1)
!2542 = !DICompositeType(tag: DW_TAG_structure_type, name: "Param", scope: !2533, file: !6, size: 64, align: 32, elements: !2543, templateParams: !59, identifier: "20ef1b97db03129d1370a0be7282ed70::Param")
!2543 = !{!2544}
!2544 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !2542, file: !6, baseType: !56, size: 32, align: 32, offset: 32)
!2545 = !DIDerivedType(tag: DW_TAG_member, name: "Implied", scope: !2535, file: !6, baseType: !2546, size: 64, align: 32, extraData: i64 2)
!2546 = !DICompositeType(tag: DW_TAG_structure_type, name: "Implied", scope: !2533, file: !6, size: 64, align: 32, elements: !59, templateParams: !59, identifier: "20ef1b97db03129d1370a0be7282ed70::Implied")
!2547 = !DIDerivedType(tag: DW_TAG_member, scope: !218, file: !6, baseType: !1179, size: 32, align: 32, flags: DIFlagArtificial)
!2548 = !DIDerivedType(tag: DW_TAG_member, name: "width", scope: !2527, file: !6, baseType: !2533, size: 64, align: 32, offset: 128)
!2549 = !DIDerivedType(tag: DW_TAG_member, name: "length", scope: !2519, file: !6, baseType: !56, size: 32, align: 32, offset: 32)
!2550 = !DIDerivedType(tag: DW_TAG_member, name: "Some", scope: !2513, file: !6, baseType: !2551, size: 64, align: 32)
!2551 = !DICompositeType(tag: DW_TAG_structure_type, name: "Some", scope: !2511, file: !6, size: 64, align: 32, elements: !2552, templateParams: !2517, identifier: "18e55d9250ed7514c952542db51e9798::Some")
!2552 = !{!2553}
!2553 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !2551, file: !6, baseType: !2519, size: 64, align: 32)
!2554 = !DIDerivedType(tag: DW_TAG_member, name: "args", scope: !2502, file: !6, baseType: !2555, size: 64, align: 32, offset: 128)
!2555 = !DICompositeType(tag: DW_TAG_structure_type, name: "&[core::fmt::ArgumentV1]", file: !6, size: 64, align: 32, elements: !2556, templateParams: !59, identifier: "1503b726543e69d7e5da181fda09a66f")
!2556 = !{!2557, !2559}
!2557 = !DIDerivedType(tag: DW_TAG_member, name: "data_ptr", scope: !2555, file: !6, baseType: !2558, size: 32, align: 32)
!2558 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*const ArgumentV1", baseType: !2450, size: 32, align: 32, dwarfAddressSpace: 0)
!2559 = !DIDerivedType(tag: DW_TAG_member, name: "length", scope: !2555, file: !6, baseType: !56, size: 32, align: 32, offset: 32)
!2560 = !DISubroutineType(types: !2561)
!2561 = !{!2502, !2505, !2555}
!2562 = !{!2563, !2564}
!2563 = !DILocalVariable(name: "pieces", arg: 1, scope: !2501, file: !2449, line: 361, type: !2505)
!2564 = !DILocalVariable(name: "args", arg: 2, scope: !2501, file: !2449, line: 361, type: !2555)
!2565 = !DILocation(line: 361, column: 25, scope: !2501)
!2566 = !DILocation(line: 361, column: 53, scope: !2501)
!2567 = !DILocation(line: 362, column: 12, scope: !2501)
!2568 = !DILocation(line: 362, column: 56, scope: !2501)
!2569 = !DILocation(line: 362, column: 41, scope: !2501)
!2570 = !DILocation(line: 365, column: 34, scope: !2501)
!2571 = !DILocation(line: 365, column: 9, scope: !2501)
!2572 = !DILocation(line: 366, column: 6, scope: !2501)
!2573 = !DILocation(line: 363, column: 13, scope: !2501)
!2574 = distinct !DISubprogram(name: "fmt", linkageName: "_ZN45_$LT$$LP$$RP$$u20$as$u20$core..fmt..Debug$GT$3fmt17h4cbe5cdd5d7bb3fbE", scope: !2575, file: !2449, line: 2316, type: !2576, scopeLine: 2316, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !59, retainedNodes: !2578)
!2575 = !DINamespace(name: "{impl#27}", scope: !220)
!2576 = !DISubroutineType(types: !2577)
!2577 = !{!198, !1049, !2459}
!2578 = !{!2579, !2580}
!2579 = !DILocalVariable(name: "self", arg: 1, scope: !2574, file: !2449, line: 2316, type: !1049)
!2580 = !DILocalVariable(name: "f", arg: 2, scope: !2574, file: !2449, line: 2316, type: !2459)
!2581 = !DILocation(line: 2316, column: 12, scope: !2574)
!2582 = !DILocation(line: 2316, column: 19, scope: !2574)
!2583 = !DILocation(line: 2317, column: 9, scope: !2574)
!2584 = !DILocation(line: 2318, column: 6, scope: !2574)
!2585 = distinct !DISubprogram(name: "get_unchecked<core::mem::maybe_uninit::MaybeUninit<()>>", linkageName: "_ZN75_$LT$usize$u20$as$u20$core..slice..index..SliceIndex$LT$$u5b$T$u5d$$GT$$GT$13get_unchecked17haf27b5046d34226dE", scope: !2587, file: !2586, line: 172, type: !2590, scopeLine: 172, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !1997, retainedNodes: !2592)
!2586 = !DIFile(filename: "/rustc/9d1b2106e23b1abd32fce1f17267604a5102f57a/library/core/src/slice/index.rs", directory: "", checksumkind: CSK_MD5, checksum: "67aeb488563dab65a34caf648ddcaa3b")
!2587 = !DINamespace(name: "{impl#2}", scope: !2588)
!2588 = !DINamespace(name: "index", scope: !2589)
!2589 = !DINamespace(name: "slice", scope: !12)
!2590 = !DISubroutineType(types: !2591)
!2591 = !{!1990, !56, !1991}
!2592 = !{!2593, !2594}
!2593 = !DILocalVariable(name: "self", arg: 1, scope: !2585, file: !2586, line: 172, type: !56)
!2594 = !DILocalVariable(name: "slice", arg: 2, scope: !2585, file: !2586, line: 172, type: !1991)
!2595 = !DILocation(line: 172, column: 29, scope: !2585)
!2596 = !DILocation(line: 172, column: 35, scope: !2585)
!2597 = !DILocation(line: 177, column: 18, scope: !2585)
!2598 = !DILocalVariable(name: "self", arg: 1, scope: !2599, file: !1928, line: 561, type: !1990)
!2599 = distinct !DISubprogram(name: "add<core::mem::maybe_uninit::MaybeUninit<()>>", linkageName: "_ZN4core3ptr9const_ptr33_$LT$impl$u20$$BP$const$u20$T$GT$3add17hf0baaba592fd0be4E", scope: !1929, file: !1928, line: 561, type: !2600, scopeLine: 561, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !1997, retainedNodes: !2602)
!2600 = !DISubroutineType(types: !2601)
!2601 = !{!1990, !1990, !56}
!2602 = !{!2598, !2603}
!2603 = !DILocalVariable(name: "count", arg: 2, scope: !2599, file: !1928, line: 561, type: !56)
!2604 = !DILocation(line: 561, column: 29, scope: !2599, inlinedAt: !2605)
!2605 = distinct !DILocation(line: 177, column: 18, scope: !2585)
!2606 = !DILocation(line: 561, column: 35, scope: !2599, inlinedAt: !2605)
!2607 = !DILocalVariable(name: "self", arg: 1, scope: !2608, file: !1928, line: 280, type: !1990)
!2608 = distinct !DISubprogram(name: "offset<core::mem::maybe_uninit::MaybeUninit<()>>", linkageName: "_ZN4core3ptr9const_ptr33_$LT$impl$u20$$BP$const$u20$T$GT$6offset17h65c64c9cc1423695E", scope: !1929, file: !1928, line: 280, type: !2609, scopeLine: 280, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !1997, retainedNodes: !2611)
!2609 = !DISubroutineType(types: !2610)
!2610 = !{!1990, !1990, !1388}
!2611 = !{!2607, !2612}
!2612 = !DILocalVariable(name: "count", arg: 2, scope: !2608, file: !1928, line: 280, type: !1388)
!2613 = !DILocation(line: 280, column: 32, scope: !2608, inlinedAt: !2614)
!2614 = distinct !DILocation(line: 566, column: 18, scope: !2599, inlinedAt: !2605)
!2615 = !DILocation(line: 280, column: 38, scope: !2608, inlinedAt: !2614)
!2616 = !DILocation(line: 285, column: 18, scope: !2608, inlinedAt: !2614)
!2617 = !DILocation(line: 178, column: 6, scope: !2585)
!2618 = distinct !DISubprogram(name: "get_unchecked<core::cell::UnsafeCell<core::mem::maybe_uninit::MaybeUninit<(locals::app::P1_T, u8)>>>", linkageName: "_ZN75_$LT$usize$u20$as$u20$core..slice..index..SliceIndex$LT$$u5b$T$u5d$$GT$$GT$13get_unchecked17hc7512e2bbf26379dE", scope: !2587, file: !2586, line: 172, type: !2619, scopeLine: 172, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !1982, retainedNodes: !2621)
!2619 = !DISubroutineType(types: !2620)
!2620 = !{!1975, !56, !1976}
!2621 = !{!2622, !2623}
!2622 = !DILocalVariable(name: "self", arg: 1, scope: !2618, file: !2586, line: 172, type: !56)
!2623 = !DILocalVariable(name: "slice", arg: 2, scope: !2618, file: !2586, line: 172, type: !1976)
!2624 = !DILocation(line: 172, column: 29, scope: !2618)
!2625 = !DILocation(line: 172, column: 35, scope: !2618)
!2626 = !DILocation(line: 177, column: 18, scope: !2618)
!2627 = !DILocalVariable(name: "self", arg: 1, scope: !2628, file: !1928, line: 561, type: !1975)
!2628 = distinct !DISubprogram(name: "add<core::cell::UnsafeCell<core::mem::maybe_uninit::MaybeUninit<(locals::app::P1_T, u8)>>>", linkageName: "_ZN4core3ptr9const_ptr33_$LT$impl$u20$$BP$const$u20$T$GT$3add17h6f62f06448dc8195E", scope: !1929, file: !1928, line: 561, type: !2629, scopeLine: 561, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !1982, retainedNodes: !2631)
!2629 = !DISubroutineType(types: !2630)
!2630 = !{!1975, !1975, !56}
!2631 = !{!2627, !2632}
!2632 = !DILocalVariable(name: "count", arg: 2, scope: !2628, file: !1928, line: 561, type: !56)
!2633 = !DILocation(line: 561, column: 29, scope: !2628, inlinedAt: !2634)
!2634 = distinct !DILocation(line: 177, column: 18, scope: !2618)
!2635 = !DILocation(line: 561, column: 35, scope: !2628, inlinedAt: !2634)
!2636 = !DILocalVariable(name: "self", arg: 1, scope: !2637, file: !1928, line: 280, type: !1975)
!2637 = distinct !DISubprogram(name: "offset<core::cell::UnsafeCell<core::mem::maybe_uninit::MaybeUninit<(locals::app::P1_T, u8)>>>", linkageName: "_ZN4core3ptr9const_ptr33_$LT$impl$u20$$BP$const$u20$T$GT$6offset17hd61f773f7bddff95E", scope: !1929, file: !1928, line: 280, type: !2638, scopeLine: 280, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !1982, retainedNodes: !2640)
!2638 = !DISubroutineType(types: !2639)
!2639 = !{!1975, !1975, !1388}
!2640 = !{!2636, !2641}
!2641 = !DILocalVariable(name: "count", arg: 2, scope: !2637, file: !1928, line: 280, type: !1388)
!2642 = !DILocation(line: 280, column: 32, scope: !2637, inlinedAt: !2643)
!2643 = distinct !DILocation(line: 566, column: 18, scope: !2628, inlinedAt: !2634)
!2644 = !DILocation(line: 280, column: 38, scope: !2637, inlinedAt: !2643)
!2645 = !DILocation(line: 285, column: 18, scope: !2637, inlinedAt: !2643)
!2646 = !DILocation(line: 178, column: 6, scope: !2618)
!2647 = distinct !DISubprogram(name: "get_unchecked<core::cell::UnsafeCell<core::mem::maybe_uninit::MaybeUninit<u8>>>", linkageName: "_ZN75_$LT$usize$u20$as$u20$core..slice..index..SliceIndex$LT$$u5b$T$u5d$$GT$$GT$13get_unchecked17hfaefdf0e800697d2E", scope: !2587, file: !2586, line: 172, type: !2648, scopeLine: 172, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !1967, retainedNodes: !2650)
!2648 = !DISubroutineType(types: !2649)
!2649 = !{!1960, !56, !1961}
!2650 = !{!2651, !2652}
!2651 = !DILocalVariable(name: "self", arg: 1, scope: !2647, file: !2586, line: 172, type: !56)
!2652 = !DILocalVariable(name: "slice", arg: 2, scope: !2647, file: !2586, line: 172, type: !1961)
!2653 = !DILocation(line: 172, column: 29, scope: !2647)
!2654 = !DILocation(line: 172, column: 35, scope: !2647)
!2655 = !DILocation(line: 177, column: 18, scope: !2647)
!2656 = !DILocalVariable(name: "self", arg: 1, scope: !2657, file: !1928, line: 561, type: !1960)
!2657 = distinct !DISubprogram(name: "add<core::cell::UnsafeCell<core::mem::maybe_uninit::MaybeUninit<u8>>>", linkageName: "_ZN4core3ptr9const_ptr33_$LT$impl$u20$$BP$const$u20$T$GT$3add17h0290579d7348d8bbE", scope: !1929, file: !1928, line: 561, type: !2658, scopeLine: 561, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !1967, retainedNodes: !2660)
!2658 = !DISubroutineType(types: !2659)
!2659 = !{!1960, !1960, !56}
!2660 = !{!2656, !2661}
!2661 = !DILocalVariable(name: "count", arg: 2, scope: !2657, file: !1928, line: 561, type: !56)
!2662 = !DILocation(line: 561, column: 29, scope: !2657, inlinedAt: !2663)
!2663 = distinct !DILocation(line: 177, column: 18, scope: !2647)
!2664 = !DILocation(line: 561, column: 35, scope: !2657, inlinedAt: !2663)
!2665 = !DILocalVariable(name: "self", arg: 1, scope: !2666, file: !1928, line: 280, type: !1960)
!2666 = distinct !DISubprogram(name: "offset<core::cell::UnsafeCell<core::mem::maybe_uninit::MaybeUninit<u8>>>", linkageName: "_ZN4core3ptr9const_ptr33_$LT$impl$u20$$BP$const$u20$T$GT$6offset17h198d97248187a96dE", scope: !1929, file: !1928, line: 280, type: !2667, scopeLine: 280, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !1967, retainedNodes: !2669)
!2667 = !DISubroutineType(types: !2668)
!2668 = !{!1960, !1960, !1388}
!2669 = !{!2665, !2670}
!2670 = !DILocalVariable(name: "count", arg: 2, scope: !2666, file: !1928, line: 280, type: !1388)
!2671 = !DILocation(line: 280, column: 32, scope: !2666, inlinedAt: !2672)
!2672 = distinct !DILocation(line: 566, column: 18, scope: !2657, inlinedAt: !2663)
!2673 = !DILocation(line: 280, column: 38, scope: !2666, inlinedAt: !2672)
!2674 = !DILocation(line: 285, column: 18, scope: !2666, inlinedAt: !2672)
!2675 = !DILocation(line: 178, column: 6, scope: !2647)
!2676 = distinct !DISubprogram(name: "get_unchecked_mut<core::mem::maybe_uninit::MaybeUninit<()>>", linkageName: "_ZN75_$LT$usize$u20$as$u20$core..slice..index..SliceIndex$LT$$u5b$T$u5d$$GT$$GT$17get_unchecked_mut17he3cea9bf2e959781E", scope: !2587, file: !2586, line: 181, type: !2677, scopeLine: 181, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !1997, retainedNodes: !2684)
!2677 = !DISubroutineType(types: !2678)
!2678 = !{!2679, !56, !2680}
!2679 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*mut MaybeUninit<()>", baseType: !91, size: 32, align: 32, dwarfAddressSpace: 0)
!2680 = !DICompositeType(tag: DW_TAG_structure_type, name: "*mut [core::mem::maybe_uninit::MaybeUninit<()>]", file: !6, size: 64, align: 32, elements: !2681, templateParams: !59, identifier: "b1a95118c1fd9fde45cbc8c27784cefe")
!2681 = !{!2682, !2683}
!2682 = !DIDerivedType(tag: DW_TAG_member, name: "data_ptr", scope: !2680, file: !6, baseType: !1990, size: 32, align: 32)
!2683 = !DIDerivedType(tag: DW_TAG_member, name: "length", scope: !2680, file: !6, baseType: !56, size: 32, align: 32, offset: 32)
!2684 = !{!2685, !2686}
!2685 = !DILocalVariable(name: "self", arg: 1, scope: !2676, file: !2586, line: 181, type: !56)
!2686 = !DILocalVariable(name: "slice", arg: 2, scope: !2676, file: !2586, line: 181, type: !2680)
!2687 = !DILocation(line: 181, column: 33, scope: !2676)
!2688 = !DILocation(line: 181, column: 39, scope: !2676)
!2689 = !DILocalVariable(name: "self", arg: 1, scope: !2690, file: !469, line: 1267, type: !2680)
!2690 = distinct !DISubprogram(name: "as_mut_ptr<core::mem::maybe_uninit::MaybeUninit<()>>", linkageName: "_ZN4core3ptr7mut_ptr41_$LT$impl$u20$$BP$mut$u20$$u5b$T$u5d$$GT$10as_mut_ptr17h2631c90bf86546e2E", scope: !2691, file: !469, line: 1267, type: !2692, scopeLine: 1267, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !1997, retainedNodes: !2694)
!2691 = !DINamespace(name: "{impl#1}", scope: !471)
!2692 = !DISubroutineType(types: !2693)
!2693 = !{!2679, !2680}
!2694 = !{!2689}
!2695 = !DILocation(line: 1267, column: 29, scope: !2690, inlinedAt: !2696)
!2696 = distinct !DILocation(line: 183, column: 18, scope: !2676)
!2697 = !DILocation(line: 1268, column: 9, scope: !2690, inlinedAt: !2696)
!2698 = !DILocation(line: 183, column: 18, scope: !2676)
!2699 = !DILocalVariable(name: "self", arg: 1, scope: !2700, file: !469, line: 671, type: !2679)
!2700 = distinct !DISubprogram(name: "add<core::mem::maybe_uninit::MaybeUninit<()>>", linkageName: "_ZN4core3ptr7mut_ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$3add17hf917a0ee983da8c9E", scope: !470, file: !469, line: 671, type: !2701, scopeLine: 671, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !1997, retainedNodes: !2703)
!2701 = !DISubroutineType(types: !2702)
!2702 = !{!2679, !2679, !56}
!2703 = !{!2699, !2704}
!2704 = !DILocalVariable(name: "count", arg: 2, scope: !2700, file: !469, line: 671, type: !56)
!2705 = !DILocation(line: 671, column: 29, scope: !2700, inlinedAt: !2706)
!2706 = distinct !DILocation(line: 183, column: 18, scope: !2676)
!2707 = !DILocation(line: 671, column: 35, scope: !2700, inlinedAt: !2706)
!2708 = !DILocalVariable(name: "self", arg: 1, scope: !2709, file: !469, line: 286, type: !2679)
!2709 = distinct !DISubprogram(name: "offset<core::mem::maybe_uninit::MaybeUninit<()>>", linkageName: "_ZN4core3ptr7mut_ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$6offset17he7bca523cb085f4dE", scope: !470, file: !469, line: 286, type: !2710, scopeLine: 286, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !1997, retainedNodes: !2712)
!2710 = !DISubroutineType(types: !2711)
!2711 = !{!2679, !2679, !1388}
!2712 = !{!2708, !2713}
!2713 = !DILocalVariable(name: "count", arg: 2, scope: !2709, file: !469, line: 286, type: !1388)
!2714 = !DILocation(line: 286, column: 32, scope: !2709, inlinedAt: !2715)
!2715 = distinct !DILocation(line: 676, column: 18, scope: !2700, inlinedAt: !2706)
!2716 = !DILocation(line: 286, column: 38, scope: !2709, inlinedAt: !2715)
!2717 = !DILocation(line: 293, column: 18, scope: !2709, inlinedAt: !2715)
!2718 = !DILocation(line: 184, column: 6, scope: !2676)
!2719 = distinct !DISubprogram(name: "get_unchecked<core::cell::UnsafeCell<core::mem::maybe_uninit::MaybeUninit<u8>>, usize>", linkageName: "_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$13get_unchecked17h92887aea7def6c01E", scope: !2721, file: !2720, line: 380, type: !2722, scopeLine: 380, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !2731, retainedNodes: !2728)
!2720 = !DIFile(filename: "/rustc/9d1b2106e23b1abd32fce1f17267604a5102f57a/library/core/src/slice/mod.rs", directory: "", checksumkind: CSK_MD5, checksum: "f64e3e89ab2196197386a94879bbb826")
!2721 = !DINamespace(name: "{impl#0}", scope: !2589)
!2722 = !DISubroutineType(types: !2723)
!2723 = !{!1557, !2724, !56}
!2724 = !DICompositeType(tag: DW_TAG_structure_type, name: "&[core::cell::UnsafeCell<core::mem::maybe_uninit::MaybeUninit<u8>>]", file: !6, size: 64, align: 32, elements: !2725, templateParams: !59, identifier: "d825d5768fc56771cc7d751c6bc15487")
!2725 = !{!2726, !2727}
!2726 = !DIDerivedType(tag: DW_TAG_member, name: "data_ptr", scope: !2724, file: !6, baseType: !1960, size: 32, align: 32)
!2727 = !DIDerivedType(tag: DW_TAG_member, name: "length", scope: !2724, file: !6, baseType: !56, size: 32, align: 32, offset: 32)
!2728 = !{!2729, !2730}
!2729 = !DILocalVariable(name: "self", arg: 1, scope: !2719, file: !2720, line: 380, type: !2724)
!2730 = !DILocalVariable(name: "index", arg: 2, scope: !2719, file: !2720, line: 380, type: !56)
!2731 = !{!1968, !2732}
!2732 = !DITemplateTypeParameter(name: "I", type: !56)
!2733 = !DILocation(line: 380, column: 36, scope: !2719)
!2734 = !DILocation(line: 380, column: 43, scope: !2719)
!2735 = !DILocation(line: 387, column: 20, scope: !2719)
!2736 = !DILocation(line: 388, column: 6, scope: !2719)
!2737 = distinct !DISubprogram(name: "get_unchecked<core::cell::UnsafeCell<core::mem::maybe_uninit::MaybeUninit<(locals::app::P1_T, u8)>>, usize>", linkageName: "_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$13get_unchecked17h990719cc23fef4feE", scope: !2721, file: !2720, line: 380, type: !2738, scopeLine: 380, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !2747, retainedNodes: !2744)
!2738 = !DISubroutineType(types: !2739)
!2739 = !{!1614, !2740, !56}
!2740 = !DICompositeType(tag: DW_TAG_structure_type, name: "&[core::cell::UnsafeCell<core::mem::maybe_uninit::MaybeUninit<(locals::app::P1_T, u8)>>]", file: !6, size: 64, align: 32, elements: !2741, templateParams: !59, identifier: "dddf50bbdad9a6b12986ee5238bc99ca")
!2741 = !{!2742, !2743}
!2742 = !DIDerivedType(tag: DW_TAG_member, name: "data_ptr", scope: !2740, file: !6, baseType: !1975, size: 32, align: 32)
!2743 = !DIDerivedType(tag: DW_TAG_member, name: "length", scope: !2740, file: !6, baseType: !56, size: 32, align: 32, offset: 32)
!2744 = !{!2745, !2746}
!2745 = !DILocalVariable(name: "self", arg: 1, scope: !2737, file: !2720, line: 380, type: !2740)
!2746 = !DILocalVariable(name: "index", arg: 2, scope: !2737, file: !2720, line: 380, type: !56)
!2747 = !{!1983, !2732}
!2748 = !DILocation(line: 380, column: 36, scope: !2737)
!2749 = !DILocation(line: 380, column: 43, scope: !2737)
!2750 = !DILocation(line: 387, column: 20, scope: !2737)
!2751 = !DILocation(line: 388, column: 6, scope: !2737)
!2752 = distinct !DISubprogram(name: "get_unchecked<core::mem::maybe_uninit::MaybeUninit<()>, usize>", linkageName: "_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$13get_unchecked17hd2da507ba33e3510E", scope: !2721, file: !2720, line: 380, type: !2753, scopeLine: 380, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !2762, retainedNodes: !2759)
!2753 = !DISubroutineType(types: !2754)
!2754 = !{!790, !2755, !56}
!2755 = !DICompositeType(tag: DW_TAG_structure_type, name: "&[core::mem::maybe_uninit::MaybeUninit<()>]", file: !6, size: 64, align: 32, elements: !2756, templateParams: !59, identifier: "85b79451cd40abf41cabf9be675995cf")
!2756 = !{!2757, !2758}
!2757 = !DIDerivedType(tag: DW_TAG_member, name: "data_ptr", scope: !2755, file: !6, baseType: !1990, size: 32, align: 32)
!2758 = !DIDerivedType(tag: DW_TAG_member, name: "length", scope: !2755, file: !6, baseType: !56, size: 32, align: 32, offset: 32)
!2759 = !{!2760, !2761}
!2760 = !DILocalVariable(name: "self", arg: 1, scope: !2752, file: !2720, line: 380, type: !2755)
!2761 = !DILocalVariable(name: "index", arg: 2, scope: !2752, file: !2720, line: 380, type: !56)
!2762 = !{!1998, !2732}
!2763 = !DILocation(line: 380, column: 36, scope: !2752)
!2764 = !DILocation(line: 380, column: 43, scope: !2752)
!2765 = !DILocation(line: 387, column: 20, scope: !2752)
!2766 = !DILocation(line: 388, column: 6, scope: !2752)
!2767 = distinct !DISubprogram(name: "get_unchecked_mut<core::mem::maybe_uninit::MaybeUninit<()>, usize>", linkageName: "_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$17get_unchecked_mut17hcb783b7fcc0d5eb1E", scope: !2721, file: !2720, line: 416, type: !2768, scopeLine: 416, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !2762, retainedNodes: !2774)
!2768 = !DISubroutineType(types: !2769)
!2769 = !{!462, !2770, !56}
!2770 = !DICompositeType(tag: DW_TAG_structure_type, name: "&mut [core::mem::maybe_uninit::MaybeUninit<()>]", file: !6, size: 64, align: 32, elements: !2771, templateParams: !59, identifier: "4662826c7e0c3313875cfd4bc9b82742")
!2771 = !{!2772, !2773}
!2772 = !DIDerivedType(tag: DW_TAG_member, name: "data_ptr", scope: !2770, file: !6, baseType: !1990, size: 32, align: 32)
!2773 = !DIDerivedType(tag: DW_TAG_member, name: "length", scope: !2770, file: !6, baseType: !56, size: 32, align: 32, offset: 32)
!2774 = !{!2775, !2776}
!2775 = !DILocalVariable(name: "self", arg: 1, scope: !2767, file: !2720, line: 416, type: !2770)
!2776 = !DILocalVariable(name: "index", arg: 2, scope: !2767, file: !2720, line: 416, type: !56)
!2777 = !DILocation(line: 416, column: 40, scope: !2767)
!2778 = !DILocation(line: 416, column: 51, scope: !2767)
!2779 = !DILocation(line: 423, column: 24, scope: !2767)
!2780 = !DILocation(line: 424, column: 6, scope: !2767)
!2781 = distinct !DISubprogram(name: "clone", linkageName: "_ZN4core5clone5impls51_$LT$impl$u20$core..clone..Clone$u20$for$u20$u8$GT$5clone17h0bd6b78f87b73bf1E", scope: !2783, file: !2782, line: 183, type: !2786, scopeLine: 183, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !59, retainedNodes: !2788)
!2782 = !DIFile(filename: "/rustc/9d1b2106e23b1abd32fce1f17267604a5102f57a/library/core/src/clone.rs", directory: "", checksumkind: CSK_MD5, checksum: "b6de6f12ae3fec739445f85b3cdce5ba")
!2783 = !DINamespace(name: "{impl#6}", scope: !2784)
!2784 = !DINamespace(name: "impls", scope: !2785)
!2785 = !DINamespace(name: "clone", scope: !12)
!2786 = !DISubroutineType(types: !2787)
!2787 = !{!73, !530}
!2788 = !{!2789}
!2789 = !DILocalVariable(name: "self", arg: 1, scope: !2781, file: !2782, line: 183, type: !530)
!2790 = !DILocation(line: 183, column: 30, scope: !2781)
!2791 = !DILocation(line: 184, column: 25, scope: !2781)
!2792 = !DILocation(line: 185, column: 22, scope: !2781)
!2793 = distinct !DISubprogram(name: "lt", linkageName: "_ZN4core3cmp5impls54_$LT$impl$u20$core..cmp..PartialOrd$u20$for$u20$u8$GT$2lt17h931a40755a640d7eE", scope: !2795, file: !2794, line: 1332, type: !2798, scopeLine: 1332, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !59, retainedNodes: !2800)
!2794 = !DIFile(filename: "/rustc/9d1b2106e23b1abd32fce1f17267604a5102f57a/library/core/src/cmp.rs", directory: "", checksumkind: CSK_MD5, checksum: "49b54ab07f16cc9d3a8aee05f1871a57")
!2795 = !DINamespace(name: "{impl#56}", scope: !2796)
!2796 = !DINamespace(name: "impls", scope: !2797)
!2797 = !DINamespace(name: "cmp", scope: !12)
!2798 = !DISubroutineType(types: !2799)
!2799 = !{!1359, !530, !530}
!2800 = !{!2801, !2802}
!2801 = !DILocalVariable(name: "self", arg: 1, scope: !2793, file: !2794, line: 1332, type: !530)
!2802 = !DILocalVariable(name: "other", arg: 2, scope: !2793, file: !2794, line: 1332, type: !530)
!2803 = !DILocation(line: 1332, column: 23, scope: !2793)
!2804 = !DILocation(line: 1332, column: 30, scope: !2793)
!2805 = !DILocation(line: 1332, column: 52, scope: !2793)
!2806 = !DILocation(line: 1332, column: 62, scope: !2793)
!2807 = !DILocation(line: 1332, column: 72, scope: !2793)
!2808 = distinct !DISubprogram(name: "from", linkageName: "_ZN4core7convert3num65_$LT$impl$u20$core..convert..From$LT$u8$GT$$u20$for$u20$usize$GT$4from17h5a3358a42c13f974E", scope: !2810, file: !2809, line: 54, type: !2812, scopeLine: 54, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !59, retainedNodes: !2814)
!2809 = !DIFile(filename: "/rustc/9d1b2106e23b1abd32fce1f17267604a5102f57a/library/core/src/convert/num.rs", directory: "", checksumkind: CSK_MD5, checksum: "e7ebb3584f81cf78638b4d6debc43a3b")
!2810 = !DINamespace(name: "{impl#42}", scope: !2811)
!2811 = !DINamespace(name: "num", scope: !2175)
!2812 = !DISubroutineType(types: !2813)
!2813 = !{!56, !73}
!2814 = !{!2815}
!2815 = !DILocalVariable(name: "small", arg: 1, scope: !2808, file: !2809, line: 54, type: !73)
!2816 = !DILocation(line: 54, column: 21, scope: !2808)
!2817 = !DILocation(line: 55, column: 17, scope: !2808)
!2818 = !DILocation(line: 56, column: 14, scope: !2808)
!2819 = distinct !DISubprogram(name: "from", linkageName: "_ZN4core7convert3num66_$LT$impl$u20$core..convert..From$LT$u16$GT$$u20$for$u20$usize$GT$4from17hb2e480a8ed69ba2dE", scope: !2820, file: !2809, line: 54, type: !2821, scopeLine: 54, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !59, retainedNodes: !2823)
!2820 = !DINamespace(name: "{impl#70}", scope: !2811)
!2821 = !DISubroutineType(types: !2822)
!2822 = !{!56, !558}
!2823 = !{!2824}
!2824 = !DILocalVariable(name: "small", arg: 1, scope: !2819, file: !2809, line: 54, type: !558)
!2825 = !DILocation(line: 54, column: 21, scope: !2819)
!2826 = !DILocation(line: 55, column: 17, scope: !2819)
!2827 = !DILocation(line: 56, column: 14, scope: !2819)
!2828 = distinct !DISubprogram(name: "forward_unchecked", linkageName: "_ZN46_$LT$u8$u20$as$u20$core..iter..range..Step$GT$17forward_unchecked17ha8d94eade2e46a9aE", scope: !2829, file: !2210, line: 189, type: !2830, scopeLine: 189, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !59, retainedNodes: !2832)
!2829 = !DINamespace(name: "{impl#29}", scope: !2212)
!2830 = !DISubroutineType(types: !2831)
!2831 = !{!73, !73, !56}
!2832 = !{!2833, !2834}
!2833 = !DILocalVariable(name: "start", arg: 1, scope: !2828, file: !2210, line: 189, type: !73)
!2834 = !DILocalVariable(name: "n", arg: 2, scope: !2828, file: !2210, line: 189, type: !56)
!2835 = !DILocation(line: 189, column: 37, scope: !2828)
!2836 = !DILocation(line: 189, column: 50, scope: !2828)
!2837 = !DILocation(line: 191, column: 42, scope: !2828)
!2838 = !DILocalVariable(name: "self", arg: 1, scope: !2839, file: !1446, line: 460, type: !73)
!2839 = distinct !DISubprogram(name: "unchecked_add", linkageName: "_ZN4core3num20_$LT$impl$u20$u8$GT$13unchecked_add17h4cb6963de0952c5cE", scope: !2840, file: !1446, line: 460, type: !731, scopeLine: 460, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !59, retainedNodes: !2841)
!2840 = !DINamespace(name: "{impl#6}", scope: !1448)
!2841 = !{!2838, !2842}
!2842 = !DILocalVariable(name: "rhs", arg: 2, scope: !2839, file: !1446, line: 460, type: !73)
!2843 = !DILocation(line: 460, column: 43, scope: !2839, inlinedAt: !2844)
!2844 = distinct !DILocation(line: 191, column: 22, scope: !2828)
!2845 = !DILocation(line: 460, column: 49, scope: !2839, inlinedAt: !2844)
!2846 = !DILocation(line: 463, column: 22, scope: !2839, inlinedAt: !2844)
!2847 = !DILocation(line: 191, column: 22, scope: !2828)
!2848 = !DILocation(line: 192, column: 10, scope: !2828)
!2849 = distinct !DISubprogram(name: "load", linkageName: "_ZN4core4sync6atomic11AtomicUsize4load17hbff932a919f9f519E", scope: !48, file: !2406, line: 1500, type: !2850, scopeLine: 1500, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !59, retainedNodes: !2853)
!2850 = !DISubroutineType(types: !2851)
!2851 = !{!56, !2852, !210}
!2852 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&AtomicUsize", baseType: !48, size: 32, align: 32, dwarfAddressSpace: 0)
!2853 = !{!2854, !2855}
!2854 = !DILocalVariable(name: "self", arg: 1, scope: !2849, file: !2406, line: 1500, type: !2852)
!2855 = !DILocalVariable(name: "order", arg: 2, scope: !2849, file: !2406, line: 1500, type: !210)
!2856 = !DILocation(line: 1500, column: 25, scope: !2849)
!2857 = !DILocation(line: 1500, column: 32, scope: !2849)
!2858 = !DILocation(line: 1502, column: 38, scope: !2849)
!2859 = !DILocalVariable(name: "self", arg: 1, scope: !2860, file: !449, line: 1913, type: !2863)
!2860 = distinct !DISubprogram(name: "get<usize>", linkageName: "_ZN4core4cell19UnsafeCell$LT$T$GT$3get17h51ba91e829e1cf1cE", scope: !53, file: !449, line: 1913, type: !2861, scopeLine: 1913, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !57, retainedNodes: !2864)
!2861 = !DISubroutineType(types: !2862)
!2862 = !{!2409, !2863}
!2863 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&UnsafeCell<usize>", baseType: !53, size: 32, align: 32, dwarfAddressSpace: 0)
!2864 = !{!2859}
!2865 = !DILocation(line: 1913, column: 22, scope: !2860, inlinedAt: !2866)
!2866 = distinct !DILocation(line: 1502, column: 38, scope: !2849)
!2867 = !DILocation(line: 1502, column: 26, scope: !2849)
!2868 = !DILocation(line: 1503, column: 14, scope: !2849)
!2869 = distinct !DISubprogram(name: "store", linkageName: "_ZN4core4sync6atomic11AtomicUsize5store17h7e08c04f0f3a30f3E", scope: !48, file: !2406, line: 1526, type: !2870, scopeLine: 1526, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !59, retainedNodes: !2872)
!2870 = !DISubroutineType(types: !2871)
!2871 = !{null, !2852, !56, !210}
!2872 = !{!2873, !2874, !2875}
!2873 = !DILocalVariable(name: "self", arg: 1, scope: !2869, file: !2406, line: 1526, type: !2852)
!2874 = !DILocalVariable(name: "val", arg: 2, scope: !2869, file: !2406, line: 1526, type: !56)
!2875 = !DILocalVariable(name: "order", arg: 3, scope: !2869, file: !2406, line: 1526, type: !210)
!2876 = !DILocation(line: 1526, column: 26, scope: !2869)
!2877 = !DILocation(line: 1526, column: 33, scope: !2869)
!2878 = !DILocation(line: 1526, column: 49, scope: !2869)
!2879 = !DILocation(line: 1528, column: 39, scope: !2869)
!2880 = !DILocation(line: 1913, column: 22, scope: !2860, inlinedAt: !2881)
!2881 = distinct !DILocation(line: 1528, column: 39, scope: !2869)
!2882 = !DILocation(line: 1528, column: 26, scope: !2869)
!2883 = !DILocation(line: 1529, column: 14, scope: !2869)
!2884 = distinct !DISubprogram(name: "fmt<i64>", linkageName: "_ZN52_$LT$$RF$mut$u20$T$u20$as$u20$core..fmt..Display$GT$3fmt17h50bf9aa690e1dd41E", scope: !2885, file: !2449, line: 2110, type: !2488, scopeLine: 2110, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !154, templateParams: !27, retainedNodes: !2886)
!2885 = !DINamespace(name: "{impl#55}", scope: !220)
!2886 = !{!2887, !2888}
!2887 = !DILocalVariable(name: "self", arg: 1, scope: !2884, file: !2449, line: 2110, type: !366)
!2888 = !DILocalVariable(name: "f", arg: 2, scope: !2884, file: !2449, line: 2110, type: !2459)
!2889 = !DILocation(line: 2110, column: 20, scope: !2884)
!2890 = !DILocation(line: 2110, column: 27, scope: !2884)
!2891 = !DILocation(line: 2110, column: 71, scope: !2884)
!2892 = !DILocation(line: 2110, column: 62, scope: !2884)
!2893 = !DILocation(line: 2110, column: 84, scope: !2884)
