
# HybridConsensusChain

## Overview
The **HybridConsensusChain** project is a blockchain implementation that combines **Proof-of-Work (PoW)** and **Proof-of-Stake (PoS)** consensus mechanisms. This project demonstrates the creation of a blockchain that uses PoW for mining blocks and PoS for validating transactions, ensuring a balance between computational security and energy efficiency.

---

## Features
1. **Genesis Block Initialization**:
   - Creates the first block with default values.
2. **Proof-of-Work (PoW)**:
   - Mines blocks by solving a cryptographic puzzle with a given difficulty level.
3. **Proof-of-Stake (PoS)**:
   - Selects a validator based on their stake and validates transactions.
4. **Blockchain State Management**:
   - Maintains the chain of blocks with appropriate links and metadata.

---

## File Structure
```
HybridConsensusChain/
├── src/
│   ├── consensus/
│   │   ├── mod.rs
│   │   ├── pow.rs
│   │   └── pos.rs
│   ├── models/
│   │   ├── mod.rs
│   │   └── block.rs
│   ├── utils/
│   │   └── hash.rs
│   └── main.rs
```

---

## How It Works
### **Genesis Block**
- The first block of the blockchain with index `0`, no validator, and a default hash.

### **Proof-of-Work (PoW)**
- Mines blocks by finding a hash that satisfies the difficulty criteria (hash starts with a specified number of leading zeros).

### **Proof-of-Stake (PoS)**
- Selects a validator based on their stake in the blockchain and validates the block.

---

## Observations
1. **Genesis Block**:
   - **Index**: 0
   - **Data**: `"Genesis Block"`
   - **Hash**: `""` (default value for the genesis block)
   - **Validator**: `None`

2. **Mined Block (PoW)**:
   - **Index**: 1
   - **Data**: `"Transaction Data 1"`
   - **Hash**: `"0000b1a3c23d0825ae91c723046e83ff493155fec47f7089f02ba9ec99d666ed"`
   - **Nonce**: `3898` (nonce value required to solve the PoW puzzle)
   - **Validator**: `None`

3. **Validated Block (PoS)**:
   - **Index**: 2
   - **Data**: `"Transaction Data 2"`
   - **Hash**: `"pos_hash_1735412449680"`
   - **Validator**: `Some("Alice")`

---

## Sample Output

```plaintext
Mined Block: Block { 
    index: 1, 
    timestamp: 1735412449656, 
    data: "Transaction Data 1", 
    previous_hash: "", 
    hash: "0000b1a3c23d0825ae91c723046e83ff493155fec47f7089f02ba9ec99d666ed", 
    nonce: 3898, 
    validator: None 
}
Validated Block: Block { 
    index: 2, 
    timestamp: 1735412449680, 
    data: "Transaction Data 2", 
    previous_hash: "0000b1a3c23d0825ae91c723046e83ff493155fec47f7089f02ba9ec99d666ed", 
    hash: "pos_hash_1735412449680", 
    nonce: 0, 
    validator: Some("Alice") 
}
Blockchain: [
    Block { 
        index: 0, 
        timestamp: 1735412449656, 
        data: "Genesis Block", 
        previous_hash: "0", 
        hash: "", 
        nonce: 0, 
        validator: None 
    },
    Block { 
        index: 1, 
        timestamp: 1735412449656, 
        data: "Transaction Data 1", 
        previous_hash: "", 
        hash: "0000b1a3c23d0825ae91c723046e83ff493155fec47f7089f02ba9ec99d666ed", 
        nonce: 3898, 
        validator: None 
    },
    Block { 
        index: 2, 
        timestamp: 1735412449680, 
        data: "Transaction Data 2", 
        previous_hash: "0000b1a3c23d0825ae91c723046e83ff493155fec47f7089f02ba9ec99d666ed", 
        hash: "pos_hash_1735412449680", 
        nonce: 0, 
        validator: Some("Alice") 
    }
]
```

---

## How to Run

1. **Clone the Repository**:
   ```
   git clone <repository_url>
   cd HybridConsensusChain
   ```

2. **Build the Project**:
   ```
   cargo build
   ```

3. **Run the Project**:
   ```
   cargo run
   ```

4. **Test the Output**:
   - Verify the blockchain creation and output as shown in the "Sample Output" section.

---

## Future Enhancements (Next Action Items)
1. Add transaction fees and rewards for PoW miners and PoS validators.
2. Implement block verification logic to ensure chain integrity.
3. Introduce a graphical user interface for easier blockchain exploration.

---

## Contributing
Feel free to open issues or submit pull requests to enhance the project.

---

## License
This project is licensed under the MIT License.
```

### Notes:
- Replace `<repository_url>` with the actual URL of your Git repository.
- Add any additional observations or output examples if needed.
