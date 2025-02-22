use alloy_sol_types::SolValue;
use crate::{Cheatcode, Cheatcodes, CheatsCtxt, Result, Vm::*};
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

impl Cheatcode for terryGetStorageAtCall {
    fn apply_stateful(&self, ccx: &mut CheatsCtxt) -> Result {
        let Self { target, slot } = *self;
        
        // let value = ccx.ecx.sload(address, slot.into())?;
        let value = ccx.ecx.journaled_state.sload(target, slot.into(), &mut ccx.ecx.db)?;
        
        // TODO: 冷账户、零值、启用随机存储时默认值的处理
        
        Ok(value.abi_encode())
    }
}