This is a minimal example for a program crash when using Jemalloc on M1 Macs when building for an x86_64 target. Note that this bug does not occur when building for an aarch64 target. However, since some of our production software depends on C libraries that currently do not support ARM, we are forced to target x86_64 instead of ARM on the Rust side as well.

The program crashes with `Exception: EXC_BAD_ACCESS (code=2, address=0x100f11688)` when run from CLion, or `terminated by signal SIGBUS (Misaligned address error)` when run from command line.

The stack trace is:
```
[Inlined] atomic_store_p atomic.h:83
[Inlined] rtree_leaf_elm_write_commit rtree.h:285
[Inlined] rtree_leaf_elm_write rtree.h:304
emap_rtree_write_acquired emap.c:151
_rjem_je_emap_register_boundary emap.c:173
extent_register_impl extent.c:328
_rjem_je_extent_alloc_wrapper extent.c:1019
_rjem_je_ecache_alloc_grow extent.c:126
pac_alloc_real pac.c:124
pac_alloc_impl pac.c:178
pai_alloc pai.h:43
_rjem_je_pa_alloc pa.c:139
_rjem_je_arena_extent_alloc_large arena.c:338
_rjem_je_large_palloc large.c:37
_rjem_je_arena_palloc arena.c:1230
[Inlined] ipallocztm jemalloc_internal_inlines_c.h:80
_rjem_je_tsd_tcache_data_init tcache.c:717
_rjem_je_tsd_tcache_enabled_data_init tcache.c:644
tsd_data_init tsd.c:244
_rjem_je_tsd_fetch_slow tsd.c:297
[Inlined] tsd_fetch_impl tsd.h:422
[Inlined] tsd_fetch tsd.h:448
_rjem_je_malloc_tsd_boot0 tsd.c:446
malloc_init_hard jemalloc.c:2135
[Inlined] malloc_init jemalloc.c:298
jemalloc_constructor jemalloc.c:4322
invocation function for block in dyld4::Loader::findAndRunAllInitializers(dyld4::RuntimeState&) const 0x00000002010d1d2b
invocation function for block in dyld3::MachOAnalyzer::forEachInitializer(Diagnostics&, dyld3::MachOAnalyzer::VMAddrConverter const&, void (unsigned int) block_pointer, void const*) const 0x00000002010f809b
invocation function for block in dyld3::MachOFile::forEachSection(void (dyld3::MachOFile::SectionInfo const&, bool, bool&) block_pointer) const 0x00000002010ef83a
dyld3::MachOFile::forEachLoadCommand(Diagnostics&, void (load_command const*, bool&) block_pointer) const 0x00000002010bedb3
dyld3::MachOFile::forEachSection(void (dyld3::MachOFile::SectionInfo const&, bool, bool&) block_pointer) const 0x00000002010ef5cb
dyld3::MachOAnalyzer::forEachInitializerPointerSection(Diagnostics&, void (unsigned int, unsigned int, unsigned char const*, bool&) block_pointer) const 0x00000002010f7acc
dyld3::MachOAnalyzer::forEachInitializer(Diagnostics&, dyld3::MachOAnalyzer::VMAddrConverter const&, void (unsigned int) block_pointer, void const*) const 0x00000002010f7d3e
dyld4::Loader::findAndRunAllInitializers(dyld4::RuntimeState&) const 0x00000002010d1c5e
dyld4::Loader::runInitializersBottomUp(dyld4::RuntimeState&, dyld3::Array<dyld4::Loader const*>&) const 0x00000002010d1dea
dyld4::Loader::runInitializersBottomUpPlusUpwardLinks(dyld4::RuntimeState&) const 0x00000002010d1e8e
dyld4::APIs::runAllInitializersForMain() 0x00000002010e5496
dyld4::prepare(dyld4::APIs&, dyld3::MachOAnalyzer const*) 0x00000002010c337d
start 0x00000002010c24d4
```

This issue is preventing us from using Jemalloc when building and testing on M1 Macs, however we do use Jemalloc in all other circumstances without issues.

# System Information

## X86_64 Toolchain (Bug present)
Build & Run
```
$ cargo build
$ file ./target/debug/JemallocTest
./target/debug/JemallocTest: Mach-O 64-bit executable x86_64
$ ./target/debug/JemallocTest
fish: Job 1, './target/debug/JemallocTest' terminated by signal SIGBUS (Misaligned address error)
```

```
$ uname -a
Darwin bcd074405562.ant.amazon.com 21.5.0 Darwin Kernel Version 21.5.0: Tue Apr 26 21:08:37 PDT 2022; root:xnu-8020.121.3~4/RELEASE_ARM64_T6000 arm64
```

```
$ rustup show
Default host: x86_64-apple-darwin
rustup home:  /Users/Xxx/.rustup

installed toolchains
--------------------

stable-aarch64-apple-darwin
stable-x86_64-apple-darwin (default)
nightly-x86_64-apple-darwin

active toolchain
----------------

/Users/Xxx/.rustup/toolchains/stable-x86_64-apple-darwin (overridden by '/Users/Xxx/CLionProjects/JemallocTest/rust-toolchain.toml')
rustc 1.62.1 (e092d0b6b 2022-07-16)
```

## AARCH64 Toolchain (Bug is not present)

Build & Run
```
$ cargo build
$ file ./target/debug/JemallocTest
./target/debug/JemallocTest: Mach-O 64-bit executable arm64
$ ./target/debug/JemallocTest
Hello World!
```

```
$ uname -a
Darwin bcd074405562.ant.amazon.com 21.5.0 Darwin Kernel Version 21.5.0: Tue Apr 26 21:08:37 PDT 2022; root:xnu-8020.121.3~4/RELEASE_ARM64_T6000 arm64
```

```
$ rustup show
Default host: x86_64-apple-darwin
rustup home:  /Users/Xxx/.rustup

installed toolchains
--------------------

stable-aarch64-apple-darwin
stable-x86_64-apple-darwin (default)
nightly-x86_64-apple-darwin

active toolchain
----------------

/Users/Xxx/.rustup/toolchains/stable-aarch64-apple-darwin (overridden by '/Users/Xxx/CLionProjects/JemallocTest/rust-toolchain.toml')
rustc 1.63.0 (4b91a6ea7 2022-08-08)
```