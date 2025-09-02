
pub struct ELF64_Rela {
    /// Location to apply the reloc action.
    r_offset: u64,

    /// This member gives both the symbol table index 
    /// with respect to which the relocation must be 
    /// made and the type of relocation to apply. 
    /// Relocation types are processor-specific. 
    r_info: u64,

    /// This member specifies a constant addend used 
    /// to compute the value to be stored into the relocatable field.
    r_addend: Option<u64>,
}
