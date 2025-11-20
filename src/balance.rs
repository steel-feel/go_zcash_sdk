use anyhow::anyhow;
use iso_currency::Currency;
use rust_decimal::{prelude::FromPrimitive, Decimal};
use tracing::{info, warn};
use uuid::Uuid;
use zcash_client_backend::{
    data_api::{wallet::ConfirmationsPolicy, Account as _, WalletRead},
    tor,
};
use zcash_client_sqlite::WalletDb;
use zcash_keys::keys::UnifiedAddressRequest;
use zcash_protocol::value::{Zatoshis, COIN};

use crate::{config::{get_wallet_network, select_account}, data::get_db_paths, error};


pub async fn wallet_balance(wallet_name: String, account_id:Option<Uuid>  ) -> Result<(), anyhow::Error> 
{
       let wallet_dir: Option<String> = Some(wallet_name.to_owned());
       let params = get_wallet_network(wallet_dir.as_ref())?;

        let (_, db_data) = get_db_paths(wallet_dir.as_ref());
        let db_data = WalletDb::for_path(db_data, params, (), ())?;
        let account = select_account(&db_data, account_id)?;

        let address = db_data
            .get_last_generated_address_matching(
                account.id(),
                UnifiedAddressRequest::AllAvailableKeys,
            )?
            .ok_or(error::Error::InvalidRecipient)?;

        // let printer = if let Some(currency) = self.convert {
        //     let tor = tor_client(wallet_dir.as_ref()).await?;
        //     ValuePrinter::with_exchange_rate(&tor, currency).await?
        // } else {
        //     ValuePrinter::ZecOnly
        // };

        if let Some(wallet_summary) = db_data.get_wallet_summary(ConfirmationsPolicy::default())? {
            let balance = wallet_summary
                .account_balances()
                .get(&account.id())
                .ok_or_else(|| anyhow!("Missing account 0"))?;

            println!("{}", address.encode(&params));
            println!("     Height: {}", wallet_summary.chain_tip_height());
            let scan_progress = wallet_summary.progress().scan();
            println!(
                "     Synced: {:0.3}%",
                (*scan_progress.numerator() as f64) * 100f64
                    / (*scan_progress.denominator() as f64)
            );
            if let Some(progress) = wallet_summary.progress().recovery() {
                println!(
                    "     Recovered: {:0.3}%",
                    (*progress.numerator() as f64) * 100f64 / (*progress.denominator() as f64)
                );
            }
            println!("    Balance: {:?}", (balance.total()));
            println!(
                "     Sapling Spendable: {:?}",
                (balance.sapling_balance().spendable_value()),
            );
            println!(
                "     Orchard Spendable: {:?}",
                (balance.orchard_balance().spendable_value()),
            );
            #[cfg(feature = "transparent-inputs")]
            println!(
                "  Unshielded Spendable: {:?}",
                (balance.unshielded_balance().spendable_value()),
            );
        } else {
            println!("Insufficient information to build a wallet summary.");
        }

        Ok(())
    }

