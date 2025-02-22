use alloy_primitives::{keccak256, Address, U256};
use alloy_sol_types::SolValue;
use crate::{Cheatcode, CheatsCtxt, Result, Vm::*};
use crate::evm::journaled_account;

impl Cheatcode for terrySetStorageAtCall {
    fn apply_stateful(&self, ccx: &mut CheatsCtxt) -> Result {
        let Self { target, slot, value } = *self;
        ensure_not_precompile!(&target, ccx);
        // ensure the account is touched
        let _ = journaled_account(ccx.ecx, target)?;
        // ccx.ecx.sstore(target, slot.into(), value.into())?; 
        ccx.ecx.journaled_state.sstore(target, slot.into(), value.into(), &mut ccx.ecx.db)?;
        Ok(Default::default())
    }
}

impl Cheatcode for terrySetMappingStorageAt_0Call {
    fn apply_stateful(&self, ccx: &mut CheatsCtxt) -> Result {
        let Self { target, slot, key, value } = *self;
        ensure_not_precompile!(&target, ccx);
        // ensure the account is touched
        let _ = journaled_account(ccx.ecx, target)?;

        let data_slot = compute_mapping_slot(slot, key);
        ccx.ecx.journaled_state.sstore(target, data_slot.into(), value.into(), &mut ccx.ecx.db)?;
        Ok(Default::default())
    }
}

impl Cheatcode for terrySetMappingStorageAt_1Call {
    fn apply_stateful(&self, ccx: &mut CheatsCtxt) -> Result {
        let Self { target, slot, key, value } = *self;
        ensure_not_precompile!(&target, ccx);
        // ensure the account is touched
        let _ = journaled_account(ccx.ecx, target)?;
        
        let data_slot = compute_mapping_slot(slot, address_to_u256(key));
        ccx.ecx.journaled_state.sstore(target, data_slot.into(), value.into(), &mut ccx.ecx.db)?;
        Ok(Default::default())
    }
}

fn address_to_u256(address: Address) -> U256 {
    // 将 Address 转换为字节数组
    let bytes = address.as_slice();
    // 将字节数组转换为 U256
    // 注意：U256 是 32 字节，而 Address 是 20 字节，所以需要在高位补零
    let mut u256_bytes = [0u8; 32];
    u256_bytes[12..].copy_from_slice(bytes); // 将 20 字节拷贝到 U256 的低 20 字节
    U256::from_be_bytes(u256_bytes)
}

fn compute_mapping_slot(base_slot: U256, key: U256) -> U256 {
    let mut input = Vec::new();
    input.extend_from_slice(&key.to_be_bytes::<32>());
    input.extend_from_slice(&base_slot.to_be_bytes::<32>());
    let hashed = keccak256(&input);
    U256::from_be_bytes(hashed.0)
}

impl Cheatcode for terryGetStorageAtCall {
    fn apply_stateful(&self, ccx: &mut CheatsCtxt) -> Result {
        let Self { target, slot } = *self;
        
        // let value = ccx.ecx.sload(address, slot.into())?;
        let value = ccx.ecx.journaled_state.sload(target, slot.into(), &mut ccx.ecx.db)?;
        
        // TODO: 冷账户、零值、启用随机存储时默认值的处理
        
        Ok(value.abi_encode())
    }
}

impl Cheatcode for terryGetMappingStorageAt_0Call {
    fn apply_stateful(&self, ccx: &mut CheatsCtxt) -> Result {
        let Self { target, slot, key } = *self;

        // TODO: 冷账户、零值、启用随机存储时默认值的处理
        let data_slot = compute_mapping_slot(slot, key);
        let value = ccx.ecx.journaled_state.sload(target, data_slot.into(), &mut ccx.ecx.db)?;
        
        Ok(value.abi_encode())
    }
}

impl Cheatcode for terryGetMappingStorageAt_1Call {
    fn apply_stateful(&self, ccx: &mut CheatsCtxt) -> Result {
        let Self { target, slot, key } = *self;

        // TODO: 冷账户、零值、启用随机存储时默认值的处理
        let data_slot = compute_mapping_slot(slot, address_to_u256(key));
        let value = ccx.ecx.journaled_state.sload(target, data_slot.into(), &mut ccx.ecx.db)?;
        
        Ok(value.abi_encode())
    }
}