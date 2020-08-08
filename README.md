This is an example showing broken behavior in a rust project.
```
E:\software\rust\broken_loop>cargo build
   Compiling typenum v1.12.0
   Compiling semver-parser v0.7.0
   Compiling proc-macro2 v1.0.19
   Compiling unicode-xid v0.2.1
   Compiling cortex-m v0.6.3
   Compiling stable_deref_trait v1.2.0
   Compiling syn v1.0.36
   Compiling vcell v0.1.2
   Compiling bitfield v0.13.2
   Compiling cortex-m-rt v0.6.12
   Compiling cortex-m-semihosting v0.3.5
   Compiling r0 v0.2.2
   Compiling broken_loop v0.1.0 (E:\software\rust\broken_loop)
   Compiling panic-halt v0.2.0
   Compiling volatile-register v0.2.0
   Compiling semver v0.9.0
   Compiling rustc_version v0.2.3
   Compiling quote v1.0.7
   Compiling bare-metal v0.2.5
   Compiling generic-array v0.13.2
   Compiling generic-array v0.12.3
   Compiling as-slice v0.1.3
   Compiling aligned v0.3.4
   Compiling cortex-m-rt-macros v0.1.8
    Finished dev [unoptimized + debuginfo] target(s) in 23.60s


E:\software\rust\broken_loop>cargo objdump -- --disassemble-functions=main
    Finished dev [unoptimized + debuginfo] target(s) in 0.07s

broken_loop:    file format ELF32-arm-little


Disassembly of section .text:

000000c0 main:
      c0: 80 b5                         push    {r7, lr}
      c2: 00 af                         add     r7, sp, #0
      c4: 00 f0 02 f8                   bl      #4
      c8: fe de                         trap
      ca: d4 d4                         bmi     #-88 <tw4w0r638ityor3+0x76>

E:\software\rust\broken_loop>


E:\software\rust\broken_loop>cargo build
   Compiling typenum v1.12.0
   Compiling semver-parser v0.7.0
   Compiling proc-macro2 v1.0.19
   Compiling unicode-xid v0.2.1
   Compiling cortex-m v0.6.3
   Compiling stable_deref_trait v1.2.0
   Compiling syn v1.0.36
   Compiling vcell v0.1.2
   Compiling bitfield v0.13.2
   Compiling cortex-m-rt v0.6.12
   Compiling cortex-m-semihosting v0.3.5
   Compiling r0 v0.2.2
   Compiling broken_loop v0.1.0 (E:\software\rust\broken_loop)
   Compiling panic-halt v0.2.0
   Compiling volatile-register v0.2.0
   Compiling semver v0.9.0
   Compiling rustc_version v0.2.3
   Compiling quote v1.0.7
   Compiling bare-metal v0.2.5
   Compiling generic-array v0.13.2
   Compiling generic-array v0.12.3
   Compiling as-slice v0.1.3
   Compiling aligned v0.3.4
   Compiling cortex-m-rt-macros v0.1.8
    Finished dev [unoptimized + debuginfo] target(s) in 23.60s

E:\software\rust\broken_loop>cargo objdump -- --disassemble-functions=main
    Finished dev [unoptimized + debuginfo] target(s) in 0.07s

broken_loop:    file format ELF32-arm-little


Disassembly of section .text:

000000c0 main:
      c0: 80 b5                         push    {r7, lr}
      c2: 00 af                         add     r7, sp, #0
      c4: 00 f0 02 f8                   bl      #4
      c8: fe de                         trap
      ca: d4 d4                         bmi     #-88 <tw4w0r638ityor3+0x76>

E:\software\rust\broken_loop>rustc -V
rustc 1.44.0 (49cae5576 2020-06-01)

E:\software\rust\broken_loop>

```
