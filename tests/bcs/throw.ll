; ModuleID = 'throw.cpp'
source_filename = "throw.cpp"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-linux-gnu"

@.str = private unnamed_addr constant [19 x i8] c"throwing a message\00", align 1
@_ZTIPKc = external constant i8*

; Function Attrs: mustprogress noinline optnone sspstrong uwtable
define dso_local i32 @_Z6ithrowv() #0 {
  %1 = call i8* @__cxa_allocate_exception(i64 8) #3
  %2 = bitcast i8* %1 to i8**
  store i8* getelementptr inbounds ([19 x i8], [19 x i8]* @.str, i64 0, i64 0), i8** %2, align 16
  call void @__cxa_throw(i8* %1, i8* bitcast (i8** @_ZTIPKc to i8*), i8* null) #4
  unreachable
}

declare i8* @__cxa_allocate_exception(i64)

declare void @__cxa_throw(i8*, i8*, i8*)

; Function Attrs: mustprogress noinline optnone sspstrong uwtable
define dso_local i8* @_Z3foov() #0 personality i8* bitcast (i32 (...)* @__gxx_personality_v0 to i8*) {
  %1 = alloca i8*, align 8
  %2 = alloca i8*, align 8
  %3 = alloca i32, align 4
  %4 = alloca i8*, align 8
  %5 = invoke i32 @_Z6ithrowv()
          to label %6 unwind label %7

6:                                                ; preds = %0
  br label %19

7:                                                ; preds = %0
  %8 = landingpad { i8*, i32 }
          catch i8* bitcast (i8** @_ZTIPKc to i8*)
  %9 = extractvalue { i8*, i32 } %8, 0
  store i8* %9, i8** %2, align 8
  %10 = extractvalue { i8*, i32 } %8, 1
  store i32 %10, i32* %3, align 4
  br label %11

11:                                               ; preds = %7
  %12 = load i32, i32* %3, align 4
  %13 = call i32 @llvm.eh.typeid.for(i8* bitcast (i8** @_ZTIPKc to i8*)) #3
  %14 = icmp eq i32 %12, %13
  br i1 %14, label %15, label %22

15:                                               ; preds = %11
  %16 = load i8*, i8** %2, align 8
  %17 = call i8* @__cxa_begin_catch(i8* %16) #3
  store i8* %17, i8** %4, align 8
  %18 = load i8*, i8** %4, align 8
  store i8* %18, i8** %1, align 8
  call void @__cxa_end_catch() #3
  br label %20

19:                                               ; preds = %6
  store i8* null, i8** %1, align 8
  br label %20

20:                                               ; preds = %19, %15
  %21 = load i8*, i8** %1, align 8
  ret i8* %21

22:                                               ; preds = %11
  %23 = load i8*, i8** %2, align 8
  %24 = load i32, i32* %3, align 4
  %25 = insertvalue { i8*, i32 } undef, i8* %23, 0
  %26 = insertvalue { i8*, i32 } %25, i32 %24, 1
  resume { i8*, i32 } %26
}

declare i32 @__gxx_personality_v0(...)

; Function Attrs: nounwind readnone
declare i32 @llvm.eh.typeid.for(i8*) #1

declare i8* @__cxa_begin_catch(i8*)

declare void @__cxa_end_catch()

; Function Attrs: mustprogress noinline norecurse optnone sspstrong uwtable
define dso_local i32 @main() #2 {
  %1 = call i8* @_Z3foov()
  ret i32 0
}

attributes #0 = { mustprogress noinline optnone sspstrong uwtable "frame-pointer"="all" "min-legal-vector-width"="0" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #1 = { nounwind readnone }
attributes #2 = { mustprogress noinline norecurse optnone sspstrong uwtable "frame-pointer"="all" "min-legal-vector-width"="0" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #3 = { nounwind }
attributes #4 = { noreturn }

!llvm.module.flags = !{!0, !1, !2, !3, !4}
!llvm.ident = !{!5}

!0 = !{i32 1, !"wchar_size", i32 4}
!1 = !{i32 7, !"PIC Level", i32 2}
!2 = !{i32 7, !"PIE Level", i32 2}
!3 = !{i32 7, !"uwtable", i32 1}
!4 = !{i32 7, !"frame-pointer", i32 2}
!5 = !{!"clang version 13.0.1"}
