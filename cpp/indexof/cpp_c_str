	.section	__TEXT,__text,regular,pure_instructions
	.macosx_version_min 10, 13
	.globl	__Z10read_linesPKc      ## -- Begin function _Z10read_linesPKc
	.p2align	4, 0x90
__Z10read_linesPKc:                     ## @_Z10read_linesPKc
Lfunc_begin0:
	.cfi_startproc
	.cfi_personality 155, ___gxx_personality_v0
	.cfi_lsda 16, Lexception0
## BB#0:
	pushq	%rbp
Lcfi0:
	.cfi_def_cfa_offset 16
Lcfi1:
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
Lcfi2:
	.cfi_def_cfa_register %rbp
	pushq	%r15
	pushq	%r14
	pushq	%r13
	pushq	%r12
	pushq	%rbx
	subq	$648, %rsp              ## imm = 0x288
Lcfi3:
	.cfi_offset %rbx, -56
Lcfi4:
	.cfi_offset %r12, -48
Lcfi5:
	.cfi_offset %r13, -40
Lcfi6:
	.cfi_offset %r14, -32
Lcfi7:
	.cfi_offset %r15, -24
	movq	%rsi, %r15
	movq	%rdi, %r13
	movq	___stack_chk_guard@GOTPCREL(%rip), %rax
	movq	(%rax), %rax
	movq	%rax, -48(%rbp)
	movq	$0, 16(%r13)
	movq	$0, 8(%r13)
	movq	$0, (%r13)
	leaq	-200(%rbp), %r14
	leaq	-608(%rbp), %r12
	movq	__ZTCNSt3__114basic_ifstreamIcNS_11char_traitsIcEEEE0_NS_13basic_istreamIcS2_EE@GOTPCREL(%rip), %rax
	leaq	64(%rax), %rcx
	movq	%rcx, -200(%rbp)
	addq	$24, %rax
	movq	%rax, %xmm0
	movdqa	%xmm0, -624(%rbp)
Ltmp0:
	movq	%r14, %rdi
	movq	%r12, %rsi
	callq	__ZNSt3__18ios_base4initEPv
Ltmp1:
## BB#1:
	movq	$0, -64(%rbp)
	movl	$-1, -56(%rbp)
	movq	__ZTVNSt3__114basic_ifstreamIcNS_11char_traitsIcEEEE@GOTPCREL(%rip), %rbx
	leaq	24(%rbx), %rax
	movq	%rax, -672(%rbp)        ## 8-byte Spill
	movq	%rax, -624(%rbp)
	addq	$64, %rbx
	movq	%rbx, -200(%rbp)
Ltmp3:
	movq	%r12, %rdi
	callq	__ZNSt3__113basic_filebufIcNS_11char_traitsIcEEEC2Ev
Ltmp4:
## BB#2:
	cmpq	$0, -488(%rbp)
	je	LBB0_3
LBB0_9:
	movq	-624(%rbp), %rax
	movq	-24(%rax), %rax
	leaq	-624(%rbp,%rax), %rdi
	movl	-592(%rbp,%rax), %esi
	orl	$4, %esi
Ltmp8:
	callq	__ZNSt3__18ios_base5clearEj
Ltmp9:
	jmp	LBB0_10
LBB0_3:
Ltmp6:
	leaq	L_.str.5(%rip), %rsi
	movq	%r15, %rdi
	callq	_fopen
Ltmp7:
## BB#4:
	movq	%rax, -488(%rbp)
	testq	%rax, %rax
	je	LBB0_9
## BB#5:
	movl	$8, -216(%rbp)
LBB0_10:
	movq	%rbx, -664(%rbp)        ## 8-byte Spill
	pxor	%xmm0, %xmm0
	movdqa	%xmm0, -656(%rbp)
	movq	$0, -640(%rbp)
	leaq	-624(%rbp), %r12
	leaq	-680(%rbp), %rbx
	movq	__ZNSt3__15ctypeIcE2idE@GOTPCREL(%rip), %r14
	jmp	LBB0_11
	.p2align	4, 0x90
LBB0_24:                                ##   in Loop: Header=BB0_11 Depth=1
	movq	%rax, (%rcx)
	addq	$8, 8(%r13)
LBB0_11:                                ## =>This Inner Loop Header: Depth=1
	movq	-624(%rbp), %rax
	movq	-24(%rax), %rsi
	addq	%r12, %rsi
Ltmp11:
	movq	%rbx, %rdi
	callq	__ZNKSt3__18ios_base6getlocEv
Ltmp12:
## BB#12:                               ##   in Loop: Header=BB0_11 Depth=1
Ltmp13:
	movq	%rbx, %rdi
	movq	%r14, %rsi
	callq	__ZNKSt3__16locale9use_facetERNS0_2idE
Ltmp14:
## BB#13:                               ##   in Loop: Header=BB0_11 Depth=1
	movq	(%rax), %rcx
Ltmp15:
	movl	$10, %esi
	movq	%rax, %rdi
	callq	*56(%rcx)
	movl	%eax, %r15d
Ltmp16:
## BB#14:                               ##   in Loop: Header=BB0_11 Depth=1
	movq	%rbx, %rdi
	callq	__ZNSt3__16localeD1Ev
Ltmp18:
	movsbl	%r15b, %edx
	movq	%r12, %rdi
	leaq	-656(%rbp), %rsi
	callq	__ZNSt3__17getlineIcNS_11char_traitsIcEENS_9allocatorIcEEEERNS_13basic_istreamIT_T0_EES9_RNS_12basic_stringIS6_S7_T1_EES6_
Ltmp19:
## BB#15:                               ##   in Loop: Header=BB0_11 Depth=1
	movq	(%rax), %rcx
	movq	-24(%rcx), %rcx
	testb	$5, 32(%rax,%rcx)
	jne	LBB0_16
## BB#23:                               ##   in Loop: Header=BB0_11 Depth=1
	testb	$1, -656(%rbp)
	movq	-640(%rbp), %rdi
	leaq	-655(%rbp), %rax
	cmoveq	%rax, %rdi
	callq	_strdup
	movq	%rax, -680(%rbp)
	movq	8(%r13), %rcx
	cmpq	16(%r13), %rcx
	jb	LBB0_24
## BB#25:                               ##   in Loop: Header=BB0_11 Depth=1
Ltmp21:
	movq	%r13, %rdi
	movq	%rbx, %rsi
	callq	__ZNSt3__16vectorIPKcNS_9allocatorIS2_EEE21__push_back_slow_pathIS2_EEvOT_
Ltmp22:
	jmp	LBB0_11
LBB0_16:
	testb	$1, -656(%rbp)
	je	LBB0_18
## BB#17:
	movq	-640(%rbp), %rdi
	callq	__ZdlPv
LBB0_18:
	movq	-672(%rbp), %rax        ## 8-byte Reload
	movq	%rax, -624(%rbp)
	movq	-664(%rbp), %rax        ## 8-byte Reload
	movq	%rax, -200(%rbp)
	leaq	-608(%rbp), %rdi
	callq	__ZNSt3__113basic_filebufIcNS_11char_traitsIcEEED2Ev
	movq	__ZTTNSt3__114basic_ifstreamIcNS_11char_traitsIcEEEE@GOTPCREL(%rip), %rsi
	addq	$8, %rsi
	leaq	-624(%rbp), %rdi
	callq	__ZNSt3__113basic_istreamIcNS_11char_traitsIcEEED2Ev
	leaq	-200(%rbp), %rdi
	callq	__ZNSt3__19basic_iosIcNS_11char_traitsIcEEED2Ev
	movq	___stack_chk_guard@GOTPCREL(%rip), %rax
	movq	(%rax), %rax
	cmpq	-48(%rbp), %rax
	jne	LBB0_35
## BB#19:
	movq	%r13, %rax
	addq	$648, %rsp              ## imm = 0x288
	popq	%rbx
	popq	%r12
	popq	%r13
	popq	%r14
	popq	%r15
	popq	%rbp
	retq
LBB0_35:
	callq	___stack_chk_fail
LBB0_6:
Ltmp5:
	movq	%rax, %r12
	jmp	LBB0_8
LBB0_20:
Ltmp2:
	movq	%rax, %r12
	movq	%r14, %rdi
	jmp	LBB0_31
LBB0_7:
Ltmp10:
	movq	%r12, %rdi
	movq	%rax, %r12
	callq	__ZNSt3__113basic_filebufIcNS_11char_traitsIcEEED2Ev
LBB0_8:
	movq	__ZTTNSt3__114basic_ifstreamIcNS_11char_traitsIcEEEE@GOTPCREL(%rip), %rsi
	addq	$8, %rsi
	leaq	-624(%rbp), %rdi
	callq	__ZNSt3__113basic_istreamIcNS_11char_traitsIcEEED2Ev
	movq	%r14, %rdi
	jmp	LBB0_31
LBB0_26:
Ltmp23:
	jmp	LBB0_27
LBB0_34:
Ltmp17:
	movq	%rax, %r12
	leaq	-680(%rbp), %rdi
	callq	__ZNSt3__16localeD1Ev
	leaq	-200(%rbp), %rbx
	testb	$1, -656(%rbp)
	jne	LBB0_29
	jmp	LBB0_30
LBB0_22:
Ltmp20:
LBB0_27:
	movq	%rax, %r12
	leaq	-200(%rbp), %rbx
	testb	$1, -656(%rbp)
	je	LBB0_30
LBB0_29:
	movq	-640(%rbp), %rdi
	callq	__ZdlPv
LBB0_30:
	movq	-672(%rbp), %rax        ## 8-byte Reload
	movq	%rax, -624(%rbp)
	movq	-664(%rbp), %rax        ## 8-byte Reload
	movq	%rax, -200(%rbp)
	leaq	-608(%rbp), %rdi
	callq	__ZNSt3__113basic_filebufIcNS_11char_traitsIcEEED2Ev
	movq	__ZTTNSt3__114basic_ifstreamIcNS_11char_traitsIcEEEE@GOTPCREL(%rip), %rsi
	addq	$8, %rsi
	leaq	-624(%rbp), %rdi
	callq	__ZNSt3__113basic_istreamIcNS_11char_traitsIcEEED2Ev
	movq	%rbx, %rdi
LBB0_31:
	callq	__ZNSt3__19basic_iosIcNS_11char_traitsIcEEED2Ev
	movq	(%r13), %rdi
	testq	%rdi, %rdi
	je	LBB0_33
## BB#32:
	movq	%rdi, 8(%r13)
	callq	__ZdlPv
LBB0_33:
	movq	%r12, %rdi
	callq	__Unwind_Resume
Lfunc_end0:
	.cfi_endproc
	.section	__TEXT,__gcc_except_tab
	.p2align	2
GCC_except_table0:
Lexception0:
	.byte	255                     ## @LPStart Encoding = omit
	.byte	155                     ## @TType Encoding = indirect pcrel sdata4
	.asciz	"\352\200\200"          ## @TType base offset
	.byte	3                       ## Call site Encoding = udata4
	.byte	104                     ## Call site table length
Lset0 = Ltmp0-Lfunc_begin0              ## >> Call Site 1 <<
	.long	Lset0
Lset1 = Ltmp1-Ltmp0                     ##   Call between Ltmp0 and Ltmp1
	.long	Lset1
Lset2 = Ltmp2-Lfunc_begin0              ##     jumps to Ltmp2
	.long	Lset2
	.byte	0                       ##   On action: cleanup
Lset3 = Ltmp3-Lfunc_begin0              ## >> Call Site 2 <<
	.long	Lset3
Lset4 = Ltmp4-Ltmp3                     ##   Call between Ltmp3 and Ltmp4
	.long	Lset4
Lset5 = Ltmp5-Lfunc_begin0              ##     jumps to Ltmp5
	.long	Lset5
	.byte	0                       ##   On action: cleanup
Lset6 = Ltmp8-Lfunc_begin0              ## >> Call Site 3 <<
	.long	Lset6
Lset7 = Ltmp7-Ltmp8                     ##   Call between Ltmp8 and Ltmp7
	.long	Lset7
Lset8 = Ltmp10-Lfunc_begin0             ##     jumps to Ltmp10
	.long	Lset8
	.byte	0                       ##   On action: cleanup
Lset9 = Ltmp11-Lfunc_begin0             ## >> Call Site 4 <<
	.long	Lset9
Lset10 = Ltmp12-Ltmp11                  ##   Call between Ltmp11 and Ltmp12
	.long	Lset10
Lset11 = Ltmp20-Lfunc_begin0            ##     jumps to Ltmp20
	.long	Lset11
	.byte	0                       ##   On action: cleanup
Lset12 = Ltmp13-Lfunc_begin0            ## >> Call Site 5 <<
	.long	Lset12
Lset13 = Ltmp16-Ltmp13                  ##   Call between Ltmp13 and Ltmp16
	.long	Lset13
Lset14 = Ltmp17-Lfunc_begin0            ##     jumps to Ltmp17
	.long	Lset14
	.byte	0                       ##   On action: cleanup
Lset15 = Ltmp18-Lfunc_begin0            ## >> Call Site 6 <<
	.long	Lset15
Lset16 = Ltmp19-Ltmp18                  ##   Call between Ltmp18 and Ltmp19
	.long	Lset16
Lset17 = Ltmp20-Lfunc_begin0            ##     jumps to Ltmp20
	.long	Lset17
	.byte	0                       ##   On action: cleanup
Lset18 = Ltmp21-Lfunc_begin0            ## >> Call Site 7 <<
	.long	Lset18
Lset19 = Ltmp22-Ltmp21                  ##   Call between Ltmp21 and Ltmp22
	.long	Lset19
Lset20 = Ltmp23-Lfunc_begin0            ##     jumps to Ltmp23
	.long	Lset20
	.byte	0                       ##   On action: cleanup
Lset21 = Ltmp22-Lfunc_begin0            ## >> Call Site 8 <<
	.long	Lset21
Lset22 = Lfunc_end0-Ltmp22              ##   Call between Ltmp22 and Lfunc_end0
	.long	Lset22
	.long	0                       ##     has no landing pad
	.byte	0                       ##   On action: cleanup
	.p2align	2
                                        ## -- End function
	.section	__TEXT,__text,regular,pure_instructions
	.globl	__ZNSt3__114basic_ifstreamIcNS_11char_traitsIcEEED1Ev ## -- Begin function _ZNSt3__114basic_ifstreamIcNS_11char_traitsIcEEED1Ev
	.weak_def_can_be_hidden	__ZNSt3__114basic_ifstreamIcNS_11char_traitsIcEEED1Ev
	.p2align	4, 0x90
__ZNSt3__114basic_ifstreamIcNS_11char_traitsIcEEED1Ev: ## @_ZNSt3__114basic_ifstreamIcNS_11char_traitsIcEEED1Ev
	.cfi_startproc
## BB#0:
	pushq	%rbp
Lcfi8:
	.cfi_def_cfa_offset 16
Lcfi9:
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
Lcfi10:
	.cfi_def_cfa_register %rbp
	pushq	%r14
	pushq	%rbx
Lcfi11:
	.cfi_offset %rbx, -32
Lcfi12:
	.cfi_offset %r14, -24
	movq	%rdi, %rbx
	movq	__ZTVNSt3__114basic_ifstreamIcNS_11char_traitsIcEEEE@GOTPCREL(%rip), %rax
	leaq	24(%rax), %rcx
	movq	%rcx, (%rbx)
	addq	$64, %rax
	movq	%rax, 424(%rbx)
	leaq	16(%rbx), %rdi
	callq	__ZNSt3__113basic_filebufIcNS_11char_traitsIcEEED2Ev
	movq	__ZTTNSt3__114basic_ifstreamIcNS_11char_traitsIcEEEE@GOTPCREL(%rip), %rsi
	addq	$8, %rsi
	leaq	424(%rbx), %r14
	movq	%rbx, %rdi
	callq	__ZNSt3__113basic_istreamIcNS_11char_traitsIcEEED2Ev
	movq	%r14, %rdi
	popq	%rbx
	popq	%r14
	popq	%rbp
	jmp	__ZNSt3__19basic_iosIcNS_11char_traitsIcEEED2Ev ## TAILCALL
	.cfi_endproc
                                        ## -- End function
	.section	__TEXT,__literal8,8byte_literals
	.p2align	3               ## -- Begin function main
LCPI2_0:
	.quad	4696837146684686336     ## double 1.0E+6
	.section	__TEXT,__text,regular,pure_instructions
	.globl	_main
	.p2align	4, 0x90
_main:                                  ## @main
Lfunc_begin1:
	.cfi_startproc
	.cfi_personality 155, ___gxx_personality_v0
	.cfi_lsda 16, Lexception1
## BB#0:
	pushq	%rbp
Lcfi13:
	.cfi_def_cfa_offset 16
Lcfi14:
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
Lcfi15:
	.cfi_def_cfa_register %rbp
	pushq	%r15
	pushq	%r14
	pushq	%r13
	pushq	%r12
	pushq	%rbx
	subq	$72, %rsp
Lcfi16:
	.cfi_offset %rbx, -56
Lcfi17:
	.cfi_offset %r12, -48
Lcfi18:
	.cfi_offset %r13, -40
Lcfi19:
	.cfi_offset %r14, -32
Lcfi20:
	.cfi_offset %r15, -24
	leaq	L_.str(%rip), %rsi
	leaq	-64(%rbp), %rdi
	callq	__Z10read_linesPKc
Ltmp24:
	leaq	L_.str.1(%rip), %rsi
	leaq	-104(%rbp), %rdi
	callq	__Z10read_linesPKc
Ltmp25:
## BB#1:
	callq	__ZNSt3__16chrono12system_clock3nowEv
	movq	%rax, -72(%rbp)         ## 8-byte Spill
	movq	-104(%rbp), %r15
	movq	-96(%rbp), %rax
	movq	%rax, -80(%rbp)         ## 8-byte Spill
	cmpq	%rax, %r15
	je	LBB2_8
## BB#2:
	movq	-64(%rbp), %r13
	movq	-56(%rbp), %r14
	subq	%r13, %r14
	je	LBB2_8
## BB#3:
	sarq	$3, %r14
	.p2align	4, 0x90
LBB2_4:                                 ## =>This Loop Header: Depth=1
                                        ##     Child Loop BB2_5 Depth 2
	movq	(%r15), %rbx
	movl	$1, %r12d
	.p2align	4, 0x90
LBB2_5:                                 ##   Parent Loop BB2_4 Depth=1
                                        ## =>  This Inner Loop Header: Depth=2
	movq	-8(%r13,%r12,8), %rsi
	movq	%rbx, %rdi
	callq	_strcmp
	testl	%eax, %eax
	je	LBB2_7
## BB#6:                                ##   in Loop: Header=BB2_5 Depth=2
	cmpq	%r12, %r14
	leaq	1(%r12), %r12
	ja	LBB2_5
LBB2_7:                                 ##   in Loop: Header=BB2_4 Depth=1
	addq	$8, %r15
	cmpq	-80(%rbp), %r15         ## 8-byte Folded Reload
	jne	LBB2_4
LBB2_8:
	callq	__ZNSt3__16chrono12system_clock3nowEv
	subq	-72(%rbp), %rax         ## 8-byte Folded Reload
	cvtsi2sdq	%rax, %xmm0
	divsd	LCPI2_0(%rip), %xmm0
	leaq	L_.str.2(%rip), %rdi
	movb	$1, %al
	callq	_printf
	movq	-104(%rbp), %rdi
	testq	%rdi, %rdi
	je	LBB2_10
## BB#9:
	movq	%rdi, -96(%rbp)
	callq	__ZdlPv
LBB2_10:
	movq	-64(%rbp), %rdi
	testq	%rdi, %rdi
	je	LBB2_12
## BB#11:
	movq	%rdi, -56(%rbp)
	callq	__ZdlPv
LBB2_12:
	xorl	%eax, %eax
	addq	$72, %rsp
	popq	%rbx
	popq	%r12
	popq	%r13
	popq	%r14
	popq	%r15
	popq	%rbp
	retq
LBB2_13:
Ltmp26:
	movq	%rax, %rbx
	movq	-64(%rbp), %rdi
	testq	%rdi, %rdi
	je	LBB2_15
## BB#14:
	movq	%rdi, -56(%rbp)
	callq	__ZdlPv
LBB2_15:
	movq	%rbx, %rdi
	callq	__Unwind_Resume
Lfunc_end1:
	.cfi_endproc
	.section	__TEXT,__gcc_except_tab
	.p2align	2
GCC_except_table2:
Lexception1:
	.byte	255                     ## @LPStart Encoding = omit
	.byte	155                     ## @TType Encoding = indirect pcrel sdata4
	.byte	41                      ## @TType base offset
	.byte	3                       ## Call site Encoding = udata4
	.byte	39                      ## Call site table length
Lset23 = Lfunc_begin1-Lfunc_begin1      ## >> Call Site 1 <<
	.long	Lset23
Lset24 = Ltmp24-Lfunc_begin1            ##   Call between Lfunc_begin1 and Ltmp24
	.long	Lset24
	.long	0                       ##     has no landing pad
	.byte	0                       ##   On action: cleanup
Lset25 = Ltmp24-Lfunc_begin1            ## >> Call Site 2 <<
	.long	Lset25
Lset26 = Ltmp25-Ltmp24                  ##   Call between Ltmp24 and Ltmp25
	.long	Lset26
Lset27 = Ltmp26-Lfunc_begin1            ##     jumps to Ltmp26
	.long	Lset27
	.byte	0                       ##   On action: cleanup
Lset28 = Ltmp25-Lfunc_begin1            ## >> Call Site 3 <<
	.long	Lset28
Lset29 = Lfunc_end1-Ltmp25              ##   Call between Ltmp25 and Lfunc_end1
	.long	Lset29
	.long	0                       ##     has no landing pad
	.byte	0                       ##   On action: cleanup
	.p2align	2
                                        ## -- End function
	.section	__TEXT,__text,regular,pure_instructions
	.globl	__ZTv0_n24_NSt3__114basic_ifstreamIcNS_11char_traitsIcEEED1Ev ## -- Begin function _ZTv0_n24_NSt3__114basic_ifstreamIcNS_11char_traitsIcEEED1Ev
	.weak_def_can_be_hidden	__ZTv0_n24_NSt3__114basic_ifstreamIcNS_11char_traitsIcEEED1Ev
	.p2align	4, 0x90
__ZTv0_n24_NSt3__114basic_ifstreamIcNS_11char_traitsIcEEED1Ev: ## @_ZTv0_n24_NSt3__114basic_ifstreamIcNS_11char_traitsIcEEED1Ev
	.cfi_startproc
## BB#0:
	pushq	%rbp
Lcfi21:
	.cfi_def_cfa_offset 16
Lcfi22:
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
Lcfi23:
	.cfi_def_cfa_register %rbp
	pushq	%r14
	pushq	%rbx
Lcfi24:
	.cfi_offset %rbx, -32
Lcfi25:
	.cfi_offset %r14, -24
	movq	(%rdi), %rax
	movq	-24(%rax), %rax
	leaq	(%rdi,%rax), %rbx
	movq	__ZTVNSt3__114basic_ifstreamIcNS_11char_traitsIcEEEE@GOTPCREL(%rip), %rcx
	leaq	24(%rcx), %rdx
	movq	%rdx, (%rdi,%rax)
	leaq	424(%rdi,%rax), %r14
	addq	$64, %rcx
	movq	%rcx, 424(%rdi,%rax)
	leaq	16(%rdi,%rax), %rdi
	callq	__ZNSt3__113basic_filebufIcNS_11char_traitsIcEEED2Ev
	movq	__ZTTNSt3__114basic_ifstreamIcNS_11char_traitsIcEEEE@GOTPCREL(%rip), %rsi
	addq	$8, %rsi
	movq	%rbx, %rdi
	callq	__ZNSt3__113basic_istreamIcNS_11char_traitsIcEEED2Ev
	movq	%r14, %rdi
	popq	%rbx
	popq	%r14
	popq	%rbp
	jmp	__ZNSt3__19basic_iosIcNS_11char_traitsIcEEED2Ev ## TAILCALL
	.cfi_endproc
                                        ## -- End function
	.globl	__ZNSt3__114basic_ifstreamIcNS_11char_traitsIcEEED0Ev ## -- Begin function _ZNSt3__114basic_ifstreamIcNS_11char_traitsIcEEED0Ev
	.weak_def_can_be_hidden	__ZNSt3__114basic_ifstreamIcNS_11char_traitsIcEEED0Ev
	.p2align	4, 0x90
__ZNSt3__114basic_ifstreamIcNS_11char_traitsIcEEED0Ev: ## @_ZNSt3__114basic_ifstreamIcNS_11char_traitsIcEEED0Ev
	.cfi_startproc
## BB#0:
	pushq	%rbp
Lcfi26:
	.cfi_def_cfa_offset 16
Lcfi27:
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
Lcfi28:
	.cfi_def_cfa_register %rbp
	pushq	%r14
	pushq	%rbx
Lcfi29:
	.cfi_offset %rbx, -32
Lcfi30:
	.cfi_offset %r14, -24
	movq	%rdi, %rbx
	movq	__ZTVNSt3__114basic_ifstreamIcNS_11char_traitsIcEEEE@GOTPCREL(%rip), %rax
	leaq	24(%rax), %rcx
	movq	%rcx, (%rbx)
	leaq	424(%rbx), %r14
	addq	$64, %rax
	movq	%rax, 424(%rbx)
	leaq	16(%rbx), %rdi
	callq	__ZNSt3__113basic_filebufIcNS_11char_traitsIcEEED2Ev
	movq	__ZTTNSt3__114basic_ifstreamIcNS_11char_traitsIcEEEE@GOTPCREL(%rip), %rsi
	addq	$8, %rsi
	movq	%rbx, %rdi
	callq	__ZNSt3__113basic_istreamIcNS_11char_traitsIcEEED2Ev
	movq	%r14, %rdi
	callq	__ZNSt3__19basic_iosIcNS_11char_traitsIcEEED2Ev
	movq	%rbx, %rdi
	popq	%rbx
	popq	%r14
	popq	%rbp
	jmp	__ZdlPv                 ## TAILCALL
	.cfi_endproc
                                        ## -- End function
	.globl	__ZTv0_n24_NSt3__114basic_ifstreamIcNS_11char_traitsIcEEED0Ev ## -- Begin function _ZTv0_n24_NSt3__114basic_ifstreamIcNS_11char_traitsIcEEED0Ev
	.weak_def_can_be_hidden	__ZTv0_n24_NSt3__114basic_ifstreamIcNS_11char_traitsIcEEED0Ev
	.p2align	4, 0x90
__ZTv0_n24_NSt3__114basic_ifstreamIcNS_11char_traitsIcEEED0Ev: ## @_ZTv0_n24_NSt3__114basic_ifstreamIcNS_11char_traitsIcEEED0Ev
	.cfi_startproc
## BB#0:
	pushq	%rbp
Lcfi31:
	.cfi_def_cfa_offset 16
Lcfi32:
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
Lcfi33:
	.cfi_def_cfa_register %rbp
	pushq	%r14
	pushq	%rbx
Lcfi34:
	.cfi_offset %rbx, -32
Lcfi35:
	.cfi_offset %r14, -24
	movq	(%rdi), %rax
	movq	-24(%rax), %rax
	leaq	(%rdi,%rax), %rbx
	movq	__ZTVNSt3__114basic_ifstreamIcNS_11char_traitsIcEEEE@GOTPCREL(%rip), %rcx
	leaq	24(%rcx), %rdx
	movq	%rdx, (%rdi,%rax)
	leaq	424(%rdi,%rax), %r14
	addq	$64, %rcx
	movq	%rcx, 424(%rdi,%rax)
	leaq	16(%rdi,%rax), %rdi
	callq	__ZNSt3__113basic_filebufIcNS_11char_traitsIcEEED2Ev
	movq	__ZTTNSt3__114basic_ifstreamIcNS_11char_traitsIcEEEE@GOTPCREL(%rip), %rsi
	addq	$8, %rsi
	movq	%rbx, %rdi
	callq	__ZNSt3__113basic_istreamIcNS_11char_traitsIcEEED2Ev
	movq	%r14, %rdi
	callq	__ZNSt3__19basic_iosIcNS_11char_traitsIcEEED2Ev
	movq	%rbx, %rdi
	popq	%rbx
	popq	%r14
	popq	%rbp
	jmp	__ZdlPv                 ## TAILCALL
	.cfi_endproc
                                        ## -- End function
	.globl	__ZNSt3__113basic_filebufIcNS_11char_traitsIcEEED1Ev ## -- Begin function _ZNSt3__113basic_filebufIcNS_11char_traitsIcEEED1Ev
	.weak_def_can_be_hidden	__ZNSt3__113basic_filebufIcNS_11char_traitsIcEEED1Ev
	.p2align	4, 0x90
__ZNSt3__113basic_filebufIcNS_11char_traitsIcEEED1Ev: ## @_ZNSt3__113basic_filebufIcNS_11char_traitsIcEEED1Ev
	.cfi_startproc
## BB#0:
	pushq	%rbp
Lcfi36:
	.cfi_def_cfa_offset 16
Lcfi37:
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
Lcfi38:
	.cfi_def_cfa_register %rbp
	popq	%rbp
	jmp	__ZNSt3__113basic_filebufIcNS_11char_traitsIcEEED2Ev ## TAILCALL
	.cfi_endproc
                                        ## -- End function
	.globl	__ZNSt3__113basic_filebufIcNS_11char_traitsIcEEED2Ev ## -- Begin function _ZNSt3__113basic_filebufIcNS_11char_traitsIcEEED2Ev
	.weak_def_can_be_hidden	__ZNSt3__113basic_filebufIcNS_11char_traitsIcEEED2Ev
	.p2align	4, 0x90
__ZNSt3__113basic_filebufIcNS_11char_traitsIcEEED2Ev: ## @_ZNSt3__113basic_filebufIcNS_11char_traitsIcEEED2Ev
Lfunc_begin2:
	.cfi_startproc
	.cfi_personality 155, ___gxx_personality_v0
	.cfi_lsda 16, Lexception2
## BB#0:
	pushq	%rbp
Lcfi39:
	.cfi_def_cfa_offset 16
Lcfi40:
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
Lcfi41:
	.cfi_def_cfa_register %rbp
	pushq	%r15
	pushq	%r14
	pushq	%rbx
	pushq	%rax
Lcfi42:
	.cfi_offset %rbx, -40
Lcfi43:
	.cfi_offset %r14, -32
Lcfi44:
	.cfi_offset %r15, -24
	movq	%rdi, %rbx
	movq	__ZTVNSt3__113basic_filebufIcNS_11char_traitsIcEEEE@GOTPCREL(%rip), %rax
	addq	$16, %rax
	movq	%rax, (%rbx)
	movq	120(%rbx), %r14
	testq	%r14, %r14
	je	LBB7_5
## BB#1:
Ltmp27:
	movq	%rbx, %rdi
	callq	__ZNSt3__113basic_filebufIcNS_11char_traitsIcEEE4syncEv
Ltmp28:
## BB#2:
	movq	%r14, %rdi
	callq	_fclose
	testl	%eax, %eax
	jne	LBB7_5
## BB#3:
	movq	$0, 120(%rbx)
LBB7_5:
	cmpb	$0, 400(%rbx)
	je	LBB7_8
## BB#6:
	movq	64(%rbx), %rdi
	testq	%rdi, %rdi
	je	LBB7_8
## BB#7:
	callq	__ZdaPv
LBB7_8:
	cmpb	$0, 401(%rbx)
	je	LBB7_11
## BB#9:
	movq	104(%rbx), %rdi
	testq	%rdi, %rdi
	je	LBB7_11
## BB#10:
	callq	__ZdaPv
LBB7_11:
	movq	%rbx, %rdi
	addq	$8, %rsp
	popq	%rbx
	popq	%r14
	popq	%r15
	popq	%rbp
	jmp	__ZNSt3__115basic_streambufIcNS_11char_traitsIcEEED2Ev ## TAILCALL
LBB7_4:
Ltmp29:
	movq	%rax, %r15
	movq	%r14, %rdi
	callq	_fclose
	movq	%r15, %rdi
	callq	___cxa_begin_catch
Ltmp30:
	callq	___cxa_end_catch
Ltmp31:
	jmp	LBB7_5
LBB7_12:
Ltmp32:
	movq	%rax, %r14
	movq	%rbx, %rdi
	callq	__ZNSt3__115basic_streambufIcNS_11char_traitsIcEEED2Ev
	movq	%r14, %rdi
	callq	___clang_call_terminate
Lfunc_end2:
	.cfi_endproc
	.section	__TEXT,__gcc_except_tab
	.p2align	2
GCC_except_table7:
Lexception2:
	.byte	255                     ## @LPStart Encoding = omit
	.byte	155                     ## @TType Encoding = indirect pcrel sdata4
	.asciz	"\257\200"              ## @TType base offset
	.byte	3                       ## Call site Encoding = udata4
	.byte	39                      ## Call site table length
Lset30 = Ltmp27-Lfunc_begin2            ## >> Call Site 1 <<
	.long	Lset30
Lset31 = Ltmp28-Ltmp27                  ##   Call between Ltmp27 and Ltmp28
	.long	Lset31
Lset32 = Ltmp29-Lfunc_begin2            ##     jumps to Ltmp29
	.long	Lset32
	.byte	1                       ##   On action: 1
Lset33 = Ltmp28-Lfunc_begin2            ## >> Call Site 2 <<
	.long	Lset33
Lset34 = Ltmp30-Ltmp28                  ##   Call between Ltmp28 and Ltmp30
	.long	Lset34
	.long	0                       ##     has no landing pad
	.byte	0                       ##   On action: cleanup
Lset35 = Ltmp30-Lfunc_begin2            ## >> Call Site 3 <<
	.long	Lset35
Lset36 = Ltmp31-Ltmp30                  ##   Call between Ltmp30 and Ltmp31
	.long	Lset36
Lset37 = Ltmp32-Lfunc_begin2            ##     jumps to Ltmp32
	.long	Lset37
	.byte	1                       ##   On action: 1
	.byte	1                       ## >> Action Record 1 <<
                                        ##   Catch TypeInfo 1
	.byte	0                       ##   No further actions
                                        ## >> Catch TypeInfos <<
	.long	0                       ## TypeInfo 1
	.p2align	2
                                        ## -- End function
	.section	__TEXT,__text,regular,pure_instructions
	.private_extern	___clang_call_terminate ## -- Begin function __clang_call_terminate
	.globl	___clang_call_terminate
	.weak_def_can_be_hidden	___clang_call_terminate
	.p2align	4, 0x90
___clang_call_terminate:                ## @__clang_call_terminate
## BB#0:
	pushq	%rax
	callq	___cxa_begin_catch
	callq	__ZSt9terminatev
                                        ## -- End function
	.globl	__ZNSt3__113basic_filebufIcNS_11char_traitsIcEEED0Ev ## -- Begin function _ZNSt3__113basic_filebufIcNS_11char_traitsIcEEED0Ev
	.weak_def_can_be_hidden	__ZNSt3__113basic_filebufIcNS_11char_traitsIcEEED0Ev
	.p2align	4, 0x90
__ZNSt3__113basic_filebufIcNS_11char_traitsIcEEED0Ev: ## @_ZNSt3__113basic_filebufIcNS_11char_traitsIcEEED0Ev
	.cfi_startproc
## BB#0:
	pushq	%rbp
Lcfi45:
	.cfi_def_cfa_offset 16
Lcfi46:
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
Lcfi47:
	.cfi_def_cfa_register %rbp
	pushq	%rbx
	pushq	%rax
Lcfi48:
	.cfi_offset %rbx, -24
	movq	%rdi, %rbx
	callq	__ZNSt3__113basic_filebufIcNS_11char_traitsIcEEED2Ev
	movq	%rbx, %rdi
	addq	$8, %rsp
	popq	%rbx
	popq	%rbp
	jmp	__ZdlPv                 ## TAILCALL
	.cfi_endproc
                                        ## -- End function
	.globl	__ZNSt3__113basic_filebufIcNS_11char_traitsIcEEE5imbueERKNS_6localeE ## -- Begin function _ZNSt3__113basic_filebufIcNS_11char_traitsIcEEE5imbueERKNS_6localeE
	.weak_def_can_be_hidden	__ZNSt3__113basic_filebufIcNS_11char_traitsIcEEE5imbueERKNS_6localeE
	.p2align	4, 0x90
__ZNSt3__113basic_filebufIcNS_11char_traitsIcEEE5imbueERKNS_6localeE: ## @_ZNSt3__113basic_filebufIcNS_11char_traitsIcEEE5imbueERKNS_6localeE
	.cfi_startproc
## BB#0:
	pushq	%rbp
Lcfi49:
	.cfi_def_cfa_offset 16
Lcfi50:
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
Lcfi51:
	.cfi_def_cfa_register %rbp
	pushq	%r14
	pushq	%rbx
Lcfi52:
	.cfi_offset %rbx, -32
Lcfi53:
	.cfi_offset %r14, -24
	movq	%rsi, %r14
	movq	%rdi, %rbx
	movq	(%rbx), %rax
	callq	*48(%rax)
	movq	__ZNSt3__17codecvtIcc11__mbstate_tE2idE@GOTPCREL(%rip), %rsi
	movq	%r14, %rdi
	callq	__ZNKSt3__16locale9use_facetERNS0_2idE
	movq	%rax, 128(%rbx)
	movq	(%rax), %rcx
	movzbl	402(%rbx), %r14d
	movq	%rax, %rdi
	callq	*56(%rcx)
	movb	%al, 402(%rbx)
	movzbl	%al, %ecx
	cmpl	%ecx, %r14d
	je	LBB10_10
## BB#1:
	movq	$0, 56(%rbx)
	movq	$0, 48(%rbx)
	movq	$0, 40(%rbx)
	movq	$0, 32(%rbx)
	movq	$0, 24(%rbx)
	movq	$0, 16(%rbx)
	movb	400(%rbx), %cl
	testb	%al, %al
	je	LBB10_6
## BB#2:
	testb	%cl, %cl
	je	LBB10_5
## BB#3:
	movq	64(%rbx), %rdi
	testq	%rdi, %rdi
	je	LBB10_5
## BB#4:
	callq	__ZdaPv
LBB10_5:
	movb	401(%rbx), %al
	movb	%al, 400(%rbx)
	movq	104(%rbx), %rax
	movq	112(%rbx), %rcx
	movq	%rcx, 96(%rbx)
	movq	%rax, 64(%rbx)
	movb	$0, 401(%rbx)
	movq	$0, 112(%rbx)
	movq	$0, 104(%rbx)
	jmp	LBB10_10
LBB10_6:
	testb	%cl, %cl
	je	LBB10_7
LBB10_9:
	movq	96(%rbx), %rdi
	movq	%rdi, 112(%rbx)
	callq	__Znam
	movq	%rax, 104(%rbx)
	movb	$1, 401(%rbx)
	jmp	LBB10_10
LBB10_7:
	movq	64(%rbx), %rax
	leaq	88(%rbx), %rcx
	cmpq	%rcx, %rax
	je	LBB10_9
## BB#8:
	movq	96(%rbx), %rdi
	movq	%rdi, 112(%rbx)
	movq	%rax, 104(%rbx)
	movb	$0, 401(%rbx)
	callq	__Znam
	movq	%rax, 64(%rbx)
	movb	$1, 400(%rbx)
LBB10_10:
	popq	%rbx
	popq	%r14
	popq	%rbp
	retq
	.cfi_endproc
                                        ## -- End function
	.globl	__ZNSt3__113basic_filebufIcNS_11char_traitsIcEEE6setbufEPcl ## -- Begin function _ZNSt3__113basic_filebufIcNS_11char_traitsIcEEE6setbufEPcl
	.weak_def_can_be_hidden	__ZNSt3__113basic_filebufIcNS_11char_traitsIcEEE6setbufEPcl
	.p2align	4, 0x90
__ZNSt3__113basic_filebufIcNS_11char_traitsIcEEE6setbufEPcl: ## @_ZNSt3__113basic_filebufIcNS_11char_traitsIcEEE6setbufEPcl
	.cfi_startproc
## BB#0:
	pushq	%rbp
Lcfi54:
	.cfi_def_cfa_offset 16
Lcfi55:
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
Lcfi56:
	.cfi_def_cfa_register %rbp
	pushq	%r15
	pushq	%r14
	pushq	%r12
	pushq	%rbx
Lcfi57:
	.cfi_offset %rbx, -48
Lcfi58:
	.cfi_offset %r12, -40
Lcfi59:
	.cfi_offset %r14, -32
Lcfi60:
	.cfi_offset %r15, -24
	movq	%rdx, %r15
	movq	%rsi, %r14
	movq	%rdi, %rbx
	movq	$0, 56(%rbx)
	movq	$0, 48(%rbx)
	movq	$0, 40(%rbx)
	movq	$0, 32(%rbx)
	movq	$0, 24(%rbx)
	movq	$0, 16(%rbx)
	cmpb	$0, 400(%rbx)
	je	LBB11_3
## BB#1:
	movq	64(%rbx), %rdi
	testq	%rdi, %rdi
	je	LBB11_3
## BB#2:
	callq	__ZdaPv
LBB11_3:
	cmpb	$0, 401(%rbx)
	je	LBB11_6
## BB#4:
	movq	104(%rbx), %rdi
	testq	%rdi, %rdi
	je	LBB11_6
## BB#5:
	callq	__ZdaPv
LBB11_6:
	movq	%r15, 96(%rbx)
	cmpq	$9, %r15
	jb	LBB11_11
## BB#7:
	movb	402(%rbx), %r12b
	testq	%r14, %r14
	je	LBB11_13
## BB#8:
	testb	%r12b, %r12b
	je	LBB11_13
## BB#9:
	xorl	%ecx, %ecx
	movq	%r14, %rax
	jmp	LBB11_10
LBB11_11:
	leaq	88(%rbx), %rax
	movq	%rax, 64(%rbx)
	movq	$8, 96(%rbx)
	movb	402(%rbx), %r12b
	xorl	%ecx, %ecx
	movb	%cl, 400(%rbx)
	testb	%r12b, %r12b
	jne	LBB11_12
LBB11_14:
	cmpq	$7, %r15
	movl	$8, %edi
	cmovgq	%r15, %rdi
	movq	%rdi, 112(%rbx)
	testq	%r14, %r14
	je	LBB11_17
## BB#15:
	cmpq	$7, %rdi
	ja	LBB11_16
LBB11_17:
	callq	__Znam
	movq	%rax, %r14
	movb	$1, %al
	jmp	LBB11_18
LBB11_13:
	movq	%r15, %rdi
	callq	__Znam
	movb	$1, %cl
LBB11_10:
	movq	%rax, 64(%rbx)
	movb	%cl, 400(%rbx)
	testb	%r12b, %r12b
	je	LBB11_14
LBB11_12:
	movq	$0, 112(%rbx)
	xorl	%r14d, %r14d
LBB11_16:
	xorl	%eax, %eax
LBB11_18:
	movq	%r14, 104(%rbx)
	movb	%al, 401(%rbx)
	movq	%rbx, %rax
	popq	%rbx
	popq	%r12
	popq	%r14
	popq	%r15
	popq	%rbp
	retq
	.cfi_endproc
                                        ## -- End function
	.globl	__ZNSt3__113basic_filebufIcNS_11char_traitsIcEEE7seekoffExNS_8ios_base7seekdirEj ## -- Begin function _ZNSt3__113basic_filebufIcNS_11char_traitsIcEEE7seekoffExNS_8ios_base7seekdirEj
	.weak_def_can_be_hidden	__ZNSt3__113basic_filebufIcNS_11char_traitsIcEEE7seekoffExNS_8ios_base7seekdirEj
	.p2align	4, 0x90
__ZNSt3__113basic_filebufIcNS_11char_traitsIcEEE7seekoffExNS_8ios_base7seekdirEj: ## @_ZNSt3__113basic_filebufIcNS_11char_traitsIcEEE7seekoffExNS_8ios_base7seekdirEj
	.cfi_startproc
## BB#0:
	pushq	%rbp
Lcfi61:
	.cfi_def_cfa_offset 16
Lcfi62:
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
Lcfi63:
	.cfi_def_cfa_register %rbp
	pushq	%r15
	pushq	%r14
	pushq	%r13
	pushq	%r12
	pushq	%rbx
	pushq	%rax
Lcfi64:
	.cfi_offset %rbx, -56
Lcfi65:
	.cfi_offset %r12, -48
Lcfi66:
	.cfi_offset %r13, -40
Lcfi67:
	.cfi_offset %r14, -32
Lcfi68:
	.cfi_offset %r15, -24
	movl	%ecx, %r15d
	movq	%rdx, %r12
	movq	%rsi, %r14
	movq	%rdi, %rbx
	movq	128(%r14), %rdi
	testq	%rdi, %rdi
	je	LBB12_10
## BB#1:
	movq	(%rdi), %rax
	callq	*48(%rax)
	movl	%eax, %r13d
	cmpq	$0, 120(%r14)
	je	LBB12_5
## BB#2:
	testq	%r12, %r12
	je	LBB12_4
## BB#3:
	testl	%r13d, %r13d
	jle	LBB12_5
LBB12_4:
	movq	(%r14), %rax
	movq	%r14, %rdi
	callq	*48(%rax)
	testl	%eax, %eax
	jne	LBB12_5
## BB#6:
	cmpl	$3, %r15d
	jae	LBB12_5
## BB#7:
	movq	120(%r14), %rdi
	movslq	%r13d, %rax
	imulq	%rax, %r12
	xorl	%esi, %esi
	testl	%eax, %eax
	cmovgq	%r12, %rsi
	movl	%r15d, %edx
	callq	_fseeko
	testl	%eax, %eax
	je	LBB12_8
LBB12_5:
	movq	$0, 120(%rbx)
	movq	$0, 112(%rbx)
	movq	$0, 104(%rbx)
	movq	$0, 96(%rbx)
	movq	$0, 88(%rbx)
	movq	$0, 80(%rbx)
	movq	$0, 72(%rbx)
	movq	$0, 64(%rbx)
	movq	$0, 56(%rbx)
	movq	$0, 48(%rbx)
	movq	$0, 40(%rbx)
	movq	$0, 32(%rbx)
	movq	$0, 24(%rbx)
	movq	$0, 16(%rbx)
	movq	$0, 8(%rbx)
	movq	$0, (%rbx)
	movq	$-1, %rax
LBB12_9:
	movq	%rax, 128(%rbx)
	movq	%rbx, %rax
	addq	$8, %rsp
	popq	%rbx
	popq	%r12
	popq	%r13
	popq	%r14
	popq	%r15
	popq	%rbp
	retq
LBB12_8:
	movq	120(%r14), %rdi
	callq	_ftello
	addq	$136, %r14
	movl	$16, %ecx
	movq	%rbx, %rdi
	movq	%r14, %rsi
	rep;movsq
	jmp	LBB12_9
LBB12_10:
	movl	$8, %edi
	callq	___cxa_allocate_exception
	movq	%rax, %rbx
	movq	%rbx, %rdi
	callq	__ZNSt8bad_castC1Ev
	movq	__ZTISt8bad_cast@GOTPCREL(%rip), %rsi
	movq	__ZNSt8bad_castD1Ev@GOTPCREL(%rip), %rdx
	movq	%rbx, %rdi
	callq	___cxa_throw
	.cfi_endproc
                                        ## -- End function
	.globl	__ZNSt3__113basic_filebufIcNS_11char_traitsIcEEE7seekposENS_4fposI11__mbstate_tEEj ## -- Begin function _ZNSt3__113basic_filebufIcNS_11char_traitsIcEEE7seekposENS_4fposI11__mbstate_tEEj
	.weak_def_can_be_hidden	__ZNSt3__113basic_filebufIcNS_11char_traitsIcEEE7seekposENS_4fposI11__mbstate_tEEj
	.p2align	4, 0x90
__ZNSt3__113basic_filebufIcNS_11char_traitsIcEEE7seekposENS_4fposI11__mbstate_tEEj: ## @_ZNSt3__113basic_filebufIcNS_11char_traitsIcEEE7seekposENS_4fposI11__mbstate_tEEj
	.cfi_startproc
## BB#0:
	pushq	%rbp
Lcfi69:
	.cfi_def_cfa_offset 16
Lcfi70:
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
Lcfi71:
	.cfi_def_cfa_register %rbp
	pushq	%r15
	pushq	%r14
	pushq	%rbx
	pushq	%rax
Lcfi72:
	.cfi_offset %rbx, -40
Lcfi73:
	.cfi_offset %r14, -32
Lcfi74:
	.cfi_offset %r15, -24
	movq	%rsi, %r14
	movq	%rdi, %rbx
	cmpq	$0, 120(%r14)
	je	LBB13_2
## BB#1:
	movq	(%r14), %rax
	movq	%r14, %rdi
	callq	*48(%rax)
	testl	%eax, %eax
	jne	LBB13_2
## BB#3:
	leaq	16(%rbp), %r15
	movq	120(%r14), %rdi
	movq	128(%r15), %rsi
	xorl	%edx, %edx
	callq	_fseeko
	testl	%eax, %eax
	je	LBB13_4
LBB13_2:
	movq	$0, 120(%rbx)
	movq	$0, 112(%rbx)
	movq	$0, 104(%rbx)
	movq	$0, 96(%rbx)
	movq	$0, 88(%rbx)
	movq	$0, 80(%rbx)
	movq	$0, 72(%rbx)
	movq	$0, 64(%rbx)
	movq	$0, 56(%rbx)
	movq	$0, 48(%rbx)
	movq	$0, 40(%rbx)
	movq	$0, 32(%rbx)
	movq	$0, 24(%rbx)
	movq	$0, 16(%rbx)
	movq	$0, 8(%rbx)
	movq	$0, (%rbx)
	movq	$-1, 128(%rbx)
LBB13_5:
	movq	%rbx, %rax
	addq	$8, %rsp
	popq	%rbx
	popq	%r14
	popq	%r15
	popq	%rbp
	retq
LBB13_4:
	addq	$136, %r14
	movl	$16, %ecx
	movq	%r14, %rdi
	movq	%r15, %rsi
	rep;movsq
	movl	$136, %edx
	movq	%rbx, %rdi
	movq	%r15, %rsi
	callq	_memcpy
	jmp	LBB13_5
	.cfi_endproc
                                        ## -- End function
	.globl	__ZNSt3__113basic_filebufIcNS_11char_traitsIcEEE4syncEv ## -- Begin function _ZNSt3__113basic_filebufIcNS_11char_traitsIcEEE4syncEv
	.weak_def_can_be_hidden	__ZNSt3__113basic_filebufIcNS_11char_traitsIcEEE4syncEv
	.p2align	4, 0x90
__ZNSt3__113basic_filebufIcNS_11char_traitsIcEEE4syncEv: ## @_ZNSt3__113basic_filebufIcNS_11char_traitsIcEEE4syncEv
	.cfi_startproc
## BB#0:
	pushq	%rbp
Lcfi75:
	.cfi_def_cfa_offset 16
Lcfi76:
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
Lcfi77:
	.cfi_def_cfa_register %rbp
	pushq	%r15
	pushq	%r14
	pushq	%r13
	pushq	%r12
	pushq	%rbx
	subq	$136, %rsp
Lcfi78:
	.cfi_offset %rbx, -56
Lcfi79:
	.cfi_offset %r12, -48
Lcfi80:
	.cfi_offset %r13, -40
Lcfi81:
	.cfi_offset %r14, -32
Lcfi82:
	.cfi_offset %r15, -24
	movq	%rdi, %r15
	movq	___stack_chk_guard@GOTPCREL(%rip), %rax
	movq	(%rax), %rax
	movq	%rax, -48(%rbp)
	xorl	%r13d, %r13d
	cmpq	$0, 120(%r15)
	je	LBB14_24
## BB#1:
	movq	128(%r15), %rax
	testq	%rax, %rax
	je	LBB14_27
## BB#2:
	movl	396(%r15), %ecx
	testb	$16, %cl
	jne	LBB14_6
## BB#3:
	testb	$8, %cl
	je	LBB14_23
## BB#4:
	leaq	264(%r15), %rsi
	leaq	-176(%rbp), %rdi
	movl	$16, %ecx
	rep;movsq
	cmpb	$0, 402(%r15)
	je	LBB14_13
## BB#5:
	movq	32(%r15), %r14
	subq	24(%r15), %r14
	jmp	LBB14_17
LBB14_6:
	movq	48(%r15), %rax
	cmpq	40(%r15), %rax
	je	LBB14_8
## BB#7:
	movq	(%r15), %rax
	movl	$-1, %r13d
	movl	$-1, %esi
	movq	%r15, %rdi
	callq	*104(%rax)
	cmpl	$-1, %eax
	je	LBB14_24
LBB14_8:
	leaq	136(%r15), %r14
	movl	$-1, %r13d
	.p2align	4, 0x90
LBB14_9:                                ## =>This Inner Loop Header: Depth=1
	movq	64(%r15), %rdx
	movq	128(%r15), %rdi
	movq	96(%r15), %rcx
	addq	%rdx, %rcx
	movq	(%rdi), %rax
	movq	%r14, %rsi
	leaq	-176(%rbp), %r8
	callq	*40(%rax)
	movl	%eax, %r12d
	movq	-176(%rbp), %rbx
	movq	64(%r15), %rdi
	movq	120(%r15), %rcx
	subq	%rdi, %rbx
	movl	$1, %esi
	movq	%rbx, %rdx
	callq	_fwrite
	cmpq	%rbx, %rax
	jne	LBB14_24
## BB#10:                               ##   in Loop: Header=BB14_9 Depth=1
	cmpl	$1, %r12d
	je	LBB14_9
## BB#11:
	cmpl	$2, %r12d
	je	LBB14_24
## BB#12:
	movq	120(%r15), %rdi
	callq	_fflush
	testl	%eax, %eax
	jne	LBB14_24
	jmp	LBB14_23
LBB14_13:
	movq	(%rax), %rcx
	movq	%rax, %rdi
	callq	*48(%rcx)
	movq	72(%r15), %rcx
	movq	80(%r15), %r14
	subq	%rcx, %r14
	testl	%eax, %eax
	jle	LBB14_15
## BB#14:
	cltq
	movq	32(%r15), %rcx
	subq	24(%r15), %rcx
	imulq	%rax, %rcx
	addq	%rcx, %r14
LBB14_17:
	xorl	%ebx, %ebx
LBB14_18:
	movq	120(%r15), %rdi
	negq	%r14
	movl	$1, %edx
	movq	%r14, %rsi
	callq	_fseeko
	testl	%eax, %eax
	je	LBB14_20
## BB#19:
	movl	$-1, %r13d
	jmp	LBB14_24
LBB14_20:
	testb	%bl, %bl
	je	LBB14_22
## BB#21:
	leaq	136(%r15), %rdi
	leaq	-176(%rbp), %rsi
	movl	$16, %ecx
	rep;movsq
LBB14_22:
	movq	64(%r15), %rax
	movq	%rax, 80(%r15)
	movq	%rax, 72(%r15)
	movl	$0, 396(%r15)
	movq	$0, 32(%r15)
	movq	$0, 24(%r15)
	movq	$0, 16(%r15)
LBB14_23:
	xorl	%r13d, %r13d
LBB14_24:
	movq	___stack_chk_guard@GOTPCREL(%rip), %rax
	movq	(%rax), %rax
	cmpq	-48(%rbp), %rax
	jne	LBB14_26
## BB#25:
	movl	%r13d, %eax
	addq	$136, %rsp
	popq	%rbx
	popq	%r12
	popq	%r13
	popq	%r14
	popq	%r15
	popq	%rbp
	retq
LBB14_15:
	movq	24(%r15), %r8
	cmpq	32(%r15), %r8
	je	LBB14_17
## BB#16:
	movq	64(%r15), %rdx
	movq	128(%r15), %rdi
	subq	16(%r15), %r8
	movq	(%rdi), %rax
	leaq	-176(%rbp), %rsi
	callq	*64(%rax)
	cltq
	addq	72(%r15), %r14
	subq	%rax, %r14
	subq	64(%r15), %r14
	movb	$1, %bl
	jmp	LBB14_18
LBB14_26:
	callq	___stack_chk_fail
LBB14_27:
	movl	$8, %edi
	callq	___cxa_allocate_exception
	movq	%rax, %rbx
	movq	%rbx, %rdi
	callq	__ZNSt8bad_castC1Ev
	movq	__ZTISt8bad_cast@GOTPCREL(%rip), %rsi
	movq	__ZNSt8bad_castD1Ev@GOTPCREL(%rip), %rdx
	movq	%rbx, %rdi
	callq	___cxa_throw
	.cfi_endproc
                                        ## -- End function
	.globl	__ZNSt3__113basic_filebufIcNS_11char_traitsIcEEE9underflowEv ## -- Begin function _ZNSt3__113basic_filebufIcNS_11char_traitsIcEEE9underflowEv
	.weak_def_can_be_hidden	__ZNSt3__113basic_filebufIcNS_11char_traitsIcEEE9underflowEv
	.p2align	4, 0x90
__ZNSt3__113basic_filebufIcNS_11char_traitsIcEEE9underflowEv: ## @_ZNSt3__113basic_filebufIcNS_11char_traitsIcEEE9underflowEv
	.cfi_startproc
## BB#0:
	pushq	%rbp
Lcfi83:
	.cfi_def_cfa_offset 16
Lcfi84:
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
Lcfi85:
	.cfi_def_cfa_register %rbp
	pushq	%r15
	pushq	%r14
	pushq	%r13
	pushq	%r12
	pushq	%rbx
	subq	$24, %rsp
Lcfi86:
	.cfi_offset %rbx, -56
Lcfi87:
	.cfi_offset %r12, -48
Lcfi88:
	.cfi_offset %r13, -40
Lcfi89:
	.cfi_offset %r14, -32
Lcfi90:
	.cfi_offset %r15, -24
	movq	%rdi, %rbx
	cmpq	$0, 120(%rbx)
	je	LBB15_3
## BB#1:
	testb	$8, 396(%rbx)
	jne	LBB15_4
## BB#2:
	movq	$0, 56(%rbx)
	movq	$0, 48(%rbx)
	movq	$0, 40(%rbx)
	leaq	104(%rbx), %rax
	leaq	112(%rbx), %rcx
	leaq	64(%rbx), %rdx
	leaq	96(%rbx), %rsi
	cmpb	$0, 402(%rbx)
	cmoveq	%rcx, %rsi
	cmoveq	%rax, %rdx
	movq	(%rdx), %rax
	movq	(%rsi), %rsi
	addq	%rax, %rsi
	movq	%rax, 16(%rbx)
	movq	%rsi, 24(%rbx)
	movq	%rsi, 32(%rbx)
	movl	$8, 396(%rbx)
	movb	$1, %al
	testq	%rsi, %rsi
	je	LBB15_10
LBB15_5:
	testb	%al, %al
	je	LBB15_11
LBB15_6:
	movq	32(%rbx), %rax
	xorl	%r14d, %r14d
	cmpq	%rax, %rsi
	je	LBB15_12
LBB15_7:
	movzbl	(%rsi), %eax
	leaq	16(%rbx), %r12
	leaq	-41(%rbp), %rcx
	cmpq	%rcx, (%r12)
	jne	LBB15_9
	jmp	LBB15_8
LBB15_3:
	movl	$-1, %eax
	jmp	LBB15_9
LBB15_4:
	movq	24(%rbx), %rsi
	xorl	%eax, %eax
	testq	%rsi, %rsi
	jne	LBB15_5
LBB15_10:
	leaq	-40(%rbp), %rsi
	leaq	-41(%rbp), %rcx
	movq	%rcx, 16(%rbx)
	movq	%rsi, 24(%rbx)
	movq	%rsi, 32(%rbx)
	testb	%al, %al
	jne	LBB15_6
LBB15_11:
	movq	32(%rbx), %rax
	movq	%rax, %rcx
	subq	16(%rbx), %rcx
	movq	%rcx, %rdx
	shrq	$63, %rdx
	addq	%rcx, %rdx
	sarq	%rdx
	cmpq	$4, %rdx
	movl	$4, %r14d
	cmovbq	%rdx, %r14
	cmpq	%rax, %rsi
	jne	LBB15_7
LBB15_12:
	leaq	16(%rbx), %r12
	movq	16(%rbx), %rdi
	subq	%r14, %rsi
	movq	%r14, %rdx
	callq	_memmove
	cmpb	$0, 402(%rbx)
	je	LBB15_15
## BB#13:
	movq	16(%rbx), %rdi
	movq	32(%rbx), %rdx
	subq	%r14, %rdx
	subq	%rdi, %rdx
	addq	%r14, %rdi
	movq	120(%rbx), %rcx
	movl	$1, %esi
	callq	_fread
	testq	%rax, %rax
	je	LBB15_19
## BB#14:
	movq	16(%rbx), %rcx
	leaq	(%rcx,%r14), %rdx
	addq	%rdx, %rax
	movq	%rdx, 24(%rbx)
	movq	%rax, 32(%rbx)
	movzbl	(%rcx,%r14), %eax
	leaq	-41(%rbp), %rcx
	cmpq	%rcx, (%r12)
	jne	LBB15_9
	jmp	LBB15_8
LBB15_15:
	movq	72(%rbx), %rsi
	movq	80(%rbx), %rax
	movq	%rax, %rdx
	subq	%rsi, %rdx
	je	LBB15_17
## BB#16:
	movq	64(%rbx), %rdi
	callq	_memmove
	movq	72(%rbx), %rsi
	movq	80(%rbx), %rax
LBB15_17:
	subq	%rsi, %rax
	movq	64(%rbx), %rdx
	addq	%rdx, %rax
	movq	%rax, 72(%rbx)
	leaq	88(%rbx), %rcx
	cmpq	%rcx, %rdx
	je	LBB15_20
## BB#18:
	movq	96(%rbx), %rcx
	jmp	LBB15_21
LBB15_20:
	movl	$8, %ecx
LBB15_21:
	addq	%rcx, %rdx
	movq	%rdx, 80(%rbx)
	movq	112(%rbx), %rcx
	subq	%r14, %rcx
	subq	%rax, %rdx
	cmpq	%rcx, %rdx
	cmovaeq	%rcx, %rdx
	leaq	136(%rbx), %r15
	leaq	264(%rbx), %rdi
	movl	$16, %ecx
	movq	%r15, %rsi
	rep;movsq
	movq	120(%rbx), %rcx
	movl	$1, %esi
	movq	%rax, %rdi
	callq	_fread
	testq	%rax, %rax
	je	LBB15_19
## BB#22:
	movq	128(%rbx), %rdi
	testq	%rdi, %rdi
	je	LBB15_29
## BB#23:
	leaq	72(%rbx), %r8
	addq	72(%rbx), %rax
	movq	%rax, 80(%rbx)
	movq	16(%rbx), %r10
	movq	64(%rbx), %rdx
	leaq	(%r10,%r14), %r9
	addq	112(%rbx), %r10
	movq	(%rdi), %r11
	leaq	-56(%rbp), %r13
	movq	%r15, %rsi
	movq	%rax, %rcx
	pushq	%r13
	pushq	%r10
	callq	*32(%r11)
	addq	$16, %rsp
	cmpl	$3, %eax
	jne	LBB15_26
## BB#24:
	movq	64(%rbx), %r14
	movq	80(%rbx), %rax
	movq	%r14, 16(%rbx)
	jmp	LBB15_27
LBB15_26:
	movq	-56(%rbp), %rax
	addq	(%r12), %r14
	cmpq	%r14, %rax
	je	LBB15_19
LBB15_27:
	movq	%r14, 24(%rbx)
	movq	%rax, 32(%rbx)
	movzbl	(%r14), %eax
	leaq	-41(%rbp), %rcx
	cmpq	%rcx, (%r12)
	jne	LBB15_9
	jmp	LBB15_8
LBB15_19:
	movl	$-1, %eax
	leaq	-41(%rbp), %rcx
	cmpq	%rcx, (%r12)
	jne	LBB15_9
LBB15_8:
	movq	$0, 16(%rbx)
	movq	$0, 24(%rbx)
	movq	$0, 32(%rbx)
LBB15_9:
	addq	$24, %rsp
	popq	%rbx
	popq	%r12
	popq	%r13
	popq	%r14
	popq	%r15
	popq	%rbp
	retq
LBB15_29:
	movl	$8, %edi
	callq	___cxa_allocate_exception
	movq	%rax, %rbx
	movq	%rbx, %rdi
	callq	__ZNSt8bad_castC1Ev
	movq	__ZTISt8bad_cast@GOTPCREL(%rip), %rsi
	movq	__ZNSt8bad_castD1Ev@GOTPCREL(%rip), %rdx
	movq	%rbx, %rdi
	callq	___cxa_throw
	.cfi_endproc
                                        ## -- End function
	.globl	__ZNSt3__113basic_filebufIcNS_11char_traitsIcEEE9pbackfailEi ## -- Begin function _ZNSt3__113basic_filebufIcNS_11char_traitsIcEEE9pbackfailEi
	.weak_def_can_be_hidden	__ZNSt3__113basic_filebufIcNS_11char_traitsIcEEE9pbackfailEi
	.p2align	4, 0x90
__ZNSt3__113basic_filebufIcNS_11char_traitsIcEEE9pbackfailEi: ## @_ZNSt3__113basic_filebufIcNS_11char_traitsIcEEE9pbackfailEi
	.cfi_startproc
## BB#0:
	pushq	%rbp
Lcfi91:
	.cfi_def_cfa_offset 16
Lcfi92:
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
Lcfi93:
	.cfi_def_cfa_register %rbp
	movl	$-1, %eax
	cmpq	$0, 120(%rdi)
	je	LBB16_7
## BB#1:
	movq	24(%rdi), %rcx
	cmpq	%rcx, 16(%rdi)
	jae	LBB16_7
## BB#2:
	cmpl	$-1, %esi
	je	LBB16_3
## BB#4:
	testb	$16, 392(%rdi)
	jne	LBB16_6
## BB#5:
	cmpb	%sil, -1(%rcx)
	jne	LBB16_7
LBB16_6:
	decq	%rcx
	movq	%rcx, 24(%rdi)
	movb	%sil, (%rcx)
	movl	%esi, %eax
LBB16_7:
	popq	%rbp
	retq
LBB16_3:
	decq	%rcx
	movq	%rcx, 24(%rdi)
	xorl	%eax, %eax
	popq	%rbp
	retq
	.cfi_endproc
                                        ## -- End function
	.globl	__ZNSt3__113basic_filebufIcNS_11char_traitsIcEEE8overflowEi ## -- Begin function _ZNSt3__113basic_filebufIcNS_11char_traitsIcEEE8overflowEi
	.weak_def_can_be_hidden	__ZNSt3__113basic_filebufIcNS_11char_traitsIcEEE8overflowEi
	.p2align	4, 0x90
__ZNSt3__113basic_filebufIcNS_11char_traitsIcEEE8overflowEi: ## @_ZNSt3__113basic_filebufIcNS_11char_traitsIcEEE8overflowEi
	.cfi_startproc
## BB#0:
	pushq	%rbp
Lcfi94:
	.cfi_def_cfa_offset 16
Lcfi95:
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
Lcfi96:
	.cfi_def_cfa_register %rbp
	pushq	%r15
	pushq	%r14
	pushq	%r13
	pushq	%r12
	pushq	%rbx
	subq	$56, %rsp
Lcfi97:
	.cfi_offset %rbx, -56
Lcfi98:
	.cfi_offset %r12, -48
Lcfi99:
	.cfi_offset %r13, -40
Lcfi100:
	.cfi_offset %r14, -32
Lcfi101:
	.cfi_offset %r15, -24
	movq	%rdi, %r14
	cmpq	$0, 120(%r14)
	je	LBB17_30
## BB#1:
	testb	$16, 396(%r14)
	jne	LBB17_2
## BB#3:
	movq	$0, 32(%r14)
	movq	$0, 24(%r14)
	movq	$0, 16(%r14)
	movq	96(%r14), %rcx
	cmpq	$9, %rcx
	jb	LBB17_4
## BB#5:
	cmpb	$0, 402(%r14)
	je	LBB17_7
## BB#6:
	movq	64(%r14), %rax
	jmp	LBB17_8
LBB17_2:
	movq	40(%r14), %rax
	movq	56(%r14), %r12
	jmp	LBB17_10
LBB17_4:
	xorl	%eax, %eax
	xorl	%r12d, %r12d
	jmp	LBB17_9
LBB17_7:
	movq	104(%r14), %rax
	movq	112(%r14), %rcx
LBB17_8:
	leaq	-1(%rax,%rcx), %r12
LBB17_9:
	movq	%rax, 48(%r14)
	movq	%rax, 40(%r14)
	movq	%r12, 56(%r14)
	movl	$16, 396(%r14)
LBB17_10:
	movq	%rax, %r13
	cmpl	$-1, %esi
	movq	48(%r14), %rcx
	je	LBB17_14
## BB#11:
	testq	%rcx, %rcx
	jne	LBB17_13
## BB#12:
	leaq	-40(%rbp), %rax
	leaq	-41(%rbp), %rcx
	movq	%rcx, 48(%r14)
	movq	%rcx, 40(%r14)
	movq	%rax, 56(%r14)
LBB17_13:
	movb	%sil, (%rcx)
	movq	40(%r14), %rax
	movq	48(%r14), %rcx
	incq	%rcx
	movq	%rcx, 48(%r14)
LBB17_14:
	movq	%rcx, %r15
	subq	%rax, %r15
	je	LBB17_18
## BB#15:
	movl	%esi, -68(%rbp)         ## 4-byte Spill
	cmpb	$0, 402(%r14)
	je	LBB17_19
## BB#16:
	movq	120(%r14), %rcx
	movl	$1, %esi
	movq	%rax, %rdi
	movq	%r15, %rdx
	callq	_fwrite
	movq	%rax, %rcx
	movl	$-1, %eax
	cmpq	%r15, %rcx
	jne	LBB17_31
LBB17_17:
	movq	%r13, 48(%r14)
	movq	%r13, 40(%r14)
	movq	%r12, 56(%r14)
	movl	-68(%rbp), %esi         ## 4-byte Reload
LBB17_18:
	xorl	%eax, %eax
	cmpl	$-1, %esi
	cmovnel	%esi, %eax
	jmp	LBB17_31
LBB17_19:
	movq	%r13, -80(%rbp)         ## 8-byte Spill
	movq	%r12, -88(%rbp)         ## 8-byte Spill
	movq	64(%r14), %r9
	movq	%r9, -64(%rbp)
	movq	128(%r14), %rdi
	testq	%rdi, %rdi
	je	LBB17_32
## BB#20:
	leaq	136(%r14), %r15
	movq	96(%r14), %r10
	addq	%r9, %r10
	movq	(%rdi), %r11
	leaq	-64(%rbp), %rbx
	leaq	-56(%rbp), %r8
	movq	%r15, %rsi
	movq	%rax, %rdx
	pushq	%rbx
	pushq	%r10
	callq	*24(%r11)
	jmp	LBB17_21
	.p2align	4, 0x90
LBB17_29:                               ##   in Loop: Header=BB17_21 Depth=1
	movq	64(%r14), %r9
	movq	96(%r14), %rax
	addq	%r9, %rax
	movq	(%rdi), %r10
	movq	%r15, %rsi
	leaq	-56(%rbp), %r8
	leaq	-64(%rbp), %rbx
	pushq	%rbx
	pushq	%rax
	callq	*24(%r10)
LBB17_21:                               ## =>This Inner Loop Header: Depth=1
	addq	$16, %rsp
	movl	%eax, %r13d
	movq	40(%r14), %rdi
	cmpq	%rdi, -56(%rbp)
	je	LBB17_30
## BB#22:                               ##   in Loop: Header=BB17_21 Depth=1
	cmpl	$3, %r13d
	je	LBB17_23
## BB#25:                               ##   in Loop: Header=BB17_21 Depth=1
	cmpl	$1, %r13d
	ja	LBB17_30
## BB#26:                               ##   in Loop: Header=BB17_21 Depth=1
	movq	-64(%rbp), %r12
	movq	64(%r14), %rdi
	movq	120(%r14), %rcx
	subq	%rdi, %r12
	movl	$1, %esi
	movq	%r12, %rdx
	callq	_fwrite
	cmpq	%r12, %rax
	jne	LBB17_30
## BB#27:                               ##   in Loop: Header=BB17_21 Depth=1
	cmpl	$1, %r13d
	jne	LBB17_24
## BB#28:                               ##   in Loop: Header=BB17_21 Depth=1
	movq	-56(%rbp), %rdx
	movq	48(%r14), %rax
	movq	%rdx, 40(%r14)
	movq	%rax, 56(%r14)
	subl	%edx, %eax
	movslq	%eax, %rcx
	addq	%rdx, %rcx
	movq	%rcx, 48(%r14)
	movq	128(%r14), %rdi
	testq	%rdi, %rdi
	jne	LBB17_29
LBB17_32:
	movl	$8, %edi
	callq	___cxa_allocate_exception
	movq	%rax, %rbx
	movq	%rbx, %rdi
	callq	__ZNSt8bad_castC1Ev
	movq	__ZTISt8bad_cast@GOTPCREL(%rip), %rsi
	movq	__ZNSt8bad_castD1Ev@GOTPCREL(%rip), %rdx
	movq	%rbx, %rdi
	callq	___cxa_throw
LBB17_23:
	movq	48(%r14), %r15
	movq	120(%r14), %rcx
	subq	%rdi, %r15
	movl	$1, %esi
	movq	%r15, %rdx
	callq	_fwrite
	cmpq	%r15, %rax
	jne	LBB17_30
LBB17_24:
	movq	-88(%rbp), %r12         ## 8-byte Reload
	movq	-80(%rbp), %r13         ## 8-byte Reload
	jmp	LBB17_17
LBB17_30:
	movl	$-1, %eax
LBB17_31:
	addq	$56, %rsp
	popq	%rbx
	popq	%r12
	popq	%r13
	popq	%r14
	popq	%r15
	popq	%rbp
	retq
	.cfi_endproc
                                        ## -- End function
	.globl	__ZNSt3__113basic_filebufIcNS_11char_traitsIcEEEC2Ev ## -- Begin function _ZNSt3__113basic_filebufIcNS_11char_traitsIcEEEC2Ev
	.weak_def_can_be_hidden	__ZNSt3__113basic_filebufIcNS_11char_traitsIcEEEC2Ev
	.p2align	4, 0x90
__ZNSt3__113basic_filebufIcNS_11char_traitsIcEEEC2Ev: ## @_ZNSt3__113basic_filebufIcNS_11char_traitsIcEEEC2Ev
Lfunc_begin3:
	.cfi_startproc
	.cfi_personality 155, ___gxx_personality_v0
	.cfi_lsda 16, Lexception3
## BB#0:
	pushq	%rbp
Lcfi102:
	.cfi_def_cfa_offset 16
Lcfi103:
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
Lcfi104:
	.cfi_def_cfa_register %rbp
	pushq	%r15
	pushq	%r14
	pushq	%rbx
	pushq	%rax
Lcfi105:
	.cfi_offset %rbx, -40
Lcfi106:
	.cfi_offset %r14, -32
Lcfi107:
	.cfi_offset %r15, -24
	movq	%rdi, %rbx
	callq	__ZNSt3__115basic_streambufIcNS_11char_traitsIcEEEC2Ev
	movq	__ZTVNSt3__113basic_filebufIcNS_11char_traitsIcEEEE@GOTPCREL(%rip), %rax
	addq	$16, %rax
	movq	%rax, (%rbx)
	leaq	96(%rbx), %rdi
	movq	$0, 80(%rbx)
	movq	$0, 72(%rbx)
	movq	$0, 64(%rbx)
	movl	$307, %esi              ## imm = 0x133
	callq	___bzero
	leaq	8(%rbx), %r14
	leaq	-32(%rbp), %r15
	movq	%r15, %rdi
	movq	%r14, %rsi
	callq	__ZNSt3__16localeC1ERKS0_
Ltmp33:
	movq	__ZNSt3__17codecvtIcc11__mbstate_tE2idE@GOTPCREL(%rip), %rsi
	movq	%r15, %rdi
	callq	__ZNKSt3__16locale9has_facetERNS0_2idE
	movl	%eax, %r15d
Ltmp34:
## BB#1:
	leaq	-32(%rbp), %rdi
	callq	__ZNSt3__16localeD1Ev
	testb	%r15b, %r15b
	je	LBB18_4
## BB#2:
	leaq	-32(%rbp), %r15
	movq	%r15, %rdi
	movq	%r14, %rsi
	callq	__ZNSt3__16localeC1ERKS0_
Ltmp36:
	movq	__ZNSt3__17codecvtIcc11__mbstate_tE2idE@GOTPCREL(%rip), %rsi
	movq	%r15, %rdi
	callq	__ZNKSt3__16locale9use_facetERNS0_2idE
Ltmp37:
## BB#3:
	movq	%rax, 128(%rbx)
	leaq	-32(%rbp), %rdi
	callq	__ZNSt3__16localeD1Ev
	movq	128(%rbx), %rdi
	movq	(%rdi), %rax
	callq	*56(%rax)
	movb	%al, 402(%rbx)
LBB18_4:
	movq	(%rbx), %rax
Ltmp39:
	xorl	%esi, %esi
	movl	$4096, %edx             ## imm = 0x1000
	movq	%rbx, %rdi
	callq	*24(%rax)
Ltmp40:
## BB#5:
	addq	$8, %rsp
	popq	%rbx
	popq	%r14
	popq	%r15
	popq	%rbp
	retq
LBB18_9:
Ltmp38:
	movq	%rax, %r14
	leaq	-32(%rbp), %rdi
	callq	__ZNSt3__16localeD1Ev
	jmp	LBB18_7
LBB18_6:
Ltmp41:
	movq	%rax, %r14
LBB18_7:
	movq	%rbx, %rdi
	callq	__ZNSt3__115basic_streambufIcNS_11char_traitsIcEEED2Ev
	movq	%r14, %rdi
	callq	__Unwind_Resume
LBB18_8:
Ltmp35:
	movq	%rax, %rdi
	callq	___clang_call_terminate
Lfunc_end3:
	.cfi_endproc
	.section	__TEXT,__gcc_except_tab
	.p2align	2
GCC_except_table18:
Lexception3:
	.byte	255                     ## @LPStart Encoding = omit
	.byte	155                     ## @TType Encoding = indirect pcrel sdata4
	.asciz	"\326\200\200"          ## @TType base offset
	.byte	3                       ## Call site Encoding = udata4
	.byte	78                      ## Call site table length
Lset38 = Lfunc_begin3-Lfunc_begin3      ## >> Call Site 1 <<
	.long	Lset38
Lset39 = Ltmp33-Lfunc_begin3            ##   Call between Lfunc_begin3 and Ltmp33
	.long	Lset39
	.long	0                       ##     has no landing pad
	.byte	0                       ##   On action: cleanup
Lset40 = Ltmp33-Lfunc_begin3            ## >> Call Site 2 <<
	.long	Lset40
Lset41 = Ltmp34-Ltmp33                  ##   Call between Ltmp33 and Ltmp34
	.long	Lset41
Lset42 = Ltmp35-Lfunc_begin3            ##     jumps to Ltmp35
	.long	Lset42
	.byte	1                       ##   On action: 1
Lset43 = Ltmp36-Lfunc_begin3            ## >> Call Site 3 <<
	.long	Lset43
Lset44 = Ltmp37-Ltmp36                  ##   Call between Ltmp36 and Ltmp37
	.long	Lset44
Lset45 = Ltmp38-Lfunc_begin3            ##     jumps to Ltmp38
	.long	Lset45
	.byte	0                       ##   On action: cleanup
Lset46 = Ltmp37-Lfunc_begin3            ## >> Call Site 4 <<
	.long	Lset46
Lset47 = Ltmp39-Ltmp37                  ##   Call between Ltmp37 and Ltmp39
	.long	Lset47
	.long	0                       ##     has no landing pad
	.byte	0                       ##   On action: cleanup
Lset48 = Ltmp39-Lfunc_begin3            ## >> Call Site 5 <<
	.long	Lset48
Lset49 = Ltmp40-Ltmp39                  ##   Call between Ltmp39 and Ltmp40
	.long	Lset49
Lset50 = Ltmp41-Lfunc_begin3            ##     jumps to Ltmp41
	.long	Lset50
	.byte	0                       ##   On action: cleanup
Lset51 = Ltmp40-Lfunc_begin3            ## >> Call Site 6 <<
	.long	Lset51
Lset52 = Lfunc_end3-Ltmp40              ##   Call between Ltmp40 and Lfunc_end3
	.long	Lset52
	.long	0                       ##     has no landing pad
	.byte	0                       ##   On action: cleanup
	.byte	1                       ## >> Action Record 1 <<
                                        ##   Catch TypeInfo 1
	.byte	0                       ##   No further actions
                                        ## >> Catch TypeInfos <<
	.long	0                       ## TypeInfo 1
	.p2align	2
                                        ## -- End function
	.section	__TEXT,__text,regular,pure_instructions
	.globl	__ZNSt3__17getlineIcNS_11char_traitsIcEENS_9allocatorIcEEEERNS_13basic_istreamIT_T0_EES9_RNS_12basic_stringIS6_S7_T1_EES6_ ## -- Begin function _ZNSt3__17getlineIcNS_11char_traitsIcEENS_9allocatorIcEEEERNS_13basic_istreamIT_T0_EES9_RNS_12basic_stringIS6_S7_T1_EES6_
	.weak_def_can_be_hidden	__ZNSt3__17getlineIcNS_11char_traitsIcEENS_9allocatorIcEEEERNS_13basic_istreamIT_T0_EES9_RNS_12basic_stringIS6_S7_T1_EES6_
	.p2align	4, 0x90
__ZNSt3__17getlineIcNS_11char_traitsIcEENS_9allocatorIcEEEERNS_13basic_istreamIT_T0_EES9_RNS_12basic_stringIS6_S7_T1_EES6_: ## @_ZNSt3__17getlineIcNS_11char_traitsIcEENS_9allocatorIcEEEERNS_13basic_istreamIT_T0_EES9_RNS_12basic_stringIS6_S7_T1_EES6_
Lfunc_begin4:
	.cfi_startproc
	.cfi_personality 155, ___gxx_personality_v0
	.cfi_lsda 16, Lexception4
## BB#0:
	pushq	%rbp
Lcfi108:
	.cfi_def_cfa_offset 16
Lcfi109:
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
Lcfi110:
	.cfi_def_cfa_register %rbp
	pushq	%r15
	pushq	%r14
	pushq	%r12
	pushq	%rbx
	subq	$16, %rsp
Lcfi111:
	.cfi_offset %rbx, -48
Lcfi112:
	.cfi_offset %r12, -40
Lcfi113:
	.cfi_offset %r14, -32
Lcfi114:
	.cfi_offset %r15, -24
	movl	%edx, %r14d
	movq	%rsi, %rbx
	movq	%rdi, %r15
Ltmp42:
	leaq	-40(%rbp), %rdi
	movl	$1, %edx
	movq	%r15, %rsi
	callq	__ZNSt3__113basic_istreamIcNS_11char_traitsIcEEE6sentryC1ERS3_b
Ltmp43:
## BB#1:
	cmpb	$0, -40(%rbp)
	je	LBB19_23
## BB#2:
	testb	$1, (%rbx)
	jne	LBB19_3
## BB#4:
	movw	$0, (%rbx)
	jmp	LBB19_5
LBB19_3:
	movq	16(%rbx), %rax
	movb	$0, (%rax)
	movq	$0, 8(%rbx)
LBB19_5:
	xorl	%eax, %eax
	.p2align	4, 0x90
LBB19_6:                                ## =>This Inner Loop Header: Depth=1
	movq	%rax, %r12
	movq	(%r15), %rax
	movq	-24(%rax), %rax
	movq	40(%r15,%rax), %rdi
	movq	24(%rdi), %rax
	cmpq	32(%rdi), %rax
	je	LBB19_7
## BB#14:                               ##   in Loop: Header=BB19_6 Depth=1
	leaq	1(%rax), %rcx
	movq	%rcx, 24(%rdi)
	movzbl	(%rax), %eax
	cmpb	%r14b, %al
	jne	LBB19_17
	jmp	LBB19_16
	.p2align	4, 0x90
LBB19_7:                                ##   in Loop: Header=BB19_6 Depth=1
	movq	(%rdi), %rax
Ltmp45:
	callq	*80(%rax)
Ltmp46:
## BB#8:                                ##   in Loop: Header=BB19_6 Depth=1
	cmpl	$-1, %eax
	je	LBB19_9
## BB#15:                               ##   in Loop: Header=BB19_6 Depth=1
	cmpb	%r14b, %al
	je	LBB19_16
LBB19_17:                               ##   in Loop: Header=BB19_6 Depth=1
Ltmp48:
	movsbl	%al, %esi
	movq	%rbx, %rdi
	callq	__ZNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEE9push_backEc
Ltmp49:
## BB#18:                               ##   in Loop: Header=BB19_6 Depth=1
	leaq	1(%r12), %rax
	testb	$1, (%rbx)
	je	LBB19_6
## BB#19:                               ##   in Loop: Header=BB19_6 Depth=1
	cmpq	$-17, 8(%rbx)
	jne	LBB19_6
## BB#20:
	movl	$4, %ecx
	jmp	LBB19_21
LBB19_16:
	xorl	%ecx, %ecx
LBB19_21:
	incq	%r12
LBB19_22:
	movl	%ecx, %esi
	orl	$4, %esi
	testq	%r12, %r12
	cmovnel	%ecx, %esi
	movq	(%r15), %rax
	movq	-24(%rax), %rax
	leaq	(%r15,%rax), %rdi
	orl	32(%r15,%rax), %esi
Ltmp51:
	callq	__ZNSt3__18ios_base5clearEj
Ltmp52:
	jmp	LBB19_23
LBB19_9:
	movl	$2, %ecx
	jmp	LBB19_22
LBB19_28:
Ltmp53:
	jmp	LBB19_11
LBB19_10:
Ltmp44:
	jmp	LBB19_11
LBB19_13:
Ltmp47:
	jmp	LBB19_11
LBB19_27:
Ltmp50:
LBB19_11:
	movq	%rax, %rdi
	movq	%r15, %rbx
	callq	___cxa_begin_catch
	movq	(%r15), %rax
	addq	-24(%rax), %rbx
Ltmp54:
	movq	%rbx, %rdi
	callq	__ZNSt3__18ios_base33__set_badbit_and_consider_rethrowEv
Ltmp55:
## BB#12:
	callq	___cxa_end_catch
LBB19_23:
	movq	%r15, %rax
	addq	$16, %rsp
	popq	%rbx
	popq	%r12
	popq	%r14
	popq	%r15
	popq	%rbp
	retq
LBB19_24:
Ltmp56:
	movq	%rax, %rbx
Ltmp57:
	callq	___cxa_end_catch
Ltmp58:
## BB#25:
	movq	%rbx, %rdi
	callq	__Unwind_Resume
LBB19_26:
Ltmp59:
	movq	%rax, %rdi
	callq	___clang_call_terminate
Lfunc_end4:
	.cfi_endproc
	.section	__TEXT,__gcc_except_tab
	.p2align	2
GCC_except_table19:
Lexception4:
	.byte	255                     ## @LPStart Encoding = omit
	.byte	155                     ## @TType Encoding = indirect pcrel sdata4
	.byte	125                     ## @TType base offset
	.byte	3                       ## Call site Encoding = udata4
	.byte	117                     ## Call site table length
Lset53 = Ltmp42-Lfunc_begin4            ## >> Call Site 1 <<
	.long	Lset53
Lset54 = Ltmp43-Ltmp42                  ##   Call between Ltmp42 and Ltmp43
	.long	Lset54
Lset55 = Ltmp44-Lfunc_begin4            ##     jumps to Ltmp44
	.long	Lset55
	.byte	1                       ##   On action: 1
Lset56 = Ltmp45-Lfunc_begin4            ## >> Call Site 2 <<
	.long	Lset56
Lset57 = Ltmp46-Ltmp45                  ##   Call between Ltmp45 and Ltmp46
	.long	Lset57
Lset58 = Ltmp47-Lfunc_begin4            ##     jumps to Ltmp47
	.long	Lset58
	.byte	1                       ##   On action: 1
Lset59 = Ltmp48-Lfunc_begin4            ## >> Call Site 3 <<
	.long	Lset59
Lset60 = Ltmp49-Ltmp48                  ##   Call between Ltmp48 and Ltmp49
	.long	Lset60
Lset61 = Ltmp50-Lfunc_begin4            ##     jumps to Ltmp50
	.long	Lset61
	.byte	1                       ##   On action: 1
Lset62 = Ltmp51-Lfunc_begin4            ## >> Call Site 4 <<
	.long	Lset62
Lset63 = Ltmp52-Ltmp51                  ##   Call between Ltmp51 and Ltmp52
	.long	Lset63
Lset64 = Ltmp53-Lfunc_begin4            ##     jumps to Ltmp53
	.long	Lset64
	.byte	1                       ##   On action: 1
Lset65 = Ltmp52-Lfunc_begin4            ## >> Call Site 5 <<
	.long	Lset65
Lset66 = Ltmp54-Ltmp52                  ##   Call between Ltmp52 and Ltmp54
	.long	Lset66
	.long	0                       ##     has no landing pad
	.byte	0                       ##   On action: cleanup
Lset67 = Ltmp54-Lfunc_begin4            ## >> Call Site 6 <<
	.long	Lset67
Lset68 = Ltmp55-Ltmp54                  ##   Call between Ltmp54 and Ltmp55
	.long	Lset68
Lset69 = Ltmp56-Lfunc_begin4            ##     jumps to Ltmp56
	.long	Lset69
	.byte	0                       ##   On action: cleanup
Lset70 = Ltmp55-Lfunc_begin4            ## >> Call Site 7 <<
	.long	Lset70
Lset71 = Ltmp57-Ltmp55                  ##   Call between Ltmp55 and Ltmp57
	.long	Lset71
	.long	0                       ##     has no landing pad
	.byte	0                       ##   On action: cleanup
Lset72 = Ltmp57-Lfunc_begin4            ## >> Call Site 8 <<
	.long	Lset72
Lset73 = Ltmp58-Ltmp57                  ##   Call between Ltmp57 and Ltmp58
	.long	Lset73
Lset74 = Ltmp59-Lfunc_begin4            ##     jumps to Ltmp59
	.long	Lset74
	.byte	1                       ##   On action: 1
Lset75 = Ltmp58-Lfunc_begin4            ## >> Call Site 9 <<
	.long	Lset75
Lset76 = Lfunc_end4-Ltmp58              ##   Call between Ltmp58 and Lfunc_end4
	.long	Lset76
	.long	0                       ##     has no landing pad
	.byte	0                       ##   On action: cleanup
	.byte	1                       ## >> Action Record 1 <<
                                        ##   Catch TypeInfo 1
	.byte	0                       ##   No further actions
                                        ## >> Catch TypeInfos <<
	.long	0                       ## TypeInfo 1
	.p2align	2
                                        ## -- End function
	.section	__TEXT,__text,regular,pure_instructions
	.globl	__ZNSt3__16vectorIPKcNS_9allocatorIS2_EEE21__push_back_slow_pathIS2_EEvOT_ ## -- Begin function _ZNSt3__16vectorIPKcNS_9allocatorIS2_EEE21__push_back_slow_pathIS2_EEvOT_
	.weak_def_can_be_hidden	__ZNSt3__16vectorIPKcNS_9allocatorIS2_EEE21__push_back_slow_pathIS2_EEvOT_
	.p2align	4, 0x90
__ZNSt3__16vectorIPKcNS_9allocatorIS2_EEE21__push_back_slow_pathIS2_EEvOT_: ## @_ZNSt3__16vectorIPKcNS_9allocatorIS2_EEE21__push_back_slow_pathIS2_EEvOT_
Lfunc_begin5:
	.cfi_startproc
	.cfi_personality 155, ___gxx_personality_v0
	.cfi_lsda 16, Lexception5
## BB#0:
	pushq	%rbp
Lcfi115:
	.cfi_def_cfa_offset 16
Lcfi116:
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
Lcfi117:
	.cfi_def_cfa_register %rbp
	pushq	%r15
	pushq	%r14
	pushq	%r13
	pushq	%r12
	pushq	%rbx
	subq	$24, %rsp
Lcfi118:
	.cfi_offset %rbx, -56
Lcfi119:
	.cfi_offset %r12, -48
Lcfi120:
	.cfi_offset %r13, -40
Lcfi121:
	.cfi_offset %r14, -32
Lcfi122:
	.cfi_offset %r15, -24
	movq	%rsi, %r8
	movq	%rdi, %r13
	movq	(%r13), %rsi
	movq	8(%r13), %r12
	subq	%rsi, %r12
	movq	%r12, %r15
	sarq	$3, %r15
	leaq	1(%r15), %rax
	movq	%rax, %rcx
	shrq	$61, %rcx
	jne	LBB20_15
## BB#1:
	movabsq	$2305843009213693951, %rcx ## imm = 0x1FFFFFFFFFFFFFFF
	movq	16(%r13), %r14
	subq	%rsi, %r14
	movq	%r14, %rdx
	sarq	$3, %rdx
	movabsq	$1152921504606846974, %rdi ## imm = 0xFFFFFFFFFFFFFFE
	cmpq	%rdi, %rdx
	ja	LBB20_2
## BB#8:
	sarq	$2, %r14
	cmpq	%rax, %r14
	cmovbq	%rax, %r14
	testq	%r14, %r14
	je	LBB20_9
## BB#10:
	movq	%r8, -48(%rbp)          ## 8-byte Spill
	movq	%rsi, -56(%rbp)         ## 8-byte Spill
	cmpq	%rcx, %r14
	jbe	LBB20_3
## BB#11:
	movl	$16, %edi
	callq	___cxa_allocate_exception
	movq	%rax, %rbx
Ltmp60:
	leaq	L_.str.15(%rip), %rsi
	movq	%rbx, %rdi
	callq	__ZNSt11logic_errorC2EPKc
Ltmp61:
## BB#12:
	movq	__ZTVSt12length_error@GOTPCREL(%rip), %rax
	addq	$16, %rax
	movq	%rax, (%rbx)
	movq	__ZTISt12length_error@GOTPCREL(%rip), %rsi
	movq	__ZNSt12length_errorD1Ev@GOTPCREL(%rip), %rdx
	movq	%rbx, %rdi
	callq	___cxa_throw
LBB20_2:
	movq	%r8, -48(%rbp)          ## 8-byte Spill
	movq	%rsi, -56(%rbp)         ## 8-byte Spill
	movq	%rcx, %r14
LBB20_3:
	leaq	(,%r14,8), %rdi
	callq	__Znwm
	movq	%rax, %rbx
	movq	-56(%rbp), %rsi         ## 8-byte Reload
	movq	-48(%rbp), %r8          ## 8-byte Reload
LBB20_4:
	leaq	(%rbx,%r14,8), %r14
	movq	(%r8), %rax
	movq	%rax, (%rbx,%r15,8)
	leaq	8(%rbx,%r15,8), %r15
	testq	%r12, %r12
	jle	LBB20_6
## BB#5:
	movq	%rbx, %rdi
	movq	%r12, %rdx
	movq	%rsi, %r12
	callq	_memcpy
	movq	%r12, %rsi
LBB20_6:
	movq	%rbx, (%r13)
	movq	%r15, 8(%r13)
	movq	%r14, 16(%r13)
	testq	%rsi, %rsi
	je	LBB20_14
## BB#7:
	movq	%rsi, %rdi
	addq	$24, %rsp
	popq	%rbx
	popq	%r12
	popq	%r13
	popq	%r14
	popq	%r15
	popq	%rbp
	jmp	__ZdlPv                 ## TAILCALL
LBB20_14:
	addq	$24, %rsp
	popq	%rbx
	popq	%r12
	popq	%r13
	popq	%r14
	popq	%r15
	popq	%rbp
	retq
LBB20_9:
	xorl	%r14d, %r14d
	xorl	%ebx, %ebx
	jmp	LBB20_4
LBB20_15:
	movq	%r13, %rdi
	callq	__ZNKSt3__120__vector_base_commonILb1EE20__throw_length_errorEv
LBB20_13:
Ltmp62:
	movq	%rax, %r14
	movq	%rbx, %rdi
	callq	___cxa_free_exception
	movq	%r14, %rdi
	callq	__Unwind_Resume
Lfunc_end5:
	.cfi_endproc
	.section	__TEXT,__gcc_except_tab
	.p2align	2
GCC_except_table20:
Lexception5:
	.byte	255                     ## @LPStart Encoding = omit
	.byte	155                     ## @TType Encoding = indirect pcrel sdata4
	.byte	41                      ## @TType base offset
	.byte	3                       ## Call site Encoding = udata4
	.byte	39                      ## Call site table length
Lset77 = Lfunc_begin5-Lfunc_begin5      ## >> Call Site 1 <<
	.long	Lset77
Lset78 = Ltmp60-Lfunc_begin5            ##   Call between Lfunc_begin5 and Ltmp60
	.long	Lset78
	.long	0                       ##     has no landing pad
	.byte	0                       ##   On action: cleanup
Lset79 = Ltmp60-Lfunc_begin5            ## >> Call Site 2 <<
	.long	Lset79
Lset80 = Ltmp61-Ltmp60                  ##   Call between Ltmp60 and Ltmp61
	.long	Lset80
Lset81 = Ltmp62-Lfunc_begin5            ##     jumps to Ltmp62
	.long	Lset81
	.byte	0                       ##   On action: cleanup
Lset82 = Ltmp61-Lfunc_begin5            ## >> Call Site 3 <<
	.long	Lset82
Lset83 = Lfunc_end5-Ltmp61              ##   Call between Ltmp61 and Lfunc_end5
	.long	Lset83
	.long	0                       ##     has no landing pad
	.byte	0                       ##   On action: cleanup
	.p2align	2
                                        ## -- End function
	.section	__TEXT,__cstring,cstring_literals
L_.str:                                 ## @.str
	.asciz	"words.txt"

L_.str.1:                               ## @.str.1
	.asciz	"shuffled_words.txt"

L_.str.2:                               ## @.str.2
	.asciz	"%.2f\n"

	.section	__DATA,__data
	.globl	__ZTVNSt3__114basic_ifstreamIcNS_11char_traitsIcEEEE ## @_ZTVNSt3__114basic_ifstreamIcNS_11char_traitsIcEEEE
	.weak_def_can_be_hidden	__ZTVNSt3__114basic_ifstreamIcNS_11char_traitsIcEEEE
	.p2align	3
__ZTVNSt3__114basic_ifstreamIcNS_11char_traitsIcEEEE:
	.quad	424
	.quad	0
	.quad	__ZTINSt3__114basic_ifstreamIcNS_11char_traitsIcEEEE
	.quad	__ZNSt3__114basic_ifstreamIcNS_11char_traitsIcEEED1Ev
	.quad	__ZNSt3__114basic_ifstreamIcNS_11char_traitsIcEEED0Ev
	.quad	-424
	.quad	-424
	.quad	__ZTINSt3__114basic_ifstreamIcNS_11char_traitsIcEEEE
	.quad	__ZTv0_n24_NSt3__114basic_ifstreamIcNS_11char_traitsIcEEED1Ev
	.quad	__ZTv0_n24_NSt3__114basic_ifstreamIcNS_11char_traitsIcEEED0Ev

	.globl	__ZTTNSt3__114basic_ifstreamIcNS_11char_traitsIcEEEE ## @_ZTTNSt3__114basic_ifstreamIcNS_11char_traitsIcEEEE
	.weak_def_can_be_hidden	__ZTTNSt3__114basic_ifstreamIcNS_11char_traitsIcEEEE
	.p2align	4
__ZTTNSt3__114basic_ifstreamIcNS_11char_traitsIcEEEE:
	.quad	__ZTVNSt3__114basic_ifstreamIcNS_11char_traitsIcEEEE+24
	.quad	__ZTCNSt3__114basic_ifstreamIcNS_11char_traitsIcEEEE0_NS_13basic_istreamIcS2_EE+24
	.quad	__ZTCNSt3__114basic_ifstreamIcNS_11char_traitsIcEEEE0_NS_13basic_istreamIcS2_EE+64
	.quad	__ZTVNSt3__114basic_ifstreamIcNS_11char_traitsIcEEEE+64

	.globl	__ZTCNSt3__114basic_ifstreamIcNS_11char_traitsIcEEEE0_NS_13basic_istreamIcS2_EE ## @_ZTCNSt3__114basic_ifstreamIcNS_11char_traitsIcEEEE0_NS_13basic_istreamIcS2_EE
	.weak_def_can_be_hidden	__ZTCNSt3__114basic_ifstreamIcNS_11char_traitsIcEEEE0_NS_13basic_istreamIcS2_EE
	.p2align	4
__ZTCNSt3__114basic_ifstreamIcNS_11char_traitsIcEEEE0_NS_13basic_istreamIcS2_EE:
	.quad	424
	.quad	0
	.quad	__ZTINSt3__113basic_istreamIcNS_11char_traitsIcEEEE
	.quad	__ZNSt3__113basic_istreamIcNS_11char_traitsIcEEED1Ev
	.quad	__ZNSt3__113basic_istreamIcNS_11char_traitsIcEEED0Ev
	.quad	-424
	.quad	-424
	.quad	__ZTINSt3__113basic_istreamIcNS_11char_traitsIcEEEE
	.quad	__ZTv0_n24_NSt3__113basic_istreamIcNS_11char_traitsIcEEED1Ev
	.quad	__ZTv0_n24_NSt3__113basic_istreamIcNS_11char_traitsIcEEED0Ev

	.section	__TEXT,__const
	.globl	__ZTSNSt3__114basic_ifstreamIcNS_11char_traitsIcEEEE ## @_ZTSNSt3__114basic_ifstreamIcNS_11char_traitsIcEEEE
	.weak_definition	__ZTSNSt3__114basic_ifstreamIcNS_11char_traitsIcEEEE
	.p2align	4
__ZTSNSt3__114basic_ifstreamIcNS_11char_traitsIcEEEE:
	.asciz	"NSt3__114basic_ifstreamIcNS_11char_traitsIcEEEE"

	.section	__DATA,__data
	.globl	__ZTINSt3__114basic_ifstreamIcNS_11char_traitsIcEEEE ## @_ZTINSt3__114basic_ifstreamIcNS_11char_traitsIcEEEE
	.weak_definition	__ZTINSt3__114basic_ifstreamIcNS_11char_traitsIcEEEE
	.p2align	4
__ZTINSt3__114basic_ifstreamIcNS_11char_traitsIcEEEE:
	.quad	__ZTVN10__cxxabiv120__si_class_type_infoE+16
	.quad	__ZTSNSt3__114basic_ifstreamIcNS_11char_traitsIcEEEE
	.quad	__ZTINSt3__113basic_istreamIcNS_11char_traitsIcEEEE

	.globl	__ZTVNSt3__113basic_filebufIcNS_11char_traitsIcEEEE ## @_ZTVNSt3__113basic_filebufIcNS_11char_traitsIcEEEE
	.weak_def_can_be_hidden	__ZTVNSt3__113basic_filebufIcNS_11char_traitsIcEEEE
	.p2align	3
__ZTVNSt3__113basic_filebufIcNS_11char_traitsIcEEEE:
	.quad	0
	.quad	__ZTINSt3__113basic_filebufIcNS_11char_traitsIcEEEE
	.quad	__ZNSt3__113basic_filebufIcNS_11char_traitsIcEEED1Ev
	.quad	__ZNSt3__113basic_filebufIcNS_11char_traitsIcEEED0Ev
	.quad	__ZNSt3__113basic_filebufIcNS_11char_traitsIcEEE5imbueERKNS_6localeE
	.quad	__ZNSt3__113basic_filebufIcNS_11char_traitsIcEEE6setbufEPcl
	.quad	__ZNSt3__113basic_filebufIcNS_11char_traitsIcEEE7seekoffExNS_8ios_base7seekdirEj
	.quad	__ZNSt3__113basic_filebufIcNS_11char_traitsIcEEE7seekposENS_4fposI11__mbstate_tEEj
	.quad	__ZNSt3__113basic_filebufIcNS_11char_traitsIcEEE4syncEv
	.quad	__ZNSt3__115basic_streambufIcNS_11char_traitsIcEEE9showmanycEv
	.quad	__ZNSt3__115basic_streambufIcNS_11char_traitsIcEEE6xsgetnEPcl
	.quad	__ZNSt3__113basic_filebufIcNS_11char_traitsIcEEE9underflowEv
	.quad	__ZNSt3__115basic_streambufIcNS_11char_traitsIcEEE5uflowEv
	.quad	__ZNSt3__113basic_filebufIcNS_11char_traitsIcEEE9pbackfailEi
	.quad	__ZNSt3__115basic_streambufIcNS_11char_traitsIcEEE6xsputnEPKcl
	.quad	__ZNSt3__113basic_filebufIcNS_11char_traitsIcEEE8overflowEi

	.section	__TEXT,__const
	.globl	__ZTSNSt3__113basic_filebufIcNS_11char_traitsIcEEEE ## @_ZTSNSt3__113basic_filebufIcNS_11char_traitsIcEEEE
	.weak_definition	__ZTSNSt3__113basic_filebufIcNS_11char_traitsIcEEEE
	.p2align	4
__ZTSNSt3__113basic_filebufIcNS_11char_traitsIcEEEE:
	.asciz	"NSt3__113basic_filebufIcNS_11char_traitsIcEEEE"

	.section	__DATA,__data
	.globl	__ZTINSt3__113basic_filebufIcNS_11char_traitsIcEEEE ## @_ZTINSt3__113basic_filebufIcNS_11char_traitsIcEEEE
	.weak_definition	__ZTINSt3__113basic_filebufIcNS_11char_traitsIcEEEE
	.p2align	4
__ZTINSt3__113basic_filebufIcNS_11char_traitsIcEEEE:
	.quad	__ZTVN10__cxxabiv120__si_class_type_infoE+16
	.quad	__ZTSNSt3__113basic_filebufIcNS_11char_traitsIcEEEE
	.quad	__ZTINSt3__115basic_streambufIcNS_11char_traitsIcEEEE

	.section	__TEXT,__cstring,cstring_literals
L_.str.5:                               ## @.str.5
	.asciz	"r"

L_.str.15:                              ## @.str.15
	.asciz	"allocator<T>::allocate(size_t n) 'n' exceeds maximum supported size"


.subsections_via_symbols
