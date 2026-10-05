#!/usr/bin/env python3
"""Generate/check a source-backed architecture atlas; no Rust code is executed.

Requires Python >=3.11 and architecture-requirements.txt. This is deliberately
a syntactic index, not compiler type inference. Unresolved calls remain explicit.
"""
from __future__ import annotations

import argparse
from collections import Counter, defaultdict
import hashlib
from functools import lru_cache
import json
import os
from pathlib import Path
import re
import subprocess
import sys

from tree_sitter import Language, Parser
import tree_sitter_rust

ROOT = Path(__file__).resolve().parents[1]
OUT = ROOT / "docs/architecture/generated"
PARSER = Parser(Language(tree_sitter_rust.language()))
SYMBOLS = {"function_item", "function_signature_item", "struct_item", "enum_item",
           "trait_item", "type_item", "const_item", "static_item", "macro_definition"}


def text(node):
    return node.text.decode("utf-8") if node else ""


def field(node, name):
    return node.child_by_field_name(name)


def walk(node):
    yield node
    for child in node.named_children:
        yield from walk(child)


def attrs(node):
    result = []
    prev = node.prev_named_sibling
    while prev and prev.type in {"attribute_item", "line_comment", "block_comment"}:
        if prev.type == "attribute_item":
            result.insert(0, text(prev))
        prev = prev.prev_named_sibling
    return result


def visibility(node):
    return next((text(c) for c in node.named_children if c.type == "visibility_modifier"), "private")


def imports(node, prefix=""):
    """Return (local alias, path), including public and wildcard use trees."""
    if node.type == "scoped_use_list":
        scope = text(field(node, "path"))
        listing = field(node, "list") or node.named_children[-1]
        for child in listing.named_children:
            yield from imports(child, prefix + scope + "::")
    elif node.type == "use_list":
        for child in node.named_children:
            yield from imports(child, prefix)
    elif node.type == "use_as_clause":
        path = text(field(node, "path") or node.named_children[0])
        yield text(field(node, "alias") or node.named_children[-1]), prefix + path
    elif node.type == "use_wildcard":
        yield "*", prefix + text(node)
    else:
        value = prefix + text(node)
        if value.endswith("::self"):
            value = value[:-6]
        yield value.split("::")[-1], value


def rel(path):
    return str(Path(path).relative_to(ROOT))


def link(path, line=None, origin=None):
    target = os.path.relpath(ROOT / path, (origin or OUT / "index.md").parent)
    return target + (f"#L{line}" if line else "")


def esc(value):
    return str(value).replace("|", "&#124;").replace("\n", " ").replace("`", "'")


def mermaid_label(value):
    return str(value).replace("\n", " ").replace('"', "'").replace("<", "&lt;").replace(">", "&gt;")


def analyze():
    metadata = json.loads(subprocess.check_output(
        ["cargo", "metadata", "--no-deps", "--format-version", "1", "--offline"], cwd=ROOT))
    packages, files, symbols, uses, modules, calls, macros, includes = [], [], [], [], [], [], [], []
    bodies = {}
    expected_symbols = Counter()
    expected_calls = 0
    for pkg in sorted(metadata["packages"], key=lambda p: p["name"]):
        base = Path(pkg["manifest_path"]).parent
        targets = [{"name": t["name"], "kind": t["kind"], "path": rel(t["src_path"])}
                   for t in pkg["targets"]]
        package = {"name": pkg["name"], "directory": base.name, "manifest": rel(pkg["manifest_path"]),
                   "targets": targets, "dependencies": [
                       {"name": d["name"], "alias": d.get("rename") or d["name"].replace("-", "_"),
                        "kind": d["kind"] or "normal", "target": d.get("target")}
                       for d in pkg["dependencies"] if d.get("path")]}
        packages.append(package)
        target_paths = {t["path"]: t for t in targets}
        source_paths = sorted(set(base.glob("src/**/*.rs")) | set(base.glob("tests/**/*.rs"))
                              | set(base.glob("examples/**/*.rs")) | set(base.glob("benches/**/*.rs"))
                              | ({base / "build.rs"} if (base / "build.rs").exists() else set()))
        for path in source_paths:
            relative = rel(path)
            src_rel = path.relative_to(base)
            parts = list(src_rel.parts)
            target = target_paths.get(relative)
            kind = (target["kind"][0] if target else
                    "test" if parts[0] == "tests" else "example" if parts[0] == "examples" else "source")
            is_test = parts[0] in {"tests", "benches"} or "fixture" in (target or {}).get("name", "")
            if parts[0] == "src":
                module_parts = parts[1:]
                module_parts[-1] = Path(module_parts[-1]).stem
                if module_parts[-1] in {"lib", "mod"}:
                    module_parts.pop()
                module = "::".join(module_parts)
            else:
                module = "::".join(parts).removesuffix(".rs")
            # Binary roots must not collide with their package's library root.
            scope = pkg["name"] + ("::" + module if module else "")
            source = path.read_bytes()
            tree = PARSER.parse(source)
            if tree.root_node.has_error:
                raise ValueError(f"Rust parse error in {relative}; refusing incomplete inventory")
            for node in walk(tree.root_node):
                if node.type in SYMBOLS and text(field(node, "name")):
                    expected_symbols[node.type] += 1
                if node.type == "call_expression":
                    expected_calls += 1
            files.append({"path": relative, "package": pkg["name"], "scope": scope,
                          "kind": kind, "test": is_test, "sha256": hashlib.sha256(source).hexdigest()})

            def visit(node, scope, owner=None, trait_impl=False, inherited_test=is_test, inherited_cfg=()):
                a = attrs(node)
                cfg = tuple(inherited_cfg) + tuple(x for x in a if "cfg" in x)
                test = inherited_test or any(re.search(r"\btest\b", re.sub(r"not\s*\(\s*test\s*\)", "", x)) for x in a if "cfg" in x)
                if node.type == "macro_invocation" and text(field(node, "macro")) == "include":
                    literal = re.fullmatch(r'include!\s*\(\s*"([^"]+)"\s*\)', text(node))
                    if literal:
                        included = path.parent/literal[1]
                        includes.append({"from": relative, "path": rel(included), "test": test,
                                         "scope": scope, "line": node.start_point.row+1})
                if node.type in {"mod_item", "impl_item", "trait_item"}:
                    if node.type == "mod_item":
                        name = text(field(node, "name"))
                        modules.append({"package": pkg["name"], "scope": scope, "name": name,
                                        "path": relative, "line": node.start_point.row + 1,
                                        "visibility": visibility(node), "attributes": a,
                                        "inline": field(node, "body") is not None,
                                        "test": test})
                        scope += "::" + name
                    elif node.type == "impl_item":
                        owner = re.sub(r"<.*>", "", text(field(node, "type")))
                        trait_impl = field(node, "trait") is not None
                    elif node.type == "trait_item":
                        owner = text(field(node, "name"))
                        trait_impl = True
                if node.type == "use_declaration":
                    arg = field(node, "argument") or node.named_children[-1]
                    for alias, target_path in imports(arg):
                        uses.append({"scope": scope, "alias": alias, "target": target_path,
                                     "visibility": visibility(node), "path": relative,
                                     "line": node.start_point.row + 1})
                if node.type in SYMBOLS:
                    name = text(field(node, "name"))
                    if name:
                        qualified = scope + "::" + (owner + "::" if owner and node.type.startswith("function") else "") + name
                        item = {"id": f"{relative}:{node.start_point.row + 1}:{name}", "name": name,
                                "qualified": qualified, "scope": scope, "owner": owner,
                                "trait_impl": trait_impl, "kind": node.type,
                                "visibility": visibility(node), "path": relative,
                                "line": node.start_point.row + 1, "end_line": node.end_point.row + 1,
                                "test": test or any(re.match(r"#\[(?:[\w]+::)?test(?:\(|\])", x) for x in a), "cfg": list(cfg),
                                "package": pkg["name"]}
                        # A trait_item itself is not an implementation member.
                        if node.type == "trait_item":
                            item["qualified"] = scope + "::" + name
                        symbols.append(item)
                        if node.type == "function_item":
                            body = field(node, "body")
                            bodies[item["id"]] = body
                            # Local items can also live inside nested blocks or extern blocks.
                            def local_items(n):
                                for child in n.named_children:
                                    if child.type in SYMBOLS | {"mod_item", "impl_item"}:
                                        visit(child, qualified, None, False, test, cfg)
                                    else:
                                        local_items(child)
                            if body:
                                local_items(body)
                            return
                        if node.type in {"const_item", "static_item"}:
                            bodies[item["id"]] = field(node, "value")
                for child in node.named_children:
                    visit(child, scope, owner, trait_impl, test, cfg)

            visit(tree.root_node, scope)

    # Propagate #[cfg(test)] from out-of-line module declarations to their files.
    # This covers src/live_tests.rs helpers, which have no #[test] themselves.
    test_files = {f["path"] for f in files if f["test"]}
    test_files.update(i["path"] for i in includes if i["test"])
    changed = True
    while changed:
        changed = False
        for m in modules:
            if m["inline"] or not (m["test"] or m["path"] in test_files):
                continue
            parent = ROOT / m["path"]
            directory = parent.parent if parent.stem in {"lib", "main", "mod"} else parent.with_suffix("")
            explicit = next((re.search(r'path\s*=\s*"([^"]+)"',a) for a in m["attributes"] if "path" in a),None)
            candidates = [parent.parent / explicit[1]] if explicit else [directory/(m["name"]+".rs"),directory/m["name"]/"mod.rs"]
            for candidate in candidates:
                if candidate.is_file() and rel(candidate) not in test_files:
                    test_files.add(rel(candidate)); changed = True
    for f in files:
        f["test"] = f["test"] or f["path"] in test_files
    for s in symbols:
        s["test"] = s["test"] or s["path"] in test_files

    by_qualified = defaultdict(list)
    by_name = defaultdict(list)
    imports_by_scope = defaultdict(list)
    for symbol in symbols:
        by_qualified[symbol["qualified"]].append(symbol)
        if symbol["kind"] == "function_item":
            by_name[symbol["name"]].append(symbol)
    for use in uses:
        imports_by_scope[use["scope"]].append(use)
    package_map = {p["name"]: p for p in packages}
    known_prefixes = {"::".join(s["qualified"].split("::")[:n])
                      for s in symbols for n in range(1, len(s["qualified"].split("::")) + 1)}

    @lru_cache(maxsize=50000)
    def paths(value, scope, depth=0):
        if depth > 8:
            return set()
        value = re.sub(r"::<[^<>]*>", "", value).strip()
        parts = value.split("::")
        package = scope.split("::")[0]
        if parts[0] == "crate":
            return {package + "::" + "::".join(parts[1:])}
        if parts[0] == "self":
            return {scope + "::" + "::".join(parts[1:])}
        if parts[0] == "super":
            return paths("::".join(parts[1:]), scope.rsplit("::", 1)[0], depth + 1)
        dep = next((d for d in package_map[package]["dependencies"] if d["alias"] == parts[0]), None)
        if dep:
            return {dep["name"] + ("::" + "::".join(parts[1:]) if len(parts) > 1 else "")}
        # A package binary can import its own library by the library target name.
        if any(t["kind"] == ["lib"] and t["name"] == parts[0] for t in package_map[package]["targets"]):
            return {package + ("::" + "::".join(parts[1:]) if len(parts) > 1 else "")}
        result = {scope + "::" + value}
        for u in imports_by_scope[scope]:
            if u["alias"] == parts[0]:
                result |= paths(u["target"], scope, depth + 1) if len(parts) == 1 else {
                    p + "::" + "::".join(parts[1:]) for p in paths(u["target"], scope, depth + 1)}
            elif u["alias"] == "*" and scope + "::" + value not in known_prefixes:
                result |= {p + "::" + value for p in paths(u["target"].removesuffix("::*"), scope, depth + 1)
                           if p + "::" + value in known_prefixes}
        return result

    @lru_cache(maxsize=50000)
    def expand_exports(path, depth=0):
        if depth > 8:
            return {path}
        result = {path}
        parts = path.split("::")
        for n in range(1, len(parts)):
            parent, name = "::".join(parts[:n]), parts[n]
            for u in imports_by_scope[parent]:
                if u["visibility"] == "private":
                    continue
                if u["alias"] == name:
                    for p in paths(u["target"], parent):
                        result |= expand_exports(p + ("::" + "::".join(parts[n+1:]) if parts[n+1:] else ""), depth+1)
                elif u["alias"] == "*" and path not in known_prefixes:
                    for p in paths(u["target"].removesuffix("::*"), parent):
                        candidate = p + "::" + "::".join(parts[n:])
                        if candidate in known_prefixes:
                            result |= expand_exports(candidate, depth+1)
        return result

    def body_nodes(node):
        if node is None:
            return
        yield node
        for child in node.named_children:
            if child.type not in SYMBOLS | {"mod_item", "impl_item"}:
                yield from body_nodes(child)

    for symbol in symbols:
        if symbol["id"] not in bodies:
            continue
        # A locally bound callable can shadow an imported/free function name.
        bindings = {text(field(n, "pattern")).removeprefix("mut ") for n in body_nodes(bodies[symbol["id"]])
                    if n.type == "let_declaration" and field(n, "pattern") is not None}
        for node in body_nodes(bodies[symbol["id"]]):
            if node.type == "macro_invocation":
                macros.append({"caller": symbol["id"], "line": node.start_point.row+1,
                               "expression": text(field(node, "macro"))})
            if node.type != "call_expression":
                continue
            fn = field(node, "function")
            expr = text(fn)
            original = expr
            if fn.type == "generic_function":
                fn = field(fn, "function") or fn.named_children[0]
                expr = text(fn)
            targets = set()
            resolution = "unresolved"
            if expr.startswith("Self::") and symbol["owner"] and not symbol["trait_impl"]:
                targets = {symbol["scope"] + "::" + symbol["owner"] + expr[4:]}
            elif fn.type == "field_expression":
                receiver = text(field(fn, "value"))
                method = text(field(fn, "field"))
                if receiver == "self" and symbol["owner"] and not symbol["trait_impl"]:
                    targets = {symbol["scope"] + "::" + symbol["owner"] + "::" + method}
            elif fn.type in {"identifier", "scoped_identifier"} and expr not in bindings:
                for p in paths(expr, symbol["scope"]):
                    targets |= expand_exports(p)
            candidates = {s["id"]: s for p in targets for s in by_qualified[p]
                          if s["kind"] == "function_item" and not s["trait_impl"]}
            target = None
            if len(candidates) == 1:
                target = next(iter(candidates))
                resolution = "syntactic-direct"
            elif len(candidates) > 1:
                resolution = "ambiguous-cfg-or-overload"
            elif fn.type == "field_expression":
                resolution = "receiver-type-required"
            else:
                resolution = "external-constructor-callback-or-unresolved"
            calls.append({"caller": symbol["id"], "line": node.start_point.row+1,
                          "expression": original, "target": target, "resolution": resolution})
    inputs = {f["path"]: f["sha256"] for f in files}
    for p in [ROOT / "Cargo.toml", ROOT / "Cargo.lock", ROOT / "rust-toolchain.toml",
              Path(__file__), ROOT / "scripts/architecture-requirements.txt"] + [ROOT/p["manifest"] for p in packages]:
        inputs[rel(p)] = hashlib.sha256(p.read_bytes()).hexdigest()
    digest = hashlib.sha256(json.dumps(inputs, sort_keys=True).encode()).hexdigest()
    observed_symbols = Counter(s["kind"] for s in symbols)
    if observed_symbols != expected_symbols or len(calls) != expected_calls:
        raise ValueError(f"Incomplete AST coverage: symbols {observed_symbols} / {expected_symbols}; "
                         f"calls {len(calls)} / {expected_calls}")
    if len(symbols) != len({s["id"] for s in symbols}):
        raise ValueError("Duplicate symbol IDs; refuse ambiguous source navigation")
    return {"format": 1, "source_digest": digest, "inputs": inputs,
            "packages": packages, "files": files, "modules": modules, "symbols": symbols,
            "uses": uses, "calls": calls, "macro_sites": macros, "literal_includes": includes}


def outputs(data):
    result = {}
    result[rel(OUT/"inventory.json")] = json.dumps(data, ensure_ascii=False, indent=2) + "\n"
    symbols = {s["id"]: s for s in data["symbols"]}
    outgoing = defaultdict(list)
    incoming = defaultdict(list)
    for c in data["calls"]:
        outgoing[c["caller"]].append(c)
        if c["target"]:
            incoming[c["target"]].append(c)
    pkgs = data["packages"]
    index = ["# Generated source atlas", "", "Generated by `scripts/code-architecture.py`; do not edit by hand.",
             "", "[Architecture guide](../README.md) · [Method and limitations](../method.md)", "",
             f"Source digest: `{data['source_digest']}`.", "",
             "Declared visibility is not effective export reachability. Tests/cfg branches are indexed, not executed.",
             "Calls are syntactic sites; unresolved receiver dispatch, constructors, callbacks and macro expansion are not fabricated as direct edges.", "",
             "| Packages | Rust files | Symbols | Call sites | Syntactically resolved | Macro sites |",
             "|---:|---:|---:|---:|---:|---:|",
             f"| {len(pkgs)} | {len(data['files'])} | {len(symbols)} | {len(data['calls'])} | {sum(c['target'] is not None for c in data['calls'])} | {len(data['macro_sites'])} |", "",
             "The complete inventory includes test/example/build sources, every parsed function and every call expression outside unexpanded macros. No caller found means no indexed direct caller, not dead code.", "",
             "| Package | Library / binary / test targets | Source atlas |", "|---|---|---|"]
    deps = ["flowchart LR"]
    ids = {p["name"]: f"p{i}" for i,p in enumerate(pkgs)}
    for p in pkgs:
        deps.append(f'  {ids[p["name"]]}["{p["name"]}"]')
    for p in pkgs:
        for d in p["dependencies"]:
            if d["kind"] == "normal" and d["name"] in ids:
                deps.append(f'  {ids[p["name"]]} --> {ids[d["name"]]}')
    result[rel(OUT/"dependencies.mmd")] = "\n".join(deps)+"\n"
    for p in pkgs:
        directory = p["directory"]
        crate_out = ROOT/"crates"/directory/"docs/generated"
        page = crate_out/"index.md"
        index.append(f"| `{p['name']}` | {len(p['targets'])} targets | [{directory}]({link(rel(page))}) |")
        lines = [f"# {p['name']} — generated atlas", "",
                 f"[All packages]({link(rel(OUT/'index.md'), origin=page)}) · "
                 "[Maintained explanation](../README.md)", "",
                 "## Cargo targets", "", "| Target | Kind | Entry |", "|---|---|---|"]
        for t in p["targets"]:
            lines.append(f"| `{t['name']}` | {', '.join(t['kind'])} | [{t['path']}]({link(t['path'], origin=page)}) |")
        lines += ["", "## Direct workspace dependencies", "", "| Dependency | Kind | Target cfg |", "|---|---|---|"]
        for d in p["dependencies"]:
            lines.append(f"| `{d['name']}` | {d['kind']} | {esc(d['target'] or 'all')} |")
        lines += ["", "[Public/restricted API and direct callers](api.md)",
                  "", "## Source files / lexical modules", "", "`src` files have full symbol/call tables and partitioned function graphs. Integration tests and examples remain in the JSON inventory.",
                  "File-derived paths below are lexical navigation, not a compiler-resolved module tree; inline module declarations are listed separately.", "",
                  "| File | Symbols | Call sites | Detail |", "|---|---:|---:|---|"]
        package_files = [f for f in data["files"] if f["package"] == p["name"]]
        for f in package_files:
            fs = [s for s in symbols.values() if s["path"] == f["path"]]
            fc = [c for s in fs for c in outgoing[s["id"]]]
            slug = f["path"].split(f"crates/{directory}/",1)[1].replace("/", "--").removesuffix(".rs")
            source_page = crate_out/(slug+".md")
            detailed = "/src/" in f["path"] or f["path"].endswith("/build.rs")
            detail = f"[Symbols and calls]({slug}.md)" if detailed else "`inventory.json`"
            lines.append(f"| [{f['path']}]({link(f['path'],origin=page)}) | {len(fs)} | {len(fc)} | {detail} |")
            if not detailed:
                continue
            content = [f"# {f['scope']}", "", f"[Package atlas](index.md) · [Source]({link(f['path'],origin=source_page)})", "",
                       "## Declarations", "", "Visibility is the declaration spelling; trait members and reexports require their enclosing interface. `cfg` is not evaluated.", "",
                       "| Symbol | Kind | Visibility | Test / cfg |", "|---|---|---|---|"]
            for s in fs:
                content.append(f"| [{esc(s['qualified'])}]({link(s['path'],s['line'],source_page)}) | {s['kind']} | `{s['visibility']}` | {esc(('test; ' if s['test'] else '') + ' '.join(s['cfg']))} |")
            content += ["", "## Imports / reexports", "", "| Local name | Source path | Visibility |", "|---|---|---|"]
            for u in data["uses"]:
                if u["path"] == f["path"]:
                    content.append(f"| `{esc(u['alias'])}` | `{esc(u['target'])}` | `{u['visibility']}` |")
            content += ["", "## Module declarations", "", "| Module | Visibility | Attributes |", "|---|---|---|"]
            for m in data["modules"]:
                if m["path"] == f["path"]:
                    content.append(f"| `{m['scope']}::{m['name']}` | `{m['visibility']}` | {esc(' '.join(m['attributes']))} |")
            content += ["", "## Function call graphs", "", "Edges below are syntactically resolved calls only, including private functions. Graphs partition callers into groups of 20; they are not execution order. All unresolved sites are listed below and in the JSON inventory.", ""]
            functions = [s for s in fs if s["kind"] == "function_item" and not s["test"]]
            for offset in range(0, len(functions), 20):
                group = functions[offset:offset+20]
                edges = {(s["id"],c["target"]) for s in group for c in outgoing[s["id"]] if c["target"]}
                nodes = sorted({s["id"] for s in group} | {b for _,b in edges})
                nodeids = {n:f"n{i}" for i,n in enumerate(nodes)}
                graph = ["flowchart TD"]
                for n in nodes:
                    graph.append(f'  {nodeids[n]}["{mermaid_label(symbols[n]["qualified"])}"]')
                for a,b in sorted(edges):
                    graph.append(f"  {nodeids[a]} --> {nodeids[b]}")
                content += [f"<details><summary>Functions {offset+1}–{offset+len(group)}: {len(edges)} direct edges</summary>", "", "```mermaid", *graph, "```", "", "</details>", ""]
            content += ["## Call sites", "", "Includes test functions (marked in declarations). Receiver-type-required sites need type analysis/manual tracing. Calls in closures are attributed to their enclosing function; their occurrence here does not mean the closure executes immediately.", "",
                        "| Caller | Callee expression | Source lines | Target / classification |", "|---|---|---|---|"]
            grouped = defaultdict(list)
            for c in fc:
                grouped[(c["caller"],c["expression"],c["target"],c["resolution"])].append(c["line"])
            for (caller,expr,target,resolution), nums in grouped.items():
                dst = symbols.get(target)
                destination = f"[{esc(dst['qualified'])}]({link(dst['path'],dst['line'],source_page)})" if dst else resolution
                line_links = ", ".join(f"[{n}]({link(f['path'],n,source_page)})" for n in sorted(set(nums)))
                content.append(f"| `{esc(symbols[caller]['name'])}` | `{esc(expr)}` | {line_links} | {destination} |")
            result[rel(source_page)] = "\n".join(content)+"\n"
        # Module-level call relationships use file identities, not guessed imports.
        pedges = Counter()
        for c in data["calls"]:
            a,b = symbols[c["caller"]], symbols.get(c["target"])
            if b and a["package"] == p["name"] and b["package"] == p["name"] and a["path"] != b["path"] and not a["test"]:
                pedges[(a["path"],b["path"])] += 1
        lines += ["", "## Cross-file direct calls", "", "Source files stand in for out-of-line modules; see declarations for inline modules. Counts are syntactic call sites, not runtime frequency.", "", "```mermaid", "flowchart LR"]
        node_paths = sorted({x for pair in pedges for x in pair})
        mids = {x:f"m{i}" for i,x in enumerate(node_paths)}
        for path in node_paths:
            lines.append(f'  {mids[path]}["{path.split("/src/")[-1]}"]')
        for (a,b),count in sorted(pedges.items()):
            lines.append(f'  {mids[a]} -->|"{count}"| {mids[b]}')
        if not node_paths:
            lines.append('  none["No resolved cross-file calls; inspect the call-site inventory"]')
        lines += ["```", ""]
        result[rel(page)] = "\n".join(lines)
        api_page = crate_out/"api.md"
        api = [f"# {p['name']} — public/restricted declarations and direct callers", "",
               "[Package atlas](index.md)", "",
               "Includes pub, pub(crate), pub(super), and other restricted declarations; a pub method in a binary is not an importable library API. A missing direct caller is not evidence of dead code. Trait implementation methods are indexed in source pages even without a pub keyword.", "",
               "| Declaration | Visibility | Direct caller functions (all indexed configurations) |", "|---|---|---|"]
        for s in symbols.values():
            if s["package"] != p["name"] or s["visibility"] == "private" or s["test"]:
                continue
            callers = sorted({c["caller"] for c in incoming[s["id"]]})
            refs = [f"[{esc(symbols[c]['qualified'])}]({link(symbols[c]['path'],symbols[c]['line'],api_page)})" for c in callers]
            api.append(f"| [{esc(s['qualified'])}]({link(s['path'],s['line'],api_page)}) | `{s['visibility']}` | {'; '.join(refs) or ('not a function' if not s['kind'].startswith('function') else 'no resolved direct caller')} |")
        result[rel(api_page)] = "\n".join(api)+"\n"
    index += ["", "## Normal workspace dependencies", "", "Arrows mean Cargo dependency, not runtime calls. Per-package tables distinguish dev/build dependencies.", "", "```mermaid", *deps, "```", "",
              "## Cross-package direct calls", "", "Only resolved non-test call sites. This is distinct from the Cargo dependency graph.", "", "```mermaid", "flowchart LR"]
    cross = Counter()
    for c in data["calls"]:
        a,b = symbols[c["caller"]], symbols.get(c["target"])
        if b and not a["test"] and a["package"] != b["package"]:
            cross[(a["package"],b["package"])] += 1
    for name in sorted({n for pair in cross for n in pair}):
        index.append(f'  {ids[name]}["{name}"]')
    for (a,b),count in sorted(cross.items()):
        index.append(f'  {ids[a]} -->|"{count} sites"| {ids[b]}')
    index += ["```", "", "Running `python3 scripts/code-architecture.py` also writes the machine-readable inventory `docs/architecture/generated/inventory.json` (not tracked in Git; regenerate it locally when needed). It includes call expressions, resolution status, symbol IDs, source lines, imports, module declarations, cfg annotations and per-file SHA-256 fingerprints.", ""]
    result[rel(OUT/"index.md")] = "\n".join(index)
    return result


def check_links():
    failures = []
    documents = ([ROOT/"README.md"] + sorted((ROOT/"docs").rglob("*.md"))
                 + sorted((ROOT/"spec").glob("*.md"))
                 + sorted((ROOT/"crates").glob("*/docs/**/*.md")))
    @lru_cache(maxsize=None)
    def line_count(path):
        return len(path.read_text().splitlines())
    for path in documents:
        if "reviews" in path.parts:
            continue  # historical links are not current navigation contracts
        body = re.sub(r"```.*?```", "", path.read_text(), flags=re.S)
        for target in re.findall(r"\]\(([^\s)]+)(?:\s+\"[^\"]*\")?\)", body):
            if re.match(r"[a-zA-Z]+:",target) or target.startswith("#"):
                continue
            location, _, anchor = target.partition("#")
            dest = (path.parent/location).resolve()
            if not dest.exists():
                failures.append(f"{rel(path)}: missing {target}")
            elif re.fullmatch(r"L\d+",anchor) and dest.is_file():
                if int(anchor[1:]) > line_count(dest):
                    failures.append(f"{rel(path)}: source line out of range {target}")
    return failures


INVENTORY = rel(OUT/"inventory.json")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true")
    parser.add_argument("--links", action="store_true")
    args = parser.parse_args()
    if args.links:
        errors = check_links()
    else:
        data = analyze()
        generated = outputs(data)
        errors = []
        generated_roots = [OUT] + sorted((ROOT/"crates").glob("*/docs/generated"))
        existing = {rel(p) for folder in generated_roots for p in folder.rglob("*") if p.is_file()}
        for name, content in generated.items():
            path = ROOT/name
            if args.check and name == INVENTORY:
                # The JSON inventory is a local build product, not tracked in Git.
                continue
            if args.check:
                if not path.exists() or path.read_text() != content:
                    errors.append(f"stale generated file: {rel(path)}")
            else:
                path.parent.mkdir(parents=True, exist_ok=True)
                path.write_text(content)
        for obsolete in sorted(existing-set(generated)-{INVENTORY}):
            errors.append(f"obsolete generated file (remove after review): {obsolete}")
        print(f"{len(data['packages'])} packages; {len(data['files'])} Rust files; "
              f"{len(data['symbols'])} symbols; {len(data['calls'])} call sites; "
              f"{sum(c['target'] is not None for c in data['calls'])} resolved; {len(generated)} artifacts")
    for error in errors:
        print(error, file=sys.stderr)
    return bool(errors)


if __name__ == "__main__":
    sys.exit(main())
