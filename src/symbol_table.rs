use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    Int,
    Bool,
    Float,
    Double,
    Char,
    Void,
    Long,
    Short,
    Signed,
    Unsigned,
    String,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub data_type: Option<DataType>,
    pub declared: bool,
    pub line: usize,
    pub column: usize,
}

#[derive(Default)]
pub struct SymbolTable {
    entries: Vec<Symbol>,
    index: HashMap<String, usize>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record(&mut self, name: &str, line: usize, column: usize) {
        if self.index.contains_key(name) {
            return;
        }

        let index = self.entries.len();
        self.entries.push(Symbol {
            name: name.to_string(),
            data_type: None,
            declared: false,
            line,
            column,
        });

        self.index.insert(name.to_string(), index);
    }

    pub fn declare(&mut self, name: &str, data_type: DataType) -> bool {
        if let Some(&index) = self.index.get(name) {
            let symbol = &mut self.entries[index];
            symbol.data_type = Some(data_type);
            symbol.declared = true;
            true
        } else {
            false
        }
    }

    pub fn is_declared(&self, name: &str) -> bool {
        self.index
            .get(name)
            .map(|&idx| self.entries[idx].declared)
            .unwrap_or(false)
    }

    pub fn declare_at(&mut self, index: usize, data_type: DataType) {
        let symbol = &mut self.entries[index];
        symbol.data_type = Some(data_type);
        symbol.declared = true;
    }

    pub fn is_declared_at(&self, index: usize) -> bool {
        self.entries[index].declared
    }

    pub fn data_type_at(&self, index: usize) -> Option<DataType> {
        self.entries[index].data_type
    }

    pub fn get_symbol(&self, name: &str) -> Option<&Symbol> {
        self.index.get(name).map(|&index| &self.entries[index])
    }

    pub fn get_index(&self, name: &str) -> Option<&usize> {
        self.index.get(name)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Symbol> {
        self.entries.iter()
    }

    pub fn name_of(&self, index: usize) -> &str {
        &self.entries[index].name
    }

    pub fn display(&self) {
        if self.entries.is_empty() {
            println!("(Tabela de símbolos vazia)");
            return;
        }

        println!("╔════╦══════════════════════╦════════════╦════════╗");
        println!("║ #  ║ Nome                 ║ Tipo       ║ Linha  ║");
        println!("╠════╬══════════════════════╬════════════╬════════╣");
        for (i, symbol) in self.entries.iter().enumerate() {
            let data_type = symbol
                .data_type
                .map(|t| format!("{:?}", t))
                .unwrap_or_else(|| "-".to_string());
            println!(
                "║ {:<2} ║ {:<20} ║ {:<10} ║ {:<6} ║",
                i, symbol.name, data_type, symbol.line
            );
        }
        println!("╚════╩══════════════════════╩════════════╩════════╝");
    }
}
