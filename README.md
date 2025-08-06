# 🧾 SolTokenTerminal

SolTokenTerminal is a Rust-based command-line interface (CLI) tool that enables developers and power users to seamlessly interact with Solana tokens. It supports standard and advanced SPL token operations including minting, transferring, confidential transfers, and querying balances.

---

## ✨ Features

- ✅ **Mint Tokens**  
  Mint new SPL tokens to any associated token account.

- 🔁 **Transfer Tokens**  
  Perform standard SPL token transfers between wallets.

- 🕵️‍♂️ **Confidential Transfers**  
  Use Solana's confidential transfer extensions for private transactions.

- 💰 **Check Balances**  
  View token balances of any wallet (including associated token accounts).

- 🧩 **Modular Design**  
  Easily extendable to support additional SPL token extensions or other Solana programs.

---
What is a Confidential Transfer?
A confidential token transfer system ensures:

Sender, receiver, and amount are obfuscated from public view.

Transfer validity (e.g., no double spending, sufficient balance) is verifiable.

Only authorized parties can view details (if allowed).