use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};


use groth16_solana::groth16::{Groth16Verifyingkey, Groth16Verifier};

use ark_bn254;
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize, Compress, Validate};

use std::ops::Neg;
type G1 = ark_bn254::g1::G1Affine;

// Define the entry point of the smart contract
entrypoint!(process_instruction);

const NR_PUBLIC_INPUTS: usize = 1; // Adjust this number as per your requirement

// Main processing function
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {

    // Convert public inputs slice to array of arrays
    let mut public_inputs = [[0u8; 32]; NR_PUBLIC_INPUTS];
    public_inputs[0] = instruction_data[256..288].try_into().unwrap();

    // Proof_a preprocessing
    let proof_a = instruction_data[0..64].try_into().unwrap();
    // let proof_a: G1 = G1::deserialize_with_mode(
    //     &*[&change_endianness(&instruction_data[0..64]), &[0u8][..]].concat(),
    //     Compress::No,
    //     Validate::Yes,
    // ).unwrap();
    // let mut proof_a_neg = [0u8; 65];
    // proof_a
    //     .neg()
    //     .x
    //     .serialize_with_mode(&mut proof_a_neg[..32], Compress::No)
    //     .unwrap();
    // proof_a
    //     .neg()
    //     .y
    //     .serialize_with_mode(&mut proof_a_neg[32..], Compress::No)
    //     .unwrap();

    // let proof_a = change_endianness(&proof_a_neg[..64]).try_into().unwrap();
    let proof_b = instruction_data[64..192].try_into().unwrap();
    let proof_c = instruction_data[192..256].try_into().unwrap();

    // Initialize the verifier
    let mut verifier =
        Groth16Verifier::new(&proof_a, &proof_b, &proof_c, &public_inputs, &VERIFYING_KEY)
            .map_err(|_| ProgramError::Custom(0))?; // Use a custom error code

    // Perform the verification
    let result = verifier.verify();
    match result {
        Ok(true) => msg!("Verification succeeded"),
        Ok(false) => msg!("Verification failed"),
        Err(e) => msg!("Verification error: {:?}", e),
    }

    Ok(())
}

const PI_A_LENGTH: usize = 64;
const PI_B_LENGTH: usize = 128;
const PI_C_LENGTH: usize = 64;
// Adjust the PUBLIC_INPUT_LENGTH as per your requirement
const PUBLIC_INPUT_LENGTH: usize = 32; // Example length

fn change_endianness(bytes: &[u8]) -> Vec<u8> {
    let mut vec = Vec::new();
    for b in bytes.chunks(32) {
        for byte in b.iter().rev() {
            vec.push(*byte);
        }
    }
    vec
}

// Define the VERIFYINGKEY constant
pub const VERIFYING_KEY: Groth16Verifyingkey = Groth16Verifyingkey {
    nr_pubinputs: 2,

    vk_alpha_g1: [
        46, 198, 28, 80, 85, 219, 64, 95, 16, 86, 37, 55, 105, 174, 107, 82, 67, 212, 66, 53, 189,
        244, 65, 129, 153, 249, 14, 192, 208, 23, 189, 255, 46, 213, 140, 248, 251, 76, 224, 235,
        78, 79, 47, 37, 11, 253, 131, 73, 220, 6, 86, 57, 31, 37, 150, 116, 245, 62, 83, 76, 46,
        234, 4, 132,
    ],

    vk_beta_g2: [
        40, 239, 114, 219, 169, 186, 198, 208, 56, 242, 155, 131, 18, 151, 60, 17, 8, 209, 95, 232,
        155, 207, 165, 191, 9, 240, 203, 222, 208, 254, 251, 118, 23, 244, 194, 167, 148, 204, 162,
        27, 134, 119, 235, 184, 191, 212, 15, 40, 60, 80, 108, 153, 207, 223, 171, 38, 222, 36,
        166, 12, 84, 109, 176, 64, 14, 48, 121, 113, 255, 10, 248, 201, 22, 70, 211, 61, 239, 180,
        243, 240, 193, 117, 248, 132, 92, 108, 103, 68, 96, 104, 143, 249, 30, 26, 84, 65, 6, 82,
        118, 185, 31, 130, 171, 49, 7, 175, 0, 68, 128, 209, 81, 253, 68, 111, 106, 183, 12, 127,
        60, 70, 105, 211, 9, 21, 170, 72, 70, 58,
    ],

    vk_gamme_g2: [
        25, 142, 147, 147, 146, 13, 72, 58, 114, 96, 191, 183, 49, 251, 93, 37, 241, 170, 73, 51,
        53, 169, 231, 18, 151, 228, 133, 183, 174, 243, 18, 194, 24, 0, 222, 239, 18, 31, 30, 118,
        66, 106, 0, 102, 94, 92, 68, 121, 103, 67, 34, 212, 247, 94, 218, 221, 70, 222, 189, 92,
        217, 146, 246, 237, 9, 6, 137, 208, 88, 95, 240, 117, 236, 158, 153, 173, 105, 12, 51, 149,
        188, 75, 49, 51, 112, 179, 142, 243, 85, 172, 218, 220, 209, 34, 151, 91, 18, 200, 94, 165,
        219, 140, 109, 235, 74, 171, 113, 128, 141, 203, 64, 143, 227, 209, 231, 105, 12, 67, 211,
        123, 76, 230, 204, 1, 102, 250, 125, 170,
    ],

    vk_delta_g2: [
        24, 192, 59, 38, 123, 253, 143, 209, 31, 2, 6, 232, 161, 211, 127, 130, 243, 195, 167, 30,
        70, 1, 188, 224, 50, 84, 152, 107, 192, 21, 180, 237, 28, 114, 45, 187, 38, 122, 20, 254,
        202, 71, 245, 235, 178, 126, 211, 179, 176, 61, 10, 103, 34, 65, 197, 118, 5, 27, 150, 189,
        46, 60, 94, 185, 35, 207, 37, 209, 197, 84, 87, 106, 62, 27, 41, 116, 198, 235, 14, 90,
        222, 127, 26, 30, 171, 63, 255, 141, 41, 53, 206, 215, 237, 66, 117, 12, 27, 178, 142, 201,
        231, 67, 82, 137, 245, 78, 19, 88, 40, 84, 123, 79, 2, 191, 80, 218, 78, 125, 94, 231, 178,
        101, 121, 12, 31, 4, 17, 239,
    ],

    vk_ic: &[
        [
            26, 117, 81, 38, 33, 231, 103, 28, 33, 207, 192, 9, 40, 163, 239, 213, 143, 75, 215,
            83, 82, 99, 239, 43, 187, 29, 215, 94, 171, 232, 109, 97, 1, 34, 199, 15, 152, 146, 18,
            109, 191, 206, 154, 14, 65, 233, 113, 21, 117, 171, 39, 197, 154, 75, 176, 199, 151,
            75, 117, 63, 170, 66, 13, 98,
        ],
        [
            26, 34, 168, 154, 124, 160, 104, 241, 73, 142, 20, 231, 181, 8, 8, 182, 0, 225, 51,
            233, 173, 217, 93, 237, 166, 202, 87, 151, 55, 51, 87, 197, 43, 145, 207, 212, 5, 45,
            208, 198, 22, 238, 212, 232, 126, 17, 37, 125, 180, 176, 67, 75, 207, 58, 102, 122,
            182, 244, 89, 209, 133, 253, 4, 255,
        ],
    ],
};

#[cfg(test)]
mod tests {
    use super::*;
    use solana_program::{
        account_info::AccountInfo,
        pubkey::Pubkey,
    };

    pub const PROOF: [u8; 256] = [
            12,  69, 221, 178, 220, 208,  17,   7, 234,  16,  51,
            42,  50, 106, 219, 202,  78, 222, 119, 169, 184, 211,
            76, 131,  71,  57,  32, 171, 185, 117, 128,  16,  57,
            21,  36, 112,  85,  59, 210,  62, 247, 220, 209,  19,
           143, 247,  78, 219,  32,  24, 110,  50, 216, 217, 187,
            60, 168,  79, 131, 178, 173,  42, 188,   0,
            25, 120,  46,  12, 233, 167, 180, 171, 145, 195, 225,
           117, 135,  14, 180,  12, 140, 147,   2, 177, 137, 216,
           241, 194,  23,  22,  61,  40,  28,  89, 230,  52,  25,
           166,  27, 205, 124, 163,  48,  98, 183, 127,  29, 181,
            94,  14,  38,  62,  19, 187, 151,  60, 130,  14, 154,
           120,  71, 160,  49, 154,  34,  93, 156, 154,
             7, 232,  14,   4, 178, 212,  38, 159,  87, 240,   3,
           186,  93,   8,  66, 138, 173, 169, 181, 153,   1, 193,
            92, 226, 141,  99, 141,  28,  41, 157,  66,  46,  45,
            51,  55, 253, 230, 173, 224, 134,  91, 167,  50, 116,
           115, 239, 241, 103, 242,  52, 169,  28, 148, 247,  27,
           130,  63, 238, 180, 195,  46,  57, 211, 248,
            48,  35,   8,  20, 104, 100,  83,  77,  95, 105,   9,
           233,  82, 245, 216, 125, 126, 127, 220, 152, 182,  53,
             9, 178,  58, 100, 117, 162, 132, 132,  15, 181,   1,
            16,  88, 217,  88, 119,  90, 130, 202,  73,  55, 198,
           207,  73, 113,  38, 158, 182, 118, 210,  97,  57, 149,
           122, 179, 153,   7,  40, 189, 105,  54, 232
    ];

    pub const PUBLIC_INPUTS: [[u8; 32]; 1] = [
        [
            0, 0, 0, 0,  0, 0, 0, 0, 0,
            0, 0, 0, 0,  0, 0, 0, 0, 0,
            0, 0, 0, 0,  0, 0, 0, 0, 0,
            0, 0, 0, 0, 12
        ]
    ];

    // Helper function to create a mock AccountInfo with a specified lifetime
    fn mock_account_info<'a>(
        key: &'a Pubkey, 
        lamports: &'a mut u64, 
        data: &'a mut [u8], 
        owner: &'a Pubkey
    ) -> AccountInfo<'a> {
        AccountInfo::new(
            key,
            false,
            true,
            lamports,
            data,
            owner,
            false,
            0,
        )
    }

    #[test]
    fn test_process_instruction() {
        let program_id = Pubkey::new_unique();
        let key = Pubkey::new_unique();
        let owner = Pubkey::new_unique();
        let mut lamports = 0;
        let mut data = vec![0; 100]; // Adjust the size as needed
        let account = mock_account_info(&key, &mut lamports, &mut data, &owner);
        let accounts = vec![account];

        // Create instruction data using valid proof and public inputs
        let mut instruction_data = Vec::new();
        instruction_data.extend_from_slice(&PROOF); // Valid proof
        for input in PUBLIC_INPUTS.iter() {
            instruction_data.extend_from_slice(input); // Valid public inputs
        }

        // Call the process_instruction function
        let result = process_instruction(&program_id, &accounts, &instruction_data);

        // Assert the expected result
        assert!(result.is_ok());
        // Additional assertions based on your program's logic
    }
}
