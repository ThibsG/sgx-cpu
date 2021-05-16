use raw_cpuid::{CpuId, SgxSectionInfo};
use std::fmt;

pub struct SgxCpuInfo(CpuId);

impl SgxCpuInfo {
    pub fn new() -> Self {
        Self { 0: CpuId::new() }
    }
}

impl Into<CpuId> for SgxCpuInfo {
    fn into(self) -> CpuId {
        self.0
    }
}

impl fmt::Display for SgxCpuInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "# CPU features")?;
        let feature_info = self
            .0
            .get_feature_info()
            .expect("Cannot get feature information");
        writeln!(f, "stepping {}", feature_info.stepping_id())?;
        writeln!(
            f,
            "model {} (extended {})",
            feature_info.model_id(),
            feature_info.extended_model_id()
        )?;
        writeln!(
            f,
            "family {} (extended {})",
            feature_info.family_id(),
            feature_info.extended_family_id()
        )?;
        writeln!(f, "{} SMX support", emoji(feature_info.has_smx()))?;

        writeln!(f, "\n# Intel SGX capabilities")?;
        let extended_features = self
            .0
            .get_extended_feature_info()
            .expect("Cannot get extended features information");
        writeln!(f, "{} SGX availability", emoji(extended_features.has_sgx()))?;
        writeln!(
            f,
            "{} SGX launch control configuration",
            emoji(extended_features.has_sgx_lc())
        )?;
        let sgx_info = self.0.get_sgx_info().expect("Cannot get SGX information");
        writeln!(f, "{} SGX 1 support", emoji(sgx_info.has_sgx1()))?;
        writeln!(f, "{} SGX 2 support", emoji(sgx_info.has_sgx2()))?;
        Ok(())
    }
}

fn emoji(b: bool) -> String {
    if b {
        "✅".to_owned()
    } else {
        "❌".to_owned()
    }
}

impl fmt::Debug for SgxCpuInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "# CPU features")?;
        let feature_info = self
            .0
            .get_feature_info()
            .expect("Cannot get feature information");
        writeln!(f, "{:#02x?}", feature_info)?;
        writeln!(f, "stepping {}", feature_info.stepping_id())?;
        writeln!(f, "model {}", feature_info.model_id())?;
        writeln!(f, "extended model {}", feature_info.extended_model_id())?;
        writeln!(f, "family {}", feature_info.family_id())?;
        writeln!(f, "extended family {}", feature_info.extended_family_id())?;
        // TODO add support for processor type in `raw_cpuid` crate
        // writeln!(f, "processor type {}", feature_info.processor_id());
        writeln!(f, "SMX support {}", feature_info.has_smx())?;

        writeln!(f, "\n# Extended feature bits")?;
        let extended_features = self
            .0
            .get_extended_feature_info()
            .expect("Cannot get extended features information");
        writeln!(f, "{:#02x?}", extended_features)?;
        writeln!(f, "SGX available: {}", extended_features.has_sgx())?;
        writeln!(f, "SGX launch control: {}", extended_features.has_sgx_lc())?;

        writeln!(f, "\n# Intel SGX capabilities")?;
        writeln!(f, "\n## Sub-leaf 0 (ECX=0)")?;
        let sgx_info = self.0.get_sgx_info().expect("Cannot get SGX information");
        writeln!(f, "{:#02x?}", sgx_info)?;
        writeln!(f, "SGX 1 supported: {}", sgx_info.has_sgx1())?;
        writeln!(f, "SGX 2 supported: {}", sgx_info.has_sgx2())?;
        writeln!(
            f,
            "MaxEnclaveSize_Not64: {:#02x}",
            sgx_info.max_enclave_size_non_64bit()
        )?;
        writeln!(
            f,
            "MaxEnclaveSize_64: {:#02x}",
            sgx_info.max_enclave_size_64bit()
        )?;

        writeln!(f, "\n## Sub-leaf 1 (ECX=1)")?;
        let (eax, ecx) = sgx_info.secs_attributes();
        writeln!(f, "eax: {:#02x?}, ebx: 0, ecx: {:#02x?}, edx: 0", eax, ecx)?;

        for (idx, SgxSectionInfo::Epc(epc_section)) in sgx_info.iter().enumerate() {
            writeln!(f, "Sub-leaf {} (ECX={})", idx + 2, idx + 2)?;
            writeln!(
                f,
                "EPC (Enclave Page Cache) section:\n{:#02x?}",
                epc_section
            )?;
            writeln!(
                f,
                "physical base address: {:#02x?}",
                epc_section.physical_base()
            )?;
            writeln!(
                f,
                "size of EPC section in Processor Reserved Memory: {} MB",
                epc_section.size() / 1_048_576
            )?;
        }
        Ok(())
    }
}
