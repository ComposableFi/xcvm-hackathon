fn main() {
    // SQLX depends on `DATABASE_URL`. If we change that, we want cargo to rebuild as the macro needs to recheck.
    println!("run npx hardhat compile in evm_contract folder to get ABI");
    println!("cargo:rerun-if-env-changed=DATABASE_URL");
    println!("cargo:rerun-if-changed=../evm_contracts/build/*");
}
