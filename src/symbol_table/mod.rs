use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SymbolTable {
    table: HashMap<String, u16>,
    next: u16,
}

impl SymbolTable {
    pub fn new() -> Self {
        let mut st = SymbolTable {
            table: HashMap::new(),
            next: 16,
        };

        st.table.insert("SP".to_string(), 0);
        st.table.insert("LCL".to_string(), 1);
        st.table.insert("ARG".to_string(), 2);
        st.table.insert("THIS".to_string(), 3);
        st.table.insert("THAT".to_string(), 4);
        for ii in 0..=15 {
            st.table.insert(format!("R{}", ii), ii);
        }
        st.table.insert("SCREEN".to_string(), 16384);
        st.table.insert("KBD".to_string(), 24576);

        st
    }

    pub fn add_entry(&mut self, symbol: String, address: u16) {
        self.table.insert(symbol, address);
    }

    pub fn get_address(&mut self, symbol: &str) -> u16 {
        let retrieved = self.table.get(symbol);
        if let Some(address) = retrieved {
            *address
        } else {
            self.table.insert(symbol.to_string(), self.next);
            self.next += 1;
            self.next - 1
        }
    }
}

impl Default for SymbolTable {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_and_get_entry() {
        let mut table = SymbolTable::new();
        table.add_entry("LOOP".to_string(), 4);
        assert_eq!(table.get_address("LOOP"), 4);
    }
}
