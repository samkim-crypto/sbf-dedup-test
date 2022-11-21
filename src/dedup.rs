use {
    crate::id,
    borsh::{BorshDeserialize, BorshSerialize},
    fxhash::FxBuildHasher,
    solana_program::{
        account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, instruction::Instruction,
        msg, pubkey::Pubkey,
    },
    std::collections::HashSet,
};

entrypoint!(process_instruction);
pub fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = DedupInstruction::try_from_slice(instruction_data).unwrap();
    match instruction {
        DedupInstruction::DedupVector { length } => {
            process_dedup_vector(length);
        }
        DedupInstruction::DedupHashSet { length } => {
            process_dedup_hashset(length);
        }
    }
    Ok(())
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub enum DedupInstruction {
    /// Dedup a vector using `Vector::contains` in O(n^2) time
    DedupVector { length: u16 },

    /// Dedup a vector using a HashSet in O(n) time.
    DedupHashSet { length: u16 },
}

pub fn instruction_dedup_vector(length: usize) -> Instruction {
    Instruction {
        program_id: id(),
        accounts: vec![],
        data: DedupInstruction::DedupVector {
            length: length as u16,
        }
        .try_to_vec()
        .unwrap(),
    }
}

pub fn instruction_dedup_hashset(length: usize) -> Instruction {
    Instruction {
        program_id: id(),
        accounts: vec![],
        data: DedupInstruction::DedupHashSet {
            length: length as u16,
        }
        .try_to_vec()
        .unwrap(),
    }
}

fn process_dedup_vector(length: u16) {
    msg!("dedup_vector with length {}", length);

    let vector_pre_dedup: Vec<u16> = (0..length).collect();
    let mut vector_post_dedup = vec![];
    for x in vector_pre_dedup {
        if !vector_post_dedup.contains(&x) {
            vector_post_dedup.push(x);
        }
    }
}

fn process_dedup_hashset(length: u16) {
    msg!("dedup_hashset with length {}", length);

    let vector_pre_dedup: Vec<u16> = (0..length).collect();
    let mut hashset_post_dedup = HashSet::<u16, FxBuildHasher>::default();
    for x in vector_pre_dedup {
        if !hashset_post_dedup.contains(&x) {
            hashset_post_dedup.insert(x);
        }
    }
}
