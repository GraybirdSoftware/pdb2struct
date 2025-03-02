use std::collections::BTreeSet;








pub fn hpp_main(pdb: &mut pdb::PDB<'_, std::fs::File>) -> pdb::Result<String> {    


    let type_information = pdb.type_information()?;
    let mut type_finder = type_information.finder();

    let mut needed_types: BTreeSet<pdb::TypeIndex> = BTreeSet::new();


    
    Ok("test".to_string())
    //"test".to_string()
}


fn type_formatter() { 

}