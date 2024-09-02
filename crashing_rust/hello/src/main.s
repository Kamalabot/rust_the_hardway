	.text
	.file	"main.f62b31d3d4361a55-cgu.0"
	.section	.text._ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h78cbd944dc3be38fE,"ax",@progbits
	.p2align	4, 0x90
	.type	_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h78cbd944dc3be38fE,@function
_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h78cbd944dc3be38fE:
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
	callq	_ZN4core3ops8function6FnOnce9call_once17hca833e7eb12ae0d6E
	#APP
	#NO_APP
	popq	%rax
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end0:
	.size	_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h78cbd944dc3be38fE, .Lfunc_end0-_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h78cbd944dc3be38fE
	.cfi_endproc

	.section	.text._ZN3std2rt10lang_start17h0798d0888f60b55bE,"ax",@progbits
	.hidden	_ZN3std2rt10lang_start17h0798d0888f60b55bE
	.globl	_ZN3std2rt10lang_start17h0798d0888f60b55bE
	.p2align	4, 0x90
	.type	_ZN3std2rt10lang_start17h0798d0888f60b55bE,@function
_ZN3std2rt10lang_start17h0798d0888f60b55bE:
	.cfi_startproc
	subq	$24, %rsp
	.cfi_def_cfa_offset 32
	movl	%ecx, %eax
	movq	%rdx, %rcx
	movq	%rsi, %rdx
	movq	%rdi, 16(%rsp)
	leaq	16(%rsp), %rdi
	leaq	.L__unnamed_1(%rip), %rsi
	movzbl	%al, %r8d
	callq	*_ZN3std2rt19lang_start_internal17hcee5ed89fc25829aE@GOTPCREL(%rip)
	movq	%rax, 8(%rsp)
	movq	8(%rsp), %rax
	addq	$24, %rsp
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end1:
	.size	_ZN3std2rt10lang_start17h0798d0888f60b55bE, .Lfunc_end1-_ZN3std2rt10lang_start17h0798d0888f60b55bE
	.cfi_endproc

	.section	".text._ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hf0015d10eda7b76eE","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hf0015d10eda7b76eE,@function
_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hf0015d10eda7b76eE:
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
	movq	(%rdi), %rdi
	callq	_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h78cbd944dc3be38fE
	callq	_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17h56d3ebc4c53902b9E
	movb	%al, 7(%rsp)
	movzbl	7(%rsp), %eax
	popq	%rcx
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end2:
	.size	_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hf0015d10eda7b76eE, .Lfunc_end2-_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hf0015d10eda7b76eE
	.cfi_endproc

	.section	.text._ZN4core3fmt9Arguments9new_const17hdf28aa01378c372eE,"ax",@progbits
	.p2align	4, 0x90
	.type	_ZN4core3fmt9Arguments9new_const17hdf28aa01378c372eE,@function
_ZN4core3fmt9Arguments9new_const17hdf28aa01378c372eE:
	.cfi_startproc
	movq	%rdi, %rax
	movq	%rsi, (%rdi)
	movq	$1, 8(%rdi)
	movq	.L__unnamed_2(%rip), %rdx
	movq	.L__unnamed_2+8(%rip), %rcx
	movq	%rdx, 32(%rdi)
	movq	%rcx, 40(%rdi)
	movl	$8, %ecx
	movq	%rcx, 16(%rdi)
	movq	$0, 24(%rdi)
	retq
.Lfunc_end3:
	.size	_ZN4core3fmt9Arguments9new_const17hdf28aa01378c372eE, .Lfunc_end3-_ZN4core3fmt9Arguments9new_const17hdf28aa01378c372eE
	.cfi_endproc

	.section	".text._ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h77c66b055072aabfE","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h77c66b055072aabfE,@function
_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h77c66b055072aabfE:
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
	movq	(%rdi), %rdi
	callq	_ZN4core3ops8function6FnOnce9call_once17h859ea9ca125d47d7E
	popq	%rcx
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end4:
	.size	_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h77c66b055072aabfE, .Lfunc_end4-_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h77c66b055072aabfE
	.cfi_endproc

	.section	.text._ZN4core3ops8function6FnOnce9call_once17h859ea9ca125d47d7E,"ax",@progbits
	.p2align	4, 0x90
	.type	_ZN4core3ops8function6FnOnce9call_once17h859ea9ca125d47d7E,@function
_ZN4core3ops8function6FnOnce9call_once17h859ea9ca125d47d7E:
.Lfunc_begin0:
	.cfi_startproc
	.cfi_personality 155, DW.ref.rust_eh_personality
	.cfi_lsda 27, .Lexception0
	subq	$40, %rsp
	.cfi_def_cfa_offset 48
	movq	%rdi, 8(%rsp)
.Ltmp0:
	leaq	8(%rsp), %rdi
	callq	_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hf0015d10eda7b76eE
.Ltmp1:
	movl	%eax, 4(%rsp)
	jmp	.LBB5_3
.LBB5_1:
	movq	24(%rsp), %rdi
	callq	_Unwind_Resume@PLT
.LBB5_2:
.Ltmp2:
	movq	%rax, %rcx
	movl	%edx, %eax
	movq	%rcx, 24(%rsp)
	movl	%eax, 32(%rsp)
	jmp	.LBB5_1
.LBB5_3:
	movl	4(%rsp), %eax
	addq	$40, %rsp
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end5:
	.size	_ZN4core3ops8function6FnOnce9call_once17h859ea9ca125d47d7E, .Lfunc_end5-_ZN4core3ops8function6FnOnce9call_once17h859ea9ca125d47d7E
	.cfi_endproc
	.section	.gcc_except_table._ZN4core3ops8function6FnOnce9call_once17h859ea9ca125d47d7E,"a",@progbits
	.p2align	2, 0x0
GCC_except_table5:
.Lexception0:
	.byte	255
	.byte	255
	.byte	1
	.uleb128 .Lcst_end0-.Lcst_begin0
.Lcst_begin0:
	.uleb128 .Ltmp0-.Lfunc_begin0
	.uleb128 .Ltmp1-.Ltmp0
	.uleb128 .Ltmp2-.Lfunc_begin0
	.byte	0
	.uleb128 .Ltmp1-.Lfunc_begin0
	.uleb128 .Lfunc_end5-.Ltmp1
	.byte	0
	.byte	0
.Lcst_end0:
	.p2align	2, 0x0

	.section	.text._ZN4core3ops8function6FnOnce9call_once17hca833e7eb12ae0d6E,"ax",@progbits
	.p2align	4, 0x90
	.type	_ZN4core3ops8function6FnOnce9call_once17hca833e7eb12ae0d6E,@function
_ZN4core3ops8function6FnOnce9call_once17hca833e7eb12ae0d6E:
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
	callq	*%rdi
	popq	%rax
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end6:
	.size	_ZN4core3ops8function6FnOnce9call_once17hca833e7eb12ae0d6E, .Lfunc_end6-_ZN4core3ops8function6FnOnce9call_once17hca833e7eb12ae0d6E
	.cfi_endproc

	.section	".text._ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17he88c693f386887ecE","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17he88c693f386887ecE,@function
_ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17he88c693f386887ecE:
	.cfi_startproc
	retq
.Lfunc_end7:
	.size	_ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17he88c693f386887ecE, .Lfunc_end7-_ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17he88c693f386887ecE
	.cfi_endproc

	.section	".text._ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17h56d3ebc4c53902b9E","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17h56d3ebc4c53902b9E,@function
_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17h56d3ebc4c53902b9E:
	.cfi_startproc
	xorl	%eax, %eax
	retq
.Lfunc_end8:
	.size	_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17h56d3ebc4c53902b9E, .Lfunc_end8-_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17h56d3ebc4c53902b9E
	.cfi_endproc

	.section	.text._ZN4main4main17ha7bc4b365d06755fE,"ax",@progbits
	.p2align	4, 0x90
	.type	_ZN4main4main17ha7bc4b365d06755fE,@function
_ZN4main4main17ha7bc4b365d06755fE:
	.cfi_startproc
	subq	$56, %rsp
	.cfi_def_cfa_offset 64
	leaq	8(%rsp), %rdi
	leaq	.L__unnamed_3(%rip), %rsi
	callq	_ZN4core3fmt9Arguments9new_const17hdf28aa01378c372eE
	leaq	8(%rsp), %rdi
	callq	*_ZN3std2io5stdio6_print17h1a47f568e96855e1E@GOTPCREL(%rip)
	addq	$56, %rsp
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end9:
	.size	_ZN4main4main17ha7bc4b365d06755fE, .Lfunc_end9-_ZN4main4main17ha7bc4b365d06755fE
	.cfi_endproc

	.section	.text.main,"ax",@progbits
	.globl	main
	.p2align	4, 0x90
	.type	main,@function
main:
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
	movq	%rsi, %rdx
	movslq	%edi, %rsi
	leaq	_ZN4main4main17ha7bc4b365d06755fE(%rip), %rdi
	xorl	%ecx, %ecx
	callq	_ZN3std2rt10lang_start17h0798d0888f60b55bE
	popq	%rcx
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end10:
	.size	main, .Lfunc_end10-main
	.cfi_endproc

	.type	.L__unnamed_1,@object
	.section	.data.rel.ro..L__unnamed_1,"aw",@progbits
	.p2align	3, 0x0
.L__unnamed_1:
	.asciz	"\000\000\000\000\000\000\000\000\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h77c66b055072aabfE
	.quad	_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hf0015d10eda7b76eE
	.quad	_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hf0015d10eda7b76eE
	.size	.L__unnamed_1, 48

	.type	.L__unnamed_2,@object
	.section	.rodata.cst16,"aM",@progbits,16
	.p2align	3, 0x0
.L__unnamed_2:
	.zero	8
	.zero	8
	.size	.L__unnamed_2, 16

	.type	.L__unnamed_4,@object
	.section	.rodata..L__unnamed_4,"a",@progbits
.L__unnamed_4:
	.ascii	"Hello, world!\n"
	.size	.L__unnamed_4, 14

	.type	.L__unnamed_3,@object
	.section	.data.rel.ro..L__unnamed_3,"aw",@progbits
	.p2align	3, 0x0
.L__unnamed_3:
	.quad	.L__unnamed_4
	.asciz	"\016\000\000\000\000\000\000"
	.size	.L__unnamed_3, 16

	.hidden	DW.ref.rust_eh_personality
	.weak	DW.ref.rust_eh_personality
	.section	.data.DW.ref.rust_eh_personality,"awG",@progbits,DW.ref.rust_eh_personality,comdat
	.p2align	3, 0x0
	.type	DW.ref.rust_eh_personality,@object
	.size	DW.ref.rust_eh_personality, 8
DW.ref.rust_eh_personality:
	.quad	rust_eh_personality
	.ident	"rustc version 1.80.1 (3f5fd8dd4 2024-08-06)"
	.section	".note.GNU-stack","",@progbits
