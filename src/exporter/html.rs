//! Exportador HTML com estilos incorporados (não depende de arquivos externos).

use crate::ast::AstNode;
use crate::linemap::LineMap;
use crate::symbol_table::SymbolTable;
use crate::token::Token;

use std::io;
use std::path::Path;

use super::{collect_symbols, html_escape, token_name, token_value, write_output};

/// Gera o documento HTML completo em memória.
pub fn render(
    tokens: &[Token],
    ast: &AstNode,
    symbols: &SymbolTable,
    linemap: &LineMap,
) -> String {
    let symbol_rows = collect_symbols(tokens, symbols, linemap);
    let mut html = String::new();

    html.push_str(
        r#"<!DOCTYPE html>
<html lang="pt-BR">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <meta name="color-scheme" content="dark">
  <title>Relatório do compilador</title>
  <style>
    :root {
      color-scheme: light;
      --bg: #fff7fa;
      --surface: #ffffff;
      --surface-raised: #fff0f5;
      --border: #f1d5e0;
      --text: #573746;
      --muted: #987b89;
      --accent: #d66b98;
      --accent-2: #aa8acb;
      --success: #548d73;
      --warning: #b47a42;
      --shadow: 0 12px 32px rgba(151, 91, 119, .09);
    }
    * { box-sizing: border-box; }
    body {
      margin: 0;
      background: radial-gradient(ellipse at 12% 0%, #ffe5ef 0, transparent 34rem),
                  radial-gradient(ellipse at 95% 18%, #f2eaff 0, transparent 30rem), var(--bg);
      color: var(--text);
      font: 15px/1.6 Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
    }
    .page { width: min(1180px, calc(100% - 40px)); margin: 0 auto; padding: 54px 0 72px; }
    .eyebrow { color: var(--accent); font-size: .76rem; font-weight: 800; letter-spacing: .16em; text-transform: uppercase; }
    h1 { margin: 8px 0 10px; color: #633b50; font-size: clamp(2rem, 5vw, 3.15rem); line-height: 1.12; letter-spacing: -.045em; }
    .intro { max-width: 700px; margin: 0; color: var(--muted); font-size: 1.02rem; }
    .metrics { display: grid; grid-template-columns: repeat(3, minmax(0, 1fr)); gap: 14px; margin: 30px 0 34px; }
    .metric { padding: 17px 19px; border: 1px solid var(--border); border-radius: 15px; background: rgba(255,255,255,.88); box-shadow: var(--shadow); }
    .metric span { display: block; color: var(--muted); font-size: .82rem; }
    .metric strong { display: block; margin-top: 3px; color: #b6537f; font-size: 1.7rem; letter-spacing: -.035em; }
    .section { margin-top: 24px; padding: 24px; border: 1px solid var(--border); border-radius: 18px; background: rgba(255,255,255,.9); box-shadow: var(--shadow); }
    .section-heading { display: flex; align-items: flex-start; justify-content: space-between; gap: 16px; margin-bottom: 18px; }
    h2 { margin: 0; color: #70475b; font-size: 1.22rem; letter-spacing: -.02em; }
    .section-description { margin: 4px 0 0; color: var(--muted); font-size: .88rem; }
    .count { flex: 0 0 auto; padding: 4px 10px; border: 1px solid #efc8d8; border-radius: 999px; background: #fff2f7; color: #b6537f; font-size: .78rem; font-weight: 750; }
    .table-wrap { width: 100%; overflow-x: auto; border: 1px solid var(--border); border-radius: 12px; }
    table { width: 100%; border-collapse: collapse; text-align: left; white-space: nowrap; }
    th, td { padding: 11px 14px; border-bottom: 1px solid #f5e4eb; }
    th { background: #fff0f5; color: #936579; font-size: .75rem; font-weight: 800; letter-spacing: .055em; text-transform: uppercase; }
    td { color: #684b59; font-size: .88rem; }
    tbody tr:last-child td { border-bottom: 0; }
    tbody tr:hover { background: #fff8fb; }
    .index, .position { color: #a58b98; font-variant-numeric: tabular-nums; }
    .code, .ast { font-family: "SFMono-Regular", Consolas, "Liberation Mono", monospace; }
    .code { color: #9b5a86; }
    .category { display: inline-block; padding: 3px 8px; border-radius: 7px; background: #f3eafa; color: #8966a8; font-size: .78rem; }
    .ast { overflow: auto; margin: 0; padding: 21px; border: 1px solid var(--border); border-radius: 12px; background: #fff9fc; color: #765568; font-size: .88rem; line-height: 1.75; tab-size: 4; }
    .status { display: inline-block; padding: 3px 8px; border-radius: 7px; font-size: .76rem; font-weight: 750; }
    .declared { background: #eaf6ef; color: var(--success); }
    .undeclared { background: #fff3e4; color: var(--warning); }
    .empty { padding: 20px; color: var(--muted); text-align: center; }
    footer { margin-top: 28px; color: #ad91a0; font-size: .78rem; text-align: center; }

    .ast-toolbar { display: flex; align-items: center; justify-content: space-between; gap: 12px; margin: 4px 0 16px; }
    .ast-search { display: flex; align-items: center; gap: 10px; min-width: 220px; max-width: 420px; flex: 1; padding: 0 13px; border: 1px solid var(--border); border-radius: 11px; background: #fffafd; color: #a58b98; }
    .ast-search > span { font-size: 1.2rem; }
    .ast-search input { width: 100%; padding: 11px 0; border: 0; outline: 0; background: transparent; color: var(--text); font: inherit; font-size: .86rem; }
    .ast-search input::placeholder { color: #b99eac; }
    .ast-search:focus-within { border-color: #df8eaf; box-shadow: 0 0 0 3px rgba(214, 107, 152, .12); }
    .ast-actions { display: flex; flex-wrap: wrap; gap: 7px; }
    .ast-actions button { padding: 8px 11px; border: 1px solid #edcfdb; border-radius: 9px; background: #fff3f7; color: #925a73; font: inherit; font-size: .76rem; cursor: pointer; transition: border-color .15s, background .15s, transform .15s; }
    .ast-actions button:hover { border-color: #dc9ab5; background: #ffeaf2; }
    .ast-actions button:active { transform: translateY(1px); }
    .ast-actions button[aria-pressed="true"] { border-color: #df8eaf; color: #ad4e79; background: #fce5ef; }
    .ast-workspace { display: grid; grid-template-columns: minmax(0, 1.65fr) minmax(230px, .8fr); gap: 14px; align-items: start; }
    .ast-tree-panel, .ast-inspector { min-width: 0; border: 1px solid var(--border); border-radius: 13px; background: #fffafd; }
    .ast-tree-caption { display: flex; justify-content: space-between; gap: 12px; padding: 12px 15px; border-bottom: 1px solid #f1dfe7; color: #a48796; font-size: .72rem; }
    .ast-tree-caption > span:first-child { color: #82576c; }
    .legend-dot { display: inline-block; width: 7px; height: 7px; margin-right: 6px; border-radius: 50%; background: var(--accent); box-shadow: 0 0 9px rgba(214, 107, 152, .35); }
    .ast-tree { min-height: 220px; max-height: 670px; overflow: auto; padding: 14px 13px 18px; scrollbar-color: #e5b8ca transparent; }
    .ast-branch { position: relative; margin: 2px 0; }
    .ast-branch > .ast-children { position: relative; margin: 3px 0 5px 16px; padding-left: 17px; border-left: 1px solid #efd5e0; }
    .ast-branch > .ast-children::before { position: absolute; top: 0; left: -1px; width: 9px; border-top: 1px solid #efd5e0; content: ""; }
    .ast-node { position: relative; display: flex; align-items: center; gap: 9px; width: fit-content; max-width: 100%; min-height: 34px; padding: 5px 10px 5px 7px; border: 1px solid transparent; border-radius: 8px; color: #67495a; cursor: pointer; font-size: .82rem; text-align: left; transition: background .13s, border-color .13s; }
    .ast-node:hover { border-color: #f0cbd9; background: #fff1f6; }
    .ast-node.is-match:not(.selected) { border-color: #e9b6cb; background: #fff0f6; }
    .ast-node.selected { border-color: #df9bb7; background: #fceaf1; box-shadow: inset 2px 0 0 var(--accent); }
    .ast-node:focus-visible { outline: 2px solid var(--accent); outline-offset: 2px; }
    summary.ast-node { list-style: none; }
    summary.ast-node::-webkit-details-marker { display: none; }
    .ast-chevron, .ast-chevron-spacer { display: inline-grid; flex: 0 0 13px; width: 13px; place-items: center; color: #b18a9d; font-size: 1rem; }
    .ast-chevron { transition: transform .15s; }
    .ast-branch[open] > summary .ast-chevron { transform: rotate(90deg); }
    .ast-symbol { display: inline-grid; flex: 0 0 22px; width: 22px; height: 22px; place-items: center; border-radius: 6px; background: #f3eafa; color: #8966a8; font-size: .7rem; font-weight: 800; }
    .ast-label { overflow: hidden; font-weight: 700; text-overflow: ellipsis; white-space: nowrap; }
    .ast-detail { overflow: hidden; color: #a08493; font-family: "SFMono-Regular", Consolas, monospace; font-size: .74rem; text-overflow: ellipsis; white-space: nowrap; }
    .ast-kind { flex: 0 0 auto; padding: 2px 6px; border: 1px solid #f0e1e8; border-radius: 5px; color: #a18191; font-size: .62rem; letter-spacing: .045em; text-transform: uppercase; }
    .ast-span { display: none; margin-left: 2px; color: #b397a5; font: .68rem "SFMono-Regular", Consolas, monospace; }
    .show-spans .ast-span { display: inline; }
    .kind-program .ast-symbol { color: #438b79; background: #e4f4ef; }
    .kind-function .ast-symbol { color: #8263ad; background: #f0eafa; }
    .kind-declaration .ast-symbol { color: #a77a36; background: #fbf1dc; }
    .kind-assignment .ast-symbol, .kind-binary .ast-symbol { color: #547da9; background: #eaf2fb; }
    .kind-identifier .ast-symbol { color: #54876d; background: #eaf5ed; }
    .kind-integer .ast-symbol, .kind-float .ast-symbol, .kind-string .ast-symbol { color: #b6537f; background: #fce8f0; }
    .kind-token .ast-symbol { color: #9564a8; background: #f3eafa; }
    .ast-inspector { position: sticky; top: 16px; min-height: 260px; padding: 18px; background: linear-gradient(155deg, #fff0f6, #fffafd 65%); }
    .inspector-kicker { color: #b18a9d; font-size: .64rem; font-weight: 850; letter-spacing: .13em; }
    .inspector-icon { display: grid; width: 38px; height: 38px; margin: 17px 0 12px; place-items: center; border: 1px solid #efc6d7; border-radius: 11px; background: #fce8f0; color: var(--accent); font-size: 1.35rem; }
    .inspector-kind { color: #bd5e88; font-size: .72rem; font-weight: 800; letter-spacing: .05em; text-transform: uppercase; }
    .ast-inspector h3 { margin: 5px 0 6px; color: #70475b; font-size: 1.05rem; overflow-wrap: anywhere; }
    .ast-inspector p { min-height: 38px; margin: 0; color: var(--muted); font-size: .8rem; overflow-wrap: anywhere; }
    .inspector-data { display: grid; gap: 9px; margin-top: 20px; padding-top: 15px; border-top: 1px solid var(--border); }
    .inspector-data > div { display: flex; justify-content: space-between; gap: 10px; font-size: .74rem; }
    .inspector-data span { color: #a18493; }
    .inspector-data code { color: #80566b; font-family: "SFMono-Regular", Consolas, monospace; }
    .inspector-hint { margin-top: 18px; padding: 10px; border-radius: 8px; background: #fff0f6; color: #9b7185; font-size: .7rem; line-height: 1.5; }
    .ast-no-results { padding: 20px; color: var(--muted); text-align: center; font-size: .84rem; }
    .ast-branch[hidden], .ast-leaf[hidden] { display: none !important; }
    .sr-only { position: absolute; width: 1px; height: 1px; padding: 0; margin: -1px; overflow: hidden; clip: rect(0,0,0,0); white-space: nowrap; border: 0; }

    @media (max-width: 680px) {
      .page { width: min(100% - 24px, 1180px); padding-top: 34px; }
      .metrics { grid-template-columns: 1fr; gap: 9px; margin: 23px 0; }
      .metric { padding: 13px 16px; }
      .metric strong { font-size: 1.4rem; }
      .section { padding: 16px; }
      .section-heading { flex-direction: column; gap: 8px; }
      .ast-toolbar { align-items: stretch; flex-direction: column; }
      .ast-search { max-width: none; }
      .ast-workspace { grid-template-columns: 1fr; }
      .ast-inspector { position: static; }
      .ast-tree { max-height: 520px; }
      .ast-node { flex-wrap: wrap; }
    }
  </style>
</head>
<body>
  <main class="page">
    <header>
      <div class="eyebrow">Relatório de compilação</div>
      <h1>Resultado do compilador</h1>
      <p class="intro">Visualização consolidada da análise léxica, da árvore sintática abstrata e da tabela de símbolos.</p>
    </header>
"#,
    );

    html.push_str(&format!(
        "    <section class=\"metrics\" aria-label=\"Resumo\">\n      <div class=\"metric\"><span>Tokens</span><strong>{}</strong></div>\n      <div class=\"metric\"><span>Nós / entradas da AST</span><strong>{}</strong></div>\n      <div class=\"metric\"><span>Símbolos</span><strong>{}</strong></div>\n    </section>\n",
        tokens.len(),
        count_ast_nodes(ast),
        symbol_rows.len(),
    ));

    html.push_str("    <section class=\"section\" id=\"tokens\">\n      <div class=\"section-heading\"><div><h2>Lista de tokens</h2><p class=\"section-description\">Tokens reconhecidos pelo analisador léxico, na ordem de leitura.</p></div><span class=\"count\">");
    html.push_str(&format!("{} tokens</span></div>\n", tokens.len()));

    if tokens.is_empty() {
        html.push_str("      <div class=\"empty\">Nenhum token encontrado.</div>\n");
    } else {
        html.push_str("      <div class=\"table-wrap\"><table><thead><tr><th>#</th><th>Categoria</th><th>Valor / Lexema</th><th>Token</th><th>Span</th></tr></thead><tbody>\n");
        for (index, token) in tokens.iter().enumerate() {
            html.push_str(&format!(
                "        <tr><td class=\"index\">{index}</td><td><span class=\"category\">{}</span></td><td class=\"code\">{}</td><td>{}</td><td class=\"position\">{}..{}</td></tr>\n",
                html_escape(token.kind.category()),
                html_escape(&token_value(token)),
                html_escape(&token_name(&token.kind)),
                token.span.start,
                token.span.end,
            ));
        }
        html.push_str("      </tbody></table></div>\n");
    }
    html.push_str("    </section>\n");

    html.push_str(&format!(
        "    <section class=\"section\" id=\"ast\">\n      <div class=\"section-heading\"><div><h2>Árvore sintática abstrata</h2><p class=\"section-description\">Explore a estrutura do programa: expanda os ramos, pesquise nós e selecione um elemento para inspecionar seus detalhes.</p></div><span class=\"count\">{} nós</span></div>\n",
        count_ast_nodes(ast),
    ));
    html.push_str(
        r#"      <div class="ast-toolbar">
        <label class="ast-search" for="ast-search"><span class="sr-only">Pesquisar na AST</span><span aria-hidden="true">⌕</span><input id="ast-search" type="search" placeholder="Pesquisar por nome, tipo ou valor…" autocomplete="off"></label>
        <div class="ast-actions">
          <button type="button" data-ast-action="expand">Expandir tudo</button>
          <button type="button" data-ast-action="collapse">Recolher tudo</button>
          <button type="button" data-ast-action="spans" aria-pressed="false">Mostrar spans</button>
        </div>
      </div>
      <div class="ast-workspace">
        <div class="ast-tree-panel">
          <div class="ast-tree-caption"><span><i class="legend-dot"></i> Hierarquia sintática</span><span>Selecione um nó para inspecionar</span></div>
          <div class="ast-tree" id="ast-tree" aria-label="Árvore sintática">
"#,
    );
    render_ast_node(&mut html, ast, symbols, linemap, 0);
    html.push_str(
        r#"          </div>
          <div class="ast-no-results" id="ast-no-results" hidden>Nenhum nó corresponde à pesquisa.</div>
        </div>
        <aside class="ast-inspector" aria-live="polite" aria-label="Detalhes do nó selecionado">
          <div class="inspector-kicker">INSPEÇÃO DO NÓ</div>
          <div class="inspector-icon" id="inspector-icon">◇</div>
          <div class="inspector-kind" id="inspector-kind">Nenhum nó selecionado</div>
          <h3 id="inspector-label">Selecione um elemento da árvore</h3>
          <p id="inspector-detail">Os detalhes do nó selecionado aparecerão aqui.</p>
          <div class="inspector-data">
            <div><span>Posição (span)</span><code id="inspector-span">—</code></div>
            <div><span>Nível</span><code id="inspector-depth">—</code></div>
            <div><span>Filhos</span><code id="inspector-children">—</code></div>
          </div>
          <div class="inspector-hint">Dica: use a pesquisa para localizar identificadores, declarações, operadores ou literais.</div>
        </aside>
      </div>
    </section>
"#,
    );

    html.push_str(&format!(
        "    <section class=\"section\" id=\"symbols\">\n      <div class=\"section-heading\"><div><h2>Tabela de símbolos</h2><p class=\"section-description\">Identificadores registrados pelo lexer, com tipo e posição da primeira ocorrência.</p></div><span class=\"count\">{} símbolos</span></div>\n",
        symbol_rows.len(),
    ));

    if symbol_rows.is_empty() {
        html.push_str("      <div class=\"empty\">Tabela de símbolos vazia.</div>\n");
    } else {
        html.push_str("      <div class=\"table-wrap\"><table><thead><tr><th>#</th><th>Nome</th><th>Tipo</th><th>Linha</th><th>Coluna</th><th>Status</th></tr></thead><tbody>\n");
        for row in &symbol_rows {
            let (status_class, status_text) = if row.declared {
                ("declared", "Declarado")
            } else {
                ("undeclared", "Não declarado")
            };
            html.push_str(&format!(
                "        <tr><td class=\"index\">{}</td><td class=\"code\">{}</td><td>{}</td><td class=\"position\">{}</td><td class=\"position\">{}</td><td><span class=\"status {}\">{}</span></td></tr>\n",
                row.index,
                html_escape(&row.name),
                html_escape(&row.data_type),
                row.line,
                row.column,
                status_class,
                status_text,
            ));
        }
        html.push_str("      </tbody></table></div>\n");
    }

    html.push_str(
        r#"    </section>
    <footer>Relatório gerado pelo módulo exporter.</footer>
  </main>
  <script>
    (() => {
      const tree = document.getElementById('ast-tree');
      if (!tree) return;
      const search = document.getElementById('ast-search');
      const noResults = document.getElementById('ast-no-results');
      const inspector = {
        kind: document.getElementById('inspector-kind'),
        label: document.getElementById('inspector-label'),
        detail: document.getElementById('inspector-detail'),
        span: document.getElementById('inspector-span'),
        depth: document.getElementById('inspector-depth'),
        children: document.getElementById('inspector-children'),
        icon: document.getElementById('inspector-icon')
      };
      const branchNodes = () => [...tree.querySelectorAll('details.ast-branch')];
      const allNodes = () => [...tree.querySelectorAll('.ast-node')];

      function selectNode(node) {
        allNodes().forEach(item => item.classList.toggle('selected', item === node));
        inspector.kind.textContent = node.dataset.kind || 'Elemento';
        inspector.label.textContent = node.dataset.label || 'Nó';
        inspector.detail.textContent = node.dataset.detail || 'Sem detalhes adicionais.';
        inspector.span.textContent = node.dataset.span || '—';
        inspector.depth.textContent = node.dataset.depth || '—';
        inspector.children.textContent = node.dataset.children || '0';
        inspector.icon.textContent = node.dataset.glyph || '◇';
      }

      tree.addEventListener('click', event => {
        const node = event.target.closest('.ast-node');
        if (node && tree.contains(node)) selectNode(node);
      });
      tree.addEventListener('keydown', event => {
        const node = event.target.closest('.ast-leaf');
        if (node && (event.key === 'Enter' || event.key === ' ')) {
          event.preventDefault();
          selectNode(node);
        }
      });

      search.addEventListener('input', () => {
        const query = search.value.trim().toLocaleLowerCase('pt-BR');
        const nodes = allNodes();
        nodes.forEach(node => {
          const haystack = (node.dataset.search || '').toLocaleLowerCase('pt-BR');
          node.classList.toggle('is-match', !!query && haystack.includes(query));
          if (node.classList.contains('ast-leaf')) node.hidden = !!query && !haystack.includes(query);
        });
        let visibleBranches = 0;
        branchNodes().reverse().forEach(branch => {
          const matched = !query || !!branch.querySelector('.ast-node.is-match');
          branch.hidden = !matched;
          if (matched) {
            visibleBranches++;
            if (query) branch.open = true;
          }
        });
        noResults.hidden = !query || (visibleBranches > 0 || nodes.some(node => node.classList.contains('is-match')));
      });

      document.querySelectorAll('[data-ast-action]').forEach(button => {
        button.addEventListener('click', () => {
          const action = button.dataset.astAction;
          if (action === 'expand' || action === 'collapse') {
            branchNodes().forEach(branch => {
              if (!branch.hidden) branch.open = action === 'expand';
            });
          } else if (action === 'spans') {
            const enabled = tree.classList.toggle('show-spans');
            button.setAttribute('aria-pressed', String(enabled));
            button.textContent = enabled ? 'Ocultar spans' : 'Mostrar spans';
          }
        });
      });

      const firstNode = tree.querySelector('.ast-node');
      if (firstNode) selectNode(firstNode);
    })();
  </script>
</body>
</html>
"#,
    );

    html
}

/// Gera e salva o documento HTML no caminho informado.
pub fn export(
    path: impl AsRef<Path>,
    tokens: &[Token],
    ast: &AstNode,
    symbols: &SymbolTable,
    linemap: &LineMap,
) -> io::Result<()> {
    write_output(path, &render(tokens, ast, symbols, linemap))
}

fn count_ast_nodes(node: &AstNode) -> usize {
    use AstNode::*;
    match node {
        Program(items) => 1 + items.iter().map(count_ast_nodes).sum::<usize>(),
        FunctionDef { body, .. } => 1 + body.iter().map(count_ast_nodes).sum::<usize>(),
        Declaration { init, .. } => 1 + init.as_deref().map(count_ast_nodes).unwrap_or(0),
        Assignment { value, .. } => 1 + count_ast_nodes(value),
        BinaryOp { left, right, .. } => 1 + count_ast_nodes(left) + count_ast_nodes(right),
        Identifier(..) | IntLiteral(..) | FloatLiteral(..) | StringLiteral(..) => 1,
    }
}


/// Renderiza a AST como uma árvore HTML navegável, mantendo os dados da árvore
/// original (inclusive os spans) e sem depender de JavaScript externo.
fn render_ast_node(
    out: &mut String,
    node: &AstNode,
    symbols: &SymbolTable,
    linemap: &LineMap,
    depth: usize,
) {
    use crate::ast::{AstNode::*, BinOp};

    let (kind, label, detail, glyph, children, assignment_operator):
        (&str, String, String, &str, Vec<&AstNode>, bool) = match node {
        Program(items) => (
            "Programa",
            "Programa".to_string(),
            format!("{} item(ns) no nível superior", items.len()),
            "⌘",
            items.iter().collect(),
            false,
        ),
        FunctionDef { name, return_type, body, .. } => (
            "Função",
            "Definição de função".to_string(),
            format!("{} · retorno {:?}", symbols.name_of(*name), return_type),
            "ƒ",
            body.iter().collect(),
            false,
        ),
        Declaration { var, data_type, init, .. } => (
            "Declaração",
            "Declaração".to_string(),
            format!("{} : {:?}", symbols.name_of(*var), data_type),
            "D",
            init.as_deref().into_iter().collect(),
            false,
        ),
        Assignment { target, value, .. } => (
            "Atribuição",
            "Atribuição".to_string(),
            format!("destino: {}", symbols.name_of(*target)),
            "=",
            vec![value.as_ref()],
            true,
        ),
        BinaryOp { op, left, right, .. } => {
            let operator = match *op {
                BinOp::Add => '+',
                BinOp::Sub => '-',
                BinOp::Mul => '*',
                BinOp::Div => '/',
            };
            (
                "Expressão binária",
                "Expressão binária".to_string(),
                format!("operador: {operator}"),
                "±",
                vec![left.as_ref(), right.as_ref()],
                false,
            )
        }
        Identifier(index, _) => (
            "Identificador",
            "Identificador".to_string(),
            format!("{} · índice #{}", symbols.name_of(*index), index),
            "x",
            Vec::new(),
            false,
        ),
        IntLiteral(value, _) => (
            "Literal inteiro",
            "Literal inteiro".to_string(),
            value.to_string(),
            "#",
            Vec::new(),
            false,
        ),
        FloatLiteral(value, _) => (
            "Literal decimal",
            "Literal decimal".to_string(),
            value.to_string(),
            "#",
            Vec::new(),
            false,
        ),
        StringLiteral(value, _) => (
            "Literal string",
            "Literal string".to_string(),
            format!("{value:?}"),
            "\"",
            Vec::new(),
            false,
        ),
    };

    let span = node.span();
    let (start_line, start_column) = linemap.location(span.start);
    let (end_line, end_column) = linemap.location(span.end);
    let span_text = format!(
        "{}..{} · L{}:C{}–L{}:C{}",
        span.start, span.end, start_line, start_column, end_line, end_column
    );
    let visible_children = children.len() + if assignment_operator { 1 } else { 0 };
    let search_text = format!("{kind} {label} {detail} {span_text}");
    let class = match node {
        Program(_) => "kind-program",
        FunctionDef { .. } => "kind-function",
        Declaration { .. } => "kind-declaration",
        Assignment { .. } => "kind-assignment",
        BinaryOp { .. } => "kind-binary",
        Identifier(..) => "kind-identifier",
        IntLiteral(..) => "kind-integer",
        FloatLiteral(..) => "kind-float",
        StringLiteral(..) => "kind-string",
    };

    if visible_children > 0 {
        let open = if depth == 0 { " open" } else { "" };
        out.push_str(&format!(
            "<details class=\"ast-branch\"{open}><summary class=\"ast-node {class}\" data-kind=\"{}\" data-label=\"{}\" data-detail=\"{}\" data-span=\"{}\" data-depth=\"{}\" data-children=\"{}\" data-glyph=\"{}\" data-search=\"{}\"><span class=\"ast-chevron\" aria-hidden=\"true\">›</span><span class=\"ast-symbol\" aria-hidden=\"true\">{}</span><span class=\"ast-label\">{}</span><span class=\"ast-detail\">{}</span><span class=\"ast-kind\">{}</span><span class=\"ast-span\">{}</span></summary><div class=\"ast-children\">",
            html_escape(kind), html_escape(&label), html_escape(&detail), html_escape(&span_text), depth, visible_children, html_escape(glyph), html_escape(&search_text),
            html_escape(glyph), html_escape(&label), html_escape(&detail), html_escape(kind), html_escape(&span_text),
        ));
        if assignment_operator {
            render_ast_auxiliary(out, "Operador", "OP_ATRIB", "=", "—", depth + 1);
        }
        for child in children {
            render_ast_node(out, child, symbols, linemap, depth + 1);
        }
        out.push_str("</div></details>");
    } else {
        out.push_str(&format!(
            "<div class=\"ast-node ast-leaf {class}\" role=\"button\" tabindex=\"0\" data-kind=\"{}\" data-label=\"{}\" data-detail=\"{}\" data-span=\"{}\" data-depth=\"{}\" data-children=\"0\" data-glyph=\"{}\" data-search=\"{}\"><span class=\"ast-chevron-spacer\"></span><span class=\"ast-symbol\" aria-hidden=\"true\">{}</span><span class=\"ast-label\">{}</span><span class=\"ast-detail\">{}</span><span class=\"ast-kind\">{}</span><span class=\"ast-span\">{}</span></div>",
            html_escape(kind), html_escape(&label), html_escape(&detail), html_escape(&span_text), depth, html_escape(glyph), html_escape(&search_text),
            html_escape(glyph), html_escape(&label), html_escape(&detail), html_escape(kind), html_escape(&span_text),
        ));
    }
}

fn render_ast_auxiliary(out: &mut String, kind: &str, label: &str, detail: &str, span: &str, depth: usize) {
    let search = format!("{kind} {label} {detail} {span}");
    out.push_str(&format!(
        "<div class=\"ast-node ast-leaf kind-token\" role=\"button\" tabindex=\"0\" data-kind=\"{}\" data-label=\"{}\" data-detail=\"{}\" data-span=\"{}\" data-depth=\"{}\" data-children=\"0\" data-glyph=\"↳\" data-search=\"{}\"><span class=\"ast-chevron-spacer\"></span><span class=\"ast-symbol\" aria-hidden=\"true\">↳</span><span class=\"ast-label\">{}</span><span class=\"ast-detail\">{}</span><span class=\"ast-kind\">{}</span><span class=\"ast-span\">{}</span></div>",
        html_escape(kind), html_escape(label), html_escape(detail), html_escape(span), depth, html_escape(&search),
        html_escape(label), html_escape(detail), html_escape(kind), html_escape(span),
    ));
}
