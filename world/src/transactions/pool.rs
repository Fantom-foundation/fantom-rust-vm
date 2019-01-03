//! Holds a pool of transactions

use transactions::Transaction;

pub struct TransactionPool {
    transactions: Vec<Transaction>
}