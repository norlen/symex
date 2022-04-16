; ModuleID = './samples/simple/main.c'
source_filename = "./samples/simple/main.c"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-linux-gnu"

@__const.main.data = private unnamed_addr constant [5 x i32] [i32 0, i32 1, i32 2, i32 3, i32 4], align 16

; Function Attrs: noinline nounwind optnone sspstrong uwtable
define dso_local i32 @foo(i32* %0, i32 %1) #0 {
  %3 = alloca i32, align 4
  %4 = alloca i32*, align 8
  %5 = alloca i32, align 4
  store i32* %0, i32** %4, align 8
  store i32 %1, i32* %5, align 4
  %6 = load i32*, i32** %4, align 8
  %7 = load i32, i32* %5, align 4
  %8 = sext i32 %7 to i64
  %9 = getelementptr inbounds i32, i32* %6, i64 %8
  %10 = load i32, i32* %9, align 4
  %11 = icmp eq i32 %10, 5
  br i1 %11, label %12, label %19

12:                                               ; preds = %2
  %13 = load i32*, i32** %4, align 8
  %14 = load i32, i32* %5, align 4
  %15 = sext i32 %14 to i64
  %16 = getelementptr inbounds i32, i32* %13, i64 %15
  %17 = load i32, i32* %16, align 4
  %18 = add nsw i32 %17, 5
  store i32 %18, i32* %3, align 4
  br label %27

19:                                               ; preds = %2
  %20 = load i32*, i32** %4, align 8
  %21 = load i32, i32* %5, align 4
  %22 = add nsw i32 %21, 2
  %23 = sext i32 %22 to i64
  %24 = getelementptr inbounds i32, i32* %20, i64 %23
  %25 = load i32, i32* %24, align 4
  %26 = add nsw i32 %25, 3
  store i32 %26, i32* %3, align 4
  br label %27

27:                                               ; preds = %19, %12
  %28 = load i32, i32* %3, align 4
  ret i32 %28
}

; Function Attrs: noinline nounwind optnone sspstrong uwtable
define dso_local i32 @main() #0 {
  %1 = alloca i32, align 4
  %2 = alloca i32, align 4
  %3 = alloca i32, align 4
  %4 = alloca [5 x i32], align 16
  %5 = alloca i32, align 4
  %6 = alloca i32, align 4
  store i32 0, i32* %1, align 4
  store i32 25, i32* %2, align 4
  store i32 7, i32* %3, align 4
  %7 = bitcast [5 x i32]* %4 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 16 %7, i8* align 16 bitcast ([5 x i32]* @__const.main.data to i8*), i64 20, i1 false)
  %8 = getelementptr inbounds [5 x i32], [5 x i32]* %4, i64 0, i64 0
  %9 = call i32 @foo(i32* %8, i32 2)
  store i32 %9, i32* %5, align 4
  %10 = load i32, i32* %2, align 4
  switch i32 %10, label %14 [
    i32 3, label %11
    i32 4, label %12
    i32 5, label %13
  ]

11:                                               ; preds = %0
  br label %14

12:                                               ; preds = %0
  br label %14

13:                                               ; preds = %0
  store i32 7, i32* %2, align 4
  br label %14

14:                                               ; preds = %0, %13, %12, %11
  ret i32 0
}

; Function Attrs: argmemonly nofree nounwind willreturn
declare void @llvm.memcpy.p0i8.p0i8.i64(i8* noalias nocapture writeonly, i8* noalias nocapture readonly, i64, i1 immarg) #1

attributes #0 = { noinline nounwind optnone sspstrong uwtable "frame-pointer"="all" "min-legal-vector-width"="0" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #1 = { argmemonly nofree nounwind willreturn }

!llvm.module.flags = !{!0, !1, !2, !3, !4}
!llvm.ident = !{!5}

!0 = !{i32 1, !"wchar_size", i32 4}
!1 = !{i32 7, !"PIC Level", i32 2}
!2 = !{i32 7, !"PIE Level", i32 2}
!3 = !{i32 7, !"uwtable", i32 1}
!4 = !{i32 7, !"frame-pointer", i32 2}
!5 = !{!"clang version 13.0.1"}
