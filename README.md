# sgx-cpu

A tiny application giving information from your CPU about Intel© SGX support.

This is a Rust rewriting of [`SGX-hardware`](https://github.com/ayeks/SGX-hardware) that can be run easily using `Cargo`.

It is based on [`rust-cpuid`](https://github.com/gz/rust-cpuid) library.

Use the [ark.intel.com](ark.intel.com) database to list all Intel CPUs that have the SGX feature.

## Example

```
$ cargo run
SGX CPU information

# CPU features
stepping 13
model 14 (extended 9)
family 6 (extended 0)
❌ SMX support

# Intel SGX capabilities
✅ SGX availability
✅ SGX launch control configuration
✅ SGX 1 support
❌ SGX 2 support
```

getting details using verbose mode

```
$ cargo run -- -v
SGX CPU information

# CPU features
FeatureInfo {
    eax: 0x906ed,
    ebx: 0x100800,
    edx_ecx: SSE3 | PCLMULQDQ | DTES64 | MONITOR | DSCPL | VMX | EIST | TM2 | SSSE3 | FMA | CMPXCHG16B | PDCM | PCID | SSE41 | SSE42 | X2APIC | MOVBE | POPCNT | TSC_DEADLINE | AESNI | XSAVE | OSXSAVE | AVX | F16C | RDRAND | FPU | VME | DE | PSE | TSC | MSR | PAE | MCE | CX8 | APIC | SEP | MTRR | PGE | MCA | CMOV | PAT | PSE36 | CLFSH | DS | ACPI | MMX | FXSR | SSE | SSE2 | SS | HTT | TM | PBE | 0x0x4800,
}
stepping 13
model 14
extended model 9
family 6
extended family 0
SMX support false

# Extended feature bits
ExtendedFeatures {
    eax: 0x0,
    ebx: FSGSBASE | ADJUST_MSR | SGX | BMI1 | AVX2 | SMEP | BMI2 | REP_MOVSB_STOSB | INVPCID | DEPRECATE_FPU_CS_DS | MPX | RDSEED | ADX | SMAP | CLFLUSHOPT | PROCESSOR_TRACE,
    ecx: SGX_LC,
    edx: 0xbc000600,
}
SGX available: true
SGX launch control: true

# Intel SGX capabilities

## Sub-leaf 0 (ECX=0)
SgxInfo {
    eax: 0x1,
    ebx: 0x0,
    ecx: 0x0,
    edx: 0x241f,
    eax1: 0x36,
    ebx1: 0x0,
    ecx1: 0x1f,
    edx1: 0x0,
}
SGX 1 supported: true
SGX 2 supported: false
MaxEnclaveSize_Not64: 0x1f
MaxEnclaveSize_64: 0x24

## Sub-leaf 1 (ECX=1)
eax: 0x36, ebx: 0, ecx: 0x1f, edx: 0
Sub-leaf 2 (ECX=2)
EPC (Enclave Page Cache) section:
EpcSection {
    eax: 0x70200001,
    ebx: 0x0,
    ecx: 0x5d80001,
    edx: 0x0,
}
physical base address: 0x70200000
size of EPC section in Processor Reserved Memory: 93 MB
```