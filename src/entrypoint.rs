//entrypoint.rs is used as entrypoint to the program
entrypoint!(process_instruction); //call the program, all calls go thru here
fn process_instruction( //BPF loader, w/ 3 arguments below
    program_id: &Pubkey, //current program id
    accounts: &[AccountInfo],//used to store state
    instruction_data: &[u8],//data passed by caller
) -> ProgramResult {
    msg!(
        "process_instruction: {}: {} accounts, data={:?}",
        program_id,
        accounts.len(),
        instruction_data
    );
    Ok(())
}