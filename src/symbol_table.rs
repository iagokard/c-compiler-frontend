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
    String,
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

    pub fn get_index(&self, name: &str) -> Option<&usize> {
        self.index.get(name)
    }

    pub fn name_of(&self, index: usize) -> &str {
        &self.entries[index].name
    }

    pub fn display(&self) {
        if self.entries.is_empty() {
            println!("(Tabela de símbolos vazia)");
            return;
        }

        const HEADERS: [&str; 5] = ["#", "Nome", "Tipo", "Linha", "Coluna"];
        const RIGHT_ALIGNED: [bool; 5] = [true, false, false, true, true];

        let rows: Vec<[String; 5]> = self
            .entries
            .iter()
            .enumerate()
            .map(|(i, symbol)| {
                let data_type = symbol
                    .data_type
                    .map(|t| format!("{t:?}"))
                    .unwrap_or_else(|| "-".to_string());
                [
                    i.to_string(),
                    symbol.name.clone(),
                    data_type,
                    symbol.line.to_string(),
                    symbol.column.to_string(),
                ]
            })
            .collect();

        let mut widths = [0usize; 5];
        for (w, h) in widths.iter_mut().zip(HEADERS.iter()) {
            *w = h.chars().count();
        }
        for row in &rows {
            for (w, cell) in widths.iter_mut().zip(row.iter()) {
                *w = (*w).max(cell.chars().count());
            }
        }

        let fmt_cell = |cell: &str, w: usize, right: bool| -> String {
            if right {
                format!("{cell:>w$}", w = w)
            } else {
                format!("{cell:<w$}", w = w)
            }
        };

        let border = |l: &str, m: &str, r: &str| -> String {
            let mut s = String::from(l);
            for (i, w) in widths.iter().enumerate() {
                if i > 0 {
                    s.push_str(m);
                }
                s.push_str(&"═".repeat(w + 2));
            }
            s.push_str(r);
            s
        };

        let total: usize = widths.iter().map(|w| w + 2).sum::<usize>() + (widths.len() - 1);

        println!("{}", border("╔", "╦", "╗"));

        let title = "TABELA DE SÍMBOLOS";
        let pad = total.saturating_sub(title.chars().count());
        let lp = pad / 2;
        let rp = pad - lp;
        println!("║{}{}{}║", " ".repeat(lp), title, " ".repeat(rp));

        println!("{}", border("╠", "╬", "╣"));

        print!("║");
        for (i, (w, h)) in widths.iter().zip(HEADERS.iter()).enumerate() {
            print!(" {} ║", fmt_cell(h, *w, RIGHT_ALIGNED[i]));
        }
        println!();

        println!("{}", border("╠", "╬", "╣"));

        for row in &rows {
            print!("║");
            for (i, (w, cell)) in widths.iter().zip(row.iter()).enumerate() {
                print!(" {} ║", fmt_cell(cell, *w, RIGHT_ALIGNED[i]));
            }
            println!();
        }

        println!("{}", border("╚", "╩", "╝"));
        println!("Total: {} símbolo(s).", self.entries.len());
    }
}
