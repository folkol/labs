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
	movq	%rax, -640(%rbp)        ## 8-byte Spill
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
LBB0_10:
	movq	-624(%rbp), %rax
	movq	-24(%rax), %rax
	leaq	-624(%rbp,%rax), %rdi
	movl	-592(%rbp,%rax), %esi
	orl	$4, %esi
Ltmp8:
	callq	__ZNSt3__18ios_base5clearEj
Ltmp9:
	jmp	LBB0_11
LBB0_3:
Ltmp6:
	leaq	L_.str.6(%rip), %rsi
	movq	%r15, %rdi
	callq	_fopen
Ltmp7:
## BB#4:
	movq	%rax, -488(%rbp)
	testq	%rax, %rax
	je	LBB0_10
## BB#5:
	movl	$8, -216(%rbp)
LBB0_11:
	movq	%rbx, -632(%rbp)        ## 8-byte Spill
	pxor	%xmm0, %xmm0
	movdqa	%xmm0, -672(%rbp)
	movq	$0, -656(%rbp)
	leaq	-624(%rbp), %r12
	leaq	-680(%rbp), %rbx
	leaq	-672(%rbp), %r15
	jmp	LBB0_12
	.p2align	4, 0x90
LBB0_31:                                ##   in Loop: Header=BB0_12 Depth=1
	addq	$24, 8(%r13)
LBB0_12:                                ## =>This Inner Loop Header: Depth=1
	movq	-624(%rbp), %rax
	movq	-24(%rax), %rsi
	addq	%r12, %rsi
Ltmp11:
	movq	%rbx, %rdi
	callq	__ZNKSt3__18ios_base6getlocEv
Ltmp12:
## BB#13:                               ##   in Loop: Header=BB0_12 Depth=1
Ltmp13:
	movq	%rbx, %rdi
	movq	__ZNSt3__15ctypeIcE2idE@GOTPCREL(%rip), %rsi
	callq	__ZNKSt3__16locale9use_facetERNS0_2idE
Ltmp14:
## BB#14:                               ##   in Loop: Header=BB0_12 Depth=1
	movq	(%rax), %rcx
Ltmp15:
	movl	$10, %esi
	movq	%rax, %rdi
	callq	*56(%rcx)
	movl	%eax, %r14d
Ltmp16:
## BB#15:                               ##   in Loop: Header=BB0_12 Depth=1
	movq	%rbx, %rdi
	callq	__ZNSt3__16localeD1Ev
Ltmp18:
	movsbl	%r14b, %edx
	movq	%r12, %rdi
	movq	%r15, %rsi
	callq	__ZNSt3__17getlineIcNS_11char_traitsIcEENS_9allocatorIcEEEERNS_13basic_istreamIT_T0_EES9_RNS_12basic_stringIS6_S7_T1_EES6_
Ltmp19:
## BB#16:                               ##   in Loop: Header=BB0_12 Depth=1
	movq	(%rax), %rcx
	movq	-24(%rcx), %rcx
	testb	$5, 32(%rax,%rcx)
	jne	LBB0_17
## BB#29:                               ##   in Loop: Header=BB0_12 Depth=1
	movq	8(%r13), %rdi
	cmpq	16(%r13), %rdi
	jae	LBB0_32
## BB#30:                               ##   in Loop: Header=BB0_12 Depth=1
Ltmp22:
	movq	%r15, %rsi
	callq	__ZNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEC1ERKS5_
Ltmp23:
	jmp	LBB0_31
	.p2align	4, 0x90
LBB0_32:                                ##   in Loop: Header=BB0_12 Depth=1
Ltmp20:
	movq	%r13, %rdi
	movq	%r15, %rsi
	callq	__ZNSt3__16vectorINS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEENS4_IS6_EEE24__emplace_back_slow_pathIJRS6_EEEvDpOT_
Ltmp21:
	jmp	LBB0_12
LBB0_17:
	testb	$1, -672(%rbp)
	leaq	-200(%rbp), %rbx
	je	LBB0_19
## BB#18:
	movq	-656(%rbp), %rdi
	callq	__ZdlPv
LBB0_19:
	movq	-640(%rbp), %rax        ## 8-byte Reload
	movq	%rax, -624(%rbp)
	movq	-632(%rbp), %rax        ## 8-byte Reload
	movq	%rax, -200(%rbp)
	leaq	-608(%rbp), %rdi
	callq	__ZNSt3__113basic_filebufIcNS_11char_traitsIcEEED2Ev
	movq	__ZTTNSt3__114basic_ifstreamIcNS_11char_traitsIcEEEE@GOTPCREL(%rip), %rsi
	addq	$8, %rsi
	leaq	-624(%rbp), %rdi
	callq	__ZNSt3__113basic_istreamIcNS_11char_traitsIcEEED2Ev
	movq	%rbx, %rdi
	callq	__ZNSt3__19basic_iosIcNS_11char_traitsIcEEED2Ev
	movq	___stack_chk_guard@GOTPCREL(%rip), %rax
	movq	(%rax), %rax
	cmpq	-48(%rbp), %rax
	jne	LBB0_40
## BB#20:
	movq	%r13, %rax
	addq	$648, %rsp              ## imm = 0x288
	popq	%rbx
	popq	%r12
	popq	%r13
	popq	%r14
	popq	%r15
	popq	%rbp
	retq
LBB0_40:
	callq	___stack_chk_fail
LBB0_6:
Ltmp5:
	movq	%rax, %r15
	jmp	LBB0_9
LBB0_21:
Ltmp2:
	movq	%rax, %r15
	jmp	LBB0_22
LBB0_7:
Ltmp10:
	movq	%rax, %r15
	movq	%r12, %rdi
	jmp	LBB0_8
LBB0_39:
Ltmp17:
	movq	%rax, %r15
	leaq	-680(%rbp), %rdi
	callq	__ZNSt3__16localeD1Ev
	jmp	LBB0_26
LBB0_25:
Ltmp24:
	movq	%rax, %r15
LBB0_26:
	testb	$1, -672(%rbp)
	leaq	-200(%rbp), %r14
	leaq	-608(%rbp), %rbx
	je	LBB0_28
## BB#27:
	movq	-656(%rbp), %rdi
	callq	__ZdlPv
LBB0_28:
	movq	-640(%rbp), %rax        ## 8-byte Reload
	movq	%rax, -624(%rbp)
	movq	-632(%rbp), %rax        ## 8-byte Reload
	movq	%rax, -200(%rbp)
	movq	%rbx, %rdi
LBB0_8:
	callq	__ZNSt3__113basic_filebufIcNS_11char_traitsIcEEED2Ev
LBB0_9:
	movq	__ZTTNSt3__114basic_ifstreamIcNS_11char_traitsIcEEEE@GOTPCREL(%rip), %rsi
	addq	$8, %rsi
	leaq	-624(%rbp), %rdi
	callq	__ZNSt3__113basic_istreamIcNS_11char_traitsIcEEED2Ev
LBB0_22:
	movq	%r14, %rdi
	callq	__ZNSt3__19basic_iosIcNS_11char_traitsIcEEED2Ev
	movq	(%r13), %r14
	testq	%r14, %r14
	je	LBB0_38
## BB#23:
	movq	8(%r13), %rax
	cmpq	%r14, %rax
	je	LBB0_24
	.p2align	4, 0x90
LBB0_33:                                ## =>This Inner Loop Header: Depth=1
	leaq	-24(%rax), %rbx
	testb	$1, -24(%rax)
	je	LBB0_35
## BB#34:                               ##   in Loop: Header=BB0_33 Depth=1
	movq	-8(%rax), %rdi
	callq	__ZdlPv
LBB0_35:                                ##   in Loop: Header=BB0_33 Depth=1
	cmpq	%rbx, %r14
	movq	%rbx, %rax
	jne	LBB0_33
## BB#36:
	movq	(%r13), %rdi
LBB0_37:
	movq	%r14, 8(%r13)
	callq	__ZdlPv
LBB0_38:
	movq	%r15, %rdi
	callq	__Unwind_Resume
LBB0_24:
	movq	%r14, %rdi
	jmp	LBB0_37
Lfunc_end0:
	.cfi_endproc
	.section	__TEXT,__gcc_except_tab
	.p2align	2
GCC_except_table0:
Lexception0:
	.byte	255                     ## @LPStart Encoding = omit
	.byte	155                     ## @TType Encoding = indirect pcrel sdata4
	.byte	93                      ## @TType base offset
	.byte	3                       ## Call site Encoding = udata4
	.byte	91                      ## Call site table length
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
Lset11 = Ltmp24-Lfunc_begin0            ##     jumps to Ltmp24
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
Lset16 = Ltmp21-Ltmp18                  ##   Call between Ltmp18 and Ltmp21
	.long	Lset16
Lset17 = Ltmp24-Lfunc_begin0            ##     jumps to Ltmp24
	.long	Lset17
	.byte	0                       ##   On action: cleanup
Lset18 = Ltmp21-Lfunc_begin0            ## >> Call Site 7 <<
	.long	Lset18
Lset19 = Lfunc_end0-Ltmp21              ##   Call between Ltmp21 and Lfunc_end0
	.long	Lset19
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
	leaq	-96(%rbp), %rdi
	callq	__Z10read_linesPKc
Ltmp25:
	leaq	L_.str.1(%rip), %rsi
	leaq	-72(%rbp), %rdi
	callq	__Z10read_linesPKc
Ltmp26:
## BB#1:
	callq	__ZNSt3__16chrono12system_clock3nowEv
	movq	%rax, -104(%rbp)        ## 8-byte Spill
	movq	-72(%rbp), %r12
	movq	-64(%rbp), %rax
	movq	%rax, -112(%rbp)        ## 8-byte Spill
	cmpq	%rax, %r12
	je	LBB2_22
	.p2align	4, 0x90
LBB2_2:                                 ## =>This Loop Header: Depth=1
                                        ##     Child Loop BB2_6 Depth 2
                                        ##       Child Loop BB2_9 Depth 3
                                        ##     Child Loop BB2_4 Depth 2
	movq	-96(%rbp), %rbx
	movq	-88(%rbp), %r15
	cmpq	%r15, %rbx
	je	LBB2_11
## BB#3:                                ##   in Loop: Header=BB2_2 Depth=1
	movzbl	(%r12), %eax
	movq	%rax, %r14
	shrq	%r14
	leaq	1(%r12), %r13
	andb	$1, %al
	cmovneq	16(%r12), %r13
	testb	%al, %al
	cmovneq	8(%r12), %r14
	testq	%r14, %r14
	je	LBB2_4
	.p2align	4, 0x90
LBB2_6:                                 ##   Parent Loop BB2_2 Depth=1
                                        ## =>  This Loop Header: Depth=2
                                        ##       Child Loop BB2_9 Depth 3
	movzbl	(%rbx), %ecx
	movq	%rcx, %rax
	shrq	%rax
	testb	$1, %cl
	movq	8(%rbx), %rdx
	cmoveq	%rax, %rdx
	cmpq	%r14, %rdx
	jne	LBB2_13
## BB#7:                                ##   in Loop: Header=BB2_6 Depth=2
	testb	$1, %cl
	jne	LBB2_12
## BB#8:                                ##   in Loop: Header=BB2_6 Depth=2
	xorl	%ecx, %ecx
	.p2align	4, 0x90
LBB2_9:                                 ##   Parent Loop BB2_2 Depth=1
                                        ##     Parent Loop BB2_6 Depth=2
                                        ## =>    This Inner Loop Header: Depth=3
	movzbl	1(%rbx,%rcx), %edx
	cmpb	(%r13,%rcx), %dl
	jne	LBB2_13
## BB#10:                               ##   in Loop: Header=BB2_9 Depth=3
	incq	%rcx
	cmpq	%rcx, %rax
	jne	LBB2_9
	jmp	LBB2_11
	.p2align	4, 0x90
LBB2_12:                                ##   in Loop: Header=BB2_6 Depth=2
	movq	16(%rbx), %rdi
	movq	%r13, %rsi
	movq	%r14, %rdx
	callq	_memcmp
	testl	%eax, %eax
	je	LBB2_11
LBB2_13:                                ##   in Loop: Header=BB2_6 Depth=2
	addq	$24, %rbx
	cmpq	%r15, %rbx
	jne	LBB2_6
	jmp	LBB2_14
	.p2align	4, 0x90
LBB2_4:                                 ##   Parent Loop BB2_2 Depth=1
                                        ## =>  This Inner Loop Header: Depth=2
	movzbl	(%rbx), %eax
	movq	%rax, %rcx
	shrq	%rcx
	testb	$1, %al
	cmovneq	8(%rbx), %rcx
	testq	%rcx, %rcx
	je	LBB2_11
## BB#5:                                ##   in Loop: Header=BB2_4 Depth=2
	addq	$24, %rbx
	cmpq	%r15, %rbx
	jne	LBB2_4
	jmp	LBB2_14
	.p2align	4, 0x90
LBB2_11:                                ##   in Loop: Header=BB2_2 Depth=1
	cmpq	%r15, %rbx
	jne	LBB2_21
LBB2_14:                                ##   in Loop: Header=BB2_2 Depth=1
Ltmp28:
	movl	$14, %edx
	movq	__ZNSt3__14coutE@GOTPCREL(%rip), %rdi
	leaq	L_.str.2(%rip), %rsi
	callq	__ZNSt3__124__put_character_sequenceIcNS_11char_traitsIcEEEERNS_13basic_ostreamIT_T0_EES7_PKS4_m
Ltmp29:
## BB#15:                               ##   in Loop: Header=BB2_2 Depth=1
	movzbl	(%r12), %ecx
	leaq	1(%r12), %rsi
	movq	%rcx, %rdx
	shrq	%rdx
	andb	$1, %cl
	cmovneq	16(%r12), %rsi
	testb	%cl, %cl
	cmovneq	8(%r12), %rdx
Ltmp30:
	movq	%rax, %rdi
	callq	__ZNSt3__124__put_character_sequenceIcNS_11char_traitsIcEEEERNS_13basic_ostreamIT_T0_EES7_PKS4_m
	movq	%rax, %r14
Ltmp31:
## BB#16:                               ##   in Loop: Header=BB2_2 Depth=1
	movq	(%r14), %rax
	movq	-24(%rax), %rsi
	addq	%r14, %rsi
Ltmp32:
	leaq	-48(%rbp), %rdi
	callq	__ZNKSt3__18ios_base6getlocEv
Ltmp33:
## BB#17:                               ##   in Loop: Header=BB2_2 Depth=1
Ltmp34:
	leaq	-48(%rbp), %rdi
	movq	__ZNSt3__15ctypeIcE2idE@GOTPCREL(%rip), %rsi
	callq	__ZNKSt3__16locale9use_facetERNS0_2idE
Ltmp35:
## BB#18:                               ##   in Loop: Header=BB2_2 Depth=1
	movq	(%rax), %rcx
Ltmp36:
	movl	$10, %esi
	movq	%rax, %rdi
	callq	*56(%rcx)
	movl	%eax, %ebx
Ltmp37:
## BB#19:                               ##   in Loop: Header=BB2_2 Depth=1
	leaq	-48(%rbp), %rdi
	callq	__ZNSt3__16localeD1Ev
Ltmp39:
	movsbl	%bl, %esi
	movq	%r14, %rdi
	callq	__ZNSt3__113basic_ostreamIcNS_11char_traitsIcEEE3putEc
Ltmp40:
## BB#20:                               ##   in Loop: Header=BB2_2 Depth=1
Ltmp41:
	movq	%r14, %rdi
	callq	__ZNSt3__113basic_ostreamIcNS_11char_traitsIcEEE5flushEv
Ltmp42:
LBB2_21:                                ##   in Loop: Header=BB2_2 Depth=1
	addq	$24, %r12
	cmpq	-112(%rbp), %r12        ## 8-byte Folded Reload
	jne	LBB2_2
LBB2_22:
	callq	__ZNSt3__16chrono12system_clock3nowEv
	subq	-104(%rbp), %rax        ## 8-byte Folded Reload
	cvtsi2sdq	%rax, %xmm0
	divsd	LCPI2_0(%rip), %xmm0
	leaq	L_.str.3(%rip), %rdi
	movb	$1, %al
	callq	_printf
	movq	-72(%rbp), %r14
	testq	%r14, %r14
	je	LBB2_30
## BB#23:
	movq	-64(%rbp), %rax
	cmpq	%r14, %rax
	je	LBB2_24
	.p2align	4, 0x90
LBB2_25:                                ## =>This Inner Loop Header: Depth=1
	leaq	-24(%rax), %rbx
	testb	$1, -24(%rax)
	je	LBB2_27
## BB#26:                               ##   in Loop: Header=BB2_25 Depth=1
	movq	-8(%rax), %rdi
	callq	__ZdlPv
LBB2_27:                                ##   in Loop: Header=BB2_25 Depth=1
	cmpq	%rbx, %r14
	movq	%rbx, %rax
	jne	LBB2_25
## BB#28:
	movq	-72(%rbp), %rdi
	jmp	LBB2_29
LBB2_24:
	movq	%r14, %rdi
LBB2_29:
	movq	%r14, -64(%rbp)
	callq	__ZdlPv
LBB2_30:
	movq	-96(%rbp), %r14
	testq	%r14, %r14
	je	LBB2_38
## BB#31:
	movq	-88(%rbp), %rax
	cmpq	%r14, %rax
	je	LBB2_32
	.p2align	4, 0x90
LBB2_33:                                ## =>This Inner Loop Header: Depth=1
	leaq	-24(%rax), %rbx
	testb	$1, -24(%rax)
	je	LBB2_35
## BB#34:                               ##   in Loop: Header=BB2_33 Depth=1
	movq	-8(%rax), %rdi
	callq	__ZdlPv
LBB2_35:                                ##   in Loop: Header=BB2_33 Depth=1
	cmpq	%rbx, %r14
	movq	%rbx, %rax
	jne	LBB2_33
## BB#36:
	movq	-96(%rbp), %rdi
	jmp	LBB2_37
LBB2_32:
	movq	%r14, %rdi
LBB2_37:
	movq	%r14, -88(%rbp)
	callq	__ZdlPv
LBB2_38:
	xorl	%eax, %eax
	addq	$72, %rsp
	popq	%rbx
	popq	%r12
	popq	%r13
	popq	%r14
	popq	%r15
	popq	%rbp
	retq
LBB2_57:
Ltmp27:
	movq	%rax, %r14
	movq	-96(%rbp), %r15
	testq	%r15, %r15
	jne	LBB2_49
	jmp	LBB2_56
LBB2_39:
Ltmp38:
	movq	%rax, %r14
	leaq	-48(%rbp), %rdi
	callq	__ZNSt3__16localeD1Ev
	movq	-72(%rbp), %r15
	testq	%r15, %r15
	je	LBB2_48
LBB2_41:
	movq	-64(%rbp), %rax
	cmpq	%r15, %rax
	je	LBB2_42
	.p2align	4, 0x90
LBB2_43:                                ## =>This Inner Loop Header: Depth=1
	leaq	-24(%rax), %rbx
	testb	$1, -24(%rax)
	je	LBB2_45
## BB#44:                               ##   in Loop: Header=BB2_43 Depth=1
	movq	-8(%rax), %rdi
	callq	__ZdlPv
LBB2_45:                                ##   in Loop: Header=BB2_43 Depth=1
	cmpq	%rbx, %r15
	movq	%rbx, %rax
	jne	LBB2_43
## BB#46:
	movq	-72(%rbp), %rdi
	jmp	LBB2_47
LBB2_42:
	movq	%r15, %rdi
LBB2_47:
	movq	%r15, -64(%rbp)
	callq	__ZdlPv
	movq	-96(%rbp), %r15
	testq	%r15, %r15
	jne	LBB2_49
	jmp	LBB2_56
LBB2_58:
Ltmp43:
	movq	%rax, %r14
	movq	-72(%rbp), %r15
	testq	%r15, %r15
	jne	LBB2_41
LBB2_48:
	movq	-96(%rbp), %r15
	testq	%r15, %r15
	je	LBB2_56
LBB2_49:
	movq	-88(%rbp), %rax
	cmpq	%r15, %rax
	je	LBB2_50
	.p2align	4, 0x90
LBB2_51:                                ## =>This Inner Loop Header: Depth=1
	leaq	-24(%rax), %rbx
	testb	$1, -24(%rax)
	je	LBB2_53
## BB#52:                               ##   in Loop: Header=BB2_51 Depth=1
	movq	-8(%rax), %rdi
	callq	__ZdlPv
LBB2_53:                                ##   in Loop: Header=BB2_51 Depth=1
	cmpq	%rbx, %r15
	movq	%rbx, %rax
	jne	LBB2_51
## BB#54:
	movq	-96(%rbp), %rdi
LBB2_55:
	movq	%r15, -88(%rbp)
	callq	__ZdlPv
LBB2_56:
	movq	%r14, %rdi
	callq	__Unwind_Resume
LBB2_50:
	movq	%r15, %rdi
	jmp	LBB2_55
Lfunc_end1:
	.cfi_endproc
	.section	__TEXT,__gcc_except_tab
	.p2align	2
GCC_except_table2:
Lexception1:
	.byte	255                     ## @LPStart Encoding = omit
	.byte	155                     ## @TType Encoding = indirect pcrel sdata4
	.asciz	"\320"                  ## @TType base offset
	.byte	3                       ## Call site Encoding = udata4
	.byte	78                      ## Call site table length
Lset20 = Lfunc_begin1-Lfunc_begin1      ## >> Call Site 1 <<
	.long	Lset20
Lset21 = Ltmp25-Lfunc_begin1            ##   Call between Lfunc_begin1 and Ltmp25
	.long	Lset21
	.long	0                       ##     has no landing pad
	.byte	0                       ##   On action: cleanup
Lset22 = Ltmp25-Lfunc_begin1            ## >> Call Site 2 <<
	.long	Lset22
Lset23 = Ltmp26-Ltmp25                  ##   Call between Ltmp25 and Ltmp26
	.long	Lset23
Lset24 = Ltmp27-Lfunc_begin1            ##     jumps to Ltmp27
	.long	Lset24
	.byte	0                       ##   On action: cleanup
Lset25 = Ltmp28-Lfunc_begin1            ## >> Call Site 3 <<
	.long	Lset25
Lset26 = Ltmp33-Ltmp28                  ##   Call between Ltmp28 and Ltmp33
	.long	Lset26
Lset27 = Ltmp43-Lfunc_begin1            ##     jumps to Ltmp43
	.long	Lset27
	.byte	0                       ##   On action: cleanup
Lset28 = Ltmp34-Lfunc_begin1            ## >> Call Site 4 <<
	.long	Lset28
Lset29 = Ltmp37-Ltmp34                  ##   Call between Ltmp34 and Ltmp37
	.long	Lset29
Lset30 = Ltmp38-Lfunc_begin1            ##     jumps to Ltmp38
	.long	Lset30
	.byte	0                       ##   On action: cleanup
Lset31 = Ltmp39-Lfunc_begin1            ## >> Call Site 5 <<
	.long	Lset31
Lset32 = Ltmp42-Ltmp39                  ##   Call between Ltmp39 and Ltmp42
	.long	Lset32
Lset33 = Ltmp43-Lfunc_begin1            ##     jumps to Ltmp43
	.long	Lset33
	.byte	0                       ##   On action: cleanup
Lset34 = Ltmp42-Lfunc_begin1            ## >> Call Site 6 <<
	.long	Lset34
Lset35 = Lfunc_end1-Ltmp42              ##   Call between Ltmp42 and Lfunc_end1
	.long	Lset35
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
Ltmp44:
	movq	%rbx, %rdi
	callq	__ZNSt3__113basic_filebufIcNS_11char_traitsIcEEE4syncEv
Ltmp45:
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
Ltmp46:
	movq	%rax, %r15
	movq	%r14, %rdi
	callq	_fclose
	movq	%r15, %rdi
	callq	___cxa_begin_catch
Ltmp47:
	callq	___cxa_end_catch
Ltmp48:
	jmp	LBB7_5
LBB7_12:
Ltmp49:
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
Lset36 = Ltmp44-Lfunc_begin2            ## >> Call Site 1 <<
	.long	Lset36
Lset37 = Ltmp45-Ltmp44                  ##   Call between Ltmp44 and Ltmp45
	.long	Lset37
Lset38 = Ltmp46-Lfunc_begin2            ##     jumps to Ltmp46
	.long	Lset38
	.byte	1                       ##   On action: 1
Lset39 = Ltmp45-Lfunc_begin2            ## >> Call Site 2 <<
	.long	Lset39
Lset40 = Ltmp47-Ltmp45                  ##   Call between Ltmp45 and Ltmp47
	.long	Lset40
	.long	0                       ##     has no landing pad
	.byte	0                       ##   On action: cleanup
Lset41 = Ltmp47-Lfunc_begin2            ## >> Call Site 3 <<
	.long	Lset41
Lset42 = Ltmp48-Ltmp47                  ##   Call between Ltmp47 and Ltmp48
	.long	Lset42
Lset43 = Ltmp49-Lfunc_begin2            ##     jumps to Ltmp49
	.long	Lset43
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
Ltmp50:
	movq	__ZNSt3__17codecvtIcc11__mbstate_tE2idE@GOTPCREL(%rip), %rsi
	movq	%r15, %rdi
	callq	__ZNKSt3__16locale9has_facetERNS0_2idE
	movl	%eax, %r15d
Ltmp51:
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
Ltmp53:
	movq	__ZNSt3__17codecvtIcc11__mbstate_tE2idE@GOTPCREL(%rip), %rsi
	movq	%r15, %rdi
	callq	__ZNKSt3__16locale9use_facetERNS0_2idE
Ltmp54:
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
Ltmp56:
	xorl	%esi, %esi
	movl	$4096, %edx             ## imm = 0x1000
	movq	%rbx, %rdi
	callq	*24(%rax)
Ltmp57:
## BB#5:
	addq	$8, %rsp
	popq	%rbx
	popq	%r14
	popq	%r15
	popq	%rbp
	retq
LBB18_9:
Ltmp55:
	movq	%rax, %r14
	leaq	-32(%rbp), %rdi
	callq	__ZNSt3__16localeD1Ev
	jmp	LBB18_7
LBB18_6:
Ltmp58:
	movq	%rax, %r14
LBB18_7:
	movq	%rbx, %rdi
	callq	__ZNSt3__115basic_streambufIcNS_11char_traitsIcEEED2Ev
	movq	%r14, %rdi
	callq	__Unwind_Resume
LBB18_8:
Ltmp52:
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
Lset44 = Lfunc_begin3-Lfunc_begin3      ## >> Call Site 1 <<
	.long	Lset44
Lset45 = Ltmp50-Lfunc_begin3            ##   Call between Lfunc_begin3 and Ltmp50
	.long	Lset45
	.long	0                       ##     has no landing pad
	.byte	0                       ##   On action: cleanup
Lset46 = Ltmp50-Lfunc_begin3            ## >> Call Site 2 <<
	.long	Lset46
Lset47 = Ltmp51-Ltmp50                  ##   Call between Ltmp50 and Ltmp51
	.long	Lset47
Lset48 = Ltmp52-Lfunc_begin3            ##     jumps to Ltmp52
	.long	Lset48
	.byte	1                       ##   On action: 1
Lset49 = Ltmp53-Lfunc_begin3            ## >> Call Site 3 <<
	.long	Lset49
Lset50 = Ltmp54-Ltmp53                  ##   Call between Ltmp53 and Ltmp54
	.long	Lset50
Lset51 = Ltmp55-Lfunc_begin3            ##     jumps to Ltmp55
	.long	Lset51
	.byte	0                       ##   On action: cleanup
Lset52 = Ltmp54-Lfunc_begin3            ## >> Call Site 4 <<
	.long	Lset52
Lset53 = Ltmp56-Ltmp54                  ##   Call between Ltmp54 and Ltmp56
	.long	Lset53
	.long	0                       ##     has no landing pad
	.byte	0                       ##   On action: cleanup
Lset54 = Ltmp56-Lfunc_begin3            ## >> Call Site 5 <<
	.long	Lset54
Lset55 = Ltmp57-Ltmp56                  ##   Call between Ltmp56 and Ltmp57
	.long	Lset55
Lset56 = Ltmp58-Lfunc_begin3            ##     jumps to Ltmp58
	.long	Lset56
	.byte	0                       ##   On action: cleanup
Lset57 = Ltmp57-Lfunc_begin3            ## >> Call Site 6 <<
	.long	Lset57
Lset58 = Lfunc_end3-Ltmp57              ##   Call between Ltmp57 and Lfunc_end3
	.long	Lset58
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
Ltmp59:
	leaq	-40(%rbp), %rdi
	movl	$1, %edx
	movq	%r15, %rsi
	callq	__ZNSt3__113basic_istreamIcNS_11char_traitsIcEEE6sentryC1ERS3_b
Ltmp60:
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
Ltmp62:
	callq	*80(%rax)
Ltmp63:
## BB#8:                                ##   in Loop: Header=BB19_6 Depth=1
	cmpl	$-1, %eax
	je	LBB19_9
## BB#15:                               ##   in Loop: Header=BB19_6 Depth=1
	cmpb	%r14b, %al
	je	LBB19_16
LBB19_17:                               ##   in Loop: Header=BB19_6 Depth=1
Ltmp65:
	movsbl	%al, %esi
	movq	%rbx, %rdi
	callq	__ZNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEE9push_backEc
Ltmp66:
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
Ltmp68:
	callq	__ZNSt3__18ios_base5clearEj
Ltmp69:
	jmp	LBB19_23
LBB19_9:
	movl	$2, %ecx
	jmp	LBB19_22
LBB19_28:
Ltmp70:
	jmp	LBB19_11
LBB19_10:
Ltmp61:
	jmp	LBB19_11
LBB19_13:
Ltmp64:
	jmp	LBB19_11
LBB19_27:
Ltmp67:
LBB19_11:
	movq	%rax, %rdi
	movq	%r15, %rbx
	callq	___cxa_begin_catch
	movq	(%r15), %rax
	addq	-24(%rax), %rbx
Ltmp71:
	movq	%rbx, %rdi
	callq	__ZNSt3__18ios_base33__set_badbit_and_consider_rethrowEv
Ltmp72:
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
Ltmp73:
	movq	%rax, %rbx
Ltmp74:
	callq	___cxa_end_catch
Ltmp75:
## BB#25:
	movq	%rbx, %rdi
	callq	__Unwind_Resume
LBB19_26:
Ltmp76:
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
Lset59 = Ltmp59-Lfunc_begin4            ## >> Call Site 1 <<
	.long	Lset59
Lset60 = Ltmp60-Ltmp59                  ##   Call between Ltmp59 and Ltmp60
	.long	Lset60
Lset61 = Ltmp61-Lfunc_begin4            ##     jumps to Ltmp61
	.long	Lset61
	.byte	1                       ##   On action: 1
Lset62 = Ltmp62-Lfunc_begin4            ## >> Call Site 2 <<
	.long	Lset62
Lset63 = Ltmp63-Ltmp62                  ##   Call between Ltmp62 and Ltmp63
	.long	Lset63
Lset64 = Ltmp64-Lfunc_begin4            ##     jumps to Ltmp64
	.long	Lset64
	.byte	1                       ##   On action: 1
Lset65 = Ltmp65-Lfunc_begin4            ## >> Call Site 3 <<
	.long	Lset65
Lset66 = Ltmp66-Ltmp65                  ##   Call between Ltmp65 and Ltmp66
	.long	Lset66
Lset67 = Ltmp67-Lfunc_begin4            ##     jumps to Ltmp67
	.long	Lset67
	.byte	1                       ##   On action: 1
Lset68 = Ltmp68-Lfunc_begin4            ## >> Call Site 4 <<
	.long	Lset68
Lset69 = Ltmp69-Ltmp68                  ##   Call between Ltmp68 and Ltmp69
	.long	Lset69
Lset70 = Ltmp70-Lfunc_begin4            ##     jumps to Ltmp70
	.long	Lset70
	.byte	1                       ##   On action: 1
Lset71 = Ltmp69-Lfunc_begin4            ## >> Call Site 5 <<
	.long	Lset71
Lset72 = Ltmp71-Ltmp69                  ##   Call between Ltmp69 and Ltmp71
	.long	Lset72
	.long	0                       ##     has no landing pad
	.byte	0                       ##   On action: cleanup
Lset73 = Ltmp71-Lfunc_begin4            ## >> Call Site 6 <<
	.long	Lset73
Lset74 = Ltmp72-Ltmp71                  ##   Call between Ltmp71 and Ltmp72
	.long	Lset74
Lset75 = Ltmp73-Lfunc_begin4            ##     jumps to Ltmp73
	.long	Lset75
	.byte	0                       ##   On action: cleanup
Lset76 = Ltmp72-Lfunc_begin4            ## >> Call Site 7 <<
	.long	Lset76
Lset77 = Ltmp74-Ltmp72                  ##   Call between Ltmp72 and Ltmp74
	.long	Lset77
	.long	0                       ##     has no landing pad
	.byte	0                       ##   On action: cleanup
Lset78 = Ltmp74-Lfunc_begin4            ## >> Call Site 8 <<
	.long	Lset78
Lset79 = Ltmp75-Ltmp74                  ##   Call between Ltmp74 and Ltmp75
	.long	Lset79
Lset80 = Ltmp76-Lfunc_begin4            ##     jumps to Ltmp76
	.long	Lset80
	.byte	1                       ##   On action: 1
Lset81 = Ltmp75-Lfunc_begin4            ## >> Call Site 9 <<
	.long	Lset81
Lset82 = Lfunc_end4-Ltmp75              ##   Call between Ltmp75 and Lfunc_end4
	.long	Lset82
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
	.globl	__ZNSt3__16vectorINS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEENS4_IS6_EEE24__emplace_back_slow_pathIJRS6_EEEvDpOT_ ## -- Begin function _ZNSt3__16vectorINS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEENS4_IS6_EEE24__emplace_back_slow_pathIJRS6_EEEvDpOT_
	.weak_def_can_be_hidden	__ZNSt3__16vectorINS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEENS4_IS6_EEE24__emplace_back_slow_pathIJRS6_EEEvDpOT_
	.p2align	4, 0x90
__ZNSt3__16vectorINS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEENS4_IS6_EEE24__emplace_back_slow_pathIJRS6_EEEvDpOT_: ## @_ZNSt3__16vectorINS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEENS4_IS6_EEE24__emplace_back_slow_pathIJRS6_EEEvDpOT_
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
	pushq	%rax
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
	movq	%rsi, %r14
	movq	%rdi, %r15
	movabsq	$768614336404564650, %r13 ## imm = 0xAAAAAAAAAAAAAAA
	movq	(%r15), %rdx
	movq	8(%r15), %rbx
	subq	%rdx, %rbx
	sarq	$3, %rbx
	movabsq	$-6148914691236517205, %rsi ## imm = 0xAAAAAAAAAAAAAAAB
	imulq	%rsi, %rbx
	leaq	1(%rbx), %rax
	cmpq	%r13, %rax
	ja	LBB20_19
## BB#1:
	movq	16(%r15), %rcx
	subq	%rdx, %rcx
	sarq	$3, %rcx
	imulq	%rsi, %rcx
	movabsq	$384307168202282324, %rdx ## imm = 0x555555555555554
	cmpq	%rdx, %rcx
	ja	LBB20_4
## BB#2:
	addq	%rcx, %rcx
	cmpq	%rax, %rcx
	cmovbq	%rax, %rcx
	testq	%rcx, %rcx
	movq	%rcx, %r13
	je	LBB20_3
LBB20_4:
	leaq	(,%r13,8), %rax
	leaq	(%rax,%rax,2), %rdi
	callq	__Znwm
	movq	%rax, %r12
LBB20_5:
	leaq	(%rbx,%rbx,2), %rax
	leaq	(%r12,%rax,8), %rbx
Ltmp77:
	movq	%rbx, %rdi
	movq	%r14, %rsi
	callq	__ZNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEC1ERKS5_
Ltmp78:
## BB#6:
	leaq	(%r13,%r13,2), %rax
	leaq	(%r12,%rax,8), %r8
	leaq	24(%rbx), %rdx
	movq	(%r15), %rax
	movq	8(%r15), %rsi
	cmpq	%rax, %rsi
	je	LBB20_7
	.p2align	4, 0x90
LBB20_8:                                ## =>This Inner Loop Header: Depth=1
	movq	-8(%rsi), %rdi
	movq	%rdi, -8(%rbx)
	movq	-24(%rsi), %rdi
	movq	-16(%rsi), %rcx
	movq	%rcx, -16(%rbx)
	movq	%rdi, -24(%rbx)
	movq	$0, -8(%rsi)
	movq	$0, -16(%rsi)
	movq	$0, -24(%rsi)
	leaq	-24(%rsi), %rsi
	addq	$-24, %rbx
	cmpq	%rsi, %rax
	jne	LBB20_8
## BB#9:
	movq	(%r15), %r14
	movq	8(%r15), %rax
	jmp	LBB20_10
LBB20_7:
	movq	%rax, %r14
LBB20_10:
	movq	%rbx, (%r15)
	movq	%rdx, 8(%r15)
	movq	%r8, 16(%r15)
	cmpq	%r14, %rax
	je	LBB20_14
	.p2align	4, 0x90
LBB20_11:                               ## =>This Inner Loop Header: Depth=1
	leaq	-24(%rax), %rbx
	testb	$1, -24(%rax)
	je	LBB20_13
## BB#12:                               ##   in Loop: Header=BB20_11 Depth=1
	movq	-8(%rax), %rdi
	callq	__ZdlPv
LBB20_13:                               ##   in Loop: Header=BB20_11 Depth=1
	cmpq	%rbx, %r14
	movq	%rbx, %rax
	jne	LBB20_11
LBB20_14:
	testq	%r14, %r14
	je	LBB20_15
## BB#20:
	movq	%r14, %rdi
	addq	$8, %rsp
	popq	%rbx
	popq	%r12
	popq	%r13
	popq	%r14
	popq	%r15
	popq	%rbp
	jmp	__ZdlPv                 ## TAILCALL
LBB20_15:
	addq	$8, %rsp
	popq	%rbx
	popq	%r12
	popq	%r13
	popq	%r14
	popq	%r15
	popq	%rbp
	retq
LBB20_3:
	xorl	%r13d, %r13d
	xorl	%r12d, %r12d
	jmp	LBB20_5
LBB20_19:
	movq	%r15, %rdi
	callq	__ZNKSt3__120__vector_base_commonILb1EE20__throw_length_errorEv
LBB20_16:
Ltmp79:
	movq	%rax, %rbx
	testq	%r12, %r12
	je	LBB20_18
## BB#17:
	movq	%r12, %rdi
	callq	__ZdlPv
LBB20_18:
	movq	%rbx, %rdi
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
Lset83 = Lfunc_begin5-Lfunc_begin5      ## >> Call Site 1 <<
	.long	Lset83
Lset84 = Ltmp77-Lfunc_begin5            ##   Call between Lfunc_begin5 and Ltmp77
	.long	Lset84
	.long	0                       ##     has no landing pad
	.byte	0                       ##   On action: cleanup
Lset85 = Ltmp77-Lfunc_begin5            ## >> Call Site 2 <<
	.long	Lset85
Lset86 = Ltmp78-Ltmp77                  ##   Call between Ltmp77 and Ltmp78
	.long	Lset86
Lset87 = Ltmp79-Lfunc_begin5            ##     jumps to Ltmp79
	.long	Lset87
	.byte	0                       ##   On action: cleanup
Lset88 = Ltmp78-Lfunc_begin5            ## >> Call Site 3 <<
	.long	Lset88
Lset89 = Lfunc_end5-Ltmp78              ##   Call between Ltmp78 and Lfunc_end5
	.long	Lset89
	.long	0                       ##     has no landing pad
	.byte	0                       ##   On action: cleanup
	.p2align	2
                                        ## -- End function
	.section	__TEXT,__text,regular,pure_instructions
	.globl	__ZNSt3__124__put_character_sequenceIcNS_11char_traitsIcEEEERNS_13basic_ostreamIT_T0_EES7_PKS4_m ## -- Begin function _ZNSt3__124__put_character_sequenceIcNS_11char_traitsIcEEEERNS_13basic_ostreamIT_T0_EES7_PKS4_m
	.weak_def_can_be_hidden	__ZNSt3__124__put_character_sequenceIcNS_11char_traitsIcEEEERNS_13basic_ostreamIT_T0_EES7_PKS4_m
	.p2align	4, 0x90
__ZNSt3__124__put_character_sequenceIcNS_11char_traitsIcEEEERNS_13basic_ostreamIT_T0_EES7_PKS4_m: ## @_ZNSt3__124__put_character_sequenceIcNS_11char_traitsIcEEEERNS_13basic_ostreamIT_T0_EES7_PKS4_m
Lfunc_begin6:
	.cfi_startproc
	.cfi_personality 155, ___gxx_personality_v0
	.cfi_lsda 16, Lexception6
## BB#0:
	pushq	%rbp
Lcfi123:
	.cfi_def_cfa_offset 16
Lcfi124:
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
Lcfi125:
	.cfi_def_cfa_register %rbp
	pushq	%r15
	pushq	%r14
	pushq	%r13
	pushq	%r12
	pushq	%rbx
	subq	$40, %rsp
Lcfi126:
	.cfi_offset %rbx, -56
Lcfi127:
	.cfi_offset %r12, -48
Lcfi128:
	.cfi_offset %r13, -40
Lcfi129:
	.cfi_offset %r14, -32
Lcfi130:
	.cfi_offset %r15, -24
	movq	%rdx, %r14
	movq	%rsi, %r15
	movq	%rdi, %rbx
Ltmp80:
	leaq	-80(%rbp), %rdi
	movq	%rbx, %rsi
	callq	__ZNSt3__113basic_ostreamIcNS_11char_traitsIcEEE6sentryC1ERS3_
Ltmp81:
## BB#1:
	cmpb	$0, -80(%rbp)
	je	LBB21_10
## BB#2:
	movq	(%rbx), %rax
	movq	-24(%rax), %rax
	leaq	(%rbx,%rax), %r12
	movq	40(%rbx,%rax), %rdi
	movl	8(%rbx,%rax), %r13d
	movl	144(%rbx,%rax), %eax
	cmpl	$-1, %eax
	jne	LBB21_7
## BB#3:
Ltmp83:
	movq	%rdi, -64(%rbp)         ## 8-byte Spill
	leaq	-56(%rbp), %rdi
	movq	%r12, %rsi
	callq	__ZNKSt3__18ios_base6getlocEv
Ltmp84:
## BB#4:
Ltmp85:
	movq	__ZNSt3__15ctypeIcE2idE@GOTPCREL(%rip), %rsi
	leaq	-56(%rbp), %rdi
	callq	__ZNKSt3__16locale9use_facetERNS0_2idE
Ltmp86:
## BB#5:
	movq	(%rax), %rcx
Ltmp87:
	movl	$32, %esi
	movq	%rax, %rdi
	callq	*56(%rcx)
	movb	%al, -41(%rbp)          ## 1-byte Spill
Ltmp88:
## BB#6:
	leaq	-56(%rbp), %rdi
	callq	__ZNSt3__16localeD1Ev
	movsbl	-41(%rbp), %eax         ## 1-byte Folded Reload
	movl	%eax, 144(%r12)
	movq	-64(%rbp), %rdi         ## 8-byte Reload
LBB21_7:
	addq	%r15, %r14
	andl	$176, %r13d
	cmpl	$32, %r13d
	movq	%r15, %rdx
	cmoveq	%r14, %rdx
Ltmp90:
	movsbl	%al, %r9d
	movq	%r15, %rsi
	movq	%r14, %rcx
	movq	%r12, %r8
	callq	__ZNSt3__116__pad_and_outputIcNS_11char_traitsIcEEEENS_19ostreambuf_iteratorIT_T0_EES6_PKS4_S8_S8_RNS_8ios_baseES4_
Ltmp91:
## BB#8:
	testq	%rax, %rax
	jne	LBB21_10
## BB#9:
	movq	(%rbx), %rax
	movq	-24(%rax), %rax
	leaq	(%rbx,%rax), %rdi
	movl	32(%rbx,%rax), %esi
	orl	$5, %esi
Ltmp93:
	callq	__ZNSt3__18ios_base5clearEj
Ltmp94:
LBB21_10:
	leaq	-80(%rbp), %rdi
	callq	__ZNSt3__113basic_ostreamIcNS_11char_traitsIcEEE6sentryD1Ev
LBB21_11:
	movq	%rbx, %rax
	addq	$40, %rsp
	popq	%rbx
	popq	%r12
	popq	%r13
	popq	%r14
	popq	%r15
	popq	%rbp
	retq
LBB21_12:
Ltmp95:
	jmp	LBB21_15
LBB21_13:
Ltmp89:
	movq	%rax, %r14
	leaq	-56(%rbp), %rdi
	callq	__ZNSt3__16localeD1Ev
	jmp	LBB21_16
LBB21_14:
Ltmp92:
LBB21_15:
	movq	%rax, %r14
LBB21_16:
	leaq	-80(%rbp), %rdi
	callq	__ZNSt3__113basic_ostreamIcNS_11char_traitsIcEEE6sentryD1Ev
	jmp	LBB21_18
LBB21_17:
Ltmp82:
	movq	%rax, %r14
LBB21_18:
	movq	%r14, %rdi
	callq	___cxa_begin_catch
	movq	(%rbx), %rax
	movq	%rbx, %rdi
	addq	-24(%rax), %rdi
Ltmp96:
	callq	__ZNSt3__18ios_base33__set_badbit_and_consider_rethrowEv
Ltmp97:
## BB#19:
	callq	___cxa_end_catch
	jmp	LBB21_11
LBB21_20:
Ltmp98:
	movq	%rax, %rbx
Ltmp99:
	callq	___cxa_end_catch
Ltmp100:
## BB#21:
	movq	%rbx, %rdi
	callq	__Unwind_Resume
LBB21_22:
Ltmp101:
	movq	%rax, %rdi
	callq	___clang_call_terminate
Lfunc_end6:
	.cfi_endproc
	.section	__TEXT,__gcc_except_tab
	.p2align	2
GCC_except_table21:
Lexception6:
	.byte	255                     ## @LPStart Encoding = omit
	.byte	155                     ## @TType Encoding = indirect pcrel sdata4
	.asciz	"\213\201"              ## @TType base offset
	.byte	3                       ## Call site Encoding = udata4
	.ascii	"\202\001"              ## Call site table length
Lset90 = Ltmp80-Lfunc_begin6            ## >> Call Site 1 <<
	.long	Lset90
Lset91 = Ltmp81-Ltmp80                  ##   Call between Ltmp80 and Ltmp81
	.long	Lset91
Lset92 = Ltmp82-Lfunc_begin6            ##     jumps to Ltmp82
	.long	Lset92
	.byte	1                       ##   On action: 1
Lset93 = Ltmp83-Lfunc_begin6            ## >> Call Site 2 <<
	.long	Lset93
Lset94 = Ltmp84-Ltmp83                  ##   Call between Ltmp83 and Ltmp84
	.long	Lset94
Lset95 = Ltmp92-Lfunc_begin6            ##     jumps to Ltmp92
	.long	Lset95
	.byte	1                       ##   On action: 1
Lset96 = Ltmp85-Lfunc_begin6            ## >> Call Site 3 <<
	.long	Lset96
Lset97 = Ltmp88-Ltmp85                  ##   Call between Ltmp85 and Ltmp88
	.long	Lset97
Lset98 = Ltmp89-Lfunc_begin6            ##     jumps to Ltmp89
	.long	Lset98
	.byte	1                       ##   On action: 1
Lset99 = Ltmp90-Lfunc_begin6            ## >> Call Site 4 <<
	.long	Lset99
Lset100 = Ltmp91-Ltmp90                 ##   Call between Ltmp90 and Ltmp91
	.long	Lset100
Lset101 = Ltmp92-Lfunc_begin6           ##     jumps to Ltmp92
	.long	Lset101
	.byte	1                       ##   On action: 1
Lset102 = Ltmp93-Lfunc_begin6           ## >> Call Site 5 <<
	.long	Lset102
Lset103 = Ltmp94-Ltmp93                 ##   Call between Ltmp93 and Ltmp94
	.long	Lset103
Lset104 = Ltmp95-Lfunc_begin6           ##     jumps to Ltmp95
	.long	Lset104
	.byte	1                       ##   On action: 1
Lset105 = Ltmp94-Lfunc_begin6           ## >> Call Site 6 <<
	.long	Lset105
Lset106 = Ltmp96-Ltmp94                 ##   Call between Ltmp94 and Ltmp96
	.long	Lset106
	.long	0                       ##     has no landing pad
	.byte	0                       ##   On action: cleanup
Lset107 = Ltmp96-Lfunc_begin6           ## >> Call Site 7 <<
	.long	Lset107
Lset108 = Ltmp97-Ltmp96                 ##   Call between Ltmp96 and Ltmp97
	.long	Lset108
Lset109 = Ltmp98-Lfunc_begin6           ##     jumps to Ltmp98
	.long	Lset109
	.byte	0                       ##   On action: cleanup
Lset110 = Ltmp97-Lfunc_begin6           ## >> Call Site 8 <<
	.long	Lset110
Lset111 = Ltmp99-Ltmp97                 ##   Call between Ltmp97 and Ltmp99
	.long	Lset111
	.long	0                       ##     has no landing pad
	.byte	0                       ##   On action: cleanup
Lset112 = Ltmp99-Lfunc_begin6           ## >> Call Site 9 <<
	.long	Lset112
Lset113 = Ltmp100-Ltmp99                ##   Call between Ltmp99 and Ltmp100
	.long	Lset113
Lset114 = Ltmp101-Lfunc_begin6          ##     jumps to Ltmp101
	.long	Lset114
	.byte	1                       ##   On action: 1
Lset115 = Ltmp100-Lfunc_begin6          ## >> Call Site 10 <<
	.long	Lset115
Lset116 = Lfunc_end6-Ltmp100            ##   Call between Ltmp100 and Lfunc_end6
	.long	Lset116
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
	.private_extern	__ZNSt3__116__pad_and_outputIcNS_11char_traitsIcEEEENS_19ostreambuf_iteratorIT_T0_EES6_PKS4_S8_S8_RNS_8ios_baseES4_ ## -- Begin function _ZNSt3__116__pad_and_outputIcNS_11char_traitsIcEEEENS_19ostreambuf_iteratorIT_T0_EES6_PKS4_S8_S8_RNS_8ios_baseES4_
	.globl	__ZNSt3__116__pad_and_outputIcNS_11char_traitsIcEEEENS_19ostreambuf_iteratorIT_T0_EES6_PKS4_S8_S8_RNS_8ios_baseES4_
	.weak_def_can_be_hidden	__ZNSt3__116__pad_and_outputIcNS_11char_traitsIcEEEENS_19ostreambuf_iteratorIT_T0_EES6_PKS4_S8_S8_RNS_8ios_baseES4_
	.p2align	4, 0x90
__ZNSt3__116__pad_and_outputIcNS_11char_traitsIcEEEENS_19ostreambuf_iteratorIT_T0_EES6_PKS4_S8_S8_RNS_8ios_baseES4_: ## @_ZNSt3__116__pad_and_outputIcNS_11char_traitsIcEEEENS_19ostreambuf_iteratorIT_T0_EES6_PKS4_S8_S8_RNS_8ios_baseES4_
Lfunc_begin7:
	.cfi_startproc
	.cfi_personality 155, ___gxx_personality_v0
	.cfi_lsda 16, Lexception7
## BB#0:
	pushq	%rbp
Lcfi131:
	.cfi_def_cfa_offset 16
Lcfi132:
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
Lcfi133:
	.cfi_def_cfa_register %rbp
	pushq	%r15
	pushq	%r14
	pushq	%r13
	pushq	%r12
	pushq	%rbx
	subq	$72, %rsp
Lcfi134:
	.cfi_offset %rbx, -56
Lcfi135:
	.cfi_offset %r12, -48
Lcfi136:
	.cfi_offset %r13, -40
Lcfi137:
	.cfi_offset %r14, -32
Lcfi138:
	.cfi_offset %r15, -24
	movq	%r8, %r14
	movq	%rcx, %r12
	movq	%rdi, %r13
	testq	%r13, %r13
	je	LBB22_17
## BB#1:
	movl	%r9d, -44(%rbp)         ## 4-byte Spill
	movq	%r12, %rax
	subq	%rsi, %rax
	movq	24(%r14), %rcx
	xorl	%r15d, %r15d
	subq	%rax, %rcx
	cmovgq	%rcx, %r15
	movq	%rdx, -104(%rbp)        ## 8-byte Spill
	movq	%rdx, %rbx
	subq	%rsi, %rbx
	testq	%rbx, %rbx
	jle	LBB22_3
## BB#2:
	movq	(%r13), %rax
	movq	%r13, %rdi
	movq	%rbx, %rdx
	callq	*96(%rax)
	cmpq	%rbx, %rax
	jne	LBB22_17
LBB22_3:
	testq	%r15, %r15
	jle	LBB22_13
## BB#4:
	movq	%r12, -88(%rbp)         ## 8-byte Spill
	movq	%r14, -96(%rbp)         ## 8-byte Spill
	xorps	%xmm0, %xmm0
	movaps	%xmm0, -80(%rbp)
	movq	$0, -64(%rbp)
	cmpq	$23, %r15
	jae	LBB22_8
## BB#5:
	movl	%r15d, %eax
	addb	%al, %al
	movb	%al, -80(%rbp)
	leaq	-79(%rbp), %r14
	movq	%r14, %r12
	jmp	LBB22_9
LBB22_8:
	leaq	16(%r15), %rbx
	andq	$-16, %rbx
	movq	%rbx, %rdi
	callq	__Znwm
	movq	%rax, %r12
	movq	%r12, -64(%rbp)
	orq	$1, %rbx
	movq	%rbx, -80(%rbp)
	movq	%r15, -72(%rbp)
	leaq	-79(%rbp), %r14
LBB22_9:
	movl	-44(%rbp), %eax         ## 4-byte Reload
	movzbl	%al, %esi
	movq	%r12, %rdi
	movq	%r15, %rdx
	callq	_memset
	movb	$0, (%r12,%r15)
	testb	$1, -80(%rbp)
	cmovneq	-64(%rbp), %r14
	movq	(%r13), %rax
Ltmp102:
	movq	%r13, %rdi
	movq	%r14, %rsi
	movq	%r15, %rdx
	callq	*96(%rax)
	movq	%rax, %rbx
Ltmp103:
## BB#10:
	testb	$1, -80(%rbp)
	movq	-96(%rbp), %r14         ## 8-byte Reload
	movq	-88(%rbp), %r12         ## 8-byte Reload
	je	LBB22_12
## BB#11:
	movq	-64(%rbp), %rdi
	callq	__ZdlPv
LBB22_12:
	cmpq	%r15, %rbx
	jne	LBB22_17
LBB22_13:
	movq	-104(%rbp), %rsi        ## 8-byte Reload
	subq	%rsi, %r12
	testq	%r12, %r12
	jle	LBB22_15
## BB#14:
	movq	(%r13), %rax
	movq	%r13, %rdi
	movq	%r12, %rdx
	callq	*96(%rax)
	cmpq	%r12, %rax
	jne	LBB22_17
LBB22_15:
	movq	$0, 24(%r14)
	jmp	LBB22_18
LBB22_17:
	xorl	%r13d, %r13d
LBB22_18:
	movq	%r13, %rax
	addq	$72, %rsp
	popq	%rbx
	popq	%r12
	popq	%r13
	popq	%r14
	popq	%r15
	popq	%rbp
	retq
LBB22_19:
Ltmp104:
	movq	%rax, %rbx
	testb	$1, -80(%rbp)
	je	LBB22_21
## BB#20:
	movq	-64(%rbp), %rdi
	callq	__ZdlPv
LBB22_21:
	movq	%rbx, %rdi
	callq	__Unwind_Resume
Lfunc_end7:
	.cfi_endproc
	.section	__TEXT,__gcc_except_tab
	.p2align	2
GCC_except_table22:
Lexception7:
	.byte	255                     ## @LPStart Encoding = omit
	.byte	155                     ## @TType Encoding = indirect pcrel sdata4
	.byte	41                      ## @TType base offset
	.byte	3                       ## Call site Encoding = udata4
	.byte	39                      ## Call site table length
Lset117 = Lfunc_begin7-Lfunc_begin7     ## >> Call Site 1 <<
	.long	Lset117
Lset118 = Ltmp102-Lfunc_begin7          ##   Call between Lfunc_begin7 and Ltmp102
	.long	Lset118
	.long	0                       ##     has no landing pad
	.byte	0                       ##   On action: cleanup
Lset119 = Ltmp102-Lfunc_begin7          ## >> Call Site 2 <<
	.long	Lset119
Lset120 = Ltmp103-Ltmp102               ##   Call between Ltmp102 and Ltmp103
	.long	Lset120
Lset121 = Ltmp104-Lfunc_begin7          ##     jumps to Ltmp104
	.long	Lset121
	.byte	0                       ##   On action: cleanup
Lset122 = Ltmp103-Lfunc_begin7          ## >> Call Site 3 <<
	.long	Lset122
Lset123 = Lfunc_end7-Ltmp103            ##   Call between Ltmp103 and Lfunc_end7
	.long	Lset123
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
	.asciz	"Couldn't find "

L_.str.3:                               ## @.str.3
	.asciz	"Took %.2f seconds\n"

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
L_.str.6:                               ## @.str.6
	.asciz	"r"


.subsections_via_symbols
